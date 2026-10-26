//! textOutput.C, displayParameters.C, storePrecision.C and generated enum_str.C.
//! Safe stream/file ownership glue uses the existing canonical pulsar and logger.
use crate::{
    PRECISION_BITS, TimingError,
    clock_diagnostics::{Diagnostics, boundary},
    observation::{SourceObservation, read, read_string},
    pulsar::SourcePulsar,
    pulsar_lifecycle::PulsarContext,
    source_format::Argument,
};
use rug::Float;
use std::{
    fs::File,
    io::{BufRead, Write},
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};
#[path = "core_output_source.rs"]
mod source;

pub struct OutputIo<'a> {
    pub stdout: &'a mut dyn Write,
    pub stderr: &'a mut dyn Write,
    pub stdin: &'a mut dyn BufRead,
    pub directory: &'a Path,
}
impl OutputIo<'_> {
    pub(crate) fn write(&mut self, bytes: &[u8]) -> Result<(), TimingError> {
        self.stdout
            .write_all(bytes)
            .map_err(|e| TimingError::new("", None, "stdout", e.to_string()))
    }
    fn create(&self, name: &[u8]) -> Option<File> {
        File::create(self.directory.join(std::ffi::OsStr::from_bytes(name))).ok()
    }
    fn input_token(&mut self) -> Result<Vec<u8>, TimingError> {
        self.stdout
            .flush()
            .map_err(|e| TimingError::new("", None, "stdout", e.to_string()))?;
        let mut result = vec![];
        loop {
            let bytes = self
                .stdin
                .fill_buf()
                .map_err(|e| TimingError::new("", None, "stdin", e.to_string()))?;
            if bytes.is_empty() {
                break;
            }
            let b = bytes[0];
            self.stdin.consume(1);
            if crate::clock_text::whitespace(b) {
                if !result.is_empty() {
                    break;
                }
            } else {
                result.push(b);
            }
        }
        if result.is_empty() {
            return Err(boundary("output_filename_response"));
        }
        Ok(result)
    }
    pub fn drain(&mut self, d: &mut Diagnostics) -> Result<(), TimingError> {
        for (err, bytes) in d.events.drain(..) {
            let out = if err {
                &mut self.stderr
            } else {
                &mut self.stdout
            };
            out.write_all(&bytes)
                .map_err(|e| TimingError::new("", None, "diagnostic", e.to_string()))?;
        }
        Ok(())
    }
}
fn write_file(file: &mut Option<File>, bytes: &[u8]) -> Result<(), TimingError> {
    file.as_mut()
        .ok_or_else(|| boundary("output_file_not_open"))?
        .write_all(bytes)
        .map_err(|e| TimingError::new("", None, "output_file", e.to_string()))
}
pub(crate) fn output_rows(
    context: &PulsarContext,
    p: usize,
) -> Result<&[SourceObservation], TimingError> {
    let owner =
        read(&context.pulsars[p].obsn, "obsn")?.ok_or_else(|| boundary("output_observations"))?;
    context
        .observations(owner)
        .map_err(|e| TimingError::new("", None, "output_observations", e.to_string()))?
        .get(..(*read(&context.pulsars[p].nobs, "nobs")?).max(0) as usize)
        .ok_or_else(|| boundary("output_nobs"))
}
fn output_covar(context: &PulsarContext, p: usize, i: usize, j: usize) -> Result<f64, TimingError> {
    let owner =
        read(&context.pulsars[p].covar, "covar")?.ok_or_else(|| boundary("output_covariance"))?;
    let matrix = context
        .matrix(owner)
        .map_err(|e| TimingError::new("", None, "covar", e.to_string()))?;
    matrix
        .row(i)
        .map_err(|e| TimingError::new("", None, "covar", e.to_string()))?
        .get(j)
        .copied()
        .ok_or_else(|| boundary("output_covariance_column"))
}
fn output_constraint(context: &PulsarContext, p: usize, i: usize) -> Result<Vec<u8>, TimingError> {
    let owner = read(
        &context.pulsars[p].constraint_special[i],
        "constraint_special",
    )?
    .ok_or_else(|| boundary("output_constraint_payload"))?;
    read_string(
        context
            .payload(owner)
            .map_err(|e| TimingError::new("", None, "constraint_special", e.to_string()))?,
        "constraint_special",
    )
}
#[derive(Default)]
pub struct OutputOptions {
    pub global_parameter: f64,
    pub global_count: i32,
    pub residuals: bool,
    pub new_par: Option<PathBuf>,
}
pub fn text_output(
    context: &mut PulsarContext,
    options: &OutputOptions,
    io: &mut OutputIo<'_>,
) -> Result<(), TimingError> {
    crate::clkcorr::with_diagnostics(|d| {
        io.drain(d)?;
        let result = source::text_output_inner(
            context,
            options.global_parameter,
            options.global_count,
            i32::from(options.residuals),
            i32::from(options.new_par.is_some()),
            options
                .new_par
                .as_deref()
                .unwrap_or(Path::new(""))
                .as_os_str()
                .as_bytes(),
            io,
            d,
        );
        io.drain(d)?;
        result
    })
}
fn calc_rms_inner(
    context: &mut PulsarContext,
    p: usize,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    // fda6ad4 textOutput.C::calcRMS:1898-1899 tests current flags but reads
    // pulsar zero's cutoffs. Use the same current pulsar for both, as t2Fit does.
    let (psr, rows) = context.pulsar_observations_mut(p)?;
    source::calc_rms_values(psr, rows, d)
}
pub fn calc_rms(context: &mut PulsarContext, p: usize) -> Result<f64, TimingError> {
    crate::clkcorr::with_diagnostics(|d| calc_rms_inner(context, p, d))
}
pub fn m2(mf: &Float, sini: &Float, m1: &Float, out: &mut dyn Write) -> Result<f64, TimingError> {
    let mut err = std::io::sink();
    let mut input = std::io::empty();
    source::m2_inner(
        mf.clone(),
        sini.clone(),
        m1.clone(),
        &mut OutputIo {
            stdout: out,
            stderr: &mut err,
            stdin: &mut input,
            directory: Path::new(""),
        },
    )
}
pub fn dglep(psr: &SourcePulsar, glitch: i32, fractional_phase: f64) -> Result<f64, TimingError> {
    // The public helper takes a plain caller-supplied index, unlike a validated
    // parameter cell. Bound it before the source's parallel glitch-array reads.
    let index = usize::try_from(glitch).map_err(|_| boundary("glitch_index"))?;
    psr.param.slots[crate::tempo2::Label::param_glf0 as usize]
        .cells
        .get(index)
        .ok_or_else(|| boundary("glitch_index"))?;
    crate::clkcorr::with_diagnostics(|d| source::dglep_inner(psr, glitch, fractional_phase, d))
}
pub fn print_glitch(psr: &SourcePulsar, io: &mut OutputIo<'_>) -> Result<(), TimingError> {
    crate::clkcorr::with_diagnostics(|d| {
        let result = source::print_glitch_inner(psr, io, d);
        io.drain(d)?;
        result
    })
}
/// storePrecision.C::recordPrecision has no executable statements at the pin.
pub fn record_precision(
    _psr: &mut SourcePulsar,
    _precision: &Float,
    _routine: &[u8],
    _comment: &[u8],
) {
}

