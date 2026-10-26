//! TKspectrum.C:1041–1642, source sigma-z algorithm and one-based state.
// Copyright (C) 2006-2009 George Hobbs, Russell Edwards. GPL-3.0-or-later.
#![allow(clippy::needless_range_loop)]
use crate::{
    TimingError, clock_diagnostics::boundary, observation::read, pulsar_lifecycle::PulsarContext,
    tksvd::ld,
};

#[derive(Debug, Default)]
pub struct SigmaZ {
    pub data: Vec<f64>,
    pub utjd: Vec<f64>,
    pub sigmai: Vec<f64>,
    pub indx: Vec<usize>,
    pub nusewt: i32,
    pub ndim: usize,
    pub ntunits: i32,
    pub nbintype: i32,
    pub npt1last: usize,
    pub npt2last: usize,
    pub ncubic: usize,
    pub ncubics: usize,
    pub ntau: usize,
    pub utjd1: f64,
    pub utjd2: f64,
    pub utmean: f64,
    pub tausec: f64,
    pub tauensure: f64,
    pub taumin: f64,
    pub taumax: f64,
    pub tdiffmin: f64,
    pub addvar: f64,
    pub prtl: [f64; 5],
    pub warnings: Vec<String>,
    ready: bool,
    npass: usize,
    ntaulast: Option<usize>,
    scalefact: f64,
    scalecon: f64,
}
#[derive(Debug, serde::Serialize)]
pub struct SigmaPoint {
    pub log10_tau_year: f64,
    pub log10_sigma_bias: f64,
    pub error_plus: f64,
    pub error_minus: f64,
    pub fits: usize,
    pub independent_bins: usize,
}
#[derive(Debug, serde::Serialize)]
pub struct CubicFit {
    pub p4: f64,
    pub cov4: f64,
    pub chi_df: Option<f64>,
    pub average_weight: Option<f64>,
}
impl SigmaZ {
    /// readin uses every active row, including source-deleted observations.
    pub fn readin(&mut self, context: &mut PulsarContext, p: usize) -> Result<(), TimingError> {
        self.ready = false;
        self.warnings.clear();
        let (psr, rows) = context.pulsar_observations_mut(p)?;
        let n = *read(&psr.nobs, "nobs")? as usize;
        if n < 2 {
            return Err(boundary(
                "sigma-z readin needs two rows for indexx8/tdiffmin",
            ));
        }
        let mut mean = 0.0;
        for o in &rows[..n] {
            mean = (ld(mean) + read(&o.residual, "residual")?).to_f64();
        }
        mean /= n as f64;
        self.ntunits = 0;
        self.tauensure = -86400.0;
        // 0053: TKspectrum.C global arrays have 90000 cells but use indices
        // 1..=npt. Own n+1 cells, correcting the npt==90000 overrun only.
        self.data = vec![0.0; n + 1];
        self.utjd = vec![0.0; n + 1];
        self.sigmai = vec![0.0; n + 1];
        let t1 = read(&rows[0].sat, "SAT")?.to_f64();
        let (mut ave, mut var, mut sumwt) = (0.0, 0.0, 0.0);
        for (i, o) in rows[..n].iter().enumerate() {
            let sat = read(&o.sat, "SAT")?;
            if i > 0 && read(&rows[i - 1].sat, "SAT")? > sat {
                self.warnings.push(format!(
                    "{}: ERROR: The TOAs need to be time sorted, sorry!",
                    if self.nusewt == 0 || self.nusewt == -3 {
                        1
                    } else {
                        2
                    }
                ));
            }
            let t = (sat.to_f64() - t1) * 86400.0;
            let x = 1e-6 * ((read(&o.residual, "residual")?.clone() - mean).to_f64() / 1e-6);
            let sig = if self.nusewt == 0 || self.nusewt == -3 {
                1.0
            } else {
                1e-6 * *read(&o.toaErr, "toaErr")?
            };
            let j = i + 1;
            self.utjd[j] = t;
            self.data[j] = x;
            self.sigmai[j] = sig;
            let wt = 1.0 / sig / sig;
            ave += wt * x;
            var += wt * x * x;
            sumwt += wt;
        }
        self.indx = indexx8(&self.utjd)?;
        self.tdiffmin = self.utjd[self.indx[2]] - self.utjd[self.indx[1]];
        for j in 1..n {
            let dt = self.utjd[self.indx[j + 1]] - self.utjd[self.indx[j]];
            if self.tdiffmin > dt {
                self.tdiffmin = dt;
            }
        }
        ave /= sumwt;
        var /= sumwt;
        let rms = (var - ave * ave).sqrt();
        self.taumax = self.utjd[self.indx[n]] - self.utjd[self.indx[1]];
        self.taumin = self.taumax / (n as f64 - 1.0);
        let mut test = self.taumax;
        loop {
            test /= 2.0;
            if test <= self.taumin || test.is_nan() {
                break;
            }
        }
        self.taumin = test;
        if self.nusewt >= 0 {
            self.addvar = rms;
        }
        self.ready = true;
        Ok(())
    }
    pub fn getprtj(&mut self, n: usize) -> Result<[f64; 5], TimingError> {
        if !self.ready || n == 0 || n >= self.indx.len() {
            return Err(boundary("sigma-z getprtj state/index"));
        }
        let x = (self.utjd[self.indx[n]] - self.utmean) / 86400.0;
        self.prtl = [0.0, 1.0, x, x * x, x * x * x];
        Ok(self.prtl)
    }
    pub fn getweights(&self, n: usize) -> Result<f64, TimingError> {
        if !self.ready || n == 0 || n >= self.indx.len() {
            return Err(boundary("sigma-z getweights state/index"));
        }
        Ok(match self.nusewt {
            0 | -3 => 1.0 / self.addvar / self.addvar,
            -1 => 1.0 / self.sigmai[self.indx[n]].powf(2.0),
            v if v == -2 || v > 0 => {
                1.0 / (self.sigmai[self.indx[n]].powf(2.0) + self.addvar.powf(2.0))
            }
            _ => return Err(boundary("sigma-z source unwritten weight selector")),
        })
    }
    /// fit4 returns None at the source's nfit=0 early return, without invented outputs.
    pub fn fit4(&mut self, _ndostats: i32) -> Result<Option<CubicFit>, TimingError> {
        if !self.ready || !(1..=4).contains(&self.ndim) {
            return Err(boundary("sigma-z fit4 state/dimension"));
        }
        self.npass += 1;
        self.ncubic += 1;
        self.ncubics += 1;
        if self.scalefact == 0.0 {
            self.scalefact = 1.0;
        }
        if self.scalecon == 0.0 {
            self.scalecon = 1.0;
        }
        if self.ndim > 1 {
            self.scalefact = 86400.0f64.powf((self.ndim - 1) as f64);
        }
        if self.ndim != 4 {
            self.scalecon = if self.ntunits >= -2 {
                3.0
            } else {
                match self.ndim {
                    3 => 3.0 * self.tdiffmin,
                    2 => 6.0 * self.tdiffmin * self.tdiffmin,
                    _ => 6.0 * self.tdiffmin * self.tdiffmin * self.tdiffmin,
                }
            };
        }
        let (mut npt1, mut npt2) = (0, 0);
        for n in 1..self.indx.len() {
            if self.utjd[self.indx[n]] >= self.utjd1 - 1e-5 {
                if self.utjd[self.indx[n]] + (4 - self.ndim) as f64 * self.tdiffmin
                    > self.utjd2 + 1e-5
                {
                    break;
                }
                if npt2 == 0 {
                    npt1 = n;
                }
                npt2 = n;
            }
        }
        if npt1 == 0
            || npt2 == 0
            || npt2 - npt1 + 1 < self.ndim
            || (npt1 == self.npt1last && npt2 == self.npt2last)
            || (self.utjd[self.indx[npt2]] + (4.0 - self.ndim as f64) * self.tdiffmin
                - self.utjd[self.indx[npt1]]
                < self.tausec / 2.0f64.sqrt())
        {
            return Ok(None);
        }
        self.npt1last = npt1;
        self.npt2last = npt2;
        let mut am = [[0.0; 21]; 21];
        let mut fvec = [0.0; 5];
        let mut par = [0.0; 5];
        for n in npt1..=npt2 {
            let wt = self.getweights(n)?;
            self.getprtj(n)?;
            let y = self.data[self.indx[n]];
            for i in 1..=self.ndim {
                fvec[i] += y * self.prtl[i] * wt;
                for j in 1..=self.ndim {
                    am[i][j] += self.prtl[i] * self.prtl[j] * wt;
                }
            }
        }
        let (inv, _, bad) = mat20(&am, self.ndim)?;
        if bad != 0 {
            return Ok(None);
        }
        for i in 1..=self.ndim {
            for j in 1..=self.ndim {
                par[i] += inv[i][j] * fvec[j];
            }
        }
        let mut p4 = par[self.ndim] / self.scalefact;
        let mut cov4 = inv[self.ndim][self.ndim] / self.scalefact / self.scalefact;
        if self.ndim != 4 {
            p4 /= self.scalecon;
            cov4 = cov4 / self.scalecon / self.scalecon;
        }
        self.ntaulast = Some(self.ntau);
        let (mut sumwt, mut chisqu, mut nok) = (0.0, 0.0, 0);
        for n in npt1..=npt2 {
            let wt = self.getweights(n)?;
            self.getprtj(n)?;
            let mut yf = 0.0;
            for i in 1..=self.ndim {
                yf += self.prtl[i] * par[i];
            }
            let res = self.data[self.indx[n]] - yf;
            chisqu += wt * res * res;
            sumwt += wt;
            nok += 1;
        }
        let wrote = nok > 1 && sumwt > 1e-50;
        Ok(Some(CubicFit {
            p4,
            cov4,
            chi_df: wrote.then(|| {
                if nok > 4 {
                    chisqu / (nok as f64 - self.ndim as f64)
                } else {
                    1.0
                }
            }),
            average_weight: wrote.then(|| sumwt / nok as f64),
        }))
    }
    pub fn calculate(
        &mut self,
        context: &mut PulsarContext,
        p: usize,
        weights: i32,
        mintau: f64,
    ) -> Result<Vec<SigmaPoint>, TimingError> {
        self.ready = false;
        let result = (|| {
            if weights < -3 || !mintau.is_finite() || mintau < 0.0 {
                return Err(boundary(
                    "sigma-z undefined weight or nonterminating mintau",
                ));
            }
            self.nusewt = weights;
            self.npt1last = 0;
            self.npt2last = 0;
            self.utjd1 = 0.0;
            self.ndim = 4;
            self.readin(context, p)?;
            self.tausec = self.taumax;
            self.ntau = 0;
            self.ncubics = 0;
            let npt = self.indx.len() - 1;
            if weights >= 0 {
                self.utjd1 = self.utjd[self.indx[1]] - 1e-2;
                self.utjd2 = self.utjd[self.indx[npt]] + 1e-2;
                self.utmean = (self.utjd1 + self.utjd2) / 2.0;
                // fit4's later duplicate-window returns leave caller outputs intact.
                let mut last = None;
                for _ in 0..=weights {
                    if let Some(f) = self.fit4(0)? {
                        last = Some(f);
                    }
                    let f = last
                        .as_ref()
                        .ok_or_else(|| boundary("sigma-z initial fit left outputs unwritten"))?;
                    let addvsq = self.addvar * self.addvar
                        + (f.chi_df
                            .ok_or_else(|| boundary("sigma-z unwritten chi_df"))?
                            - 1.0)
                            / f.average_weight
                                .ok_or_else(|| boundary("sigma-z unwritten average weight"))?;
                    if addvsq > 0.0 {
                        self.addvar = addvsq.sqrt();
                    } else if weights != 0 {
                        self.addvar = 0.0;
                    }
                }
            }
            let mut result = vec![];
            loop {
                self.ntau += 1;
                self.ncubic = 0;
                let (mut sumwt, mut sz, mut uncor_end, mut sumwtuncor) = (0.0, 0.0, 0.0, 0.0);
                let (mut ntot, mut nuncor) = (0, 0);
                self.npt1last = 0;
                self.npt2last = 0;
                let step = match self.nbintype {
                    0 | 2 => self.tausec,
                    1 => self.taumin,
                    _ => return Err(boundary("sigma-z undefined nbintype")),
                };
                if step <= 0.0 || !step.is_finite() {
                    return Err(boundary("sigma-z nonadvancing bin step"));
                }
                self.utjd1 = self.utjd[self.indx[1]] - step;
                loop {
                    let old = self.utjd1;
                    self.utjd1 += step;
                    if self.utjd1 == old {
                        return Err(boundary("sigma-z bin iteration cannot advance"));
                    }
                    if self.utjd1 >= self.utjd[self.indx[npt]] {
                        break;
                    }
                    self.utjd2 = self.utjd1 + self.tausec;
                    self.utmean = (self.utjd1 + self.utjd2) / 2.0;
                    let independent = self.utjd1 >= uncor_end;
                    if independent {
                        uncor_end = self.utjd2;
                    }
                    self.ncubics += 1;
                    if let Some(f) = self.fit4(0)? {
                        let wt = 1.0 / f.cov4;
                        sz += wt * f.p4 * f.p4;
                        sumwt += wt;
                        ntot += 1;
                        if independent {
                            sumwtuncor += wt;
                            nuncor += 1;
                        }
                    }
                }
                if ntot > 0 && !(sz <= 0.0 || sumwt <= 0.0) {
                    let rootsz =
                        self.tausec * self.tausec * (sz / sumwt).sqrt() / (2.0 * 5.0f64.sqrt());
                    let (mut bias, mut dp, mut dm) = (0.0, 0.0, 0.0);
                    if sumwtuncor > 1e-50 {
                        let x = nuncor as f64;
                        bias = 0.17 / x;
                        dm = 0.31 / x.sqrt();
                        dp = if nuncor > 1 {
                            0.31 / (x - 1.0).sqrt()
                        } else {
                            0.52
                        };
                    }
                    result.push(SigmaPoint {
                        log10_tau_year: (self.tausec / 86400.0 / 365.25).log10(),
                        log10_sigma_bias: rootsz.log10() - bias,
                        error_plus: dp,
                        error_minus: -dm,
                        fits: ntot,
                        independent_bins: nuncor,
                    });
                }
                let old = self.tausec;
                self.tausec = match self.nbintype {
                    0 => self.tausec / 2.0,
                    1 => self.tausec - self.taumin,
                    _ => self.tausec + self.taumin,
                };
                if !(ntot > 0
                    || (self.tausec >= self.tauensure && self.tauensure >= 0.0)
                    || self.tausec / 86400.0 / 365.25 > mintau)
                {
                    break;
                }
                if self.tausec == old {
                    return Err(boundary("sigma-z tau iteration cannot advance"));
                }
            }
            Ok(result)
        })();
        if result.is_err() {
            self.ready = false;
        }
        result
    }
}
/// indexx8 source heap sort, retaining tie order and one-based indices.
pub fn indexx8(arr: &[f64]) -> Result<Vec<usize>, TimingError> {
    let n = arr.len().saturating_sub(1);
    if n < 2 {
        return Err(boundary("indexx8 source invalid count below two"));
    }
    let mut indx: Vec<_> = (0..=n).collect();
    let (mut l, mut ir) = (n / 2 + 1, n);
    loop {
        let indxt;
        if l > 1 {
            l -= 1;
            indxt = indx[l];
        } else {
            indxt = indx[ir];
            indx[ir] = indx[1];
            ir -= 1;
            if ir == 1 {
                indx[1] = indxt;
                return Ok(indx);
            }
        }
        let q = arr[indxt];
        let (mut i, mut j) = (l, l + l);
        while j <= ir {
            if j < ir && arr[indx[j]] < arr[indx[j + 1]] {
                j += 1;
            }
            if q < arr[indx[j]] {
                indx[i] = indx[j];
                i = j;
                j += j;
            } else {
                break;
            }
        }
        indx[i] = indxt;
    }
}
/// mat20 full pivot elimination; determinant is log10(abs(product of pivots)).
#[allow(clippy::type_complexity)]
pub fn mat20(sam: &[[f64; 21]; 21], n: usize) -> Result<([[f64; 21]; 21], f64, i32), TimingError> {
    if n > 20 {
        return Err(boundary("mat20 one-based dimension"));
    }
    let mut a = *sam;
    let mut index = [[0usize; 2]; 21];
    let mut ipivot = [0; 21];
    let mut determ = 0.0;
    for i in 1..=n {
        let (mut amax, mut selected) = (0.0, None);
        for j in 1..=n {
            if ipivot[j] != 1 {
                for k in 1..=n {
                    if ipivot[k] > 1 {
                        return Ok((a, determ, 0));
                    }
                    if ipivot[k] < 1 && f64::abs(amax) - a[j][k].abs() < 0.0 {
                        selected = Some((j, k));
                        amax = a[j][k];
                    }
                }
            }
        }
        // 0053: TKspectrum.C:1379 consumes uninitialized irow/icolum if no
        // nonzero pivot exists. Classify singularity using its existing nbad=1.
        let Some((irow, icolum)) = selected else {
            return Ok((a, determ, 1));
        };
        ipivot[icolum] += 1;
        if irow != icolum {
            for l in 1..=n {
                let swap = a[irow][l];
                a[irow][l] = a[icolum][l];
                a[icolum][l] = swap;
            }
        }
        index[i] = [irow, icolum];
        let pivot = a[icolum][icolum];
        let x = pivot.abs();
        if x <= 1e-70 || x >= 1e70 {
            return Ok((a, determ, 1));
        }
        determ += x.log10();
        a[icolum][icolum] = 1.0;
        for l in 1..=n {
            a[icolum][l] /= pivot;
        }
        for l1 in 1..=n {
            if l1 != icolum {
                let t = a[l1][icolum];
                a[l1][icolum] = 0.0;
                for l in 1..=n {
                    a[l1][l] -= a[icolum][l] * t;
                }
            }
        }
    }
    for l in (1..=n).rev() {
        if index[l][0] != index[l][1] {
            for row in a.iter_mut().take(n + 1).skip(1) {
                row.swap(index[l][0], index[l][1]);
            }
        }
    }
    Ok((a, determ, 0))
}
