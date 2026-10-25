//! readParfile.C source readers at fda6ad4; canonical state and byte scanner.
//! Dispatch predicates retain source order, including shadowed keyword families.
use crate::{
    PRECISION_BITS, TimingError, clock_text,
    observation::{read, read_string, write_string},
    pulsar::SourcePulsar,
    source_text::{SourceText, extended},
    tempo2::{Label, Label::*, ParameterCell},
};
use rug::Float;
use std::path::Path;
#[path = "source_par_dispatch.rs"]
mod dispatch;

fn zero() -> Float {
    Float::with_val(PRECISION_BITS, 0)
}
fn cell(psr: &SourcePulsar, label: Label, index: usize) -> &ParameterCell {
    &psr.param.slots[label as usize].cells[index]
}
fn cell_mut(psr: &mut SourcePulsar, label: Label, index: usize) -> &mut ParameterCell {
    &mut psr.param.slots[label as usize].cells[index]
}
fn display_warning(psr: &SourcePulsar, key: &[u8], message: &[u8]) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| {
        s.diagnostics
            .display(1, key, message, b"", *read(&psr.noWarnings, "noWarnings")?)
    })
}
fn suffix(key: &[u8], offset: usize) -> Result<Option<i32>, TimingError> {
    Ok(clock_text::integer_prefix(key.get(offset..).unwrap_or_default())?.map(|v| v.0))
}

/// readValue: source uncertainty/link/period semantics, independently callable.
pub fn read_value(
    psr: &mut SourcePulsar,
    key: &[u8],
    s: &mut SourceText,
    label: Label,
    index: usize,
) -> Result<i32, TimingError> {
    let line = s
        .line_chunk(1000)
        .ok_or_else(|| s.error("readValue", "missing value line"))?;
    let tokens: Vec<_> = clock_text::tokens(&line).take(3).collect();
    let value = tokens
        .first()
        .ok_or_else(|| s.error("readValue", "missing value"))?;
    let k = key.to_ascii_uppercase();
    let linked = match (k.as_slice(), value.to_ascii_uppercase().as_slice()) {
        (b"DSHK", b"PX") => Some(param_px),
        (b"SINI", b"KIN") => Some(param_kin),
        (b"KIN", b"STIG") => Some(param_stig),
        (b"D_AOP", b"PBDOT") => Some(param_pbdot),
        (b"DMMODEL", b"DM") => Some(param_dm),
        _ => None,
    };
    if let Some(target) = linked {
        let p = &mut psr.param.slots[label as usize];
        let n = *read(&p.n_link_to, "nLinkTo")? as usize;
        *p.link_to
            .get_mut(n)
            .ok_or_else(|| s.error("linkTo", "capacity"))? = Some(target as i32);
        p.n_link_to = Some(n as i32 + 1);
        let p = &mut psr.param.slots[target as usize];
        let n = *read(&p.n_link_from, "nLinkFrom")? as usize;
        *p.link_from
            .get_mut(n)
            .ok_or_else(|| s.error("linkFrom", "capacity"))? = Some(label as i32);
        p.n_link_from = Some(n as i32 + 1);
        if label == param_dshk {
            let px = cell(psr, param_px, 0).val.clone();
            cell_mut(psr, label, 0).val = px;
            // readParfile.C:2778 resolves the link here, including the source
            // multiple-link failure before the prefit and paramSet writes.
            let linked = crate::tempo2_util::get_parameter_value(psr, label as usize, 0)?;
            cell_mut(psr, label, 0).prefit = Some(linked);
        }
        cell_mut(psr, label, index).param_set = 1;
        if label == param_dmmodel && tokens.len() == 2 && matches!(tokens[1], b"0" | b"1" | b"2") {
            cell_mut(psr, label, index).fit_flag = (tokens[1][0] - b'0') as i32;
        }
        return Ok(0);
    }
    if label == param_raj || label == param_decj {
        let (a, b) = if label == param_raj {
            (&mut psr.rajStrPre[..], &mut psr.rajStrPost[..])
        } else {
            (&mut psr.decjStrPre[..], &mut psr.decjStrPost[..])
        };
        write_string(a, value, "coordinate string")?;
        write_string(b, value, "coordinate string")?;
    }
    let f0 = cell(psr, param_f, 0).clone();
    let c = psr.param.slots[label as usize]
        .cells
        .get_mut(index)
        .ok_or_else(|| s.error("readValue", "parameter capacity"))?;
    read_numeric(c, &k, &tokens, s)?;
    if k == b"RAJ" && c.val < 0 {
        crate::clkcorr::lifecycle_diagnostics(|d| {
            d.stdout(
                &[
                    b"ERROR: have negative RAJ: ".as_slice(),
                    &clock_text::format_number(c.val.to_f64(), clock_text::NumberFormat::Fourteen),
                    b"\n",
                ]
                .concat(),
            )
        })?;
        return Ok(-1);
    }
    if matches!(k.as_slice(), b"P" | b"P0") {
        c.val = Float::with_val(PRECISION_BITS, 1) / &c.val;
        c.err *= &c.val;
        c.err *= &c.val;
    } else if matches!(k.as_slice(), b"P1" | b"PDOT") {
        if f0.param_set == 0 {
            crate::clkcorr::lifecycle_diagnostics(|d| {
                d.stdout(b"Error: Tempo2 used frequencies and frequency derivatives (instead of P0 and P1 etc.).\n       You must set P0 before using P1 (or just use F0 and F1 in any order).\n")
            })?;
            return Ok(-1);
        }
        c.val = -f0.val.clone() * &f0.val * &c.val * 1e-15f64;
        let a = (Float::with_val(PRECISION_BITS, 2) / &f0.val) * &c.val * &c.val * &f0.err;
        let b = f0.val.clone() * &f0.val * &c.err;
        c.err = a.clone() * a + b.clone() * b;
    }
    c.param_set = 1;
    c.prefit = Some(c.val.clone());
    Ok(0)
}

fn read_numeric(
    c: &mut ParameterCell,
    key: &[u8],
    t: &[&[u8]],
    s: &SourceText,
) -> Result<(), TimingError> {
    let normalized: Vec<Vec<u8>> = t
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .map(|&b| {
                    if (i == 0 || i == t.len() - 1) && (b == b'D' || b == b'd') {
                        b'e'
                    } else {
                        b
                    }
                })
                .collect()
        })
        .collect();
    let t: Vec<&[u8]> = normalized.iter().map(Vec::as_slice).collect();
    c.val = match key {
        b"RAJ" | b"DECJ" => {
            let mut text = t[0].to_vec();
            let turn = if key == b"RAJ" {
                hms_turn(&mut text)
            } else {
                dms_turn(&text)?
            };
            Float::with_val(
                PRECISION_BITS,
                crate::tempo2_util::turn_deg(turn) * std::f64::consts::PI / 180.0,
            )
        }
        b"EPHVER" => match t[0].to_ascii_uppercase().as_slice() {
            b"2" | b"TEMPO1" => Float::with_val(PRECISION_BITS, 2),
            b"5" | b"TEMPO2" => Float::with_val(PRECISION_BITS, 5),
            _ => {
                crate::clkcorr::lifecycle_diagnostics(|d| {
                    d.stdout(&[b"ERROR: Unknown EPHVER >".as_slice(), t[0], b"<\n"].concat())
                })?;
                return Err(s.error("EPHVER", "unknown ephemeris version"));
            }
        },
        _ => extended(t[0])?,
    };
    // readValue returns immediately after assigning a negative RA, before
    // resetting flags or errors (readParfile.C:2858-2863).
    if key == b"RAJ" && c.val < 0 {
        return Ok(());
    }
    c.fit_flag = 0;
    c.err = zero();
    if t.len() == 2 {
        if matches!(t[1], b"0" | b"1" | b"2") {
            c.fit_flag = (t[1][0] - b'0') as i32;
        } else {
            c.prefit_err = Some(extended(t[1])?);
        }
    } else if t.len() == 3 {
        if let Some((flag, _)) = clock_text::integer_prefix(t[1])? {
            c.fit_flag = flag;
        }
        c.prefit_err = Some(extended(t[2])?);
    }
    // readParfile.C::readValue:2901-2932 at fda6ad4 leaves prefitErr unwritten
    // without an uncertainty, then divides it for coordinates; checkAllSet also
    // copies it for default epochs. Supply the missing zero uncertainty only for
    // an unwritten destination. Defined retained uncertainties keep source behavior.
    if c.prefit_err.is_none() {
        c.prefit_err = Some(zero());
    }
    if key == b"RAJ" {
        *c.prefit_err.as_mut().unwrap() /= 12.0 * 60.0 * 60.0 / std::f64::consts::PI;
    }
    if key == b"DECJ" {
        *c.prefit_err.as_mut().unwrap() /= 180.0 * 60.0 * 60.0 / std::f64::consts::PI;
    }
    Ok(())
}

