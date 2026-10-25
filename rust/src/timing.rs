//! tempo2.C fixed-parameter formBatsAll -> formResiduals orchestration and Rust
//! output projection. Provenance/serialization have no standalone source counterpart.
use crate::{
    RuntimeContext, TimingError,
    bats::{self, BatInput, Number},
    observation::{SourceObservation, read},
    pulsar_lifecycle::PulsarContext,
    tempo2::{ParameterCell, ParameterStore},
};
use serde::Serialize;

/// Evaluate source residuals on canonical, already-prepared BAT state.
pub fn form_residuals(context: &mut PulsarContext, remove_mean: bool) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| {
        crate::residual_source::form_residuals_inner(
            context,
            i32::from(remove_mean),
            s.source.max_observations,
            &mut s.diagnostics,
        )
        .map(|_| ())
    })
}
#[derive(Debug, Serialize)]
pub struct TimingResult {
    pub schema_version: u32,
    pub status: &'static str,
    pub engine: &'static str,
    pub runtime_root: bats::ByteString,
    pub inputs: Vec<bats::InputRecord>,
    pub resources: Vec<bats::ResourceRecord>,
    pub diagnostics: Vec<bats::Diagnostic>,
    pub remove_mean: bool,
    pub pulsars: Vec<TimingPulsar>,
}
#[derive(Debug, Serialize)]
pub struct TimingPulsar {
    pub name: bats::ByteString,
    pub units: bats::Selector,
    pub time_ephemeris: bats::Selector,
    pub coordinate_frame: bats::Selector,
    pub sorted: i32,
    pub observations: Vec<TimingObservation>,
    pub reference_observation: Option<TimingObservation>,
    pub parameter_mutations: Vec<ParameterMutation>,
}
#[derive(Debug, Serialize)]
pub struct TimingObservation {
    #[serde(flatten)]
    pub barycentric: bats::BatObservation,
    pub input_index: Option<usize>,
    pub pet_mjd: Number,
    pub binary_delay_s: Number,
    pub phase_turn: Number,
    pub nphase_turn: Number,
    #[serde(serialize_with = "crate::serialize_count")]
    pub pulse_number: i64,
    pub residual_s: Number,
    pub residualtn_s: Number,
    pub averaged: std::collections::BTreeMap<&'static str, bats::Quantity>,
}
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct ParameterState {
    pub value: Number,
    pub error: Number,
    pub set: i32,
    pub fit: i32,
}
impl ParameterState {
    fn from(c: &ParameterCell) -> Self {
        Self {
            value: Number::extended(&c.val),
            error: Number::extended(&c.err),
            set: c.param_set,
            fit: c.fit_flag,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct ParameterMutation {
    pub label: &'static str,
    pub index: usize,
    pub before: ParameterState,
    pub after: ParameterState,
}
fn project(
    o: &SourceObservation,
    bat: bats::BatObservation,
    average: bool,
    dm_average: bool,
) -> Result<TimingObservation, TimingError> {
    let averaged = [
        ("bat", o.averagebat, "MJD"),
        ("residual", o.averageres, "s"),
        ("uncertainty", o.averageerr, "s"),
        ("dm_bat", o.averagedmbat, "MJD"),
        ("dm_residual", o.averagedmres, "s at 1400 MHz"),
        ("dm_uncertainty", o.averagedmerr, "s at 1400 MHz"),
    ]
    .into_iter()
    .map(|(k, v, unit)| {
        (
            k,
            bats::Quantity {
                value: v
                    .filter(|_| {
                        if k.starts_with("dm_") {
                            dm_average
                        } else {
                            average
                        }
                    })
                    .map(Number::double),
                unit,
            },
        )
    })
    .collect();
    Ok(TimingObservation {
        barycentric: bat,
        input_index: o.input_index,
        pet_mjd: Number::extended(read(&o.pet, "pet")?),
        binary_delay_s: Number::extended(read(&o.torb, "torb")?),
        phase_turn: Number::extended(read(&o.phase, "phase")?),
        nphase_turn: Number::extended(read(&o.nphase, "nphase")?),
        pulse_number: *read(&o.pulseN, "pulseN")?,
        residual_s: Number::extended(read(&o.residual, "residual")?),
        residualtn_s: Number::extended(read(&o.residualtn, "residualtn")?),
        averaged,
    })
}
pub fn evaluate(
    input: &mut BatInput,
    runtime: &mut RuntimeContext,
    remove_mean: bool,
) -> Result<TimingResult, TimingError> {
    let before: Vec<ParameterStore> = input
        .context
        .pulsars
        .iter()
        .map(|p| p.param.clone())
        .collect();
    bats::form_bats_all(&mut input.context, runtime)?;
    form_residuals(&mut input.context, remove_mean)?;
    let bat = bats::project(input, runtime)?;
    let mut pulsars = vec![];
    for (i, b) in bat.pulsars.into_iter().enumerate() {
        let p = &input.context.pulsars[i];
        let view = p
            .obsn
            .flatten()
            .ok_or_else(|| crate::clock_diagnostics::boundary("timing_observation_owner"))?;
        let rows = input
            .context
            .observations(view)
            .map_err(|e| TimingError::new(&runtime.root, None, "timing output", e.to_string()))?;
        let mut mutations = vec![];
        for (l, (old, new)) in before[i]
            .slots
            .iter()
            .zip(&p.param.slots)
            .enumerate()
            .take(crate::tempo2::Label::param_LAST as usize)
        {
            for (index, (a, b)) in old.cells.iter().zip(&new.cells).enumerate() {
                // Signed finite/class projection, including unchanged NaNs.
                let astate = ParameterState::from(a);
                let bstate = ParameterState::from(b);
                if astate != bstate {
                    mutations.push(ParameterMutation {
                        label: crate::tempo2::PARAMETER_NAMES[l],
                        index,
                        before: astate,
                        after: bstate,
                    });
                }
            }
        }
        pulsars.push(TimingPulsar {
            name: b.name,
            units: b.units,
            time_ephemeris: b.time_ephemeris,
            coordinate_frame: b.coordinate_frame,
            sorted: b.sorted,
            observations: b
                .observations
                .into_iter()
                .zip(rows)
                .map(|(b, o)| {
                    project(
                        o,
                        b,
                        p.AverageResiduals == Some(1),
                        p.AverageDMResiduals == Some(1),
                    )
                })
                .collect::<Result<_, _>>()?,
            reference_observation: b
                .reference_observation
                .map(|b| project(&p.tzrobs, b, false, false))
                .transpose()?,
            parameter_mutations: mutations,
        });
    }
    Ok(TimingResult {
        schema_version: 1,
        status: "complete",
        engine: bat.engine,
        runtime_root: bat.runtime_root,
        inputs: bat.inputs,
        resources: bat.resources,
        diagnostics: bat.diagnostics,
        remove_mean,
        pulsars,
    })
}

/// Source inverse-variance averages of the current residuals; writes avg.dat.
pub fn average_residuals(context: &mut PulsarContext, pulsar: usize) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| {
        let (p, rows) = context.pulsar_observations_mut(pulsar)?;
        crate::averaging::averageResiduals(p, rows, &mut s.diagnostics)
    })
}
/// Source DM-scaled averages at 1400 MHz, on the same canonical rows.
pub fn average_dm_residuals(context: &mut PulsarContext, pulsar: usize) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| {
        let (p, rows) = context.pulsar_observations_mut(pulsar)?;
        crate::averaging::averageDMResiduals(p, rows, &mut s.diagnostics)
    })
}
#[cfg(test)]
#[path = "timing_tests.rs"]
mod tests;
