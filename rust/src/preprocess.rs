//! preProcess.C and preProcessSimple.C source sequencing on canonical state.
use crate::{
    PRECISION_BITS, TimingError,
    bats::{ConsumedInput, consume},
    clkcorr::ClockSession,
    clock_text,
    observation::{SourceObservation, read, read_string, write_string},
    pulsar::{SeriesType, SourcePulsar},
    pulsar_lifecycle::PulsarContext,
    source_text::extended,
    tempo2::{Label, Label::*},
};
use rug::Float;
use std::{os::unix::ffi::OsStrExt, path::Path};
pub(crate) fn value(p: &SourcePulsar, l: Label, i: usize) -> Float {
    p.param.slots[l as usize].cells[i].val.clone()
}
pub(crate) fn is_set(p: &SourcePulsar, l: Label, i: usize) -> bool {
    p.param.slots[l as usize].cells[i].param_set == 1
}
fn number(bytes: &[u8]) -> Result<f64, TimingError> {
    clock_text::number_prefix(bytes)
        .map(|v| v.0)
        .ok_or_else(|| crate::clock_diagnostics::boundary("preprocess_numeric_conversion"))
}
pub(crate) type ObservationFlag = (Vec<u8>, Vec<u8>);
pub(crate) fn flags(o: &SourceObservation) -> Result<Vec<ObservationFlag>, TimingError> {
    (0..*read(&o.nFlags, "nFlags")?)
        .map(|i| {
            Ok((
                read_string(&o.flagID[i as usize], "flagID")?,
                read_string(&o.flagVal[i as usize], "flagVal")?,
            ))
        })
        .collect()
}
fn alias(session: &mut ClockSession, site: &[u8]) -> Result<Vec<u8>, TimingError> {
    let mut out = [None; 256];
    session.observatories.lookup_alias(
        site,
        &mut out,
        session.root.as_deref(),
        session.alias.as_deref(),
        &mut session.diagnostics,
    )?;
    read_string(&out, "observatory alias")
}
fn flag_selection(selection: &[u8], o: &SourceObservation) -> Result<bool, TimingError> {
    let fs = flags(o)?;
    let mut tokens = selection.split(|&b| b == b' ').filter(|v| !v.is_empty());
    let mut selected = false;
    while let Some(id) = tokens.next() {
        // preProcessSimple.C and readTimfile.C::writeTim consume exactly the
        // following token for a flag. Numeric minus tokens and leading words
        // are ignored; a missing value is read only after an ID matches.
        if id.starts_with(b"-") && !id.get(1).is_some_and(u8::is_ascii_digit) {
            let value = tokens.next();
            for (a, b) in &fs {
                if a == id {
                    let value = value.ok_or_else(|| {
                        crate::clock_diagnostics::boundary("selection_flag_unwritten")
                    })?;
                    if b == value {
                        selected = true;
                        break;
                    }
                }
            }
        }
    }
    Ok(selected)
}
pub(crate) fn offset_applies(
    session: &mut ClockSession,
    p: &SourcePulsar,
    o: &SourceObservation,
    k: usize,
) -> Result<bool, TimingError> {
    let freq = *read(&o.freq, "freq")?;
    let sat = read(&o.sat, "sat")?;
    let (f1, f2, t1, t2) = (
        *read(&p.tOffset_f1[k], "tOffset_f1")?,
        *read(&p.tOffset_f2[k], "tOffset_f2")?,
        *read(&p.tOffset_t1[k], "tOffset_t1")?,
        *read(&p.tOffset_t2[k], "tOffset_t2")?,
    );
    let selected = alias(session, &read_string(&p.tOffsetSite[k], "tOffsetSite")?)?;
    let site = alias(session, &read_string(&o.telID, "telID")?)?;
    if (f1 == 0.0 || freq > f1)
        && (f2 == 0.0 || freq < f2)
        && (t1 == 0.0 || sat > &t1)
        && (t2 == 0.0 || sat < &t2)
        && (selected == b"0" || selected == site)
    {
        let sel = read_string(&p.tOffsetFlags[k], "tOffsetFlags")?;
        Ok(sel.is_empty() || flag_selection(&sel, o)?)
    } else {
        Ok(false)
    }
}

