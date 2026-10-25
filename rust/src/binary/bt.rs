//! BTmodel.C model, derivatives, updates and private helpers.
//! TEMPO2 fda6ad4; GPL-3.0-or-later; original TEMPO2 contributors.
#![allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_assignments,
    unused_parens
)]
// Source predicates retain NaN semantics; signatures, constants and explicit
// promotion boundaries follow the pinned routine for numerical comparison.
#![allow(
    clippy::manual_range_contains,
    clippy::neg_cmp_op_on_partial_ord,
    clippy::if_same_then_else,
    clippy::too_many_arguments,
    clippy::excessive_precision,
    clippy::neg_multiply,
    clippy::unnecessary_cast
)]
use super::*;
/// BTmodel.C::BTmodel (pinned source line 37).
pub(super) fn BTmodel(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut ipos: i32,
    mut param: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut torb: f64;
    let mut tt0: f64;
    let mut orbits: f64;
    let mut pb: f64;
    let mut pbdot: f64;
    let mut xpbdot: f64;
    let mut ecc: f64;
    let mut edot: f64;
    let mut asini: f64;
    let mut xdot: f64;
    let mut omdot: f64;
    let mut omega: f64;
    let mut gamma: f64;
    let mut norbits: i32;
    let mut phase: f64;
    let mut ep: f64;
    let mut dep: f64;
    let mut bige: f64;
    let mut tt: f64;
    let mut som: f64;
    let mut com: f64;
    let mut alpha: f64;
    let mut beta: f64;
    let mut sbe: f64;
    let mut cbe: f64;
    let mut q: f64;
    let mut r: f64;
    let mut s: f64;
    d.version("BTmodel.C", "BTmodel()");
    tt0 = (((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
        - (psr.param.slots[param_t0 as usize].cells[0].val).clone())
        * Float::with_val(PRECISION_BITS, 86400.0f64))
    .to_f64();
    pb = ((psr.param.slots[param_pb as usize].cells[0].val).clone()
        * Float::with_val(PRECISION_BITS, 86400.0f64))
    .to_f64();
    edot = 0.0f64;
    ecc = ((psr.param.slots[param_ecc as usize].cells[0].val).clone()
        + Float::with_val(PRECISION_BITS, (edot * tt0)))
    .to_f64();
    if ((ecc < 0.0f64) || (ecc > 1.0f64)) {
        {
            d.stdout(
                &[
                    b"BTmodel: problem with eccentricity = ".to_vec(),
                    crate::clock_text::extended_general_six(
                        &((psr.param.slots[param_ecc as usize].cells[0].val).clone()),
                    ),
                    b"\n".to_vec(),
                ]
                .concat(),
            );
            return Err(boundary("BTmodel.C::BTmodel: source exit"));
        }
    }
    if (psr.param.slots[param_pbdot as usize].cells[0].param_set == 1i32) {
        pbdot = ((psr.param.slots[param_pbdot as usize].cells[0].val).clone()).to_f64();
    } else {
        pbdot = 0.0f64;
    }
    xpbdot = 0.0f64;
    if (psr.param.slots[param_a1dot as usize].cells[0].param_set == 1i32) {
        xdot = ((psr.param.slots[param_a1dot as usize].cells[0].val).clone()).to_f64();
    } else {
        xdot = 0.0f64;
    }
    asini = ((psr.param.slots[param_a1 as usize].cells[0].val).clone()
        + Float::with_val(PRECISION_BITS, (xdot * tt0)))
    .to_f64();
    if (psr.param.slots[param_omdot as usize].cells[0].param_set == 1i32) {
        omdot = ((psr.param.slots[param_omdot as usize].cells[0].val).clone()).to_f64();
    } else {
        omdot = 0.0f64;
    }
    omega = (((psr.param.slots[param_om as usize].cells[0].val).clone()
        + Float::with_val(PRECISION_BITS, ((omdot * tt0) / (86400.0f64 * 365.25f64))))
        / Float::with_val(PRECISION_BITS, (180.0f64 / std::f64::consts::PI)))
    .to_f64();
    if (psr.param.slots[param_gamma as usize].cells[0].param_set == 1i32) {
        gamma = ((psr.param.slots[param_gamma as usize].cells[0].val).clone()).to_f64();
    } else {
        gamma = 0.0f64;
    }
    torb = 0.0f64;
    orbits = ((tt0 / pb) - ((0.5f64 * (pbdot + xpbdot)) * (tt0 / pb).powf((2i32) as f64)));
    norbits = source_i32(orbits)?;
    if (orbits < 0.0f64) {
        norbits = norbits
            .checked_sub(1)
            .ok_or_else(|| boundary("binary orbit count overflow"))?;
    }
    phase = ((2.0f64 * std::f64::consts::PI) * (orbits - (norbits) as f64));
    ep = (phase + ((ecc * (phase).sin()) * (1.0f64 + (ecc * (phase).cos()))));
    loop {
        {
            dep = ((phase - (ep - (ecc * (ep).sin()))) / (1.0f64 - (ecc * (ep).cos())));
            ep += dep;
        }
        if !((dep).abs() > 1.0e-12f64) {
            break;
        }
    }
    bige = ep;
    tt = (1.0f64 - (ecc * ecc));
    som = (omega).sin();
    com = (omega).cos();
    alpha = (asini * som);
    beta = ((asini * com) * (tt).sqrt());
    sbe = (bige).sin();
    cbe = (bige).cos();
    q = ((alpha * (cbe - ecc)) + ((beta + gamma) * sbe));
    r = (((-alpha) * sbe) + (beta * cbe));
    s = (1.0f64 / (1.0f64 - (ecc * cbe)));
    torb += ((-q) + ((((((2i32) as f64 * std::f64::consts::PI) / pb) * q) * r) * s));
    if (param == (-1i32)) {
        return Ok(torb);
    }
    if (param == (param_pb as i32)) {
        return Ok(
            ((((((((-2.0f64) * std::f64::consts::PI) * r) * s) / pb) * 86400.0f64) * tt0)
                / (86400.0f64 * pb))
                * 86400.0f64,
        );
    } else {
        if (param == (param_a1 as i32)) {
            return Ok((som * (cbe - ecc)) + ((com * sbe) * (tt).sqrt()));
        } else {
            if (param == (param_ecc as i32)) {
                return Ok(((-(((alpha * ((1.0f64 + (sbe * sbe)) - (ecc * cbe))) * tt)
                    - ((beta * (cbe - ecc)) * sbe)))
                    * s)
                    / tt);
            } else {
                if (param == (param_om as i32)) {
                    return Ok(asini * ((com * (cbe - ecc)) - ((som * (tt).sqrt()) * sbe)));
                } else {
                    if (param == (param_t0 as i32)) {
                        return Ok(
                            (((((-2.0f64) * std::f64::consts::PI) / pb) * r) * s) * 86400.0f64
                        );
                    } else {
                        if (param == (param_pbdot as i32)) {
                            return Ok((0.5f64
                                * ((((((((-2.0f64) * std::f64::consts::PI) * r) * s) / pb)
                                    * 86400.0f64)
                                    * tt0)
                                    / (86400.0f64 * pb)))
                                * tt0);
                        } else {
                            if (param == (param_a1dot as i32)) {
                                return Ok(
                                    ((som * (cbe - ecc)) + ((com * sbe) * (tt).sqrt())) * tt0
                                );
                            } else {
                                if (param == (param_omdot as i32)) {
                                    return Ok((asini
                                        * ((com * (cbe - ecc)) - ((som * (tt).sqrt()) * sbe)))
                                        * tt0);
                                } else {
                                    if (param == (param_edot as i32)) {
                                        return Ok((((-(((alpha
                                            * ((1.0f64 + (sbe * sbe)) - (ecc * cbe)))
                                            * tt)
                                            - ((beta * (cbe - ecc)) * sbe)))
                                            * s)
                                            / tt)
                                            * tt0);
                                    } else {
                                        if (param == (param_gamma as i32)) {
                                            return Ok(sbe);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(0.0f64)
}

/// BTmodel.C::updateBT (pinned source line 144).
pub(super) fn updateBT(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut val: f64,
    mut err: f64,
    mut pos: i32,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    if (pos == (param_pb as i32)) {
        {
            psr.param.slots[param_pb as usize].cells[0].val =
                ((psr.param.slots[param_pb as usize].cells[0].val).clone()
                    + Float::with_val(PRECISION_BITS, val));
            psr.param.slots[param_pb as usize].cells[0].err = Float::with_val(PRECISION_BITS, err);
        }
    } else {
        if (((((pos == (param_a1 as i32)) || (pos == (param_ecc as i32)))
            || (pos == (param_t0 as i32)))
            || (pos == (param_gamma as i32)))
            || (pos == (param_edot as i32)))
        {
            {
                psr.param.slots[(pos) as usize].cells[0].val =
                    ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                        + Float::with_val(PRECISION_BITS, val));
                psr.param.slots[(pos) as usize].cells[0].err = Float::with_val(PRECISION_BITS, err);
            }
        } else {
            if (pos == (param_om as i32)) {
                {
                    psr.param.slots[(pos) as usize].cells[0].val =
                        ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                            + Float::with_val(
                                PRECISION_BITS,
                                ((val * 180.0f64) / std::f64::consts::PI),
                            ));
                    psr.param.slots[(pos) as usize].cells[0].err =
                        Float::with_val(PRECISION_BITS, ((err * 180.0f64) / std::f64::consts::PI));
                }
            } else {
                if (pos == (param_pbdot as i32)) {
                    {
                        psr.param.slots[(pos) as usize].cells[0].val =
                            ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                                + Float::with_val(PRECISION_BITS, val));
                        psr.param.slots[(pos) as usize].cells[0].err =
                            Float::with_val(PRECISION_BITS, err);
                    }
                } else {
                    if (pos == (param_omdot as i32)) {
                        {
                            psr.param.slots[(pos) as usize].cells[0].val =
                                ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                                    + Float::with_val(
                                        PRECISION_BITS,
                                        (((val * (86400.0f64 * 365.25f64)) * 180.0f64)
                                            / std::f64::consts::PI),
                                    ));
                            psr.param.slots[(pos) as usize].cells[0].err = Float::with_val(
                                PRECISION_BITS,
                                (((err * (86400.0f64 * 365.25f64)) * 180.0f64)
                                    / std::f64::consts::PI),
                            );
                        }
                    } else {
                        if (pos == (param_a1dot as i32)) {
                            {
                                psr.param.slots[(pos) as usize].cells[0].val =
                                    ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                                        + Float::with_val(PRECISION_BITS, val));
                                psr.param.slots[(pos) as usize].cells[0].err =
                                    Float::with_val(PRECISION_BITS, err);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
