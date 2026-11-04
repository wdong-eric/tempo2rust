//! Display projection of the canonical Rust timing workflow; no timing model here.
use std::path::Path;
use tempo2_rust::{RuntimeContext, TimingError, bats, execution, timing};

/// Same load/preprocess/evaluate sequence as `tempo2-rust timing` with mean removal.
/// One outer admission keeps process diagnostics with their owning workflow.
pub fn load(par: &Path, tim: &Path, runtime: &Path) -> Result<timing::TimingResult, TimingError> {
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
        timing::evaluate(&mut input, &mut resources, true)
    })
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
}
