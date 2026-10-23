//! Supported readParfile.C / readValue path.
use crate::input::ignored_line;
use crate::longdouble::parse_longdouble;
use crate::tempo2_util::{dms_turn, hms_turn, parse_binary64, turn_deg};
use crate::{
    Conventions, PRECISION_BITS, Parameter, ParameterLine, Scalar, SourceFile, TimingError,
};
use rug::Float;
use std::collections::BTreeMap;

const NUMERIC: [&str; 13] = [
    "F0", "F1", "F2", "F3", "PEPOCH", "RAJ", "DECJ", "POSEPOCH", "DM", "DMEPOCH", "EPHVER",
    "TZRMJD", "TZRFRQ",
];

/// PAR controls from readParfile.C; source spelling/order is retained.
#[derive(Clone, Debug, serde::Serialize)]
pub struct FitControls {
    pub mode: u8,
    pub nits: usize,
    pub mode_defaulted: bool,
    pub nits_defaulted: bool,
    pub declarations: Vec<ParameterLine>,
}

impl Default for FitControls {
    fn default() -> Self {
        Self {
            mode: 0,
            nits: 1,
            mode_defaulted: true,
            nits_defaulted: true,
            declarations: Vec::new(),
        }
    }
}

/// Validated PAR contents used to assemble the owned TimingInput.
pub(crate) struct ParFile {
    pub param: BTreeMap<String, Parameter>,
    pub text_parameters: BTreeMap<String, ParameterLine>,
    pub conventions: Conventions,
    pub fit_controls: FitControls,
}

fn read_position(token: &str, ra: bool) -> Result<Scalar, String> {
    let parts: Vec<_> = token.split(':').collect();
    if parts.len() != 3 {
        return Err("expected hh:mm:ss for RAJ or signed dd:mm:ss for DECJ".into());
    }
    let first = parse_binary64(parts[0])?;
    let min = parse_binary64(parts[1])?;
    let sec = parse_binary64(parts[2])?;
    let limit = if ra { 24.0 } else { 90.0 };
    if !(0.0..60.0).contains(&min)
        || !(0.0..60.0).contains(&sec)
        || first.fract() != 0.0
        || min.fract() != 0.0
        || first.abs() > limit
        || (ra && (first.is_sign_negative() || first == limit))
        || (!ra && first.abs() == limit && (min != 0.0 || sec != 0.0))
    {
        return Err("sexagesimal coordinate outside the supported range".into());
    }
    // readValue -> hms_turn/dms_turn -> turn_deg: preserve binary64 operation order.
    let turn = if ra {
        hms_turn(first, min, sec)
    } else {
        dms_turn(
            first,
            min,
            sec,
            if parts[0].starts_with('-') { -1.0 } else { 1.0 },
        )
    };
    Ok(Scalar(Float::with_val(
        PRECISION_BITS,
        (turn_deg(turn) * std::f64::consts::PI) / 180.0,
    )))
}

fn read_value(
    par: &SourceFile,
    pmtr: &str,
    tokens: &[&str],
    original: ParameterLine,
) -> Result<Parameter, TimingError> {
    let error = |message: String| TimingError::new(&par.path, Some(original.line), pmtr, message);
    if tokens.len() > 4 {
        return Err(error(
            "expected value [fit_flag] [uncertainty]; extra fields are unsupported".into(),
        ));
    }
    let value = if pmtr == "RAJ" || pmtr == "DECJ" {
        read_position(tokens[1], pmtr == "RAJ")
    } else {
        parse_longdouble(tokens[1])
    }
    .map_err(error)?;
    let mut fit_flag = 0;
    let uncertainty = match tokens.len() {
        3 if matches!(tokens[2], "0" | "1" | "2") => {
            fit_flag = tokens[2].parse().unwrap();
            None
        }
        3 => Some(parse_longdouble(tokens[2]).map_err(error)?),
        4 => {
            fit_flag = match tokens[2] {
                "0" => 0,
                "1" => 1,
                "2" => 2,
                _ => return Err(error("fit flag must be 0, 1 or 2".into())),
            };
            Some(parse_longdouble(tokens[3]).map_err(error)?)
        }
        _ => None,
    };
    if uncertainty.as_ref().is_some_and(|s| s.0 < 0) {
        return Err(error("uncertainty must be nonnegative".into()));
    }
    if pmtr == "F0" && value.0 <= 0 {
        return Err(error("spin frequency must be positive".into()));
    }
    if pmtr == "EPHVER" && tokens[1] != "5" {
        return Err(error(
            "only EPHVER 5 is supported; TEMPO1 emulation is unavailable".into(),
        ));
    }
    Ok(Parameter {
        value,
        set: true,
        fit_flag,
        uncertainty,
        original: Some(original),
    })
}

