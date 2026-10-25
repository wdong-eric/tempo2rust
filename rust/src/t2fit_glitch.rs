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
/// t2fit_glitch.C::t2FitFunc_stdGlitch (pinned source line 11).
pub(crate) fn t2FitFunc_stdGlitch(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if (label == (param_glph as i32)) {
        {
            if ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
                >= (psr.param.slots[(param_glep as i32) as usize].cells[(k) as usize].val).clone())
            {
                return Ok((Float::with_val(PRECISION_BITS, 1.0f64)
                    / (psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize].val)
                        .clone())
                .to_f64());
            } else {
                return Ok(0.0f64);
            }
        }
    } else {
        if (((label == (param_glf0d as i32)) || (label == (param_glf0d2 as i32)))
            || (label == (param_glf0d3 as i32)))
        {
            {
                let mut dt1: Float;
                let mut expf: Float;
                let mut tp: Float;
                let mut tgl: Float;
                let mut our_gltd: Option<i32> = None;
                if (label == (param_glf0d as i32)) {
                    our_gltd = Some((param_gltd as i32));
                }
                if (label == (param_glf0d2 as i32)) {
                    our_gltd = Some((param_gltd2 as i32));
                }
                if (label == (param_glf0d3 as i32)) {
                    our_gltd = Some((param_gltd3 as i32));
                }
                tp = (((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
                    - (psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize]
                        .val)
                        .clone())
                    * Float::with_val(PRECISION_BITS, 86400.0f64));
                tgl = (((psr.param.slots[(param_glep as i32) as usize].cells[(k) as usize].val)
                    .clone()
                    - (psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize]
                        .val)
                        .clone())
                    * Float::with_val(PRECISION_BITS, 86400.0f64));
                dt1 = ((tp).clone() - (tgl).clone());
                if ((psr.param.slots[(*read(&our_gltd, "our_gltd")?) as usize].cells[(k) as usize]
                    .val)
                    .clone()
                    != Float::with_val(PRECISION_BITS, 0.0f64))
                {
                    expf = (((-(dt1).clone()) / Float::with_val(PRECISION_BITS, 86400.0f64))
                        / (psr.param.slots[(*read(&our_gltd, "our_gltd")?) as usize].cells
                            [(k) as usize]
                            .val)
                            .clone())
                    .exp();
                } else {
                    expf = Float::with_val(PRECISION_BITS, 1.0f64);
                }
                if ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
                    >= (psr.param.slots[(param_glep as i32) as usize].cells[(k) as usize].val)
                        .clone())
                {
                    {
                        return Ok(((((psr.param.slots
                            [(*read(&our_gltd, "our_gltd")?) as usize]
                            .cells[(k) as usize]
                            .val)
                            .clone()
                            * Float::with_val(PRECISION_BITS, 86400.0f64))
                            * (Float::with_val(PRECISION_BITS, 1.0f64) - (expf).clone()))
                            / (psr.param.slots[(param_f as i32) as usize].cells
                                [(0i32) as usize]
                                .val)
                                .clone())
                        .to_f64());
                    }
                } else {
                    return Ok(0.0f64);
                }
            }
        } else {
            if (((label == (param_gltd as i32)) || (label == (param_gltd2 as i32)))
                || (label == (param_gltd3 as i32)))
            {
                {
                    let mut dt1: Float;
                    let mut expf: Float;
                    let mut tp: Float;
                    let mut tgl: Float;
                    let mut our_glf0d: Option<i32> = None;
                    if (label == (param_gltd as i32)) {
                        our_glf0d = Some((param_glf0d as i32));
                    }
                    if (label == (param_gltd2 as i32)) {
                        our_glf0d = Some((param_glf0d2 as i32));
                    }
                    if (label == (param_gltd3 as i32)) {
                        our_glf0d = Some((param_glf0d3 as i32));
                    }
                    let mut our_gltd: Option<i32> = Some(label);
                    tp = (((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
                        - (psr.param.slots[(param_pepoch as i32) as usize].cells
                            [(0i32) as usize]
                            .val)
                            .clone())
                        * Float::with_val(PRECISION_BITS, 86400.0f64));
                    tgl = (((psr.param.slots[(param_glep as i32) as usize].cells[(k) as usize]
                        .val)
                        .clone()
                        - (psr.param.slots[(param_pepoch as i32) as usize].cells
                            [(0i32) as usize]
                            .val)
                            .clone())
                        * Float::with_val(PRECISION_BITS, 86400.0f64));
                    dt1 = ((tp).clone() - (tgl).clone());
                    if ((psr.param.slots[(*read(&our_gltd, "our_gltd")?) as usize].cells
                        [(k) as usize]
                        .val)
                        .clone()
                        != Float::with_val(PRECISION_BITS, 0.0f64))
                    {
                        expf = (((-(dt1).clone()) / Float::with_val(PRECISION_BITS, 86400.0f64))
                            / (psr.param.slots[(*read(&our_gltd, "our_gltd")?) as usize].cells
                                [(k) as usize]
                                .val)
                                .clone())
                        .exp();
                    } else {
                        expf = Float::with_val(PRECISION_BITS, 1.0f64);
                    }
                    if ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
                        >= (psr.param.slots[(param_glep as i32) as usize].cells[(k) as usize].val)
                            .clone())
                    {
                        return Ok(((((psr.param.slots
                            [(*read(&our_glf0d, "our_glf0d")?) as usize]
                            .cells[(k) as usize]
                            .val)
                            .clone()
                            * (Float::with_val(PRECISION_BITS, 1.0f64)
                                - ((Float::with_val(PRECISION_BITS, 1.0f64)
                                    + (((dt1).clone()
                                        / Float::with_val(PRECISION_BITS, 86400.0f64))
                                        / (psr.param.slots
                                            [(*read(&our_gltd, "our_gltd")?) as usize]
                                            .cells[(k) as usize]
                                            .val)
                                            .clone()))
                                    * (expf).clone())))
                            / (psr.param.slots[(param_f as i32) as usize].cells
                                [(0i32) as usize]
                                .val)
                                .clone())
                            * Float::with_val(PRECISION_BITS, 86400.0f64))
                        .to_f64());
                    } else {
                        return Ok(0.0f64);
                    }
                }
            } else {
                if (label == (param_glf0 as i32)) {
                    {
                        if ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
                            >= (psr.param.slots[(param_glep as i32) as usize].cells[(k) as usize]
                                .val)
                                .clone())
                        {
                            {
                                return Ok(((((read(
                                    &rows[(ipos) as usize].bbat,
                                    "psr[p].obsn[ipos].bbat",
                                )?)
                                .clone()
                                    - (psr.param.slots[(param_glep as i32) as usize].cells
                                        [(k) as usize]
                                        .val)
                                        .clone())
                                    * Float::with_val(PRECISION_BITS, 86400.0f64))
                                    / (psr.param.slots[(param_f as i32) as usize].cells
                                        [(0i32) as usize]
                                        .val)
                                        .clone())
                                .to_f64());
                            }
                        } else {
                            return Ok(0.0f64);
                        }
                    }
                } else {
                    if (label == (param_glf1 as i32)) {
                        {
                            if ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?)
                                .clone()
                                >= (psr.param.slots[(param_glep as i32) as usize].cells
                                    [(k) as usize]
                                    .val)
                                    .clone())
                            {
                                return Ok(((Float::with_val(PRECISION_BITS, 0.5f64)
                                    * (((read(
                                        &rows[(ipos) as usize].bbat,
                                        "psr[p].obsn[ipos].bbat",
                                    )?)
                                    .clone()
                                        - (psr.param.slots[(param_glep as i32) as usize]
                                            .cells[(k) as usize]
                                            .val)
                                            .clone())
                                        * Float::with_val(PRECISION_BITS, 86400.0f64))
                                    .pow(&(Float::with_val(PRECISION_BITS, 2i32))))
                                    / (psr.param.slots[(param_f as i32) as usize].cells
                                        [(0i32) as usize]
                                        .val)
                                        .clone())
                                .to_f64());
                            } else {
                                return Ok(0.0f64);
                            }
                        }
                    } else {
                        if (label == (param_glf2 as i32)) {
                            {
                                if ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?)
                                    .clone()
                                    >= (psr.param.slots[(param_glep as i32) as usize].cells
                                        [(k) as usize]
                                        .val)
                                        .clone())
                                {
                                    {
                                        return Ok(((Float::with_val(
                                            PRECISION_BITS,
                                            (1.0f64 / 6.0f64),
                                        ) * ((((read(
                                            &rows[(ipos) as usize].bbat,
                                            "psr[p].obsn[ipos].bbat",
                                        )?)
                                        .clone()
                                            - (psr.param.slots
                                                [(param_glep as i32) as usize]
                                                .cells
                                                [(k) as usize]
                                                .val)
                                                .clone())
                                            * Float::with_val(PRECISION_BITS, 86400.0f64))
                                            / Float::with_val(PRECISION_BITS, 1.0e9f64))
                                        .pow(&(Float::with_val(PRECISION_BITS, 3i32))))
                                            / (psr.param.slots[(param_f as i32) as usize].cells
                                                [(0i32) as usize]
                                                .val)
                                                .clone())
                                        .to_f64());
                                    }
                                } else {
                                    return Ok(0.0f64);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    d.log(
        b'e',
        "t2fit_glitch.C",
        96,
        format!(
            "Unknown glitch parameter: label={} str={} k={}",
            label,
            crate::tempo2::PARAMETER_NAMES[(label) as usize],
            k
        )
        .as_bytes(),
    );
    return Err(boundary("t2fit_glitch.C::t2FitFunc_stdGlitch: assertion"));
}

/// t2fit_glitch.C::t2UpdateFunc_stdGlitch (pinned source line 100).
pub(crate) fn t2UpdateFunc_stdGlitch(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut error: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    let mut F: f64 = (1i32) as f64;
    if (label == (param_glf2 as i32)) {
        F = 1e-27f64;
    }
    psr.param.slots[(label) as usize].cells[(k) as usize].val =
        ((psr.param.slots[(label) as usize].cells[(k) as usize].val).clone()
            - Float::with_val(PRECISION_BITS, (val * F)));
    psr.param.slots[(label) as usize].cells[(k) as usize].err =
        Float::with_val(PRECISION_BITS, (error * F));
    Ok(())
}

/// t2fit_glitch.C::t2FitFunc_expdip (pinned source line 110).
pub(crate) fn t2FitFunc_expdip(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut val: f64;
    let mut dt: f64;
    let mut dm: f64;
    let mut tau: f64;
    let mut freq: f64;
    let mut gamma: f64;
    freq = (*read(&rows[(ipos) as usize].freqSSB, "psr[p].obsn[ipos].freqSSB")? / 1.4e9f64);
    dt = ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
        - (psr.param.slots[(param_expep as i32) as usize].cells[(k) as usize].val).clone())
    .to_f64();
    dm =
        ((psr.param.slots[(param_expph as i32) as usize].cells[(k) as usize].val).clone()).to_f64();
    tau = ((psr.param.slots[(param_exptau as i32) as usize].cells[(k) as usize].val).clone())
        .to_f64();
    if (psr.param.slots[(param_expindex as i32) as usize].cells[(k) as usize].param_set == 1i32) {
        {
            gamma = ((psr.param.slots[(param_expindex as i32) as usize].cells[(k) as usize].val)
                .clone())
            .to_f64();
        }
    } else {
        {
            gamma = (-2i32) as f64;
        }
    }
    if (label == (param_expph as i32)) {
        {
            if (dt > (0i32) as f64) {
                {
                    return Ok({
                        val = ((Float::with_val(PRECISION_BITS, freq))
                            .pow(&(Float::with_val(PRECISION_BITS, gamma)))
                            * Float::with_val(PRECISION_BITS, ((-dt) / tau).exp()))
                        .to_f64();
                        val
                    });
                }
            } else {
                {
                    return Ok(0.0f64);
                }
            }
        }
    }
    if (label == (param_exptau as i32)) {
        {
            if (dt > (0i32) as f64) {
                {
                    return Ok({
                        val = (((((Float::with_val(PRECISION_BITS, dm)
                            * (Float::with_val(PRECISION_BITS, freq))
                                .pow(&(Float::with_val(PRECISION_BITS, gamma))))
                            * Float::with_val(PRECISION_BITS, ((-dt) / tau).exp()))
                            * Float::with_val(PRECISION_BITS, dt))
                            / Float::with_val(PRECISION_BITS, tau))
                            / Float::with_val(PRECISION_BITS, tau))
                        .to_f64();
                        val
                    });
                }
            } else {
                {
                    return Ok(0.0f64);
                }
            }
        }
    }
    if (label == (param_expindex as i32)) {
        {
            if (dt > (0i32) as f64) {
                {
                    return Ok({
                        val = ((Float::with_val(PRECISION_BITS, (dm * (freq).ln()))
                            * (Float::with_val(PRECISION_BITS, freq))
                                .pow(&(Float::with_val(PRECISION_BITS, gamma))))
                            * Float::with_val(PRECISION_BITS, ((-dt) / tau).exp()))
                        .to_f64();
                        val
                    });
                }
            } else {
                {
                    return Ok(0.0f64);
                }
            }
        }
    }
    if (label == (param_expep as i32)) {
        {
            if (dt > (0i32) as f64) {
                {
                    return Ok({
                        val = (((Float::with_val(PRECISION_BITS, dm)
                            * (Float::with_val(PRECISION_BITS, freq))
                                .pow(&(Float::with_val(PRECISION_BITS, gamma))))
                            * Float::with_val(PRECISION_BITS, ((-dt) / tau).exp()))
                            / Float::with_val(PRECISION_BITS, tau))
                        .to_f64();
                        val
                    });
                }
            } else {
                {
                    return Ok(0.0f64);
                }
            }
        }
    }
    return Ok((0i32) as f64);
}

