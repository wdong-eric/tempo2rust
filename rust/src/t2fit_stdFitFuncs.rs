//! Source fitting callbacks, TEMPO2 fda6ad4, GPL-3.0-or-later.
#![allow(
    clippy::double_parens,
    clippy::assign_op_pattern,
    clippy::redundant_guards,
    clippy::useless_format
)]
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens)]
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
    observation::{SourceObservation, read, read_string},
    pulsar::SourcePulsar,
    tempo2::Label::*,
};
use rug::{Float, ops::Pow};
/// t2fit_stdFitFuncs.C::t2FitFunc_zero (pinned source line 14).
pub(crate) fn t2FitFunc_zero(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    return Ok((1i32) as f64);
}

/// t2fit_stdFitFuncs.C::t2UpdateFunc_zero (pinned source line 17).
pub(crate) fn t2UpdateFunc_zero(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut err: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    psr.offset = Some(val);
    if (err >= (0i32) as f64) {
        psr.offset_e = Some(err);
    }
    Ok(())
}

/// t2fit_stdFitFuncs.C::t2FitFunc_stdFreq (pinned source line 25).
pub(crate) fn t2FitFunc_stdFreq(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !((label == (param_f as i32)) || (label == (param_brake as i32))) {
        return Err(boundary(
            "t2fit_stdFitFuncs.C::t2FitFunc_stdFreq: assertion",
        ));
    }
    let mut arg3: f64;
    let mut arg4: f64;
    let mut f1: f64;
    let mut f0: f64;
    let mut bindex: f64;
    let mut t2: f64;
    t2 = (x * 86400.0f64);
    arg3 = ((t2 * t2) * t2);
    arg4 = (((t2 * t2) * t2) * t2);
    f0 = ((psr.param.slots[(param_f as i32) as usize].cells[(0i32) as usize].val).clone()).to_f64();
    f1 = ((psr.param.slots[(param_f as i32) as usize].cells[(1i32) as usize].val).clone()).to_f64();
    bindex = (0i32) as f64;
    if (label == (param_brake as i32)) {
        return Ok((((((f1 * f1) / f0) * arg3) / 6.0f64)
            + ((((((((((4i32) as f64 * bindex) - (1i32) as f64) * f1) * f1) * f1) / f0) / f0)
                * arg4)
                / 24.0f64)));
    }
    let mut v: f64 = (0i32) as f64;
    match k {
        source_case if source_case == 0 => {
            v = crate::std_freq::std_freq(x, f0, 0);
            if (psr.param.slots[(param_brake as i32) as usize].cells[(0i32) as usize].param_set
                == 1i32)
            {
                {
                    v = (v
                        + ((((((((((-bindex) * f1) * f1) / f0) / f0) * arg3) / 6.0f64)
                            - (((((((((((2i32) as f64 * bindex)
                                * (((2i32) as f64 * bindex) - (1i32) as f64))
                                * f1)
                                * f1)
                                * f1)
                                / f0)
                                / f0)
                                / f0)
                                * arg4)
                                / 24.0f64))
                            * 84000.0f64)
                            / f0));
                }
            }
            return Ok(v);
        }
        source_case if source_case == 1 => {
            v = crate::std_freq::std_freq(x, f0, 1);
            if (psr.param.slots[(param_brake as i32) as usize].cells[(0i32) as usize].param_set
                == 1i32)
            {
                {
                    v = (v
                        + ((((((((((2i32) as f64 * bindex) * f1) / f0) * arg3) / 6.0f64)
                            + (((((((((3i32) as f64 * bindex)
                                * (((2i32) as f64 * bindex) - (1i32) as f64))
                                * f1)
                                * f1)
                                / f0)
                                / f0)
                                * arg4)
                                / 24.0f64))
                            * 86400.0f64)
                            * 86400.0f64)
                            / f0));
                }
            }
            return Ok(v);
        }
        source_case if source_case == 2 => {
            return Ok(crate::std_freq::std_freq(x, f0, 2));
        }
        source_case if source_case == 3 => {
            return Ok(crate::std_freq::std_freq(x, f0, 3));
        }
        source_case if source_case == 4 => {
            return Ok((((((((1.0f64 / 120.0f64) * x) * x) * x) * x) * x) / 1.0e18f64));
        }
        _ => {
            let mut native_factor = 1.0f64 / 3628800.0;
            let factor = match k {
                5 => 1.0 / 720.0,
                6 => 1.0 / 5040.0,
                7 => 1.0 / 40320.0,
                8 => 1.0 / 362880.0,
                9 => native_factor,
                _ => {
                    for d in 11..=if k <= 13 { k + 1 } else { 14 } {
                        native_factor /= d as f64;
                    }
                    native_factor
                }
            };
            let mut base = Float::with_val(PRECISION_BITS, x);
            if k > 13 {
                base /= k;
            }
            // powl is powq in the actual preprocessed source: promote after the
            // native-long-double factorial division and retain extended multiplication.
            let result = Float::with_val(PRECISION_BITS, factor)
                * base.pow(&Float::with_val(PRECISION_BITS, k + 1))
                / Float::with_val(PRECISION_BITS, if k < 10 { 1e18 } else { 1e23 });
            return Ok(result.to_f64());
        }
    }
}

