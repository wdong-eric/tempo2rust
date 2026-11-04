//! Native ABI adapter for tempo2pred.c/cheby2d.c/t1polyco.c. C owns the exposed
//! layout and native scalars; Rust borrows them synchronously and runs AF leaves.
use crate::{
    cheby2d::{self, Cheby2D},
    predictor_number::{N, n},
    t1polyco::{self, PhasePolynomial},
    tempo2pred::{self, Prediction},
};
use std::{
    ffi::{c_int, c_void},
    panic::{AssertUnwindSafe, catch_unwind},
    ptr::NonNull,
};

// Necessity: Rust has no portable C long-double ABI. C performs only typed
// representation conversion/allocation and phase dispatch. Pointers are borrowed
// for the call, properly aligned native storage; no C pointer escapes an owner.
unsafe extern "C" {
    fn t2r_ld_parts(
        p: *const c_void,
        i: usize,
        significand: *mut u64,
        exponent: *mut c_int,
    ) -> c_int;
    fn t2r_ld_from_parts(p: *mut c_void, i: usize, significand: u64, exponent: c_int, kind: c_int);
    fn t2r_ld_alloc(n: usize) -> *mut c_void;
    fn t2r_ld_free(p: *mut c_void);
    fn t2r_phase_call(p: *const c_void, mjd: *const c_void, freq: *const c_void, out: *mut c_void);
}
type Callback =
    unsafe extern "C" fn(*mut c_void, *mut c_void, c_int, c_int, *mut c_void, *mut c_void);

// Private adapter helpers may only be called after the exported shim established
// the documented C extent/lifetime preconditions. Only the macOS evaluation
// accessor below views native binary64 storage; other reads construct N owners.
fn read(p: *const c_void, i: usize) -> Result<N, String> {
    if p.is_null() {
        return Err("null native scalar backing".into());
    }
    let (mut significand, mut exponent) = (0, 0);
    // C reads the current initialized p[i], writes only these exclusive local
    // integers, and retains no pointers. No Rust reference aliases native data.
    let kind = unsafe { t2r_ld_parts(p, i, &mut significand, &mut exponent) };
    let value = match kind.abs() {
        1 => N::from_native_parts(significand, exponent),
        2 => n(f64::INFINITY),
        3 => n(f64::NAN),
        _ => unreachable!("native scalar kind"),
    };
    Ok(if kind < 0 { -value } else { value })
}
fn write(p: *mut c_void, i: usize, v: &N) -> Result<(), String> {
    if p.is_null() {
        return Err("null native output backing".into());
    }
    let value = v.as_float();
    let (significand, exponent, kind) = if let Some(significand) = value.get_significand() {
        // Rug's borrowed, positive significand is padded to whole GMP limbs.
        // Both qualified 53/64-bit profiles fit in u64, including that padding.
        (
            significand.to_u64().expect("ordinary significand fits u64"),
            value.get_exp().unwrap() - significand.significant_bits() as i32,
            1,
        )
    } else if value.is_nan() {
        (0, 0, 3)
    } else if value.is_infinite() {
        (0, 0, 2)
    } else {
        (0, 0, 1)
    };
    let kind = if value.is_sign_negative() {
        -kind
    } else {
        kind
    };
    // C writes one exclusive native cell. The canonical owner has already
    // rounded to the host range/precision, so integer conversion and ldexpl
    // reconstruct it exactly. No borrowed Rug limb or C pointer escapes.
    unsafe {
        t2r_ld_from_parts(p, i, significand, exponent, kind);
    }
    Ok(())
}
fn values(p: *const c_void, count: usize) -> Result<Vec<N>, String> {
    (0..count).map(|i| read(p, i)).collect()
}
fn shape(nx: c_int, ny: c_int) -> Result<(usize, usize), String> {
    if nx <= 0 || ny <= 0 || nx.checked_mul(ny).is_none() {
        return Err("invalid native polynomial dimensions".into());
    }
    Ok((nx as usize, ny as usize))
}
fn polynomial(p: *const c_void, nx: c_int, ny: c_int) -> Result<Cheby2D, String> {
    let (x, y) = shape(nx, ny)?;
    Cheby2D::from_coefficients(x, y, values(p, x * y)?)
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn native_coefficient(value: f64) -> f64 {
    // t2r_ld_parts preserves Darwin's positive, quiet NaN input convention.
    if value.is_nan() { f64::NAN } else { value }
}

fn evaluator(p: *const c_void, nx: c_int, ny: c_int) -> Result<impl FnOnce(&N, &N) -> N, String> {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        let (nx, ny) = shape(nx, ny)?;
        if p.is_null() {
            return Err("null native scalar backing".into());
        }
        Ok(move |x: &N, y: &N| {
            // The C ABI guards establish binary64 size/alignment. The caller
            // supplies initialized, aligned, single-allocation backing without
            // mutation for this call; checked c_int extent fits isize bytes.
            // This slice ends here, before output publication or any callback.
            let coeff = unsafe { std::slice::from_raw_parts(p.cast::<f64>(), nx * ny) };
            cheby2d::evaluate(nx, ny, |i| native_coefficient(coeff[i]), x, y)
        })
    }
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        let poly = polynomial(p, nx, ny)?;
        Ok(move |x: &N, y: &N| poly.evaluate(x, y))
    }
}
// tempo2pred.c/cheby2d.c/t1polyco.c counterparts acquire an extra Rust/Rug
// lifetime here: foreign pthread exit retained 544 bytes in 16 MPFR cache
// allocations on Linux (0109). A calling-thread owner releases only its local
// caches. This fixes the Rust integration, not a native TEMPO2 algorithm.
struct ForeignThreadCache;
impl Drop for ForeignThreadCache {
    fn drop(&mut self) {
        rug::float::free_cache(rug::float::FreeCache::Local);
    }
}
thread_local! {
    static FOREIGN_THREAD_CACHE: ForeignThreadCache = const { ForeignThreadCache };
}

