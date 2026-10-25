//! preProcess.C built-in selection helpers, preserving loop and mutation order.
use crate::{
    TimingError,
    bats::{ConsumedInput, consume},
    clock_text,
    observation::{SourceObservation, read},
    preprocess::flags,
    pulsar_lifecycle::PulsarContext,
    source_text::SourceText,
};
use std::path::Path;
fn matching(o: &SourceObservation, id: &[u8], val: &[u8]) -> Result<bool, TimingError> {
    Ok(o.deleted == Some(0) && flags(o)?.iter().any(|(a, b)| a == id && b == val))
}

pub fn logic_flag(line: &[u8], context: &mut PulsarContext) -> Result<(), TimingError> {
    let mut s = SourceText::new(Path::new("logicFlag"), line.to_vec());
    s.required_token("command")?;
    let id = s.required_token("flag ID")?;
    let op = s.required_token("operation")?;
    let arg = s.required_token("argument")?;
    // s3 is source-unwritten for the ordinary four-token EXIST form, but never
    // read on that branch. Require it only for the numeric branch that uses it.
    let action = s.token();
    for p in 0..context.pulsars.len() {
        let (p, rows) = context.pulsar_observations_mut(p)?;
        for o in &mut rows[..*read(&p.nobs, "nobs")? as usize] {
            let found = if o.deleted == Some(0) {
                flags(o)?.into_iter().find(|(a, _)| a == &id)
            } else {
                None
            };
            if op.eq_ignore_ascii_case(b"EXIST") {
                if found.is_none() && arg.eq_ignore_ascii_case(b"PASS")
                    || found.is_some() && arg.eq_ignore_ascii_case(b"REJECT")
                {
                    o.deleted = Some(1);
                }
            } else if let Some((_, v)) = found
                && let Some((v, _)) = clock_text::number_prefix(&v)
            {
                let action = action
                    .as_deref()
                    .ok_or_else(|| s.error("logic action", "source-unwritten token"))?;
                if action.eq_ignore_ascii_case(b"REJECT")
                    && matches!(op.as_slice(), b">" | b"<" | b"=")
                {
                    let x = clock_text::number_prefix(&arg)
                        .ok_or_else(|| s.error("logic threshold", "conversion"))?
                        .0;
                    if op == b">" && v > x || op == b"<" && v < x || op == b"=" && v == x {
                        o.deleted = Some(1);
                    }
                }
            }
        }
    }
    Ok(())
}
pub fn process_flag(line: &[u8], context: &mut PulsarContext) -> Result<(), TimingError> {
    let mut s = SourceText::new(Path::new("processFlag"), line.to_vec());
    s.required_token("command")?;
    let id = s.required_token("flag ID")?;
    let val = s.required_token("flag value")?;
    let mode = s.required_token("mode")?.to_ascii_lowercase();
    let range = if matches!(
        mode.as_slice(),
        b"mjdpass" | b"mjdreject" | b"errpass" | b"errreject" | b"freqpass" | b"freqreject"
    ) {
        Some((s.double("lower")?, s.double("upper")?))
    } else {
        None
    };
    for p in 0..context.pulsars.len() {
        let (p, rows) = context.pulsar_observations_mut(p)?;
        for o in &mut rows[..*read(&p.nobs, "nobs")? as usize] {
            if matching(o, &id, &val)? {
                if mode == b"reject" {
                    o.deleted = Some(1);
                } else if let Some((a, b)) = range {
                    let x = if mode.starts_with(b"mjd") {
                        read(&o.sat, "sat")?.to_f64()
                    } else if mode.starts_with(b"err") {
                        *read(&o.toaErr, "toaErr")?
                    } else {
                        *read(&o.freq, "freq")?
                    };
                    if mode.ends_with(b"pass") && (x < a || x > b)
                        || mode.ends_with(b"reject") && x >= a && x <= b
                    {
                        o.deleted = Some(1);
                    }
                }
            }
        }
    }
    Ok(())
}
pub fn process_simultaneous(line: &[u8], context: &mut PulsarContext) -> Result<(), TimingError> {
    let t: Vec<_> = clock_text::tokens(line).take(7).collect();
    if t.len() < 6 {
        return crate::clkcorr::lifecycle_diagnostics(|d| {
            let mut msg = b"Problem in processing line: ".to_vec();
            msg.extend(line);
            msg.push(b'\n');
            d.stdout(&msg);
        });
    }
    let dt = t
        .get(6)
        .and_then(|v| clock_text::number_prefix(v))
        .map(|v| v.0)
        .unwrap_or(60.0);
    let only = t[0] == b"PROCESS" && t[1] == b"SIMUL" && t[5].eq_ignore_ascii_case(b"only");
    for p in 0..context.pulsars.len() {
        let (p, rows) = context.pulsar_observations_mut(p)?;
        let n = *read(&p.nobs, "nobs")? as usize;
        for i in 0..n {
            let mut simul = false;
            for j in 0..n {
                if i != j
                    && (read(&rows[i].sat, "sat")?.clone() - read(&rows[j].sat, "sat")?).abs()
                        * 86400.0f64
                        < dt
                {
                    let mut a = matching(&rows[i], t[2], t[3])?;
                    let mut b = matching(&rows[j], t[2], t[4])?;
                    if only {
                        if a && b {
                            simul = true;
                            break;
                        }
                        // The source retains found1/found2 for the reverse search.
                        a |= matching(&rows[i], t[2], t[4])?;
                        b |= matching(&rows[j], t[2], t[3])?;
                        if a && b {
                            simul = true;
                            break;
                        }
                    } else if a && b {
                        if t[5].eq_ignore_ascii_case(b"toaerr") {
                            let selected = if read(&rows[i].toaErr, "toaErr")?
                                < read(&rows[j].toaErr, "toaErr")?
                            {
                                j
                            } else {
                                i
                            };
                            rows[selected].deleted = Some(1);
                        } else if t[5] == b"1" {
                            rows[j].deleted = Some(1);
                        } else if t[5] == b"2" {
                            rows[i].deleted = Some(1);
                        }
                    }
                }
            }
            if only && !simul {
                rows[i].deleted = Some(1);
            }
        }
    }
    Ok(())
}
pub fn use_select_file(
    path: &Path,
    context: &mut PulsarContext,
    inputs: &mut Vec<ConsumedInput>,
) -> Result<(), TimingError> {
    let mut s = consume(path, "selection", inputs).inspect_err(|_| {
        use std::os::unix::ffi::OsStrExt;
        let _ = crate::clkcorr::lifecycle_diagnostics(|d| {
            d.stdout(
                &[
                    b"Unable to open select file: >".as_slice(),
                    path.as_os_str().as_bytes(),
                    b"<\n",
                ]
                .concat(),
            )
        });
    })?;
    let mut previous = vec![];
    loop {
        let (line, eof) = if let Some(line) = s.line_chunk(1000) {
            previous = line.clone();
            (line, false)
        } else {
            (previous.clone(), true)
        };
        let t: Vec<_> = clock_text::tokens(&line).collect();
        if t.len() >= 2 {
            if matches!(t[0], b"PASS" | b"REJECT") && matches!(t[1], b"MJD" | b"FREQ" | b"TOAERR") {
                let mut scanner = SourceText::new(path, line.clone());
                scanner.token();
                scanner.token();
                let a = scanner.double("lower")?;
                let b = scanner.double("upper")?;
                for p in 0..context.pulsars.len() {
                    let (p, rows) = context.pulsar_observations_mut(p)?;
                    for o in &mut rows[..*read(&p.nobs, "nobs")? as usize] {
                        if o.deleted == Some(0) {
                            let x = match t[1] {
                                b"MJD" => read(&o.sat, "sat")?.to_f64(),
                                b"FREQ" => *read(&o.freq, "freq")?,
                                _ => *read(&o.toaErr, "toaErr")?,
                            };
                            if t[0] == b"PASS" && (x < a || x > b)
                                || t[0] == b"REJECT" && x >= a && x <= b
                            {
                                o.deleted = Some(1);
                            }
                        }
                    }
                }
            } else if t[0] == b"PASS" && t[1].starts_with(b"-") {
                let parts: Vec<_> = line
                    .split(|&b| b == b' ')
                    .filter(|v| !v.is_empty())
                    .collect();
                let id = parts
                    .get(1)
                    .ok_or_else(|| s.error("PASS", "literal-space strtok has no flag ID"))?;
                let values: Vec<_> = parts
                    .iter()
                    .skip(2)
                    .map(|v| v.strip_suffix(b"\n").unwrap_or(v))
                    .collect();
                if values.len() >= 20 {
                    crate::clkcorr::lifecycle_diagnostics(|d| {
                        d.stdout(b"ERROR: too many flags in the select file\n")
                    })?;
                    return Err(s.error("PASS", "too many flags"));
                }
                for p in 0..context.pulsars.len() {
                    let (p, rows) = context.pulsar_observations_mut(p)?;
                    for o in &mut rows[..*read(&p.nobs, "nobs")? as usize] {
                        if o.deleted == Some(0)
                            && !flags(o)?
                                .iter()
                                .any(|(a, b)| a == id && values.contains(&b.as_slice()))
                        {
                            o.deleted = Some(1);
                        }
                    }
                }
                previous = parts[0].to_vec();
            } else if t[0] == b"PROCESS" && t[1] == b"SIMUL" {
                process_simultaneous(&line, context)?;
            } else if t[0] == b"PROCESS" && t[1].starts_with(b"-") {
                process_flag(&line, context)?;
            } else if t[0].eq_ignore_ascii_case(b"LOGIC") && t[1].starts_with(b"-") {
                logic_flag(&line, context)?;
            }
        }
        if eof {
            break;
        }
    }
    Ok(())
}