/// t2fit_stdFitFuncs.C::t2UpdateFunc_stdFreq (pinned source line 95).
pub(crate) fn t2UpdateFunc_stdFreq(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut error: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    crate::std_freq::update_spin(&mut psr.param, k as usize, val, error);
    Ok(())
}

/// t2fit_stdFitFuncs.C::t2FitFunc_stdDm (pinned source line 199).
pub(crate) fn t2FitFunc_stdDm(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !(label == (param_dm as i32)) {
        return Err(boundary("t2fit_stdFitFuncs.C::t2FitFunc_stdDm: assertion"));
    }
    if (*read(&rows[(ipos) as usize].freq, "psr[p].obsn[ipos].freq")? == (0i32) as f64) {
        return Ok((0i32) as f64);
    } else {
        if (k == 0i32) {
            return Ok((Float::with_val(PRECISION_BITS, 1.0f64)
                / (Float::with_val(PRECISION_BITS, 2.41e-4f64)
                    * (Float::with_val(
                        PRECISION_BITS,
                        (*read(&rows[(ipos) as usize].freqSSB, "psr[p].obsn[ipos].freqSSB")?
                            / 1.0e6f64),
                    ))
                    .pow(&(Float::with_val(PRECISION_BITS, 2i32)))))
            .to_f64());
        } else {
            {
                let mut yrs: f64 = (((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?)
                    .clone()
                    - (psr.param.slots[(param_dmepoch as i32) as usize].cells[(0i32) as usize]
                        .val)
                        .clone())
                    / Float::with_val(PRECISION_BITS, 365.25f64))
                .to_f64();
                let mut series_factor: f64 = 1.0f64;
                if ((*read(&psr.dm_series_type, "psr[p].dm_series_type")? as i32) as i32 == 2i32) {
                    {
                        {
                            let mut i: i32 = 1i32;
                            while (i <= k) {
                                series_factor = (series_factor * (i) as f64);
                                i = (i + 1);
                            }
                        }
                    }
                }
                return Ok((((1.0f64
                    / (2.41e-4f64
                        * (*read(
                            &rows[(ipos) as usize].freqSSB,
                            "psr[p].obsn[ipos].freqSSB",
                        )? / 1.0e6f64)
                            .powf((2i32) as f64)))
                    * (yrs).powf((k) as f64))
                    / series_factor));
            }
        }
    }
}

/// t2fit_stdFitFuncs.C::t2FitFunc_stdCm (pinned source line 217).
pub(crate) fn t2FitFunc_stdCm(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !(label == (param_cm as i32)) {
        return Err(boundary("t2fit_stdFitFuncs.C::t2FitFunc_stdCm: assertion"));
    }
    let mut gam: f64 = *read(&psr.TNChromIdx, "psr[p].TNChromIdx")?;
    if (*read(&rows[(ipos) as usize].freq, "psr[p].obsn[ipos].freq")? == (0i32) as f64) {
        return Ok((0i32) as f64);
    } else {
        if (k == 0i32) {
            {
                return Ok((Float::with_val(PRECISION_BITS, 1.0f64)
                    / (Float::with_val(
                        PRECISION_BITS,
                        (*read(&rows[(ipos) as usize].freqSSB, "psr[p].obsn[ipos].freqSSB")?
                            / 1.4e9f64),
                    ))
                    .pow(&(Float::with_val(PRECISION_BITS, gam))))
                .to_f64());
            }
        } else {
            {
                let mut series_factor: f64 = 1.0f64;
                if ((*read(&psr.dm_series_type, "psr[p].dm_series_type")? as i32) as i32 == 2i32) {
                    {
                        {
                            let mut i: i32 = 1i32;
                            while (i <= k) {
                                series_factor = (series_factor * (i) as f64);
                                i = (i + 1);
                            }
                        }
                    }
                }
                let mut yrs: f64 = (((read(&rows[(ipos) as usize].sat, "psr[p].obsn[ipos].sat")?)
                    .clone()
                    - (psr.param.slots[(param_dmepoch as i32) as usize].cells[(0i32) as usize]
                        .val)
                        .clone())
                    / Float::with_val(PRECISION_BITS, 365.25f64))
                .to_f64();
                return Ok((((Float::with_val(PRECISION_BITS, 1.0f64)
                    / (Float::with_val(
                        PRECISION_BITS,
                        (*read(&rows[(ipos) as usize].freqSSB, "psr[p].obsn[ipos].freqSSB")?
                            / 1.4e9f64),
                    ))
                    .pow(&(Float::with_val(PRECISION_BITS, gam))))
                    * Float::with_val(PRECISION_BITS, (yrs).powf((k) as f64)))
                    / Float::with_val(PRECISION_BITS, series_factor))
                .to_f64());
            }
        }
    }
}

