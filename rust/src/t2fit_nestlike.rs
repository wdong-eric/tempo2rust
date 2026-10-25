//! t2fit_nestlike.C and packed-polynomial helpers from t2fit_nestlike.h.
#![allow(non_snake_case)]
use crate::pulsar_lifecycle::{LifecycleBoundary, fail};
pub const T2_TNSUBPOLY_FLAG_POSTFIT: u64 = 1;
pub const T2_TNSUBPOLY_FLAG_NESTLIKE: u64 = 2;
pub fn t2ClampSubPolyOrder(order: i32) -> i32 {
    order.clamp(0, 3)
}
pub fn t2DecodeTNSubPolyOrdStored(packed: u64, shift: i32) -> Result<i32, LifecycleBoundary> {
    if !(0..64).contains(&shift) {
        return Err(fail("t2DecodeTNSubPolyOrdStored", "shift", "invalid_shift"));
    }
    Ok(t2ClampSubPolyOrder(((packed >> shift) & 0xff) as i32))
}
pub fn t2EncodeTNsubtractPoly(flags: u64, red: i32, dm: i32, chrom: i32) -> u64 {
    (flags & 0xff)
        | ((t2ClampSubPolyOrder(red) as u64 & 0xff) << 8)
        | ((t2ClampSubPolyOrder(dm) as u64 & 0xff) << 16)
        | ((t2ClampSubPolyOrder(chrom) as u64 & 0xff) << 24)
}
pub fn t2DecodeTNsubtractPolyFlags(packed: u64) -> u64 {
    packed & 0xff
}
pub fn t2DecodeTNRedSubPolyOrd(packed: u64) -> i32 {
    t2DecodeTNSubPolyOrdStored(packed, 8).unwrap()
}
pub fn t2DecodeTNDMSubPolyOrd(packed: u64) -> i32 {
    t2DecodeTNSubPolyOrdStored(packed, 16).unwrap()
}
pub fn t2DecodeTNChromSubPolyOrd(packed: u64) -> i32 {
    t2DecodeTNSubPolyOrdStored(packed, 24).unwrap()
}
pub fn t2SetTNRedSubPolyOrd(packed: u64, order: i32) -> u64 {
    (packed & !(0xff << 8)) | ((t2ClampSubPolyOrder(order) as u64 & 0xff) << 8)
}
pub fn t2SetTNDMSubPolyOrd(packed: u64, order: i32) -> u64 {
    (packed & !(0xff << 16)) | ((t2ClampSubPolyOrder(order) as u64 & 0xff) << 16)
}
pub fn t2SetTNChromSubPolyOrd(packed: u64, order: i32) -> u64 {
    (packed & !(0xff << 24)) | ((t2ClampSubPolyOrder(order) as u64 & 0xff) << 24)
}

use crate::{
    TimingError,
    clock_diagnostics::boundary,
    observation::{SourceObservation, read, read_string},
    pulsar::{MAX_TNShapeCoef, SourcePulsar},
    tempo2::{Label, Label::*},
};
use std::f64::consts::PI;

