//! Byte-preserving fscanf/fgets cursor for the source PAR/TIM readers.
use crate::{PRECISION_BITS, TimingError, clock_text};
use rug::Float;
use std::path::{Path, PathBuf};

pub struct SourceText {
    pub path: PathBuf,
    pub bytes: Vec<u8>,
    pub position: usize,
}
impl SourceText {
    pub fn new(path: &Path, bytes: Vec<u8>) -> Self {
        Self {
            path: path.into(),
            bytes,
            position: 0,
        }
    }
    pub fn line(&self) -> usize {
        self.bytes[..self.position]
            .iter()
            .filter(|&&b| b == b'\n')
            .count()
            + 1
    }
    pub fn error(&self, field: &str, message: impl Into<String>) -> TimingError {
        TimingError::new(&self.path, Some(self.line()), field, message.into())
    }
    fn skip_space(&mut self) {
        while self.position < self.bytes.len() && clock_text::whitespace(self.bytes[self.position])
        {
            self.position += 1;
        }
    }
    pub fn token(&mut self) -> Option<Vec<u8>> {
        self.skip_space();
        let start = self.position;
        while self.position < self.bytes.len() && !clock_text::whitespace(self.bytes[self.position])
        {
            self.position += 1;
        }
        (start != self.position).then(|| self.bytes[start..self.position].to_vec())
    }
    pub fn required_token(&mut self, field: &str) -> Result<Vec<u8>, TimingError> {
        self.token()
            .ok_or_else(|| self.error(field, "missing source token"))
    }
    /// fgets: includes a newline if reached and consumes at most size-1 bytes.
    pub fn line_chunk(&mut self, size: usize) -> Option<Vec<u8>> {
        if self.position == self.bytes.len() {
            return None;
        }
        let start = self.position;
        let stop = (start + size.saturating_sub(1)).min(self.bytes.len());
        while self.position < stop {
            let b = self.bytes[self.position];
            self.position += 1;
            if b == b'\n' {
                break;
            }
        }
        Some(self.bytes[start..self.position].to_vec())
    }
    pub fn integer(&mut self, field: &str) -> Result<i32, TimingError> {
        self.scan_integer()?
            .ok_or_else(|| self.error(field, "integer conversion failed"))
    }
    /// A failed sscanf conversion leaves its destination and the token untouched.
    pub fn scan_integer(&mut self) -> Result<Option<i32>, TimingError> {
        self.skip_space();
        let Some((v, n)) = clock_text::integer_prefix(&self.bytes[self.position..])? else {
            return Ok(None);
        };
        self.position += n;
        Ok(Some(v))
    }
    pub fn double(&mut self, field: &str) -> Result<f64, TimingError> {
        self.scan_double()
            .ok_or_else(|| self.error(field, "binary64 conversion failed"))
    }
    pub fn scan_double(&mut self) -> Option<f64> {
        self.skip_space();
        let (v, n) = clock_text::number_prefix(&self.bytes[self.position..])?;
        self.position += n;
        Some(v)
    }
    pub fn scan(&mut self) -> SourceScan<'_> {
        SourceScan {
            source: self,
            matching: true,
        }
    }
}

/// One fscanf conversion list: preserve assigned prefixes and stop after the
/// first failed conversion, retaining each destination that was not assigned.
pub struct SourceScan<'a> {
    source: &'a mut SourceText,
    matching: bool,
}
impl SourceScan<'_> {
    pub fn double(&mut self, out: &mut Option<f64>) {
        if self.matching {
            if let Some(v) = self.source.scan_double() {
                *out = Some(v);
            } else {
                self.matching = false;
            }
        }
    }
    pub fn float(&mut self, out: &mut Option<f32>) {
        if self.matching {
            self.source.skip_space();
            if let Some((v, n)) =
                clock_text::float_prefix(&self.source.bytes[self.source.position..])
            {
                self.source.position += n;
                *out = Some(v);
            } else {
                self.matching = false;
            }
        }
    }
    pub fn integer(&mut self, out: &mut Option<i32>) -> Result<(), TimingError> {
        if self.matching {
            if let Some(v) = self.source.scan_integer()? {
                *out = Some(v);
            } else {
                self.matching = false;
            }
        }
        Ok(())
    }
    pub fn string(&mut self, out: &mut [Option<u8>], field: &str) -> Result<(), TimingError> {
        if self.matching {
            if let Some(v) = self.source.token() {
                crate::observation::write_string(out, &v, field)?;
            } else {
                self.matching = false;
            }
        }
        Ok(())
    }
}

