//! bootstrap.C: source resampling, partial observation copies and seeded ran1.
use crate::{
    TimingError, clock_diagnostics::boundary, observation::read, pulsar::SourcePulsar,
    pulsar_lifecycle::PulsarContext, tempo2::Label::*, tkfit::FitNumber, tksvd::ld,
};
use rug::Float;
use serde::Serialize;
use std::path::Path;

/// Owned counterpart of random(long*)'s static state. A negative seed resets it.
pub struct BootstrapRng {
    table: Vec<Float>,
    state: [i64; 3],
    initialized: bool,
}
impl Default for BootstrapRng {
    fn default() -> Self {
        Self {
            table: vec![ld(0.0); 97],
            state: [0; 3],
            initialized: false,
        }
    }
}
impl BootstrapRng {
    pub fn random(&mut self, seed: &mut i64) -> Result<f64, TimingError> {
        let rm1 = ld(1.0 / 259100.0);
        let rm2 = ld(1.0 / 134456.0);
        if *seed < 0 || !self.initialized {
            self.initialized = true;
            self.state[0] = 54773_i64
                .checked_sub(*seed)
                .ok_or_else(|| boundary("bootstrap RNG seed overflow"))?
                % 259100;
            self.state[0] = (7141 * self.state[0] + 54773) % 259100;
            self.state[1] = self.state[0] % 134456;
            self.state[0] = (7141 * self.state[0] + 54773) % 259100;
            self.state[2] = self.state[0] % 243000;
            for value in &mut self.table {
                self.state[0] = (7141 * self.state[0] + 54773) % 259100;
                self.state[1] = (8121 * self.state[1] + 28411) % 134456;
                *value = (ld(self.state[0] as f64) + ld(self.state[1] as f64) * &rm2) * &rm1;
            }
            *seed = 1;
        }
        self.state[0] = (7141 * self.state[0] + 54773) % 259100;
        self.state[1] = (8121 * self.state[1] + 28411) % 134456;
        self.state[2] = (4561 * self.state[2] + 51349) % 243000;
        let j = 97 * self.state[2] / 243000;
        if !(0..97).contains(&j) {
            return Err(boundary("bootstrap RNG table index"));
        }
        let result = self.table[j as usize].to_f64();
        self.table[j as usize] = (ld(self.state[0] as f64) + ld(self.state[1] as f64) * rm2) * rm1;
        Ok(result)
    }
}

#[derive(Debug, Serialize)]
pub struct BootstrapIteration {
    pub sampled_indices: Vec<usize>,
    pub fitted_values: Vec<FitNumber>,
}
#[derive(Debug, Serialize)]
pub struct BootstrapResult {
    pub columns: Vec<(usize, usize)>,
    pub iterations: Vec<BootstrapIteration>,
    pub values: Vec<FitNumber>,
    pub errors: Vec<FitNumber>,
    pub seed: i64,
    pub output_file: Option<std::path::PathBuf>,
}

/// Bootstrap on already fitted canonical state. The source scratch slot npsr is
/// owned for this call; only the active pulsars remain in the caller's context.
/// `testout.tim` is written before every fit, using source format "fred".
pub fn bootstrap(
    context: &mut PulsarContext,
    p: usize,
    runtime: &Path,
    output_directory: &Path,
) -> Result<BootstrapResult, TimingError> {
    let source = context
        .pulsars
        .get(p)
        .ok_or_else(|| boundary("bootstrap pulsar index"))?;
    let exponent = *read(&source.bootStrap, "bootStrap")?;
    if exponent > 12 {
        return Err(boundary("bootstrap MAX_ITER=4096; exponent exceeds 12"));
    }
    let nobs = *read(&source.nobs, "nobs")? as usize;
    let scratch = context.pulsars.len();
    context.pulsars.push(SourcePulsar::default());
    context
        .initialise_one(scratch, nobs as i32, 1, 1)
        .map_err(|e| boundary(&e.to_string()))?;
    let result = run(context, p, scratch, exponent, runtime, output_directory);
    // Safe ownership replaces the caller's spare native array slot. This does
    // not restore the source's deliberately retained observation mutations.
    let cleanup = context
        .destroy_one(scratch)
        .map_err(|e| boundary(&e.to_string()));
    context.pulsars.pop();
    match result {
        Ok(value) => {
            cleanup?;
            Ok(value)
        }
        Err(e) => Err(e),
    }
}