pub fn setup_parameter_file_defaults(
    psr: &mut SourcePulsar,
    root: &Path,
) -> Result<(), TimingError> {
    use std::os::unix::ffi::OsStrExt;
    write_string(&mut psr.clock[..], b"TT(TAI)", "clock")?;
    write_string(&mut psr.ephemeris[..], b"DE405", "ephemeris")?;
    write_string(
        &mut psr.JPL_EPHEMERIS[..],
        root.join("ephemeris/DE405.1950.2050")
            .as_os_str()
            .as_bytes(),
        "JPL_EPHEMERIS",
    )
}

pub struct EclipticParameters {
    longitude: ParameterCell,
    latitude: ParameterCell,
}
impl Default for EclipticParameters {
    fn default() -> Self {
        let mut p = crate::tempo2::SourceParameter::allocated(2, 100);
        Self {
            latitude: p.cells.pop().unwrap(),
            longitude: p.cells.pop().unwrap(),
        }
    }
}

/// readSimpleParfile: no default/reset wrapper, including repeated caller state.
pub fn read_simple_parfile(
    context: &mut crate::pulsar_lifecycle::PulsarContext,
    p: usize,
    s: &mut SourceText,
    root: &Path,
) -> Result<(), TimingError> {
    let mut coordinates = EclipticParameters::default();
    while let Some(key) = s.token() {
        check_line(context, p, &key, s, &mut coordinates, root)?;
    }
    check_all_set_named(&mut context.pulsars[p], &coordinates, s, b"")
}

pub fn check_all_set(
    psr: &mut SourcePulsar,
    ecl: &EclipticParameters,
    s: &SourceText,
) -> Result<(), TimingError> {
    use std::os::unix::ffi::OsStrExt;
    check_all_set_named(psr, ecl, s, s.path.as_os_str().as_bytes())
}
fn check_all_set_named(
    psr: &mut SourcePulsar,
    ecl: &EclipticParameters,
    s: &SourceText,
    filename: &[u8],
) -> Result<(), TimingError> {
    if cell(psr, param_pepoch, 0).param_set == 0 {
        crate::clkcorr::lifecycle_diagnostics(|d| {
            d.stdout(
                &[
                    b"ERROR [PAR1]: Have not set a period epoch in ".as_slice(),
                    filename,
                    b"\n",
                ]
                .concat(),
            )
        })?;
        return Err(s.error("PEPOCH", "period epoch not set"));
    }
    for (target, needed, short, label) in [
        (
            param_waveepoch,
            cell(psr, param_wave_om, 0).param_set == 1,
            b"WAVEEPOCH".as_slice(),
            b"WAVEEPOCH (MJD)".as_slice(),
        ),
        (
            param_waveepoch_dm,
            cell(psr, param_wave_dm, 0).param_set == 1,
            b"WAVEEPOCHDM",
            b"WAVEEPOCHDM (MJD)",
        ),
        (param_posepoch, true, b"POSEPOCH", b"POSEPOCH (MJD)"),
        (param_dmepoch, true, b"DMEPOCH", b"DMEPOCH (MJD)"),
    ] {
        if needed && cell(psr, target, 0).param_set == 0 {
            if matches!(target, param_posepoch | param_dmepoch) {
                let (key, message) = if target == param_posepoch {
                    (
                        b"PAR1".as_slice(),
                        b"Have not set a position epoch. The period epoch will be used instead."
                            .as_slice(),
                    )
                } else {
                    (
                        b"PAR2".as_slice(),
                        b"Have not set a DM epoch. The period epoch will be used instead."
                            .as_slice(),
                    )
                };
                crate::clkcorr::with_process(|session| {
                    session.diagnostics.display(
                        1,
                        key,
                        message,
                        filename,
                        *read(&psr.noWarnings, "noWarnings")?,
                    )
                })?;
            }
            let src = psr.param.slots[param_pepoch as usize].clone();
            crate::copy_param(&src, &mut psr.param.slots[target as usize])
                .map_err(|e| s.error("copyParam", e.to_string()))?;
            let c = cell_mut(psr, target, 0);
            c.label[..label.len()].copy_from_slice(label);
            c.label[label.len()] = 0;
            c.shortlabel[..short.len()].copy_from_slice(short);
            c.shortlabel[short.len()] = 0;
            if matches!(target, param_waveepoch | param_waveepoch_dm) {
                let value = c.val.to_f64();
                crate::clkcorr::lifecycle_diagnostics(|d| {
                    d.stdout(
                        &[
                            b"Setting waveepoch to ".as_slice(),
                            &clock_text::format_number(value, clock_text::NumberFormat::GeneralSix),
                            b"\n",
                        ]
                        .concat(),
                    )
                })?;
            }
        }
    }
    if ecl.longitude.param_set == 1 && ecl.latitude.param_set == 1 {
        for (label, v) in [(param_raj, &ecl.longitude), (param_decj, &ecl.latitude)] {
            let c = cell_mut(psr, label, 0);
            c.val = v.val.clone() * std::f64::consts::PI / 180.0;
            c.prefit = Some(c.val.clone());
            c.param_set = 1;
            if ecl.longitude.fit_flag == 1 || ecl.latitude.fit_flag == 1 {
                c.fit_flag = 1;
            }
        }
        psr.eclCoord = Some(1);
    } else if ecl.longitude.param_set == 1 || ecl.latitude.param_set == 1 {
        let name = read_string(&psr.name[..], "name")?;
        crate::clkcorr::lifecycle_diagnostics(|d| {
            d.log(
                b'e',
                "readParfile.C",
                2496,
                &[
                    b"Cannot mix ecliptic and equatorial coordinates ".as_slice(),
                    &name,
                ]
                .concat(),
            )
        })?;
        return Err(s.error(
            "coordinates",
            "cannot mix ecliptic and equatorial coordinates",
        ));
    }
    let clock = read_string(&psr.clock[..], "clock")?;
    if clock == b"UNCORR" {
        write_string(&mut psr.clock[..], b"TT(TAI)", "clock")?;
        write_string(&mut psr.clockFromOverride[..], b"UTC", "clockFromOverride")?;
    } else if !clock.starts_with(b"TT") {
        display_warning(
            psr,
            b"CLK1",
            &[
                b"CLK parameter '".as_slice(),
                &clock,
                b"' is not a realization of TT!",
            ]
            .concat(),
        )?;
        let replacement = match clock.as_slice() {
            b"UTC(NIST)" => b"TT(UTC(NIST))".as_slice(),
            b"PTB" => b"TT(UTC(PTB))",
            b"AT1" => b"TT(TA(NIST))",
            _ => b"TT(TAI)",
        };
        if psr.noWarnings != Some(2) {
            crate::clkcorr::lifecycle_diagnostics(|d| {
                d.stdout(
                    &[
                        b"-> Using ".as_slice(),
                        replacement,
                        b" instead.(You should set CLK to what you really mean!)\n",
                    ]
                    .concat(),
                )
            })?;
        }
        write_string(&mut psr.clock[..], replacement, "clock")?;
    }
    check_binary_parameter_consistency(psr, s)
}
fn check_binary_parameter_consistency(
    psr: &mut SourcePulsar,
    s: &SourceText,
) -> Result<(), TimingError> {
    let model = read_string(&psr.binaryModel[..], "binaryModel")?;
    if model.eq_ignore_ascii_case(b"DDGR")
        && psr.param.slots[param_fb as usize]
            .cells
            .iter()
            .any(|c| c.param_set == 1)
    {
        crate::clkcorr::lifecycle_diagnostics(|d| {
            d.log(b'e', "readParfile.C", 2395,
            b"BINARY DDGR does not support FB parameters; use PB/PBDOT or select a binary model that evaluates FBn.")
        })?;
        return Err(s.error("BINARY DDGR", "FB parameters are unsupported; use PB/PBDOT"));
    }
    if model.eq_ignore_ascii_case(b"ELL1")
        && cell(psr, param_fb, 1).param_set == 1
        && cell(psr, param_pbdot, 0).param_set == 1
    {
        display_warning(psr, b"BINFB1", b"Both PBDOT and FB1 are set for ELL1; explicit FB1 takes precedence and PBDOT is disabled.")?;
        let c = cell_mut(psr, param_pbdot, 0);
        c.param_set = 0;
        c.fit_flag = 0;
    }
    Ok(())
}

