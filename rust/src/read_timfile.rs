//! TEMPO2 readTimfile.C::readTimfile/readTim FORMAT 1 subset; source directives,
//! per-TOA flags and other formats are outside this owned reader's profile.
use crate::input::ignored_line;
use crate::longdouble::parse_longdouble;
use crate::tempo2_util::parse_binary64;
use crate::{Observation, OriginalObservation, SourceFile, TimingError};

pub(crate) fn read_timfile(tim: &SourceFile) -> Result<Vec<Observation>, TimingError> {
    let mut format_seen = false;
    let mut obsn = Vec::new();
    for (index, raw) in tim.text.lines().enumerate() {
        let tokens: Vec<_> = raw.split_whitespace().collect();
        if ignored_line(&tokens) {
            continue;
        }
        let error = |field: &str, message: String| {
            TimingError::new(&tim.path, Some(index + 1), field, message)
        };
        if tokens[0].eq_ignore_ascii_case("FORMAT") {
            if format_seen || tokens != ["FORMAT", "1"] {
                return Err(error(
                    "FORMAT",
                    "require one FORMAT 1 header before all observations".into(),
                ));
            }
            format_seen = true;
            continue;
        }
        if !format_seen {
            return Err(error(
                "FORMAT",
                "FORMAT 1 header is required before observations".into(),
            ));
        }
        // Directives are not observation filenames, even when they have five tokens.
        if matches!(
            tokens[0].to_ascii_uppercase().as_str(),
            "INCLUDE"
                | "HEAD"
                | "TOAS"
                | "PROFILE_DIR"
                | "GLOBAL_EFAC"
                | "EFLOOR"
                | "SIGMA"
                | "MODE"
                | "EFAC"
                | "EQUAD"
                | "TIME"
                | "PHASE"
                | "JUMP"
                | "SKIP"
                | "NOSKIP"
                | "END"
                | "INFO"
                | "FMIN"
                | "FMAX"
                | "EMIN"
                | "EMAX"
                | "ESET"
                | "DITHER"
                | "SIM"
                | "TRACK"
                | "T2EFAC"
                | "T2EQUAD"
        ) {
            return Err(error(tokens[0], "unsupported TIM directive".into()));
        }
        if tokens.len() != 5 {
            return Err(error(tokens[0], "expected name frequency_MHz MJD uncertainty_us @; directives and flags are unsupported".into()));
        }
        if tokens[4] != "@" && !tokens[4].eq_ignore_ascii_case("pks") {
            return Err(error(
                "site",
                format!(
                    "only barycentric @ or PKS is supported, got {:?}",
                    tokens[4]
                ),
            ));
        }
        let sat = parse_longdouble(tokens[2]).map_err(|e| error("MJD", e))?;
        let freq = parse_binary64(tokens[1]).map_err(|e| error("frequency_MHz", e))?;
        let toa_err = parse_binary64(tokens[3]).map_err(|e| error("uncertainty_us", e))?;
        if freq < 0.0 {
            return Err(error(
                "frequency_MHz",
                "frequency must be nonnegative".into(),
            ));
        }
        if toa_err < 0.0 {
            return Err(error(
                "uncertainty_us",
                "uncertainty must be nonnegative".into(),
            ));
        }
        obsn.push(Observation {
            sat_mjd: sat,
            frequency_mhz: freq,
            toa_error_us: toa_err,
            original: OriginalObservation {
                index: obsn.len(),
                line: index + 1,
                name: tokens[0].into(),
                raw_line: raw.into(),
                frequency_mhz_text: tokens[1].into(),
                sat_mjd_text: tokens[2].into(),
                toa_error_us_text: tokens[3].into(),
                site: tokens[4].into(),
            },
        });
    }
    if obsn.is_empty() {
        return Err(TimingError::new(
            &tim.path,
            None,
            "observations",
            "no FORMAT 1 observations supplied",
        ));
    }
    Ok(obsn)
}
