//! choleskyRoutines.C smoothing, spectral estimates and covariance helpers.
use crate::{
    TimingError, clock_diagnostics::boundary, observation::read, pulsar_lifecycle::PulsarContext,
    tempo2::Label::param_pepoch,
};

pub struct Residuals {
    pub time_days: Vec<f64>,
    pub residual_s: Vec<f64>,
    pub sigma_s: Vec<f64>,
}
/// T2obtainTimingResiduals: SAT, deletion only; START/FINISH are not applied.
pub fn timing_residuals(
    context: &mut PulsarContext,
    index: usize,
) -> Result<Residuals, TimingError> {
    let (p, obs) = context.pulsar_observations_mut(index)?;
    let mut out = Residuals {
        time_days: vec![],
        residual_s: vec![],
        sigma_s: vec![],
    };
    for o in obs.iter().take(*read(&p.nobs, "nobs")? as usize) {
        if *read(&o.deleted, "deleted")? == 0 {
            out.time_days.push(
                (read(&o.sat, "sat")?.clone() - &p.param.slots[param_pepoch as usize].cells[0].val)
                    .to_f64(),
            );
            out.residual_s.push(read(&o.residual, "residual")?.to_f64());
            out.sigma_s.push(*read(&o.toaErr, "toaErr")? * 1e-6);
        }
    }
    Ok(out)
}

pub fn white_noise_level(spectrum: &[f64], nlast: usize) -> Result<f64, TimingError> {
    if nlast > spectrum.len() {
        return Err(boundary("T2getWhiteNoiseLevel count"));
    }
    let mut sum = 0.0;
    for i in 1..=nlast {
        sum += spectrum[spectrum.len() - i];
    }
    Ok(sum / nlast as f64)
}

/// T2cubicFit is explicitly unweighted and rescales its reported errors.
pub fn cubic_fit(x: &[f64], y: &[f64]) -> Result<crate::tkfit::LeastSquares, TimingError> {
    crate::tkfit::fit_polynomial(x, y, 4).map_err(|e| boundary(&e))
}

fn cubic(x: f64, coefficients: &[f64; 4]) -> f64 {
    coefficients[0]
        + coefficients[1] * x
        + coefficients[2] * x.powf(2.0)
        + coefficients[3] * x.powf(3.0)
}

/// T2findSmoothCurve: division order differs from T2interpolate below.
pub fn smooth_curve(
    x: &[f64],
    y: &[f64],
    errors: &[f64],
    coefficients: &[f64; 4],
    width: f64,
) -> Result<Vec<f64>, TimingError> {
    if x.len() != y.len() || x.len() != errors.len() {
        return Err(boundary("T2findSmoothCurve dimensions"));
    }
    let difference: Vec<_> = x
        .iter()
        .zip(y)
        .map(|(t, v)| v - cubic(*t, coefficients))
        .collect();
    let mut result = Vec::new();
    for &t in x {
        let (mut top, mut bottom) = (0.0, 0.0);
        for j in 0..x.len() {
            let weight = (-((t - x[j]) / width).abs()).exp();
            top += difference[j] * weight / errors[j] / errors[j];
            bottom += weight / errors[j] / errors[j];
        }
        result.push(top / bottom + cubic(t, coefficients));
    }
    Ok(result)
}