fn check_line_fields(
    psr: &mut SourcePulsar,
    key: &[u8],
    s: &mut SourceText,
    ecl: &mut EclipticParameters,
    root: &Path,
) -> Result<(), TimingError> {
    use std::os::unix::ffi::OsStrExt;
    let id = dispatch::dispatch(key);
    if dispatch::leaf(id, psr, key, s)? || dispatch::tables(id, psr, key, s)? {
        return Ok(());
    }
    match id {
        0 | 334 | 336 | 337 => {
            s.line_chunk(1000);
        }
        3 => {
            let path = s.required_token("EOP_FILE")?;
            write_string(&mut psr.eopc04_file[..], &path, "EOP_FILE")?;
            crate::clkcorr::lifecycle_diagnostics(|d| {
                d.stdout(
                    &[
                        b"WARNING: All pulsars will use the EOPC04 file: ".as_slice(),
                        &path,
                        b"\n",
                    ]
                    .concat(),
                )
            })?;
        }
        6 => {
            let line = s.line_chunk(80).unwrap_or_default();
            if let Some(&c) = line.iter().find(|&&c| c > 47) {
                psr.robust = Some(if c == b'0' { 0 } else { c });
            }
        }
        7 => cell_mut(psr, param_track, 0).param_set = 0,
        14 => {
            psr.units = Some(2);
            psr.timeEphemeris = Some(2);
            psr.dilateFreq = Some(0);
            psr.planetShapiro = Some(0);
            psr.t2cMethod = Some(2);
            psr.correctTroposphere = Some(0);
        }
        15 | 17 | 18 => {
            let token = s.required_token("boolean")?;
            let yes = i32::from(matches!(token.first(), Some(b'1' | b'y' | b'Y')));
            match id {
                15 => psr.dilateFreq = Some(yes),
                17 => psr.planetShapiro = Some(yes),
                _ => psr.correctTroposphere = Some(yes),
            }
        }
        19 => {
            psr.setUnits = Some(1);
            match s.required_token("UNITS")?.to_ascii_uppercase().as_slice() {
                b"TCB" | b"SI" => psr.units = Some(1),
                b"TDB" => psr.units = Some(2),
                _ => (),
            }
        }
        20 => {
            read_value(psr, key, s, param_ne_sw, 0)?;
            psr.ne_sw = Some(cell(psr, param_ne_sw, 0).val.to_f64());
        }
        22 => match s.required_token("TIMEEPH")?.to_ascii_uppercase().as_slice() {
            b"IF99" => psr.timeEphemeris = Some(1),
            b"FB90" => psr.timeEphemeris = Some(2),
            _ => (),
        },
        23 => match s
            .required_token("T2CMETHOD")?
            .to_ascii_uppercase()
            .as_slice()
        {
            b"IAU2000B" => psr.t2cMethod = Some(1),
            b"TEMPO" => {
                psr.t2cMethod = Some(2);
                if *read(&psr.useCalceph, "useCalceph")? != 0 {
                    display_warning(
                        psr,
                        b"T2C",
                        b"Using Calceph with TEMPO T2CMETHOD. Setting T2CMETHOD to IAU2000B.",
                    )?;
                    psr.t2cMethod = Some(1);
                }
            }
            _ => (),
        },
        24 => {
            let rest = s.line_chunk(1024).unwrap_or_default();
            crate::clkcorr::with_resources(root, |session| {
                session.define_sequence(Some(&rest), *read(&psr.noWarnings, "noWarnings")?)
            })?;
        }
        30 | 31 | 33 => {
            let name = s.required_token("ephemeris")?;
            write_string(&mut psr.ephemeris[..], &name, "ephemeris")?;
            let path = if id == 30 {
                let suffix = match name.as_slice() {
                    b"DE414" => b".1960.2020".as_slice(),
                    b"INPOP06" => b"",
                    _ => b".1950.2050",
                };
                let mut path = root.as_os_str().as_bytes().to_vec();
                path.extend_from_slice(b"/ephemeris/");
                path.extend(&name);
                path.extend(suffix);
                path
            } else {
                name
            };
            write_string(&mut psr.JPL_EPHEMERIS[..], &path, "JPL_EPHEMERIS")?;
            if id == 33 {
                psr.useCalceph = Some(1);
                if psr.t2cMethod == Some(2) {
                    display_warning(
                        psr,
                        b"T2C",
                        b"Using Calceph with TEMPO T2CMETHOD. Setting T2CMETHOD to IAU2000B.",
                    )?;
                    psr.t2cMethod = Some(1);
                }
            }
        }
        32 => {
            psr.AverageResiduals = Some(1);
            let mut scan = s.scan();
            scan.string(&mut psr.AverageFlag[..], "AverageFlag")?;
            scan.float(&mut psr.AverageEpochWidth);
        }
        35..=38 => {
            let ecliptic = !key.to_ascii_uppercase().starts_with(b"PMRA")
                && !key.to_ascii_uppercase().starts_with(b"PMDEC");
            if id == 35 || id == 36 {
                let mixed = if ecliptic {
                    cell(psr, param_raj, 0).param_set != 0
                        || cell(psr, param_decj, 0).param_set != 0
                        || (psr.eclCoord == Some(0)
                            && (cell(psr, param_pmra, 0).param_set != 0
                                || cell(psr, param_pmdec, 0).param_set != 0))
                } else {
                    psr.eclCoord != Some(0)
                        || ecl.latitude.param_set != 0
                        || ecl.longitude.param_set != 0
                };
                if mixed {
                    let name = read_string(&psr.name[..], "name")?;
                    crate::clkcorr::lifecycle_diagnostics(|d| {
                        if id == 35 && ecliptic {
                            d.log(
                                b'm',
                                "readParfile.C",
                                521,
                                format!(
                                    "ra_set={} dec_set={} ecl={} pmra_set={} pmdec_set={}",
                                    cell(psr, param_raj, 0).param_set,
                                    cell(psr, param_decj, 0).param_set,
                                    i32::from(psr.eclCoord == Some(0)),
                                    cell(psr, param_pmra, 0).param_set,
                                    cell(psr, param_pmdec, 0).param_set
                                )
                                .as_bytes(),
                            );
                        }
                        let line = match (id, ecliptic) {
                            (35, true) => 524,
                            (35, false) => 531,
                            (36, true) => 542,
                            _ => 548,
                        };
                        d.log(
                            b'e',
                            "readParfile.C",
                            line,
                            &[
                                b"Cannot mix ecliptic and equatorial coordinates ".as_slice(),
                                &name,
                            ]
                            .concat(),
                        );
                    })?;
                    return Err(s.error(
                        "proper motion",
                        "cannot mix ecliptic and equatorial coordinates",
                    ));
                }
            }
            let label = match id {
                35 => param_pmra,
                36 => param_pmdec,
                37 => param_pmra2,
                _ => param_pmdec2,
            };
            if ecliptic && id <= 36 {
                psr.eclCoord = Some(1);
            }
            read_value(psr, key, s, label, 0)?;
            if ecliptic {
                psr.eclCoord = Some(1);
            }
        }
        45..=47 => {
            let label = match id {
                45 => {
                    psr.setTelVelX = Some(0);
                    param_tel_dx
                }
                46 => {
                    psr.setTelVelY = Some(1);
                    param_tel_dy
                }
                _ => {
                    psr.setTelVelZ = Some(1);
                    param_tel_dz
                }
            };
            read_value(psr, key, s, label, 0)?;
        }
        48..=53 => {
            crate::clkcorr::lifecycle_diagnostics(|d| {
                let mut message = b"parameter '".to_vec();
                message.extend_from_slice(key);
                message.extend_from_slice(b"' not used");
                d.log(b'e', "readParfile.C", 593 + (id - 48) * 3, &message)
            })?;
            let label = [
                param_tel_vx,
                param_tel_vy,
                param_tel_vz,
                param_tel_x0,
                param_tel_y0,
                param_tel_z0,
            ][id - 48];
            read_value(psr, key, s, label, 0)?;
        }
        62 | 63 => {
            let line = s
                .line_chunk(1000)
                .ok_or_else(|| s.error("ecliptic", "missing value"))?;
            let t: Vec<_> = clock_text::tokens(&line).take(3).collect();
            if t.is_empty() {
                return Err(s.error("ecliptic", "missing value"));
            }
            let c = if id == 62 {
                &mut ecl.longitude
            } else {
                &mut ecl.latitude
            };
            read_numeric(c, key, &t, s)?;
            c.param_set = 1;
            c.prefit = Some(c.val.clone());
        }
        88 => {
            let value = s.required_token("REFPHS")?;
            match value.to_ascii_uppercase().as_slice() {
                b"MEAN" => psr.refphs = Some(0),
                b"TZR" => psr.refphs = Some(1),
                _ => crate::clkcorr::lifecycle_diagnostics(|d| {
                    d.log(
                        b'w',
                        "readParfile.C",
                        842,
                        &[b"Unknown REFPHS '".as_slice(), &value, b"'"].concat(),
                    )
                })?,
            }
        }
        124 | 125 | 130 | 136 => {
            let label = match id {
                124 | 125 => param_wave_om,
                130 => param_wave_dm,
                _ => param_dmmodel,
            };
            read_value(psr, key, s, label, 0)?;
            if id == 125 {
                cell_mut(psr, label, 0).val *= 2.0 * std::f64::consts::PI / 365.25;
            }
            if id == 124 || id == 125 {
                psr.waveScale = Some(0.0);
            }
            if id == 136 {
                psr.dmoffsCMnum = Some(0);
                psr.dmoffsDMnum = Some(0);
            }
        }
        142 => s.scan().double(&mut psr.constraint_efactor),
        168 => {
            psr.gwecc_pulsarTermOn = Some(s.integer("GWECC_PSRTERM")?);
            if cell(psr, param_gwecc, 0).param_set == 1 {
                let c = cell_mut(psr, param_gwecc, 0);
                c.label[..10].copy_from_slice(b"GWECC_AMP\0");
                c.shortlabel[..10].copy_from_slice(b"GWECC_AMP\0");
            }
        }
        254 => {
            let model = s.required_token("BINARY")?;
            if model.eq_ignore_ascii_case(b"BT1P") || model.eq_ignore_ascii_case(b"BT2P") {
                display_warning(psr, b"BIN3", b"Converting binary model to T2 model")?;
            }
            write_string(
                &mut psr.binaryModel[..],
                if model.eq_ignore_ascii_case(b"BT1P") || model.eq_ignore_ascii_case(b"BT2P") {
                    b"T2"
                } else {
                    &model
                },
                "BINARY",
            )?;
        }
        257 | 277 | 280 | 284 | 287 | 288 | 291 | 293 => {
            let (label, index) = match id {
                257 => (param_edot, 0),
                277 => (param_pbdot, 0),
                280 => (param_xpbdot, 0),
                284 => (param_a1dot, 0),
                287 => (param_a2dot, 0),
                288 => (param_a1dot, 1),
                291 => (param_eps1dot, 0),
                _ => (param_eps2dot, 0),
            };
            read_value(psr, key, s, label, index)?;
            let c = cell_mut(psr, label, index);
            if !matches!(id, 287 | 288) && c.val.clone().abs() > 1e-7f64 {
                c.val *= 1e-12f64;
            }
            if id != 257 {
                c.prefit = Some(c.val.clone());
            }
        }
        266 => {
            read_value(psr, key, s, param_pb, 0)?;
            if psr.nCompanion == Some(0) {
                psr.nCompanion = Some(1);
            }
        }
        335 => {
            let line = s.line_chunk(80).unwrap_or_default();
            let name = clock_text::tokens(&line)
                .next()
                .ok_or_else(|| s.error("DM_SERIES", "unwritten series name"))?;
            if !matches!(name.first(), Some(b'p' | b'P' | b't' | b'T')) {
                crate::clkcorr::lifecycle_diagnostics(|d| {
                    d.log(
                        b'w',
                        "readParfile.C",
                        2365,
                        &[
                            b"Not sure how to interpret '".as_slice(),
                            name,
                            b"' as a DM_SERIES... Assume Taylor.",
                        ]
                        .concat(),
                    )
                })?;
            }
            psr.dm_series_type = Some(match name.first() {
                Some(b'p' | b'P') => crate::pulsar::SeriesType::series_simple_pn,
                _ => crate::pulsar::SeriesType::series_taylor_pn,
            });
        }
        338 => {
            crate::clkcorr::with_process(|session| {
                session.diagnostics.display(
                    1,
                    b"MISC1",
                    b"Unknown parameter in par file: ",
                    key,
                    *read(&psr.noWarnings, "noWarnings")?,
                )
            })?;
        }
        _ => return check_complex(id, psr, key, s),
    }
    Ok(())
}

