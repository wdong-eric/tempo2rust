//! formResiduals.C::formResiduals; full source phase and residual lifecycle.
//! GPL-3.0-or-later, TEMPO2 contributors; fixed Rug128 production arithmetic.
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
use crate::ordinary_long_double as ordinary;
use crate::{
    PRECISION_BITS, TimingError,
    clock_diagnostics::{Diagnostics, boundary},
    observation::{read, read_string},
    pulsar_lifecycle::PulsarContext,
    tempo2::Label::*,
};
use rug::{Float, ops::Pow};
/// formResiduals.C::formResiduals (pinned source line 396).
pub(crate) fn form_residuals_inner(
    context: &mut PulsarContext,
    removeMean: i32,
    max_observations: i32,
    d: &mut Diagnostics,
) -> Result<Vec<Float>, TimingError> {
    let mut references = Vec::with_capacity(context.pulsars.len());
    let mut residual: Float;
    let mut nphase: Float;
    let mut phase2: Float;
    let mut phase3: Float;
    let mut phase4: Float;
    let mut phase2state: Float;
    let mut lastResidual: Float = Float::with_val(PRECISION_BITS, 0i32);
    let mut priorResidual: Float = Float::with_val(PRECISION_BITS, 0i32);
    let mut phase5: Vec<Option<Float>> = vec![None; max_observations as usize];
    let mut lastBat: Float = Float::with_val(PRECISION_BITS, 0.0f64);
    let mut priorBat: Float = Float::with_val(PRECISION_BITS, 0.0f64);
    let mut phaseJ: Float;
    let mut phaseW: Float;
    let mut phaseShape: Float;
    let mut ftpd: Float;
    let mut fct: Float;
    let mut ff0: Float;
    let mut phaseint: Float;
    let mut torb: Float;
    let mut deltaT: Float;
    let mut dt00: Float = Float::with_val(PRECISION_BITS, 0.0f64);
    let mut phas1: Float = Float::with_val(PRECISION_BITS, 0.0f64);
    let mut mean: Float;
    let mut tnmean: Float;
    let mut ct00: Float = Float::with_val(PRECISION_BITS, 0.0f64);
    let mut dtm1s: i32 = 0i32;
    let mut nmean: i32;
    let mut ntnmean: i32;
    let mut ntpd: i32;
    let mut nf0: i32;
    let mut i: i32;
    let mut p: i32;
    let mut k: i32;
    let mut l: i32;
    let mut time: i32 = 0i32;
    let mut ntrk: i32 = 0i32;
    let mut gotit: i32 = 0i32;
    let mut pn0: i64 = (-1i32) as i64;
    let mut pnAdd: i64 = (-1i32) as i64;
    let mut prev_p: Option<f64> = None;
    let mut prev_e: Option<f64> = None;
    let mut prev_epoch: Option<f64> = None;
    let mut prev_theta: Option<f64> = None;
    let mut prev_a: Option<f64> = None;
    let mut coalesceFlag: Option<i32> = Some(0i32);
    let mut prev_p_p: Option<f64> = None;
    let mut prev_e_p: Option<f64> = None;
    let mut prev_epoch_p: Option<f64> = None;
    let mut prev_theta_p: Option<f64> = None;
    let mut prev_a_p: Option<f64> = None;
    let mut coalesceFlag_p: Option<i32> = Some(0i32);
    d.version("formResiduals.C", "formResiduals()");
    d.timing("formResiduals.C", 438, "Enter formresiduals()".as_bytes());
    for p in 0..context.pulsars.len() {
        let n = *read(&context.pulsars[p].nobs, "nobs")?;
        if context.pulsars[p].refphs == Some(1) {
            context.ensure_observation_capacity(p, n as usize + 1)?;
        }
        let (psr, rows) = context.pulsar_observations_mut(p)?;
        {
            mean = Float::with_val(PRECISION_BITS, Float::parse("0.0").unwrap());
            nmean = 0i32;
            tnmean = Float::with_val(PRECISION_BITS, Float::parse("0.0").unwrap());
            ntnmean = 0i32;
            if ((*read(&psr.refphs, "psr[p].refphs")?) as i32 == 1i32) {
                {
                    if (*read(&psr.nobs, "psr[p].nobs")? == max_observations) {
                        {
                            d.log(b'e', "formResiduals.C", 481, b"Error, need at least 1 spare observation to reference to REFPHS TZR");
                            return Err(boundary("formResiduals.C::formResiduals: source exit"));
                        }
                    }
                    psr.nobs = Some(*read(&psr.nobs, "psr[p].nobs")? + 1);
                    rows[(*read(&psr.nobs, "nobs")? - 1) as usize] = psr.tzrobs.clone();
                }
            }
            {
                i = 0i32;
                while (i < *read(&psr.nobs, "psr[p].nobs")?) {
                    {
                        torb = Float::with_val(PRECISION_BITS, 0.0f64);
                        if ((psr.param.slots[param_pb as usize].cells[0].param_set == 1i32)
                            || (psr.param.slots[param_fb as usize].cells[0].param_set == 1i32))
                        {
                            {
                                torb = crate::binary::model_inner(psr, rows, i as usize, -1, 0, d)?;
                            }
                        }
                        rows[(i) as usize].torb = Some((torb).clone());
                        deltaT = ((((read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?)
                            .clone()
                            - (psr.param.slots[param_pepoch as usize].cells[0].val).clone())
                            * Float::with_val(PRECISION_BITS, 86400.0f64))
                            + (torb).clone());
                        fct = (((read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?).clone()
                            - Float::with_val(
                                PRECISION_BITS,
                                crate::ifteph::extended_i32(
                                    &((read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?)
                                        .clone()),
                                    "binary_integer",
                                )?,
                            ))
                            - ((psr.param.slots[param_pepoch as usize].cells[0].val).clone()
                                - Float::with_val(
                                    PRECISION_BITS,
                                    crate::ifteph::extended_i32(
                                        &((psr.param.slots[param_pepoch as usize].cells[0].val)
                                            .clone()),
                                        "binary_integer",
                                    )?,
                                )));
                        ftpd = ((fct).clone()
                            + ((torb).clone() / Float::with_val(PRECISION_BITS, 86400.0f64)));
                        rows[(i) as usize].pet = Some(
                            (read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?).clone()
                                - ((torb).clone() / Float::with_val(PRECISION_BITS, 86400.0f64)),
                        );
                        nf0 = crate::ifteph::extended_i32(
                            &((psr.param.slots[param_f as usize].cells[0].val).clone()),
                            "F0 integer part exceeds source i32 range",
                        )?;
                        ntpd = (crate::ifteph::extended_i32(
                            &((read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?).clone()),
                            "binary_integer",
                        )?)
                        .checked_sub(crate::ifteph::extended_i32(
                            &((psr.param.slots[param_pepoch as usize].cells[0].val).clone()),
                            "binary_integer",
                        )?)
                        .ok_or_else(|| {
                            boundary("integer day difference exceeds source i32 range")
                        })?;
                        ff0 = ((psr.param.slots[param_f as usize].cells[0].val).clone()
                            - Float::with_val(
                                PRECISION_BITS,
                                crate::ifteph::extended_i32(
                                    &((psr.param.slots[param_f as usize].cells[0].val).clone()),
                                    "binary_integer",
                                )?,
                            ));
                        phase2 = (((Float::with_val(PRECISION_BITS, nf0) * (ftpd).clone())
                            + (Float::with_val(PRECISION_BITS, ntpd) * (ff0).clone()))
                            + ((ftpd).clone() * (ff0).clone()));
                        phase2 = ((phase2).clone() * Float::with_val(PRECISION_BITS, 86400.0f64));
                        let mut arg: Float = ((deltaT).clone() * (deltaT).clone());
                        phase3 = ((Float::with_val(PRECISION_BITS, 0.5f64)
                            * (psr.param.slots[param_f as usize].cells[1].val).clone())
                            * (arg).clone());
                        arg = ((arg).clone() * (deltaT).clone());
                        if (psr.param.slots[param_f as usize].cells[2].param_set == 1i32) {
                            phase3 = ((phase3).clone()
                                + (((psr.param.slots[param_f as usize].cells[2].val).clone()
                                    / Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("6.0").unwrap(),
                                    ))
                                    * (arg).clone()));
                        }
                        arg = ((arg).clone() * (deltaT).clone());
                        if (psr.param.slots[param_f as usize].cells[3].param_set == 1i32) {
                            phase3 = ((phase3).clone()
                                + (((psr.param.slots[param_f as usize].cells[3].val).clone()
                                    / Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("24.0").unwrap(),
                                    ))
                                    * (arg).clone()));
                        }
                        arg = ((arg).clone() * (deltaT).clone());
                        if (psr.param.slots[param_f as usize].cells[4].param_set == 1i32) {
                            phase3 = ((phase3).clone()
                                + (((psr.param.slots[param_f as usize].cells[4].val).clone()
                                    / Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("120.0").unwrap(),
                                    ))
                                    * (arg).clone()));
                        }
                        arg = ((arg).clone() * (deltaT).clone());
                        if (psr.param.slots[param_f as usize].cells[5].param_set == 1i32) {
                            phase3 = ((phase3).clone()
                                + (((psr.param.slots[param_f as usize].cells[5].val).clone()
                                    / Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("720.0").unwrap(),
                                    ))
                                    * (arg).clone()));
                        }
                        arg = ((arg).clone() * (deltaT).clone());
                        if (psr.param.slots[param_f as usize].cells[6].param_set == 1i32) {
                            phase3 = ((phase3).clone()
                                + (((psr.param.slots[param_f as usize].cells[6].val).clone()
                                    / Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("5040.0").unwrap(),
                                    ))
                                    * (arg).clone()));
                        }
                        arg = ((arg).clone() * (deltaT).clone());
                        if (psr.param.slots[param_f as usize].cells[7].param_set == 1i32) {
                            phase3 = ((phase3).clone()
                                + (((psr.param.slots[param_f as usize].cells[7].val).clone()
                                    / Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("40320.0").unwrap(),
                                    ))
                                    * (arg).clone()));
                        }
                        arg = ((arg).clone() * (deltaT).clone());
                        if (psr.param.slots[param_f as usize].cells[8].param_set == 1i32) {
                            phase3 = ((phase3).clone()
                                + (((psr.param.slots[param_f as usize].cells[8].val).clone()
                                    / Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("362880.0").unwrap(),
                                    ))
                                    * (arg).clone()));
                        }
                        arg = ((arg).clone() * (deltaT).clone());
                        if (psr.param.slots[param_f as usize].cells[9].param_set == 1i32) {
                            phase3 = ((phase3).clone()
                                + (((psr.param.slots[param_f as usize].cells[9].val).clone()
                                    / Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("3628800.0").unwrap(),
                                    ))
                                    * (arg).clone()));
                        }
                        arg = ((arg).clone() * (deltaT).clone());
                        if (psr.param.slots[param_f as usize].cells[10].param_set == 1i32) {
                            phase3 = ((phase3).clone()
                                + (((psr.param.slots[param_f as usize].cells[10].val).clone()
                                    / Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("39916800.0").unwrap(),
                                    ))
                                    * (arg).clone()));
                        }
                        arg = ((arg).clone() * (deltaT).clone());
                        if (psr.param.slots[param_f as usize].cells[11].param_set == 1i32) {
                            phase3 = ((phase3).clone()
                                + (((psr.param.slots[param_f as usize].cells[11].val).clone()
                                    / Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("479001600.0").unwrap(),
                                    ))
                                    * (arg).clone()));
                        }
                        arg = ((arg).clone() * (deltaT).clone());
                        if (psr.param.slots[param_f as usize].cells[12].param_set == 1i32) {
                            phase3 = ((phase3).clone()
                                + (((psr.param.slots[param_f as usize].cells[12].val).clone()
                                    / Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("6227020800.0").unwrap(),
                                    ))
                                    * (arg).clone()));
                        }
                        phase2state = Float::with_val(PRECISION_BITS, 0.0f64);
                        {
                            k = 0i32;
                            while (k < *read(
                                &psr.param.slots[param_stateSwitchT as usize].a_size,
                                "psr[p].param[param_stateSwitchT].aSize",
                            )?) {
                                {
                                    if (psr.param.slots[param_stateSwitchT as usize].cells
                                        [(k) as usize]
                                        .param_set
                                        == 1i32)
                                    {
                                        {
                                            let mut tp: Float;
                                            let mut dt1: Float;
                                            let mut tgl: Float;
                                            let mut signV: i32 = 1i32;
                                            tp = ((Float::with_val(PRECISION_BITS, ntpd)
                                                + (ftpd).clone())
                                                * Float::with_val(PRECISION_BITS, 86400.0f64));
                                            tgl = (((psr.param.slots
                                                [param_stateSwitchT as usize]
                                                .cells
                                                [(k) as usize]
                                                .val)
                                                .clone()
                                                - (psr.param.slots[param_pepoch as usize].cells
                                                    [0]
                                                .val)
                                                    .clone())
                                                * Float::with_val(PRECISION_BITS, 86400.0f64));
                                            if ((tp).clone() >= (tgl).clone()) {
                                                {
                                                    dt1 = ((tp).clone() - (tgl).clone());
                                                    if ((k % 2i32) == 0i32) {
                                                        signV = 1i32;
                                                    } else {
                                                        signV = (-1i32);
                                                    }
                                                    phase2state = ((phase2state).clone()
                                                        + (((Float::with_val(
                                                            PRECISION_BITS,
                                                            0.5f64,
                                                        ) * ((psr.param.slots
                                                            [param_df1 as usize]
                                                            .cells[0]
                                                            .val)
                                                            .clone()
                                                            * Float::with_val(
                                                                PRECISION_BITS,
                                                                signV,
                                                            )))
                                                            * (dt1).clone())
                                                            * (dt1).clone()));
                                                }
                                            }
                                        }
                                    }
                                }
                                k += 1;
                            }
                        }
                        if ((psr.param.slots[param_brake as usize].cells[0].param_set == 1i32)
                            && (psr.param.slots[param_f as usize].cells[1].param_set == 1i32))
                        {
                            {
                                let mut F2brake: Float;
                                let mut F3brake: Float;
                                let mut F1: Float;
                                let mut F0: Float;
                                let mut bindex: Float;
                                let mut arg3: Float;
                                let mut arg4: Float;
                                arg3 = (((deltaT).clone() * (deltaT).clone()) * (deltaT).clone());
                                arg4 = ((arg3).clone() * (deltaT).clone());
                                F1 = (psr.param.slots[param_f as usize].cells[1].val).clone();
                                F0 = (psr.param.slots[param_f as usize].cells[0].val).clone();
                                bindex =
                                    (psr.param.slots[param_brake as usize].cells[0].val).clone();
                                F2brake = ((((bindex).clone() * (F1).clone()) * (F1).clone())
                                    / (F0).clone());
                                F3brake = (((((((bindex).clone()
                                    * ((Float::with_val(PRECISION_BITS, 2i32)
                                        * (bindex).clone())
                                        - Float::with_val(PRECISION_BITS, 1i32)))
                                    * (F1).clone())
                                    * (F1).clone())
                                    * (F1).clone())
                                    / (F0).clone())
                                    / (F0).clone());
                                phase3 = ((phase3).clone()
                                    + ((((F2brake).clone() * (arg3).clone())
                                        / Float::with_val(PRECISION_BITS, 6.0f64))
                                        + (((F3brake).clone() * (arg4).clone())
                                            / Float::with_val(PRECISION_BITS, 24.0f64))));
                            }
                        }
                        phase4 = Float::with_val(PRECISION_BITS, 0.0f64);
                        {
                            k = 0i32;
                            while (k < *read(
                                &psr.param.slots[param_glep as usize].a_size,
                                "psr[p].param[param_glep].aSize",
                            )?) {
                                {
                                    if (psr.param.slots[param_glep as usize].cells[(k) as usize]
                                        .param_set
                                        == 1i32)
                                    {
                                        {
                                            let mut tp: Float;
                                            let mut dt1: Float;
                                            let mut expf: Float;
                                            let mut tgl: Float;
                                            tp = ((Float::with_val(PRECISION_BITS, ntpd)
                                                + (ftpd).clone())
                                                * Float::with_val(PRECISION_BITS, 86400.0f64));
                                            tgl = (((psr.param.slots[param_glep as usize].cells
                                                [(k) as usize]
                                                .val)
                                                .clone()
                                                - (psr.param.slots[param_pepoch as usize].cells
                                                    [0]
                                                .val)
                                                    .clone())
                                                * Float::with_val(PRECISION_BITS, 86400.0f64));
                                            if ((tp).clone() >= (tgl).clone()) {
                                                {
                                                    let mut expf2: f64;
                                                    let mut expf3: f64;
                                                    dt1 = ((tp).clone() - (tgl).clone());
                                                    if ((psr.param.slots[param_gltd as usize]
                                                        .cells
                                                        [(k) as usize]
                                                        .val)
                                                        .clone()
                                                        != Float::with_val(PRECISION_BITS, 0.0f64))
                                                    {
                                                        expf = (((-(dt1).clone())
                                                            / Float::with_val(
                                                                PRECISION_BITS,
                                                                86400.0f64,
                                                            ))
                                                            / (psr.param.slots
                                                                [param_gltd as usize]
                                                                .cells
                                                                [(k) as usize]
                                                                .val)
                                                                .clone())
                                                        .exp();
                                                    } else {
                                                        expf =
                                                            Float::with_val(PRECISION_BITS, 1.0f64);
                                                    }
                                                    if ((psr.param.slots[param_gltd2 as usize]
                                                        .cells
                                                        [(k) as usize]
                                                        .val)
                                                        .clone()
                                                        != Float::with_val(PRECISION_BITS, 0.0f64))
                                                    {
                                                        expf2 = ((((-(dt1).clone())
                                                            / Float::with_val(
                                                                PRECISION_BITS,
                                                                86400.0f64,
                                                            ))
                                                            / (psr.param.slots
                                                                [param_gltd2 as usize]
                                                                .cells
                                                                [(k) as usize]
                                                                .val)
                                                                .clone())
                                                        .exp())
                                                        .to_f64();
                                                    } else {
                                                        expf2 = 1.0f64;
                                                    }
                                                    if ((psr.param.slots[param_gltd3 as usize]
                                                        .cells
                                                        [(k) as usize]
                                                        .val)
                                                        .clone()
                                                        != Float::with_val(PRECISION_BITS, 0.0f64))
                                                    {
                                                        expf3 = ((((-(dt1).clone())
                                                            / Float::with_val(
                                                                PRECISION_BITS,
                                                                86400.0f64,
                                                            ))
                                                            / (psr.param.slots
                                                                [param_gltd3 as usize]
                                                                .cells
                                                                [(k) as usize]
                                                                .val)
                                                                .clone())
                                                        .exp())
                                                        .to_f64();
                                                    } else {
                                                        expf3 = 1.0f64;
                                                    }
                                                    phase4 = ((phase4).clone()
                                                        + (((((((psr.param.slots
                                                            [param_glph as usize]
                                                            .cells
                                                            [(k) as usize]
                                                            .val)
                                                            .clone()
                                                            + ((psr.param.slots
                                                                [param_glf0 as usize]
                                                                .cells
                                                                [(k) as usize]
                                                                .val)
                                                                .clone()
                                                                * (dt1).clone()))
                                                            + (((Float::with_val(
                                                                PRECISION_BITS,
                                                                0.5f64,
                                                            ) * (psr.param.slots
                                                                [param_glf1 as usize]
                                                                .cells
                                                                [(k) as usize]
                                                                .val)
                                                                .clone())
                                                                * (dt1).clone())
                                                                * (dt1).clone()))
                                                            + ((((Float::with_val(
                                                                PRECISION_BITS,
                                                                (1.0f64 / 6.0f64),
                                                            ) * (psr.param.slots
                                                                [param_glf2 as usize]
                                                                .cells
                                                                [(k) as usize]
                                                                .val)
                                                                .clone())
                                                                * (dt1).clone())
                                                                * (dt1).clone())
                                                                * (dt1).clone()))
                                                            + ((((psr.param.slots
                                                                [param_glf0d as usize]
                                                                .cells
                                                                [(k) as usize]
                                                                .val)
                                                                .clone()
                                                                * (psr.param.slots
                                                                    [param_gltd as usize]
                                                                    .cells
                                                                    [(k) as usize]
                                                                    .val)
                                                                    .clone())
                                                                * Float::with_val(
                                                                    PRECISION_BITS,
                                                                    86400.0f64,
                                                                ))
                                                                * (Float::with_val(
                                                                    PRECISION_BITS,
                                                                    1.0f64,
                                                                ) - (expf).clone())))
                                                            + ((((psr.param.slots
                                                                [param_glf0d2 as usize]
                                                                .cells
                                                                [(k) as usize]
                                                                .val)
                                                                .clone()
                                                                * (psr.param.slots
                                                                    [param_gltd2 as usize]
                                                                    .cells
                                                                    [(k) as usize]
                                                                    .val)
                                                                    .clone())
                                                                * Float::with_val(
                                                                    PRECISION_BITS,
                                                                    86400.0f64,
                                                                ))
                                                                * Float::with_val(
                                                                    PRECISION_BITS,
                                                                    (1.0f64 - expf2),
                                                                )))
                                                            + ((((psr.param.slots
                                                                [param_glf0d3 as usize]
                                                                .cells
                                                                [(k) as usize]
                                                                .val)
                                                                .clone()
                                                                * (psr.param.slots
                                                                    [param_gltd3 as usize]
                                                                    .cells
                                                                    [(k) as usize]
                                                                    .val)
                                                                    .clone())
                                                                * Float::with_val(
                                                                    PRECISION_BITS,
                                                                    86400.0f64,
                                                                ))
                                                                * Float::with_val(
                                                                    PRECISION_BITS,
                                                                    (1.0f64 - expf3),
                                                                ))));
                                                }
                                            }
                                        }
                                    }
                                }
                                k += 1;
                            }
                        }
                        {
                            k = 0i32;
                            while (k < *read(
                                &psr.param.slots[param_nudot_epoch as usize].a_size,
                                "psr[p].param[param_nudot_epoch].aSize",
                            )?) {
                                {
                                    if (psr.param.slots[param_nudot_epoch as usize].cells
                                        [(k) as usize]
                                        .param_set
                                        == 1i32)
                                    {
                                        {
                                            let mut w: f64 = (0i32) as f64;
                                            if (psr.param.slots
                                                [param_nudot_transition_time as usize]
                                                .cells
                                                [(k) as usize]
                                                .param_set
                                                != 0)
                                            {
                                                {
                                                    w = ((psr.param.slots
                                                        [param_nudot_transition_time as usize]
                                                        .cells
                                                        [(k) as usize]
                                                        .val)
                                                        .clone()
                                                        * Float::with_val(
                                                            PRECISION_BITS,
                                                            86400.0f64,
                                                        ))
                                                    .to_f64();
                                                }
                                            }
                                            let mut tp: f64 =
                                                ((Float::with_val(PRECISION_BITS, ntpd)
                                                    + (ftpd).clone())
                                                    * Float::with_val(PRECISION_BITS, 86400.0f64))
                                                .to_f64();
                                            let mut tgl: f64 = (((psr.param.slots
                                                [param_nudot_epoch as usize]
                                                .cells
                                                [(k) as usize]
                                                .val)
                                                .clone()
                                                - (psr.param.slots[param_pepoch as usize].cells
                                                    [0]
                                                .val)
                                                    .clone())
                                                * Float::with_val(PRECISION_BITS, 86400.0f64))
                                            .to_f64();
                                            let mut tgl2: f64 = (tgl + w);
                                            let mut delta_nudot: f64 =
                                                ((psr.param.slots[param_nudot_amp as usize].cells
                                                    [(k) as usize]
                                                    .val)
                                                    .clone())
                                                .to_f64();
                                            if ((tp > tgl) && (tp < tgl2)) {
                                                {
                                                    phase4 = ((phase4).clone()
                                                        + Float::with_val(
                                                            PRECISION_BITS,
                                                            (((delta_nudot
                                                                * (tp - tgl).powf((3i32) as f64))
                                                                / 6.0f64)
                                                                / w),
                                                        ));
                                                }
                                            } else {
                                                if (tp >= tgl2) {
                                                    {
                                                        let mut c: f64 =
                                                            (((delta_nudot * w) * w) / 6.0f64);
                                                        phase4 = ((phase4).clone()
                                                            + Float::with_val(
                                                                PRECISION_BITS,
                                                                ((((delta_nudot
                                                                    * (tp - tgl2)
                                                                        .powf((2i32) as f64))
                                                                    / 2.0f64)
                                                                    + (((delta_nudot * w)
                                                                        * (tp - tgl2))
                                                                        / 2.0f64))
                                                                    + c),
                                                            ));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                k += 1;
                            }
                        }
                        {
                            k = 0i32;
                            while (k < *read(
                                &psr.param.slots[param_expep as usize].a_size,
                                "psr[p].param[param_expep].aSize",
                            )?) {
                                {
                                    if (psr.param.slots[param_expep as usize].cells[(k) as usize]
                                        .param_set
                                        == 1i32)
                                    {
                                        {
                                            // formResiduals.C:675-695: ordinary assignments and exp, with
                                            // powl promoted to the existing Rug128 timing role (0104).
                                            let freq = ordinary::round(
                                                *read(
                                                    &rows[i as usize].freqSSB,
                                                    "psr[p].obsn[i].freqSSB",
                                                )? / 1.4e9,
                                            );
                                            let dt = ordinary::round(
                                                read(
                                                    &rows[i as usize].bbat,
                                                    "psr[p].obsn[i].bbat",
                                                )?
                                                .clone()
                                                    - &psr.param.slots[param_expep as usize].cells
                                                        [k as usize]
                                                        .val,
                                            );
                                            let dm = ordinary::round(
                                                &psr.param.slots[param_expph as usize].cells
                                                    [k as usize]
                                                    .val,
                                            );
                                            let tau = ordinary::round(
                                                &psr.param.slots[param_exptau as usize].cells
                                                    [k as usize]
                                                    .val,
                                            );
                                            let index = &psr.param.slots[param_expindex as usize]
                                                .cells[k as usize];
                                            let gamma = if index.param_set == 1 {
                                                ordinary::round(&index.val)
                                            } else {
                                                ordinary::round(-2)
                                            };
                                            if dt > 0 {
                                                let exponent = ordinary::div(&(-dt.clone()), &tau);
                                                phase4 += Float::with_val(PRECISION_BITS, &dm)
                                                    * Float::with_val(PRECISION_BITS, &freq).pow(
                                                        &Float::with_val(PRECISION_BITS, &gamma),
                                                    )
                                                    * Float::with_val(
                                                        PRECISION_BITS,
                                                        ordinary::exp(&exponent),
                                                    )
                                                    * &psr.param.slots[param_f as usize].cells[0]
                                                        .val;
                                            }
                                        }
                                    }
                                }
                                k += 1;
                            }
                        }
                        {
                            k = 0i32;
                            while (k < *read(
                                &psr.param.slots[param_gausep as usize].a_size,
                                "psr[p].param[param_gausep].aSize",
                            )?) {
                                {
                                    if (psr.param.slots[param_gausep as usize].cells[(k) as usize]
                                        .param_set
                                        == 1i32)
                                    {
                                        {
                                            // formResiduals.C:705-728: every exponent operation is
                                            // ordinary; the wide amplitude product narrows into val.
                                            let freq = ordinary::round(
                                                *read(
                                                    &rows[i as usize].freqSSB,
                                                    "psr[p].obsn[i].freqSSB",
                                                )? / 1.4e9,
                                            );
                                            let dt = ordinary::round(
                                                read(
                                                    &rows[i as usize].bbat,
                                                    "psr[p].obsn[i].bbat",
                                                )?
                                                .clone()
                                                    - &psr.param.slots[param_gausep as usize].cells
                                                        [k as usize]
                                                        .val,
                                            );
                                            let amp = ordinary::round(
                                                &psr.param.slots[param_gausamp as usize].cells
                                                    [k as usize]
                                                    .val,
                                            );
                                            let sig = ordinary::round(
                                                &psr.param.slots[param_gaussig as usize].cells
                                                    [k as usize]
                                                    .val,
                                            );
                                            let index = &psr.param.slots[param_gausindex as usize]
                                                .cells[k as usize];
                                            let gamma = if index.param_set == 1 {
                                                ordinary::round(&index.val)
                                            } else {
                                                ordinary::round(0)
                                            };
                                            let exponent = ordinary::mul(&(-dt.clone()), &dt);
                                            let exponent =
                                                ordinary::div(&exponent, &ordinary::round(2));
                                            let exponent = ordinary::div(&exponent, &sig);
                                            let exponent = ordinary::div(&exponent, &sig);
                                            let val = ordinary::round(
                                                Float::with_val(PRECISION_BITS, &amp)
                                                    * Float::with_val(PRECISION_BITS, &freq).pow(
                                                        &Float::with_val(PRECISION_BITS, &gamma),
                                                    )
                                                    * Float::with_val(
                                                        PRECISION_BITS,
                                                        ordinary::exp(&exponent),
                                                    ),
                                            );
                                            phase4 += Float::with_val(PRECISION_BITS, &val)
                                                * &psr.param.slots[param_f as usize].cells[0].val;
                                        }
                                    }
                                }
                                k += 1;
                            }
                        }
                        if (*read(
                            &psr.param.slots[param_glep as usize].a_size,
                            "psr[p].param[param_glep].aSize",
                        )? > 0i32)
                        {
                            {}
                        }
                        phaseJ = Float::with_val(PRECISION_BITS, 0.0f64);
                        {
                            k = 1i32;
                            while (k <= *read(&psr.nJumps, "psr[p].nJumps")?) {
                                {
                                    {
                                        l = 0i32;
                                        while (l < *read(
                                            &rows[(i) as usize].obsNjump,
                                            "psr[p].obsn[i].obsNjump",
                                        )?) {
                                            {
                                                if ((*read(
                                                    &rows[(i) as usize].jump[(l) as usize],
                                                    "psr[p].obsn[i].jump[l]",
                                                )? == k)
                                                    && ((*read(
                                                        &psr.jumpSAT[(l) as usize],
                                                        "psr[p].jumpSAT[l]",
                                                    )?)
                                                        as i32
                                                        == 0i32))
                                                {
                                                    {
                                                        phaseJ = ((phaseJ).clone()
                                                            + (Float::with_val(
                                                                PRECISION_BITS,
                                                                (*read(
                                                                    &psr.jumpVal[(k) as usize],
                                                                    "psr[p].jumpVal[k]",
                                                                )? * *read(
                                                                    &rows[(i) as usize].jumpScale
                                                                        [(l) as usize],
                                                                    "psr[p].obsn[i].jumpScale[l]",
                                                                )?),
                                                            ) * (psr.param.slots
                                                                [param_f as usize]
                                                                .cells[0]
                                                                .val)
                                                                .clone()));
                                                    }
                                                }
                                            }
                                            l += 1;
                                        }
                                    }
                                }
                                k += 1;
                            }
                        }
                        if ((psr.param.slots[param_fddc as usize].cells[0].param_set == 1i32)
                            && (*read(&rows[(i) as usize].freqSSB, "psr[p].obsn[i].freqSSB")?
                                > (1i32) as f64))
                        {
                            phaseJ = ((phaseJ).clone()
                                - (((psr.param.slots[param_fddc as usize].cells[0].val).clone()
                                    / (Float::with_val(
                                        PRECISION_BITS,
                                        *read(
                                            &rows[(i) as usize].freqSSB,
                                            "psr[p].obsn[i].freqSSB",
                                        )?,
                                    ) * Float::with_val(PRECISION_BITS, 1.0e-6f64))
                                    .pow(
                                        &((psr.param.slots[param_fddi as usize].cells[0].val)
                                            .clone()),
                                    ))
                                    * (psr.param.slots[param_f as usize].cells[0].val).clone()));
                        }
                        {
                            k = 0i32;
                            while (k < *read(
                                &psr.param.slots[param_fd as usize].a_size,
                                "psr[p].param[param_fd].aSize",
                            )?) {
                                {
                                    if ((psr.param.slots[param_fd as usize].cells[(k) as usize]
                                        .param_set
                                        == 1i32)
                                        && (*read(
                                            &rows[(i) as usize].freqSSB,
                                            "psr[p].obsn[i].freqSSB",
                                        )? > (1i32) as f64))
                                    {
                                        phaseJ = ((phaseJ).clone()
                                            - (((psr.param.slots[param_fd as usize].cells
                                                [(k) as usize]
                                                .val)
                                                .clone()
                                                * (Float::with_val(
                                                    PRECISION_BITS,
                                                    ((Float::with_val(
                                                        PRECISION_BITS,
                                                        *read(
                                                            &rows[(i) as usize].freqSSB,
                                                            "psr[p].obsn[i].freqSSB",
                                                        )?,
                                                    ) / Float::with_val(
                                                        PRECISION_BITS,
                                                        1.0e9f64,
                                                    ))
                                                    .to_f64())
                                                    .ln(),
                                                ))
                                                .pow(
                                                    &(Float::with_val(PRECISION_BITS, (k + 1i32))),
                                                ))
                                                * (psr.param.slots[param_f as usize].cells[0]
                                                    .val)
                                                    .clone()));
                                    }
                                }
                                k += 1;
                            }
                        }
                        {
                            k = 1i32;
                            while (k <= *read(&psr.nfdJumps, "psr[p].nfdJumps")?) {
                                {
                                    {
                                        l = 0i32;
                                        while (l < *read(
                                            &rows[(i) as usize].obsNfdjump,
                                            "psr[p].obsn[i].obsNfdjump",
                                        )?) {
                                            {
                                                if (*read(
                                                    &rows[(i) as usize].fdjump[(l) as usize],
                                                    "psr[p].obsn[i].fdjump[l]",
                                                )? == k)
                                                {
                                                    {
                                                        let mut idx: i32;
                                                        idx = *read(
                                                            &psr.fdjumpIdx[(k) as usize],
                                                            "psr[p].fdjumpIdx[k]",
                                                        )?;
                                                        if (idx == (-2i32)) {
                                                            {
                                                                phaseJ = ((phaseJ).clone() - ((Float::with_val(PRECISION_BITS, (*read(&psr.fdjumpVal[(k) as usize], "psr[p].fdjumpVal[k]")? * (*read(&rows[(i) as usize].freqSSB, "psr[p].obsn[i].freqSSB")? / 1e6f64).powf((-2i32) as f64))) * (psr.param.slots[param_f as usize].cells[0].val).clone()) / Float::with_val(PRECISION_BITS, 2.41e-4f64)));
                                                            }
                                                        } else {
                                                            {
                                                                if (*read(
                                                                    &psr.fdjump_log,
                                                                    "psr[p].fdjump_log",
                                                                )? != 0)
                                                                {
                                                                    {
                                                                        phaseJ = ((phaseJ).clone() - (Float::with_val(PRECISION_BITS, (*read(&psr.fdjumpVal[(k) as usize], "psr[p].fdjumpVal[k]")? * ((*read(&rows[(i) as usize].freqSSB, "psr[p].obsn[i].freqSSB")? / 1e9f64).ln()).powf((idx) as f64))) * (psr.param.slots[param_f as usize].cells[0].val).clone()));
                                                                    }
                                                                } else {
                                                                    {
                                                                        phaseJ = ((phaseJ).clone() - (Float::with_val(PRECISION_BITS, (*read(&psr.fdjumpVal[(k) as usize], "psr[p].fdjumpVal[k]")? * (*read(&rows[(i) as usize].freqSSB, "psr[p].obsn[i].freqSSB")? / 1e9f64).powf((idx) as f64))) * (psr.param.slots[param_f as usize].cells[0].val).clone()));
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            l += 1;
                                        }
                                    }
                                }
                                k += 1;
                            }
                        }
                        // formResiduals.C:796-804: shapelet coefficients belong to current p.
                        // The original psr-> coefficient access read pulsar zero in a multi-pulsar call.
                        phaseShape = Float::with_val(PRECISION_BITS, 0i32);
                        {
                            let mut iTNShape: i32 = 0i32;
                            while (iTNShape
                                < *read(&psr.nTNShapeletEvents, "psr[p].nTNShapeletEvents")?)
                            {
                                {
                                    if (*read(
                                        &psr.TNShapeletEvFScale[(iTNShape) as usize],
                                        "psr[p].TNShapeletEvFScale[iTNShape]",
                                    )? == 0.0f64)
                                    {
                                        {
                                            phaseShape = ((phaseShape).clone()
                                                - (Float::with_val(
                                                    PRECISION_BITS,
                                                    crate::shapelet::evaluate_shapelet(
                                                        *read(
                                                            &psr.TNShapeletEvN[(iTNShape) as usize],
                                                            "psr->TNShapeletEvN[iTNShape]",
                                                        )?,
                                                        *read(
                                                            &psr.TNShapeletEvPos
                                                                [(iTNShape) as usize],
                                                            "psr->TNShapeletEvPos[iTNShape]",
                                                        )?,
                                                        *read(
                                                            &psr.TNShapeletEvWidth
                                                                [(iTNShape) as usize],
                                                            "psr->TNShapeletEvWidth[iTNShape]",
                                                        )?,
                                                        &psr.TNShapeletEvCoef[(iTNShape) as usize]
                                                            .iter()
                                                            .take(
                                                                (*read(
                                                                    &psr.TNShapeletEvN
                                                                        [(iTNShape) as usize],
                                                                    "psr->TNShapeletEvN[iTNShape]",
                                                                )?)
                                                                    as usize,
                                                            )
                                                            .map(|v| {
                                                                read(v, "shapelet coefficient")
                                                                    .copied()
                                                            })
                                                            .collect::<Result<Vec<_>, _>>()?,
                                                        ((read(
                                                            &rows[(i) as usize].bat,
                                                            "psr[p].obsn[i].bat",
                                                        )?)
                                                        .clone())
                                                        .to_f64(),
                                                    )?,
                                                ) * (psr.param.slots[param_f as usize].cells
                                                    [0]
                                                .val)
                                                    .clone()));
                                        }
                                    }
                                }
                                iTNShape += 1;
                            }
                        }
                        phaseW = Float::with_val(PRECISION_BITS, 0.0f64);
                        if (psr.param.slots[param_wave_om as usize].cells[0].param_set == 1i32) {
                            {
                                let mut om: f64;
                                let mut dt: f64;
                                let mut om_eff: f64;
                                dt = ((read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?)
                                    .clone()
                                    - (psr.param.slots[param_waveepoch as usize].cells[0].val)
                                        .clone())
                                .to_f64();
                                om = ((psr.param.slots[param_wave_om as usize].cells[0].val)
                                    .clone())
                                .to_f64();
                                {
                                    k = 0i32;
                                    while (k < *read(&psr.nWhite, "psr[p].nWhite")?) {
                                        {
                                            if (*read(&psr.waveScale, "psr[p].waveScale")?
                                                == (0i32) as f64)
                                            {
                                                {
                                                    if (k == 0i32) {
                                                        {
                                                            om_eff = (om * ((k) as f64 + 1.0f64));
                                                        }
                                                    } else {
                                                        {
                                                            om_eff = (om * ((k) as f64 + 1.0f64));
                                                        }
                                                    }
                                                    phaseW = ((phaseW).clone()
                                                        + ((Float::with_val(
                                                            PRECISION_BITS,
                                                            (*read(
                                                                &psr.wave_sine[(k) as usize],
                                                                "psr[p].wave_sine[k]",
                                                            )? * (om_eff * dt).sin()),
                                                        ) * (psr.param.slots
                                                            [param_f as usize]
                                                            .cells[0]
                                                            .val)
                                                            .clone())
                                                            + (Float::with_val(
                                                                PRECISION_BITS,
                                                                (*read(
                                                                    &psr.wave_cos[(k) as usize],
                                                                    "psr[p].wave_cos[k]",
                                                                )? * (om_eff * dt).cos()),
                                                            ) * (psr.param.slots
                                                                [param_f as usize]
                                                                .cells[0]
                                                                .val)
                                                                .clone())));
                                                }
                                            } else {
                                                if (*read(&psr.waveScale, "psr[p].waveScale")?
                                                    == (1i32) as f64)
                                                {
                                                    {
                                                        let mut freq: f64 = *read(
                                                            &rows[(i) as usize].freq,
                                                            "psr[p].obsn[i].freq",
                                                        )?;
                                                        phaseW = ((phaseW).clone()
                                                            + (((Float::with_val(
                                                                PRECISION_BITS,
                                                                (*read(
                                                                    &psr.wave_sine[(k) as usize],
                                                                    "psr[p].wave_sine[k]",
                                                                )? * ((om * (k + 1i32) as f64)
                                                                    * dt)
                                                                    .sin()),
                                                            ) * (psr.param.slots
                                                                [param_f as usize]
                                                                .cells[0]
                                                                .val)
                                                                .clone())
                                                                + (Float::with_val(
                                                                    PRECISION_BITS,
                                                                    (*read(
                                                                        &psr.wave_cos[(k) as usize],
                                                                        "psr[p].wave_cos[k]",
                                                                    )? * ((om
                                                                        * (k + 1i32) as f64)
                                                                        * dt)
                                                                        .cos()),
                                                                ) * (psr.param.slots
                                                                    [param_f as usize]
                                                                    .cells[0]
                                                                    .val)
                                                                    .clone()))
                                                                / Float::with_val(
                                                                    PRECISION_BITS,
                                                                    ((2.41e-4f64 * freq) * freq),
                                                                )));
                                                    }
                                                } else {
                                                    if (*read(&psr.waveScale, "psr[p].waveScale")?
                                                        == (2i32) as f64)
                                                    {
                                                        {
                                                            let mut freq: f64 = *read(
                                                                &rows[(i) as usize].freq,
                                                                "psr[p].obsn[i].freq",
                                                            )?;
                                                            phaseW = ((phaseW).clone()
                                                                + ((Float::with_val(
                                                                    PRECISION_BITS,
                                                                    (*read(
                                                                        &psr.wave_sine
                                                                            [(k) as usize],
                                                                        "psr[p].wave_sine[k]",
                                                                    )? * ((om
                                                                        * (k + 1i32) as f64)
                                                                        * dt)
                                                                        .sin()),
                                                                ) * (psr.param.slots
                                                                    [param_f as usize]
                                                                    .cells[0]
                                                                    .val)
                                                                    .clone())
                                                                    + (Float::with_val(
                                                                        PRECISION_BITS,
                                                                        (*read(
                                                                            &psr.wave_cos
                                                                                [(k) as usize],
                                                                            "psr[p].wave_cos[k]",
                                                                        )? * ((om
                                                                            * (k + 1i32) as f64)
                                                                            * dt)
                                                                            .cos()),
                                                                    ) * (psr.param.slots
                                                                        [param_f as usize]
                                                                        .cells[0]
                                                                        .val)
                                                                        .clone())));
                                                            phaseW = ((phaseW).clone()
                                                                + (((Float::with_val(
                                                                    PRECISION_BITS,
                                                                    (*read(
                                                                        &psr.wave_sine_dm
                                                                            [(k) as usize],
                                                                        "psr[p].wave_sine_dm[k]",
                                                                    )? * ((om
                                                                        * (k + 1i32) as f64)
                                                                        * dt)
                                                                        .sin()),
                                                                ) * (psr.param.slots
                                                                    [param_f as usize]
                                                                    .cells[0]
                                                                    .val)
                                                                    .clone())
                                                                    + (Float::with_val(
                                                                        PRECISION_BITS,
                                                                        (*read(
                                                                            &psr.wave_cos_dm
                                                                                [(k) as usize],
                                                                            "psr[p].wave_cos_dm[k]",
                                                                        )? * ((om
                                                                            * (k + 1i32) as f64)
                                                                            * dt)
                                                                            .cos()),
                                                                    ) * (psr.param.slots
                                                                        [param_f as usize]
                                                                        .cells[0]
                                                                        .val)
                                                                        .clone()))
                                                                    / Float::with_val(
                                                                        PRECISION_BITS,
                                                                        ((2.41e-4f64 * freq)
                                                                            * freq),
                                                                    )));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        k += 1;
                                    }
                                }
                            }
                        }
                        if (psr.param.slots[param_wave_dm as usize].cells[0].param_set == 1i32) {
                            {
                                let mut om: f64;
                                let mut dt: f64;
                                dt = ((read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?)
                                    .clone()
                                    - (psr.param.slots[param_waveepoch_dm as usize].cells[0].val)
                                        .clone())
                                .to_f64();
                                om = ((psr.param.slots[param_wave_dm as usize].cells[0].val)
                                    .clone())
                                .to_f64();
                                let mut freq: f64 =
                                    *read(&rows[(i) as usize].freq, "psr[p].obsn[i].freq")?;
                                {
                                    k = 0i32;
                                    while (k < *read(&psr.nWhite_dm, "psr[p].nWhite_dm")?) {
                                        {
                                            phaseW = ((phaseW).clone()
                                                + (((Float::with_val(
                                                    PRECISION_BITS,
                                                    (*read(
                                                        &psr.wave_sine_dm[(k) as usize],
                                                        "psr[p].wave_sine_dm[k]",
                                                    )? * ((om * (k + 1i32) as f64) * dt).sin()),
                                                ) * (psr.param.slots[param_f as usize]
                                                    .cells[0]
                                                    .val)
                                                    .clone())
                                                    + (Float::with_val(
                                                        PRECISION_BITS,
                                                        (*read(
                                                            &psr.wave_cos_dm[(k) as usize],
                                                            "psr[p].wave_cos_dm[k]",
                                                        )? * ((om * (k + 1i32) as f64) * dt)
                                                            .cos()),
                                                    ) * (psr.param.slots[param_f as usize]
                                                        .cells[0]
                                                        .val)
                                                        .clone()))
                                                    / Float::with_val(
                                                        PRECISION_BITS,
                                                        ((2.41e-4f64 * freq) * freq),
                                                    )));
                                        }
                                        k += 1;
                                    }
                                }
                            }
                        }
                        if (psr.param.slots[param_quad_om as usize].cells[0].param_set == 1i32) {
                            {
                                let mut omega_g: Option<f64> = None;
                                let mut resp: Float;
                                let mut resc: Float;
                                let mut res_r: Option<Float> = None;
                                let mut res_i: Option<Float> = None;
                                let mut lambda_p: f64;
                                let mut beta_p: f64;
                                let mut lambda: f64;
                                let mut beta: f64;
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
                                let mut dt: f64;
                                dt = (((read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?)
                                    .clone()
                                    - Float::with_val(
                                        PRECISION_BITS,
                                        *read(&psr.quadEpoch, "psr[p].quadEpoch")?,
                                    ))
                                    * Float::with_val(PRECISION_BITS, 86400.0f64))
                                .to_f64();
                                if (psr.param.slots[param_raj as usize].cells[1].param_set == 1i32)
                                {
                                    lambda_p = ((psr.param.slots[param_raj as usize].cells[1].val)
                                        .clone())
                                    .to_f64();
                                } else {
                                    lambda_p = ((psr.param.slots[param_raj as usize].cells[0].val)
                                        .clone())
                                    .to_f64();
                                }
                                if (psr.param.slots[param_decj as usize].cells[1].param_set == 1i32)
                                {
                                    beta_p = ((psr.param.slots[param_decj as usize].cells[1].val)
                                        .clone())
                                    .to_f64();
                                } else {
                                    beta_p = ((psr.param.slots[param_decj as usize].cells[0].val)
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
                                    * (Float::with_val(PRECISION_BITS, (lambda - lambda_p)))
                                        .cos())
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
                                {
                                    k = 0i32;
                                    while (k < *read(&psr.nQuad, "psr[p].nQuad")?) {
                                        {
                                            omega_g = Some(
                                                ((psr.param.slots[param_quad_om as usize].cells
                                                    [0]
                                                .val)
                                                    .clone())
                                                .to_f64()
                                                    * (k + 1i32) as f64,
                                            );
                                            res_r = Some(
                                                ((Float::with_val(
                                                    PRECISION_BITS,
                                                    *read(
                                                        &psr.quad_aplus_r[(k) as usize],
                                                        "psr[p].quad_aplus_r[k]",
                                                    )?,
                                                ) * (resp).clone())
                                                    + (Float::with_val(
                                                        PRECISION_BITS,
                                                        *read(
                                                            &psr.quad_across_r[(k) as usize],
                                                            "psr[p].quad_across_r[k]",
                                                        )?,
                                                    ) * (resc).clone()))
                                                    * (Float::with_val(
                                                        PRECISION_BITS,
                                                        (*read(&omega_g, "omega_g")? * dt),
                                                    ))
                                                    .sin(),
                                            );
                                            res_i = Some(
                                                ((Float::with_val(
                                                    PRECISION_BITS,
                                                    *read(
                                                        &psr.quad_aplus_i[(k) as usize],
                                                        "psr[p].quad_aplus_i[k]",
                                                    )?,
                                                ) * (resp).clone())
                                                    + (Float::with_val(
                                                        PRECISION_BITS,
                                                        *read(
                                                            &psr.quad_across_i[(k) as usize],
                                                            "psr[p].quad_across_i[k]",
                                                        )?,
                                                    ) * (resc).clone()))
                                                    * (Float::with_val(
                                                        PRECISION_BITS,
                                                        (*read(&omega_g, "omega_g")? * dt),
                                                    ))
                                                    .cos(),
                                            );
                                            if (*read(&psr.gwsrc_psrdist, "psr[p].gwsrc_psrdist")?
                                                > (0i32) as f64)
                                            {
                                                {
                                                    d.stdout(&[b"Pulsar distance = ".to_vec(),crate::clock_text::format_number(*read(&psr.gwsrc_psrdist, "psr[p].gwsrc_psrdist")?, crate::clock_text::NumberFormat::GeneralSix),b"\n".to_vec()].concat());
                                                    res_r = Some((read(&res_r, "res_r")?).clone() - (((Float::with_val(PRECISION_BITS, *read(&psr.quad_aplus_r[(k) as usize], "psr[p].quad_aplus_r[k]")?) * (resp).clone()) + (Float::with_val(PRECISION_BITS, *read(&psr.quad_across_r[(k) as usize], "psr[p].quad_across_r[k]")?) * (resc).clone())) * (Float::with_val(PRECISION_BITS, ((*read(&omega_g, "omega_g")? * dt) - (((((1i32) as f64 - cosTheta) * *read(&psr.gwsrc_psrdist, "psr[p].gwsrc_psrdist")?) / 299792458.0f64) * *read(&omega_g, "omega_g")?)))).sin()));
                                                    res_i = Some((read(&res_i, "res_i")?).clone() - (((Float::with_val(PRECISION_BITS, *read(&psr.quad_aplus_i[(k) as usize], "psr[p].quad_aplus_i[k]")?) * (resp).clone()) + (Float::with_val(PRECISION_BITS, *read(&psr.quad_across_i[(k) as usize], "psr[p].quad_across_i[k]")?) * (resc).clone())) * (Float::with_val(PRECISION_BITS, ((*read(&omega_g, "omega_g")? * dt) - (((((1i32) as f64 - cosTheta) * *read(&psr.gwsrc_psrdist, "psr[p].gwsrc_psrdist")?) / 299792458.0f64) * *read(&omega_g, "omega_g")?)))).cos()));
                                                }
                                            }
                                            if (((1i32) as f64 - cosTheta) == 0.0f64) {
                                                {
                                                    res_r = Some(Float::with_val(
                                                        PRECISION_BITS,
                                                        0.0f64,
                                                    ));
                                                    res_i = Some(Float::with_val(
                                                        PRECISION_BITS,
                                                        0.0f64,
                                                    ));
                                                }
                                            } else {
                                                {
                                                    res_r = Some(
                                                        (Float::with_val(
                                                            PRECISION_BITS,
                                                            Float::parse("1.0").unwrap(),
                                                        ) / ((Float::with_val(
                                                            PRECISION_BITS,
                                                            Float::parse("2.0").unwrap(),
                                                        ) * Float::with_val(
                                                            PRECISION_BITS,
                                                            *read(&omega_g, "omega_g")?,
                                                        )) * (Float::with_val(
                                                            PRECISION_BITS,
                                                            Float::parse("1.0").unwrap(),
                                                        ) - Float::with_val(
                                                            PRECISION_BITS,
                                                            cosTheta,
                                                        )))) * (read(&res_r, "res_r")?).clone(),
                                                    );
                                                    res_i = Some(
                                                        (Float::with_val(
                                                            PRECISION_BITS,
                                                            Float::parse("1.0").unwrap(),
                                                        ) / ((Float::with_val(
                                                            PRECISION_BITS,
                                                            Float::parse("2.0").unwrap(),
                                                        ) * Float::with_val(
                                                            PRECISION_BITS,
                                                            *read(&omega_g, "omega_g")?,
                                                        )) * (Float::with_val(
                                                            PRECISION_BITS,
                                                            Float::parse("1.0").unwrap(),
                                                        ) - Float::with_val(
                                                            PRECISION_BITS,
                                                            cosTheta,
                                                        )))) * (read(&res_i, "res_i")?).clone(),
                                                    );
                                                }
                                            }
                                            phaseW = ((phaseW).clone()
                                                + (((read(&res_r, "res_r")?).clone()
                                                    + (read(&res_i, "res_i")?).clone())
                                                    * (psr.param.slots[param_f as usize].cells
                                                        [0]
                                                    .val)
                                                        .clone()));
                                        }
                                        k += 1;
                                    }
                                }
                            }
                        }
                        if (psr.param.slots[param_gwecc as usize].cells[0].param_set == 1i32) {
                            {
                                if (i == 0i32) {
                                    {
                                        let mut ra_p: f64 =
                                            ((psr.param.slots[param_raj as usize].cells[0].val)
                                                .clone())
                                            .to_f64();
                                        let mut dec_p: f64 =
                                            ((-((psr.param.slots[param_decj as usize].cells[0]
                                                .val)
                                                .clone())
                                            .to_f64())
                                                + (std::f64::consts::PI / 2.0f64));
                                        let mut ra_g: f64 =
                                            *read(&psr.gwecc_ra, "psr[p].gwecc_ra")?;
                                        let mut dec_g: f64 =
                                            *read(&psr.gwecc_dec, "psr[p].gwecc_dec")?;
                                        d.stdout(
                                            &[
                                                b"Have ra_p ".to_vec(),
                                                crate::clock_text::format_number(
                                                    ra_p,
                                                    crate::clock_text::NumberFormat::GeneralSix,
                                                ),
                                                b" ra_g ".to_vec(),
                                                crate::clock_text::format_number(
                                                    ra_g,
                                                    crate::clock_text::NumberFormat::GeneralSix,
                                                ),
                                                b" dec_p ".to_vec(),
                                                crate::clock_text::format_number(
                                                    dec_p,
                                                    crate::clock_text::NumberFormat::GeneralSix,
                                                ),
                                                b" dec_g ".to_vec(),
                                                crate::clock_text::format_number(
                                                    dec_g,
                                                    crate::clock_text::NumberFormat::GeneralSix,
                                                ),
                                                b"\n".to_vec(),
                                            ]
                                            .concat(),
                                        );
                                        let mut cosMu: f64 =
                                            ((((Float::with_val(PRECISION_BITS, dec_g)).cos()
                                                * (Float::with_val(PRECISION_BITS, dec_p)).cos())
                                                * (Float::with_val(
                                                    PRECISION_BITS,
                                                    (ra_g - ra_p),
                                                ))
                                                .cos())
                                                + ((Float::with_val(PRECISION_BITS, dec_g)).sin()
                                                    * (Float::with_val(PRECISION_BITS, dec_p))
                                                        .sin()))
                                            .to_f64();
                                        prev_p = Some(
                                            (*read(
                                                &psr.gwecc_orbital_period,
                                                "psr[p].gwecc_orbital_period",
                                            )? * 365.25f64)
                                                * 86400.0f64,
                                        );
                                        prev_e = Some(*read(&psr.gwecc_e, "psr[p].gwecc_e")?);
                                        prev_epoch =
                                            Some(*read(&psr.gwecc_epoch, "psr[p].gwecc_epoch")?);
                                        prev_theta = Some(*read(
                                            &psr.gwecc_theta_0,
                                            "psr[p].gwecc_theta_0",
                                        )?);
                                        prev_a = Some(
                                            (((6.67e-11f64
                                                * (*read(&psr.gwecc_m1, "psr[p].gwecc_m1")?
                                                    + *read(&psr.gwecc_m2, "psr[p].gwecc_m2")?))
                                                * 1.9891e30f64)
                                                * ((2.0f64 * std::f64::consts::PI)
                                                    / *read(&prev_p, "prev_p")?)
                                                .powf(-2.0f64))
                                            .powf(1.0f64 / 3.0f64),
                                        );
                                        prev_p_p = Some(
                                            (*read(
                                                &psr.gwecc_orbital_period,
                                                "psr[p].gwecc_orbital_period",
                                            )? * 365.25f64)
                                                * 86400.0f64,
                                        );
                                        prev_e_p = Some(*read(&psr.gwecc_e, "psr[p].gwecc_e")?);
                                        prev_epoch_p = Some(
                                            *read(&psr.gwecc_epoch, "psr[p].gwecc_epoch")?
                                                + (((1.0f64 / 86400.0f64)
                                                    * ((*read(
                                                        &psr.gwecc_psrdist,
                                                        "psr[p].gwecc_psrdist",
                                                    )? * 3.08568e19f64)
                                                        / 2.998e8f64))
                                                    * (1.0f64 - cosMu)),
                                        );
                                        prev_theta_p = Some(*read(
                                            &psr.gwecc_theta_0,
                                            "psr[p].gwecc_theta_0",
                                        )?);
                                        prev_a_p = Some(
                                            (((6.67e-11f64
                                                * (*read(&psr.gwecc_m1, "psr[p].gwecc_m1")?
                                                    + *read(&psr.gwecc_m2, "psr[p].gwecc_m2")?))
                                                * 1.9891e30f64)
                                                * ((2.0f64 * std::f64::consts::PI)
                                                    / *read(&prev_p, "prev_p")?)
                                                .powf(-2.0f64))
                                            .powf(1.0f64 / 3.0f64),
                                        );
                                    }
                                }
                                if (*read(&psr.gwecc_pulsarTermOn, "psr[p].gwecc_pulsarTermOn")?
                                    == 0i32)
                                {
                                    {
                                        d.stdout(b"ONLY EARTH\n");
                                        phaseW = ((phaseW).clone()
                                            + (((psr.param.slots[param_gwecc as usize].cells[0]
                                                .val)
                                                .clone()
                                                * crate::gw_ecc::eccRes(
                                                    psr,
                                                    rows,
                                                    i,
                                                    &mut coalesceFlag,
                                                    &mut prev_p,
                                                    &mut prev_e,
                                                    &mut prev_a,
                                                    &mut prev_epoch,
                                                    &mut prev_theta,
                                                    d,
                                                )?)
                                                * (psr.param.slots[param_f as usize].cells[0]
                                                    .val)
                                                    .clone()));
                                    }
                                }
                                if (*read(&psr.gwecc_pulsarTermOn, "psr[p].gwecc_pulsarTermOn")?
                                    == 1i32)
                                {
                                    {
                                        d.stdout(b"PULSAR + EARTH\n");
                                        phaseW = ((phaseW).clone()
                                            + (((psr.param.slots[param_gwecc as usize].cells[0]
                                                .val)
                                                .clone()
                                                * (crate::gw_ecc::eccRes(
                                                    psr,
                                                    rows,
                                                    i,
                                                    &mut coalesceFlag,
                                                    &mut prev_p,
                                                    &mut prev_e,
                                                    &mut prev_a,
                                                    &mut prev_epoch,
                                                    &mut prev_theta,
                                                    d,
                                                )? - crate::gw_ecc::eccRes(
                                                    psr,
                                                    rows,
                                                    i,
                                                    &mut coalesceFlag_p,
                                                    &mut prev_p_p,
                                                    &mut prev_e_p,
                                                    &mut prev_a_p,
                                                    &mut prev_epoch_p,
                                                    &mut prev_theta_p,
                                                    d,
                                                )?))
                                                * (psr.param.slots[param_f as usize].cells[0]
                                                    .val)
                                                    .clone()));
                                    }
                                }
                                if (*read(&psr.gwecc_pulsarTermOn, "psr[p].gwecc_pulsarTermOn")?
                                    == 2i32)
                                {
                                    {
                                        d.stdout(b"ONLY PULSAR\n");
                                        phaseW = ((phaseW).clone()
                                            + (((psr.param.slots[param_gwecc as usize].cells[0]
                                                .val)
                                                .clone()
                                                * (-crate::gw_ecc::eccRes(
                                                    psr,
                                                    rows,
                                                    i,
                                                    &mut coalesceFlag_p,
                                                    &mut prev_p_p,
                                                    &mut prev_e_p,
                                                    &mut prev_a_p,
                                                    &mut prev_epoch_p,
                                                    &mut prev_theta_p,
                                                    d,
                                                )?))
                                                * (psr.param.slots[param_f as usize].cells[0]
                                                    .val)
                                                    .clone()));
                                    }
                                }
                            }
                        }
                        if ((psr.param.slots[param_gwsingle as usize].cells[0].param_set == 1i32)
                            || (psr.param.slots[param_cgw as usize].cells[0].param_set == 1i32))
                        {
                            {
                                let mut omega_g: Option<f64> = None;
                                let mut resp: Float;
                                let mut resc: Float;
                                let mut res_r: Option<Float> = None;
                                let mut res_i: Option<Float> = None;
                                let mut lambda_p: f64;
                                let mut beta_p: f64;
                                let mut lambda: f64;
                                let mut beta: f64;
                                let mut time: Float;
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
                                time = (((read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?)
                                    .clone()
                                    - Float::with_val(
                                        PRECISION_BITS,
                                        *read(&psr.gwsrc_epoch, "psr[p].gwsrc_epoch")?,
                                    ))
                                    * Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("86400.0").unwrap(),
                                    ));
                                lambda_p = ((psr.param.slots[param_raj as usize].cells[0].val)
                                    .clone())
                                .to_f64();
                                beta_p = ((psr.param.slots[param_decj as usize].cells[0].val)
                                    .clone())
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
                                    * (Float::with_val(PRECISION_BITS, (lambda - lambda_p)))
                                        .cos())
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
                                if (psr.param.slots[param_gwsingle as usize].cells[0].param_set
                                    == 1i32)
                                {
                                    omega_g = Some(
                                        ((psr.param.slots[param_gwsingle as usize].cells[0].val)
                                            .clone())
                                        .to_f64(),
                                    );
                                } else {
                                    if (psr.param.slots[param_cgw as usize].cells[0].param_set
                                        == 1i32)
                                    {
                                        omega_g = Some(
                                            ((psr.param.slots[param_cgw as usize].cells[0].val)
                                                .clone())
                                            .to_f64(),
                                        );
                                    }
                                }
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
                                if (psr.param.slots[param_gwsingle as usize].cells[0].param_set
                                    == 1i32)
                                {
                                    {
                                        res_r = Some(
                                            ((Float::with_val(
                                                PRECISION_BITS,
                                                *read(&psr.gwsrc_aplus_r, "psr[p].gwsrc_aplus_r")?,
                                            ) * (resp).clone())
                                                + (Float::with_val(
                                                    PRECISION_BITS,
                                                    *read(
                                                        &psr.gwsrc_across_r,
                                                        "psr[p].gwsrc_across_r",
                                                    )?,
                                                ) * (resc).clone()))
                                                * (Float::with_val(
                                                    PRECISION_BITS,
                                                    *read(&omega_g, "omega_g")?,
                                                ) * (time).clone())
                                                .sin(),
                                        );
                                        res_i = Some(
                                            ((Float::with_val(
                                                PRECISION_BITS,
                                                *read(&psr.gwsrc_aplus_i, "psr[p].gwsrc_aplus_i")?,
                                            ) * (resp).clone())
                                                + (Float::with_val(
                                                    PRECISION_BITS,
                                                    *read(
                                                        &psr.gwsrc_across_i,
                                                        "psr[p].gwsrc_across_i",
                                                    )?,
                                                ) * (resc).clone()))
                                                * (Float::with_val(
                                                    PRECISION_BITS,
                                                    *read(&omega_g, "omega_g")?,
                                                ) * (time).clone())
                                                .cos(),
                                        );
                                        if (*read(&psr.gwsrc_psrdist, "psr[p].gwsrc_psrdist")?
                                            > (0i32) as f64)
                                        {
                                            {
                                                res_r = Some(
                                                    (read(&res_r, "res_r")?).clone()
                                                        + (((Float::with_val(
                                                            PRECISION_BITS,
                                                            *read(
                                                                &psr.gwsrc_aplus_r,
                                                                "psr[p].gwsrc_aplus_r",
                                                            )?,
                                                        ) * (resp).clone())
                                                            + (Float::with_val(
                                                                PRECISION_BITS,
                                                                *read(
                                                                    &psr.gwsrc_across_r,
                                                                    "psr[p].gwsrc_across_r",
                                                                )?,
                                                            ) * (resc).clone()))
                                                            * ((Float::with_val(
                                                                PRECISION_BITS,
                                                                *read(&omega_g, "omega_g")?,
                                                            ) * (time).clone())
                                                                - Float::with_val(
                                                                    PRECISION_BITS,
                                                                    (((((1i32) as f64
                                                                        - cosTheta)
                                                                        * *read(
                                                                            &psr.gwsrc_psrdist,
                                                                            "psr[p].gwsrc_psrdist",
                                                                        )?)
                                                                        / 299792458.0f64)
                                                                        * *read(
                                                                            &omega_g, "omega_g",
                                                                        )?),
                                                                ))
                                                            .sin()),
                                                );
                                                res_i = Some(
                                                    (read(&res_i, "res_i")?).clone()
                                                        + (((Float::with_val(
                                                            PRECISION_BITS,
                                                            *read(
                                                                &psr.gwsrc_aplus_i,
                                                                "psr[p].gwsrc_aplus_i",
                                                            )?,
                                                        ) * (resp).clone())
                                                            + (Float::with_val(
                                                                PRECISION_BITS,
                                                                *read(
                                                                    &psr.gwsrc_across_i,
                                                                    "psr[p].gwsrc_across_i",
                                                                )?,
                                                            ) * (resc).clone()))
                                                            * ((Float::with_val(
                                                                PRECISION_BITS,
                                                                *read(&omega_g, "omega_g")?,
                                                            ) * (time).clone())
                                                                - Float::with_val(
                                                                    PRECISION_BITS,
                                                                    (((((1i32) as f64
                                                                        - cosTheta)
                                                                        * *read(
                                                                            &psr.gwsrc_psrdist,
                                                                            "psr[p].gwsrc_psrdist",
                                                                        )?)
                                                                        / 299792458.0f64)
                                                                        * *read(
                                                                            &omega_g, "omega_g",
                                                                        )?),
                                                                ))
                                                            .cos()),
                                                );
                                            }
                                        }
                                        if (((1i32) as f64 - cosTheta) == 0.0f64) {
                                            {
                                                res_r =
                                                    Some(Float::with_val(PRECISION_BITS, 0.0f64));
                                                res_i =
                                                    Some(Float::with_val(PRECISION_BITS, 0.0f64));
                                            }
                                        } else {
                                            {
                                                res_r = Some(
                                                    (Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("1.0").unwrap(),
                                                    ) / ((Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("2.0").unwrap(),
                                                    ) * Float::with_val(
                                                        PRECISION_BITS,
                                                        *read(&omega_g, "omega_g")?,
                                                    )) * (Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("1.0").unwrap(),
                                                    ) - Float::with_val(
                                                        PRECISION_BITS,
                                                        cosTheta,
                                                    )))) * (read(&res_r, "res_r")?).clone(),
                                                );
                                                res_i = Some(
                                                    (Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("1.0").unwrap(),
                                                    ) / ((Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("2.0").unwrap(),
                                                    ) * Float::with_val(
                                                        PRECISION_BITS,
                                                        *read(&omega_g, "omega_g")?,
                                                    )) * (Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("1.0").unwrap(),
                                                    ) - Float::with_val(
                                                        PRECISION_BITS,
                                                        cosTheta,
                                                    )))) * (read(&res_i, "res_i")?).clone(),
                                                );
                                            }
                                        }
                                    }
                                } else {
                                    if (psr.param.slots[param_cgw as usize].cells[0].param_set
                                        == 1i32)
                                    {
                                        {
                                            res_r = Some(
                                                ((Float::with_val(
                                                    PRECISION_BITS,
                                                    (*read(&psr.cgw_h0, "psr[p].cgw_h0")?
                                                        / *read(&omega_g, "omega_g")?),
                                                ) * ((Float::with_val(
                                                    PRECISION_BITS,
                                                    (((1i32) as f64
                                                        + (*read(
                                                            &psr.cgw_cosinc,
                                                            "psr[p].cgw_cosinc",
                                                        )?)
                                                        .powf((2i32) as f64))
                                                        * ((2i32) as f64
                                                            * *read(
                                                                &psr.cgw_angpol,
                                                                "psr[p].cgw_angpol",
                                                            )?)
                                                        .cos()),
                                                ) * (Float::with_val(
                                                    PRECISION_BITS,
                                                    *read(&omega_g, "omega_g")?,
                                                ) * (time).clone())
                                                .sin())
                                                    + (Float::with_val(
                                                        PRECISION_BITS,
                                                        (((2i32) as f64
                                                            * *read(
                                                                &psr.cgw_cosinc,
                                                                "psr[p].cgw_cosinc",
                                                            )?)
                                                            * ((2i32) as f64
                                                                * *read(
                                                                    &psr.cgw_angpol,
                                                                    "psr[p].cgw_angpol",
                                                                )?)
                                                            .sin()),
                                                    ) * (Float::with_val(
                                                        PRECISION_BITS,
                                                        *read(&omega_g, "omega_g")?,
                                                    ) * (time).clone())
                                                    .cos())))
                                                    * (resp).clone())
                                                    + ((Float::with_val(
                                                        PRECISION_BITS,
                                                        (*read(&psr.cgw_h0, "psr[p].cgw_h0")?
                                                            / *read(&omega_g, "omega_g")?),
                                                    ) * ((Float::with_val(
                                                        PRECISION_BITS,
                                                        (((1i32) as f64
                                                            + (*read(
                                                                &psr.cgw_cosinc,
                                                                "psr[p].cgw_cosinc",
                                                            )?)
                                                            .powf((2i32) as f64))
                                                            * ((2i32) as f64
                                                                * *read(
                                                                    &psr.cgw_angpol,
                                                                    "psr[p].cgw_angpol",
                                                                )?)
                                                            .sin()),
                                                    ) * (Float::with_val(
                                                        PRECISION_BITS,
                                                        *read(&omega_g, "omega_g")?,
                                                    ) * (time).clone())
                                                    .sin())
                                                        - (Float::with_val(
                                                            PRECISION_BITS,
                                                            (((2i32) as f64
                                                                * *read(
                                                                    &psr.cgw_cosinc,
                                                                    "psr[p].cgw_cosinc",
                                                                )?)
                                                                * ((2i32) as f64
                                                                    * *read(
                                                                        &psr.cgw_angpol,
                                                                        "psr[p].cgw_angpol",
                                                                    )?)
                                                                .cos()),
                                                        ) * (Float::with_val(
                                                            PRECISION_BITS,
                                                            *read(&omega_g, "omega_g")?,
                                                        ) * (time).clone())
                                                        .cos())))
                                                        * (resc).clone()),
                                            );
                                            res_i = Some(Float::with_val(PRECISION_BITS, 0.0f64));
                                            d.stdout(
                                                &[
                                                    b"res_r = ".to_vec(),
                                                    crate::clock_text::format_number(
                                                        ((read(&res_r, "res_r")?).clone()).to_f64(),
                                                        crate::clock_text::NumberFormat::GeneralSix,
                                                    ),
                                                    b" ".to_vec(),
                                                    crate::clock_text::format_number(
                                                        ((read(&res_i, "res_i")?).clone()).to_f64(),
                                                        crate::clock_text::NumberFormat::GeneralSix,
                                                    ),
                                                    b" ".to_vec(),
                                                    crate::clock_text::format_number(
                                                        ((resp).clone()).to_f64(),
                                                        crate::clock_text::NumberFormat::GeneralSix,
                                                    ),
                                                    b" ".to_vec(),
                                                    crate::clock_text::format_number(
                                                        ((resc).clone()).to_f64(),
                                                        crate::clock_text::NumberFormat::GeneralSix,
                                                    ),
                                                    b" omega = ".to_vec(),
                                                    crate::clock_text::format_number(
                                                        *read(&omega_g, "omega_g")?,
                                                        crate::clock_text::NumberFormat::GeneralSix,
                                                    ),
                                                    b" ".to_vec(),
                                                    crate::clock_text::format_number(
                                                        *read(&psr.cgw_h0, "psr[p].cgw_h0")?,
                                                        crate::clock_text::NumberFormat::GeneralSix,
                                                    ),
                                                    b" ".to_vec(),
                                                    crate::clock_text::format_number(
                                                        *read(
                                                            &psr.cgw_cosinc,
                                                            "psr[p].cgw_cosinc",
                                                        )?,
                                                        crate::clock_text::NumberFormat::GeneralSix,
                                                    ),
                                                    b" ".to_vec(),
                                                    crate::clock_text::format_number(
                                                        *read(
                                                            &psr.cgw_angpol,
                                                            "psr[p].cgw_angpol",
                                                        )?,
                                                        crate::clock_text::NumberFormat::GeneralSix,
                                                    ),
                                                    b" time = ".to_vec(),
                                                    crate::clock_text::format_number(
                                                        ((time).clone()).to_f64(),
                                                        crate::clock_text::NumberFormat::GeneralSix,
                                                    ),
                                                    b"\n".to_vec(),
                                                ]
                                                .concat(),
                                            );
                                            if (*read(&psr.gwsrc_psrdist, "psr[p].gwsrc_psrdist")?
                                                > (0i32) as f64)
                                            {
                                                {
                                                    let mut omega_prime_g: f64;
                                                    let mut h0_prime: f64;
                                                    if (*read(&psr.cgw_mc, "psr[p].cgw_mc")?
                                                        == (0i32) as f64)
                                                    {
                                                        {
                                                            omega_prime_g =
                                                                *read(&omega_g, "omega_g")?;
                                                            h0_prime = *read(
                                                                &psr.cgw_h0,
                                                                "psr[p].cgw_h0",
                                                            )?;
                                                        }
                                                    } else {
                                                        {
                                                            omega_prime_g = (((2i32) as f64
                                                                * std::f64::consts::PI)
                                                                * ((((((((((1i32) as f64
                                                                    - cosTheta)
                                                                    * *read(
                                                                        &psr.gwsrc_psrdist,
                                                                        "psr[p].gwsrc_psrdist",
                                                                    )?)
                                                                    / 299792458.0f64)
                                                                    * 256.0f64)
                                                                    / 5.0f64)
                                                                    / (299792458.0f64)
                                                                        .powf((5i32) as f64))
                                                                    * (std::f64::consts::PI)
                                                                        .powf(8.0f64 / 3.0f64))
                                                                    * (1.3271243999e20f64
                                                                        * *read(
                                                                            &psr.cgw_mc,
                                                                            "psr[p].cgw_mc",
                                                                        )?)
                                                                    .powf(5.0f64 / 3.0f64))
                                                                    + ((*read(
                                                                        &omega_g, "omega_g",
                                                                    )? / 2.0f64)
                                                                        / std::f64::consts::PI)
                                                                        .powf((-8.0f64) / 3.0f64))
                                                                .powf((-3.0f64) / 8.0f64));
                                                            h0_prime = (*read(
                                                                &psr.cgw_h0,
                                                                "psr[p].cgw_h0",
                                                            )? * (omega_prime_g
                                                                / *read(&omega_g, "omega_g")?)
                                                            .powf(2.0f64 / 3.0f64));
                                                            d.stdout(&[b"Using: omega_prime_g = ".to_vec(),crate::clock_text::format_number(omega_prime_g, crate::clock_text::NumberFormat::GeneralSix),b", omega_g = ".to_vec(),crate::clock_text::format_number(*read(&omega_g, "omega_g")?, crate::clock_text::NumberFormat::GeneralSix),b", h0_prime = ".to_vec(),crate::clock_text::format_number(h0_prime, crate::clock_text::NumberFormat::GeneralSix),b", h0 = ".to_vec(),crate::clock_text::format_number(*read(&psr.cgw_h0, "psr[p].cgw_h0")?, crate::clock_text::NumberFormat::GeneralSix),b"\n".to_vec()].concat());
                                                        }
                                                    }
                                                    res_r = Some((read(&res_r, "res_r")?).clone() - (((Float::with_val(PRECISION_BITS, (h0_prime / omega_prime_g)) * ((Float::with_val(PRECISION_BITS, (((1i32) as f64 + (*read(&psr.cgw_cosinc, "psr[p].cgw_cosinc")?).powf((2i32) as f64)) * ((2i32) as f64 * *read(&psr.cgw_angpol, "psr[p].cgw_angpol")?).cos())) * ((Float::with_val(PRECISION_BITS, omega_prime_g) * (time).clone()) - Float::with_val(PRECISION_BITS, (((((1i32) as f64 - cosTheta) * *read(&psr.gwsrc_psrdist, "psr[p].gwsrc_psrdist")?) / 299792458.0f64) * omega_prime_g))).sin()) + (Float::with_val(PRECISION_BITS, (((2i32) as f64 * *read(&psr.cgw_cosinc, "psr[p].cgw_cosinc")?) * ((2i32) as f64 * *read(&psr.cgw_angpol, "psr[p].cgw_angpol")?).sin())) * ((Float::with_val(PRECISION_BITS, omega_prime_g) * (time).clone()) - Float::with_val(PRECISION_BITS, (((((1i32) as f64 - cosTheta) * *read(&psr.gwsrc_psrdist, "psr[p].gwsrc_psrdist")?) / 299792458.0f64) * omega_prime_g))).cos()))) * (resp).clone()) + ((Float::with_val(PRECISION_BITS, (h0_prime / omega_prime_g)) * ((Float::with_val(PRECISION_BITS, (((1i32) as f64 + (*read(&psr.cgw_cosinc, "psr[p].cgw_cosinc")?).powf((2i32) as f64)) * ((2i32) as f64 * *read(&psr.cgw_angpol, "psr[p].cgw_angpol")?).sin())) * ((Float::with_val(PRECISION_BITS, omega_prime_g) * (time).clone()) - Float::with_val(PRECISION_BITS, (((((1i32) as f64 - cosTheta) * *read(&psr.gwsrc_psrdist, "psr[p].gwsrc_psrdist")?) / 299792458.0f64) * omega_prime_g))).sin()) - (Float::with_val(PRECISION_BITS, (((2i32) as f64 * *read(&psr.cgw_cosinc, "psr[p].cgw_cosinc")?) * ((2i32) as f64 * *read(&psr.cgw_angpol, "psr[p].cgw_angpol")?).cos())) * ((Float::with_val(PRECISION_BITS, omega_prime_g) * (time).clone()) - Float::with_val(PRECISION_BITS, (((((1i32) as f64 - cosTheta) * *read(&psr.gwsrc_psrdist, "psr[p].gwsrc_psrdist")?) / 299792458.0f64) * omega_prime_g))).cos()))) * (resc).clone())));
                                                }
                                            }
                                            if (((1i32) as f64 - cosTheta) == 0.0f64) {
                                                {
                                                    res_r = Some(Float::with_val(
                                                        PRECISION_BITS,
                                                        0.0f64,
                                                    ));
                                                    res_i = Some(Float::with_val(
                                                        PRECISION_BITS,
                                                        0.0f64,
                                                    ));
                                                }
                                            } else {
                                                {
                                                    res_r = Some(
                                                        ((-Float::with_val(
                                                            PRECISION_BITS,
                                                            Float::parse("1.0").unwrap(),
                                                        )) / (Float::with_val(
                                                            PRECISION_BITS,
                                                            Float::parse("2.0").unwrap(),
                                                        ) * (Float::with_val(
                                                            PRECISION_BITS,
                                                            Float::parse("1.0").unwrap(),
                                                        ) - Float::with_val(
                                                            PRECISION_BITS,
                                                            cosTheta,
                                                        )))) * (read(&res_r, "res_r")?).clone(),
                                                    );
                                                    res_i = Some(
                                                        ((-Float::with_val(
                                                            PRECISION_BITS,
                                                            Float::parse("1.0").unwrap(),
                                                        )) / (Float::with_val(
                                                            PRECISION_BITS,
                                                            Float::parse("2.0").unwrap(),
                                                        ) * (Float::with_val(
                                                            PRECISION_BITS,
                                                            Float::parse("1.0").unwrap(),
                                                        ) - Float::with_val(
                                                            PRECISION_BITS,
                                                            cosTheta,
                                                        )))) * (read(&res_i, "res_i")?).clone(),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                                phaseW = ((phaseW).clone()
                                    + (((read(&res_r, "res_r")?).clone()
                                        + (read(&res_i, "res_i")?).clone())
                                        * (psr.param.slots[param_f as usize].cells[0].val)
                                            .clone()));
                            }
                        }
                        if ((psr.param.slots[param_gwm_amp as usize].cells[0].param_set == 1i32)
                            && (psr.param.slots[param_gwm_amp as usize].cells[1].param_set == 0i32))
                        {
                            {
                                let mut lambda_p: f64;
                                let mut beta_p: f64;
                                let mut lambda: f64;
                                let mut beta: f64;
                                let mut n1: f64;
                                let mut n2: f64;
                                let mut n3: f64;
                                let mut cosTheta: f64;
                                let mut g1: f64;
                                let mut g2: f64;
                                let mut g3: f64;
                                time = crate::ifteph::extended_i32(
                                    &(((read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?)
                                        .clone()
                                        - Float::with_val(
                                            PRECISION_BITS,
                                            *read(&psr.gwm_epoch, "psr[p].gwm_epoch")?,
                                        ))
                                        * Float::with_val(
                                            PRECISION_BITS,
                                            Float::parse("86400.0").unwrap(),
                                        )),
                                    "binary_integer",
                                )?;
                                if (psr.param.slots[param_raj as usize].cells[1].param_set == 1i32)
                                {
                                    lambda_p = ((psr.param.slots[param_raj as usize].cells[1].val)
                                        .clone())
                                    .to_f64();
                                } else {
                                    lambda_p = ((psr.param.slots[param_raj as usize].cells[0].val)
                                        .clone())
                                    .to_f64();
                                }
                                if (psr.param.slots[param_decj as usize].cells[1].param_set == 1i32)
                                {
                                    beta_p = ((psr.param.slots[param_decj as usize].cells[1].val)
                                        .clone())
                                    .to_f64();
                                } else {
                                    beta_p = ((psr.param.slots[param_decj as usize].cells[0].val)
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
                                    * (Float::with_val(PRECISION_BITS, (lambda - lambda_p)))
                                        .cos())
                                    + ((Float::with_val(PRECISION_BITS, beta)).sin()
                                        * (Float::with_val(PRECISION_BITS, beta_p)).sin())))
                                .to_f64();
                                if ((read(&rows[(i) as usize].sat, "psr[p].obsn[i].sat")?).clone()
                                    >= Float::with_val(
                                        PRECISION_BITS,
                                        *read(&psr.gwm_epoch, "psr[p].gwm_epoch")?,
                                    ))
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
                                                            ((0.5f64 * std::f64::consts::PI)
                                                                - beta),
                                                        ))
                                                        .cos())
                                                    .to_f64(),
                                                );
                                                d2 = Some(
                                                    (Float::with_val(PRECISION_BITS, g2)
                                                        * (Float::with_val(
                                                            PRECISION_BITS,
                                                            ((0.5f64 * std::f64::consts::PI)
                                                                - beta),
                                                        ))
                                                        .cos())
                                                    .to_f64(),
                                                );
                                                d3 = Some(
                                                    1.0f64
                                                        + (g3
                                                            * ((0.5f64 * std::f64::consts::PI)
                                                                - beta)
                                                                .cos()),
                                                );
                                                md = (((*read(&d1, "d1")? * *read(&d1, "d1")?)
                                                    + (*read(&d2, "d2")? * *read(&d2, "d2")?))
                                                    + (*read(&d3, "d3")? * *read(&d3, "d3")?))
                                                .sqrt();
                                                d1 = Some(*read(&d1, "d1")? / md);
                                                d2 = Some(*read(&d2, "d2")? / md);
                                                d3 = Some(*read(&d3, "d3")? / md);
                                            }
                                        } else {
                                            if (beta < (0i32) as f64) {
                                                {
                                                    d1 = Some(
                                                        (Float::with_val(PRECISION_BITS, g1)
                                                            * (Float::with_val(
                                                                PRECISION_BITS,
                                                                (((-0.5f64)
                                                                    * std::f64::consts::PI)
                                                                    - beta),
                                                            ))
                                                            .cos())
                                                        .to_f64(),
                                                    );
                                                    d2 = Some(
                                                        (Float::with_val(PRECISION_BITS, g2)
                                                            * (Float::with_val(
                                                                PRECISION_BITS,
                                                                (((-0.5f64)
                                                                    * std::f64::consts::PI)
                                                                    - beta),
                                                            ))
                                                            .cos())
                                                        .to_f64(),
                                                    );
                                                    d3 = Some(
                                                        (-1.0f64)
                                                            + (g3
                                                                * (((-0.5f64)
                                                                    * std::f64::consts::PI)
                                                                    - beta)
                                                                    .cos()),
                                                    );
                                                    md = (((*read(&d1, "d1")?
                                                        * *read(&d1, "d1")?)
                                                        + (*read(&d2, "d2")? * *read(&d2, "d2")?))
                                                        + (*read(&d3, "d3")? * *read(&d3, "d3")?))
                                                    .sqrt();
                                                    d1 = Some(*read(&d1, "d1")? / md);
                                                    d2 = Some(*read(&d2, "d2")? / md);
                                                    d3 = Some(*read(&d3, "d3")? / md);
                                                }
                                            }
                                        }
                                        a1 = ((*read(&d2, "d2")? * g3) - (*read(&d3, "d3")? * g2));
                                        a2 = ((*read(&d3, "d3")? * g1) - (*read(&d1, "d1")? * g3));
                                        a3 = ((*read(&d1, "d1")? * g2) - (*read(&d2, "d2")? * g1));
                                        ma = (((a1 * a1) + (a2 * a2)) + (a3 * a3)).sqrt();
                                        a1 /= ma;
                                        a2 /= ma;
                                        a3 /= ma;
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
                                                g1 *= cosTheta;
                                                g2 *= cosTheta;
                                                g3 *= cosTheta;
                                                l1 = (n1 - g1);
                                                l2 = (n2 - g2);
                                                l3 = (n3 - g3);
                                                cosPhi = ((((l1 * m1) + (l2 * m2)) + (l3 * m3))
                                                    / (((l1 * l1) + (l2 * l2)) + (l3 * l3)).sqrt());
                                                cos2Phi =
                                                    ((((2i32) as f64 * cosPhi) * cosPhi) - 1.0f64);
                                            }
                                        } else {
                                            {
                                                cos2Phi = (0i32) as f64;
                                            }
                                        }
                                        dt = (((read(
                                            &rows[(i) as usize].sat,
                                            "psr[p].obsn[i].sat",
                                        )?)
                                        .clone()
                                            - Float::with_val(
                                                PRECISION_BITS,
                                                *read(&psr.gwm_epoch, "psr[p].gwm_epoch")?,
                                            ))
                                            * Float::with_val(PRECISION_BITS, 86400.0f64));
                                        scale = Float::with_val(
                                            PRECISION_BITS,
                                            (((-0.5f64) * cos2Phi) * ((1i32) as f64 - cosTheta)),
                                        );
                                        d.stdout(
                                            &[
                                                read_string(&psr.name[..], "string")?,
                                                b" scale = ".to_vec(),
                                                crate::clock_text::format_number(
                                                    ((scale).clone()).to_f64(),
                                                    crate::clock_text::NumberFormat::GeneralSix,
                                                ),
                                                b" ".to_vec(),
                                                crate::clock_text::format_number(
                                                    cos2Phi,
                                                    crate::clock_text::NumberFormat::GeneralSix,
                                                ),
                                                b" ".to_vec(),
                                                crate::clock_text::format_number(
                                                    cosTheta,
                                                    crate::clock_text::NumberFormat::GeneralSix,
                                                ),
                                                b"\n".to_vec(),
                                            ]
                                            .concat(),
                                        );
                                        phaseW = ((phaseW).clone()
                                            + ((((scale).clone()
                                                * ((psr.param.slots[param_gwm_amp as usize]
                                                    .cells[0]
                                                    .val)
                                                    .clone()
                                                    * (psr.param.slots[param_f as usize]
                                                        .cells[0]
                                                        .val)
                                                        .clone()))
                                                * (dt).clone())
                                                + Float::with_val(
                                                    PRECISION_BITS,
                                                    *read(&psr.gwm_dphase, "psr[p].gwm_dphase")?,
                                                )));
                                    }
                                }
                            }
                        }
                        if ((psr.param.slots[param_gwm_amp as usize].cells[0].param_set == 1i32)
                            && (psr.param.slots[param_gwm_amp as usize].cells[1].param_set == 1i32))
                        {
                            {
                                if ((read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?)
                                    .clone()
                                    >= Float::with_val(
                                        PRECISION_BITS,
                                        *read(&psr.gwm_epoch, "psr[p].gwm_epoch")?,
                                    ))
                                {
                                    {
                                        let mut dt: Float;
                                        let mut resp: Float;
                                        let mut resc: Float;
                                        let mut lambda_p: f64;
                                        let mut beta_p: f64;
                                        let mut lambda: f64;
                                        let mut beta: f64;
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
                                        lambda_p = ((psr.param.slots[param_raj as usize].cells[0]
                                            .val)
                                            .clone())
                                        .to_f64();
                                        beta_p = ((psr.param.slots[param_decj as usize].cells[0]
                                            .val)
                                            .clone())
                                        .to_f64();
                                        lambda = *read(&psr.gwm_raj, "psr[p].gwm_raj")?;
                                        beta = *read(&psr.gwm_decj, "psr[p].gwm_decj")?;
                                        n1 = ((Float::with_val(PRECISION_BITS, lambda_p)).cos()
                                            * (Float::with_val(PRECISION_BITS, beta_p)).cos())
                                        .to_f64();
                                        n2 = ((Float::with_val(PRECISION_BITS, lambda_p)).sin()
                                            * (Float::with_val(PRECISION_BITS, beta_p)).cos())
                                        .to_f64();
                                        n3 = ((Float::with_val(PRECISION_BITS, beta_p)).sin())
                                            .to_f64();
                                        cosTheta = ((((Float::with_val(PRECISION_BITS, beta))
                                            .cos()
                                            * (Float::with_val(PRECISION_BITS, beta_p)).cos())
                                            * (Float::with_val(
                                                PRECISION_BITS,
                                                (lambda - lambda_p),
                                            ))
                                            .cos())
                                            + ((Float::with_val(PRECISION_BITS, beta)).sin()
                                                * (Float::with_val(PRECISION_BITS, beta_p)).sin()))
                                        .to_f64();
                                        e11p = (((Float::with_val(PRECISION_BITS, lambda)).sin())
                                            .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
                                            - (((Float::with_val(PRECISION_BITS, lambda)).cos())
                                                .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
                                                * ((Float::with_val(PRECISION_BITS, beta)).sin())
                                                    .pow(
                                                        &(Float::with_val(PRECISION_BITS, 2i32)),
                                                    )))
                                        .to_f64();
                                        e21p = (((-(Float::with_val(PRECISION_BITS, lambda))
                                            .sin())
                                            * (Float::with_val(PRECISION_BITS, lambda)).cos())
                                            * (((Float::with_val(PRECISION_BITS, beta)).sin())
                                                .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
                                                + Float::with_val(PRECISION_BITS, 1i32)))
                                        .to_f64();
                                        e31p = (((Float::with_val(PRECISION_BITS, lambda)).cos()
                                            * (Float::with_val(PRECISION_BITS, beta)).sin())
                                            * (Float::with_val(PRECISION_BITS, beta)).cos())
                                        .to_f64();
                                        e12p = (((-(Float::with_val(PRECISION_BITS, lambda))
                                            .sin())
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
                                                    .pow(
                                                        &(Float::with_val(PRECISION_BITS, 2i32)),
                                                    )))
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
                                                + (n2
                                                    * (((n1 * e21p) + (n2 * e22p))
                                                        + (n3 * e23p))))
                                                + (n3
                                                    * (((n1 * e31p) + (n2 * e32p)) + (n3 * e33p)))),
                                        );
                                        if (((1i32) as f64 - cosTheta) == 0.0f64) {
                                            resp = Float::with_val(PRECISION_BITS, 0.0f64);
                                        } else {
                                            resp = ((Float::with_val(
                                                PRECISION_BITS,
                                                Float::parse("1.0").unwrap(),
                                            ) / (Float::with_val(
                                                PRECISION_BITS,
                                                Float::parse("2.0").unwrap(),
                                            ) * (Float::with_val(
                                                PRECISION_BITS,
                                                Float::parse("1.0").unwrap(),
                                            ) - Float::with_val(
                                                PRECISION_BITS,
                                                cosTheta,
                                            )))) * (resp).clone());
                                        }
                                        psr.quad_ifunc_geom_p = Some(((resp).clone()).to_f64());
                                        if d.debug {
                                            d.log(
                                                b'd',
                                                "formResiduals.C",
                                                1400,
                                                &[
                                                    b"Resp2 = ".to_vec(),
                                                    read_string(&psr.name[..], "string")?,
                                                    b" ".to_vec(),
                                                    crate::clock_text::format_number(
                                                        ((resp).clone()).to_f64(),
                                                        crate::clock_text::NumberFormat::GeneralSix,
                                                    ),
                                                    b" ".to_vec(),
                                                    crate::clock_text::format_number(
                                                        *read(
                                                            &psr.quad_ifunc_geom_p,
                                                            "psr[p].quad_ifunc_geom_p",
                                                        )?,
                                                        crate::clock_text::NumberFormat::GeneralSix,
                                                    ),
                                                ]
                                                .concat(),
                                            );
                                        }
                                        lambda = *read(&psr.gwm_raj, "psr[p].gwm_raj")?;
                                        beta = *read(&psr.gwm_decj, "psr[p].gwm_decj")?;
                                        e11c = ((Float::with_val(
                                            PRECISION_BITS,
                                            ((2i32) as f64 * lambda),
                                        ))
                                        .sin()
                                            * (Float::with_val(PRECISION_BITS, beta)).sin())
                                        .to_f64();
                                        e21c = ((-(Float::with_val(
                                            PRECISION_BITS,
                                            ((2i32) as f64 * lambda),
                                        ))
                                        .cos())
                                            * (Float::with_val(PRECISION_BITS, beta)).sin())
                                        .to_f64();
                                        e31c = ((-(Float::with_val(PRECISION_BITS, lambda)).sin())
                                            * (Float::with_val(PRECISION_BITS, beta)).cos())
                                        .to_f64();
                                        e12c = ((-(Float::with_val(
                                            PRECISION_BITS,
                                            ((2i32) as f64 * lambda),
                                        ))
                                        .cos())
                                            * (Float::with_val(PRECISION_BITS, beta)).sin())
                                        .to_f64();
                                        e22c = ((-(Float::with_val(
                                            PRECISION_BITS,
                                            ((2i32) as f64 * lambda),
                                        ))
                                        .sin())
                                            * (Float::with_val(PRECISION_BITS, beta)).sin())
                                        .to_f64();
                                        e32c = ((Float::with_val(PRECISION_BITS, lambda)).cos()
                                            * (Float::with_val(PRECISION_BITS, beta)).cos())
                                        .to_f64();
                                        e13c = ((-(Float::with_val(PRECISION_BITS, lambda)).sin())
                                            * (Float::with_val(PRECISION_BITS, beta)).cos())
                                        .to_f64();
                                        e23c = ((Float::with_val(PRECISION_BITS, lambda)).cos()
                                            * (Float::with_val(PRECISION_BITS, beta)).cos())
                                        .to_f64();
                                        e33c = (0i32) as f64;
                                        resc = Float::with_val(
                                            PRECISION_BITS,
                                            (((n1 * (((n1 * e11c) + (n2 * e12c)) + (n3 * e13c)))
                                                + (n2
                                                    * (((n1 * e21c) + (n2 * e22c))
                                                        + (n3 * e23c))))
                                                + (n3
                                                    * (((n1 * e31c) + (n2 * e32c)) + (n3 * e33c)))),
                                        );
                                        if (((1i32) as f64 - cosTheta) == 0.0f64) {
                                            resc = Float::with_val(PRECISION_BITS, 0.0f64);
                                        } else {
                                            resc = ((Float::with_val(
                                                PRECISION_BITS,
                                                Float::parse("1.0").unwrap(),
                                            ) / (Float::with_val(
                                                PRECISION_BITS,
                                                Float::parse("2.0").unwrap(),
                                            ) * (Float::with_val(
                                                PRECISION_BITS,
                                                Float::parse("1.0").unwrap(),
                                            ) - Float::with_val(
                                                PRECISION_BITS,
                                                cosTheta,
                                            )))) * (resc).clone());
                                        }
                                        psr.quad_ifunc_geom_c = Some(((resc).clone()).to_f64());
                                        dt = (((read(
                                            &rows[(i) as usize].bbat,
                                            "psr[p].obsn[i].bbat",
                                        )?)
                                        .clone()
                                            - Float::with_val(
                                                PRECISION_BITS,
                                                *read(&psr.gwm_epoch, "psr[p].gwm_epoch")?,
                                            ))
                                            * Float::with_val(
                                                PRECISION_BITS,
                                                Float::parse("86400.0").unwrap(),
                                            ));
                                        phaseW = ((phaseW).clone()
                                            + ((((psr.param.slots[param_f as usize].cells[0]
                                                .val)
                                                .clone()
                                                * (dt).clone())
                                                * (psr.param.slots[param_gwm_amp as usize]
                                                    .cells[0]
                                                    .val)
                                                    .clone())
                                                * Float::with_val(
                                                    PRECISION_BITS,
                                                    *read(
                                                        &psr.quad_ifunc_geom_p,
                                                        "psr[p].quad_ifunc_geom_p",
                                                    )?,
                                                )));
                                        phaseW = ((phaseW).clone()
                                            + ((((psr.param.slots[param_f as usize].cells[0]
                                                .val)
                                                .clone()
                                                * (dt).clone())
                                                * (psr.param.slots[param_gwm_amp as usize]
                                                    .cells[1]
                                                    .val)
                                                    .clone())
                                                * Float::with_val(
                                                    PRECISION_BITS,
                                                    *read(
                                                        &psr.quad_ifunc_geom_c,
                                                        "psr[p].quad_ifunc_geom_c",
                                                    )?,
                                                )));
                                    }
                                }
                            }
                        }
                        if (psr.param.slots[param_gwcs_amp as usize].cells[0].param_set == 1i32) {
                            {
                                let mut lambda_p: f64;
                                let mut beta_p: f64;
                                let mut lambda: f64;
                                let mut beta: f64;
                                let mut dt: f64;
                                let mut n1: f64;
                                let mut n2: f64;
                                let mut n3: f64;
                                let mut cosTheta: f64;
                                let mut width: f64;
                                let mut width_day: f64;
                                let mut extra1: f64;
                                let mut extra2: f64;
                                let mut resp: Float;
                                let mut resc: Float;
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
                                lambda_p = ((psr.param.slots[param_raj as usize].cells[0].val)
                                    .clone())
                                .to_f64();
                                beta_p = ((psr.param.slots[param_decj as usize].cells[0].val)
                                    .clone())
                                .to_f64();
                                lambda = *read(&psr.gwcs_raj, "psr[p].gwcs_raj")?;
                                beta = *read(&psr.gwcs_decj, "psr[p].gwcs_decj")?;
                                n1 = ((Float::with_val(PRECISION_BITS, lambda_p)).cos()
                                    * (Float::with_val(PRECISION_BITS, beta_p)).cos())
                                .to_f64();
                                n2 = ((Float::with_val(PRECISION_BITS, lambda_p)).sin()
                                    * (Float::with_val(PRECISION_BITS, beta_p)).cos())
                                .to_f64();
                                n3 = ((Float::with_val(PRECISION_BITS, beta_p)).sin()).to_f64();
                                cosTheta = ((((Float::with_val(PRECISION_BITS, beta)).cos()
                                    * (Float::with_val(PRECISION_BITS, beta_p)).cos())
                                    * (Float::with_val(PRECISION_BITS, (lambda - lambda_p)))
                                        .cos())
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
                                if (((1i32) as f64 - cosTheta) == 0.0f64) {
                                    resp = Float::with_val(PRECISION_BITS, 0.0f64);
                                } else {
                                    resp = ((Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1.0").unwrap(),
                                    ) / (Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("2.0").unwrap(),
                                    ) * (Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1.0").unwrap(),
                                    ) - Float::with_val(
                                        PRECISION_BITS,
                                        cosTheta,
                                    )))) * (resp).clone());
                                }
                                psr.gwcs_geom_p = Some(((resp).clone()).to_f64());
                                if d.debug {
                                    d.log(
                                        b'd',
                                        "formResiduals.C",
                                        1493,
                                        &[
                                            b"Resp2 = ".to_vec(),
                                            read_string(&psr.name[..], "string")?,
                                            b" ".to_vec(),
                                            crate::clock_text::format_number(
                                                ((resp).clone()).to_f64(),
                                                crate::clock_text::NumberFormat::GeneralSix,
                                            ),
                                            b" ".to_vec(),
                                            crate::clock_text::format_number(
                                                *read(
                                                    &psr.quad_ifunc_geom_p,
                                                    "psr[p].quad_ifunc_geom_p",
                                                )?,
                                                crate::clock_text::NumberFormat::GeneralSix,
                                            ),
                                        ]
                                        .concat(),
                                    );
                                }
                                lambda = *read(&psr.gwcs_raj, "psr[p].gwcs_raj")?;
                                beta = *read(&psr.gwcs_decj, "psr[p].gwcs_decj")?;
                                e11c =
                                    ((Float::with_val(PRECISION_BITS, ((2i32) as f64 * lambda)))
                                        .sin()
                                        * (Float::with_val(PRECISION_BITS, beta)).sin())
                                    .to_f64();
                                e21c = ((-(Float::with_val(
                                    PRECISION_BITS,
                                    ((2i32) as f64 * lambda),
                                ))
                                .cos())
                                    * (Float::with_val(PRECISION_BITS, beta)).sin())
                                .to_f64();
                                e31c = ((-(Float::with_val(PRECISION_BITS, lambda)).sin())
                                    * (Float::with_val(PRECISION_BITS, beta)).cos())
                                .to_f64();
                                e12c = ((-(Float::with_val(
                                    PRECISION_BITS,
                                    ((2i32) as f64 * lambda),
                                ))
                                .cos())
                                    * (Float::with_val(PRECISION_BITS, beta)).sin())
                                .to_f64();
                                e22c = ((-(Float::with_val(
                                    PRECISION_BITS,
                                    ((2i32) as f64 * lambda),
                                ))
                                .sin())
                                    * (Float::with_val(PRECISION_BITS, beta)).sin())
                                .to_f64();
                                e32c = ((Float::with_val(PRECISION_BITS, lambda)).cos()
                                    * (Float::with_val(PRECISION_BITS, beta)).cos())
                                .to_f64();
                                e13c = ((-(Float::with_val(PRECISION_BITS, lambda)).sin())
                                    * (Float::with_val(PRECISION_BITS, beta)).cos())
                                .to_f64();
                                e23c = ((Float::with_val(PRECISION_BITS, lambda)).cos()
                                    * (Float::with_val(PRECISION_BITS, beta)).cos())
                                .to_f64();
                                e33c = (0i32) as f64;
                                resc = Float::with_val(
                                    PRECISION_BITS,
                                    (((n1 * (((n1 * e11c) + (n2 * e12c)) + (n3 * e13c)))
                                        + (n2 * (((n1 * e21c) + (n2 * e22c)) + (n3 * e23c))))
                                        + (n3 * (((n1 * e31c) + (n2 * e32c)) + (n3 * e33c)))),
                                );
                                if (((1i32) as f64 - cosTheta) == 0.0f64) {
                                    resc = Float::with_val(PRECISION_BITS, 0.0f64);
                                } else {
                                    resc = ((Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1.0").unwrap(),
                                    ) / (Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("2.0").unwrap(),
                                    ) * (Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1.0").unwrap(),
                                    ) - Float::with_val(
                                        PRECISION_BITS,
                                        cosTheta,
                                    )))) * (resc).clone());
                                }
                                psr.gwcs_geom_c = Some(((resc).clone()).to_f64());
                                dt = (((read(&rows[(i) as usize].sat, "psr[p].obsn[i].sat")?)
                                    .clone()
                                    - Float::with_val(
                                        PRECISION_BITS,
                                        *read(&psr.gwcs_epoch, "psr[p].gwcs_epoch")?,
                                    ))
                                    * Float::with_val(PRECISION_BITS, 86400.0f64))
                                .to_f64();
                                width = (*read(&psr.gwcs_width, "psr[p].gwcs_width")? * 86400.0f64);
                                width_day = *read(&psr.gwcs_width, "psr[p].gwcs_width")?;
                                if ((read(&rows[(i) as usize].sat, "psr[p].obsn[i].sat")?).clone()
                                    < Float::with_val(
                                        PRECISION_BITS,
                                        (*read(&psr.gwcs_epoch, "psr[p].gwcs_epoch")?
                                            - (width_day / 2.0f64)),
                                    ))
                                {
                                    {
                                        extra1 = (0i32) as f64;
                                        extra2 = (0i32) as f64;
                                    }
                                } else {
                                    if ((read(&rows[(i) as usize].sat, "psr[p].obsn[i].sat")?)
                                        .clone()
                                        <= Float::with_val(
                                            PRECISION_BITS,
                                            *read(&psr.gwcs_epoch, "psr[p].gwcs_epoch")?,
                                        ))
                                    {
                                        {
                                            extra1 = ((psr.param.slots[param_gwcs_amp as usize]
                                                .cells[0]
                                                .val)
                                                .clone()
                                                * Float::with_val(
                                                    PRECISION_BITS,
                                                    (((3.0f64 / 4.0f64)
                                                        * ((0.5f64 * width)
                                                            .powf(4.0f64 / 3.0f64)
                                                            - ((dt).abs()).powf(4.0f64 / 3.0f64)))
                                                        - ((0.5f64 * width).powf(1.0f64 / 3.0f64)
                                                            * (dt + (0.5f64 * width)))),
                                                ))
                                            .to_f64();
                                            extra2 = ((psr.param.slots[param_gwcs_amp as usize]
                                                .cells[1]
                                                .val)
                                                .clone()
                                                * Float::with_val(
                                                    PRECISION_BITS,
                                                    (((3.0f64 / 4.0f64)
                                                        * ((0.5f64 * width)
                                                            .powf(4.0f64 / 3.0f64)
                                                            - ((dt).abs()).powf(4.0f64 / 3.0f64)))
                                                        - ((0.5f64 * width).powf(1.0f64 / 3.0f64)
                                                            * (dt + (0.5f64 * width)))),
                                                ))
                                            .to_f64();
                                        }
                                    } else {
                                        if ((read(&rows[(i) as usize].sat, "psr[p].obsn[i].sat")?)
                                            .clone()
                                            <= Float::with_val(
                                                PRECISION_BITS,
                                                (*read(&psr.gwcs_epoch, "psr[p].gwcs_epoch")?
                                                    + (width_day / 2.0f64)),
                                            ))
                                        {
                                            {
                                                extra1 = ((psr.param.slots
                                                    [param_gwcs_amp as usize]
                                                    .cells[0]
                                                    .val)
                                                    .clone()
                                                    * Float::with_val(
                                                        PRECISION_BITS,
                                                        (((3.0f64 / 4.0f64)
                                                            * ((0.5f64 * width)
                                                                .powf(4.0f64 / 3.0f64)
                                                                + ((dt).abs())
                                                                    .powf(4.0f64 / 3.0f64)))
                                                            - ((0.5f64 * width)
                                                                .powf(1.0f64 / 3.0f64)
                                                                * (dt + (0.5f64 * width)))),
                                                    ))
                                                .to_f64();
                                                extra2 = ((psr.param.slots
                                                    [param_gwcs_amp as usize]
                                                    .cells[1]
                                                    .val)
                                                    .clone()
                                                    * Float::with_val(
                                                        PRECISION_BITS,
                                                        (((3.0f64 / 4.0f64)
                                                            * ((0.5f64 * width)
                                                                .powf(4.0f64 / 3.0f64)
                                                                + ((dt).abs())
                                                                    .powf(4.0f64 / 3.0f64)))
                                                            - ((0.5f64 * width)
                                                                .powf(1.0f64 / 3.0f64)
                                                                * (dt + (0.5f64 * width)))),
                                                    ))
                                                .to_f64();
                                            }
                                        } else {
                                            {
                                                extra1 =
                                                    (Float::with_val(PRECISION_BITS, (-0.25f64))
                                                        * ((Float::with_val(
                                                            PRECISION_BITS,
                                                            (0.5f64).powf(1.0f64 / 3.0f64),
                                                        ) * (psr.param.slots
                                                            [param_gwcs_amp as usize]
                                                            .cells[0]
                                                            .val)
                                                            .clone())
                                                            * Float::with_val(
                                                                PRECISION_BITS,
                                                                (width).powf(4.0f64 / 3.0f64),
                                                            )))
                                                    .to_f64();
                                                extra2 =
                                                    (Float::with_val(PRECISION_BITS, (-0.25f64))
                                                        * ((Float::with_val(
                                                            PRECISION_BITS,
                                                            (0.5f64).powf(1.0f64 / 3.0f64),
                                                        ) * (psr.param.slots
                                                            [param_gwcs_amp as usize]
                                                            .cells[1]
                                                            .val)
                                                            .clone())
                                                            * Float::with_val(
                                                                PRECISION_BITS,
                                                                (width).powf(4.0f64 / 3.0f64),
                                                            )))
                                                    .to_f64();
                                            }
                                        }
                                    }
                                }
                                phaseW = ((phaseW).clone()
                                    + ((psr.param.slots[param_f as usize].cells[0].val).clone()
                                        * Float::with_val(
                                            PRECISION_BITS,
                                            ((extra1
                                                * *read(&psr.gwcs_geom_p, "psr[p].gwcs_geom_p")?)
                                                + (extra2
                                                    * *read(
                                                        &psr.gwcs_geom_c,
                                                        "psr[p].gwcs_geom_c",
                                                    )?)),
                                        )));
                            }
                        }
                        if (psr.param.slots[param_clk_offs as usize].cells[0].param_set == 1i32) {
                            {
                                let mut j: i32;
                                d.stdout(
                                    &[
                                        b"IN HERE SETTING THE CLOCKS ".to_vec(),
                                        (*read(&psr.clkOffsN, "psr[p].clkOffsN")?)
                                            .to_string()
                                            .into_bytes(),
                                        b"\n".to_vec(),
                                    ]
                                    .concat(),
                                );
                                {
                                    j = 0i32;
                                    while (j < (*read(&psr.clkOffsN, "psr[p].clkOffsN")? - 1i32)) {
                                        {
                                            if (((read(
                                                &rows[(i) as usize].bbat,
                                                "psr[p].obsn[i].bbat",
                                            )?)
                                            .clone()
                                                >= Float::with_val(
                                                    PRECISION_BITS,
                                                    *read(
                                                        &psr.clk_offsT[(j) as usize],
                                                        "psr[p].clk_offsT[j]",
                                                    )?,
                                                ))
                                                && ((read(
                                                    &rows[(i) as usize].bbat,
                                                    "psr[p].obsn[i].bbat",
                                                )?)
                                                .clone()
                                                    < Float::with_val(
                                                        PRECISION_BITS,
                                                        *read(
                                                            &psr.clk_offsT[(j + 1i32) as usize],
                                                            "psr[p].clk_offsT[j + 1]",
                                                        )?,
                                                    )))
                                            {
                                                {
                                                    phaseW = ((phaseW).clone()
                                                        + (Float::with_val(
                                                            PRECISION_BITS,
                                                            *read(
                                                                &psr.clk_offsV[(j) as usize],
                                                                "psr[p].clk_offsV[j]",
                                                            )?,
                                                        ) * (psr.param.slots
                                                            [param_f as usize]
                                                            .cells[0]
                                                            .val)
                                                            .clone()));
                                                    break;
                                                }
                                            }
                                        }
                                        j += 1;
                                    }
                                }
                            }
                        }
                        if (psr.param.slots[param_ifunc as usize].cells[0].param_set == 1i32) {
                            {
                                let mut wi: Float;
                                let mut t1: Float;
                                let mut dt: Float;
                                let mut speriod: Float;
                                let mut tt: Float;
                                if ((psr.param.slots[param_ifunc as usize].cells[0].val).clone()
                                    == Float::with_val(PRECISION_BITS, 1i32))
                                {
                                    {
                                        t1 = Float::with_val(
                                            PRECISION_BITS,
                                            Float::parse("0.0").unwrap(),
                                        );
                                        speriod = Float::with_val(
                                            PRECISION_BITS,
                                            (*read(&psr.ifuncT[1], "psr[p].ifuncT[1]")?
                                                - *read(&psr.ifuncT[0], "psr[p].ifuncT[0]")?),
                                        );
                                        {
                                            k = 0i32;
                                            while (k < *read(&psr.ifuncN, "psr[p].ifuncN")?) {
                                                {
                                                    dt = ((read(
                                                        &rows[(i) as usize].bbat,
                                                        "psr[p].obsn[i].bbat",
                                                    )?)
                                                    .clone()
                                                        - Float::with_val(
                                                            PRECISION_BITS,
                                                            *read(
                                                                &psr.ifuncT[(k) as usize],
                                                                "psr[p].ifuncT[k]",
                                                            )?,
                                                        ));
                                                    wi = Float::with_val(PRECISION_BITS, 1i32);
                                                    if ((dt).clone()
                                                        == Float::with_val(PRECISION_BITS, 0i32))
                                                    {
                                                        {
                                                            t1 = ((t1).clone()
                                                                + ((wi).clone()
                                                                    * Float::with_val(
                                                                        PRECISION_BITS,
                                                                        *read(
                                                                            &psr.ifuncV
                                                                                [(k) as usize],
                                                                            "psr[p].ifuncV[k]",
                                                                        )?,
                                                                    )));
                                                        }
                                                    } else {
                                                        {
                                                            tt = ((Float::with_val(
                                                                PRECISION_BITS,
                                                                std::f64::consts::PI,
                                                            ) / (speriod).clone())
                                                                * (dt).clone());
                                                            t1 = ((t1).clone()
                                                                + ((((wi).clone()
                                                                    * Float::with_val(
                                                                        PRECISION_BITS,
                                                                        *read(
                                                                            &psr.ifuncV
                                                                                [(k) as usize],
                                                                            "psr[p].ifuncV[k]",
                                                                        )?,
                                                                    ))
                                                                    * ((tt).clone()).sin())
                                                                    / (tt).clone()));
                                                        }
                                                    }
                                                }
                                                k += 1;
                                            }
                                        }
                                        phaseW = ((phaseW).clone()
                                            + ((psr.param.slots[param_f as usize].cells[0].val)
                                                .clone()
                                                * (t1).clone()));
                                    }
                                } else {
                                    if ((psr.param.slots[param_ifunc as usize].cells[0].val)
                                        .clone()
                                        == Float::with_val(PRECISION_BITS, 2i32))
                                    {
                                        {
                                            let mut ival: Option<f64> =
                                                Some(crate::ifunc::interpolate(
                                                    &psr.ifuncT
                                                        .iter()
                                                        .take(
                                                            (*read(&psr.ifuncN, "psr[p].ifuncN")?)
                                                                as usize,
                                                        )
                                                        .map(|v| read(v, "ifuncT").copied())
                                                        .collect::<Result<Vec<_>, _>>()?,
                                                    &psr.ifuncV
                                                        .iter()
                                                        .take(
                                                            (*read(&psr.ifuncN, "psr[p].ifuncN")?)
                                                                as usize,
                                                        )
                                                        .map(|v| read(v, "ifuncV").copied())
                                                        .collect::<Result<Vec<_>, _>>()?,
                                                    ((read(
                                                        &rows[(i) as usize].sat,
                                                        "psr[p].obsn[i].sat",
                                                    )?)
                                                    .clone())
                                                    .to_f64(),
                                                    *read(&psr.ifuncN, "psr[p].ifuncN")?,
                                                    d,
                                                )?);
                                            phaseW = ((phaseW).clone()
                                                + ((psr.param.slots[param_f as usize].cells[0]
                                                    .val)
                                                    .clone()
                                                    * Float::with_val(
                                                        PRECISION_BITS,
                                                        *read(&ival, "ival")?,
                                                    )));
                                        }
                                    } else {
                                        if ((psr.param.slots[param_ifunc as usize].cells[0].val)
                                            .clone()
                                            == Float::with_val(PRECISION_BITS, 0i32))
                                        {
                                            {
                                                let mut k: i32;
                                                let mut ival: Option<f64> = None;
                                                {
                                                    k = 0i32;
                                                    while (k
                                                        < (*read(&psr.ifuncN, "psr[p].ifuncN")?
                                                            - 1i32))
                                                    {
                                                        {
                                                            if (((read(
                                                                &rows[(i) as usize].sat,
                                                                "psr[p].obsn[i].sat",
                                                            )?)
                                                            .clone())
                                                            .to_f64()
                                                                >= *read(
                                                                    &psr.ifuncT[(k) as usize],
                                                                    "psr[p].ifuncT[k]",
                                                                )?)
                                                            {
                                                                {
                                                                    ival = Some(*read(
                                                                        &psr.ifuncV[(k) as usize],
                                                                        "psr[p].ifuncV[k]",
                                                                    )?);
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                        k += 1;
                                                    }
                                                }
                                                phaseW = ((phaseW).clone()
                                                    + ((psr.param.slots[param_f as usize].cells
                                                        [0]
                                                    .val)
                                                        .clone()
                                                        * Float::with_val(
                                                            PRECISION_BITS,
                                                            *read(&ival, "ival")?,
                                                        )));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if (psr.param.slots[param_quad_ifunc_p as usize].cells[0].param_set == 1i32)
                        {
                            {
                                let mut resp: Float;
                                let mut lambda_p: f64;
                                let mut beta_p: f64;
                                let mut lambda: f64;
                                let mut beta: f64;
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
                                let mut cosTheta: f64;
                                if ((*read(&psr.quad_ifunc_geom_p, "psr[p].quad_ifunc_geom_p")?
                                    == (0i32) as f64)
                                    && ((((psr.param.slots[param_quad_ifunc_p as usize].cells
                                        [0]
                                    .val)
                                        .clone()
                                        == Float::with_val(PRECISION_BITS, 0i32))
                                        || ((psr.param.slots[param_quad_ifunc_p as usize].cells
                                            [0]
                                        .val)
                                            .clone()
                                            == Float::with_val(PRECISION_BITS, 1i32)))
                                        || ((psr.param.slots[param_quad_ifunc_p as usize].cells
                                            [0]
                                        .val)
                                            .clone()
                                            == Float::with_val(PRECISION_BITS, 3i32))))
                                {
                                    {
                                        if ((psr.param.slots[param_quad_ifunc_p as usize].cells[0]
                                            .val)
                                            .clone()
                                            == Float::with_val(PRECISION_BITS, 3i32))
                                        {
                                            psr.quad_ifunc_geom_p = Some((1i32) as f64);
                                        } else {
                                            {
                                                lambda_p = ((psr.param.slots[param_raj as usize]
                                                    .cells[0]
                                                    .val)
                                                    .clone())
                                                .to_f64();
                                                beta_p = ((psr.param.slots[param_decj as usize]
                                                    .cells[0]
                                                    .val)
                                                    .clone())
                                                .to_f64();
                                                lambda = *read(
                                                    &psr.quad_ifunc_p_RA,
                                                    "psr[p].quad_ifunc_p_RA",
                                                )?;
                                                beta = *read(
                                                    &psr.quad_ifunc_p_DEC,
                                                    "psr[p].quad_ifunc_p_DEC",
                                                )?;
                                                n1 = ((Float::with_val(PRECISION_BITS, lambda_p))
                                                    .cos()
                                                    * (Float::with_val(PRECISION_BITS, beta_p))
                                                        .cos())
                                                .to_f64();
                                                n2 = ((Float::with_val(PRECISION_BITS, lambda_p))
                                                    .sin()
                                                    * (Float::with_val(PRECISION_BITS, beta_p))
                                                        .cos())
                                                .to_f64();
                                                n3 = ((Float::with_val(PRECISION_BITS, beta_p))
                                                    .sin())
                                                .to_f64();
                                                cosTheta =
                                                    ((((Float::with_val(PRECISION_BITS, beta))
                                                        .cos()
                                                        * (Float::with_val(
                                                            PRECISION_BITS,
                                                            beta_p,
                                                        ))
                                                        .cos())
                                                        * (Float::with_val(
                                                            PRECISION_BITS,
                                                            (lambda - lambda_p),
                                                        ))
                                                        .cos())
                                                        + ((Float::with_val(
                                                            PRECISION_BITS,
                                                            beta,
                                                        ))
                                                        .sin()
                                                            * (Float::with_val(
                                                                PRECISION_BITS,
                                                                beta_p,
                                                            ))
                                                            .sin()))
                                                    .to_f64();
                                                e11p =
                                                    (((Float::with_val(PRECISION_BITS, lambda))
                                                        .sin())
                                                    .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
                                                        - (((Float::with_val(
                                                            PRECISION_BITS,
                                                            lambda,
                                                        ))
                                                        .cos())
                                                        .pow(
                                                            &(Float::with_val(
                                                                PRECISION_BITS,
                                                                2i32,
                                                            )),
                                                        ) * ((Float::with_val(
                                                            PRECISION_BITS,
                                                            beta,
                                                        ))
                                                        .sin())
                                                        .pow(
                                                            &(Float::with_val(
                                                                PRECISION_BITS,
                                                                2i32,
                                                            )),
                                                        )))
                                                    .to_f64();
                                                e21p = (((-(Float::with_val(
                                                    PRECISION_BITS,
                                                    lambda,
                                                ))
                                                .sin())
                                                    * (Float::with_val(PRECISION_BITS, lambda))
                                                        .cos())
                                                    * (((Float::with_val(PRECISION_BITS, beta))
                                                        .sin())
                                                    .pow(
                                                        &(Float::with_val(PRECISION_BITS, 2i32)),
                                                    ) + Float::with_val(PRECISION_BITS, 1i32)))
                                                .to_f64();
                                                e31p =
                                                    (((Float::with_val(PRECISION_BITS, lambda))
                                                        .cos()
                                                        * (Float::with_val(PRECISION_BITS, beta))
                                                            .sin())
                                                        * (Float::with_val(PRECISION_BITS, beta))
                                                            .cos())
                                                    .to_f64();
                                                e12p = (((-(Float::with_val(
                                                    PRECISION_BITS,
                                                    lambda,
                                                ))
                                                .sin())
                                                    * (Float::with_val(PRECISION_BITS, lambda))
                                                        .cos())
                                                    * (((Float::with_val(PRECISION_BITS, beta))
                                                        .sin())
                                                    .pow(
                                                        &(Float::with_val(PRECISION_BITS, 2i32)),
                                                    ) + Float::with_val(PRECISION_BITS, 1i32)))
                                                .to_f64();
                                                e22p =
                                                    (((Float::with_val(PRECISION_BITS, lambda))
                                                        .cos())
                                                    .pow(&(Float::with_val(PRECISION_BITS, 2i32)))
                                                        - (((Float::with_val(
                                                            PRECISION_BITS,
                                                            lambda,
                                                        ))
                                                        .sin())
                                                        .pow(
                                                            &(Float::with_val(
                                                                PRECISION_BITS,
                                                                2i32,
                                                            )),
                                                        ) * ((Float::with_val(
                                                            PRECISION_BITS,
                                                            beta,
                                                        ))
                                                        .sin())
                                                        .pow(
                                                            &(Float::with_val(
                                                                PRECISION_BITS,
                                                                2i32,
                                                            )),
                                                        )))
                                                    .to_f64();
                                                e32p =
                                                    (((Float::with_val(PRECISION_BITS, lambda))
                                                        .sin()
                                                        * (Float::with_val(PRECISION_BITS, beta))
                                                            .sin())
                                                        * (Float::with_val(PRECISION_BITS, beta))
                                                            .cos())
                                                    .to_f64();
                                                e13p =
                                                    (((Float::with_val(PRECISION_BITS, lambda))
                                                        .cos()
                                                        * (Float::with_val(PRECISION_BITS, beta))
                                                            .sin())
                                                        * (Float::with_val(PRECISION_BITS, beta))
                                                            .cos())
                                                    .to_f64();
                                                e23p =
                                                    (((Float::with_val(PRECISION_BITS, lambda))
                                                        .sin()
                                                        * (Float::with_val(PRECISION_BITS, beta))
                                                            .sin())
                                                        * (Float::with_val(PRECISION_BITS, beta))
                                                            .cos())
                                                    .to_f64();
                                                e33p = (-((Float::with_val(PRECISION_BITS, beta))
                                                    .cos())
                                                .pow(&(Float::with_val(PRECISION_BITS, 2i32))))
                                                .to_f64();
                                                resp = Float::with_val(
                                                    PRECISION_BITS,
                                                    (((n1
                                                        * (((n1 * e11p) + (n2 * e12p))
                                                            + (n3 * e13p)))
                                                        + (n2
                                                            * (((n1 * e21p) + (n2 * e22p))
                                                                + (n3 * e23p))))
                                                        + (n3
                                                            * (((n1 * e31p) + (n2 * e32p))
                                                                + (n3 * e33p)))),
                                                );
                                                if (((1i32) as f64 - cosTheta) == 0.0f64) {
                                                    resp = Float::with_val(PRECISION_BITS, 0.0f64);
                                                } else {
                                                    resp = ((Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("1.0").unwrap(),
                                                    ) / (Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("2.0").unwrap(),
                                                    ) * (Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("1.0").unwrap(),
                                                    ) - Float::with_val(
                                                        PRECISION_BITS,
                                                        cosTheta,
                                                    )))) * (resp).clone());
                                                }
                                                psr.quad_ifunc_geom_p =
                                                    Some(((resp).clone()).to_f64());
                                            }
                                        }
                                    }
                                }
                                let mut ival: Option<f64> = Some(crate::ifunc::interpolate(
                                    &psr.quad_ifuncT_p
                                        .iter()
                                        .take(
                                            (*read(&psr.quad_ifuncN_p, "psr[p].quad_ifuncN_p")?)
                                                as usize,
                                        )
                                        .map(|v| read(v, "quad_ifuncT_p").copied())
                                        .collect::<Result<Vec<_>, _>>()?,
                                    &psr.quad_ifuncV_p
                                        .iter()
                                        .take(
                                            (*read(&psr.quad_ifuncN_p, "psr[p].quad_ifuncN_p")?)
                                                as usize,
                                        )
                                        .map(|v| read(v, "quad_ifuncV_p").copied())
                                        .collect::<Result<Vec<_>, _>>()?,
                                    ((read(&rows[(i) as usize].sat, "psr[p].obsn[i].sat")?)
                                        .clone())
                                    .to_f64(),
                                    *read(&psr.quad_ifuncN_p, "psr[p].quad_ifuncN_p")?,
                                    d,
                                )?);
                                phaseW = ((phaseW).clone()
                                    + (((psr.param.slots[param_f as usize].cells[0].val).clone()
                                        * Float::with_val(PRECISION_BITS, *read(&ival, "ival")?))
                                        * Float::with_val(
                                            PRECISION_BITS,
                                            *read(
                                                &psr.quad_ifunc_geom_p,
                                                "psr[p].quad_ifunc_geom_p",
                                            )?,
                                        )));
                            }
                        }
                        if (psr.param.slots[param_quad_ifunc_c as usize].cells[0].param_set == 1i32)
                        {
                            {
                                let mut resc: Float;
                                let mut lambda_p: f64;
                                let mut beta_p: f64;
                                let mut lambda: f64;
                                let mut beta: f64;
                                let mut n1: f64;
                                let mut n2: f64;
                                let mut n3: f64;
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
                                if ((*read(&psr.quad_ifunc_geom_c, "psr[p].quad_ifunc_geom_c")?
                                    == (0i32) as f64)
                                    && ((((psr.param.slots[param_quad_ifunc_c as usize].cells
                                        [0]
                                    .val)
                                        .clone()
                                        == Float::with_val(PRECISION_BITS, 0i32))
                                        || ((psr.param.slots[param_quad_ifunc_c as usize].cells
                                            [0]
                                        .val)
                                            .clone()
                                            == Float::with_val(PRECISION_BITS, 1i32)))
                                        || ((psr.param.slots[param_quad_ifunc_c as usize].cells
                                            [0]
                                        .val)
                                            .clone()
                                            == Float::with_val(PRECISION_BITS, 3i32))))
                                {
                                    {
                                        if ((psr.param.slots[param_quad_ifunc_c as usize].cells[0]
                                            .val)
                                            .clone()
                                            == Float::with_val(PRECISION_BITS, 3i32))
                                        {
                                            psr.quad_ifunc_geom_c = Some((1i32) as f64);
                                        } else {
                                            {
                                                lambda_p = ((psr.param.slots[param_raj as usize]
                                                    .cells[0]
                                                    .val)
                                                    .clone())
                                                .to_f64();
                                                beta_p = ((psr.param.slots[param_decj as usize]
                                                    .cells[0]
                                                    .val)
                                                    .clone())
                                                .to_f64();
                                                lambda = *read(
                                                    &psr.quad_ifunc_c_RA,
                                                    "psr[p].quad_ifunc_c_RA",
                                                )?;
                                                beta = *read(
                                                    &psr.quad_ifunc_c_DEC,
                                                    "psr[p].quad_ifunc_c_DEC",
                                                )?;
                                                n1 = ((Float::with_val(PRECISION_BITS, lambda_p))
                                                    .cos()
                                                    * (Float::with_val(PRECISION_BITS, beta_p))
                                                        .cos())
                                                .to_f64();
                                                n2 = ((Float::with_val(PRECISION_BITS, lambda_p))
                                                    .sin()
                                                    * (Float::with_val(PRECISION_BITS, beta_p))
                                                        .cos())
                                                .to_f64();
                                                n3 = ((Float::with_val(PRECISION_BITS, beta_p))
                                                    .sin())
                                                .to_f64();
                                                cosTheta =
                                                    ((((Float::with_val(PRECISION_BITS, beta))
                                                        .cos()
                                                        * (Float::with_val(
                                                            PRECISION_BITS,
                                                            beta_p,
                                                        ))
                                                        .cos())
                                                        * (Float::with_val(
                                                            PRECISION_BITS,
                                                            (lambda - lambda_p),
                                                        ))
                                                        .cos())
                                                        + ((Float::with_val(
                                                            PRECISION_BITS,
                                                            beta,
                                                        ))
                                                        .sin()
                                                            * (Float::with_val(
                                                                PRECISION_BITS,
                                                                beta_p,
                                                            ))
                                                            .sin()))
                                                    .to_f64();
                                                e11c =
                                                    (((2i32) as f64 * lambda).sin() * (beta).sin());
                                                e21c = ((-((2i32) as f64 * lambda).cos())
                                                    * (beta).sin());
                                                e31c = ((-(lambda).sin()) * (beta).cos());
                                                e12c = ((-((2i32) as f64 * lambda).cos())
                                                    * (beta).sin());
                                                e22c = ((-((2i32) as f64 * lambda).sin())
                                                    * (beta).sin());
                                                e32c = ((lambda).cos() * (beta).cos());
                                                e13c = ((-(lambda).sin()) * (beta).cos());
                                                e23c = ((lambda).cos() * (beta).cos());
                                                e33c = (0i32) as f64;
                                                resc = Float::with_val(
                                                    PRECISION_BITS,
                                                    (((n1
                                                        * (((n1 * e11c) + (n2 * e12c))
                                                            + (n3 * e13c)))
                                                        + (n2
                                                            * (((n1 * e21c) + (n2 * e22c))
                                                                + (n3 * e23c))))
                                                        + (n3
                                                            * (((n1 * e31c) + (n2 * e32c))
                                                                + (n3 * e33c)))),
                                                );
                                                if (((1i32) as f64 - cosTheta) == 0.0f64) {
                                                    resc = Float::with_val(PRECISION_BITS, 0.0f64);
                                                } else {
                                                    resc = ((Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("1.0").unwrap(),
                                                    ) / (Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("2.0").unwrap(),
                                                    ) * (Float::with_val(
                                                        PRECISION_BITS,
                                                        Float::parse("1.0").unwrap(),
                                                    ) - Float::with_val(
                                                        PRECISION_BITS,
                                                        cosTheta,
                                                    )))) * (resc).clone());
                                                }
                                                psr.quad_ifunc_geom_c =
                                                    Some(((resc).clone()).to_f64());
                                            }
                                        }
                                    }
                                }
                                let mut ival: Option<f64> = Some(crate::ifunc::interpolate(
                                    &psr.quad_ifuncT_c
                                        .iter()
                                        .take(
                                            (*read(&psr.quad_ifuncN_c, "psr[p].quad_ifuncN_c")?)
                                                as usize,
                                        )
                                        .map(|v| read(v, "quad_ifuncT_c").copied())
                                        .collect::<Result<Vec<_>, _>>()?,
                                    &psr.quad_ifuncV_c
                                        .iter()
                                        .take(
                                            (*read(&psr.quad_ifuncN_c, "psr[p].quad_ifuncN_c")?)
                                                as usize,
                                        )
                                        .map(|v| read(v, "quad_ifuncV_c").copied())
                                        .collect::<Result<Vec<_>, _>>()?,
                                    ((read(&rows[(i) as usize].sat, "psr[p].obsn[i].sat")?)
                                        .clone())
                                    .to_f64(),
                                    *read(&psr.quad_ifuncN_c, "psr[p].quad_ifuncN_c")?,
                                    d,
                                )?);
                                phaseW = ((phaseW).clone()
                                    + (((psr.param.slots[param_f as usize].cells[0].val).clone()
                                        * Float::with_val(PRECISION_BITS, *read(&ival, "ival")?))
                                        * Float::with_val(
                                            PRECISION_BITS,
                                            *read(
                                                &psr.quad_ifunc_geom_c,
                                                "psr[p].quad_ifunc_geom_c",
                                            )?,
                                        )));
                            }
                        }
                        if ((psr.param.slots[param_quad_ifunc_p as usize].cells[0].param_set
                            == 1i32)
                            && ((psr.param.slots[param_quad_ifunc_p as usize].cells[0].val)
                                .clone()
                                == Float::with_val(PRECISION_BITS, 2i32)))
                        {
                            {
                                let mut lp: f64;
                                let mut bp: f64;
                                let mut lg: f64;
                                let mut bg: f64;
                                if (*read(&psr.quad_ifunc_geom_p, "psr[p].quad_ifunc_geom_p")?
                                    == (0i32) as f64)
                                {
                                    {
                                        if (*read(&psr.simflag, "psr[p].simflag")? == 1i32) {
                                            {
                                                lp = *read(&psr.rasim, "psr[p].rasim")?;
                                                bp = *read(&psr.decsim, "psr[p].decsim")?;
                                            }
                                        } else {
                                            {
                                                lp = ((psr.param.slots[param_raj as usize].cells
                                                    [0]
                                                .val)
                                                    .clone())
                                                .to_f64();
                                                bp = ((psr.param.slots[param_decj as usize].cells
                                                    [0]
                                                .val)
                                                    .clone())
                                                .to_f64();
                                            }
                                        }
                                        lg = *read(&psr.quad_ifunc_p_RA, "psr[p].quad_ifunc_p_RA")?;
                                        bg = *read(
                                            &psr.quad_ifunc_p_DEC,
                                            "psr[p].quad_ifunc_p_DEC",
                                        )?;
                                        let mut px: f64;
                                        let mut py: f64;
                                        let mut pz: f64;
                                        let mut gx: f64;
                                        let mut gy: f64;
                                        let mut gz: f64;
                                        px = ((bp).cos() * (lp).cos());
                                        py = ((bp).cos() * (lp).sin());
                                        pz = (bp).sin();
                                        gx = ((bg).cos() * (lg).cos());
                                        gy = ((bg).cos() * (lg).sin());
                                        gz = (bg).sin();
                                        let mut ctheta: f64;
                                        ctheta = (((px * gx) + (py * gy)) + (pz * gz));
                                        let mut rho: f64;
                                        let mut rhox: f64;
                                        let mut rhoy: f64;
                                        let mut rhoz: f64;
                                        rhox = (px - (ctheta * gx));
                                        rhoy = (py - (ctheta * gy));
                                        rhoz = (pz - (ctheta * gz));
                                        let mut pol: f64;
                                        let mut polx: f64;
                                        let mut poly: f64;
                                        let mut polz: f64;
                                        polx = ((bg).sin() * (lg).cos());
                                        poly = ((bg).sin() * (lg).sin());
                                        polz = (-(bg).cos());
                                        rho = (((rhox * rhox) + (rhoy * rhoy)) + (rhoz * rhoz))
                                            .sqrt();
                                        pol = (((polx * polx) + (poly * poly)) + (polz * polz))
                                            .sqrt();
                                        let mut phi: f64;
                                        let mut cphi: f64;
                                        let mut c2phi: f64;
                                        phi = ((((rhox * polx) + (rhoy * poly)) + (rhoz * polz))
                                            / (rho * pol))
                                            .acos();
                                        cphi = ((((rhox * polx) + (rhoy * poly)) + (rhoz * polz))
                                            / (rho * pol));
                                        c2phi = ((((2i32) as f64 * cphi) * cphi) - (1i32) as f64);
                                        d.stderr(
                                            &[(phi).to_string().into_bytes(), b"\n".to_vec()]
                                                .concat(),
                                        );
                                        let mut resp: Float;
                                        resp = Float::with_val(
                                            PRECISION_BITS,
                                            ((0.5f64 * ((1i32) as f64 - ctheta)) * c2phi),
                                        );
                                        psr.quad_ifunc_geom_p = Some(((resp).clone()).to_f64());
                                    }
                                }
                                let mut ival: Option<f64> = Some(crate::ifunc::interpolate(
                                    &psr.quad_ifuncT_p
                                        .iter()
                                        .take(
                                            (*read(&psr.quad_ifuncN_p, "psr[p].quad_ifuncN_p")?)
                                                as usize,
                                        )
                                        .map(|v| read(v, "quad_ifuncT_p").copied())
                                        .collect::<Result<Vec<_>, _>>()?,
                                    &psr.quad_ifuncV_p
                                        .iter()
                                        .take(
                                            (*read(&psr.quad_ifuncN_p, "psr[p].quad_ifuncN_p")?)
                                                as usize,
                                        )
                                        .map(|v| read(v, "quad_ifuncV_p").copied())
                                        .collect::<Result<Vec<_>, _>>()?,
                                    ((read(&rows[(i) as usize].sat, "psr[p].obsn[i].sat")?)
                                        .clone())
                                    .to_f64(),
                                    *read(&psr.quad_ifuncN_p, "psr[p].quad_ifuncN_p")?,
                                    d,
                                )?);
                                phaseW = ((phaseW).clone()
                                    + (((psr.param.slots[param_f as usize].cells[0].val).clone()
                                        * Float::with_val(PRECISION_BITS, *read(&ival, "ival")?))
                                        * Float::with_val(
                                            PRECISION_BITS,
                                            *read(
                                                &psr.quad_ifunc_geom_p,
                                                "psr[p].quad_ifunc_geom_p",
                                            )?,
                                        )));
                            }
                        }
                        if ((psr.param.slots[param_quad_ifunc_c as usize].cells[0].param_set
                            == 1i32)
                            && ((psr.param.slots[param_quad_ifunc_c as usize].cells[0].val)
                                .clone()
                                == Float::with_val(PRECISION_BITS, 2i32)))
                        {
                            {
                                let mut lp: f64;
                                let mut bp: f64;
                                let mut lg: f64;
                                let mut bg: f64;
                                if (*read(&psr.quad_ifunc_geom_c, "psr[p].quad_ifunc_geom_c")?
                                    == (0i32) as f64)
                                {
                                    {
                                        if (*read(&psr.simflag, "psr[p].simflag")? == 1i32) {
                                            {
                                                lp = *read(&psr.rasim, "psr[p].rasim")?;
                                                bp = *read(&psr.decsim, "psr[p].decsim")?;
                                            }
                                        } else {
                                            {
                                                lp = ((psr.param.slots[param_raj as usize].cells
                                                    [0]
                                                .val)
                                                    .clone())
                                                .to_f64();
                                                bp = ((psr.param.slots[param_decj as usize].cells
                                                    [0]
                                                .val)
                                                    .clone())
                                                .to_f64();
                                            }
                                        }
                                        lg = *read(&psr.quad_ifunc_c_RA, "psr[p].quad_ifunc_c_RA")?;
                                        bg = *read(
                                            &psr.quad_ifunc_c_DEC,
                                            "psr[p].quad_ifunc_c_DEC",
                                        )?;
                                        let mut px: f64;
                                        let mut py: f64;
                                        let mut pz: f64;
                                        let mut gx: f64;
                                        let mut gy: f64;
                                        let mut gz: f64;
                                        px = ((bp).cos() * (lp).cos());
                                        py = ((bp).cos() * (lp).sin());
                                        pz = (bp).sin();
                                        gx = ((bg).cos() * (lg).cos());
                                        gy = ((bg).cos() * (lg).sin());
                                        gz = (bg).sin();
                                        let mut ctheta: f64;
                                        ctheta = (((px * gx) + (py * gy)) + (pz * gz));
                                        let mut rho: f64;
                                        let mut rhox: f64;
                                        let mut rhoy: f64;
                                        let mut rhoz: f64;
                                        rhox = (px - (ctheta * gx));
                                        rhoy = (py - (ctheta * gy));
                                        rhoz = (pz - (ctheta * gz));
                                        let mut pol: f64;
                                        let mut polx: f64;
                                        let mut poly: f64;
                                        let mut polz: f64;
                                        let mut thetax: f64;
                                        let mut thetay: f64;
                                        let mut thetaz: f64;
                                        let mut phix: f64;
                                        let mut phiy: f64;
                                        let mut phiz: f64;
                                        thetax = ((bg).sin() * (lg).cos());
                                        thetay = ((bg).sin() * (lg).sin());
                                        thetaz = (-(bg).cos());
                                        phix = (-(lg).sin());
                                        phiy = (lg).cos();
                                        phiz = (0i32) as f64;
                                        polx = (thetax + phix);
                                        poly = (thetay + phiy);
                                        polz = (thetaz + phiz);
                                        pol = (((polx * polx) + (poly * poly)) + (polz * polz))
                                            .sqrt();
                                        rho = (((rhox * rhox) + (rhoy * rhoy)) + (rhoz * rhoz))
                                            .sqrt();
                                        let mut cphi: f64;
                                        let mut c2phi: f64;
                                        cphi = (((((rhox * polx) + (rhoy * poly))
                                            + (rhoz * polz))
                                            / rho)
                                            / pol);
                                        c2phi = ((((2i32) as f64 * cphi) * cphi) - (1i32) as f64);
                                        let mut resc: Float;
                                        resc = Float::with_val(
                                            PRECISION_BITS,
                                            ((0.5f64 * ((1i32) as f64 - ctheta)) * c2phi),
                                        );
                                        psr.quad_ifunc_geom_c = Some(((resc).clone()).to_f64());
                                    }
                                }
                                let mut ival: Option<f64> = Some(crate::ifunc::interpolate(
                                    &psr.quad_ifuncT_c
                                        .iter()
                                        .take(
                                            (*read(&psr.quad_ifuncN_c, "psr[p].quad_ifuncN_c")?)
                                                as usize,
                                        )
                                        .map(|v| read(v, "quad_ifuncT_c").copied())
                                        .collect::<Result<Vec<_>, _>>()?,
                                    &psr.quad_ifuncV_c
                                        .iter()
                                        .take(
                                            (*read(&psr.quad_ifuncN_c, "psr[p].quad_ifuncN_c")?)
                                                as usize,
                                        )
                                        .map(|v| read(v, "quad_ifuncV_c").copied())
                                        .collect::<Result<Vec<_>, _>>()?,
                                    ((read(&rows[(i) as usize].sat, "psr[p].obsn[i].sat")?)
                                        .clone())
                                    .to_f64(),
                                    *read(&psr.quad_ifuncN_c, "psr[p].quad_ifuncN_c")?,
                                    d,
                                )?);
                                phaseW = ((phaseW).clone()
                                    + (((psr.param.slots[param_f as usize].cells[0].val).clone()
                                        * Float::with_val(PRECISION_BITS, *read(&ival, "ival")?))
                                        * Float::with_val(
                                            PRECISION_BITS,
                                            *read(
                                                &psr.quad_ifunc_geom_c,
                                                "psr[p].quad_ifunc_geom_c",
                                            )?,
                                        )));
                            }
                        }
                        if ((psr.param.slots[param_gwb_amp as usize].cells[0].param_set == 1i32)
                            && (psr.param.slots[param_gwb_amp as usize].cells[1].param_set == 1i32))
                        {
                            {
                                let mut dt: Float;
                                let mut prefac: Float;
                                let mut resp: Float;
                                let mut resc: Float;
                                let mut lambda_p: f64;
                                let mut beta_p: f64;
                                let mut lambda: f64;
                                let mut beta: f64;
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
                                lambda_p = ((psr.param.slots[param_raj as usize].cells[0].val)
                                    .clone())
                                .to_f64();
                                beta_p = ((psr.param.slots[param_decj as usize].cells[0].val)
                                    .clone())
                                .to_f64();
                                lambda = *read(&psr.gwb_raj, "psr[p].gwb_raj")?;
                                beta = *read(&psr.gwb_decj, "psr[p].gwb_decj")?;
                                n1 = ((Float::with_val(PRECISION_BITS, lambda_p)).cos()
                                    * (Float::with_val(PRECISION_BITS, beta_p)).cos())
                                .to_f64();
                                n2 = ((Float::with_val(PRECISION_BITS, lambda_p)).sin()
                                    * (Float::with_val(PRECISION_BITS, beta_p)).cos())
                                .to_f64();
                                n3 = ((Float::with_val(PRECISION_BITS, beta_p)).sin()).to_f64();
                                cosTheta = ((((Float::with_val(PRECISION_BITS, beta)).cos()
                                    * (Float::with_val(PRECISION_BITS, beta_p)).cos())
                                    * (Float::with_val(PRECISION_BITS, (lambda - lambda_p)))
                                        .cos())
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
                                if (((1i32) as f64 - cosTheta) == 0.0f64) {
                                    resp = Float::with_val(PRECISION_BITS, 0.0f64);
                                } else {
                                    resp = ((Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1.0").unwrap(),
                                    ) / (Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("2.0").unwrap(),
                                    ) * (Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1.0").unwrap(),
                                    ) - Float::with_val(
                                        PRECISION_BITS,
                                        cosTheta,
                                    )))) * (resp).clone());
                                }
                                psr.gwb_geom_p = Some(((resp).clone()).to_f64());
                                lambda = *read(&psr.gwb_raj, "psr[p].gwb_raj")?;
                                beta = *read(&psr.gwb_decj, "psr[p].gwb_decj")?;
                                e11c =
                                    ((Float::with_val(PRECISION_BITS, ((2i32) as f64 * lambda)))
                                        .sin()
                                        * (Float::with_val(PRECISION_BITS, beta)).sin())
                                    .to_f64();
                                e21c = ((-(Float::with_val(
                                    PRECISION_BITS,
                                    ((2i32) as f64 * lambda),
                                ))
                                .cos())
                                    * (Float::with_val(PRECISION_BITS, beta)).sin())
                                .to_f64();
                                e31c = ((-(Float::with_val(PRECISION_BITS, lambda)).sin())
                                    * (Float::with_val(PRECISION_BITS, beta)).cos())
                                .to_f64();
                                e12c = ((-(Float::with_val(
                                    PRECISION_BITS,
                                    ((2i32) as f64 * lambda),
                                ))
                                .cos())
                                    * (Float::with_val(PRECISION_BITS, beta)).sin())
                                .to_f64();
                                e22c = ((-(Float::with_val(
                                    PRECISION_BITS,
                                    ((2i32) as f64 * lambda),
                                ))
                                .sin())
                                    * (Float::with_val(PRECISION_BITS, beta)).sin())
                                .to_f64();
                                e32c = ((Float::with_val(PRECISION_BITS, lambda)).cos()
                                    * (Float::with_val(PRECISION_BITS, beta)).cos())
                                .to_f64();
                                e13c = ((-(Float::with_val(PRECISION_BITS, lambda)).sin())
                                    * (Float::with_val(PRECISION_BITS, beta)).cos())
                                .to_f64();
                                e23c = ((Float::with_val(PRECISION_BITS, lambda)).cos()
                                    * (Float::with_val(PRECISION_BITS, beta)).cos())
                                .to_f64();
                                e33c = (0i32) as f64;
                                resc = Float::with_val(
                                    PRECISION_BITS,
                                    (((n1 * (((n1 * e11c) + (n2 * e12c)) + (n3 * e13c)))
                                        + (n2 * (((n1 * e21c) + (n2 * e22c)) + (n3 * e23c))))
                                        + (n3 * (((n1 * e31c) + (n2 * e32c)) + (n3 * e33c)))),
                                );
                                if (((1i32) as f64 - cosTheta) == 0.0f64) {
                                    resc = Float::with_val(PRECISION_BITS, 0.0f64);
                                } else {
                                    resc = ((Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1.0").unwrap(),
                                    ) / (Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("2.0").unwrap(),
                                    ) * (Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1.0").unwrap(),
                                    ) - Float::with_val(
                                        PRECISION_BITS,
                                        cosTheta,
                                    )))) * (resc).clone());
                                }
                                psr.gwb_geom_c = Some(((resc).clone()).to_f64());
                                dt = (((read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?)
                                    .clone()
                                    - Float::with_val(
                                        PRECISION_BITS,
                                        *read(&psr.gwb_epoch, "psr[p].gwb_epoch")?,
                                    ))
                                    / Float::with_val(
                                        PRECISION_BITS,
                                        *read(&psr.gwb_width, "psr[p].gwb_width")?,
                                    ));
                                prefac = ((dt).clone()
                                    * ((Float::with_val(
                                        PRECISION_BITS,
                                        (-(dt).clone()).to_f64(),
                                    ) * (dt).clone())
                                        / Float::with_val(PRECISION_BITS, 2.0f64))
                                    .exp());
                                d.stderr(
                                    &[
                                        crate::clock_text::format_number(
                                            *read(&psr.gwb_width, "psr[p].gwb_width")?,
                                            crate::clock_text::NumberFormat::ScientificThree,
                                        ),
                                        b" ".to_vec(),
                                        crate::clock_text::format_number(
                                            *read(&psr.gwb_epoch, "psr[p].gwb_epoch")?,
                                            crate::clock_text::NumberFormat::ScientificThree,
                                        ),
                                        b" ".to_vec(),
                                        crate::clock_text::format_number(
                                            ((prefac).clone()).to_f64(),
                                            crate::clock_text::NumberFormat::ScientificThree,
                                        ),
                                        b"\n".to_vec(),
                                    ]
                                    .concat(),
                                );
                                phaseW = ((phaseW).clone()
                                    + ((((psr.param.slots[param_f as usize].cells[0].val)
                                        .clone()
                                        * (prefac).clone())
                                        * (psr.param.slots[param_gwb_amp as usize].cells[0].val)
                                            .clone())
                                        * Float::with_val(
                                            PRECISION_BITS,
                                            *read(&psr.gwb_geom_p, "psr[p].gwb_geom_p")?,
                                        )));
                                phaseW = ((phaseW).clone()
                                    + ((((psr.param.slots[param_f as usize].cells[0].val)
                                        .clone()
                                        * (prefac).clone())
                                        * (psr.param.slots[param_gwb_amp as usize].cells[1].val)
                                            .clone())
                                        * Float::with_val(
                                            PRECISION_BITS,
                                            *read(&psr.gwb_geom_c, "psr[p].gwb_geom_c")?,
                                        )));
                            }
                        }
                        phase5[(i) as usize] = Some(
                            ((((((phase2).clone() + (phase3).clone()) + (phase4).clone())
                                + (phaseJ).clone())
                                + (phaseW).clone())
                                + (phase2state).clone())
                                + (phaseShape).clone(),
                        );
                        if (*read(&rows[(i) as usize].nFlags, "psr[p].obsn[i].nFlags")? > 0i32) {
                            {
                                let mut k: i32;
                                let mut extra: Float;
                                {
                                    k = 0i32;
                                    while (k < *read(
                                        &rows[(i) as usize].nFlags,
                                        "psr[p].obsn[i].nFlags",
                                    )?) {
                                        {
                                            if ((i32::from(
                                                read_string(
                                                    &rows[(i) as usize].flagID[(k) as usize][..],
                                                    "string",
                                                )? != "-radd".as_bytes(),
                                            )) == 0i32)
                                            {
                                                {
                                                    extra = crate::source_text::extended(
                                                        &read_string(
                                                            &rows[(i) as usize].flagVal
                                                                [(k) as usize][..],
                                                            "string",
                                                        )?,
                                                    )?;
                                                    phase5[(i) as usize] = Some(
                                                        (read(&phase5[(i) as usize], "phase5[i]")?)
                                                            .clone()
                                                            + ((extra).clone()
                                                                * (psr.param.slots
                                                                    [param_f as usize]
                                                                    .cells[0]
                                                                    .val)
                                                                    .clone()),
                                                    );
                                                }
                                            }
                                            if ((i32::from(
                                                read_string(
                                                    &rows[(i) as usize].flagID[(k) as usize][..],
                                                    "string",
                                                )? != "-padd".as_bytes(),
                                            )) == 0i32)
                                            {
                                                {
                                                    extra = crate::source_text::extended(
                                                        &read_string(
                                                            &rows[(i) as usize].flagVal
                                                                [(k) as usize][..],
                                                            "string",
                                                        )?,
                                                    )?;
                                                    phase5[(i) as usize] = Some(
                                                        (read(&phase5[(i) as usize], "phase5[i]")?)
                                                            .clone()
                                                            + (extra).clone(),
                                                    );
                                                }
                                            }
                                        }
                                        k += 1;
                                    }
                                }
                            }
                        }
                        if (psr.param.slots[param_iperharm as usize].cells[0].param_set == 1i32) {
                            {
                                let mut phaseTMP: f64 = (crate::tempo2_util::fortran_mod(
                                    (read(&phase5[(i) as usize], "phase5[i]")?).clone(),
                                    &Float::with_val(PRECISION_BITS, 1.0f64),
                                )?)
                                .to_f64();
                                if (Float::with_val(PRECISION_BITS, phaseTMP)
                                    >= (Float::with_val(PRECISION_BITS, (1.0f64 / 2.0f64))
                                        / (psr.param.slots[param_iperharm as usize].cells[0].val)
                                            .clone()))
                                {
                                    {
                                        while (Float::with_val(PRECISION_BITS, phaseTMP)
                                            >= (Float::with_val(PRECISION_BITS, (1.0f64 / 2.0f64))
                                                / (psr.param.slots[param_iperharm as usize].cells
                                                    [0]
                                                .val)
                                                    .clone()))
                                        {
                                            phase5[(i) as usize] = Some(
                                                (read(&phase5[(i) as usize], "phase5[i]")?).clone()
                                                    - (Float::with_val(PRECISION_BITS, 1.0f64)
                                                        / (psr.param.slots
                                                            [param_iperharm as usize]
                                                            .cells[0]
                                                            .val)
                                                            .clone()),
                                            );
                                            phaseTMP = (Float::with_val(PRECISION_BITS, phaseTMP)
                                                - (Float::with_val(PRECISION_BITS, 1.0f64)
                                                    / (psr.param.slots[param_iperharm as usize]
                                                        .cells[0]
                                                        .val)
                                                        .clone()))
                                            .to_f64();
                                        }
                                    }
                                } else {
                                    if (Float::with_val(PRECISION_BITS, phaseTMP)
                                        < (Float::with_val(PRECISION_BITS, ((-1.0f64) / 2.0f64))
                                            / (psr.param.slots[param_iperharm as usize].cells[0]
                                                .val)
                                                .clone()))
                                    {
                                        {
                                            while (Float::with_val(PRECISION_BITS, phaseTMP)
                                                < (Float::with_val(
                                                    PRECISION_BITS,
                                                    ((-1.0f64) / 2.0f64),
                                                ) / (psr.param.slots[param_iperharm as usize]
                                                    .cells[0]
                                                    .val)
                                                    .clone()))
                                            {
                                                phase5[(i) as usize] = Some(
                                                    (read(&phase5[(i) as usize], "phase5[i]")?)
                                                        .clone()
                                                        + (Float::with_val(PRECISION_BITS, 1.0f64)
                                                            / (psr.param.slots
                                                                [param_iperharm as usize]
                                                                .cells[0]
                                                                .val)
                                                                .clone()),
                                                );
                                                phaseTMP =
                                                    (Float::with_val(PRECISION_BITS, phaseTMP)
                                                        + (Float::with_val(
                                                            PRECISION_BITS,
                                                            1.0f64,
                                                        ) / (psr.param.slots
                                                            [param_iperharm as usize]
                                                            .cells[0]
                                                            .val)
                                                            .clone()))
                                                    .to_f64();
                                            }
                                        }
                                    }
                                }
                                phase5[(i) as usize] =
                                    Some((read(&phase5[(i) as usize], "phase5[i]")?).clone());
                            }
                        } else {
                            {
                                phase5[(i) as usize] =
                                    Some((read(&phase5[(i) as usize], "phase5[i]")?).clone());
                            }
                        }
                    }
                    i += 1;
                }
            }
            // formResiduals.C:2164-2183: selector and reference row belong to p, not psr[0].
            let mut startSet: bool = ((psr.param.slots[param_start as usize].cells[0].param_set
                == 1i32)
                && (psr.param.slots[param_start as usize].cells[0].fit_flag == 1i32));
            let mut finishSet: bool = ((psr.param.slots[param_finish as usize].cells[0].param_set
                == 1i32)
                && (psr.param.slots[param_finish as usize].cells[0].fit_flag == 1i32));
            let mut bat_startSet: bool =
                ((psr.param.slots[param_start as usize].cells[0].param_set == 1i32)
                    && (psr.param.slots[param_start as usize].cells[0].fit_flag == 2i32));
            let mut bat_finishSet: bool =
                ((psr.param.slots[param_finish as usize].cells[0].param_set == 1i32)
                    && (psr.param.slots[param_finish as usize].cells[0].fit_flag == 2i32));
            let mut start: Float = Float::with_val(PRECISION_BITS, 1e10f64);
            let mut finish: Float = Float::with_val(PRECISION_BITS, 0i32);
            if (startSet || bat_startSet) {
                start = (psr.param.slots[param_start as usize].cells[0].val).clone();
            }
            if (finishSet || bat_finishSet) {
                finish = (psr.param.slots[param_finish as usize].cells[0].val).clone();
            }
            {
                i = 0i32;
                while (i < *read(&psr.nobs, "psr[p].nobs")?) {
                    {
                        // formResiduals.C:2183: reference selection belongs to the current pulsar.
                        let o = &rows[i as usize];
                        if (*read(&o.deleted, "o->deleted")? != 0) {
                            i += 1;
                            continue;
                        }
                        if (startSet
                            && ((read(&o.sat, "o->sat")?).clone()
                                < ((start).clone()
                                    - Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1e-11").unwrap(),
                                    ))))
                        {
                            i += 1;
                            continue;
                        }
                        if (finishSet
                            && ((read(&o.sat, "o->sat")?).clone()
                                > ((finish).clone()
                                    + Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1e-11").unwrap(),
                                    ))))
                        {
                            i += 1;
                            continue;
                        }
                        if (bat_startSet
                            && ((read(&o.bat, "o->bat")?).clone()
                                < ((start).clone()
                                    - Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1e-11").unwrap(),
                                    ))))
                        {
                            i += 1;
                            continue;
                        }
                        if (bat_finishSet
                            && ((read(&o.bat, "o->bat")?).clone()
                                > ((finish).clone()
                                    + Float::with_val(
                                        PRECISION_BITS,
                                        Float::parse("1e-11").unwrap(),
                                    ))))
                        {
                            i += 1;
                            continue;
                        }
                        phas1 = crate::tempo2_util::fortran_mod(
                            (read(&phase5[(i) as usize], "phase5[i]")?).clone(),
                            &Float::with_val(PRECISION_BITS, Float::parse("1.0").unwrap()),
                        )?;
                        break;
                    }
                }
            }
            references.push(phas1.clone());
            {
                i = 0i32;
                while (i < *read(&psr.nobs, "psr[p].nobs")?) {
                    {
                        phase5[(i) as usize] = Some(
                            (read(&phase5[(i) as usize], "phase5[i]")?).clone() - (phas1).clone(),
                        );
                        nphase = crate::tempo2_util::fortran_nlong(
                            &(read(&phase5[(i) as usize], "phase5[i]")?).clone(),
                        );
                        rows[(i) as usize].nphase = Some((nphase).clone());
                        residual = ((read(&phase5[(i) as usize], "phase5[i]")?).clone()
                            - (nphase).clone());
                        if (*read(&rows[(i) as usize].deleted, "psr[p].obsn[i].deleted")? != 1i32) {
                            gotit = 1i32;
                        } else {
                            gotit = 0i32;
                        }
                        if (((psr.param.slots[param_track as usize].cells[0].param_set == 1i32)
                            && ((psr.param.slots[param_track as usize].cells[0].val).clone()
                                > Float::with_val(PRECISION_BITS, 0i32)))
                            && (time == 1i32))
                        {
                            {
                                residual =
                                    ((residual).clone() + Float::with_val(PRECISION_BITS, ntrk));
                                if (gotit == 1i32) {
                                    {
                                        if (((((read(
                                            &rows[(i) as usize].bbat,
                                            "psr[p].obsn[i].bbat",
                                        )?)
                                        .clone()
                                            - (ct00).clone())
                                            < Float::with_val(
                                                PRECISION_BITS,
                                                Float::parse("0.0").unwrap(),
                                            ))
                                            && (((read(
                                                &rows[(i) as usize].bbat,
                                                "psr[p].obsn[i].bbat",
                                            )?)
                                            .clone()
                                                - (ct00).clone())
                                            .abs()
                                                > Float::with_val(PRECISION_BITS, 1i32)))
                                            && (i > 0i32))
                                        {
                                            {
                                                d.stdout(b"ERROR: Points must be in time order for tracking to work\n");
                                                d.stdout(
                                                    &[
                                                        b"Pulsar = ".to_vec(),
                                                        read_string(&psr.name[..], "string")?,
                                                        b"\n".to_vec(),
                                                    ]
                                                    .concat(),
                                                );
                                                d.stdout(&[b"Observation ".to_vec(),(i).to_string().into_bytes(),b" (".to_vec(),read_string(&rows[(i) as usize].fname[..], "string")?,b") has BBAT = ".to_vec(),crate::clock_text::format_number(((read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?).clone()).to_f64(), crate::clock_text::NumberFormat::GeneralFive),b"\n".to_vec()].concat());
                                                d.stdout(&[b"Observation ".to_vec(),(i - 1i32).to_string().into_bytes(),b" (".to_vec(),read_string(&rows[(i - 1i32) as usize].fname[..], "string")?,b") has BBAT = ".to_vec(),crate::clock_text::format_number(((read(&rows[(i - 1i32) as usize].bbat, "psr[p].obsn[i - 1].bbat")?).clone()).to_f64(), crate::clock_text::NumberFormat::GeneralFive),b"\n".to_vec()].concat());
                                                d.stdout(
                                                    &[
                                                        b"Difference = ".to_vec(),
                                                        crate::clock_text::extended_general_six(
                                                            &((read(
                                                                &rows[(i) as usize].bbat,
                                                                "psr[p].obsn[i].bbat",
                                                            )?)
                                                            .clone()),
                                                        ),
                                                        b" ".to_vec(),
                                                        crate::clock_text::extended_general_six(
                                                            &((ct00).clone()),
                                                        ),
                                                        b" ".to_vec(),
                                                        crate::clock_text::extended_general_six(
                                                            &((read(
                                                                &rows[(i) as usize].bbat,
                                                                "psr[p].obsn[i].bbat",
                                                            )?)
                                                            .clone()
                                                                - (ct00).clone()),
                                                        ),
                                                        b"\n".to_vec(),
                                                    ]
                                                    .concat(),
                                                );
                                                return Err(boundary(
                                                    "formResiduals.C::formResiduals: source exit",
                                                ));
                                            }
                                        }
                                        if (((read(
                                            &rows[(i) as usize].bbat,
                                            "psr[p].obsn[i].bbat",
                                        )?)
                                        .clone()
                                            - (ct00).clone())
                                            < ((psr.param.slots[param_track as usize].cells[0]
                                                .val)
                                                .clone())
                                            .abs())
                                        {
                                            {
                                                if ((((residual).clone()
                                                    + Float::with_val(PRECISION_BITS, 1.0f64))
                                                    - (dt00).clone())
                                                .abs()
                                                    < ((residual).clone() - (dt00).clone()).abs())
                                                {
                                                    {
                                                        residual = ((residual).clone()
                                                            + Float::with_val(
                                                                PRECISION_BITS,
                                                                1.0f64,
                                                            ));
                                                        ntrk = (ntrk).checked_add(1i32).ok_or_else(|| boundary("tracking signed integer overflow"))?;
                                                        d.stdout(
                                                            &[
                                                                b"ADDING PHASE ".to_vec(),
                                                                read_string(
                                                                    &rows[(i) as usize].fname[..],
                                                                    "string",
                                                                )?,
                                                                b"\n".to_vec(),
                                                            ]
                                                            .concat(),
                                                        );
                                                    }
                                                } else {
                                                    if ((((residual).clone()
                                                        - Float::with_val(PRECISION_BITS, 1.0f64))
                                                        - (dt00).clone())
                                                    .abs()
                                                        < ((residual).clone() - (dt00).clone())
                                                            .abs())
                                                    {
                                                        {
                                                            residual = ((residual).clone()
                                                                - Float::with_val(
                                                                    PRECISION_BITS,
                                                                    1.0f64,
                                                                ));
                                                            ntrk = (ntrk).checked_sub(1i32).ok_or_else(|| boundary("tracking signed integer overflow"))?;
                                                            d.stdout(&[b"SUBTRACT PHASE ".to_vec(),read_string(&rows[(i) as usize].fname[..], "string")?,b"\n".to_vec()].concat());
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if (((psr.param.slots[param_track as usize].cells[0].val).clone()).to_f64()
                            == (-1i32) as f64)
                        {
                            {
                                if (dtm1s == 0i32) {
                                    d.stdout(b"Attempting tracking via the gradient method\n");
                                }
                                if (((dtm1s > 1i32)
                                    && (((read(&rows[(i) as usize].bat, "psr[p].obsn[i].bat")?)
                                        .clone()
                                        - (lastBat).clone())
                                    .abs()
                                        > Float::with_val(PRECISION_BITS, 1i32)))
                                    && (((lastBat).clone() - (priorBat).clone()).abs()
                                        > Float::with_val(PRECISION_BITS, 1i32)))
                                {
                                    {
                                        let mut m1: f64;
                                        let mut m2: f64;
                                        let mut m3: f64;
                                        let mut m4: f64;
                                        m1 = (Float::with_val(
                                            PRECISION_BITS,
                                            ((lastResidual).clone() - (priorResidual).clone())
                                                .to_f64(),
                                        ) / ((lastBat).clone() - (priorBat).clone()))
                                        .to_f64();
                                        m2 = (Float::with_val(
                                            PRECISION_BITS,
                                            ((residual).clone() - (lastResidual).clone()).to_f64(),
                                        ) / ((read(
                                            &rows[(i) as usize].bat,
                                            "psr[p].obsn[i].bat",
                                        )?)
                                        .clone()
                                            - (lastBat).clone()))
                                        .to_f64();
                                        m3 = (Float::with_val(
                                            PRECISION_BITS,
                                            (((residual).clone()
                                                - Float::with_val(PRECISION_BITS, 1i32))
                                                - (lastResidual).clone())
                                            .to_f64(),
                                        ) / ((read(
                                            &rows[(i) as usize].bat,
                                            "psr[p].obsn[i].bat",
                                        )?)
                                        .clone()
                                            - (lastBat).clone()))
                                        .to_f64();
                                        m4 = (Float::with_val(
                                            PRECISION_BITS,
                                            (((residual).clone()
                                                + Float::with_val(PRECISION_BITS, 1i32))
                                                - (lastResidual).clone())
                                            .to_f64(),
                                        ) / ((read(
                                            &rows[(i) as usize].bat,
                                            "psr[p].obsn[i].bat",
                                        )?)
                                        .clone()
                                            - (lastBat).clone()))
                                        .to_f64();
                                        if (((m1 - m2).abs() < (m1 - m3).abs())
                                            && ((m1 - m2).abs() < (m1 - m4).abs()))
                                        {
                                            {}
                                        } else {
                                            if (((m1 - m3).abs() < (m1 - m2).abs())
                                                && ((m1 - m3).abs() < (m1 - m4).abs()))
                                            {
                                                {
                                                    residual = ((residual).clone()
                                                        - Float::with_val(PRECISION_BITS, 1.0f64));
                                                    ntrk = (ntrk).checked_sub(1i32).ok_or_else(
                                                        || {
                                                            boundary(
                                                                "tracking signed integer overflow",
                                                            )
                                                        },
                                                    )?;
                                                }
                                            } else {
                                                if (((m1 - m4).abs() < (m1 - m2).abs())
                                                    && ((m1 - m4).abs() < (m1 - m3).abs()))
                                                {
                                                    {
                                                        residual = ((residual).clone()
                                                            + Float::with_val(
                                                                PRECISION_BITS,
                                                                1.0f64,
                                                            ));
                                                        ntrk = (ntrk).checked_add(1i32).ok_or_else(|| boundary("tracking signed integer overflow"))?;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if (((psr.param.slots[param_track as usize].cells[0].val).clone()).to_f64()
                            == (-2i32) as f64)
                        {
                            {
                                let mut pnNew: i64;
                                let mut pnAct: Option<i64> = None;
                                let mut addPhase: i64;
                                nf0 = crate::ifteph::extended_i32(
                                    &((psr.param.slots[param_f as usize].cells[0].val).clone()),
                                    "F0 integer part exceeds source i32 range",
                                )?;
                                ntpd = (crate::ifteph::extended_i32(
                                    &((read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?)
                                        .clone()),
                                    "binary_integer",
                                )?)
                                .checked_sub(crate::ifteph::extended_i32(
                                    &((read(&rows[0].bbat, "psr[p].obsn[0].bbat")?).clone()),
                                    "binary_integer",
                                )?)
                                .ok_or_else(|| {
                                    boundary("integer day difference exceeds source i32 range")
                                })?;
                                phaseint = Float::with_val(
                                    PRECISION_BITS,
                                    ((nf0.checked_mul(ntpd).ok_or_else(|| {
                                        boundary(
                                            "integer F0 * day difference exceeds source i32 range",
                                        )
                                    })?) as f64
                                        * 86400.0f64),
                                );
                                pnNew = crate::form_residuals::pulse_integer(
                                    &((phaseint).clone()
                                        + crate::tempo2_util::fortran_nlong(
                                            &(read(&phase5[(i) as usize], "phase5[i]")?).clone(),
                                        )),
                                )
                                .ok_or_else(|| {
                                    TimingError::new(
                                        "",
                                        None,
                                        "pulseN",
                                        "full pulse count outside signed 64-bit range",
                                    )
                                })?;
                                if d.debug {
                                    d.log(
                                        b'd',
                                        "formResiduals.C",
                                        2292,
                                        &[
                                            b"Have ".to_vec(),
                                            crate::clock_text::format_number(
                                                ((read(
                                                    &rows[(i) as usize].sat,
                                                    "psr[p].obsn[i].sat",
                                                )?)
                                                .clone())
                                                .to_f64(),
                                                crate::clock_text::NumberFormat::GeneralSix,
                                            ),
                                            b" ".to_vec(),
                                            ((pnNew).checked_sub(pn0).ok_or_else(|| {
                                                boundary("tracking signed integer overflow")
                                            })?)
                                            .to_string()
                                            .into_bytes(),
                                            b" ".to_vec(),
                                            (pn0).to_string().into_bytes(),
                                            b" ".to_vec(),
                                            (pnNew).to_string().into_bytes(),
                                        ]
                                        .concat(),
                                    );
                                }
                                if (pn0 == (-1i32) as i64) {
                                    {
                                        pn0 = pnNew;
                                        pnNew = (0i32) as i64;
                                    }
                                } else {
                                    pnNew = (pnNew).checked_sub(pn0).ok_or_else(|| {
                                        boundary("tracking signed integer overflow")
                                    })?;
                                }
                                {
                                    let mut kk: i32 = 0i32;
                                    while (kk
                                        < *read(
                                            &rows[(i) as usize].nFlags,
                                            "psr[p].obsn[i].nFlags",
                                        )?)
                                    {
                                        {
                                            if ((i32::from(
                                                read_string(
                                                    &rows[(i) as usize].flagID[(kk) as usize][..],
                                                    "string",
                                                )? != "-pnadd".as_bytes(),
                                            )) == 0i32)
                                            {
                                                {
                                                    if let Some(value) =
                                                        source_i64_bytes(&read_string(
                                                            &rows[(i) as usize].flagVal
                                                                [(kk) as usize][..],
                                                            "string",
                                                        )?)?
                                                    {
                                                        pnAct = Some(value);
                                                    }
                                                    pnAdd = (pnAdd)
                                                        .checked_add(*read(&pnAct, "pnAct")?)
                                                        .ok_or_else(|| {
                                                            boundary(
                                                                "tracking signed integer overflow",
                                                            )
                                                        })?;
                                                }
                                            }
                                        }
                                        kk += 1;
                                    }
                                }
                                {
                                    let mut kk: i32 = 0i32;
                                    while (kk
                                        < *read(
                                            &rows[(i) as usize].nFlags,
                                            "psr[p].obsn[i].nFlags",
                                        )?)
                                    {
                                        {
                                            if ((i32::from(
                                                read_string(
                                                    &rows[(i) as usize].flagID[(kk) as usize][..],
                                                    "string",
                                                )? != "-pn".as_bytes(),
                                            )) == 0i32)
                                            {
                                                {
                                                    if let Some(value) =
                                                        source_i64_bytes(&read_string(
                                                            &rows[(i) as usize].flagVal
                                                                [(kk) as usize][..],
                                                            "string",
                                                        )?)?
                                                    {
                                                        pnAct = Some(value);
                                                    }
                                                    pnAct = Some((*read(&pnAct, "pnAct")?).checked_add(pnAdd).ok_or_else(|| boundary("tracking signed integer overflow"))?);
                                                    addPhase = (pnNew)
                                                        .checked_sub(*read(&pnAct, "pnAct")?)
                                                        .ok_or_else(|| {
                                                            boundary(
                                                                "tracking signed integer overflow",
                                                            )
                                                        })?;
                                                    residual = ((residual).clone()
                                                        + Float::with_val(
                                                            PRECISION_BITS,
                                                            addPhase,
                                                        ));
                                                    ntrk =
                                                        i32::try_from(addPhase).map_err(|_| {
                                                            boundary("source_integer_narrowing")
                                                        })?;
                                                    if d.debug {
                                                        d.log(
                                                            b'd',
                                                            "formResiduals.C",
                                                            2317,
                                                            &[
                                                                b"*** Adding phase ".to_vec(),
                                                                (addPhase).to_string().into_bytes(),
                                                                b" ***".to_vec(),
                                                            ]
                                                            .concat(),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                        kk += 1;
                                    }
                                }
                            }
                        }
                        if (gotit == 1i32) {
                            {
                                dtm1s += 1;
                                dt00 = (residual).clone();
                                ct00 = (read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?)
                                    .clone();
                            }
                        }
                        if ((gotit == 1i32) && (time == 0i32)) {
                            time = 1i32;
                        }
                        rows[(i) as usize].residual = Some(
                            (residual).clone()
                                / (psr.param.slots[param_f as usize].cells[0].val).clone(),
                        );
                        rows[(i) as usize].residualtn = Some(
                            (residual).clone()
                                / (psr.param.slots[param_f as usize].cells[0].val).clone(),
                        );
                        priorResidual = (lastResidual).clone();
                        priorBat = (lastBat).clone();
                        lastResidual = (residual).clone();
                        lastBat = (read(&rows[(i) as usize].bat, "psr[p].obsn[i].bat")?).clone();
                        {
                            k = 0i32;
                            while (k < *read(&psr.nPhaseJump, "psr[p].nPhaseJump")?) {
                                {
                                    if ((read(&rows[(i) as usize].sat, "psr[p].obsn[i].sat")?)
                                        .clone()
                                        > (read(
                                            &rows[(*read(
                                                &psr.phaseJumpID[(k) as usize],
                                                "psr[p].phaseJumpID[k]",
                                            )?)
                                                as usize]
                                                .sat,
                                            "psr[p].obsn[psr[p].phaseJumpID[k]].sat",
                                        )?)
                                        .clone())
                                    {
                                        rows[(i) as usize].residual = Some(
                                            (read(
                                                &rows[(i) as usize].residual,
                                                "psr[p].obsn[i].residual",
                                            )?)
                                            .clone()
                                                + (Float::with_val(
                                                    PRECISION_BITS,
                                                    (*read(
                                                        &psr.phaseJumpDir[(k) as usize],
                                                        "psr[p].phaseJumpDir[k]",
                                                    )?)
                                                        as f64,
                                                ) / (psr.param.slots[param_f as usize].cells
                                                    [0]
                                                .val)
                                                    .clone()),
                                        );
                                    }
                                }
                                k += 1;
                            }
                        }
                        nf0 = crate::ifteph::extended_i32(
                            &((psr.param.slots[param_f as usize].cells[0].val).clone()),
                            "F0 integer part exceeds source i32 range",
                        )?;
                        ntpd = (crate::ifteph::extended_i32(
                            &((read(&rows[(i) as usize].bbat, "psr[p].obsn[i].bbat")?).clone()),
                            "binary_integer",
                        )?)
                        .checked_sub(crate::ifteph::extended_i32(
                            &((psr.param.slots[param_pepoch as usize].cells[0].val).clone()),
                            "binary_integer",
                        )?)
                        .ok_or_else(|| {
                            boundary("integer day difference exceeds source i32 range")
                        })?;
                        phaseint = Float::with_val(
                            PRECISION_BITS,
                            ((nf0.checked_mul(ntpd).ok_or_else(|| {
                                boundary("integer F0 * day difference exceeds source i32 range")
                            })?) as f64
                                * 86400.0f64),
                        );
                        rows[(i) as usize].phase = Some(
                            (phaseint).clone()
                                + (read(&phase5[(i) as usize], "phase5[i]")?).clone(),
                        );
                        rows[(i) as usize].pulseN = Some(
                            crate::form_residuals::pulse_integer(
                                &((phaseint).clone()
                                    + crate::tempo2_util::fortran_nlong(
                                        &(read(&phase5[(i) as usize], "phase5[i]")?).clone(),
                                    )),
                            )
                            .ok_or_else(|| {
                                TimingError::new(
                                    "",
                                    None,
                                    "pulseN",
                                    "full pulse count outside signed 64-bit range",
                                )
                            })?
                            .checked_sub((ntrk) as i64)
                            .ok_or_else(|| boundary("full pulse count tracking overflow"))?,
                        );
                        {
                            k = 0i32;
                            while (k < *read(&psr.nPhaseJump, "psr[p].nPhaseJump")?) {
                                if ((read(&rows[(i) as usize].sat, "psr[p].obsn[i].sat")?).clone()
                                    > (read(
                                        &rows[(*read(
                                            &psr.phaseJumpID[(k) as usize],
                                            "psr[p].phaseJumpID[k]",
                                        )?) as usize]
                                            .sat,
                                        "psr[p].obsn[psr[p].phaseJumpID[k]].sat",
                                    )?)
                                    .clone())
                                {
                                    rows[(i) as usize].pulseN = Some(
                                        (*read(
                                            &rows[(i) as usize].pulseN,
                                            "psr[p].obsn[i].pulseN",
                                        )?)
                                        .checked_sub(
                                            (*read(
                                                &psr.phaseJumpDir[(k) as usize],
                                                "psr[p].phaseJumpDir[k]",
                                            )?) as i64,
                                        )
                                        .ok_or_else(
                                            || boundary("phase jump pulse count overflow"),
                                        )?,
                                    );
                                }
                                k += 1;
                            }
                        }
                        if (*read(&rows[(i) as usize].deleted, "psr[p].obsn[i].deleted")? != 1i32) {
                            {
                                mean = ((mean).clone()
                                    + (read(
                                        &rows[(i) as usize].residual,
                                        "psr[p].obsn[i].residual",
                                    )?)
                                    .clone());
                                nmean += 1;
                            }
                        }
                    }
                    i += 1;
                }
            }
            if (removeMean == 1i32) {
                {
                    mean = ((mean).clone() / Float::with_val(PRECISION_BITS, nmean));
                    {
                        i = 0i32;
                        while (i < *read(&psr.nobs, "psr[p].nobs")?) {
                            {
                                rows[(i) as usize].residual = Some(
                                    (read(
                                        &rows[(i) as usize].residual,
                                        "psr[p].obsn[i].residual",
                                    )?)
                                    .clone()
                                        - (mean).clone(),
                                );
                            }
                            i += 1;
                        }
                    }
                }
            }
            // formResiduals.C:2388-2392: omit dead newmean accumulation, which reads
            // uninitialized TNRedSignal and is never consumed. No signal is fabricated.
            if ((*read(&psr.refphs, "psr[p].refphs")?) as i32 == 1i32) {
                {
                    psr.tzrobs = rows[(*read(&psr.nobs, "nobs")? - 1) as usize].clone();
                    psr.nobs = Some(*read(&psr.nobs, "psr[p].nobs")? - 1);
                    d.log(
                        b'm',
                        "formResiduals.C",
                        2400,
                        &[
                            b"SHIFT RESIDUALS BY ".to_vec(),
                            crate::clock_text::format_number(
                                ((read(&psr.tzrobs.residual, "psr[p].tzrobs.residual")?).clone())
                                    .to_f64(),
                                crate::clock_text::NumberFormat::GeneralSix,
                            ),
                            b" (".to_vec(),
                            crate::clock_text::format_number(
                                ((read(&psr.tzrobs.bbat, "psr[p].tzrobs.bbat")?).clone()).to_f64(),
                                crate::clock_text::NumberFormat::GeneralSix,
                            ),
                            b" ".to_vec(),
                            crate::clock_text::format_number(
                                ((read(&psr.tzrobs.sat, "psr[p].tzrobs.sat")?).clone()).to_f64(),
                                crate::clock_text::NumberFormat::GeneralSix,
                            ),
                            b")\n".to_vec(),
                        ]
                        .concat(),
                    );
                    {
                        i = 0i32;
                        while (i < *read(&psr.nobs, "psr[p].nobs")?) {
                            {
                                rows[(i) as usize].residual = Some(
                                    (read(
                                        &rows[(i) as usize].residual,
                                        "psr[p].obsn[i].residual",
                                    )?)
                                    .clone()
                                        - (read(&psr.tzrobs.residual, "psr[p].tzrobs.residual")?)
                                            .clone(),
                                );
                            }
                            i += 1;
                        }
                    }
                }
            }
            if ((((*read(&psr.TNsubtractDM, "psr[p].TNsubtractDM")? == 1i32)
                || (*read(&psr.TNsubtractRed, "psr[p].TNsubtractRed")? == 1i32))
                || (*read(&psr.TNsubtractChrom, "psr[p].TNsubtractChrom")? == 1i32))
                || (*read(&psr.TNsubtractSWGP, "psr[p].TNsubtractSWGP")? == 1i32))
            {
                {
                    {
                        i = 0i32;
                        while (i < *read(&psr.nobs, "psr[p].nobs")?) {
                            {
                                // formResiduals.C:2410,2450: source <= includes a spare/removed TZR row;
                                // both noise loops now cover the active [0,nobs) membership only.
                                rows[(i) as usize].residualtn = Some(
                                    (read(
                                        &rows[(i) as usize].residual,
                                        "psr[p].obsn[i].residual",
                                    )?)
                                    .clone(),
                                );
                                if (*read(&psr.TNsubtractRed, "psr[p].TNsubtractRed")? == 1i32) {
                                    {
                                        rows[(i) as usize].residualtn = Some(
                                            (read(
                                                &rows[(i) as usize].residualtn,
                                                "psr[p].obsn[i].residualtn",
                                            )?)
                                            .clone()
                                                - Float::with_val(
                                                    PRECISION_BITS,
                                                    *read(
                                                        &rows[(i) as usize].TNRedSignal,
                                                        "psr[p].obsn[i].TNRedSignal",
                                                    )?,
                                                ),
                                        );
                                        if (*read(&psr.dmoffsCMnum, "psr[p].dmoffsCMnum")? > 0i32) {
                                            {
                                                rows[(i) as usize].residualtn = Some(
                                                    (read(
                                                        &rows[(i) as usize].residualtn,
                                                        "psr[p].obsn[i].residualtn",
                                                    )?)
                                                    .clone()
                                                        - Float::with_val(
                                                            PRECISION_BITS,
                                                            crate::ifunc::interpolate(
                                                                &psr.dmoffsCM_mjd
                                                                    .iter()
                                                                    .take(
                                                                        (*read(
                                                                            &psr.dmoffsCMnum,
                                                                            "psr[p].dmoffsCMnum",
                                                                        )?)
                                                                            as usize,
                                                                    )
                                                                    .map(|v| {
                                                                        read(v, "dmoffsCM_mjd")
                                                                            .copied()
                                                                    })
                                                                    .collect::<Result<Vec<_>, _>>(
                                                                    )?,
                                                                &psr.dmoffsCM
                                                                    .iter()
                                                                    .take(
                                                                        (*read(
                                                                            &psr.dmoffsCMnum,
                                                                            "psr[p].dmoffsCMnum",
                                                                        )?)
                                                                            as usize,
                                                                    )
                                                                    .map(|v| {
                                                                        read(v, "dmoffsCM").copied()
                                                                    })
                                                                    .collect::<Result<Vec<_>, _>>(
                                                                    )?,
                                                                ((read(
                                                                    &rows[(i) as usize].sat,
                                                                    "psr[p].obsn[i].sat",
                                                                )?)
                                                                .clone())
                                                                .to_f64(),
                                                                *read(
                                                                    &psr.dmoffsCMnum,
                                                                    "psr[p].dmoffsCMnum",
                                                                )?,
                                                                d,
                                                            )?,
                                                        ),
                                                );
                                            }
                                        }
                                    }
                                }
                                if (*read(&psr.TNsubtractDM, "psr[p].TNsubtractDM")? == 1i32) {
                                    {
                                        rows[(i) as usize].residualtn = Some(
                                            (read(
                                                &rows[(i) as usize].residualtn,
                                                "psr[p].obsn[i].residualtn",
                                            )?)
                                            .clone()
                                                - Float::with_val(
                                                    PRECISION_BITS,
                                                    *read(
                                                        &rows[(i) as usize].TNDMSignal,
                                                        "psr[p].obsn[i].TNDMSignal",
                                                    )?,
                                                ),
                                        );
                                    }
                                }
                                if (*read(&psr.TNsubtractSWGP, "psr[p].TNsubtractSWGP")? == 1i32) {
                                    {
                                        rows[(i) as usize].residualtn = Some(
                                            (read(
                                                &rows[(i) as usize].residualtn,
                                                "psr[p].obsn[i].residualtn",
                                            )?)
                                            .clone()
                                                - Float::with_val(
                                                    PRECISION_BITS,
                                                    *read(
                                                        &rows[(i) as usize].SWGPSignal,
                                                        "psr[p].obsn[i].SWGPSignal",
                                                    )?,
                                                ),
                                        );
                                    }
                                }
                                if (*read(&psr.TNsubtractChrom, "psr[p].TNsubtractChrom")? == 1i32)
                                {
                                    {
                                        rows[(i) as usize].residualtn = Some(
                                            (read(
                                                &rows[(i) as usize].residualtn,
                                                "psr[p].obsn[i].residualtn",
                                            )?)
                                            .clone()
                                                - Float::with_val(
                                                    PRECISION_BITS,
                                                    *read(
                                                        &rows[(i) as usize].TNChromSignal,
                                                        "psr[p].obsn[i].TNChromSignal",
                                                    )?,
                                                ),
                                        );
                                    }
                                }
                                if (*read(&rows[(i) as usize].deleted, "psr[p].obsn[i].deleted")?
                                    != 1i32)
                                {
                                    {
                                        tnmean = ((tnmean).clone()
                                            + (read(
                                                &rows[(i) as usize].residualtn,
                                                "psr[p].obsn[i].residualtn",
                                            )?)
                                            .clone());
                                        ntnmean += 1;
                                    }
                                }
                            }
                            i += 1;
                        }
                    }
                    if (removeMean == 1i32) {
                        {
                            tnmean = ((tnmean).clone()
                                / Float::with_val(PRECISION_BITS, (ntnmean) as f64));
                            {
                                i = 0i32;
                                while (i < *read(&psr.nobs, "psr[p].nobs")?) {
                                    {
                                        rows[(i) as usize].residualtn = Some(
                                            (read(
                                                &rows[(i) as usize].residualtn,
                                                "psr[p].obsn[i].residualtn",
                                            )?)
                                            .clone()
                                                - (tnmean).clone(),
                                        );
                                    }
                                    i += 1;
                                }
                            }
                        }
                    }
                }
            }
            if (*read(&psr.AverageResiduals, "psr[p].AverageResiduals")? == 1i32) {
                {
                    // formResiduals.C:2463: call on current pulsar instead of repeating psr[0].
                    crate::averaging::averageResiduals(psr, rows, d)?;
                }
            }
            if (*read(&psr.AverageDMResiduals, "psr[p].AverageDMResiduals")? == 1i32) {
                {
                    // formResiduals.C:2468: the DM average has the same current-pulsar ownership.
                    crate::averaging::averageDMResiduals(psr, rows, d)?;
                }
            }
        }
    }
    d.timing("formResiduals.C", 2454, "Leave formresiduals()".as_bytes());
    Ok(references)
}

fn source_i64_bytes(bytes: &[u8]) -> Result<Option<i64>, TimingError> {
    let start = bytes
        .iter()
        .position(|b| !b.is_ascii_whitespace())
        .unwrap_or(bytes.len());
    let sign = usize::from(bytes.get(start).is_some_and(|b| *b == b'+' || *b == b'-'));
    let count = bytes[start + sign..]
        .iter()
        .take_while(|b| b.is_ascii_digit())
        .count();
    if count == 0 {
        return Ok(None);
    }
    std::str::from_utf8(&bytes[start..start + sign + count])
        .map_err(|_| boundary("pulse_number_scan"))?
        .parse()
        .map(Some)
        .map_err(|_| boundary("pulse_number_range"))
}
