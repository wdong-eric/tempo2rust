// Adopted PKS subset of global.C::formBatsAll, calculate_bclt.C::calculate_bclt
// and formBats.C::formBats. Fused terms follow shapiro_delay.C::shapiro_delay,
// dm_delays.C::dm_delays (constant DM/spherical wind), vectorPulsar.C::vectorPulsar
// (fixed direction) and secularMotion.C::secularMotion (zero correction only).
// This owned entry also calls formResiduals.C::formResiduals through the Rust kernel;
// its scope is broader than the source formBatsAll call, not whole-file coverage.
use crate::{
    PRECISION_BITS, RuntimeContext, Scalar, TimingError, TimingInput, TimingResult,
    clkcorr::ClockStep, runtime::Resource, tempo2_util::dot_product, tropo::Weather,
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
    let alpha = input.parameter_cell("RAJ").val.to_f64();
    let delta = input.parameter_cell("DECJ").val.to_f64();
    let pos_pulsar = [
        alpha.cos() * delta.cos(),
        alpha.sin() * delta.cos(),
        delta.sin(),
    ];
    let norm = dot_product(pos_pulsar, pos_pulsar).sqrt();
    let pulsar_direction = pos_pulsar.map(|x| x / norm);
    let dm = input.parameter_cell("DM").val.to_f64();
    let mut corrections = Vec::new();
    let mut bats = Vec::new();
    let mut warnings = Vec::new();
    let mut tropo_warned = false;
    for (obs, original) in input
        .source_observations()
        .iter()
        .zip(&input.observation_metadata)
    {
        let calculated = (|| {
            let sat = crate::observation::read(&obs.sat, "sat")?;
            let mut clock_observation = obs.clone();
            clock_observation.seed_clock_caller();
            let (clock_steps, clock_tt) = crate::clkcorr::with_session(&runtime.root, |session| {
                session.queries.clear();
                // The selected timing input has no SAT jumps; direct source-field
                // tests exercise toa2utc's jump branch without extending PAR input.
                let pulsar =
                    crate::toa2utc::ClockPulsarView::single_observation(input.source_pulsar());
                let mut observations = [vec![clock_observation.clone()]];
                crate::toa2utc::toa2utc(session, &[pulsar], &mut observations, 1)?;
                clock_observation = observations[0].remove(0);
                let total = crate::clkcorr::get_correction_tt(&clock_observation)?;
                let steps = session.steps(
                    &runtime.root,
                    *crate::observation::read(
                        &clock_observation.nclock_correction,
                        "nclock_correction",
                    )? as usize,
                )?;
                session.export_resources(&runtime.root, &mut runtime.resources);
                warnings.extend(
                    session
                        .diagnostics
                        .events
                        .drain(..)
                        .map(|(_, b)| String::from_utf8_lossy(&b).into_owned()),
                );
                Ok((steps, total))
            })?;
            let tt = sat.clone() + clock_tt / 86400.0f64;
            // global.C: rough TT->TB, first ephemeris, site, refined TT->TB,
            // refined ephemeris. Legacy repeat calls can inherit previous vectors;
            // owned evaluation starts with zero vectors so prior input/failure
            // cannot affect a new result (same-context recovery regression).
            clock_observation.earth_ssb = [Some(0.0); 6];
            clock_observation.observatory_earth[..3].fill(Some(0.0));
            clock_observation.siteVel = [Some(0.0); 3];
            let initial_time_correction =
                crate::clkcorr::with_resources(&runtime.root, |session| {
                    let result = crate::tt2tdb::frontend(
                        session,
                        input.source_pulsar(),
                        &mut clock_observation,
                    )?;
                    session.export_time_resources(&runtime.root, &mut runtime.resources);
                    Ok(result)
                })?;
            let jd = (tt.clone()
                + initial_time_correction.correction_tt_teph / 86400.0f64
                + 2400000.5f64)
                .to_f64();
            let earth_initial =
                crate::read_ephemeris::read_ephemeris(&mut runtime.jpl, jd)?.earth_ssb;
            let (clock_utc_steps, clock_utc) =
                crate::clkcorr::with_session(&runtime.root, |session| {
                    session.queries.clear();
                    let total =
                        session.get_correction(&clock_observation, None, Some(b"UTC"), 1)?;
                    let steps = session.steps(&runtime.root, session.queries.len())?;
                    session.export_resources(&runtime.root, &mut runtime.resources);
                    warnings.extend(
                        session
                            .diagnostics
                            .events
                            .drain(..)
                            .map(|(_, b)| String::from_utf8_lossy(&b).into_owned()),
                    );
                    Ok((steps, total))
                })?;
            let utc = sat.clone() + clock_utc / 86400.0f64;
            let eop = crate::clkcorr::with_resources(&runtime.root, |session| {
                session.eop.initialize(
                    Some(&runtime.root),
                    b"earth/eopc04_IAU2000.62-now",
                    &mut session.diagnostics,
                )?;
                let mut values = [0.0; 4];
                session.eop.get_eop(
                    utc.to_f64(),
                    &mut values,
                    [0, 1, 2, 3],
                    2,
                    &mut session.diagnostics,
                )?;
                session.export_terrestrial_resources(&runtime.root, &mut runtime.resources);
                Ok(values)
            })?;
            let site = crate::get_obs_coord::get_obs_coord(&runtime.observatory, &tt, &utc, eop)?;
            clock_observation.earth_ssb = earth_initial.map(Some);
            clock_observation.observatory_earth[..3]
                .copy_from_slice(&site.observatory_earth.map(Some));
            clock_observation.siteVel = site.site_velocity.map(Some);
            let time_correction = crate::clkcorr::with_resources(&runtime.root, |session| {
                let result = crate::tt2tdb::frontend(
                    session,
                    input.source_pulsar(),
                    &mut clock_observation,
                )?;
                session.export_time_resources(&runtime.root, &mut runtime.resources);
                Ok(result)
            })?;
            let jd = (tt + time_correction.correction_tt_teph.clone() / 86400.0f64 + 2400000.5f64)
                .to_f64();
            let crate::read_ephemeris::EphemerisState {
                sun_ssb,
                earth_ssb,
                planets_earth,
            } = crate::read_ephemeris::read_ephemeris(&mut runtime.jpl, jd)?;
            // The established PKS frontend selects propagation corrections for
            // every supplied TOA; initialiseOne leaves this observation cell unwritten.
            clock_observation.delayCorr = Some(1);
            clock_observation.zenith = site.zenith.map(Some);
            let weather = crate::clkcorr::with_resources(&runtime.root, |session| {
                let weather = crate::tropo::compute_observation(
                    session,
                    &crate::tropo::TropospherePulsar {
                        source: input.source_pulsar(),
                        position: Some(pos_pulsar),
                        warnings: Some(1),
                    },
                    &mut clock_observation,
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
                Ok(weather)
            })?;
            let tropospheric_delay = *crate::observation::read(
                &clock_observation.troposphericDelay,
                "troposphericDelay",
            )?;
            let observatory_ssb = std::array::from_fn(|i| earth_ssb[i] + site.observatory_earth[i]);
            let observatory_sun =
                std::array::from_fn(|i| -sun_ssb[i] + earth_ssb[i] + site.observatory_earth[i]);
            let observatory_velocity_ssb =
                std::array::from_fn(|i| earth_ssb[i + 3] + site.site_velocity[i]);
            let mut dt_ssb = 0.0f64;
            let mut shapiro = [0.0; 6];
            let mut roemer = 0.0;
            let mut dispersion = 0.0;
            let mut solar_dispersion = 0.0;
            let mut freq_ssb = 0.0;
            for iteration in 0..100 {
                let dt_ssb_old = dt_ssb;
                // No supplied PM, parallax or acceleration terms in this profile.
                roemer = dot_product(pos_pulsar, observatory_ssb);
                let sun_distance = dot_product(observatory_sun, observatory_sun).sqrt();
                let cos_theta_sun = dot_product(pulsar_direction, observatory_sun) / sun_distance;
                shapiro[0] = -2.0
                    * 4.925490947e-6
                    * (sun_distance / 499.00478364 * (1.0 + cos_theta_sun)).ln();
                for (i, gravitational_time) in
                    [1.2061e-11, 4.70255e-9, 1.40797e-9, 2.14539e-10, 2.54488e-10]
                        .into_iter()
                        .enumerate()
                {
                    let observatory_planet =
                        std::array::from_fn(|j| -planets_earth[i][j] + site.observatory_earth[j]);
                    let radius = dot_product(observatory_planet, observatory_planet).sqrt();
                    let cos_theta = dot_product(pulsar_direction, observatory_planet) / radius;
                    shapiro[i + 1] = -2.0
                        * gravitational_time
                        * (radius / 499.00478364 * (1.0 + cos_theta)).ln();
                }
                freq_ssb = *crate::observation::read(&obs.freq, "freq")?
                    * 1.0e6
                    * (1.0 - dot_product(pulsar_direction, observatory_velocity_ssb));
                if freq_ssb > 0.0 && time_correction.einstein_rate != 0.0 {
                    freq_ssb /= time_correction.einstein_rate;
                }
                if freq_ssb <= 1.0 {
                    dispersion = 0.0;
                    solar_dispersion = 0.0;
                } else {
                    dispersion = dm / 2.41e-4 / 1.0e-12 / freq_ssb / freq_ssb;
                    let spherical = 1.0e6 * 1.49598e11 * 1.49598e11 / 299792458.0 / 7.436e6
                        * cos_theta_sun.acos()
                        / sun_distance
                        / (1.0 - cos_theta_sun * cos_theta_sun).sqrt()
                        / freq_ssb
                        / freq_ssb;
                    solar_dispersion = 4.0 * spherical;
                }
                // Source iteration uses Sun+Jupiter only, though final BAT uses all.
                dt_ssb = roemer - (dispersion + solar_dispersion) - (shapiro[0] + shapiro[2]);
                if (dt_ssb - dt_ssb_old).abs() <= 1.0e-10 {
                    break;
                }
                if iteration == 99 {
                    warnings.push("calculate_bclt: dt_SSB reached 100 iterations".into());
                }
            }
            // calculate_bclt can carry nonfinite corrections into a nominal
            // result. Reject them at this boundary instead of publishing NaN;
            // the resource-recovery example exercises overflowing DM and reuse.
            if [
                roemer,
                dispersion,
                solar_dispersion,
                tropospheric_delay,
                freq_ssb,
            ]
            .iter()
            .chain(&shapiro)
            .any(|v| !v.is_finite())
            {
                return Err(TimingError::new(
                    &input.tim.path,
                    Some(original.line),
                    "propagation",
                    "nonfinite correction",
                ));
            }
            let shapiro_total =
                shapiro[0] + (shapiro[1] + shapiro[2] + shapiro[3] + shapiro[4] + shapiro[5]);
            let propagation = (time_correction.correction_tt_tb.clone() - tropospheric_delay
                + roemer
                - shapiro_total
                - dispersion
                - solar_dispersion)
                / 86400.0f64;
            let bat_corr = Float::with_val(PRECISION_BITS, clock_tt / 86400.0f64) + &propagation;
            let bat = (sat.clone() + clock_tt / 86400.0f64) + propagation;
            let scalar = |x| Scalar(Float::with_val(PRECISION_BITS, x));
            let correction = Corrections {
                index: original.index,
                clock_steps,
                clock_tt_s: scalar(clock_tt),
                clock_utc_steps,
                tt_tb_s: Scalar(time_correction.correction_tt_tb),
                tt_teph_s: Scalar(time_correction.correction_tt_teph),
                earth_ssb_light_s: [
                    scalar(earth_ssb[0]),
                    scalar(earth_ssb[1]),
                    scalar(earth_ssb[2]),
                ],
                observatory_earth_light_s: site.observatory_earth.map(scalar),
                roemer_s: scalar(roemer),
                shapiro_s: shapiro.map(scalar),
                dispersion_s: scalar(dispersion),
                solar_dispersion_s: scalar(solar_dispersion),
                troposphere_s: scalar(tropospheric_delay),
                frequency_ssb_hz: scalar(freq_ssb),
                bat_correction_day: Scalar(bat_corr),
                weather,
            };
            Ok((correction, Scalar(bat)))
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
