//! tropo.C at fda6ad4: all twelve meteorology, Niell and driver routines.
//! Shared tables live in ClockSession; direct wrappers retain caller-owned state.
use crate::{
    PRECISION_BITS, Scalar, TimingError,
    clkcorr::ClockSession,
    clock_diagnostics::{Diagnostics, boundary, terminal},
    clock_text::{self, c_bytes},
    observation::{read, read_string},
    pulsar::SourcePulsar,
    pulsar_lifecycle::PulsarContext,
    tabulated_function::TabulatedFunction,
    tempo2::SourceObservation,
};
use rug::Float;
use serde::Serialize;
use std::{
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

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

#[derive(Default, Clone, Debug)]
pub(crate) struct MeteorologyFunction {
    pub table: TabulatedFunction,
    pub site: Vec<u8>, // known initialized prefix of siteName[256], including tails
}
impl MeteorologyFunction {
    pub fn load(&mut self, path: &Path, diagnostics: &mut Diagnostics) -> Result<(), TimingError> {
        self.table.load(path, diagnostics)?;
        let Some(site) = clock_text::tokens(&self.table.header[1..]).next() else {
            diagnostics.stderr(
                &[
                    b"Error parsing meterology file ".as_slice(),
                    path.as_os_str().as_bytes(),
                    b": first line must be of form # site_name\n",
                ]
                .concat(),
            );
            return Err(terminal(path, "meteorology_site"));
        };
        // load() guarantees a '#' header of at most 255 bytes; this token
        // therefore fits siteName[256], including its NUL terminator.
        self.site.resize(self.site.len().max(site.len() + 1), 0);
        self.site[..site.len()].copy_from_slice(site);
        self.site[site.len()] = 0;
        Ok(())
    }
    pub fn get_value(&self, mjd: f64, diagnostics: &mut Diagnostics) -> Result<f64, TimingError> {
        self.table.get_value(mjd, diagnostics)
    }
    pub fn start(&self) -> Result<f64, TimingError> {
        self.table.start()
    }
    pub fn end(&self) -> Result<f64, TimingError> {
        self.table.end()
    }
}
#[derive(Default)]
pub(crate) struct Meteorology {
    pub initialized: bool,
    pub wet: Vec<MeteorologyFunction>,
    pub pressure: Vec<MeteorologyFunction>,
}
pub(crate) fn initialize_meteorology_table(
    root: Option<&Path>,
    warnings: i32,
    path: &[u8],
    extension: &[u8],
    tables: &mut Vec<MeteorologyFunction>,
    description: &[u8],
    diagnostics: &mut Diagnostics,
) -> Result<(), TimingError> {
    tables.clear();
    let root = root.ok_or_else(|| boundary("meteorology_environment_storage"))?;
    let pattern = [
        root.as_os_str().as_bytes(),
        b"/",
        c_bytes(path),
        b"/*.",
        c_bytes(extension),
    ]
    .concat();
    if pattern.len() >= 1024 {
        return Err(boundary("meteorology_path_storage"));
    }
    let paths = crate::clock_glob::files(&pattern);
    if paths.is_empty() {
        if warnings == 0 {
            diagnostics.stdout(
                &[
                    b"No .".as_slice(),
                    extension,
                    b" files in $TEMPO2/",
                    path,
                    b"\n",
                ]
                .concat(),
            );
            return Ok(());
        }
        // tropo.C:281-288 continues through null gl_pathv on this host. No
        // native successful value exists; the caller may retry initialization.
        return Err(boundary("meteorology_glob_nomatch"));
    }
    let mut func = MeteorologyFunction::default();
    for file in paths {
        func.load(&file, diagnostics)?;
        tables.push(func.clone());
        if warnings == 0 {
            diagnostics.stdout(
                &[
                    b"Loaded ".as_slice(),
                    description,
                    b" for site ",
                    c_bytes(&func.site),
                    b" from ",
                    &func.table.name,
                    b"\n",
                ]
                .concat(),
            );
        }
    }
    Ok(())
}
pub(crate) struct MeteorologicalValue {
    pub value: f64,
    pub reason: &'static str,
    pub resource: Option<PathBuf>,
}
impl MeteorologicalValue {
    fn weather(self, pressure: bool) -> WeatherValue {
        WeatherValue {
            effective_value: Scalar(Float::with_val(PRECISION_BITS, self.value)),
            unit: if pressure { "kPa" } else { "s" },
            reason: self.reason,
            resource: self.resource.map(|p| p.to_string_lossy().into_owned()),
        }
    }
}
impl Meteorology {
    pub fn initialize(
        &mut self,
        root: Option<&Path>,
        warnings: i32,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), TimingError> {
        initialize_meteorology_table(
            root,
            warnings,
            b"atmosphere",
            b"zwd",
            &mut self.wet,
            b"zenith wet delays",
            diagnostics,
        )?;
        initialize_meteorology_table(
            root,
            warnings,
            b"atmosphere",
            b"sap",
            &mut self.pressure,
            b"surface atmospheric pressures",
            diagnostics,
        )?;
        self.initialized = true;
        Ok(())
    }
    pub fn get_value(
        &mut self,
        root: Option<&Path>,
        pressure: bool,
        site: &[u8],
        mjd: f64,
        warnings: i32,
        diagnostics: &mut Diagnostics,
    ) -> Result<MeteorologicalValue, TimingError> {
        if !self.initialized {
            self.initialize(root, warnings, diagnostics)?;
        }
        let tables = if pressure { &self.pressure } else { &self.wet };
        let mut seen = false;
        for f in tables {
            if c_bytes(&f.site).eq_ignore_ascii_case(c_bytes(site)) {
                seen = true;
                if f.start()? <= mjd && f.end()? >= mjd {
                    let value = f.get_value(mjd, diagnostics)?;
                    return Ok(MeteorologicalValue {
                        value,
                        reason: if value == 0.0 {
                            "zero table value"
                        } else {
                            "table interpolation"
                        },
                        resource: Some(f.table.path.clone()),
                    });
                }
            }
        }
        Ok(MeteorologicalValue {
            value: 0.0,
            reason: if seen {
                "epoch outside weather coverage"
            } else {
                "no weather table for site"
            },
            resource: None,
        })
    }
    pub fn wet(
        &mut self,
        root: Option<&Path>,
        site: &[u8],
        mjd: f64,
        warnings: i32,
        diagnostics: &mut Diagnostics,
    ) -> Result<MeteorologicalValue, TimingError> {
        let value = self.get_value(root, false, site, mjd, warnings, diagnostics)?;
        if value.value == 0.0 {
            if site.len() + 64 >= 1000 {
                return Err(boundary("meteorology_message_storage"));
            }
            diagnostics.display(
                3,
                b"TROP1",
                &[
                    b"Assume zero zenith wet delay (no data) for site ".as_slice(),
                    site,
                    b"  at MJD",
                ]
                .concat(),
                &clock_text::format_number(mjd, clock_text::NumberFormat::One),
                warnings,
            )?;
        }
        Ok(value)
    }
    pub fn pressure(
        &mut self,
        root: Option<&Path>,
        site: &[u8],
        mjd: f64,
        warnings: i32,
        diagnostics: &mut Diagnostics,
    ) -> Result<MeteorologicalValue, TimingError> {
        let mut value = self.get_value(root, true, site, mjd, warnings, diagnostics)?;
        if value.value == 0.0 {
            if site.len() + 74 >= 1000 {
                return Err(boundary("meteorology_message_storage"));
            }
            diagnostics.display(
                3,
                b"TROP2",
                &[
                    b"Assume standard atmospheric pressure (no data) for site ".as_slice(),
                    site,
                    b" at MJD",
                ]
                .concat(),
                &clock_text::format_number(mjd, clock_text::NumberFormat::One),
                warnings,
            )?;
            value.value = 101.325;
        }
        Ok(value)
    }
}

fn latitude_interval(latitude: f64) -> Result<(usize, usize, f64), TimingError> {
    let absolute = latitude.abs() * 180.0 / std::f64::consts::PI;
    let converted = (absolute / 15.0).floor();
    if !converted.is_finite() || converted < i32::MIN as f64 + 1.0 || converted > i32::MAX as f64 {
        return Err(boundary("nmf_latitude_integer"));
    }
    let i = converted as i32 - 1;
    if i < 0 {
        Ok((0, 0, 0.0))
    } else if i >= 4 {
        Ok((4, 4, 0.0))
    } else {
        Ok((i as usize, i as usize + 1, absolute / 15.0 - 1.0 - i as f64))
    }
}
fn niell_mapping(sin_elevation: f64, a: f64, b: f64, c: f64) -> f64 {
    (1.0 + a / (1.0 + b / (1.0 + c)))
        / (sin_elevation + a / (sin_elevation + b / (sin_elevation + c)))
}
pub(crate) fn nmf_hydrostatic(
    utc: f64,
    latitude: f64,
    height: f64,
    elevation: f64,
) -> Result<f64, TimingError> {
    let averages: [[f64; 5]; 3] = [
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
    let amplitudes: [[f64; 5]; 3] = [
        [0.0, 1.2709626e-5, 2.6523662e-5, 3.4000452e-5, 4.1202191e-5],
        [0.0, 2.1414979e-5, 3.0160779e-5, 7.2562722e-5, 11.723375e-5],
        [0.0, 0.0128400e-5, 4.3497037e-5, 84.795348e-5, 170.37206e-5],
    ];
    let (i, j, f) = latitude_interval(latitude)?;
    let cos_phase = ((utc - 53398.0) * 2.0 * std::f64::consts::PI / 365.25).cos()
        * if latitude < 0.0 { -1.0 } else { 1.0 };
    // tropo.C:116-127: only the coefficient fmadd/fmsub sites emitted by
    // GCC16 -O2 are fused; W arithmetic evidence includes bounded witnesses.
    let mut mapping_coefficients = [0.0; 3];
    for k in 0..3 {
        mapping_coefficients[k] = if i == j {
            (-amplitudes[k][i]).mul_add(cos_phase, averages[k][i])
        } else {
            (-f.mul_add(amplitudes[k][j], (1.0 - f) * amplitudes[k][i])).mul_add(
                cos_phase,
                f.mul_add(averages[k][j], (1.0 - f) * averages[k][i]),
            )
        };
    }
    let sin_elevation = elevation.sin();
    let mapping_hydrostatic = niell_mapping(
        sin_elevation,
        mapping_coefficients[0],
        mapping_coefficients[1],
        mapping_coefficients[2],
    );
    // tropo.C:156 fuses the height correction product with the basic value.
    Ok((height * 1.0e-3).mul_add(
        1.0 / sin_elevation - niell_mapping(sin_elevation, 2.53e-5, 5.49e-3, 1.14e-3),
        mapping_hydrostatic,
    ))
}
pub(crate) fn nmf_wet(latitude: f64, elevation: f64) -> Result<f64, TimingError> {
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
    let (i, j, f) = latitude_interval(latitude)?;
    let (a, b, c) = if i == j {
        (a[i], b[i], c[i])
    } else {
        // Fix TEMPO2 fda6ad4 tropo.C::NMF_wet:206: the upper c coefficient
        // incorrectly comes from bs, breaking continuity at latitude knots.
        // Interpolate cs at both ends. W's disposable native one-line patch,
        // explicit changed-field contract and knot-continuity regression retain
        // the original witness. Source O2 contractions remain expression-local.
        (
            f.mul_add(a[j], (1.0 - f) * a[i]),
            f.mul_add(b[j], (1.0 - f) * b[i]),
            f.mul_add(c[j], (1.0 - f) * c[i]),
        )
    };
    Ok(niell_mapping(elevation.sin(), a, b, c))
}

/// Borrowed canonical fields with explicit inputs for the established frontend.
pub(crate) struct TropospherePulsar<'a> {
    pub source: &'a SourcePulsar,
    pub position: Option<[f64; 3]>,
    pub warnings: Option<i32>,
}
pub(crate) fn compute_observation(
    session: &mut ClockSession,
    psr: &TropospherePulsar<'_>,
    obs: &mut SourceObservation,
    index: usize,
    warned: &mut bool,
) -> Result<Option<Weather>, TimingError> {
    obs.troposphericDelay = Some(0.0);
    let correct = *read(&obs.delayCorr, "delayCorr")? != 0
        && *read(&psr.source.correctTroposphere, "correctTroposphere")? != 0;
    if correct && *read(&obs.zenith[2], "zenith[2]")? == 0.0 && !*warned {
        session.diagnostics.log(b'd',"tropo.C",408,format!("WARNING: Tropospheric delay correction not possible with T2CMETHOD TEMPO {index} {:.6}",read(&obs.zenith[2],"zenith[2]")?).as_bytes());
        *warned = true;
    } else if read_string(&obs.telID, "telID")? == b"STL_FBAT" {
        session.diagnostics.log(
            b'd',
            "tropo.C",
            413,
            b"Not correcting for tropospheric delay",
        );
        *warned = true;
    } else if correct {
        let site = session.observatories.get(
            &read_string(&obs.telID, "telID")?,
            session.root.as_deref(),
            session.alias.as_deref(),
            &mut session.diagnostics,
        )?;
        if !site.name()?.eq_ignore_ascii_case(b"COE") {
            let position = match psr.position {
                Some(p) => p,
                None => [
                    *read(&psr.source.posPulsarEquatorial[0], "posPulsarEquatorial[0]")?,
                    *read(&psr.source.posPulsarEquatorial[1], "posPulsarEquatorial[1]")?,
                    *read(&psr.source.posPulsarEquatorial[2], "posPulsarEquatorial[2]")?,
                ],
            };
            let zenith = [
                *read(&obs.zenith[0], "zenith[0]")?,
                *read(&obs.zenith[1], "zenith[1]")?,
                *read(&obs.zenith[2], "zenith[2]")?,
            ];
            let elevation =
                (crate::tempo2_util::dot_product(zenith, position) / site.height).asin();
            let warnings = psr
                .warnings
                .or(psr.source.noWarnings)
                .ok_or_else(|| boundary("troposphere_unwritten_warnings"))?;
            let sat = read(&obs.sat, "sat")?.to_f64();
            let code = site.code()?;
            let pressure = session.meteorology.pressure(
                session.root.as_deref(),
                &code,
                sat,
                warnings,
                &mut session.diagnostics,
            )?;
            // tropo.C:434-435 and :450 use two fmsub and one fmadd in U's
            // O2 profile; preserve these expression-specific boundaries.
            let hydrostatic = 0.02268 * pressure.value
                / (299792458.0
                    * (-2.8e-7f64)
                        .mul_add(site.height, (-0.00266f64).mul_add(site.latitude.cos(), 1.0)));
            let from = read_string(&psr.source.clockFromOverride[..], "clockFromOverride")?;
            let correction =
                session.run(|s| s.get_correction(obs, Some(&from), Some(b"UTC"), warnings))?;
            // Source adds the binary64 correction/SECDAY to longdouble SAT before
            // the NMF double argument conversion. Rug128 remains the backend.
            let utc = (read(&obs.sat, "sat")?.clone() + correction / 86400.0).to_f64();
            let mh = nmf_hydrostatic(utc, site.latitude, site.height, elevation)?;
            let wet = session.meteorology.wet(
                session.root.as_deref(),
                &code,
                sat,
                warnings,
                &mut session.diagnostics,
            )?;
            let mw = nmf_wet(site.latitude, elevation)?;
            obs.troposphericDelay = Some(hydrostatic.mul_add(mh, wet.value * mw));
            return Ok(Some(Weather {
                pressure: pressure.weather(true),
                zenith_wet_delay: wet.weather(false),
            }));
        }
    }
    Ok(None)
}
/// Full source signed loops on T's canonical owners, including shared views.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "full driver precedes broader frontend support")
)]
pub(crate) fn compute_tropospheric_delays(
    session: &mut ClockSession,
    context: &mut PulsarContext,
    npsr: i32,
) -> Result<(), TimingError> {
    session
        .diagnostics
        .version("tropo.C", "computer_tropospheric_delays()");
    for p in 0..npsr {
        let (psr, observations) = context.pulsar_observations_mut(p as usize)?;
        let nobs = *read(&psr.nobs, "nobs")?;
        let mut warned = false;
        for i in 0..nobs {
            let obs = observations
                .get_mut(i as usize)
                .ok_or_else(|| boundary("troposphere_observation_storage"))?;
            compute_observation(
                session,
                &TropospherePulsar {
                    source: psr,
                    position: None,
                    warnings: None,
                },
                obs,
                i as usize,
                &mut warned,
            )?;
        }
    }
    Ok(())
}
