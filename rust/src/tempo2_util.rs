//! Numeric utilities corresponding to the supported tempo2Util.C routines.
use crate::longdouble::normalized_decimal;
use rug::Float;

pub(crate) fn dot_product(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub(crate) fn parse_binary64(token: &str) -> Result<f64, String> {
    let normalized = normalized_decimal(token)?;
    let value: f64 = normalized.parse().map_err(|e| format!("{token:?}: {e}"))?;
    let nonzero = normalized
        .split('e')
        .next()
        .unwrap()
        .bytes()
        .any(|c| matches!(c, b'1'..=b'9'));
    if value.is_finite() && (value != 0.0 || !nonzero) {
        Ok(value)
    } else {
        Err(format!(
            "{token:?} is outside the finite binary64 range (including underflow)"
        ))
    }
}

// The reader validates sexagesimal components before these source-order conversions.
pub(crate) fn hms_turn(hr: f64, min: f64, sec: f64) -> f64 {
    hr / 24.0 + min / 1440.0 + sec / 86400.0
}

pub(crate) fn dms_turn(deg: f64, min: f64, sec: f64, sign: f64) -> f64 {
    (deg.abs() / 360.0 + min / 21600.0 + sec / 1296000.0) * sign
}

pub(crate) fn turn_deg(turn: f64) -> f64 {
    turn * 360.0
}

pub(crate) fn fortran_mod(a: Float, p: f64) -> Float {
    // Rug % retains the source's signed, truncating-quotient remainder semantics.
    a % p
}

pub(crate) fn fortran_nlong(x: &Float) -> Float {
    // fortran_nlong: signed half addition then truncation, not ties-to-even.
    (x.clone() + if x > &0 { 0.5f64 } else { -0.5f64 }).trunc()
}