pub fn pre_process_simple1(
    context: &mut PulsarContext,
    p: usize,
    tempo1: i32,
    last: f64,
    root: &Path,
) -> Result<(), TimingError> {
    crate::clkcorr::with_resources(root, |session| simple1(session, context, p, tempo1, last))
}
fn simple1(
    session: &mut ClockSession,
    context: &mut PulsarContext,
    p: usize,
    mut tempo1: i32,
    last: f64,
) -> Result<(), TimingError> {
    session
        .diagnostics
        .log(b'd', "preProcessSimple.C", 46, b"In preProcessSimple1");
    session.source.ecliptic_obliquity = 84381.4059;
    let (p, rows) = context.pulsar_observations_mut(p)?;
    let rows = &rows[..*read(&p.nobs, "nobs")? as usize];
    for label in [param_wave_om, param_wave_dm] {
        if is_set(p, label, 0) && value(p, label, 0) == 0 {
            let mut first = Float::with_val(PRECISION_BITS, -1);
            let mut last = first.clone();
            for o in rows {
                if o.deleted == Some(0) {
                    let sat = read(&o.sat, "sat")?;
                    if first == -1 || sat < &first {
                        first = sat.clone();
                    }
                    if last == -1 || sat > &last {
                        last = sat.clone();
                    }
                }
            }
            p.param.slots[label as usize].cells[0].val =
                Float::with_val(PRECISION_BITS, 2.0 * std::f64::consts::PI)
                    / (last - first)
                    / (1.0 + 4.0 / (*read(&p.nWhite, "nWhite")? as f64));
            if label == param_wave_om {
                session.diagnostics.stderr(
                    &[
                        clock_text::format_number(
                            value(p, label, 0).to_f64(),
                            clock_text::NumberFormat::ScientificThree,
                        )
                        .as_slice(),
                        b"\n",
                    ]
                    .concat(),
                );
            }
        }
    }
    if is_set(p, param_ephver, 0) {
        tempo1 = i32::from(value(p, param_ephver, 0) < 5);
        if tempo1 == 1 {
            session.diagnostics.stdout(b"************************************************* \nWarning: you are running in tempo1 emulation mode \n************************************************* \n");
        }
    } else {
        let c = &mut p.param.slots[param_ephver as usize].cells[0];
        c.param_set = 1;
        c.val = Float::with_val(PRECISION_BITS, if tempo1 == 1 { 2 } else { 5 });
    }
    if last > 0.0 {
        let o = rows
            .last()
            .ok_or_else(|| crate::clock_diagnostics::boundary("last_empty_observations"))?;
        let c = &mut p.param.slots[param_start as usize].cells[0];
        c.val = read(&o.sat, "sat")?.clone() - last;
        c.fit_flag = 1;
        c.param_set = 1;
    }
    p.tempo1 = Some(0);
    if tempo1 == 1 {
        p.tempo1 = Some(1);
        p.units = Some(2);
        p.timeEphemeris = Some(2);
        p.dilateFreq = Some(0);
        p.planetShapiro = Some(0);
        p.t2cMethod = Some(2);
        p.correctTroposphere = Some(0);
        p.ne_sw = Some(9.961);
        session.source.ecliptic_obliquity = 84381.412;
    }
    if p.units == Some(100) {
        crate::units::transform_units_inner(p, 2, 1, &mut session.diagnostics)?;
    }
    Ok(())
}