/// t2fit_stdFitFuncs.C::t2UpdateFunc_simpleAdd (pinned source line 238).
pub(crate) fn t2UpdateFunc_simpleAdd(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut error: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    psr.param.slots[(label) as usize].cells[(k) as usize].val =
        ((psr.param.slots[(label) as usize].cells[(k) as usize].val).clone()
            + Float::with_val(PRECISION_BITS, val));
    if (error >= (0i32) as f64) {
        psr.param.slots[(label) as usize].cells[(k) as usize].err =
            Float::with_val(PRECISION_BITS, error);
    }
    Ok(())
}

/// t2fit_stdFitFuncs.C::t2UpdateFunc_simpleMinus (pinned source line 243).
pub(crate) fn t2UpdateFunc_simpleMinus(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut error: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    psr.param.slots[(label) as usize].cells[(k) as usize].val =
        ((psr.param.slots[(label) as usize].cells[(k) as usize].val).clone()
            - Float::with_val(PRECISION_BITS, val));
    if (error >= (0i32) as f64) {
        psr.param.slots[(label) as usize].cells[(k) as usize].err =
            Float::with_val(PRECISION_BITS, error);
    }
    Ok(())
}

/// t2fit_stdFitFuncs.C::t2FitFunc_telPos (pinned source line 321).
pub(crate) fn t2FitFunc_telPos(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut maclaurin_scale_factor: f64 = 1.0f64;
    if (k == 2i32) {
        maclaurin_scale_factor = 0.5f64;
    }
    if (k == 3i32) {
        maclaurin_scale_factor = (1.0f64 / 6.0f64);
    }
    if !(k <= 3i32) {
        return Err(boundary("t2fit_stdFitFuncs.C::t2FitFunc_telPos: assertion"));
    }
    let mut dt: Float = ((Float::with_val(PRECISION_BITS, x)
        + (psr.param.slots[(param_pepoch as i32) as usize].cells[(0i32) as usize].val).clone())
        - (psr.param.slots[(param_telEpoch as i32) as usize].cells[(0i32) as usize].val).clone());
    let mut arg: Float = (dt).clone();
    {
        let mut i: i32 = 0i32;
        while (i < k) {
            {
                arg = ((arg).clone() * (dt).clone());
            }
            i = (i + 1);
        }
    }
    arg = ((arg).clone() * Float::with_val(PRECISION_BITS, maclaurin_scale_factor));
    if ((i32::from(
        read_string(&rows[(ipos) as usize].telID[..], "string")? != "STL_FBAT".as_bytes(),
    )) == 0i32)
    {
        {
            match label {
                source_case if source_case == (param_telx as i32) => {
                    return Ok((Float::with_val(
                        PRECISION_BITS,
                        *read(&psr.posPulsar[(0i32) as usize], "psr[p].posPulsar[0]")?,
                    ) * (arg).clone())
                    .to_f64());
                }
                source_case if source_case == (param_tely as i32) => {
                    return Ok((Float::with_val(
                        PRECISION_BITS,
                        *read(&psr.posPulsar[(1i32) as usize], "psr[p].posPulsar[1]")?,
                    ) * (arg).clone())
                    .to_f64());
                }
                source_case if source_case == (param_telz as i32) => {
                    return Ok((Float::with_val(
                        PRECISION_BITS,
                        *read(&psr.posPulsar[(2i32) as usize], "psr[p].posPulsar[2]")?,
                    ) * (arg).clone())
                    .to_f64());
                }
                _ => {
                    return Err(boundary("t2fit_stdFitFuncs.C::t2FitFunc_telPos: assertion"));
                }
            }
        }
    } else {
        {
            return Ok((0i32) as f64);
        }
    }
}

