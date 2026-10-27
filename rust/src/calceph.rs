//! readEphemeris_calceph.C: owned provider calls and source units/epoch boundaries.
use crate::{PRECISION_BITS, TimingError, pulsar_lifecycle::PulsarContext};
use rug::Float;
#[cfg(not(feature = "calceph"))]
pub(crate) fn unavailable_in(
    d: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<(), TimingError> {
    d.stdout(b"ERROR: unable to use calceph library routines as library not installed\n");
    Err(crate::clock_diagnostics::boundary(
        "CALCEPH_library_unavailable",
    ))
}
pub fn read_ephemeris_calceph(context: &mut PulsarContext) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| positions(s, context, &mut vec![]))
}
pub fn tt2tb_calceph(context: &mut PulsarContext) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| time(s, context, &mut vec![]))
}
pub fn convert_units(v: &mut [f64; 6], units: i32, ecl: i32) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| {
        convert_units_in(v, units, ecl, s.source.ecliptic_obliquity);
        Ok(())
    })
}
fn convert_units_in(v: &mut [f64; 6], units: i32, ecl: i32, obliquity: f64) {
    let scale = if units == 1 {
        (Float::with_val(PRECISION_BITS, 1) + crate::ifteph::IFTE_KM1).to_f64()
    } else {
        1.0
    };
    for x in v.iter_mut() {
        *x = *x * 1000.0 * scale / 299792458.0;
    }
    if ecl == 1 {
        for chunk in v.chunks_exact_mut(3) {
            let mut r = [chunk[0], chunk[1], chunk[2]];
            crate::vector_pulsar::rotate(&mut r, false, obliquity);
            chunk.copy_from_slice(&r);
        }
    }
}
pub(crate) fn positions(
    s: &mut crate::clkcorr::ClockSession,
    c: &mut PulsarContext,
    resources: &mut Vec<crate::Resource>,
) -> Result<(), TimingError> {
    #[cfg(feature = "calceph")]
    {
        enabled::driver(s, c, resources, false)
    }
    #[cfg(not(feature = "calceph"))]
    {
        let _ = (c, resources);
        unavailable_in(&mut s.diagnostics)
    }
}
pub(crate) fn time(
    s: &mut crate::clkcorr::ClockSession,
    c: &mut PulsarContext,
    resources: &mut Vec<crate::Resource>,
) -> Result<(), TimingError> {
    #[cfg(feature = "calceph")]
    {
        enabled::driver(s, c, resources, true)
    }
    #[cfg(not(feature = "calceph"))]
    {
        let _ = (c, resources);
        unavailable_in(&mut s.diagnostics)
    }
}