pub fn pre_process_simple2(
    context: &mut PulsarContext,
    p: usize,
    start: f32,
    ndm: i32,
    dm: Option<&[f32]>,
    trim: i32,
    root: &Path,
) -> Result<(), TimingError> {
    crate::clkcorr::with_resources(root, |session| {
        simple2(session, context, p, start, ndm, dm, trim)
    })
}
fn simple2(
    session: &mut ClockSession,
    context: &mut PulsarContext,
    p: usize,
    start: f32,
    ndm: i32,
    dm: Option<&[f32]>,
    trim: i32,
) -> Result<(), TimingError> {
    let (p, rows) = context.pulsar_observations_mut(p)?;
    session
        .diagnostics
        .log(b'd', "preProcessSimple.C", 141, b"In preProcessSimple2");
    for (index, o) in rows[..*read(&p.nobs, "nobs")? as usize]
        .iter_mut()
        .enumerate()
    {
        if p.fitMode == Some(1) && o.toaErr == Some(0.0) {
            session.diagnostics.stdout(
                &[
                    format!("Error: the uncertainty for TOA {} (MJD ", index + 1).as_bytes(),
                    &clock_text::format_number(
                        read(&o.sat, "sat")?.to_f64(),
                        clock_text::NumberFormat::Six,
                    ),
                    b") is equal to zero\n ",
                ]
                .concat(),
            );
            return Err(TimingError::new(
                "",
                None,
                "zero_weighted_TOA_uncertainty",
                "source boundary: weighted TOA uncertainty is zero",
            ));
        }
        // preProcessSimple2 resets the initial observer vectors but omits siteVel.
        // tt2tdb.C reads it before get_obsCoord's first write. Supply zero only
        // for unwritten rough-pass velocity; repeated calls retain defined state.
        for v in &mut o.siteVel {
            if v.is_none() {
                *v = Some(0.0);
            }
        }
        o.earthMoonBary_ssb.fill(Some(0.0));
        o.earthMoonBary_earth.fill(Some(0.0));
        o.observatory_earth.fill(Some(0.0));
        o.earth_ssb.fill(Some(0.0));
        let shift = Float::with_val(PRECISION_BITS, *read(&o.phaseOffset, "phaseOffset")?)
            / value(p, param_f, 0)
            / 86400.0f64;
        *o.sat
            .as_mut()
            .ok_or_else(|| crate::clock_diagnostics::boundary("sat_unwritten"))? += shift;
        for k in 0..*read(&p.nToffset, "nToffset")? as usize {
            if offset_applies(session, p, o, k)? {
                *o.sat.as_mut().unwrap() += *read(&p.tOffset[k], "tOffset")? / 86400.0f64;
            }
        }
        // preProcessSimple.C:207-260 keeps the current flag for every following
        // value. Unlike TOFFSET selectors, a bare flag is legal and matches none.
        let fs = flags(o)?;
        for (selector, pass) in [(&p.filterStr[..], false), (&p.passStr[..], true)] {
            let text = read_string(selector, "filter/pass")?;
            if text.is_empty() {
                continue;
            }
            let mut flag = None;
            let mut found = false;
            for token in text.split(|&b| b == b' ').filter(|s| !s.is_empty()) {
                if token.starts_with(b"-") {
                    flag = Some(token);
                } else {
                    let id = if pass {
                        flag.ok_or_else(|| {
                            crate::clock_diagnostics::boundary("pass_flag_unwritten")
                        })?
                    } else {
                        *flag.get_or_insert(token)
                    };
                    if fs.iter().any(|(a, b)| a == id && b == token) {
                        found = true;
                        if pass {
                            break;
                        }
                    }
                }
            }
            if (pass && !found) || (!pass && found) {
                o.deleted = Some(1);
            }
        }
        for fd in [false, true] {
            let count = if fd {
                *read(&p.nfdJumps, "nfdJumps")?
            } else {
                *read(&p.nJumps, "nJumps")?
            };
            for k in 1..=count as usize {
                let text =
                    read_string(if fd { &p.fdjumpStr[k] } else { &p.jumpStr[k] }, "jumpStr")?;
                let t: Vec<_> = clock_text::tokens(&text).collect();
                let kind = t
                    .first()
                    .ok_or_else(|| crate::clock_diagnostics::boundary("empty_jump_selector"))?
                    .to_ascii_uppercase();
                let arg = *t
                    .get(1)
                    .ok_or_else(|| crate::clock_diagnostics::boundary("jump_selector_argument"))?;
                let mut yes = false;
                let mut scale = 1.0;
                match kind.as_slice() {
                    b"MJD" | b"FREQ" => {
                        let a = number(arg)?;
                        let b =
                            number(t.get(2).ok_or_else(|| {
                                crate::clock_diagnostics::boundary("jump_range")
                            })?)?;
                        yes = if kind == b"MJD" {
                            let sat = read(&o.sat, "sat")?;
                            sat >= &a && sat < &b
                        } else {
                            let freq = *read(&o.freq, "freq")?;
                            freq >= a && freq < b
                        };
                    }
                    b"NAME" => {
                        let name = read_string(&o.fname, "fname")?;
                        yes = name.windows(arg.len()).any(|v| v == arg);
                    }
                    b"TEL" => {
                        yes = !alias(session, arg)?.eq_ignore_ascii_case(&alias(
                            session,
                            &read_string(&o.telID, "telID")?,
                        )?);
                    }
                    b"SCALED" if !fd => {
                        for (id, val) in flags(o)? {
                            if id == arg {
                                yes = true;
                                if let Ok(v) = number(&val) {
                                    scale = v;
                                }
                            }
                        }
                    }
                    _ => {
                        if kind.starts_with(b"-") {
                            yes = flags(o)?.iter().any(|(id, val)| id == t[0] && val == arg);
                        }
                    }
                }
                if yes {
                    if fd {
                        let n = *read(&o.obsNfdjump, "obsNfdjump")? as usize;
                        *o.fdjump.get_mut(n).ok_or_else(|| {
                            crate::clock_diagnostics::boundary("fdjump_capacity")
                        })? = Some(k as i32);
                        o.obsNfdjump = Some(n as i32 + 1);
                    } else {
                        let n = *read(&o.obsNjump, "obsNjump")? as usize;
                        *o.jump
                            .get_mut(n)
                            .ok_or_else(|| crate::clock_diagnostics::boundary("jump_capacity"))? =
                            Some(k as i32);
                        o.jumpScale[n] = Some(scale);
                        o.obsNjump = Some(n as i32 + 1);
                    }
                }
            }
        }
        for (id, val) in flags(o)? {
            if id == b"-to" {
                *o.sat.as_mut().unwrap() += extended(&val)? / 86400.0f64;
            }
        }
        if start != 0.0
            && ndm != 0
            && let Some(dm) = dm
        {
            let i = crate::ifteph::extended_i32(
                &(read(&o.sat, "sat")?.clone() - start as f64 + 0.5f64),
                "DM_table_index",
            )?;
            if i <= 0 || i >= ndm {
                o.deleted = Some(1);
            } else if trim == 0 {
                let v = *dm
                    .get(i as usize)
                    .ok_or_else(|| crate::clock_diagnostics::boundary("DM_table_extent"))?;
                let n = *read(&o.nFlags, "nFlags")? as usize;
                write_string(
                    o.flagID
                        .get_mut(n)
                        .ok_or_else(|| crate::clock_diagnostics::boundary("flag_capacity"))?,
                    b"-dm",
                    "flagID",
                )?;
                write_string(
                    &mut o.flagVal[n],
                    &crate::clock_text::format_number(
                        v as f64,
                        crate::clock_text::NumberFormat::GeneralSix,
                    ),
                    "flagVal",
                )?;
                o.nFlags = Some(n as i32 + 1);
                if n + 1 >= 40 {
                    session
                        .diagnostics
                        .stdout(b"Number of different flags in the .tim file > MAX_FLAGS (40)\n");
                    return Err(crate::clock_diagnostics::boundary("MAX_FLAGS"));
                }
            }
        }
    }
    session.diagnostics.log(
        b'd',
        "preProcessSimple.C",
        417,
        b"Complete preProcessSimple2",
    );
    Ok(())
}
pub fn pre_process_simple3(context: &mut PulsarContext, p: usize) -> Result<(), TimingError> {
    crate::clkcorr::lifecycle_diagnostics(|d| {
        d.log(b'd', "preProcessSimple.C", 423, b"Start preProcessSimple3")
    })?;
    let (p, rows) = context.pulsar_observations_mut(p)?;
    let rows = &mut rows[..*read(&p.nobs, "nobs")? as usize];
    let fjump = read_string(&p.fjumpID[..], "fjumpID")?;
    if !fjump.is_empty() {
        let mut values = vec![];
        for o in rows.iter() {
            if o.deleted == Some(0) {
                for (id, val) in flags(o)? {
                    if id == fjump && !values.contains(&val) {
                        if val.len() >= 16 {
                            return Err(crate::clock_diagnostics::boundary("FJUMP_value_capacity"));
                        }
                        values.push(val);
                        if values.len() >= crate::pulsar::MAX_JUMPS {
                            crate::clkcorr::lifecycle_diagnostics(|d| {
                                d.stdout(b"preProcessSimple3: Number of different flags in the .tim file > MAX_JUMPS (2000)\n")
                            })?;
                            return Err(crate::clock_diagnostics::boundary("FJUMP_count_capacity"));
                        }
                    }
                }
            }
        }
        for fd in [false, true] {
            for val in values.iter().skip(1) {
                let count = if fd { &mut p.nfdJumps } else { &mut p.nJumps };
                let n = *read(count, "jump count")? + 1;
                *count = Some(n);
                let mut text = fjump.clone();
                text.push(b' ');
                text.extend(val);
                write_string(
                    if fd {
                        &mut p.fdjumpStr[n as usize]
                    } else {
                        &mut p.jumpStr[n as usize]
                    },
                    &text,
                    "jumpStr",
                )?;
                for o in rows.iter_mut() {
                    for (_, v) in flags(o)? {
                        if &v == val {
                            let count = if fd {
                                &mut o.obsNfdjump
                            } else {
                                &mut o.obsNjump
                            };
                            let i = *read(count, "jump count")? as usize;
                            let list = if fd { &mut o.fdjump } else { &mut o.jump };
                            *list.get_mut(i).ok_or_else(|| {
                                crate::clock_diagnostics::boundary("FJUMP_observation_capacity")
                            })? = Some(n);
                            *count = Some(i as i32 + 1);
                        }
                    }
                }
                if fd {
                    p.fitfdJump[n as usize] = Some(1);
                } else {
                    p.fitJump[n as usize] = Some(1);
                }
            }
        }
    }
    for i in 0..*read(&p.nPhaseJump, "nPhaseJump")? as usize {
        if i == 0 {
            crate::clkcorr::lifecycle_diagnostics(|d| {
                d.stdout(b"WARNING: Use of phase jumps => .tim file must be sorted in time order\n")
            })?;
        }
        // Source assignment in the condition is deliberate source behavior.
        p.phaseJumpID[i] = Some(-1);
        let phase = read(&p.phaseJump[i], "phaseJump")?.to_f64();
        for (k, o) in rows.iter().enumerate().skip(1) {
            if read(&o.sat, "sat")?.to_f64() > phase {
                p.phaseJumpID[i] = Some(k as i32 - 1);
                break;
            }
        }
    }
    crate::clkcorr::lifecycle_diagnostics(|d| {
        d.log(
            b'd',
            "preProcessSimple.C",
            517,
            b"Complete preProcessSimple3",
        )
    })?;
    Ok(())
}
pub fn pre_process_simple(
    context: &mut PulsarContext,
    p: usize,
    root: &Path,
) -> Result<(), TimingError> {
    crate::clkcorr::lifecycle_diagnostics(|d| {
        d.version("preProcessSimple.C", "preProcessSimple()")
    })?;
    pre_process_simple1(context, p, 0, -1.0, root)?;
    pre_process_simple2(context, p, 0.0, 0, None, 0, root)?;
    pre_process_simple3(context, p)
}

