//! tempo2pred.h ordinary C `long double`, distinct from TKlongdouble's timing type.
//! Owned arithmetic/format glue, not a native ABI layout. AJ/AL qualify the ABI.
use rug::{
    Float,
    float::Round,
    ops::{
        AddAssignRound, AddFromRound, AssignRound, DivAssignRound, DivFromRound, MulAssignRound,
        MulFromRound, SubAssignRound, SubFromRound,
    },
};
use serde::{Serialize, Serializer};
use std::cmp::Ordering;
use std::{fmt, ops, str::FromStr};

pub use crate::ordinary_long_double::NATIVE_BITS;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PredictorNumber(Float);
pub(crate) type N = PredictorNumber;
pub(crate) fn n(x: f64) -> N {
    N::from(x)
}
impl PredictorNumber {
    /// Exact finite native scalar transport. The C shim supplies at most
    /// NATIVE_BITS significand bits and an already representable exponent.
    pub(crate) fn from_native_parts(significand: u64, exponent: i32) -> Self {
        let mut value = Float::with_val(NATIVE_BITS, significand);
        value <<= exponent;
        Self(value)
    }

    pub fn from_float(x: &Float) -> Self {
        let (v, direction) = Float::with_val_round(NATIVE_BITS, x, Round::Nearest);
        Self::rounded(v, direction)
    }
    fn rounded(v: Float, direction: Ordering) -> Self {
        Self(crate::ordinary_long_double::rounded(v, direction))
    }
    fn rounded_reused(mut v: Float, direction: Ordering, clear_nan_sign: bool) -> Self {
        // MPFR can leave the old destination sign on NaN; fresh results started
        // positive. Division instead sets a sign for 0/0 and inf/inf, so its
        // caller clears only input-NaN propagation. Keep the original first
        // rounding direction for the shared subnormal/range boundary.
        if clear_nan_sign && v.is_nan() {
            v.abs_mut();
        }
        Self::rounded(v, direction)
    }
    pub fn as_float(&self) -> &Float {
        &self.0
    }
    pub fn to_f64(&self) -> f64 {
        self.0.to_f64()
    }
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }
    pub(crate) fn quad_cos(&self) -> Self {
        // cheby2d.c includes TKlongdouble.float128.h: cosl is cosq. The input
        // expression is native long double; the quad result narrows on assignment.
        Self::from_float(&Float::with_val(113, &self.0).cos())
    }
    /// Source GCC16 arm64 contracts these explicit sites; x87 long double
    /// has no equivalent native FMA. AF preserves the measured macOS boundary;
    /// the Linux compilation remains subject to AJ/AL native qualification.
    pub(crate) fn source_mul_add(&self, b: &Self, c: &Self) -> Self {
        self.clone().source_mul_add_owned(b, c)
    }
    pub(crate) fn source_mul_add_owned(mut self, b: &Self, c: &Self) -> Self {
        if NATIVE_BITS == 53 {
            let value = self.to_f64().mul_add(b.to_f64(), c.to_f64());
            let direction = self.0.assign_round(value, Round::Nearest);
            Self::rounded_reused(self.0, direction, true)
        } else {
            self * b + c
        }
    }
    pub(crate) fn sqrt(&self) -> Self {
        let (v, direction) = Float::with_val_round(NATIVE_BITS, self.0.sqrt_ref(), Round::Nearest);
        Self::rounded(v, direction)
    }
    pub(crate) fn abs_double(&self) -> Self {
        n(self.to_f64().abs())
    }
    pub(crate) fn ceil_count(&self) -> Result<usize, String> {
        self.0
            .clone()
            .ceil()
            .to_integer()
            .and_then(|i| i.to_usize())
            .filter(|&i| i <= i32::MAX as usize)
            .ok_or_else(|| "predictor count outside source int range".into())
    }
    pub fn fixed(&self, places: u32) -> String {
        String::from_utf8(crate::source_text::fixed(&self.0, places)).expect("numeric ASCII")
    }
    pub fn significant(&self, digits: usize) -> String {
        self.0.to_string_radix(10, Some(digits))
    }
    pub(crate) fn parse_prefix(token: &[u8]) -> Result<Self, String> {
        let len = crate::clock_text::extended_prefix_len(token);
        if len == 0 {
            return Err("missing predictor numeric conversion".into());
        }
        Self::parse_number(std::str::from_utf8(&token[..len]).map_err(|e| e.to_string())?)
    }
    /// Complete numeric token. The C ABI's private C-locale representation
    /// bypasses the public source scanner, whose prefix grammar uses libc locale.
    pub(crate) fn parse_number(s: &str) -> Result<Self, String> {
        let s = s.trim();
        let lower = s.to_ascii_lowercase();
        if let Some(hex) = lower.trim_start_matches(['+', '-']).strip_prefix("0x") {
            let (mantissa, exponent) = hex.split_once('p').unwrap_or((hex, "0"));
            let exponent: i32 = exponent
                .parse()
                .map_err(|_| "hex predictor exponent range")?;
            let parsed = Float::parse_radix(mantissa, 16).map_err(|e| e.to_string())?;
            let (mut v, mut direction) = Float::with_val_round(NATIVE_BITS, parsed, Round::Nearest);
            v <<= exponent;
            if lower.starts_with('-') {
                v = -v;
                direction = direction.reverse();
            }
            return Ok(Self::rounded(v, direction));
        }
        let parsed = Float::parse(s).map_err(|e| e.to_string())?;
        let (v, direction) = Float::with_val_round(NATIVE_BITS, parsed, Round::Nearest);
        Ok(Self::rounded(v, direction))
    }
}
impl From<f64> for PredictorNumber {
    fn from(x: f64) -> Self {
        // f64 already fits either ordinary significand/range. Avoid allocating
        // a temporary Float merely to copy it at the same precision.
        let (value, direction) = Float::with_val_round(NATIVE_BITS, x, Round::Nearest);
        Self::rounded(value, direction)
    }
}
impl FromStr for PredictorNumber {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let s = crate::longdouble::normalized_decimal(s)?;
        let v = Float::parse(&s).map_err(|e| e.to_string())?;
        let (v, direction) = Float::with_val_round(NATIVE_BITS, v, Round::Nearest);
        let v = Self::rounded(v, direction);
        if v.is_finite() {
            Ok(v)
        } else {
            Err("nonfinite predictor input".into())
        }
    }
}
impl fmt::Display for PredictorNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0.to_string_radix(10, None))
    }
}
impl Serialize for PredictorNumber {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        crate::FitNumber(self.0.clone()).serialize(s)
    }
}
macro_rules! arithmetic {
    ($trait:ident,$method:ident,$assign:ident,$from:ident,$clear_invalid_nan:literal,$op:tt) => {
        impl ops::$trait<&N> for &N { type Output=N; fn $method(self,rhs:&N)->N { { let (v, direction) = Float::with_val_round(NATIVE_BITS, &self.0 $op &rhs.0, Round::Nearest); N::rounded(v, direction) } } }
        // cheby2d.c/t1polyco.c ordinary temporaries: reuse consumed owners,
        // keeping the same ordered MPFR operation and range-rounding boundary.
        impl ops::$trait<N> for N { type Output=N; fn $method(self,rhs:N)->N { self $op &rhs } }
        impl ops::$trait<&N> for N { type Output=N; fn $method(mut self,rhs:&N)->N { let clear_nan = $clear_invalid_nan || self.0.is_nan() || rhs.0.is_nan(); let direction = self.0.$assign(&rhs.0, Round::Nearest); N::rounded_reused(self.0, direction, clear_nan) } }
        impl ops::$trait<N> for &N { type Output=N; fn $method(self,mut rhs:N)->N { let clear_nan = $clear_invalid_nan || self.0.is_nan() || rhs.0.is_nan(); let direction = rhs.0.$from(&self.0, Round::Nearest); N::rounded_reused(rhs.0, direction, clear_nan) } }
        impl ops::$trait<f64> for N { type Output=N; fn $method(mut self,rhs:f64)->N { let clear_nan = $clear_invalid_nan || self.0.is_nan() || rhs.is_nan(); let direction = self.0.$assign(rhs, Round::Nearest); N::rounded_reused(self.0, direction, clear_nan) } }
        impl ops::$trait<f64> for &N { type Output=N; fn $method(self,rhs:f64)->N { let (v, direction) = Float::with_val_round(NATIVE_BITS, &self.0 $op rhs, Round::Nearest); N::rounded(v, direction) } }
    }
}
arithmetic!(Add,add,add_assign_round,add_from_round,true,+);
arithmetic!(Sub,sub,sub_assign_round,sub_from_round,true,-);
arithmetic!(Mul,mul,mul_assign_round,mul_from_round,true,*);
arithmetic!(Div,div,div_assign_round,div_from_round,false,/);
impl ops::Neg for N {
    type Output = N;
    fn neg(self) -> N {
        Self(-self.0)
    }
}
