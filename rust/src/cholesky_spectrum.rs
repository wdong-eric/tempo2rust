//! TKspectrum.C dependency closure for automatic covariance estimation.
//! The full standalone TKspectrum preprocessing/plotting interface is separate.
use crate::{TimingError, clock_diagnostics::boundary, tksvd::ld};
use std::f64::consts::PI;

#[derive(Debug, serde::Serialize)]
pub struct Spectrum {
    #[serde(serialize_with = "crate::tkfit::serialize_numbers")]
    pub frequency_per_day: Vec<f64>,
    #[serde(serialize_with = "crate::tkfit::serialize_numbers")]
    pub density_year3: Vec<f64>,
    #[serde(serialize_with = "crate::tkfit::serialize_numbers")]
    pub real: Vec<f64>,
    #[serde(serialize_with = "crate::tkfit::serialize_numbers")]
    pub imaginary: Vec<f64>,
    #[serde(serialize_with = "crate::tkfit::serialize_numbers")]
    pub error: Vec<f64>,
}

/// calcSpectraErr_complex and calcSpectra[Err], two-column SVD per frequency.
pub fn calculate(
    lower: &[Vec<f64>],
    x: &[f64],
    y: &[f64],
    nfit: i32,
) -> Result<Spectrum, TimingError> {
    let n = x.len();
    if n < 2 || y.len() != n {
        return Err(boundary("calcSpectra dimensions"));
    }
    let nf = if nfit < 0 { n / 2 - 1 } else { nfit as usize };
    let white_y = crate::tk_matrix::forward_sub_vec(lower, y).map_err(|e| boundary(&e))?;
    let mut output = Spectrum {
        frequency_per_day: vec![],
        density_year3: vec![],
        real: vec![],
        imaginary: vec![],
        error: vec![],
    };
    for k in 0..nf {
        let omega = 2.0 * PI / ((x[n - 1] - x[0]) * n as f64 / (n - 1) as f64) * (k + 1) as f64;
        let matrix: Vec<_> = x
            .iter()
            .map(|t| vec![(omega * t).cos(), (omega * t).sin()])
            .collect();
        let white_matrix =
            crate::tk_matrix::forward_sub(lower, &matrix).map_err(|e| boundary(&e))?;
        let fit = crate::tkfit::constrained_least_squares(
            y,
            &white_y,
            &matrix,
            &white_matrix,
            &[],
            &[],
            1e-40,
            true,
        )
        .map_err(|e| boundary(&e))?;
        let v = (x[n - 1] - x[0]) / 365.25 / 2.0 / (365.25_f64 * 86400.0).powf(2.0);
        let real = v.sqrt() * fit.increments[0].as_float().to_f64();
        let imaginary = v.sqrt() * fit.increments[1].as_float().to_f64();
        output.frequency_per_day.push(omega / 2.0 / PI);
        output.real.push(real);
        output.imaginary.push(imaginary);
        output
            .density_year3
            .push(real.powf(2.0) + imaginary.powf(2.0));
        output.error.push(
            (x[n - 1] - x[0]) / 365.25 / 2.0
                * (fit.errors[0].as_float().to_f64().powf(2.0)
                    + fit.errors[1].as_float().to_f64().powf(2.0))
                / (365.25_f64 * 86400.0).powf(2.0),
        );
    }
    Ok(output)
}

/// T2calculateSpectra's released weighted/unweighted Lomb paths. Like TKsortit,
/// this sorts x and y in place while leaving the supplied uncertainties alone.
pub fn periodogram(
    x: &mut [f64],
    y: &mut [f64],
    errors: &[f64],
    prewhite: i32,
    weighted: bool,
) -> Result<Spectrum, TimingError> {
    let n = x.len();
    if n < 2 || y.len() != n || (weighted && errors.len() < n) {
        return Err(boundary("T2calculateSpectra dimensions"));
    }
    while let Some(i) = x.windows(2).position(|p| p[0] > p[1]) {
        x.swap(i, i + 1);
        y.swap(i, i + 1);
    }
    let mean = y.iter().sum::<f64>() / n as f64;
    let mut values: Vec<_> = y.iter().map(|v| v - mean).collect();
    if prewhite == 1 || prewhite == 2 {
        for _ in 0..prewhite {
            // TKfirstDifference uses a float scratch buffer and float subtraction.
            let single: Vec<f32> = values.iter().map(|v| *v as f32).collect();
            values[0] = 0.0;
            for i in 1..n {
                values[i] = (single[i] - single[i - 1]) as f64;
            }
        }
        if prewhite == 2 {
            values[1] = 0.0;
        }
    }
    let (frequencies, mut power, real, imaginary, variance) = if weighted {
        let (x, y, re, im) = weighted_lomb(x, &values, errors)?;
        (x, y, re, im, 1.0)
    } else {
        let (x, y, var) = lomb(x, &values, 1.0, 1.0)?;
        (x, y, vec![], vec![], var)
    };
    let nspec = power.len();
    let span = x[n - 1] - x[0];
    for (freq, value) in frequencies.iter().zip(&mut power) {
        if weighted {
            *value = (*value / (365.25_f64 * 86400.0).powf(2.0)) * (span / 365.25) / 2.0;
        } else {
            *value *= 2.0 * variance * nspec as f64;
            *value = (*value / (365.25_f64 * 86400.0).powf(2.0)) * 2.0 * (span / 365.25)
                / n as f64
                / n as f64;
        }
        if *freq != 0.0 && (prewhite == 1 || prewhite == 2) {
            let filter = 4.0 * (PI * freq * (x[1] - x[0])).sin().powf(2.0);
            *value /= if prewhite == 1 {
                filter
            } else {
                filter.powf(2.0)
            };
        }
    }
    // TKspectrum copies nSpec values but reports nSpec-1 to its caller.
    let reported = nspec
        .checked_sub(1)
        .ok_or_else(|| boundary("TKspectrum negative reported count"))?;
    Ok(Spectrum {
        frequency_per_day: frequencies[..reported].to_vec(),
        density_year3: power[..reported].to_vec(),
        real,
        imaginary,
        error: vec![],
    })
}

