//! readEphemeris_calceph.C no-HAVE_CALCEPH entries and always-built unit helper.
use crate::{PRECISION_BITS, TimingError, pulsar_lifecycle::PulsarContext};
use rug::Float;
pub(crate) fn unavailable() -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| unavailable_in(&mut s.diagnostics))
}
pub(crate) fn unavailable_in(
    d: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<(), TimingError> {
    d.stdout(b"ERROR: unable to use calceph library routines as library not installed\n");
    Err(crate::clock_diagnostics::boundary(
        "CALCEPH_library_unavailable",
    ))
}
pub fn read_ephemeris_calceph(_context: &mut PulsarContext) -> Result<(), TimingError> {
    unavailable()
}
pub fn tt2tb_calceph(_context: &mut PulsarContext) -> Result<(), TimingError> {
    unavailable()
}
pub fn convert_units(v: &mut [f64; 6], units: i32, ecl: i32) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| {
        let scale = if units == 1 {
            (Float::with_val(PRECISION_BITS, 1) + crate::ifteph::IFTE_KM1).to_f64()
        } else {
            1.0
        };
        for x in v.iter_mut() {
            *x = *x * 1000.0 * scale / 299792458.0;
        }
        if ecl == 1 {
            for chunk in v.chunks_exact_mut(3) {
                let mut r = [chunk[0], chunk[1], chunk[2]];
                crate::vector_pulsar::rotate(&mut r, false, s.source.ecliptic_obliquity);
                chunk.copy_from_slice(&r);
            }
        }
        Ok(())
    })
}
