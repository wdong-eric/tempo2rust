//! ELL1kmodel.C model, derivatives, updates and private helpers.
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
/// ELL1kmodel.C::ELL1kmodel (pinned source line 59).
pub(super) fn ELL1kmodel(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut ipos: i32,
    mut param: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut an: f64;
    let mut x0: f64;
    let mut m2: f64;
    let mut tt0: f64;
    let mut orbits: f64;
    let mut phase: f64;
    let mut e1: f64;
    let mut e2: f64;
    let mut dre: f64;
    let mut drep: f64;
    let mut drepp: f64;
    let mut brace: f64;
    let mut dlogbr: f64;
    let mut ds: f64;
    let mut da: f64;
    let mut pb: f64;
    let mut omdot: f64;
    let mut eps1: f64;
    let mut eps2: f64;
    let mut eps1dot: f64;
    let mut eps2dot: f64;
    let mut si: f64;
    let mut a0: f64;
    let mut b0: f64;
    let mut d2bar: f64;
    let mut torb: f64;
    let mut Csigma: f64;
    let mut Cx: f64;
    let mut Ceps1: f64;
    let mut Ceps2: f64;
    let mut Cm2: f64;
    let mut Csi: f64;
    let mut ct: f64;
    let mut t0asc: f64;
    let mut pbdot: f64;
    let mut xpbdot: f64;
    let mut x: f64;
    let mut xdot: f64;
    let mut am2: f64;
    let mut Comdot: f64;
    let mut norbits: i32;
    let mut SUNMASS: f64 = 4.925490947e-6f64;
    d.version("ELL1kmodel.C", "ELL1kmodel()");
    a0 = 0.0f64;
    b0 = 0.0f64;
    pb = ((psr.param.slots[param_pb as usize].cells[0].val).clone()
        * Float::with_val(PRECISION_BITS, 86400.0f64))
    .to_f64();
    if (psr.param.slots[param_pbdot as usize].cells[0].param_set == 1i32) {
        pbdot = ((psr.param.slots[param_pbdot as usize].cells[0].val).clone()).to_f64();
    } else {
        pbdot = 0.0f64;
    }
    if (psr.param.slots[param_omdot as usize].cells[0].param_set == 1i32) {
        omdot = (((psr.param.slots[param_omdot as usize].cells[0].val).clone()
            * Float::with_val(PRECISION_BITS, std::f64::consts::PI))
            / Float::with_val(PRECISION_BITS, (((180i32) as f64 * 86400.0f64) * 365.25f64)))
        .to_f64();
    } else {
        omdot = 0.0f64;
    }
    let mut nb: f64 = ((2.0f64 * std::f64::consts::PI) / pb);
    an = (((2.0f64 * std::f64::consts::PI) / pb) - omdot);
    if (psr.param.slots[param_sini as usize].cells[0].param_set == 1i32) {
        si = (crate::tempo2_util::get_parameter_value_inner(
            psr,
            param_sini as usize,
            (0i32) as usize,
            d,
        )?)
        .to_f64();
    } else {
        si = 0.0f64;
    }
    if (si > 1.0f64) {
        {
            d.display(
                1i32,
                "BIN1".as_bytes(),
                "SIN I > 1.0, setting to 1: should probably use DDS model".as_bytes(),
                "".as_bytes(),
                *read(&psr.noWarnings, "psr[p].noWarnings")?,
            )?;
            si = 1.0f64;
            psr.param.slots[param_sini as usize].cells[0].val =
                Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap());
        }
    }
    x0 = ((psr.param.slots[param_a1 as usize].cells[0].val).clone()).to_f64();
    if (psr.param.slots[param_a1dot as usize].cells[0].param_set == 1i32) {
        xdot = ((psr.param.slots[param_a1dot as usize].cells[0].val).clone()).to_f64();
    } else {
        xdot = 0.0f64;
    }
    t0asc = ((psr.param.slots[param_tasc as usize].cells[0].val).clone()).to_f64();
    if (psr.param.slots[param_m2 as usize].cells[0].param_set == 1i32) {
        am2 = ((psr.param.slots[param_m2 as usize].cells[0].val).clone()).to_f64();
    } else {
        am2 = 0.0f64;
    }
    m2 = (am2 * SUNMASS);
    xpbdot = 0.0f64;
    eps1 = ((psr.param.slots[param_eps1 as usize].cells[0].val).clone()).to_f64();
    eps2 = ((psr.param.slots[param_eps2 as usize].cells[0].val).clone()).to_f64();
    if (psr.param.slots[param_eps1dot as usize].cells[0].param_set == 1i32) {
        eps1dot = ((psr.param.slots[param_eps1dot as usize].cells[0].val).clone()).to_f64();
    } else {
        eps1dot = (0i32) as f64;
    }
    if (psr.param.slots[param_eps2dot as usize].cells[0].param_set == 1i32) {
        eps2dot = ((psr.param.slots[param_eps2dot as usize].cells[0].val).clone()).to_f64();
    } else {
        eps2dot = (0i32) as f64;
    }
    eps1dot = (eps2 * omdot);
    eps2dot = ((-eps1) * omdot);
    ct = ((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()).to_f64();
    tt0 = ((ct - t0asc) * 86400.0f64);
    orbits = ((tt0 / pb) - ((0.5f64 * (pbdot + xpbdot)) * (tt0 / pb).powf((2i32) as f64)));
    norbits = source_i32(orbits)?;
    if (orbits < 0.0f64) {
        norbits = norbits
            .checked_sub(1)
            .ok_or_else(|| boundary("binary orbit count overflow"))?;
    }
    phase = ((2.0f64 * std::f64::consts::PI) * (orbits - (norbits) as f64));
    x = (x0 + (xdot * tt0));
    e1 = ((eps1 * (omdot * tt0).cos()) + (eps2 * (omdot * tt0).sin()));
    e2 = ((eps2 * (omdot * tt0).cos()) - (eps1 * (omdot * tt0).sin()));
    dre = (x
        * ((phase).sin()
            - (0.5f64
                * ((e1 * ((2.0f64 * phase).cos() + (3i32) as f64))
                    - (e2 * (2.0f64 * phase).sin())))));
    drep = (x * (phase).cos());
    drepp = ((-x) * (phase).sin());
    brace = ((1i32) as f64 - (si * (phase).sin()));
    dlogbr = (brace).ln();
    ds = (((-2i32) as f64 * m2) * dlogbr);
    da = ((a0 * (phase).sin()) + (b0 * (phase).cos()));
    d2bar = (((dre
        * ((((1i32) as f64 - (an * drep)) + (an * drep).powf((2i32) as f64))
            + (((0.5f64 * (an).powf((2i32) as f64)) * dre) * drepp)))
        + ds)
        + da);
    torb = (-d2bar);
    if (param == (-1i32)) {
        return Ok(torb);
    }
    Csigma = (x * (phase).cos());
    Cx = (phase).sin();
    Ceps1 = (((-0.5f64) * x)
        * (((((2i32) as f64 * phase).cos() + (3i32) as f64) * (omdot * tt0).cos())
            + (((2i32) as f64 * phase).sin() * (omdot * tt0).sin())));
    Ceps2 = (((-0.5f64) * x)
        * (((((2i32) as f64 * phase).cos() + (3i32) as f64) * (omdot * tt0).sin())
            - (((2i32) as f64 * phase).sin() * (omdot * tt0).cos())));
    Cm2 = ((-2i32) as f64 * dlogbr);
    Csi = ((((2i32) as f64 * m2) * (phase).sin()) / brace);
    Comdot = ((((-0.5f64) * x)
        * (((((2i32) as f64 * phase).cos() + (3i32) as f64) * e2)
            + (((2i32) as f64 * phase).sin() * e1)))
        * tt0);
    if (param == (param_pb as i32)) {
        return Ok(((((-Csigma) * nb) * 86400.0f64) * tt0) / (pb * 86400.0f64));
    } else {
        if (param == (param_a1 as i32)) {
            return Ok(Cx);
        } else {
            if (param == (param_eps1 as i32)) {
                return Ok(Ceps1);
            } else {
                if (param == (param_tasc as i32)) {
                    return Ok(((-Csigma) * nb) * 86400.0f64);
                } else {
                    if (param == (param_eps2 as i32)) {
                        return Ok(Ceps2);
                    } else {
                        if (param == (param_eps1dot as i32)) {
                            return Ok(Ceps1 * tt0);
                        } else {
                            if (param == (param_eps2dot as i32)) {
                                return Ok(Ceps2 * tt0);
                            } else {
                                if (param == (param_pbdot as i32)) {
                                    return Ok((0.5f64 * tt0)
                                        * (((((-Csigma) * nb) * 86400.0f64) * tt0)
                                            / (pb * 86400.0f64)));
                                } else {
                                    if (param == (param_a1dot as i32)) {
                                        return Ok(Cx * tt0);
                                    } else {
                                        if (param == (param_sini as i32)) {
                                            return Ok(Csi);
                                        } else {
                                            if (param == (param_m2 as i32)) {
                                                return Ok(Cm2 * SUNMASS);
                                            } else {
                                                if (param == (param_omdot as i32)) {
                                                    return Ok(Comdot);
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

/// ELL1kmodel.C::updateELL1k (pinned source line 189).
pub(super) fn updateELL1k(
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
        if (((((((((pos == (param_a1 as i32)) || (pos == (param_eps1 as i32)))
            || (pos == (param_eps2 as i32)))
            || (pos == (param_tasc as i32)))
            || (pos == (param_sini as i32)))
            || (pos == (param_m2 as i32)))
            || (pos == (param_eps1dot as i32)))
            || (pos == (param_eps2dot as i32)))
            || (pos == (param_a1dot as i32)))
        {
            {
                psr.param.slots[(pos) as usize].cells[0].val =
                    ((psr.param.slots[(pos) as usize].cells[0].val).clone()
                        + Float::with_val(PRECISION_BITS, val));
                psr.param.slots[(pos) as usize].cells[0].err = Float::with_val(PRECISION_BITS, err);
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
                            (((err * (86400.0f64 * 365.25f64)) * 180.0f64) / std::f64::consts::PI),
                        );
                    }
                }
            }
        }
    }
    Ok(())
}
