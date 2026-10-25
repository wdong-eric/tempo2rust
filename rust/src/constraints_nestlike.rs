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
    observation::{SourceObservation, read, read_string},
    pulsar::SourcePulsar,
    tempo2::Label::*,
};
/// constraints_nestlike.C::qp_term_cutoff (pinned source line 7).
pub(crate) fn qp_term_cutoff(
    mut freq_days: f64,
    mut logP: f64,
    mut f0: f64,
    mut sig: f64,
    mut lam: f64,
    mut df: f64,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    let mut A: f64 = (10.0f64).powf(logP);
    let mut sigf0: f64 = (f0 * sig).max((0.5f64 * df));
    let mut ret: f64 = (0i32) as f64;
    if (freq_days > (0.5f64 * (f0 - ((f0 * f0) - (((16i32) as f64 * sigf0) * sigf0)).sqrt()))) {
        {
            {
                let mut ih: i32 = 1i32;
                while (ih <= 10i32) {
                    {
                        ret = (ret
                            + (((A * ((-(ih - 1i32)) as f64 / lam).exp())
                                * ((-(freq_days - (f0 * (ih) as f64)).powf((2i32) as f64))
                                    / ((2i32) as f64
                                        * ((ih) as f64 * sigf0).powf((2i32) as f64)))
                                .exp())
                                / (ih) as f64));
                    }
                    ih = (ih + 1);
                }
            }
        }
    }
    return Ok(ret);
}

