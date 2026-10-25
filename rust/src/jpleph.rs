//! Safe translation of jpleph.c, jpleph.h and jpl_int.h at fda6ad4a9753ad7dc5d291406f2f310a9161423f.
//! Project Pluto (C) 2011, Piotr A. Dybczynski / Bill J. Gray, GPL-2.0-or-later.
//! Source notation, binary64 grouping, selectors and cache-before-read are retained.
//! Decision 0017 supplies owned-state and invalid-access dispositions; stdio alone is native.
// Keep source indices visible across the scientific recurrences and body mappings.
#![allow(clippy::needless_range_loop)]
use crate::stdio::Stream;
use std::{
    fs::File,
    io,
    sync::atomic::{AtomicI32, Ordering},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Boundary(pub &'static str);
impl std::fmt::Display for Boundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JPL boundary: {}", self.0)
    }
}
impl std::error::Error for Boundary {}
type Result<T> = std::result::Result<T, Boundary>;

// Safe storage preserves the sequential global error contract, not C data races.
static INIT_ERROR: AtomicI32 = AtomicI32::new(-9);
pub(crate) fn jpl_init_error_code() -> i32 {
    INIT_ERROR.load(Ordering::SeqCst)
}
pub(crate) fn init_error(code: i32) {
    INIT_ERROR.store(code, Ordering::SeqCst);
}

#[cfg(test)]
pub(crate) fn reset_init_status_for_test() {
    init_error(-9);
}

pub(crate) fn swap_32_bit_val(bytes: &mut [u8; 4]) {
    bytes.reverse();
}
pub(crate) fn swap_64_bit_val(bytes: &mut [u8]) -> Result<()> {
    if !bytes.len().is_multiple_of(8) {
        return Err(Boundary("swap_storage"));
    }
    for word in bytes.chunks_exact_mut(8) {
        word.reverse();
    }
    Ok(())
}
pub(crate) fn dimension(idx: usize) -> usize {
    match idx {
        11 => 2,
        14 => 1,
        _ => 3,
    }
}
fn word(bytes: &[u8], at: usize, swap: bool) -> u32 {
    let mut b: [u8; 4] = bytes[at..at + 4].try_into().unwrap();
    if swap {
        swap_32_bit_val(&mut b);
    }
    u32::from_ne_bytes(b)
}
fn double(bytes: &[u8], at: usize, swap: bool) -> f64 {
    let mut b: [u8; 8] = bytes[at..at + 8].try_into().unwrap();
    if swap {
        b.reverse();
    }
    f64::from_ne_bytes(b)
}
fn read(stream: &mut Stream, bytes: &mut [u8], size: usize, count: usize) -> Result<usize> {
    stream
        .read(bytes, size, count)
        .map_err(|_| Boundary("read_storage"))
}

pub(crate) struct Ephemeris {
    pub ephem_start: f64,
    pub ephem_end: f64,
    pub ephem_step: f64,
    pub ncon: u32,
    pub au: f64,
    pub emrat: f64,
    pub ipt: [[u32; 3]; 15],
    pub ephemeris_version: u32,
    pub kernel_size: u32,
    pub recsize: u32,
    pub ncoeff: u32,
    pub swap_bytes: bool,
    pub curr_cache_loc: u32,
    pub cache: Vec<u8>,
    pub pvsun: Option<[f64; 9]>,
    pub pvsun_t: Option<f64>,
    pub iinfo: Interpolation,
    pub stream: Stream,
}

