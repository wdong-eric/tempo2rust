//! Source fitting callbacks, TEMPO2 fda6ad4, GPL-3.0-or-later.
#![allow(
    clippy::double_parens,
    clippy::assign_op_pattern,
    clippy::redundant_guards,
    clippy::useless_format
)]
#![allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_assignments,
    unused_parens
)]
#![allow(
    clippy::excessive_precision,
    clippy::too_many_arguments,
    clippy::manual_range_contains,
    clippy::neg_cmp_op_on_partial_ord,
    clippy::neg_multiply,
    clippy::unnecessary_cast,
    clippy::needless_return
)]
use crate::binary::source_i32;
use crate::{
    TimingError,
    clock_diagnostics::{Diagnostics, boundary},
    observation::{SourceObservation, read},
    pulsar::SourcePulsar,
    tempo2::Label::*,
};
/// t2fit_fitwaves.C::t2FitFunc_fitwaves (pinned source line 5).
pub(crate) fn t2FitFunc_fitwaves(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut kk: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !((label == (param_wave_dm as i32)) || (label == (param_wave_om as i32))) {
        return Err(boundary("t2fit_fitwaves.C::t2FitFunc_fitwaves: assertion"));
    }
    let mut Xoff: f64 = (0i32) as f64;
    let mut om: f64 =
        ((psr.param.slots[(label) as usize].cells[(0i32) as usize].val).clone()).to_f64();
    let mut afactor: f64 = 1.0f64;
    let mut w_epoch_par: i32 = (param_waveepoch as i32);
    let mut waveScale: i32 = source_i32(*read(&psr.waveScale, "psr[p].waveScale")?)?;
    if (label == (param_wave_dm as i32)) {
        {
            w_epoch_par = (param_waveepoch_dm as i32);
        }
    }
    if (((label == (param_wave_dm as i32)) || (waveScale == 1i32))
        || ((waveScale == 2i32) && (kk < (*read(&psr.nWhite, "psr[p].nWhite")? * 2i32))))
    {
        {
            let mut freq: f64 =
                (*read(&rows[(ipos) as usize].freqSSB, "psr[p].obsn[ipos].freqSSB")? / 1.0e6f64);
            afactor = (1.0f64 / ((2.41e-4f64 * freq) * freq));
        }
    }
    // t2fit_fitwaves.C used nWhite even for WAVDM: a zero divisor or repeated
    // harmonics when the two counts differ. check_callbacks.py preserves both runs.
    let n = if label == param_wave_dm as i32 {
        *read(&psr.nWhite_dm, "nWhite_dm")?
    } else {
        *read(&psr.nWhite, "nWhite")?
    };
    if n == 0 {
        return Err(boundary("fitwaves: zero harmonic count"));
    }
    let mut k: i32 = kk % (n * 2);
    if (psr.param.slots[(w_epoch_par) as usize].cells[(0i32) as usize].param_set != 0) {
        {
            Xoff = ((psr.param.slots[(w_epoch_par) as usize].cells[(0i32) as usize].val).clone()
                - (psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize].val)
                    .clone())
            .to_f64();
        }
    }
    if ((k % 2i32) == 0i32) {
        return Ok(
            (afactor * ((om * (((k) as f64 / 2.0f64).floor() + (1i32) as f64)) * (x - Xoff)).cos())
        );
    } else {
        return Ok(
            (afactor * ((om * (((k) as f64 / 2.0f64).floor() + (1i32) as f64)) * (x - Xoff)).sin())
        );
    }
}

/// t2fit_fitwaves.C::t2UpdateFunc_fitwaves (pinned source line 32).
pub(crate) fn t2UpdateFunc_fitwaves(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut error: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    let mut waveScale: i32 = source_i32(*read(&psr.waveScale, "psr[p].waveScale")?)?;
    if (((label == (param_wave_dm as i32)) || (waveScale == 1i32))
        || ((waveScale == 2i32) && (k < (*read(&psr.nWhite, "psr[p].nWhite")? * 2i32))))
    {
        {
            if ((k % 2i32) == 0i32) {
                {
                    d.log(
                        b'm',
                        "t2fit_fitwaves.C",
                        37,
                        format!("dm.COS({}) = {}", (k / 2i32), val).as_bytes(),
                    );
                    psr.wave_cos_dm[(k / 2i32) as usize] = Some(
                        (*read(
                            &psr.wave_cos_dm[(k / 2i32) as usize],
                            "psr[p].wave_cos_dm[k / 2]",
                        )? - val),
                    );
                    psr.wave_cos_dm_err[(k / 2i32) as usize] = Some(error);
                }
            } else {
                {
                    d.log(
                        b'm',
                        "t2fit_fitwaves.C",
                        41,
                        format!("dm.SIN({}) = {}", (k / 2i32), val).as_bytes(),
                    );
                    psr.wave_sine_dm[(k / 2i32) as usize] = Some(
                        (*read(
                            &psr.wave_sine_dm[(k / 2i32) as usize],
                            "psr[p].wave_sine_dm[k / 2]",
                        )? - val),
                    );
                    psr.wave_sine_dm_err[(k / 2i32) as usize] = Some(error);
                }
            }
        }
    } else {
        {
            k = (k % (*read(&psr.nWhite, "psr[p].nWhite")? * 2i32));
            if ((k % 2i32) == 0i32) {
                {
                    d.log(
                        b'm',
                        "t2fit_fitwaves.C",
                        49,
                        format!("COS({}) = {}", (k / 2i32), val).as_bytes(),
                    );
                    psr.wave_cos[(k / 2i32) as usize] = Some(
                        (*read(&psr.wave_cos[(k / 2i32) as usize], "psr[p].wave_cos[k / 2]")?
                            - val),
                    );
                    psr.wave_cos_err[(k / 2i32) as usize] = Some(error);
                }
            } else {
                {
                    d.log(
                        b'm',
                        "t2fit_fitwaves.C",
                        53,
                        format!("SIN({}) = {}", (k / 2i32), val).as_bytes(),
                    );
                    psr.wave_sine[(k / 2i32) as usize] = Some(
                        (*read(
                            &psr.wave_sine[(k / 2i32) as usize],
                            "psr[p].wave_sine[k / 2]",
                        )? - val),
                    );
                    psr.wave_sine_err[(k / 2i32) as usize] = Some(error);
                }
            }
        }
    }
    Ok(())
}