/// t2fit_glitch.C::t2FitFunc_gausdip (pinned source line 204).
pub(crate) fn t2FitFunc_gausdip(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut val: f64;
    let mut dt: f64;
    let mut amp: f64;
    let mut sig: f64;
    let mut freq: f64;
    let mut gamma: f64;
    freq = (*read(&rows[(ipos) as usize].freqSSB, "psr[p].obsn[ipos].freqSSB")? / 1.4e9f64);
    dt = ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
        - (psr.param.slots[(param_gausep as i32) as usize].cells[(k) as usize].val).clone())
    .to_f64();
    amp = ((psr.param.slots[(param_gausamp as i32) as usize].cells[(k) as usize].val).clone())
        .to_f64();
    sig = ((psr.param.slots[(param_gaussig as i32) as usize].cells[(k) as usize].val).clone())
        .to_f64();
    if (psr.param.slots[(param_gausindex as i32) as usize].cells[(k) as usize].param_set == 1i32) {
        {
            gamma = ((psr.param.slots[(param_gausindex as i32) as usize].cells[(k) as usize].val)
                .clone())
            .to_f64();
        }
    } else {
        {
            gamma = (0i32) as f64;
        }
    }
    val = ((Float::with_val(PRECISION_BITS, amp)
        * (Float::with_val(PRECISION_BITS, freq)).pow(&(Float::with_val(PRECISION_BITS, gamma))))
        * Float::with_val(
            PRECISION_BITS,
            (((((-dt) * dt) / 2.0f64) / sig) / sig).exp(),
        ))
    .to_f64();
    if (label == (param_gausamp as i32)) {
        {
            return Ok((val / amp));
        }
    }
    if (label == (param_gausep as i32)) {
        {
            return Ok((val * (((-dt) / sig) / sig)));
        }
    }
    if (label == (param_gaussig as i32)) {
        {
            return Ok((val * (((((dt * dt) / 6.0f64) / sig) / sig) / sig)));
        }
    }
    if (label == (param_gausindex as i32)) {
        {
            return Ok((val * (freq).ln()));
        }
    }
    return Ok((0i32) as f64);
}