impl Ephemeris {
    /// jpl_init_ephemeris: a null result and a live -7/-8 handle are distinct.
    /// Optional callers supply initialized native-order raw bytes to retain even
    /// partial-element fread mutations. No packed struct or uninitialized bytes.
    pub fn init(
        file: io::Result<File>,
        names: Option<&mut [u8]>,
        values: Option<&mut [u8]>,
    ) -> Result<Option<Self>> {
        init_error(0);
        let mut stream = match file.and_then(Stream::from_file) {
            Ok(s) => s,
            Err(_) => {
                init_error(-1);
                return Ok(None);
            }
        };
        let mut title = [0; 84];
        let mut header = [0; 204];
        let mut status = if read(&mut stream, &mut title, 84, 1)? != 1 {
            -3
        } else if stream.seek(2652, 0) != 0 {
            -2
        } else if read(&mut stream, &mut header, 204, 1)? != 1 {
            -4
        } else {
            0
        };
        if status != 0 {
            init_error(status);
            return Ok(None);
        }
        // atoi(title+26): preserve whitespace/sign/no-digit zero, but never scan
        // past the title or overflow C int. Only an in-bounds non-digit yields zero.
        let mut at = 26;
        while at < 84 && matches!(title[at], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
            at += 1;
        }
        let negative = at < 84 && title[at] == b'-';
        if at < 84 && (title[at] == b'-' || title[at] == b'+') {
            at += 1;
        }
        if at == title.len() {
            return Err(Boundary("title_conversion"));
        }
        let mut version = 0_i64;
        while at < 84 && title[at].is_ascii_digit() {
            version = version
                .checked_mul(10)
                .and_then(|x| x.checked_add(i64::from(title[at] - b'0')))
                .ok_or(Boundary("title_conversion"))?;
            at += 1;
        }
        if negative {
            version = -version;
        }
        if (at == title.len()) || i32::try_from(version).is_err() {
            return Err(Boundary("title_conversion"));
        }
        let swap_bytes = word(&header, 24, false) > 65536;
        let mut ncon = word(&header, 24, swap_bytes);
        let mut ipt = [[0_u32; 3]; 15];
        for (i, row) in ipt[..13].iter_mut().enumerate() {
            let base = if i == 12 { 192 } else { 44 + 12 * i };
            for (j, v) in row.iter_mut().enumerate() {
                *v = word(&header, base + 4 * j, swap_bytes);
            }
        }
        if version >= 430 && ncon != 400 {
            if ncon > 400 {
                stream.seek(i64::from(ncon - 400) * 6, 1);
            }
            let mut extended = [0; 24];
            if read(&mut stream, &mut extended, 4, 6)? != 6 {
                status = -10;
                init_error(status);
            }
            for i in 0..2 {
                for j in 0..3 {
                    ipt[i + 13][j] = word(&extended, i * 12 + j * 4, swap_bytes);
                }
            }
        } else {
            // Only this marker is written. The first comparison normally clears
            // both rows; a second comparison would consume unread source words.
            ipt[13][0] = u32::MAX;
        }
        let next = |r: [u32; 3]| r[0].wrapping_add(r[1].wrapping_mul(r[2]).wrapping_mul(3));
        let invalid_extended = if ipt[13][0] != next(ipt[12]) {
            true
        } else if version < 430 || ncon == 400 {
            return Err(Boundary("unwritten_extended_header"));
        } else {
            ipt[14][0] != next(ipt[13])
        };
        if invalid_extended {
            ipt[13..].fill([0; 3]);
        }
        let emrat = double(&header, 36, swap_bytes);
        // Source comparisons deliberately allow NaN; do not replace with contains.
        #[allow(clippy::manual_range_contains)]
        if emrat > 81.3008 || emrat < 81.30055 {
            status = -5;
            init_error(status);
        }
        if status != 0 {
            return Ok(None);
        }
        let mut kernel_size = 4_u32;
        for (i, r) in ipt.iter().enumerate() {
            kernel_size = kernel_size.wrapping_add(
                2_u32
                    .wrapping_mul(r[1])
                    .wrapping_mul(r[2])
                    .wrapping_mul(dimension(i) as u32),
            );
        }
        let recsize = kernel_size.wrapping_mul(4);
        let ncoeff = kernel_size / 2;
        #[cfg(test)]
        if crate::stdio::observe(2, 0, 0) {
            init_error(-6);
            return Ok(None);
        }
        let mut cache = Vec::new();
        if cache.try_reserve_exact(recsize as usize).is_err() {
            init_error(-6);
            return Ok(None);
        }
        cache.resize(recsize as usize, 0);
        if ncon == 400 {
            let mut buff = [0; 6];
            stream.seek(2856, 0);
            while read(&mut stream, &mut buff, 6, 1)? != 0 && !buff.contains(&0) {
                ncon = ncon.wrapping_add(1);
            }
        }
        if let Some(values) = values {
            stream.seek(i64::from(recsize), 0);
            if read(&mut stream, values, 8, ncon as usize)? != ncon as usize {
                status = -7;
                init_error(status);
            } else if swap_bytes {
                swap_64_bit_val(&mut values[..ncon as usize * 8])?;
            }
        }
        if status == 0
            && let Some(names) = names
        {
            stream.seek(252, 0);
            for i in 0..ncon as usize {
                if i == 400 {
                    stream.seek(2856, 0);
                }
                let name = names
                    .get_mut(i * 6..i * 6 + 6)
                    .ok_or(Boundary("name_storage"))?;
                if read(&mut stream, name, 6, 1)? != 1 {
                    init_error(-8);
                    break;
                }
            }
        }
        Ok(Some(Self {
            ephem_start: double(&header, 0, swap_bytes),
            ephem_end: double(&header, 8, swap_bytes),
            ephem_step: double(&header, 16, swap_bytes),
            ncon,
            au: double(&header, 28, swap_bytes),
            emrat,
            ipt,
            ephemeris_version: version as u32,
            kernel_size,
            recsize,
            ncoeff,
            swap_bytes,
            curr_cache_loc: u32::MAX,
            cache,
            pvsun: None,
            pvsun_t: None,
            iinfo: Interpolation::new(),
            stream,
        }))
    }