pub fn interpolate(
    x: &[f64],
    y: &[f64],
    errors: &[f64],
    coefficients: &[f64; 4],
    step: i32,
    width: f64,
) -> Result<(Vec<f64>, Vec<f64>), TimingError> {
    if x.is_empty() || x.len() != y.len() || x.len() != errors.len() {
        return Err(boundary("T2interpolate dimensions"));
    }
    let count = ((x[x.len() - 1] - x[0]) / step as f64).ceil();
    if !count.is_finite() || count < 0.0 || count > i32::MAX as f64 {
        return Err(boundary("T2interpolate output count"));
    }
    let (mut times, mut values) = (Vec::new(), Vec::new());
    for i in 0..count as usize {
        let t = x[0]
            + (i as i32)
                .checked_mul(step)
                .ok_or_else(|| boundary("T2interpolate integer product"))? as f64;
        let (mut top, mut bottom) = (0.0, 0.0);
        for j in 0..x.len() {
            let difference = y[j] - cubic(x[j], coefficients);
            let weight = (-((t - x[j]) / width).abs()).exp();
            top += difference / errors[j] / errors[j] * weight;
            bottom += 1.0 / errors[j] / errors[j] * weight;
        }
        times.push(t);
        values.push(top / bottom + cubic(t, coefficients));
    }
    Ok((times, values))
}

pub fn high_frequency_residuals(y: &[f64], smooth: &[f64]) -> Result<Vec<f64>, TimingError> {
    if y.len() != smooth.len() {
        return Err(boundary("T2getHighFreqRes dimensions"));
    }
    Ok(y.iter().zip(smooth).map(|(a, b)| a - b).collect())
}

#[derive(Clone, Debug)]
pub struct SpectralGuess {
    pub alpha: f64,
    pub amplitude: f64,
    pub corner_per_day: f64,
    pub nfit: usize,
    pub white_corner_per_day: f64,
}
/// T2guess_vals. An absent white crossing is the source's out-of-bounds boundary.
pub fn guess_spectrum(
    x: &[f64],
    y: &[f64],
    white: f64,
    prewhite: i32,
) -> Result<SpectralGuess, TimingError> {
    let n = x.len();
    if n < 2 || y.len() != n {
        return Err(boundary("T2guess_vals dimensions"));
    }
    let mut i = 1;
    while *y
        .get(i)
        .ok_or_else(|| boundary("T2guess_vals white crossing exceeds spectrum"))?
        > white
    {
        i += 1;
    }
    if prewhite != 0 {
        i += 1;
    }
    if i < 2 {
        return Ok(SpectralGuess {
            alpha: -0.01,
            amplitude: 0.0,
            corner_per_day: x[0],
            nfit: n / 2,
            white_corner_per_day: x[0],
        });
    }
    let white_corner = *x
        .get(i)
        .ok_or_else(|| boundary("T2guess_vals crossing+1 exceeds spectrum"))?;
    let (lx, ly): (Vec<_>, Vec<_>) = x[..i]
        .iter()
        .zip(&y[..i])
        .map(|(x, y)| (x.log10(), y.log10()))
        .unzip();
    let (mut mx, mut my, mut vx, mut covariance) = (0.0, 0.0, 0.0, 0.0);
    for j in 0..i {
        mx += lx[j] / i as f64;
        my += ly[j] / i as f64;
    }
    for j in 0..i {
        let tx = lx[j] - mx;
        let ty = ly[j] - my;
        vx += tx * tx / i as f64;
        covariance += tx * ty / i as f64;
    }
    let mut alpha = covariance / vx;
    // Source powf is single precision, including multiplication of its results.
    let mut amplitude =
        (10.0_f32.powf((my - alpha * mx) as f32) * (x[0] as f32).powf(alpha as f32)) as f64;
    if alpha.partial_cmp(&0.0) != Some(std::cmp::Ordering::Less) {
        alpha = -0.01;
        amplitude = 0.0;
    }
    // The second source if/else overwrites fc_white from the preceding branch.
    let fc_white = if amplitude < 0.0 {
        alpha = -0.01;
        amplitude = 0.0;
        x[0]
    } else {
        white_corner
    };
    Ok(SpectralGuess {
        alpha,
        amplitude,
        corner_per_day: x[0],
        nfit: i,
        white_corner_per_day: fc_white,
    })
}

