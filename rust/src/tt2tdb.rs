//! Complete tt2tdb.C counterpart at fda6ad4; one body for full and strict callers.
use crate::{
    PRECISION_BITS, TimingError,
    clkcorr::ClockSession,
    clock_diagnostics::{boundary, terminal},
    ifteph::{
        IFTE_KM1, IFTE_LC, IFTE_MJD0, IFTE_TEPH0, extended_i32, native_i32, split_ifte_epoch,
    },
    observation::{read, read_string},
    pulsar::SourcePulsar,
    pulsar_lifecycle::PulsarContext,
    tempo2::SourceObservation,
    tempo2_util::{dot_product, fortran_mod},
};
use rug::Float;
use std::os::unix::ffi::OsStrExt;

pub(crate) struct FbCache {
    pub content: Option<String>,
    pub tdbnrl: i32,
    pub buf: [f64; 16],
}
impl Default for FbCache {
    fn default() -> Self {
        Self {
            content: None,
            tdbnrl: -1,
            buf: [0.0; 16],
        }
    }
}

pub(crate) fn init_ifte(session: &mut ClockSession) -> Result<(), TimingError> {
    let root = session
        .root
        .as_deref()
        .ok_or_else(|| boundary("IFTE_environment"))?;
    session.ifte.init(
        &root.join("ephemeris/TIMEEPH_short.te405"),
        &mut session.diagnostics,
    )
}
pub(crate) fn if_delta_t(session: &mut ClockSession, mjd: &Float) -> Result<f64, TimingError> {
    let (jd0, jd1) = split_ifte_epoch(mjd)?;
    Ok(session.ifte.delta_t(jd0, jd1, &mut session.diagnostics)? * 86400.0)
}
pub(crate) fn fb_delta_t(session: &mut ClockSession, mjd: &Float) -> Result<f64, TimingError> {
    let root = session
        .root
        .as_deref()
        .ok_or_else(|| boundary("FB_environment"))?;
    let path = root.join("ephemeris/TDB.1950.2050");
    let mut filename = path.as_os_str().as_bytes().to_vec();
    filename.push(0);
    if session
        .fortran
        .open_file(&mut filename, &mut session.diagnostics)?
        != 0
    {
        return Err(boundary("FB_open_failed"));
    }
    let result = (|| {
        let reader = &mut session.fortran;
        let key = reader.resource.as_ref().unwrap().sha256.clone();
        // Bug fix: tt2tdb.C::FB_deltaT:221-276 (fda6ad4) keys buf only by
        // record, despite opening a new file each call. Hash and consume the
        // same opened bytes (open_resource rewinds before fdopen). Changed
        // content starts cold; publish this key and candidate only on success.
        let changed = session.fb.content.as_ref() != Some(&key);
        let mut buf = if changed { [0.0; 16] } else { session.fb.buf };
        let last = if changed { -1 } else { session.fb.tdbnrl };
        let start = reader.read_double()?;
        let end = reader.read_double()?;
        let width = reader.read_int()?;
        let count = reader.read_int()?;
        for _ in 0..5 {
            reader.read_double()?;
        }
        // tt2tdb.C::FB_deltaT:238-246 (fda6ad4) ignores EOF, even in these
        // unused header doubles. Reject fabricated EOF bytes (0065), including
        // nr==1 calls that would otherwise return a plausible zero correction.
        reader.require_complete()?;
        let tdt = mjd.clone() + 2400000.5f64;
        let day = extended_i32(&tdt, "FB_JD_int")?;
        let fraction = Float::with_val(PRECISION_BITS, &tdt - day);
        let (jda, jdb) = if fraction >= 0.5 {
            (day as f64 + 0.5, (fraction - 0.5f64).to_f64())
        } else {
            (day as f64 - 0.5, (fraction + 0.5f64).to_f64())
        };
        let nr = native_i32((jda - start) / width as f64, "FB_record_int")?
            .checked_add(2)
            .ok_or_else(|| boundary("FB_record_int"))?;
        if nr < 1 || tdt > end {
            session.diagnostics.stdout(
                format!(
                    "ERROR [CLK4]: Date {:.10} out of range of TDB-TDT table\n",
                    tdt.to_f64()
                )
                .as_bytes(),
            );
            return Err(terminal(&path, "FB_date"));
        }
        if nr != last {
            for _ in 0..nr - 1 {
                for k in 0..count {
                    *buf.get_mut(k as usize)
                        .ok_or_else(|| boundary("FB_coefficient_storage"))? =
                        reader.read_double()?;
                }
            }
            // tt2tdb.C::FB_deltaT:270-273 plus read_fortran.h:127-147
            // (fda6ad4) can turn partial coefficients plus EOF 0xff bytes into
            // finite values. Require the attempted reads, not a global length:
            // nr==1 and signed count no-ops legitimately consume no coefficients.
            reader.require_complete()?;
        }
        let offset = nr
            .checked_sub(2)
            .and_then(|n| n.checked_mul(width))
            .ok_or_else(|| boundary("FB_record_offset_int"))?;
        let t = ((jda - (offset as f64 + start)) + jdb) / width as f64;
        let dt1 = native_i32(t, "FB_interval_int")? as f64;
        let tc = (2.0f64
            * (fortran_mod(
                Float::with_val(PRECISION_BITS, t),
                &Float::with_val(PRECISION_BITS, 1),
            )? + dt1)
            - 1.0f64)
            .to_f64();
        let mut pc = [0.0; 18];
        pc[0] = 1.0;
        pc[1] = 0.0;
        let mut twot = 0.0;
        if tc != pc[1] {
            pc[1] = tc;
            twot = tc + tc;
        }
        // Actual file counts control native stack/cache accesses; negative and
        // zero counts are defined no-ops, while >16 exceeds the coefficient buf.
        if count > 16 {
            return Err(boundary("FB_coefficient_storage"));
        }
        for k in 3..=count {
            let k = k as usize; // tt2tdb.C:299 GCC16 O2 fnmsub; Y arithmetic.py recurrence witness.
            pc[k - 1] = twot.mul_add(pc[k - 2], -pc[k - 3]);
        }
        let mut ctatv = 0.0;
        for j in (1..=count).rev() {
            let j = j as usize; // tt2tdb.C:304 O2 fmadd; Y arithmetic.py reverse-sum witness.
            ctatv = pc[j - 1].mul_add(buf[j - 1], ctatv);
        }
        session.fb = FbCache {
            content: Some(key),
            tdbnrl: nr,
            buf,
        };
        Ok(ctatv)
    })();
    // FB exits natively on failure. Release its distinct owner before returning
    // a recoverable library error; IFTE and unrelated clock state remain usable.
    session.fortran.close_file()?;
    result
}

