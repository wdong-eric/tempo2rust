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
use crate::{
    PRECISION_BITS, TimingError,
    clock_diagnostics::{Diagnostics, boundary},
    observation::{SourceObservation, read},
    pulsar::SourcePulsar,
    tempo2::Label::*,
};
use rug::{Float, ops::Pow};
/// t2fit_dmother.C::t2FitFunc_dmx (pinned source line 12).
pub(crate) fn t2FitFunc_dmx(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !(label == (param_dmx as i32)) {
        return Err(boundary("t2fit_dmother.C::t2FitFunc_dmx: assertion"));
    }
    if (((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()
        > (psr.param.slots[(param_dmxr1 as i32) as usize].cells[(k) as usize].val).clone())
        && ((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()
            < (psr.param.slots[(param_dmxr2 as i32) as usize].cells[(k) as usize].val).clone()))
    {
        {
            return Ok((Float::with_val(PRECISION_BITS, 1.0f64)
                / (Float::with_val(PRECISION_BITS, 2.41e-4f64)
                    * (Float::with_val(
                        PRECISION_BITS,
                        (*read(&rows[(ipos) as usize].freqSSB, "psr[p].obsn[ipos].freqSSB")?
                            / 1.0e6f64),
                    ))
                    .pow(&(Float::with_val(PRECISION_BITS, 2i32)))))
            .to_f64());
        }
    } else {
        {
            return Ok(0.0f64);
        }
    }
}

/// t2fit_dmother.C::t2FitFunc_chromx (pinned source line 24).
pub(crate) fn t2FitFunc_chromx(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !(label == (param_chromx as i32)) {
        return Err(boundary("t2fit_dmother.C::t2FitFunc_chromx: assertion"));
    }
    if (((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()
        > (psr.param.slots[(param_chromxr1 as i32) as usize].cells[(k) as usize].val).clone())
        && ((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()
            < (psr.param.slots[(param_chromxr2 as i32) as usize].cells[(k) as usize].val).clone()))
    {
        {
            return Ok((Float::with_val(PRECISION_BITS, 1.0f64)
                / (Float::with_val(
                    PRECISION_BITS,
                    (*read(&rows[(ipos) as usize].freqSSB, "psr[p].obsn[ipos].freqSSB")?
                        / 1.4e9f64),
                ))
                .pow(
                    &(Float::with_val(
                        PRECISION_BITS,
                        *read(&psr.TNChromIdx, "psr[p].TNChromIdx")?,
                    )),
                ))
            .to_f64());
        }
    } else {
        {
            return Ok(0.0f64);
        }
    }
}

/// t2fit_dmother.C::t2FitFunc_fddc (pinned source line 36).
pub(crate) fn t2FitFunc_fddc(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !(label == (param_fddc as i32)) {
        return Err(boundary("t2fit_dmother.C::t2FitFunc_fddc: assertion"));
    }
    return Ok((1.0f64
        / (*read(&rows[(ipos) as usize].freqSSB, "psr[p].obsn[ipos].freqSSB")? / 1.0e6f64).powf(
            ((psr.param.slots[(param_fddi as i32) as usize].cells[(0i32) as usize].val).clone())
                .to_f64(),
        )));
}

/// t2fit_dmother.C::t2FitFunc_fd (pinned source line 41).
pub(crate) fn t2FitFunc_fd(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !(label == (param_fd as i32)) {
        return Err(boundary("t2fit_dmother.C::t2FitFunc_fd: assertion"));
    }
    return Ok(
        ((*read(&rows[(ipos) as usize].freqSSB, "psr[p].obsn[ipos].freqSSB")? / 1e9f64).ln())
            .powf((k + 1i32) as f64),
    );
}

/// t2fit_dmother.C::t2FitFunc_dmsinusoids (pinned source line 46).
pub(crate) fn t2FitFunc_dmsinusoids(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if (label == (param_dm_sin1yr as i32)) {
        {
            return Ok(((Float::with_val(PRECISION_BITS, 1.0f64)
                / (Float::with_val(PRECISION_BITS, 2.41e-4f64)
                    * (Float::with_val(
                        PRECISION_BITS,
                        (*read(&rows[(ipos) as usize].freqSSB, "psr[p].obsn[ipos].freqSSB")?
                            / 1.0e6f64),
                    ))
                    .pow(&(Float::with_val(PRECISION_BITS, 2i32)))))
                * (Float::with_val(
                    PRECISION_BITS,
                    (((2i32) as f64 * std::f64::consts::PI) / 365.25f64),
                ) * ((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()
                    - (psr.param.slots[(param_dmepoch as i32) as usize].cells
                        [(0i32) as usize]
                        .val)
                        .clone()))
                .sin())
            .to_f64());
        }
    } else {
        if (label == (param_dm_cos1yr as i32)) {
            {
                return Ok(((Float::with_val(PRECISION_BITS, 1.0f64)
                    / (Float::with_val(PRECISION_BITS, 2.41e-4f64)
                        * (Float::with_val(
                            PRECISION_BITS,
                            (*read(&rows[(ipos) as usize].freqSSB, "psr[p].obsn[ipos].freqSSB")?
                                / 1.0e6f64),
                        ))
                        .pow(&(Float::with_val(PRECISION_BITS, 2i32)))))
                    * (Float::with_val(
                        PRECISION_BITS,
                        (((2i32) as f64 * std::f64::consts::PI) / 365.25f64),
                    ) * ((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?)
                        .clone()
                        - (psr.param.slots[(param_dmepoch as i32) as usize].cells
                            [(0i32) as usize]
                            .val)
                            .clone()))
                    .cos())
                .to_f64());
            }
        } else {
            {
                d.log(
                    b'e',
                    "t2fit_dmother.C",
                    52,
                    format!("Called dmsinusoids without dmsin1yr or dmcos1yr").as_bytes(),
                );
                return Ok((0i32) as f64);
            }
        }
    }
}
