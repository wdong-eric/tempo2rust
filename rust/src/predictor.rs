//! tempo2.C:359-503 / getInputs.C: PAR-only predictor preparation and output glue.
use crate::{
    PRECISION_BITS, RuntimeContext, TimingError,
    bats::{ConsumedInput, InputPair},
    observation::{SourceObservation, read, read_string, write_string},
    pulsar_lifecycle::PulsarContext,
    tempo2::Label::*,
};
use rug::Float;
use std::path::Path;
#[derive(Debug)]
pub struct PredictorInput {
    pub context: PulsarContext,
    pub inputs: Vec<ConsumedInput>,
}
pub fn load_par(
    par: &Path,
    runtime: &Path,
    arguments: &[Vec<u8>],
) -> Result<PredictorInput, TimingError> {
    let mut context = PulsarContext::unwritten(1);
    context
        .initialise(1, 0, 1)
        .map_err(|e| error(e.to_string()))?;
    let mut inputs = vec![];
    // No TIM is synthesized or read. The empty path disables the existing
    // readParfile fallback to a TIM header when the requested PAR cannot open.
    crate::source_par::read_parfiles(
        &mut context,
        &[InputPair {
            par: par.into(),
            tim: Default::default(),
        }],
        runtime,
        &mut inputs,
    )?;
    crate::preprocess::pre_process(&mut context, arguments, runtime, &mut inputs)?;
    Ok(PredictorInput { context, inputs })
}
pub fn cli_defaults(context: &mut PulsarContext, site: &str) -> Result<Vec<String>, TimingError> {
    let p = &mut context.pulsars[0];
    let mut warnings = vec![];
    let epoch = p.param.slots[param_pepoch as usize].cells[0].val.clone();
    for (l, v) in [
        (param_tzrmjd, epoch),
        (param_tzrfrq, Float::with_val(PRECISION_BITS, 1400)),
    ] {
        let c = &mut p.param.slots[l as usize].cells[0];
        if c.param_set == 0 {
            warnings.push(format!("{} not set; setting to {}", l as usize, v));
            c.param_set = 1;
            c.val = v;
        }
    }
    if read_string(&p.tzrsite[..], "tzrsite")? == b"NULL" {
        write_string(&mut p.tzrsite[..], site.as_bytes(), "tzrsite")?;
        warnings.push(format!("tzrsite not set; setting to {site}"));
    }
    Ok(warnings)
}
pub(crate) fn error(message: impl Into<String>) -> TimingError {
    TimingError::new("predictor", None, "predictor", message)
}
pub(crate) fn synthetic(
    o: &mut SourceObservation,
    sat: Float,
    freq: f64,
    site: &[u8],
    name: &[u8],
) -> Result<(), TimingError> {
    o.initialize_missing_clock_scratch();
    // fda6ad4 initialise.C:80-101 leaves these vectors unwritten; PAR-only
    // preprocessing has nobs=0. The first tt2tb pass reads them before ephemeris
    // evaluation. Use preProcessSimple.C:155-162's zero initial state only for
    // fresh rows, retaining calculated vectors on repeated construction.
    for vector in [
        &mut o.earthMoonBary_ssb,
        &mut o.earthMoonBary_earth,
        &mut o.observatory_earth,
        &mut o.earth_ssb,
    ] {
        for value in vector {
            value.get_or_insert(0.);
        }
    }
    for value in &mut o.siteVel {
        value.get_or_insert(0.);
    }
    o.sat = Some(sat);
    o.freq = Some(freq);
    o.deleted = Some(0);
    o.nFlags = Some(0);
    o.efac = Some(1.);
    o.phaseOffset = Some(0.);
    let bary = site == b"@" || site.eq_ignore_ascii_case(b"bat");
    o.clockCorr = Some(i32::from(!bary));
    o.delayCorr = Some(i32::from(!bary));
    write_string(&mut o.fname, name, "fname")?;
    write_string(&mut o.telID, site, "telID")?;
    // fda6ad4 cheby2d_int.C:73-119 and polyco.C:243-291 synthesize rows after
    // preprocessing, omitting empty jump counts consumed by formResiduals and
    // Cheby's uncertainty consumed by writeTim. No TIM jump/noise data exists
    // here: initialize only those artificial-row fields to their empty values.
    o.equad = Some(0.);
    o.origErr = Some(0.);
    o.obsNjump = Some(0);
    o.obsNfdjump = Some(0);
    o.toaErr = Some(0.);
    Ok(())
}
pub(crate) fn reference(context: &mut PulsarContext) -> Result<(), TimingError> {
    let (p, rows) = context.pulsar_observations_mut(0)?;
    let sat = &p.param.slots[param_tzrmjd as usize].cells[0];
    let freq = &p.param.slots[param_tzrfrq as usize].cells[0];
    if sat.param_set == 0 || freq.param_set == 0 {
        return Err(error("TZRMJD/TZRFRQ required"));
    }
    synthetic(
        &mut rows[0],
        sat.val.clone(),
        freq.val.to_f64(),
        &read_string(&p.tzrsite[..], "tzrsite")?,
        b"reference",
    )
}
pub(crate) fn timing(
    context: &mut PulsarContext,
    runtime: &mut RuntimeContext,
    tim: &Path,
) -> Result<(), TimingError> {
    crate::source_tim::write_tim(tim, context, 0, b"tempo2", &runtime.root)?;
    crate::bats::form_bats_all(context, runtime)?;
    crate::timing::form_residuals(context, false)
}
pub fn spin_frequency(context: &PulsarContext) -> Result<Float, TimingError> {
    let p = context
        .pulsars()
        .first()
        .ok_or_else(|| error("missing pulsar"))?;
    let c = &p.param.slots[param_f as usize].cells[0];
    if c.param_set == 0 {
        return Err(error("F0 required"));
    }
    Ok(c.val.clone())
}
pub(crate) fn observations(context: &PulsarContext) -> Result<&[SourceObservation], TimingError> {
    let p = &context.pulsars()[0];
    let rows = context
        .observations(read(&p.obsn, "obsn")?.ok_or_else(|| error("missing observations"))?)
        .map_err(|e| error(e.to_string()))?;
    Ok(&rows[..*read(&p.nobs, "nobs")? as usize])
}

#[derive(serde::Serialize)]
pub struct GenerationObservation {
    pub sat_mjd: crate::FitNumber,
    pub bat_mjd: crate::FitNumber,
    pub phase_turn: crate::FitNumber,
    pub frequency_mhz: f64,
}
/// Snapshot of the retained synthesized observations after the source operation.
pub fn snapshot(context: &PulsarContext) -> Result<Vec<GenerationObservation>, TimingError> {
    observations(context)?
        .iter()
        .map(|o| {
            Ok(GenerationObservation {
                sat_mjd: crate::FitNumber(read(&o.sat, "sat")?.clone()),
                bat_mjd: crate::FitNumber(read(&o.bat, "bat")?.clone()),
                phase_turn: crate::FitNumber(read(&o.phase, "phase")?.clone()),
                frequency_mhz: *read(&o.freq, "freq")?,
            })
        })
        .collect()
}