/// t2fit_stdFitFuncs.C::t2FitFunc_jump (pinned source line 354).
pub(crate) fn t2FitFunc_jump(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    {
        let mut l: i32 = 0i32;
        while (l < *read(
            &rows[(ipos) as usize].obsNjump,
            "psr[p].obsn[ipos].obsNjump",
        )?) {
            {
                if (*read(
                    &rows[(ipos) as usize].jump[(l) as usize],
                    "psr[p].obsn[ipos].jump[l]",
                )? == k)
                {
                    {
                        if ((*read(&psr.jumpSAT[(l) as usize], "psr[p].jumpSAT[l]")?) as i32
                            == 0i32)
                        {
                            {
                                return Ok((-*read(
                                    &rows[(ipos) as usize].jumpScale[(l) as usize],
                                    "psr[p].obsn[ipos].jumpScale[l]",
                                )?));
                            }
                        } else {
                            {
                                return Ok(((-1.0f64)
                                    * ((*read(
                                        &rows[(ipos) as usize].freq,
                                        "psr[p].obsn[ipos].freq",
                                    )? * 1e6f64)
                                        / *read(
                                            &rows[(ipos) as usize].freqSSB,
                                            "psr[p].obsn[ipos].freqSSB",
                                        )?)));
                            }
                        }
                    }
                }
            }
            l = (l + 1);
        }
    }
    return Ok((0i32) as f64);
}

/// t2fit_stdFitFuncs.C::t2UpdateFunc_jump (pinned source line 367).
pub(crate) fn t2UpdateFunc_jump(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut err: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    psr.jumpVal[(k) as usize] =
        Some((*read(&psr.jumpVal[(k) as usize], "psr[p].jumpVal[k]")? + val));
    psr.jumpValErr[(k) as usize] = Some(err);
    Ok(())
}

