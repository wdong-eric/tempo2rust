//! DDGRmodel.C model, derivatives, updates and private helpers.
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
/// DDGRmodel.C::DDGRmodel (pinned source line 48).
pub(super) fn DDGRmodel(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut ipos: i32,
    mut param: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut an: f64;
    let mut afac: f64;
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
    let mut f0: f64;
    let mut cm: f64;
    let mut pbdot: Option<f64> = None;
    let mut xpbdot: f64;
    let mut phase: f64;
    let mut u: f64;
    let mut du: f64;
    let mut gamma: Option<f64> = None;
    let mut m: f64;
    let mut m1: f64;
    let mut arr: Option<f64> = None;
    let mut ar: Option<f64> = None;
    let mut xk: Option<f64> = None;
    let mut fac: f64;
    let mut xomdot: f64;
    let mut dtdm2: f64;
    let mut dgmdm2: f64;
    let mut dsidm2: f64;
    let mut dkdm2: f64;
    let mut dthdm2: f64;
    let mut dpbdm2: f64;
    let mut darrdm2: f64;
    let mut dnum: f64;
    let mut denom: f64;
    let mut orbits: f64;
    let mut a0aligned: f64;
    let mut norbits: i32;
    let mut cu: f64;
    let mut onemecu: f64;
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
    let mut si: Option<f64> = None;
    let mut dlogbr: f64;
    let mut ds: f64;
    let mut da: f64;
    let mut a0: f64;
    let mut b0: f64;
    let mut d2bar: f64;
    let mut torb: f64;
    let mut am: f64;
    let mut csigma: f64;
    let mut ce: f64;
    let mut cx: f64;
    let mut comega: f64;
    let mut cgamma: f64;
    let mut cdth: f64;
    let mut cm2: f64;
    let mut csi: f64;
    let mut fact1: f64;
    let mut fact2: f64;
    let mut fact3: f64;
    let mut fact4: f64;
    let mut fact5: f64;
    let mut fact6: f64;
    let mut fact7: f64;
    let mut fact8: f64;
    let mut denumm: f64;
    let mut denomm: f64;
    let mut darrdm: f64;
    let mut ck: f64;
    let mut dkdm: f64;
    let mut cdr: f64;
    let mut ddrdm: f64;
    let mut cpbdot: f64;
    let mut dpbdm: f64;
    let mut csini: f64;
    let mut dsidm: f64;
    let mut an0: f64;
    let mut dgamdm: f64;
    let mut dthdm: f64;
    let mut ddrdm2: f64;
    d.version("DDGRmodel.C", "DDGRmodel()");
    t0 = ((psr.param.slots[param_t0 as usize].cells[0].val).clone()).to_f64();
    ct = ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()).to_f64();
    tt0 = ((ct - t0) * 86400.0f64);
    f0 = ((psr.param.slots[param_f as usize].cells[0].val).clone()).to_f64();
    xomdot = 0.0f64;
    afac = 0.0f64;
    if (psr.param.slots[param_sini as usize].cells[0].param_set == 1i32) {
        si = Some(
            (crate::tempo2_util::get_parameter_value_inner(
                psr,
                param_sini as usize,
                (0i32) as usize,
                d,
            )?)
            .to_f64(),
        );
    } else {
        si = Some(0.0f64);
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
    if (psr.param.slots[param_mtot as usize].cells[0].param_set == 1i32) {
        am = ((psr.param.slots[param_mtot as usize].cells[0].val).clone()).to_f64();
    } else {
        am = 0.0f64;
    }
    m = (am * SUNMASS);
    m2 = (am2 * SUNMASS);
    m1 = (m - m2);
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
        pbdot = Some(((psr.param.slots[param_pbdot as usize].cells[0].val).clone()).to_f64());
    } else {
        pbdot = Some(0.0f64);
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
    mass2dd(
        am, am2, x, ecc, an, &mut arr, &mut ar, &mut xk, &mut si, &mut gamma, &mut pbdot, d,
    )?;
    k = *read(&xk, "xk")?;
    dr = ((((3.0f64 * (m1).powf((2i32) as f64)) + ((6.0f64 * m1) * m2))
        + (2.0f64 * (m2).powf((2i32) as f64)))
        / (*read(&arr, "arr")? * m));
    er = (ecc * (1.0f64 + dr));
    dth = (((((3.5f64 * m1) * m1) + (((6i32) as f64 * m1) * m2)) + (((2i32) as f64 * m2) * m2))
        / (*read(&arr, "arr")? * m));
    eth = (ecc * (1.0f64 + dth));
    orbits = ((tt0 / pb)
        - ((0.5f64 * (*read(&pbdot, "pbdot")? + xpbdot)) * (tt0 / pb).powf((2i32) as f64)));
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
            fac = (1.0f64 / (1.0f64 - (ecc * (u).cos())));
            du = ((phase - (u - (ecc * (u).sin()))) * fac);
            u += du;
        }
        if !((du).abs() > 1.0e-14f64) {
            break;
        }
    }
    ae = (2.0f64
        * ((((1i32) as f64 + ecc) / ((1i32) as f64 - ecc)).sqrt() * (0.5f64 * u).tan()).atan());
    if (ae < 0.0f64) {
        ae += (2.0f64 * std::f64::consts::PI);
    }
    ae = ((((2.0f64 * std::f64::consts::PI) * orbits) + ae) - phase);
    omega = ((omz / rad2deg) + ((k + (xomdot / (((an * rad2deg) * 365.25f64) * 86400.0f64))) * ae));
    su = (u).sin();
    cu = (u).cos();
    sw = (omega).sin();
    cw = (omega).cos();
    alpha = (x * sw);
    beta = ((x * ((1i32) as f64 - (eth).powf((2i32) as f64)).sqrt()) * cw);
    bg = (beta + *read(&gamma, "gamma")?);
    dre = ((alpha * (cu - er)) + (bg * su));
    drep = (((-alpha) * su) + (bg * cu));
    drepp = (((-alpha) * cu) - (bg * su));
    onemecu = (1.0f64 - (ecc * cu));
    anhat = (an / onemecu);
    cume = (cu - ecc);
    sqr1me2 = ((1i32) as f64 - (ecc).powf((2i32) as f64)).sqrt();
    brace = (onemecu - (*read(&si, "si")? * ((sw * cume) + ((sqr1me2 * cw) * su))));
    if (brace <= (0i32) as f64) {
        {
            d.stdout(b"ERROR: In DDGR model, brace < 0\n");
            return Err(boundary("DDGRmodel.C::DDGRmodel: source exit"));
        }
    }
    dlogbr = (brace).ln();
    ds = (((-2i32) as f64 * m2) * dlogbr);
    a0aligned = ((an * *read(&ar, "ar")?)
        / ((((2.0f64 * std::f64::consts::PI) * f0) * *read(&si, "si")?) * sqr1me2));
    a0 = (afac * a0aligned);
    b0 = 0.0f64;
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
    an0 = (m / (*read(&arr, "arr")?).powf((3i32) as f64)).sqrt();
    csigma = ((x * (((-sw) * su) + ((sqr1me2 * cw) * cu))) / onemecu);
    ce = (((su * csigma) - (x * sw)) - ((((ecc * x) * cw) * su) / sqr1me2));
    cx = ((sw * cume) + ((sqr1me2 * cw) * su));
    comega = (x * ((cw * cume) - ((sqr1me2 * sw) * su)));
    cgamma = su;
    cm2 = ((-2i32) as f64 * dlogbr);
    csi = ((((2i32) as f64 * m2) * ((sw * cume) + ((sqr1me2 * cw) * su))) / brace);
    fact1 = ((m / ((2i32) as f64 * *read(&arr, "arr")?))
        * ((((m - m2) * m2) / (m).powf((2i32) as f64)) - (9i32) as f64));
    fact2 = ((((3i32) as f64 * m) / ((2i32) as f64 * (*read(&arr, "arr")?).powf((4i32) as f64)))
        * (1.0f64 + fact1));
    fact3 = (((m / (2i32) as f64) * *read(&arr, "arr")?)
        * ((m2 / (m).powf((2i32) as f64))
            - ((((2i32) as f64 * (m - m2)) * m2) / (m).powf((3i32) as f64))));
    fact4 = (((((1i32) as f64 + fact1) * (3i32) as f64) * m)
        / (((2i32) as f64 * (*read(&arr, "arr")?).powf((4i32) as f64)) * an0));
    fact5 = ((an0 * fact1) / *read(&arr, "arr")?);
    denumm = ((((1i32) as f64 + fact1)
        / (((2i32) as f64 * (*read(&arr, "arr")?).powf((3i32) as f64)) * an0))
        + (an0 * ((fact1 / m) + fact3)));
    denomm = (fact4 + fact5);
    darrdm = (denumm / denomm);
    dnum = ((an0 * (m - ((2i32) as f64 * m2))) / (((2i32) as f64 * *read(&arr, "arr")?) * m));
    denom = (((an0 * fact1) / *read(&arr, "arr")?) + (fact2 / an0));
    darrdm2 = (dnum / denom);
    dgmdm2 = (((((m + ((2i32) as f64 * m2)) / *read(&arr, "arr")?)
        - (((m2 * (m + m2)) * darrdm2) / (*read(&arr, "arr")?).powf((2i32) as f64)))
        * ecc)
        / (an * m));
    cdth = ((((((-ecc) * ecc) * x) * cw) * su) / sqr1me2);
    dthdm2 = ((((-dth) * darrdm2) / *read(&arr, "arr")?) - ((m + m2) / (*read(&arr, "arr")? * m)));
    dkdm = ((k / m) - ((k * darrdm) / *read(&arr, "arr")?));
    dsidm2 = ((-((m * x) / (*read(&arr, "arr")? * m2)))
        * ((1.0f64 / m2) + (darrdm2 / *read(&arr, "arr")?)));
    ck = (ae * comega);
    dkdm2 = (((-k) * darrdm2) / *read(&arr, "arr")?);
    cdr = (((-ecc) * x) * sw);
    ddrdm2 = ((((-dr) * darrdm2) / *read(&arr, "arr")?)
        - (((2i32) as f64 * m2) / (*read(&arr, "arr")? * m)));
    dtdm2 = ((-2i32) as f64 * dlogbr);
    csini = ((((2i32) as f64 * m2) * ((sw * cume) + ((sqr1me2 * cw) * su))) / brace);
    dsidm = ((-((m * x) / (*read(&arr, "arr")? * m2)))
        * (((-1.0f64) / m) + (darrdm / *read(&arr, "arr")?)));
    dpbdm =
        ((*read(&pbdot, "pbdot")? / (m - m2)) - (*read(&pbdot, "pbdot")? / ((3i32) as f64 * m)));
    cpbdot = ((((-csigma) * an) * (tt0).powf((2i32) as f64)) / ((2i32) as f64 * pb));
    ddrdm = ((((-dr) / m) - ((dr * darrdm) / *read(&arr, "arr")?))
        + ((6i32) as f64 / *read(&arr, "arr")?));
    dpbdm2 = ((*read(&pbdot, "pbdot")? / m2) - (*read(&pbdot, "pbdot")? / (m - m2)));
    cm2 = ((((((dtdm2 + (cgamma * dgmdm2)) + (csini * dsidm2)) + (ck * dkdm2)) + (cdr * ddrdm2))
        + (cdth * dthdm2))
        + (cpbdot * dpbdm2));
    fact6 = (1.0f64 / (*read(&arr, "arr")? * m));
    fact7 = ((-(m + m2)) / (*read(&arr, "arr")? * (m).powf((2i32) as f64)));
    fact8 = (((-(m + m2)) * darrdm) / ((*read(&arr, "arr")?).powf((2i32) as f64) * m));
    dgamdm = (((ecc * m2) / an) * ((fact6 + fact7) + fact8));
    dthdm = ((((-dth) / m) - ((dth * darrdm) / *read(&arr, "arr")?))
        + ((((7i32) as f64 * m) - m2) / (*read(&arr, "arr")? * m)));
    cm = ((((((ck * dkdm) + (cgamma * dgamdm)) + (cdr * ddrdm)) + (cdth * dthdm))
        + (cpbdot * dpbdm))
        + (csini * dsidm));
    if (param == (-2i32)) {
        {
            psr.param.slots[param_sini as usize].cells[0].param_set = 1i32;
            psr.param.slots[param_sini as usize].cells[0].val =
                Float::with_val(PRECISION_BITS, *read(&si, "si")?);
            psr.param.slots[param_omdot as usize].cells[0].param_set = 1i32;
            psr.param.slots[param_omdot as usize].cells[0].val = Float::with_val(
                PRECISION_BITS,
                (((360.0f64 * 365.25f64) * *read(&xk, "xk")?) / (pb / 86400.0f64)),
            );
            psr.param.slots[param_gamma as usize].cells[0].param_set = 1i32;
            psr.param.slots[param_gamma as usize].cells[0].val =
                Float::with_val(PRECISION_BITS, *read(&gamma, "gamma")?);
            psr.param.slots[param_pbdot as usize].cells[0].param_set = 1i32;
            psr.param.slots[param_pbdot as usize].cells[0].val =
                Float::with_val(PRECISION_BITS, *read(&pbdot, "pbdot")?);
            psr.param.slots[param_dtheta as usize].cells[0].param_set = 1i32;
            psr.param.slots[param_dtheta as usize].cells[0].val =
                Float::with_val(PRECISION_BITS, dth);
            psr.param.slots[param_dr as usize].cells[0].param_set = 1i32;
            psr.param.slots[param_dr as usize].cells[0].val = Float::with_val(PRECISION_BITS, dr);
            return Ok((0i32) as f64);
        }
    }
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
                    if (param == (param_t0 as i32)) {
                        return Ok(((-csigma) * an) * 86400.0f64);
                    } else {
                        if (param == (param_pbdot as i32)) {
                            return Ok((0.5f64 * tt0)
                                * (((((-csigma) * an) * 86400.0f64) * tt0) / (pb * 86400.0f64)));
                        } else {
                            if (param == (param_xpbdot as i32)) {
                                return Ok((0.5f64 * tt0)
                                    * (((((-csigma) * an) * 86400.0f64) * tt0)
                                        / (pb * 86400.0f64)));
                            } else {
                                if (param == (param_sini as i32)) {
                                    return Ok(csi);
                                } else {
                                    if (param == (param_m2 as i32)) {
                                        return Ok(cm2 * SUNMASS);
                                    } else {
                                        if (param == (param_mtot as i32)) {
                                            return Ok(cm * SUNMASS);
                                        } else {
                                            if (param == (param_a1dot as i32)) {
                                                return Ok(cx * tt0);
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

/// DDGRmodel.C::updateDDGR (pinned source line 298).
pub(super) fn updateDDGR(
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
        if ((((((pos == (param_a1 as i32)) || (pos == (param_ecc as i32)))
            || (pos == (param_t0 as i32)))
            || (pos == (param_sini as i32)))
            || (pos == (param_m2 as i32)))
            || (pos == (param_mtot as i32)))
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
                if ((pos == (param_pbdot as i32)) || (pos == (param_xpbdot as i32))) {
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
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// DDGRmodel.C::mass2dd (pinned source line 336).
pub(super) fn mass2dd(
    mut am: f64,
    mut am2: f64,
    mut x: f64,
    mut ecc: f64,
    mut an: f64,
    mut arr: &mut Option<f64>,
    mut ar: &mut Option<f64>,
    mut xk: &mut Option<f64>,
    mut si: &mut Option<f64>,
    mut gamma: &mut Option<f64>,
    mut pbdot: &mut Option<f64>,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    let mut SUNMASS: f64 = 4.925490947e-6f64;
    let mut ARRTOL: f64 = 1.0e-10f64;
    let mut m: f64;
    let mut m2: f64;
    let mut m1: f64;
    let mut arr0: f64;
    let mut arrold: f64;
    m = (am * SUNMASS);
    m2 = (am2 * SUNMASS);
    m1 = (m - m2);
    if (m < (0i32) as f64) {
        {
            d.stdout(b"ERROR: problem in DDGR model (mtot < 0)\n");
            return Err(boundary("DDGRmodel.C::mass2dd: source exit"));
        }
    }
    arr0 = (m / (an * an)).powf(1.0f64 / 3.0f64);
    *arr = Some(arr0);
    loop {
        {
            arrold = *read(&*arr, "*arr")?;
            *arr = Some(
                arr0 * (1.0f64
                    + ((((((m1 * m2) / (m).powf((2i32) as f64)) - 9.0f64) * 0.5f64) * m)
                        / *read(&*arr, "*arr")?))
                .powf(2.0f64 / 3.0f64),
            );
        }
        if !(((*read(&*arr, "*arr")? - arrold) / *read(&*arr, "*arr")?).abs() > ARRTOL) {
            break;
        }
    }
    *arr = Some(
        arr0 * (1.0f64
            + ((((((m1 * m2) / (m).powf((2i32) as f64)) - 9.0f64) * 0.5f64) * m)
                / *read(&*arr, "*arr")?))
        .powf(2.0f64 / 3.0f64),
    );
    *ar = Some((*read(&*arr, "*arr")? * m2) / m);
    *si = Some(x / *read(&*ar, "*ar")?);
    *xk = Some((3.0f64 * m) / (arr0 * (1.0f64 - (ecc * ecc))));
    *gamma = Some(((ecc * m2) * (m1 + ((2i32) as f64 * m2))) / ((an * arr0) * m));
    *pbdot = Some(
        ((((((-(((96.0f64 * 2.0f64) * std::f64::consts::PI) / 5.0f64))
            * (an).powf(5.0f64 / 3.0f64))
            * (1.0f64 - (ecc).powf((2i32) as f64)).powf(-3.5f64))
            * (((1i32) as f64 + ((73.0f64 / (24i32) as f64) * (ecc).powf((2i32) as f64)))
                + ((37.0f64 / (96i32) as f64) * (ecc).powf((4i32) as f64))))
            * m1)
            * m2)
            * (m).powf((-1.0f64) / 3.0f64),
    );
    Ok(())
}
