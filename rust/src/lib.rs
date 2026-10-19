//! Fixed-parameter timing for the declared isolated-pulsar, TCB `@` subset.
//! See README.md for accepted input syntax and scientific conventions.

mod form_residuals;
mod input;
mod longdouble;
mod read_parfile;
mod read_timfile;
mod tempo2_util;

pub use form_residuals::form_residuals as evaluate_barycentric;
pub use input::load;
pub use longdouble::{PRECISION_BITS, Scalar};

use serde::Serialize;
use std::{collections::BTreeMap, fmt, path::PathBuf};

/// Context is attached at the input or observation boundary; no partial result is returned.
#[derive(Debug)]
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
    /// Already barycentric TCB, in MJD days; never an observatory clock epoch.
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

/// Owns validated input. Read-only access keeps parser-established invariants intact.
#[derive(Clone, Debug, Serialize)]
pub struct TimingInput {
    pub(crate) par: SourceFile,
    pub(crate) tim: SourceFile,
    pub(crate) parameters: BTreeMap<String, Parameter>,
    pub(crate) text_parameters: BTreeMap<String, ParameterLine>,
    pub(crate) observations: Vec<Observation>,
    pub(crate) conventions: Conventions,
}

impl TimingInput {
    pub fn par_source(&self) -> &SourceFile {
        &self.par
    }
    pub fn tim_source(&self) -> &SourceFile {
        &self.tim
    }
    pub fn parameters(&self) -> &BTreeMap<String, Parameter> {
        &self.parameters
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