/// t2fit_stdFitFuncs.C::t2FitFunc_fdjump (pinned source line 374).
pub(crate) fn t2FitFunc_fdjump(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    {
        let mut l: i32 = 0i32;
        while (l < *read(
            &rows[(ipos) as usize].obsNfdjump,
            "psr[p].obsn[ipos].obsNfdjump",
        )?) {
            {
                if (*read(
                    &rows[(ipos) as usize].fdjump[(l) as usize],
                    "psr[p].obsn[ipos].fdjump[l]",
                )? == k)
                {
                    {
                        let mut idx: i32 =
                            *read(&psr.fdjumpIdx[(k) as usize], "psr[p].fdjumpIdx[k]")?;
                        if (idx == (-2i32)) {
                            {
                                return Ok(((*read(
                                    &rows[(ipos) as usize].freqSSB,
                                    "psr[p].obsn[ipos].freqSSB",
                                )? / 1e6f64)
                                    .powf((-2i32) as f64)
                                    / 2.41e-4f64));
                            }
                        } else {
                            {
                                if (*read(&psr.fdjump_log, "psr[p].fdjump_log")? != 0) {
                                    {
                                        return Ok(((*read(
                                            &rows[(ipos) as usize].freqSSB,
                                            "psr[p].obsn[ipos].freqSSB",
                                        )? / 1e9f64)
                                            .ln())
                                        .powf((idx) as f64));
                                    }
                                } else {
                                    {
                                        return Ok((*read(
                                            &rows[(ipos) as usize].freqSSB,
                                            "psr[p].obsn[ipos].freqSSB",
                                        )? / 1e9f64)
                                            .powf((idx) as f64));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            l = (l + 1);
        }
    }
    return Ok((0i32) as f64);
}

/// t2fit_stdFitFuncs.C::t2UpdateFunc_fdjump (pinned source line 395).
pub(crate) fn t2UpdateFunc_fdjump(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut err: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    psr.fdjumpVal[(k) as usize] =
        Some((*read(&psr.fdjumpVal[(k) as usize], "psr[p].fdjumpVal[k]")? + val));
    psr.fdjumpValErr[(k) as usize] = Some(err);
    Ok(())
}

/// t2fit_stdFitFuncs.C::t2FitFunc_notImplemented (pinned source line 401).
pub(crate) fn t2FitFunc_notImplemented(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut x: f64,
    mut ipos: i32,
    mut label: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    d.log(
        b'e',
        "t2fit_stdFitFuncs.C",
        402,
        format!("Parameter not implemented").as_bytes(),
    );
    return Err(boundary(
        "t2fit_stdFitFuncs.C::t2FitFunc_notImplemented: source exit",
    ));
}

/// t2fit_stdFitFuncs.C::t2UpdateFunc_notImplemented (pinned source line 405).
pub(crate) fn t2UpdateFunc_notImplemented(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut label: i32,
    mut k: i32,
    mut val: f64,
    mut err: f64,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    d.log(
        b'e',
        "t2fit_stdFitFuncs.C",
        406,
        format!("Parameter not implemented").as_bytes(),
    );
    return Err(boundary(
        "t2fit_stdFitFuncs.C::t2UpdateFunc_notImplemented: source exit",
    ));
}

/// t2fit_stdFitFuncs.C::t2FitFunc_planet. Source used psr-> and ignored ipsr;
/// the two-pulsar native witness in check_callbacks.py demonstrates the mismatch.
pub fn planet(
    psr: &SourcePulsar,
    rows: &[SourceObservation],
    i: usize,
    k: usize,
) -> Result<f64, TimingError> {
    let v = rows[i]
        .planet_ssb
        .get(k)
        .ok_or_else(|| boundary("planet derivative capacity"))?;
    Ok(crate::tempo2_util::dot_product(
        [
            *read(&psr.posPulsar[0], "posPulsar")?,
            *read(&psr.posPulsar[1], "posPulsar")?,
            *read(&psr.posPulsar[2], "posPulsar")?,
        ],
        [
            *read(&v[0], "planet_ssb")?,
            *read(&v[1], "planet_ssb")?,
            *read(&v[2], "planet_ssb")?,
        ],
    ))
}
/// t2FitFunc_telPos_delta: spacecraft, step or interpolated displacement.
pub fn tel_delta(
    psr: &SourcePulsar,
    rows: &[SourceObservation],
    i: usize,
    label: i32,
    k: usize,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let (axis, n, t) = match label {
        l if l == param_tel_dx as i32 => (0, &psr.nTelDX, &psr.telDX_t[..]),
        l if l == param_tel_dy as i32 => (1, &psr.nTelDY, &psr.telDY_t[..]),
        l if l == param_tel_dz as i32 => (2, &psr.nTelDZ, &psr.telDZ_t[..]),
        _ => return Err(boundary("tel delta label")),
    };
    let pos = *read(&psr.posPulsar[axis], "posPulsar")?;
    let sat = read(&rows[i].sat, "sat")?;
    match source_i32(psr.param.slots[label as usize].cells[0].val.to_f64())? {
        -1 => Ok(if read_string(&rows[i].telID, "telID")? == b"STL_FBAT" {
            pos / 299792458.0 * 1000.0
        } else {
            0.0
        }),
        2 => Ok(
            if sat >= read(&t[k], "tel delta time")? && sat < read(&t[k + 1], "tel delta time")? {
                pos
            } else {
                0.0
            },
        ),
        _ => {
            let n = *read(n, "tel delta count")?;
            let t = t
                .iter()
                .take(n as usize)
                .map(|v| read(v, "tel delta time").copied())
                .collect::<Result<Vec<_>, _>>()?;
            Ok(pos * crate::ifunc::basis(&t, sat.to_f64(), n, k as i32, d)?)
        }
    }
}
pub fn update_tel_delta(
    psr: &mut SourcePulsar,
    label: i32,
    k: usize,
    value: f64,
    error: f64,
) -> Result<(), TimingError> {
    let (v, e) = match label {
        l if l == param_tel_dx as i32 => (&mut psr.telDX_v, &mut psr.telDX_e),
        l if l == param_tel_dy as i32 => (&mut psr.telDY_v, &mut psr.telDY_e),
        l if l == param_tel_dz as i32 => (&mut psr.telDZ_v, &mut psr.telDZ_e),
        _ => return Err(boundary("tel delta label")),
    };
    v[k] = Some(*read(&v[k], "tel delta value")? - value);
    e[k] = Some(error);
    Ok(())
}
