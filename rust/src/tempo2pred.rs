//! tempo2pred.c: generic dispatch, owned I/O, metadata and frequency plans.
use crate::{
    cheby2d::ChebyModelSet,
    predictor_number::{N, n},
    t1polyco::T1PolycoSet,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    io::{Cursor, Read, Seek, SeekFrom, Write},
    path::Path,
};
#[derive(Clone, Debug, Default, Serialize)]
pub enum T2Predictor {
    #[default]
    None,
    Cheby(ChebyModelSet),
    T1(T1PolycoSet),
}
#[derive(Debug, Serialize)]
pub struct PredictorFile {
    pub predictor: T2Predictor,
    pub bytes: Vec<u8>,
    pub sha256: String,
    pub format: &'static str,
}
#[derive(Debug, Serialize)]
pub struct Prediction {
    pub segment: Option<usize>,
    pub out_of_range: bool,
    pub phase_turn: N,
    pub frequency_hz: N,
}
#[derive(Debug, Serialize)]
pub struct Metadata {
    pub kind: &'static str,
    pub psrname: String,
    pub sitename: String,
    pub start_mjd: N,
    pub end_mjd: N,
    pub freq_start_mhz: N,
    pub freq_end_mhz: N,
}
#[derive(Debug, Serialize)]
pub struct PlanSegment {
    pub start_mjd: N,
    pub end_mjd: N,
    pub average_frequency_hz: N,
    pub start_segment: Option<usize>,
    pub end_segment: Option<usize>,
    pub out_of_range: bool,
}
#[derive(Debug, Serialize)]
pub struct FrequencyPlan {
    pub metadata: Metadata,
    pub phase0_turn: N,
    pub frequency_mhz: N,
    pub segments: Vec<PlanSegment>,
}
impl T2Predictor {
    pub fn read(path: &Path) -> Result<PredictorFile, String> {
        let f = std::fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
        Self::read_stream(f)
    }
    pub fn read_stream(mut input: impl Read + Seek) -> Result<PredictorFile, String> {
        let start = input.stream_position().map_err(|e| e.to_string())?;
        let mut bytes = vec![];
        input.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
        // fda6ad4 tempo2pred.c:51-71 destroys a union as Cheby before knowing
        // its active kind; GetPlan_Ext:350 passes uninitialized storage. Parse
        // into a fresh owned enum; failed reads never expose invalid storage.
        let mut cursor = Cursor::new(&bytes);
        let parsed = if let Ok(c) = ChebyModelSet::read(&mut cursor) {
            Ok(Self::Cheby(c))
        } else {
            cursor.set_position(0);
            T1PolycoSet::read(&mut cursor).map(Self::T1)
        };
        let consumed = if parsed.is_ok() { cursor.position() } else { 0 };
        input
            .seek(SeekFrom::Start(start + consumed))
            .map_err(|e| e.to_string())?;
        let predictor = parsed?;
        let format = if matches!(predictor, Self::Cheby(_)) {
            "ChebyModelSet"
        } else if bytes.starts_with(b"TEMPO2") {
            "TEMPO2 polyco"
        } else {
            "TEMPO1 polyco"
        };
        Ok(PredictorFile {
            predictor,
            sha256: format!("{:x}", Sha256::digest(&bytes)),
            bytes,
            format,
        })
    }
    pub fn read_into(&mut self, path: &Path) -> Result<PredictorFile, String> {
        let f = Self::read(path)?;
        *self = f.predictor.clone();
        Ok(f)
    }
    pub fn write_stream(&self, output: impl Write) -> Result<(), String> {
        match self {
            Self::None => Err("no predictor backing".into()),
            Self::Cheby(c) => c.write(output),
            Self::T1(t) => t.write(output),
        }
    }
    pub fn write(&self, path: &Path) -> Result<(), String> {
        let mut bytes = vec![];
        self.write_stream(&mut bytes)?;
        std::fs::write(path, bytes).map_err(|e| format!("{}: {e}", path.display()))
    }
    pub fn insert(&mut self, from: &Self) -> Result<(), String> {
        match (&mut *self, from) {
            (_, Self::T1(_)) => Err("T2Predictor_Insert is source-unimplemented for T1".into()),
            (Self::None, _) => {
                *self = from.clone();
                Ok(())
            }
            (Self::Cheby(a), Self::Cheby(b)) => {
                a.insert(b);
                Ok(())
            }
            (_, _) => Err("T2Predictor_Insert: different kinds".into()),
        }
    }
    pub fn copy_from(&mut self, from: &Self) -> Result<(), String> {
        if matches!(from, Self::T1(_)) {
            return Err("T2Predictor_Copy delegates to unimplemented T1 insertion".into());
        }
        *self = from.clone();
        Ok(())
    }
    pub fn keep(&mut self, mjds: &[N]) -> Result<(), String> {
        match self {
            Self::None => Ok(()),
            Self::Cheby(c) => {
                c.keep(mjds);
                Ok(())
            }
            Self::T1(_) => Err("T2Predictor_Keep is source-unimplemented for T1".into()),
        }
    }
    pub fn metadata(&self) -> Result<Metadata, String> {
        match self {
            Self::None => Err("no predictor backing".into()),
            Self::Cheby(c) => {
                let a = c.segments.first().ok_or("empty Cheby set")?;
                let b = c.segments.last().unwrap();
                Ok(Metadata {
                    kind: "Cheby",
                    psrname: a.psrname.clone(),
                    sitename: a.sitename.clone(),
                    start_mjd: a.mjd_start.clone(),
                    end_mjd: b.mjd_end.clone(),
                    freq_start_mhz: a.freq_start.clone(),
                    freq_end_mhz: a.freq_end.clone(),
                })
            }
            Self::T1(c) => {
                let a = c.segments.first().ok_or("empty T1 set")?;
                let b = c.segments.last().unwrap();
                Ok(Metadata {
                    kind: "T1",
                    psrname: a.psrname.clone(),
                    sitename: a.sitename.clone(),
                    start_mjd: &a.mjd_mid - n(a.span as f64) / 2880.,
                    end_mjd: &b.mjd_mid + n(b.span as f64) / 2880.,
                    freq_start_mhz: n(a.frequency_obs),
                    freq_end_mhz: n(a.frequency_obs),
                })
            }
        }
    }
    pub fn evaluate(&self, mjd: &N, freq: &N) -> Result<Prediction, String> {
        self.metadata()?;
        match self {
            Self::Cheby(c) => {
                if let Some(i) = c.nearest_index(mjd) {
                    let m = &c.segments[i];
                    Ok(Prediction {
                        segment: Some(i),
                        out_of_range: false,
                        phase_turn: m.phase(mjd, freq),
                        frequency_hz: m.frequency(mjd, freq),
                    })
                } else {
                    Ok(Prediction {
                        segment: None,
                        out_of_range: true,
                        phase_turn: n(-1.),
                        frequency_hz: n(-1.),
                    })
                }
            }
            Self::T1(c) => {
                let i = c.nearest_index(mjd)?;
                let m = &c.segments[i];
                Ok(Prediction {
                    segment: Some(i),
                    out_of_range: false,
                    phase_turn: m.phase(mjd, freq),
                    frequency_hz: m.frequency(mjd, freq),
                })
            }
            Self::None => unreachable!(),
        }
    }
    pub fn plan(&self, start: &N, end: &N, step: &N, freq: &N) -> Result<FrequencyPlan, String> {
        if end < start || step <= &n(0.) || ![start, end, step, freq].iter().all(|v| v.is_finite())
        {
            return Err("plan requires finite inputs, end >= start, step > 0 seconds".into());
        }
        let count = ((end - start) * 86400. / step).ceil_count()?;
        let mut last = self.evaluate(start, freq)?;
        let phase0 = last.phase_turn.clone();
        let mut lastmjd = start.clone();
        let mut segments = vec![];
        for i in 0..count {
            let mut mjd = start + step * (i + 1) as f64 / 86400.;
            if &mjd > end {
                mjd = end.clone();
            }
            let next = self.evaluate(&mjd, freq)?;
            let average = (&next.phase_turn - &last.phase_turn) / (&mjd - &lastmjd) / 86400.;
            segments.push(PlanSegment {
                start_mjd: lastmjd,
                end_mjd: mjd.clone(),
                average_frequency_hz: average,
                start_segment: last.segment,
                end_segment: next.segment,
                out_of_range: last.out_of_range || next.out_of_range,
            });
            lastmjd = mjd;
            last = next;
        }
        Ok(FrequencyPlan {
            metadata: self.metadata()?,
            phase0_turn: phase0,
            frequency_mhz: freq.clone(),
            segments,
        })
    }
}
pub fn get_plan(
    path: &Path,
    start: &N,
    end: &N,
    step: &N,
    freq: &N,
) -> Result<FrequencyPlan, String> {
    get_plan_ext(path, start, end, step, freq)
}
pub fn get_plan_ext(
    path: &Path,
    start: &N,
    end: &N,
    step: &N,
    freq: &N,
) -> Result<FrequencyPlan, String> {
    T2Predictor::read(path)?
        .predictor
        .plan(start, end, step, freq)
}
