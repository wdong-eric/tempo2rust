//! Corresponds to TEMPO2 longdouble.h; uses the approved fixed MPFR precision.
use rug::Float;
use serde::{Serialize, Serializer};
use std::{fmt, str::FromStr};

pub const PRECISION_BITS: u32 = 128;

/// A finite MPFR scalar constructed at the fixed M1a precision.
/// Serialized as round-trip decimal text, never through binary64.
#[derive(Clone, Debug, PartialEq)]
pub struct Scalar(pub(crate) Float);

impl Scalar {
    pub fn as_float(&self) -> &Float {
        &self.0
    }
}

pub(crate) fn normalized_decimal(token: &str) -> Result<String, String> {
    let normalized = token.replace(['D', 'd', 'E'], "e");
    let unsigned = normalized.strip_prefix(['+', '-']).unwrap_or(&normalized);
    let mut parts = unsigned.split('e');
    let mantissa = parts.next().unwrap_or("");
    let exponent = parts.next();
    let digits = mantissa.bytes().filter(u8::is_ascii_digit).count();
    let valid_mantissa = digits > 0
        && mantissa.bytes().all(|c| c.is_ascii_digit() || c == b'.')
        && mantissa.bytes().filter(|&c| c == b'.').count() <= 1;
    let valid_exponent = exponent.is_none_or(|value| {
        let value = value.strip_prefix(['+', '-']).unwrap_or(value);
        !value.is_empty() && value.bytes().all(|c| c.is_ascii_digit())
    });
    if valid_mantissa && valid_exponent && parts.next().is_none() {
        Ok(normalized)
    } else {
        Err(format!(
            "expected a complete finite decimal token, got {token:?}"
        ))
    }
}

/// Source-named decimal parser; the representation remains fixed Rug128, not a C ABI type.
pub(crate) fn parse_longdouble(token: &str) -> Result<Scalar, String> {
    let normalized = normalized_decimal(token)?;
    let parsed = Float::parse(&normalized).map_err(|e| format!("{token:?}: {e}"))?;
    let value = Float::with_val(PRECISION_BITS, parsed);
    let nonzero = normalized
        .split('e')
        .next()
        .unwrap()
        .bytes()
        .any(|c| matches!(c, b'1'..=b'9'));
    if value.is_finite() && (value != 0 || !nonzero) {
        Ok(Scalar(value))
    } else {
        Err(format!(
            "{token:?} is outside the finite MPFR range (including underflow)"
        ))
    }
}

impl FromStr for Scalar {
    type Err = String;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        parse_longdouble(token)
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0.to_string_radix(10, None))
    }
}

impl Serialize for Scalar {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
