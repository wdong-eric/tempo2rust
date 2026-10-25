//! Rust-only owned projections of tempo2.h pulsar/observation/parameter and
//! initialise.C::initialiseOne state, with error/JSON glue; no standalone counterpart
//! or C ABI equivalence. Reexports selected readParfile.C::readParfile,
//! readTimfile.C::readTimfile, global.C::formBatsAll, formResiduals.C::formResiduals
//! and t2fit.C::t2Fit responsibilities through the Rust modules.
//! Timing and selected spin fitting for isolated-pulsar TCB `@` and PKS profiles.
//! See README.md for accepted input syntax and scientific conventions.

mod averaging;
pub mod bats;
pub mod binary;
pub mod calceph;
mod clkcorr;
mod clock_diagnostics;
mod clock_glob;
#[cfg(test)]
mod clock_tests;
mod clock_text;
mod eop;
#[cfg(test)]
mod fitting_tests;
mod form_bats;
mod form_residuals;
mod get_obs_coord;
pub mod global;
mod gw_ecc;
mod ifteph;
pub mod ifunc;
mod initialise;
mod input;
mod jpleph;
mod longdouble;
mod observation;
#[cfg(test)]
mod observation_storage_tests;
mod observatory;
#[cfg(test)]
mod parameter_storage_tests;
pub mod preprocess;
pub mod propagation;
#[cfg(test)]
mod propagation_helper_tests;
pub mod pulsar;
pub mod pulsar_lifecycle;
#[cfg(test)]
mod pulsar_lifecycle_tests;
pub mod read_ephemeris;
mod read_fortran;
pub mod read_fortran2;
pub mod read_jbo_bat;
mod read_parfile;
mod read_timfile;
pub mod refphs;
mod residual_source;
#[cfg(test)]
mod resource_tests;
mod runtime;
pub mod selection;
pub mod shapelet;
pub mod solar_wind;
pub mod sort_toas;
pub mod source_par;
#[cfg(test)]
mod source_par_corpus_tests;
#[cfg(test)]
mod source_par_tests;
mod source_state;
pub mod source_text;
pub mod source_tim;
#[cfg(test)]
mod source_tim_tests;
mod std_freq;
mod stdio;
mod t2fit;
pub mod t2fit_nestlike;
mod tabulated_function;
mod tai2ut1;
pub mod tempo2;
mod tempo2_util;
pub mod timing;
pub mod tk_matrix;
mod tkfit;
mod tksvd;
mod toa2utc;
mod tropo;
mod tt2tdb;
pub mod units;
pub mod update_epoch;
mod vector_pulsar;

pub use clkcorr::{ClockStep, take_clock_diagnostics};
pub use clock_diagnostics::ClockDiagnostic;
pub use form_bats::{Corrections, TerrestrialResult, form_bats_all as evaluate_terrestrial};
pub use form_residuals::form_residuals as evaluate_barycentric;
pub use input::load;
pub use longdouble::{PRECISION_BITS, Scalar};
pub use read_parfile::FitControls;
pub use runtime::{Resource, RuntimeContext};
pub use t2fit::{FitCycle, FitOptions, FitResult, FitTiming, fit_barycentric, fit_terrestrial};
pub use tempo2_util::{copy_param, get_parameter_value};
pub use tkfit::{FitNumber, LeastSquares};
pub use tropo::{Weather, WeatherValue};

use serde::Serialize;
use std::{collections::BTreeMap, fmt, path::PathBuf};

/// Context is attached at the input or observation boundary; no partial result is returned.
#[derive(Clone, Debug)]
pub struct TimingError {
    pub path: PathBuf,
    pub line: Option<usize>,
    pub field: String,
    pub message: String,
}

impl TimingError {
    pub(crate) fn new(
        path: impl Into<PathBuf>,
        line: Option<usize>,
        field: &str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            line,
            field: field.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for TimingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.display())?;
        if let Some(line) = self.line {
            write!(f, ":{line}")?;
        }
        write!(f, ": {}: {}", self.field, self.message)
    }
}
impl std::error::Error for TimingError {}

#[derive(Clone, Debug, Serialize)]
pub struct SourceFile {
    pub path: PathBuf,
    pub sha256: String,
    pub text: String,
}

