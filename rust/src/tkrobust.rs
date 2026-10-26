//! TKrobust.C; source Huber scale iteration and H/B/W final reweighting.
use crate::tkfit::{FitDiagnostics, FitNumber, FitOutputs, LeastSquares, least_squares_outputs};
use serde::Serialize;

/// TKrobust's ordered scale solves followed by its final weighted solve.
#[derive(Clone, Debug, Serialize)]
pub struct Iteration {
    pub iteration: usize,
    pub phase: &'static str,
    pub scale: FitNumber,
    pub next_scale: Option<FitNumber>,
    pub phi: Option<FitNumber>,
    pub dphi: Option<FitNumber>,
    #[serde(serialize_with = "crate::tkfit::serialize_numbers")]
    pub white_data: Vec<f64>,
    #[serde(serialize_with = "crate::tkfit::serialize_matrix")]
    pub white_matrix: Vec<Vec<f64>>,
    #[serde(serialize_with = "crate::tkfit::serialize_numbers")]
    pub residual: Vec<f64>,
    pub solver: LeastSquares,
}

/// TKrobustLeastSquares convenience wrapper. Pinned TKfit.C discarded `kind`
/// by forwarding 0; check_solver.py demonstrates the outlier/wrapper mismatch.
#[allow(clippy::too_many_arguments)]
pub fn least_squares(
    data: &[f64],
    white_data: &[f64],
    matrix: &[Vec<f64>],
    white_matrix: &[Vec<f64>],
    tolerance: f64,
    rescale: bool,
    kind: u8,
) -> Result<LeastSquares, String> {
    robust_least_squares(
        data,
        white_data,
        matrix,
        white_matrix,
        &[],
        &[],
        tolerance,
        rescale,
        kind,
        None,
    )
}

/// Reweights residuals in place, as in TKrobust_reweight_{huber,bisquare,welsch}.
pub fn reweight(
    resid: &mut [f64],
    data: &mut [f64],
    matrix: &mut [Vec<f64>],
    scale: f64,
    kind: u8,
) -> (f64, f64) {
    let (mut phi, mut dphi) = (0.0, 0.0);
    for ((r, y), row) in resid.iter_mut().zip(data).zip(matrix) {
        *r /= scale;
        let weight = match kind {
            b'B' => {
                if r.abs() < 4.685 {
                    let q = (*r / 4.685).powf(2.0);
                    phi += (*r * (1.0 - q).powf(2.0)).powf(2.0);
                    dphi += (1.0 - 5.0 * q) * (1.0 - q).powf(2.0);
                    (1.0 - q).powf(2.0)
                } else {
                    0.0
                }
            }
            b'W' => {
                let q = (*r / 2.11).powf(2.0);
                phi += (2.0 * 2.11 * *r * (-q).exp()).powf(2.0);
                dphi += 2.0 * 2.11 * (1.0 - 2.0 * q) * (-q).exp();
                2.0 * 2.11 * (-r.powf(2.0)).exp()
            }
            _ => {
                if r.abs() < 1.345 {
                    phi += *r * *r;
                    dphi += 1.0;
                    1.0
                } else {
                    phi += 1.345 * 1.345;
                    1.345 / r.abs()
                }
            }
        };
        *y *= weight;
        for a in row {
            *a *= weight;
        }
    }
    (phi, dphi)
}

