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
};
/// t2fit_dmmodel.C::t2FitFunc_dmmodelDM (pinned source line 9).
pub(crate) fn t2FitFunc_dmmodelDM(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !(k < *read(&psr.dmoffsDMnum, "psr[p].dmoffsDMnum")?) {
        return Err(boundary("t2fit_dmmodel.C::t2FitFunc_dmmodelDM: assertion"));
    }
    let mut dmf: f64 = (1.0f64
        / (2.41e-4f64
            * (*read(&rows[(ipos) as usize].freqSSB, "psr[p].obsn[ipos].freqSSB")? / 1.0e6f64)
                .powf((2i32) as f64)));
    return Ok((dmf
        * crate::ifunc::basis(
            &psr.dmoffsDM_mjd
                .iter()
                .take((*read(&psr.dmoffsDMnum, "psr[p].dmoffsDMnum")?) as usize)
                .map(|v| read(v, "dmoffsDM_mjd").copied())
                .collect::<Result<Vec<_>, _>>()?,
            ((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()).to_f64(),
            *read(&psr.dmoffsDMnum, "psr[p].dmoffsDMnum")?,
            k,
            d,
        )?));
}

/// t2fit_dmmodel.C::t2UpdateFunc_dmmodelDM (pinned source line 14).
pub(crate) fn t2UpdateFunc_dmmodelDM(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut err: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    if !(k < *read(&psr.dmoffsDMnum, "psr[p].dmoffsDMnum")?) {
        return Err(boundary(
            "t2fit_dmmodel.C::t2UpdateFunc_dmmodelDM: assertion",
        ));
    }
    psr.dmoffsDM[(k) as usize] =
        Some((*read(&psr.dmoffsDM[(k) as usize], "psr[p].dmoffsDM[k]")? + val));
    psr.dmoffsDM_error[(k) as usize] = Some(err);
    Ok(())
}

/// t2fit_dmmodel.C::t2FitFunc_dmmodelCM (pinned source line 19).
pub(crate) fn t2FitFunc_dmmodelCM(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut idx: i32 = (k - *read(&psr.dmoffsDMnum, "psr[p].dmoffsDMnum")?);
    if !(idx < *read(&psr.dmoffsCMnum, "psr[p].dmoffsCMnum")?) {
        return Err(boundary("t2fit_dmmodel.C::t2FitFunc_dmmodelCM: assertion"));
    }
    return crate::ifunc::basis(
        &psr.dmoffsCM_mjd
            .iter()
            .take((*read(&psr.dmoffsCMnum, "psr[p].dmoffsCMnum")?) as usize)
            .map(|v| read(v, "dmoffsCM_mjd").copied())
            .collect::<Result<Vec<_>, _>>()?,
        ((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?).clone()).to_f64(),
        *read(&psr.dmoffsCMnum, "psr[p].dmoffsCMnum")?,
        idx,
        d,
    );
}

/// t2fit_dmmodel.C::t2UpdateFunc_dmmodelCM (pinned source line 24).
pub(crate) fn t2UpdateFunc_dmmodelCM(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut err: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    let mut idx: i32 = (k - *read(&psr.dmoffsDMnum, "psr[p].dmoffsDMnum")?);
    if !(idx < *read(&psr.dmoffsCMnum, "psr[p].dmoffsCMnum")?) {
        return Err(boundary(
            "t2fit_dmmodel.C::t2UpdateFunc_dmmodelCM: assertion",
        ));
    }
    psr.dmoffsCM[(idx) as usize] = Some(val);
    psr.dmoffsCM_error[(idx) as usize] = Some(err);
    Ok(())
}
