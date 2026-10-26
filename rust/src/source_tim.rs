//! readTimfile.C source reader. Local controls reset on INCLUDE; INFO is process state.
use crate::{
    PRECISION_BITS, TimingError,
    bats::{ConsumedInput, consume},
    clock_text,
    observation::{SourceObservation, read, read_string, write_string},
    pulsar::SourcePulsar,
    source_text::{SourceText, extended},
};
use rug::Float;
use std::{os::unix::ffi::OsStrExt, path::Path};

/// Direct source readTim entry; rows retain the final unused/partially written slot.
pub fn read_tim(
    path: &Path,
    psr: &mut SourcePulsar,
    rows: &mut Vec<SourceObservation>,
    jump: &mut i32,
    _fdjump: &mut i32,
    root: &Path,
    inputs: &mut Vec<ConsumedInput>,
) -> Result<(), TimingError> {
    crate::clkcorr::with_resources(root, |session| {
        read_tim_inner(path, psr, rows, jump, session, inputs, &mut vec![])
    })
}

fn read_tim_inner(
    path: &Path,
    psr: &mut SourcePulsar,
    rows: &mut Vec<SourceObservation>,
    jump: &mut i32,
    session: &mut crate::clkcorr::ClockSession,
    inputs: &mut Vec<ConsumedInput>,
    stack: &mut Vec<std::path::PathBuf>,
) -> Result<(), TimingError> {
    let mut source = consume(path, "TIM", inputs).inspect_err(|_| {
        session.diagnostics.stdout(
            &[
                b"ERROR [FILE4]: Unable to open timfile >".as_slice(),
                path.as_os_str().as_bytes(),
                b"<\n",
            ]
            .concat(),
        );
    })?;
    let canonical = std::fs::canonicalize(path)
        .map_err(|e| TimingError::new(path, None, "TIM", e.to_string()))?;
    if stack.contains(&canonical) {
        return Err(TimingError::new(path, None, "INCLUDE", "recursive include"));
    }
    stack.push(canonical.clone());
    let fixed = *read(&psr.fixedFormat, "fixedFormat")?;
    let mut format = 1;
    if fixed > 0 {
        for _ in 0..fixed {
            source.line_chunk(1000);
        }
    } else {
        for token in clock_text::tokens(&source.bytes) {
            if token == b"FORMAT" {
                format = 0;
            } else if token == b"HEAD" {
                format = 2;
            }
        }
    }
    if format == 2 {
        while let Some(line) = source.line_chunk(1000) {
            if line
                .strip_suffix(b"\n")
                .unwrap_or(&line)
                .eq_ignore_ascii_case(b"TOAS")
            {
                break;
            }
        }
        format = 1;
    }
    let (
        mut efac,
        mut eset,
        mut emax,
        mut efloor,
        mut emin,
        mut equad,
        mut fmax,
        mut fmin,
        mut sigma,
        mut time,
    ) = (1.0, -1.0, -1.0, -1.0, -1.0, 0.0, -1.0, -1.0, 0.0, 0.0);
    let mut skip = false;
    let mut profile = vec![];
    let mut previous = vec![];
    let mut previous_scans = 0;
    loop {
        let (mut line, eof, stop) = match source.line_chunk(1000) {
            Some(l) => {
                // fgets sets feof on a short final line without a newline. The
                // successful row is processed, but there is no stale-line replay.
                let stop =
                    source.position == source.bytes.len() && l.len() < 999 && !l.ends_with(b"\n");
                (l, false, stop)
            }
            None => {
                // readTimfile.C:198-202 removes the last byte of a prior partial
                // row before fgets; a failed read then retains that shortened row.
                if format == 0 && previous_scans < 5 && previous.len() > 1 {
                    previous.pop();
                }
                (previous.clone(), true, true)
            }
        };
        if line.is_empty() && eof {
            break;
        }
        let n = *read(&psr.nobs, "nobs")? as usize;
        if rows.len() <= n {
            rows.resize_with(n + 1, SourceObservation::default);
        }
        if format == 0 {
            while line.last().is_some_and(|b| clock_text::whitespace(*b)) {
                line.pop();
            }
        } else if line.last() == Some(&b'\n') {
            line.pop();
        }
        previous = line.clone();
        let o = &mut rows[n];
        let valid = if format == 0 {
            read_format_one(
                &line,
                o,
                &profile,
                &source,
                &mut previous_scans,
                &mut session.diagnostics,
            )?
        } else if !eof {
            read_legacy(&line, o, &source, &mut session.diagnostics)?
        } else {
            false
        };
        if valid && !eof {
            o.initialize_missing_clock_scratch();

            let site = read_string(&o.telID, "telID")?;
            if site.starts_with(b"@") || site.eq_ignore_ascii_case(b"bat") {
                o.clockCorr = Some(0);
                o.delayCorr = Some(0);
            } else {
                o.clockCorr = Some(i32::from(site != b"STL" && site != b"STL_FBAT"));
                o.delayCorr = Some(1);
            }
            let err = *read(&o.toaErr, "toaErr")?;
            let mut err = (err.powi(2) + equad * equad).sqrt();
            o.equad = Some(equad);
            if sigma != 0.0 {
                err = sigma;
            }
            err *= efac;
            o.efac = Some(efac);
            if eset > -1.0 {
                err = eset;
            }
            o.jump[0] = Some(*jump);
            o.obsNjump = Some(1);
            o.fdjump[0] = Some(*jump);
            o.obsNfdjump = Some(1);
            if time != 0.0 {
                session.diagnostics.log(
                    b'd',
                    "readTimfile.C",
                    501,
                    &[
                        b"psr ".as_slice(),
                        &read_string(&psr.name[..], "name")?,
                        b" adding time >",
                        &clock_text::format_number(time, clock_text::NumberFormat::GeneralSix),
                        b"<",
                    ]
                    .concat(),
                );
                *o.sat
                    .as_mut()
                    .ok_or_else(|| source.error("sat", "unwritten"))? += time / 60.0 / 60.0 / 24.0;
            }
            if efloor != -1.0 && err < efloor {
                err = efloor;
            }
            if (emax != -1.0 && err > emax) || (emin != -1.0 && err < emin) {
                o.deleted = Some(1);
            }
            o.toaErr = Some(err);
            let freq = *read(&o.freq, "freq")?;
            let keep = !(fmax != -1.0 && freq > fmax || fmin != -1.0 && freq < fmin || skip);
            if session.source.tim_info != -1 {
                append_flag(
                    o,
                    b"-i",
                    session.source.tim_info.to_string().as_bytes(),
                    &source,
                    &mut session.diagnostics,
                )?;
            }
            o.origErr = Some(err);
            if keep {
                psr.nobs = Some(n as i32 + 1);
            }
            if psr.nobs.unwrap() > session.source.max_observations - 2 {
                session
                    .diagnostics
                    .stderr(b"Too many TOAs! Use the -npsr and -nobs command line arguments!\n");
                return Err(source.error("MAX_OBSN", "source observation capacity exceeded"));
            }
        } else if !eof {
            let mut directive = SourceText::new(path, line);
            if let Some(key) = directive.token() {
                if skip {
                    if key.eq_ignore_ascii_case(b"NOSKIP") {
                        skip = false;
                    }
                    continue;
                }
                match key.to_ascii_uppercase().as_slice() {
                    b"END" => break,
                    b"PROFILE_DIR" => {
                        if let Some(value) = directive.token() {
                            profile = value;
                        }
                    }
                    // readTimfile.C:544-603 ignores sscanf's count here. Retain
                    // initialized destinations on failed conversion; TIME below
                    // instead scans a fresh, unwritten local and must succeed.
                    b"GLOBAL_EFAC" => {
                        if let Ok(v) = directive.double("GLOBAL_EFAC") {
                            psr.T2globalEfac = Some(v);
                        }
                    }
                    b"EFAC" => {
                        if let Ok(v) = directive.double("EFAC") {
                            efac = v;
                        }
                    }
                    b"EFLOOR" => {
                        if let Ok(v) = directive.double("EFLOOR") {
                            efloor = v;
                        }
                    }
                    b"EMAX" => {
                        if let Ok(v) = directive.double("EMAX") {
                            emax = v;
                        }
                    }
                    b"EMIN" => {
                        if let Ok(v) = directive.double("EMIN") {
                            emin = v;
                        }
                    }
                    b"ESET" => {
                        if let Ok(v) = directive.double("ESET") {
                            eset = v;
                        }
                    }
                    b"FMAX" => {
                        if let Ok(v) = directive.double("FMAX") {
                            fmax = v;
                        }
                    }
                    b"FMIN" => {
                        if let Ok(v) = directive.double("FMIN") {
                            fmin = v;
                        }
                    }
                    b"EQUAD" => {
                        if let Ok(v) = directive.double("EQUAD") {
                            equad = v;
                        }
                    }
                    b"SIGMA" => {
                        if let Ok(v) = directive.double("SIGMA") {
                            sigma = v;
                        }
                    }
                    b"TIME" => {
                        let delta = directive.double("TIME")?;
                        time += delta;
                        session.diagnostics.log(
                            b'd',
                            "readTimfile.C",
                            590,
                            &[
                                b"Updating time: ".as_slice(),
                                &clock_text::format_number(
                                    delta,
                                    clock_text::NumberFormat::GeneralSix,
                                ),
                                b" ",
                                &clock_text::format_number(
                                    time,
                                    clock_text::NumberFormat::GeneralSix,
                                ),
                            ]
                            .concat(),
                        );
                    }
                    b"INFO" => {
                        if let Some(v) = directive.scan_integer()? {
                            session.source.tim_info = v;
                        }
                    }
                    b"MODE" => {
                        if let Some(v) = directive.scan_integer()? {
                            psr.fitMode = Some(v);
                        }
                        session.diagnostics.display(
                            1,
                            b"TIM1",
                            b"Please place MODE flags in the parameter file",
                            b"",
                            *read(&psr.noWarnings, "noWarnings")?,
                        )?;
                    }
                    b"SKIP" => skip = true,
                    b"NOSKIP" => skip = false,
                    b"JUMP" => *jump += 1,
                    b"T2EFAC" | b"T2EQUAD" => {
                        let isefac = key.eq_ignore_ascii_case(b"T2EFAC");
                        let (count, ids, vals, numbers) = if isefac {
                            (
                                &mut psr.nT2efac,
                                &mut psr.T2efacFlagID[..],
                                &mut psr.T2efacFlagVal[..],
                                &mut psr.T2efacVal[..],
                            )
                        } else {
                            (
                                &mut psr.nT2equad,
                                &mut psr.T2equadFlagID[..],
                                &mut psr.T2equadFlagVal[..],
                                &mut psr.T2equadVal[..],
                            )
                        };
                        let i = *read(count, "noise count")? as usize;
                        write_string(
                            ids.get_mut(i)
                                .ok_or_else(|| source.error("noise", "capacity"))?,
                            &directive.required_token("flag ID")?,
                            "flag ID",
                        )?;
                        write_string(
                            &mut vals[i],
                            &directive.required_token("flag value")?,
                            "flag value",
                        )?;
                        numbers[i] = Some(directive.double("noise value")?);
                        *count = Some(i as i32 + 1);
                    }
                    b"PHASE" => {
                        let prior = n
                            .checked_sub(1)
                            .ok_or_else(|| source.error("PHASE", "no previous observation"))?;
                        let j = *read(&psr.nPhaseJump, "nPhaseJump")? as usize;
                        *psr.phaseJump
                            .get_mut(j)
                            .ok_or_else(|| source.error("PHASE", "capacity"))? =
                            Some(read(&rows[prior].sat, "sat")?.clone() + 1.0 / 86400.0f64);
                        psr.phaseJumpID[j] = Some(prior as i32);
                        psr.phaseJumpDir[j] = Some(directive.integer("PHASE")?);
                        psr.nPhaseJump = Some(j as i32 + 1);
                    }
                    b"INCLUDE" => {
                        if let Some(name) = directive.token() {
                            session.diagnostics.stdout(
                                &[
                                    b"Current filename = ".as_slice(),
                                    path.as_os_str().as_bytes(),
                                    b"\n",
                                ]
                                .concat(),
                            );
                            // Source strcat preserves an initial slash in newtim,
                            // rather than treating it as an absolute replacement.
                            let include = [
                                canonical.parent().unwrap().as_os_str().as_bytes(),
                                b"/",
                                &name,
                            ]
                            .concat();
                            let include = Path::new(std::ffi::OsStr::from_bytes(&include));
                            session.diagnostics.stdout(
                                &[
                                    b"Rel path = ".as_slice(),
                                    include.as_os_str().as_bytes(),
                                    b"\n",
                                ]
                                .concat(),
                            );
                            read_tim_inner(include, psr, rows, jump, session, inputs, stack)?;
                        } else {
                            session.diagnostics.stdout(
                                &[
                                    b"Unable to parse INCLUDE line >".as_slice(),
                                    &directive.bytes,
                                    b"<\n",
                                ]
                                .concat(),
                            );
                        }
                    }
                    _ => (),
                }
            }
        }
        if stop {
            break;
        }
    }
    stack.pop();
    Ok(())
}

