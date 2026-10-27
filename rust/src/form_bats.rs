//! Bounded PKS adapter over the shared full input-to-BAT routines. Keeps the
//! established residual/fit guards and JSON projection; no timing formulas live here.
use crate::{
    PRECISION_BITS, RuntimeContext, Scalar, TimingError, TimingInput, TimingResult,
    clkcorr::ClockStep, runtime::Resource, tropo::Weather,
};
use rug::Float;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Corrections {
    pub index: usize,
    pub clock_steps: Vec<ClockStep>,
    pub clock_tt_s: Scalar,
    pub clock_utc_steps: Vec<ClockStep>,
    pub tt_tb_s: Scalar,
    pub tt_teph_s: Scalar,
    pub earth_ssb_light_s: [Scalar; 3],
    pub observatory_earth_light_s: [Scalar; 3],
    pub roemer_s: Scalar,
    /// Sun, Venus, Jupiter, Saturn, Uranus, Neptune, in that order.
    pub shapiro_s: [Scalar; 6],
    pub dispersion_s: Scalar,
    pub solar_dispersion_s: Scalar,
    pub troposphere_s: Scalar,
    pub frequency_ssb_hz: Scalar,
    pub bat_correction_day: Scalar,
    pub weather: Weather,
}
#[derive(Debug, Serialize)]
pub struct TerrestrialResult {
    #[serde(flatten)]
    pub timing: TimingResult,
    pub corrections: Vec<Corrections>,
    pub runtime_root: std::path::PathBuf,
    pub resources: Vec<Resource>,
    pub warnings: Vec<String>,
    pub retained_numerics: [&'static str; 1],
}

/// Evaluate the declared isolated PKS model using explicit owned runtime resources.
/// The input is borrowed; the returned value exists only after every stage succeeds.
/// Combines the supported formBatsAll stages with the existing residual operation.
pub fn form_bats_all(
    input: &TimingInput,
    runtime: &mut RuntimeContext,
) -> Result<TerrestrialResult, TimingError> {
    let _workflow = crate::execution::enter();
    input.require_tcb()?;
    for i in 0..input.source_observations().len() {
        if !input.observation_site(i)?.eq_ignore_ascii_case(b"pks") {
            return Err(TimingError::new(
                &input.tim.path,
                None,
                "site",
                "terrestrial evaluation requires PKS observations",
            ));
        }
    }
    for key in ["RAJ", "DECJ", "DM", "POSEPOCH", "DMEPOCH"] {
        if !input.parameter_metadata.contains_key(key) {
            return Err(TimingError::new(
                &input.par.path,
                None,
                key,
                "required for the declared PKS profile",
            ));
        }
    }
    for (key, expected) in [("CLK", "TT(TAI)"), ("EPHEM", "DE405")] {
        let field = if key == "CLK" {
            &input.source_pulsar().clock[..]
        } else {
            &input.source_pulsar().ephemeris[..]
        };
        if !input.text_parameters.contains_key(key)
            || crate::observation::read_string(field, key)? != expected.as_bytes()
        {
            return Err(TimingError::new(
                &input.par.path,
                None,
                key,
                format!("PKS profile requires {key} {expected}"),
            ));
        }
    }
    let mut source = input.source.clone();
    {
        use std::os::unix::ffi::OsStrExt;
        let (p, rows) = source.pulsar_observations_mut(0)?;
        crate::observation::write_string(
            &mut p.JPL_EPHEMERIS[..],
            runtime
                .root
                .join("ephemeris/DE405.1950.2050")
                .as_os_str()
                .as_bytes(),
            "JPL_EPHEMERIS",
        )?;
        // The bounded loader intentionally exposes only five TIM columns. Supply
        // its already-declared no-flags/no-offset terrestrial caller controls.
        for o in rows {
            o.seed_clock_caller();
            o.delayCorr = Some(1);
            o.deleted = Some(0);
            o.nFlags = Some(0);
            o.phaseOffset = Some(0.0);
            o.origErr = o.toaErr;
            o.efac = Some(1.0);
            o.equad = Some(0.0);
            o.obsNjump = Some(0);
            o.obsNfdjump = Some(0);
        }
    }
    crate::preprocess::pre_process_simple(&mut source, 0, &runtime.root)?;
    crate::global::form_bats_all(&mut source, runtime)?;
    let (p, rows) = source.pulsar_observations_mut(0)?;
    let mut corrections = Vec::new();
    let mut bats = Vec::new();
    let mut warnings = Vec::new();
    let mut tropo_warned = false;
    for (obs, original) in rows.iter().zip(&input.observation_metadata) {
        let calculated = (|| {
            use crate::observation::read;
            // The historical JSON includes table-selection metadata absent from
            // the source observation record. Query the shared clock/weather
            // routines on a detached row; numerical results below come solely
            // from the full canonical driver above.
            let (clock_steps, clock_utc_steps, weather) =
                crate::clkcorr::with_resources(&runtime.root, |session| {
                    let mut presentation = obs.clone();
                    session.queries.clear();
                    let view = crate::toa2utc::ClockPulsarView::single_observation(p);
                    let mut rows = [vec![presentation.clone()]];
                    crate::toa2utc::toa2utc(session, &[view], &mut rows, 1)?;
                    presentation = rows[0].remove(0);
                    let clock_steps = session.steps(
                        &runtime.root,
                        *read(&presentation.nclock_correction, "nclock_correction")? as usize,
                    )?;
                    session.queries.clear();
                    session.get_correction(&presentation, None, Some(b"UTC"), 1)?;
                    let clock_utc_steps = session.steps(&runtime.root, session.queries.len())?;
                    let weather = crate::tropo::compute_observation(
                        session,
                        &crate::tropo::TropospherePulsar {
                            source: p,
                            position: None,
                            warnings: Some(1),
                        },
                        &mut presentation,
                        corrections.len(),
                        &mut tropo_warned,
                    )?
                    .ok_or_else(|| {
                        crate::clock_diagnostics::boundary("PKS_troposphere_not_calculated")
                    })?;
                    session.export_resources(&runtime.root, &mut runtime.resources);
                    warnings.extend(
                        session
                            .diagnostics
                            .events
                            .drain(..)
                            .map(|(_, b)| String::from_utf8_lossy(&b).into_owned()),
                    );
                    Ok((clock_steps, clock_utc_steps, weather))
                })?;
            let shapiro = [
                *read(&obs.shapiroDelaySun, "shapiroDelaySun")?,
                *read(&obs.shapiroDelayVenus, "shapiroDelayVenus")?,
                *read(&obs.shapiroDelayJupiter, "shapiroDelayJupiter")?,
                *read(&obs.shapiroDelaySaturn, "shapiroDelaySaturn")?,
                *read(&obs.shapiroDelayUranus, "shapiroDelayUranus")?,
                *read(&obs.shapiroDelayNeptune, "shapiroDelayNeptune")?,
            ];
            let dispersion = *read(&obs.tdis1, "tdis1")?;
            let solar = *read(&obs.tdis2, "tdis2")?;
            let tropo = *read(&obs.troposphericDelay, "troposphericDelay")?;
            let freq = *read(&obs.freqSSB, "freqSSB")?;
            if [dispersion, solar, tropo, freq]
                .iter()
                .chain(&shapiro)
                .any(|v| !v.is_finite())
                || !read(&obs.roemer, "roemer")?.is_finite()
            {
                return Err(TimingError::new(
                    &input.tim.path,
                    Some(original.line),
                    "propagation",
                    "nonfinite correction",
                ));
            }
            let scalar = |v| Scalar(Float::with_val(PRECISION_BITS, v));
            let earth = [
                *read(&obs.earth_ssb[0], "earth_ssb")?,
                *read(&obs.earth_ssb[1], "earth_ssb")?,
                *read(&obs.earth_ssb[2], "earth_ssb")?,
            ];
            let site = [
                *read(&obs.observatory_earth[0], "observatory_earth")?,
                *read(&obs.observatory_earth[1], "observatory_earth")?,
                *read(&obs.observatory_earth[2], "observatory_earth")?,
            ];
            Ok((
                Corrections {
                    index: original.index,
                    clock_steps,
                    clock_tt_s: scalar(crate::clkcorr::get_correction_tt(obs)?),
                    clock_utc_steps,
                    tt_tb_s: Scalar(read(&obs.correctionTT_TB, "correctionTT_TB")?.clone()),
                    tt_teph_s: Scalar(read(&obs.correctionTT_Teph, "correctionTT_Teph")?.clone()),
                    earth_ssb_light_s: earth.map(scalar),
                    observatory_earth_light_s: site.map(scalar),
                    roemer_s: Scalar(read(&obs.roemer, "roemer")?.clone()),
                    shapiro_s: shapiro.map(scalar),
                    dispersion_s: scalar(dispersion),
                    solar_dispersion_s: scalar(solar),
                    troposphere_s: scalar(tropo),
                    frequency_ssb_hz: scalar(freq),
                    bat_correction_day: Scalar(read(&obs.batCorr, "batCorr")?.clone()),
                    weather,
                },
                Scalar(read(&obs.bat, "bat")?.clone()),
            ))
        })()
        .map_err(|mut error: TimingError| {
            error.message = format!(
                "observation {} ({}) at SAT {}: {}",
                original.index,
                original.name,
                obs.sat
                    .as_ref()
                    .map(|v| v.to_string_radix(10, None))
                    .unwrap_or_else(|| "unwritten".into()),
                error.message
            );
            error
        })?;
        corrections.push(calculated.0);
        bats.push(calculated.1);
    }
    let mut timing = crate::form_residuals::form_residuals_at(input, &bats)?;
    timing.engine.support_profile = "M1b isolated PKS TCB DE405 IF99 TT(TAI) IAU2000B F0-F3";
    let mut resources = runtime.resources.clone();
    for resource in &mut resources {
        if corrections.iter().any(|correction| {
            correction
                .clock_steps
                .iter()
                .chain(&correction.clock_utc_steps)
                .any(|step| step.resource == resource.path)
        }) {
            resource.role = "applied clock".into();
        }
    }
    Ok(TerrestrialResult {
        timing,
        corrections,
        runtime_root: runtime.root.clone(),
        resources,
        warnings,
        retained_numerics: [
            "Bundled SOFA Fortran closure from TEMPO2 fda6ad4; unchanged; native/upstream_manifest.json",
        ],
    })
}
