//! Complete ifteph.C counterpart at fda6ad4; source state lives in ClockSession.
//! Direct IFTEinterp callers own separate InterpolationInfo. GPL-3.0-or-later.
#![allow(non_snake_case)]
use crate::{
    PRECISION_BITS, TimingError,
    clock_diagnostics::{Diagnostics, boundary, terminal},
    runtime::{Resource, open_resource},
    stdio::Stream,
};
use rug::{Float, float::Round};
use std::path::{Path, PathBuf};

pub(crate) const IFTE_LC: f64 = 1.48082686742e-8;
pub(crate) const IFTE_KM1: f64 = 1.55051979176e-8;
pub(crate) const IFTE_MJD0: f64 = 43144.0003725;
pub(crate) const IFTE_TEPH0: f64 = -65.564518e-6;

pub(crate) fn native_i32(value: f64, field: &str) -> Result<i32, TimingError> {
    let v = value.trunc();
    if v >= i32::MIN as f64 && v <= i32::MAX as f64 {
        Ok(v as i32)
    } else {
        Err(boundary(field))
    }
}
pub(crate) fn extended_i32(value: &Float, field: &str) -> Result<i32, TimingError> {
    value
        .to_integer_round(Round::Zero)
        .and_then(|(n, _)| n.to_i32())
        .ok_or_else(|| boundary(field))
}
pub(crate) fn split_ifte_epoch(mjd: &Float) -> Result<(f64, f64), TimingError> {
    let day = extended_i32(mjd, "IFTE_epoch_int")?;
    Ok((
        2400000.0 + day as f64,
        (Float::with_val(PRECISION_BITS, mjd - day) + 0.5f64).to_f64(),
    ))
}

pub(crate) fn IFTswap4(word: &mut [u8; 4]) {
    word.reverse();
}
pub(crate) fn IFTswapInt(word: &mut i32) {
    let mut bytes = word.to_ne_bytes();
    IFTswap4(&mut bytes);
    *word = i32::from_ne_bytes(bytes);
}
pub(crate) fn IFTswapInts(words: &mut [i32], n: i32) -> Result<(), TimingError> {
    for i in 0..n {
        IFTswapInt(
            words
                .get_mut(i as usize)
                .ok_or_else(|| boundary("IFTswapInts_storage"))?,
        );
    }
    Ok(())
}
pub(crate) fn IFTswap8(word: &mut [u8; 8]) {
    word.reverse();
}
pub(crate) fn IFTswapDouble(word: &mut f64) {
    let mut bytes = word.to_ne_bytes();
    IFTswap8(&mut bytes);
    *word = f64::from_ne_bytes(bytes);
}
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "complete raw byte helper counterpart")
)]
pub(crate) fn IFTswap8N(words: &mut [u8], n: i32) -> Result<(), TimingError> {
    for i in 0..n {
        let start = i as usize * 8;
        IFTswap8(
            words
                .get_mut(start..start + 8)
                .ok_or_else(|| boundary("IFTswap8N_storage"))?
                .try_into()
                .unwrap(),
        );
    }
    Ok(())
}
pub(crate) fn IFTswapDoubles(words: &mut [f64], n: i32) -> Result<(), TimingError> {
    for i in 0..n {
        IFTswapDouble(
            words
                .get_mut(i as usize)
                .ok_or_else(|| boundary("IFTswapDoubles_storage"))?,
        );
    }
    Ok(())
}

