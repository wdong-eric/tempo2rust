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

pub(crate) struct ClockPulsar {
    pub nobs: i32,
    pub njumps: i32,
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub no_warnings: i32,
    pub jump_value: Vec<f64>,
    pub jump_sat: Vec<i8>,
    pub jump_string: Vec<Vec<u8>>,
}
impl Default for ClockPulsar {
    fn default() -> Self {
        Self {
            nobs: 0,
            njumps: 0,
            from: Vec::new(),
            to: Vec::new(),
            no_warnings: 0,
            jump_value: vec![0.0; 2000],
            jump_sat: vec![0; 2000],
            jump_string: vec![Vec::new(); 2000],
        }
    }
}
pub(crate) fn toa2utc(
    session: &mut ClockSession,
    pulsars: &[ClockPulsar],
    observations: &mut [Vec<SourceObservation>],
    npsr: i32,
) -> Result<(), TimingError> {
    session.diagnostics.version("toa2utc.C", "toa2utc__1()");
    for p in 0..npsr {
        let psr = pulsars
            .get(p as usize)
            .ok_or_else(|| boundary("clock_pulsar_storage"))?;
        for i in 0..psr.nobs {
            let obs = observations
                .get_mut(p as usize)
                .and_then(|v| v.get_mut(i as usize))
                .ok_or_else(|| boundary("clock_observation_storage"))?;
            if *read(&obs.clockCorr, "clockCorr")? != 0 && *read(&obs.clockCorr, "clockCorr")? != 2
            {
                session.get_clock_corrections(
                    obs,
                    Some(&psr.from),
                    Some(&psr.to),
                    psr.no_warnings,
                )?;
            }
            for k in 1..=psr.njumps {
                for l in 0..*read(&obs.obsNjump, "obsNjump")? {
                    let l = l as usize;
                    if *read(
                        obs.jump
                            .get(l)
                            .ok_or_else(|| boundary("clock_jump_storage"))?,
                        &format!("jump[{l}]"),
                    )? == k
                        && *psr
                            .jump_sat
                            .get(l)
                            .ok_or_else(|| boundary("clock_jump_storage"))?
                            == 1
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
                        obs.correctionsTT[0].correction = Some(
                            *psr.jump_value
                                .get(k as usize)
                                .ok_or_else(|| boundary("clock_jump_storage"))?,
                        );
                        let label = psr
                            .jump_string
                            .get(k as usize)
                            .ok_or_else(|| boundary("clock_jump_storage"))?;
                        format_string(
                            &mut obs.correctionsTT[0].corrects_to[..31],
                            &[b"SATJMP ".as_slice(), c_bytes(label)].concat(),
                        );
                    }
                }
            }
        }
    }
    Ok(())
}