    pub fn get_double(&self, value: i32) -> Result<f64> {
        match value {
            0 => Ok(self.ephem_start),
            8 => Ok(self.ephem_end),
            16 => Ok(self.ephem_step),
            28 => Ok(self.au),
            36 => Ok(self.emrat),
            _ => Err(Boundary("double_selector")),
        }
    }
    // Full source selector API is exercised directly, beyond the timing subset.
    #[allow(dead_code)]
    pub fn get_long(&self, value: i32) -> Result<i64> {
        let word = match value {
            24 => self.ncon,
            44..=220 if (value - 44) % 4 == 0 => {
                let index = (value - 44) as usize / 4;
                self.ipt[index / 3][index % 3]
            }
            224 => self.ephemeris_version,
            228 => self.kernel_size,
            232 => self.recsize,
            236 => self.ncoeff,
            240 => u32::from(self.swap_bytes),
            _ => return Err(Boundary("integer_selector")),
        };
        Ok(i64::from(word as i32))
    }
    // Full constant API is exercised directly, beyond the timing subset.
    #[allow(dead_code)]
    pub fn get_constant(&mut self, idx: i32, name: &mut [u8; 7]) -> Result<f64> {
        let mut value = [0; 8];
        name[0] = 0;
        if idx >= 0 && idx < self.ncon as i32 {
            let seek_loc = if idx < 400 {
                252 + i64::from(idx) * 6
            } else {
                2856 + i64::from(
                    (idx - 400)
                        .checked_mul(6)
                        .ok_or(Boundary("constant_offset"))?,
                )
            };
            self.stream.seek(seek_loc, 0);
            if read(&mut self.stream, &mut name[..6], 1, 6)? != 0 {
                name[6] = 0;
                self.stream
                    .seek(i64::from(self.recsize) + i64::from(idx) * 8, 0);
                if read(&mut self.stream, &mut value, 1, 8)? != 0 && self.swap_bytes {
                    swap_64_bit_val(&mut value)?;
                }
            }
        }
        Ok(f64::from_ne_bytes(value))
    }
    pub fn pvsun(&self) -> Option<&[f64; 9]> {
        self.pvsun.as_ref()
    }
}

