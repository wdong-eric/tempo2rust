//! cholesky.C covariance grammar/kernels; choleskyRoutines.C source decomposition.
#![allow(clippy::needless_range_loop)]
use crate::{
    TimingError,
    bats::{ConsumedInput, consume},
    clock_diagnostics::boundary,
    clock_text::{NumberFormat, format_number},
    observation::{read, read_string},
    pulsar_lifecycle::PulsarContext,
    tkfit::FitDiagnostics,
    tksvd::ld,
};
use std::path::{Path, PathBuf};

/// TKspectrum.C::TK_fft, including the source recurrence and forward normalization.
pub fn fft(dir: i32, x: &mut [f64], y: &mut [f64]) -> Result<(), TimingError> {
    let n = x.len();
    if n < 2 || !n.is_power_of_two() || y.len() != n {
        return Err(boundary("TK_fft dimensions"));
    }
    let m = ((n as f64).ln() / 2.0f64.ln() + 0.1) as usize;
    let i2 = n >> 1;
    let mut j = 0;
    for i in 0..n - 1 {
        if i < j {
            x.swap(i, j);
            y.swap(i, j);
        }
        let mut k = i2;
        while k <= j {
            j -= k;
            k >>= 1;
        }
        j += k;
    }
    let (mut c1, mut c2) = (-1.0f64, 0.0f64);
    let mut l2 = 1;
    for _ in 0..m {
        let l1 = l2;
        l2 <<= 1;
        let (mut u1, mut u2): (f64, f64) = (1.0, 0.0);
        for j in 0..l1 {
            for i in (j..n).step_by(l2) {
                let i1 = i + l1;
                // GCC16 arm64 TK_fft contracts these butterflies/recurrences;
                // AE's archive disassembly and shared-input FFT probe fix the order.
                let t1 = u1.mul_add(x[i1], -(u2 * y[i1]));
                let t2 = u1.mul_add(y[i1], u2 * x[i1]);
                x[i1] = x[i] - t1;
                y[i1] = y[i] - t2;
                x[i] += t1;
                y[i] += t2;
            }
            let z = c1.mul_add(u1, -(u2 * c2));
            u2 = c2.mul_add(u1, u2 * c1);
            u1 = z;
        }
        c2 = ((1.0 - c1) / 2.0).sqrt();
        if dir == 1 {
            c2 = -c2;
        }
        c1 = ((1.0 + c1) / 2.0).sqrt();
    }
    if dir == 1 {
        for i in 0..n {
            x[i] /= n as f64;
            y[i] /= n as f64;
        }
    }
    Ok(())
}
/// T2cholDecomposition mutates the lower triangle and returns its diagonal.
/// Products narrow to binary64 before the extended-precision subtraction.
pub fn decompose(a: &mut [Vec<f64>]) -> Result<Vec<f64>, TimingError> {
    let n = a.len();
    if n == 0 || a.iter().any(|r| r.len() != n) {
        return Err(boundary("Cholesky dimensions"));
    }
    let mut p = vec![0.0; n];
    for i in 0..n {
        for j in i..n {
            let mut sum = ld(a[i][j]);
            for k in (0..i).rev() {
                sum -= a[i][k] * a[j][k];
            }
            if i == j {
                if sum <= 0 {
                    return Err(boundary(
                        "T2cholDecomposition: matrix is not positive definite",
                    ));
                }
                p[i] = sum.sqrt().to_f64();
            } else {
                a[j][i] = (sum / p[i]).to_f64();
            }
        }
    }
    Ok(p)
}
/// cholesky_formL, sharing the source decomposition with its direct API.
pub fn form_l(matrix: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, TimingError> {
    let n = matrix.len();
    let mut a = matrix.to_vec();
    let p = decompose(&mut a)?;
    let mut l = vec![vec![0.0; n]; n];
    for i in 0..n {
        l[i][..i].copy_from_slice(&a[i][..i]);
        l[i][i] = p[i];
    }
    Ok(l)
}
pub fn form_linv(matrix: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, TimingError> {
    let mut l = form_l(matrix)?;
    let n = l.len();
    let p: Vec<_> = (0..n).map(|i| l[i][i]).collect();
    let mut out = vec![vec![0.0; n]; n];
    for i in 0..n {
        l[i][i] = 1.0 / p[i];
        out[i][i] = l[i][i];
        for j in i + 1..n {
            let mut sum = 0.0;
            for k in i..j {
                // GCC16 arm64 cholesky_formLinv contracts this subtraction.
                sum = (-l[j][k]).mul_add(l[k][i], sum);
            }
            l[j][i] = sum / p[j];
            out[j][i] = l[j][i];
        }
    }
    Ok(out)
}
fn days(x: &[f64]) -> Result<usize, TimingError> {
    let span = (*x.last().ok_or_else(|| boundary("empty covariance times"))? - x[0] + 1e-10).ceil();
    if !span.is_finite() || span < 0.0 || span > i32::MAX as f64 {
        return Err(boundary("covariance time span"));
    }
    Ok(span as usize)
}
pub fn covar_function_to_matrix(
    c: &[f64],
    x: &[f64],
    nc: usize,
) -> Result<Vec<Vec<f64>>, TimingError> {
    let n = x.len();
    if nc > n {
        return Err(boundary("covariance constraint count"));
    }
    let mut m = vec![vec![0.0; n]; n];
    for i in 0..n - nc {
        for j in 0..n - nc {
            let t0 = (x[i] - x[j]).abs();
            let t1 = t0.floor() as usize;
            let t = t0 - t1 as f64;
            let a = c
                .get(t1)
                .ok_or_else(|| boundary("covariance lag exceeds ndays"))?;
            let b = c
                .get(t1 + 1)
                .ok_or_else(|| boundary("covariance lag+1 exceeds ndays"))?;
            // Archive cholesky_covarFunc2matrix fuses the first product/add.
            m[i][j] = a.mul_add(1.0 - t, b * t);
        }
    }
    Ok(m)
}
pub fn qp_term(freq: f64, logp: f64, f0: f64, sig: f64, lam: f64, df: f64) -> f64 {
    let a = 10.0f64.powf(logp);
    let sigf0 = (f0 * sig).max(0.5 * df);
    let mut ret = 0.0;
    if freq > 0.5 * (f0 - (f0 * f0 - 16.0 * sigf0 * sigf0).sqrt()) {
        for ih in 1..=10 {
            let h = ih as f64;
            ret += a
                * (-(h - 1.0) / lam).exp()
                * (-(freq - f0 * h).powf(2.0) / (2.0 * (h * sigf0).powf(2.0))).exp()
                / h;
        }
    }
    ret
}
/// T2calculateCovarFunc and QP/redpink variants. `model` selects the source PSD.
pub fn powerlaw_covariance(
    alpha: f64,
    fc: f64,
    amp: f64,
    model: &str,
    extra: &[f64],
    ndays: usize,
) -> Result<Vec<f64>, TimingError> {
    let arguments = match model {
        "" => 0,
        "beta" => 1,
        "redpink" => 2,
        "qp" => 4,
        _ => return Err(boundary("unknown powerlaw covariance shape")),
    };
    if extra.len() != arguments {
        return Err(boundary("powerlaw covariance shape arguments"));
    }
    if !fc.is_finite() || fc <= 0.0 {
        return Err(boundary(
            "powerlaw corner makes source FFT length unbounded",
        ));
    }
    let mut n = 128usize;
    while n < (ndays + 1) * 2 || (n as f64) < 2.0 * 365.25 / fc {
        n = n
            .checked_mul(2)
            .filter(|n| *n <= i32::MAX as usize)
            .ok_or_else(|| boundary("covariance FFT size overflow"))?;
    }
    let spectrum = |freq: f64| -> f64 {
        let base = amp / (1.0 + (freq.abs() / fc).powf(2.0)).powf(alpha / 2.0);
        match model {
            "beta" => {
                amp * (freq.abs() / fc).powf(extra[0])
                    / (1.0 + (freq.abs() / fc).powf(2.0)).powf(alpha / 2.0)
            }
            "redpink" => {
                let red = (1.0 + (freq.abs() / fc).powf(2.0)).powf(-alpha / 2.0);
                let pink_amplitude = amp * (1.0 + (extra[1] / fc).powf(2.0)).powf(-alpha / 2.0);
                let pink = (1.0 + (freq.abs() / extra[1]).powf(2.0)).powf(-extra[0] / 2.0);
                // T2calculateCovarFunc_redpink has distinct DC/loop contractions
                // in the matched GCC16 arm64 object; retain each operation order.
                if freq == 0.0 {
                    pink_amplitude.mul_add(pink, base)
                } else {
                    red.mul_add(amp, pink_amplitude * pink)
                }
            }
            "qp" if freq != 0.0 => {
                let f0 = extra[1];
                let logp = extra[0]
                    + (amp / (1.0 + ((f0 * 365.25).abs() / fc).powf(2.0)).powf(alpha / 2.0))
                        .log10();
                let f = freq / 365.25;
                base + (f / f0).powf(-4.0)
                    * qp_term(f, logp, f0, extra[2], extra[3], 1.0 / ndays as f64)
            }
            _ => base,
        }
    };
    let mut re = vec![0.0; n];
    // choleskyRoutines.C omits imaginary DC initialization in three builders.
    // AE 0053 initializes it to zero for the real PSD (native guarded witness).
    let mut im = vec![0.0; n];
    re[0] = spectrum(0.0);
    for i in 1..=n / 2 {
        let p = spectrum(i as f64 / (n as f64 * (1.0 / 365.25)));
        re[i] = p;
        re[n - i] = p;
    }
    fft(1, &mut re, &mut im)?;
    Ok(re[..=ndays]
        .iter()
        .map(|p| p * (86400.0f64 * 365.25).powf(2.0) * 365.25)
        .collect())
}
fn values<'a>(tokens: impl Iterator<Item = &'a str>) -> Result<Vec<f64>, TimingError> {
    let tokens: Vec<_> = tokens.collect();
    let mut source = crate::source_text::SourceText::new(
        Path::new("<covariance model>"),
        tokens.join(" ").into_bytes(),
    );
    (0..tokens.len())
        .map(|_| source.double("covariance number"))
        .collect()
}
/// cholesky_readFromCovarianceFunction. The first scale value is ignored by
/// the source; required daily samples retain their declared input identity.
pub fn read_covariance_function(
    selector: &str,
    x: &[f64],
    nc: usize,
    inputs: &mut Vec<ConsumedInput>,
) -> Result<Vec<Vec<f64>>, TimingError> {
    if nc >= x.len() {
        return Err(boundary("covariance function has no data rows"));
    }
    let mut text = consume(Path::new(selector), "covariance function", inputs)?;
    let ndays = days(&x[..x.len() - nc])?;
    text.double("covariance scale")?;
    let c = (0..=ndays)
        .map(|_| text.double("covariance day"))
        .collect::<Result<Vec<_>, _>>()?;
    covar_function_to_matrix(&c, x, nc)
}

/// cholesky_ecm reads (np+nc)^2 row-major entries; later entries are ignored.
pub fn read_extra_covariance(
    path: &Path,
    np: usize,
    nc: usize,
    inputs: &mut Vec<ConsumedInput>,
) -> Result<Vec<Vec<f64>>, TimingError> {
    let mut text = consume(path, "covariance matrix", inputs)?;
    let n = np
        .checked_add(nc)
        .ok_or_else(|| boundary("ECM dimensions overflow"))?;
    let count = n
        .checked_mul(n)
        .ok_or_else(|| boundary("ECM dimensions overflow"))?;
    if n == 0 {
        return Err(boundary("ECM empty matrix"));
    }
    let v = (0..count)
        .map(|_| text.double("ECM element"))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(v.chunks(n).map(|r| r.to_vec()).collect())
}

pub fn squared_exponential(amplitude: f64, sigma: f64, ndays: usize) -> Vec<f64> {
    let k = 1.0 / (2.0 * sigma * sigma);
    (0..=ndays)
        .map(|i| {
            let x = i as f64;
            amplitude * (-x * x * k).exp()
        })
        .collect()
}
pub fn matern15(amplitude: f64, sigma: f64, ndays: usize) -> Vec<f64> {
    let k = 3.0f64.sqrt() / sigma;
    (0..=ndays)
        .map(|i| {
            let x = i as f64;
            amplitude * (1.0 + k * x) * (-x * k).exp()
        })
        .collect()
}
pub fn matern25(amplitude: f64, sigma: f64, ndays: usize) -> Vec<f64> {
    let k = 5.0f64.sqrt() / sigma;
    let k2 = 5.0 / sigma / sigma / 3.0;
    // cholesky_matern25_kernel contracts the quadratic term on GCC16 arm64.
    (0..=ndays)
        .map(|i| {
            let x = i as f64;
            amplitude * (k2 * x).mul_add(x, 1.0 + k * x) * (-x * k).exp()
        })
        .collect()
}
pub fn dm_covariance_parameters(alpha: f64, amplitude: f64, scale: f64, ndays: usize) -> Vec<f64> {
    (0..=ndays)
        .map(|i| amplitude * (-((i as f64 + 1e-10) / scale).powf(alpha)).exp())
        .collect()
}
/// cholesky_dmModel ignores its source ref_freq argument. D_d is in us^2,
/// the lag and observed span are days, and the resulting covariance is s^2.
pub fn dm_covariance(
    structure: f64,
    lag_days: f64,
    span_days: f64,
    ndays: usize,
) -> Result<Vec<f64>, TimingError> {
    let fc = 1.0 / (span_days.ceil() / 365.25);
    let amplitude = 0.0112
        * (structure * 1e-12)
        * (lag_days * 86400.0).powf(-5.0 / 3.0)
        * (365.25f64 * 86400.0).powf(-1.0 / 3.0)
        * fc.powf(-8.0 / 3.0);
    powerlaw_covariance(8.0 / 3.0, fc, amplitude, "", &[], ndays)
}

/// getCholeskyDiagonals: stores sum(log(sigma)), without the full-L offset 1.
pub fn diagonals(context: &mut PulsarContext, p: usize, sigma: &[f64]) -> Vec<f64> {
    let mut det = 0.0;
    for &v in sigma {
        det += v.ln();
    }
    context.pulsars[p].detL = Some(det);
    sigma.to_vec()
}
/// addCovar: strict MJD cases, with source endpoint extension outside the window.
/// Without SAT coordinates (source ip==NULL), the complete data block is added.
pub fn add_covariance(
    m: &mut [Vec<f64>],
    mm: &[Vec<f64>],
    sat: Option<&[rug::Float]>,
    nc: usize,
    start: f64,
    end: f64,
) -> Result<(), TimingError> {
    let size = m.len();
    if nc > size
        || mm.len() != size
        || m.iter().chain(mm).any(|r| r.len() != size)
        || sat.is_some_and(|s| s.len() < size - nc)
    {
        return Err(boundary("addCovar dimensions"));
    }
    let n = m.len() - nc;
    if let Some(sat) = sat {
        let istart = (0..n).find(|&i| sat[i] > start).unwrap_or(0);
        let iend = (0..n).rev().find(|&i| sat[i] < end).unwrap_or(0);
        for i in 0..n {
            for j in 0..n {
                // addCovar's nine strict cases: equality to a boundary adds nothing.
                let remap = |k: usize| {
                    if sat[k] > start && sat[k] < end {
                        Some(k)
                    } else if sat[k] < start {
                        Some(istart)
                    } else if sat[k] > end {
                        Some(iend)
                    } else {
                        None
                    }
                };
                if let (Some(a), Some(b)) = (remap(i), remap(j)) {
                    m[i][j] +=
                        if (sat[i] > end && sat[j] < start) || (sat[i] < start && sat[j] > end) {
                            mm[istart][iend]
                        } else {
                            mm[a][b]
                        };
                }
            }
        }
    } else {
        for i in 0..n {
            for j in 0..n {
                m[i][j] += mm[i][j];
            }
        }
    }
    Ok(())
}

struct ModelReader<'a> {
    x: &'a [f64],
    sat: Option<&'a [rug::Float]>,
    freq: Option<&'a [f64]>,
    name: &'a [u8],
    nc: usize,
    inputs: &'a mut Vec<ConsumedInput>,
    stack: Vec<String>,
    diagnostics: Option<&'a FitDiagnostics>,
    files: Vec<PathBuf>,
}
impl ModelReader<'_> {
    fn read(
        &mut self,
        selector: &str,
        m: &mut [Vec<f64>],
        mut psr: String,
        mut start: f64,
        mut end: f64,
    ) -> Result<(), TimingError> {
        if self.stack.iter().any(|s| s == selector) {
            return Err(boundary("recursive covariance INCLUDE cycle"));
        }
        self.stack.push(selector.into());
        let content = if selector.trim_start().starts_with("MODEL") {
            selector.as_bytes().to_vec()
        } else {
            consume(Path::new(selector), "covariance model", self.inputs)?.bytes
        };
        let text =
            std::str::from_utf8(&content).map_err(|_| boundary("covariance text encoding"))?;
        if text.as_bytes().first().is_some_and(u8::is_ascii_digit) {
            let cov = read_covariance_function(selector, self.x, self.nc, self.inputs)?;
            m.clone_from_slice(&cov);
            self.stack.pop();
            return Ok(());
        }
        let mut lines = text.lines();
        while let Some(line) = lines.next() {
            let t: Vec<_> = line.split_whitespace().collect();
            if t.is_empty() || t[0].starts_with('#') {
                continue;
            }
            if matches!(t[0], "CLEAR" | "END") {
                psr.clear();
                start = 0.0;
                end = 99999.0;
                continue;
            }
            if t[0] == "PSR" {
                psr = t
                    .get(1)
                    .ok_or_else(|| boundary("PSR needs name"))?
                    .to_string();
            }
            if !psr.is_empty() && psr.as_bytes() != self.name {
                continue;
            }
            if t[0] == "MJD" {
                let v = values(t[1..].iter().copied().take(2))?;
                if v.len() != 2 {
                    return Err(boundary("MJD needs bounds"));
                }
                start = v[0];
                end = v[1];
                continue;
            }
            if t[0] == "INCLUDE" {
                self.read(
                    t.get(1).ok_or_else(|| boundary("INCLUDE needs path"))?,
                    m,
                    psr.clone(),
                    start,
                    end,
                )?;
                continue;
            }
            let mut mm = match t[0] {
                "DCF" => read_covariance_function(
                    t.get(1).ok_or_else(|| boundary("DCF needs path"))?,
                    self.x,
                    self.nc,
                    self.inputs,
                )?,
                "ECM" => {
                    // cholesky_readT2CholModel allocates np+1 rows, whereas ECM
                    // reads np+nc. More than one constraint exceeds that storage.
                    if self.nc > 1 {
                        return Err(boundary("ECM source model allocation with constraints"));
                    }
                    let mut extra = read_extra_covariance(
                        Path::new(t.get(1).ok_or_else(|| boundary("ECM needs path"))?),
                        self.x.len(),
                        self.nc,
                        self.inputs,
                    )?;
                    extra.truncate(self.x.len());
                    for row in &mut extra {
                        row.truncate(self.x.len());
                    }
                    extra
                }
                "MODEL" => {
                    let kind = *t.get(1).ok_or_else(|| boundary("MODEL needs type"))?;
                    if kind == "T2" {
                        continue;
                    }
                    let nd = days(self.x)?;
                    let required = match kind {
                        "1" => 0,
                        "2" => 0,
                        "T2SqExp" | "T2SqExpDM" | "T2Matern1.5" | "T2Matern1.5DM"
                        | "T2Matern2.5" | "T2Matern2.5DM" | "T2YrPowerLaw_X" => 2,
                        "T2PowerLaw_QPc" => 7,
                        "T2PowerLaw_RedPink" => 5,
                        _ => 3,
                    };
                    let mut v = values(t[2..].iter().copied().take(required))?;
                    if v.len() < required {
                        return Err(boundary("missing covariance model arguments"));
                    }
                    let c = match kind {
                        "1" | "2" => {
                            for _ in 0..if kind == "1" { 3 } else { 4 } {
                                let l = lines
                                    .next()
                                    .ok_or_else(|| boundary("short legacy covariance model"))?;
                                v.push(
                                    l.split_whitespace()
                                        .nth(1)
                                        .ok_or_else(|| {
                                            boundary("legacy covariance model argument")
                                        })?
                                        .parse()
                                        .map_err(|_| boundary("legacy covariance model number"))?,
                                );
                            }
                            if kind == "1" {
                                powerlaw_covariance(v[0], v[1], v[2], "", &[], nd)?
                            } else {
                                powerlaw_covariance(v[0], v[2], v[3], "beta", &v[1..2], nd)?
                            }
                        }
                        "T2SqExp" | "T2SqExpDM" => squared_exponential(v[0], v[1], nd),
                        "T2Matern1.5" | "T2Matern1.5DM" => matern15(v[0], v[1], nd),
                        "T2Matern2.5" | "T2Matern2.5DM" => matern25(v[0], v[1], nd),
                        "DMCovarParam" => dm_covariance_parameters(v[0], v[1], v[2], nd),
                        "DM" => {
                            dm_covariance(v[0], v[1], self.x[self.x.len() - 1] - self.x[0], nd)?
                        }
                        "T2PowerLaw" | "T2PowerLawDM" => {
                            powerlaw_covariance(v[0], v[2], v[1], "", &[], nd)?
                        }
                        "T2YrPowerLaw" | "T2YrPowerLawDM" => {
                            powerlaw_covariance(v[0], v[2], v[1] * v[2].powf(-v[0]), "", &[], nd)?
                        }
                        "T2YrPowerLaw_X" => powerlaw_covariance(
                            v[0],
                            0.05,
                            v[1] * 0.05f64.powf(-v[0]),
                            "",
                            &[],
                            nd,
                        )?,
                        "T2PowerLaw_QPc" => {
                            powerlaw_covariance(v[0], v[2], v[1], "qp", &v[3..7], nd)?
                        }
                        "T2PowerLaw_RedPink" => {
                            powerlaw_covariance(v[0], v[2], v[1], "redpink", &v[3..5], nd)?
                        }
                        _ => return Err(boundary(&format!("unknown covariance model {kind}"))),
                    };
                    if kind != "DMCovarParam"
                        && let Some(io) = self.diagnostics.filter(|d| d.debug)
                    {
                        let mut bytes = Vec::new();
                        for &v in &c[..nd] {
                            bytes.extend(format_number(v, NumberFormat::GeneralSix));
                            bytes.push(b'\n');
                        }
                        self.files
                            .push(io.write("newDCF", &bytes).map_err(|e| boundary(&e))?);
                    }
                    let mut mm = covar_function_to_matrix(&c, self.x, self.nc)?;
                    if matches!(
                        kind,
                        "T2PowerLawDM"
                            | "T2YrPowerLawDM"
                            | "T2SqExpDM"
                            | "T2Matern1.5DM"
                            | "T2Matern2.5DM"
                    ) {
                        let f = self
                            .freq
                            .ok_or_else(|| boundary("DM covariance needs observations"))?;
                        covariance_to_dm(&mut mm, f)?;
                    }
                    mm
                }
                _ => continue,
            };
            add_covariance(m, &mm, self.sat, self.nc, start, end)?;
            mm.clear();
        }
        self.stack.pop();
        Ok(())
    }
}

