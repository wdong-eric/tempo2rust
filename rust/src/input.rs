//! Rust-only load/provenance glue for readParfile.C::readParfile/readValue,
//! readTimfile.C::readTimfile/readTim and selected initialise.C::initialiseOne defaults.
//! SourceFile and immutable TimingInput have no standalone TEMPO2 counterpart.
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
    let (obsn, observation_metadata) = read_timfile(&tim)?;
    let terrestrial = crate::observation::read_string(&obsn[0].telID, "telID")? != b"@";
    for o in &obsn {
        if (crate::observation::read_string(&o.telID, "telID")? != b"@") != terrestrial {
            return Err(TimingError::new(
                &tim.path,
                None,
                "site",
                "mixed barycentric and terrestrial observations are outside the supported profiles",
            ));
        }
    }
    if terrestrial {
        psr.conventions.time_scale = "SAT: observatory clock; model/BAT/BBAT: TCB";
        psr.conventions.corrections = "PKS: TT(TAI), IF99, DE405, IAU2000B, frequency dilation, Sun/planet Shapiro, DM, IPM=1 NE_SW=4 SWM=0, troposphere with TEMPO2 meteorology fallback";
    }
    let mut source = crate::pulsar_lifecycle::PulsarContext::unwritten(1);
    let capacity = i32::try_from(obsn.len()).map_err(|_| {
        TimingError::new(
            &tim.path,
            None,
            "nobs",
            "source observation count exceeds i32",
        )
    })?;
    // Explicit quiet bounded caller; raw initialiseOne still allocates full covariance.
    // Reuse process diagnostics; a lifecycle fault belongs to this context.
    source
        .initialise(1, capacity, 1)
        .map_err(|e| TimingError::new(&par.path, None, "pulsar storage", e.to_string()))?;
    let p = &mut source.pulsars[0];
    p.param = psr.param;
    p.nobs = Some(capacity);
    p.fitMode = Some(i32::from(psr.fit_controls.mode));
    p.nits = Some(psr.fit_controls.nits as i32);
    for (key, field) in [
        ("PSRJ", &mut p.name[..]),
        ("CLK", &mut p.clock[..]),
        ("EPHEM", &mut p.ephemeris[..]),
        ("TZRSITE", &mut p.tzrsite[..]),
    ] {
        if let Some(value) = psr.text_parameters.get(key) {
            crate::observation::write_string(field, value.tokens[0].as_bytes(), key)
                .map_err(|e| TimingError::new(&par.path, Some(value.line), key, e.message))?;
        }
    }
    let view = p.obsn.unwrap().unwrap();
    for (dst, src) in source
        .observations_mut(view)
        .expect("new source owner")
        .iter_mut()
        .zip(obsn)
    {
        *dst = src;
    }
    Ok(TimingInput {
        par,
        tim,
        source,
        parameter_metadata: psr.metadata,
        fit_metadata: crate::read_parfile::FitControlMetadata {
            mode_defaulted: psr.fit_controls.mode_defaulted,
            nits_defaulted: psr.fit_controls.nits_defaulted,
            declarations: psr.fit_controls.declarations,
        },
        text_parameters: psr.text_parameters,
        observation_metadata,
        conventions: psr.conventions,
    })
}

pub(crate) fn ignored_line(tokens: &[&str]) -> bool {
    tokens.is_empty() || tokens[0].starts_with('#') || tokens[0] == "C"
}
