//! TEMPO2 TKfit.C: shared nonaccelerated least squares and constraint augmentation.
// GPL-3.0-or-later; source covariance and mixed-precision chi-square boundaries.
#![allow(clippy::needless_range_loop)]
use crate::tksvd::{ld, tk_backsubstitution_svd, tk_singular_value_decomposition};
use rug::Float;
use serde::{Serialize, Serializer};

/// Fit diagnostic decimal transport. Unlike input Scalar, source results can be nonfinite.
#[derive(Clone, Debug)]
pub struct FitNumber(pub(crate) Float);
impl From<crate::Scalar> for FitNumber {
    fn from(value: crate::Scalar) -> Self {
        Self(value.0)
    }
}
impl From<f64> for FitNumber {
    fn from(value: f64) -> Self {
        let mut number = ld(value);
        // MPFR's binary64 assignment canonicalizes NaN sign; the diagnostic
        // boundary must retain the sign of the defined source double result.
        if value.is_nan() {
            number.copysign_mut(&ld(if value.is_sign_negative() { -1.0 } else { 1.0 }));
        }
        Self(number)
    }
}
impl FitNumber {
    pub fn as_float(&self) -> &Float {
        &self.0
    }
}
impl Serialize for FitNumber {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let text = if self.0.is_nan() {
            if self.0.is_sign_negative() {
                "-NaN".into()
            } else {
                "NaN".into()
            }
        } else if self.0.is_infinite() {
            if self.0 > 0 {
                "+Infinity".into()
            } else {
                "-Infinity".into()
            }
        } else {
            self.0.to_string_radix(10, None)
        };
        serializer.serialize_str(&text)
    }
}

pub(crate) fn serialize_numbers<S: Serializer>(
    values: &[f64],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(values.iter().copied().map(FitNumber::from))
}
pub(crate) fn serialize_matrix<S: Serializer>(
    values: &[Vec<f64>],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(
        values
            .iter()
            .map(|row| row.iter().copied().map(FitNumber::from).collect::<Vec<_>>()),
    )
}

#[derive(Clone, Debug, Serialize)]
pub struct LeastSquares {
    pub singular_values: Vec<FitNumber>,
    pub retained: Vec<bool>,
    pub rank: usize,
    pub dof: usize,
    pub increments: Vec<FitNumber>,
    pub covariance: Vec<Vec<FitNumber>>,
    pub errors_unscaled: Vec<FitNumber>,
    pub errors: Vec<FitNumber>,
    pub chi_square: FitNumber,
    pub error_multiplier: FitNumber,
    pub linearized_s: Vec<FitNumber>,
    pub robust_history: Vec<crate::tkrobust::Iteration>,
    pub diagnostic_files: Vec<std::path::PathBuf>,
}

/// Owned destination for TKfit's writeResiduals bit mask and debug covariance.
/// Each solve overwrites source filenames; robust fitting retains the first
/// prefit/design files and the final weighted solve's postfit files.
#[derive(Clone, Debug, Serialize)]
pub struct FitDiagnostics {
    pub directory: std::path::PathBuf,
    pub write_residuals: u8,
    pub debug: bool,
}
impl FitDiagnostics {
    pub(crate) fn write(&self, name: &str, bytes: &[u8]) -> Result<std::path::PathBuf, String> {
        let path = self.directory.join(name);
        std::fs::write(&path, bytes).map_err(|e| format!("{}: {e}", path.display()))?;
        Ok(path)
    }
}

/// The three independently optional TKfit output pointers. Unrequested vectors
/// in `LeastSquares` are empty; chi-square remains source zero without parameters.
#[derive(Clone, Copy, Debug)]
pub struct FitOutputs {
    pub parameters: bool,
    pub errors: bool,
    pub covariance: bool,
}
impl FitOutputs {
    pub const ALL: Self = Self {
        parameters: true,
        errors: true,
        covariance: true,
    };
}

pub fn tk_least_squares(
    data: &[f64],
    white_data: &[f64],
    matrix: &[Vec<f64>],
    white_matrix: &[Vec<f64>],
    rescale: bool,
) -> Result<LeastSquares, String> {
    constrained_least_squares(
        data,
        white_data,
        matrix,
        white_matrix,
        &[],
        &[],
        1e-27,
        rescale,
    )
}

