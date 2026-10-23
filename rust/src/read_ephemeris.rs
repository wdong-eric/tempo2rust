//! Selected readEphemeris.C::readEphemeris body/epoch/scaling responsibility.
//! jpleph.rs supplies the source reader; ownership and error returns are adapters.
use crate::{
    PRECISION_BITS, TimingError,
    ifteph::IFTE_KM1,
    jpleph::{Ephemeris, jpl_init_error_code},
};
use rug::Float;
use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::Mutex,
};

// Preserve an initialization and its global status query as a sequential pair.
static JPL_INIT_LOCK: Mutex<()> = Mutex::new(());

pub(crate) struct JplEphemeris {
    handle: Ephemeris,
    path: PathBuf,
    pub start_jd: f64,
    pub end_jd: f64,
    one_au_m: f64,
}

pub(crate) struct EphemerisState {
    pub sun_ssb: [f64; 6],
    pub earth_ssb: [f64; 6],
    /// Venus, Jupiter, Saturn, Uranus, Neptune.
    pub planets_earth: [[f64; 6]; 5],
}

pub(crate) fn read_ephemeris(
    jpl: &mut JplEphemeris,
    jd: f64,
) -> Result<EphemerisState, TimingError> {
    // Keep readEphemeris.C's call order, including the otherwise unused Sun/Earth,
    // EMB and nutation reads; reader cache state then follows the source passes.
    let sun_ssb = jpl.interpolate_state(jd, 11, 12)?;
    jpl.interpolate_state(jd, 11, 3)?;
    let mut planets_earth = [[0.0; 6]; 5];
    for (index, target) in [(1, 5), (2, 6), (0, 2), (3, 7), (4, 8)] {
        planets_earth[index] = jpl.interpolate_state(jd, target, 3)?;
    }
    let earth_ssb = jpl.interpolate_state(jd, 3, 12)?;
    jpl.interpolate_state(jd, 13, 12)?;
    jpl.interpolate_state(jd, 13, 3)?;
    jpl.interpolate_state(jd, 14, 3)?;
    Ok(EphemerisState {
        sun_ssb,
        earth_ssb,
        planets_earth,
    })
}

impl JplEphemeris {
    pub fn open(path: &Path, file: File) -> Result<Self, TimingError> {
        let error = |message: String| TimingError::new(path, None, "JPL", message);
        let _guard = JPL_INIT_LOCK
            .lock()
            .map_err(|_| error("JPL initialization lock poisoned".into()))?;
        let handle = Ephemeris::init(Ok(file), None, None)
            .map_err(|e| error(e.to_string()))?
            .ok_or_else(|| error(format!("initialization error {}", jpl_init_error_code())))?;
        let start_jd = handle.ephem_start;
        let end_jd = handle.ephem_end;
        let one_au_m = handle.get_double(28).map_err(|e| error(e.to_string()))? * 1000.0;
        // readEphemeris.C: one_au *= IFTE_K promotes the product to longdouble.
        let k = Float::with_val(PRECISION_BITS, 1.0) + IFTE_KM1;
        let one_au_m = (Float::with_val(PRECISION_BITS, one_au_m) * k).to_f64();
        Ok(Self {
            handle,
            path: path.into(),
            start_jd,
            end_jd,
            one_au_m,
        })
    }
    pub fn interpolate_state(
        &mut self,
        jd: f64,
        target: i32,
        centre: i32,
    ) -> Result<[f64; 6], TimingError> {
        // NaN bypasses jpl_state's range comparisons and reaches the same
        // unchecked record conversion; reject it at the timing adapter.
        if !jd.is_finite() {
            return Err(TimingError::new(
                &self.path,
                None,
                "JPL",
                "nonfinite interpolation epoch",
            ));
        }
        let mut result = [0.0; 6];
        let code = self
            .handle
            .pleph(jd, target, centre, &mut result, 1)
            .map_err(|e| TimingError::new(&self.path, None, "JPL", e.to_string()))?;
        // readEphemeris.C ignores jpl_pleph's status and may publish stale vectors.
        // Reader read/range failures are errors here; no vector escapes on failure.
        if code != 0 {
            return Err(TimingError::new(
                &self.path,
                None,
                "JPL",
                format!("JPL error {code} at JD {jd}, target {target}, centre {centre}"),
            ));
        }
        if target != 14 {
            for x in &mut result[..3] {
                *x *= self.one_au_m / 299792458.0;
            }
            for x in &mut result[3..] {
                *x *= self.one_au_m / 86400.0 / 299792458.0;
            }
        }
        if result.iter().any(|v| !v.is_finite()) {
            return Err(TimingError::new(
                &self.path,
                None,
                "JPL",
                "nonfinite interpolated vector",
            ));
        }
        Ok(result)
    }
}
