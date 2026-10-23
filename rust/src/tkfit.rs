//! TEMPO2 TKfit.C: nonaccelerated TKleastSquares with no constraints/robust fit.
// GPL-3.0-or-later; source covariance and mixed-precision chi-square boundaries.
#![allow(clippy::needless_range_loop)]
use crate::tksvd::{ld, tk_backsubstitution_svd, tk_singular_value_decomposition};
use rug::Float;
use serde::{Serialize, Serializer};

/// Fit diagnostic decimal transport. Unlike input Scalar, source results can be nonfinite.
#[derive(Clone, Debug)]
pub struct FitNumber(pub(crate) Float);
impl From<f64> for FitNumber {
    fn from(value: f64) -> Self {
        Self(ld(value))
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
            "NaN".into()
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

#[derive(Debug, Serialize)]
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
}

pub(crate) fn tk_least_squares(
    data: &[f64],
    white_data: &[f64],
    matrix: &[Vec<f64>],
    white_matrix: &[Vec<f64>],
    rescale: bool,
) -> Result<LeastSquares, String> {
    let n = matrix.len();
    let nf = matrix[0].len();
    // TKfit.C returns before initializing estimates here. An owned Result cannot
    // publish that storage as a solution; retain the boundary difference explicitly.
    if nf > n {
        return Err("TKfit source boundary: n < p returns without initializing fit outputs".into());
    }
    let mut svd = tk_singular_value_decomposition(
        white_matrix
            .iter()
            .map(|row| row.iter().map(|v| ld(*v)).collect())
            .collect(),
    )?;
    let singular_values = svd.w.iter().cloned().map(FitNumber).collect();
    let mut wmax = svd.w[0].clone();
    for w in &svd.w {
        if w > &wmax {
            wmax = w.clone();
        }
    }
    let cutoff = ld(1e-27) * &wmax;
    for w in &mut svd.w {
        if *w < cutoff {
            *w = ld(0.0);
        }
    }
    let retained: Vec<_> = svd.w.iter().map(|w| *w != 0).collect();
    let rank = retained.iter().filter(|v| **v).count();
    let wt: Vec<_> = svd
        .w
        .iter()
        .map(|w| if *w != 0 { ld(1.0) / w / w } else { ld(0.0) })
        .collect();
    let mut covariance = vec![vec![FitNumber::from(0.0); nf]; nf];
    for i in 0..nf {
        for j in 0..=i {
            let mut sum = ld(0.0);
            for k in 0..nf {
                sum += svd.v[i][k].clone() * &svd.v[j][k] * &wt[k];
            }
            covariance[i][j] = FitNumber::from(sum.to_f64());
            covariance[j][i] = covariance[i][j].clone();
        }
    }
    let errors_unscaled: Vec<_> = (0..nf)
        .map(|i| FitNumber::from(covariance[i][i].0.to_f64().sqrt()))
        .collect();
    let p = tk_backsubstitution_svd(&svd, white_data);
    let increments: Vec<_> = p.iter().map(|v| FitNumber::from(v.to_f64())).collect();
    let mut chisq = 0.0;
    for j in 0..n {
        let mut sum = ld(0.0);
        for k in 0..nf {
            sum += p[k].clone() * white_matrix[j][k];
        }
        // Source running chisq is double, but the row square and addition are longdouble.
        chisq = (ld(chisq) + (ld(white_data[j]) - sum).square()).to_f64();
    }
    let dof = n - nf;
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
    let linearized_s = matrix
        .iter()
        .zip(data)
        .map(|(row, y)| {
            let mut sum = 0.0;
            for j in 0..nf {
                sum += row[j] * increments[j].0.to_f64();
            }
            FitNumber::from(y - sum)
        })
        .collect();
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
    })
}
