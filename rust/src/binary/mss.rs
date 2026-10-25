//! MSSmodel.C model, derivatives, updates and private helpers.
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
/// MSSmodel.C::MSSmodel (pinned source line 49).
pub(super) fn MSSmodel(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut obs: i32,
    mut param: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut SUNMASS: f64 = 4.925490947e-6f64;
    let mut RAD: f64 = (180.0f64 / std::f64::consts::PI);
    let mut an: f64;
    let mut ecc0: f64;
    let mut x0: f64;
    let mut omega0: f64;
    let mut k: f64;
    let mut xi: f64;
    let mut m2: f64;
    let mut orbits: f64;
    let mut phase: f64;
    let mut ecc: f64;
    let mut er: f64;
    let mut eth: f64;
    let mut du: f64;
    let mut om2dot: f64;
    let mut x2dot: f64;
    let mut si: f64;
    let mut b0: f64;
    let mut a0: f64;
    let mut u: f64;
    let mut su: f64;
    let mut cu: f64;
    let mut onemecu: f64;
    let mut cae: f64;
    let mut sae: f64;
    let mut ae: f64;
    let mut x: f64;
    let mut sw: f64;
    let mut cw: f64;
    let mut alpha: f64;
    let mut beta: f64;
    let mut bg: f64;
    let mut dre: f64;
    let mut drep: f64;
    let mut drepp: f64;
    let mut anhat: f64;
    let mut sqr1me2: f64;
    let mut cume: f64;
    let mut brace: f64;
    let mut dlogbr: f64;
    let mut ds: f64;
    let mut da: f64;
    let mut d2bar: f64;
    let mut torb: f64;
    let mut edot: f64;
    let mut dr: f64;
    let mut dth: f64;
    let mut gamma: f64;
    let mut csigma: f64;
    let mut ce: f64;
    let mut cx: f64;
    let mut comega: f64;
    let mut cm2: f64;
    let mut norbits: i32;
    let mut pb: f64;
    let mut eccentricity: f64;
    let mut a1: f64;
    let mut omega: f64;
    let mut omdot: f64;
    let mut xdot: f64;
    let mut am2: f64;
    let mut pbdot: f64;
    let mut tt0: f64;
    let mut shapmax: f64;
    let mut e2dot: f64;
    let mut orbpx: f64;
    d.version("MSSmodel.C", "MSSmodel()");
    tt0 = (((read(&rows[(obs) as usize].bbat, "psr[p].obsn[obs].bbat")?).clone()
        - (psr.param.slots[param_t0 as usize].cells[0].val).clone())
        * Float::with_val(PRECISION_BITS, 86400.0f64))
    .to_f64();
    if (psr.param.slots[param_pbdot as usize].cells[0].param_set == 1i32) {
        pbdot = ((psr.param.slots[param_pbdot as usize].cells[0].val).clone()).to_f64();
    } else {
        pbdot = 0.0f64;
    }
    if (psr.param.slots[param_m2 as usize].cells[0].param_set == 1i32) {
        am2 = ((psr.param.slots[param_m2 as usize].cells[0].val).clone()).to_f64();
    } else {
        am2 = 0.0f64;
    }
    if (psr.param.slots[param_a1dot as usize].cells[0].param_set == 1i32) {
        xdot = ((psr.param.slots[param_a1dot as usize].cells[0].val).clone()).to_f64();
    } else {
        xdot = 0.0f64;
    }
    if (psr.param.slots[param_a2dot as usize].cells[0].param_set == 1i32) {
        x2dot = ((psr.param.slots[param_a2dot as usize].cells[0].val).clone()).to_f64();
    } else {
        x2dot = 0.0f64;
    }
    if (psr.param.slots[param_e2dot as usize].cells[0].param_set == 1i32) {
        e2dot = (Float::with_val(PRECISION_BITS, 1e-20f64)
            * (psr.param.slots[param_e2dot as usize].cells[0].val).clone())
        .to_f64();
    } else {
        e2dot = 0.0f64;
    }
    if (psr.param.slots[param_orbpx as usize].cells[0].param_set == 1i32) {
        orbpx = (Float::with_val(PRECISION_BITS, (1.0f64 / 3.086e21f64))
            * (psr.param.slots[param_orbpx as usize].cells[0].val).clone())
        .to_f64();
    } else {
        orbpx = 0.0f64;
    }
    if (psr.param.slots[param_dtheta as usize].cells[0].param_set == 1i32) {
        dth = ((psr.param.slots[param_dtheta as usize].cells[0].val).clone()).to_f64();
    } else {
        dth = 0.0f64;
    }
    if (psr.param.slots[param_sini as usize].cells[0].param_set == 1i32) {
        si = ((psr.param.slots[param_sini as usize].cells[0].val).clone()).to_f64();
    } else {
        si = 0.0f64;
    }
    if (psr.param.slots[param_a0 as usize].cells[0].param_set == 1i32) {
        a0 = ((psr.param.slots[param_a0 as usize].cells[0].val).clone()).to_f64();
    } else {
        a0 = 0.0f64;
    }
    if (psr.param.slots[param_b0 as usize].cells[0].param_set == 1i32) {
        b0 = ((psr.param.slots[param_b0 as usize].cells[0].val).clone()).to_f64();
    } else {
        b0 = 0.0f64;
    }
    if (psr.param.slots[param_dr as usize].cells[0].param_set == 1i32) {
        dr = ((psr.param.slots[param_dr as usize].cells[0].val).clone()).to_f64();
    } else {
        dr = 0.0f64;
    }
    if (psr.param.slots[param_omdot as usize].cells[0].param_set == 1i32) {
        omdot = ((psr.param.slots[param_omdot as usize].cells[0].val).clone()).to_f64();
    } else {
        omdot = 0.0f64;
    }
    if (psr.param.slots[param_gamma as usize].cells[0].param_set == 1i32) {
        gamma = ((psr.param.slots[param_gamma as usize].cells[0].val).clone()).to_f64();
    } else {
        gamma = 0.0f64;
    }
    if (psr.param.slots[param_om2dot as usize].cells[0].param_set == 1i32) {
        om2dot = ((psr.param.slots[param_om2dot as usize].cells[0].val).clone()).to_f64();
    } else {
        om2dot = 0.0f64;
    }
    if (psr.param.slots[param_edot as usize].cells[0].param_set == 1i32) {
        edot = ((psr.param.slots[param_edot as usize].cells[0].val).clone()).to_f64();
    } else {
        edot = 0.0f64;
    }
    if (psr.param.slots[param_shapmax as usize].cells[0].param_set == 1i32) {
        shapmax = ((psr.param.slots[param_shapmax as usize].cells[0].val).clone()).to_f64();
    } else {
        shapmax = 0.0f64;
    }
    pb = ((psr.param.slots[param_pb as usize].cells[0].val).clone()
        * Float::with_val(PRECISION_BITS, 86400.0f64))
    .to_f64();
    eccentricity = ((psr.param.slots[param_ecc as usize].cells[0].val).clone()).to_f64();
    a1 = ((psr.param.slots[param_a1 as usize].cells[0].val).clone()).to_f64();
    omega = ((psr.param.slots[param_om as usize].cells[0].val).clone()).to_f64();
    an = ((2.0f64 * std::f64::consts::PI) / pb);
    ecc0 = eccentricity;
    x0 = a1;
    omega0 = (omega / RAD);
    k = ((omdot / an) / ((RAD * 365.25f64) * 86400.0f64));
    xi = (xdot / an);
    m2 = (am2 * SUNMASS);
    tt0 = ((((read(&rows[(obs) as usize].bbat, "psr[p].obsn[obs].bbat")?).clone()).to_f64()
        - ((psr.param.slots[param_t0 as usize].cells[0].val).clone()).to_f64())
        * 86400.0f64);
    orbits = ((tt0 / pb) - ((0.5f64 * pbdot) * (tt0 / pb).powf((2i32) as f64)));
    norbits = source_i32(orbits)?;
    if (orbits < 0.0f64) {
        norbits = norbits
            .checked_sub(1)
            .ok_or_else(|| boundary("binary orbit count overflow"))?;
    }
    phase = ((2.0f64 * std::f64::consts::PI) * (orbits - (norbits) as f64));
    ecc = ((ecc0 + (edot * tt0)) + (((0.5f64 * e2dot) * tt0) * tt0));
    er = (ecc * (1.0f64 + dr));
    eth = (ecc * (1.0f64 + dth));
    u = (phase + ((ecc * (phase).sin()) * ((1i32) as f64 + (ecc * (phase).cos()))));
    loop {
        {
            du = ((phase - (u - (ecc * (u).sin()))) / (1.0f64 - (ecc * (u).cos())));
            u += du;
        }
        if !((du).abs() > 1.0e-14f64) {
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
    let mut xii: f64 = (((1e-20f64 * x2dot) / an) / an);
    let mut oii: f64 = (((1e-20f64 * om2dot) / an) / an);
    x = ((x0 + (xi * ae)) + ((0.5f64 * xii) * (ae).powf((2i32) as f64)));
    omega = ((omega0 + (k * ae)) + ((0.5f64 * oii) * (ae).powf(2.0f64)));
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
    brace = (onemecu - (si * ((sw * cume) + ((sqr1me2 * cw) * su))));
    dlogbr = (brace).ln();
    ds = (((-2.0f64) * m2) * dlogbr);
    da = ((a0 * ((omega + ae).sin() + (ecc * sw))) + (b0 * ((omega + ae).cos() + (ecc * cw))));
    d2bar = (((dre
        * (((1i32) as f64 - (anhat * drep))
            + ((anhat).powf((2i32) as f64)
                * (((drep).powf((2i32) as f64) + ((0.5f64 * dre) * drepp))
                    - (((((0.5f64 * ecc) * su) * dre) * drep) / onemecu)))))
        + ds)
        + da);
    torb = (-d2bar);
    let mut shapparam: f64;
    shapparam = (-(((1i32) as f64 - (ecc * cu))
        - ((((omega).sin() * (cu - ecc))
            + ((((1i32) as f64 - (ecc * ecc)).sqrt() * (omega).cos()) * su))
            * si))
        .ln());
    torb -= (shapmax * shapparam);
    let mut so: f64 = (omega).sin();
    let mut co: f64 = (omega).cos();
    let mut cpx: f64;
    cpx = (((((1e2f64 * 299792458.0f64) * x) * x) / 2.0f64)
        * (((((((1.0f64 / si) / si) - 0.5f64)
            + (((0.5f64 * ecc) * ecc)
                * (((1i32) as f64 + (so * so)) - (((3i32) as f64 / si) / si))))
            - ((((2i32) as f64 * ecc) * (((1.0f64 / si) / si) - (so * so))) * (cu - ecc)))
            + ((((1i32) as f64 - (ecc * ecc)).sqrt() * ((2i32) as f64 * omega).sin())
                * ((ecc * su) - (0.5f64 * ((2i32) as f64 * u).sin()))))
            + ((0.5f64
                * (((2i32) as f64 * omega).cos()
                    + ((ecc * ecc) * (((1.0f64 / si) / si) + (co * co)))))
                * ((2i32) as f64 * u).cos())));
    if (orbpx != (0i32) as f64) {
        {
            torb -= (cpx * orbpx);
        }
    }
    if (param == (-1i32)) {
        return Ok(torb);
    }
    csigma = ((x * (((-sw) * su) + ((sqr1me2 * cw) * cu))) / onemecu);
    ce = (((su * csigma) - (x * sw)) - ((((ecc * x) * cw) * su) / sqr1me2));
    cx = ((sw * cume) + ((sqr1me2 * cw) * su));
    comega = (x * ((cw * cume) - ((sqr1me2 * sw) * su)));
    cm2 = ((-2i32) as f64 * dlogbr);
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
                    return Ok((comega * ae) / an);
                } else {
                    if (param == (param_pb as i32)) {
                        return Ok(((((-csigma) * an) * 86400.0f64) * tt0) / (pb * 86400.0f64));
                    } else {
                        if (param == (param_t0 as i32)) {
                            return Ok(((-csigma) * an) * 86400.0f64);
                        } else {
                            if (param == (param_a1dot as i32)) {
                                return Ok((cx * ae) / an);
                            } else {
                                if (param == (param_pbdot as i32)) {
                                    return Ok((0.5f64 * tt0)
                                        * (((((-csigma) * an) * 86400.0f64) * tt0)
                                            / (pb * 86400.0f64)));
                                } else {
                                    if (param == (param_a2dot as i32)) {
                                        return Ok(
                                            (((((1e-20f64 * 0.5f64) * cx) * ae) / an) * ae) / an
                                        );
                                    } else {
                                        if (param == (param_om2dot as i32)) {
                                            return Ok((((((1e-20f64 * 0.5f64) * comega) * ae)
                                                / an)
                                                * ae)
                                                / an);
                                        } else {
                                            if (param == (param_shapmax as i32)) {
                                                return Ok(shapparam);
                                            } else {
                                                if (param == (param_m2 as i32)) {
                                                    return Ok(cm2 * SUNMASS);
                                                } else {
                                                    if (param == (param_edot as i32)) {
                                                        return Ok(ce * tt0);
                                                    } else {
                                                        if (param == (param_e2dot as i32)) {
                                                            return Ok((((1e-20f64 * 0.5f64)
                                                                * ce)
                                                                * tt0)
                                                                * tt0);
                                                        } else {
                                                            if (param == (param_orbpx as i32)) {
                                                                return Ok(
                                                                    (1.0f64 / 3.086e21f64) * cpx
                                                                );
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
            }
        }
    }
    Ok(0.0f64)
}

/// MSSmodel.C::updateMSS (pinned source line 275).
pub(super) fn updateMSS(
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
        if (((pos == (param_a1 as i32)) || (pos == (param_ecc as i32)))
            || (pos == (param_t0 as i32)))
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
                                        ((((val * (180i32) as f64) / std::f64::consts::PI)
                                            * 365.25f64)
                                            * 86400.0f64),
                                    ));
                            psr.param.slots[(pos) as usize].cells[0].err = Float::with_val(
                                PRECISION_BITS,
                                ((((err * (180i32) as f64) / std::f64::consts::PI) * 365.25f64)
                                    * 86400.0f64),
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
                        } else {
                            if (pos == (param_a2dot as i32)) {
                                {
                                    psr.param.slots[(pos) as usize].cells[0].val =
                                        ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                                            + Float::with_val(PRECISION_BITS, val));
                                    psr.param.slots[(pos) as usize].cells[0].err =
                                        Float::with_val(PRECISION_BITS, err);
                                }
                            } else {
                                if (pos == (param_om2dot as i32)) {
                                    {
                                        psr.param.slots[(pos) as usize].cells[0].val =
                                            ((psr.param.slots[(pos) as usize].cells[0].val)
                                                .clone()
                                                + Float::with_val(PRECISION_BITS, val));
                                        psr.param.slots[(pos) as usize].cells[0].err =
                                            Float::with_val(PRECISION_BITS, err);
                                    }
                                } else {
                                    if (pos == (param_shapmax as i32)) {
                                        {
                                            psr.param.slots[(pos) as usize].cells[0].val =
                                                ((psr.param.slots[(pos) as usize].cells[0].val)
                                                    .clone()
                                                    + Float::with_val(PRECISION_BITS, val));
                                            psr.param.slots[(pos) as usize].cells[0].err =
                                                Float::with_val(PRECISION_BITS, err);
                                        }
                                    } else {
                                        if (pos == (param_m2 as i32)) {
                                            {
                                                psr.param.slots[(pos) as usize].cells[0].val =
                                                    ((psr.param.slots[(pos) as usize].cells[0]
                                                        .val)
                                                        .clone()
                                                        + Float::with_val(PRECISION_BITS, val));
                                                psr.param.slots[(pos) as usize].cells[0].err =
                                                    Float::with_val(PRECISION_BITS, err);
                                            }
                                        } else {
                                            if (pos == (param_sini as i32)) {
                                                {
                                                    psr.param.slots[(pos) as usize].cells[0].val =
                                                        ((psr.param.slots[(pos) as usize].cells
                                                            [0]
                                                        .val)
                                                            .clone()
                                                            + Float::with_val(PRECISION_BITS, val));
                                                    psr.param.slots[(pos) as usize].cells[0].err =
                                                        Float::with_val(PRECISION_BITS, err);
                                                }
                                            } else {
                                                if (pos == (param_edot as i32)) {
                                                    {
                                                        psr.param.slots[(pos) as usize].cells[0]
                                                            .val = ((psr.param.slots
                                                            [(pos) as usize]
                                                            .cells[0]
                                                            .val)
                                                            .clone()
                                                            + Float::with_val(PRECISION_BITS, val));
                                                        psr.param.slots[(pos) as usize].cells[0]
                                                            .err =
                                                            Float::with_val(PRECISION_BITS, err);
                                                    }
                                                } else {
                                                    if (pos == (param_e2dot as i32)) {
                                                        {
                                                            psr.param.slots[(pos) as usize].cells
                                                                [0]
                                                            .val = ((psr.param.slots
                                                                [(pos) as usize]
                                                                .cells[0]
                                                                .val)
                                                                .clone()
                                                                + Float::with_val(
                                                                    PRECISION_BITS,
                                                                    val,
                                                                ));
                                                            psr.param.slots[(pos) as usize].cells
                                                                [0]
                                                            .err = Float::with_val(
                                                                PRECISION_BITS,
                                                                err,
                                                            );
                                                        }
                                                    } else {
                                                        if (pos == (param_orbpx as i32)) {
                                                            {
                                                                psr.param.slots[(pos) as usize]
                                                                    .cells[0]
                                                                    .val = ((psr.param.slots
                                                                    [(pos) as usize]
                                                                    .cells[0]
                                                                    .val)
                                                                    .clone()
                                                                    + Float::with_val(
                                                                        PRECISION_BITS,
                                                                        val,
                                                                    ));
                                                                psr.param.slots[(pos) as usize]
                                                                    .cells[0]
                                                                    .err = Float::with_val(
                                                                    PRECISION_BITS,
                                                                    err,
                                                                );
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
            }
        }
    }
    Ok(())
}