fn read_format_one(
    line: &[u8],
    o: &mut SourceObservation,
    profile: &[u8],
    source: &SourceText,
    scans: &mut usize,
    diagnostics: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<bool, TimingError> {
    *scans = if matches!(line.first(), Some(b'C' | b'#')) {
        5
    } else {
        0
    };
    if line.is_empty() || matches!(line.first(), Some(b'C' | b'#')) {
        return Ok(false);
    }
    o.deleted = Some(0);
    let mut s = SourceText::new(&source.path, line.to_vec());
    if line[0] == b'I' {
        s.token();
        *scans += 1;
        o.deleted = Some(1);
    }
    let Some(name) = s.token() else {
        return Ok(false);
    };
    write_string(&mut o.fname, &name, "fname")?;
    *scans += 1;
    if !profile.is_empty() && line[0] != b'I' {
        let full = [profile, b"/", &name].concat();
        for (i, byte) in o.fname[..500].iter_mut().enumerate() {
            *byte = Some(full.get(i).copied().unwrap_or(0));
        }
    }
    let Ok(freq) = s.double("freq") else {
        return Ok(false);
    };
    o.freq = Some(freq);
    *scans += 1;
    let Some(sat) = s.token() else {
        return Ok(false);
    };
    *scans += 1;
    let Ok(err) = s.double("toaErr") else {
        return Ok(false);
    };
    o.toaErr = Some(err);
    *scans += 1;
    // The I prefix contributes one successful scan. Five scans therefore accept
    // a missing site and retain telID; its later consumer diagnoses unwritten
    // storage. This is defined for N's deleted_missing_site initialized caller.
    if let Some(site) = s.token() {
        write_string(&mut o.telID, &site, "telID")?;
        *scans += 1;
    } else if line[0] != b'I' {
        return Ok(false);
    }

    if sat.len() < 5 {
        return Err(source.error(
            "sat",
            "source five-byte day split needs at least five bytes",
        ));
    }
    o.sat_day = Some(extended(&sat[..5])?);
    let mut fraction = vec![b'0'];
    fraction.extend(&sat[5..]);
    o.sat_sec = Some(extended(&fraction)?);
    o.sat = Some(extended(&sat)?);
    o.phaseOffset = Some(0.0);
    o.nFlags = Some(0);
    o.toaDMErr = Some(0.0);
    let mut i = 0;
    while i + 1 < line.len() {
        if line[i] == b'-'
            && !line[i + 1].is_ascii_digit()
            && let Some(space) = line[i..].iter().position(|&b| b == b' ')
        {
            let id = &line[i..i + space];
            i += space + 1;
            if line[i..].contains(&b' ') {
                while i < line.len() && line[i] == b' ' {
                    i += 1;
                }
            }
            let end = line[i..]
                .iter()
                .position(|&b| b == b' ')
                .map(|n| i + n)
                .unwrap_or(line.len());
            let val = &line[i..end];
            append_flag(o, id, val, source, diagnostics)?;
            if id == b"-addsat" {
                let offset = clock_text::number_prefix(val)
                    .ok_or_else(|| source.error("-addsat", "conversion"))?
                    .0;
                *o.sat.as_mut().unwrap() += Float::with_val(PRECISION_BITS, offset) / 86400.0f64;
            }
            if id == b"-dme" {
                let n = o.nFlags.unwrap() as usize;
                if n < 2 {
                    return Err(source.error("-dme", "source reads the preceding flag"));
                }
                let dme = clock_text::number_prefix(val)
                    .ok_or_else(|| source.error("-dme", "conversion"))?
                    .0;
                clock_text::number_prefix(&read_string(&o.flagVal[n - 2], "DM flag")?)
                    .ok_or_else(|| source.error("-dme", "preceding DM conversion"))?;
                let freq = freq * 1e6;
                o.toaDMErr = Some(dme / 2.410e-4 / 1e-12 / freq / freq * 1e6);
            }
            i = end;
        }
        i += 1;
    }
    Ok(true)
}
fn append_flag(
    o: &mut SourceObservation,
    id: &[u8],
    val: &[u8],
    s: &SourceText,
    diagnostics: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<(), TimingError> {
    let n = *read(&o.nFlags, "nFlags")? as usize;
    write_string(
        o.flagID
            .get_mut(n)
            .ok_or_else(|| s.error("flagID", "capacity"))?,
        id,
        "flagID",
    )?;
    write_string(&mut o.flagVal[n], val, "flagVal")?;
    o.nFlags = Some(n as i32 + 1);
    if n + 1 >= crate::observation::MAX_FLAGS {
        diagnostics.stdout(
            format!(
                "Number of different flags in the .tim file > MAX_FLAGS ({})\n",
                crate::observation::MAX_FLAGS
            )
            .as_bytes(),
        );
        return Err(s.error("flags", "source MAX_FLAGS exit"));
    }
    Ok(())
}
fn read_legacy(
    line: &[u8],
    o: &mut SourceObservation,
    s: &SourceText,
    diagnostics: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<bool, TimingError> {
    o.nFlags = Some(0);
    let Some(key) = clock_text::tokens(line).next() else {
        return Ok(false);
    };
    if key.eq_ignore_ascii_case(b"C") {
        return Ok(false);
    }
    o.deleted = Some(0);
    if key.eq_ignore_ascii_case(b"INCLUDE") {
        return Ok(false);
    }
    let field = |a: usize, b: usize| -> Result<&[u8], TimingError> {
        let tail = line
            .get(a..)
            .ok_or_else(|| s.error("legacy TIM", "unwritten fixed-field backing"))?;
        Ok(&tail[..tail.len().min(b - a)])
    };
    let scan = |a, b, dest: &mut Option<f64>| -> Result<bool, TimingError> {
        if let Some((v, _)) = clock_text::number_prefix(field(a, b)?) {
            *dest = Some(v);
            Ok(true)
        } else {
            Ok(false)
        }
    };
    let mut valid = true;
    if line[0] == b' ' {
        if line.len() < 79 {
            return Ok(false);
        }
        // readTimfile.C:371 copies the whole remaining line with strncpy(500),
        // then terminates at 25. Bytes after that terminator remain observable.
        for (i, byte) in o.fname[..500].iter_mut().enumerate() {
            *byte = Some(line.get(i + 1).copied().unwrap_or(0));
        }
        o.fname[25] = Some(0);
        valid &= scan(25, 34, &mut o.freq)?;
        o.sat = Some(extended(field(34, 55)?)?);
        valid &= scan(55, 63, &mut o.phaseOffset)?;
        valid &= scan(63, 71, &mut o.toaErr)?;
        if let Some(tel) = clock_text::tokens(field(79, line.len())?).next() {
            write_string(&mut o.telID, tel, "telID")?;
        }
        o.telID[1] = Some(0);
    } else if line.get(1) == Some(&b' ') {
        if let Some(tel) = clock_text::tokens(line).next() {
            write_string(&mut o.telID, tel, "telID")?;
        }
        o.telID[1] = Some(0);
        write_string(&mut o.fname, b"NOT SET", "fname")?;
        valid &= scan(15, 24, &mut o.freq)?;
        o.sat = Some(extended(field(25, 45)?)?);
        valid &= scan(45, 54, &mut o.toaErr)?;
        if let Some((dm, _)) = clock_text::number_prefix(field(68, 78)?) {
            append_flag(
                o,
                b"-dmo",
                &clock_text::format_number(dm, clock_text::NumberFormat::GeneralTen),
                s,
                diagnostics,
            )?;
        }
    } else if line.get(14) == Some(&b'.') {
        write_string(&mut o.fname, b"NOT SET", "fname")?;
        o.sat = Some(extended(field(9, 28)?)?);
        valid &= scan(28, 34, &mut o.toaErr)?;
        valid &= scan(34, 45, &mut o.freq)?;
        valid &= scan(45, 55, &mut o.phaseOffset)?;
        if let Some(tel) = clock_text::tokens(field(57, line.len())?).next() {
            write_string(&mut o.telID, tel, "telID")?;
        }
        o.telID[2] = Some(0);
    } else {
        return Ok(false);
    }
    Ok(valid)
}

/// readTimfile with first-pulsar JBO/deletion selection retained.
pub fn read_timfiles(
    context: &mut crate::pulsar_lifecycle::PulsarContext,
    paths: &[std::path::PathBuf],
    root: &Path,
    inputs: &mut Vec<ConsumedInput>,
) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| {
        s.diagnostics.version("readTimfile.C", "readTimfile()");
        s.diagnostics
            .log(b'd', "readTimfile.C", 66, b"In reading tim file");
        Ok(())
    })?;
    let jbo = *read(&context.pulsars[0].jboFormat, "jboFormat")?;
    let deletion = read_string(&context.pulsars[0].deleteFileName[..], "deleteFileName")?;
    for (p, path) in paths.iter().enumerate() {
        let view = context.pulsars[p]
            .obsn
            .flatten()
            .ok_or_else(|| crate::clock_diagnostics::boundary("TIM_observation_owner"))?;
        let mut rows = context
            .observations(view)
            .map_err(|e| TimingError::new(path, None, "observations", e.to_string()))?
            .to_vec();
        let result = if jbo != 0 {
            crate::clkcorr::with_process(|s| {
                s.diagnostics
                    .log(b'm', "readTimfile.C", 84, b"Reading JBO format!?");
                Ok(())
            })?;
            crate::read_jbo_bat::read_jbo_bat(path, &mut context.pulsars[p], &mut rows, inputs)
        } else {
            read_tim(
                path,
                &mut context.pulsars[p],
                &mut rows,
                &mut 0,
                &mut 0,
                root,
                inputs,
            )
        };
        // Publish partial canonical writes even on a source error; the public load
        // wrapper returns no successful BatInput until all input files complete.
        context
            .free_observations(view)
            .map_err(|e| TimingError::new(path, None, "observations", e.to_string()))?;
        let view = context.adopt_observations(rows);
        context.pulsars[p].obsn = Some(Some(view));
        result?;
        crate::clkcorr::with_process(|s| {
            s.diagnostics.log(
                b'd',
                "readTimfile.C",
                89,
                &[b"Checking for deleted points >".as_slice(), &deletion, b"<"].concat(),
            );
            Ok(())
        })?;
        if deletion != b"NONE" {
            crate::clkcorr::with_process(|s| {
                s.diagnostics
                    .log(b'd', "readTimfile.C", 94, b"In checking for deleted points");
                Ok(())
            })?;
            let deletion = Path::new(std::ffi::OsStr::from_bytes(&deletion));
            if let Ok(mut s) = consume(deletion, "deleted TOAs", inputs) {
                let count = context.pulsars[p].nobs.unwrap() as usize;
                let rows = context
                    .observations_mut(view)
                    .map_err(|e| TimingError::new(path, None, "observations", e.to_string()))?;
                while let Some(t) = s.token() {
                    let sat = extended(&t)?;
                    for o in &mut rows[..count] {
                        if (read(&o.sat, "sat")?.clone() - &sat).abs() < 0.001f64 {
                            o.deleted = Some(1);
                        }
                    }
                }
            } else {
                crate::clkcorr::with_process(|s| {
                    s.diagnostics.stdout(
                        &[
                            b"Warning: unable to open ".as_slice(),
                            deletion.as_os_str().as_bytes(),
                            b"\n",
                        ]
                        .concat(),
                    );
                    Ok(())
                })?;
            }
        }
    }
    crate::clkcorr::with_process(|s| {
        s.diagnostics
            .log(b'd', "readTimfile.C", 116, b"Leaving readTimfile");
        Ok(())
    })?;
    Ok(())
}

pub fn remove_cr2(line: &mut Vec<u8>) -> Result<(), TimingError> {
    crate::source_par::remove_cr(line)
}

/// writeTim source order, including its reversible offsets and retained origErr.
pub fn write_tim(
    path: &Path,
    context: &mut crate::pulsar_lifecycle::PulsarContext,
    index: usize,
    format: &[u8],
    root: &Path,
) -> Result<(), TimingError> {
    use crate::clock_text::{NumberFormat as F, format_number as num};
    use crate::tempo2::Label::param_f;
    use std::io::Write;
    let mut output = match std::fs::File::create(path) {
        Ok(file) => file,
        Err(e) => {
            crate::clkcorr::with_process(|s| {
                s.diagnostics.stdout(
                    &[
                        b"Sorry, unable to write to file ".as_slice(),
                        path.as_os_str().as_bytes(),
                        b"\n",
                    ]
                    .concat(),
                );
                Ok(())
            })?;
            return Err(TimingError::new(path, None, "writeTim", e.to_string()));
        }
    };
    let mut emit = |bytes: &[u8]| {
        output
            .write_all(bytes)
            .map_err(|e| TimingError::new(path, None, "writeTim", e.to_string()))
    };
    crate::clkcorr::with_resources(root, |session| {
        let (p, rows) = context
            .pulsar_observation_backing_mut(index)
            .map_err(|e| TimingError::new(path, None, "writeTim backing", e.to_string()))?;
        if format == b"tempo2" {
            emit(b"FORMAT 1\n")?;
        }
        if p.fitMode == Some(1) {
            emit(b"MODE 1\n")?;
        }
        if *read(&p.T2globalEfac, "T2globalEfac")? != 1.0 {
            emit(
                &[
                    b"GLOBAL_EFAC ".as_slice(),
                    &num(p.T2globalEfac.unwrap(), F::GeneralFive),
                    b"\n",
                ]
                .concat(),
            )?;
        }
        for (keyword, n, ids, vals, numbers) in [
            (
                b"T2EQUAD".as_slice(),
                p.nT2equad,
                &p.T2equadFlagID[..],
                &p.T2equadFlagVal[..],
                &p.T2equadVal[..],
            ),
            (
                b"T2EFAC".as_slice(),
                p.nT2efac,
                &p.T2efacFlagID[..],
                &p.T2efacFlagVal[..],
                &p.T2efacVal[..],
            ),
        ] {
            for i in 0..*read(&n, "noise count")? as usize {
                emit(
                    &[
                        keyword,
                        b" ",
                        &read_string(&ids[i], "noise flag ID")?,
                        b" ",
                        &read_string(&vals[i], "noise flag value")?,
                        b" ",
                        &num(*read(&numbers[i], "noise value")?, F::GeneralFive),
                        b"\n",
                    ]
                    .concat(),
                )?;
            }
        }
        // Source accesses observation zero even when nobs is zero. An empty
        // canonical allocation has no assigned EFAC/EQUAD and is an error.
        let first = rows
            .first()
            .ok_or_else(|| crate::clock_diagnostics::boundary("writeTim_observation_zero"))?;
        let mut efac = *read(&first.efac, "efac")?;
        let mut equad = *read(&first.equad, "equad")?;
        if equad != 0.0 {
            emit(&[b"EQUAD ".as_slice(), &num(equad, F::Six), b"\n"].concat())?;
        }
        if efac != 1.0 {
            emit(&[b"EFAC ".as_slice(), &num(efac, F::Six), b"\n"].concat())?;
        }
        for o in &mut rows[..*read(&p.nobs, "nobs")? as usize] {
            if *read(&o.equad, "equad")? != equad {
                equad = o.equad.unwrap();
                emit(&[b"EQUAD ".as_slice(), &num(equad, F::Six), b"\n"].concat())?;
            }
            if *read(&o.efac, "efac")? != efac {
                efac = o.efac.unwrap();
                emit(&[b"EFAC ".as_slice(), &num(efac, F::Six), b"\n"].concat())?;
            }
            if *read(&o.deleted, "deleted")? == 1 {
                emit(b"C")?;
            }
            if format == b"tempo2" {
                let mut sat = read(&o.sat, "sat")?.clone();
                let f = crate::preprocess::value(p, param_f, 0);
                if f > 0 {
                    sat -= Float::with_val(PRECISION_BITS, *read(&o.phaseOffset, "phaseOffset")?)
                        / f
                        / 86400.0f64;
                }
                for k in 0..*read(&p.nToffset, "nToffset")? as usize {
                    if crate::preprocess::offset_applies(session, p, o, k)? {
                        sat -= *read(&p.tOffset[k], "tOffset")? / 86400.0f64;
                    }
                }
                let flags = crate::preprocess::flags(o)?;
                for key in [b"-to".as_slice(), b"-addsat".as_slice()] {
                    for (id, val) in &flags {
                        if id == key {
                            sat -= extended(val)? / 86400.0f64;
                        }
                    }
                }
                let name = read_string(&o.fname, "fname")?;
                let name = clock_text::tokens(&name)
                    .next()
                    .ok_or_else(|| crate::clock_diagnostics::boundary("writeTim_name_unwritten"))?;
                let interim = *read(&o.origErr, "origErr")? / efac;
                let interim = (interim.powf(2.0) - equad.powf(2.0)).sqrt();
                emit(
                    &[
                        b" ".as_slice(),
                        &name[..name.len().min(999)],
                        b" ",
                        &num(*read(&o.freq, "freq")?, F::Eight),
                        b" ",
                        &crate::source_text::fixed(&sat, 17),
                        b" ",
                        &num(interim, F::Five),
                        b" ",
                        &read_string(&o.telID, "telID")?,
                        b" ",
                    ]
                    .concat(),
                )?;
                if interim < 0.0 {
                    return Err(crate::clock_diagnostics::boundary(
                        "negative_written_uncertainty",
                    ));
                }
                for (id, val) in flags {
                    if !id.eq_ignore_ascii_case(b"-selectAll") {
                        emit(&[id.as_slice(), b" ", val.as_slice(), b" "].concat())?;
                    }
                }
                emit(b"\n")?;
            } else {
                let mut name = read_string(&o.fname, "fname")?;
                if name.len() > 22 {
                    o.fname[22] = Some(0);
                    name.truncate(22);
                }
                while name.len() < 25 {
                    name.push(b' ');
                }
                emit(
                    &[
                        b" ".as_slice(),
                        &name,
                        &num(*read(&o.freq, "freq")?, F::WidthEightThree),
                        b"  ",
                        &crate::source_text::fixed(read(&o.sat, "sat")?, 13),
                        b"    0.00 ",
                        &num(*read(&o.origErr, "origErr")?, F::WidthSevenTwo),
                        b"        ",
                        &read_string(&o.telID, "telID")?,
                        b"\n",
                    ]
                    .concat(),
                )?;
            }
        }
        Ok(())
    })
}