#[allow(clippy::too_many_arguments)]
pub fn robust_least_squares(
    data: &[f64],
    white_data: &[f64],
    matrix: &[Vec<f64>],
    white_matrix: &[Vec<f64>],
    constraints: &[Vec<f64>],
    targets: &[f64],
    tolerance: f64,
    rescale: bool,
    kind: u8,
    diagnostics: Option<&FitDiagnostics>,
) -> Result<LeastSquares, String> {
    if kind <= 48 {
        return least_squares_outputs(
            Some(data),
            Some(white_data),
            matrix,
            white_matrix,
            constraints,
            targets,
            tolerance,
            rescale,
            FitOutputs::ALL,
            diagnostics,
        );
    }
    if !matches!(kind, b'H' | b'B' | b'W') {
        crate::clkcorr::with_process(|s| {
            let _: () = s.diagnostics.log(
                b'e',
                "TKrobust.C",
                113,
                format!(
                    "Robust type '{}' not supported. (fall back to huber!)",
                    kind as char
                )
                .as_bytes(),
            );
            Ok(())
        })
        .map_err(|e| e.to_string())?;
    }
    let mut sigma: f64 = 1.0;
    let mut old: f64 = 0.0;
    let mut count = 0;
    let mut resid = vec![0.0; data.len()];
    let mut first_covariance = None;
    let mut last_covariance = vec![];
    let mut history = vec![];
    let mut io = diagnostics.cloned();
    let mut diagnostic_files = vec![];
    while (old - sigma).abs() > 1e-5 {
        let mut y = white_data.to_vec();
        let mut m = white_matrix.to_vec();
        let weights = (count > 0).then(|| reweight(&mut resid, &mut y, &mut m, sigma, b'H'));
        let fit = least_squares_outputs(
            Some(data),
            Some(&y),
            matrix,
            &m,
            constraints,
            targets,
            tolerance,
            rescale,
            FitOutputs::ALL,
            io.as_ref(),
        )?;
        if count == 0 {
            first_covariance = Some(fit.covariance.clone());
            diagnostic_files = fit.diagnostic_files.clone();
            if let Some(io) = &mut io {
                io.write_residuals &= 4;
            }
        }
        for (i, row) in white_matrix.iter().enumerate() {
            let mut sum = 0.0;
            for (v, p) in row.iter().zip(&fit.increments) {
                sum += p.as_float().to_f64() * v;
            }
            resid[i] = white_data[i] - sum;
        }
        last_covariance = fit.covariance.clone();
        let abs: Vec<_> = resid.iter().map(|r| r.abs()).collect();
        let median = crate::toolkit::find_median_d(&abs).map_err(|e| e.to_string())?;
        old = sigma;
        sigma = median / 0.6745;
        history.push(Iteration {
            iteration: count,
            phase: "scale",
            scale: old.into(),
            next_scale: Some(sigma.into()),
            phi: weights.map(|w| w.0.into()),
            dphi: weights.map(|w| w.1.into()),
            white_data: y,
            white_matrix: m,
            residual: resid.clone(),
            solver: fit,
        });
        count += 1;
        if count > 16 {
            crate::clkcorr::with_process(|s| {
                let _: () = s.diagnostics.log(
                    b'w',
                    "TKrobust.C",
                    84,
                    b"Warning, robust fit did not converge after 16 iterations, aborting...",
                );
                Ok(())
            })
            .map_err(|e| e.to_string())?;
            break;
        }
    }
    let mut y = white_data.to_vec();
    let mut m = white_matrix.to_vec();
    let (phi, dphi) = reweight(&mut resid, &mut y, &mut m, sigma, kind);
    let mut fit = least_squares_outputs(
        Some(data),
        Some(&y),
        matrix,
        &m,
        constraints,
        targets,
        tolerance,
        rescale,
        FitOutputs::ALL,
        io.as_ref(),
    )?;
    history.push(Iteration {
        iteration: count,
        phase: "final",
        scale: sigma.into(),
        next_scale: None,
        phi: Some(phi.into()),
        dphi: Some(dphi.into()),
        white_data: y,
        white_matrix: m,
        residual: resid,
        solver: fit.clone(),
    });
    let n = data.len() as f64;
    let chisq = sigma * sigma * n * n * phi / (dphi * dphi);
    let ecorrect = if rescale { chisq / fit.dof as f64 } else { 1.0 };
    // TKrobust.C:131-137 deliberately returns the first covariance and errors
    // from the last scale-iteration covariance, not the final weighted solve.
    fit.covariance = first_covariance.ok_or("robust scale loop produced no covariance")?;
    fit.errors = last_covariance
        .iter()
        .enumerate()
        .map(|(i, row)| FitNumber::from((row[i].as_float().to_f64() * ecorrect).sqrt()))
        .collect();
    fit.chi_square = chisq.into();
    fit.error_multiplier = ecorrect.sqrt().into();
    fit.robust_history = history;
    fit.diagnostic_files = diagnostic_files;
    Ok(fit)
}
