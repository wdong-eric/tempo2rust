//! t1polyco.c: both TEMPO1/TEMPO2 readers, writer, evaluation and nearest set.
use crate::predictor_number::{N, n};
use serde::Serialize;
use std::io::{BufRead, Write};
#[derive(Clone, Debug, Serialize)]
pub struct T1Polyco {
    pub psrname: String,
    pub date_string: String,
    pub utc_string: String,
    pub mjd_mid: N,
    pub dm: f64,
    pub doppler: f64,
    pub log10rms: f64,
    pub reference_phase: N,
    pub frequency_psr_0: N,
    pub sitename: String,
    pub span: i32,
    pub frequency_obs: f64,
    pub binary_phase: f64,
    pub binary_frequency: f64,
    coeff: Vec<N>,
}
impl T1Polyco {
    pub fn new(coeff: Vec<N>) -> Result<Self, String> {
        Self::empty().with_coefficients(coeff)
    }
    pub fn with_coefficients(mut self, coeff: Vec<N>) -> Result<Self, String> {
        if !(2..=32).contains(&coeff.len()) {
            return Err(
                "T1 requires 2..=32 coefficients: source coeff[1] and coeff[32] storage".into(),
            );
        }
        self.coeff = coeff;
        Ok(self)
    }
    pub fn coefficients(&self) -> &[N] {
        &self.coeff
    }
    pub fn phase(&self, mjd: &N, freq: &N) -> N {
        PhasePolynomial {
            mjd_mid: &self.mjd_mid,
            reference_phase: &self.reference_phase,
            frequency_psr_0: &self.frequency_psr_0,
            dm: self.dm,
            doppler: self.doppler,
            frequency_obs: self.frequency_obs,
            coeff: &self.coeff,
            count: self.coeff.len(),
        }
        .phase(mjd, freq)
    }
    pub fn frequency(&self, mjd: &N, _freq: &N) -> N {
        frequency_leaf(
            &self.mjd_mid,
            &self.frequency_psr_0,
            &self.coeff[1..],
            self.coeff.len(),
            mjd,
        )
    }
    pub fn write(&self, mut out: impl Write) -> Result<(), String> {
        let name = self.psrname.strip_prefix('J').unwrap_or(&self.psrname);
        let mut s = format!(
            "{:<10.10} {:>9.9}{:>11.11}{:>20}{:>21.6} {:>6.3}{:>7.3}\n{:>20}{:>18}{:>5}{:>5}{:>5}{:>10.3}{:>16.14}\n",
            name,
            self.date_string,
            self.utc_string,
            self.mjd_mid.fixed(11),
            self.dm,
            self.doppler,
            self.log10rms,
            self.reference_phase.fixed(6),
            self.frequency_psr_0.fixed(12),
            self.sitename,
            self.span,
            self.coeff.len(),
            self.frequency_obs,
            self.binary_phase
        );
        for (i, c) in self.coeff.iter().enumerate() {
            s.push_str(&format!("{:>25}", scientific(c.as_float(), 17)));
            if (i + 1) % 3 == 0 {
                s.push('\n');
            }
        }
        // fda6ad4 t1polyco.c:111 omits newline for ncoeff%3 != 0, joining the
        // next header to coefficients. polyco.C:431 supplies it; match that
        // valid set grammar so writing two non-multiple-of-three models works.
        if !self.coeff.len().is_multiple_of(3) {
            s.push('\n');
        }
        out.write_all(s.as_bytes()).map_err(|e| e.to_string())
    }
    pub fn read(input: &mut impl BufRead) -> Result<Self, String> {
        let a = read_line(input)?;
        if a.get(..6).is_some_and(|x| x.eq_ignore_ascii_case("TEMPO2")) {
            return Self::read_new(input);
        }
        let b = read_line(input)?;
        let int = |s: &str, start, len| -> Result<i32, String> {
            crate::clock_text::integer_prefix(field(s, start, len)?.as_bytes())
                .map_err(|e| e.to_string())?
                .map(|x| x.0)
                .ok_or("missing T1 integer".into())
        };
        let nc = int(&b, 48, 5)?;
        if !(2..=32).contains(&nc) {
            return Err("T1 coefficient count outside 2..=32".into());
        }
        let mut coeff = vec![];
        while coeff.len() < nc as usize {
            let c = read_line(input)?;
            for i in 0..3 {
                if coeff.len() < nc as usize {
                    coeff.push(grab(&c, i * 25, 25)?);
                }
            }
        }
        // Isolated polyco.C records deliberately leave binary metadata blank.
        // The original grab helper reads an uninitialized local on blank input.
        // Use the source new-format's established zero pair for absent metadata;
        // the original bytes retained by generic I/O keep this absence inspectable.
        let bin = field(&b, 63, 16).unwrap_or("");
        let (bp, bf) = if bin.trim().is_empty() {
            (0., 0.)
        } else {
            (grab(&b, 63, 7)?.to_f64(), grab(&b, 70, 9)?.to_f64())
        };
        // fda6ad4 polyco.C:419 writes Doppler with minimum width %6.3f;
        // t1polyco.c:193-194 assumes exactly six columns. Barycentric source
        // values (-9999.990) overflow that width and shift log10rms. Consume
        // the writer's signed decimal plus three digits before the next field.
        let doppler_width = a
            .get(73..)
            .and_then(|s| {
                let point = s.find('.')?;
                let prefix = &s[..point];
                (prefix
                    .trim_start()
                    .trim_start_matches(['+', '-'])
                    .bytes()
                    .all(|b| b.is_ascii_digit())
                    && s.as_bytes()
                        .get(point + 1..point + 4)?
                        .iter()
                        .all(u8::is_ascii_digit))
                .then_some((point + 4).max(6))
            })
            .unwrap_or(6);
        Ok(Self {
            psrname: field(&a, 0, 11)?.into(),
            date_string: field(&a, 11, 9)?.into(),
            utc_string: field(&a, 20, 11)?.into(),
            mjd_mid: grab(&a, 31, 20)?,
            dm: grab(&a, 51, 21)?.to_f64(),
            doppler: grab(&a, 73, doppler_width)?.to_f64(),
            log10rms: grab(&a, 73 + doppler_width, 7)?.to_f64(),
            reference_phase: grab(&b, 0, 20)?,
            frequency_psr_0: grab(&b, 21, 18)?,
            sitename: field(&b, 38, 5)?.into(),
            span: int(&b, 43, 5)?,
            frequency_obs: grab(&b, 53, 10)?.to_f64(),
            binary_phase: bp,
            binary_frequency: bf,
            coeff,
        })
    }
    pub fn read_new(input: &mut impl BufRead) -> Result<Self, String> {
        let mut tokens = vec![];
        while tokens.len() < 15 {
            tokens.extend(read_line(input)?.split_whitespace().map(str::to_owned));
        }
        let nc: i32 = tokens[11]
            .parse()
            .map_err(|_| "invalid new T1 coefficient count")?;
        if !(2..=32).contains(&nc) {
            return Err("T1 coefficient count outside 2..=32".into());
        }
        while tokens.len() < 15 + nc as usize {
            tokens.extend(read_line(input)?.split_whitespace().map(str::to_owned));
        }
        let num = |i: usize| N::parse_prefix(tokens[i].as_bytes());
        // New-format %lf fields parse directly to double; the fixed reader's
        // native-long-double intermediate is a distinct source boundary.
        let double = |i: usize| {
            crate::clock_text::number_prefix(tokens[i].as_bytes())
                .map(|v| v.0)
                .ok_or("invalid T1 double field")
        };
        Ok(Self {
            psrname: tokens[0].clone(),
            date_string: tokens[1].clone(),
            utc_string: tokens[2].clone(),
            mjd_mid: num(3)?,
            dm: double(4)?,
            doppler: double(5)?,
            log10rms: double(6)?,
            reference_phase: num(7)?,
            frequency_psr_0: num(8)?,
            sitename: tokens[9].clone(),
            span: tokens[10].parse().map_err(|_| "invalid T1 span")?,
            frequency_obs: double(12)?,
            binary_phase: double(13)?,
            binary_frequency: double(14)?,
            coeff: (15..15 + nc as usize).map(num).collect::<Result<_, _>>()?,
        })
    }
    pub(crate) fn empty() -> Self {
        Self {
            psrname: String::new(),
            date_string: String::new(),
            utc_string: String::new(),
            mjd_mid: n(0.),
            dm: 0.,
            doppler: 0.,
            log10rms: 0.,
            reference_phase: n(0.),
            frequency_psr_0: n(0.),
            sitename: String::new(),
            span: 0,
            frequency_obs: 0.,
            binary_phase: 0.,
            binary_frequency: 0.,
            coeff: vec![],
        }
    }
}
pub(crate) fn scientific(v: &rug::Float, places: usize) -> String {
    if !v.is_finite() {
        return v.to_string_radix(10, None).to_lowercase();
    }
    if v.is_zero() {
        return format!(
            "{}0.{}e+00",
            if v.is_sign_negative() { "-" } else { "" },
            "0".repeat(places)
        );
    }
    let (negative, digits, exp) = v.to_sign_string_exp(10, Some(places + 1));
    let e = exp.expect("finite nonzero") - 1;
    format!(
        "{}{}.{}e{:+03}",
        if negative { "-" } else { "" },
        &digits[..1],
        &digits[1..],
        e
    )
}
fn read_line(input: &mut impl BufRead) -> Result<String, String> {
    let mut s = String::new();
    if input.read_line(&mut s).map_err(|e| e.to_string())? == 0 {
        Err("T1 end of input".into())
    } else {
        Ok(s)
    }
}
fn field(s: &str, start: usize, len: usize) -> Result<&str, String> {
    s.get(start..start + len)
        .map(str::trim)
        .ok_or_else(|| "short T1 fixed-format field".into())
}
fn grab(s: &str, start: usize, len: usize) -> Result<N, String> {
    N::parse_prefix(field(s, start, len)?.replace('D', "e").as_bytes())
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct T1PolycoSet {
    pub segments: Vec<T1Polyco>,
    pub trailing_error: Option<String>,
}
impl T1PolycoSet {
    pub fn nearest_index(&self, mjd: &N) -> Result<usize, String> {
        if self.segments.is_empty() {
            return Err("empty T1 set".into());
        }
        let index = nearest_index(self.segments.iter().map(|m| &m.mjd_mid), mjd);
        Ok(index)
    }
    pub fn read(input: &mut impl BufRead) -> Result<Self, String> {
        let mut set = Self::default();
        loop {
            if input.fill_buf().map_err(|e| e.to_string())?.is_empty() {
                break;
            }
            match T1Polyco::read(input) {
                Ok(m) => set.segments.push(m),
                Err(e) => {
                    set.trailing_error = Some(e);
                    break;
                }
            }
        }
        if set.segments.is_empty() {
            Err(set.trailing_error.unwrap_or("empty T1 file".into()))
        } else {
            Ok(set)
        }
    }
    pub fn write(&self, mut out: impl Write) -> Result<(), String> {
        for m in &self.segments {
            m.write(&mut out)?;
        }
        Ok(())
    }
}

// t1polyco.c:39-84 reads fixed backing independently of ncoeff. This borrowed
// leaf preserves counts 0/1 for the C ABI without weakening owned file models.
pub(crate) struct PhasePolynomial<'a> {
    pub mjd_mid: &'a N,
    pub reference_phase: &'a N,
    pub frequency_psr_0: &'a N,
    pub dm: f64,
    pub doppler: f64,
    pub frequency_obs: f64,
    pub coeff: &'a [N],
    pub count: usize,
}
impl PhasePolynomial<'_> {
    pub(crate) fn phase(&self, mjd: &N, freq: &N) -> N {
        let dt = (mjd - self.mjd_mid) * 1440.;
        // fda6ad4 t1polyco.c:47/52/59: GCC16 arm64 contracts these four
        // sites (AF native-v8 disassembly), including the final subtraction.
        let mut phase =
            (&dt * 60.).source_mul_add(self.frequency_psr_0, self.reference_phase) + &self.coeff[0];
        let mut spin = self.frequency_psr_0 + &self.coeff[1] / 60.;
        let mut arg = dt.clone();
        for i in 1..self.count {
            phase = self.coeff[i].source_mul_add(&arg, &phase);
            if i < self.count - 1 {
                spin = spin + &self.coeff[i + 1] * (i + 1) as f64 * &arg / 60.;
            }
            arg = arg * &dt;
        }
        // T1 phase uses observing frequency; its instantaneous-frequency routine
        // intentionally does not. The two source contracts are not merged.
        let factor = spin * self.dm
            / "2.41e-4".parse::<N>().expect("literal")
            / n(self.doppler).source_mul_add(&"1e-4".parse::<N>().expect("literal"), &n(1.));
        (-factor).source_mul_add(
            &(n(1.) / (freq * freq) - n(1.) / n(self.frequency_obs * self.frequency_obs)),
            &phase,
        )
    }
}
pub(crate) fn frequency_leaf(
    mjd_mid: &N,
    frequency_psr_0: &N,
    coeff_from_one: &[N],
    count: usize,
    mjd: &N,
) -> N {
    let dt = (mjd - mjd_mid) * 1440.;
    let mut spin = frequency_psr_0 + &coeff_from_one[0] / 60.;
    let mut arg = dt.clone();
    for i in 2..count {
        spin = spin + &coeff_from_one[i - 1] * i as f64 * &arg / 60.;
        arg = arg * &dt;
    }
    spin
}

pub(crate) fn nearest_index<'a>(midpoints: impl IntoIterator<Item = &'a N>, mjd: &N) -> usize {
    let (mut index, mut best) = (0, n(1e6));
    for (i, m) in midpoints.into_iter().enumerate() {
        let offset = (m - mjd).abs_double();
        if offset < best {
            index = i;
            best = offset;
        }
    }
    index
}

// t1polyco.c metadata span conversion, shared by Rust and the native adapter.
pub(crate) fn span_bound(mid: &N, span: i32, sign: i32) -> N {
    if sign < 0 {
        mid - n(span as f64) / 2880.
    } else {
        mid + n(span as f64) / 2880.
    }
}
