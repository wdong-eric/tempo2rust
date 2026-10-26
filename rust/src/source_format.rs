//! TKlongdouble.C typed formatting, with the selected float128 formatter's
//! zero-success return convention. C varargs are replaced by checked values.
use crate::{TimingError, clock_diagnostics::boundary};
use rug::Float;
use std::io::Write;

#[derive(Clone, Debug)]
pub enum Argument {
    Extended(Float),
    Double(f64),
    Signed(i64),
    Unsigned(u64),
    Bytes(Vec<u8>),
}
impl From<Float> for Argument {
    fn from(x: Float) -> Self {
        Self::Extended(x)
    }
}
impl From<f64> for Argument {
    fn from(x: f64) -> Self {
        Self::Double(x)
    }
}
impl From<f32> for Argument {
    fn from(x: f32) -> Self {
        Self::Double(x as f64)
    }
}
impl From<i32> for Argument {
    fn from(x: i32) -> Self {
        Self::Signed(x as i64)
    }
}
impl From<i64> for Argument {
    fn from(x: i64) -> Self {
        Self::Signed(x)
    }
}
impl From<u32> for Argument {
    fn from(x: u32) -> Self {
        Self::Unsigned(x as u64)
    }
}
impl From<u64> for Argument {
    fn from(x: u64) -> Self {
        Self::Unsigned(x)
    }
}
impl From<usize> for Argument {
    fn from(x: usize) -> Self {
        Self::Unsigned(x as u64)
    }
}
impl From<Vec<u8>> for Argument {
    fn from(x: Vec<u8>) -> Self {
        Self::Bytes(x)
    }
}
impl From<&[u8]> for Argument {
    fn from(x: &[u8]) -> Self {
        Self::Bytes(x.to_vec())
    }
}
impl From<&str> for Argument {
    fn from(x: &str) -> Self {
        Self::Bytes(x.as_bytes().to_vec())
    }
}

/// Format extended fixed/scientific/general decimals at the actual stored precision.
fn decimal(v: &Float, conversion: u8, precision: usize, alternate: bool) -> Vec<u8> {
    if !v.is_finite() {
        return crate::source_text::fixed(v, 0);
    }
    if conversion == b'f' {
        let mut s = crate::source_text::fixed(v, precision as u32);
        if precision == 0 && alternate {
            s.push(b'.');
        }
        return s;
    }
    let significant = if conversion == b'e' {
        precision + 1
    } else {
        precision.max(1)
    };
    let (negative, mut digits, exp) = v.to_sign_string_exp(10, Some(significant));
    let exp = exp.unwrap_or(1); // printf's exponent for zero is zero.
    if v.is_zero() {
        digits = "0".repeat(significant);
    }
    let scientific = conversion == b'e' || exp < -3 || exp > significant as i32;
    if conversion == b'g' && !alternate {
        while digits.len() > 1 && digits.ends_with('0') {
            digits.pop();
        }
    }
    let mut text = if negative {
        "-".to_string()
    } else {
        String::new()
    };
    if scientific {
        text.push_str(&digits[..1]);
        if digits.len() > 1 || alternate {
            text.push('.');
            text.push_str(&digits[1..]);
        }
        text.push_str(&format!("e{:+03}", exp - 1));
    } else if exp <= 0 {
        text.push_str("0.");
        text.push_str(&"0".repeat((-exp) as usize));
        text.push_str(&digits);
    } else {
        let whole = exp as usize;
        if whole >= digits.len() {
            text.push_str(&digits);
            text.push_str(&"0".repeat(whole - digits.len()));
            if alternate {
                text.push('.');
            }
        } else {
            text.push_str(&digits[..whole]);
            text.push('.');
            text.push_str(&digits[whole..]);
        }
    }
    text.into_bytes()
}

