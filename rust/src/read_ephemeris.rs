//! TEMPO2 readEphemeris.C using retained jpleph.c numerical operations.
// Narrow interface to the pinned low-level JPL library; timing remains in Rust.
use crate::{PRECISION_BITS, TimingError, ifteph::IFTE_KM1};
use rug::Float;
use std::{
    ffi::{CString, c_char, c_int, c_long, c_void},
    path::{Path, PathBuf},
    ptr::NonNull,
    sync::Mutex,
};

// Necessity: the accepted JPL interpolation dependency exposes only a C ABI.
// Safety contract: live exclusive native handles, NUL-terminated paths and six
// aligned f64 output elements. Null optional init arrays request no constants.
unsafe extern "C" {
    fn jpl_init_ephemeris(path: *const c_char, names: *mut c_void, values: *mut f64)
    -> *mut c_void;
    fn jpl_init_error_code() -> c_int;
    fn jpl_close_ephemeris(ephem: *mut c_void);
    fn jpl_pleph(
        ephem: *mut c_void,
        jd: f64,
        target: c_int,
        centre: c_int,
        result: *mut f64,
        velocity: c_int,
    ) -> c_int;
    fn jpl_get_double(ephem: *const c_void, field: c_int) -> f64;
}

// The numerical handle is owned, but upstream's init error code is global.
// Serialize initialization/error retrieval so concurrent independent contexts
// cannot report each other's errors (readEphemeris.C instead exits on failure).
static JPL_INIT_LOCK: Mutex<()> = Mutex::new(());