/// Full preProcess entry. Arguments are byte-preserving source argv entries.
pub fn pre_process(
    context: &mut PulsarContext,
    args: &[Vec<u8>],
    root: &Path,
    inputs: &mut Vec<ConsumedInput>,
) -> Result<(), TimingError> {
    crate::clkcorr::lifecycle_diagnostics(|d| {
        d.version("preProcess.C", "preProcess()");
        d.log(b'd', "preProcess.C", 74, b"In preProcess");
    })?;
    let (mut epoch, mut select, mut global, mut plugin, mut modify, mut dmfile) =
        (b"NONE".to_vec(), vec![], vec![], vec![], vec![], vec![]);
    let (mut name, mut fits, mut sets) = (None, vec![], vec![]);
    let (mut last, mut simulate, mut tempo1, mut nojump, mut nofit, mut trim) =
        (-1.0, 0.0, 0, false, false, 0);
    crate::clkcorr::with_resources(root, |s| {
        s.source.ecliptic_obliquity = 84381.4059;
        Ok(())
    })?;
    let mut i = 0;
    while i < args.len() {
        let arg = |offset: usize| {
            args.get(i + offset)
                .cloned()
                .ok_or_else(|| crate::clock_diagnostics::boundary("preprocess_argv_operand"))
        };
        match args[i].as_slice() {
            b"-epoch" => {
                epoch = arg(1)?;
                i += 1;
            }
            b"-last" => {
                last = number(&arg(1)?)?;
                i += 1;
            }
            b"-setdm" => {
                dmfile = arg(1)?;
                i += 1;
            }
            b"-select" => {
                select = arg(1)?;
                i += 1;
            }
            b"-splug" => {
                plugin = arg(1)?;
                i += 1;
            }
            b"-global" => {
                crate::clkcorr::with_resources(root, |s| {
                    s.source.force_global_fit = 1;
                    Ok(())
                })?;
                global = arg(1)?;
                i += 1;
            }
            b"-modify" => {
                modify = arg(1)?;
                i += 1;
            }
            b"-trimonly" => trim = 1,
            b"-tempo1" => tempo1 = 1,
            b"-nojump" => nojump = true,
            b"-nofit" => nofit = true,
            b"-name" => name = Some(arg(1)?),
            b"-fit" => {
                if fits.len() >= 100 {
                    crate::clkcorr::lifecycle_diagnostics(|d| {
                        d.stdout(b"Too many -fit options! Maximum is 100\n")
                    })?;
                    return Err(crate::clock_diagnostics::boundary("MAX_FITSET"));
                }
                fits.push(arg(1)?);
            }
            b"-set" => {
                if sets.len() >= 100 {
                    crate::clkcorr::lifecycle_diagnostics(|d| {
                        d.stdout(b"Too many -set options! Maximum is 100\n")
                    })?;
                    return Err(crate::clock_diagnostics::boundary("MAX_FITSET"));
                }
                sets.push((arg(1)?, extended(&arg(2)?)?));
            }
            b"-simulate" => simulate = number(&arg(1)?)?,
            b"-clock" => {
                let clock = arg(1)?;
                for p in &mut context.pulsars {
                    write_string(&mut p.clock[..], &clock, "clock")?;
                }
            }
            _ => (),
        }
        i += 1;
    }
    crate::clkcorr::lifecycle_diagnostics(|d| {
        d.log(b'd', "preProcess.C", 147, b"Parsed command line")
    })?;
    // preProcess.C:48 declares ndm without initialization and line 480 passes
    // it by value even without -setdm. That read is undefined before simple2's
    // short circuit. Match preProcessSimple's explicit no-table count of zero.
    let mut ndm = 0;
    let mut dm = vec![];
    let mut start = 0.0f32;
    for index in 0..context.pulsars.len() {
        let p = &mut context.pulsars[index];
        for label in 0..p.param.slots.len() {
            if *read(&p.param.slots[label].n_link_to, "nLinkTo")? > 0 {
                p.param.slots[label].cells[0].val =
                    crate::tempo2_util::get_parameter_value(p, label, 0)?;
                p.param.slots[label].cells[0].prefit =
                    Some(crate::tempo2_util::get_parameter_value(p, label, 0)?);
            }
        }
        if let Some(name) = &name {
            write_string(&mut p.name[..], name, "name")?;
        }
        if nojump {
            p.nJumps = Some(0);
        }
        if read_string(&p.whiteNoiseModelFile[..], "whiteNoiseModelFile")? != b"NULL" {
            read_white_noise_model_file(context, index, inputs)?;
        }
        let (p, rows) = context.pulsar_observations_mut(index)?;
        if p.dm_series_type == Some(SeriesType::series_undefined) {
            p.dm_series_type = Some(SeriesType::series_taylor_pn);
            if p.param.slots[param_dm as usize]
                .cells
                .iter()
                .skip(2)
                .any(|c| c.param_set != 0)
            {
                let name = read_string(&p.name[..], "name")?;
                crate::clkcorr::lifecycle_diagnostics(|d| {
                    d.log(b'w',"preProcess.C",171,&[b"PSR ".as_slice(),&name,b" uses DM2+ but does not define DM_SERIES. Assume Taylor. This has behaviour has changed since June 2020!\nSee https://bitbucket.org/psrsoft/tempo2/issues/27/tempo2-dm-polynomial-is-not-a-taylor\n"].concat())
                })?;
            }
        }
        if nofit {
            for (i, param) in p.param.slots.iter_mut().enumerate() {
                if i != param_start as usize && i != param_finish as usize {
                    for c in &mut param.cells[..param.a_size.unwrap() as usize] {
                        c.fit_flag = 0;
                    }
                }
            }
            for j in 0..=*read(&p.nJumps, "nJumps")? as usize {
                p.fitJump[j] = Some(0);
            }
            for j in 1..=*read(&p.nfdJumps, "nfdJumps")? as usize {
                p.fitfdJump[j] = Some(0);
            }
        }
        for name in &fits {
            for param in &mut p.param.slots {
                for c in &mut param.cells[..param.a_size.unwrap() as usize] {
                    if clock_text::c_bytes(&c.shortlabel).eq_ignore_ascii_case(name) {
                        if c.param_set != 1 {
                            c.param_set = 1;
                            c.val = Float::with_val(PRECISION_BITS, 0);
                            c.prefit = Some(c.val.clone());
                        }
                        c.fit_flag = 1;
                    }
                }
            }
        }
        if simulate != 0.0 {
            for o in &mut rows[..*read(&p.nobs, "nobs")? as usize] {
                let delta = (read(&o.sat, "sat")?.clone() * 0.003f64).sin() * (simulate / 86400.0);
                *o.sat.as_mut().unwrap() += delta;
            }
        }
        for (name, val) in &sets {
            if name.eq_ignore_ascii_case(b"NITS") {
                p.nits = Some(crate::ifteph::extended_i32(val, "NITS_int")?);
            }
            for param in &mut p.param.slots {
                for c in &mut param.cells[..param.a_size.unwrap() as usize] {
                    if clock_text::c_bytes(&c.shortlabel).eq_ignore_ascii_case(name) {
                        c.val = val.clone();
                        c.prefit = Some(val.clone());
                        c.param_set = 1;
                    }
                }
            }
        }
        pre_process_simple1(context, index, tempo1, last, root)?;
        if epoch != b"NONE" {
            crate::update_epoch::update_epoch_str(context, index, &epoch)?;
        }
        let p = &mut context.pulsars[index];
        for label in [param_pepoch, param_posepoch, param_dmepoch, param_track] {
            let c = &mut p.param.slots[label as usize].cells[0];
            if c.param_set == 1 && c.fit_flag == 1 {
                c.fit_flag = 0;
                crate::clkcorr::lifecycle_diagnostics(|d| {
                    let mut m = b"Warning: Cannot fit for ".to_vec();
                    m.extend(clock_text::c_bytes(&c.shortlabel).to_ascii_lowercase());
                    m.push(b'\n');
                    d.stdout(&m);
                })?;
            }
        }
        if !dmfile.is_empty() {
            let mut s = consume(
                Path::new(std::ffi::OsStr::from_bytes(&dmfile)),
                "DM table",
                inputs,
            )?;
            ndm = 0;
            dm.clear();
            let mut previous = vec![];
            loop {
                let (line, eof) = if let Some(line) = s.line_chunk(1000) {
                    let eof =
                        s.position == s.bytes.len() && line.len() < 999 && !line.ends_with(b"\n");
                    previous = line.clone();
                    (line, eof)
                } else {
                    (previous.clone(), true)
                };
                if line.first() == Some(&b'#') {
                    crate::clkcorr::lifecycle_diagnostics(|d| {
                        d.log(
                            b'd',
                            "preProcess.C",
                            302,
                            b"preProces():skipping line in dmfile\n",
                        )
                    })?;
                }
                if line.first() != Some(&b'#')
                    && let Some((mjd, n)) = clock_text::float_prefix(&line)
                    && let Some((v, _)) = clock_text::float_prefix(&line[n..])
                {
                    if ndm >= 10000 {
                        return Err(s.error("DM table", "source capacity"));
                    }
                    if ndm == 0 {
                        start = mjd;
                    }
                    dm.push(v);
                    ndm += 1;
                }
                if eof {
                    break;
                }
            }
        }
        weight_errors(context, index)?;
        if !modify.is_empty() {
            let mut s = consume(
                Path::new(std::ffi::OsStr::from_bytes(&modify)),
                "flag modifications",
                inputs,
            )
            .inspect_err(|_| {
                let _ = crate::clkcorr::lifecycle_diagnostics(|d| {
                    d.stdout(
                        &[
                            b"Unable to open >".as_slice(),
                            &modify,
                            b"< to modify the flags\n",
                        ]
                        .concat(),
                    )
                });
            })?;
            while let Some(id) = s.token() {
                let Some(val) = s.token() else { break };
                let a = s.double("lower MJD")?;
                let b = s.double("upper MJD")?;
                let Some(new) = s.token() else { break };
                let (p, rows) = context.pulsar_observations_mut(index)?;
                for o in &mut rows[..*read(&p.nobs, "nobs")? as usize] {
                    let sat = read(&o.sat, "sat")?.to_f64();
                    for j in 0..*read(&o.nFlags, "nFlags")? as usize {
                        if read_string(&o.flagID[j], "flagID")? == id
                            && read_string(&o.flagVal[j], "flagVal")? == val
                            && sat > a
                            && sat < b
                        {
                            write_string(&mut o.flagVal[j], &new, "flagVal")?;
                        }
                    }
                }
            }
        }
        pre_process_simple2(context, index, start, ndm, Some(&dm), trim, root)?;
        if !plugin.is_empty() {
            return Err(TimingError::new(
                Path::new(std::ffi::OsStr::from_bytes(&plugin)),
                None,
                "selection plugin",
                "external plugin ABI is unavailable at the source selection boundary",
            ));
        }
        if !select.is_empty() {
            crate::clkcorr::lifecycle_diagnostics(|d| {
                d.log(b'd', "preProcess.C", 540, b"Using select file")
            })?;
            crate::selection::use_select_file(
                Path::new(std::ffi::OsStr::from_bytes(&select)),
                context,
                inputs,
            )?;
        }
        if !select.is_empty() {
            crate::clkcorr::lifecycle_diagnostics(|d| {
                d.log(b'd', "preProcess.C", 542, b"Complete using select file")
            })?;
        }
        pre_process_simple3(context, index)?;
    }
    if !global.is_empty() {
        crate::clkcorr::lifecycle_diagnostics(|d| d.stdout(b"Setting global parameters\n"))?;
        crate::source_par::read_parfile_global(
            context,
            Path::new(std::ffi::OsStr::from_bytes(&global)),
            root,
            inputs,
        )?;
        crate::clkcorr::lifecycle_diagnostics(|d| {
            d.stdout(b"Complete setting global parameters\n")
        })?;
    }
    Ok(())
}

