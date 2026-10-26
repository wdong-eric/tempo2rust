//! cheby2d_int.C: canonical timing callbacks, model/set construction and testing.
use crate::{
    PRECISION_BITS, RuntimeContext, TimingError,
    cheby2d::{Cheby2D, ChebyModel, ChebyModelSet, ErrorSummary},
    observation::{read, read_string, write_string},
    predictor::{self, error},
    predictor_number::{N, n},
    pulsar_lifecycle::PulsarContext,
    tempo2::Label::*,
};
use rug::Float;
use std::path::Path;
#[derive(Clone, Debug)]
pub struct ChebyRequest {
    pub site: String,
    pub start_mjd: N,
    pub end_mjd: N,
    pub segment_days: N,
    pub overlap_days: N,
    pub freq_start_mhz: N,
    pub freq_end_mhz: N,
    pub time_coeffs: usize,
    pub freq_coeffs: usize,
}
pub fn cheby_model_func(
    model: &mut ChebyModel,
    context: &mut PulsarContext,
    runtime: &mut RuntimeContext,
    x: &[N],
    y: &[N],
    compute_dispersion: bool,
    tim: &Path,
) -> Result<Vec<N>, TimingError> {
    let nx = x.len();
    let count = x
        .len()
        .checked_mul(y.len())
        .and_then(|n| n.checked_add(1))
        .filter(|n| *n <= i32::MAX as usize)
        .ok_or_else(|| error("Cheby grid overflow"))?;
    if x.is_empty() || y.is_empty() {
        return Err(error("empty Cheby timing grid"));
    }
    context.ensure_observation_capacity(0, count)?;
    context.pulsars[0].nobs = Some(count as i32);
    {
        let (p, rows) = context.pulsar_observations_mut(0)?;
        p.param.slots[param_track as usize].cells[0].param_set = 0;
        p.T2globalEfac = Some(1.0f64);
        for (iy, y) in y.iter().enumerate() {
            for (ix, x) in x.iter().enumerate() {
                let sat = Float::with_val(PRECISION_BITS, model.mjd_start.as_float())
                    + (Float::with_val(PRECISION_BITS, x.as_float()) + 1i32)
                        * 0.5f64
                        * Float::with_val(
                            PRECISION_BITS,
                            (&model.mjd_end - &model.mjd_start).as_float(),
                        );
                let freq = (Float::with_val(PRECISION_BITS, model.freq_start.as_float())
                    + (Float::with_val(PRECISION_BITS, y.as_float()) + 1i32)
                        * 0.5f64
                        * Float::with_val(
                            PRECISION_BITS,
                            (&model.freq_end - &model.freq_start).as_float(),
                        ))
                .to_f64();
                predictor::synthetic(
                    &mut rows[1 + iy * nx + ix],
                    sat,
                    freq,
                    model.sitename.as_bytes(),
                    b"predictor",
                )?;
            }
        }
    }
    predictor::reference(context)?;
    predictor::timing(context, runtime, tim)?;
    let rows = predictor::observations(context)?;
    if compute_dispersion {
        let i0 = x.len() / 2;
        let i1 = i0 + (y.len() - 1) * x.len();
        // Source indices deliberately omit the reference-row +1.0f64 Preserve them.
        let f0 = N::from_float(&Float::with_val(113, *read(&rows[i0].freq, "freq")?).square());
        let f1 = N::from_float(&Float::with_val(113, *read(&rows[i1].freq, "freq")?).square());
        let denominator = n(1.0f64) / f0 - n(1.0f64) / f1;
        if denominator == n(0.0f64) {
            return Err(error("Cheby dispersion samples have identical frequencies"));
        }
        model.dispersion_constant = N::from_float(
            &((read(&rows[i0].phase, "phase")?.clone() - read(&rows[i1].phase, "phase")?)
                / Float::with_val(PRECISION_BITS, denominator.as_float())),
        );
    }
    let rphase = N::from_float(read(&rows[0].phase, "phase")?);
    rows[1..]
        .iter()
        .map(|o| {
            let freq = *read(&o.freq, "freq")?;
            let delay = &model.dispersion_constant / n(freq * freq);
            Ok(N::from_float(
                &(read(&o.phase, "phase")?.clone()
                    - Float::with_val(PRECISION_BITS, rphase.as_float())
                    - Float::with_val(PRECISION_BITS, delay.as_float())),
            ))
        })
        .collect()
}
pub fn construct_model(
    model: &mut ChebyModel,
    context: &mut PulsarContext,
    runtime: &mut RuntimeContext,
    tim: &Path,
) -> Result<(), TimingError> {
    model.psrname = String::from_utf8(read_string(&context.pulsars[0].name[..], "name")?)
        .map_err(|e| error(e.to_string()))?;
    model.cheby = Cheby2D::construct(model.cheby.nx(), model.cheby.ny(), |x, y| {
        cheby_model_func(model, context, runtime, x, y, true, tim).map_err(|e| e.to_string())
    })
    .map_err(error)?;
    model.frequency_cheby = model.cheby.x_derivative().map_err(error)?;
    Ok(())
}
pub fn test_model(
    model: &mut ChebyModel,
    context: &mut PulsarContext,
    runtime: &mut RuntimeContext,
    nmjd: usize,
    nfreq: usize,
    tim: &Path,
) -> Result<ErrorSummary, TimingError> {
    let polynomial = model.cheby.clone();
    polynomial
        .test(nmjd, nfreq, |x, y| {
            cheby_model_func(model, context, runtime, x, y, false, tim).map_err(|e| e.to_string())
        })
        .map_err(error)
}
pub fn construct_set(
    request: &ChebyRequest,
    context: &mut PulsarContext,
    runtime: &mut RuntimeContext,
    tim: &Path,
) -> Result<ChebyModelSet, TimingError> {
    let r = request;
    if r.end_mjd <= r.start_mjd
        || r.segment_days <= r.overlap_days
        || r.overlap_days < n(0.0f64)
        || r.freq_end_mhz <= r.freq_start_mhz
        || r.time_coeffs < 2
        || r.freq_coeffs < 2
    {
        return Err(error("invalid Cheby construction range/dimensions"));
    }
    // C++ ceil(long double) retains native precision; source assignment to int.
    let count = ((&r.end_mjd - &r.start_mjd) / (&r.segment_days - &r.overlap_days))
        .ceil_count()
        .map_err(error)?;
    let p = &mut context.pulsars[0];
    for (l, v) in [
        (param_tzrmjd, &r.start_mjd),
        (param_tzrfrq, &r.freq_start_mhz),
    ] {
        let c = &mut p.param.slots[l as usize].cells[0];
        if c.param_set == 0 {
            c.param_set = 1;
            c.val = Float::with_val(PRECISION_BITS, v.as_float());
        }
    }
    if read_string(&p.tzrsite[..], "tzrsite")?.is_empty() {
        write_string(&mut p.tzrsite[..], r.site.as_bytes(), "tzrsite")?;
    }
    let mut set = ChebyModelSet::default();
    for i in 0..count {
        let start = &r.start_mjd + (&r.segment_days - &r.overlap_days) * i as f64;
        let mut end = &start + &r.segment_days;
        if end > r.end_mjd {
            end = r.end_mjd.clone();
        }
        let cheby = Cheby2D::new(r.time_coeffs, r.freq_coeffs).map_err(error)?;
        let mut model = ChebyModel {
            psrname: String::new(),
            sitename: r.site.clone(),
            mjd_start: start,
            mjd_end: end,
            freq_start: r.freq_start_mhz.clone(),
            freq_end: r.freq_end_mhz.clone(),
            dispersion_constant: n(0.0f64),
            frequency_cheby: cheby.x_derivative().map_err(error)?,
            cheby,
        };
        construct_model(&mut model, context, runtime, tim)?;
        set.segments.push(model);
    }
    Ok(set)
}
pub fn test_set(
    set: &mut ChebyModelSet,
    context: &mut PulsarContext,
    runtime: &mut RuntimeContext,
    nmjd: usize,
    nfreq: usize,
    tim: &Path,
) -> Result<ErrorSummary, TimingError> {
    let count = set.segments.len();
    if count == 0 {
        return Err(error("empty Cheby test set"));
    }
    let (mut ss, mut mav) = (n(0.0f64), n(0.0f64));
    for m in &mut set.segments {
        let e = test_model(m, context, runtime, nmjd / count, nfreq / count, tim)?;
        ss = ss + &e.rms_turn * &e.rms_turn;
        if e.mav_turn > mav {
            mav = e.mav_turn;
        }
    }
    // cheby2d_int.C C++ sqrt overload operates on native long double.
    Ok(ErrorSummary {
        rms_turn: (ss / count as f64).sqrt(),
        mav_turn: mav,
    })
}
