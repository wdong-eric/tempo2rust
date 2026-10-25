//! Binary dispatch from formResiduals.C and the two binary callbacks in
//! t2fit_stdFitFuncs.C. Model-specific scientific bodies live in source-named modules.
use crate::{
    PRECISION_BITS, TimingError,
    clock_diagnostics::{Diagnostics, boundary},
    observation::{SourceObservation, read, read_string},
    pulsar::SourcePulsar,
    pulsar_lifecycle::PulsarContext,
    tempo2::{Label, Label::*},
};
use rug::{Float, ops::Pow};
mod bt;
mod btj;
mod btx;
mod dd;
mod ddgr;
mod ddh;
mod ddk;
mod dds;
mod ell1;
mod ell1h;
mod ell1k;
mod mss;
mod t2;
mod t2_pta;

pub(crate) fn source_i32(v: f64) -> Result<i32, TimingError> {
    let t = v.trunc();
    if t.is_finite() && t >= i32::MIN as f64 && t <= i32::MAX as f64 {
        Ok(t as i32)
    } else {
        Err(boundary("binary_float_to_int"))
    }
}

/// Direct source model call, including native DD's extended return.
pub fn model(
    context: &mut PulsarContext,
    pulsar: usize,
    observation: usize,
    selector: i32,
    subindex: usize,
) -> Result<Float, TimingError> {
    crate::clkcorr::with_process(|s| {
        let (p, rows) = context.pulsar_observations_mut(pulsar)?;
        model_inner(p, rows, observation, selector, subindex, &mut s.diagnostics)
    })
}
pub(crate) fn model_inner(
    p: &mut SourcePulsar,
    rows: &[SourceObservation],
    i: usize,
    selector: i32,
    k: usize,
    d: &mut Diagnostics,
) -> Result<Float, TimingError> {
    if i >= rows.len() {
        return Err(boundary("binary_observation_index"));
    }
    let i = i32::try_from(i).map_err(|_| boundary("binary_observation_count"))?;
    let k = i32::try_from(k).map_err(|_| boundary("binary_subindex"))?;
    let name = read_string(&p.binaryModel[..], "binaryModel")?;
    if name == b"DD" {
        return dd::DDmodel(p, rows, i, selector, d);
    }
    let v = match name.as_slice() {
        b"BT" => bt::BTmodel(p, rows, i, selector, d),
        b"BTJ" => btj::BTJmodel(p, rows, i, selector, k, d),
        b"BTX" => btx::BTXmodel(p, rows, i, selector, k, d),
        b"DDK" => ddk::DDKmodel(p, rows, i, selector, d),
        b"DDS" => dds::DDSmodel(p, rows, i, selector, d),
        b"DDGR" => ddgr::DDGRmodel(p, rows, i, selector, d),
        b"DDH" => ddh::DDHmodel(p, rows, i, selector, d),
        b"MSS" => mss::MSSmodel(p, rows, i, selector, d),
        b"ELL1" => ell1::ELL1model(p, rows, i, selector, k, d),
        b"ELL1H" => ell1h::ELL1Hmodel(p, rows, i, selector, d),
        b"ELL1k" => ell1k::ELL1kmodel(p, rows, i, selector, d),
        b"T2" => t2::T2model(p, rows, i, selector, k, d),
        b"T2-PTA" => t2_pta::T2_PTAmodel(p, rows, i, selector, k, d),
        _ => {
            d.stdout(&[b"Warning: Unknown binary model '".as_slice(), &name, b"'\n"].concat());
            return Err(boundary("unknown_binary_model"));
        }
    }?;
    Ok(Float::with_val(PRECISION_BITS, v))
}
/// t2fit_stdFitFuncs.C::t2FitFunc_binaryModels: selected pulsar; f64 narrowing.
pub fn fit_function(
    context: &mut PulsarContext,
    pulsar: usize,
    observation: usize,
    label: Label,
    subindex: usize,
) -> Result<f64, TimingError> {
    crate::clkcorr::with_process(|s| {
        let (p, rows) = context.pulsar_observations_mut(pulsar)?;
        let value = model_inner(
            p,
            rows,
            observation,
            label as i32,
            subindex,
            &mut s.diagnostics,
        )?
        .to_f64();
        if s.diagnostics.debug {
            s.diagnostics.log(
                b'd',
                "t2fit_stdFitFuncs.C",
                157,
                &[
                    read_string(&p.name[..], "name")?,
                    format!("({pulsar}) ").into_bytes(),
                    read_string(&p.binaryModel[..], "binaryModel")?,
                    format!(
                        " {observation} {}({subindex}) ",
                        crate::tempo2::PARAMETER_NAMES[label as usize]
                    )
                    .into_bytes(),
                    crate::clock_text::format_number(
                        value,
                        crate::clock_text::NumberFormat::GeneralSix,
                    ),
                ]
                .concat(),
            );
        }
        Ok(value)
    })
}
/// t2fit_stdFitFuncs.C::t2UpdateFunc_binaryModels; each source owns its coordinates.
pub fn update(
    context: &mut PulsarContext,
    pulsar: usize,
    label: Label,
    subindex: usize,
    value: f64,
    error: f64,
) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| {
        let p = context
            .pulsars
            .get_mut(pulsar)
            .ok_or_else(|| boundary("binary_pulsar_index"))?;
        let name = read_string(&p.binaryModel[..], "binaryModel")?;
        let k = i32::try_from(subindex).map_err(|_| boundary("binary_subindex"))?;
        let l = label as i32;
        let d = &mut s.diagnostics;
        match name.as_slice() {
            b"BT" => bt::updateBT(p, &[], value, error, l, d),
            b"BTJ" => btj::updateBTJ(p, &[], value, error, l, k, d),
            b"BTX" => btx::updateBTX(p, &[], value, error, l, k, d),
            b"DD" => dd::updateDD(p, &[], value, error, l, d),
            b"DDK" => ddk::updateDDK(p, &[], value, error, l, d),
            b"DDS" => dds::updateDDS(p, &[], value, error, l, d),
            b"DDGR" => ddgr::updateDDGR(p, &[], value, error, l, d),
            b"DDH" => ddh::updateDDH(p, &[], value, error, l, d),
            b"MSS" => mss::updateMSS(p, &[], value, error, l, d),
            b"ELL1" => ell1::updateELL1(p, &[], value, error, l, k, d),
            b"ELL1H" => ell1h::updateELL1H(p, &[], value, error, l, d),
            b"ELL1k" => ell1k::updateELL1k(p, &[], value, error, l, d),
            b"T2" => t2::updateT2(p, &[], value, error, l, k, d),
            b"T2-PTA" => t2_pta::updateT2_PTA(p, &[], value, error, l, k, d),
            _ => Ok(()),
        }
    })
}
#[cfg(test)]
mod tests;
