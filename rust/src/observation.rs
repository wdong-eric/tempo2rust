//! tempo2.h::observation/clock_correction and tempo2Util.C:293 assignment.
//! Owned typed fields, not a C ABI. initialise.C allocates observations without
//! writing them; None denotes unwritten storage, including individual bytes.
#![allow(non_snake_case)]
use crate::{TimingError, clock_diagnostics::boundary};
use rug::Float;

pub const MAX_FILELEN: usize = 500;
pub const MAX_FLAGS: usize = 40;
pub const MAX_FLAG_LEN: usize = 32;
pub const MAX_CLK_CORR: usize = 30;

#[derive(Clone, Debug, Default)]
pub struct SourceClockCorrection {
    pub correction: Option<f64>,
    pub corrects_to: [Option<u8>; 32],
}

/// Complete source record. Read-only access through TimingInput preserves its
/// frontend invariants. Clone copies initializedness, full backing and all tails;
/// it neither reads unwritten cells nor implements the rest of copyPSR.
#[derive(Clone, Debug)]
pub struct SourceObservation {
    pub sat: Option<Float>,
    pub origsat: Option<Float>,
    pub sat_day: Option<Float>,
    pub sat_sec: Option<Float>,
    pub bat: Option<Float>,
    pub batCorr: Option<Float>,
    pub bbat: Option<Float>,
    pub pet: Option<Float>,
    pub clockCorr: Option<i32>,
    pub delayCorr: Option<i32>,
    pub deleted: Option<i32>,
    pub prefitResidual: Option<Float>,
    pub residual: Option<Float>,
    pub residualtn: Option<Float>,
    pub addedNoise: Option<f64>,
    pub TNRedSignal: Option<f64>,
    pub TNRedErr: Option<f64>,
    pub TNDMSignal: Option<f64>,
    pub TNDMErr: Option<f64>,
    pub SWGPSignal: Option<f64>,
    pub SWGPErr: Option<f64>,
    pub TNGroupSignal: Option<f64>,
    pub TNGroupErr: Option<f64>,
    pub TNChromSignal: Option<f64>,
    pub TNChromErr: Option<f64>,
    pub freq: Option<f64>,
    pub freqSSB: Option<f64>,
    pub toaErr: Option<f64>,
    pub toaDMErr: Option<f64>,
    pub origErr: Option<f64>,
    pub phaseOffset: Option<f64>,
    pub averagebat: Option<f64>,
    pub averageres: Option<f64>,
    pub averageerr: Option<f64>,
    pub averagedmbat: Option<f64>,
    pub averagedmres: Option<f64>,
    pub averagedmerr: Option<f64>,
    pub fname: [Option<u8>; MAX_FILELEN + 1],
    pub telID: [Option<u8>; 100],
    pub correctionsTT: [SourceClockCorrection; MAX_CLK_CORR],
    pub nclock_correction: Option<i32>,
    pub correctionTT_TB: Option<Float>,
    pub einsteinRate: Option<f64>,
    pub correctionTT_calcEph: Option<Float>,
    pub correctionTT_Teph: Option<Float>,
    pub correctionUT1: Option<Float>,
    pub sun_ssb: [Option<f64>; 6],
    pub sun_earth: [Option<f64>; 6],
    pub planet_ssb: [[Option<f64>; 6]; 9],
    pub planet_ssb_tmr: [[Option<f64>; 6]; 9],
    pub planet_ssb_derv: [[Option<f64>; 6]; 9],
    pub jupiter_earth: [Option<f64>; 6],
    pub saturn_earth: [Option<f64>; 6],
    pub venus_earth: [Option<f64>; 6],
    pub uranus_earth: [Option<f64>; 6],
    pub neptune_earth: [Option<f64>; 6],
    pub earthMoonBary_ssb: [Option<f64>; 6],
    pub earthMoonBary_earth: [Option<f64>; 6],
    pub earth_ssb: [Option<f64>; 6],
    pub observatory_earth: [Option<f64>; 6],
    pub psrPos: [Option<f64>; 3],
    pub zenith: [Option<f64>; 3],
    pub nutations: [Option<f64>; 6],
    pub siteVel: [Option<f64>; 3],
    pub shklovskii: Option<Float>,
    pub shapiroDelaySun: Option<f64>,
    pub shapiroDelayJupiter: Option<f64>,
    pub shapiroDelaySaturn: Option<f64>,
    pub shapiroDelayVenus: Option<f64>,
    pub shapiroDelayUranus: Option<f64>,
    pub shapiroDelayNeptune: Option<f64>,
    pub troposphericDelay: Option<f64>,
    pub tdis1: Option<f64>,
    pub tdis2: Option<f64>,
    pub roemer: Option<Float>,
    pub torb: Option<Float>,
    pub nphase: Option<Float>,
    pub phase: Option<Float>,
    pub pulseN: Option<i64>,
    pub flagID: [[Option<u8>; MAX_FLAG_LEN]; MAX_FLAGS],
    pub flagVal: [[Option<u8>; MAX_FLAG_LEN]; MAX_FLAGS],
    pub nFlags: Option<i32>,
    pub jump: [Option<i32>; MAX_FLAGS],
    pub jumpScale: [Option<f64>; MAX_FLAGS],
    pub obsNjump: Option<i32>,
    pub fdjump: [Option<i32>; MAX_FLAGS],
    pub obsNfdjump: Option<i32>,
    pub efac: Option<f64>,
    pub equad: Option<f64>,
    pub snr: Option<f64>,
    pub pnoise: Option<f64>,
    pub tobs: Option<f64>,
    pub chisq: Option<f64>,
    pub bline: Option<f64>,
    pub spherical_solar_wind: Option<f64>,
}
impl Default for SourceObservation {
    fn default() -> Self {
        Self {
            sat: None,
            origsat: None,
            sat_day: None,
            sat_sec: None,
            bat: None,
            batCorr: None,
            bbat: None,
            pet: None,
            clockCorr: None,
            delayCorr: None,
            deleted: None,
            prefitResidual: None,
            residual: None,
            residualtn: None,
            addedNoise: None,
            TNRedSignal: None,
            TNRedErr: None,
            TNDMSignal: None,
            TNDMErr: None,
            SWGPSignal: None,
            SWGPErr: None,
            TNGroupSignal: None,
            TNGroupErr: None,
            TNChromSignal: None,
            TNChromErr: None,
            freq: None,
            freqSSB: None,
            toaErr: None,
            toaDMErr: None,
            origErr: None,
            phaseOffset: None,
            averagebat: None,
            averageres: None,
            averageerr: None,
            averagedmbat: None,
            averagedmres: None,
            averagedmerr: None,
            fname: [None; MAX_FILELEN + 1],
            telID: [None; 100],
            correctionsTT: std::array::from_fn(|_| SourceClockCorrection::default()),
            nclock_correction: None,
            correctionTT_TB: None,
            einsteinRate: None,
            correctionTT_calcEph: None,
            correctionTT_Teph: None,
            correctionUT1: None,
            sun_ssb: [None; 6],
            sun_earth: [None; 6],
            planet_ssb: [[None; 6]; 9],
            planet_ssb_tmr: [[None; 6]; 9],
            planet_ssb_derv: [[None; 6]; 9],
            jupiter_earth: [None; 6],
            saturn_earth: [None; 6],
            venus_earth: [None; 6],
            uranus_earth: [None; 6],
            neptune_earth: [None; 6],
            earthMoonBary_ssb: [None; 6],
            earthMoonBary_earth: [None; 6],
            earth_ssb: [None; 6],
            observatory_earth: [None; 6],
            psrPos: [None; 3],
            zenith: [None; 3],
            nutations: [None; 6],
            siteVel: [None; 3],
            shklovskii: None,
            shapiroDelaySun: None,
            shapiroDelayJupiter: None,
            shapiroDelaySaturn: None,
            shapiroDelayVenus: None,
            shapiroDelayUranus: None,
            shapiroDelayNeptune: None,
            troposphericDelay: None,
            tdis1: None,
            tdis2: None,
            roemer: None,
            torb: None,
            nphase: None,
            phase: None,
            pulseN: None,
            flagID: [[None; MAX_FLAG_LEN]; MAX_FLAGS],
            flagVal: [[None; MAX_FLAG_LEN]; MAX_FLAGS],
            nFlags: None,
            jump: [None; MAX_FLAGS],
            jumpScale: [None; MAX_FLAGS],
            obsNjump: None,
            fdjump: [None; MAX_FLAGS],
            obsNfdjump: None,
            efac: None,
            equad: None,
            snr: None,
            pnoise: None,
            tobs: None,
            chisq: None,
            bline: None,
            spherical_solar_wind: None,
        }
    }
}

