//! read_fortran2.h stream lifetime, independent of the FB read_fortran.h stream.
use crate::{TimingError, clock_diagnostics::boundary};

pub fn open_file2(filename: &mut [u8], swap: &mut i32) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| {
        *swap = i32::from(cfg!(target_endian = "little"));
        s.diagnostics
            .stdout(format!("Byte swapping = {}\n", *swap).as_bytes());
        let status = s.fortran2.open_file(filename, &mut s.diagnostics)?;
        if let Some(resource) = &mut s.fortran2.resource {
            resource.role = "Fortran2 input".into();
        }
        if status != 0 {
            return Err(boundary("Fortran2_open_failed"));
        }
        Ok(())
    })
}
pub fn close_file2() -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| s.fortran2.close_file())
}
pub fn read_character2(len: i32, out: &mut [u8]) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|s| {
        if s.fortran2.swap_byte == 1 {
            for i in (0..len).rev() {
                *out.get_mut(i as usize)
                    .ok_or_else(|| boundary("Fortran2_character_storage"))? =
                    s.fortran2.read_char()? as u8;
            }
        } else {
            s.fortran2.read_character(len, out)?;
        }
        Ok(())
    })
}
pub fn read_int2() -> Result<i32, TimingError> {
    crate::clkcorr::with_process(|s| s.fortran2.read_int())
}
pub fn read_float2() -> Result<f32, TimingError> {
    crate::clkcorr::with_process(|s| s.fortran2.read_float())
}
pub fn read_double2() -> Result<f64, TimingError> {
    crate::clkcorr::with_process(|s| s.fortran2.read_double())
}
pub fn read_record_int2() -> Result<i32, TimingError> {
    crate::clkcorr::with_process(|s| s.fortran2.read_record_int())
}
