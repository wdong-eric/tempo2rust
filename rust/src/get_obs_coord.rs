//! TEMPO2 get_obsCoord.C::get_obsCoord_IAU2000B, selected PKS geometry.
//! Retained SOFA: sofa/{pom00,rxp,sxp,pxp,c2t00b,trxpv,trxp}.for routines
//! iau_POM00/RXP/SXP/PXP/C2T00B/TRXPV/TRXP and their pinned dependency closure.
use crate::{TimingError, runtime::Observatory};
use rug::Float;
// Necessity: retain the accepted SOFA Fortran implementation through its C ABI.
// Safety contract: by-reference f64 scalars and contiguous column-major arrays
// of 3/6/9 elements; no pointers escape. The call site owns all output storage.
unsafe extern "C" {
    fn iau_pom00_(xp: *const f64, yp: *const f64, sp: *const f64, r: *mut f64);
    fn iau_rxp_(r: *const f64, p: *const f64, rp: *mut f64);
    fn iau_sxp_(s: *const f64, p: *const f64, sp: *mut f64);
    fn iau_pxp_(a: *const f64, b: *const f64, axb: *mut f64);
    fn iau_c2t00b_(
        tta: *const f64,
        ttb: *const f64,
        uta: *const f64,
        utb: *const f64,
        xp: *const f64,
        yp: *const f64,
        r: *mut f64,
    );
    fn iau_trxpv_(r: *const f64, pv: *const f64, trpv: *mut f64);
    fn iau_trxp_(r: *const f64, p: *const f64, trp: *mut f64);
}
pub(crate) struct SiteGeometry {
    pub observatory_earth: [f64; 3],
    pub site_velocity: [f64; 3],
    pub zenith: [f64; 3],
}
pub(crate) fn get_obs_coord(
    obs: &Observatory,
    tt: &Float,
    utc: &Float,
    eop: [f64; 4],
) -> Result<SiteGeometry, TimingError> {
    let [xp, yp, dut1, dut1dot] = eop;
    let tt_jd = tt.clone() + 2400000.5f64;
    let tt1 = tt_jd.clone().trunc().to_f64();
    let tt2 = (tt_jd - tt1).to_f64();
    let ut1_jd = utc.clone() + (dut1 / 86400.0f64) + 2400000.5f64;
    let ut1 = ut1_jd.clone().trunc().to_f64();
    let ut2 = (ut1_jd - ut1).to_f64();
    let mut terrestrial_state = [0.0; 6];
    terrestrial_state[..3].copy_from_slice(&obs.xyz);
    let zenith_terrestrial = [
        obs.height * obs.longitude.cos() * obs.latitude.cos(),
        obs.height * obs.longitude.sin() * obs.latitude.cos(),
        obs.height * obs.latitude.sin(),
    ];
    let mut polar_motion_matrix = [0.0; 9];
    let mut rotation_pole = [0.0; 3];
    let mut angular_velocity = [0.0; 3];
    let north = [0.0, 0.0, 1.0];
    #[allow(clippy::excessive_precision)] // Retain the pinned source's literal.
    let eradot = 2.0 * std::f64::consts::PI * 1.00273781191135448 * (1.0 + dut1dot) / 86400.0f64;
    let mut celestial_to_terrestrial = [0.0; 9];
    let mut celestial_state = [0.0; 6];
    let mut result = SiteGeometry {
        observatory_earth: [0.0; 3],
        site_velocity: [0.0; 3],
        zenith: [0.0; 3],
    };
    // Necessity: SOFA rotations are retained native numerical dependencies.
    // SAFETY: all f64 pointers are aligned and live for the calls; matrices/vectors
    // have the required 9/6/3 initialized elements. Output arrays are exclusive.
    // pxp reads terrestrial_state[0..3] and writes its disjoint [3..6] slice;
    // all other input/output buffers are distinct. Fortran retains no pointers,
    // and -frecursive gives each call its own local state (see build.rs).
    unsafe {
        iau_pom00_(&xp, &yp, &0.0, polar_motion_matrix.as_mut_ptr());
        iau_rxp_(
            polar_motion_matrix.as_ptr(),
            north.as_ptr(),
            rotation_pole.as_mut_ptr(),
        );
        iau_sxp_(
            &eradot,
            rotation_pole.as_ptr(),
            angular_velocity.as_mut_ptr(),
        );
        iau_pxp_(
            angular_velocity.as_ptr(),
            terrestrial_state.as_ptr(),
            terrestrial_state.as_mut_ptr().add(3),
        );
        iau_c2t00b_(
            &tt1,
            &tt2,
            &ut1,
            &ut2,
            &xp,
            &yp,
            celestial_to_terrestrial.as_mut_ptr(),
        );
        iau_trxpv_(
            celestial_to_terrestrial.as_ptr(),
            terrestrial_state.as_ptr(),
            celestial_state.as_mut_ptr(),
        );
        iau_trxp_(
            celestial_to_terrestrial.as_ptr(),
            zenith_terrestrial.as_ptr(),
            result.zenith.as_mut_ptr(),
        );
        iau_sxp_(
            &(1.0 / 299792458.0),
            celestial_state.as_ptr(),
            result.observatory_earth.as_mut_ptr(),
        );
        iau_sxp_(
            &(1.0 / 299792458.0),
            celestial_state.as_ptr().add(3),
            result.site_velocity.as_mut_ptr(),
        );
    }
    Ok(result)
}