fn check_complex(
    id: usize,
    psr: &mut SourcePulsar,
    key: &[u8],
    s: &mut SourceText,
) -> Result<(), TimingError> {
    if let Some((label, offset, subtract, strict)) = indexed_parameter(id) {
        if let Some(n) = suffix(key, offset)? {
            let index = n - subtract;
            if index < 0 {
                return Err(s.error("parameter index", "negative source index"));
            }
            let size = psr.param.slots[label as usize].a_size.unwrap();
            if id == 117 {
                crate::clkcorr::lifecycle_diagnostics(|d| {
                    d.stdout(format!("Setting {n} {size}\n").as_bytes())
                })?;
            }
            if if strict { n < size } else { index < size } {
                read_value(psr, key, s, label, index as usize)?;
            } else if id == 97 {
                crate::clkcorr::lifecycle_diagnostics(|d| {
                    d.log(b'e', "readParfile.C", 869,
                    b"Currently only period derivatives up to order 12 are available. All higher derivatives will be ignored!")
                })?;
            }
            if id == 98 {
                psr.ndmx = Some(psr.ndmx.unwrap().max(index + 1));
            }
            if id == 101 {
                psr.nSx = Some(psr.nSx.unwrap().max(index + 1));
            }
            if id == 109 {
                psr.nchromx = Some(psr.nchromx.unwrap().max(index + 1));
            }
            if id == 267 && index < size {
                psr.nCompanion = Some(psr.nCompanion.unwrap().max(n));
            }
        }
        return Ok(());
    }

    match id {
        26 => read_jump(psr, key, s, None)?,
        29 => {
            let upper = key.to_ascii_uppercase();
            if let Some((n, consumed)) = clock_text::integer_prefix(&key[2..])?
                && consumed + 2 == key.len()
            {
                if n < psr.param.slots[param_f as usize].a_size.unwrap() {
                    if n < 1 {
                        return Err(s.error("FD", "negative parameter index"));
                    }
                    read_value(psr, key, s, param_fd, (n - 1) as usize)?;
                }
                return Ok(());
            }
            if upper.starts_with(b"FDJUMP_SCALE") || upper.starts_with(b"FDJUMPLOG") {
                let line = s.line_chunk(1024).unwrap_or_default();
                let scale = clock_text::tokens(&line)
                    .next()
                    .ok_or_else(|| s.error("FDJUMP", "missing scale"))?;
                if upper.starts_with(b"FDJUMPLOG") {
                    psr.fdjump_pint_format = Some(1);
                    psr.fdjump_log = Some(u8::from(!scale.eq_ignore_ascii_case(b"N")));
                } else {
                    psr.fdjump_log = Some(u8::from(!scale.eq_ignore_ascii_case(b"LINEAR")));
                }
            } else {
                let n = if upper.starts_with(b"FDJUMPDM") {
                    -2
                } else if upper.starts_with(b"FDJUMP") {
                    suffix(key, 6)?.unwrap_or(0)
                } else {
                    psr.fdjump_pint_format = Some(1);
                    suffix(key, 2)?.unwrap_or(0)
                };
                if n != 0 {
                    read_jump(psr, key, s, Some(n))?;
                }
            }
        }
        34 => {
            let line = s
                .line_chunk(1000)
                .ok_or_else(|| s.error("TOFFSET", "missing line"))?;
            let mut row = SourceText::new(&s.path, line.clone());
            let n = *read(&psr.nToffset, "nToffset")? as usize;
            if n >= psr.tOffset.len() {
                return Err(s.error("TOFFSET", "capacity"));
            }
            let mut scan = row.scan();
            scan.double(&mut psr.tOffset_f1[n]);
            scan.double(&mut psr.tOffset_f2[n]);
            scan.double(&mut psr.tOffset_t1[n]);
            scan.double(&mut psr.tOffset_t2[n]);
            scan.string(&mut psr.tOffsetSite[n], "tOffsetSite")?;
            scan.double(&mut psr.tOffset[n]);
            let line = line.strip_suffix(b"\n").unwrap_or(&line);
            let flags = line
                .iter()
                .position(|&b| b == b'-')
                .map(|i| &line[i..])
                .unwrap_or_default();
            write_string(&mut psr.tOffsetFlags[n], flags, "tOffsetFlags")?;
            psr.nToffset = Some(n as i32 + 1);
        }
        123 => {
            if let Some(n @ 1..=9) = suffix(key, 11)? {
                read_value(psr, key, s, param_dmassplanet, (n - 1) as usize)?;
            }
        }
        137..=139 => {
            let mjd = s.double("MJD")?;
            let value = s.double("offset")?;
            // DMOFF/_DM never read the local err; _CM initializes it to zero.
            let err = s.scan_double().unwrap_or(0.0);
            if id != 139 {
                let n = *read(&psr.dmoffsDMnum, "dmoffsDMnum")? as usize;
                *psr.dmoffsDM_mjd
                    .get_mut(n)
                    .ok_or_else(|| s.error("dmoffsDM", "capacity"))? = Some(mjd);
                psr.dmoffsDM[n] = Some(value);
                psr.dmoffsDM_error[n] = Some(0.0);
                psr.dmoffsDM_weight[n] = Some(1.0);
                psr.dmoffsDMnum = Some(n as i32 + 1);
            }
            if id != 138 {
                let n = *read(&psr.dmoffsCMnum, "dmoffsCMnum")? as usize;
                *psr.dmoffsCM_mjd
                    .get_mut(n)
                    .ok_or_else(|| s.error("dmoffsCM", "capacity"))? = Some(mjd);
                psr.dmoffsCM[n] = Some(if id == 137 { 0.0 } else { value });
                psr.dmoffsCM_error[n] = Some(if id == 137 { 0.0 } else { err });
                psr.dmoffsCM_weight[n] = Some(1.0);
                psr.dmoffsCMnum = Some(n as i32 + 1);
            }
        }
        141 => {
            let n = *read(&psr.ne_sw_ifuncN, "ne_sw_ifuncN")? as usize;
            let mjd = s.double("MJD")?;
            let value = s.double("solar offset")?;
            let err = s.double("solar error").unwrap_or(0.0);
            *psr.ne_sw_ifuncT
                .get_mut(n)
                .ok_or_else(|| s.error("ne_sw_ifunc", "capacity"))? = Some(mjd);
            psr.ne_sw_ifuncV[n] = Some(value);
            psr.ne_sw_ifuncE[n] = Some(err);
            psr.ne_sw_ifuncN = Some(n as i32 + 1);
        }
        201 => {
            let n = *read(&psr.nPhaseJump, "nPhaseJump")? as usize;
            let dir = s.integer("phaseJumpDir")?;
            *psr.phaseJumpDir
                .get_mut(n)
                .ok_or_else(|| s.error("phaseJump", "capacity"))? = Some(dir);
            psr.phaseJump[n] = Some(extended(&s.required_token("phaseJump")?)?);
            psr.phaseJumpID[n] = Some(-1);
            psr.nPhaseJump = Some(n as i32 + 1);
        }
        232..=235 => {
            use crate::t2fit_nestlike::*;
            let old = *read(&psr.TNsubtractPoly, "TNsubtractPoly")?;
            psr.TNsubtractPoly = Some(if id == 232 {
                let text = s.required_token("TNsubtractPoly")?;
                // strtoull base 0 consumes a prefix; failed conversion is zero.
                let negative = text.first() == Some(&b'-');
                let text = if matches!(text.first(), Some(b'+' | b'-')) {
                    &text[1..]
                } else {
                    &text[..]
                };
                let (radix, text) = if text.starts_with(b"0x") || text.starts_with(b"0X") {
                    (16, &text[2..])
                } else if text.starts_with(b"0") {
                    (8, text)
                } else {
                    (10, text)
                };
                let mut flags = 0u64;
                for &digit in text {
                    let Some(d) = (digit as char).to_digit(radix) else {
                        break;
                    };
                    flags = flags
                        .checked_mul(radix as u64)
                        .and_then(|v| v.checked_add(d as u64))
                        .unwrap_or(u64::MAX);
                }
                let flags = if negative {
                    flags.wrapping_neg()
                } else {
                    flags
                };
                t2EncodeTNsubtractPoly(
                    flags & 255,
                    t2DecodeTNRedSubPolyOrd(old),
                    t2DecodeTNDMSubPolyOrd(old),
                    t2DecodeTNChromSubPolyOrd(old),
                )
            } else {
                let order = s.integer("polynomial order")?;
                match id {
                    233 => t2SetTNRedSubPolyOrd(old, order),
                    234 => t2SetTNDMSubPolyOrd(old, order),
                    _ => t2SetTNChromSubPolyOrd(old, order),
                }
            });
        }
        247 => {
            crate::clkcorr::lifecycle_diagnostics(|d| {
                d.stdout(b"\nWARNING: Using tempo RNAMP parameter: setting TNRedC to 100!\n")
            })?;
            let t = s.required_token("RNAMP")?;
            let t: Vec<_> = t
                .iter()
                .map(|&b| if matches!(b, b'd' | b'D') { b'e' } else { b })
                .collect();
            if let Some((v, _)) = clock_text::number_prefix(&t) {
                psr.TNRedAmp = Some(v);
            }
            let value = *read(&psr.TNRedAmp, "TNRedAmp")?;
            psr.TNRedAmp = Some(
                (2.0 * std::f64::consts::PI * 3.0f64.powf(0.5) / (86400.0 * 365.25 * 1e6) * value)
                    .log10(),
            );
            psr.TNRedC = Some(100);
        }
        248 => {
            s.scan().double(&mut psr.TNRedGam);
            psr.TNRedGam = Some(-*read(&psr.TNRedGam, "TNRedGam")?);
        }
        253 => {
            let n = *read(&psr.nTNShapeletEvents, "nTNShapeletEvents")? as usize;
            let count = s.integer("TNShapeletEvN")?;
            *psr.TNShapeletEvN
                .get_mut(n)
                .ok_or_else(|| s.error("TNShapeletEvents", "capacity"))? = Some(count);
            psr.TNShapeletEvPos[n] = Some(s.double("TNShapeletEvPos")?);
            psr.TNShapeletEvWidth[n] = Some(s.double("TNShapeletEvWidth")?);
            psr.TNShapeletEvFScale[n] = Some(s.double("TNShapeletEvFScale")?);
            if count > crate::pulsar::MAX_TNShapeCoef as i32 {
                crate::clkcorr::lifecycle_diagnostics(|d| {
                    d.log(
                        b'e',
                        "readParfile.C",
                        1989,
                        format!(
                            "Max number of Shapelet coefs is {}",
                            crate::pulsar::MAX_TNShapeCoef
                        )
                        .as_bytes(),
                    )
                })?;
            }
            let count = count.min(crate::pulsar::MAX_TNShapeCoef as i32);
            psr.TNShapeletEvN[n] = Some(count);
            for i in 0..count {
                match s.scan_double() {
                    Some(v) => psr.TNShapeletEvCoef[n][i as usize] = Some(v),
                    None => {
                        crate::clkcorr::lifecycle_diagnostics(|d| {
                            d.log(b'w',"readParfile.C",1995,format!("Couldn't read Shape coef for shape {n} coef={i}; setting coefs to zero.").as_bytes())
                        })?;
                        break;
                    }
                }
            }
            if let Ok(flag) = s.integer("shapelet fit flag") {
                psr.TNShapeletEvFitFlag[n] = Some(flag);
            }
            if !matches!(psr.TNShapeletEvFScale[n], Some(0.0 | 2.0)) {
                crate::clkcorr::lifecycle_diagnostics(|d| {
                    d.log(b'e', "readParfile.C", 2009,
                        b"Currently only can handle shapelets with freq scale 2.0 (DM) or 0.0 (red)")
                })?;
                return Err(s.error("shapelet", "source supports frequency scale 0 or 2"));
            }
            psr.nTNShapeletEvents = Some(n as i32 + 1);
        }
        259 => {
            let offset = if key.get(1) == Some(&b'_') { 2 } else { 4 };
            if let Some(n) = suffix(key, offset)? {
                if n < 1 {
                    return Err(s.error("ECC", "negative index"));
                }
                if n - 1 < psr.param.slots[param_ecc as usize].a_size.unwrap() {
                    read_value(psr, key, s, param_ecc, (n - 1) as usize)?;
                }
            }
        }
        268 => {
            let n = if key.len() == 2 {
                0
            } else {
                suffix(key, 2)?.ok_or_else(|| s.error("FB", "index conversion"))?
            };
            if n < 0 {
                return Err(s.error("FB", "negative index"));
            }
            read_value(psr, key, s, param_fb, n as usize)?;
        }
        _ => {
            return Err(s.error(
                String::from_utf8_lossy(key).as_ref(),
                format!("unhandled source dispatch D{id:03}"),
            ));
        }
    }
    Ok(())
}