pub fn read_white_noise_model_file(
    context: &mut PulsarContext,
    index: usize,
    inputs: &mut Vec<ConsumedInput>,
) -> Result<(), TimingError> {
    let path = read_string(
        &context.pulsars[index].whiteNoiseModelFile[..],
        "whiteNoiseModelFile",
    )?;
    let mut s = consume(
        Path::new(std::ffi::OsStr::from_bytes(&path)),
        "white noise model",
        inputs,
    )
    .inspect_err(|_| {
        let _ = crate::clkcorr::lifecycle_diagnostics(|d| {
            d.stdout(
                &[
                    b"ERROR: Unable to open file white noise model file >".as_slice(),
                    &path,
                    b"<\n",
                ]
                .concat(),
            )
        });
    })?;
    crate::clkcorr::lifecycle_diagnostics(|d| {
        d.stdout(
            &[
                b"Reading white noise model from >".as_slice(),
                &path,
                b"<\n",
            ]
            .concat(),
        )
    })?;
    let (p, rows) = context.pulsar_observations_mut(index)?;
    let rows = &mut rows[..*read(&p.nobs, "nobs")? as usize];
    while let Some(key) = s.token() {
        match key.as_slice() {
            b"JITTER" | b"START_TIME_JITTER" => {
                let id = s.required_token("flag ID")?;
                let val = s.required_token("flag value")?;
                let integration = if key == b"JITTER" {
                    Some(s.required_token("integration flag")?)
                } else {
                    None
                };
                let w0 = s.double("w0")?;
                let t0 = if integration.is_some() {
                    s.double("t0")?
                } else {
                    0.0
                };
                for o in rows.iter_mut() {
                    let flags = flags(o)?;
                    if flags.iter().any(|(a, b)| a == &id && b == &val) {
                        let mut wn = w0;
                        if let Some(id) = &integration {
                            let mut tobs = -1.0;
                            for (a, b) in &flags {
                                if a == id
                                    && let Some((v, _)) = clock_text::number_prefix(b)
                                {
                                    tobs = v;
                                }
                            }
                            if tobs <= -1.0 || tobs.is_nan() {
                                continue;
                            }
                            wn = w0 * (t0 / tobs).sqrt();
                        }
                        let e0 = *read(&o.toaErr, "toaErr")? * 1e-6;
                        o.toaErr = Some((e0.powi(2) + wn.powi(2)).sqrt() / 1e-6);
                    }
                }
            }
            b"TOAERR_ZERO" => {
                for o in rows.iter_mut() {
                    o.toaErr = Some(0.0);
                }
            }
            _ => (),
        }
    }
    Ok(())
}