pub(crate) struct Interpolation {
    pub posn_coeff: [f64; 18],
    pub vel_coeff: [f64; 18],
    pub twot: f64,
    pub n_posn_avail: usize,
    pub n_vel_avail: usize,
}
impl Interpolation {
    pub fn new() -> Self {
        let mut posn_coeff = [0.0; 18];
        posn_coeff[0] = 1.0;
        posn_coeff[1] = -2.0;
        let mut vel_coeff = [0.0; 18];
        vel_coeff[1] = 1.0;
        Self {
            posn_coeff,
            vel_coeff,
            twot: 0.0,
            n_posn_avail: 2,
            n_vel_avail: 2,
        }
    }
}

impl Interpolation {
    /// interp: reverse accumulation and recurrence grouping match the C source.
    #[allow(clippy::too_many_arguments)]
    pub fn interp(
        &mut self,
        coef: &[u8],
        t: [f64; 2],
        ncf: usize,
        ncm: usize,
        na: u32,
        velocity_flag: i32,
        posvel: &mut [f64],
    ) -> Result<()> {
        let dna = f64::from(na);
        let temp = dna * t[0];
        // C's unsigned conversion precedes its assertions. Check it explicitly;
        // Rust's saturating cast would invent a result for NaN/overflow.
        if !temp.is_finite() || temp.trunc() < 0.0 || temp.trunc() > f64::from(u32::MAX) {
            return Err(Boundary("interval_conversion"));
        }
        let mut l = temp as u32;
        let mut tc = 2.0 * temp.fract() - 1.0;
        if ncf >= 18 {
            return Err(Boundary("chebyshev_count"));
        }
        if l == na {
            l = l.checked_sub(1).ok_or(Boundary("interval_index"))?;
            tc = 1.0;
        }
        if !(-1.0..=1.0).contains(&tc) {
            return Err(Boundary("chebyshev_time"));
        }
        if tc != self.posn_coeff[1] {
            self.n_posn_avail = 2;
            self.n_vel_avail = 2;
            self.posn_coeff[1] = tc;
            self.twot = tc + tc;
        }
        if self.n_posn_avail < ncf {
            for i in self.n_posn_avail..ncf {
                self.posn_coeff[i] = self.twot * self.posn_coeff[i - 1] - self.posn_coeff[i - 2];
            }
            self.n_posn_avail = ncf;
        }
        let quantities = if velocity_flag == 3 {
            3
        } else if velocity_flag > 1 {
            2
        } else {
            1
        };
        if posvel.len()
            < ncm
                .checked_mul(quantities)
                .ok_or(Boundary("output_storage"))?
        {
            return Err(Boundary("output_storage"));
        }
        let start = ncf
            .checked_mul(l as usize)
            .and_then(|x| x.checked_mul(ncm))
            .ok_or(Boundary("coefficient_storage"))?;
        let end = ncf
            .checked_mul(ncm)
            .and_then(|x| x.checked_add(start))
            .and_then(|x| x.checked_mul(8))
            .ok_or(Boundary("coefficient_storage"))?;
        if end > coef.len() {
            return Err(Boundary("coefficient_storage"));
        }
        for i in 0..ncm {
            posvel[i] = 0.0;
            for j in (0..ncf).rev() {
                posvel[i] += self.posn_coeff[j] * double(coef, (start + ncf * i + j) * 8, false);
            }
        }
        if velocity_flag <= 1 {
            return Ok(());
        }
        // ncf=0 would underflow the source's unsigned derivative loop.
        if ncf == 0 {
            return Err(Boundary("derivative_count"));
        }
        if self.n_vel_avail < ncf {
            for i in self.n_vel_avail..ncf {
                self.vel_coeff[i] = self.twot * self.vel_coeff[i - 1]
                    + self.posn_coeff[i - 1]
                    + self.posn_coeff[i - 1]
                    - self.vel_coeff[i - 2];
            }
            self.n_vel_avail = ncf;
        }
        let vfac = (dna + dna) / t[1];
        for i in 0..ncm {
            let mut tval = 0.0;
            for j in (1..ncf).rev() {
                tval += self.vel_coeff[j] * double(coef, (start + ncf * i + j) * 8, false);
            }
            posvel[ncm + i] = tval * vfac;
        }
        if velocity_flag == 3 {
            let mut accel_coeffs = [0.0; 18];
            for i in 2..ncf {
                accel_coeffs[i] = 4.0 * self.vel_coeff[i - 1] + self.twot * accel_coeffs[i - 1]
                    - accel_coeffs[i - 2];
            }
            for i in 0..ncm {
                let mut tval = 0.0;
                for j in (0..ncf).rev() {
                    tval += accel_coeffs[j] * double(coef, (start + ncf * i + j) * 8, false);
                }
                posvel[ncm * 2 + i] = tval * vfac * vfac;
            }
        }
        Ok(())
    }
}