/// T2fitSpectra's numerical fit with already selected model controls. Interactive
/// parameter selection has its own stateful adapter; this is the aval=1 path.
pub fn fit_spectrum(
    x: &[f64],
    y: &[f64],
    alpha: f64,
    fc: f64,
    nfit: usize,
    amplitude: f64,
    beta: Option<f64>,
) -> Result<(f64, f64), TimingError> {
    if x.is_empty() || x.len() != y.len() || nfit > x.len() {
        return Err(boundary("T2fitSpectra dimensions"));
    }
    let scale = if amplitude == -1.0 {
        let mut sum = 0.0;
        for i in 0..nfit {
            let base = (1.0 + ((x[i] * 365.25).abs() / fc).powf(2.0)).powf(alpha / 2.0);
            let model = if let Some(beta) = beta {
                (x[i] * 365.25 / fc).powf(beta) / base
            } else {
                1.0 / base
            };
            sum += y[i] / model;
        }
        sum / nfit as f64
    } else {
        amplitude
    };
    let mut variance = 0.0;
    let df = x[0] * 365.25;
    for v in &y[..nfit] {
        variance += v * df;
    }
    variance *= (86400.0_f64 * 365.25).powf(2.0);
    Ok((scale, variance))
}

/// T2calculateCholesky. Its fitVar/highFreqRes/errorScale/dcmflag inputs are unused.
pub fn calculate_cholesky(
    x: &[f64],
    errors: &[f64],
    alpha: f64,
    fc: f64,
    amplitude: f64,
    beta: Option<f64>,
) -> Result<(Vec<f64>, Vec<Vec<f64>>), TimingError> {
    if x.is_empty() || errors.len() != x.len() {
        return Err(boundary("T2calculateCholesky dimensions"));
    }
    let span = (x[x.len() - 1] - x[0] + 1e-10).ceil();
    if !span.is_finite() || span < 0.0 || span > i32::MAX as f64 {
        return Err(boundary("T2calculateCholesky span"));
    }
    let function = crate::cholesky::powerlaw_covariance(
        alpha,
        fc,
        amplitude,
        if beta.is_some() { "beta" } else { "powerlaw" },
        &[beta.unwrap_or(0.0)],
        span as usize,
    )?;
    let mut matrix = crate::cholesky::covar_function_to_matrix(&function, x, 0)?;
    for i in 0..x.len() {
        matrix[i][i] += errors[i] * errors[i];
    }
    Ok((function, crate::cholesky::form_l(&matrix)?))
}

pub fn daily_covariance(
    x: &[f64],
    y: &[f64],
    errors: &[f64],
    weighted: bool,
) -> Result<(Vec<f64>, Vec<usize>, f64), TimingError> {
    let n = x.len();
    if n == 0 || y.len() != n || errors.len() != n {
        return Err(boundary("T2calculateDailyCovariance dimensions"));
    }
    let span = x[n - 1] - x[0] + 0.5;
    if !span.is_finite() || span < 0.0 || span >= i32::MAX as f64 {
        return Err(boundary("T2calculateDailyCovariance span"));
    }
    let nd = span as usize + 1;
    let (mut covariance, mut count, mut weights) = (vec![0.0; nd], vec![0; nd], vec![0.0; nd]);
    let (mut zero, mut nzero) = (0.0, 0);
    for i in 0..n {
        for j in i..n {
            let dt = (x[i] - x[j]).abs();
            if dt == 0.0 {
                zero += y[i] * y[j];
                nzero += 1;
            } else {
                let bin = (dt + 0.5) as usize;
                if bin >= nd {
                    return Err(boundary("daily covariance lag exceeds first/last span"));
                }
                covariance[bin] += if weighted {
                    y[i] * y[j] / (errors[i] * errors[j])
                } else {
                    y[i] * y[j]
                };
                count[bin] += 1;
                weights[bin] += 1.0 / (errors[i] * errors[j]);
            }
        }
    }
    for i in 0..nd {
        if count[i] > 0 {
            covariance[i] /= if weighted {
                weights[i]
            } else {
                count[i] as f64
            };
        }
    }
    Ok((covariance, count, zero / nzero as f64))
}
