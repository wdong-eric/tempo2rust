//! Selected t2fit.C::t2Fit/t2Fit_getFitData/t2Fit_fillFitInfo/
//! t2Fit_buildDesignMatrix/t2Fit_updateParameters for local ZERO/F0-F3 fitting;
//! tempo2.C::main supplies the NITS lifecycle. Other fit paths are outside this port.
//! Rust-only owned results/JSON expose source fit coordinates separately from model values.
use crate::{
    Corrections, EngineProvenance, FitControls, FitNumber, LeastSquares, Resource, RuntimeContext,
    Scalar, TerrestrialResult, TimingError, TimingInput, TimingResult, ToaResult,
    evaluate_barycentric, evaluate_terrestrial,
    std_freq::{SPIN, spin_errors, spin_values, std_freq, update_std_freq},
    tkfit::tk_least_squares,
    tksvd::ld,
};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize)]
pub struct FitOptions {
    pub rescale_err_chisq: bool,
}
impl Default for FitOptions {
    fn default() -> Self {
        Self {
            rescale_err_chisq: true,
        }
    }
}

/// Raw observations retain the original fixed-kernel residual; centered_s is fit data.
#[derive(Debug, Serialize)]
pub struct FitTiming {
    pub raw_residual_semantics: &'static str,
    pub reference_phase_turn: Scalar,
    pub mean_s: Scalar,
    pub observations: Vec<ToaResult>,
    pub centered_s: Vec<Scalar>,
    pub corrections: Vec<Corrections>,
    pub runtime_root: Option<PathBuf>,
    pub resources: Vec<Resource>,
    pub warnings: Vec<String>,
    pub retained_numerics: Vec<&'static str>,
}
impl FitTiming {
    fn from_timing(timing: TimingResult) -> Self {
        let mut mean = ld(0.0);
        for o in &timing.observations {
            mean += &o.residual_s.0;
        }
        mean /= timing.observations.len();
        let centered_s = timing
            .observations
            .iter()
            .map(|o| Scalar(o.residual_s.0.clone() - &mean))
            .collect();
        Self {
            raw_residual_semantics: "fixed-model raw diagnostic before unweighted mean removal",
            reference_phase_turn: timing.reference_phase_turn,
            mean_s: Scalar(mean),
            observations: timing.observations,
            centered_s,
            corrections: Vec::new(),
            runtime_root: None,
            resources: Vec::new(),
            warnings: Vec::new(),
            retained_numerics: Vec::new(),
        }
    }
    fn from_terrestrial(result: TerrestrialResult) -> Self {
        let mut timing = Self::from_timing(result.timing);
        timing.corrections = result.corrections;
        timing.runtime_root = Some(result.runtime_root);
        timing.resources = result.resources;
        timing.warnings = result.warnings;
        timing.retained_numerics = result.retained_numerics.to_vec();
        timing
    }
}

