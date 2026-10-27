//! Public general source input ownership and consumed-byte provenance.
use crate::{TimingError, pulsar_lifecycle::PulsarContext, source_text::SourceText};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Serialize)]
pub struct InputPair {
    pub par: PathBuf,
    pub tim: PathBuf,
}
#[derive(Clone, Debug, Serialize)]
pub struct ConsumedInput {
    pub path: PathBuf,
    pub role: String,
    pub sha256: String,
    pub bytes: Vec<u8>,
}
#[derive(Debug)]
pub struct BatInput {
    pub context: PulsarContext,
    pub pairs: Vec<InputPair>,
    pub inputs: Vec<ConsumedInput>,
}

pub(crate) fn consume(
    path: &Path,
    role: &str,
    inputs: &mut Vec<ConsumedInput>,
) -> Result<SourceText, TimingError> {
    let bytes = fs::read(path).map_err(|e| TimingError::new(path, None, role, e.to_string()))?;
    inputs.push(ConsumedInput {
        path: path.into(),
        role: role.into(),
        sha256: format!("{:x}", Sha256::digest(&bytes)),
        bytes: bytes.clone(),
    });
    Ok(SourceText::new(path, bytes))
}

pub fn load_pairs(pairs: &[InputPair], runtime: &Path) -> Result<BatInput, TimingError> {
    let _workflow = crate::execution::enter();
    if pairs.is_empty() {
        return Err(TimingError::new(
            runtime,
            None,
            "input pairs",
            "at least one PAR/TIM pair is required",
        ));
    }
    let mut context = PulsarContext::unwritten(pairs.len());
    context
        .initialise(
            i32::try_from(pairs.len()).map_err(|_| {
                TimingError::new(runtime, None, "input pairs", "source count overflow")
            })?,
            0,
            1,
        )
        .map_err(|e| TimingError::new(runtime, None, "initialise", e.to_string()))?;
    let mut inputs = vec![];
    crate::source_par::read_parfiles(&mut context, pairs, runtime, &mut inputs)?;
    crate::source_tim::read_timfiles(
        &mut context,
        &pairs.iter().map(|p| p.tim.clone()).collect::<Vec<_>>(),
        runtime,
        &mut inputs,
    )?;
    for i in 0..context.pulsars.len() {
        let (p, rows) = context.pulsar_observations_mut(i)?;
        let n = *crate::observation::read(&p.nobs, "nobs")?;
        for (index, row) in rows.iter_mut().take(n.max(0) as usize).enumerate() {
            row.input_index = Some(index);
        }
    }
    Ok(BatInput {
        context,
        pairs: pairs.to_vec(),
        inputs,
    })
}

pub fn preprocess(
    input: &mut BatInput,
    arguments: &[Vec<u8>],
    runtime: &Path,
) -> Result<(), TimingError> {
    let _workflow = crate::execution::enter();
    crate::preprocess::pre_process(&mut input.context, arguments, runtime, &mut input.inputs)
}

pub use crate::global::{form_bats_all, update_bats_all};

