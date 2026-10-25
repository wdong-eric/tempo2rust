//! refphs.C synthetic observation ownership and complete reference record copies.
use crate::{
    TimingError,
    observation::{SourceObservation, read, read_string, write_string},
    preprocess::value,
    pulsar_lifecycle::PulsarContext,
    tempo2::Label::*,
};
pub fn refphs_init(context: &mut PulsarContext) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| init(context, s.source.max_observations, &mut s.diagnostics))
}
pub(crate) fn init(
    context: &mut PulsarContext,
    max_obsn: i32,
    d: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<(), TimingError> {
    for i in 0..context.pulsars.len() {
        if context.pulsars[i].refphs == Some(1) {
            d.log(
                b'd',
                "refphs.C",
                9,
                b"Creating extra observation to store TZR parameters for reference phase",
            );
            let n = *read(&context.pulsars[i].nobs, "nobs")?;
            if n == max_obsn {
                d.log(
                    b'e',
                    "refphs.C",
                    12,
                    b"Error, need at least 1 spare observation to reference to REFPHS TZR",
                );
                return Err(crate::clock_diagnostics::boundary(
                    "REFPHS_spare_observation",
                ));
            }
            let n = usize::try_from(n)
                .map_err(|_| crate::clock_diagnostics::boundary("REFPHS_negative_observation"))?;
            context.ensure_observation_capacity(i, n + 1)?;
            context.pulsars[i].nobs = Some(n as i32 + 1);
            let (p, rows) = context.pulsar_observations_mut(i)?;
            rows[n] = SourceObservation::zeroed();
            let o = &mut rows[n];
            o.sat = Some(value(p, param_tzrmjd, 0));
            o.freq = Some(value(p, param_tzrfrq, 0).to_f64());
            let site = read_string(&p.tzrsite[..], "tzrsite")?;
            write_string(&mut o.telID, &site, "telID")?;
            if site.starts_with(b"@") || site.eq_ignore_ascii_case(b"bat") {
                o.clockCorr = Some(0);
                o.delayCorr = Some(0);
            } else {
                o.clockCorr = Some(i32::from(site != b"STL" && site != b"STL_FBAT"));
                o.delayCorr = Some(1);
            }
        }
    }
    Ok(())
}
pub fn refphs_clean(context: &mut PulsarContext) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| clean(context, &mut s.diagnostics))
}
pub(crate) fn clean(
    context: &mut PulsarContext,
    d: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<(), TimingError> {
    for i in 0..context.pulsars.len() {
        d.log(
            b'd',
            "refphs.C",
            49,
            b"Deleting extra observation to store TZR parameters for reference phase",
        );
        // refphs.C:50 tests psr[0] while init:8 tests psr[p]. Mixed modes
        // therefore remove a real last TOA or retain an extra reference row.
        // Test the same pulsar as init so cleanup removes exactly its TZR row.
        if context.pulsars[i].refphs == Some(1) {
            let (p, rows) = context.pulsar_observations_mut(i)?;
            let last = read(&p.nobs, "nobs")?
                .checked_sub(1)
                .and_then(|n| usize::try_from(n).ok())
                .ok_or_else(|| crate::clock_diagnostics::boundary("REFPHS_empty_cleanup"))?;
            p.tzrobs = rows
                .get(last)
                .ok_or_else(|| crate::clock_diagnostics::boundary("REFPHS_cleanup_capacity"))?
                .clone();
            p.nobs = Some(last as i32);
        }
    }
    Ok(())
}