/// Historical scalar output snapshot, not the source array-valued parameter.
#[derive(Clone, Debug, Serialize)]
pub struct Parameter {
    pub value: Scalar,
    pub set: bool,
    pub fit_flag: u8,
    /// Supplied uncertainty in the original parameter's units, used only as metadata.
    pub uncertainty: Option<Scalar>,
    pub original: Option<ParameterLine>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ParameterLine {
    pub line: usize,
    pub keyword: String,
    pub tokens: Vec<String>,
    pub raw_line: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct OriginalObservation {
    pub index: usize,
    pub line: usize,
    pub name: String,
    pub raw_line: String,
    pub frequency_mhz_text: String,
    pub sat_mjd_text: String,
    pub toa_error_us_text: String,
    pub site: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Observation {
    pub original: OriginalObservation,
    /// MJD days: barycentric TCB for @, observatory-clock time for PKS.
    pub sat_mjd: Scalar,
    pub frequency_mhz: f64,
    pub toa_error_us: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Conventions {
    pub time_scale: &'static str,
    pub units_resolution: String,
    pub phase_reference: &'static str,
    pub remove_mean: bool,
    pub fit_performed: bool,
    pub input_order: &'static str,
    pub corrections: &'static str,
}

/// Strict frontend provenance; no authoritative physical values or fit flags.
#[derive(Clone, Debug, Default)]
pub(crate) struct ParameterMetadata {
    uncertainty: Option<Scalar>,
    original: Option<ParameterLine>,
}

/// Owns validated input. Read-only access keeps parser-established invariants intact.
#[derive(Clone, Debug)]
pub struct TimingInput {
    pub(crate) par: SourceFile,
    pub(crate) tim: SourceFile,
    pub(crate) source: pulsar_lifecycle::PulsarContext,
    pub(crate) parameter_metadata: BTreeMap<String, ParameterMetadata>,
    pub(crate) text_parameters: BTreeMap<String, ParameterLine>,
    pub(crate) observation_metadata: Vec<OriginalObservation>,
    pub(crate) conventions: Conventions,
    // Kept out of the existing fixed-evaluation JSON contract. Fits expose controls explicitly.
    pub(crate) fit_metadata: read_parfile::FitControlMetadata,
}

impl TimingInput {
    /// Detached controls snapshot. Source mode/nits are authoritative;
    /// declarations and default provenance remain immutable frontend metadata.
    pub fn fit_controls(&self) -> FitControls {
        FitControls {
            mode: self.source_pulsar().fitMode.expect("validated mode") as u8,
            nits: self.source_pulsar().nits.expect("validated NITS") as usize,
            mode_defaulted: self.fit_metadata.mode_defaulted,
            nits_defaulted: self.fit_metadata.nits_defaulted,
            declarations: self.fit_metadata.declarations.clone(),
        }
    }
    pub fn source_pulsar(&self) -> &pulsar::SourcePulsar {
        &self.source.pulsars[0]
    }
    pub(crate) fn require_tcb(&self) -> Result<(), TimingError> {
        if self.source_pulsar().units == Some(1) {
            Ok(())
        } else {
            Err(TimingError::new(
                &self.par.path,
                None,
                "UNITS",
                "canonical source units are outside the supported TCB profile",
            ))
        }
    }
    #[cfg(test)]
    pub(crate) fn source_observations_mut(&mut self) -> &mut [tempo2::SourceObservation] {
        let view = self.source.pulsars[0].obsn.unwrap().unwrap();
        self.source
            .observations_mut(view)
            .expect("validated observation owner")
    }
    pub fn par_source(&self) -> &SourceFile {
        &self.par
    }
    pub fn tim_source(&self) -> &SourceFile {
        &self.tim
    }
    /// Fresh owned compatibility view. Mutating it cannot modify this input.
    pub fn parameters(&self) -> BTreeMap<String, Parameter> {
        self.parameter_metadata
            .iter()
            .map(|(key, meta)| {
                let cell = self.parameter_cell(key);
                (
                    key.clone(),
                    Parameter {
                        value: Scalar(cell.val.clone()),
                        set: cell.param_set != 0,
                        fit_flag: cell.fit_flag.try_into().expect("strict frontend fit flag"),
                        uncertainty: meta.uncertainty.clone(),
                        original: meta.original.clone(),
                    },
                )
            })
            .collect()
    }
    /// Source storage from the bounded frontend bridge: prefit fields remain
    /// unwritten; supplied uncertainty is frontend metadata, not source err.
    pub fn parameter_store(&self) -> &tempo2::ParameterStore {
        &self.source.pulsars[0].param
    }
    pub(crate) fn parameter_cell(&self, key: &str) -> &tempo2::ParameterCell {
        let (label, i) = read_parfile::numeric_location(key);
        // Only the validated frontend and fit callbacks can mutate this owner.
        &self.source.pulsars[0].param.slots[label as usize].cells[i]
    }
    pub(crate) fn parameter_cell_mut(&mut self, key: &str) -> &mut tempo2::ParameterCell {
        let (label, i) = read_parfile::numeric_location(key);
        &mut self.source.pulsars[0].param.slots[label as usize].cells[i]
    }
    pub fn text_parameters(&self) -> &BTreeMap<String, ParameterLine> {
        &self.text_parameters
    }
    /// Detached compatibility snapshot. Numeric values come from source storage;
    /// original text remains provenance. Mutating the snapshot does not change input.
    pub fn observations(&self) -> Vec<Observation> {
        self.source_observations()
            .iter()
            .zip(&self.observation_metadata)
            .map(|(o, original)| Observation {
                original: original.clone(),
                sat_mjd: Scalar(o.sat.as_ref().expect("validated frontend SAT").clone()),
                frequency_mhz: o.freq.expect("validated frontend frequency"),
                toa_error_us: o.toaErr.expect("validated frontend uncertainty"),
            })
            .collect()
    }
    /// Complete read-only source records. The bounded frontend writes only sat,
    /// freq, toaErr, fname and telID; remaining cells/bytes are explicitly unwritten.
    pub fn source_observations(&self) -> &[tempo2::SourceObservation] {
        let p = self.source_pulsar();
        &self
            .source
            .observations(p.obsn.unwrap().unwrap())
            .expect("validated observation owner")[..p.nobs.unwrap() as usize]
    }
    /// Current source site, independent of historical token provenance.
    pub fn observation_site(&self, index: usize) -> Result<Vec<u8>, TimingError> {
        let o = self
            .source_observations()
            .get(index)
            .ok_or_else(|| crate::clock_diagnostics::boundary("clock_observation_storage"))?;
        observation::read_string(&o.telID, "telID")
    }
    pub fn conventions(&self) -> &Conventions {
        &self.conventions
    }
}

#[derive(Debug, Serialize)]
pub struct EngineProvenance {
    pub name: &'static str,
    pub version: &'static str,
    pub numeric_backend: &'static str,
    pub significand_bits: u32,
    pub arithmetic_rounding: &'static str,
    pub support_profile: &'static str,
    pub source_correspondence: &'static str,
}

#[derive(Debug, Serialize)]
pub struct ToaResult {
    pub index: usize,
    pub sat_mjd: Scalar,
    pub bat_mjd: Scalar,
    pub bbat_mjd: Scalar,
    pub phase_turn: Scalar,
    #[serde(serialize_with = "serialize_count")]
    pub nphase_turn: i64,
    #[serde(serialize_with = "serialize_count")]
    pub pulse_number: i64,
    pub residual_s: Scalar,
}

fn serialize_count<S: serde::Serializer>(value: &i64, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
}

#[derive(Debug, Serialize)]
pub struct TimingResult {
    pub schema_version: u8,
    pub status: &'static str,
    pub engine: EngineProvenance,
    pub input: TimingInput,
    /// Signed fractional reduced phase of the first input TOA, before subtraction.
    pub reference_phase_turn: Scalar,
    pub observations: Vec<ToaResult>,
}

// Preserve the existing JSON contract while deriving its scalar view on demand.
impl Serialize for TimingInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut out = serializer.serialize_struct("TimingInput", 6)?;
        out.serialize_field("par", &self.par)?;
        out.serialize_field("tim", &self.tim)?;
        out.serialize_field("parameters", &self.parameters())?;
        out.serialize_field("text_parameters", &self.text_parameters)?;
        out.serialize_field("observations", &self.observations())?;
        out.serialize_field("conventions", &self.conventions)?;
        out.end()
    }
}

#[cfg(test)]
mod terrestrial_tests;

#[cfg(test)]
mod time_ephemeris_tests;

#[cfg(test)]
mod observer_coordinates_tests;

#[cfg(test)]
mod corpus_wire;
#[cfg(test)]
mod preprocessing_corpus_tests;
#[cfg(test)]
mod propagation_corpus_tests;
#[cfg(test)]
mod source_tim_corpus_tests;
