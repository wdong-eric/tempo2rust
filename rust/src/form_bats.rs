// Rust port of global.C / calculate_bclt.C / formBats.C for the adopted subset.
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
    pub retained_numerics: [&'static str; 2],
}

/// Evaluate the declared isolated PKS model using explicit owned runtime resources.
/// The input is borrowed; the returned value exists only after every stage succeeds.
/// Combines the supported formBatsAll stages with the existing residual operation.
pub fn form_bats_all(
    input: &TimingInput,
    runtime: &mut RuntimeContext,
) -> Result<TerrestrialResult, TimingError> {
    if input
        .observations
        .iter()
        .any(|o| !o.original.site.eq_ignore_ascii_case("pks"))
    {
        return Err(TimingError::new(
            &input.tim.path,
            None,
            "site",
            "terrestrial evaluation requires PKS observations",
        ));
    }
    for key in ["RAJ", "DECJ", "DM", "POSEPOCH", "DMEPOCH"] {
        if !input.parameters.contains_key(key) {
            return Err(TimingError::new(
                &input.par.path,
                None,
                key,
                "required for the declared PKS profile",
            ));
        }
    }
    for (key, expected) in [("CLK", "TT(TAI)"), ("EPHEM", "DE405")] {
        if input
            .text_parameters
            .get(key)
            .is_none_or(|p| p.tokens[0] != expected)
        {
            return Err(TimingError::new(
                &input.par.path,
                None,
                key,
                format!("PKS profile requires {key} {expected}"),
            ));
        }
    }
    let alpha = input.parameters["RAJ"].value.0.to_f64();
    let delta = input.parameters["DECJ"].value.0.to_f64();
    let pos_pulsar = [
        alpha.cos() * delta.cos(),
        alpha.sin() * delta.cos(),
        delta.sin(),
    ];
    let norm = dot_product(pos_pulsar, pos_pulsar).sqrt();
    let pulsar_direction = pos_pulsar.map(|x| x / norm);
    let dm = input.parameters["DM"].value.0.to_f64();
    let mut corrections = Vec::new();
    let mut bats = Vec::new();
    let mut warnings = Vec::new();
    for obs in &input.observations {
        let calculated = (|| {
            let sat = &obs.sat_mjd.0;
            let clock_steps = runtime.clocks.get_clock_corrections(
                &runtime.root,
                &mut runtime.resources,
                &runtime.observatory.clock,
                "TT(TAI)",
                sat.to_f64(),
                &mut warnings,
            )?;
            let clock_tt = clock_steps
                .iter()
                .fold(0.0, |s, correction| s + correction.correction_s.0.to_f64());
            let tt = sat.clone() + clock_tt / 86400.0f64;
            // global.C: rough TT->TB, first ephemeris, site, refined TT->TB,
            // refined ephemeris. Legacy repeat calls can inherit previous vectors;
            // owned evaluation starts with zero vectors so prior input/failure
            // cannot affect a new result (same-context recovery regression).
            let initial_time_correction =
                crate::tt2tdb::tt2tb(&runtime.ifte, &tt, [0.0; 3], [0.0; 3], [0.0; 3])?;
            let jd = (tt.clone()
                + initial_time_correction.correction_tt_teph / 86400.0f64
                + 2400000.5f64)
                .to_f64();
            let earth_initial =
                crate::read_ephemeris::read_ephemeris(&mut runtime.jpl, jd)?.earth_ssb;
            let clock_utc_steps = runtime.clocks.get_clock_corrections(
                &runtime.root,
                &mut runtime.resources,
                &runtime.observatory.clock,
                "UTC",
                sat.to_f64(),
                &mut warnings,
            )?;
            let clock_utc = clock_utc_steps
                .iter()
                .fold(0.0, |s, correction| s + correction.correction_s.0.to_f64());
            let utc = sat.clone() + clock_utc / 86400.0f64;
            let site = crate::get_obs_coord::get_obs_coord(
                &runtime.observatory,
                &tt,
                &utc,
                &runtime.eop,
                &mut warnings,
            )?;
            let time_correction = crate::tt2tdb::tt2tb(
                &runtime.ifte,
                &tt,
                [earth_initial[3], earth_initial[4], earth_initial[5]],
                site.observatory_earth,
                site.site_velocity,
            )?;
            let jd = (tt + time_correction.correction_tt_teph.clone() / 86400.0f64 + 2400000.5f64)
                .to_f64();
            let crate::read_ephemeris::EphemerisState {
                sun_ssb,
                earth_ssb,
                planets_earth,
            } = crate::read_ephemeris::read_ephemeris(&mut runtime.jpl, jd)?;
            let weather = Weather {
                pressure: crate::tropo::get_meteorological_value(
                    &runtime.surface_pressure_tables,
                    &runtime.observatory.code,
                    sat.to_f64(),
                    true,
                    &mut warnings,
                )?,
                zenith_wet_delay: crate::tropo::get_meteorological_value(
                    &runtime.zenith_wet_delay_tables,
                    &runtime.observatory.code,
                    sat.to_f64(),
                    false,
                    &mut warnings,
                )?,
            };
            let tropospheric_delay = crate::tropo::compute_tropospheric_delay(
                &runtime.observatory,
                site.zenith,
                pos_pulsar,
                utc.to_f64(),
                &weather,
            );
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
                freq_ssb = obs.frequency_mhz
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
                    Some(obs.original.line),
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
                index: obs.original.index,
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
                obs.original.index, obs.original.name, obs.sat_mjd, error.message
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
            "JPL low-level reader from TEMPO2 fda6ad4; initialized header state and recoverable read cache; native/departures.patch",
        ],
    })
}