/// cholesky_cov2dm uses observing frequencies in MHz, including each supplied row.
pub fn covariance_to_dm(matrix: &mut [Vec<f64>], frequency_mhz: &[f64]) -> Result<(), TimingError> {
    let n = matrix.len();
    if frequency_mhz.len() != n || matrix.iter().any(|r| r.len() != n) {
        return Err(boundary("DM covariance dimensions"));
    }
    for i in 0..n {
        let g1 = 1.0 / (2.41e-4 * frequency_mhz[i] * frequency_mhz[i]);
        for j in 0..n {
            let g2 = 1.0 / (2.41e-4 * frequency_mhz[j] * frequency_mhz[j]);
            matrix[i][j] *= g1 * g2;
        }
    }
    Ok(())
}

/// cholesky_readT2CholModel and recursive INCLUDE. Paths are resolved in the
/// caller's working directory; absent SAT disables the source MJD mask.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn read_model(
    selector: &str,
    x: &[f64],
    sat: Option<&[rug::Float]>,
    frequency_mhz: Option<&[f64]>,
    name: &[u8],
    nc: usize,
    inputs: &mut Vec<ConsumedInput>,
    diagnostics: Option<&FitDiagnostics>,
) -> Result<(Vec<Vec<f64>>, Vec<PathBuf>), TimingError> {
    if x.is_empty() || nc > x.len() || sat.is_some_and(|v| v.len() < x.len() - nc) {
        return Err(boundary("covariance model dimensions"));
    }
    let mut m = vec![vec![0.0; x.len()]; x.len()];
    let mut reader = ModelReader {
        x,
        sat,
        freq: frequency_mhz,
        name,
        nc,
        inputs,
        stack: vec![],
        diagnostics,
        files: vec![],
    };
    reader.read(selector, &mut m, String::new(), 0.0, 99999.0)?;
    Ok((m, reader.files))
}

