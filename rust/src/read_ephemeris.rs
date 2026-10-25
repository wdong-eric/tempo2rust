//! Full readEphemeris.C native-JPL body, epoch and scaling responsibility.
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

#[cfg(test)]
#[path = "planetary_corpus_tests.rs"]
mod corpus_tests;

// Preserve an initialization and its global status query as a sequential pair.
static JPL_INIT_LOCK: Mutex<()> = Mutex::new(());

pub(crate) struct JplEphemeris {
    handle: Ephemeris,
    path: PathBuf,
    pub start_jd: f64,
    pub end_jd: f64,
    one_au_m: f64,
}

impl JplEphemeris {
    pub fn open(path: &Path, file: File, units: i32) -> Result<Self, TimingError> {
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
        let one_au_m = if units == 1 {
            (Float::with_val(PRECISION_BITS, one_au_m) * k).to_f64()
        } else {
            one_au_m
        };
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
        if code != 0 && !(target == 14 && code == -3) {
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

/// Complete native-JPL source driver. Each source pass opens each pulsar resource.
pub fn read_ephemeris(
    context: &mut crate::pulsar_lifecycle::PulsarContext,
    root: &Path,
    resources: &mut Vec<crate::runtime::Resource>,
    add_noise: i32,
) -> Result<(), TimingError> {
    crate::clkcorr::with_resources(root, |session| {
        driver(session, context, resources, add_noise)
    })
}
pub(crate) fn driver(
    session: &mut crate::clkcorr::ClockSession,
    context: &mut crate::pulsar_lifecycle::PulsarContext,
    resources: &mut Vec<crate::runtime::Resource>,
    add_noise: i32,
) -> Result<(), TimingError> {
    use crate::{
        observation::{read, read_string},
        preprocess::{flags, is_set, value},
        tempo2::Label::*,
    };
    use std::{ffi::OsStr, os::unix::ffi::OsStrExt};
    session
        .diagnostics
        .version("readEphemeris.C", "readEphemeris()");
    if add_noise != 0 {
        session
            .diagnostics
            .stdout(format!("WARNING: Adding noise into ephemeris {add_noise}\n").as_bytes());
    }
    for index in 0..context.pulsars.len() {
        let (p, rows) = context.pulsar_observations_mut(index)?;
        let path = read_string(&p.JPL_EPHEMERIS[..], "JPL_EPHEMERIS")?;
        let path = Path::new(OsStr::from_bytes(&path));
        let opened = (|| {
            let (file, resource) = crate::runtime::open_resource(path, "JPL planetary ephemeris")
                .inspect_err(|_| crate::jpleph::init_error(-1))?;
            let jpl = JplEphemeris::open(path, file, *read(&p.units, "units")?)?;
            Ok::<_, TimingError>((jpl, resource))
        })();
        let (mut jpl, mut resource) = opened.inspect_err(|_| {
            session.diagnostics.stdout(
                &[
                    b"ERROR [EPHEM1]: Ephemeris file '".as_slice(),
                    path.as_os_str().as_bytes(),
                    b"' not loaded\n",
                ]
                .concat(),
            );
        })?;
        if session.diagnostics.debug {
            use crate::clock_text::{NumberFormat, format_number};
            session.diagnostics.stdout(
                &[
                    b"one_au = ".as_slice(),
                    &format_number(jpl.handle.au * 1000.0, NumberFormat::GeneralSix),
                    b", emrat = ",
                    &format_number(jpl.handle.emrat, NumberFormat::GeneralSix),
                    b"\n",
                ]
                .concat(),
            );
        }
        resource.coverage_mjd = Some([jpl.start_jd - 2400000.5, jpl.end_jd - 2400000.5]);
        if let Some(root) = &session.root {
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
        let ecl = p.eclCoord == Some(1);
        let obliquity = session.source.ecliptic_obliquity;
        let count = (*read(&p.nobs, "nobs")?).max(0) as usize;
        for o in &mut rows[..count] {
            let jd = read(&o.sat, "sat")?.clone()
                + crate::clkcorr::get_correction_tt(o)? / 86400.0f64
                + read(&o.correctionTT_Teph, "correctionTT_Teph")?.clone() / 86400.0f64
                + 2400000.5f64;
            o.sun_ssb = planet_vector(&mut jpl, jd.to_f64(), 11, 12, ecl, obliquity)?.map(Some);
            o.sun_earth = planet_vector(&mut jpl, jd.to_f64(), 11, 3, ecl, obliquity)?.map(Some);
            for k in 0..9 {
                if is_set(p, param_dphaseplanet, k) || is_set(p, param_dmassplanet, k) {
                    o.planet_ssb[k] =
                        planet_vector(&mut jpl, jd.to_f64(), k as i32 + 1, 12, ecl, obliquity)?
                            .map(Some);
                }
            }
            for k in 0..9 {
                if is_set(p, param_dphaseplanet, k) || is_set(p, param_dmassplanet, k) {
                    o.planet_ssb_tmr[k] = planet_vector(
                        &mut jpl,
                        (jd.clone() + 1e-6f64).to_f64(),
                        k as i32 + 1,
                        12,
                        ecl,
                        obliquity,
                    )?
                    .map(Some);
                }
            }
            for k in 0..9 {
                for j in 0..6 {
                    // readEphemeris.C:247-255 subtracts inactive arrays that neither
                    // initialiseOne nor readTim writes. Preserve every assigned tail;
                    // initialize only genuinely unwritten inactive inputs to zero.
                    // Active missing inputs still fail at their real producer.
                    if !is_set(p, param_dphaseplanet, k) && !is_set(p, param_dmassplanet, k) {
                        if o.planet_ssb[k][j].is_none() {
                            o.planet_ssb[k][j] = Some(0.0);
                        }
                        if o.planet_ssb_tmr[k][j].is_none() {
                            o.planet_ssb_tmr[k][j] = Some(0.0);
                        }
                    }
                    o.planet_ssb_derv[k][j] = Some(
                        *read(&o.planet_ssb_tmr[k][j], "planet_ssb_tmr")?
                            - *read(&o.planet_ssb[k][j], "planet_ssb")?,
                    );
                }
            }
            for (out, target) in [
                (&mut o.jupiter_earth, 5),
                (&mut o.saturn_earth, 6),
                (&mut o.venus_earth, 2),
                (&mut o.uranus_earth, 7),
                (&mut o.neptune_earth, 8),
            ] {
                *out = planet_vector(&mut jpl, jd.to_f64(), target, 3, ecl, obliquity)?.map(Some);
            }
            let mut earth = jpl.interpolate_state(jd.to_f64(), 3, 12)?;
            for k in 0..9 {
                if p.param.slots[param_dmassplanet as usize].cells[k].param_set != 0 {
                    for (j, component) in earth.iter_mut().enumerate() {
                        *component = (Float::with_val(PRECISION_BITS, *component)
                            - value(p, param_dmassplanet, k)
                                * read(&o.planet_ssb[k][j], "planet_ssb mass")?)
                        .to_f64();
                    }
                }
            }
            if ecl {
                rotate_prefix(&mut earth, obliquity);
                let mut velocity = [earth[3], earth[4], earth[5]];
                crate::vector_pulsar::rotate(&mut velocity, false, obliquity);
                earth[3..].copy_from_slice(&velocity);
            }
            o.earth_ssb = earth.map(Some);
            o.earthMoonBary_ssb =
                planet_vector(&mut jpl, jd.to_f64(), 13, 12, ecl, obliquity)?.map(Some);
            o.earthMoonBary_earth =
                planet_vector(&mut jpl, jd.to_f64(), 13, 3, ecl, obliquity)?.map(Some);
            o.nutations = jpl.interpolate_state(jd.to_f64(), 14, 3)?.map(Some);
            for (axis, n, t, v) in [
                (0, p.nTelDX, &p.telDX_t[..], &p.telDX_v[..]),
                (1, p.nTelDY, &p.telDY_t[..], &p.telDY_v[..]),
                (2, p.nTelDZ, &p.telDZ_t[..], &p.telDZ_v[..]),
            ] {
                let n = *read(&n, "nTelD")?;
                if n > 0 {
                    let sat = read(&o.sat, "sat")?.to_f64();
                    let mut offset = 0.0;
                    for k in 0..(n - 1) as usize {
                        let t0 = *read(&t[k], "telD_t")?;
                        let t1 = *read(&t[k + 1], "telD_t")?;
                        if sat >= t0 && sat < t1 {
                            let v0 = *read(&v[k], "telD_v")?;
                            let v1 = *read(&v[k + 1], "telD_v")?;
                            let m = (v0 - v1) / (t0 - t1);
                            // Z's pinned GCC16/O2 object contracts both source
                            // interpolation expressions to fmsub/fmadd.
                            let c = (-m).mul_add(t0, v0);
                            offset = m.mul_add(sat, c);
                            break;
                        }
                    }
                    o.earth_ssb[axis] = Some(o.earth_ssb[axis].unwrap() + offset);
                }
            }
            if read_string(&o.telID, "telID")? == b"STL_BAT" {
                for (id, val) in flags(o)? {
                    for (axis, key) in [(0, b"-telx"), (1, b"-tely"), (2, b"-telz")] {
                        if id == key
                            && let Some((v, _)) = crate::clock_text::number_prefix(&val)
                        {
                            o.earth_ssb[axis] = Some(v);
                        }
                    }
                }
                o.earth_ssb[3..].fill(Some(0.0));
            }
        }
    }
    Ok(())
}
fn rotate_prefix(v: &mut [f64; 6], obliquity: f64) {
    let mut position = [v[0], v[1], v[2]];
    crate::vector_pulsar::rotate(&mut position, false, obliquity);
    v[..3].copy_from_slice(&position);
}
fn planet_vector(
    jpl: &mut JplEphemeris,
    jd: f64,
    target: i32,
    centre: i32,
    ecliptic: bool,
    obliquity: f64,
) -> Result<[f64; 6], TimingError> {
    let mut v = jpl.interpolate_state(jd, target, centre)?;
    if ecliptic {
        rotate_prefix(&mut v, obliquity);
    }
    Ok(v)
}

pub(crate) struct RandomState {
    table: Vec<Float>,
    ix1: i32,
    ix2: i32,
    ix3: i32,
    initialized: bool,
    result: Float,
    gaussian: f64,
    has_gaussian: bool,
}
impl Default for RandomState {
    fn default() -> Self {
        Self {
            table: vec![Float::with_val(PRECISION_BITS, 0); 100],
            ix1: 0,
            ix2: 0,
            ix3: 0,
            initialized: false,
            result: Float::with_val(PRECISION_BITS, 0),
            gaussian: 0.0,
            has_gaussian: false,
        }
    }
}
impl RandomState {
    fn random(&mut self, seed: &mut i64) -> Result<f64, TimingError> {
        use crate::tempo2_util::fortran_mod;
        let modu = |x: i64, m: i64| -> Result<i32, TimingError> {
            crate::ifteph::extended_i32(
                &fortran_mod(
                    Float::with_val(PRECISION_BITS, x),
                    &Float::with_val(PRECISION_BITS, m),
                )?,
                "random2_state_int",
            )
        };
        let rm1 = 1.0f64 / 259100.0;
        let rm2 = 1.0f64 / 134456.0;
        if *seed < 0 || !self.initialized {
            self.initialized = true;
            self.ix1 = modu(
                54773i64.checked_sub(*seed).ok_or_else(|| {
                    crate::clock_diagnostics::boundary("random2_seed_long_overflow")
                })?,
                259100,
            )?;
            self.ix1 = modu(7141 * self.ix1 as i64 + 54773, 259100)?;
            self.ix2 = modu(self.ix1 as i64, 134456)?;
            self.ix1 = modu(7141 * self.ix1 as i64 + 54773, 259100)?;
            self.ix3 = modu(self.ix1 as i64, 243000)?;
            for j in 0..97 {
                self.ix1 = modu(7141 * self.ix1 as i64 + 54773, 259100)?;
                self.ix2 = modu(8121 * self.ix2 as i64 + 28411, 134456)?;
                self.table[j] = (Float::with_val(PRECISION_BITS, self.ix2) * rm2 + self.ix1) * rm1;
            }
            *seed = 1;
        }
        self.ix1 = modu(7141 * self.ix1 as i64 + 54773, 259100)?;
        self.ix2 = modu(8121 * self.ix2 as i64 + 28411, 134456)?;
        self.ix3 = modu(4561 * self.ix3 as i64 + 51349, 243000)?;
        let j = (97 * self.ix3) / 243000;
        if !(0..=96).contains(&j) {
            return Err(crate::clock_diagnostics::boundary("random2_shuffle_index"));
        }
        self.result = self.table[j as usize].clone();
        let result = self.result.to_f64();
        self.table[j as usize] = (Float::with_val(PRECISION_BITS, self.ix2) * rm2 + self.ix1) * rm1;
        Ok(result)
    }
    fn gaussian(&mut self, seed: &mut i64) -> Result<f64, TimingError> {
        if self.has_gaussian {
            self.has_gaussian = false;
            return Ok(self.gaussian);
        }
        loop {
            // GCC16 O2 contracts these expressions in the accepted Z profile.
            let v1 = self.random(seed)?.mul_add(2.0, -1.0);
            let v2 = self.random(seed)?.mul_add(2.0, -1.0);
            let rsq = v1.mul_add(v1, v2 * v2);
            if rsq >= 1.0 || rsq == 0.0 {
                continue;
            }
            let fac = (-2.0 * rsq.ln() / rsq).sqrt();
            self.gaussian = v1 * fac;
            self.has_gaussian = true;
            return Ok(v2 * fac);
        }
    }
}
pub fn random2(seed: &mut i64) -> Result<f64, TimingError> {
    crate::clkcorr::with_process(|s| s.source.random.random(seed))
}
pub fn gasdev(seed: &mut i64) -> Result<f64, TimingError> {
    crate::clkcorr::with_process(|s| s.source.random.gaussian(seed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    fn number(x: f64) -> Value {
        json!({"class":"finite","bits":format!("{:016x}",x.to_bits())})
    }
    fn quad(x: &Float) -> Value {
        let x = Float::with_val(113, x);
        let sign = (x.is_sign_negative() as u128) << 127;
        let bits = if x.is_zero() {
            sign
        } else {
            let (n, e) = x.clone().abs().to_integer_exp().unwrap();
            let n = n.to_u128().unwrap();
            let len = 128 - n.leading_zeros();
            let m = if len > 113 {
                n >> (len - 113)
            } else {
                n << (113 - len)
            };
            sign | (((e + len as i32 - 1 + 16383) as u128) << 112) | (m - (1 << 112))
        };
        json!({"class":"finite","little_endian_bytes":bits.to_le_bytes().iter().map(|b|format!("{b:02x}")).collect::<String>()})
    }
    #[test]
    #[ignore = "check_random.py supplies accepted Z observer histories"]
    fn z_random_histories() {
        let path = std::path::PathBuf::from(std::env::var_os("AC_RANDOM_CASES").unwrap());
        let cases: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let mut failures = vec![];
        for case in cases.as_array().unwrap() {
            let mut state = RandomState::default();
            let mut seeds = [0i64; 2];
            for (i, event) in case["events"].as_array().unwrap().iter().enumerate() {
                let a = event["action"].as_array().unwrap();
                let op = a[0].as_str().unwrap();
                let k = a[1].as_str().unwrap().parse::<usize>().unwrap();
                let actual = match op {
                    "seed" => {
                        seeds[k] = a[2].as_str().unwrap().parse().unwrap();
                        json!({})
                    }
                    "random" => json!({"value":number(state.random(&mut seeds[k]).unwrap())}),
                    "gaussian" => json!({"value":number(state.gaussian(&mut seeds[k]).unwrap())}),
                    _ => unreachable!(),
                };
                if actual != event["result"] {
                    failures.push(format!(
                        "{} event {i} result: {actual} != {}",
                        case["id"], event["result"]
                    ));
                }
                assert_eq!(
                    json!(seeds),
                    event["seeds"],
                    "{} event {i} seeds",
                    case["id"]
                );
                let private = &event["private"];
                if !private["random"].is_null() {
                    let random = json!({"iff":i32::from(state.initialized),"ix":[state.ix1,state.ix2,state.ix3],"result":quad(&state.result),"r":state.table.iter().map(quad).collect::<Vec<_>>()});
                    if random != private["random"] {
                        failures.push(format!("{} event {i} random state", case["id"]));
                    }
                }
                if !private["gaussian"].is_null() {
                    let gaussian =
                        json!({"iset":i32::from(state.has_gaussian),"gset":number(state.gaussian)});
                    if gaussian != private["gaussian"] {
                        failures.push(format!(
                            "{} event {i} gaussian: {gaussian} != {}",
                            case["id"], private["gaussian"]
                        ));
                    }
                }
            }
        }
        std::fs::write(
            path.with_file_name("differences.json"),
            serde_json::to_vec_pretty(&failures).unwrap(),
        )
        .unwrap();
        assert!(
            failures.is_empty(),
            "{} discrepancies: {:?}",
            failures.len(),
            &failures[..failures.len().min(5)]
        );
    }
}
