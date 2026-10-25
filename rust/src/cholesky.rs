//! cholesky.C covariance grammar/kernels; choleskyRoutines.C source decomposition.
#![allow(clippy::needless_range_loop)]
use crate::{
    TimingError,
    bats::{ConsumedInput, consume},
    clock_diagnostics::boundary,
    observation::{read, read_string},
    pulsar_lifecycle::PulsarContext,
    tksvd::ld,
};
use std::path::Path;

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
/// T2cholDecomposition + cholesky_formL. Products narrow before extended sum.
pub fn form_l(matrix: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, TimingError> {
    let n = matrix.len();
    if n == 0 || matrix.iter().any(|r| r.len() != n) {
        return Err(boundary("Cholesky dimensions"));
    }
    let mut a = matrix.to_vec();
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
                sum -= l[j][k] * l[k][i];
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
            m[i][j] = a * (1.0 - t) + b * t;
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
                amp * (1.0 + (freq.abs() / fc).powf(2.0)).powf(-alpha / 2.0)
                    + amp
                        * (1.0 + (extra[1] / fc).powf(2.0)).powf(-alpha / 2.0)
                        * (1.0 + (freq.abs() / extra[1]).powf(2.0)).powf(-extra[0] / 2.0)
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
    tokens
        .map(|s| s.parse().map_err(|_| boundary("invalid covariance number")))
        .collect()
}
fn dcf(
    selector: &str,
    x: &[f64],
    nc: usize,
    inputs: &mut Vec<ConsumedInput>,
) -> Result<Vec<Vec<f64>>, TimingError> {
    let text = consume(Path::new(selector), "covariance function", inputs)?;
    let v = values(
        std::str::from_utf8(&text.bytes)
            .map_err(|_| boundary("covariance text encoding"))?
            .split_whitespace(),
    )?;
    let ndays = days(&x[..x.len() - nc])?;
    let c = v
        .get(1..ndays + 2)
        .ok_or_else(|| boundary("incorrect number of covariance days"))?;
    covar_function_to_matrix(c, x, nc)
}
struct ModelReader<'a> {
    x: &'a [f64],
    sat: Option<&'a [rug::Float]>,
    freq: Option<&'a [f64]>,
    name: &'a [u8],
    nc: usize,
    inputs: &'a mut Vec<ConsumedInput>,
    stack: Vec<String>,
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
            let cov = dcf(selector, self.x, self.nc, self.inputs)?;
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
                let v = values(t[1..].iter().copied())?;
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
                "DCF" => dcf(
                    t.get(1).ok_or_else(|| boundary("DCF needs path"))?,
                    self.x,
                    self.nc,
                    self.inputs,
                )?,
                "ECM" => {
                    let s = consume(
                        Path::new(t.get(1).ok_or_else(|| boundary("ECM needs path"))?),
                        "covariance matrix",
                        self.inputs,
                    )?;
                    let v = values(
                        std::str::from_utf8(&s.bytes)
                            .map_err(|_| boundary("ECM text encoding"))?
                            .split_whitespace(),
                    )?;
                    let n = self.x.len();
                    if self.nc > 0 || v.len() < n * n {
                        return Err(boundary("ECM source dimensions"));
                    }
                    v[..n * n].chunks(n).map(|r| r.to_vec()).collect()
                }
                "MODEL" => {
                    let kind = *t.get(1).ok_or_else(|| boundary("MODEL needs type"))?;
                    if kind == "T2" {
                        continue;
                    }
                    let mut v = values(t[2..].iter().copied())?;
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
                        "T2SqExp" | "T2SqExpDM" => (0..=nd)
                            .map(|i| {
                                let x = i as f64;
                                v[0] * (-x * x * (1.0 / (2.0 * v[1] * v[1]))).exp()
                            })
                            .collect(),
                        "T2Matern1.5" | "T2Matern1.5DM" => (0..=nd)
                            .map(|i| {
                                let x = i as f64;
                                let k = 3.0f64.sqrt() / v[1];
                                v[0] * (1.0 + k * x) * (-x * k).exp()
                            })
                            .collect(),
                        "T2Matern2.5" | "T2Matern2.5DM" => (0..=nd)
                            .map(|i| {
                                let x = i as f64;
                                let k = 5.0f64.sqrt() / v[1];
                                let k2 = 5.0 / v[1] / v[1] / 3.0;
                                v[0] * (1.0 + k * x + k2 * x * x) * (-x * k).exp()
                            })
                            .collect(),
                        "DMCovarParam" => (0..=nd)
                            .map(|i| v[1] * (-((i as f64 + 1e-10) / v[2]).powf(v[0])).exp())
                            .collect(),
                        "DM" => {
                            let fc = 1.0 / ((self.x[self.x.len() - 1] - self.x[0]).ceil() / 365.25);
                            let a = 0.0112
                                * (v[0] * 1e-12)
                                * (v[1] * 86400.0).powf(-5.0 / 3.0)
                                * (365.25f64 * 86400.0).powf(-1.0 / 3.0)
                                * fc.powf(-8.0 / 3.0);
                            powerlaw_covariance(8.0 / 3.0, fc, a, "", &[], nd)?
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
                    let mut mm = covar_function_to_matrix(&c, self.x, self.nc)?;
                    if kind.ends_with("DM") {
                        let f = self
                            .freq
                            .ok_or_else(|| boundary("DM covariance needs observations"))?;
                        for i in 0..mm.len() {
                            for j in 0..mm.len() {
                                mm[i][j] *= (1.0 / (2.41e-4 * f[i] * f[i]))
                                    * (1.0 / (2.41e-4 * f[j] * f[j]));
                            }
                        }
                    }
                    mm
                }
                _ => continue,
            };
            let n = self.x.len() - self.nc;
            if let Some(sat) = self.sat {
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
                            m[i][j] += if (sat[i] > end && sat[j] < start)
                                || (sat[i] < start && sat[j] > end)
                            {
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
            mm.clear();
        }
        self.stack.pop();
        Ok(())
    }
}
/// Covariance selector on selected canonical observations; returns C and lower L.
#[allow(clippy::type_complexity)]
pub fn get_cholesky_matrix(
    context: &mut PulsarContext,
    p: usize,
    selector: &str,
    indices: &[usize],
    x: &[f64],
    sigma: &[f64],
    inputs: &mut Vec<ConsumedInput>,
) -> Result<(Vec<Vec<f64>>, Vec<Vec<f64>>), TimingError> {
    if x.is_empty() || x.len() != indices.len() || x.len() != sigma.len() {
        return Err(boundary("getCholeskyMatrix dimensions"));
    }
    // A 1x1 allocation takes the source's diagonal branch before reading a model.
    if x.len() == 1 {
        context.pulsars[p].detL = Some(sigma[0].ln());
        return Ok((vec![vec![sigma[0] * sigma[0]]], vec![vec![sigma[0]]]));
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
    let mut m = vec![vec![0.0; x.len()]; x.len()];
    if selector == "PSRJ" {
        m = dcf(
            &format!("covarFunc.dat_{}", String::from_utf8_lossy(&name)),
            x,
            0,
            inputs,
        )?;
    } else {
        ModelReader {
            x,
            sat: Some(&sat),
            freq: Some(&freq),
            name: &name,
            nc: 0,
            inputs,
            stack: vec![],
        }
        .read(selector, &mut m, String::new(), 0.0, 99999.0)?;
    }
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
    for i in 0..m.len() {
        m[i][i] += sigma[i] * sigma[i];
    }
    // Nonaccelerated T2cholDecomposition exits on a nonpositive pivot, before
    // getCholeskyMatrix's accelerated formL return-code retry can be reached.
    let l = form_l(&m)?;
    let mut det = 1.0;
    for i in 0..l.len() {
        det += l[i][i].ln();
    }
    context.pulsars[p].detL = Some(det);
    Ok((m, l))
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
        m = dcf(
            &format!("covarFunc.dat_{}", String::from_utf8_lossy(&name)),
            &x,
            0,
            inputs,
        )?;
    } else {
        ModelReader {
            x: &x,
            sat: None,
            freq: None,
            name: &name,
            nc: 0,
            inputs,
            stack: vec![],
        }
        .read(selector, &mut m, String::new(), 0.0, 99999.0)?;
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