impl Ephemeris {
    /// jpl_state: each pv row is an initialized slice, permitting the source's
    /// direct acceleration demands without imposing its stale six-slot spelling.
    pub fn state(
        &mut self,
        et: f64,
        list: &[i32; 14],
        pv: &mut [&mut [f64]],
        nut: &mut [f64],
        bary: i32,
    ) -> Result<i32> {
        let block_loc = (et - self.ephem_start) / self.ephem_step;
        let aufac = 1.0 / self.au;
        if et < self.ephem_start || et > self.ephem_end {
            return Ok(-1);
        }
        if !block_loc.is_finite()
            || block_loc.trunc() < 0.0
            || block_loc.trunc() > f64::from(u32::MAX)
        {
            return Err(Boundary("record_conversion"));
        }
        let mut nr = block_loc as u32;
        let mut t = [block_loc - f64::from(nr), self.ephem_step];
        if t[0] == 0.0 && nr != 0 {
            t[0] = 1.0;
            nr -= 1;
        }
        if nr != self.curr_cache_loc {
            // Canonical deterministic behavior: commit before either I/O operation.
            // A failed/partial read leaves old/raw bytes and retry skips all I/O.
            self.curr_cache_loc = nr;
            let offset = nr.wrapping_add(2).wrapping_mul(self.recsize);
            if self.stream.seek(i64::from(offset), 0) != 0 {
                return Ok(-6);
            }
            if read(&mut self.stream, &mut self.cache, 8, self.ncoeff as usize)?
                != self.ncoeff as usize
            {
                return Ok(-2);
            }
            if self.swap_bytes {
                swap_64_bit_val(&mut self.cache[..self.ncoeff as usize * 8])?;
            }
        }
        let recompute_pvsun = self.pvsun_t != Some(et);
        if recompute_pvsun {
            self.pvsun_t = Some(et);
        }
        for n_intervals in [1, 2, 4, 8] {
            for i in 0..15 {
                // Select before indexing: never form C's unused ipt[15]/pv[14].
                let (iptr, quantities) = if i == 14 {
                    (self.ipt[10], if recompute_pvsun { 3 } else { 0 })
                } else {
                    (self.ipt[if i < 10 { i } else { i + 1 }], list[i] as u32)
                };
                // C assigns list to unsigned, passes its bits back to int for
                // interp, and wraps quantities*3 for AU scaling. Optional rows
                // skip that scaling; flags >3 request velocity, negative flags
                // request position only. Bound actual storage, not flag values.
                if n_intervals == iptr[2] && quantities != 0 {
                    let offset = iptr[0]
                        .checked_sub(1)
                        .and_then(|x| (x as usize).checked_mul(8))
                        .ok_or(Boundary("coefficient_storage"))?;
                    let coef = self
                        .cache
                        .get(offset..)
                        .ok_or(Boundary("coefficient_storage"))?;
                    let dest: &mut [f64] = if i < 10 {
                        pv.get_mut(i).ok_or(Boundary("output_storage"))?
                    } else if i == 14 {
                        self.pvsun.get_or_insert([0.0; 9])
                    } else {
                        &mut *nut
                    };
                    self.iinfo.interp(
                        coef,
                        t,
                        iptr[1] as usize,
                        dimension(i + 1),
                        n_intervals,
                        quantities as i32,
                        dest,
                    )?;
                    if i < 10 || i == 14 {
                        let dest = dest
                            .get_mut(..quantities.wrapping_mul(3) as usize)
                            .ok_or(Boundary("output_storage"))?;
                        for x in dest {
                            *x *= aufac;
                        }
                    }
                }
            }
        }
        if bary == 0 {
            for i in 0..9 {
                let n = (list[i] as u32).wrapping_mul(3) as usize;
                if n == 0 {
                    continue;
                }
                let row = pv
                    .get_mut(i)
                    .and_then(|p| p.get_mut(..n))
                    .ok_or(Boundary("output_storage"))?;
                let sun = self
                    .pvsun
                    .as_ref()
                    .ok_or(Boundary("sun_uninitialized"))?
                    .get(..n)
                    .ok_or(Boundary("output_storage"))?;
                for j in 0..n {
                    row[j] -= sun[j];
                }
            }
        }
        Ok(0)
    }