/// extract_enums.sh label table, including marker entries and empty sentinel.
pub const LABEL_STR: [&str; 167] = {
    let mut names = [""; 167];
    let mut i = 0;
    while i < crate::tempo2::PARAMETER_NAMES.len() {
        names[i] = crate::tempo2::PARAMETER_NAMES[i];
        i += 1;
    }
    names
};
pub const CONSTRAINT_STR: [&str; 69] = [
    "constraint_dmmodel_mean",
    "constraint_dmmodel_dm1",
    "constraint_dmmodel_cw_0",
    "constraint_dmmodel_cw_1",
    "constraint_dmmodel_cw_2",
    "constraint_dmmodel_cw_3",
    "constraint_ifunc_cov",
    "constraint_ifunc_x0",
    "constraint_ifunc_0",
    "constraint_ifunc_1",
    "constraint_ifunc_2",
    "constraint_tel_dx_0",
    "constraint_tel_dx_1",
    "constraint_tel_dx_2",
    "constraint_tel_dy_0",
    "constraint_tel_dy_1",
    "constraint_tel_dy_2",
    "constraint_tel_dz_0",
    "constraint_tel_dz_1",
    "constraint_tel_dz_2",
    "constraint_quad_ifunc_p_0",
    "constraint_quad_ifunc_p_1",
    "constraint_quad_ifunc_p_2",
    "constraint_quad_ifunc_c_0",
    "constraint_quad_ifunc_c_1",
    "constraint_quad_ifunc_c_2",
    "constraint_dmmodel_cw_year_sin",
    "constraint_dmmodel_cw_year_cos",
    "constraint_dmmodel_cw_year_xsin",
    "constraint_dmmodel_cw_year_xcos",
    "constraint_dmmodel_cw_year_sin2",
    "constraint_dmmodel_cw_year_cos2",
    "constraint_dmmodel_cw_px",
    "constraint_ifunc_year_sin",
    "constraint_ifunc_year_cos",
    "constraint_ifunc_year_xsin",
    "constraint_ifunc_year_xcos",
    "constraint_ifunc_year_sin2",
    "constraint_ifunc_year_cos2",
    "constraint_qifunc_p_year_sin",
    "constraint_qifunc_p_year_cos",
    "constraint_qifunc_p_year_xsin",
    "constraint_qifunc_p_year_xcos",
    "constraint_qifunc_p_year_sin2",
    "constraint_qifunc_p_year_cos2",
    "constraint_qifunc_c_year_sin",
    "constraint_qifunc_c_year_cos",
    "constraint_qifunc_c_year_xsin",
    "constraint_qifunc_c_year_xcos",
    "constraint_qifunc_c_year_sin2",
    "constraint_qifunc_c_year_cos2",
    "constraint_red_sin",
    "constraint_red_cos",
    "constraint_band_red_sin",
    "constraint_band_red_cos",
    "constraint_red_dm_sin",
    "constraint_red_dm_cos",
    "constraint_red_chrom_sin",
    "constraint_red_chrom_cos",
    "constraint_swgp_sin",
    "constraint_swgp_cos",
    "constraint_group_red_sin",
    "constraint_group_red_cos",
    "constraint_jitter",
    "constraint_param",
    "constraint_ne_sw_ifunc_sin",
    "constraint_ne_sw_ifunc_sigma",
    "constraint_LAST",
    "",
];
pub const SERIES_STR: [&str; 4] = [
    "series_undefined",
    "series_simple_pn",
    "series_taylor_pn",
    "",
];

fn source_nint(x: f64) -> Result<i32, TimingError> {
    crate::binary::source_i32(if x < 0.0 { x - 0.5 } else { x + 0.5 })
}

pub use crate::display_parameters::display_parameters;
