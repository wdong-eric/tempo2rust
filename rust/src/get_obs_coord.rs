//! All six get_obsCoord.C routines at fda6ad4; retained source branch order.
//! Retained SOFA: sofa/{pom00,rxp,sxp,pxp,c2t00b,trxpv,trxp}.for routines
//! iau_POM00/RXP/SXP/PXP/C2T00B/TRXPV/TRXP and their pinned dependency closure.
use crate::{TimingError, runtime::Observatory};
use crate::{
    clkcorr::ClockSession,
    clock_diagnostics::{boundary, terminal},
    observation::{read, read_string},
    pulsar::SourcePulsar,
    pulsar_lifecycle::PulsarContext,
    tempo2::{Label::*, SourceObservation},
};
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
pub(crate) fn iau2000b(
    s: &mut ClockSession,
    trs: [f64; 3],
    zenith_terrestrial: [f64; 3],
    tt: &Float,
    utc: &Float,
    file: &[u8],
) -> Result<(SiteGeometry, f64), TimingError> {
    let tt_jd = tt.clone() + 2400000.5f64;
    let tt1 = crate::ifteph::extended_i32(&tt_jd, "IAU_TT_JD_int")? as f64;
    let tt2 = (tt_jd - tt1).to_f64();
    // Source splits TT before EOP. This is the single validated IAU boundary
    // shared by the complete driver and the strict frontend.
    s.diagnostics
        .version("get_obsCoord.C", "get_obsCoord_IAU2000B()");
    s.eop
        .initialize(s.root.as_deref(), file, &mut s.diagnostics)?;
    let mut eop = [0.0; 4];
    s.eop
        .get_eop(utc.to_f64(), &mut eop, [0, 1, 2, 3], 2, &mut s.diagnostics)?;
    let [xp, yp, dut1, dut1dot] = eop;
    let ut1_jd = utc.clone() + (dut1 / 86400.0f64) + 2400000.5f64;
    let ut1 = crate::ifteph::extended_i32(&ut1_jd, "IAU_UT1_JD_int")? as f64;
    let ut2 = (ut1_jd - ut1).to_f64();
    let mut terrestrial_state = [0.0; 6];
    terrestrial_state[..3].copy_from_slice(&trs);
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
    Ok((result, dut1))
}

/// get_obsCoord.C::remove_white: preserve byte-level branch order and stale tails.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "direct source helper retained for complete observer API"
    )
)]
pub(crate) fn remove_white(bytes: &mut [u8]) -> Result<(), TimingError> {
    let mut end = bytes
        .iter()
        .position(|&b| b == 0)
        .ok_or_else(|| boundary("remove_white_termination"))?;
    let mut i = 0;
    let mut leading = true;
    while i < end {
        let separator = matches!(bytes[i], b' ' | b'\t');
        if (separator && (leading || matches!(bytes[i - 1], b' ' | b'\t'))) || bytes[i] == b'\n' {
            // fda6ad4 get_obsCoord.C::remove_white:464,469,474 uses strcpy on
            // potentially overlapping suffixes (e.g. "  x"). Move the same bytes
            // including NUL safely; retain stale tail bytes. Central corrected
            // memmove buffers and observer_coordinates_tests protect this fix (0053).
            bytes.copy_within(i + 1..=end, i);
            end -= 1;
        } else {
            if !separator {
                leading = false;
            }
            i += 1;
        }
    }
    Ok(())
}