/// Defined source printf conversions with typed arguments; byte strings retain paths.
/// fda6ad4 TKlongdouble.C:86-190 has unmatched va_end and fixed buffers. Checked
/// parsing and owned output remove those undefined operations, retaining all
/// defined conversion values and the original successful return value (zero).
pub fn format(fmt: &str, args: &[Argument]) -> Result<Vec<u8>, TimingError> {
    let bytes = fmt.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    let mut a = 0;
    while i < bytes.len() {
        if bytes[i] != b'%' {
            out.push(bytes[i]);
            i += 1;
            continue;
        }
        i += 1;
        if bytes.get(i) == Some(&b'%') {
            out.push(b'%');
            i += 1;
            continue;
        }
        let start = i - 1;
        let mut flags = Vec::new();
        while bytes.get(i).is_some_and(|b| b"-+ #0".contains(b)) {
            flags.push(bytes[i]);
            i += 1;
        }
        let w = i;
        while bytes.get(i).is_some_and(u8::is_ascii_digit) {
            i += 1;
        }
        let width = if i > w {
            fmt[w..i]
                .parse::<usize>()
                .map_err(|_| boundary("format_width"))?
        } else {
            0
        };
        let mut precision = None;
        if bytes.get(i) == Some(&b'.') {
            i += 1;
            let p = i;
            while bytes.get(i).is_some_and(u8::is_ascii_digit) {
                i += 1;
            }
            precision = Some(if i > p {
                fmt[p..i]
                    .parse::<usize>()
                    .map_err(|_| boundary("format_precision"))?
            } else {
                0
            });
        }
        let l = i;
        while bytes.get(i).is_some_and(|b| b"lLh".contains(b)) {
            i += 1;
        }
        let length = &fmt[l..i];
        let c = *bytes.get(i).ok_or_else(|| boundary("format_conversion"))?;
        i += 1;
        let arg = args
            .get(a)
            .ok_or_else(|| boundary("format_missing_argument"))?;
        a += 1;
        let plus = flags.contains(&b'+');
        let space = flags.contains(&b' ');
        let alt = flags.contains(&b'#');
        let mut s = match (c, arg) {
            (b'f' | b'e' | b'g', Argument::Extended(v)) if length == "L" => {
                decimal(v, c, precision.unwrap_or(6), alt)
            }
            (b'f' | b'e' | b'g' | b'F' | b'E' | b'G', Argument::Double(v))
                if length.is_empty() || length == "l" =>
            {
                // Use the existing typed libc boundary for binary64 spelling.
                out.extend(crate::clock_text::format_double(&fmt[start..i], *v)?);
                continue;
            }
            (b's', Argument::Bytes(v)) if length.is_empty() => {
                let b = crate::clock_text::c_bytes(v);
                b[..precision.unwrap_or(b.len()).min(b.len())].to_vec()
            }
            (b'c', Argument::Signed(v)) if length.is_empty() => vec![*v as u8],
            (b'd' | b'i', Argument::Signed(v)) if ["", "h", "hh", "l", "ll"].contains(&length) => {
                let v = match length {
                    "hh" => *v as i8 as i64,
                    "h" => *v as i16 as i64,
                    "" => *v as i32 as i64,
                    _ => *v,
                };
                v.to_string().into_bytes()
            }
            (b'u' | b'x' | b'X' | b'o', Argument::Unsigned(v))
                if ["", "h", "hh", "l", "ll"].contains(&length) =>
            {
                let v = match length {
                    "hh" => *v as u8 as u64,
                    "h" => *v as u16 as u64,
                    "" => *v as u32 as u64,
                    _ => *v,
                };
                match c {
                    b'x' => format!("{v:x}"),
                    b'X' => format!("{v:X}"),
                    b'o' => format!("{v:o}"),
                    _ => v.to_string(),
                }
                .into_bytes()
            }
            _ => return Err(boundary("format_argument_type")),
        };
        let numeric = c != b's' && c != b'c';
        if matches!(c, b'd' | b'i' | b'f' | b'e' | b'g') && !s.starts_with(b"-") {
            if plus {
                s.insert(0, b'+');
            } else if space {
                s.insert(0, b' ');
            }
        }
        if matches!(c, b'd' | b'i' | b'u' | b'x' | b'X' | b'o') {
            let sign = usize::from(s.first().is_some_and(|b| b"-+ ".contains(b)));
            let zero = s[sign..] == *b"0";
            if precision == Some(0) && s[sign..] == *b"0" {
                s.truncate(sign);
            }
            if let Some(n) = precision {
                while s.len() - sign < n {
                    s.insert(sign, b'0');
                }
            }
            if alt {
                let prefix = match c {
                    b'x' if !zero => b"0x".as_slice(),
                    b'X' if !zero => b"0X",
                    b'o' if !s.starts_with(b"0") => b"0",
                    _ => b"",
                };
                s.splice(sign..sign, prefix.iter().copied());
            }
        }
        let n = width.saturating_sub(s.len());
        if flags.contains(&b'-') {
            s.extend(std::iter::repeat_n(b' ', n));
        } else if flags.contains(&b'0')
            && numeric
            && (!matches!(c, b'd' | b'i' | b'u' | b'x' | b'X' | b'o') || precision.is_none())
            && !s.windows(3).any(|x| x == b"nan" || x == b"inf")
        {
            let mut prefix = usize::from(s.first().is_some_and(|b| b"-+ ".contains(b)));
            if matches!(c, b'x' | b'X')
                && alt
                && s.get(prefix..prefix + 2)
                    .is_some_and(|p| p == b"0x" || p == b"0X")
            {
                prefix += 2;
            }
            s.splice(prefix..prefix, std::iter::repeat_n(b'0', n));
        } else {
            s.splice(0..0, std::iter::repeat_n(b' ', n));
        }
        out.extend(s);
    }
    Ok(out)
}
pub fn ld_vsprintf(out: &mut Vec<u8>, fmt: &str, args: &[Argument]) -> Result<i32, TimingError> {
    *out = format(fmt, args)?;
    Ok(0)
}
pub fn ld_sprintf(out: &mut Vec<u8>, fmt: &str, args: &[Argument]) -> Result<i32, TimingError> {
    ld_vsprintf(out, fmt, args)
}
pub fn ld_fprintf(out: &mut dyn Write, fmt: &str, args: &[Argument]) -> Result<i32, TimingError> {
    out.write_all(&format(fmt, args)?)
        .map_err(|e| TimingError::new("", None, "output", e.to_string()))?;
    Ok(0)
}
/// The source stdout variant, with its destination supplied by the caller.
pub fn ld_printf(stdout: &mut dyn Write, fmt: &str, args: &[Argument]) -> Result<i32, TimingError> {
    ld_fprintf(stdout, fmt, args)
}
pub fn print_longdouble(v: &Float) -> Vec<u8> {
    decimal(v, b'g', 6, false)
}
pub fn parse_longdouble(v: &[u8]) -> Result<Float, TimingError> {
    crate::source_text::extended(v)
}