fn indexed_parameter(id: usize) -> Option<(Label, usize, i32, bool)> {
    Some(match id {
        64..=85 => {
            let labels = [
                param_glep,
                param_glph,
                param_glf0,
                param_glf1,
                param_glf2,
                param_glf0d,
                param_glf0d2,
                param_glf0d3,
                param_gltd,
                param_gltd2,
                param_gltd3,
                param_nudot_amp,
                param_nudot_transition_time,
                param_nudot_epoch,
                param_expep,
                param_expph,
                param_exptau,
                param_expindex,
                param_gausep,
                param_gausamp,
                param_gaussig,
                param_gausindex,
            ];
            let offsets = [
                5, 5, 5, 5, 5, 6, 7, 7, 5, 6, 6, 10, 11, 12, 6, 6, 7, 9, 7, 8, 8, 10,
            ];
            (labels[id - 64], offsets[id - 64], 1, true)
        }
        97 => (param_f, 1, 0, false),
        98 => (param_dmx, 4, 1, false),
        99 => (param_dmxr1, 6, 1, false),
        100 => (param_dmxr2, 6, 1, false),
        101 => (param_sx, 3, 1, false),
        102 => (param_sxr1, 5, 1, false),
        103 => (param_sxr2, 5, 1, false),
        104 => (param_sxer, 5, 1, false),
        106 => (param_dm, 2, 0, false),
        108 => (param_cm, 2, 0, false),
        109 => (param_chromx, 7, 1, false),
        110 => (param_chromxr1, 9, 1, false),
        111 => (param_chromxr2, 9, 1, false),
        113 => (param_telx, 4, 0, false),
        115 => (param_tely, 4, 0, false),
        117 => (param_telz, 4, 0, false),
        256 => (param_a1, 3, 1, false),
        261 => (param_t0, 3, 1, false),
        264 => (param_om, 3, 1, false),
        267 => (param_pb, 3, 1, false),
        278 => (param_pbdot, 6, 1, false),
        285 => (param_a1dot, 5, 1, false),
        286 => (param_a2dot, 6, 1, false),
        _ => return None,
    })
}

