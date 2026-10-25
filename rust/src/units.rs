//! units.C::scale_param/xform_mjd/transform_units, source order and Rug128.
use crate::{
    PRECISION_BITS, TimingError,
    ifteph::{IFTE_KM1, IFTE_MJD0},
    observation::{read, read_string},
    pulsar::SourcePulsar,
    tempo2::{Label::*, SourceParameter},
};
use rug::{Float, ops::Pow};
pub fn scale_param(p: &mut SourceParameter, i: usize, f: &Float) -> Result<(), TimingError> {
    let c = p
        .cells
        .get_mut(i)
        .ok_or_else(|| crate::clock_diagnostics::boundary("scale_param_index"))?;
    c.val *= f;
    c.err *= f;
    *c.prefit
        .as_mut()
        .ok_or_else(|| crate::clock_diagnostics::boundary("scale_param_prefit_unwritten"))? *= f;
    *c.prefit_err
        .as_mut()
        .ok_or_else(|| crate::clock_diagnostics::boundary("scale_param_prefitErr_unwritten"))? *= f;
    Ok(())
}
pub fn xform_mjd(p: &mut SourceParameter, i: usize, f: &Float) -> Result<(), TimingError> {
    let prefit = p.cells[0].prefit.clone();
    let c = p
        .cells
        .get_mut(i)
        .ok_or_else(|| crate::clock_diagnostics::boundary("xform_mjd_index"))?;
    c.val = (c.val.clone() - IFTE_MJD0) * f + IFTE_MJD0;
    c.err *= f;
    c.prefit = Some(
        (prefit.ok_or_else(|| crate::clock_diagnostics::boundary("xform_mjd_prefit_unwritten"))?
            - IFTE_MJD0)
            * f
            + IFTE_MJD0,
    );
    *c.prefit_err
        .as_mut()
        .ok_or_else(|| crate::clock_diagnostics::boundary("xform_mjd_prefitErr_unwritten"))? *= f;
    Ok(())
}
pub fn transform_units(p: &mut SourcePulsar, from: i32, to: i32) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| transform_units_inner(p, from, to, &mut s.diagnostics))
}
pub(crate) fn transform_units_inner(
    p: &mut SourcePulsar,
    from: i32,
    to: i32,
    d: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<(), TimingError> {
    d.version("units.C", "transform_units()");
    let k = Float::with_val(PRECISION_BITS, 1) + IFTE_KM1;
    let f = if to == 2 {
        Float::with_val(PRECISION_BITS, 1)
    } else {
        k.clone()
    } / if from == 2 {
        Float::with_val(PRECISION_BITS, 1)
    } else {
        k
    };
    let one = Float::with_val(PRECISION_BITS, 1);
    let inv = one.clone() / &f;
    let mut power = f.clone();
    for i in 0..10 {
        scale_param(
            &mut p.param.slots[param_f as usize],
            i,
            &(one.clone() / &power),
        )?;
        power *= &f;
    }
    let exponent = (one.clone() + &p.param.slots[param_fddi as usize].cells[0].val).to_f64();
    scale_param(
        &mut p.param.slots[param_fddc as usize],
        0,
        &f.clone().pow(exponent),
    )?;
    let mut v = inv.clone();
    for i in 0..p.param.slots[param_dm as usize].a_size.unwrap() as usize {
        scale_param(&mut p.param.slots[param_dm as usize], i, &v)?;
        v /= &f;
    }
    let mut v = f.clone().pow(1.0 - *read(&p.TNChromIdx, "TNChromIdx")?);
    for i in 0..p.param.slots[param_cm as usize].a_size.unwrap() as usize {
        scale_param(&mut p.param.slots[param_cm as usize], i, &v)?;
        v /= &f;
    }
    for label in [param_pmrv, param_pmra, param_pmdec] {
        scale_param(&mut p.param.slots[label as usize], 0, &inv)?;
    }
    for label in [param_pmra2, param_pmdec2] {
        scale_param(&mut p.param.slots[label as usize], 0, &(inv.clone() / &f))?;
    }
    for label in [
        param_posepoch,
        param_pepoch,
        param_start,
        param_finish,
        param_t0,
    ] {
        xform_mjd(&mut p.param.slots[label as usize], 0, &f)?;
    }
    for (label, factor) in [
        (param_pb, &f),
        (param_fb, &inv),
        (param_a1, &f),
        (param_edot, &inv),
        (param_omdot, &inv),
    ] {
        scale_param(&mut p.param.slots[label as usize], 0, factor)?;
    }
    xform_mjd(&mut p.param.slots[param_tasc as usize], 0, &f)?;
    if read_string(&p.tzrsite[..], "tzrsite")? == b"@" {
        xform_mjd(&mut p.param.slots[param_tzrmjd as usize], 0, &f)?;
        scale_param(&mut p.param.slots[param_tzrfrq as usize], 0, &inv)?;
    }
    for i in 0..p.param.slots[param_bpjep as usize].a_size.unwrap() as usize {
        xform_mjd(&mut p.param.slots[param_bpjep as usize], i, &f)?;
        scale_param(&mut p.param.slots[param_bpja1 as usize], i, &f)?;
        scale_param(&mut p.param.slots[param_bpjpb as usize], i, &f)?;
    }
    Ok(())
}
