//! Native ABI adapter for tempo2pred.c/cheby2d.c/t1polyco.c. C owns the exposed
//! layout and native scalars; Rust borrows them synchronously and runs AF leaves.
use crate::{
    cheby2d::{self, Cheby2D},
    predictor_number::{N, n},
    t1polyco::{self, PhasePolynomial},
    tempo2pred::{self, Prediction},
};
use std::{
    ffi::{CStr, CString, c_char, c_int, c_void},
    panic::{AssertUnwindSafe, catch_unwind},
    ptr::NonNull,
};

// Necessity: Rust has no portable C long-double ABI. C performs only typed
// representation conversion/allocation and phase dispatch. Pointers are borrowed
// for the call, properly aligned native storage; no C pointer escapes an owner.
unsafe extern "C" {
    fn t2r_ld_get(p: *const c_void, i: usize, s: *mut c_char) -> c_int;
    fn t2r_ld_set(p: *mut c_void, i: usize, s: *const c_char) -> c_int;
    fn t2r_ld_alloc(n: usize) -> *mut c_void;
    fn t2r_ld_free(p: *mut c_void);
    fn t2r_phase_call(p: *const c_void, mjd: *const c_void, freq: *const c_void, out: *mut c_void);
}
type Callback =
    unsafe extern "C" fn(*mut c_void, *mut c_void, c_int, c_int, *mut c_void, *mut c_void);

// Private adapter helpers may only be called after the exported shim established
// the documented C extent/lifetime preconditions. Rust never dereferences the
// foreign scalar layout or aliases its storage as a Rust numeric type.
fn read(p: *const c_void, i: usize) -> Result<N, String> {
    if p.is_null() {
        return Err("null native scalar backing".into());
    }
    let mut text = [0 as c_char; 128];
    // C's snprintf writes at most 128 bytes including NUL; p[i] is live native
    // storage supplied by the wrapper, and conversion retains the entire value.
    if unsafe { t2r_ld_get(p, i, text.as_mut_ptr()) } != 0 {
        return Err("native scalar formatting failed".into());
    }
    // The initialized buffer above is NUL terminated by snprintf.
    N::parse_number(
        unsafe { CStr::from_ptr(text.as_ptr()) }
            .to_str()
            .map_err(|e| e.to_string())?,
    )
}
fn write(p: *mut c_void, i: usize, v: &N) -> Result<(), String> {
    if p.is_null() {
        return Err("null native output backing".into());
    }
    let text = if v.as_float().is_nan() {
        if v.as_float().is_sign_negative() {
            "-nan".into()
        } else {
            "nan".into()
        }
    } else {
        v.significant(25)
    };
    let text = CString::new(text).map_err(|e| e.to_string())?;
    // p[i] is an exclusive initialized or writable native scalar cell; C copies
    // the complete decimal value before text is dropped. No native value uses f64.
    if unsafe { t2r_ld_set(p, i, text.as_ptr()) } != 0 {
        return Err("native scalar parsing failed".into());
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
fn finish(f: impl FnOnce() -> Result<(), String>) -> c_int {
    match catch_unwind(AssertUnwindSafe(f)) {
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
        write(
            out,
            0,
            &polynomial(p, nx, ny)?.evaluate(&read(x, 0)?, &read(y, 0)?),
        )
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
        let a = values(v, 7)?;
        let poly = polynomial(p, nx, ny)?;
        let time = [&a[0], &a[1]];
        let band = [&a[2], &a[3]];
        write(
            out,
            0,
            &if frequency != 0 {
                cheby2d::model_frequency(&poly, time, band, &a[5], &a[6])
            } else {
                cheby2d::model_phase(&poly, time, band, &a[4], &a[5], &a[6])
            },
        )
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