fn read_jump(
    psr: &mut SourcePulsar,
    key: &[u8],
    s: &mut SourceText,
    fd: Option<i32>,
) -> Result<(), TimingError> {
    let line = s
        .line_chunk(1024)
        .ok_or_else(|| s.error("JUMP", "missing line"))?;
    let line = line.strip_suffix(b"\n").unwrap_or(&line);
    let n = if fd.is_some() {
        let n = *read(&psr.nfdJumps, "nfdJumps")? + 1;
        psr.nfdJumps = Some(n);
        n
    } else {
        let n = *read(&psr.nJumps, "nJumps")? + 1;
        psr.nJumps = Some(n);
        n
    } as usize;
    let (strings, values, fits) = if let Some(index) = fd {
        psr.fdjumpIdx[n] = Some(index);
        (
            &mut psr.fdjumpStr[..],
            &mut psr.fdjumpVal[..],
            &mut psr.fitfdJump[..],
        )
    } else {
        (
            &mut psr.jumpStr[..],
            &mut psr.jumpVal[..],
            &mut psr.fitJump[..],
        )
    };
    write_string(
        strings
            .get_mut(n)
            .ok_or_else(|| s.error("JUMP", "capacity"))?,
        line,
        "jumpStr",
    )?;
    fits[n] = Some(1);
    let tokens: Vec<_> = clock_text::tokens(line).take(5).collect();
    let kind = tokens
        .first()
        .ok_or_else(|| s.error("JUMP", "empty selector"))?
        .to_ascii_uppercase();
    let vi = if matches!(kind.as_slice(), b"MJD" | b"FREQ") {
        Some(3)
    } else if matches!(kind.as_slice(), b"NAME" | b"TEL")
        || fd.is_none() && kind == b"SCALED"
        || kind.starts_with(b"-")
    {
        Some(2)
    } else {
        None
    };
    if let Some(vi) = vi
        && tokens.len() > vi
    {
        if let Some((v, _)) = clock_text::number_prefix(tokens[vi]) {
            values[n] = Some(v);
        }
        fits[n] = Some(i32::from(
            tokens
                .get(vi + 1)
                .and_then(|v| clock_text::integer_prefix(v).ok().flatten())
                .is_some_and(|v| v.0 == 1),
        ));
    }
    if fd.is_none() && key.eq_ignore_ascii_case(b"SATJUMP") {
        psr.jumpSAT[n] = Some(1);
    }
    Ok(())
}

