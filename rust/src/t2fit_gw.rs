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
    PRECISION_BITS, TimingError,
    clock_diagnostics::{Diagnostics, boundary},
    observation::{SourceObservation, read},
    pulsar::SourcePulsar,
    tempo2::Label::*,
};
use rug::{Float, ops::Pow};
/// t2fit_gw.C::t2FitFunc_gwm_amp (pinned source line 5).
pub(crate) fn t2FitFunc_gwm_amp(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !(label == (param_gwm_amp as i32)) {
        return Err(boundary("t2fit_gw.C::t2FitFunc_gwm_amp: assertion"));
    }
    let mut dt: Float;
    let mut res: Option<f64> = None;
    if (psr.param.slots[(param_gwm_amp as i32) as usize].cells[(1i32) as usize].param_set == 1i32) {
        {
            dt = (((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
                - Float::with_val(PRECISION_BITS, *read(&psr.gwm_epoch, "psr[p].gwm_epoch")?))
                * Float::with_val(PRECISION_BITS, Float::parse("86400.0").unwrap()));
            if ((dt).clone() > Float::with_val(PRECISION_BITS, 0i32)) {
                {
                    if (k == 0i32) {
                        res = Some(
                            (Float::with_val(
                                PRECISION_BITS,
                                *read(&psr.quad_ifunc_geom_p, "psr[p].quad_ifunc_geom_p")?,
                            ) * (dt).clone())
                            .to_f64(),
                        );
                    } else {
                        if (k == 1i32) {
                            res = Some(
                                (Float::with_val(
                                    PRECISION_BITS,
                                    *read(&psr.quad_ifunc_geom_c, "psr[p].quad_ifunc_geom_c")?,
                                ) * (dt).clone())
                                .to_f64(),
                            );
                        }
                    }
                }
            } else {
                res = Some((0i32) as f64);
            }
        }
    } else {
        {
            let mut n1: f64;
            let mut n2: f64;
            let mut n3: f64;
            let mut cosTheta: f64;
            let mut lambda_p: f64;
            let mut beta_p: f64;
            let mut lambda: f64;
            let mut beta: f64;
            let mut g1: f64;
            let mut g2: f64;
            let mut g3: f64;
            if (psr.param.slots[(param_raj as i32) as usize].cells[(1i32) as usize].param_set
                == 1i32)
            {
                lambda_p = ((psr.param.slots[(param_raj as i32) as usize].cells[(1i32) as usize]
                    .val)
                    .clone())
                .to_f64();
            } else {
                lambda_p = ((psr.param.slots[(param_raj as i32) as usize].cells[(0i32) as usize]
                    .val)
                    .clone())
                .to_f64();
            }
            if (psr.param.slots[(param_decj as i32) as usize].cells[(1i32) as usize].param_set
                == 1i32)
            {
                beta_p = ((psr.param.slots[(param_decj as i32) as usize].cells[(1i32) as usize]
                    .val)
                    .clone())
                .to_f64();
            } else {
                beta_p = ((psr.param.slots[(param_decj as i32) as usize].cells[(0i32) as usize]
                    .val)
                    .clone())
                .to_f64();
            }
            lambda = *read(&psr.gwm_raj, "psr[p].gwm_raj")?;
            beta = *read(&psr.gwm_decj, "psr[p].gwm_decj")?;
            g1 = ((-(Float::with_val(PRECISION_BITS, lambda)).cos())
                * (Float::with_val(PRECISION_BITS, beta)).cos())
            .to_f64();
            g2 = ((-(Float::with_val(PRECISION_BITS, lambda)).sin())
                * (Float::with_val(PRECISION_BITS, beta)).cos())
            .to_f64();
            g3 = (-(Float::with_val(PRECISION_BITS, beta)).sin()).to_f64();
            n1 = ((Float::with_val(PRECISION_BITS, lambda_p)).cos()
                * (Float::with_val(PRECISION_BITS, beta_p)).cos())
            .to_f64();
            n2 = ((Float::with_val(PRECISION_BITS, lambda_p)).sin()
                * (Float::with_val(PRECISION_BITS, beta_p)).cos())
            .to_f64();
            n3 = ((Float::with_val(PRECISION_BITS, beta_p)).sin()).to_f64();
            cosTheta = (-((((Float::with_val(PRECISION_BITS, beta)).cos()
                * (Float::with_val(PRECISION_BITS, beta_p)).cos())
                * (Float::with_val(PRECISION_BITS, (lambda - lambda_p))).cos())
                + ((Float::with_val(PRECISION_BITS, beta)).sin()
                    * (Float::with_val(PRECISION_BITS, beta_p)).sin())))
            .to_f64();
            if ((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()
                >= Float::with_val(PRECISION_BITS, *read(&psr.gwm_epoch, "psr[p].gwm_epoch")?))
            {
                {
                    let mut dt: Float;
                    let mut scale: Float;
                    let mut cos2Phi: f64;
                    let mut cosPhi: f64;
                    let mut l1: f64;
                    let mut l2: f64;
                    let mut l3: f64;
                    let mut m1: f64;
                    let mut m2: f64;
                    let mut m3: f64;
                    let mut d1: Option<f64> = None;
                    let mut d2: Option<f64> = None;
                    let mut d3: Option<f64> = None;
                    let mut md: f64;
                    let mut a1: f64;
                    let mut a2: f64;
                    let mut a3: f64;
                    let mut ma: f64;
                    if (beta == 0.0f64) {
                        {
                            d1 = Some(0.0f64);
                            d2 = Some(0.0f64);
                            d3 = Some(1.0f64);
                        }
                    }
                    if (beta > (0i32) as f64) {
                        {
                            d1 = Some(
                                (Float::with_val(PRECISION_BITS, g1)
                                    * (Float::with_val(
                                        PRECISION_BITS,
                                        ((0.5f64 * std::f64::consts::PI) - beta),
                                    ))
                                    .cos())
                                .to_f64(),
                            );
                            d2 = Some(
                                (Float::with_val(PRECISION_BITS, g2)
                                    * (Float::with_val(
                                        PRECISION_BITS,
                                        ((0.5f64 * std::f64::consts::PI) - beta),
                                    ))
                                    .cos())
                                .to_f64(),
                            );
                            d3 = Some(
                                (1.0f64 + (g3 * ((0.5f64 * std::f64::consts::PI) - beta).cos())),
                            );
                            md = (((*read(&d1, "d1")? * *read(&d1, "d1")?)
                                + (*read(&d2, "d2")? * *read(&d2, "d2")?))
                                + (*read(&d3, "d3")? * *read(&d3, "d3")?))
                            .sqrt();
                            d1 = Some((*read(&d1, "d1")? / md));
                            d2 = Some((*read(&d2, "d2")? / md));
                            d3 = Some((*read(&d3, "d3")? / md));
                        }
                    } else {
                        if (beta < (0i32) as f64) {
                            {
                                d1 = Some(
                                    (Float::with_val(PRECISION_BITS, g1)
                                        * (Float::with_val(
                                            PRECISION_BITS,
                                            (((-0.5f64) * std::f64::consts::PI) - beta),
                                        ))
                                        .cos())
                                    .to_f64(),
                                );
                                d2 = Some(
                                    (Float::with_val(PRECISION_BITS, g2)
                                        * (Float::with_val(
                                            PRECISION_BITS,
                                            (((-0.5f64) * std::f64::consts::PI) - beta),
                                        ))
                                        .cos())
                                    .to_f64(),
                                );
                                d3 = Some(
                                    ((-1.0f64)
                                        + (g3 * (((-0.5f64) * std::f64::consts::PI) - beta).cos())),
                                );
                                md = (((*read(&d1, "d1")? * *read(&d1, "d1")?)
                                    + (*read(&d2, "d2")? * *read(&d2, "d2")?))
                                    + (*read(&d3, "d3")? * *read(&d3, "d3")?))
                                .sqrt();
                                d1 = Some((*read(&d1, "d1")? / md));
                                d2 = Some((*read(&d2, "d2")? / md));
                                d3 = Some((*read(&d3, "d3")? / md));
                            }
                        }
                    }
                    a1 = ((*read(&d2, "d2")? * g3) - (*read(&d3, "d3")? * g2));
                    a2 = ((*read(&d3, "d3")? * g1) - (*read(&d1, "d1")? * g3));
                    a3 = ((*read(&d1, "d1")? * g2) - (*read(&d2, "d2")? * g1));
                    ma = (((a1 * a1) + (a2 * a2)) + (a3 * a3)).sqrt();
                    a1 = (a1 / ma);
                    a2 = (a2 / ma);
                    a3 = (a3 / ma);
                    m1 = ((Float::with_val(PRECISION_BITS, *read(&d1, "d1")?)
                        * (Float::with_val(
                            PRECISION_BITS,
                            *read(&psr.gwm_phi, "psr[p].gwm_phi")?,
                        ))
                        .cos())
                        + (Float::with_val(PRECISION_BITS, a1)
                            * (Float::with_val(
                                PRECISION_BITS,
                                *read(&psr.gwm_phi, "psr[p].gwm_phi")?,
                            ))
                            .sin()))
                    .to_f64();
                    m2 = ((Float::with_val(PRECISION_BITS, *read(&d2, "d2")?)
                        * (Float::with_val(
                            PRECISION_BITS,
                            *read(&psr.gwm_phi, "psr[p].gwm_phi")?,
                        ))
                        .cos())
                        + (Float::with_val(PRECISION_BITS, a2)
                            * (Float::with_val(
                                PRECISION_BITS,
                                *read(&psr.gwm_phi, "psr[p].gwm_phi")?,
                            ))
                            .sin()))
                    .to_f64();
                    m3 = ((Float::with_val(PRECISION_BITS, *read(&d3, "d3")?)
                        * (Float::with_val(
                            PRECISION_BITS,
                            *read(&psr.gwm_phi, "psr[p].gwm_phi")?,
                        ))
                        .cos())
                        + (Float::with_val(PRECISION_BITS, a3)
                            * (Float::with_val(
                                PRECISION_BITS,
                                *read(&psr.gwm_phi, "psr[p].gwm_phi")?,
                            ))
                            .sin()))
                    .to_f64();
                    if ((cosTheta != 1.0f64) && (cosTheta != (-1.0f64))) {
                        {
                            g1 = (g1 * cosTheta);
                            g2 = (g2 * cosTheta);
                            g3 = (g3 * cosTheta);
                            l1 = (n1 - g1);
                            l2 = (n2 - g2);
                            l3 = (n3 - g3);
                            cosPhi = ((((l1 * m1) + (l2 * m2)) + (l3 * m3))
                                / (((l1 * l1) + (l2 * l2)) + (l3 * l3)).sqrt());
                            cos2Phi = ((((2i32) as f64 * cosPhi) * cosPhi) - 1.0f64);
                        }
                    } else {
                        {
                            cos2Phi = (0i32) as f64;
                        }
                    }
                    dt = (((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()
                        - Float::with_val(
                            PRECISION_BITS,
                            *read(&psr.gwm_epoch, "psr[p].gwm_epoch")?,
                        ))
                        * Float::with_val(PRECISION_BITS, 86400.0f64));
                    scale = Float::with_val(
                        PRECISION_BITS,
                        (((-0.5f64) * cos2Phi) * ((1i32) as f64 - cosTheta)),
                    );
                    res = Some(((scale).clone() * (dt).clone()).to_f64());
                }
            } else {
                res = Some((0i32) as f64);
            }
        }
    }
    return Ok(*read(&res, "res")?);
}

/// t2fit_gw.C::t2FitFunc_gwb_amp (pinned source line 165).
pub(crate) fn t2FitFunc_gwb_amp(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut res: f64 = (0i32) as f64;
    let mut dt: Float;
    let mut prefac: Float;
    if (psr.param.slots[(param_gwb_amp as i32) as usize].cells[(1i32) as usize].param_set == 1i32) {
        {
            dt = (((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
                - Float::with_val(PRECISION_BITS, *read(&psr.gwb_epoch, "psr[p].gwb_epoch")?))
                / Float::with_val(PRECISION_BITS, *read(&psr.gwb_width, "psr[p].gwb_width")?));
            prefac = ((dt).clone()
                * ((Float::with_val(PRECISION_BITS, (-(dt).clone()).to_f64()) * (dt).clone())
                    / Float::with_val(PRECISION_BITS, 2.0f64))
                .exp());
            if (k == 0i32) {
                {
                    res = (Float::with_val(
                        PRECISION_BITS,
                        *read(&psr.gwb_geom_p, "psr[p].gwb_geom_p")?,
                    ) * (prefac).clone())
                    .to_f64();
                }
            } else {
                if (k == 1i32) {
                    {
                        res = (Float::with_val(
                            PRECISION_BITS,
                            *read(&psr.gwb_geom_c, "psr[p].gwb_geom_c")?,
                        ) * (prefac).clone())
                        .to_f64();
                    }
                } else {
                    {
                        res = (0i32) as f64;
                    }
                }
            }
        }
    }
    return Ok(res);
}

/// t2fit_gw.C::t2FitFunc_gwcs_amp (pinned source line 192).
pub(crate) fn t2FitFunc_gwcs_amp(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut res: f64 = (0i32) as f64;
    let mut dt: Float;
    let mut prefac: Float;
    let mut extra: Option<f64> = None;
    let mut width: Float;
    let mut width_day: Float;
    if (psr.param.slots[(param_gwcs_amp as i32) as usize].cells[(0i32) as usize].param_set == 1i32)
    {
        {
            dt = (((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
                - Float::with_val(PRECISION_BITS, *read(&psr.gwcs_epoch, "psr[p].gwcs_epoch")?))
                * Float::with_val(PRECISION_BITS, 86400.0f64));
            width = Float::with_val(
                PRECISION_BITS,
                (*read(&psr.gwcs_width, "psr[p].gwcs_width")? * 86400.0f64),
            );
            width_day =
                Float::with_val(PRECISION_BITS, *read(&psr.gwcs_width, "psr[p].gwcs_width")?);
            if ((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()
                < (Float::with_val(PRECISION_BITS, *read(&psr.gwcs_epoch, "psr[p].gwcs_epoch")?)
                    - ((width_day).clone() / Float::with_val(PRECISION_BITS, 2.0f64))))
            {
                res = (0i32) as f64;
            } else {
                if ((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()
                    < Float::with_val(PRECISION_BITS, *read(&psr.gwcs_epoch, "psr[p].gwcs_epoch")?))
                {
                    {
                        extra = Some(
                            ((Float::with_val(PRECISION_BITS, (3.0f64 / 4.0f64))
                                * ((Float::with_val(PRECISION_BITS, 0.5f64) * (width).clone())
                                    .pow(&(Float::with_val(PRECISION_BITS, (4.0f64 / 3.0f64))))
                                    - (((dt).clone()).abs()).pow(
                                        &(Float::with_val(PRECISION_BITS, (4.0f64 / 3.0f64))),
                                    )))
                                - ((Float::with_val(PRECISION_BITS, 0.5f64) * (width).clone())
                                    .pow(&(Float::with_val(PRECISION_BITS, (1.0f64 / 3.0f64))))
                                    * ((dt).clone()
                                        + (Float::with_val(PRECISION_BITS, 0.5f64)
                                            * (width).clone()))))
                            .to_f64(),
                        );
                    }
                } else {
                    if ((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()
                        < (Float::with_val(
                            PRECISION_BITS,
                            *read(&psr.gwcs_epoch, "psr[p].gwcs_epoch")?,
                        ) + ((width_day).clone() / Float::with_val(PRECISION_BITS, 2.0f64))))
                    {
                        {
                            extra = Some(
                                ((Float::with_val(PRECISION_BITS, (3.0f64 / 4.0f64))
                                    * ((Float::with_val(PRECISION_BITS, 0.5f64)
                                        * (width).clone())
                                    .pow(&(Float::with_val(PRECISION_BITS, (4.0f64 / 3.0f64))))
                                        + (((dt).clone()).abs()).pow(
                                            &(Float::with_val(PRECISION_BITS, (4.0f64 / 3.0f64))),
                                        )))
                                    - ((Float::with_val(PRECISION_BITS, 0.5f64)
                                        * (width).clone())
                                    .pow(&(Float::with_val(PRECISION_BITS, (1.0f64 / 3.0f64))))
                                        * ((dt).clone()
                                            + (Float::with_val(PRECISION_BITS, 0.5f64)
                                                * (width).clone()))))
                                .to_f64(),
                            );
                        }
                    } else {
                        {
                            extra = Some(
                                (Float::with_val(PRECISION_BITS, (-0.25f64))
                                    * (Float::with_val(
                                        PRECISION_BITS,
                                        (0.5f64).powf((1.0f64 / 3.0f64)),
                                    ) * ((width).clone()).pow(
                                        &(Float::with_val(PRECISION_BITS, (4.0f64 / 3.0f64))),
                                    )))
                                .to_f64(),
                            );
                        }
                    }
                }
            }
        }
    }
    if (k == 0i32) {
        res = (*read(&psr.gwcs_geom_p, "psr[p].gwcs_geom_p")? * *read(&extra, "extra")?);
    } else {
        if (k == 1i32) {
            res = (*read(&psr.gwcs_geom_c, "psr[p].gwcs_geom_c")? * *read(&extra, "extra")?);
        }
    }
    return Ok(res);
}

/// t2fit_gw.C::t2FitFunc_quad_om (pinned source line 238).
pub(crate) fn t2FitFunc_quad_om(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !(label == (param_quad_om as i32)) {
        return Err(boundary("t2fit_gw.C::t2FitFunc_quad_om: assertion"));
    }
    let mut afunc: f64 = (0i32) as f64;
    let mut n1: f64;
    let mut n2: f64;
    let mut n3: f64;
    let mut e11p: f64;
    let mut e21p: f64;
    let mut e31p: f64;
    let mut e12p: f64;
    let mut e22p: f64;
    let mut e32p: f64;
    let mut e13p: f64;
    let mut e23p: f64;
    let mut e33p: f64;
    let mut e11c: f64;
    let mut e21c: f64;
    let mut e31c: f64;
    let mut e12c: f64;
    let mut e22c: f64;
    let mut e32c: f64;
    let mut e13c: f64;
    let mut e23c: f64;
    let mut e33c: f64;
    let mut cosTheta: f64;
    let mut omega_g: f64;
    let mut resp: Float;
    let mut resc: Float;
    let mut lambda_p: f64;
    let mut beta_p: f64;
    let mut lambda: f64;
    let mut beta: f64;
    let mut time: Float;
    time = (((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
        - Float::with_val(PRECISION_BITS, *read(&psr.quadEpoch, "psr[p].quadEpoch")?))
        * Float::with_val(PRECISION_BITS, Float::parse("86400.0").unwrap()));
    if (psr.param.slots[(param_raj as i32) as usize].cells[(1i32) as usize].param_set == 1i32) {
        lambda_p = ((psr.param.slots[(param_raj as i32) as usize].cells[(1i32) as usize].val)
            .clone())
        .to_f64();
    } else {
        lambda_p = ((psr.param.slots[(param_raj as i32) as usize].cells[(0i32) as usize].val)
            .clone())
        .to_f64();
    }
    if (psr.param.slots[(param_raj as i32) as usize].cells[(1i32) as usize].param_set == 1i32) {
        beta_p = ((psr.param.slots[(param_decj as i32) as usize].cells[(1i32) as usize].val)
            .clone())
        .to_f64();
    } else {
        beta_p = ((psr.param.slots[(param_decj as i32) as usize].cells[(0i32) as usize].val)
            .clone())
        .to_f64();
    }
    lambda = *read(&psr.quadRA, "psr[p].quadRA")?;
    beta = *read(&psr.quadDEC, "psr[p].quadDEC")?;
    n1 = ((Float::with_val(PRECISION_BITS, lambda_p)).cos()
        * (Float::with_val(PRECISION_BITS, beta_p)).cos())
    .to_f64();
    n2 = ((Float::with_val(PRECISION_BITS, lambda_p)).sin()
        * (Float::with_val(PRECISION_BITS, beta_p)).cos())
    .to_f64();
    n3 = ((Float::with_val(PRECISION_BITS, beta_p)).sin()).to_f64();
    cosTheta = ((((Float::with_val(PRECISION_BITS, beta)).cos()
        * (Float::with_val(PRECISION_BITS, beta_p)).cos())
        * (Float::with_val(PRECISION_BITS, (lambda - lambda_p))).cos())
        + ((Float::with_val(PRECISION_BITS, beta)).sin()
            * (Float::with_val(PRECISION_BITS, beta_p)).sin()))
    .to_f64();
    e11p = (((Float::with_val(PRECISION_BITS, lambda)).sin())
        .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
        - (((Float::with_val(PRECISION_BITS, lambda)).cos())
            .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
            * ((Float::with_val(PRECISION_BITS, beta)).sin())
                .pow(&(Float::with_val(PRECISION_BITS, 2i32)))))
    .to_f64();
    e21p = (((-(Float::with_val(PRECISION_BITS, lambda)).sin())
        * (Float::with_val(PRECISION_BITS, lambda)).cos())
        * (((Float::with_val(PRECISION_BITS, beta)).sin())
            .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
            + Float::with_val(PRECISION_BITS, 1i32)))
    .to_f64();
    e31p = (((Float::with_val(PRECISION_BITS, lambda)).cos()
        * (Float::with_val(PRECISION_BITS, beta)).sin())
        * (Float::with_val(PRECISION_BITS, beta)).cos())
    .to_f64();
    e12p = (((-(Float::with_val(PRECISION_BITS, lambda)).sin())
        * (Float::with_val(PRECISION_BITS, lambda)).cos())
        * (((Float::with_val(PRECISION_BITS, beta)).sin())
            .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
            + Float::with_val(PRECISION_BITS, 1i32)))
    .to_f64();
    e22p = (((Float::with_val(PRECISION_BITS, lambda)).cos())
        .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
        - (((Float::with_val(PRECISION_BITS, lambda)).sin())
            .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
            * ((Float::with_val(PRECISION_BITS, beta)).sin())
                .pow(&(Float::with_val(PRECISION_BITS, 2i32)))))
    .to_f64();
    e32p = (((Float::with_val(PRECISION_BITS, lambda)).sin()
        * (Float::with_val(PRECISION_BITS, beta)).sin())
        * (Float::with_val(PRECISION_BITS, beta)).cos())
    .to_f64();
    e13p = (((Float::with_val(PRECISION_BITS, lambda)).cos()
        * (Float::with_val(PRECISION_BITS, beta)).sin())
        * (Float::with_val(PRECISION_BITS, beta)).cos())
    .to_f64();
    e23p = (((Float::with_val(PRECISION_BITS, lambda)).sin()
        * (Float::with_val(PRECISION_BITS, beta)).sin())
        * (Float::with_val(PRECISION_BITS, beta)).cos())
    .to_f64();
    e33p = (-((Float::with_val(PRECISION_BITS, beta)).cos())
        .pow(&(Float::with_val(PRECISION_BITS, 2i32))))
    .to_f64();
    resp = Float::with_val(
        PRECISION_BITS,
        (((n1 * (((n1 * e11p) + (n2 * e12p)) + (n3 * e13p)))
            + (n2 * (((n1 * e21p) + (n2 * e22p)) + (n3 * e23p))))
            + (n3 * (((n1 * e31p) + (n2 * e32p)) + (n3 * e33p)))),
    );
    e11c = (((2i32) as f64 * lambda).sin() * (beta).sin());
    e21c = ((-((2i32) as f64 * lambda).cos()) * (beta).sin());
    e31c = ((-(lambda).sin()) * (beta).cos());
    e12c = ((-((2i32) as f64 * lambda).cos()) * (beta).sin());
    e22c = ((-((2i32) as f64 * lambda).sin()) * (beta).sin());
    e32c = ((lambda).cos() * (beta).cos());
    e13c = ((-(lambda).sin()) * (beta).cos());
    e23c = ((lambda).cos() * (beta).cos());
    e33c = (0i32) as f64;
    resc = Float::with_val(
        PRECISION_BITS,
        (((n1 * (((n1 * e11c) + (n2 * e12c)) + (n3 * e13c)))
            + (n2 * (((n1 * e21c) + (n2 * e22c)) + (n3 * e23c))))
            + (n3 * (((n1 * e31c) + (n2 * e32c)) + (n3 * e33c)))),
    );
    omega_g = (((psr.param.slots[(param_quad_om as i32) as usize].cells[(0i32) as usize].val)
        .clone())
    .to_f64()
        * (source_i32(((k) as f64 / 4.0f64))? + 1i32) as f64);
    if ((Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
        - Float::with_val(PRECISION_BITS, cosTheta))
        == Float::with_val(PRECISION_BITS, 0i32))
    {
        afunc = (0i32) as f64;
    } else {
        {
            if ((k % 4i32) == 0i32) {
                afunc = (((resp).clone()
                    * (Float::with_val(PRECISION_BITS, omega_g) * (time).clone()).sin())
                    / ((Float::with_val(PRECISION_BITS, Float::parse("2.0").unwrap())
                        * Float::with_val(PRECISION_BITS, omega_g))
                        * (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                            - Float::with_val(PRECISION_BITS, cosTheta))))
                .to_f64();
            } else {
                if ((k % 4i32) == 1i32) {
                    afunc = (((resp).clone()
                        * (Float::with_val(PRECISION_BITS, omega_g) * (time).clone()).cos())
                        / ((Float::with_val(PRECISION_BITS, Float::parse("2.0").unwrap())
                            * Float::with_val(PRECISION_BITS, omega_g))
                            * (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                                - Float::with_val(PRECISION_BITS, cosTheta))))
                    .to_f64();
                } else {
                    if ((k % 4i32) == 2i32) {
                        afunc = (((resc).clone()
                            * (Float::with_val(PRECISION_BITS, omega_g) * (time).clone()).sin())
                            / ((Float::with_val(PRECISION_BITS, Float::parse("2.0").unwrap())
                                * Float::with_val(PRECISION_BITS, omega_g))
                                * (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                                    - Float::with_val(PRECISION_BITS, cosTheta))))
                        .to_f64();
                    } else {
                        if ((k % 4i32) == 3i32) {
                            afunc = (((resc).clone()
                                * (Float::with_val(PRECISION_BITS, omega_g) * (time).clone())
                                    .cos())
                                / ((Float::with_val(
                                    PRECISION_BITS,
                                    Float::parse("2.0").unwrap(),
                                ) * Float::with_val(PRECISION_BITS, omega_g))
                                    * (Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1.0").unwrap(),
                                    ) - Float::with_val(PRECISION_BITS, cosTheta))))
                            .to_f64();
                        }
                    }
                }
            }
            if (*read(&psr.gwsrc_psrdist, "psr[p].gwsrc_psrdist")? > (0i32) as f64) {
                {
                    if ((k % 4i32) == 0i32) {
                        afunc = (Float::with_val(PRECISION_BITS, afunc)
                            - (((resp).clone()
                                * ((Float::with_val(PRECISION_BITS, omega_g) * (time).clone())
                                    - Float::with_val(
                                        PRECISION_BITS,
                                        (((((1i32) as f64 - cosTheta)
                                            * *read(&psr.gwsrc_psrdist, "psr[p].gwsrc_psrdist")?)
                                            / 299792458.0f64)
                                            * omega_g),
                                    ))
                                .sin())
                                / ((Float::with_val(
                                    PRECISION_BITS,
                                    Float::parse("2.0").unwrap(),
                                ) * Float::with_val(PRECISION_BITS, omega_g))
                                    * (Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1.0").unwrap(),
                                    ) - Float::with_val(PRECISION_BITS, cosTheta)))))
                        .to_f64();
                    } else {
                        if ((k % 4i32) == 1i32) {
                            afunc = (Float::with_val(PRECISION_BITS, afunc)
                                - (((resp).clone()
                                    * ((Float::with_val(PRECISION_BITS, omega_g)
                                        * (time).clone())
                                        - Float::with_val(
                                            PRECISION_BITS,
                                            (((((1i32) as f64 - cosTheta)
                                                * *read(
                                                    &psr.gwsrc_psrdist,
                                                    "psr[p].gwsrc_psrdist",
                                                )?)
                                                / 299792458.0f64)
                                                * omega_g),
                                        ))
                                    .cos())
                                    / ((Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("2.0").unwrap(),
                                    ) * Float::with_val(PRECISION_BITS, omega_g))
                                        * (Float::with_val(
                                            PRECISION_BITS,
                                            Float::parse("1.0").unwrap(),
                                        ) - Float::with_val(PRECISION_BITS, cosTheta)))))
                            .to_f64();
                        } else {
                            if ((k % 4i32) == 2i32) {
                                afunc = (Float::with_val(PRECISION_BITS, afunc)
                                    - (((resc).clone()
                                        * ((Float::with_val(PRECISION_BITS, omega_g)
                                            * (time).clone())
                                            - Float::with_val(
                                                PRECISION_BITS,
                                                (((((1i32) as f64 - cosTheta)
                                                    * *read(
                                                        &psr.gwsrc_psrdist,
                                                        "psr[p].gwsrc_psrdist",
                                                    )?)
                                                    / 299792458.0f64)
                                                    * omega_g),
                                            ))
                                        .sin())
                                        / ((Float::with_val(
                                            PRECISION_BITS,
                                            Float::parse("2.0").unwrap(),
                                        ) * Float::with_val(PRECISION_BITS, omega_g))
                                            * (Float::with_val(
                                                PRECISION_BITS,
                                                Float::parse("1.0").unwrap(),
                                            ) - Float::with_val(PRECISION_BITS, cosTheta)))))
                                .to_f64();
                            } else {
                                if ((k % 4i32) == 3i32) {
                                    afunc = (Float::with_val(PRECISION_BITS, afunc)
                                        - (((resc).clone()
                                            * ((Float::with_val(PRECISION_BITS, omega_g)
                                                * (time).clone())
                                                - Float::with_val(
                                                    PRECISION_BITS,
                                                    (((((1i32) as f64 - cosTheta)
                                                        * *read(
                                                            &psr.gwsrc_psrdist,
                                                            "psr[p].gwsrc_psrdist",
                                                        )?)
                                                        / 299792458.0f64)
                                                        * omega_g),
                                                ))
                                            .cos())
                                            / ((Float::with_val(
                                                PRECISION_BITS,
                                                Float::parse("2.0").unwrap(),
                                            ) * Float::with_val(PRECISION_BITS, omega_g))
                                                * (Float::with_val(
                                                    PRECISION_BITS,
                                                    Float::parse("1.0").unwrap(),
                                                ) - Float::with_val(
                                                    PRECISION_BITS,
                                                    cosTheta,
                                                )))))
                                    .to_f64();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return Ok(afunc);
}

/// t2fit_gw.C::t2UpdateFunc_quad_om (pinned source line 326).
pub(crate) fn t2UpdateFunc_quad_om(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut err: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    let mut quad_j: i32 = (k / 4i32);
    let mut quad_k: i32 = (k % 4i32);
    match quad_k {
        source_case if source_case == 0 => {
            psr.quad_aplus_r[(quad_j) as usize] = Some(
                (*read(
                    &psr.quad_aplus_r[(quad_j) as usize],
                    "psr[p].quad_aplus_r[quad_j]",
                )? - val),
            );
            psr.quad_aplus_r_e[(quad_j) as usize] = Some(err);
        }
        source_case if source_case == 1 => {
            psr.quad_aplus_i[(quad_j) as usize] = Some(
                (*read(
                    &psr.quad_aplus_i[(quad_j) as usize],
                    "psr[p].quad_aplus_i[quad_j]",
                )? - val),
            );
            psr.quad_aplus_i_e[(quad_j) as usize] = Some(err);
        }
        source_case if source_case == 2 => {
            psr.quad_across_r[(quad_j) as usize] = Some(
                (*read(
                    &psr.quad_across_r[(quad_j) as usize],
                    "psr[p].quad_across_r[quad_j]",
                )? - val),
            );
            psr.quad_across_r_e[(quad_j) as usize] = Some(err);
        }
        source_case if source_case == 3 => {
            psr.quad_across_i[(quad_j) as usize] = Some(
                (*read(
                    &psr.quad_across_i[(quad_j) as usize],
                    "psr[p].quad_across_i[quad_j]",
                )? - val),
            );
            psr.quad_across_i_e[(quad_j) as usize] = Some(err);
        }
        _ => {}
    }
    Ok(())
}

/// t2fit_gw.C::t2FitFunc_gwsingle (pinned source line 351).
pub(crate) fn t2FitFunc_gwsingle(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !(label == (param_gwsingle as i32)) {
        return Err(boundary("t2fit_gw.C::t2FitFunc_gwsingle: assertion"));
    }
    let mut afunc: f64 = (0i32) as f64;
    let mut n1: f64;
    let mut n2: f64;
    let mut n3: f64;
    let mut e11p: f64;
    let mut e21p: f64;
    let mut e31p: f64;
    let mut e12p: f64;
    let mut e22p: f64;
    let mut e32p: f64;
    let mut e13p: f64;
    let mut e23p: f64;
    let mut e33p: f64;
    let mut e11c: f64;
    let mut e21c: f64;
    let mut e31c: f64;
    let mut e12c: f64;
    let mut e22c: f64;
    let mut e32c: f64;
    let mut e13c: f64;
    let mut e23c: f64;
    let mut e33c: f64;
    let mut cosTheta: f64;
    let mut omega_g: f64;
    let mut resp: Float;
    let mut resc: Float;
    let mut lambda_p: f64;
    let mut beta_p: f64;
    let mut lambda: f64;
    let mut beta: f64;
    let mut time: Float;
    time = (((read(&rows[(ipos) as usize].bbat, "psr[p].obsn[ipos].bbat")?).clone()
        - Float::with_val(
            PRECISION_BITS,
            *read(&psr.gwsrc_epoch, "psr[p].gwsrc_epoch")?,
        ))
        * Float::with_val(PRECISION_BITS, Float::parse("86400.0").unwrap()));
    lambda_p = ((psr.param.slots[(param_raj as i32) as usize].cells[(0i32) as usize].val).clone())
        .to_f64();
    beta_p = ((psr.param.slots[(param_decj as i32) as usize].cells[(0i32) as usize].val).clone())
        .to_f64();
    lambda = *read(&psr.gwsrc_ra, "psr[p].gwsrc_ra")?;
    beta = *read(&psr.gwsrc_dec, "psr[p].gwsrc_dec")?;
    n1 = ((Float::with_val(PRECISION_BITS, lambda_p)).cos()
        * (Float::with_val(PRECISION_BITS, beta_p)).cos())
    .to_f64();
    n2 = ((Float::with_val(PRECISION_BITS, lambda_p)).sin()
        * (Float::with_val(PRECISION_BITS, beta_p)).cos())
    .to_f64();
    n3 = ((Float::with_val(PRECISION_BITS, beta_p)).sin()).to_f64();
    cosTheta = ((((Float::with_val(PRECISION_BITS, beta)).cos()
        * (Float::with_val(PRECISION_BITS, beta_p)).cos())
        * (Float::with_val(PRECISION_BITS, (lambda - lambda_p))).cos())
        + ((Float::with_val(PRECISION_BITS, beta)).sin()
            * (Float::with_val(PRECISION_BITS, beta_p)).sin()))
    .to_f64();
    e11p = (((Float::with_val(PRECISION_BITS, lambda)).sin())
        .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
        - (((Float::with_val(PRECISION_BITS, lambda)).cos())
            .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
            * ((Float::with_val(PRECISION_BITS, beta)).sin())
                .pow(&(Float::with_val(PRECISION_BITS, 2i32)))))
    .to_f64();
    e21p = (((-(Float::with_val(PRECISION_BITS, lambda)).sin())
        * (Float::with_val(PRECISION_BITS, lambda)).cos())
        * (((Float::with_val(PRECISION_BITS, beta)).sin())
            .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
            + Float::with_val(PRECISION_BITS, 1i32)))
    .to_f64();
    e31p = (((Float::with_val(PRECISION_BITS, lambda)).cos()
        * (Float::with_val(PRECISION_BITS, beta)).sin())
        * (Float::with_val(PRECISION_BITS, beta)).cos())
    .to_f64();
    e12p = (((-(Float::with_val(PRECISION_BITS, lambda)).sin())
        * (Float::with_val(PRECISION_BITS, lambda)).cos())
        * (((Float::with_val(PRECISION_BITS, beta)).sin())
            .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
            + Float::with_val(PRECISION_BITS, 1i32)))
    .to_f64();
    e22p = (((Float::with_val(PRECISION_BITS, lambda)).cos())
        .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
        - (((Float::with_val(PRECISION_BITS, lambda)).sin())
            .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
            * ((Float::with_val(PRECISION_BITS, beta)).sin())
                .pow(&(Float::with_val(PRECISION_BITS, 2i32)))))
    .to_f64();
    e32p = (((Float::with_val(PRECISION_BITS, lambda)).sin()
        * (Float::with_val(PRECISION_BITS, beta)).sin())
        * (Float::with_val(PRECISION_BITS, beta)).cos())
    .to_f64();
    e13p = (((Float::with_val(PRECISION_BITS, lambda)).cos()
        * (Float::with_val(PRECISION_BITS, beta)).sin())
        * (Float::with_val(PRECISION_BITS, beta)).cos())
    .to_f64();
    e23p = (((Float::with_val(PRECISION_BITS, lambda)).sin()
        * (Float::with_val(PRECISION_BITS, beta)).sin())
        * (Float::with_val(PRECISION_BITS, beta)).cos())
    .to_f64();
    e33p = (-((Float::with_val(PRECISION_BITS, beta)).cos())
        .pow(&(Float::with_val(PRECISION_BITS, 2i32))))
    .to_f64();
    omega_g = ((psr.param.slots[(param_gwsingle as i32) as usize].cells[(0i32) as usize].val)
        .clone())
    .to_f64();
    resp = Float::with_val(
        PRECISION_BITS,
        (((n1 * (((n1 * e11p) + (n2 * e12p)) + (n3 * e13p)))
            + (n2 * (((n1 * e21p) + (n2 * e22p)) + (n3 * e23p))))
            + (n3 * (((n1 * e31p) + (n2 * e32p)) + (n3 * e33p)))),
    );
    e11c = (((2i32) as f64 * lambda).sin() * (beta).sin());
    e21c = ((-((2i32) as f64 * lambda).cos()) * (beta).sin());
    e31c = ((-(lambda).sin()) * (beta).cos());
    e12c = ((-((2i32) as f64 * lambda).cos()) * (beta).sin());
    e22c = ((-((2i32) as f64 * lambda).sin()) * (beta).sin());
    e32c = ((lambda).cos() * (beta).cos());
    e13c = ((-(lambda).sin()) * (beta).cos());
    e23c = ((lambda).cos() * (beta).cos());
    e33c = (0i32) as f64;
    resc = Float::with_val(
        PRECISION_BITS,
        (((n1 * (((n1 * e11c) + (n2 * e12c)) + (n3 * e13c)))
            + (n2 * (((n1 * e21c) + (n2 * e22c)) + (n3 * e23c))))
            + (n3 * (((n1 * e31c) + (n2 * e32c)) + (n3 * e33c)))),
    );
    if ((Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
        - Float::with_val(PRECISION_BITS, cosTheta))
        == Float::with_val(PRECISION_BITS, 0i32))
    {
        afunc = (0i32) as f64;
    } else {
        {
            if (k == 0i32) {
                afunc = (((resp).clone()
                    * (Float::with_val(PRECISION_BITS, omega_g) * (time).clone()).sin())
                    / ((Float::with_val(PRECISION_BITS, Float::parse("2.0").unwrap())
                        * Float::with_val(PRECISION_BITS, omega_g))
                        * (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                            - Float::with_val(PRECISION_BITS, cosTheta))))
                .to_f64();
            } else {
                if (k == 1i32) {
                    afunc = (((resc).clone()
                        * (Float::with_val(PRECISION_BITS, omega_g) * (time).clone()).sin())
                        / ((Float::with_val(PRECISION_BITS, Float::parse("2.0").unwrap())
                            * Float::with_val(PRECISION_BITS, omega_g))
                            * (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                                - Float::with_val(PRECISION_BITS, cosTheta))))
                    .to_f64();
                } else {
                    if (k == 2i32) {
                        afunc = (((resp).clone()
                            * (Float::with_val(PRECISION_BITS, omega_g) * (time).clone()).cos())
                            / ((Float::with_val(PRECISION_BITS, Float::parse("2.0").unwrap())
                                * Float::with_val(PRECISION_BITS, omega_g))
                                * (Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap())
                                    - Float::with_val(PRECISION_BITS, cosTheta))))
                        .to_f64();
                    } else {
                        if (k == 3i32) {
                            afunc = (((resc).clone()
                                * (Float::with_val(PRECISION_BITS, omega_g) * (time).clone())
                                    .cos())
                                / ((Float::with_val(
                                    PRECISION_BITS,
                                    Float::parse("2.0").unwrap(),
                                ) * Float::with_val(PRECISION_BITS, omega_g))
                                    * (Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1.0").unwrap(),
                                    ) - Float::with_val(PRECISION_BITS, cosTheta))))
                            .to_f64();
                        }
                    }
                }
            }
        }
    }
    return Ok(afunc);
}

/// t2fit_gw.C::t2UpdateFunc_gwsingle (pinned source line 421).
pub(crate) fn t2UpdateFunc_gwsingle(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut err: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    match k {
        source_case if source_case == 0 => {
            psr.gwsrc_aplus_r = Some((*read(&psr.gwsrc_aplus_r, "psr[p].gwsrc_aplus_r")? - val));
        }
        source_case if source_case == 1 => {
            psr.gwsrc_across_r = Some((*read(&psr.gwsrc_across_r, "psr[p].gwsrc_across_r")? - val));
        }
        source_case if source_case == 2 => {
            psr.gwsrc_aplus_i = Some((*read(&psr.gwsrc_aplus_i, "psr[p].gwsrc_aplus_i")? - val));
        }
        source_case if source_case == 3 => {
            psr.gwsrc_across_i = Some((*read(&psr.gwsrc_across_i, "psr[p].gwsrc_across_i")? - val));
        }
        _ => {}
    }
    Ok(())
}