fn finish(f: impl FnOnce() -> Result<(), String>) -> c_int {
    match catch_unwind(AssertUnwindSafe(|| {
        FOREIGN_THREAD_CACHE.with(|_| {});
        f()
    })) {
        Ok(Ok(())) => 0,
        Ok(Err(e)) => {
            eprintln!("tempo2pred-rust: {e}");
            -1
        }
        Err(_) => {
            eprintln!("tempo2pred-rust: Rust panic contained at native boundary");
            -1
        }
    }
}
struct NativeBuffer(NonNull<c_void>);
impl NativeBuffer {
    fn new(count: usize) -> Result<Self, String> {
        // Native allocator provides native scalar size/alignment. This unique
        // owner releases exactly once; callback access ends before Drop.
        NonNull::new(unsafe { t2r_ld_alloc(count) })
            .map(Self)
            .ok_or("native allocation failed".into())
    }
    fn from_values(v: &[N]) -> Result<Self, String> {
        let p = Self::new(v.len())?;
        for (i, n) in v.iter().enumerate() {
            write(p.0.as_ptr(), i, n)?;
        }
        Ok(p)
    }
}
impl Drop for NativeBuffer {
    fn drop(&mut self) {
        // Paired native malloc/free; all synchronous callback borrows have ended.
        unsafe {
            t2r_ld_free(self.0.as_ptr());
        }
    }
}
fn callback(func: Option<Callback>, info: *mut c_void, x: &[N], y: &[N]) -> Result<Vec<N>, String> {
    let func = func.ok_or("null predictor callback")?;
    let xx = NativeBuffer::from_values(x)?;
    let yy = NativeBuffer::from_values(y)?;
    let z = NativeBuffer::new(x.len() * y.len())?;
    // The C callback receives native arrays of exactly the advertised extents
    // and the unchanged opaque context. It must initialize every z cell, must
    // not retain pointers, throw C++ exceptions or longjmp across Rust frames.
    unsafe {
        func(
            xx.0.as_ptr(),
            yy.0.as_ptr(),
            x.len() as c_int,
            y.len() as c_int,
            z.0.as_ptr(),
            info,
        );
    }
    values(z.0.as_ptr(), x.len() * y.len())
}