#[derive(Debug, Serialize)]
pub struct ByteString {
    pub bytes: Vec<u8>,
    pub text: Option<String>,
}
impl ByteString {
    fn new(bytes: Vec<u8>) -> Self {
        let text = String::from_utf8(bytes.clone()).ok();
        Self { bytes, text }
    }
}
#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum Number {
    Finite(String),
    Nonfinite { class: &'static str },
}
impl Number {
    pub(crate) fn extended(v: &rug::Float) -> Self {
        if v.is_nan() {
            Self::Nonfinite {
                class: if v.is_sign_negative() { "-nan" } else { "nan" },
            }
        } else if v.is_infinite() {
            Self::Nonfinite {
                class: if v.is_sign_negative() {
                    "-infinity"
                } else {
                    "+infinity"
                },
            }
        } else {
            Self::Finite(v.to_string_radix(10, None))
        }
    }
    pub(crate) fn double(v: f64) -> Self {
        if v.is_nan() {
            Self::Nonfinite {
                class: if v.is_sign_negative() { "-nan" } else { "nan" },
            }
        } else if v.is_finite() {
            Self::Finite(v.to_string())
        } else {
            Self::extended(&rug::Float::with_val(crate::PRECISION_BITS, v))
        }
    }
}
#[derive(Debug, Serialize)]
pub struct BatResult {
    pub schema_version: u32,
    pub status: &'static str,
    pub engine: &'static str,
    pub runtime_root: ByteString,
    pub inputs: Vec<InputRecord>,
    pub resources: Vec<ResourceRecord>,
    pub diagnostics: Vec<Diagnostic>,
    pub pulsars: Vec<BatPulsar>,
}
#[derive(Debug, Serialize)]
pub struct InputRecord {
    pub path: ByteString,
    pub role: String,
    pub sha256: String,
    pub bytes: Vec<u8>,
}
#[derive(Debug, Serialize)]
pub struct ResourceRecord {
    pub path: ByteString,
    pub role: String,
    pub sha256: String,
    pub size_bytes: usize,
    pub coverage_mjd: Option<[f64; 2]>,
}
#[derive(Debug, Serialize)]
pub struct Diagnostic {
    pub source_channel: &'static str,
    pub bytes: Vec<u8>,
}
#[derive(Debug, Serialize)]
pub struct Selector {
    pub code: i32,
    pub label: Option<&'static str>,
}
#[derive(Debug, Serialize)]
pub struct BatPulsar {
    pub name: ByteString,
    pub units: Selector,
    pub time_ephemeris: Selector,
    pub coordinate_frame: Selector,
    pub sorted: i32,
    pub observations: Vec<BatObservation>,
    pub reference_observation: Option<BatObservation>,
}
#[derive(Debug, Serialize)]
pub struct Flag {
    pub id: ByteString,
    pub value: ByteString,
}
#[derive(Debug, Serialize)]
pub struct ClockCorrection {
    pub seconds: Number,
    pub corrects_to: ByteString,
}
#[derive(Debug, Serialize)]
pub struct Quantity {
    pub value: Option<Number>,
    pub unit: &'static str,
}
#[derive(Debug, Serialize)]
pub struct BatObservation {
    pub canonical_index: Option<usize>,
    pub filename: ByteString,
    pub site: ByteString,
    pub flags: Vec<Flag>,
    pub deleted: i32,
    pub clock_correction: i32,
    pub delay_correction: i32,
    pub sat_mjd: Number,
    pub bat_mjd: Number,
    pub bbat_mjd: Number,
    pub frequency: Quantity,
    pub frequency_ssb: Quantity,
    pub uncertainty: Quantity,
    pub clock_steps: Vec<ClockCorrection>,
    pub corrections: std::collections::BTreeMap<&'static str, Quantity>,
}
fn project_observation(
    o: &crate::tempo2::SourceObservation,
    index: Option<usize>,
) -> Result<BatObservation, TimingError> {
    use crate::observation::{read, read_string};
    let mut corrections = std::collections::BTreeMap::new();
    for (name, value) in [
        ("tt_to_tb", &o.correctionTT_TB),
        ("tt_to_teph", &o.correctionTT_Teph),
        ("ut1", &o.correctionUT1),
        ("roemer", &o.roemer),
        ("shklovskii", &o.shklovskii),
    ] {
        corrections.insert(
            name,
            Quantity {
                value: value.as_ref().map(Number::extended),
                unit: "s",
            },
        );
    }
    for (name, value) in [
        ("troposphere", o.troposphericDelay),
        ("dispersion_chromatic", o.tdis1),
        ("solar_wind", o.tdis2),
        ("shapiro_sun", o.shapiroDelaySun),
        ("shapiro_jupiter", o.shapiroDelayJupiter),
        ("shapiro_saturn", o.shapiroDelaySaturn),
        ("shapiro_venus", o.shapiroDelayVenus),
        ("shapiro_uranus", o.shapiroDelayUranus),
        ("shapiro_neptune", o.shapiroDelayNeptune),
    ] {
        corrections.insert(
            name,
            Quantity {
                value: value.map(Number::double),
                unit: "s",
            },
        );
    }
    corrections.insert(
        "bat_correction",
        Quantity {
            value: o.batCorr.as_ref().map(Number::extended),
            unit: "day",
        },
    );
    corrections.insert(
        "einstein_rate",
        Quantity {
            value: o.einsteinRate.map(Number::double),
            unit: "dimensionless",
        },
    );
    let steps = (0..*read(&o.nclock_correction, "nclock_correction")?)
        .map(|i| {
            let c = &o.correctionsTT[i as usize];
            Ok(ClockCorrection {
                seconds: Number::double(*read(&c.correction, "clock correction")?),
                corrects_to: ByteString::new(read_string(&c.corrects_to, "corrects_to")?),
            })
        })
        .collect::<Result<Vec<_>, TimingError>>()?;
    corrections.insert(
        "clock_to_tt",
        Quantity {
            value: Some(Number::double(crate::clkcorr::get_correction_tt(o)?)),
            unit: "s",
        },
    );
    Ok(BatObservation {
        canonical_index: index,
        filename: ByteString::new(read_string(&o.fname, "fname")?),
        site: ByteString::new(read_string(&o.telID, "telID")?),
        flags: crate::preprocess::flags(o)?
            .into_iter()
            .map(|(id, value)| Flag {
                id: ByteString::new(id),
                value: ByteString::new(value),
            })
            .collect(),
        deleted: *read(&o.deleted, "deleted")?,
        clock_correction: *read(&o.clockCorr, "clockCorr")?,
        delay_correction: *read(&o.delayCorr, "delayCorr")?,
        sat_mjd: Number::extended(read(&o.sat, "sat")?),
        bat_mjd: Number::extended(read(&o.bat, "bat")?),
        bbat_mjd: Number::extended(read(&o.bbat, "bbat")?),
        frequency: Quantity {
            value: o.freq.map(Number::double),
            unit: "MHz",
        },
        frequency_ssb: Quantity {
            value: o.freqSSB.map(Number::double),
            unit: if o.delayCorr == Some(0) { "MHz" } else { "Hz" },
        },
        uncertainty: Quantity {
            value: o.toaErr.map(Number::double),
            unit: "us",
        },
        clock_steps: steps,
        corrections,
    })
}
pub fn evaluate(
    input: &mut BatInput,
    runtime: &mut crate::RuntimeContext,
) -> Result<BatResult, TimingError> {
    let _workflow = crate::execution::enter();
    form_bats_all(&mut input.context, runtime)?;
    project(input, runtime)
}
pub(crate) fn project(
    input: &BatInput,
    runtime: &crate::RuntimeContext,
) -> Result<BatResult, TimingError> {
    use crate::observation::{read, read_string};
    use std::os::unix::ffi::OsStrExt;
    let mut pulsars = vec![];
    for p in &input.context.pulsars {
        let n = *read(&p.nobs, "nobs")? as usize;
        let view = p
            .obsn
            .flatten()
            .ok_or_else(|| crate::clock_diagnostics::boundary("BAT_output_observation_owner"))?;
        let rows = input
            .context
            .observations(view)
            .map_err(|e| TimingError::new(&runtime.root, None, "BAT output", e.to_string()))?;
        let units = *read(&p.units, "units")?;
        let time = *read(&p.timeEphemeris, "timeEphemeris")?;
        let frame = *read(&p.eclCoord, "eclCoord")?;
        pulsars.push(BatPulsar {
            name: ByteString::new(read_string(&p.name[..], "name")?),
            units: Selector {
                code: units,
                label: match units {
                    1 => Some("TCB / SI"),
                    2 => Some("TDB"),
                    _ => None,
                },
            },
            time_ephemeris: Selector {
                code: time,
                label: match time {
                    1 => Some("IF99"),
                    2 => Some("FB90"),
                    _ => None,
                },
            },
            coordinate_frame: Selector {
                code: frame,
                label: match frame {
                    0 => Some("equatorial"),
                    1 => Some("ecliptic"),
                    _ => None,
                },
            },
            sorted: *read(&p.sorted, "sorted")? as i32,
            observations: rows[..n]
                .iter()
                .enumerate()
                .map(|(i, o)| project_observation(o, Some(i)))
                .collect::<Result<_, _>>()?,
            reference_observation: if p.refphs == Some(1) {
                Some(project_observation(&p.tzrobs, None)?)
            } else {
                None
            },
        });
    }
    Ok(BatResult {
        schema_version: 1,
        status: "complete",
        engine: "TEMPO2 fda6ad4 source port; native JPL; Rug128 extended arithmetic",
        runtime_root: ByteString::new(runtime.root.as_os_str().as_bytes().to_vec()),
        inputs: input
            .inputs
            .iter()
            .map(|r| InputRecord {
                path: ByteString::new(r.path.as_os_str().as_bytes().to_vec()),
                role: r.role.clone(),
                sha256: r.sha256.clone(),
                bytes: r.bytes.clone(),
            })
            .collect(),
        resources: runtime
            .resources
            .iter()
            .map(|r| ResourceRecord {
                path: ByteString::new(r.source_path.as_os_str().as_bytes().to_vec()),
                role: r.role.clone(),
                sha256: r.sha256.clone(),
                size_bytes: r.size_bytes,
                coverage_mjd: r.coverage_mjd,
            })
            .collect(),
        diagnostics: crate::take_clock_diagnostics()
            .into_iter()
            .map(|d| Diagnostic {
                source_channel: if d.source_stderr { "stderr" } else { "stdout" },
                bytes: d.bytes,
            })
            .collect(),
        pulsars,
    })
}