#[derive(Debug, Serialize)]
pub struct FitCycle {
    pub iteration: usize,
    pub before_values: [Scalar; 4],
    pub before_errors: [FitNumber; 4],
    pub after_values: [Scalar; 4],
    pub after_errors: [FitNumber; 4],
    pub prefit: FitTiming,
    pub postfit: FitTiming,
    pub row_indices: Vec<usize>,
    pub x_days: Vec<FitNumber>,
    pub sigma_s: Vec<FitNumber>,
    pub data_s: Vec<FitNumber>,
    pub white_data: Vec<FitNumber>,
    pub matrix: Vec<Vec<FitNumber>>,
    pub white_matrix: Vec<Vec<FitNumber>>,
    pub solver: LeastSquares,
    pub offset_s: FitNumber,
    /// None means the source never initialized offset_e; it is not a NaN encoding.
    pub offset_error_s: Option<FitNumber>,
    pub offset_error_state: &'static str,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct FitResult {
    pub schema_version: &'static str,
    pub status: &'static str,
    pub engine: EngineProvenance,
    pub input: TimingInput,
    pub controls: FitControls,
    pub mode: u8,
    pub nits: usize,
    pub rescale_requested: bool,
    pub rescale_effective: bool,
    pub remove_mean: bool,
    pub fit_performed: bool,
    pub columns: Vec<String>,
    pub column_scales: Vec<&'static str>,
    pub physical_parameter_order: [&'static str; 4],
    pub covariance_coordinates: &'static str,
    pub chi_square_units: &'static str,
    pub cycles: Vec<FitCycle>,
}

pub fn fit_barycentric(
    input: &TimingInput,
    options: &FitOptions,
) -> Result<FitResult, TimingError> {
    t2_fit(input, options, |model| {
        evaluate_barycentric(model).map(FitTiming::from_timing)
    })
}

pub fn fit_terrestrial(
    input: &TimingInput,
    runtime: &mut RuntimeContext,
    options: &FitOptions,
) -> Result<FitResult, TimingError> {
    t2_fit(input, options, |model| {
        evaluate_terrestrial(model, runtime).map(FitTiming::from_terrestrial)
    })
}

fn t2_fit(
    input: &TimingInput,
    options: &FitOptions,
    mut evaluate: impl FnMut(&TimingInput) -> Result<FitTiming, TimingError>,
) -> Result<FitResult, TimingError> {
    for (key, meta) in &input.parameter_metadata {
        let param = input.parameter_cell(key);
        if param.fit_flag == 2 || (param.fit_flag != 0 && !SPIN.contains(&key.as_str())) {
            return Err(TimingError::new(
                &input.par.path,
                meta.original.as_ref().map(|p| p.line),
                key,
                "M2 fits local F0-F3 only; global or other active fit flags are unsupported",
            ));
        }
    }
    let selected: Vec<_> = (0..4)
        .filter(|&k| {
            input.parameter_cell(SPIN[k]).param_set != 0
                && input.parameter_cell(SPIN[k]).fit_flag == 1
        })
        .collect();
    let columns: Vec<_> = std::iter::once("ZERO".to_string())
        .chain(selected.iter().map(|&k| SPIN[k].to_string()))
        .collect();
    let column_scales = std::iter::once("seconds")
        .chain(selected.iter().map(|&k| {
            [
                "x_days * 24 * 3600 / F0",
                "0.5 * x_days * x_days",
                "(1/6) * x_days * x_days * x_days / 1e9",
                "(1/24) * x_days / 1e18 * x_days * x_days * x_days",
            ][k]
        }))
        .collect();
    let mut model = input.clone();
    // The frontend leaves source err initialized; callbacks update those same cells.
    let mut offset_error = None;
    let controls = input.fit_controls();
    let rescale = controls.mode == 0 || options.rescale_err_chisq;
    let mut cycles = Vec::new();
    let mut nonfinite = false;
    for iteration in 1..=controls.nits {
        let before_values = spin_values(&model).map(Scalar);
        let before_errors = spin_errors(&model);
        let prefit = evaluate(&model)?;
        let x: Vec<_> = prefit
            .observations
            .iter()
            .map(|o| (o.bbat_mjd.0.clone() - &model.parameter_cell("PEPOCH").val).to_f64())
            .collect();
        let data: Vec<_> = prefit.centered_s.iter().map(|v| v.0.to_f64()).collect();
        let sigma: Vec<_> = input
            .source_observations()
            .iter()
            .map(|o| crate::observation::read(&o.toaErr, "toaErr").map(|v| *v * 1e-6))
            .collect::<Result<_, _>>()?;
        let f0 = model.parameter_cell("F0").val.to_f64();
        let matrix: Vec<Vec<f64>> = x
            .iter()
            .map(|&x| {
                std::iter::once(1.0)
                    .chain(selected.iter().map(|&k| std_freq(x, f0, k)))
                    .collect()
            })
            .collect();
        for (i, row) in matrix.iter().enumerate() {
            // t2Fit_buildDesignMatrix exits on NaN. Keep its stage/predicate, returning Err.
            if row.iter().any(|v| v.is_nan()) {
                return Err(TimingError::new(
                    &input.tim.path,
                    Some(input.observation_metadata[i].line),
                    "design matrix",
                    format!("cycle {iteration}: source exits on NaN matrix element"),
                ));
            }
        }
        let white_data: Vec<_> = data
            .iter()
            .zip(&sigma)
            .map(|(&y, &s)| y / if controls.mode == 0 { 1.0 } else { s })
            .collect();
        let white_matrix: Vec<Vec<_>> = matrix
            .iter()
            .zip(&sigma)
            .map(|(row, &s)| {
                row.iter()
                    .map(|&v| v / if controls.mode == 0 { 1.0 } else { s })
                    .collect()
            })
            .collect();
        let solver = tk_least_squares(&data, &white_data, &matrix, &white_matrix, rescale)
            .map_err(|e| {
                TimingError::new(
                    &input.tim.path,
                    None,
                    "fit",
                    format!("cycle {iteration}: {e}"),
                )
            })?;
        let offset = solver.increments[0].clone();
        if solver.errors[0].0.to_f64() >= 0.0 {
            offset_error = Some(solver.errors[0].clone());
        }
        for (j, &k) in selected.iter().enumerate() {
            update_std_freq(
                &mut model,
                k,
                solver.increments[j + 1].0.to_f64(),
                solver.errors[j + 1].0.to_f64(),
            );
        }
        let mut warnings = Vec::new();
        // TKfit.C warns above 2^(sizeof(source longdouble)*8-17), i.e. 2^111.
        if solver
            .singular_values
            .iter()
            .any(|w| w.0 > 2.0f64.powi(111))
        {
            warnings.push(
                "source warning: wmax very large; precision issues likely to break fit".into(),
            );
        }
        if solver.rank < columns.len() {
            warnings.push(format!("SVD retained {} of {} modes; zero variance in discarded modes is a pseudoinverse artifact",solver.rank,columns.len()));
        }
        if solver.errors.iter().any(|e| !e.0.is_finite()) {
            nonfinite = true;
            warnings.push("source nonfinite fit errors preserved; error >= 0 callbacks may retain previous physical errors".into());
        }
        if offset_error.is_none() {
            warnings.push(
                "source offset_e remains uninitialized; no numerical offset error is exported"
                    .into(),
            );
        }
        // ZERO is stored, not subtracted by formResiduals. Reevaluate exactly that model.
        let postfit = evaluate(&model)?;
        cycles.push(FitCycle {
            iteration,
            before_values,
            before_errors,
            after_values: spin_values(&model).map(Scalar),
            after_errors: spin_errors(&model),
            row_indices: input.observation_metadata.iter().map(|o| o.index).collect(),
            prefit,
            postfit,
            x_days: x.into_iter().map(FitNumber::from).collect(),
            sigma_s: sigma.into_iter().map(FitNumber::from).collect(),
            data_s: data.into_iter().map(FitNumber::from).collect(),
            white_data: white_data.into_iter().map(FitNumber::from).collect(),
            matrix: matrix
                .into_iter()
                .map(|r| r.into_iter().map(FitNumber::from).collect())
                .collect(),
            white_matrix: white_matrix
                .into_iter()
                .map(|r| r.into_iter().map(FitNumber::from).collect())
                .collect(),
            solver,
            offset_s: offset,
            offset_error_s: offset_error.clone(),
            offset_error_state: if offset_error.is_some() {
                "initialized by source callback"
            } else {
                "undefined source storage; no value"
            },
            warnings,
        });
    }
    Ok(FitResult {
        schema_version: "tempo2-fit-v1",
        status: if nonfinite {
            "completed_with_source_nonfinite"
        } else {
            "completed"
        },
        engine: EngineProvenance {
            name: "tempo2-rust",
            version: env!("CARGO_PKG_VERSION"),
            numeric_backend: "Rug 1.30.0 / MPFR; native reference long double and TKsign_d/TKretMax_d are binary64",
            significand_bits: crate::PRECISION_BITS,
            arithmetic_rounding: "nearest ties to even; nonfused source operations; T2_SVD_TOL binary64 1e-27",
            support_profile: "M2 isolated TCB @ / PKS, ZERO and local F0-F3",
            source_correspondence: "TEMPO2 fda6ad4a9753ad7dc5d291406f2f310a9161423f; A2 7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572; t2fit.C / TKfit.C / TKsvd.C / t2fit_stdFitFuncs.C / tempo2.C",
        },
        input: input.clone(),
        controls: controls.clone(),
        mode: controls.mode,
        nits: controls.nits,
        rescale_requested: options.rescale_err_chisq,
        rescale_effective: rescale,
        remove_mean: true,
        fit_performed: true,
        columns,
        column_scales,
        physical_parameter_order: SPIN,
        covariance_coordinates: "raw unscaled source fit coordinates (not physical spin covariance)",
        chi_square_units: if controls.mode == 0 {
            "seconds squared (unweighted)"
        } else {
            "dimensionless (weighted)"
        },
        cycles,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn later_cycle_error_publishes_no_partial_fit() {
        let root =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../validation/fitting/inputs");
        let input = crate::load(&root.join("all.par"), &root.join("bary.tim")).unwrap();
        let before = serde_json::to_vec(&input).unwrap();
        let mut calls = 0;
        let error = t2_fit(&input, &FitOptions::default(), |model| {
            calls += 1;
            if calls == 4 {
                return Err(TimingError::new(
                    &input.tim.path,
                    None,
                    "test evaluation",
                    "injected second-cycle postfit failure",
                ));
            }
            evaluate_barycentric(model).map(FitTiming::from_timing)
        })
        .unwrap_err();
        assert_eq!(calls, 4);
        assert!(error.message.contains("second-cycle"));
        assert_eq!(before, serde_json::to_vec(&input).unwrap());
        assert_eq!(
            fit_barycentric(&input, &FitOptions::default())
                .unwrap()
                .cycles
                .len(),
            2
        );
    }
}
