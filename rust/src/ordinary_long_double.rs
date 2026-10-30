//! Host ordinary C long double rounding, distinct from the Rug128 timing role.
use rug::{
    Float,
    float::{Round, Special},
    ops::AssignRound,
};
use std::cmp::Ordering;

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

// macOS keeps the established native double libm path. Linux ordinary exp/log
// use 64-bit MPFR results; AL separately measures their correspondence to libm.
pub(crate) fn exp(value: &Float) -> Float {
    if NATIVE_BITS == 53 {
        round(value.to_f64().exp())
    } else {
        round(value.exp_ref())
    }
}
pub(crate) fn ln(value: &Float) -> Float {
    if NATIVE_BITS == 53 {
        round(value.to_f64().ln())
    } else {
        round(value.ln_ref())
    }
}