/// constraints_nestlike.C::constraints_nestlike_red (pinned source line 23).
pub(crate) fn constraints_nestlike_red(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut iconstraint: i32,
    mut iparam: i32,
    mut constraintk: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !((iconstraint == (crate::pulsar::Constraint::constraint_red_sin as i32))
        || (iconstraint == (crate::pulsar::Constraint::constraint_red_cos as i32)))
    {
        return Err(boundary(
            "constraints_nestlike.C::constraints_nestlike_red: assertion",
        ));
    }
    if ((constraintk == k)
        && (((iconstraint == (crate::pulsar::Constraint::constraint_red_sin as i32))
            && (iparam == (param_red_sin as i32)))
            || ((iconstraint == (crate::pulsar::Constraint::constraint_red_cos as i32))
                && (iparam == (param_red_cos as i32)))))
    {
        {
            let mut maxtspan: f64 =
                ((psr.param.slots[(param_finish as i32) as usize].cells[(0i32) as usize].val)
                    .clone()
                    - (psr.param.slots[(param_start as i32) as usize].cells[(0i32) as usize].val)
                        .clone())
                .to_f64();
            let mut RedFLow: f64 = (10.0f64).powf(*read(&psr.TNRedFLow, "psr[p].TNRedFLow")?);
            let mut freq0: f64 = (RedFLow * (1.0f64 / maxtspan));
            let mut freq: f64 = (freq0 * ((k) as f64 + 1.0f64));
            let mut df: f64 = (RedFLow / (maxtspan * 86400.0f64));
            let mut RedAmp: f64 = (10.0f64).powf(*read(&psr.TNRedAmp, "psr[p].TNRedAmp")?);
            let mut RedIndex: f64 = *read(&psr.TNRedGam, "psr[p].TNRedGam")?;
            let mut RedAmp2: f64 = (10.0f64).powf(*read(&psr.TNRedAmp2, "psr[p].TNRedAmp2")?);
            let mut RedIndex2: f64 = *read(&psr.TNRedGam2, "psr[p].TNRedGam2")?;
            let mut have2powerlaw: bool =
                (*read(&psr.TNRedAmp2, "psr[p].TNRedAmp2")? != (0i32) as f64);
            if ((k == 0i32) && (*read(&psr.TNRed_log_freqs, "psr[p].TNRed_log_freqs")? > 0i32)) {
                {
                    df = ((freq
                        - (freq0
                            * (*read(&psr.TNRed_log_factor, "psr[p].TNRed_log_factor")?)
                                .powf((-1i32) as f64)))
                        / 86400.0f64);
                }
            }
            if (k >= *read(&psr.TNRedC, "psr[p].TNRedC")?) {
                {
                    let mut subharm: i32 = ((k - *read(&psr.TNRedC, "psr[p].TNRedC")?) + 1i32);
                    freq = (freq0
                        * (*read(&psr.TNRed_log_factor, "psr[p].TNRed_log_factor")?)
                            .powf((-subharm) as f64));
                    if (subharm == *read(&psr.TNRed_log_freqs, "psr[p].TNRed_log_freqs")?) {
                        {
                            df = (freq / 86400.0f64);
                        }
                    } else {
                        {
                            df = ((freq
                                - (freq0
                                    * (*read(&psr.TNRed_log_factor, "psr[p].TNRed_log_factor")?)
                                        .powf(((-subharm) - 1i32) as f64)))
                                / 86400.0f64);
                        }
                    }
                }
            }
            let mut rho: f64;
            if (*read(&psr.TNRedCorner, "psr[p].TNRedCorner")? > (0i32) as f64) {
                {
                    let mut RedCorner: f64 =
                        (*read(&psr.TNRedCorner, "psr[p].TNRedCorner")? / maxtspan);
                    rho = ((((((1i32) as f64
                        + ((1.0f64 / 365.25f64) / RedCorner).powf((RedIndex / (2i32) as f64)))
                    .powf((2i32) as f64)
                        * (((RedAmp * RedAmp) / 12.0f64)
                            / (std::f64::consts::PI * std::f64::consts::PI)))
                        / ((1i32) as f64 + (freq / RedCorner).powf((RedIndex / (2i32) as f64)))
                            .powf((2i32) as f64))
                        * df)
                        * (1.0f64 / 3.16e7f64).powf((-3.0f64)));
                    if have2powerlaw {
                        {
                            rho = (rho
                                + ((((((1i32) as f64
                                    + ((1.0f64 / 365.25f64) / RedCorner)
                                        .powf((RedIndex2 / (2i32) as f64)))
                                .powf((2i32) as f64)
                                    * (((RedAmp2 * RedAmp2) / 12.0f64)
                                        / (std::f64::consts::PI * std::f64::consts::PI)))
                                    / ((1i32) as f64
                                        + (freq / RedCorner).powf((RedIndex2 / (2i32) as f64)))
                                    .powf((2i32) as f64))
                                    * df)
                                    * (1.0f64 / 3.16e7f64).powf((-3.0f64))));
                        }
                    }
                }
            } else {
                {
                    rho = ((((((RedAmp * RedAmp) / 12.0f64)
                        / (std::f64::consts::PI * std::f64::consts::PI))
                        * (1.0f64 / 3.16e7f64).powf((-3.0f64)))
                        * (freq * 365.25f64).powf((-RedIndex)))
                        * df);
                    if have2powerlaw {
                        {
                            rho = (rho
                                + ((((((RedAmp2 * RedAmp2) / 12.0f64)
                                    / (std::f64::consts::PI * std::f64::consts::PI))
                                    * (1.0f64 / 3.16e7f64).powf((-3.0f64)))
                                    * (freq * 365.25f64).powf((-RedIndex2)))
                                    * df));
                        }
                    }
                }
            }
            if (*read(&psr.TN_QpPeriod, "psr[p].TN_QpPeriod")? > (0i32) as f64) {
                {
                    let mut qp_f0: f64 = (1.0f64 / *read(&psr.TN_QpPeriod, "psr[p].TN_QpPeriod")?);
                    let mut logPqp: f64 = (*read(&psr.TN_QpRatio, "psr[p].TN_QpRatio")?
                        + ((((((RedAmp * RedAmp) / 12.0f64)
                            / (std::f64::consts::PI * std::f64::consts::PI))
                            * (1.0f64 / 3.16e7f64).powf((-3.0f64)))
                            * (qp_f0 * 365.25f64).powf((-RedIndex)))
                            * df)
                            .log10());
                    if have2powerlaw {
                        {
                            logPqp = (logPqp
                                + ((((((RedAmp2 * RedAmp2) / 12.0f64)
                                    / (std::f64::consts::PI * std::f64::consts::PI))
                                    * (1.0f64 / 3.16e7f64).powf((-3.0f64)))
                                    * (qp_f0 * 365.25f64).powf((-RedIndex2)))
                                    * df)
                                    .log10());
                        }
                    }
                    let mut qp: f64 = ((freq / qp_f0).powf((-4i32) as f64)
                        * qp_term_cutoff(
                            freq,
                            logPqp,
                            qp_f0,
                            *read(&psr.TN_QpSig, "psr[p].TN_QpSig")?,
                            *read(&psr.TN_QpLam, "psr[p].TN_QpLam")?,
                            df,
                            d,
                        )?);
                    rho = (rho + qp);
                }
            }
            return Ok((1.0f64 / (rho).sqrt()));
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints_nestlike.C::constraints_nestlike_red_dm (pinned source line 105).
pub(crate) fn constraints_nestlike_red_dm(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut iconstraint: i32,
    mut iparam: i32,
    mut constraintk: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !((iconstraint == (crate::pulsar::Constraint::constraint_red_dm_sin as i32))
        || (iconstraint == (crate::pulsar::Constraint::constraint_red_dm_cos as i32)))
    {
        return Err(boundary(
            "constraints_nestlike.C::constraints_nestlike_red_dm: assertion",
        ));
    }
    if ((constraintk == k)
        && (((iconstraint == (crate::pulsar::Constraint::constraint_red_dm_sin as i32))
            && (iparam == (param_red_dm_sin as i32)))
            || ((iconstraint == (crate::pulsar::Constraint::constraint_red_dm_cos as i32))
                && (iparam == (param_red_dm_cos as i32)))))
    {
        {
            let mut maxtspan: f64 =
                ((psr.param.slots[(param_finish as i32) as usize].cells[(0i32) as usize].val)
                    .clone()
                    - (psr.param.slots[(param_start as i32) as usize].cells[(0i32) as usize].val)
                        .clone())
                .to_f64();
            let mut DMAmp: f64 = (10.0f64).powf(*read(&psr.TNDMAmp, "psr[p].TNDMAmp")?);
            let mut freq: f64 = (((k) as f64 + 1.0f64) / maxtspan);
            let mut DMFLow: f64 = 1.0f64;
            let mut freq0: f64 = (DMFLow * (1.0f64 / maxtspan));
            let mut DMIndex: f64 = *read(&psr.TNDMGam, "psr[p].TNDMGam")?;
            let mut df: f64 = (freq0 / 86400.0f64);
            if ((k == 0i32) && (*read(&psr.TNDM_log_freqs, "psr[p].TNDM_log_freqs")? > 0i32)) {
                {
                    df = ((freq
                        - (freq0
                            * (*read(&psr.TNDM_log_factor, "psr[p].TNDM_log_factor")?)
                                .powf((-1i32) as f64)))
                        / 86400.0f64);
                }
            }
            if (k >= *read(&psr.TNDMC, "psr[p].TNDMC")?) {
                {
                    let mut subharm: i32 = ((k - *read(&psr.TNDMC, "psr[p].TNDMC")?) + 1i32);
                    freq = (freq0
                        * (*read(&psr.TNDM_log_factor, "psr[p].TNDM_log_factor")?)
                            .powf((-subharm) as f64));
                    if (subharm == *read(&psr.TNDM_log_freqs, "psr[p].TNDM_log_freqs")?) {
                        {
                            df = (freq / 86400.0f64);
                        }
                    } else {
                        {
                            df = ((freq
                                - (freq0
                                    * (*read(&psr.TNDM_log_factor, "psr[p].TNDM_log_factor")?)
                                        .powf(((-subharm) - 1i32) as f64)))
                                / 86400.0f64);
                        }
                    }
                }
            }
            let mut rho: f64 = ((((DMAmp * DMAmp) * (1.0f64 / 3.16e7f64).powf((-3i32) as f64))
                * (freq * 365.25f64).powf((-DMIndex)))
                * df);
            return Ok((1.0f64 / (rho).sqrt()));
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints_nestlike.C::constraints_nestlike_red_chrom (pinned source line 150).
pub(crate) fn constraints_nestlike_red_chrom(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut iconstraint: i32,
    mut iparam: i32,
    mut constraintk: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !((iconstraint == (crate::pulsar::Constraint::constraint_red_chrom_sin as i32))
        || (iconstraint == (crate::pulsar::Constraint::constraint_red_chrom_cos as i32)))
    {
        return Err(boundary(
            "constraints_nestlike.C::constraints_nestlike_red_chrom: assertion",
        ));
    }
    if ((constraintk == k)
        && (((iconstraint == (crate::pulsar::Constraint::constraint_red_chrom_sin as i32))
            && (iparam == (param_red_chrom_sin as i32)))
            || ((iconstraint == (crate::pulsar::Constraint::constraint_red_chrom_cos as i32))
                && (iparam == (param_red_chrom_cos as i32)))))
    {
        {
            let mut maxtspan: f64 =
                ((psr.param.slots[(param_finish as i32) as usize].cells[(0i32) as usize].val)
                    .clone()
                    - (psr.param.slots[(param_start as i32) as usize].cells[(0i32) as usize].val)
                        .clone())
                .to_f64();
            let mut ChromAmp: f64 = (10.0f64).powf(*read(&psr.TNChromAmp, "psr[p].TNChromAmp")?);
            let mut freq: f64 = (((k) as f64 + 1.0f64) / maxtspan);
            let mut ChromIndex: f64 = *read(&psr.TNChromGam, "psr[p].TNChromGam")?;
            let mut ChromFLow: f64 = 1.0f64;
            let mut freq0: f64 = (ChromFLow * (1.0f64 / maxtspan));
            let mut df: f64 = (freq0 / 86400.0f64);
            if ((k == 0i32) && (*read(&psr.TNChrom_log_freqs, "psr[p].TNChrom_log_freqs")? > 0i32))
            {
                {
                    df = ((freq
                        - (freq0
                            * (*read(&psr.TNChrom_log_factor, "psr[p].TNChrom_log_factor")?)
                                .powf((-1i32) as f64)))
                        / 86400.0f64);
                }
            }
            if (k >= *read(&psr.TNChromC, "psr[p].TNChromC")?) {
                {
                    let mut subharm: i32 = ((k - *read(&psr.TNChromC, "psr[p].TNChromC")?) + 1i32);
                    freq = (freq0
                        * (*read(&psr.TNChrom_log_factor, "psr[p].TNChrom_log_factor")?)
                            .powf((-subharm) as f64));
                    if (subharm == *read(&psr.TNChrom_log_freqs, "psr[p].TNChrom_log_freqs")?) {
                        {
                            df = (freq / 86400.0f64);
                        }
                    } else {
                        {
                            df = ((freq
                                - (freq0
                                    * (*read(
                                        &psr.TNChrom_log_factor,
                                        "psr[p].TNChrom_log_factor",
                                    )?)
                                    .powf(((-subharm) - 1i32) as f64)))
                                / 86400.0f64);
                        }
                    }
                }
            }
            let mut rho: f64 = (((((((ChromAmp * ChromAmp) / 12.0f64) / std::f64::consts::PI)
                / std::f64::consts::PI)
                * (1.0f64 / 3.16e7f64).powf((-3i32) as f64))
                * (freq * 365.25f64).powf((-ChromIndex)))
                * df);
            return Ok((1.0f64 / (rho).sqrt()));
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints_nestlike.C::constraints_nestlike_swgp (pinned source line 199).
pub(crate) fn constraints_nestlike_swgp(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut iconstraint: i32,
    mut iparam: i32,
    mut constraintk: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !((iconstraint == (crate::pulsar::Constraint::constraint_swgp_sin as i32))
        || (iconstraint == (crate::pulsar::Constraint::constraint_swgp_cos as i32)))
    {
        return Err(boundary(
            "constraints_nestlike.C::constraints_nestlike_swgp: assertion",
        ));
    }
    if ((constraintk == k)
        && (((iconstraint == (crate::pulsar::Constraint::constraint_swgp_sin as i32))
            && (iparam == (param_swgp_sin as i32)))
            || ((iconstraint == (crate::pulsar::Constraint::constraint_swgp_cos as i32))
                && (iparam == (param_swgp_cos as i32)))))
    {
        {
            let mut maxtspan: f64 =
                ((psr.param.slots[(param_finish as i32) as usize].cells[(0i32) as usize].val)
                    .clone()
                    - (psr.param.slots[(param_start as i32) as usize].cells[(0i32) as usize].val)
                        .clone())
                .to_f64();
            let mut SWAmp: f64 = (10.0f64).powf(*read(&psr.SWGPAmp, "psr[p].SWGPAmp")?);
            let mut freq: f64 = (((k) as f64 + 1.0f64) / maxtspan);
            let mut SWIndex: f64 = *read(&psr.SWGPGam, "psr[p].SWGPGam")?;
            let mut df: f64 = ((1.0f64 / maxtspan) / 86400.0f64);
            let mut rho: f64 = (((((((SWAmp * SWAmp) / 12.0f64) / std::f64::consts::PI)
                / std::f64::consts::PI)
                * (1.0f64 / 3.16e7f64).powf((-3i32) as f64))
                * (freq * 365.25f64).powf((-SWIndex)))
                * df);
            return Ok((1.0f64 / (rho).sqrt()));
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints_nestlike.C::constraints_nestlike_jitter (pinned source line 231).
pub(crate) fn constraints_nestlike_jitter(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut iconstraint: i32,
    mut iparam: i32,
    mut constraintk: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !(iconstraint == (crate::pulsar::Constraint::constraint_jitter as i32)) {
        return Err(boundary(
            "constraints_nestlike.C::constraints_nestlike_jitter: assertion",
        ));
    }
    if ((iparam == (param_jitter as i32)) && (constraintk == k)) {
        {
            let mut ecorrval: f64 = (0i32) as f64;
            let mut iobs: i32 = k;
            {
                let mut iecorr: i32 = 0i32;
                while (iecorr < *read(&psr.nTNECORR, "psr[p].nTNECORR")?) {
                    {
                        {
                            let mut iflag: i32 = 0i32;
                            while (iflag
                                < *read(&rows[(iobs) as usize].nFlags, "psr[p].obsn[iobs].nFlags")?)
                            {
                                {
                                    if (((i32::from(
                                        read_string(
                                            &rows[(iobs) as usize].flagID[(iflag) as usize][..],
                                            "string",
                                        )? != read_string(
                                            &psr.TNECORRFlagID[(iecorr) as usize][..],
                                            "string",
                                        )?,
                                    )) == 0i32)
                                        && ((i32::from(
                                            read_string(
                                                &rows[(iobs) as usize].flagVal[(iflag) as usize][..],
                                                "string",
                                            )? != read_string(
                                                &psr.TNECORRFlagVal[(iecorr) as usize][..],
                                                "string",
                                            )?,
                                        )) == 0i32))
                                    {
                                        {
                                            ecorrval = *read(
                                                &psr.TNECORRVal[(iecorr) as usize],
                                                "psr[p].TNECORRVal[iecorr]",
                                            )?;
                                            break;
                                        }
                                    }
                                    if (ecorrval > (0i32) as f64) {
                                        break;
                                    }
                                }
                                iflag = (iflag + 1);
                            }
                        }
                    }
                    iecorr = (iecorr + 1);
                }
            }
            {
                let mut iecorr: i32 = 0i32;
                while (iecorr < *read(&psr.nTNSECORR, "psr[p].nTNSECORR")?) {
                    {
                        {
                            let mut iflag: i32 = 0i32;
                            while (iflag
                                < *read(&rows[(iobs) as usize].nFlags, "psr[p].obsn[iobs].nFlags")?)
                            {
                                {
                                    if (((i32::from(
                                        read_string(
                                            &rows[(iobs) as usize].flagID[(iflag) as usize][..],
                                            "string",
                                        )? != read_string(
                                            &psr.TNSECORRFlagID[(iecorr) as usize][..],
                                            "string",
                                        )?,
                                    )) == 0i32)
                                        && ((i32::from(
                                            read_string(
                                                &rows[(iobs) as usize].flagVal[(iflag) as usize][..],
                                                "string",
                                            )? != read_string(
                                                &psr.TNSECORRFlagVal[(iecorr) as usize][..],
                                                "string",
                                            )?,
                                        )) == 0i32))
                                    {
                                        {
                                            let mut tobsval: f64 = *read(
                                                &rows[(iobs) as usize].tobs,
                                                "psr[p].obsn[iobs].tobs",
                                            )?;
                                            ecorrval = (*read(
                                                &psr.TNSECORRVal[(iecorr) as usize],
                                                "psr[p].TNSECORRVal[iecorr]",
                                            )? / (tobsval / 3600.0f64).sqrt());
                                            d.stderr(
                                                format!(
                                                    "TOBS ERR TNSECORR  {} {} {}\n",
                                                    tobsval,
                                                    ecorrval,
                                                    *read(
                                                        &psr.TNSECORRVal[(iecorr) as usize],
                                                        "psr[p].TNSECORRVal[iecorr]"
                                                    )?
                                                )
                                                .as_bytes(),
                                            );
                                            break;
                                        }
                                    }
                                    if (ecorrval > (0i32) as f64) {
                                        break;
                                    }
                                }
                                iflag = (iflag + 1);
                            }
                        }
                    }
                    iecorr = (iecorr + 1);
                }
            }
            return Ok((1e6f64 / ecorrval));
        }
    } else {
        {
            return Ok((0i32) as f64);
        }
    }
}

/// constraints_nestlike.C::constraints_nestlike_band (pinned source line 298).
pub(crate) fn constraints_nestlike_band(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut iconstraint: i32,
    mut iparam: i32,
    mut constraintk: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !((iconstraint == (crate::pulsar::Constraint::constraint_band_red_sin as i32))
        || (iconstraint == (crate::pulsar::Constraint::constraint_band_red_cos as i32)))
    {
        return Err(boundary(
            "constraints_nestlike.C::constraints_nestlike_band: assertion",
        ));
    }
    if ((constraintk == k)
        && (((iconstraint == (crate::pulsar::Constraint::constraint_band_red_sin as i32))
            && (iparam == (param_band_red_sin as i32)))
            || ((iconstraint == (crate::pulsar::Constraint::constraint_band_red_cos as i32))
                && (iparam == (param_band_red_cos as i32)))))
    {
        {
            let mut iband: i32 = 0i32;
            let mut ichan: i32 = k;
            while (ichan
                >= *read(
                    &psr.TNBandNoiseC[(iband) as usize],
                    "psr[p].TNBandNoiseC[iband]",
                )?)
            {
                ichan = (ichan
                    - *read(
                        &psr.TNBandNoiseC[(iband) as usize],
                        "psr[p].TNBandNoiseC[iband]",
                    )?);
                iband = (iband + 1);
            }
            let mut BandAmp: f64 = (10.0f64).powf(*read(
                &psr.TNBandNoiseAmp[(iband) as usize],
                "psr[p].TNBandNoiseAmp[iband]",
            )?);
            let mut BandSpec: f64 = *read(
                &psr.TNBandNoiseGam[(iband) as usize],
                "psr[p].TNBandNoiseGam[iband]",
            )?;
            let mut maxtspan: f64 =
                ((psr.param.slots[(param_finish as i32) as usize].cells[(0i32) as usize].val)
                    .clone()
                    - (psr.param.slots[(param_start as i32) as usize].cells[(0i32) as usize].val)
                        .clone())
                .to_f64();
            let mut freq: f64 = (((ichan) as f64 + 1.0f64) / maxtspan);
            let mut rho: f64 = ((((((BandAmp * BandAmp) / 12.0f64)
                / (std::f64::consts::PI * std::f64::consts::PI))
                * (1.0f64 / 3.16e7f64).powf((-3i32) as f64))
                * (freq * 365.25f64).powf((-BandSpec)))
                / (((maxtspan * (24i32) as f64) * (60i32) as f64) * (60i32) as f64));
            return Ok((1.0f64 / (rho).sqrt()));
        }
    } else {
        return Ok((0i32) as f64);
    }
}

/// constraints_nestlike.C::constraints_nestlike_group (pinned source line 343).
pub(crate) fn constraints_nestlike_group(
    psr: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    mut ipsr: i32,
    mut iconstraint: i32,
    mut iparam: i32,
    mut constraintk: i32,
    mut k: i32,
    d: &mut Diagnostics,
) -> Result<f64, TimingError> {
    if !((iconstraint == (crate::pulsar::Constraint::constraint_group_red_sin as i32))
        || (iconstraint == (crate::pulsar::Constraint::constraint_group_red_cos as i32)))
    {
        return Err(boundary(
            "constraints_nestlike.C::constraints_nestlike_group: assertion",
        ));
    }
    if ((constraintk == k)
        && (((iconstraint == (crate::pulsar::Constraint::constraint_group_red_sin as i32))
            && (iparam == (param_group_red_sin as i32)))
            || ((iconstraint == (crate::pulsar::Constraint::constraint_group_red_cos as i32))
                && (iparam == (param_group_red_cos as i32)))))
    {
        {
            let mut igroup: i32 = 0i32;
            let mut ichan: i32 = k;
            while (ichan
                >= *read(
                    &psr.TNGroupNoiseC[(igroup) as usize],
                    "psr[p].TNGroupNoiseC[igroup]",
                )?)
            {
                ichan = (ichan
                    - *read(
                        &psr.TNGroupNoiseC[(igroup) as usize],
                        "psr[p].TNGroupNoiseC[igroup]",
                    )?);
                igroup = (igroup + 1);
            }
            let mut GroupAmp: f64 = (10.0f64).powf(*read(
                &psr.TNGroupNoiseAmp[(igroup) as usize],
                "psr[p].TNGroupNoiseAmp[igroup]",
            )?);
            let mut GroupSpec: f64 = *read(
                &psr.TNGroupNoiseGam[(igroup) as usize],
                "psr[p].TNGroupNoiseGam[igroup]",
            )?;
            let mut maxtspan: f64 =
                ((psr.param.slots[(param_finish as i32) as usize].cells[(0i32) as usize].val)
                    .clone()
                    - (psr.param.slots[(param_start as i32) as usize].cells[(0i32) as usize].val)
                        .clone())
                .to_f64();
            let mut freq: f64 = (((ichan) as f64 + 1.0f64) / maxtspan);
            let mut rho: f64 = ((((((GroupAmp * GroupAmp) / 12.0f64)
                / (std::f64::consts::PI * std::f64::consts::PI))
                * (1.0f64 / 3.16e7f64).powf((-3i32) as f64))
                * (freq * 365.25f64).powf((-GroupSpec)))
                / (((maxtspan * (24i32) as f64) * (60i32) as f64) * (60i32) as f64));
            return Ok((1.0f64 / (rho).sqrt()));
        }
    } else {
        return Ok((0i32) as f64);
    }
}
