//! Active updateEpoch.C branches. The source's `&& 0 == 1` binary block stays inactive.
use crate::{
    PRECISION_BITS, TimingError,
    observation::{read, write_string},
    preprocess::{is_set, value},
    pulsar::SourcePulsar,
    pulsar_lifecycle::PulsarContext,
    source_text::extended,
    tempo2::Label::*,
};
use rug::Float;
use std::f64::consts::PI;

pub fn update_epoch_str(
    context: &mut PulsarContext,
    p: usize,
    new_epoch: &[u8],
) -> Result<(), TimingError> {
    // Source deliberately reads START/FINISH from psr[0], even for target p>0.
    let first = &context.pulsars[0];
    let start = &first.param.slots[param_start as usize].cells[0];
    let end = &first.param.slots[param_finish as usize].cells[0];
    let start_set = start.param_set == 1 && start.fit_flag == 1;
    let end_set = end.param_set == 1 && end.fit_flag == 1;
    let bat_start = start.param_set == 1 && start.fit_flag == 2;
    let bat_end = end.param_set == 1 && end.fit_flag == 2;
    let lo = start.val.clone() - 1e-11f64;
    let hi = end.val.clone() + 1e-11f64;
    let (target, rows) = context.pulsar_observations_mut(p)?;
    let mut earliest = Float::with_val(PRECISION_BITS, -1);
    let mut latest = earliest.clone();
    for o in &rows[..*read(&target.nobs, "nobs")? as usize] {
        if *read(&o.deleted, "deleted")? != 0 {
            continue;
        }
        let sat = read(&o.sat, "sat")?;
        if (start_set && sat < &lo)
            || (end_set && sat > &hi)
            || (bat_start && read(&o.bat, "bat")? < &lo)
            || (bat_end && read(&o.bat, "bat")? > &hi)
        {
            continue;
        }
        if earliest == -1 {
            earliest = sat.clone();
            latest = sat.clone();
        }
        if &earliest > sat {
            earliest = sat.clone();
        }
        if &latest < sat {
            latest = sat.clone();
        }
    }
    let n = match new_epoch.to_ascii_uppercase().as_slice() {
        b"CENTRE" | b"CENTER" => Float::with_val(
            PRECISION_BITS,
            crate::ifteph::extended_i32(&((earliest + latest) / 2.0f64), "epoch_center_int")?,
        ),
        b"LEFT" => Float::with_val(
            PRECISION_BITS,
            crate::ifteph::extended_i32(&earliest, "epoch_left_int")?,
        ),
        b"RIGHT" => Float::with_val(
            PRECISION_BITS,
            crate::ifteph::extended_i32(&latest, "epoch_right_int")?,
        ),
        _ => extended(new_epoch)?,
    };
    update_epoch(target, &n)
}
pub fn update_epoch(p: &mut SourcePulsar, n: &Float) -> Result<(), TimingError> {
    let dt = ((n.clone() - value(p, param_pepoch, 0)) * 86400.0f64).to_f64();
    crate::clkcorr::lifecycle_diagnostics(|d| {
        d.stdout(
            &[
                b"dt = ".as_slice(),
                &crate::clock_text::format_number(dt, crate::clock_text::NumberFormat::GeneralSix),
                b"\n",
            ]
            .concat(),
        )
    })?;
    let f2 = value(p, param_f, 2);
    p.param.slots[param_f as usize].cells[0].val =
        value(p, param_f, 0) + value(p, param_f, 1) * dt + f2.clone() * 0.5f64 * dt * dt;
    p.param.slots[param_f as usize].cells[1].val += f2.clone() * dt;
    if is_set(p, param_f, 3) {
        let f3 = value(p, param_f, 3);
        p.param.slots[param_f as usize].cells[0].val += f3.clone() * (1.0f64 / 6.0) * dt * dt * dt;
        p.param.slots[param_f as usize].cells[1].val += f3.clone() * 0.5f64 * dt * dt;
        p.param.slots[param_f as usize].cells[2].val += f3 * dt;
    }
    for i in 0..2 {
        let c = &mut p.param.slots[param_f as usize].cells[i];
        c.prefit = Some(c.val.clone());
    }
    let c = &mut p.param.slots[param_pepoch as usize].cells[0];
    c.val = n.clone();
    c.prefit = Some(n.clone());
    let dt = ((n.clone() - value(p, param_posepoch, 0)) / 365.25f64).to_f64();
    crate::clkcorr::lifecycle_diagnostics(|d| {
        d.stdout(
            &[
                b"pos dt = ".as_slice(),
                &crate::clock_text::format_number(dt, crate::clock_text::NumberFormat::GeneralSix),
                b"\n",
            ]
            .concat(),
        )
    })?;
    for (motion, position, second) in [
        (param_pmra, param_raj, false),
        (param_pmdec, param_decj, false),
        (param_pmra2, param_raj, true),
        (param_pmdec2, param_decj, true),
    ] {
        if is_set(p, motion, 0) {
            let mut shift = value(p, motion, 0);
            if position == param_raj {
                crate::clkcorr::lifecycle_diagnostics(|d| d.stdout(b"Updating RAJ\n"))?;
                shift /= value(p, param_decj, 0).cos();
            }
            shift = shift / 1000.0f64 * (PI / 180.0) / 60.0f64 / 60.0f64 * dt;
            if second {
                shift = shift * dt / 2.0f64;
            }
            let c = &mut p.param.slots[position as usize].cells[0];
            c.val += shift;
            c.prefit = Some(c.val.clone());
            let turn = (c.val.clone() / (2.0 * PI)).to_f64();
            let text = if position == param_raj {
                turn_hms(turn)?
            } else {
                turn_dms(turn)?
            };
            let (a, b) = if position == param_raj {
                (&mut p.rajStrPre[..], &mut p.rajStrPost[..])
            } else {
                (&mut p.decjStrPre[..], &mut p.decjStrPost[..])
            };
            write_string(a, &text, "coordinate pre")?;
            write_string(b, &text, "coordinate post")?;
        }
    }
    let c = &mut p.param.slots[param_posepoch as usize].cells[0];
    c.val = n.clone();
    c.prefit = Some(n.clone());
    let dt = ((n.clone() - value(p, param_dmepoch, 0)) / 365.25f64).to_f64();
    p.param.slots[param_dm as usize].cells[0].val = value(p, param_dm, 0)
        + value(p, param_dm, 1) * dt
        + value(p, param_dm, 2) * 0.5f64 * dt * dt;
    let c = &mut p.param.slots[param_dmepoch as usize].cells[0];
    c.val = n.clone();
    c.prefit = Some(n.clone());
    crate::clkcorr::lifecycle_diagnostics(|d| {
        if is_set(p, param_waveepoch, 0) {
            d.stdout(b"ERROR: Not updating the FITWAVES to new EPOCH\n");
        }
        if is_set(p, param_waveepoch_dm, 0) {
            d.stdout(b"ERROR: Not updating the FITWAVES(DM) to new EPOCH\n");
        }
    })
}
pub fn turn_hms(turn: f64) -> Result<Vec<u8>, TimingError> {
    let mut h = crate::ifteph::native_i32(turn * 24.0, "turn_hms_hour")?;
    let mut m = crate::ifteph::native_i32((turn * 24.0 - h as f64) * 60.0, "turn_hms_minute")?;
    let mut sec = ((turn * 24.0 - h as f64) * 60.0 - m as f64) * 60.0;
    if crate::ifteph::native_i32((sec * 10000.0 + 0.5) / 10000.0, "turn_hms_second")? == 60 {
        sec = 0.0;
        m += 1;
        if m == 60 {
            m = 0;
            h += 1;
            if h == 24 {
                h = 0;
            }
        }
    }
    Ok(crate::clock_text::format_coordinate(h, m, sec, None))
}
pub fn turn_dms(turn: f64) -> Result<Vec<u8>, TimingError> {
    let t = if turn < 0.0 { -turn } else { turn };
    let mut d = crate::ifteph::native_i32(t * 360.0, "turn_dms_degree")?;
    let mut m = crate::ifteph::native_i32((t * 360.0 - d as f64) * 60.0, "turn_dms_minute")?;
    let mut sec = ((t * 360.0 - d as f64) * 60.0 - m as f64) * 60.0;
    if crate::ifteph::native_i32((sec * 1000.0 + 0.5) / 1000.0, "turn_dms_second")? == 60 {
        sec = 0.0;
        m += 1;
        if m == 60 {
            m = 0;
            d += 1;
        }
    }
    Ok(crate::clock_text::format_coordinate(
        d,
        m,
        sec,
        Some(if turn < 0.0 { b'-' } else { b'+' }),
    ))
}
