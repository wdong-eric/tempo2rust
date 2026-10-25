//! DDHmodel.C model, derivatives, updates and private helpers.
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
/// DDHmodel.C::DDHmodel (pinned source line 42).
pub(super) fn DDHmodel(
    psr: &mut SourcePulsar,
    rows: &[SourceObservation],
    mut ipos: i32,
    mut param: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut an: Float;
    let mut pb: Float;
    let mut k: Float;
    let mut rad2deg: Float = Float::with_val(PRECISION_BITS, (180.0f64 / std::f64::consts::PI));
    let mut SUNMASS: Float = Float::with_val(PRECISION_BITS, 4.925490947e-6f64);
    let mut m2: Float;
    let mut tt0: Float;
    let mut t0: Float;
    let mut x: Float;
    let mut ecc: Float;
    let mut er: Float;
    let mut xdot: Float;
    let mut edot: Float;
    let mut dr: Float;
    let mut dth: Float;
    let mut eth: Float;
    let mut ct: Float;
    let mut pbdot: Float;
    let mut xpbdot: Float;
    let mut phase: Float;
    let mut u: Float;
    let mut du: Float;
    let mut gamma: Float;
    let mut orbits: Float;
    let mut norbits: i32;
    let mut h3: Float;
    let mut stig: Float;
    let mut cu: Float;
    let mut onemecu: Float;
    let mut cae: Float;
    let mut sae: Float;
    let mut ae: Float;
    let mut omega: Float;
    let mut omz: Float;
    let mut sw: Float;
    let mut cw: Float;
    let mut alpha: Float;
    let mut beta: Float;
    let mut bg: Float;
    let mut dre: Float;
    let mut drep: Float;
    let mut drepp: Float;
    let mut anhat: Float;
    let mut su: Float;
    let mut sqr1me2: Float;
    let mut cume: Float;
    let mut brace: Float;
    let mut si: Float;
    let mut dlogbr: Float;
    let mut ds: Float;
    let mut da: Float;
    let mut a0: Float;
    let mut b0: Float;
    let mut d2bar: Float;
    let mut torb: Float;
    let mut csigma: Float;
    let mut ce: Float;
    let mut cx: Float;
    let mut comega: Float;
    let mut cgamma: Float;
    let mut cm2: Float;
    let mut csi: Float;
    dr = Float::with_val(PRECISION_BITS, 0.0f64);
    dth = Float::with_val(PRECISION_BITS, 0.0f64);
    if ((psr.param.slots[param_h3 as usize].cells[0].param_set
        * psr.param.slots[param_stig as usize].cells[0].param_set)
        != 1i32)
    {
        {
            d.stdout(b"ERROR! For the DDH model, both h3 and stig must be set.\n Exiting.\n");
            return Err(boundary("DDHmodel.C::DDHmodel: source exit"));
        }
    }
    h3 = crate::tempo2_util::get_parameter_value_inner(psr, param_h3 as usize, (0i32) as usize, d)?;
    stig = crate::tempo2_util::get_parameter_value_inner(
        psr,
        param_stig as usize,
        (0i32) as usize,
        d,
    )?;
    si = ((Float::with_val(PRECISION_BITS, 2.0f64) * (stig).clone())
        / (Float::with_val(PRECISION_BITS, 1.0f64)
            + ((stig).clone()).pow(&(Float::with_val(PRECISION_BITS, 2.0f64)))));
    m2 = ((h3).clone() / ((stig).clone()).pow(&(Float::with_val(PRECISION_BITS, 3.0f64))));
    if ((si).clone() > Float::with_val(PRECISION_BITS, 1.0f64)) {
        {
            d.display(
                1i32,
                "BIN1".as_bytes(),
                "SIN I > 1.0, setting to 1: should probably use DDS model".as_bytes(),
                "".as_bytes(),
                *read(&psr.noWarnings, "psr[p].noWarnings")?,
            )?;
            si = Float::with_val(PRECISION_BITS, 1.0f64);
            psr.param.slots[param_sini as usize].cells[0].val =
                Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap());
        }
    }
    pb = ((psr.param.slots[param_pb as usize].cells[0].val).clone()
        * Float::with_val(PRECISION_BITS, 86400.0f64));
    an = (Float::with_val(PRECISION_BITS, (2.0f64 * std::f64::consts::PI)) / (pb).clone());
    k = ((psr.param.slots[param_omdot as usize].cells[0].val).clone()
        / ((((rad2deg).clone() * Float::with_val(PRECISION_BITS, 365.25f64))
            * Float::with_val(PRECISION_BITS, 86400.0f64))
            * (an).clone()));
    t0 = (psr.param.slots[param_t0 as usize].cells[0].val).clone();
    ct = (read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone();
    tt0 = (((ct).clone() - (t0).clone()) * Float::with_val(PRECISION_BITS, 86400.0f64));
    if (psr.param.slots[param_gamma as usize].cells[0].param_set == 1i32) {
        gamma = (psr.param.slots[param_gamma as usize].cells[0].val).clone();
    } else {
        gamma = Float::with_val(PRECISION_BITS, 0.0f64);
    }
    a0 = Float::with_val(PRECISION_BITS, 0.0f64);
    b0 = Float::with_val(PRECISION_BITS, 0.0f64);
    if (psr.param.slots[param_om as usize].cells[0].param_set == 1i32) {
        omz = (psr.param.slots[param_om as usize].cells[0].val).clone();
    } else {
        omz = Float::with_val(PRECISION_BITS, 0.0f64);
    }
    if (psr.param.slots[param_a1dot as usize].cells[0].param_set == 1i32) {
        xdot = (psr.param.slots[param_a1dot as usize].cells[0].val).clone();
    } else {
        xdot = Float::with_val(PRECISION_BITS, 0.0f64);
    }
    if (psr.param.slots[param_pbdot as usize].cells[0].param_set == 1i32) {
        pbdot = (psr.param.slots[param_pbdot as usize].cells[0].val).clone();
    } else {
        pbdot = Float::with_val(PRECISION_BITS, 0.0f64);
    }
    if (psr.param.slots[param_edot as usize].cells[0].param_set == 1i32) {
        edot = (psr.param.slots[param_edot as usize].cells[0].val).clone();
    } else {
        edot = Float::with_val(PRECISION_BITS, 0.0f64);
    }
    if (psr.param.slots[param_xpbdot as usize].cells[0].param_set == 1i32) {
        xpbdot = (psr.param.slots[param_xpbdot as usize].cells[0].val).clone();
    } else {
        xpbdot = Float::with_val(PRECISION_BITS, 0.0f64);
    }
    x = ((psr.param.slots[param_a1 as usize].cells[0].val).clone()
        + ((xdot).clone() * (tt0).clone()));
    ecc = ((psr.param.slots[param_ecc as usize].cells[0].val).clone()
        + ((edot).clone() * (tt0).clone()));
    er = ((ecc).clone() * (Float::with_val(PRECISION_BITS, 1.0f64) + (dr).clone()));
    eth = ((ecc).clone() * (Float::with_val(PRECISION_BITS, 1.0f64) + (dth).clone()));
    if (((ecc).clone() < Float::with_val(PRECISION_BITS, 0.0f64))
        || ((ecc).clone() > Float::with_val(PRECISION_BITS, 1.0f64)))
    {
        {
            d.stdout(
                &[
                    b"DDHmodel: problem with eccentricity = ".to_vec(),
                    crate::clock_text::extended_general_six(
                        &((psr.param.slots[param_ecc as usize].cells[0].val).clone()),
                    ),
                    b" [".to_vec(),
                    read_string(&psr.name[..], "name")?,
                    b"]\n".to_vec(),
                ]
                .concat(),
            );
            return Err(boundary("DDHmodel.C::DDHmodel: source exit"));
        }
    }
    orbits = (((tt0).clone() / (pb).clone())
        - (((Float::with_val(PRECISION_BITS, Float::parse("0.5").unwrap())
            * ((pbdot).clone() + (xpbdot).clone()))
            * ((tt0).clone() / (pb).clone()))
            * ((tt0).clone() / (pb).clone())));
    norbits = crate::ifteph::extended_i32(&((orbits).clone()), "binary_integer")?;
    if ((orbits).clone() < Float::with_val(PRECISION_BITS, 0.0f64)) {
        norbits = norbits
            .checked_sub(1)
            .ok_or_else(|| boundary("binary orbit count overflow"))?;
    }
    phase = (Float::with_val(PRECISION_BITS, (2.0f64 * std::f64::consts::PI))
        * ((orbits).clone() - Float::with_val(PRECISION_BITS, norbits)));
    u = ((phase).clone()
        + (((ecc).clone() * ((phase).clone()).sin())
            * (Float::with_val(PRECISION_BITS, 1.0f64)
                + ((ecc).clone() * ((phase).clone()).cos()))));
    loop {
        {
            du = (((phase).clone() - ((u).clone() - ((ecc).clone() * ((u).clone()).sin())))
                / (Float::with_val(PRECISION_BITS, 1.0f64)
                    - ((ecc).clone() * ((u).clone()).cos())));
            u = ((u).clone() + (du).clone());
        }
        if !(((du).clone()).abs() > Float::with_val(PRECISION_BITS, 1.0e-12f64)) {
            break;
        }
    }
    su = ((u).clone()).sin();
    cu = ((u).clone()).cos();
    onemecu = (Float::with_val(PRECISION_BITS, 1.0f64) - ((ecc).clone() * (cu).clone()));
    cae = (((cu).clone() - (ecc).clone()) / (onemecu).clone());
    sae = (((Float::with_val(PRECISION_BITS, 1.0f64)
        - ((ecc).clone()).pow(&(Float::with_val(PRECISION_BITS, 2i32))))
    .sqrt()
        * (su).clone())
        / (onemecu).clone());
    ae = ((sae).clone()).atan2(&((cae).clone()));
    if ((ae).clone() < Float::with_val(PRECISION_BITS, 0.0f64)) {
        ae = ((ae).clone() + Float::with_val(PRECISION_BITS, (2.0f64 * std::f64::consts::PI)));
    }
    ae = (((Float::with_val(PRECISION_BITS, (2.0f64 * std::f64::consts::PI)) * (orbits).clone())
        + (ae).clone())
        - (phase).clone());
    omega = (((omz).clone() / (rad2deg).clone()) + ((k).clone() * (ae).clone()));
    sw = ((omega).clone()).sin();
    cw = ((omega).clone()).cos();
    alpha = ((x).clone() * (sw).clone());
    beta = (((x).clone()
        * (Float::with_val(PRECISION_BITS, 1i32)
            - ((eth).clone()).pow(&(Float::with_val(PRECISION_BITS, 2i32))))
        .sqrt())
        * (cw).clone());
    bg = ((beta).clone() + (gamma).clone());
    dre = (((alpha).clone() * ((cu).clone() - (er).clone())) + ((bg).clone() * (su).clone()));
    drep = (((-(alpha).clone()) * (su).clone()) + ((bg).clone() * (cu).clone()));
    drepp = (((-(alpha).clone()) * (cu).clone()) - ((bg).clone() * (su).clone()));
    anhat = ((an).clone() / (onemecu).clone());
    sqr1me2 = (Float::with_val(PRECISION_BITS, 1i32)
        - ((ecc).clone()).pow(&(Float::with_val(PRECISION_BITS, 2i32))))
    .sqrt();
    cume = ((cu).clone() - (ecc).clone());
    brace = ((onemecu).clone()
        - ((si).clone()
            * (((sw).clone() * (cume).clone())
                + (((sqr1me2).clone() * (cw).clone()) * (su).clone()))));
    dlogbr = ((brace).clone()).ln();
    ds = ((Float::with_val(PRECISION_BITS, (-2i32)) * (m2).clone()) * (dlogbr).clone());
    da = (((a0).clone()
        * (((omega).clone() + (ae).clone()).sin() + ((ecc).clone() * (sw).clone())))
        + ((b0).clone()
            * (((omega).clone() + (ae).clone()).cos() + ((ecc).clone() * (cw).clone()))));
    d2bar = ((((dre).clone()
        * ((Float::with_val(PRECISION_BITS, 1i32) - ((anhat).clone() * (drep).clone()))
            + (((anhat).clone()).pow(&(Float::with_val(PRECISION_BITS, 2i32)))
                * ((((drep).clone()).pow(&(Float::with_val(PRECISION_BITS, 2i32)))
                    + ((Float::with_val(PRECISION_BITS, 0.5f64) * (dre).clone())
                        * (drepp).clone()))
                    - (((((Float::with_val(PRECISION_BITS, 0.5f64) * (ecc).clone())
                        * (su).clone())
                        * (dre).clone())
                        * (drep).clone())
                        / (onemecu).clone())))))
        + (ds).clone())
        + (da).clone());
    torb = (-(d2bar).clone());
    if (param == (-1i32)) {
        return Ok(((torb).clone()).to_f64());
    }
    csigma = (((x).clone()
        * (((-(sw).clone()) * (su).clone())
            + (((sqr1me2).clone() * (cw).clone()) * (cu).clone())))
        / (onemecu).clone());
    ce = ((((su).clone() * (csigma).clone()) - ((x).clone() * (sw).clone()))
        - (((((ecc).clone() * (x).clone()) * (cw).clone()) * (su).clone()) / (sqr1me2).clone()));
    cx = (((sw).clone() * (cume).clone()) + (((sqr1me2).clone() * (cw).clone()) * (su).clone()));
    comega = ((x).clone()
        * (((cw).clone() * (cume).clone()) - (((sqr1me2).clone() * (sw).clone()) * (su).clone())));
    cgamma = (su).clone();
    cm2 = (Float::with_val(PRECISION_BITS, (-2i32)) * (dlogbr).clone());
    csi = (((Float::with_val(PRECISION_BITS, 2i32) * (m2).clone())
        * (((sw).clone() * (cume).clone()) + (((sqr1me2).clone() * (cw).clone()) * (su).clone())))
        / (brace).clone());
    let mut lgf: Float = ((Float::with_val(PRECISION_BITS, 1.0f64)
        + ((ecc).clone() * ((ae).clone()).cos()))
        / ((Float::with_val(PRECISION_BITS, 1.0f64)
            + ((stig).clone()).pow(&(Float::with_val(PRECISION_BITS, 2.0f64))))
            - ((Float::with_val(PRECISION_BITS, 2.0f64) * (stig).clone())
                * ((ae).clone() + (omega).clone()).sin())))
    .ln();
    if (param == (param_pb as i32)) {
        return Ok((((((-(csigma).clone()) * (an).clone())
            * Float::with_val(PRECISION_BITS, 86400.0f64))
            * (tt0).clone())
            / ((pb).clone() * Float::with_val(PRECISION_BITS, 86400.0f64)))
        .to_f64());
    } else {
        if (param == (param_a1 as i32)) {
            return Ok(((cx).clone()).to_f64());
        } else {
            if (param == (param_ecc as i32)) {
                return Ok(((ce).clone()).to_f64());
            } else {
                if (param == (param_om as i32)) {
                    return Ok(((comega).clone()).to_f64());
                } else {
                    if (param == (param_omdot as i32)) {
                        return Ok((((ae).clone() * (comega).clone())
                            / (((((an).clone() * Float::with_val(PRECISION_BITS, 360.0f64))
                                / Float::with_val(
                                    PRECISION_BITS,
                                    (2.0f64 * std::f64::consts::PI),
                                ))
                                * Float::with_val(PRECISION_BITS, 365.25f64))
                                * Float::with_val(PRECISION_BITS, 86400.0f64)))
                        .to_f64());
                    } else {
                        if (param == (param_t0 as i32)) {
                            return Ok((((-(csigma).clone()) * (an).clone())
                                * Float::with_val(PRECISION_BITS, 86400.0f64))
                            .to_f64());
                        } else {
                            if (param == (param_pbdot as i32)) {
                                return Ok(((Float::with_val(PRECISION_BITS, 0.5f64)
                                    * (tt0).clone())
                                    * (((((-(csigma).clone()) * (an).clone())
                                        * Float::with_val(PRECISION_BITS, 86400.0f64))
                                        * (tt0).clone())
                                        / ((pb).clone()
                                            * Float::with_val(PRECISION_BITS, 86400.0f64))))
                                .to_f64());
                            } else {
                                if (param == (param_sini as i32)) {
                                    return Ok(((csi).clone()).to_f64());
                                } else {
                                    if (param == (param_gamma as i32)) {
                                        return Ok(((cgamma).clone()).to_f64());
                                    } else {
                                        if (param == (param_m2 as i32)) {
                                            return Ok(((cm2).clone() * (SUNMASS).clone()).to_f64());
                                        } else {
                                            if (param == (param_a1dot as i32)) {
                                                return Ok(((cx).clone() * (tt0).clone()).to_f64());
                                            } else {
                                                if (param == (param_h3 as i32)) {
                                                    return Ok(((Float::with_val(
                                                        PRECISION_BITS,
                                                        2.0f64,
                                                    ) / ((stig).clone()).pow(
                                                        &(Float::with_val(PRECISION_BITS, 3.0f64)),
                                                    )) * (lgf).clone())
                                                    .to_f64());
                                                } else {
                                                    if (param == (param_stig as i32)) {
                                                        return Ok((((Float::with_val(
                                                            PRECISION_BITS,
                                                            (-2.0f64),
                                                        ) * (m2).clone())
                                                            / (stig).clone())
                                                            * ((Float::with_val(
                                                                PRECISION_BITS,
                                                                1.0f64,
                                                            ) + (Float::with_val(
                                                                PRECISION_BITS,
                                                                3.0f64,
                                                            ) * (lgf).clone()))
                                                                - ((Float::with_val(
                                                                    PRECISION_BITS,
                                                                    1.0f64,
                                                                ) - ((stig).clone()
                                                                    * (stig).clone()))
                                                                    / ((Float::with_val(
                                                                        PRECISION_BITS,
                                                                        1.0f64,
                                                                    ) + ((stig).clone()
                                                                        * (stig).clone()))
                                                                        - ((Float::with_val(
                                                                            PRECISION_BITS,
                                                                            2.0f64,
                                                                        ) * (stig)
                                                                            .clone())
                                                                            * ((ae).clone()
                                                                                + (omega)
                                                                                    .clone(
                                                                                    ))
                                                                            .sin())))))
                                                        .to_f64());
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

/// DDHmodel.C::updateDDH (pinned source line 252).
pub(super) fn updateDDH(
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
        if ((((((((pos == (param_a1 as i32)) || (pos == (param_ecc as i32)))
            || (pos == (param_t0 as i32)))
            || (pos == (param_sini as i32)))
            || (pos == (param_m2 as i32)))
            || (pos == (param_gamma as i32)))
            || (pos == (param_h3 as i32)))
            || (pos == (param_stig as i32)))
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