fn run(
    context: &mut PulsarContext,
    p: usize,
    scratch: usize,
    exponent: i32,
    runtime: &Path,
    output_directory: &Path,
) -> Result<BootstrapResult, TimingError> {
    context
        .copy_psr(p, scratch)
        .map_err(|e| boundary(&e.to_string()))?;
    // bootstrap.C copies parameter storage from pulsar zero even for p != 0.
    let (active, copied) = context.pulsars.split_at_mut(scratch);
    for (source, dest) in active[0].param.slots.iter().zip(&mut copied[0].param.slots) {
        crate::copy_param(source, dest).map_err(|e| boundary(&e.to_string()))?;
    }
    let original_rows = {
        let (psr, obs) = context.pulsar_observations_mut(scratch)?;
        obs[..*read(&psr.nobs, "nobs")? as usize].to_vec()
    };
    let (psr, rows) = context.pulsar_observations_mut(p)?;
    let nobs = *read(&psr.nobs, "nobs")? as usize;
    for o in &mut rows[..nobs] {
        o.residual = Some(read(&o.prefitResidual, "prefitResidual")?.clone());
    }
    let mut columns = Vec::new();
    let mut initial = Vec::new();
    for (l, parameter) in psr.param.slots.iter_mut().enumerate() {
        for (k, c) in parameter
            .cells
            .iter_mut()
            .take(*read(&parameter.a_size, "parameter size")? as usize)
            .enumerate()
        {
            if c.fit_flag == 1 {
                columns.push((l, k));
                initial.push(c.val.clone());
                c.val = read(&c.prefit, "parameter prefit")?.clone();
            }
        }
    }
    if columns.len() > crate::tempo2::MAX_PARAMS {
        return Err(boundary("bootstrap fitted-coordinate capacity MAX_PARAMS"));
    }
    let start_zero = context.pulsars[0].param.slots[param_start as usize].cells[0]
        .val
        .clone();
    let finish_zero = context.pulsars[0].param.slots[param_finish as usize].cells[0]
        .val
        .clone();
    let (psr, rows) = context.pulsar_observations_mut(p)?;
    // Source reads cut flags from p and cut values from pulsar zero. Reuse the
    // canonical selector with those values, restoring p before any fit.
    let start = std::mem::replace(
        &mut psr.param.slots[param_start as usize].cells[0].val,
        start_zero,
    );
    let finish = std::mem::replace(
        &mut psr.param.slots[param_finish as usize].cells[0].val,
        finish_zero,
    );
    let mut npts = 0;
    let selected = rows[..nobs].iter().try_for_each(|o| {
        if crate::general_fitting::fit_observation_selected(psr, o)? {
            npts += 1;
        }
        Ok::<_, TimingError>(())
    });
    psr.param.slots[param_start as usize].cells[0].val = start;
    psr.param.slots[param_finish as usize].cells[0].val = finish;
    selected?;
    let nboot = if exponent < 0 { 0 } else { 1_usize << exponent };
    let mut differences = vec![vec![]; columns.len()];
    let mut rng = BootstrapRng::default();
    let mut seed = -999;
    let output_file = output_directory.join("testout.tim");
    let mut iterations = Vec::new();
    for _ in 0..nboot {
        let mut sampled_indices = Vec::new();
        let (_, rows) = context.pulsar_observations_mut(p)?;
        for o in &mut rows[..npts] {
            let i = (npts as f64 * rng.random(&mut seed)?) as usize;
            sampled_indices.push(i);
            let s = &original_rows[i];
            // Source partial copy intentionally leaves BBAT, flags, remaining
            // vectors and all other observation fields at their destination.
            o.prefitResidual = s.prefitResidual.clone();
            o.residual = s.residual.clone();
            o.sat = s.sat.clone();
            o.bat = s.bat.clone();
            o.deleted = s.deleted;
            o.freq = s.freq;
            o.freqSSB = s.freqSSB;
            o.toaErr = s.toaErr;
            o.fname = s.fname;
            o.telID = s.telID;
            o.earth_ssb[..3].clone_from_slice(&s.earth_ssb[..3]);
            o.observatory_earth[..3].clone_from_slice(&s.observatory_earth[..3]);
        }
        // writeTim("...", psr, ...) in the source always uses pulsar zero.
        crate::source_tim::write_tim(&output_file, context, 0, b"fred", runtime)?;
        context.pulsars[p].bootStrap = Some(0);
        let options = crate::general_fitting::FitOptions {
            rescale_err_chisq: context.pulsars[p].rescaleErrChisq == Some(1),
            ..Default::default()
        };
        // t2Fit(&psr[p],1,NULL): restrict the fit's active pulsar vector while
        // retaining context-owned observation/matrix/payload handles.
        let mut all = std::mem::take(&mut context.pulsars);
        context.pulsars.push(all.remove(p));
        let fitted = crate::general_fitting::t2_fit(context, &options);
        all.insert(p, context.pulsars.pop().unwrap());
        context.pulsars = all;
        fitted?;
        let psr = &context.pulsars[p];
        let mut fitted_values = Vec::new();
        for (j, &(l, k)) in columns.iter().enumerate() {
            let value = psr.param.slots[l].cells[k].val.clone();
            differences[j].push(value.clone() - &initial[j]);
            fitted_values.push(FitNumber(value));
        }
        iterations.push(BootstrapIteration {
            sampled_indices,
            fitted_values,
        });
        let (_, rows) = context.pulsar_observations_mut(p)?;
        for (o, s) in rows[..nobs].iter_mut().zip(&original_rows) {
            o.residual = Some(read(&s.prefitResidual, "scratch prefitResidual")?.clone());
        }
        for &(l, k) in &columns {
            let prefit = read(
                &context.pulsars[scratch].param.slots[l].cells[k].prefit,
                "scratch parameter prefit",
            )?
            .clone();
            let c = &mut context.pulsars[p].param.slots[l].cells[k];
            c.prefit = Some(prefit.clone());
            c.val = prefit;
        }
    }
    let (mut values, mut errors) = (Vec::new(), Vec::new());
    for (j, &(l, k)) in columns.iter().enumerate() {
        let (mut mean, mut meansq) = (ld(0.0), ld(0.0));
        for v in &differences[j] {
            mean += v.clone() / nboot;
            meansq += v.clone().square() / nboot;
        }
        let error = (meansq - mean.clone().square()).sqrt();
        let c = &mut context.pulsars[p].param.slots[l].cells[k];
        c.val = mean + &initial[j];
        c.err = error;
        values.push(FitNumber(c.val.clone()));
        errors.push(FitNumber(c.err.clone()));
    }
    Ok(BootstrapResult {
        columns,
        iterations,
        values,
        errors,
        seed,
        output_file: (nboot > 0).then_some(output_file),
    })
}
