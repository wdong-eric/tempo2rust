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
use crate::ordinary_long_double as ordinary;
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
    // t2fit_glitch.C:110-198: source locals are ordinary long double;
    // powl remains wide, then assignment to val precedes the double return.
    let freq =
        ordinary::round(*read(&rows[ipos as usize].freqSSB, "psr[p].obsn[ipos].freqSSB")? / 1.4e9);
    let dt = ordinary::round(
        read(&rows[ipos as usize].bbat, "psr[p].obsn[ipos].bbat")?.clone()
            - &psr.param.slots[param_expep as usize].cells[k as usize].val,
    );
    let dm = ordinary::round(&psr.param.slots[param_expph as usize].cells[k as usize].val);
    let tau = ordinary::round(&psr.param.slots[param_exptau as usize].cells[k as usize].val);
    let index = &psr.param.slots[param_expindex as usize].cells[k as usize];
    let gamma = if index.param_set == 1 {
        ordinary::round(&index.val)
    } else {
        ordinary::round(-2)
    };
    if dt > 0 {
        let exponent = ordinary::div(&(-dt.clone()), &tau);
        // Source overload exp(long double) and expl both return ordinary values.
        let decay = Float::with_val(PRECISION_BITS, ordinary::exp(&exponent));
        let power =
            Float::with_val(PRECISION_BITS, &freq).pow(&Float::with_val(PRECISION_BITS, &gamma));
        let val = if label == param_expph as i32 {
            power * decay
        } else if label == param_exptau as i32 {
            Float::with_val(PRECISION_BITS, &dm)
                * power
                * decay
                * Float::with_val(PRECISION_BITS, &dt)
                / Float::with_val(PRECISION_BITS, &tau)
                / Float::with_val(PRECISION_BITS, &tau)
        } else if label == param_expindex as i32 {
            let logarithm = ordinary::ln(&freq);
            let logarithmic_amplitude = ordinary::mul(&dm, &logarithm);
            Float::with_val(PRECISION_BITS, logarithmic_amplitude) * power * decay
        } else if label == param_expep as i32 {
            Float::with_val(PRECISION_BITS, &dm) * power * decay
                / Float::with_val(PRECISION_BITS, &tau)
        } else {
            return Ok(0.0);
        };
        return Ok(ordinary::round(val).to_f64());
    }
    Ok(0.0)
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
    // t2fit_glitch.C:204-257. Retain the source derivative signs and 6.
    // denominator; this is a type-boundary correction, not a formula change.
    let freq =
        ordinary::round(*read(&rows[ipos as usize].freqSSB, "psr[p].obsn[ipos].freqSSB")? / 1.4e9);
    let dt = ordinary::round(
        read(&rows[ipos as usize].bbat, "psr[p].obsn[ipos].bbat")?.clone()
            - &psr.param.slots[param_gausep as usize].cells[k as usize].val,
    );
    let amp = ordinary::round(&psr.param.slots[param_gausamp as usize].cells[k as usize].val);
    let sig = ordinary::round(&psr.param.slots[param_gaussig as usize].cells[k as usize].val);
    let index = &psr.param.slots[param_gausindex as usize].cells[k as usize];
    let gamma = if index.param_set == 1 {
        ordinary::round(&index.val)
    } else {
        ordinary::round(0)
    };
    let exponent = ordinary::mul(&(-dt.clone()), &dt);
    let exponent = ordinary::div(&exponent, &ordinary::round(2));
    let exponent = ordinary::div(&exponent, &sig);
    let exponent = ordinary::div(&exponent, &sig);
    let val = ordinary::round(
        Float::with_val(PRECISION_BITS, &amp)
            * Float::with_val(PRECISION_BITS, &freq).pow(&Float::with_val(PRECISION_BITS, &gamma))
            * Float::with_val(PRECISION_BITS, ordinary::exp(&exponent)),
    );
    let result = if label == param_gausamp as i32 {
        ordinary::div(&val, &amp)
    } else if label == param_gausep as i32 {
        let factor = ordinary::div(&(-dt.clone()), &sig);
        let factor = ordinary::div(&factor, &sig);
        ordinary::mul(&val, &factor)
    } else if label == param_gaussig as i32 {
        let factor = ordinary::mul(&dt, &dt);
        let factor = ordinary::div(&factor, &ordinary::round(6));
        let factor = ordinary::div(&factor, &sig);
        let factor = ordinary::div(&factor, &sig);
        let factor = ordinary::div(&factor, &sig);
        ordinary::mul(&val, &factor)
    } else if label == param_gausindex as i32 {
        let logarithm = ordinary::ln(&freq);
        ordinary::mul(&val, &logarithm)
    } else {
        return Ok(0.0);
    };
    Ok(result.to_f64())
}
