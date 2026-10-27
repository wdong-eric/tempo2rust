//! global.C source orchestration with one outer process lock per driver call.
use crate::{
    RuntimeContext, TimingError, clkcorr::ClockSession, observation::read,
    pulsar_lifecycle::PulsarContext, runtime::Resource,
};
fn clocks(
    s: &mut ClockSession,
    c: &mut PulsarContext,
    resources: &mut Vec<Resource>,
) -> Result<(), TimingError> {
    let count = c.pulsars.len() as i32;
    s.run(|s| {
        for i in 0..c.pulsars.len() {
            let (p, rows) = c.pulsar_observations_mut(i)?;
            let views = [crate::toa2utc::ClockPulsarView::source(p)];
            crate::toa2utc::toa2utc(s, &views, &mut [rows], 1)?;
        }
        Ok(())
    })?;
    crate::tai2ut1::tai2ut1(s, c, count)?;
    let first = c
        .pulsars
        .first()
        .ok_or_else(|| crate::clock_diagnostics::boundary("global_empty_pulsar_context"))?;
    if *read(&first.useCalceph, "useCalceph")? != 0 {
        crate::calceph::time(s, c, resources)?;
    }
    crate::tt2tdb::tt2tb(s, c, count)
}
fn ephemerides(
    s: &mut ClockSession,
    c: &mut PulsarContext,
    resources: &mut Vec<Resource>,
) -> Result<(), TimingError> {
    let count = c.pulsars.len() as i32;
    crate::vector_pulsar::vector_pulsar(s, c, count)?;
    let first = c
        .pulsars
        .first()
        .ok_or_else(|| crate::clock_diagnostics::boundary("global_empty_pulsar_context"))?;
    let calceph = *read(&first.useCalceph, "useCalceph")? != 0;
    if calceph {
        crate::calceph::positions(s, c, resources)?;
    } else {
        crate::read_ephemeris::driver(s, c, resources, 0)?;
    }
    crate::get_obs_coord::driver(s, c, count)?;
    crate::tt2tdb::tt2tb(s, c, count)?;
    if calceph {
        crate::calceph::time(s, c, resources)?;
        crate::calceph::positions(s, c, resources)
    } else {
        crate::read_ephemeris::driver(s, c, resources, 0)
    }
}
pub fn clock_corrections(c: &mut PulsarContext, r: &mut RuntimeContext) -> Result<(), TimingError> {
    crate::clkcorr::with_resources(&r.root, |s| clocks(s, c, &mut r.resources))
}
pub fn ephemeris_routines(
    c: &mut PulsarContext,
    r: &mut RuntimeContext,
) -> Result<(), TimingError> {
    crate::clkcorr::with_resources(&r.root, |s| ephemerides(s, c, &mut r.resources))
}
pub fn extra_delays(c: &mut PulsarContext, r: &mut RuntimeContext) -> Result<(), TimingError> {
    crate::clkcorr::with_resources(&r.root, |s| {
        crate::propagation::calculate(s, c, &mut r.resources)
    })
}
pub fn form_bats_all(c: &mut PulsarContext, r: &mut RuntimeContext) -> Result<(), TimingError> {
    crate::clkcorr::with_resources(&r.root, |s| {
        crate::refphs::init(c, s.source.max_observations, &mut s.diagnostics)?;
        clocks(s, c, &mut r.resources)?;
        ephemerides(s, c, &mut r.resources)?;
        crate::propagation::calculate(s, c, &mut r.resources)?;
        crate::propagation::form_bats_inner(c, &mut s.diagnostics)?;
        crate::propagation::secular_inner(c, &mut s.diagnostics)?;
        crate::refphs::clean(c, &mut s.diagnostics)?;
        s.export_resources(&r.root, &mut r.resources);
        s.export_terrestrial_resources(&r.root, &mut r.resources);
        s.export_time_resources(&r.root, &mut r.resources);
        Ok(())
    })
}
pub fn update_bats_all(c: &mut PulsarContext, r: &mut RuntimeContext) -> Result<(), TimingError> {
    crate::clkcorr::with_resources(&r.root, |s| {
        crate::vector_pulsar::vector_pulsar(s, c, c.pulsars.len() as i32)?;
        crate::propagation::calculate(s, c, &mut r.resources)?;
        crate::propagation::form_bats_inner(c, &mut s.diagnostics)?;
        crate::propagation::secular_inner(c, &mut s.diagnostics)
    })
}