#[derive(Default, Clone)]
pub(crate) struct InterpolationInfo {
    pub pc: [f64; 18],
    pub vc: [f64; 18],
    pub twot: f64,
    pub np: i32,
    pub nv: i32,
}
impl InterpolationInfo {
    /// IFTEinterp: source subinterval arithmetic and reverse component sums.
    #[allow(clippy::too_many_arguments)]
    pub fn interpolate(
        &mut self,
        coef: &[f64],
        t: [f64; 2],
        ncf: i32,
        ncm: i32,
        na: i32,
        ifl: i32,
        posvel: &mut [f64],
    ) -> Result<(), TimingError> {
        let dna = na as f64;
        let dt1 = t[0].trunc();
        let temp = dna * t[0];
        let l = native_i32(temp - dt1, "IFTE_subinterval_int")?;
        let tc = 2.0 * (temp.fract() + dt1) - 1.0;
        // Coefficients and output are external caller backing. These bounds
        // replace native pointer arithmetic, not a physical date-domain rule.
        let nc = usize::try_from(ncf)
            .ok()
            .filter(|n| *n <= 18)
            .ok_or_else(|| boundary("IFTE_ncf_storage"))?;
        let cm = ncm.max(0) as usize;
        let needed = cm
            .checked_mul(if ifl > 1 { 2 } else { 1 })
            .ok_or_else(|| boundary("IFTE_output_storage"))?;
        if posvel.len() < needed {
            return Err(boundary("IFTE_output_storage"));
        }
        let start = if nc == 0 || cm == 0 {
            0
        } else {
            usize::try_from(l)
                .ok()
                .and_then(|l| l.checked_mul(cm))
                .and_then(|x| x.checked_mul(nc))
                .ok_or_else(|| boundary("IFTE_coefficient_storage"))?
        };
        let end = start
            .checked_add(nc * cm)
            .ok_or_else(|| boundary("IFTE_coefficient_storage"))?;
        let coefficients = coef
            .get(start..end)
            .ok_or_else(|| boundary("IFTE_coefficient_storage"))?;
        if tc != self.pc[1] {
            self.np = 2;
            self.nv = 3;
            self.pc[1] = tc;
            self.twot = tc + tc;
        }
        if self.np < ncf {
            for i in self.np as usize..nc {
                // ifteph.C:433 GCC16 O2 fnmsub; bounded witness in Y arithmetic.py.
                self.pc[i] = self.twot.mul_add(self.pc[i - 1], -self.pc[i - 2]);
            }
            self.np = ncf;
        }
        for i in 0..cm {
            posvel[i] = 0.0;
            for j in (0..nc).rev() {
                // ifteph.C:442 O2 fmadd; retain this reverse accumulation.
                posvel[i] = self.pc[j].mul_add(coefficients[i * nc + j], posvel[i]);
            }
        }
        if ifl <= 1 {
            return Ok(());
        }
        let vfac = (dna + dna) / t[1];
        self.vc[2] = self.twot + self.twot;
        if self.nv < ncf {
            for i in self.nv as usize..nc {
                // ifteph.C:459 contracts only the first product/add (Y arithmetic.py).
                self.vc[i] = self.twot.mul_add(self.vc[i - 1], self.pc[i - 1]) + self.pc[i - 1]
                    - self.vc[i - 2];
            }
            self.nv = ncf;
        }
        for i in 0..cm {
            let mut tval = 0.0;
            for j in (0..nc).rev() {
                // ifteph.C:471 O2 fmadd, witnessed independently of Rust.
                tval = self.vc[j].mul_add(coefficients[i * nc + j], tval);
            }
            posvel[i + cm] = tval * vfac;
        }
        Ok(())
    }
}

