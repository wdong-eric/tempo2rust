//! Host ordinary C long double rounding, distinct from the Rug128 timing role.
use rug::{
    Float,
    float::{Round, Special},
    ops::AssignRound,
};
use std::cmp::Ordering;
use std::ffi::{CStr, CString, c_char, c_int};

// Rust has no portable C long-double ABI. These private functions borrow only
// terminated text and a 128-byte output; C owns each native scalar synchronously.
// Native exp(long double)/expl and log(long double) are expl/logl (0117).
unsafe extern "C" {
    fn t2r_ld_exp(input: *const c_char, output: *mut c_char) -> c_int;
    fn t2r_ld_log(input: *const c_char, output: *mut c_char) -> c_int;
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub const NATIVE_BITS: u32 = 53;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub const NATIVE_BITS: u32 = 64;
#[cfg(not(any(
    all(target_os = "macos", target_arch = "aarch64"),
    all(target_os = "linux", target_arch = "x86_64")
)))]
compile_error!("Ordinary-long-double profile requires qualification on this target");

pub(crate) fn round<T>(value: T) -> Float
where
    Float: AssignRound<T, Round = Round, Ordering = Ordering>,
{
    let (v, direction) = Float::with_val_round(NATIVE_BITS, value, Round::Nearest);
    rounded(v, direction)
}

pub(crate) fn rounded(mut v: Float, direction: Ordering) -> Float {
    let (normal_min, maximum) = if NATIVE_BITS == 53 {
        (-1021, 1024)
    } else {
        (-16381, 16384)
    };
    // Preserve the first rounding direction: a p-bit intermediate exactly
    // at a subnormal midpoint must not erase which side the exact value lay.
    v.subnormalize_round(normal_min, direction, Round::Nearest);
    if v.get_exp().is_some_and(|exponent| exponent > maximum) {
        v = Float::with_val(
            NATIVE_BITS,
            if v.is_sign_negative() {
                Special::NegInfinity
            } else {
                Special::Infinity
            },
        );
    }
    v
}

// Preserve macOS hardware arithmetic, including its 0/0 NaN sign. Linux rounds
// the unmaterialized MPFR operation once to the source ordinary type.
pub(crate) fn mul(a: &Float, b: &Float) -> Float {
    if NATIVE_BITS == 53 {
        round(a.to_f64() * b.to_f64())
    } else {
        round(a * b)
    }
}
pub(crate) fn div(a: &Float, b: &Float) -> Float {
    if NATIVE_BITS == 53 {
        round(a.to_f64() / b.to_f64())
    } else {
        round(a / b)
    }
}

// MPFR's get_d canonicalizes NaN sign. Source return conversion retains it;
// finite rounding, signed zero and infinities still use Rug's conversion.
pub(crate) fn to_f64(value: &Float) -> f64 {
    let result = value.to_f64();
    if value.is_nan() {
        result.copysign(if value.is_sign_negative() { -1.0 } else { 1.0 })
    } else {
        result
    }
}

fn native_math(
    value: &Float,
    operation: unsafe extern "C" fn(*const c_char, *mut c_char) -> c_int,
) -> Float {
    // 25 significant digits exceed both qualified LDBL_DECIMAL_DIG values.
    // Native output is exact %La, parsed by the existing predictor transport.
    let input = CString::new(value.to_string_radix(10, Some(25))).unwrap();
    let mut output = [0 as c_char; 128];
    // Both buffers are live for the call. C copies/parses the complete input
    // and writes at most 128 bytes, including the terminating NUL, on success.
    assert_eq!(
        unsafe { operation(input.as_ptr(), output.as_mut_ptr()) },
        0,
        "ordinary native math transport failed"
    );
    let text = unsafe { CStr::from_ptr(output.as_ptr()) }.to_str().unwrap();
    let parsed = crate::predictor_number::N::parse_number(text).unwrap();
    let mut result = round(parsed.as_float());
    // MPFR assignment may discard a NaN sign; retain the sign just emitted by C.
    if parsed.as_float().is_nan() {
        result.copysign_mut(parsed.as_float());
    }
    result
}

pub(crate) fn exp(value: &Float) -> Float {
    native_math(value, t2r_ld_exp)
}
pub(crate) fn ln(value: &Float) -> Float {
    native_math(value, t2r_ld_log)
}
