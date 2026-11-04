//! Display projection of the canonical Rust timing workflow; no timing model here.
use std::{fs, os::unix::ffi::OsStringExt, path::Path};
use tempo2_rust::{
    RuntimeContext, TimingError, bats, core_output, execution, general_fitting, timing,
};

pub struct Session {
    input: bats::BatInput,
    runtime: RuntimeContext,
    evaluated: timing::TimingResult,
    fit: Option<general_fitting::FitResult>,
}

/// Same load/preprocess/evaluate sequence as `tempo2-rust timing` with mean removal.
/// One outer admission keeps process diagnostics with their owning workflow.
pub fn load(par: &Path, tim: &Path, runtime: &Path) -> Result<Session, TimingError> {
    execution::with_workflow(|| {
        let mut resources = RuntimeContext::from_root(runtime)?;
        let mut input = bats::load_pairs(
            &[bats::InputPair {
                par: par.into(),
                tim: tim.into(),
            }],
            runtime,
        )?;
        bats::preprocess(&mut input, &[], runtime)?;
        let evaluated = timing::evaluate(&mut input, &mut resources, true)?;
        Ok(Session {
            input,
            runtime: resources,
            evaluated,
            fit: None,
        })
    })
}

impl Session {
    pub fn timing(&self) -> &timing::TimingResult {
        self.fit
            .as_ref()
            .and_then(|fit| fit.cycles.last())
            .map_or(&self.evaluated, |cycle| &cycle.postfit)
    }

    pub fn fit_result(&self) -> Option<&general_fitting::FitResult> {
        self.fit.as_ref()
    }

    /// Use the engine's owner-aware clone so failed updates cannot publish a partial model.
    fn input_snapshot(&self) -> bats::BatInput {
        bats::BatInput {
            context: self.input.context.clone(),
            pairs: self.input.pairs.clone(),
            inputs: self.input.inputs.clone(),
        }
    }

    pub fn set_excluded(&mut self, input_index: usize, excluded: bool) -> Result<(), String> {
        execution::with_workflow(|| {
            let mut input = self.input_snapshot();
            let (pulsar, rows) = input
                .context
                .pulsar_observations_mut(0)
                .map_err(|e| e.to_string())?;
            let count = pulsar.nobs.expect("loaded observation count") as usize;
            let row = rows[..count]
                .iter_mut()
                .find(|row| row.input_index == Some(input_index))
                .ok_or_else(|| format!("Input TOA {} is no longer present", input_index + 1))?;
            // plk_plug.C::deletePoint uses 1/0, identified through the original input row.
            row.deleted = Some(i32::from(excluded));
            let evaluated =
                timing::evaluate(&mut input, &mut self.runtime, true).map_err(|e| e.to_string())?;
            self.input = input;
            self.evaluated = evaluated;
            self.fit = None;
            Ok(())
        })
    }

    pub fn refit(&mut self) -> Result<(), TimingError> {
        execution::with_workflow(|| {
            let mut input = self.input_snapshot();
            let fit = general_fitting::fit(
                &mut input,
                &mut self.runtime,
                &general_fitting::FitOptions::default(),
            )?;
            self.input = input;
            self.fit = Some(fit);
            Ok(())
        })
    }

    pub fn can_save(&self) -> bool {
        self.fit.as_ref().is_some_and(|fit| !fit.cycles.is_empty())
    }

    /// Export through source writers on a snapshot: textOutput derives TZR/report fields.
    /// A fresh directory prevents overwrites and keeps any partial output on failure.
    pub fn save(&self, directory: &Path) -> Result<(), String> {
        if !self.can_save() {
            return Err("Refit the current selection before saving".into());
        }
        fs::create_dir(directory)
            .map_err(|e| format!("Cannot create {}: {e}", directory.display()))?;
        let result: Result<(), Box<dyn std::error::Error>> = execution::with_workflow(|| {
            let mut context = self.input.context.clone();
            let (mut stdout, mut stderr) = (vec![], vec![]);
            let written = core_output::text_output(
                &mut context,
                &core_output::OutputOptions {
                    new_par: Some("session.par".into()),
                    ..Default::default()
                },
                &mut core_output::OutputIo {
                    stdout: &mut stdout,
                    stderr: &mut stderr,
                    stdin: &mut std::io::empty(),
                    directory,
                },
            );
            fs::write(directory.join("engine.stdout"), stdout)?;
            fs::write(directory.join("engine.stderr"), stderr)?;
            written?;
            // The canonical PAR writer reports a failed open without returning Err.
            fs::metadata(directory.join("session.par"))?;
            let runtime = std::ffi::OsString::from_vec(self.timing().runtime_root.bytes.clone());
            tempo2_rust::source_tim::write_tim(
                &directory.join("session.tim"),
                &mut context,
                0,
                b"tempo2",
                Path::new(&runtime),
            )?;
            fs::metadata(directory.join("session.tim"))?;
            let mut bytes = serde_json::to_vec_pretty(
                &serde_json::json!({"timing": self.timing(), "fit": self.fit_result()}),
            )?;
            bytes.push(b'\n');
            fs::write(directory.join("session.json"), bytes)?;
            Ok(())
        });
        result.map_err(|e| {
            format!(
                "Save incomplete at {}; partial files remain: {e}",
                directory.display()
            )
        })
    }
}