    /// jpl_pleph. On a nonzero state status, the C continuation reads unwritten
    /// private pv; return the status and prior public mutations, never that vector.
    pub fn pleph(
        &mut self,
        et: f64,
        ntarg: i32,
        ncent: i32,
        rrd: &mut [f64; 6],
        calc_velocity: i32,
    ) -> Result<i32> {
        rrd.fill(0.0);
        if ntarg == ncent {
            return Ok(0);
        }
        let list_val = if calc_velocity != 0 { 2 } else { 1 };
        let mut list = [0; 14];
        let mut pv = [[0.0; 6]; 13];
        for i in 0..4 {
            if ntarg == i as i32 + 14 {
                if self.ipt[i + 11][1] == 0 {
                    return Ok(-3);
                }
                list[i + 10] = list_val;
                return self.state(
                    et,
                    &list,
                    &mut pv.each_mut().map(|p| p.as_mut_slice()),
                    rrd,
                    0,
                );
            }
        }
        if ntarg > 13 || ncent > 13 || ntarg < 1 || ncent < 1 {
            return Ok(-5);
        }
        for body in [ntarg, ncent] {
            let k = (body - 1) as usize;
            if k <= 9 {
                list[k] = list_val;
            }
            if k == 9 {
                list[2] = list_val;
            }
            if k == 2 {
                list[9] = list_val;
            }
            if k == 12 {
                list[2] = list_val;
            }
        }
        let rval = self.state(
            et,
            &list,
            &mut pv.each_mut().map(|p| p.as_mut_slice()),
            rrd,
            1,
        )?;
        if rval != 0 {
            return Ok(rval);
        }
        if list[..10]
            .iter()
            .enumerate()
            .any(|(i, q)| *q != 0 && ![1, 2, 4, 8].contains(&self.ipt[i][2]))
        {
            return Err(Boundary("pleph_unwritten_row"));
        }
        let n = list_val as usize * 3;
        // Only source-defined requested components are read; the cleared public
        // position-only suffix stays zero (including EMB's unwritten C velocities).
        if ntarg == 11 || ncent == 11 {
            pv[10][..n].copy_from_slice(&self.pvsun().ok_or(Boundary("sun_uninitialized"))?[..n]);
        }
        if ntarg == 12 || ncent == 12 {
            pv[11].fill(0.0);
        }
        if ntarg == 13 || ncent == 13 {
            for i in 0..n {
                pv[12][i] = pv[2][i];
            }
        }
        if ntarg * ncent == 30 && ntarg + ncent == 13 {
            pv[2].fill(0.0);
        } else {
            if list[2] != 0 {
                for i in 0..list[2] as usize * 3 {
                    pv[2][i] -= pv[9][i] / (1.0 + self.emrat);
                }
            }
            if list[9] != 0 {
                for i in 0..list[9] as usize * 3 {
                    pv[9][i] += pv[2][i];
                }
            }
        }
        for i in 0..n {
            rrd[i] = pv[ntarg as usize - 1][i] - pv[ncent as usize - 1][i];
        }
        Ok(rval)
    }
}

#[cfg(test)]
#[path = "jpleph_tests.rs"]
mod tests;