#[cfg(feature = "calceph")]
mod enabled {
    use super::*;
    use crate::{
        clkcorr::ClockSession,
        observation::{read, read_string},
        preprocess::{is_set, value},
        tempo2::Label::*,
    };
    use std::{
        ffi::{CString, c_char, c_void},
        os::unix::ffi::OsStrExt,
        path::Path,
        ptr::NonNull,
    };
    unsafe extern "C" {
        fn calceph_open(path: *const c_char) -> *mut c_void;
        fn calceph_close(eph: *mut c_void);
        fn calceph_compute(
            eph: *mut c_void,
            jd0: f64,
            jd1: f64,
            target: i32,
            centre: i32,
            pv: *mut f64,
        ) -> i32;
        fn t2_calceph_position(
            eph: *mut c_void,
            jd0: f64,
            jd1: f64,
            target: i32,
            centre: i32,
            pv: *mut f64,
        ) -> i32;
    }
    // No Send/Sync: one uniquely owned descriptor per pulsar/pass, closed on
    // all exits. The provider's C API borrows the filename only during open.
    struct Ephemeris(NonNull<c_void>);
    impl Drop for Ephemeris {
        fn drop(&mut self) {
            // SAFETY: successful open transferred this nonnull descriptor;
            // it has not been shared, freed, or retained beyond this owner.
            unsafe { calceph_close(self.0.as_ptr()) }
        }
    }
    impl Ephemeris {
        fn compute(
            &mut self,
            jd: [f64; 2],
            target: i32,
            centre: i32,
            time: bool,
        ) -> Result<[f64; 6], TimingError> {
            let mut pv = [0.0; 6];
            // SAFETY: live unique descriptor; six writable binary64 elements,
            // no aliases and no retained pointers. The time target defines only
            // pv[0]; callers consume only that component. C int is 32 bits.
            let ok = unsafe {
                if time {
                    calceph_compute(
                        self.0.as_ptr(),
                        jd[0],
                        jd[1],
                        target,
                        centre,
                        pv.as_mut_ptr(),
                    )
                } else {
                    t2_calceph_position(
                        self.0.as_ptr(),
                        jd[0],
                        jd[1],
                        target,
                        centre,
                        pv.as_mut_ptr(),
                    )
                }
            };
            // fda6ad4 readEphemeris_calceph.C:105-195,244 ignores compute status
            // and publishes unwritten/stale vectors. Return an error before
            // consuming the output; the descriptor is still closed by Drop.
            if ok == 0 {
                return Err(crate::clock_diagnostics::boundary(&format!(
                    "CALCEPH compute failed: JD {} + {}, target {target}, centre {centre}",
                    jd[0], jd[1]
                )));
            }
            Ok(pv)
        }
    }
    pub(super) fn driver(
        s: &mut ClockSession,
        c: &mut PulsarContext,
        resources: &mut Vec<crate::Resource>,
        time: bool,
    ) -> Result<(), TimingError> {
        for index in 0..c.pulsars.len() {
            let (p, rows) = c.pulsar_observations_mut(index)?;
            let path = read_string(&p.JPL_EPHEMERIS[..], "JPL_EPHEMERIS")?;
            let cpath = CString::new(path.clone())
                .map_err(|_| crate::clock_diagnostics::boundary("CALCEPH path NUL"))?;
            // SAFETY: NUL-terminated filename lives through open. Only a nonnull
            // success becomes the unique owner; no C object layout is assumed.
            let handle = unsafe { calceph_open(cpath.as_ptr()) };
            let Some(handle) = NonNull::new(handle) else {
                s.diagnostics.stdout(
                    &[
                        b"Error: unable to open ephemeris >".as_slice(),
                        &path,
                        b"< for pulsar >",
                        &read_string(&p.name[..], "name")?,
                        b"<\n",
                    ]
                    .concat(),
                );
                return Err(crate::clock_diagnostics::boundary("CALCEPH open failed"));
            };
            let mut eph = Ephemeris(handle);
            s.diagnostics
                .stdout(&[b"Successfully opened ephemeris >".as_slice(), &path, b"<\n"].concat());
            // CALCEPH owns its FILE and opens by name. This independent digest
            // records the observed resource, not a snapshot against file writers.
            let path = Path::new(std::ffi::OsStr::from_bytes(&path));
            let (_, mut resource) =
                crate::runtime::open_resource(path, "CALCEPH planetary/time ephemeris")?;
            if let Some(root) = &s.root {
                resource.path = path
                    .strip_prefix(root)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .into_owned();
            }
            if !resources
                .iter()
                .any(|r| r.source_path == resource.source_path && r.sha256 == resource.sha256)
            {
                resources.push(resource);
            }
            let units = *read(&p.units, "units")?;
            let ecl = *read(&p.eclCoord, "eclCoord")?;
            let count = *read(&p.nobs, "nobs")? as usize;
            for o in &mut rows[..count] {
                let mut jd = read(&o.sat, "sat")?.clone()
                    + crate::clkcorr::get_correction_tt(o)? / 86400.0
                    + read(&o.correctionTT_Teph, "correctionTT_Teph")?.clone() / 86400.0
                    + 2400000.5;
                if time {
                    // Source tt2tb_calceph:215 assigns to ordinary long double,
                    // unlike timing-longdouble position JD. Reuse AF rounding.
                    jd = crate::predictor_number::PredictorNumber::from_float(&jd)
                        .as_float()
                        .clone();
                }
                let day = jd
                    .clone()
                    .trunc()
                    .to_integer()
                    .and_then(|n| n.to_i32())
                    .ok_or_else(|| {
                        crate::clock_diagnostics::boundary("CALCEPH JD outside C int range")
                    })?;
                let parts = [f64::from(day), (jd - day).to_f64()];
                if time {
                    o.correctionTT_calcEph = Some(Float::with_val(
                        PRECISION_BITS,
                        -eph.compute(parts, 16, 0, true)?[0],
                    ));
                    continue;
                }
                let mut vector = |parts, target, centre| -> Result<[Option<f64>; 6], TimingError> {
                    let mut v = eph.compute(parts, target, centre, false)?;
                    convert_units_in(&mut v, units, ecl, s.source.ecliptic_obliquity);
                    Ok(v.map(Some))
                };
                o.earth_ssb = vector(parts, 3, 12)?;
                o.sun_ssb = vector(parts, 11, 12)?;
                for k in 0..9 {
                    if is_set(p, param_dphaseplanet, k) || is_set(p, param_dmassplanet, k) {
                        o.planet_ssb[k] = vector(parts, k as i32 + 1, 12)?;
                    }
                }
                for k in 0..9 {
                    if is_set(p, param_dphaseplanet, k) || is_set(p, param_dmassplanet, k) {
                        o.planet_ssb_tmr[k] = vector([parts[0] + 1.0, parts[1]], k as i32 + 1, 12)?;
                    }
                }
                for k in 0..9 {
                    for j in 0..6 {
                        // fda6ad4 readEphemeris_calceph.C:149-156 subtracts inactive
                        // storage with no producer. Preserve supplied values; absent
                        // inputs leave the difference unwritten, never a guessed zero.
                        o.planet_ssb_derv[k][j] = o.planet_ssb_tmr[k][j]
                            .zip(o.planet_ssb[k][j])
                            .map(|(a, b)| a - b);
                    }
                }
                for (out, target, centre) in [
                    (&mut o.jupiter_earth, 5, 3),
                    (&mut o.saturn_earth, 6, 3),
                    (&mut o.uranus_earth, 7, 3),
                    (&mut o.neptune_earth, 8, 3),
                    (&mut o.venus_earth, 2, 3),
                    (&mut o.earthMoonBary_ssb, 13, 12),
                    (&mut o.earthMoonBary_earth, 13, 3),
                ] {
                    *out = vector(parts, target, centre)?;
                }
                for k in 0..9 {
                    if p.param.slots[param_dmassplanet as usize].cells[k].param_set != 0 {
                        for j in 0..6 {
                            o.earth_ssb[j] = Some(
                                (Float::with_val(
                                    PRECISION_BITS,
                                    *read(&o.earth_ssb[j], "earth_ssb")?,
                                ) - value(p, param_dmassplanet, k)
                                    * *read(&o.planet_ssb[k][j], "planet_ssb")?)
                                .to_f64(),
                            );
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
