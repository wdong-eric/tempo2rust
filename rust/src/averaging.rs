//! formResiduals.C::averageResiduals/averageDMResiduals. Owned accumulator arrays
//! preserve the source iteration order and free AverageFreq with the other arrays.
use crate::{
    TimingError,
    clock_diagnostics::{Diagnostics, boundary},
    clock_text::{NumberFormat, format_number},
    observation::{SourceObservation, read, read_string},
    preprocess::flags,
    pulsar::SourcePulsar,
};
use std::{fs::File, io::Write};

#[allow(non_snake_case)]
pub(crate) fn averageResiduals(
    p: &SourcePulsar,
    rows: &mut [SourceObservation],
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    average(p, rows, false, d)
}
#[allow(non_snake_case)]
pub(crate) fn averageDMResiduals(
    p: &SourcePulsar,
    rows: &mut [SourceObservation],
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    average(p, rows, true, d)
}
fn average(
    p: &SourcePulsar,
    rows: &mut [SourceObservation],
    dm: bool,
    d: &mut Diagnostics,
) -> Result<(), TimingError> {
    let timestep = *read(&p.AverageEpochWidth, "AverageEpochWidth")?;
    let flag = read_string(&p.AverageFlag[..], "AverageFlag")?;
    d.stderr(
        &[
            b"width ".as_slice(),
            &format_number(timestep as f64, NumberFormat::ScientificThree),
            b" flag ",
            &flag,
            b"\n",
        ]
        .concat(),
    );
    let n = *read(&p.nobs, "nobs")?;
    let mut systems: Vec<Vec<u8>> = vec![];
    let (mut mintime, mut maxtime) = (100000.0f64, 0.0f64);
    for (i, o) in rows.iter().take(n.max(0) as usize).enumerate() {
        let bat = read(&o.bat, "bat")?.to_f64();
        if bat > maxtime {
            maxtime = bat;
        }
        if bat < mintime {
            mintime = bat;
        }
        let mut found = false;
        for (id, val) in flags(o)? {
            if id.eq_ignore_ascii_case(&flag) {
                // Native discovery is case sensitive; later matching is not.
                if !systems.contains(&val) {
                    systems.push(val);
                }
                found = true;
            }
        }
        if !found {
            d.stdout(
                format!("Observation {i} is missing the -f flag, please check before continuing\n")
                    .as_bytes(),
            );
            if dm {
                // formResiduals.C:85: defined early return retains prior DM fields.
                return Ok(());
            }
        }
    }
    mintime = mintime.floor();
    maxtime = maxtime.ceil();
    let numbins = crate::binary::source_i32(((maxtime - mintime) / timestep as f64).ceil())?;
    let bins = usize::try_from(numbins).map_err(|_| boundary("averaging_negative_bin_count"))?;
    let mut weight = vec![vec![0.0f64; bins]; systems.len()];
    let mut res = weight.clone();
    let mut bat = res.clone();
    let mut freq = res.clone();
    for o in rows.iter().take(n.max(0) as usize) {
        for (id, val) in flags(o)? {
            if !id.eq_ignore_ascii_case(&flag) {
                continue;
            }
            let s = systems
                .iter()
                .rposition(|v| v.eq_ignore_ascii_case(&val))
                .ok_or_else(|| boundary("averaging_system"))?;
            let b = crate::binary::source_i32(
                ((read(&o.bat, "bat")?.to_f64() - mintime) / timestep as f64).floor(),
            )?;
            let b = usize::try_from(b).map_err(|_| boundary("averaging_bin"))?;
            if b >= bins {
                return Err(boundary("averaging_bin"));
            }
            let error = (*read(&o.toaErr, "toaErr")? * 10.0f64.powf(-6.0)).powf(2.0);
            let mut r = read(&o.residual, "residual")?.to_f64();
            for (selector, signal, name) in [
                (p.TNsubtractRed, o.TNRedSignal, "TNRedSignal"),
                (p.TNsubtractDM, o.TNDMSignal, "TNDMSignal"),
                (p.TNsubtractSWGP, o.SWGPSignal, "SWGPSignal"),
                (p.TNsubtractChrom, o.TNChromSignal, "TNChromSignal"),
            ] {
                if selector == Some(1) {
                    r -= *read(&signal, name)?;
                }
            }
            weight[s][b] += 1.0 / error;
            if dm {
                let f = *read(&o.freq, "freq")?;
                // Native powf narrows the argument and result to binary32.
                res[s][b] += r * ((f / 1400.0) as f32).powf(2.0) as f64 / error;
                freq[s][b] += f / error;
            } else {
                res[s][b] += r / error;
            }
            bat[s][b] += read(&o.bat, "bat")?.to_f64() / error;
        }
    }
    for o in rows.iter_mut().take(n.max(0) as usize) {
        let fs = flags(o)?;
        let (mut ecorr, mut secorr) = (0.0f64, 0.0f64);
        for (id, val) in &fs {
            for k in 0..*read(&p.nTNECORR, "nTNECORR")? {
                let k = k as usize;
                if *id == read_string(&p.TNECORRFlagID[k], "TNECORRFlagID")?
                    && *val == read_string(&p.TNECORRFlagVal[k], "TNECORRFlagVal")?
                {
                    ecorr = *read(&p.TNECORRVal[k], "TNECORRVal")? * 10.0f64.powf(-6.0);
                }
            }
        }
        for (id, val) in &fs {
            for k in 0..*read(&p.nTNSECORR, "nTNSECORR")? {
                let k = k as usize;
                if *id == read_string(&p.TNSECORRFlagID[k], "TNSECORRFlagID")?
                    && *val == read_string(&p.TNSECORRFlagVal[k], "TNSECORRFlagVal")?
                {
                    secorr = *read(&p.TNSECORRVal[k], "TNSECORRVal")? * 10.0f64.powf(-6.0)
                        / (*read(&o.tobs, "tobs")? / 3600.0).sqrt();
                }
            }
        }
        for (id, val) in &fs {
            if !id.eq_ignore_ascii_case(&flag) {
                continue;
            }
            let s = systems
                .iter()
                .rposition(|v| v.eq_ignore_ascii_case(val))
                .ok_or_else(|| boundary("averaging_system"))?;
            let b = crate::binary::source_i32(
                ((read(&o.bat, "bat")?.to_f64() - mintime) / timestep as f64).floor(),
            )? as usize;
            let bavg = bat[s][b] / weight[s][b];
            let ravg = res[s][b] / weight[s][b];
            let err = (1.0 / weight[s][b] + ecorr.powf(2.0) + secorr.powf(2.0)).sqrt();
            if dm {
                let f = freq[s][b] / weight[s][b];
                d.stderr(
                    &[
                        b"freq: ".as_slice(),
                        &format_number(f, NumberFormat::Three),
                        b"\n",
                    ]
                    .concat(),
                );
                o.averagedmbat = Some(bavg);
                o.averagedmres = Some(ravg);
                o.averagedmerr = Some(((f / 1400.0) as f32).powf(2.0) as f64 * err);
            } else {
                o.averagebat = Some(bavg);
                o.averageres = Some(ravg);
                o.averageerr = Some(err);
            }
        }
    }
    if !dm {
        // Source writes retained fields on unmatched rows; a first-call missing
        // field is reported when read, rather than replacing it with zero.
        let err = |e: std::io::Error| {
            TimingError::new("avg.dat", None, "averageResiduals", e.to_string())
        };
        let mut file = File::create("avg.dat").map_err(err)?;
        for o in rows.iter().take(n.max(0) as usize) {
            let line = [
                format_number(*read(&o.averagebat, "averagebat")?, NumberFormat::Nine),
                b" ".to_vec(),
                format_number(
                    *read(&o.averageres, "averageres")?,
                    NumberFormat::ScientificThree,
                ),
                b" ".to_vec(),
                format_number(
                    *read(&o.averageerr, "averageerr")?,
                    NumberFormat::ScientificSix,
                ),
                b" \n".to_vec(),
            ]
            .concat();
            file.write_all(&line).map_err(err)?;
        }
        file.flush().map_err(err)?;
    }
    Ok(())
}
