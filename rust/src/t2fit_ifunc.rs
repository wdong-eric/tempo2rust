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
    TimingError,
    clock_diagnostics::{Diagnostics, boundary},
    observation::{SourceObservation, read},
    pulsar::SourcePulsar,
    tempo2::Label::*,
};
/// t2fit_ifunc.C::t2FitFunc_sifunc (pinned source line 8).
pub(crate) fn t2FitFunc_sifunc(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut ret: f64 = (0i32) as f64;
    match label {
        source_case if source_case == (param_ifunc as i32) => {
            ret = crate::ifunc::sinfunc(
                &psr.ifuncT
                    .iter()
                    // sinfunc reads the first two knots and the kth knot.
                    .take((k as usize + 1).max(2))
                    .map(|v| read(v, "ifuncT").copied())
                    .collect::<Result<Vec<_>, _>>()?,
                ((read(&rows[(ipos) as usize].bat, "psr[p].obsn[ipos].bat")?).clone()).to_f64(),
                k as usize,
            )?;
        }
        source_case if source_case == (param_quad_ifunc_p as i32) => {
            ret = (*read(&psr.quad_ifunc_geom_p, "psr[p].quad_ifunc_geom_p")?
                * crate::ifunc::sinfunc(
                    &psr.quad_ifuncT_p
                        .iter()
                        .take((k as usize + 1).max(2))
                        .map(|v| read(v, "quad_ifuncT_p").copied())
                        .collect::<Result<Vec<_>, _>>()?,
                    ((read(&rows[(ipos) as usize].bat, "psr[p].obsn[ipos].bat")?).clone()).to_f64(),
                    k as usize,
                )?);
        }
        source_case if source_case == (param_quad_ifunc_c as i32) => {
            ret = (*read(&psr.quad_ifunc_geom_c, "psr[p].quad_ifunc_geom_c")?
                * crate::ifunc::sinfunc(
                    &psr.quad_ifuncT_c
                        .iter()
                        .take((k as usize + 1).max(2))
                        .map(|v| read(v, "quad_ifuncT_c").copied())
                        .collect::<Result<Vec<_>, _>>()?,
                    ((read(&rows[(ipos) as usize].bat, "psr[p].obsn[ipos].bat")?).clone()).to_f64(),
                    k as usize,
                )?);
        }
        _ => {
            return Err(boundary("t2fit_ifunc.C::t2FitFunc_sifunc: assertion"));
        }
    }
    return Ok(ret);
}

/// t2fit_ifunc.C::t2FitFunc_ifunc (pinned source line 29).
pub(crate) fn t2FitFunc_ifunc(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut ret: f64 = (0i32) as f64;
    match label {
        source_case if source_case == (param_ifunc as i32) => {
            ret = crate::ifunc::basis(
                &psr.ifuncT
                    .iter()
                    .take((*read(&psr.ifuncN, "psr[p].ifuncN")?) as usize)
                    .map(|v| read(v, "ifuncT").copied())
                    .collect::<Result<Vec<_>, _>>()?,
                ((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()).to_f64(),
                *read(&psr.ifuncN, "psr[p].ifuncN")?,
                k,
                d,
            )?;
        }
        source_case if source_case == (param_quad_ifunc_p as i32) => {
            ret = (*read(&psr.quad_ifunc_geom_p, "psr[p].quad_ifunc_geom_p")?
                * crate::ifunc::basis(
                    &psr.quad_ifuncT_p
                        .iter()
                        .take((*read(&psr.quad_ifuncN_p, "psr[p].quad_ifuncN_p")?) as usize)
                        .map(|v| read(v, "quad_ifuncT_p").copied())
                        .collect::<Result<Vec<_>, _>>()?,
                    ((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()).to_f64(),
                    *read(&psr.quad_ifuncN_p, "psr[p].quad_ifuncN_p")?,
                    k,
                    d,
                )?);
        }
        source_case if source_case == (param_quad_ifunc_c as i32) => {
            ret = (*read(&psr.quad_ifunc_geom_c, "psr[p].quad_ifunc_geom_c")?
                * crate::ifunc::basis(
                    &psr.quad_ifuncT_c
                        .iter()
                        .take((*read(&psr.quad_ifuncN_c, "psr[p].quad_ifuncN_c")?) as usize)
                        .map(|v| read(v, "quad_ifuncT_c").copied())
                        .collect::<Result<Vec<_>, _>>()?,
                    ((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()).to_f64(),
                    *read(&psr.quad_ifuncN_c, "psr[p].quad_ifuncN_c")?,
                    k,
                    d,
                )?);
        }
        _ => {
            return Err(boundary("t2fit_ifunc.C::t2FitFunc_ifunc: assertion"));
        }
    }
    return Ok(ret);
}

/// t2fit_ifunc.C::t2UpdateFunc_ifunc (pinned source line 49).
pub(crate) fn t2UpdateFunc_ifunc(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut err: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    match label {
        source_case if source_case == (param_ifunc as i32) => {
            psr.ifuncV[(k) as usize] =
                Some((*read(&psr.ifuncV[(k) as usize], "psr[p].ifuncV[k]")? - val));
            psr.ifuncE[(k) as usize] = Some(err);
        }
        source_case if source_case == (param_quad_ifunc_p as i32) => {
            psr.quad_ifuncV_p[(k) as usize] =
                Some((*read(&psr.quad_ifuncV_p[(k) as usize], "psr[p].quad_ifuncV_p[k]")? - val));
            psr.quad_ifuncE_p[(k) as usize] = Some(err);
        }
        source_case if source_case == (param_quad_ifunc_c as i32) => {
            psr.quad_ifuncV_c[(k) as usize] =
                Some((*read(&psr.quad_ifuncV_c[(k) as usize], "psr[p].quad_ifuncV_c[k]")? - val));
            psr.quad_ifuncE_c[(k) as usize] = Some(err);
        }
        _ => {
            return Err(boundary("t2fit_ifunc.C::t2UpdateFunc_ifunc: assertion"));
        }
    }
    Ok(())
}