// Each exported symbol is called only by native/predictor.c. Its C declaration
// fixes the pointer/int ABI; the caller provides live backing of the checked
// shape and exclusive outputs. catch_unwind prevents propagation across C.
#[unsafe(no_mangle)]
extern "C" fn t2r_evaluate(
    p: *const c_void,
    nx: c_int,
    ny: c_int,
    x: *const c_void,
    y: *const c_void,
    out: *mut c_void,
) -> c_int {
    finish(|| {
        let value = evaluator(p, nx, ny)?(&read(x, 0)?, &read(y, 0)?);
        write(out, 0, &value)
    })
}
#[unsafe(no_mangle)]
extern "C" fn t2r_derivative(p: *const c_void, nx: c_int, ny: c_int, out: *mut c_void) -> c_int {
    finish(|| {
        let d = polynomial(p, nx, ny)?.x_derivative()?;
        for (i, v) in d.coefficients().iter().enumerate() {
            write(out, i, v)?;
        }
        Ok(())
    })
}
#[unsafe(no_mangle)]
extern "C" fn t2r_construct(
    nx: c_int,
    ny: c_int,
    out: *mut c_void,
    func: Option<Callback>,
    info: *mut c_void,
) -> c_int {
    finish(|| {
        let (x, y) = shape(nx, ny)?;
        let p = Cheby2D::construct(x, y, |x, y| callback(func, info, x, y))?;
        for (i, v) in p.coefficients().iter().enumerate() {
            write(out, i, v)?;
        }
        Ok(())
    })
}
#[unsafe(no_mangle)]
extern "C" fn t2r_test(
    p: *const c_void,
    nx: c_int,
    ny: c_int,
    tx: c_int,
    ty: c_int,
    func: Option<Callback>,
    info: *mut c_void,
    rms: *mut c_void,
    mav: *mut c_void,
) -> c_int {
    finish(|| {
        let (x, y) = shape(tx, ty)?;
        let e = polynomial(p, nx, ny)?.test(x, y, |x, y| callback(func, info, x, y))?;
        write(rms, 0, &e.rms_turn)?;
        write(mav, 0, &e.mav_turn)
    })
}
#[unsafe(no_mangle)]
extern "C" fn t2r_model(
    p: *const c_void,
    nx: c_int,
    ny: c_int,
    v: *const c_void,
    frequency: c_int,
    out: *mut c_void,
) -> c_int {
    finish(|| {
        let a = [
            read(v, 0)?,
            read(v, 1)?,
            read(v, 2)?,
            read(v, 3)?,
            read(v, 4)?,
            read(v, 5)?,
            read(v, 6)?,
        ];
        let evaluate = evaluator(p, nx, ny)?;
        let time = [&a[0], &a[1]];
        let band = [&a[2], &a[3]];
        let value = if frequency != 0 {
            cheby2d::model_frequency(evaluate, time, band, &a[5], &a[6])
        } else {
            cheby2d::model_phase(evaluate, time, band, &a[4], &a[5], &a[6])
        };
        write(out, 0, &value)
    })
}
fn count(n: c_int) -> Result<usize, String> {
    if !(0..=32).contains(&n) {
        Err("T1 count outside fixed coeff[32]".into())
    } else {
        Ok(n as usize)
    }
}
#[unsafe(no_mangle)]
extern "C" fn t2r_t1phase(
    p: *const c_void,
    nc: c_int,
    v: *const c_void,
    dm: f64,
    doppler: f64,
    obs: f64,
    out: *mut c_void,
) -> c_int {
    finish(|| {
        let count = count(nc)?;
        let a = values(v, 5)?;
        let coeff = values(p, count.max(2))?;
        let model = PhasePolynomial {
            mjd_mid: &a[0],
            reference_phase: &a[1],
            frequency_psr_0: &a[2],
            dm,
            doppler,
            frequency_obs: obs,
            coeff: &coeff,
            count,
        };
        write(out, 0, &model.phase(&a[3], &a[4]))
    })
}
#[unsafe(no_mangle)]
extern "C" fn t2r_t1frequency(
    p: *const c_void,
    nc: c_int,
    v: *const c_void,
    out: *mut c_void,
) -> c_int {
    finish(|| {
        let count = count(nc)?;
        let a = values(v, 3)?;
        // p starts at coeff[1]. Frequency never reads C0 or phase/dispersion state.
        let coeff = values(p, count.max(2) - 1)?;
        write(
            out,
            0,
            &t1polyco::frequency_leaf(&a[0], &a[1], &coeff, count, &a[2]),
        )
    })
}
#[unsafe(no_mangle)]
extern "C" fn t2r_nearest(p: *const c_void, nc: c_int, cheby: c_int, mjd: *const c_void) -> c_int {
    let mut index = -1;
    finish(|| {
        let count = usize::try_from(nc).map_err(|_| "negative segment count")?;
        let t = read(mjd, 0)?;
        if cheby != 0 {
            let a = values(p, count * 2)?;
            index = cheby2d::nearest_index(a.chunks_exact(2).map(|v| (&v[0], &v[1])), &t)
                .map_or(-1, |i| i as c_int);
        } else if count > 0 {
            index = t1polyco::nearest_index(values(p, count)?.iter(), &t) as c_int;
        }
        Ok(())
    });
    index
}
#[unsafe(no_mangle)]
extern "C" fn t2r_plan(
    pred: *const c_void,
    v: *const c_void,
    phase: *mut c_void,
    count: *mut c_int,
    out: *mut c_void,
) -> c_int {
    finish(|| {
        if count.is_null() {
            return Err("null plan count".into());
        }
        let a = values(v, 4)?;
        let (p, segments) = tempo2pred::plan_samples(&a[0], &a[1], &a[2], &a[3], |mjd, freq| {
            let m = NativeBuffer::from_values(std::slice::from_ref(mjd))?;
            let f = NativeBuffer::from_values(std::slice::from_ref(freq))?;
            let out = NativeBuffer::new(1)?;
            // pred stays borrowed and live until the C wrapper destroys it after
            // this call. Native dispatch writes exactly one native scalar.
            unsafe {
                t2r_phase_call(pred, m.0.as_ptr(), f.0.as_ptr(), out.0.as_ptr());
            }
            Ok(Prediction {
                segment: None,
                out_of_range: false,
                phase_turn: read(out.0.as_ptr(), 0)?,
                frequency_hz: n(0.),
            })
        })?;
        // C API requires a live int destination and frequency capacity >= the
        // source ceiling count. The checked shared engine caps it to INT_MAX.
        unsafe {
            *count = segments.len() as c_int;
        }
        write(phase, 0, &p)?;
        for (i, s) in segments.iter().enumerate() {
            write(out, i, &s.average_frequency_hz)?;
        }
        Ok(())
    })
}
#[unsafe(no_mangle)]
extern "C" fn t2r_span(mid: *const c_void, span: c_int, sign: c_int, out: *mut c_void) -> c_int {
    finish(|| write(out, 0, &t1polyco::span_bound(&read(mid, 0)?, span, sign)))
}
#[unsafe(no_mangle)]
extern "C" fn t2r_testfunc(
    x: *const c_void,
    y: *const c_void,
    nx: c_int,
    ny: c_int,
    z: *mut c_void,
) -> c_int {
    finish(|| {
        let (nx, ny) = shape(nx, ny)?;
        let x = values(x, nx)?;
        let y = values(y, ny)?;
        for (j, y) in y.iter().enumerate() {
            for (i, x) in x.iter().enumerate() {
                write(z, j * nx + i, &cheby2d::test_function(x, y))?;
            }
        }
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString, c_char};

    // Retained event-libm text transport is an independent before-path oracle.
    unsafe extern "C" {
        fn t2r_ld_get(p: *const c_void, i: usize, s: *mut c_char) -> c_int;
        fn t2r_ld_set(p: *mut c_void, i: usize, s: *const c_char) -> c_int;
    }

    fn text(p: &NativeBuffer) -> String {
        let mut out = [0 as c_char; 128];
        // p owns a live native cell; C writes a bounded NUL-terminated buffer.
        assert_eq!(unsafe { t2r_ld_get(p.0.as_ptr(), 0, out.as_mut_ptr()) }, 0);
        unsafe { CStr::from_ptr(out.as_ptr()) }
            .to_str()
            .unwrap()
            .into()
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    #[test]
    fn predictor_native_coefficient_matches_c_parts() {
        assert_eq!(std::mem::size_of::<f64>(), 8);
        assert_eq!(std::mem::align_of::<f64>(), 8);
        let mut patterns = Vec::new();
        for bits in [
            0,
            1,
            2,
            0x000f_ffff_ffff_fffe,
            0x000f_ffff_ffff_ffff,
            0x0010_0000_0000_0000,
            0x0010_0000_0000_0001,
            0x3fef_ffff_ffff_ffff,
            0x3ff0_0000_0000_0000,
            0x3ff0_0000_0000_0001,
            0x7fef_ffff_ffff_fffe,
            0x7fef_ffff_ffff_ffff,
            0x7ff0_0000_0000_0000,
            0x7ff0_0000_0000_0001,
            0x7ff7_ffff_ffff_ffff,
            0x7ff8_0000_0000_0000,
            0x7fff_ffff_ffff_ffff,
        ] {
            patterns.extend([bits, bits | (1 << 63)]);
        }
        let mut bits = 0xa076_1d64_78bd_642fu64;
        for _ in 0..256 {
            bits ^= bits << 13;
            bits ^= bits >> 7;
            bits ^= bits << 17;
            patterns.push(bits);
        }
        let backing = NativeBuffer::new(1).unwrap();
        for bits in patterns {
            // Native malloc owns an aligned eight-byte cell. Initialize its
            // representation, then compare the Rust view with C's typed read.
            unsafe { backing.0.as_ptr().cast::<u64>().write(bits) };
            let expected = read(backing.0.as_ptr(), 0).unwrap().to_f64();
            let actual = native_coefficient(unsafe { backing.0.as_ptr().cast::<f64>().read() });
            assert_eq!(actual.is_nan(), expected.is_nan(), "{bits:016x}");
            assert_eq!(
                actual.is_sign_negative(),
                expected.is_sign_negative(),
                "{bits:016x}"
            );
            if actual.is_nan() {
                assert_ne!(actual.to_bits() & (1 << 51), 0, "quiet NaN");
            } else {
                assert_eq!(actual.to_bits(), expected.to_bits(), "{bits:016x}");
            }
            assert_eq!(unsafe { backing.0.as_ptr().cast::<u64>().read() }, bits);
        }
    }

    #[test]
    fn binary_transport_matches_native_text_boundaries() {
        let mut tokens: Vec<String> = [
            "0",
            "-0",
            "inf",
            "-inf",
            "nan",
            "-nan",
            "0x1p-1074",
            "-0x1p-1074",
            "0x1p-1022",
            "0x1.fffffffffffffp1023",
            "0x1p-16445",
            "-0x1p-16445",
            "0x1p-16382",
            "0x1.fffffffffffffffep16383",
            "0x1.0000000000000002p0",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();
        let mut bits = 0x123456789abcdef0_u64;
        for exponent in [
            -16510, -16445, -16444, -16400, -16382, -1138, -1075, -1074, -1073, -1022, -64, -63, 0,
            960, 16319,
        ] {
            for _ in 0..32 {
                bits ^= bits << 13;
                bits ^= bits >> 7;
                bits ^= bits << 17;
                tokens.push(format!("0x{bits:x}p{exponent}"));
                tokens.push(format!("-0x{bits:x}p{exponent}"));
            }
        }
        let input = NativeBuffer::new(1).unwrap();
        let output = NativeBuffer::new(1).unwrap();
        let old_output = NativeBuffer::new(1).unwrap();
        for token in tokens {
            let c = CString::new(token.clone()).unwrap();
            // Each buffer owns one exclusive native cell; text remains live.
            assert_eq!(unsafe { t2r_ld_set(input.0.as_ptr(), 0, c.as_ptr()) }, 0);
            let old = N::parse_number(&text(&input)).unwrap();
            let new = read(input.0.as_ptr(), 0).unwrap();
            assert_eq!(old.as_float().is_nan(), new.as_float().is_nan(), "{token}");
            assert_eq!(
                old.as_float().is_sign_negative(),
                new.as_float().is_sign_negative(),
                "{token}"
            );
            if !old.as_float().is_nan() {
                assert_eq!(old, new, "{token}");
            }
            write(output.0.as_ptr(), 0, &new).unwrap();
            let old_text = CString::new(if new.as_float().is_nan() {
                if new.as_float().is_sign_negative() {
                    "-nan".into()
                } else {
                    "nan".into()
                }
            } else {
                new.significant(25)
            })
            .unwrap();
            assert_eq!(
                unsafe { t2r_ld_set(old_output.0.as_ptr(), 0, old_text.as_ptr()) },
                0
            );
            assert_eq!(text(&output), text(&old_output), "{token}");
        }
        assert!(read(std::ptr::null(), 0).is_err());
        assert!(write(std::ptr::null_mut(), 0, &n(1.)).is_err());
    }
}
