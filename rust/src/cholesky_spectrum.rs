//! TKspectrum.C dependency closure for automatic covariance estimation.
//! Shared canonical estimators; full TKspectrum driver is in spectrum.rs.
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
    calculate_with_diagnostics(lower, x, y, nfit, None)
}

pub fn calculate_with_diagnostics(
    lower: &[Vec<f64>],
    x: &[f64],
    y: &[f64],
    nfit: i32,
    diagnostics: Option<&crate::tkfit::FitDiagnostics>,
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
        // 0053: TKspectrum.C:1715 passes sigma-z global `data` to diagnostics.
        // The solve uses white_data, but residual files need this call's resy.
        let fit = crate::tkfit::least_squares_outputs(
            Some(y),
            Some(&white_y),
            &matrix,
            &white_matrix,
            &[],
            &[],
            1e-40,
            true,
            crate::tkfit::FitOutputs::ALL,
            diagnostics,
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

/// T2calculateSpectra's weighted/unweighted Lomb adapter. Sorts x/y and a
/// working copy of associated errors; publishes 0090 warnings to diagnostics.
pub fn periodogram(
    x: &mut [f64],
    y: &mut [f64],
    errors: &[f64],
    prewhite: i32,
    weighted: bool,
) -> Result<Spectrum, TimingError> {
    let mut errors = errors.to_vec();
    let output = crate::spectrum::analyze(
        x,
        y,
        &mut errors,
        &crate::spectrum::Options {
            prewhite,
            estimator: if weighted { 4 } else { 2 },
            ..Default::default()
        },
    )?;
    crate::clkcorr::with_process(|s| {
        for warning in &output.warnings {
            s.diagnostics.stdout(format!("{warning}\n").as_bytes());
        }
        Ok(())
    })?;
    let reported = usize::try_from(output.reported_count)
        .map_err(|_| boundary("TKspectrum negative reported count"))?;
    Ok(Spectrum {
        frequency_per_day: output.frequency_per_day[..reported].to_vec(),
        density_year3: output.values[..reported].to_vec(),
        real: output.real,
        imaginary: output.imaginary,
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
    let mean = crate::toolkit::mean_d(y);
    let variance = crate::toolkit::variance_d(y);
    let min = crate::toolkit::find_min_d(x)?;
    let max = crate::toolkit::find_max_d(x)?;
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
            s1 = st.mul_add(ct, s1);
            s2 = (ct - st).mul_add(ct + st, s2);
        }
        let tau = 0.5 * (2.0 * s1).atan2(s2);
        let (stau, ctau) = tau.sin_cos();
        let (mut s1, mut s2, mut s3, mut s4) = (0.0, 0.0, 0.0, 0.0);
        for i in 0..n {
            let (st, ct, value) = (sine[i], cosine[i], y[i] - mean);
            // GCC16/arm64 pinned native contracts these multiply/add pairs;
            // the Nyquist recurrence is sensitive to their rounding (AG witness).
            let c = ctau.mul_add(ct, st * stau);
            let s = ctau.mul_add(st, -(ct * stau));
            s1 = c.mul_add(value, s1);
            s2 = s.mul_add(value, s2);
            s3 += c.powf(2.0);
            s4 += (-ctau).mul_add(st, ct * stau).powf(2.0);
            cosine[i] += alpha[i].mul_add(ct, -(beta[i] * st));
            sine[i] += alpha[i].mul_add(st, beta[i] * ct);
        }
        frequencies.push(frequency * (j + 1) as f64);
        power.push(0.5 / variance * (s1 * s1 / s3 + s2 * s2 / s4));
    }
    Ok((frequencies, power, variance))
}

/// TK_weightLS, retaining the distinct s5 precision and corrected elimination.
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
        // 0053: GW-independent TKspectrum.C:503 at fda6ad4a omits the
        // s3 factors when eliminating a from s2*a+s3*b=s1 and
        // s3*a+s5*b=s4. Restore those factors; keep all cast positions.
        // weighted_bug.cc retains the original 2.5563/2.0581 versus 3/2 witness.
        let b = ((s4 - s1.clone() * &s3 / &s2) / (s5 - s3.clone() * &s3 / &s2)).to_f64();
        let a = ((s1 - ld(b) * s3) / s2).to_f64();
        frequencies.push(omega / 2.0 / PI);
        power.push(a * a + b * b);
        real.push(b);
        imaginary.push(a);
    }
    Ok((frequencies, power, real, imaginary))
}