fn weight_errors(context: &mut PulsarContext, index: usize) -> Result<(), TimingError> {
    let (p, rows) = context.pulsar_observations_mut(index)?;
    let rows = &mut rows[..*read(&p.nobs, "nobs")? as usize];
    if *read(&p.nT2efac, "nT2efac")? > 0
        || *read(&p.nT2equad, "nT2equad")? > 0
        || p.T2globalEfac != Some(1.0)
    {
        crate::clkcorr::lifecycle_diagnostics(|d| {
            d.stdout(b"Updating TOA errors using T2EFAC, T2EQUAD and T2GLOBALEFAC\n")
        })?;
        for o in rows.iter_mut() {
            let mut err = *read(&o.toaErr, "toaErr")?;
            for (id, val) in flags(o)? {
                for k in 0..p.nT2equad.unwrap() as usize {
                    if id == read_string(&p.T2equadFlagID[k], "T2equadFlagID")?
                        && val == read_string(&p.T2equadFlagVal[k], "T2equadFlagVal")?
                    {
                        err = (err.powi(2) + read(&p.T2equadVal[k], "T2equadVal")?.powi(2)).sqrt();
                    }
                }
                for k in 0..p.nT2efac.unwrap() as usize {
                    if id == read_string(&p.T2efacFlagID[k], "T2efacFlagID")?
                        && val == read_string(&p.T2efacFlagVal[k], "T2efacFlagVal")?
                    {
                        err *= read(&p.T2efacVal[k], "T2efacVal")?;
                    }
                }
            }
            o.toaErr = Some(err * read(&p.T2globalEfac, "T2globalEfac")?);
        }
    }
    if *read(&p.nTNEF, "nTNEF")? > 0
        || *read(&p.nTNEQ, "nTNEQ")? > 0
        || *read(&p.nTNSQ, "nTNSQ")? > 0
        || *read(&p.TNGlobalEF, "TNGlobalEF")? > 0.0
        || *read(&p.TNGlobalEQ, "TNGlobalEQ")? != 0.0
    {
        crate::clkcorr::lifecycle_diagnostics(|d| {
            d.stdout(b"Updating TOA errors using TN parameters.\n")
        })?;
        for o in rows.iter_mut() {
            let mut err = *read(&o.toaErr, "toaErr")?;
            if p.TNGlobalEF.unwrap() > 0.0 {
                err *= p.TNGlobalEF.unwrap();
            }
            if p.TNGlobalEQ.unwrap() != 0.0 {
                let q = 10.0f64.powf(p.TNGlobalEQ.unwrap() + 6.0)
                    * 10.0f64.powf(p.TNGlobalEQ.unwrap() + 6.0);
                err = (err * err + q).sqrt();
            }
            let fs = flags(o)?;
            for (id, val) in &fs {
                for k in 0..p.nTNEF.unwrap() as usize {
                    if id == &read_string(&p.TNEFFlagID[k], "TNEFFlagID")?
                        && val == &read_string(&p.TNEFFlagVal[k], "TNEFFlagVal")?
                    {
                        err *= read(&p.TNEFVal[k], "TNEFVal")?;
                    }
                }
                for k in 0..p.nTNEQ.unwrap() as usize {
                    if id == &read_string(&p.TNEQFlagID[k], "TNEQFlagID")?
                        && val == &read_string(&p.TNEQFlagVal[k], "TNEQFlagVal")?
                    {
                        let v = *read(&p.TNEQVal[k], "TNEQVal")?;
                        let q = 10.0f64.powf(v + 6.0) * 10.0f64.powf(v + 6.0);
                        err = (err * err + q).sqrt();
                    }
                }
                for k in 0..p.nTNSQ.unwrap() as usize {
                    if id == &read_string(&p.TNSQFlagID[k], "TNSQFlagID")?
                        && val == &read_string(&p.TNSQFlagVal[k], "TNSQFlagVal")?
                    {
                        let mut tobs = 0.0;
                        for (a, b) in &fs {
                            if a.eq_ignore_ascii_case(b"-tobs") {
                                tobs = if b.eq_ignore_ascii_case(b"UNKNOWN") {
                                    1.0
                                } else {
                                    clock_text::number_prefix(b).map(|v| v.0).unwrap_or(0.0)
                                };
                            }
                        }
                        let v = *read(&p.TNSQVal[k], "TNSQVal")?;
                        let q = 10.0f64.powf(v + 6.0) * 10.0f64.powf(v + 6.0) / tobs;
                        err = (err * err + q).sqrt();
                    }
                }
            }
            o.toaErr = Some(err);
        }
    }
    if *read(&p.nTNSECORR, "nTNSECORR")? > 0 {
        for o in rows {
            let mut tobs = None;
            for (a, b) in flags(o)? {
                if a.eq_ignore_ascii_case(b"-tobs") {
                    tobs = Some(if b.eq_ignore_ascii_case(b"UNKNOWN") {
                        1.0
                    } else {
                        clock_text::number_prefix(&b).map(|v| v.0).unwrap_or(0.0)
                    });
                }
            }
            o.tobs =
                Some(tobs.ok_or_else(|| {
                    crate::clock_diagnostics::boundary("TNSECORR_tobs_unwritten")
                })?);
        }
    }
    Ok(())
}
