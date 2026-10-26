//! GWsim.C source geometry, populations, harmonics and residuals (fda6ad4a).
// Copyright (C) 2006-2009 George Hobbs, Russell Edwards and TEMPO2 contributors.
// GPL-3.0-or-later. Angles radians, omega rad/s, distances metres, time seconds.
#![allow(clippy::needless_range_loop, clippy::too_many_arguments)]
use crate::{TimingError, clock_diagnostics::boundary, tksvd::ld, toolkit::ToolkitRng};
use rug::{Float, ops::Pow};
use std::f64::consts::PI;
pub type Matrix3 = [[Float; 3]; 3];
pub type Vector3 = [Float; 3];
#[derive(Clone, Debug)]
pub struct Geometry {
    pub h: Matrix3,
    pub h_im: Matrix3,
    pub kg: Vector3,
}
#[derive(Clone, Debug)]
pub struct TensorSource {
    pub theta_g: Float,
    pub phi_g: Float,
    pub omega_g: Float,
    pub phi_polar_g: Float,
    pub phase_g: Float,
    pub aplus_g: Float,
    pub aplus_im_g: Float,
    pub across_g: Float,
    pub across_im_g: Float,
    pub geometry: Option<Geometry>,
    /// Source phi_bin/theta_bin/inc_bin/dist_bin, never written by populations/files.
    pub binary_metadata: Option<[Float; 4]>,
}
impl TensorSource {
    pub fn from_prefix(v: [Float; 9]) -> Self {
        let [
            theta_g,
            phi_g,
            omega_g,
            phi_polar_g,
            phase_g,
            aplus_g,
            aplus_im_g,
            across_g,
            across_im_g,
        ] = v;
        Self {
            theta_g,
            phi_g,
            omega_g,
            phi_polar_g,
            phase_g,
            aplus_g,
            aplus_im_g,
            across_g,
            across_im_g,
            geometry: None,
            binary_metadata: None,
        }
    }
    pub fn prefix(&self) -> [&Float; 9] {
        [
            &self.theta_g,
            &self.phi_g,
            &self.omega_g,
            &self.phi_polar_g,
            &self.phase_g,
            &self.aplus_g,
            &self.aplus_im_g,
            &self.across_g,
            &self.across_im_g,
        ]
    }
}
#[derive(Clone, Debug)]
pub struct GeneralSource {
    pub theta_g: Float,
    pub phi_g: Float,
    pub omega_g: Float,
    pub phi_polar_g: Float,
    pub phase_g: Float,
    pub aplus_g: Float,
    pub aplus_im_g: Float,
    pub across_g: Float,
    pub across_im_g: Float,
    pub ast_g: Float,
    pub ast_im_g: Float,
    pub asl_g: Float,
    pub asl_im_g: Float,
    pub avx_g: Float,
    pub avx_im_g: Float,
    pub avy_g: Float,
    pub avy_im_g: Float,
    pub geometry: Option<Geometry>,
    /// Source phi_bin/theta_bin/inc_bin/dist_bin, never written by populations/files.
    pub binary_metadata: Option<[Float; 4]>,
}
impl GeneralSource {
    pub fn from_prefix(v: [Float; 17]) -> Self {
        let [
            theta_g,
            phi_g,
            omega_g,
            phi_polar_g,
            phase_g,
            aplus_g,
            aplus_im_g,
            across_g,
            across_im_g,
            ast_g,
            ast_im_g,
            asl_g,
            asl_im_g,
            avx_g,
            avx_im_g,
            avy_g,
            avy_im_g,
        ] = v;
        Self {
            theta_g,
            phi_g,
            omega_g,
            phi_polar_g,
            phase_g,
            aplus_g,
            aplus_im_g,
            across_g,
            across_im_g,
            ast_g,
            ast_im_g,
            asl_g,
            asl_im_g,
            avx_g,
            avx_im_g,
            avy_g,
            avy_im_g,
            geometry: None,
            binary_metadata: None,
        }
    }
    pub fn prefix(&self) -> [&Float; 17] {
        [
            &self.theta_g,
            &self.phi_g,
            &self.omega_g,
            &self.phi_polar_g,
            &self.phase_g,
            &self.aplus_g,
            &self.aplus_im_g,
            &self.across_g,
            &self.across_im_g,
            &self.ast_g,
            &self.ast_im_g,
            &self.asl_g,
            &self.asl_im_g,
            &self.avx_g,
            &self.avx_im_g,
            &self.avy_g,
            &self.avy_im_g,
        ]
    }
}
pub fn matrix_mult(a: &Matrix3, b: &Matrix3) -> Matrix3 {
    std::array::from_fn(|i| {
        std::array::from_fn(|j| {
            let mut v = ld(0.0);
            for k in 0..3 {
                v += a[i][k].clone() * &b[k][j];
            }
            v
        })
    })
}
pub fn dot_product(a: &Vector3, b: &Vector3) -> Float {
    a[0].clone() * &b[0] + a[1].clone() * &b[1] + a[2].clone() * &b[2]
}
pub fn setup_pulsar(ra: &Float, dec: &Float) -> Vector3 {
    [
        dec.clone().cos() * ra.clone().cos(),
        dec.clone().cos() * ra.clone().sin(),
        dec.clone().sin(),
    ]
}
fn setup(theta: &Float, phi: &Float, polar: &Float, h: Matrix3, h_im: Matrix3) -> Geometry {
    let (st, ct, sp, cp, cpp, spp) = (
        theta.clone().sin(),
        theta.clone().cos(),
        phi.clone().sin(),
        phi.clone().cos(),
        polar.clone().cos(),
        polar.clone().sin(),
    );
    let kg = [st.clone() * &cp, st.clone() * &sp, ct.clone()];
    let convert = [
        [
            cp.clone() * &ct * &cpp - sp.clone() * &spp,
            sp.clone() * &ct * &cpp + cp.clone() * &spp,
            -st.clone() * &cpp,
        ],
        [
            -cp.clone() * &ct * &spp - sp.clone() * &cpp,
            -sp.clone() * &ct * &spp + cp.clone() * &cpp,
            st.clone() * &spp,
        ],
        [cp * &st, sp * &st, ct],
    ];
    let trans = std::array::from_fn(|i| std::array::from_fn(|j| convert[j][i].clone()));
    Geometry {
        h: matrix_mult(&trans, &matrix_mult(&h, &convert)),
        h_im: matrix_mult(&trans, &matrix_mult(&h_im, &convert)),
        kg,
    }
}
impl TensorSource {
    pub fn setup(&mut self) {
        let h = |plus: &Float, cross: &Float| {
            [
                [plus.clone(), cross.clone(), ld(0.0)],
                [cross.clone(), -plus.clone(), ld(0.0)],
                [ld(0.0), ld(0.0), ld(0.0)],
            ]
        };
        self.geometry = Some(setup(
            &self.theta_g,
            &self.phi_g,
            &self.phi_polar_g,
            h(&self.aplus_g, &self.across_g),
            h(&self.aplus_im_g, &self.across_im_g),
        ));
    }
    pub fn residual(&self, kp: &Vector3, time: &Float, dist: &Float) -> Result<Float, TimingError> {
        residual(
            kp,
            self.geometry
                .as_ref()
                .ok_or_else(|| boundary("setupGW required before residual"))?,
            &self.phase_g,
            &self.omega_g,
            time,
            dist,
            &ld(299792458.0),
        )
    }
}
impl GeneralSource {
    pub fn setup(&mut self) {
        // Preserve source h[1][1]'s additional scalar-longitudinal term.
        let h = |plus: &Float, cross: &Float, st: &Float, sl: &Float, vx: &Float, vy: &Float| {
            [
                [plus.clone() + st, cross.clone(), vx.clone()],
                [cross.clone(), -plus.clone() + st + sl, vy.clone()],
                [vx.clone(), vy.clone(), sl.clone()],
            ]
        };
        self.geometry = Some(setup(
            &self.theta_g,
            &self.phi_g,
            &self.phi_polar_g,
            h(
                &self.aplus_g,
                &self.across_g,
                &self.ast_g,
                &self.asl_g,
                &self.avx_g,
                &self.avy_g,
            ),
            h(
                &self.aplus_im_g,
                &self.across_im_g,
                &self.ast_im_g,
                &self.asl_im_g,
                &self.avx_im_g,
                &self.avy_im_g,
            ),
        ));
    }
    pub fn residual(&self, kp: &Vector3, time: &Float, dist: &Float) -> Result<Float, TimingError> {
        let c = Float::with_val(
            crate::PRECISION_BITS,
            Float::parse("299792456.200").unwrap(),
        );
        residual(
            kp,
            self.geometry
                .as_ref()
                .ok_or_else(|| boundary("setupgeneralGW required before residual"))?,
            &self.phase_g,
            &self.omega_g,
            time,
            dist,
            &c,
        )
    }
}
fn residual(
    kp: &Vector3,
    g: &Geometry,
    phase: &Float,
    omega: &Float,
    time: &Float,
    dist: &Float,
    c: &Float,
) -> Result<Float, TimingError> {
    let apply = |h: &Matrix3| {
        std::array::from_fn(|i| {
            let mut v = ld(0.0);
            for k in 0..3 {
                v += h[i][k].clone() * &kp[k];
            }
            v
        })
    };
    let cos_mu = dot_product(kp, &g.kg);
    let den = ld(1.0) + cos_mu;
    // 0053: GWsim.C:263/1057 at fda6ad4a writes only real psrVal1 at
    // exact den==0, then consumes unwritten psrVal1_im. Set both source
    // geometric factors to zero at that exact branch; no near-alignment cut.
    let (re, im) = if den != 0 {
        (
            ld(0.5) / &den * dot_product(kp, &apply(&g.h)),
            ld(0.5) / &den * dot_product(kp, &apply(&g.h_im)),
        )
    } else {
        (ld(0.0), ld(0.0))
    };
    let arg = phase.clone() + time.clone() * omega;
    let earth = re.clone() * arg.clone().sin() + im.clone() * arg.cos();
    let psr = if dist == &0 {
        ld(0.0)
    } else {
        let a = phase.clone() - den * dist / c * omega + time.clone() * omega;
        re * a.clone().sin() + im * a.cos()
    };
    Ok((earth - psr) / omega)
}
pub fn psrangle(centre_long: f64, centre_lat: f64, psr_long: f64, psr_lat: f64) -> f64 {
    let dlon = psr_long - centre_long;
    let dlat = psr_lat - centre_lat;
    let a = (dlat / 2.0).sin().powf(2.0)
        + centre_lat.cos() * psr_lat.cos() * (dlon / 2.0).sin().powf(2.0);
    if a == 1.0 {
        PI
    } else {
        2.0 * a.sqrt().atan2((1.0 - a).sqrt())
    }
}
pub fn sphharm(l: i32, m: i32, x: f64) -> Result<f64, TimingError> {
    if m < 0 || m > l || x.abs() > 1.0 {
        return Err(boundary("Bad arguments in routine sphharm; source exit(0)"));
    }
    if m.checked_mul(2).and_then(|v| v.checked_add(3)).is_none() {
        return Err(boundary("sphharm source integer overflow"));
    }
    let mut pmm = 1.0;
    if m > 0 {
        let omx2 = (1.0 - x) * (1.0 + x);
        let mut fact = 1.0;
        for _ in 1..=m {
            pmm *= omx2 * fact / (fact + 1.0);
            fact += 2.0;
        }
    }
    pmm = (((2 * m + 1) as f64) * pmm / (4.0 * PI)).sqrt();
    if m & 1 != 0 {
        pmm = -pmm;
    }
    if l == m {
        return Ok(pmm);
    }
    let mut pmmp1 = x * ((2 * m + 3) as f64).sqrt() * pmm;
    if l == m + 1 {
        return Ok(pmmp1);
    }
    let mut oldfact = ((2 * m + 3) as f64).sqrt();
    for ll in m + 2..=l {
        let numerator = ll
            .checked_mul(ll)
            .and_then(|v| v.checked_mul(4))
            .and_then(|v| v.checked_sub(1))
            .ok_or_else(|| boundary("sphharm source integer overflow"))?;
        let fact = (numerator as f64 / (ll * ll - m * m) as f64).sqrt();
        let pll = (x * pmmp1 - pmm / oldfact) * fact;
        oldfact = fact;
        pmm = pmmp1;
        pmmp1 = pll;
    }
    Ok(pmmp1)
}
pub fn find_phi(prob: f64, amp: f64, phase: f64) -> f64 {
    let (mut low, mut high, mut phi) = (0.0, 2.0 * PI, 0.0);
    while high - low > 1e-10 {
        phi = 0.5 * (low + high);
        let v = phi / (2.0 * PI) + amp * (phi + phase).sin() - amp * phase.sin() - prob;
        if v > 0.0 {
            high = phi;
        } else {
            low = phi;
        }
    }
    phi
}
#[derive(Clone, Debug)]
pub struct PopulationOptions {
    pub flo: Float,
    pub fhi: Float,
    pub loglin: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct GeneralSpectrum {
    pub tensor_amp: f64,
    pub st_amp: f64,
    pub sl_amp: f64,
    pub vl_amp: f64,
    pub tensor_alpha: f64,
    pub st_alpha: f64,
    pub sl_alpha: f64,
    pub vl_alpha: f64,
}
#[derive(Clone, Copy, Debug)]
pub struct Harmonic {
    pub l: f64,
    pub m: f64,
    pub amplitude: f64,
}
#[derive(Clone, Debug)]
pub enum SkyDistribution {
    Isotropic,
    Anisotropic(Vec<Harmonic>),
    Dipole([f64; 3]),
}
#[derive(Debug)]
pub struct GwSession {
    pub rng: ToolkitRng,
    pub ngrid: usize,
    pub warnings: Vec<String>,
}
impl Default for GwSession {
    fn default() -> Self {
        Self {
            rng: ToolkitRng::default(),
            ngrid: 1000,
            warnings: vec![],
        }
    }
}
struct SkyGrid {
    n: usize,
    dcth: f64,
    dphi: f64,
    probs: Vec<f64>,
}
impl GwSession {
    fn grid(&mut self, count: usize, harms: &[Harmonic]) -> Result<SkyGrid, TimingError> {
        let mut n = self.ngrid;
        if n == 0 {
            return Err(boundary("gwsim_Ngrid zero"));
        }
        while n
            .checked_mul(n)
            .filter(|v| *v <= i32::MAX as usize)
            .ok_or_else(|| boundary("GW grid source int overflow"))?
            < count
        {
            n = n
                .checked_mul(2)
                .ok_or_else(|| boundary("GW grid overflow"))?;
        }
        let (dcth, dphi) = (2.0 / n as f64, 2.0 * PI / n as f64);
        let mut probs = Vec::with_capacity(n * n);
        let (mut norm, mut min, mut isocomp) = (0.0, 0.0, 0.0);
        let mut cth = -1.0 + 0.5 * dcth;
        for h in harms {
            if h.l == 0.0 && h.m == 0.0 {
                isocomp = h.amplitude;
            }
        }
        for i in 0..n {
            let mut phi = 0.5 * dphi;
            let plm: Vec<f64> = harms
                .iter()
                .map(|h| {
                    let l = crate::binary::source_i32(h.l)?;
                    let m = crate::binary::source_i32(h.m)?
                        .checked_abs()
                        .ok_or_else(|| boundary("harmonic abs overflow"))?;
                    sphharm(l, m, cth)
                })
                .collect::<Result<_, _>>()?;
            for j in 0..n {
                let mut p = 0.0;
                for (h, basis) in harms.iter().zip(&plm) {
                    p += h.amplitude
                        * basis
                        * if crate::binary::source_i32(h.m)? >= 0 {
                            (h.m * phi).cos()
                        } else {
                            (-h.m * phi).sin()
                        };
                }
                if i + j == 0 || p < min {
                    min = p;
                }
                norm += p;
                probs.push(p);
                phi += dphi;
            }
            cth += dcth;
        }
        if min < 0.0 {
            self.warnings.push(format!("Warning: distribution computed from harmonic file has negative probabilities at certain sky positions. Increasing isotropic component amplitude to {:.4} to renormalise this to a non-negative pdf.",isocomp-min));
            norm += (n * n) as f64 * (-min);
        } else {
            min = 0.0;
        }
        let mut sum = 0.0;
        for p in &mut probs {
            sum += *p - min;
            *p = sum / norm;
        }
        Ok(SkyGrid {
            n,
            dcth,
            dphi,
            probs,
        })
    }
    fn direction(
        &mut self,
        seed: &mut i64,
        sky: &SkyDistribution,
        grid: Option<&SkyGrid>,
    ) -> Result<(f64, f64), TimingError> {
        match sky {
            SkyDistribution::Isotropic => Ok((
                ((self.rng.ran_dev(seed)? - 0.5) * 2.0).acos(),
                self.rng.ran_dev(seed)? * 2.0 * PI,
            )),
            SkyDistribution::Anisotropic(_) => {
                let g = grid.expect("anisotropic grid prepared");
                let prob = self.rng.ran_dev(seed)?;
                let (mut low, mut high) = (-1i64, (g.n * g.n - 1) as i64);
                while high - low > 1 {
                    let mid = (high + low) / 2;
                    if g.probs[mid as usize] > prob {
                        high = mid;
                    } else {
                        low = mid;
                    }
                }
                let high = high as usize;
                Ok((
                    (-1.0
                        + (high / g.n) as f64 * 2.0 / g.n as f64
                        + g.dcth * self.rng.ran_dev(seed)?)
                    .acos(),
                    ((high % g.n) as f64 + self.rng.ran_dev(seed)?) * g.dphi,
                ))
            }
            SkyDistribution::Dipole(d) => {
                let twopi = 2.0 * PI;
                let amp = (d[1] * d[1] + d[2] * d[2]).sqrt();
                let mut phase = if d[1] != 0.0 {
                    (-d[2] / d[1]).atan()
                } else {
                    PI / 2.0
                };
                if d[1] < 0.0 {
                    phase += PI;
                }
                let prob = self.rng.ran_dev(seed)?;
                let cth = (-0.5 + (0.25 - 2.0 * twopi * d[0] * (0.5 - prob - PI * d[0])).sqrt())
                    / (twopi * d[0]);
                Ok((
                    cth.acos(),
                    find_phi(
                        self.rng.ran_dev(seed)?,
                        amp * (1.0 - cth * cth).sqrt() / (0.5 + twopi * cth * d[0]),
                        phase,
                    ),
                ))
            }
        }
    }
    fn population(
        &mut self,
        count: usize,
        seed: &mut i64,
        o: &PopulationOptions,
        amp: f64,
        alpha: f64,
        sky: &SkyDistribution,
        polarizations: usize,
    ) -> Result<Vec<[Float; 9]>, TimingError> {
        if count > i32::MAX as usize {
            return Err(boundary("GW source count int range"));
        }
        let grid = if let SkyDistribution::Anisotropic(h) = sky {
            Some(self.grid(count, h)?)
        } else {
            None
        };
        if let SkyDistribution::Dipole(d) = sky {
            self.warnings
                .push(format!("DIPOLE AMPS: {} {} {}", d[0], d[1], d[2]));
        }
        let mut out = Vec::with_capacity(count);
        for _ in 0..count {
            let (theta, phi) = self.direction(seed, sky, grid.as_ref())?;
            let phase = if matches!(sky, SkyDistribution::Dipole(_)) {
                self.rng.ran_dev(seed)? * (2.0 * PI)
            } else {
                self.rng.ran_dev(seed)? * 2.0 * PI
            };
            let u = self.rng.ran_dev(seed)?;
            let omega = if o.loglin == 1 {
                (o.flo.clone().ln() + ld(u) * (o.fhi.clone() / &o.flo).ln()).exp() * (2.0 * PI)
            } else {
                (ld(u) * (o.fhi.clone() - &o.flo) + &o.flo) * (2.0 * PI)
            };
            let factor = if o.loglin == 1 {
                (omega.clone() / (2.0 * PI)).pow(alpha) * amp
                    / (ld(count as f64) / (o.fhi.clone() / &o.flo).ln()).sqrt()
            } else {
                (omega.clone() / (2.0 * PI)).pow(alpha)
                    * amp
                    * (1.0 / count as f64).sqrt()
                    * ((o.fhi.clone() - &o.flo) * (2.0 * PI) / &omega).sqrt()
            };
            let a = factor.clone() * self.rng.gauss_dev(seed)?;
            let b = if polarizations == 2 {
                factor * self.rng.gauss_dev(seed)?
            } else {
                ld(0.0)
            };
            out.push([
                ld(theta),
                ld(phi),
                omega,
                ld(0.0),
                ld(phase),
                a,
                ld(0.0),
                b,
                ld(0.0),
            ]);
        }
        Ok(out)
    }
    pub fn background(
        &mut self,
        count: usize,
        seed: &mut i64,
        o: &PopulationOptions,
        amp: f64,
        alpha: f64,
    ) -> Result<Vec<TensorSource>, TimingError> {
        self.tensor_population(count, seed, o, amp, alpha, &SkyDistribution::Isotropic)
    }
    pub fn anisotropic_background(
        &mut self,
        count: usize,
        seed: &mut i64,
        o: &PopulationOptions,
        amp: f64,
        alpha: f64,
        harmonics: &[Harmonic],
    ) -> Result<Vec<TensorSource>, TimingError> {
        self.tensor_population(
            count,
            seed,
            o,
            amp,
            alpha,
            &SkyDistribution::Anisotropic(harmonics.to_vec()),
        )
    }
    pub fn dipole_background(
        &mut self,
        count: usize,
        seed: &mut i64,
        o: &PopulationOptions,
        amp: f64,
        alpha: f64,
        dipole: [f64; 3],
    ) -> Result<Vec<TensorSource>, TimingError> {
        self.tensor_population(count, seed, o, amp, alpha, &SkyDistribution::Dipole(dipole))
    }
    fn tensor_population(
        &mut self,
        count: usize,
        seed: &mut i64,
        o: &PopulationOptions,
        amp: f64,
        alpha: f64,
        sky: &SkyDistribution,
    ) -> Result<Vec<TensorSource>, TimingError> {
        Ok(self
            .population(count, seed, o, amp, alpha, sky, 2)?
            .into_iter()
            .map(TensorSource::from_prefix)
            .collect())
    }
    pub fn general_background(
        &mut self,
        counts: [usize; 4],
        seed: &mut i64,
        o: &PopulationOptions,
        s: GeneralSpectrum,
    ) -> Result<Vec<GeneralSource>, TimingError> {
        self.general_population(counts, seed, o, s, None)
    }
    pub fn general_anisotropic_background(
        &mut self,
        counts: [usize; 4],
        seed: &mut i64,
        o: &PopulationOptions,
        s: GeneralSpectrum,
        harmonics: &[Vec<Harmonic>; 4],
    ) -> Result<Vec<GeneralSource>, TimingError> {
        self.general_population(counts, seed, o, s, Some(harmonics))
    }
    fn general_population(
        &mut self,
        counts: [usize; 4],
        seed: &mut i64,
        o: &PopulationOptions,
        s: GeneralSpectrum,
        h: Option<&[Vec<Harmonic>; 4]>,
    ) -> Result<Vec<GeneralSource>, TimingError> {
        let mut out = vec![];
        let amps = [s.tensor_amp, s.st_amp, s.sl_amp, s.vl_amp];
        let alphas = [s.tensor_alpha, s.st_alpha, s.sl_alpha, s.vl_alpha];
        for j in 0..4 {
            if counts[j] == 0 {
                continue;
            }
            let sky = if let Some(h) = h {
                SkyDistribution::Anisotropic(h[j].clone())
            } else {
                SkyDistribution::Isotropic
            };
            for v in self.population(
                counts[j],
                seed,
                o,
                amps[j],
                alphas[j],
                &sky,
                if j == 0 || j == 3 { 2 } else { 1 },
            )? {
                let mut prefix = std::array::from_fn(|_| ld(0.0));
                prefix[..5].clone_from_slice(&v[..5]);
                // 0053: GWsim.C:1008/1206 uses across_g==0 in the linear vector
                // chain, retaining old cross and assigning its comparison to three
                // inactive fields. Explicitly write zero to every inactive amplitude.
                match j {
                    0 => {
                        prefix[5] = v[5].clone();
                        prefix[7] = v[7].clone();
                    }
                    1 => prefix[9] = v[5].clone(),
                    2 => prefix[11] = v[5].clone(),
                    _ => {
                        prefix[13] = v[5].clone();
                        prefix[15] = v[7].clone();
                    }
                }
                out.push(GeneralSource::from_prefix(prefix));
            }
        }
        Ok(out)
    }
}

/// Source caller's retained eccentric history, shared with the canonical AD body.
#[derive(Clone, Debug, serde::Serialize)]
pub struct EccentricState {
    pub coalesce_flag: i32,
    pub prev_p: f64,
    pub prev_e: f64,
    pub prev_a: f64,
    pub prev_epoch: f64,
    pub prev_theta: f64,
}
fn eccentric(
    context: &mut crate::pulsar_lifecycle::PulsarContext,
    p: usize,
    i: usize,
    state: &mut EccentricState,
    energy: Option<&mut f32>,
) -> Result<Float, TimingError> {
    let (psr, rows) = context.pulsar_observations_mut(p)?;
    if state.coalesce_flag != 1 && i >= *crate::observation::read(&psr.nobs, "nobs")? as usize {
        return Err(boundary("eccRes observation index"));
    }
    let (mut flag, mut period, mut e, mut a, mut epoch, mut theta) = (
        Some(state.coalesce_flag),
        Some(state.prev_p),
        Some(state.prev_e),
        Some(state.prev_a),
        Some(state.prev_epoch),
        Some(state.prev_theta),
    );
    let result = crate::clkcorr::with_process(|s| {
        crate::gw_ecc::ecc_res_inner(
            psr,
            rows,
            i as i32,
            &mut flag,
            &mut period,
            &mut e,
            &mut a,
            &mut epoch,
            &mut theta,
            &mut s.diagnostics,
            energy,
        )
    });
    state.coalesce_flag = flag.unwrap();
    state.prev_p = period.unwrap();
    state.prev_e = e.unwrap();
    state.prev_a = a.unwrap();
    state.prev_epoch = epoch.unwrap();
    state.prev_theta = theta.unwrap();
    result
}
/// Source eccRes return is nanoseconds (the explicit 1e9 factor).
pub fn eccentric_residual(
    context: &mut crate::pulsar_lifecycle::PulsarContext,
    p: usize,
    i: usize,
    state: &mut EccentricState,
) -> Result<Float, TimingError> {
    eccentric(context, p, i, state, None)
}
/// Residual nanoseconds and source eOut in W/m^2, with source f32 rounding.
pub fn eccentric_residual_with_energy(
    context: &mut crate::pulsar_lifecycle::PulsarContext,
    p: usize,
    i: usize,
    state: &mut EccentricState,
) -> Result<(Float, f32), TimingError> {
    let mut energy = 0.0;
    let residual = eccentric(context, p, i, state, Some(&mut energy))?;
    Ok((residual, energy))
}
pub fn rs(m1: f64) -> Result<f64, TimingError> {
    crate::clkcorr::with_process(|s| crate::gw_ecc::Rs(m1, &mut s.diagnostics))
}
pub fn dadt(ec: f64, a: f64, m1: f64, m2: f64) -> Result<f64, TimingError> {
    crate::clkcorr::with_process(|s| crate::gw_ecc::dadt(ec, a, m1, m2, &mut s.diagnostics))
}
pub fn dedt(ec: f64, a: f64, m1: f64, m2: f64) -> Result<f64, TimingError> {
    crate::clkcorr::with_process(|s| crate::gw_ecc::dedt(ec, a, m1, m2, &mut s.diagnostics))
}
pub fn dtdt(ec: f64, t: f64, p: f64) -> Result<f64, TimingError> {
    crate::clkcorr::with_process(|s| crate::gw_ecc::dtdt(ec, t, p, &mut s.diagnostics))
}
pub fn fe(ec: f64) -> Result<f64, TimingError> {
    crate::clkcorr::with_process(|s| crate::gw_ecc::Fe(ec, &mut s.diagnostics))
}
