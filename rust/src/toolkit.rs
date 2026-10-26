//! T2toolkit.C and t2toolkit_global.C at fda6ad4a: statistics and MT19937.
// Copyright (C) 2006-2009 George Hobbs, Russell Edwards. GPL-3.0-or-later.
// MT19937 attribution and license are retained in MT19937_LICENSE.txt.
use crate::{TimingError, clock_diagnostics::boundary};

pub fn convert_float1(x: &[f64]) -> Vec<f32> {
    x.iter().map(|&v| v as f32).collect()
}
pub fn convert_float2(x: &[f64], y: &[f64]) -> Result<(Vec<f32>, Vec<f32>), TimingError> {
    if x.len() != y.len() {
        return Err(boundary("TKconvertFloat2 dimensions"));
    }
    Ok((convert_float1(x), convert_float1(y)))
}
pub fn sign_d(a: f64, b: f64) -> f64 {
    if b >= 0.0 { a.abs() } else { -a.abs() }
}
pub fn ret_max_d(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
pub fn ret_min_d(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
pub fn ret_max_f(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}
pub fn ret_min_f(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
pub fn ret_min_i(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

macro_rules! stats {
    ($t:ty,$min:ident,$max:ident,$mean:ident,$median:ident,$range:ident,$sort:ident) => {
        pub fn $min(x: &[$t]) -> Result<$t, TimingError> {
            let mut v = *x
                .first()
                .ok_or_else(|| boundary("TKfindMin empty backing"))?;
            for &a in x {
                if a < v {
                    v = a;
                }
            }
            Ok(v)
        }
        pub fn $max(x: &[$t]) -> Result<$t, TimingError> {
            let mut v = *x
                .first()
                .ok_or_else(|| boundary("TKfindMax empty backing"))?;
            for &a in x {
                if a > v {
                    v = a;
                }
            }
            Ok(v)
        }
        pub fn $mean(x: &[$t]) -> $t {
            let mut sum: $t = 0.0;
            for &a in x {
                sum += a;
            }
            sum / x.len() as $t
        }
        pub fn $sort(x: &mut [$t]) {
            loop {
                let mut changed = false;
                for i in 0..x.len().saturating_sub(1) {
                    if x[i] > x[i + 1] {
                        x.swap(i, i + 1);
                        changed = true;
                    }
                }
                if !changed {
                    break;
                }
            }
        }
        pub fn $median(x: &[$t]) -> Result<$t, TimingError> {
            if x.is_empty() {
                return Err(boundary("TKfindMedian empty backing"));
            }
            let mut v = x.to_vec();
            $sort(&mut v);
            let n = v.len();
            Ok(if n % 2 == 0 {
                (v[n / 2 - 1] + v[n / 2]) / 2.0
            } else {
                v[n / 2]
            })
        }
        pub fn $range(x: &[$t]) -> Result<$t, TimingError> {
            Ok($max(x)? - $min(x)?)
        }
    };
}
stats!(
    f32,
    find_min_f,
    find_max_f,
    mean_f,
    find_median_f,
    range_f,
    sort_f
);
stats!(
    f64,
    find_min_d,
    find_max_d,
    mean_d,
    find_median_d,
    range_d,
    sort_d
);
pub fn find_rms_f(x: &[f32]) -> f32 {
    let mean = mean_f(x);
    let mut sdev = 0.0f32;
    for &v in x {
        sdev = (sdev as f64 + ((v - mean) as f64).powf(2.0)) as f32;
    }
    sdev /= (x.len() as i64 - 1) as f32;
    (sdev as f64).sqrt() as f32
}
pub fn find_rms_d(x: &[f64]) -> f64 {
    let mean = mean_d(x);
    let mut sdev = 0.0;
    for &v in x {
        sdev += (v - mean).powf(2.0);
    }
    (sdev / (x.len() as f64 - 1.0)).sqrt()
}
pub fn find_rms_weight_d(x: &[f64], e: &[f64]) -> Result<f32, TimingError> {
    if x.len() != e.len() {
        return Err(boundary("TKfindRMSweight_d dimensions"));
    }
    let (mut sum, mut sumsq, mut sumwgt) = (0.0, 0.0, 0.0);
    for (&v, &error) in x.iter().zip(e) {
        let w = 1.0 / error / error;
        sum += w * v;
        sumsq += w * v * v;
        sumwgt += w;
    }
    Ok(((sumsq - sum * sum / sumwgt) / sumwgt).sqrt() as f32)
}
pub fn variance_d(x: &[f64]) -> f64 {
    let mean = mean_d(x);
    let (mut var, mut ep) = (0.0, 0.0);
    for &v in x {
        let s = v - mean;
        ep += s;
        var += s * s;
    }
    (var - ep * ep / x.len() as f64) / (x.len() as f64 - 1.0)
}
pub fn sort_2f(x: &mut [f32], y: &mut [f32]) -> Result<(), TimingError> {
    if x.len() != y.len() {
        return Err(boundary("TKsort_2f dimensions"));
    }
    loop {
        let mut changed = false;
        for i in 0..x.len().saturating_sub(1) {
            if x[i] > x[i + 1] {
                x.swap(i, i + 1);
                y.swap(i, i + 1);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    Ok(())
}
pub fn sort_3d(x: &mut [f64], y: &mut [f64], z: &mut [f64]) -> Result<(), TimingError> {
    if x.len() != y.len() || x.len() != z.len() {
        return Err(boundary("TKsort_3d dimensions"));
    }
    loop {
        let mut changed = false;
        for i in 0..x.len().saturating_sub(1) {
            if x[i] > x[i + 1] {
                x.swap(i, i + 1);
                y.swap(i, i + 1);
                z.swap(i, i + 1);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    Ok(())
}
pub fn zero_mean_d(y: &mut [f64]) {
    let mean = mean_d(y);
    for v in y {
        *v -= mean;
    }
}
pub fn max_observations() -> Result<i32, TimingError> {
    crate::clkcorr::with_process(|s| Ok(s.source.max_observations))
}

/// T2toolkit.C:512–565. Each owner is one source process random stream.
#[derive(Clone, Debug)]
pub struct ToolkitRng {
    mt: [u32; 624],
    index: usize,
    draws: u64,
}
impl Default for ToolkitRng {
    fn default() -> Self {
        Self {
            mt: [0; 624],
            index: 625,
            draws: 0,
        }
    }
}
impl ToolkitRng {
    pub fn state(&self) -> (&[u32; 624], usize, u64) {
        (&self.mt, self.index, self.draws)
    }
    pub fn init_genrand(&mut self, seed: u64) {
        self.mt[0] = seed as u32;
        for i in 1..624 {
            self.mt[i] = 1812433253u32
                .wrapping_mul(self.mt[i - 1] ^ (self.mt[i - 1] >> 30))
                .wrapping_add(i as u32);
        }
        self.index = 624;
    }
    pub fn genrand_int32(&mut self) -> u32 {
        if self.index >= 624 {
            if self.index == 625 {
                self.init_genrand(5489);
            }
            for k in 0..624 {
                let y = (self.mt[k] & 0x80000000) | (self.mt[(k + 1) % 624] & 0x7fffffff);
                self.mt[k] =
                    self.mt[(k + 397) % 624] ^ (y >> 1) ^ if y & 1 == 1 { 0x9908b0df } else { 0 };
            }
            self.index = 0;
        }
        let mut y = self.mt[self.index];
        self.index += 1;
        self.draws += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^= y >> 18;
        y
    }
    pub fn genrand_real1(&mut self) -> f64 {
        self.genrand_int32() as f64 * (1.0 / 4294967295.0)
    }
    pub fn ran_dev(&mut self, seed: &mut i64) -> Result<f64, TimingError> {
        if *seed < 0 {
            self.init_genrand(
                seed.checked_neg()
                    .ok_or_else(|| boundary("TKranDev signed seed negation overflow"))?
                    as u64,
            );
            *seed = 1;
        }
        Ok(self.genrand_real1())
    }
    pub fn gauss_dev(&mut self, seed: &mut i64) -> Result<f64, TimingError> {
        loop {
            let x = -1.0 + 2.0 * self.ran_dev(seed)?;
            let y = -1.0 + 2.0 * self.ran_dev(seed)?;
            let r2 = x * x + y * y;
            if !(r2 > 1.0 || r2 == 0.0) {
                return Ok(y * (-2.0 * r2.ln() / r2).sqrt());
            }
        }
    }
}
pub fn set_seed_at(seconds: i64) -> Result<i64, TimingError> {
    seconds
        .checked_neg()
        .ok_or_else(|| boundary("TKsetSeed negation overflow"))
}
pub fn set_seed() -> Result<i64, TimingError> {
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| boundary(&e.to_string()))?;
    set_seed_at(i64::try_from(time.as_secs()).map_err(|_| boundary("TKsetSeed time range"))?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_sort_classes_and_associations() {
        let mut x = [2.0, 0.0, -0.0, 1.0, f64::NAN, -1.0];
        let mut y = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let mut z = y;
        sort_3d(&mut x, &mut y, &mut z).unwrap();
        assert_eq!(y, [1.0, 2.0, 3.0, 0.0, 4.0, 5.0]);
        assert_eq!(y, z);
        assert_eq!(x[1].to_bits(), (-0.0f64).to_bits());
        assert!(mean_d(&[]).is_nan());
        assert!(find_min_d(&[]).is_err());
        assert!(find_rms_d(&[1.0]).is_nan());
    }
    #[test]
    fn mt_known_default_and_reseed() {
        let mut r = ToolkitRng::default();
        assert_eq!(
            (0..5).map(|_| r.genrand_int32()).collect::<Vec<_>>(),
            [3499211612, 581869302, 3890346734, 3586334585, 545404204]
        );
        let mut seed = -5489;
        assert_eq!(
            r.ran_dev(&mut seed).unwrap(),
            3499211612.0 * (1.0 / 4294967295.0)
        );
        assert_eq!(seed, 1);
        let mut seed = i64::MIN;
        assert!(r.ran_dev(&mut seed).is_err());
        assert_eq!(set_seed_at(1234).unwrap(), -1234);
    }
}