pub(crate) fn read_parfile(par: &SourceFile) -> Result<ParFile, TimingError> {
    let mut param: BTreeMap<String, Parameter> = ["F0", "F1", "F2", "F3", "PEPOCH"]
        .into_iter()
        .map(|name| {
            (
                name.into(),
                Parameter {
                    value: Scalar(Float::with_val(PRECISION_BITS, 0)),
                    set: false,
                    fit_flag: 0,
                    uncertainty: None,
                    original: None,
                },
            )
        })
        .collect();
    let mut text_parameters: BTreeMap<String, ParameterLine> = BTreeMap::new();
    let mut fit_controls = FitControls::default();
    for (index, raw) in par.text.lines().enumerate() {
        let tokens: Vec<_> = raw.split_whitespace().collect();
        if ignored_line(&tokens) {
            continue;
        }
        let key = tokens[0].to_ascii_uppercase();
        let key = if key == "TZRFREQ" {
            "TZRFRQ".into()
        } else {
            key
        };
        let error = |message: String| TimingError::new(&par.path, Some(index + 1), &key, message);
        if tokens.len() < 2 {
            return Err(error("missing value".into()));
        }
        let original = ParameterLine {
            line: index + 1,
            keyword: tokens[0].into(),
            tokens: tokens[1..].iter().map(|t| (*t).into()).collect(),
            raw_line: raw.into(),
        };
        if matches!(key.as_str(), "MODE" | "WEIGHT" | "NITS") {
            if tokens.len() != 2 {
                return Err(error("expected one integer control value".into()));
            }
            if key == "NITS" {
                // Positive NITS is this released profile; upstream permits other counts.
                fit_controls.nits = tokens[1]
                    .parse::<i32>()
                    .ok()
                    .filter(|n| *n > 0)
                    .ok_or_else(|| {
                        error("NITS must be a positive source-range integer for M2".into())
                    })? as usize;
                fit_controls.nits_defaulted = false;
            } else {
                fit_controls.mode = tokens[1]
                    .parse::<i32>()
                    .ok()
                    .filter(|n| matches!(n, 0 | 1))
                    .ok_or_else(|| error("MODE/WEIGHT must be 0 or 1 for M2".into()))?
                    as u8;
                fit_controls.mode_defaulted = false;
            }
            // readParfile.C assigns sequentially: the last MODE/WEIGHT wins.
            fit_controls.declarations.push(original);
        } else if NUMERIC.contains(&key.as_str()) {
            if param.get(&key).is_some_and(|parameter| parameter.set) {
                return Err(error("duplicate parameter".into()));
            }
            let parameter = read_value(par, &key, &tokens, original)?;
            param.insert(key.clone(), parameter);
        } else if matches!(key.as_str(), "PSRJ" | "CLK" | "EPHEM" | "TZRSITE" | "UNITS") {
            if tokens.len() != 2 {
                return Err(error("expected exactly one text value".into()));
            }
            if key == "UNITS" && tokens[1] != "TCB" {
                return Err(error("only explicit UNITS TCB is supported".into()));
            }
            if text_parameters.insert(key.clone(), original).is_some() {
                return Err(error("duplicate parameter".into()));
            }
        } else {
            return Err(error(
                "unsupported PAR keyword (including inactive supplied higher derivatives or TRACK)"
                    .into(),
            ));
        }
    }
    for key in ["F0", "PEPOCH"] {
        if !param[key].set {
            return Err(TimingError::new(
                &par.path,
                None,
                key,
                "required parameter is missing",
            ));
        }
    }
    if !text_parameters.contains_key("PSRJ") {
        return Err(TimingError::new(
            &par.path,
            None,
            "PSRJ",
            "required pulsar identity is missing",
        ));
    }
    let units_resolution = if text_parameters.contains_key("UNITS") {
        "explicit UNITS TCB"
    } else if param.contains_key("EPHVER") {
        "UNITS omitted; initialiseOne SI_UNITS/TCB default retained with EPHVER 5"
    } else {
        return Err(TimingError::new(
            &par.path,
            None,
            "UNITS/EPHVER",
            "supply UNITS TCB or EPHVER 5 to resolve the time scale",
        ));
    };

    Ok(ParFile {
        fit_controls,
        param,
        text_parameters,
        conventions: Conventions {
            time_scale: "TCB",
            units_resolution: units_resolution.into(),
            phase_reference: "signed fractional reduced phase of first input TOA",
            remove_mean: false,
            fit_performed: false,
            input_order: "original; no cuts or sorting",
            corrections: "@ bypasses clock and propagation; isolated subset has BAT = BBAT = SAT",
        },
    })
}