pub struct CholeskyMatrix {
    pub covariance: Vec<Vec<f64>>,
    pub lower: Vec<Vec<f64>>,
    pub diagnostic_files: Vec<PathBuf>,
}

fn write_matrix(
    io: &FitDiagnostics,
    name: &str,
    m: &[Vec<f64>],
    format: NumberFormat,
) -> Result<PathBuf, TimingError> {
    let mut bytes = Vec::new();
    for (i, row) in m.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            bytes.extend(format!("{i} {j} ").as_bytes());
            bytes.extend(format_number(v, format));
            bytes.push(b'\n');
        }
        bytes.push(b'\n');
    }
    io.write(name, &bytes).map_err(|e| boundary(&e))
}
/// Covariance selector on selected canonical observations; returns C and lower L.
#[allow(clippy::too_many_arguments)]
pub fn get_cholesky_matrix(
    context: &mut PulsarContext,
    p: usize,
    selector: &str,
    indices: &[usize],
    x: &[f64],
    sigma: &[f64],
    nc: usize,
    inputs: &mut Vec<ConsumedInput>,
    diagnostics: Option<&FitDiagnostics>,
) -> Result<CholeskyMatrix, TimingError> {
    if x.is_empty() || nc > x.len() || x.len() - nc != indices.len() || x.len() != sigma.len() {
        return Err(boundary("getCholeskyMatrix dimensions"));
    }
    // A 1x1 allocation takes the source's diagonal branch before reading a model.
    if x.len() == 1 {
        let diagonal = diagonals(context, p, sigma);
        return Ok(CholeskyMatrix {
            covariance: vec![vec![sigma[0] * sigma[0]]],
            lower: vec![diagonal],
            diagnostic_files: vec![],
        });
    }
    let (psr, rows) = context.pulsar_observations_mut(p)?;
    let sat = indices
        .iter()
        .map(|&i| read(&rows[i].sat, "sat").cloned())
        .collect::<Result<Vec<_>, _>>()?;
    let freq = indices
        .iter()
        .map(|&i| read(&rows[i].freq, "freq").copied())
        .collect::<Result<Vec<_>, _>>()?;
    let name = read_string(&psr.name[..], "name")?;
    let (mut m, mut files) = if selector == "PSRJ" {
        (
            read_covariance_function(
                &format!("covarFunc.dat_{}", String::from_utf8_lossy(&name)),
                x,
                nc,
                inputs,
            )?,
            vec![],
        )
    } else {
        read_model(
            selector,
            x,
            Some(&sat),
            Some(&freq),
            &name,
            nc,
            inputs,
            diagnostics,
        )?
    };
    let extra = psr.ToAextraCovar.flatten();
    if let Some(owner) = extra {
        let e = context
            .matrix(owner)
            .map_err(|e| boundary(&e.to_string()))?;
        for i in 0..m.len() {
            for j in 0..m.len() {
                m[i][j] += e.row(i).map_err(|e| boundary(&e.to_string()))?[j];
            }
        }
    }
    for i in x.len() - nc..x.len() {
        for j in 0..x.len() {
            m[i][j] = 0.0;
            m[j][i] = 0.0;
        }
    }
    for i in 0..m.len() {
        // getCholeskyMatrix uses fmadd in the matched GCC16 arm64 profile.
        m[i][i] = sigma[i].mul_add(sigma[i], m[i][i]);
    }
    if let Some(io) = diagnostics.filter(|d| d.debug) {
        files.push(write_matrix(
            io,
            "chol.covarMatrix",
            &m,
            NumberFormat::GeneralTwentyEight,
        )?);
    }
    // Nonaccelerated T2cholDecomposition exits on a nonpositive pivot, before
    // getCholeskyMatrix's accelerated formL return-code retry can be reached.
    let l = form_l(&m)?;
    if let Some(io) = diagnostics.filter(|d| d.debug) {
        files.push(write_matrix(io, "chol.L", &l, NumberFormat::GeneralSix)?);
    }
    let mut det = 1.0;
    for i in 0..l.len() {
        det += l[i][i].ln();
    }
    context.pulsars[p].detL = Some(det);
    Ok(CholeskyMatrix {
        covariance: m,
        lower: l,
        diagnostic_files: files,
    })
}

