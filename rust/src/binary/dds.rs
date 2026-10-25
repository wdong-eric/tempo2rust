//! DDSmodel.C model, derivatives, updates and private helpers.
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
/// DDSmodel.C::DDSmodel (pinned source line 39).
pub(super) fn DDSmodel(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut ipos: i32,
    mut param: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut an: f64;
    let mut pb: f64;
    let mut k: f64;
    let mut rad2deg: f64 = (180.0f64 / std::f64::consts::PI);
    let mut SUNMASS: f64 = 4.925490947e-6f64;
    let mut m2: f64;
    let mut tt0: f64;
    let mut t0: f64;
    let mut x: f64;
    let mut ecc: f64;
    let mut er: f64;
    let mut xdot: f64;
    let mut edot: f64;
    let mut dr: f64;
    let mut dth: f64;
    let mut eth: f64;
    let mut am2: f64;
    let mut ct: f64;
    let mut pbdot: f64;
    let mut xpbdot: f64;
    let mut phase: f64;
    let mut u: f64;
    let mut du: f64;
    let mut gamma: f64;
    let mut orbits: f64;
    let mut norbits: i32;
    let mut cu: f64;
    let mut onemecu: f64;
    let mut cae: f64;
    let mut sae: f64;
    let mut ae: f64;
    let mut omega: f64;
    let mut omz: f64;
    let mut sw: f64;
    let mut cw: f64;
    let mut alpha: f64;
    let mut beta: f64;
    let mut bg: f64;
    let mut dre: f64;
    let mut drep: f64;
    let mut drepp: f64;
    let mut anhat: f64;
    let mut su: f64;
    let mut sqr1me2: f64;
    let mut cume: f64;
    let mut brace: f64;
    let mut dlogbr: f64;
    let mut ds: f64;
    let mut da: f64;
    let mut a0: f64;
    let mut b0: f64;
    let mut d2bar: f64;
    let mut torb: f64;
    let mut csigma: f64;
    let mut ce: f64;
    let mut cx: f64;
    let mut comega: f64;
    let mut cgamma: f64;
    let mut cm2: f64;
    let mut csi: f64 = 0.0f64;
    let mut shapmax: f64;
    let mut cshapmax: f64;
    let mut sdds: f64;
    d.version("DDSmodel.C", "DDSmodel()");
    dr = 0.0f64;
    dth = 0.0f64;
    if (psr.param.slots[param_shapmax as usize].cells[0].param_set == 1i32) {
        shapmax = ((psr.param.slots[param_shapmax as usize].cells[0].val).clone()).to_f64();
    } else {
        shapmax = 0.0f64;
    }
    if (psr.param.slots[param_m2 as usize].cells[0].param_set == 1i32) {
        am2 = ((psr.param.slots[param_m2 as usize].cells[0].val).clone()).to_f64();
    } else {
        am2 = 0.0f64;
    }
    pb = ((psr.param.slots[param_pb as usize].cells[0].val).clone()
        * Float::with_val(PRECISION_BITS, 86400.0f64))
    .to_f64();
    an = ((2.0f64 * std::f64::consts::PI) / pb);
    k = ((psr.param.slots[param_omdot as usize].cells[0].val).clone()
        / Float::with_val(PRECISION_BITS, (((rad2deg * 365.25f64) * 86400.0f64) * an)))
    .to_f64();
    m2 = (am2 * SUNMASS);
    t0 = ((psr.param.slots[param_t0 as usize].cells[0].val).clone()).to_f64();
    ct = ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()).to_f64();
    tt0 = ((ct - t0) * 86400.0f64);
    if (psr.param.slots[param_gamma as usize].cells[0].param_set == 1i32) {
        gamma = ((psr.param.slots[param_gamma as usize].cells[0].val).clone()).to_f64();
    } else {
        gamma = 0.0f64;
    }
    a0 = 0.0f64;
    b0 = 0.0f64;
    if (psr.param.slots[param_om as usize].cells[0].param_set == 1i32) {
        omz = ((psr.param.slots[param_om as usize].cells[0].val).clone()).to_f64();
    } else {
        omz = 0.0f64;
    }
    if (psr.param.slots[param_a1dot as usize].cells[0].param_set == 1i32) {
        xdot = ((psr.param.slots[param_a1dot as usize].cells[0].val).clone()).to_f64();
    } else {
        xdot = 0.0f64;
    }
    if (psr.param.slots[param_pbdot as usize].cells[0].param_set == 1i32) {
        pbdot = ((psr.param.slots[param_pbdot as usize].cells[0].val).clone()).to_f64();
    } else {
        pbdot = 0.0f64;
    }
    if (psr.param.slots[param_edot as usize].cells[0].param_set == 1i32) {
        edot = ((psr.param.slots[param_edot as usize].cells[0].val).clone()).to_f64();
    } else {
        edot = 0.0f64;
    }
    if (psr.param.slots[param_xpbdot as usize].cells[0].param_set == 1i32) {
        xpbdot = ((psr.param.slots[param_xpbdot as usize].cells[0].val).clone()).to_f64();
    } else {
        xpbdot = 0.0f64;
    }
    x = ((psr.param.slots[param_a1 as usize].cells[0].val).clone()
        + Float::with_val(PRECISION_BITS, (xdot * tt0)))
    .to_f64();
    ecc = ((psr.param.slots[param_ecc as usize].cells[0].val).clone()
        + Float::with_val(PRECISION_BITS, (edot * tt0)))
    .to_f64();
    er = (ecc * (1.0f64 + dr));
    eth = (ecc * (1.0f64 + dth));
    orbits = ((tt0 / pb) - ((0.5f64 * (pbdot + xpbdot)) * (tt0 / pb).powf((2i32) as f64)));
    norbits = source_i32(orbits)?;
    if (orbits < 0.0f64) {
        norbits = norbits
            .checked_sub(1)
            .ok_or_else(|| boundary("binary orbit count overflow"))?;
    }
    phase = ((2.0f64 * std::f64::consts::PI) * (orbits - (norbits) as f64));
    u = (phase + ((ecc * (phase).sin()) * (1.0f64 + (ecc * (phase).cos()))));
    loop {
        {
            du = ((phase - (u - (ecc * (u).sin()))) / (1.0f64 - (ecc * (u).cos())));
            u += du;
        }
        if !((du).abs() > 1.0e-12f64) {
            break;
        }
    }
    su = (u).sin();
    cu = (u).cos();
    onemecu = (1.0f64 - (ecc * cu));
    cae = ((cu - ecc) / onemecu);
    sae = (((1.0f64 - (ecc).powf((2i32) as f64)).sqrt() * su) / onemecu);
    ae = (sae).atan2(cae);
    if (ae < 0.0f64) {
        ae += (2.0f64 * std::f64::consts::PI);
    }
    ae = ((((2.0f64 * std::f64::consts::PI) * orbits) + ae) - phase);
    omega = ((omz / rad2deg) + (k * ae));
    sw = (omega).sin();
    cw = (omega).cos();
    alpha = (x * sw);
    beta = ((x * ((1i32) as f64 - (eth).powf((2i32) as f64)).sqrt()) * cw);
    bg = (beta + gamma);
    dre = ((alpha * (cu - er)) + (bg * su));
    drep = (((-alpha) * su) + (bg * cu));
    drepp = (((-alpha) * cu) - (bg * su));
    anhat = (an / onemecu);
    sqr1me2 = ((1i32) as f64 - (ecc).powf((2i32) as f64)).sqrt();
    cume = (cu - ecc);
    sdds = (1.0f64 - ((-1.0f64) * shapmax).exp());
    brace = (onemecu - (sdds * ((sw * cume) + ((sqr1me2 * cw) * su))));
    dlogbr = (brace).ln();
    ds = (((-2i32) as f64 * m2) * dlogbr);
    da = ((a0 * ((omega + ae).sin() + (ecc * sw))) + (b0 * ((omega + ae).cos() + (ecc * cw))));
    d2bar = (((dre
        * (((1i32) as f64 - (anhat * drep))
            + ((anhat).powf((2i32) as f64)
                * (((drep).powf((2i32) as f64) + ((0.5f64 * dre) * drepp))
                    - (((((0.5f64 * ecc) * su) * dre) * drep) / onemecu)))))
        + ds)
        + da);
    torb = (-d2bar);
    if (param == (-1i32)) {
        return Ok(torb);
    }
    csigma = ((x * (((-sw) * su) + ((sqr1me2 * cw) * cu))) / onemecu);
    ce = (((su * csigma) - (x * sw)) - ((((ecc * x) * cw) * su) / sqr1me2));
    cx = ((sw * cume) + ((sqr1me2 * cw) * su));
    comega = (x * ((cw * cume) - ((sqr1me2 * sw) * su)));
    cgamma = su;
    cm2 = ((-2i32) as f64 * dlogbr);
    cshapmax = (((((2i32) as f64 * m2) * ((sw * cume) + ((sqr1me2 * cw) * su))) / brace)
        * (1.0f64 - sdds));
    if (param == (param_pb as i32)) {
        return Ok(((((-csigma) * an) * 86400.0f64) * tt0) / (pb * 86400.0f64));
    } else {
        if (param == (param_a1 as i32)) {
            return Ok(cx);
        } else {
            if (param == (param_ecc as i32)) {
                return Ok(ce);
            } else {
                if (param == (param_om as i32)) {
                    return Ok(comega);
                } else {
                    if (param == (param_omdot as i32)) {
                        return Ok((ae * comega)
                            / ((((an * 360.0f64) / (2.0f64 * std::f64::consts::PI)) * 365.25f64)
                                * 86400.0f64));
                    } else {
                        if (param == (param_t0 as i32)) {
                            return Ok(((-csigma) * an) * 86400.0f64);
                        } else {
                            if (param == (param_pbdot as i32)) {
                                return Ok((0.5f64 * tt0)
                                    * (((((-csigma) * an) * 86400.0f64) * tt0)
                                        / (pb * 86400.0f64)));
                            } else {
                                if (param == (param_sini as i32)) {
                                    return Ok(csi);
                                } else {
                                    if (param == (param_gamma as i32)) {
                                        return Ok(cgamma);
                                    } else {
                                        if (param == (param_m2 as i32)) {
                                            return Ok(cm2 * SUNMASS);
                                        } else {
                                            if (param == (param_a1dot as i32)) {
                                                return Ok(cx * tt0);
                                            } else {
                                                if (param == (param_shapmax as i32)) {
                                                    return Ok(cshapmax);
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
        }
    }
    Ok(0.0f64)
}

/// DDSmodel.C::updateDDS (pinned source line 201).
pub(super) fn updateDDS(
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
                    + Float::with_val(PRECISION_BITS, (val / 86400.0f64)));
            psr.param.slots[param_pb as usize].cells[0].err =
                Float::with_val(PRECISION_BITS, (err / 86400.0f64));
        }
    } else {
        if (((((((pos == (param_a1 as i32)) || (pos == (param_ecc as i32)))
            || (pos == (param_t0 as i32)))
            || (pos == (param_sini as i32)))
            || (pos == (param_m2 as i32)))
            || (pos == (param_gamma as i32)))
            || (pos == (param_shapmax as i32)))
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
                    if (pos == (param_a1dot as i32)) {
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