/// Read precisely the source cell requested by the current operation.
pub(crate) fn read<'a, T>(cell: &'a Option<T>, field: &str) -> Result<&'a T, TimingError> {
    cell.as_ref()
        .ok_or_else(|| boundary(&format!("observation_unwritten_{field}")))
}

pub(crate) fn read_string(bytes: &[Option<u8>], field: &str) -> Result<Vec<u8>, TimingError> {
    let mut text = Vec::new();
    for (i, byte) in bytes.iter().enumerate() {
        let byte = *read(byte, &format!("{field}[{i}]"))?;
        if byte == 0 {
            return Ok(text);
        }
        text.push(byte);
    }
    Err(boundary(&format!("observation_missing_terminator_{field}")))
}

/// Source strcpy through NUL. A failing write retains its completed prefix.
pub(crate) fn write_string(
    bytes: &mut [Option<u8>],
    text: &[u8],
    field: &str,
) -> Result<(), TimingError> {
    for (i, byte) in crate::clock_text::c_bytes(text)
        .iter()
        .copied()
        .chain([0])
        .enumerate()
    {
        *bytes
            .get_mut(i)
            .ok_or_else(|| boundary(&format!("observation_capacity_{field}[{i}]")))? = Some(byte);
    }
    Ok(())
}

/// Source snprintf("%s") writes at most capacity-1 bytes and preserves tails.
pub(crate) fn format_string(bytes: &mut [Option<u8>], text: &[u8]) {
    if !bytes.is_empty() {
        let text = crate::clock_text::c_bytes(text);
        let n = text.len().min(bytes.len() - 1);
        for (dst, src) in bytes[..n].iter_mut().zip(&text[..n]) {
            *dst = Some(*src);
        }
        bytes[n] = Some(0);
    }
}

impl SourceObservation {
    /// Explicit existing bounded timing caller inputs, not native allocation defaults.
    pub(crate) fn seed_clock_caller(&mut self) {
        self.clockCorr = Some(1);
        self.nclock_correction = Some(0);
        self.obsNjump = Some(0);
        for c in &mut self.correctionsTT {
            c.correction = Some(0.0);
            c.corrects_to.fill(Some(0));
        }
    }
}
