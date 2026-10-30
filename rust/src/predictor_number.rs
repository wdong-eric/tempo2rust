//! tempo2pred.h ordinary C `long double`, distinct from TKlongdouble's timing type.
//! Owned arithmetic/format glue, not a native ABI layout. AJ/AL qualify the ABI.
use rug::{Float, float::Round};
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
    pub fn from_float(x: &Float) -> Self {
        let (v, direction) = Float::with_val_round(NATIVE_BITS, x, Round::Nearest);
        Self::rounded(v, direction)
    }
    fn rounded(v: Float, direction: Ordering) -> Self {
        Self(crate::ordinary_long_double::rounded(v, direction))
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
        if NATIVE_BITS == 53 {
            n(self.to_f64().mul_add(b.to_f64(), c.to_f64()))
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
        Self::from_float(&Float::with_val(NATIVE_BITS, x))
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
    ($trait:ident,$method:ident,$op:tt) => {
        impl ops::$trait<&N> for &N { type Output=N; fn $method(self,rhs:&N)->N { { let (v, direction) = Float::with_val_round(NATIVE_BITS, &self.0 $op &rhs.0, Round::Nearest); N::rounded(v, direction) } } }
        impl ops::$trait<N> for N { type Output=N; fn $method(self,rhs:N)->N { &self $op &rhs } }
        impl ops::$trait<&N> for N { type Output=N; fn $method(self,rhs:&N)->N { &self $op rhs } }
        impl ops::$trait<N> for &N { type Output=N; fn $method(self,rhs:N)->N { self $op &rhs } }
        impl ops::$trait<f64> for N { type Output=N; fn $method(self,rhs:f64)->N { &self $op &n(rhs) } }
        impl ops::$trait<f64> for &N { type Output=N; fn $method(self,rhs:f64)->N { self $op &n(rhs) } }
    }
}
arithmetic!(Add,add,+);
arithmetic!(Sub,sub,-);
arithmetic!(Mul,mul,*);
arithmetic!(Div,div,/);
impl ops::Neg for N {
    type Output = N;
    fn neg(self) -> N {
        Self(-self.0)
    }
}