/// Source strtoflt128 prefix grammar, evaluated directly at the approved Rug128.
/// Failed conversion returns zero, as strtoflt128 does; no binary64 value is used.
pub fn extended(bytes: &[u8]) -> Result<Float, TimingError> {
    let count = clock_text::extended_prefix_len(bytes);
    if count == 0 {
        return Ok(Float::with_val(PRECISION_BITS, 0));
    }
    let token = std::str::from_utf8(&bytes[..count])
        .map_err(|_| crate::clock_diagnostics::boundary("longdouble_encoding"))?
        .trim();
    let lower = token.to_ascii_lowercase();
    if lower.contains("nan") {
        let nan = Float::with_val(PRECISION_BITS, rug::float::Special::Nan);
        return Ok(if lower.starts_with('-') { -nan } else { nan });
    }
    if lower.contains("inf") {
        return Ok(Float::with_val(
            PRECISION_BITS,
            if lower.starts_with('-') {
                rug::float::Special::NegInfinity
            } else {
                rug::float::Special::Infinity
            },
        ));
    }
    let unsigned = lower.trim_start_matches(['+', '-']);
    let mut value = if let Some(hex) = unsigned.strip_prefix("0x") {
        let (mantissa, exponent) = hex.split_once('p').unwrap_or((hex, "0"));
        let exponent: i32 = exponent
            .parse()
            .map_err(|_| crate::clock_diagnostics::boundary("hex_exponent_range"))?;
        let parsed = Float::parse_radix(mantissa, 16)
            .map_err(|_| crate::clock_diagnostics::boundary("hex_mantissa"))?;
        let mut value = Float::with_val(PRECISION_BITS, parsed);
        value <<= exponent;
        if lower.starts_with('-') {
            value = -value;
        }
        value
    } else {
        let parsed = Float::parse(token)
            .map_err(|_| crate::clock_diagnostics::boundary("longdouble_conversion"))?;
        Float::with_val(PRECISION_BITS, parsed)
    };
    // TKlongdouble.float128.h::parse_longdouble uses strtoflt128. Its input
    // range includes signed zero, subnormals and overflow, unlike MPFR's much
    // wider exponent range. Apply that input-format boundary at Rug128; ordinary
    // finite inputs and all subsequent arithmetic retain the approved precision.
    let magnitude = value.clone().abs();
    let overflow = (Float::with_val(PRECISION_BITS, 1) << 16384)
        - (Float::with_val(PRECISION_BITS, 1) << 16270);
    if magnitude >= overflow {
        value = Float::with_val(
            PRECISION_BITS,
            if value.is_sign_negative() {
                rug::float::Special::NegInfinity
            } else {
                rug::float::Special::Infinity
            },
        );
    } else if magnitude < (Float::with_val(PRECISION_BITS, 1) >> 16382) {
        // Binary128 subnormal cells have the fixed quantum 2^-16494.
        value <<= 16494;
        value.round_even_mut();
        value >>= 16494;
    }
    Ok(value)
}

/// Fixed decimal rounding of the actual Rug value, with no binary64 narrowing.
pub fn fixed(value: &Float, places: u32) -> Vec<u8> {
    use rug::{Integer, ops::Pow};
    let Some((mut n, e)) = value.to_integer_exp() else {
        return if value.is_nan() {
            if value.is_sign_negative() {
                b"-nan".to_vec()
            } else {
                b"nan".to_vec()
            }
        } else if value.is_sign_negative() {
            b"-inf".to_vec()
        } else {
            b"inf".to_vec()
        };
    };
    let sign = value.is_sign_negative();
    n.abs_mut();
    n *= Integer::from(10).pow(places);
    if e >= 0 {
        n <<= e;
    } else {
        let denominator = Integer::from(1) << (-e);
        let (mut q, r) = n.div_rem(denominator.clone());
        let twice = r * 2;
        if twice > denominator || twice == denominator && q.is_odd() {
            q += 1;
        }
        n = q;
    }
    let mut digits = n.to_string();
    if places > 0 {
        while digits.len() <= places as usize {
            digits.insert(0, '0');
        }
        digits.insert(digits.len() - places as usize, '.');
    }
    if sign {
        digits.insert(0, '-');
    }
    digits.into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_cursor_keeps_numeric_suffix_and_line_remainder() {
        let mut s = SourceText::new(Path::new("scan"), b" A\n 12tail 1.25x\nnext".to_vec());
        assert_eq!(s.token().unwrap(), b"A");
        assert_eq!(s.integer("i").unwrap(), 12);
        assert_eq!(s.token().unwrap(), b"tail");
        assert_eq!(s.double("d").unwrap(), 1.25);
        assert_eq!(s.line_chunk(100).unwrap(), b"x\n");
        assert_eq!(s.token().unwrap(), b"next");
        assert!(s.token().is_none());
        assert_eq!(
            extended(b"1.234567890123456789e-3suffix")
                .unwrap()
                .to_string_radix(10, Some(19)),
            "1.234567890123456789e-3"
        );
    }
}