/// tt2tb's observation body; deltaT retains once-per-call unmatched selections.
pub(crate) fn compute_observation(
    session: &mut ClockSession,
    psr: &SourcePulsar,
    obs: &mut SourceObservation,
    delta_t: &mut f64,
) -> Result<(), TimingError> {
    let stl = read_string(&obs.telID, "telID")? == b"STL";
    if *read(&obs.clockCorr, "clockCorr")? == 0 && !stl {
        obs.correctionTT_TB = Some(Float::with_val(PRECISION_BITS, 0));
        return Ok(());
    }
    let mjd_tt = if stl {
        read(&obs.sat, "sat")?.clone()
    } else {
        read(&obs.sat, "sat")?.clone() + crate::clkcorr::get_correction_tt(obs)? / 86400.0
    };
    if *read(&psr.timeEphemeris, "timeEphemeris")? == 1 {
        *delta_t = if_delta_t(session, &mjd_tt)?;
    }
    if *read(&psr.timeEphemeris, "timeEphemeris")? == 2 {
        *delta_t = fb_delta_t(session, &mjd_tt)? - IFTE_TEPH0;
    }
    if *read(&psr.useCalceph, "useCalceph")? == 1 {
        // Bug fix: tt2tdb.C::tt2tb:99-103 (fda6ad4) reads psr[0] while
        // processing psr[p]. The supplied value belongs to this observation;
        // single-vs-multiple timing must retain each pulsar's own correction.
        *delta_t = read(&obs.correctionTT_calcEph, "correctionTT_calcEph")?.to_f64();
    }
    let earth = [
        *read(&obs.earth_ssb[3], "earth_ssb[3]")?,
        *read(&obs.earth_ssb[4], "earth_ssb[4]")?,
        *read(&obs.earth_ssb[5], "earth_ssb[5]")?,
    ];
    let site = [
        *read(&obs.observatory_earth[0], "observatory_earth[0]")?,
        *read(&obs.observatory_earth[1], "observatory_earth[1]")?,
        *read(&obs.observatory_earth[2], "observatory_earth[2]")?,
    ];
    let k = Float::with_val(PRECISION_BITS, 1) + IFTE_KM1;
    let units = *read(&psr.units, "units")?;
    let mut obs_term = dot_product(earth, site) / (1.0 - IFTE_LC);
    obs_term /= if units == 1 {
        Float::with_val(PRECISION_BITS, &k * &k).to_f64()
    } else {
        k.to_f64()
    };
    let teph = Float::with_val(
        PRECISION_BITS,
        IFTE_TEPH0 + obs_term + *delta_t / (1.0 - IFTE_LC),
    );
    obs.correctionTT_Teph = Some(teph.clone());
    obs.correctionTT_TB = Some(if units == 2 {
        teph.clone() - 0.0
    } else {
        let drift = IFTE_KM1 * (mjd_tt.clone() - IFTE_MJD0).to_f64() * 86400.0;
        Float::with_val(PRECISION_BITS, drift) + k.clone() * (teph.clone() - IFTE_TEPH0)
    });
    if *read(&psr.dilateFreq, "dilateFreq")? == 1 {
        let mjd_teph = mjd_tt + teph / 86400.0;
        let (jd0, jd1) = split_ifte_epoch(&mjd_teph)?;
        let delta_dot = session
            .ifte
            .delta_t_dot(jd0, jd1, &mut session.diagnostics)?;
        let accel = session
            .ifte
            .get_ve_dot(jd0, jd1, &mut session.diagnostics)?
            .map(|v| v * (1.0 / 86400.0));
        let velocity = [
            *read(&obs.siteVel[0], "siteVel[0]")?,
            *read(&obs.siteVel[1], "siteVel[1]")?,
            *read(&obs.siteVel[2], "siteVel[2]")?,
        ];
        let obs_term_dot =
            (dot_product(accel, site) + dot_product(earth, velocity)) / (1.0 - IFTE_LC);
        // tt2tdb.C:162-165 rescales the now-unused obsTerm, not obsTermDot.
        // Keep the resulting derivative convention; no formula fix is inferred.
        let rate = 1.0 + obs_term_dot + delta_dot / (1.0 - IFTE_LC);
        obs.einsteinRate = Some(if units == 2 {
            rate
        } else {
            (k * rate).to_f64()
        });
        for (line, label, value) in [
            (
                180,
                "corrn TT_Teph",
                read(&obs.correctionTT_Teph, "correctionTT_Teph")?.to_f64(),
            ),
            (
                181,
                "corrn TT_TB",
                read(&obs.correctionTT_TB, "correctionTT_TB")?.to_f64(),
            ),
            (
                182,
                "corrn einstein",
                read(&obs.einsteinRate, "einsteinRate")? - 1.0,
            ),
        ] {
            session.diagnostics.log(
                b'd',
                "tt2tdb.C",
                line,
                format!("{label} {value}").as_bytes(),
            );
        }
    }
    Ok(())
}

pub(crate) fn tt2tb(
    session: &mut ClockSession,
    context: &mut PulsarContext,
    npsr: i32,
) -> Result<(), TimingError> {
    session.diagnostics.version("tt2tdb.C", "tt2tdb()");
    init_ifte(session)?;
    let result = (|| {
        let mut delta_t = 0.0;
        for p in 0..npsr {
            let (psr, observations) = context.pulsar_observations_mut(p as usize)?;
            for i in 0..*read(&psr.nobs, "nobs")? {
                let obs = observations
                    .get_mut(i as usize)
                    .ok_or_else(|| boundary("tt2tb_observation_storage"))?;
                compute_observation(session, psr, obs, &mut delta_t)?;
            }
        }
        Ok(())
    })();
    session.ifte.close_file();
    result
}