/// TKrobustDefConstrainedLeastSquares nonrobust SVD branch. Constraint rows are
/// already in their source weighted fit coordinates; chi-square uses data only.
#[allow(clippy::too_many_arguments)]
pub fn constrained_least_squares(
    data: &[f64],
    white_data: &[f64],
    matrix: &[Vec<f64>],
    white_matrix: &[Vec<f64>],
    constraints: &[Vec<f64>],
    targets: &[f64],
    tolerance: f64,
    rescale: bool,
) -> Result<LeastSquares, String> {
    least_squares_outputs(
        Some(data),
        Some(white_data),
        matrix,
        white_matrix,
        constraints,
        targets,
        tolerance,
        rescale,
        FitOutputs::ALL,
        None,
    )
}

/// Shared SVD path with source optional data/parameter/error/covariance outputs.
#[allow(clippy::too_many_arguments)]
pub fn least_squares_outputs(
    data: Option<&[f64]>,
    white_data: Option<&[f64]>,
    matrix: &[Vec<f64>],
    white_matrix: &[Vec<f64>],
    constraints: &[Vec<f64>],
    targets: &[f64],
    tolerance: f64,
    rescale: bool,
    outputs: FitOutputs,
    diagnostics: Option<&FitDiagnostics>,
) -> Result<LeastSquares, String> {
    let n = matrix.len();
    let nf = matrix.first().ok_or("empty least-squares matrix")?.len();
    let compute_params = outputs.parameters && data.is_some();
    if nf == 0
        || data.is_some_and(|y| y.len() != n)
        || white_data.is_some_and(|y| y.len() != n)
        || (compute_params && white_data.is_none())
        || white_matrix.len() != n
        || targets.len() != constraints.len()
        || matrix
            .iter()
            .chain(white_matrix)
            .chain(constraints)
            .any(|r| r.len() != nf)
    {
        return Err("inconsistent least-squares dimensions".into());
    }
    let augmented_n = n + constraints.len();
    // TKfit.C returns before initializing estimates here. An owned Result cannot
    // publish that storage as a solution; retain the boundary difference explicitly.
    if nf > augmented_n {
        return Err("TKfit source boundary: n < p returns without initializing fit outputs".into());
    }
    crate::clkcorr::with_diagnostics(|d| {
        d.log(
            b'm',
            "TKfit.C",
            345,
            format!("SVD nparams={nf} nconst={} ndata={n}", constraints.len()).as_bytes(),
        );
        Ok(())
    })
    .map_err(|e| e.to_string())?;
    let mut svd = tk_singular_value_decomposition(
        white_matrix
            .iter()
            .chain(constraints)
            .map(|row| row.iter().map(|v| ld(*v)).collect())
            .collect(),
    )?;
    let singular_values: Vec<_> = svd.w.iter().cloned().map(FitNumber).collect();
    let wmax = find_max(&singular_values)?.0;
    let cutoff = ld(tolerance) * &wmax;
    for w in &mut svd.w {
        if *w < cutoff {
            *w = ld(0.0);
        }
    }
    let retained: Vec<_> = svd.w.iter().map(|w| *w != 0).collect();
    let rank = retained.iter().filter(|v| **v).count();
    let compute_covariance = outputs.covariance || outputs.errors;
    let wt: Vec<_> = svd
        .w
        .iter()
        .map(|w| if *w != 0 { ld(1.0) / w / w } else { ld(0.0) })
        .collect();
    let mut covariance = if compute_covariance {
        vec![vec![FitNumber::from(0.0); nf]; nf]
    } else {
        vec![]
    };
    for i in 0..covariance.len() {
        for j in 0..=i {
            let mut sum = ld(0.0);
            for k in 0..nf {
                sum += svd.v[i][k].clone() * &svd.v[j][k] * &wt[k];
            }
            covariance[i][j] = FitNumber::from(sum.to_f64());
            covariance[j][i] = covariance[i][j].clone();
        }
    }
    let errors_unscaled: Vec<_> = (0..if outputs.errors { nf } else { 0 })
        .map(|i| FitNumber::from(covariance[i][i].0.to_f64().sqrt()))
        .collect();
    // TKfit.C::TKrobustDefConstrainedLeastSquares:347 ignores constraint_vals
    // in the SVD branch. Nonzero prior equations require their supplied RHS,
    // as already used by the source QR branch. AE nonzero-target witness.
    let p = if compute_params {
        let augmented_data: Vec<_> = white_data.unwrap().iter().chain(targets).copied().collect();
        tk_backsubstitution_svd(&svd, &augmented_data)
    } else {
        vec![]
    };
    let increments: Vec<_> = p.iter().map(|v| FitNumber::from(v.to_f64())).collect();
    let mut chisq = 0.0;
    for j in 0..if compute_params { n } else { 0 } {
        let mut sum = ld(0.0);
        for k in 0..nf {
            sum += p[k].clone() * white_matrix[j][k];
        }
        // Source running chisq is double, but the row square and addition are longdouble.
        chisq = (ld(chisq) + (ld(white_data.unwrap()[j]) - sum).square()).to_f64();
    }
    let dof = augmented_n - nf;
    // Defined source 0/0 and infinity are retained, including square fits.
    let multiplier = if rescale {
        (chisq / dof as f64).sqrt()
    } else {
        1.0
    };
    let errors = errors_unscaled
        .iter()
        .map(|e| FitNumber::from(e.0.to_f64() * multiplier))
        .collect();
    let linearized_s = if compute_params {
        matrix
            .iter()
            .zip(data.unwrap())
            .map(|(row, y)| {
                let mut sum = 0.0;
                for j in 0..nf {
                    sum += row[j] * increments[j].0.to_f64();
                }
                FitNumber::from(y - sum)
            })
            .collect()
    } else {
        vec![]
    };
    let mut diagnostic_files = vec![];
    if let Some(io) = diagnostics {
        use crate::clock_text::{NumberFormat, format_number};
        let g = |v| format_number(v, NumberFormat::GeneralSix);
        if io.write_residuals & 1 != 0
            && let (Some(y), Some(wy)) = (data, white_data)
        {
            let mut bytes = vec![];
            for i in 0..n {
                bytes.extend(
                    [format!("{i} ").as_bytes(), &g(y[i]), b" ", &g(wy[i]), b"\n"].concat(),
                );
            }
            diagnostic_files.push(io.write("prefit.res", &bytes)?);
        }
        if io.write_residuals & 2 != 0 {
            let mut bytes = vec![];
            for i in 0..n {
                for j in 0..nf {
                    bytes.extend(
                        [
                            format!("{i} {j} ").as_bytes(),
                            &g(matrix[i][j]),
                            b" ",
                            &g(white_matrix[i][j]),
                            b"\n",
                        ]
                        .concat(),
                    );
                }
                bytes.push(b'\n');
            }
            diagnostic_files.push(io.write("design.matrix", &bytes)?);
            bytes.clear();
            for (i, row) in constraints.iter().enumerate() {
                for (j, &v) in row.iter().enumerate() {
                    bytes.extend([format!("{i} {j} ").as_bytes(), &g(v), b"\n"].concat());
                }
                bytes.push(b'\n');
            }
            diagnostic_files.push(io.write("constraints.matrix", &bytes)?);
        }
        if io.debug && compute_covariance {
            let mut bytes = vec![];
            for i in 0..nf {
                for j in 0..=i {
                    let v = covariance[i][j].0.to_f64()
                        / (covariance[i][i].0.to_f64() * covariance[j][j].0.to_f64()).sqrt();
                    bytes.extend(format_number(v, NumberFormat::SignedEight));
                    bytes.push(b' ');
                }
                bytes.push(b'\n');
            }
            diagnostic_files.push(io.write("cvm.matrix", &bytes)?);
        }
        if io.write_residuals & 4 != 0 {
            let mut bytes = vec![];
            if compute_params {
                for i in 0..n {
                    let (mut sum, mut sum_w) = (0.0, 0.0);
                    for j in 0..nf {
                        sum += matrix[i][j] * increments[j].0.to_f64();
                        sum_w += white_matrix[i][j] * increments[j].0.to_f64();
                    }
                    bytes.extend(
                        [
                            format!("{i} ").as_bytes(),
                            &g(data.unwrap()[i] - sum),
                            b" ",
                            &g(white_data.unwrap()[i] - sum_w),
                            b"\n",
                        ]
                        .concat(),
                    );
                }
            } else if outputs.parameters && data.is_none() {
                // TKfit tests outP rather than computeParam here, then reads
                // unwritten estimates/null data. No defined file exists.
                return Err("TKfit postfit diagnostics require computed parameters".into());
            }
            diagnostic_files.push(io.write("postfit.res", &bytes)?);
            if compute_params {
                bytes.clear();
                for v in &increments {
                    bytes.extend(g(v.0.to_f64()));
                    bytes.push(b'\n');
                }
                diagnostic_files.push(io.write("param.vals", &bytes)?);
            }
            if compute_covariance {
                bytes.clear();
                for row in &covariance {
                    for v in row {
                        bytes.extend(g(v.0.to_f64()));
                        bytes.push(b' ');
                    }
                    bytes.push(b'\n');
                }
                diagnostic_files.push(io.write("cov.matrix", &bytes)?);
            }
        }
    }
    if !outputs.covariance {
        covariance.clear();
    }
    Ok(LeastSquares {
        singular_values,
        retained,
        rank,
        dof,
        increments,
        covariance,
        errors_unscaled,
        errors,
        chi_square: chisq.into(),
        error_multiplier: multiplier.into(),
        linearized_s,
        robust_history: vec![],
        diagnostic_files,
    })
}