pub(crate) fn lmst(mjd: f64, olong: f64) -> Result<[f64; 2], TimingError> {
    let n = crate::ifteph::native_i32(mjd, "lmst_mjd_integer")?;
    let fmjd = mjd - n as f64;
    let tu0 = (n
        .checked_sub(51545)
        .ok_or_else(|| boundary("lmst_mjd_subtract"))? as f64
        + 0.5)
        / 3.6525e4;
    let dtu = fmjd / 3.6525e4;
    let tu = tu0 + dtu;
    let a = 24110.54841;
    let b = 8640184.812866;
    let c = 0.093104;
    let d = -6.2e-6;
    // fda6ad4 get_obsCoord.C::lmst:517-529: GCC16 -O2 contracts the
    // Horner terms and three-product sum. AA arithmetic evidence retains the
    // actual assembly and c008 witness; no global contraction is enabled.
    let gmst0 = tu0.mul_add(tu0.mul_add(tu0.mul_add(d, c), b), a) / 86400.0;
    let seconds_per_jc = 86400.0 * 36525.0;
    let bprime = 1.0 + b / seconds_per_jc;
    let cprime = 2.0 * c / seconds_per_jc;
    let dprime = 3.0 * d / seconds_per_jc;
    let sdd = tu.mul_add(tu.mul_add(dprime, cprime), bprime);
    let sum = tu0.mul_add(tu0, tu.mul_add(tu, tu * tu0));
    let gst = gmst0 + dtu * sum.mul_add(d, (tu + tu0).mul_add(c, seconds_per_jc + b)) / 86400.0;
    let xlst = gst - olong / 360.0;
    let mut xlst =
        crate::tempo2_util::fortran_mod(Float::with_val(128, xlst), &Float::with_val(128, 1))?
            .to_f64();
    if xlst < 0.0 {
        xlst += 1.0;
    }
    Ok([xlst, sdd])
}
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "direct source helper retained for complete observer API"
    )
)]
pub(crate) fn ang(i: i32, f: f64) -> Result<f64, TimingError> {
    let ia = crate::ifteph::native_i32((f / 1.0e4).floor(), "ang_degrees_integer")? as f64;
    let ib = crate::ifteph::native_i32(((f - ia * 1.0e4) / 1.0e2).floor(), "ang_minutes_integer")?
        as f64;
    let g = f - ia * 1.0e4 - ib * 1.0e2;
    let mut angle = (ia + (ib + g / 6.0e1) / 6.0e1) / 36.0e1;
    if i > 2 {
        angle *= 15.0;
    }
    if i == 1 || i == 3 {
        angle = angle * std::f64::consts::PI * 2.0;
    }
    Ok(angle)
}
pub(crate) fn get_precession_matrix(mjd: f64, delp: f64, dele: f64) -> [[f64; 3]; 3] {
    let t = (mjd - 51544.5) / 36525.0;
    let seconds_per_rad = 3600.0 * 180.0 / std::f64::consts::PI;
    // fda6ad4 get_obsCoord.C::get_precessionMatrix:568-571,587-598,629:
    // retain GCC16 O2's bounded Horner/product contractions (AA c010 witness).
    let zeta = t * t.mul_add(t.mul_add(0.017998, 0.30188), 2306.2181) / seconds_per_rad;
    let z = t * t.mul_add(t.mul_add(0.018203, 1.09468), 2306.2181) / seconds_per_rad;
    let theta = t * t.mul_add(t.mul_add(-0.041833, -0.42665), 2004.3109) / seconds_per_rad;
    let czeta = zeta.cos();
    let szeta = zeta.sin();
    let cz = z.cos();
    let sz = z.sin();
    let ctheta = theta.cos();
    let stheta = theta.sin();
    let prc = [
        [
            (czeta * ctheta).mul_add(cz, -(szeta * sz)),
            (-szeta * ctheta).mul_add(cz, -(czeta * sz)),
            -stheta * cz,
        ],
        [
            (czeta * ctheta).mul_add(sz, szeta * cz),
            (-szeta * ctheta).mul_add(sz, czeta * cz),
            -stheta * sz,
        ],
        [czeta * stheta, -szeta * stheta, ctheta],
    ];
    #[allow(clippy::excessive_precision)]
    let eps = 23.4458333333333333 * std::f64::consts::PI / 180.0;
    let ceps = eps.cos();
    let seps = eps.sin();
    let n01 = -delp * ceps;
    let n02 = -delp * seps;
    let n12 = -dele;
    let nut = [[1.0, n01, n02], [-n01, 1.0, n12], [-n02, -n12, 1.0]];
    std::array::from_fn(|j| {
        std::array::from_fn(|i| {
            nut[i][2].mul_add(
                prc[2][j],
                nut[i][0].mul_add(prc[0][j], nut[i][1] * prc[1][j]),
            )
        })
    })
}

