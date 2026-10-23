//! Active toa2utc.C::toa2utc (USE_NEW_CLK_CORR), including its SAT jump index
//! mismatch and extra shifted slot. Owned fields project only those consumed here.
use crate::{
    TimingError,
    clkcorr::{ClockObservation, ClockSession},
    clock_diagnostics::boundary,
    clock_text::{c_bytes, copy_string},
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
    observations: &mut [Vec<ClockObservation>],
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
            if obs.clock_corr != 0 && obs.clock_corr != 2 {
                session.get_clock_corrections(
                    obs,
                    Some(&psr.from),
                    Some(&psr.to),
                    psr.no_warnings,
                )?;
            }
            for k in 1..=psr.njumps {
                for l in 0..obs.jumps.len() {
                    if obs.jumps[l] == k
                        && *psr
                            .jump_sat
                            .get(l)
                            .ok_or_else(|| boundary("clock_jump_storage"))?
                            == 1
                    {
                        obs.count = obs
                            .count
                            .checked_add(1)
                            .ok_or_else(|| boundary("clock_count_integer"))?;
                        for index in (1..=obs.count).rev() {
                            let index = index as usize;
                            let old = obs
                                .corrections
                                .get(index - 1)
                                .ok_or_else(|| boundary("clock_correction_storage"))?
                                .clone();
                            let target = obs
                                .corrections
                                .get_mut(index)
                                .ok_or_else(|| boundary("clock_correction_storage"))?;
                            target.correction = old.correction;
                            let text = c_bytes(&old.corrects_to);
                            let n = text.len().min(31);
                            target.corrects_to[..n].copy_from_slice(&text[..n]);
                            target.corrects_to[n..31].fill(0); // strncpy padding, byte 31 untouched
                        }
                        obs.corrections[0].correction = *psr
                            .jump_value
                            .get(k as usize)
                            .ok_or_else(|| boundary("clock_jump_storage"))?;
                        let label = psr
                            .jump_string
                            .get(k as usize)
                            .ok_or_else(|| boundary("clock_jump_storage"))?;
                        copy_string(
                            &mut obs.corrections[0].corrects_to[..31],
                            &[b"SATJMP ".as_slice(), c_bytes(label)].concat(),
                        );
                    }
                }
            }
        }
    }
    Ok(())
}