#[derive(Debug)]
pub struct PlotToa {
    /// Position in the current result, never a replacement for the original input identity.
    pub observation: usize,
    pub sat_mjd: f64,
    pub residual_us: f64,
    pub uncertainty_us: f64,
    pub excluded: bool,
}

/// Binary64 is used only for screen coordinates. Exact Number values remain in the result.
pub fn plot_toas(pulsar: &timing::TimingPulsar) -> Result<Vec<PlotToa>, String> {
    pulsar
        .observations
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let sat_mjd = display_number(&row.barycentric.sat_mjd)?;
            let residual_us = display_number(&row.residual_s)? * 1e6;
            let uncertainty = &row.barycentric.uncertainty;
            if uncertainty.unit != "us" {
                return Err(format!(
                    "TOA {index}: unexpected uncertainty unit {}",
                    uncertainty.unit
                ));
            }
            let uncertainty_us = display_number(
                uncertainty
                    .value
                    .as_ref()
                    .ok_or_else(|| format!("TOA {index}: missing uncertainty"))?,
            )?;
            if !residual_us.is_finite() || uncertainty_us < 0.0 {
                return Err(format!(
                    "TOA {index}: residual or uncertainty cannot be plotted"
                ));
            }
            Ok(PlotToa {
                observation: index,
                sat_mjd,
                residual_us,
                uncertainty_us,
                excluded: row.barycentric.deleted != 0,
            })
        })
        .collect()
}

fn display_number(value: &bats::Number) -> Result<f64, String> {
    match value {
        bats::Number::Finite(text) => {
            let number = text.parse::<f64>().map_err(|e| e.to_string())?;
            if number.is_finite() {
                Ok(number)
            } else {
                Err(format!("Value outside finite display range: {text}"))
            }
        }
        bats::Number::Nonfinite { class } => Err(format!("Nonfinite display value: {class}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_display_values_are_reported_instead_of_dropped() {
        assert!(display_number(&bats::Number::Nonfinite { class: "-nan" }).is_err());
        assert!(display_number(&bats::Number::Finite("1e400".into())).is_err());
        assert!(
            display_number(&bats::Number::Finite("-0".into()))
                .unwrap()
                .is_sign_negative()
        );
    }

    #[test]
    #[ignore = "requires pinned reference runtime"]
    fn failed_fit_keeps_the_current_model_and_result() {
        use tempo2_rust::tempo2::Label::{param_finish, param_start};
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let mut session = load(
            &root.join("validation/fitting/inputs/f0.par"),
            &root.join("validation/fitting/inputs/bary_unsorted.tim"),
            &root.join("validation/reference/build/source/T2runtime"),
        )
        .unwrap();
        let indices: Vec<_> = session.timing().pulsars[0]
            .observations
            .iter()
            .map(|r| r.input_index.unwrap())
            .collect();
        for index in indices {
            session.set_excluded(index, true).unwrap();
        }
        let result = serde_json::to_vec(session.timing()).unwrap();
        let state = |session: &Session| {
            let pulsar = &session.input.context.pulsars()[0];
            (
                [param_start, param_finish].map(|label| {
                    let cell = &pulsar.param.slots()[label as usize].cell(0).unwrap();
                    (cell.value().clone(), cell.param_set(), cell.fit_flag())
                }),
                pulsar.nFit,
            )
        };
        let before = state(&session);
        let error = session.refit().unwrap_err();
        assert!(error.to_string().contains("ndata>0"), "{error}");
        assert_eq!(before, state(&session));
        assert_eq!(result, serde_json::to_vec(session.timing()).unwrap());
        assert!(!session.can_save());
    }
}