/// Full source driver; rows and pulsar remain owned by the same scoped context.
pub(crate) fn driver(
    s: &mut ClockSession,
    c: &mut PulsarContext,
    npsr: i32,
) -> Result<(), TimingError> {
    s.diagnostics.version("get_obsCoord.C", "get_obsCoord()");
    for p in 0..npsr {
        let (psr, rows) = c.pulsar_observations_mut(p as usize)?;
        for i in 0..*read(&psr.nobs, "nobs")? {
            let obs = rows
                .get_mut(i as usize)
                .ok_or_else(|| boundary("get_obs_coord_observation_storage"))?;
            if *read(&obs.delayCorr, "delayCorr")? != 0 {
                compute_observation(s, psr, obs)?;
            } else if i == 0 && *read(&psr.noWarnings, "noWarnings")? == 0 {
                s.diagnostics
                    .stdout(format!("Delay correction turned off for psr {p}\n").as_bytes());
            }
        }
    }
    Ok(())
}

fn compute_observation(
    s: &mut ClockSession,
    p: &mut SourcePulsar,
    o: &mut SourceObservation,
) -> Result<(), TimingError> {
    let tel = read_string(&o.telID[..], "telID")?;
    if tel == b"STL" || tel == b"STL_FBAT" {
        o.siteVel.fill(Some(0.0));
        for (axis, label) in [param_telx, param_tely, param_telz].into_iter().enumerate() {
            let par = p.param.parameter(label);
            let cell = |k| {
                par.cell(k)
                    .map_err(|_| boundary("satellite_parameter_storage"))
            };
            if cell(0)?.param_set() == 1 {
                let epoch = p
                    .param
                    .parameter(param_telEpoch)
                    .cell(0)
                    .map_err(|_| boundary("satellite_epoch_storage"))?;
                let t0 = if epoch.param_set() == 1 {
                    epoch.value().clone()
                } else {
                    Float::with_val(128, 0)
                };
                let delta_t = (read(&o.sat, "sat")?.clone() - t0) * 86400.0f64;
                let mut arg = delta_t.clone();
                let mut pos = cell(0)?.value().clone();
                if cell(1)?.param_set() == 1 {
                    pos += Float::with_val(128, cell(1)?.value() * &arg);
                    if axis == 0 && epoch.param_set() == 0 {
                        s.diagnostics
                            .stdout(b"ERROR: Using telescope velocity without setting telEpoch\n");
                        return Err(terminal(
                            std::path::Path::new(""),
                            "satellite_x_velocity_epoch",
                        ));
                    }
                }
                arg *= &delta_t;
                if cell(2)?.param_set() == 1 {
                    pos += (cell(2)?.value().clone() * 0.5f64) * &arg;
                }
                arg *= &delta_t;
                if cell(3)?.param_set() == 1 {
                    pos += (Float::with_val(128, 1) / 6) * cell(3)?.value() * &arg;
                }
                o.observatory_earth[axis] = Some(pos.to_f64());
            }
        }
        for k in 0..*read(&o.nFlags, "nFlags")? {
            let id = read_string(
                o.flagID
                    .get(k as usize)
                    .ok_or_else(|| boundary("satellite_flags_storage"))?,
                "flagID",
            )?;
            for (axis, name) in [b"-telx", b"-tely", b"-telz"].into_iter().enumerate() {
                if id == name {
                    let value = read_string(&o.flagVal[k as usize], "flagVal")?;
                    if let Some((v, _)) = crate::clock_text::number_prefix(&value) {
                        o.observatory_earth[axis] = Some(v);
                    }
                    if tel == b"STL" {
                        o.observatory_earth[axis] = Some(
                            *read(&o.observatory_earth[axis], "observatory_earth")? / 299792458.0,
                        );
                    }
                }
            }
        }
        for (axis, (count, t, v, label)) in [
            (p.nTelDX, &p.telDX_t, &p.telDX_v, param_tel_dx),
            (p.nTelDY, &p.telDY_t, &p.telDY_v, param_tel_dy),
            (p.nTelDZ, &p.telDZ_t, &p.telDZ_v, param_tel_dz),
        ]
        .into_iter()
        .enumerate()
        {
            let count = *read(&count, "nTelD")?;
            if count > 0 {
                let mut ival = 0.0;
                for k in 0..count - 1 {
                    let k = k as usize;
                    let lo = *read(
                        t.get(k)
                            .ok_or_else(|| boundary("satellite_offset_backing"))?,
                        "telD_t",
                    )?;
                    let hi = *read(
                        t.get(k + 1)
                            .ok_or_else(|| boundary("satellite_offset_backing"))?,
                        "telD_t",
                    )?;
                    let sat = read(&o.sat, "sat")?.to_f64();
                    if sat >= lo && sat < hi {
                        let vl = *read(&v[k], "telD_v")?;
                        let vh = *read(&v[k + 1], "telD_v")?;
                        // get_obsCoord.C:255-260,282-287,310-315: native O2 contracts
                        // intercept and evaluation separately; c114-c123 witness cancellation.
                        let m = (vl - vh) / (lo - hi);
                        let intercept = (-lo).mul_add(m, vl);
                        let mode = p
                            .param
                            .parameter(label)
                            .cell(0)
                            .map_err(|_| boundary("satellite_mode_storage"))?
                            .value();
                        if mode == &0 || mode == &1 {
                            ival = sat.mul_add(m, intercept);
                        } else if mode == &2 {
                            ival = vl;
                        }
                        break;
                    }
                }
                o.observatory_earth[axis] =
                    Some(*read(&o.observatory_earth[axis], "observatory_earth")? + ival);
            }
        }
    } else if tel == b"STL_BAT" {
        o.siteVel.fill(Some(0.0));
        o.observatory_earth[..3].fill(Some(0.0));
        p.correctTroposphere = Some(0);
    } else {
        let index = s.observatories.index(
            &tel,
            s.root.as_deref(),
            s.alias.as_deref(),
            &mut s.diagnostics,
        )?;
        let obs = &mut s.observatories.observatories[index];
        if tel == b"IMAG" {
            for k in 0..*read(&o.nFlags, "nFlags")? {
                let id = read_string(
                    o.flagID
                        .get(k as usize)
                        .ok_or_else(|| boundary("imag_flags_storage"))?,
                    "flagID",
                )?;
                for (axis, name) in [b"-telx", b"-tely", b"-telz"].into_iter().enumerate() {
                    if id == name {
                        let value = read_string(&o.flagVal[k as usize], "flagVal")?;
                        if let Some((v, _)) = crate::clock_text::number_prefix(&value) {
                            obs.xyz[axis] = v;
                        }
                    }
                }
            }
        }
        let site = if *read(&p.t2cMethod, "t2cMethod")? == 1 {
            let trs = obs.xyz;
            let zen = zenith_trs(obs);
            let mut utc = read(&o.sat, "sat")?.clone();
            let mut site_to_utc_s = 0.0;
            let clock = *read(&o.clockCorr, "clockCorr")?;
            if clock != 0 && clock != 2 {
                let from = read_string(&p.clockFromOverride[..], "clockFromOverride")?;
                site_to_utc_s = s.run(|s| {
                    s.get_correction(
                        o,
                        Some(&from),
                        Some(b"UTC"),
                        *read(&p.noWarnings, "noWarnings")?,
                    )
                })?;
                utc += site_to_utc_s / 86400.0;
            }
            let tt = read(&o.sat, "sat")?.clone() + crate::clkcorr::get_correction_tt(o)? / 86400.0;
            let file = read_string(&p.eopc04_file[..], "eopc04_file")?;
            let (mut site, ut1_minus_utc_s) = iau2000b(s, trs, zen, &tt, &utc, &file)?;
            // fda6ad4 get_obsCoord.C:357-374 already uses this site->UTC
            // correction and :75-76 the selected EOP UT1-UTC, but never writes
            // tempo2.h:460's site->UT1 field consumed by displayParameters:98.
            // Retain their binary64 sum in seconds. The bypass branches above
            // actually apply zero clock correction; no new clock/EOP lookup or
            // timing expression is introduced. AH's before/after row witness
            // checks that BAT, residuals, geometry and clocks remain unchanged.
            o.correctionUT1 = Some(Float::with_val(
                crate::PRECISION_BITS,
                site_to_utc_s + ut1_minus_utc_s,
            ));
            if *read(&p.eclCoord, "eclCoord")? == 1 {
                crate::vector_pulsar::rotate(
                    &mut site.observatory_earth,
                    false,
                    s.source.ecliptic_obliquity,
                );
                crate::vector_pulsar::rotate(
                    &mut site.site_velocity,
                    false,
                    s.source.ecliptic_obliquity,
                );
            }
            site
        } else {
            // Source zeroes zenith before reading nutations or UT1.
            o.zenith.fill(Some(0.0));
            legacy(obs, s.source.ecliptic_obliquity, p, o)?
        };
        o.observatory_earth[..3].copy_from_slice(&site.observatory_earth.map(Some));
        o.siteVel = site.site_velocity.map(Some);
        o.zenith = site.zenith.map(Some);
    }
    Ok(())
}
fn legacy(
    obs: &Observatory,
    obliquity: f64,
    p: &SourcePulsar,
    o: &SourceObservation,
) -> Result<SiteGeometry, TimingError> {
    use std::f64::consts::PI;
    // get_obsCoord.C:388,399,410,424: source-compiled O2 contractions;
    // c041/c127/c131 retain bounded whole-driver witnesses.
    let [x, y, z] = obs.xyz;
    let erad = z.mul_add(z, x.mul_add(x, y * y)).sqrt();
    let hlt = (z / erad).asin();
    let alng = (-y).atan2(x);
    let hrd = erad / (2.99792458e8 * 499.004786);
    let coord0 = hrd * hlt.cos() * 499.004786;
    let coord1 = coord0 * hlt.tan();
    let sat = read(&o.sat, "sat")?;
    let t = ((sat.clone() + 2400000.5f64 - 2451545.0f64) / 36525.0f64).to_f64();
    let oblq = t.mul_add(
        t.mul_add(t.mul_add(1.813e-3, -5.9e-4), -4.6815e1),
        84381.448,
    ) / 3600.0;
    let n0 = *read(&o.nutations[0], "nutations[0]")?;
    let n1 = *read(&o.nutations[1], "nutations[1]")?;
    let cos_oblq = (oblq * PI / 180.0 + n1).cos();
    let corr = read(&o.correctionUT1, "correctionUT1")?.clone() / 86400.0f64;
    let tsid = lmst((sat.clone() + &corr).to_f64(), 0.0)?[0] * (2.0 * PI);
    let ph = cos_oblq.mul_add(n0, tsid) - alng;
    let eeq = [coord0 * ph.cos(), coord0 * ph.sin(), coord1];
    let prn = get_precession_matrix((Float::with_val(128, sat.to_f64()) + corr).to_f64(), n0, n1);
    let mut pos = std::array::from_fn(|j| {
        eeq[2].mul_add(prn[j][2], eeq[0].mul_add(prn[j][0], eeq[1] * prn[j][1]))
    });
    let ecliptic = *read(&p.eclCoord, "eclCoord")? == 1;
    if ecliptic {
        crate::vector_pulsar::rotate(&mut pos, false, obliquity);
    }
    let speed = 2.0 * PI * coord0 / (86400.0 / 1.00273);
    let sitera = if speed > 1.0e-10 {
        pos[1].atan2(pos[0])
    } else {
        0.0
    };
    let mut vel = [-sitera.sin() * speed, sitera.cos() * speed, 0.0];
    if ecliptic {
        crate::vector_pulsar::rotate(&mut vel, false, obliquity);
    }
    Ok(SiteGeometry {
        observatory_earth: pos,
        site_velocity: vel,
        zenith: [0.0; 3],
    })
}
pub(crate) fn zenith_trs(obs: &Observatory) -> [f64; 3] {
    [
        obs.height * obs.longitude.cos() * obs.latitude.cos(),
        obs.height * obs.longitude.sin() * obs.latitude.cos(),
        obs.height * obs.latitude.sin(),
    ]
}