/// checkLine with the canonical owner required by allocated constraint payloads.
pub fn check_line(
    context: &mut crate::pulsar_lifecycle::PulsarContext,
    p: usize,
    key: &[u8],
    s: &mut SourceText,
    ecl: &mut EclipticParameters,
    root: &Path,
) -> Result<(), TimingError> {
    if dispatch::dispatch(key) != 143 {
        return check_line_fields(&mut context.pulsars[p], key, s, ecl, root).map_err(|mut e| {
            if e.path.as_os_str().is_empty() {
                e.path = s.path.clone();
                e.line = Some(s.line());
                e.message = format!("{}: {}", String::from_utf8_lossy(key), e.message);
            }
            e
        });
    }
    use crate::pulsar::Constraint::*;
    let name = s.required_token("CONSTRAIN")?;
    if name.eq_ignore_ascii_case(b"AUTO") {
        context.pulsars[p].auto_constraints = Some(1);
    }
    let special = match name.to_ascii_uppercase().as_slice() {
        b"PARAM" => Some(constraint_param),
        b"NE_SW_IFUNC_SIN" => Some(constraint_ne_sw_ifunc_sin),
        b"NE_SW_IFUNC_SIGMA" => Some(constraint_ne_sw_ifunc_sigma),
        b"IFUNC_COV" => Some(constraint_ifunc_cov),
        _ => None,
    };
    if let Some(constraint) = special {
        let text = s
            .line_chunk(1024)
            .ok_or_else(|| s.error("CONSTRAIN", "missing payload"))?;
        let mut bytes: Vec<_> = text.iter().copied().map(Some).collect();
        if constraint == constraint_ifunc_cov {
            bytes.push(Some(b'\n'));
            bytes.push(None);
        } else {
            bytes.push(Some(0));
            bytes.push(None);
        }
        let owner = context
            .allocate_payload(bytes)
            .map_err(|e| s.error("CONSTRAIN", e.to_string()))?;
        let psr = &mut context.pulsars[p];
        let n = *read(&psr.nconstraints, "nconstraints")? as usize;
        *psr.constraints
            .get_mut(n)
            .ok_or_else(|| s.error("constraints", "capacity"))? = Some(constraint);
        psr.constraint_special[n] = Some(Some(owner));
        psr.nconstraints = Some(n as i32 + 1);
    } else {
        let psr = &mut context.pulsars[p];
        for constraint in dispatch::constraint_values(&name) {
            let n = *read(&psr.nconstraints, "nconstraints")? as usize;
            *psr.constraints
                .get_mut(n)
                .ok_or_else(|| s.error("constraints", "capacity"))? = Some(constraint);
            psr.nconstraints = Some(n as i32 + 1);
        }
    }
    Ok(())
}

/// Full readParfile wrapper, including HEAD/fixed-header fallback and resets.
pub fn read_parfiles(
    context: &mut crate::pulsar_lifecycle::PulsarContext,
    pairs: &[crate::bats::InputPair],
    root: &Path,
    inputs: &mut Vec<crate::bats::ConsumedInput>,
) -> Result<(), TimingError> {
    use std::os::unix::ffi::OsStrExt;
    crate::clkcorr::lifecycle_diagnostics(|d| d.version("readParfile.C", "readParfile()"))?;
    let mut no_read = 0;
    for (p, pair) in pairs.iter().enumerate() {
        let mut ecl = EclipticParameters::default();
        let psr = &mut context.pulsars[p];
        psr.nPhaseJump = Some(0);
        psr.nJumps = Some(0);
        for par in &mut psr.param.slots {
            par.n_link_to = Some(0);
            par.n_link_from = Some(0);
        }
        let mut source = match crate::bats::consume(&pair.par, "PAR", inputs) {
            Ok(s) => s,
            Err(_) => {
                let mut s = match crate::bats::consume(&pair.tim, "TIM parameter header", inputs) {
                    Ok(s) => s,
                    Err(e) => {
                        crate::clkcorr::lifecycle_diagnostics(|d| {
                            d.stdout(
                                &[
                                    b"ERROR [FILE3]: Unable to open parfile ".as_slice(),
                                    pair.par.as_os_str().as_bytes(),
                                    format!(" for pulsar {p}\n").as_bytes(),
                                ]
                                .concat(),
                            )
                        })?;
                        return Err(e);
                    }
                };
                let header = s
                    .line_chunk(1000)
                    .ok_or_else(|| s.error("header", "empty TIM"))?;
                let header = header.strip_suffix(b"\n").unwrap_or(&header);
                if header.eq_ignore_ascii_case(b"HEAD") {
                    crate::clkcorr::lifecycle_diagnostics(|d| {
                        d.stdout(b"Reading from .tpo format file\n")
                    })?;
                    no_read = -1;
                } else {
                    crate::clkcorr::lifecycle_diagnostics(|d| {
                        d.stdout(
                            &[
                                b"Attempting to read parameters from fixed format header in "
                                    .as_slice(),
                                pair.tim.as_os_str().as_bytes(),
                                b"\n",
                            ]
                            .concat(),
                        )
                    })?;
                    if header.len() >= 27
                        && header[0] == b'1'
                        && matches!(header[1], b'0' | b'1')
                        && matches!(header[2], b'0' | b'1')
                    {
                        for (offset, label, index) in [
                            (1, param_f, 0),
                            (2, param_f, 1),
                            (3, param_f, 2),
                            (4, param_raj, 0),
                            (5, param_decj, 0),
                            (6, param_pmra, 0),
                            (7, param_pmdec, 0),
                            (8, param_a1, 0),
                            (9, param_ecc, 0),
                            (10, param_t0, 0),
                            (11, param_pb, 0),
                            (12, param_om, 0),
                            (13, param_omdot, 0),
                            (14, param_gamma, 0),
                            (15, param_dm, 0),
                            (16, param_px, 0),
                            (17, param_pbdot, 0),
                            (19, param_sini, 0),
                            (20, param_mtot, 0),
                            (21, param_m2, 0),
                            (22, param_dtheta, 0),
                        ] {
                            if header[offset] == b'1' {
                                cell_mut(psr, label, index).fit_flag = 1;
                            }
                        }
                        if let Some(model) = match header[26] {
                            b'1' => Some(b"BT".as_slice()),
                            b'2' => Some(b"EH".as_slice()),
                            b'3' => Some(b"DD".as_slice()),
                            b'4' => Some(b"DDGR".as_slice()),
                            b'5' => Some(b"H88".as_slice()),
                            b'6' => Some(b"BT+".as_slice()),
                            b'7' => Some(b"DDT".as_slice()),
                            b'8' => Some(b"DD+".as_slice()),
                            b'9' => Some(b"BT2P".as_slice()),
                            _ => None,
                        } {
                            write_string(&mut psr.binaryModel[..], model, "binaryModel")?;
                        }
                        write_string(&mut psr.clock[..], b"TT(TAI)", "clock")?;
                        use std::os::unix::ffi::OsStrExt;
                        write_string(
                            &mut psr.JPL_EPHEMERIS[..],
                            root.join("ephemeris/DE200.1950.2050")
                                .as_os_str()
                                .as_bytes(),
                            "JPL_EPHEMERIS",
                        )?;
                        let row = s
                            .line_chunk(1000)
                            .ok_or_else(|| s.error("header", "missing position row"))?;
                        write_string(&mut psr.name[..], &row, "name")?;
                        psr.name[12] = Some(0);
                        for (a, b, label) in [
                            (21, 40, param_raj),
                            (41, 60, param_decj),
                            (61, 70, param_pmra),
                            (71, 80, param_pmdec),
                        ] {
                            get_value(&row, a, b, psr, label, 0)?;
                        }
                        let row = s
                            .line_chunk(1000)
                            .ok_or_else(|| s.error("header", "missing spin row"))?;
                        for (a, b, label, index) in [
                            (2, 20, param_f, 0),
                            (21, 40, param_f, 1),
                            (41, 60, param_pepoch, 0),
                            (61, 70, param_f, 2),
                            (71, 80, param_px, 0),
                        ] {
                            get_value(&row, a, b, psr, label, index)?;
                        }
                        let row = s
                            .line_chunk(1000)
                            .ok_or_else(|| s.error("header", "missing DM row"))?;
                        get_value(&row, 9, 20, psr, param_dm, 0)?;
                        psr.fixedFormat = Some(4);
                        let epoch = cell(psr, param_pepoch, 0).clone();
                        let c = cell_mut(psr, param_posepoch, 0);
                        c.val = epoch.val;
                        c.prefit = epoch.prefit;
                        c.param_set = 1;
                        no_read = 1;
                    } else {
                        crate::clkcorr::lifecycle_diagnostics(|d| {
                            d.stdout(b"Unable to read pulsar timing model\n")
                        })?;
                        return Err(s.error("header", "unable to read timing model"));
                    }
                }
                s
            }
        };
        if no_read != 1 && context.pulsars[p].fixedFormat == Some(0) {
            setup_parameter_file_defaults(&mut context.pulsars[p], root)?;
            while let Some(key) = source.token() {
                let end = no_read == -1 && key.eq_ignore_ascii_case(b"TOAS");
                check_line(context, p, &key, &mut source, &mut ecl, root)?;
                if end {
                    break;
                }
            }
            check_all_set_named(
                &mut context.pulsars[p],
                &ecl,
                &source,
                pair.par.as_os_str().as_bytes(),
            )?;
        }
    }
    Ok(())
}