pub(crate) struct JplEphemeris {
    handle: NonNull<c_void>,
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
    // EMB and nutation reads; native cache state then follows the source passes.
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
    pub fn open(path: &Path, bytes: &[u8]) -> Result<Self, TimingError> {
        let error = |message: &str| TimingError::new(path, None, "JPL", message);
        // Upstream's interpolator asserts on bad dimensions and trusts offsets.
        // Validate the supported DE405 header before native allocation/indexing,
        // returning errors for malformed resources rather than aborting the host.
        if bytes.len() < 2856 {
            return Err(error("truncated JPL header"));
        }
        // atoi(title+26) in the native reader requires a terminated version token;
        // malformed digit-only titles can otherwise read past its 84-byte buffer.
        if &bytes[26..30] != b"405/" {
            return Err(error("unsupported or malformed DE405 title"));
        }
        let native_ncon = u32::from_ne_bytes(bytes[2676..2680].try_into().unwrap());
        let swapped = native_ncon > 65536;
        let read_u32 = |p| {
            let v = u32::from_ne_bytes(bytes[p..p + 4].try_into().unwrap());
            if swapped { v.swap_bytes() } else { v }
        };
        let read_f64 = |p| {
            let mut b: [u8; 8] = bytes[p..p + 8].try_into().unwrap();
            if swapped {
                b.reverse();
            }
            f64::from_ne_bytes(b)
        };
        let (start_jd, end_jd, step) = (read_f64(2652), read_f64(2660), read_f64(2668));
        if !(start_jd.is_finite()
            && end_jd.is_finite()
            && step.is_finite()
            && start_jd < end_jd
            && step > 0.0)
            || read_u32(2840) != 405
            || !(1..400).contains(&read_u32(2676))
            || !(81.30055..=81.3008).contains(&read_f64(2688))
            || !(1e8..2e8).contains(&read_f64(2680))
        {
            return Err(error("invalid or unsupported DE405 header"));
        }
        let mut pointers = Vec::new();
        let mut ncoeff = 2usize;
        for i in 0..13 {
            let p = if i == 12 { 2844 } else { 2696 + 12 * i };
            let (offset, n, sub) = (
                read_u32(p) as usize,
                read_u32(p + 4) as usize,
                read_u32(p + 8) as usize,
            );
            let dim = if i == 11 { 2 } else { 3 };
            if offset < 1 || !(2..18).contains(&n) || !(1..=32).contains(&sub) {
                return Err(error("invalid JPL interpolation dimensions"));
            }
            pointers.push((offset, n * sub * dim));
            ncoeff += n * sub * dim;
        }
        if ncoeff > 4096
            || pointers.iter().any(|(p, n)| p - 1 + n > ncoeff)
            || bytes.len() < ncoeff * 8 * 2
        {
            return Err(error("invalid JPL coefficient layout"));
        }
        // Decision 0005: step=1e-300 reached jpl_state's unchecked float->u32
        // conversion and caused SIGBUS. Its (nr+2)*recsize also multiplies u32
        // operands before fseek receives a C long. Bound the entire integral
        // grid here, including the last record's offset, before native entry.
        // Short files still use the checked native read-error/recovery path.
        let data_records = (end_jd - start_jd) / step;
        let max_seek = u64::from(u32::MAX).min(c_long::MAX as u64);
        let max_data_records = max_seek / (ncoeff as u64 * 8) - 1;
        if !data_records.is_finite()
            || data_records < 1.0
            || data_records.fract() != 0.0
            || data_records > max_data_records as f64
        {
            return Err(error("invalid JPL record grid or native seek offset"));
        }
        let name = CString::new(path.as_os_str().as_encoded_bytes())
            .map_err(|_| error("path contains NUL"))?;
        let _guard = JPL_INIT_LOCK
            .lock()
            .map_err(|_| error("JPL initialization lock poisoned"))?;
        // Necessity: create the retained C reader. SAFETY: name is NUL terminated
        // and live; optional arrays may be null by ABI contract. The lock protects
        // global initialization status. Preflight bounds the bytes just inspected,
        // but C reopens this path: unchanged file identity/content until and during
        // native use is an UNENFORCED assumption (see BEHAVIOR_DIFFERENCES.md).
        let raw = unsafe {
            jpl_init_ephemeris(name.as_ptr(), std::ptr::null_mut(), std::ptr::null_mut())
        };
        let handle = NonNull::new(raw).ok_or_else(|| {
            // Necessity: fetch the native initializer status. SAFETY: the same
            // initialization lock is held, and this function takes no pointers.
            error(&format!("native initialization error {}", unsafe {
                jpl_init_error_code()
            }))
        })?;
        // Necessity: read the native AU constant. SAFETY: successful non-null
        // handle is still live and exclusively owned; 28 is JPL_EPHEM_AU_IN_KM.
        let one_au_m = unsafe { jpl_get_double(raw, 28) } * 1000.0;
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
        // unchecked record conversion; reject it before crossing the FFI.
        if !jd.is_finite() {
            return Err(TimingError::new(
                &self.path,
                None,
                "JPL",
                "nonfinite interpolation epoch",
            ));
        }
        let mut result = [0.0; 6];
        // Necessity: run retained JPL interpolation. SAFETY: &mut self ensures
        // exclusive handle/cache access and jd is finite. Native storage bounds
        // assume the reopened file matches preflight and stays unchanged; this
        // assumption is not enforced. result owns six initialized aligned f64 slots, has no aliases,
        // and the C function retains neither pointer. Callers use source body IDs.
        let code = unsafe {
            jpl_pleph(
                self.handle.as_ptr(),
                jd,
                target,
                centre,
                result.as_mut_ptr(),
                1,
            )
        };
        // readEphemeris.C ignores jpl_pleph's status and may publish stale vectors.
        // Native read/range failures are errors here; no vector escapes on failure.
        if code != 0 {
            return Err(TimingError::new(
                &self.path,
                None,
                "JPL",
                format!("native error {code} at JD {jd}, target {target}, centre {centre}"),
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
impl Drop for JplEphemeris {
    fn drop(&mut self) {
        // Necessity: release C-owned storage using its matching allocator.
        // SAFETY: this non-cloneable handle came from successful initialization;
        // Drop runs once, with no outstanding calls or escaped pointers.
        unsafe { jpl_close_ephemeris(self.handle.as_ptr()) };
    }
}
