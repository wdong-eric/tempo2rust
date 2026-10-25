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
    clock_diagnostics::Diagnostics,
    observation::{SourceObservation, read},
    pulsar::SourcePulsar,
    tempo2::Label::*,
};
use rug::{Float, ops::Pow};
/// constraints.C::consFunc_dmmodel_mean (pinned source line 302).
pub(crate) fn consFunc_dmmodel_mean(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut i: i32,
    mut k: i32,
    mut order: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if ((i == (param_dmmodel as i32)) && (k < *read(&psr.dmoffsDMnum, "psr->dmoffsDMnum")?)) {
        {
            return Ok(*read(
                &psr.dmoffsDM_weight[(k) as usize],
                "psr->dmoffsDM_weight[k]",
            )?);
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints.C::consFunc_dmmodel_dm1 (pinned source line 314).
pub(crate) fn consFunc_dmmodel_dm1(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut i: i32,
    mut k: i32,
    mut order: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut epoch: Float =
        (psr.param.slots[(param_dmepoch as i32) as usize].cells[(0i32) as usize].val).clone();
    let mut nDM: i32 = *read(&psr.dmoffsDMnum, "psr->dmoffsDMnum")?;
    d.stdout(format!("WE ARE IN HERE\n").as_bytes());
    if ((i == (param_dmmodel as i32)) && (k < nDM)) {
        {
            return Ok((Float::with_val(
                PRECISION_BITS,
                *read(
                    &psr.dmoffsDM_weight[(k) as usize],
                    "psr->dmoffsDM_weight[k]",
                )?,
            ) * (Float::with_val(
                PRECISION_BITS,
                *read(&psr.dmoffsDM_mjd[(k) as usize], "psr->dmoffsDM_mjd[k]")?,
            ) - (epoch).clone()))
            .to_f64());
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints.C::consFunc_dmmodel_cw (pinned source line 329).
pub(crate) fn consFunc_dmmodel_cw(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut i: i32,
    mut k: i32,
    mut order: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut nDM: i32 = *read(&psr.dmoffsDMnum, "psr->dmoffsDMnum")?;
    if ((i == (param_dmmodel as i32)) && (k >= nDM)) {
        {
            k = (k - nDM);
            let mut epoch: Float =
                (psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize].val)
                    .clone();
            let mut w: Float = Float::with_val(
                PRECISION_BITS,
                *read(
                    &psr.dmoffsCM_weight[(k) as usize],
                    "psr->dmoffsCM_weight[k]",
                )?,
            );
            return Ok(((w).clone()
                * (Float::with_val(
                    PRECISION_BITS,
                    *read(&psr.dmoffsCM_mjd[(k) as usize], "psr->dmoffsCM_mjd[k]")?,
                ) - (epoch).clone())
                .pow(&(Float::with_val(PRECISION_BITS, order))))
            .to_f64());
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints.C::consFunc_dmmodel_cw_year (pinned source line 345).
pub(crate) fn consFunc_dmmodel_cw_year(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut i: i32,
    mut k: i32,
    mut order: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut nDM: i32 = *read(&psr.dmoffsDMnum, "psr->dmoffsDMnum")?;
    if ((i == (param_dmmodel as i32)) && (k >= nDM)) {
        {
            k = (k - nDM);
            let mut rc: f64;
            let mut s: [f64; 3] = [0.0; 3];
            let mut epoch: Float =
                (psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize].val)
                    .clone();
            let mut t: Float = (Float::with_val(
                PRECISION_BITS,
                *read(&psr.dmoffsCM_mjd[(k) as usize], "psr->dmoffsCM_mjd[k]")?,
            ) - (epoch).clone());
            let mut x: Float = (((Float::with_val(PRECISION_BITS, Float::parse("2.0").unwrap())
                * Float::with_val(PRECISION_BITS, rug::float::Constant::Pi))
                * (t).clone())
                / Float::with_val(PRECISION_BITS, Float::parse("365.25").unwrap()));
            let mut w: Float = Float::with_val(
                PRECISION_BITS,
                *read(
                    &psr.dmoffsCM_weight[(k) as usize],
                    "psr->dmoffsCM_weight[k]",
                )?,
            );
            match order {
                source_case if source_case == 0 => {
                    return Ok(((w).clone() * ((x).clone()).sin()).to_f64());
                }
                source_case if source_case == 1 => {
                    return Ok(((w).clone() * ((x).clone()).cos()).to_f64());
                }
                source_case if source_case == 2 => {
                    return Ok((((w).clone() * (x).clone()) * ((x).clone()).sin()).to_f64());
                }
                source_case if source_case == 3 => {
                    return Ok((((w).clone() * (x).clone()) * ((x).clone()).cos()).to_f64());
                }
                source_case if source_case == 4 => {
                    return Ok(((w).clone()
                        * (Float::with_val(PRECISION_BITS, 2i32) * (x).clone()).sin())
                    .to_f64());
                }
                source_case if source_case == 5 => {
                    return Ok(((w).clone()
                        * (Float::with_val(PRECISION_BITS, 2i32) * (x).clone()).cos())
                    .to_f64());
                }
                source_case if source_case == 6 => {
                    t = Float::with_val(
                        PRECISION_BITS,
                        (*read(&psr.dmoffsCM_mjd[(k) as usize], "psr->dmoffsCM_mjd[k]")?
                            - 52995.0f64),
                    );
                    x = (((Float::with_val(PRECISION_BITS, 2i32)
                        * Float::with_val(PRECISION_BITS, rug::float::Constant::Pi))
                        * (t).clone())
                        / Float::with_val(PRECISION_BITS, 365.25f64));
                    s[(0i32) as usize] =
                        (Float::with_val(PRECISION_BITS, (-500i32)) * ((x).clone()).sin()).to_f64();
                    s[(1i32) as usize] =
                        (Float::with_val(PRECISION_BITS, 450i32) * ((x).clone()).cos()).to_f64();
                    s[(2i32) as usize] =
                        (Float::with_val(PRECISION_BITS, 200i32) * ((x).clone()).cos()).to_f64();
                    rc = (crate::tempo2_util::dot_product(
                        [
                            *read(&psr.posPulsar[0], "vector")?,
                            *read(&psr.posPulsar[1], "vector")?,
                            *read(&psr.posPulsar[2], "vector")?,
                        ],
                        [s[0], s[1], s[2]],
                    ) / (crate::tempo2_util::dot_product(
                        [
                            *read(&psr.posPulsar[0], "vector")?,
                            *read(&psr.posPulsar[1], "vector")?,
                            *read(&psr.posPulsar[2], "vector")?,
                        ],
                        [
                            *read(&psr.posPulsar[0], "vector")?,
                            *read(&psr.posPulsar[1], "vector")?,
                            *read(&psr.posPulsar[2], "vector")?,
                        ],
                    ) * crate::tempo2_util::dot_product(
                        [s[0], s[1], s[2]],
                        [s[0], s[1], s[2]],
                    ))
                    .sqrt());
                    return Ok(((w).clone() * Float::with_val(PRECISION_BITS, rc)).to_f64());
                }
                _ => {
                    return Ok((0i32) as f64);
                }
            }
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints.C::consFunc_tel_dx (pinned source line 389).
pub(crate) fn consFunc_tel_dx(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut i: i32,
    mut k: i32,
    mut order: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if (i == (param_tel_dx as i32)) {
        {
            let mut epoch: Float =
                (psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize].val)
                    .clone();
            return Ok(((Float::with_val(
                PRECISION_BITS,
                *read(&psr.telDX_t[(k) as usize], "psr->telDX_t[k]")?,
            ) - (epoch).clone())
            .pow(&(Float::with_val(PRECISION_BITS, order))))
            .to_f64());
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints.C::consFunc_tel_dy (pinned source line 398).
pub(crate) fn consFunc_tel_dy(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut i: i32,
    mut k: i32,
    mut order: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if (i == (param_tel_dy as i32)) {
        {
            let mut epoch: Float =
                (psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize].val)
                    .clone();
            return Ok(((Float::with_val(
                PRECISION_BITS,
                *read(&psr.telDY_t[(k) as usize], "psr->telDY_t[k]")?,
            ) - (epoch).clone())
            .pow(&(Float::with_val(PRECISION_BITS, order))))
            .to_f64());
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints.C::consFunc_tel_dz (pinned source line 407).
pub(crate) fn consFunc_tel_dz(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut i: i32,
    mut k: i32,
    mut order: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if (i == (param_tel_dz as i32)) {
        {
            let mut epoch: Float =
                (psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize].val)
                    .clone();
            return Ok(((Float::with_val(
                PRECISION_BITS,
                *read(&psr.telDZ_t[(k) as usize], "psr->telDZ_t[k]")?,
            ) - (epoch).clone())
            .pow(&(Float::with_val(PRECISION_BITS, order))))
            .to_f64());
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints.C::consFunc_ifunc_X0 (pinned source line 417).
pub(crate) fn consFunc_ifunc_X0(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut i: i32,
    mut k: i32,
    mut order: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if ((i == (param_ifunc as i32)) && (k == 0i32)) {
        {
            return Ok((1i32) as f64);
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints.C::consFunc_ifunc (pinned source line 428).
pub(crate) fn consFunc_ifunc(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut i: i32,
    mut k: i32,
    mut order: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if (i == (param_ifunc as i32)) {
        {
            let mut epoch: Float =
                (psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize].val)
                    .clone();
            return Ok((Float::with_val(
                PRECISION_BITS,
                *read(&psr.ifunc_weights[(k) as usize], "psr->ifunc_weights[k]")?,
            ) * (Float::with_val(
                PRECISION_BITS,
                *read(&psr.ifuncT[(k) as usize], "psr->ifuncT[k]")?,
            ) - (epoch).clone())
            .pow(&(Float::with_val(PRECISION_BITS, order))))
            .to_f64());
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints.C::consFunc_qifunc_p_year (pinned source line 443).
pub(crate) fn consFunc_qifunc_p_year(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut i: i32,
    mut k: i32,
    mut order: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if ((i == (param_quad_ifunc_p as i32))
        && (k < *read(&psr.quad_ifuncN_p, "psr->quad_ifuncN_p")?))
    {
        {
            let mut epoch: Float =
                (psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize].val)
                    .clone();
            let mut t: Float = (Float::with_val(
                PRECISION_BITS,
                *read(
                    &psr.quad_ifuncT_p
                        [(k % *read(&psr.quad_ifuncN_p, "psr->quad_ifuncN_p")?) as usize],
                    "psr->quad_ifuncT_p[k % psr->quad_ifuncN_p]",
                )?,
            ) - (epoch).clone());
            let mut x: Float = (((Float::with_val(PRECISION_BITS, 2.0f64)
                * Float::with_val(PRECISION_BITS, rug::float::Constant::Pi))
                * (t).clone())
                / Float::with_val(PRECISION_BITS, 365.25f64));
            match order {
                source_case if source_case == 0 => {
                    return Ok((((x).clone()).sin()).to_f64());
                }
                source_case if source_case == 1 => {
                    return Ok((((x).clone()).cos()).to_f64());
                }
                source_case if source_case == 2 => {
                    return Ok(((x).clone() * ((x).clone()).sin()).to_f64());
                }
                source_case if source_case == 3 => {
                    return Ok(((x).clone() * ((x).clone()).cos()).to_f64());
                }
                source_case if source_case == 4 => {
                    return Ok(
                        ((Float::with_val(PRECISION_BITS, 2i32) * (x).clone()).sin()).to_f64(),
                    );
                }
                source_case if source_case == 5 => {
                    return Ok(
                        ((Float::with_val(PRECISION_BITS, 2i32) * (x).clone()).cos()).to_f64(),
                    );
                }
                _ => {
                    return Ok((0i32) as f64);
                }
            }
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints.C::consFunc_qifunc_c_year (pinned source line 473).
pub(crate) fn consFunc_qifunc_c_year(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut i: i32,
    mut k: i32,
    mut order: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if ((i == (param_quad_ifunc_c as i32))
        && (k < *read(&psr.quad_ifuncN_c, "psr->quad_ifuncN_c")?))
    {
        {
            let mut epoch: Float =
                (psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize].val)
                    .clone();
            let mut t: Float = (Float::with_val(
                PRECISION_BITS,
                *read(
                    &psr.quad_ifuncT_c
                        [(k % *read(&psr.quad_ifuncN_c, "psr->quad_ifuncN_c")?) as usize],
                    "psr->quad_ifuncT_c[k % psr->quad_ifuncN_c]",
                )?,
            ) - (epoch).clone());
            let mut x: Float = (((Float::with_val(PRECISION_BITS, 2.0f64)
                * Float::with_val(PRECISION_BITS, rug::float::Constant::Pi))
                * (t).clone())
                / Float::with_val(PRECISION_BITS, 365.25f64));
            match order {
                source_case if source_case == 0 => {
                    return Ok((((x).clone()).sin()).to_f64());
                }
                source_case if source_case == 1 => {
                    return Ok((((x).clone()).cos()).to_f64());
                }
                source_case if source_case == 2 => {
                    return Ok(((x).clone() * ((x).clone()).sin()).to_f64());
                }
                source_case if source_case == 3 => {
                    return Ok(((x).clone() * ((x).clone()).cos()).to_f64());
                }
                source_case if source_case == 4 => {
                    return Ok(
                        ((Float::with_val(PRECISION_BITS, 2i32) * (x).clone()).sin()).to_f64(),
                    );
                }
                source_case if source_case == 5 => {
                    return Ok(
                        ((Float::with_val(PRECISION_BITS, 2i32) * (x).clone()).cos()).to_f64(),
                    );
                }
                _ => {
                    return Ok((0i32) as f64);
                }
            }
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints.C::consFunc_ifunc_year (pinned source line 502).
pub(crate) fn consFunc_ifunc_year(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut i: i32,
    mut k: i32,
    mut order: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if ((i == (param_ifunc as i32)) && (k < *read(&psr.ifuncN, "psr->ifuncN")?)) {
        {
            let mut epoch: Float =
                (psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize].val)
                    .clone();
            let mut t: Float = (Float::with_val(
                PRECISION_BITS,
                *read(
                    &psr.ifuncT[(k % *read(&psr.ifuncN, "psr->ifuncN")?) as usize],
                    "psr->ifuncT[k % psr->ifuncN]",
                )?,
            ) - (epoch).clone());
            let mut x: Float = (((Float::with_val(PRECISION_BITS, 2.0f64)
                * Float::with_val(PRECISION_BITS, rug::float::Constant::Pi))
                * (t).clone())
                / Float::with_val(PRECISION_BITS, 365.25f64));
            match order {
                source_case if source_case == 0 => {
                    return Ok((Float::with_val(
                        PRECISION_BITS,
                        *read(&psr.ifunc_weights[(k) as usize], "psr->ifunc_weights[k]")?,
                    ) * ((x).clone()).sin())
                    .to_f64());
                }
                source_case if source_case == 1 => {
                    return Ok((Float::with_val(
                        PRECISION_BITS,
                        *read(&psr.ifunc_weights[(k) as usize], "psr->ifunc_weights[k]")?,
                    ) * ((x).clone()).cos())
                    .to_f64());
                }
                source_case if source_case == 2 => {
                    return Ok(((Float::with_val(
                        PRECISION_BITS,
                        *read(&psr.ifunc_weights[(k) as usize], "psr->ifunc_weights[k]")?,
                    ) * (x).clone())
                        * ((x).clone()).sin())
                    .to_f64());
                }
                source_case if source_case == 3 => {
                    return Ok(((Float::with_val(
                        PRECISION_BITS,
                        *read(&psr.ifunc_weights[(k) as usize], "psr->ifunc_weights[k]")?,
                    ) * (x).clone())
                        * ((x).clone()).cos())
                    .to_f64());
                }
                source_case if source_case == 4 => {
                    return Ok((Float::with_val(
                        PRECISION_BITS,
                        *read(&psr.ifunc_weights[(k) as usize], "psr->ifunc_weights[k]")?,
                    ) * (Float::with_val(PRECISION_BITS, 2i32) * (x).clone()).sin())
                    .to_f64());
                }
                source_case if source_case == 5 => {
                    return Ok((Float::with_val(
                        PRECISION_BITS,
                        *read(&psr.ifunc_weights[(k) as usize], "psr->ifunc_weights[k]")?,
                    ) * (Float::with_val(PRECISION_BITS, 2i32) * (x).clone()).cos())
                    .to_f64());
                }
                _ => {
                    return Ok((0i32) as f64);
                }
            }
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints.C::consFunc_quad_ifunc_p (pinned source line 533).
pub(crate) fn consFunc_quad_ifunc_p(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut i: i32,
    mut k: i32,
    mut order: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if (i == (param_quad_ifunc_p as i32)) {
        {
            let mut epoch: Float =
                (psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize].val)
                    .clone();
            return Ok(((Float::with_val(
                PRECISION_BITS,
                *read(&psr.quad_ifuncT_p[(k) as usize], "psr->quad_ifuncT_p[k]")?,
            ) - (epoch).clone())
            .pow(&(Float::with_val(PRECISION_BITS, order))))
            .to_f64());
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints.C::consFunc_quad_ifunc_c (pinned source line 545).
pub(crate) fn consFunc_quad_ifunc_c(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut i: i32,
    mut k: i32,
    mut order: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if (i == (param_quad_ifunc_c as i32)) {
        {
            let mut epoch: Float =
                (psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize].val)
                    .clone();
            return Ok(((Float::with_val(
                PRECISION_BITS,
                *read(&psr.quad_ifuncT_c[(k) as usize], "psr->quad_ifuncT_c[k]")?,
            ) - (epoch).clone())
            .pow(&(Float::with_val(PRECISION_BITS, order))))
            .to_f64());
        }
    } else {
        return Ok((0i32) as f64);
    }
}
