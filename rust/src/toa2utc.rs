//! Active toa2utc.C::toa2utc (USE_NEW_CLK_CORR), including its SAT jump index
//! mismatch and extra shifted slot. Reads/writes the canonical source observation.
use crate::{
    TimingError,
    clkcorr::ClockSession,
    clock_diagnostics::boundary,
    clock_text::c_bytes,
    observation::{format_string, read},
    tempo2::SourceObservation,
};

/// Borrows canonical source fields. Overrides are the existing bounded single-TOA caller.
pub struct ClockPulsarView<'a> {
    pub source: &'a crate::pulsar::SourcePulsar,
    pub nobs: Option<i32>,
    pub no_warnings: Option<i32>,
    pub target: Option<&'a [u8]>,
    pub no_sat_jumps: bool,
}
impl<'a> ClockPulsarView<'a> {
    pub fn source(source: &'a crate::pulsar::SourcePulsar) -> Self {
        Self {
            source,
            nobs: None,
            no_warnings: None,
            target: None,
            no_sat_jumps: false,
        }
    }
    pub fn single_observation(source: &'a crate::pulsar::SourcePulsar) -> Self {
        Self {
            source,
            nobs: Some(1),
            no_warnings: Some(1),
            target: Some(b"TT(TAI)"),
            no_sat_jumps: true,
        }
    }
}
pub fn toa2utc(
    session: &mut ClockSession,
    pulsars: &[ClockPulsarView<'_>],
    observations: &mut [impl AsMut<[SourceObservation]>],
    npsr: i32,
) -> Result<(), TimingError> {
    session.diagnostics.version("toa2utc.C", "toa2utc__1()");
    for p in 0..npsr {
        let psr = pulsars
            .get(p as usize)
            .ok_or_else(|| boundary("clock_pulsar_storage"))?;
        let nobs = psr
            .nobs
            .or(psr.source.nobs)
            .ok_or_else(|| boundary("observation_unwritten_nobs"))?;
        for i in 0..nobs {
            let obs = observations
                .get_mut(p as usize)
                .and_then(|v| v.as_mut().get_mut(i as usize))
                .ok_or_else(|| boundary("clock_observation_storage"))?;
            if *read(&obs.clockCorr, "clockCorr")? != 0 && *read(&obs.clockCorr, "clockCorr")? != 2
            {
                let warnings = psr
                    .no_warnings
                    .or(psr.source.noWarnings)
                    .ok_or_else(|| boundary("observation_unwritten_noWarnings"))?;
                // getClockCorrections resets this before inspecting either string.
                obs.nclock_correction = Some(0);
                let from = crate::observation::read_string(
                    &psr.source.clockFromOverride[..],
                    "clockFromOverride",
                )?;
                let to = match psr.target {
                    Some(value) => value.to_vec(),
                    None => crate::observation::read_string(&psr.source.clock[..], "clock")?,
                };
                session.get_clock_corrections(obs, Some(&from), Some(&to), warnings)?;
            }
            let njumps = if psr.no_sat_jumps {
                0
            } else {
                *read(&psr.source.nJumps, "nJumps")?
            };
            for k in 1..=njumps {
                for l in 0..*read(&obs.obsNjump, "obsNjump")? {
                    let l = l as usize;
                    if *read(
                        obs.jump
                            .get(l)
                            .ok_or_else(|| boundary("clock_jump_storage"))?,
                        &format!("jump[{l}]"),
                    )? == k
                        && *read(
                            psr.source
                                .jumpSAT
                                .get(l)
                                .ok_or_else(|| boundary("clock_jump_storage"))?,
                            &format!("jumpSAT[{l}]"),
                        )? == 1
                    {
                        obs.nclock_correction = Some(
                            read(&obs.nclock_correction, "nclock_correction")?
                                .checked_add(1)
                                .ok_or_else(|| boundary("clock_count_integer"))?,
                        );
                        for index in (1..=*read(&obs.nclock_correction, "nclock_correction")?).rev()
                        {
                            let index = index as usize;
                            let old = obs
                                .correctionsTT
                                .get(index - 1)
                                .ok_or_else(|| boundary("clock_correction_storage"))?
                                .clone();
                            let target = obs
                                .correctionsTT
                                .get_mut(index)
                                .ok_or_else(|| boundary("clock_correction_storage"))?;
                            target.correction = Some(*read(
                                &old.correction,
                                &format!("correctionsTT[{}].correction", index - 1),
                            )?);
                            let mut terminated = false;
                            for j in 0..31 {
                                let byte = if terminated {
                                    0
                                } else {
                                    *read(
                                        &old.corrects_to[j],
                                        &format!("correctionsTT[{}].corrects_to[{j}]", index - 1),
                                    )?
                                };
                                target.corrects_to[j] = Some(byte);
                                terminated |= byte == 0;
                            } // strncpy padding, byte 31 untouched
                        }
                        obs.correctionsTT[0].correction = Some(*read(
                            psr.source
                                .jumpVal
                                .get(k as usize)
                                .ok_or_else(|| boundary("clock_jump_storage"))?,
                            &format!("jumpVal[{k}]"),
                        )?);
                        let label = crate::observation::read_string(
                            psr.source
                                .jumpStr
                                .get(k as usize)
                                .ok_or_else(|| boundary("clock_jump_storage"))?,
                            &format!("jumpStr[{k}]"),
                        )?;
                        format_string(
                            &mut obs.correctionsTT[0].corrects_to[..31],
                            &[b"SATJMP ".as_slice(), c_bytes(&label)].concat(),
                        );
                    }
                }
            }
        }
    }
    Ok(())
}
