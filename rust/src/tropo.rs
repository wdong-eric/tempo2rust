// Selected tropo.C::getMeteorologicalValue/getZenithWetDelay/
// getSurfaceAtmosphericPressure/NMF_hydrostatic/NMF_wet/compute_tropospheric_delays.
// Rust combines lookup/fallback provenance and evaluates one TOA. GPL-3.0-or-later.
use crate::{
    PRECISION_BITS, Scalar, TimingError,
    runtime::{MeteorologyFunction, Observatory},
};
use rug::Float;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WeatherValue {
    pub effective_value: Scalar,
    pub unit: &'static str,
    pub reason: &'static str,
    pub resource: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct Weather {
    pub pressure: WeatherValue,
    pub zenith_wet_delay: WeatherValue,
}

pub(crate) fn get_meteorological_value(
    tables: &[MeteorologyFunction],
    site: &[u8],
    mjd: f64,
    pressure: bool,
    warnings: &mut Vec<String>,
) -> Result<WeatherValue, TimingError> {
    let mut selected = None;
    for table in tables {
        if table.site.eq_ignore_ascii_case(site) && table.table.covers(mjd)? {
            selected = Some(table);
            break;
        }
    }
    let value = if let Some(t) = selected {
        crate::clkcorr::with_diagnostics(|diagnostics| t.table.get_value(mjd, diagnostics))?
    } else {
        0.0
    };
    let reason = if value != 0.0 {
        "table interpolation"
    } else if selected.is_some() {
        "zero table value"
    } else if tables.iter().any(|t| t.site.eq_ignore_ascii_case(site)) {
        "epoch outside weather coverage"
    } else {
        "no weather table for site"
    };
    // Preserve getSurfaceAtmosphericPressure/getZenithWetDelay's zero sentinel,
    // including a measured zero. Decision 0004 explicitly adopts this fallback.
    let effective = if value == 0.0 {
        warnings.push(format!(
            "{}: {} @ {mjd}: {reason}; using {}",
            if pressure { "TROP2" } else { "TROP1" },
            String::from_utf8_lossy(site),
            if pressure {
                "101.325 kPa"
            } else {
                "zero zenith wet delay"
            }
        ));
        if pressure { 101.325 } else { 0.0 }
    } else {
        value
    };
    Ok(WeatherValue {
        effective_value: Scalar(Float::with_val(PRECISION_BITS, effective)),
        unit: if pressure { "kPa" } else { "s" },
        reason,
        resource: selected.map(|t| t.table.path.to_string_lossy().into_owned()),
    })
}

fn latitude_interval(latitude: f64) -> (usize, usize, f64) {
    let absolute = latitude.abs() * 180.0 / std::f64::consts::PI;
    let i = (absolute / 15.0).floor() as i32 - 1;
    if i < 0 {
        (0, 0, 0.0)
    } else if i >= 4 {
        (4, 4, 0.0)
    } else {
        (i as usize, i as usize + 1, absolute / 15.0 - 1.0 - i as f64)
    }
}
fn niell_mapping(sin_elevation: f64, a: f64, b: f64, c: f64) -> f64 {
    (1.0 + a / (1.0 + b / (1.0 + c)))
        / (sin_elevation + a / (sin_elevation + b / (sin_elevation + c)))
}
fn nmf_hydrostatic(utc: f64, latitude: f64, height: f64, elevation: f64) -> f64 {
    let averages = [
        [
            1.2769934e-3,
            1.2683230e-3,
            1.2465397e-3,
            1.2196049e-3,
            1.2045996e-3,
        ],
        [
            2.9153695e-3,
            2.9152299e-3,
            2.9288445e-3,
            2.9022565e-3,
            2.9024912e-3,
        ],
        [
            62.610505e-3,
            62.837393e-3,
            63.721774e-3,
            63.824265e-3,
            64.258455e-3,
        ],
    ];
    let amplitudes = [
        [0.0, 1.2709626e-5, 2.6523662e-5, 3.4000452e-5, 4.1202191e-5],
        [0.0, 2.1414979e-5, 3.0160779e-5, 7.2562722e-5, 11.723375e-5],
        [0.0, 0.0128400e-5, 4.3497037e-5, 84.795348e-5, 170.37206e-5],
    ];
    let (i, j, f) = latitude_interval(latitude);
    let cos_phase = ((utc - 53398.0) * 2.0 * std::f64::consts::PI / 365.25).cos()
        * if latitude < 0.0 { -1.0 } else { 1.0 };
    let mut mapping_coefficients = [0.0; 3];
    for k in 0..3 {
        mapping_coefficients[k] = if i == j {
            averages[k][i] - amplitudes[k][i] * cos_phase
        } else {
            f * averages[k][j] + (1.0 - f) * averages[k][i]
                - (f * amplitudes[k][j] + (1.0 - f) * amplitudes[k][i]) * cos_phase
        };
    }
    let sin_elevation = elevation.sin();
    let mapping_hydrostatic = niell_mapping(
        sin_elevation,
        mapping_coefficients[0],
        mapping_coefficients[1],
        mapping_coefficients[2],
    );
    let height_correction = height
        * 1.0e-3
        * (1.0 / sin_elevation - niell_mapping(sin_elevation, 2.53e-5, 5.49e-3, 1.14e-3));
    mapping_hydrostatic + height_correction
}
fn nmf_wet(latitude: f64, elevation: f64) -> f64 {
    let a = [
        5.8021897e-4,
        5.6794847e-4,
        5.8118019e-4,
        5.9727542e-4,
        6.1641693e-4,
    ];
    let b = [
        1.4275268e-3,
        1.5138625e-3,
        1.4572752e-3,
        1.5007428e-3,
        1.7599082e-3,
    ];
    let c = [
        4.3472961e-2,
        4.6729510e-2,
        4.3908931e-2,
        4.4626982e-2,
        5.4736038e-2,
    ];
    let (i, j, f) = latitude_interval(latitude);
    let (a, b, c) = if i == j {
        (a[i], b[i], c[i])
    } else {
        // Source NMF_wet uses b[j] in the c interpolation. Retained literally:
        // correcting this scientific coefficient requires central review.
        (
            f * a[j] + (1.0 - f) * a[i],
            f * b[j] + (1.0 - f) * b[i],
            f * b[j] + (1.0 - f) * c[i],
        )
    };
    niell_mapping(elevation.sin(), a, b, c)
}

pub(crate) fn compute_tropospheric_delay(
    obs: &Observatory,
    zenith: [f64; 3],
    position: [f64; 3],
    utc: f64,
    weather: &Weather,
) -> f64 {
    let elevation = (crate::tempo2_util::dot_product(zenith, position) / obs.height).asin();
    let pressure = weather.pressure.effective_value.0.to_f64();
    let zenith_delay_hydrostatic = 0.02268 * pressure
        / (299792458.0 * (1.0 - 0.00266 * obs.latitude.cos() - 2.8e-7 * obs.height));
    zenith_delay_hydrostatic * nmf_hydrostatic(utc, obs.latitude, obs.height, elevation)
        + weather.zenith_wet_delay.effective_value.0.to_f64() * nmf_wet(obs.latitude, elevation)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn niell_mapping_matches_pinned_source() {
        // Independent GCC16 -O2 evaluation of the unchanged NMF routines in
        // fda6ad4/tropo.C, including the wet c-coefficient expression.
        assert!(
            (nmf_hydrostatic(50000.0, -0.575959, 415.0, 0.15) - 6.373595076695443).abs() < 2e-14
        );
        assert!((nmf_wet(-0.575959, 0.15) - 6.537036088606678).abs() < 2e-14);
    }
}