/// readParfileGlobal deliberately does not apply checkAllSet/default/reset logic.
pub fn read_parfile_global(
    context: &mut crate::pulsar_lifecycle::PulsarContext,
    path: &Path,
    root: &Path,
    inputs: &mut Vec<crate::bats::ConsumedInput>,
) -> Result<(), TimingError> {
    for p in 0..context.pulsars.len() {
        let mut s = match crate::bats::consume(path, "global PAR", inputs) {
            Ok(s) => s,
            Err(e) => {
                use std::os::unix::ffi::OsStrExt;
                crate::clkcorr::lifecycle_diagnostics(|d| {
                    d.stdout(
                        &[
                            b"ERROR: unable to open file >".as_slice(),
                            path.as_os_str().as_bytes(),
                            b"<\n",
                        ]
                        .concat(),
                    )
                })?;
                return Err(e);
            }
        };
        let mut ecl = EclipticParameters::default();
        while let Some(key) = s.token() {
            // readParfile.C::readParfileGlobal:133 has unallocated elong/elat.
            // Reject the actual scratch-dependent branch instead of inventing
            // values or dereferencing native-invalid storage.
            if matches!(dispatch::dispatch(&key), 35 | 36 | 62 | 63) {
                return Err(s.error(
                    "global PAR",
                    "source-global ecliptic scratch is unallocated",
                ));
            }
            check_line(context, p, &key, &mut s, &mut ecl, root)?;
        }
        check_binary_parameter_consistency(&mut context.pulsars[p], &s)?;
    }
    Ok(())
}

pub fn get_value(
    line: &[u8],
    v1: usize,
    v2: usize,
    psr: &mut SourcePulsar,
    label: Label,
    index: usize,
) -> Result<(), TimingError> {
    let fail = || crate::clock_diagnostics::boundary("fixed_parameter_field_extent");
    let segment = line.get(v1 - 1..v2).ok_or_else(fail)?;
    if label == param_raj || label == param_decj {
        let mut token = line.get(v1..v1 + 2).ok_or_else(fail)?.to_vec();
        token.push(b':');
        token.extend(line.get(v1 + 2..v1 + 4).ok_or_else(fail)?);
        token.push(b':');
        token.extend(line.get(v1 + 4..v2).ok_or_else(fail)?);
        write_string(
            if label == param_raj {
                &mut psr.rajStrPre[..]
            } else {
                &mut psr.decjStrPre[..]
            },
            &token,
            "coordinate",
        )?;
        let mut scratch = cell(psr, label, index).clone();
        let source = SourceText::new(Path::new("fixed header"), vec![]);
        read_numeric(
            &mut scratch,
            if label == param_raj { b"RAJ" } else { b"DECJ" },
            &[&token],
            &source,
        )?;
        let c = cell_mut(psr, label, index);
        c.val = scratch.val;
        c.param_set = 1;
        if label == param_raj && c.val < 0 {
            return Ok(());
        }
        c.prefit = Some(c.val.clone());
    } else if let Some(token) = clock_text::tokens(segment).next() {
        let value = extended(token)?;
        let f0 = cell(psr, param_f, 0).val.clone();
        let f1 = cell(psr, param_f, 1).val.clone();
        let c = cell_mut(psr, label, index);
        c.val = value;
        c.param_set = 1;
        if label == param_f {
            c.val = match index {
                0 => Float::with_val(PRECISION_BITS, 1) / &c.val,
                1 => -f0.clone() * &f0 * &c.val * 1e-15f64,
                2 => (f1.clone() * 2.0 * &f1 / &f0 - c.val.clone() * &f0 * &f0) * 1e-30f64,
                _ => c.val.clone(),
            };
        } else if label == param_pepoch {
            c.val -= 2400000.5f64;
        }
        c.prefit = Some(c.val.clone());
    }
    Ok(())
}

/// tempo2Util.C::hms_turn mutates colons and accepts partial sexagesimal input.
pub fn hms_turn(line: &mut [u8]) -> f64 {
    for b in line.iter_mut() {
        if *b == b':' {
            *b = b' ';
        }
    }
    let mut s = SourceText::new(Path::new("hms_turn"), line.to_vec());
    let mut turn = 0.0;
    for (i, divisor) in [24.0, 1440.0, 86400.0].into_iter().enumerate() {
        if let Ok(v) = s.double("coordinate") {
            if i == 0 {
                turn = v / divisor;
            } else {
                turn += v / divisor;
            }
        } else {
            if i == 0 && clock_text::tokens(line).next().is_some() {
                return 1.0;
            }
            break;
        }
    }
    turn
}
pub fn dms_turn(line: &[u8]) -> Result<f64, TimingError> {
    let line = clock_text::c_bytes(line);
    if line.len() >= 40 {
        return Err(crate::clock_diagnostics::boundary("dms_turn_buffer"));
    }
    let mut line = line.to_vec();
    for b in &mut line {
        if *b == b':' {
            *b = b' ';
        }
    }
    let sign = if let Some(i) = line.iter().position(|&b| b == b'-') {
        line[i] = b' ';
        -1.0
    } else {
        1.0
    };
    let mut s = SourceText::new(Path::new("dms_turn"), line.clone());
    let mut turn = 0.0;
    let mut count = 0;
    for divisor in [360.0, 21600.0, 1296000.0] {
        if let Ok(v) = s.double("coordinate") {
            if count == 0 {
                turn = v / divisor;
            } else {
                turn += v / divisor;
            }
            count += 1;
        } else {
            break;
        }
    }
    if count == 0 {
        return Ok(if clock_text::tokens(&line).next().is_some() {
            1.0
        } else {
            0.0
        });
    }
    if turn >= 1.0 {
        turn -= 1.0;
    }
    Ok(turn * sign)
}
pub fn remove_cr(line: &mut Vec<u8>) -> Result<(), TimingError> {
    if line.is_empty() {
        return Err(crate::clock_diagnostics::boundary("removeCR_empty_string"));
    }
    if line.last() == Some(&b'\n') {
        line.pop();
    }
    Ok(())
}

#[cfg(test)]
#[path = "source_parameter_corpus_tests.rs"]
mod parameter_corpus;
