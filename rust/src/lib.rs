//! Rust-only owned projections of tempo2.h pulsar/observation/parameter and
//! initialise.C::initialiseOne state, with error/JSON glue; no standalone counterpart
//! or C ABI equivalence. Reexports selected readParfile.C::readParfile,
//! readTimfile.C::readTimfile, global.C::formBatsAll, formResiduals.C::formResiduals
//! and t2fit.C::t2Fit responsibilities through the Rust modules.
//! Timing and selected spin fitting for isolated-pulsar TCB `@` and PKS profiles.
//! See README.md for accepted input syntax and scientific conventions.

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
mod ifteph;
mod initialise;
mod input;
mod jpleph;
mod longdouble;
mod observatory;
#[cfg(test)]
mod parameter_storage_tests;
mod read_ephemeris;
mod read_parfile;
mod read_timfile;
#[cfg(test)]
mod resource_tests;
mod runtime;
mod std_freq;
mod stdio;
mod t2fit;
mod tabulated_function;
pub mod tempo2;
mod tempo2_util;
mod tkfit;
mod tksvd;
mod toa2utc;
mod tropo;
mod tt2tdb;

pub use clkcorr::{ClockStep, take_clock_diagnostics};
pub use clock_diagnostics::ClockDiagnostic;
pub use form_bats::{Corrections, TerrestrialResult, form_bats_all as evaluate_terrestrial};
pub use form_residuals::form_residuals as evaluate_barycentric;
pub use input::load;
pub use longdouble::{PRECISION_BITS, Scalar};
pub use read_parfile::FitControls;
pub use runtime::{Resource, RuntimeContext};
pub use t2fit::{FitCycle, FitOptions, FitResult, FitTiming, fit_barycentric, fit_terrestrial};
pub use tempo2_util::copy_param;
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
    pub(crate) parameter_store: tempo2::ParameterStore,
    pub(crate) parameter_metadata: BTreeMap<String, ParameterMetadata>,
    pub(crate) text_parameters: BTreeMap<String, ParameterLine>,
    pub(crate) observations: Vec<Observation>,
    pub(crate) conventions: Conventions,
    // Kept out of the existing fixed-evaluation JSON contract. Fits expose controls explicitly.
    pub(crate) fit_controls: FitControls,
}

impl TimingInput {
    pub fn fit_controls(&self) -> &FitControls {
        &self.fit_controls
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
        &self.parameter_store
    }
    pub(crate) fn parameter_cell(&self, key: &str) -> &tempo2::ParameterCell {
        let (label, i) = read_parfile::numeric_location(key);
        // Only the validated frontend and fit callbacks can mutate this owner.
        &self.parameter_store.slots[label as usize].cells[i]
    }
    pub(crate) fn parameter_cell_mut(&mut self, key: &str) -> &mut tempo2::ParameterCell {
        let (label, i) = read_parfile::numeric_location(key);
        &mut self.parameter_store.slots[label as usize].cells[i]
    }
    pub fn text_parameters(&self) -> &BTreeMap<String, ParameterLine> {
        &self.text_parameters
    }
    pub fn observations(&self) -> &[Observation] {
        &self.observations
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
        out.serialize_field("observations", &self.observations)?;
        out.serialize_field("conventions", &self.conventions)?;
        out.end()
    }
}