pub(crate) struct IfteEphemeris {
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "unused native title retained in the complete state map"
        )
    )]
    pub title: [u8; 256],
    pub start_jd: f64,
    pub end_jd: f64,
    pub step_jd: f64,
    pub ephver: i32,
    pub l_c: f64,
    pub swap_endian: i32,
    pub reclen: i32,
    pub irec: i32,
    pub buf: [f64; 322],
    pub iinfo: InterpolationInfo,
    pub ipt: [[i32; 3]; 2],
    pub stream: Option<Stream>,
    pub path: PathBuf,
    pub resource: Option<Resource>,
}
impl Default for IfteEphemeris {
    fn default() -> Self {
        Self {
            title: [0; 256],
            start_jd: 0.0,
            end_jd: 0.0,
            step_jd: 0.0,
            ephver: 0,
            l_c: 0.0,
            swap_endian: 0,
            reclen: 0,
            irec: 0,
            buf: [0.0; 322],
            iinfo: InterpolationInfo::default(),
            ipt: [[0; 3]; 2],
            stream: None,
            path: PathBuf::new(),
            resource: None,
        }
    }
}
impl IfteEphemeris {
    pub fn init(&mut self, path: &Path, diagnostics: &mut Diagnostics) -> Result<(), TimingError> {
        let result = (|| {
            // ifteph.C::IFTE_init:138-188 (fda6ad4) leaks the replaced FILE and
            // leaves a dangling pointer after close. Drop the previous sole owner;
            // any explicitly saved probe owner was transferred before this call.
            self.stream = None;
            self.resource = None;
            self.path = path.into();
            let (file, mut resource) = open_resource(path, "IF99 time ephemeris").map_err(|e| {
                diagnostics.stderr(
                    format!(
                        "Error opening time ephemeris file '{}' : {}\n",
                        path.display(),
                        e.message
                    )
                    .as_bytes(),
                );
                terminal(path, "IFTE_open")
            })?;
            let mut stream = Stream::from_file(file)
                .map_err(|e| TimingError::new(path, None, "IFTE_fdopen", e.to_string()))?;
            // Native init ignores incomplete reads into uninitialized locals. Each
            // complete field is decoded safely; truncation is a contextual boundary.
            let read =
                |stream: &mut Stream, bytes: &mut [u8], size, count| -> Result<(), TimingError> {
                    if stream
                        .read(bytes, size, count)
                        .map_err(|e| TimingError::new(path, None, "IFTE_read", e.to_string()))?
                        != count
                    {
                        return Err(boundary("IFTE_incomplete_header"));
                    }
                    Ok(())
                };
            let mut discard = [0; 252];
            read(&mut stream, &mut discard, 1, 252)?;
            read(&mut stream, &mut discard, 1, 12)?;
            let mut bytes = [0; 8];
            read(&mut stream, &mut bytes, 1, 8)?;
            self.start_jd = f64::from_ne_bytes(bytes);
            read(&mut stream, &mut bytes, 1, 8)?;
            self.end_jd = f64::from_ne_bytes(bytes);
            read(&mut stream, &mut bytes, 1, 8)?;
            self.step_jd = f64::from_ne_bytes(bytes);
            let mut integer = [0; 4];
            read(&mut stream, &mut integer, 1, 4)?;
            let mut ncon = i32::from_ne_bytes(integer);
            self.swap_endian = i32::from(ncon != 2);
            if self.swap_endian != 0 {
                IFTswapInt(&mut ncon);
            }
            if ncon != 2 {
                diagnostics.stderr(
                    format!(
                        "Cannot understan format of time ephemeris file '{}'!\n",
                        path.display()
                    )
                    .as_bytes(),
                );
                return Err(terminal(path, "IFTE_format"));
            }
            if self.swap_endian != 0 {
                IFTswapDouble(&mut self.start_jd);
                IFTswapDouble(&mut self.end_jd);
                IFTswapDouble(&mut self.step_jd);
            }
            let mut descriptors = [0; 24];
            read(&mut stream, &mut descriptors, 8, 3)?;
            for (i, b) in descriptors.chunks_exact(4).enumerate() {
                self.ipt[i / 3][i % 3] = i32::from_ne_bytes(b.try_into().unwrap());
            }
            if self.swap_endian != 0 {
                for row in &mut self.ipt {
                    IFTswapInts(row, 3)?;
                }
            }
            self.reclen = self.ipt[1][0]
                .checked_sub(1)
                .and_then(|a| {
                    3i32.checked_mul(self.ipt[1][1])
                        .and_then(|b| b.checked_mul(self.ipt[1][2]))
                        .and_then(|b| a.checked_add(b))
                })
                .and_then(|a| a.checked_mul(8))
                .ok_or_else(|| boundary("IFTE_reclen_int"))?;
            if self.reclen < 0 || stream.seek(self.reclen.into(), 0) != 0 {
                return Err(boundary("IFTE_constants_seek"));
            }
            read(&mut stream, &mut bytes, 8, 1)?;
            let mut version = f64::from_ne_bytes(bytes);
            if self.swap_endian != 0 {
                IFTswapDouble(&mut version);
            }
            self.ephver = native_i32(version.floor(), "IFTE_ephver_int")?;
            read(&mut stream, &mut bytes, 8, 1)?;
            self.l_c = f64::from_ne_bytes(bytes);
            if self.swap_endian != 0 {
                IFTswapDouble(&mut self.l_c);
            }
            self.iinfo.np = 2;
            self.iinfo.nv = 3;
            self.iinfo.pc[0] = 1.0;
            self.iinfo.pc[1] = 0.0;
            // Bug fix: ifteph.C::IFTE_init:188-193 (fda6ad4) resets pc[1] but
            // retains twot. IFTEinterp:417 skips reset at tc==0, then uses the old
            // file's argument. Reset twot with pc[1]; keep all other retained cells.
            self.iinfo.twot = 0.0;
            self.iinfo.vc[1] = 1.0;
            self.irec = -1;
            resource.coverage_mjd = Some([self.start_jd - 2400000.5, self.end_jd - 2400000.5]);
            self.resource = Some(resource);
            self.stream = Some(stream);
            Ok(())
        })();
        result.map_err(|mut error: TimingError| {
            if error.path.as_os_str().is_empty() {
                error.path = self.path.clone();
            }
            error
        })
    }
    pub fn close_file(&mut self) {
        self.stream = None;
    }
    pub fn get_values(
        &mut self,
        jd0: f64,
        jd1: f64,
        kind: i32,
        res: &mut [f64],
        diagnostics: &mut Diagnostics,
    ) -> Result<(), TimingError> {
        let result = (|| {
            let mut whole0 = (jd0 - 0.5).floor();
            let mut frac0 = jd0 - 0.5 - whole0;
            let mut whole1 = jd1.floor();
            let frac1 = jd1 - whole1;
            whole0 += whole1 + 0.5;
            frac0 += frac1;
            whole1 = frac0.floor();
            let frac1 = frac0 - whole1;
            whole0 += whole1;
            if whole0 < self.start_jd {
                diagnostics.stderr(
                    format!(
                        "Error: Requested JD={whole0:.6} is less than start JD={:.6}\n",
                        self.start_jd
                    )
                    .as_bytes(),
                );
                return Err(terminal(&self.path, "IFTE_before_start"));
            }
            let mut irec = native_i32(
                ((whole0 - self.start_jd) / self.step_jd).floor(),
                "IFTE_record_int",
            )?
            .checked_add(2)
            .ok_or_else(|| boundary("IFTE_record_int"))?;
            if whole0 == self.end_jd {
                irec = irec
                    .checked_sub(1)
                    .ok_or_else(|| boundary("IFTE_record_int"))?;
            }
            let relative = irec
                .checked_sub(2)
                .ok_or_else(|| boundary("IFTE_record_int"))?;
            // ifteph.C:250 O2 fmadd for the record origin; Y arithmetic.py.
            let t = [
                (whole0 - (self.step_jd.mul_add(relative as f64, self.start_jd)) + frac1)
                    / self.step_jd,
                self.step_jd,
            ];
            let stream = self
                .stream
                .as_mut()
                .ok_or_else(|| boundary("IFTE_stream_lifetime"))?;
            if irec != self.irec {
                let count = usize::try_from(self.reclen / 8)
                    .ok()
                    .filter(|n| *n <= 322)
                    .ok_or_else(|| boundary("IFTE_record_storage"))?;
                let offset = self
                    .reclen
                    .checked_mul(irec)
                    .ok_or_else(|| boundary("IFTE_seek_int"))?;
                if stream.seek(offset.into(), 0) < 0 {
                    diagnostics.stderr(b"Error reading time ephemeris\n");
                    return Err(terminal(&self.path, "IFTE_seek"));
                }
                let mut bytes: Vec<u8> = self.buf[..count]
                    .iter()
                    .flat_map(|v| v.to_ne_bytes())
                    .collect();
                let nread = stream
                    .read(&mut bytes, 8, count)
                    .map_err(|e| TimingError::new(&self.path, None, "IFTE_read", e.to_string()))?;
                for (v, b) in self.buf.iter_mut().zip(bytes.chunks_exact(8)) {
                    *v = f64::from_ne_bytes(b.try_into().unwrap());
                }
                if nread < count {
                    diagnostics.stderr(format!("Error reading time ephemeris: Only read {nread} coefficients, wanted {count}!\n").as_bytes());
                    // ifteph.C:267 exits after partial writes; preserve diagnostic
                    // buffer prefix but release this owner before any recovery.
                    self.stream = None;
                    return Err(terminal(&self.path, "IFTE_short_record"));
                }
                if self.swap_endian != 0 {
                    IFTswapDoubles(&mut self.buf, count as i32)?;
                }
                // Source never publishes irec after a read; retain that behavior.
            }
            let group = usize::from(kind != 1);
            let [offset, ncf, na] = self.ipt[group];
            let begin = usize::try_from(
                offset
                    .checked_sub(1)
                    .ok_or_else(|| boundary("IFTE_descriptor_offset"))?,
            )
            .map_err(|_| boundary("IFTE_descriptor_offset"))?;
            let coef = self
                .buf
                .get(begin..)
                .ok_or_else(|| boundary("IFTE_descriptor_offset"))?;
            self.iinfo
                .interpolate(coef, t, ncf, if group == 0 { 1 } else { 3 }, na, 2, res)
        })();
        result.map_err(|mut error: TimingError| {
            if error.path.as_os_str().is_empty() {
                error.path = self.path.clone();
            }
            error
        })
    }
    pub fn get_delta_t_delta_t_dot(
        &mut self,
        jd0: f64,
        jd1: f64,
        out: &mut [f64],
        a: usize,
        b: usize,
        d: &mut Diagnostics,
    ) -> Result<(), TimingError> {
        let mut r = [0.0; 2];
        self.get_values(jd0, jd1, 1, &mut r, d)?;
        *out.get_mut(a)
            .ok_or_else(|| boundary("IFTE_scalar_output"))? = r[0];
        *out.get_mut(b)
            .ok_or_else(|| boundary("IFTE_scalar_output"))? = r[1];
        Ok(())
    }
    pub fn delta_t(&mut self, jd0: f64, jd1: f64, d: &mut Diagnostics) -> Result<f64, TimingError> {
        let mut r = [0.0; 2];
        self.get_delta_t_delta_t_dot(jd0, jd1, &mut r, 0, 1, d)?;
        Ok(r[0])
    }
    pub fn delta_t_dot(
        &mut self,
        jd0: f64,
        jd1: f64,
        d: &mut Diagnostics,
    ) -> Result<f64, TimingError> {
        let mut r = [0.0; 2];
        self.get_delta_t_delta_t_dot(jd0, jd1, &mut r, 0, 1, d)?;
        Ok(r[1])
    }
    pub fn get_ve_ve_dot(
        &mut self,
        jd0: f64,
        jd1: f64,
        out: &mut [f64],
        a: usize,
        b: usize,
        d: &mut Diagnostics,
    ) -> Result<(), TimingError> {
        let mut r = [0.0; 6];
        self.get_values(jd0, jd1, 2, &mut r, d)?;
        for i in 0..3 {
            *out.get_mut(a + i)
                .ok_or_else(|| boundary("IFTE_vector_output"))? = r[i];
            *out.get_mut(b + i)
                .ok_or_else(|| boundary("IFTE_vector_output"))? = r[i + 3];
        }
        Ok(())
    }
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "complete source convenience interface")
    )]
    pub fn get_ve(
        &mut self,
        jd0: f64,
        jd1: f64,
        d: &mut Diagnostics,
    ) -> Result<[f64; 3], TimingError> {
        let mut r = [0.0; 6];
        self.get_ve_ve_dot(jd0, jd1, &mut r, 0, 3, d)?;
        Ok(r[..3].try_into().unwrap())
    }
    pub fn get_ve_dot(
        &mut self,
        jd0: f64,
        jd1: f64,
        d: &mut Diagnostics,
    ) -> Result<[f64; 3], TimingError> {
        let mut r = [0.0; 6];
        self.get_ve_ve_dot(jd0, jd1, &mut r, 0, 3, d)?;
        Ok(r[3..].try_into().unwrap())
    }
}
