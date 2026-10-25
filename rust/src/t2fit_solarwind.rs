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
    clock_diagnostics::{Diagnostics, boundary},
    observation::{SourceObservation, read},
    pulsar::SourcePulsar,
    tempo2::Label::*,
};
use rug::Float;
/// t2fit_solarwind.C::t2FitFunc_ne_sw_sin (pinned source line 6).
pub(crate) fn t2FitFunc_ne_sw_sin(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !(label == (param_ne_sw_sin as i32)) {
        return Err(boundary(
            "t2fit_solarwind.C::t2FitFunc_ne_sw_sin: assertion",
        ));
    }
    return Ok(
        ((-((Float::with_val(PRECISION_BITS, ((2i32) as f64 * std::f64::consts::PI))
            * ((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()
                - Float::with_val(PRECISION_BITS, 58818.0f64)))
            / Float::with_val(PRECISION_BITS, 4018.0f64))
        .cos())
            * Float::with_val(
                PRECISION_BITS,
                *read(
                    &rows[(ipos) as usize].spherical_solar_wind,
                    "psr[p].obsn[ipos].spherical_solar_wind",
                )?,
            ))
        .to_f64(),
    );
}

/// t2fit_solarwind.C::t2FitFunc_ne_sw (pinned source line 11).
pub(crate) fn t2FitFunc_ne_sw(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !(label == (param_ne_sw as i32)) {
        return Err(boundary("t2fit_solarwind.C::t2FitFunc_ne_sw: assertion"));
    }
    return Ok(*read(
        &rows[(ipos) as usize].spherical_solar_wind,
        "psr[p].obsn[ipos].spherical_solar_wind",
    )?);
}

/// t2fit_solarwind.C::t2FitFunc_ne_sw_ifunc (pinned source line 16).
pub(crate) fn t2FitFunc_ne_sw_ifunc(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !(k < *read(&psr.ne_sw_ifuncN, "psr[p].ne_sw_ifuncN")?) {
        return Err(boundary(
            "t2fit_solarwind.C::t2FitFunc_ne_sw_ifunc: assertion",
        ));
    }
    if !(label == (param_ne_sw_ifunc as i32)) {
        return Err(boundary(
            "t2fit_solarwind.C::t2FitFunc_ne_sw_ifunc: assertion",
        ));
    }
    return Ok((*read(
        &rows[(ipos) as usize].spherical_solar_wind,
        "psr[p].obsn[ipos].spherical_solar_wind",
    )? * crate::ifunc::basis(
        &psr.ne_sw_ifuncT
            .iter()
            .take((*read(&psr.ne_sw_ifuncN, "psr[p].ne_sw_ifuncN")?) as usize)
            .map(|v| read(v, "ne_sw_ifuncT").copied())
            .collect::<Result<Vec<_>, _>>()?,
        ((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()).to_f64(),
        *read(&psr.ne_sw_ifuncN, "psr[p].ne_sw_ifuncN")?,
        k,
        d,
    )?));
}

/// t2fit_solarwind.C::t2UpdateFunc_ne_sw_ifunc (pinned source line 23).
pub(crate) fn t2UpdateFunc_ne_sw_ifunc(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut error: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    if !(k < *read(&psr.ne_sw_ifuncN, "psr[p].ne_sw_ifuncN")?) {
        return Err(boundary(
            "t2fit_solarwind.C::t2UpdateFunc_ne_sw_ifunc: assertion",
        ));
    }
    psr.ne_sw_ifuncV[(k) as usize] =
        Some((*read(&psr.ne_sw_ifuncV[(k) as usize], "psr[p].ne_sw_ifuncV[k]")? + val));
    if (error >= (0i32) as f64) {
        psr.ne_sw_ifuncE[(k) as usize] = Some(error);
    }
    Ok(())
}

/// t2fit_solarwind.C::t2UpdateFunc_ne_sw (pinned source line 29).
pub(crate) fn t2UpdateFunc_ne_sw(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut error: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    if !(label == (param_ne_sw as i32)) {
        return Err(boundary("t2fit_solarwind.C::t2UpdateFunc_ne_sw: assertion"));
    }
    psr.param.slots[(label) as usize].cells[(k) as usize].val =
        ((psr.param.slots[(label) as usize].cells[(k) as usize].val).clone()
            + Float::with_val(PRECISION_BITS, val));
    if (error >= (0i32) as f64) {
        psr.param.slots[(label) as usize].cells[(k) as usize].err =
            Float::with_val(PRECISION_BITS, error);
    }
    psr.ne_sw =
        Some(((psr.param.slots[(label) as usize].cells[(k) as usize].val).clone()).to_f64());
    Ok(())
}