pub fn legendre(x: f64, n: usize) -> Result<f64, TimingError> {
    Ok(match n {
        0 => 1.0,
        1 => x,
        2 => 0.5 * (3.0 * x).mul_add(x, -1.0),
        3 => 0.5 * (5.0 * x * x).mul_add(x, -3.0 * x),
        _ => return Err(boundary("legendre: order exceeds cubic")),
    })
}
/// computePolySubtractCoeffs: source normal equations and pivot order.
/// Explicit mul_add matches GCC16 arm64 O2 fmadd/fmsub in legendre, ATA/ATf,
/// elimination and projection. The otherwise identical -ffp-contract=off
/// witness reproduces the unfused Rust matrix exactly but changes this highly
/// scaled fit by 24 ps; validation/general_fitting records both builds.
#[allow(clippy::needless_range_loop, clippy::too_many_arguments)]
pub fn polynomial_coefficients(
    rows: &[SourceObservation],
    freq: f64,
    is_cos: bool,
    alpha: f64,
    beta: f64,
    epoch: f64,
    max_order: i32,
) -> Result<[f64; 4], TimingError> {
    let mut c = [0.0; 4];
    if rows.len() < 2 || alpha == 0.0 {
        return Ok(c);
    }
    let n = t2ClampSubPolyOrder(max_order) as usize + 1;
    let mut a = [[0.0; 4]; 4];
    let mut b = [0.0; 4];
    for o in rows {
        let x = (read(&o.bbat, "bbat")?.clone() - epoch).to_f64();
        let xi = (x - beta) / alpha;
        let angle = 2.0 * PI * freq * x;
        let f = if is_cos { angle.cos() } else { angle.sin() };
        let mut p = [0.0; 4];
        for j in 0..n {
            p[j] = legendre(xi, j)?;
        }
        for j in 0..n {
            b[j] = p[j].mul_add(f, b[j]);
            for l in 0..n {
                a[j][l] = p[j].mul_add(p[l], a[j][l]);
            }
        }
    }
    let mut aug = [[0.0; 5]; 4];
    for j in 0..n {
        aug[j][..n].copy_from_slice(&a[j][..n]);
        aug[j][n] = b[j];
    }
    for col in 0..n {
        let mut pivot = col;
        for row in col + 1..n {
            if aug[row][col].abs() > aug[pivot][col].abs() {
                pivot = row;
            }
        }
        aug.swap(col, pivot);
        if aug[col][col].abs() < 1e-14 {
            continue;
        }
        let inv = 1.0 / aug[col][col];
        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = aug[row][col] * inv;
            for l in col..=n {
                aug[row][l] = (-factor).mul_add(aug[col][l], aug[row][l]);
            }
        }
        for l in col + 1..=n {
            aug[col][l] *= inv;
        }
        aug[col][col] = 1.0;
    }
    for j in 0..n {
        c[j] = aug[j][n];
    }
    Ok(c)
}