/// TKlomb_d, including its trigonometric recurrence and sample variance.
pub fn lomb(
    x: &[f64],
    y: &[f64],
    ofac: f64,
    hifac: f64,
) -> Result<(Vec<f64>, Vec<f64>, f64), TimingError> {
    let n = x.len();
    if n < 2 || y.len() != n {
        return Err(boundary("TKlomb_d dimensions"));
    }
    let mean = y.iter().sum::<f64>() / n as f64;
    let (mut variance, mut ep) = (0.0, 0.0);
    for value in y {
        let s = value - mean;
        ep += s;
        variance += s * s;
    }
    variance = (variance - ep * ep / n as f64) / (n - 1) as f64;
    let (mut min, mut max) = (x[0], x[0]);
    for &t in x {
        if t < min {
            min = t;
        }
        if t > max {
            max = t;
        }
    }
    let frequency = 1.0 / ((max - min) * ofac * (n as f64 / (n - 1) as f64));
    let center = (min + max) / 2.0;
    let count = 0.5 * ofac * hifac * n as f64;
    if !count.is_finite() || count < 0.0 || count > i32::MAX as f64 {
        return Err(boundary("TKlomb_d output count"));
    }
    let (mut alpha, mut beta, mut sine, mut cosine) = (vec![], vec![], vec![], vec![]);
    for &t in x {
        let theta = 2.0 * PI * frequency * (t - center);
        alpha.push(-2.0 * (theta / 2.0).sin() * (theta / 2.0).sin());
        beta.push(theta.sin());
        sine.push(theta.sin());
        cosine.push(theta.cos());
    }
    let (mut frequencies, mut power) = (vec![], vec![]);
    for j in 0..count as usize {
        let (mut s1, mut s2) = (0.0, 0.0);
        for i in 0..n {
            let (st, ct) = (sine[i], cosine[i]);
            s1 += st * ct;
            s2 += (ct - st) * (ct + st);
        }
        let tau = 0.5 * (2.0 * s1).atan2(s2);
        let (stau, ctau) = tau.sin_cos();
        let (mut s1, mut s2, mut s3, mut s4) = (0.0, 0.0, 0.0, 0.0);
        for i in 0..n {
            let (st, ct, value) = (sine[i], cosine[i], y[i] - mean);
            s1 += value * (ct * ctau + st * stau);
            s2 += value * (st * ctau - ct * stau);
            s3 += (ct * ctau + st * stau).powf(2.0);
            s4 += (ct * stau - st * ctau).powf(2.0);
            cosine[i] += alpha[i] * ct - beta[i] * sine[i];
            sine[i] += alpha[i] * sine[i] + beta[i] * ct;
        }
        frequencies.push(frequency * (j + 1) as f64);
        power.push(0.5 / variance * (s1 * s1 / s3 + s2 * s2 / s4));
    }
    Ok((frequencies, power, variance))
}

/// TK_weightLS. Preserve the source's distinct s5 precision and b expression.
#[allow(clippy::type_complexity)]
pub fn weighted_lomb(
    x: &[f64],
    y: &[f64],
    errors: &[f64],
) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), TimingError> {
    let n = x.len();
    if n < 2 || y.len() != n || errors.len() < n {
        return Err(boundary("TK_weightLS dimensions"));
    }
    let (mut min, mut max) = (x[0], x[0]);
    for &t in x {
        if t < min {
            min = t;
        }
        if t > max {
            max = t;
        }
    }
    let omega0 = (ld(2.0) * PI / ((max - min) * n as f64 / (n - 1) as f64)).to_f64();
    let (mut frequencies, mut power, mut real, mut imaginary) = (vec![], vec![], vec![], vec![]);
    for j in 0..n / 2 - 1 {
        let omega = omega0 * (j + 1) as f64;
        let (mut s1, mut s2, mut s3, mut s4, mut s5) =
            (ld(0.0), ld(0.0), ld(0.0), ld(0.0), ld(0.0));
        for i in 0..n {
            let (si, ci, e) = ((omega * x[i]).sin(), (omega * x[i]).cos(), errors[i]);
            s1 += y[i] * si / e / e;
            s2 += si * si / e / e;
            s3 += si * ci / e / e;
            s4 += y[i] * ci / e / e;
            s5 += ld(ci) * ci / e / e;
        }
        let b = ((s4 - s1.clone() / &s2) / (s5 - s3.clone() / &s2)).to_f64();
        let a = ((s1 - ld(b) * s3) / s2).to_f64();
        frequencies.push(omega / 2.0 / PI);
        power.push(a * a + b * b);
        real.push(b);
        imaginary.push(a);
    }
    Ok((frequencies, power, real, imaginary))
}
