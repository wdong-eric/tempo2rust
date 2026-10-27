//! cheby2d.c: polynomial/model/set construction, derivative, evaluation and I/O.
use crate::predictor_number::{N, n};
use serde::Serialize;
use std::io::{BufRead, Write};

#[derive(Clone, Debug, Serialize)]
pub struct Cheby2D {
    nx: usize,
    ny: usize,
    coeff: Vec<N>,
}
#[derive(Clone, Debug, Serialize)]
pub struct ErrorSummary {
    pub rms_turn: N,
    pub mav_turn: N,
}
impl Cheby2D {
    pub fn new(nx: usize, ny: usize) -> Result<Self, String> {
        Self::from_coefficients(
            nx,
            ny,
            vec![
                n(0.);
                nx.checked_mul(ny)
                    .filter(|v| *v <= i32::MAX as usize)
                    .ok_or("Cheby2D dimension overflow")?
            ],
        )
    }
    pub fn from_coefficients(nx: usize, ny: usize, coeff: Vec<N>) -> Result<Self, String> {
        // fda6ad4 cheby2d.c:211-217 accesses nx-2; :517 uses && and admits
        // a missing dimension. Reject nonpositive dimensions at the storage
        // boundary; constant nx=1 polynomials are valid but cannot be differentiated.
        if nx == 0 || ny == 0 || nx.checked_mul(ny) != Some(coeff.len()) {
            return Err("Cheby2D requires positive matching dimensions".into());
        }
        Ok(Self { nx, ny, coeff })
    }
    pub fn nx(&self) -> usize {
        self.nx
    }
    pub fn ny(&self) -> usize {
        self.ny
    }
    pub fn coefficients(&self) -> &[N] {
        &self.coeff
    }
    pub fn construct(
        nx: usize,
        ny: usize,
        mut func: impl FnMut(&[N], &[N]) -> Result<Vec<N>, String>,
    ) -> Result<Self, String> {
        let mut result = Self::new(nx, ny)?;
        let pi: N = "3.14159265358979323846264338327950288".parse()?;
        let x: Vec<_> = (0..nx)
            .map(|i| (&pi * (i as f64 + 0.5) / nx as f64).quad_cos())
            .collect();
        let y: Vec<_> = (0..ny)
            .map(|i| (&pi * (i as f64 + 0.5) / ny as f64).quad_cos())
            .collect();
        let f = func(&x, &y)?;
        if f.len() != nx * ny {
            return Err("Cheby2D callback grid extent".into());
        }
        let tx: Vec<_> = (0..nx)
            .flat_map(|i| {
                (0..nx).map({
                    let pi = pi.clone();
                    move |a| (&pi * i as f64 * (a as f64 + 0.5) / nx as f64).quad_cos()
                })
            })
            .collect();
        let ty: Vec<_> = (0..ny)
            .flat_map(|i| {
                (0..ny).map({
                    let pi = pi.clone();
                    move |a| (&pi * i as f64 * (a as f64 + 0.5) / ny as f64).quad_cos()
                })
            })
            .collect();
        // fda6ad4 cheby2d.c:187/188: GCC16 arm64 emits FMADD for both
        // accumulations (AF native-v3 disassembly); explicit source contractions.
        for i in 0..nx {
            for j in 0..ny {
                let mut sum = n(0.);
                for b in 0..ny {
                    let mut sum2 = n(0.);
                    for a in 0..nx {
                        sum2 = f[b * nx + a].source_mul_add(&tx[i * nx + a], &sum2);
                    }
                    sum = ty[j * ny + b].source_mul_add(&sum2, &sum);
                }
                result.coeff[j * nx + i] = sum * 4. / (nx * ny) as f64;
            }
        }
        Ok(result)
    }
    pub fn x_derivative(&self) -> Result<Self, String> {
        if self.nx < 2 {
            return Err("Cheby2D x derivative requires nx >= 2 (source nx-2 access)".into());
        }
        let mut d = Self::new(self.nx, self.ny)?;
        for y in 0..self.ny {
            let i = y * self.nx;
            d.coeff[i + self.nx - 2] = &self.coeff[i + self.nx - 1] * (2 * (self.nx - 1)) as f64;
            for x in (0..self.nx - 2).rev() {
                d.coeff[i + x] = self.coeff[i + x + 1]
                    .source_mul_add(&n((2 * (x + 1)) as f64), &d.coeff[i + x + 2]);
            }
        }
        Ok(d)
    }
    pub fn evaluate(&self, x: &N, y: &N) -> N {
        let (mut dj, mut dj1) = (n(0.), n(0.));
        for j in (0..self.ny).rev() {
            let (mut di, mut di1) = (n(0.), n(0.));
            for i in (1..self.nx).rev() {
                let di2 = di1;
                di1 = di;
                di = (x * 2.).source_mul_add(&di1, &(-di2)) + &self.coeff[j * self.nx + i];
            }
            di = self.coeff[j * self.nx].source_mul_add(&n(0.5), &x.source_mul_add(&di, &(-di1)));
            let dj2 = dj1;
            dj1 = dj;
            dj = if j > 0 {
                (y * 2.).source_mul_add(&dj1, &(-dj2)) + di
            } else {
                di.source_mul_add(&n(0.5), &y.source_mul_add(&dj1, &(-dj2)))
            };
        }
        dj
    }
    pub fn test(
        &self,
        nx: usize,
        ny: usize,
        mut func: impl FnMut(&[N], &[N]) -> Result<Vec<N>, String>,
    ) -> Result<ErrorSummary, String> {
        if nx == 0 || ny == 0 {
            return Err("empty Cheby2D test grid".into());
        }
        let x: Vec<_> = (0..nx)
            .map(|i| n(2.) * (i as f64 + 0.5) / nx as f64 - 1.)
            .collect();
        let y: Vec<_> = (0..ny)
            .map(|i| n(2.) * (i as f64 + 0.5) / ny as f64 - 1.)
            .collect();
        let f = func(&x, &y)?;
        if f.len() != nx * ny {
            return Err("Cheby2D test callback extent".into());
        }
        let (mut ss, mut mav) = (n(0.), n(0.));
        for (b, y) in y.iter().enumerate() {
            for (a, x) in x.iter().enumerate() {
                let diff = &f[b * nx + a] - self.evaluate(x, y);
                let abs = diff.abs_double();
                if abs > mav {
                    mav = abs;
                }
                ss = diff.source_mul_add(&diff, &ss);
            }
        }
        Ok(ErrorSummary {
            rms_turn: (ss / (nx * ny) as f64).sqrt(),
            mav_turn: mav,
        })
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct ChebyModel {
    pub psrname: String,
    pub sitename: String,
    pub mjd_start: N,
    pub mjd_end: N,
    pub freq_start: N,
    pub freq_end: N,
    pub dispersion_constant: N,
    pub cheby: Cheby2D,
    pub frequency_cheby: Cheby2D,
}
impl ChebyModel {
    pub fn phase(&self, mjd: &N, freq: &N) -> N {
        model_phase(
            &self.cheby,
            [&self.mjd_start, &self.mjd_end],
            [&self.freq_start, &self.freq_end],
            &self.dispersion_constant,
            mjd,
            freq,
        )
    }
    pub fn frequency(&self, mjd: &N, freq: &N) -> N {
        model_frequency(
            &self.frequency_cheby,
            [&self.mjd_start, &self.mjd_end],
            [&self.freq_start, &self.freq_end],
            mjd,
            freq,
        )
    }
    pub fn write(&self, mut out: impl Write) -> Result<(), String> {
        // fda6ad4 cheby2d.c:430-443 passes native long double to ld_fprintf,
        // which consumes __float128. Typed formatting fixes that variadic ABI bug
        // while preserving the 34/25 digit precision and source field ordering.
        let mut s = format!(
            "ChebyModel BEGIN\nPSRNAME {}\nSITENAME {}\nTIME_RANGE {} {}\nFREQ_RANGE {} {}\nDISPERSION_CONSTANT {}\nNCOEFF_TIME {}\nNCOEFF_FREQ {}\n",
            self.psrname,
            self.sitename,
            self.mjd_start.significant(34),
            self.mjd_end.significant(34),
            self.freq_start.significant(34),
            self.freq_end.significant(34),
            self.dispersion_constant.significant(34),
            self.cheby.nx,
            self.cheby.ny
        );
        for x in 0..self.cheby.nx {
            s.push_str("COEFFS");
            for y in 0..self.cheby.ny {
                s.push_str(&format!(
                    " {}",
                    self.cheby.coeff[y * self.cheby.nx + x].significant(25)
                ));
                if (y + 1) % 3 == 0 {
                    s.push('\n');
                }
            }
            s.push('\n');
        }
        s.push_str("ChebyModel END\n");
        out.write_all(s.as_bytes()).map_err(|e| e.to_string())
    }
    pub fn read(input: &mut impl BufRead) -> Result<Self, String> {
        let (mut name, mut site, mut time, mut freq, mut dm) = (None, None, None, None, None);
        let (mut nx, mut ny) = (None, None);
        let mut rows: Vec<Vec<N>> = vec![];
        let mut first = true;
        loop {
            let raw_line = line(input)?;
            let t: Vec<_> = raw_line.split_whitespace().collect();
            if t.is_empty() {
                continue;
            }
            if t.len() < 2 {
                return Err("ChebyModel missing argument".into());
            }
            if raw_line.starts_with('#') {
                continue;
            }
            let key = t[0].to_ascii_uppercase();
            if first && (key != "CHEBYMODEL" || !t[1].eq_ignore_ascii_case("BEGIN")) {
                return Err("ChebyModel expected BEGIN".into());
            }
            let num = |i: usize| {
                t.get(i).ok_or("missing predictor number").and_then(|s| {
                    N::parse_prefix(s.as_bytes()).map_err(|_| "invalid predictor number")
                })
            };
            match key.as_str() {
                "PSRNAME" => name = Some(t[1].to_owned()),
                "SITENAME" => site = Some(t[1].to_owned()),
                "TIME_RANGE" => time = Some((num(1)?, num(2)?)),
                "FREQ_RANGE" => freq = Some((num(1)?, num(2)?)),
                "DISPERSION_CONSTANT" => dm = Some(num(1)?),
                "NCOEFF_TIME" => nx = Some(count(t[1])?),
                "NCOEFF_FREQ" => ny = Some(count(t[1])?),
                "COEFFS" => {
                    let (nx, ny) = (
                        nx.ok_or("missing time dimension")?,
                        ny.ok_or("missing frequency dimension")?,
                    );
                    if nx < 2 || ny == 0 || rows.len() >= nx {
                        return Err("invalid ChebyModel coefficient dimensions/count".into());
                    }
                    let mut values = t[1..]
                        .iter()
                        .map(|s| N::parse_prefix(s.as_bytes()))
                        .collect::<Result<Vec<_>, _>>()?;
                    if ny >= 4 {
                        if values.len() != 3 {
                            return Err(
                                "Cheby continuation requires three coefficients per full line"
                                    .into(),
                            );
                        }
                        while values.len() < ny {
                            let next = line(input)?;
                            let chunk = next
                                .split_whitespace()
                                .map(|s| N::parse_prefix(s.as_bytes()))
                                .collect::<Result<Vec<_>, _>>()?;
                            if chunk.len() != (ny - values.len()).min(3) {
                                return Err("incomplete/excess Cheby continuation".into());
                            }
                            values.extend(chunk);
                        }
                        if ny.is_multiple_of(3) && !line(input)?.trim().is_empty() {
                            return Err("excess Cheby coefficient row".into());
                        }
                    }
                    if values.len() != ny {
                        return Err("incomplete/excess ChebyModel coefficient row".into());
                    }
                    rows.push(values);
                }
                "CHEBYMODEL" => {
                    if t[1].eq_ignore_ascii_case("END") {
                        break;
                    }
                    if !first && t[1].eq_ignore_ascii_case("BEGIN") {
                        return Err("nested ChebyModel BEGIN".into());
                    }
                }
                _ => return Err(format!("unknown ChebyModel keyword {}", t[0])),
            }
            first = false;
        }
        let (nx, ny) = (nx.ok_or("missing nx")?, ny.ok_or("missing ny")?);
        if rows.len() != nx || nx < 2 || ny == 0 {
            return Err("incomplete ChebyModel".into());
        }
        let coeff = (0..ny)
            .flat_map(|y| rows.iter().map(move |r| r[y].clone()))
            .collect();
        let cheby = Cheby2D::from_coefficients(nx, ny, coeff)?;
        let frequency_cheby = cheby.x_derivative()?;
        let (mjd_start, mjd_end) = time.ok_or("missing TIME_RANGE")?;
        let (freq_start, freq_end) = freq.ok_or("missing FREQ_RANGE")?;
        Ok(Self {
            psrname: name.ok_or("missing PSRNAME")?,
            sitename: site.ok_or("missing SITENAME")?,
            mjd_start,
            mjd_end,
            freq_start,
            freq_end,
            dispersion_constant: dm.ok_or("missing DISPERSION_CONSTANT")?,
            cheby,
            frequency_cheby,
        })
    }
}
fn line(input: &mut impl BufRead) -> Result<String, String> {
    let mut s = String::new();
    if input.read_line(&mut s).map_err(|e| e.to_string())? == 0 {
        Err("unexpected predictor EOF".into())
    } else {
        Ok(s)
    }
}
fn count(s: &str) -> Result<usize, String> {
    let i = crate::clock_text::integer_prefix(s.as_bytes())
        .map_err(|e| e.to_string())?
        .ok_or("missing predictor count")?
        .0;
    usize::try_from(i).map_err(|_| "negative predictor count".into())
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChebyModelSet {
    pub segments: Vec<ChebyModel>,
}
impl ChebyModelSet {
    pub fn nearest_index(&self, mjd: &N) -> Option<usize> {
        nearest_index(
            self.segments.iter().map(|m| (&m.mjd_start, &m.mjd_end)),
            mjd,
        )
    }
    pub fn insert(&mut self, from: &Self) {
        self.segments.extend(from.segments.iter().cloned());
    }
    pub fn keep(&mut self, mjds: &[N]) {
        let mut keep = vec![false; self.segments.len()];
        for t in mjds {
            if let Some(i) = self.nearest_index(t) {
                keep[i] = true;
            }
        }
        let mut i = 0;
        self.segments.retain(|_| {
            let v = keep[i];
            i += 1;
            v
        });
    }
    pub fn write(&self, mut out: impl Write) -> Result<(), String> {
        writeln!(out, "ChebyModelSet {} segments", self.segments.len())
            .map_err(|e| e.to_string())?;
        for m in &self.segments {
            m.write(&mut out)?;
        }
        Ok(())
    }
    pub fn read(input: &mut impl BufRead) -> Result<Self, String> {
        let l = line(input)?;
        let t: Vec<_> = l.split_whitespace().collect();
        if t.len() < 2 || !t[0].eq_ignore_ascii_case("ChebyModelSet") {
            return Err("expected ChebyModelSet".into());
        }
        let count = count(t[1])?;
        let mut segments = vec![];
        for _ in 0..count {
            segments.push(ChebyModel::read(input)?);
        }
        Ok(Self { segments })
    }
}

pub(crate) fn nearest_index<'a>(
    ranges: impl IntoIterator<Item = (&'a N, &'a N)>,
    mjd: &N,
) -> Option<usize> {
    let (mut best, mut index) = (n(1e6), None);
    for (i, (start, end)) in ranges.into_iter().enumerate() {
        if mjd < start || mjd > end {
            continue;
        }
        let offset = (start + end)
            .source_mul_add(&n(0.5), &(-mjd.clone()))
            .abs_double();
        if offset < best {
            best = offset;
            index = Some(i);
        }
    }
    index
}

/// Compiled cheby2d.c::testFunc callback, including source binary128 sinl macro.
pub fn test_function(x: &N, y: &N) -> N {
    use rug::Float;
    N::from_float(
        &(Float::with_val(113, (x * 0.5).as_float())
            + Float::with_val(113, x.as_float()).sin() * 0.1f64
            + Float::with_val(113, (y * 0.7).as_float())
            + Float::with_val(113, (y * 0.1 * y).as_float())),
    )
}

// cheby2d.c:398-424: each native call borrows only its selected polynomial.
// The shared leaves avoid constructing unrelated metadata or derivative backing.
fn model_evaluate(poly: &Cheby2D, time: [&N; 2], band: [&N; 2], mjd: &N, freq: &N) -> N {
    poly.evaluate(
        &(n(-1.) + (mjd - time[0]) * 2. / (time[1] - time[0])),
        &(n(-1.) + (freq - band[0]) * 2. / (band[1] - band[0])),
    )
}
pub(crate) fn model_phase(
    poly: &Cheby2D,
    time: [&N; 2],
    band: [&N; 2],
    dm: &N,
    mjd: &N,
    freq: &N,
) -> N {
    model_evaluate(poly, time, band, mjd, freq) + dm / (freq * freq)
}
pub(crate) fn model_frequency(
    poly: &Cheby2D,
    time: [&N; 2],
    band: [&N; 2],
    mjd: &N,
    freq: &N,
) -> N {
    model_evaluate(poly, time, band, mjd, freq) * 2. / (time[1] - time[0]) / 86400.
}
