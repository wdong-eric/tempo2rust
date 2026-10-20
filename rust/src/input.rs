//! File ownership and the public load operation.
use crate::{SourceFile, TimingError, TimingInput};
use crate::{read_parfile::read_parfile, read_timfile::read_timfile};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

fn read_source(path: &Path) -> Result<SourceFile, TimingError> {
    let text = fs::read_to_string(path)
        .map_err(|e| TimingError::new(path, None, "input", e.to_string()))?;
    Ok(SourceFile {
        path: path.into(),
        sha256: format!("{:x}", Sha256::digest(text.as_bytes())),
        text,
    })
}

/// Load a single isolated model and ordered FORMAT 1 @ or PKS observations.
pub fn load(par_path: &Path, tim_path: &Path) -> Result<TimingInput, TimingError> {
    let par = read_source(par_path)?;
    let tim = read_source(tim_path)?;
    let mut psr = read_parfile(&par)?;
    let obsn = read_timfile(&tim)?;
    let terrestrial = obsn[0].original.site != "@";
    if obsn.iter().any(|o| (o.original.site != "@") != terrestrial) {
        return Err(TimingError::new(
            &tim.path,
            None,
            "site",
            "mixed barycentric and terrestrial observations are outside the supported profiles",
        ));
    }
    if terrestrial {
        psr.conventions.time_scale = "SAT: observatory clock; model/BAT/BBAT: TCB";
        psr.conventions.corrections = "PKS: TT(TAI), IF99, DE405, IAU2000B, frequency dilation, Sun/planet Shapiro, DM, IPM=1 NE_SW=4 SWM=0, troposphere with TEMPO2 meteorology fallback";
    }
    Ok(TimingInput {
        par,
        tim,
        parameters: psr.param,
        text_parameters: psr.text_parameters,
        observations: obsn,
        conventions: psr.conventions,
    })
}

pub(crate) fn ignored_line(tokens: &[&str]) -> bool {
    tokens.is_empty() || tokens[0].starts_with('#') || tokens[0] == "C"
}
