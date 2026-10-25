//! sortToAs.C: native comparator and host qsort tie behavior, with the source latch.
use crate::{
    TimingError,
    observation::{SourceObservation, read},
    pulsar_lifecycle::PulsarContext,
};
use std::ffi::{c_int, c_void};
unsafe extern "C" {
    fn qsort(
        base: *mut c_void,
        count: usize,
        size: usize,
        compare: extern "C" fn(*const c_void, *const c_void) -> c_int,
    );
}
pub fn compare_obs(a: &SourceObservation, b: &SourceObservation) -> Result<i32, TimingError> {
    let a = read(&a.sat, "sat")?;
    let b = read(&b.sat, "sat")?;
    Ok(if a < b {
        -1
    } else if a > b {
        1
    } else {
        0
    })
}
extern "C" fn compare(a: *const c_void, b: *const c_void) -> c_int {
    // qsort supplies aligned live elements in the exclusively borrowed slice.
    // All SAT fields were checked before entering C, so this callback cannot
    // unwind. No reference escapes; equal and unordered values both return zero.
    let (a, b) = unsafe {
        (
            &*a.cast::<SourceObservation>(),
            &*b.cast::<SourceObservation>(),
        )
    };
    compare_obs(a, b).expect("validated qsort observation SAT") as c_int
}
pub fn sort_toas(context: &mut PulsarContext, index: usize) -> Result<(), TimingError> {
    let (p, rows) = context.pulsar_observations_mut(index)?;
    if *read(&p.sorted, "sorted")? == 0 {
        crate::clkcorr::with_process(|s| {
            s.diagnostics.log(
                b'm',
                "sortToAs.C",
                14,
                &[
                    b"Sorting ToAs for psr ".as_slice(),
                    &crate::observation::read_string(&p.name[..], "name")?,
                ]
                .concat(),
            );
            s.diagnostics.timing("sortToAs.C", 15, b"call qsort()");
            Ok(())
        })?;
        let rows = &mut rows[..*read(&p.nobs, "nobs")? as usize];
        for o in rows.iter() {
            read(&o.sat, "sat")?;
        }
        // Native qsort is required because Rust sort does not promise the source
        // host's equal-key order. Moving bytes is valid for these owned, unpinned
        // Rust values: qsort only permutes whole elements, and no element borrow
        // exists outside each comparator call. Ownership is unchanged on return.
        unsafe {
            qsort(
                rows.as_mut_ptr().cast(),
                rows.len(),
                std::mem::size_of::<SourceObservation>(),
                compare,
            );
        }
        p.sorted = Some(1);
        crate::clkcorr::with_process(|s| {
            s.diagnostics.timing("sortToAs.C", 18, b"exit qsort()");
            Ok(())
        })?;
    }
    Ok(())
}
