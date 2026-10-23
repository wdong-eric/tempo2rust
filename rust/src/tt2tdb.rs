// TEMPO2 tt2tdb.C::tt2tb, restricted to the accepted IF99 SI/TCB branch.
// correction_tt_teph/correction_tt_tb/einstein_rate retain source roles. GPL-3.0-or-later.
use crate::{
    PRECISION_BITS, TimingError,
    ifteph::{IFTE_KM1, IFTE_LC, IFTE_MJD0, IFTE_TEPH0, IfteEphemeris, split_ifte_epoch},
    tempo2_util::dot_product,
};
use rug::Float;
pub(crate) struct TimeCorrection {
    pub correction_tt_teph: Float,
    pub correction_tt_tb: Float,
    pub einstein_rate: f64,
}
pub(crate) fn tt2tb(
    ifte: &IfteEphemeris,
    mjd_tt: &Float,
    earth_velocity: [f64; 3],
    observatory_earth: [f64; 3],
    site_velocity: [f64; 3],
) -> Result<TimeCorrection, TimingError> {
    let (jd0, jd1) = split_ifte_epoch(mjd_tt);
    let delta_t = ifte.get_values(jd0, jd1, false)?[0] * 86400.0;
    let k = Float::with_val(PRECISION_BITS, 1.0) + IFTE_KM1;
    let mut observatory_term = dot_product(earth_velocity, observatory_earth) / (1.0 - IFTE_LC);
    observatory_term /= Float::with_val(PRECISION_BITS, &k * &k).to_f64();
    let correction_tt_teph = Float::with_val(
        PRECISION_BITS,
        IFTE_TEPH0 + observatory_term + delta_t / (1.0 - IFTE_LC),
    );
    let drift = IFTE_KM1 * (mjd_tt.clone() - IFTE_MJD0).to_f64() * 86400.0;
    let correction_tt_tb = Float::with_val(PRECISION_BITS, drift)
        + (k.clone() * (correction_tt_teph.clone() - IFTE_TEPH0));
    let mjd_teph = mjd_tt.clone() + correction_tt_teph.clone() / 86400.0f64;
    let (jd0, jd1) = split_ifte_epoch(&mjd_teph);
    let delta_t_dot = ifte.get_values(jd0, jd1, false)?[1];
    let earth_velocity_and_derivative = ifte.get_values(jd0, jd1, true)?;
    let acceleration = [
        earth_velocity_and_derivative[3] * (1.0 / 86400.0f64),
        earth_velocity_and_derivative[4] * (1.0 / 86400.0f64),
        earth_velocity_and_derivative[5] * (1.0 / 86400.0f64),
    ];
    let observatory_term_dot = (dot_product(acceleration, observatory_earth)
        + dot_product(earth_velocity, site_velocity))
        / (1.0 - IFTE_LC);
    // Source subsequently scales obsTerm again, not obsTermDot. Preserve the
    // rate actually used by dm_delays; do not silently "correct" that convention.
    let einstein_rate = (k * (1.0 + observatory_term_dot + delta_t_dot / (1.0 - IFTE_LC))).to_f64();
    Ok(TimeCorrection {
        correction_tt_teph,
        correction_tt_tb,
        einstein_rate,
    })
}