pub fn is_noise(label: Label) -> bool {
    matches!(
        label,
        param_red_sin
            | param_red_cos
            | param_red_dm_sin
            | param_red_dm_cos
            | param_red_chrom_sin
            | param_red_chrom_cos
            | param_swgp_sin
            | param_swgp_cos
            | param_band_red_sin
            | param_band_red_cos
            | param_group_red_sin
            | param_group_red_cos
            | param_jitter
            | param_shapevent
    )
}
/// All nestlike derivative families; special shapelet scale follows the dispatcher.
pub fn derivative(
    p: &SourcePulsar,
    rows: &[SourceObservation],
    x: f64,
    i: usize,
    label: Label,
    k: usize,
) -> Result<f64, TimingError> {
    if label == param_jitter {
        let dt = 1.0 / 86400.0;
        let epoch = read(&rows[k].bat, "bat")?.to_f64();
        let t = read(&rows[i].bat, "bat")?.to_f64();
        return Ok(if t > epoch - dt && t < epoch + dt {
            1.0
        } else {
            0.0
        });
    }
    if label == param_shapevent {
        let event = k / MAX_TNShapeCoef;
        let coef = k % MAX_TNShapeCoef;
        let mut coefs = vec![0.0; coef + 1];
        coefs[coef] = 1.0;
        let shape = crate::shapelet::evaluate_shapelet(
            coefs.len() as i32,
            *read(&p.TNShapeletEvPos[event], "shape position")?,
            *read(&p.TNShapeletEvWidth[event], "shape width")?,
            &coefs,
            read(&rows[i].bat, "bat")?.to_f64(),
        )?;
        return match *read(&p.TNShapeletEvFScale[event], "shape scale")? {
            0.0 => Ok(shape),
            2.0 => Ok(shape * (*read(&rows[i].freqSSB, "freqSSB")?).powf(-2.0) * 1e12 / 2.41e-4),
            _ => Err(boundary(
                "shapelet fit: source derivative pointer unwritten for this scale",
            )),
        };
    }
    let span = (p.param.slots[param_finish as usize].cells[0].val.clone()
        - &p.param.slots[param_start as usize].cells[0].val)
        .to_f64();
    let is_cos = matches!(
        label,
        param_red_cos
            | param_red_dm_cos
            | param_red_chrom_cos
            | param_swgp_cos
            | param_band_red_cos
            | param_group_red_cos
    );
    let mut ordinal = k;
    let mut projection = None;
    let freq = match label {
        param_red_sin | param_red_cos | param_red_dm_sin | param_red_dm_cos
        | param_red_chrom_sin | param_red_chrom_cos => {
            let packed = *read(&p.TNsubtractPoly, "TNsubtractPoly")?;
            let (n, flow, log_factor, order) = if matches!(label, param_red_sin | param_red_cos) {
                (
                    *read(&p.TNRedC, "TNRedC")?,
                    10.0f64.powf(*read(&p.TNRedFLow, "TNRedFLow")?),
                    *read(&p.TNRed_log_factor, "TNRed_log_factor")?,
                    t2DecodeTNRedSubPolyOrd(packed),
                )
            } else if matches!(label, param_red_dm_sin | param_red_dm_cos) {
                (
                    *read(&p.TNDMC, "TNDMC")?,
                    1.0,
                    *read(&p.TNDM_log_factor, "TNDM_log_factor")?,
                    t2DecodeTNDMSubPolyOrd(packed),
                )
            } else {
                (
                    *read(&p.TNChromC, "TNChromC")?,
                    1.0,
                    *read(&p.TNChrom_log_factor, "TNChrom_log_factor")?,
                    t2DecodeTNChromSubPolyOrd(packed),
                )
            };
            if packed & 2 != 0 {
                projection = Some(order);
            }
            if k as i32 >= n {
                flow * (1.0 / span) * log_factor.powf(-((k as i32 - n + 1) as f64))
            } else {
                flow * (k as f64 + 1.0) / span
            }
        }
        param_band_red_sin | param_band_red_cos | param_group_red_sin | param_group_red_cos => {
            let band = matches!(label, param_band_red_sin | param_band_red_cos);
            let counts = if band {
                &p.TNBandNoiseC[..]
            } else {
                &p.TNGroupNoiseC[..]
            };
            let mut group = 0;
            while ordinal
                >= *read(
                    counts
                        .get(group)
                        .ok_or_else(|| boundary("noise coefficient capacity"))?,
                    "noise count",
                )? as usize
            {
                ordinal -= *read(&counts[group], "noise count")? as usize;
                group += 1;
            }
            if band {
                let f = *read(&rows[i].freq, "freq")?;
                if !(f > *read(&p.TNBandNoiseLF[group], "band low")?
                    && f < *read(&p.TNBandNoiseHF[group], "band high")?)
                {
                    return Ok(0.0);
                }
            } else {
                let id = read_string(&p.TNGroupNoiseFlagID[group], "group flag")?;
                let value = read_string(&p.TNGroupNoiseFlagVal[group], "group value")?;
                let mut matched = false;
                for j in 0..*read(&rows[i].nFlags, "nFlags")? as usize {
                    if read_string(&rows[i].flagID[j], "flagID")? == id
                        && read_string(&rows[i].flagVal[j], "flagVal")? == value
                    {
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    return Ok(0.0);
                }
            }
            (ordinal as f64 + 1.0) / span
        }
        param_swgp_sin | param_swgp_cos => (k as f64 + 1.0) / span,
        _ => return Err(boundary("nestlike derivative label")),
    };
    let angle = 2.0 * PI * freq * x;
    let mut trig = if is_cos { angle.cos() } else { angle.sin() };
    if let Some(order) = projection {
        let epoch = p.param.slots[param_pepoch as usize].cells[0].val.to_f64();
        let a = (p.param.slots[param_start as usize].cells[0].val.clone() - epoch).to_f64();
        let b = (p.param.slots[param_finish as usize].cells[0].val.clone() - epoch).to_f64();
        let alpha = (b - a) / 2.0;
        let beta = (b + a) / 2.0;
        // t2fit_nestlike.C's static cache omitted interior BBATs and frequency.
        // Recompute from current canonical state; check_cache.py demonstrates
        // unchanged cold calls and corrected repeat/context-dependent values.
        let coeffs = polynomial_coefficients(
            &rows[..*read(&p.nobs, "nobs")? as usize],
            freq,
            is_cos,
            alpha,
            beta,
            epoch,
            order,
        )?;
        if alpha != 0.0 {
            for (n, c) in coeffs.iter().enumerate() {
                trig = (-c).mul_add(legendre((x - beta) / alpha, n)?, trig);
            }
        }
    }
    Ok(match label {
        param_red_dm_sin | param_red_dm_cos => {
            trig / (2.41e-4 * 1e-12 * (*read(&rows[i].freqSSB, "freqSSB")?).powf(2.0))
        }
        param_red_chrom_sin | param_red_chrom_cos => {
            1.0 * trig
                / ((*read(&rows[i].freqSSB, "freqSSB")?) / 1.4e9)
                    .powf(*read(&p.TNChromIdx, "TNChromIdx")?)
        }
        param_swgp_sin | param_swgp_cos => {
            trig * *read(&rows[i].spherical_solar_wind, "spherical_solar_wind")?
        }
        _ => trig,
    })
}
/// Optional files written by the source's k=0 sine updates. Chromatic noise
/// deliberately uses the same tnreddm.meta name/content as the DM callback.
pub fn update_metadata(
    p: &SourcePulsar,
    label: Label,
    k: usize,
    io: &crate::tkfit::FitDiagnostics,
) -> Result<Option<std::path::PathBuf>, TimingError> {
    if k != 0 || io.write_residuals & 4 == 0 {
        return Ok(None);
    }
    let (file, prefix) = match label {
        param_red_sin => ("tnred.meta", "RED"),
        param_red_dm_sin | param_red_chrom_sin => ("tnreddm.meta", "REDDM"),
        param_swgp_sin => ("swgp.meta", "SWGP"),
        _ => return Ok(None),
    };
    let span = (p.param.slots[param_finish as usize].cells[0].val.clone()
        - &p.param.slots[param_start as usize].cells[0].val)
        .to_f64();
    let low = if label == param_red_sin {
        10.0f64.powf(*read(&p.TNRedFLow, "TNRedFLow")?)
    } else {
        1.0
    };
    let frequency = low * (k as f64 + 1.0) / span;
    let mut fields = vec![
        ("OMEGA".to_owned(), frequency * 2.0 * std::f64::consts::PI),
        (
            "EPOCH".to_owned(),
            p.param.slots[param_pepoch as usize].cells[0].val.to_f64(),
        ),
    ];
    if prefix == "REDDM" {
        fields.push((
            "DMEPOCH".into(),
            p.param.slots[param_dmepoch as usize].cells[0].val.to_f64(),
        ));
        for (k, name) in ["DM", "DM1", "DM2"].into_iter().enumerate() {
            // This source diagnostic reads prefit even for unset DM derivatives.
            // Preserve the unwritten boundary instead of printing guessed zeros.
            fields.push((
                name.into(),
                read(
                    &p.param.slots[param_dm as usize].cells[k].prefit,
                    "noise metadata DM prefit",
                )?
                .to_f64(),
            ));
        }
    }
    let mut bytes = vec![];
    for (name, value) in fields {
        bytes.extend(format!("{prefix}_{name} ").as_bytes());
        bytes.extend(crate::clock_text::format_number(
            value,
            crate::clock_text::NumberFormat::GeneralSix,
        ));
        bytes.push(b'\n');
    }
    io.write(file, &bytes).map(Some).map_err(|e| boundary(&e))
}

pub fn update(
    p: &mut SourcePulsar,
    rows: &mut [SourceObservation],
    label: Label,
    k: usize,
    value: f64,
    error: f64,
) -> Result<(), TimingError> {
    if label == param_shapevent {
        let e = k / MAX_TNShapeCoef;
        let c = k % MAX_TNShapeCoef;
        p.TNShapeletEvCoef[e][c] =
            Some(*read(&p.TNShapeletEvCoef[e][c], "shape coefficient")? + value);
        p.TNShapeletEvCoefErr[e][c] = Some(error);
        return Ok(());
    }
    if label == param_jitter {
        return Ok(());
    }
    for i in 0..*read(&p.nobs, "nobs")? as usize {
        let x = (read(&rows[i].bbat, "bbat")?.clone()
            - &p.param.slots[param_pepoch as usize].cells[0].val)
            .to_f64();
        let y = derivative(p, rows, x, i, label, k)?;
        let o = &mut rows[i];
        let (signal, err) = match label {
            param_red_dm_sin | param_red_dm_cos => (&mut o.TNDMSignal, &mut o.TNDMErr),
            param_red_chrom_sin | param_red_chrom_cos => (&mut o.TNChromSignal, &mut o.TNChromErr),
            param_swgp_sin | param_swgp_cos => (&mut o.SWGPSignal, &mut o.SWGPErr),
            _ => (&mut o.TNRedSignal, &mut o.TNRedErr),
        };
        *signal = Some(y.mul_add(value, *read(signal, "noise signal")?));
        *err = Some(*read(err, "noise error")? + (y * error).powf(2.0));
    }
    Ok(())
}

/// Source-enabled families in prior-row order; harmonic and logarithmic counts.
pub(crate) fn families(p: &SourcePulsar) -> Result<Vec<(Label, Label, usize, usize)>, TimingError> {
    let mut out = vec![];
    let yes = |a: &Option<f64>, b: &Option<f64>| -> Result<bool, TimingError> {
        Ok(*read(a, "TN amplitude")? != 0.0 && *read(b, "TN gamma")? != 0.0)
    };
    let count = |v: &Option<i32>| -> Result<usize, TimingError> {
        usize::try_from(*read(v, "noise count")?).map_err(|_| boundary("negative noise count"))
    };
    if yes(&p.TNRedAmp, &p.TNRedGam)? {
        out.push((
            param_red_sin,
            param_red_cos,
            count(&p.TNRedC)?,
            count(&p.TNRed_log_freqs)?,
        ));
    }
    for (sin, cos, n, counts) in [
        (
            param_band_red_sin,
            param_band_red_cos,
            &p.nTNBandNoise,
            &p.TNBandNoiseC[..],
        ),
        (
            param_group_red_sin,
            param_group_red_cos,
            &p.nTNGroupNoise,
            &p.TNGroupNoiseC[..],
        ),
    ] {
        if count(n)? > 0 {
            let mut total = 0;
            for c in &counts[..count(n)?] {
                total += count(c)?;
            }
            out.push((sin, cos, total, 0));
        }
    }
    if yes(&p.TNChromAmp, &p.TNChromGam)? && *read(&p.TNChromIdx, "TNChromIdx")? != 0.0 {
        out.push((
            param_red_chrom_sin,
            param_red_chrom_cos,
            count(&p.TNChromC)?,
            count(&p.TNChrom_log_freqs)?,
        ));
    }
    if yes(&p.TNDMAmp, &p.TNDMGam)? {
        out.push((
            param_red_dm_sin,
            param_red_dm_cos,
            count(&p.TNDMC)?,
            count(&p.TNDM_log_freqs)?,
        ));
    }
    if yes(&p.SWGPAmp, &p.SWGPGam)? {
        out.push((param_swgp_sin, param_swgp_cos, count(&p.SWGPC)?, 0));
    }
    Ok(out)
}
pub(crate) fn prepare_parameters(p: &mut SourcePulsar, enable: bool) -> Result<(), TimingError> {
    for (sin, cos, n, _) in families(p)? {
        if enable && n == 0 {
            continue;
        }
        for label in [sin, cos] {
            let c = &mut p.param.slots[label as usize].cells[0];
            c.fit_flag = i32::from(enable);
            c.param_set = i32::from(enable);
        }
    }
    if enable {
        for i in 0..*read(&p.nTNShapeletEvents, "nTNShapeletEvents")? as usize {
            if p.TNShapeletEvFitFlag[i] == Some(1) {
                let c = &mut p.param.slots[param_shapevent as usize].cells[0];
                c.fit_flag = 1;
                c.param_set = 1;
            }
        }
    } else {
        let c = &mut p.param.slots[param_shapevent as usize].cells[0];
        if c.fit_flag != 0 {
            c.fit_flag = 0;
            c.param_set = 0;
        }
    }
    Ok(())
}
/// t2Fit_fillFitInfo ECORR then SECORR epoch membership; other flags within an epoch
/// are included by the source's time-only mask, and the two passes are independent.
pub(crate) fn jitter_epochs(
    p: &SourcePulsar,
    obs: &[SourceObservation],
    rows: &[crate::general_fitting::FitRow],
) -> Result<Vec<usize>, TimingError> {
    let mut epochs = vec![];
    for (n, ids, values) in [
        (&p.nTNECORR, &p.TNECORRFlagID[..], &p.TNECORRFlagVal[..]),
        (&p.nTNSECORR, &p.TNSECORRFlagID[..], &p.TNSECORRFlagVal[..]),
    ] {
        let mut flag = vec![false; rows.len()];
        for (i, row) in rows.iter().enumerate() {
            if flag[i] {
                continue;
            }
            let o = &obs[row.observation];
            'matches: for j in 0..*read(n, "ECORR count")? as usize {
                for f in 0..*read(&o.nFlags, "nFlags")? as usize {
                    if read_string(&o.flagID[f], "flagID")? == read_string(&ids[j], "ECORR ID")?
                        && read_string(&o.flagVal[f], "flagVal")?
                            == read_string(&values[j], "ECORR value")?
                    {
                        flag[i] = true;
                        epochs.push(row.observation);
                        break 'matches;
                    }
                }
            }
            if flag[i] {
                let x = row.x_days.0.to_f64();
                for j in i..rows.len() {
                    let t = rows[j].x_days.0.to_f64();
                    if t > x - 1.0 / 86400.0 && t < x + 1.0 / 86400.0 {
                        flag[j] = true;
                    }
                }
            }
        }
    }
    Ok(epochs)
}