/// TKfindMax_Ld: retain the first value on ties and unordered comparisons.
pub fn find_max(values: &[FitNumber]) -> Result<FitNumber, String> {
    let mut result = values
        .first()
        .ok_or("TKfindMax_Ld requires a value")?
        .clone();
    for value in values {
        if value.0 > result.0 {
            result = value.clone();
        }
    }
    Ok(result)
}

/// TKfitPoly: powers begin at one and multiply in ascending order.
pub fn polynomial_basis(x: f64, out: &mut [f64]) {
    let mut t = 1.0;
    for v in out {
        *v = t;
        t *= x;
    }
}
/// TKleastSquares_svd / TKleastSquares_svd_noErr non-pulsar adapter.
/// Source weight==1 uses sigma only when supplied; its fixed cutoff is 1e-10.
pub fn fit_basis(
    x: &[f64],
    y: &[f64],
    sigma: Option<&[f64]>,
    weight: i32,
    nparams: usize,
    mut basis: impl FnMut(f64, &mut [f64]) -> Result<(), String>,
) -> Result<LeastSquares, String> {
    if x.len() != y.len() || sigma.is_some_and(|s| s.len() != x.len()) {
        return Err("non-pulsar fit dimensions".into());
    }
    let mut matrix = Vec::with_capacity(x.len());
    for &x in x {
        let mut row = vec![0.0; nparams];
        basis(x, &mut row)?;
        matrix.push(row);
    }
    let mut white_matrix = matrix.clone();
    let mut white_data = y.to_vec();
    if weight == 1
        && let Some(sigma) = sigma
    {
        for i in 0..x.len() {
            white_data[i] /= sigma[i];
            for v in &mut white_matrix[i] {
                *v /= sigma[i];
            }
        }
    }
    constrained_least_squares(
        y,
        &white_data,
        &matrix,
        &white_matrix,
        &[],
        &[],
        1e-10,
        true,
    )
}
pub fn fit_polynomial(x: &[f64], y: &[f64], nparams: usize) -> Result<LeastSquares, String> {
    fit_basis(x, y, None, 0, nparams, |x, row| {
        polynomial_basis(x, row);
        Ok(())
    })
}
/// TKremovePoly_d, retaining sequential subtraction of each fitted term.
pub fn remove_polynomial(x: &[f64], y: &mut [f64], nparams: usize) -> Result<LeastSquares, String> {
    let fit = fit_polynomial(x, y, nparams)?;
    let mut row = vec![0.0; nparams];
    for (i, &x) in x.iter().enumerate() {
        polynomial_basis(x, &mut row);
        for (j, &v) in row.iter().enumerate() {
            y[i] -= v * fit.increments[j].0.to_f64();
        }
    }
    Ok(fit)
}
/// TKremovePoly_f narrows the updated ordinate after every term, not only at end.
pub fn remove_polynomial_f32(
    x: &[f32],
    y: &mut [f32],
    nparams: usize,
) -> Result<LeastSquares, String> {
    let xd: Vec<_> = x.iter().map(|v| *v as f64).collect();
    let yd: Vec<_> = y.iter().map(|v| *v as f64).collect();
    let fit = fit_polynomial(&xd, &yd, nparams)?;
    let mut row = vec![0.0; nparams];
    for (i, &x) in xd.iter().enumerate() {
        polynomial_basis(x, &mut row);
        for (j, &v) in row.iter().enumerate() {
            y[i] = (y[i] as f64 - v * fit.increments[j].0.to_f64()) as f32;
        }
    }
    Ok(fit)
}