/// getCholeskyMatrix called on IFUNC knot coordinates (source ip == NULL).
/// The historical constraint consumes lower L despite its local name "Binv";
/// retain that convention rather than silently substituting an inverse.
pub fn ifunc_cholesky(
    context: &mut PulsarContext,
    index: usize,
    selector: &str,
    inputs: &mut Vec<ConsumedInput>,
) -> Result<Vec<Vec<f64>>, TimingError> {
    let p = &context.pulsars[index];
    let n = *read(&p.ifuncN, "ifuncN")? as usize;
    let x = p
        .ifuncT
        .iter()
        .take(n)
        .map(|v| read(v, "ifuncT").copied())
        .collect::<Result<Vec<_>, _>>()?;
    if n == 1 {
        context.pulsars[index].detL = Some(1e-5f64.ln());
        return Ok(vec![vec![1e-5]]);
    }
    let name = read_string(&p.name[..], "name")?;
    let mut m = vec![vec![0.0; n]; n];
    if selector == "PSRJ" {
        m = read_covariance_function(
            &format!("covarFunc.dat_{}", String::from_utf8_lossy(&name)),
            &x,
            0,
            inputs,
        )?;
    } else {
        m = read_model(selector, &x, None, None, &name, 0, inputs, None)?.0;
    }
    if let Some(owner) = p.ToAextraCovar.flatten() {
        let extra = context
            .matrix(owner)
            .map_err(|e| boundary(&e.to_string()))?;
        for i in 0..n {
            for j in 0..n {
                m[i][j] += extra.row(i).map_err(|e| boundary(&e.to_string()))?[j];
            }
        }
    }
    for i in 0..n {
        m[i][i] += 1e-5 * 1e-5;
    }
    let l = form_l(&m)?;
    let mut det = 1.0;
    for i in 0..n {
        det += l[i][i].ln();
    }
    context.pulsars[index].detL = Some(det);
    Ok(l)
}
