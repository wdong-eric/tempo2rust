//! Eight read_fortran.h operations at fda6ad4; distinct from read_fortran2.h.
use crate::{
    TimingError,
    clock_diagnostics::{Diagnostics, boundary},
    runtime::{Resource, open_resource},
    stdio::Stream,
};
use std::{ffi::OsStr, os::unix::ffi::OsStrExt, path::Path};

#[derive(Default)]
pub(crate) struct FortranReader {
    pub stream: Option<Stream>,
    pub swap_byte: i32,
    pub resource: Option<Resource>,
}
impl FortranReader {
    pub fn open_file(
        &mut self,
        filename: &mut [u8],
        diagnostics: &mut Diagnostics,
    ) -> Result<i32, TimingError> {
        self.swap_byte = i32::from(cfg!(target_endian = "little"));
        let end = filename
            .iter()
            .position(|&v| v == 0 || v == b' ')
            .ok_or_else(|| boundary("Fortran_filename_terminator"))?;
        filename[end] = 0;
        let path = Path::new(OsStr::from_bytes(&filename[..end]));
        // read_fortran.h::open_file:59 (fda6ad4) overwrites a FILE without
        // releasing it. Safe replacement drops the sole old owner exactly once.
        self.stream = None;
        self.resource = None;
        let (file, resource) = match open_resource(path, "FB90 time ephemeris") {
            Ok(v) => v,
            Err(_) => {
                diagnostics.stdout(
                    &[
                        b"C PROGRAM: Unable to open filename >>".as_slice(),
                        &filename[..end],
                        b"<<\n",
                    ]
                    .concat(),
                );
                return Ok(1);
            }
        };
        self.stream = Some(
            Stream::from_file(file)
                .map_err(|e| TimingError::new(path, None, "Fortran_fdopen", e.to_string()))?,
        );
        self.resource = Some(resource);
        Ok(0)
    }
    pub fn close_file(&mut self) -> Result<(), TimingError> {
        // Native fclose(null/released) is invalid. Option is the sole owner and
        // a second close is a contextual boundary, never a double fclose.
        self.stream
            .take()
            .ok_or_else(|| boundary("Fortran_stream_lifetime"))?;
        Ok(())
    }
    pub fn read_char(&mut self) -> Result<i8, TimingError> {
        // read_fortran.h narrows fgetc to signed char on the accepted host.
        // EOF supplies 0xff; unlike fread(record-int), every byte is assigned.
        Ok(self
            .stream
            .as_mut()
            .ok_or_else(|| boundary("Fortran_stream_lifetime"))?
            .getc() as i8)
    }
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "complete read_fortran.h interface beyond the current FB caller"
        )
    )]
    pub fn read_character(&mut self, len: i32, out: &mut [u8]) -> Result<(), TimingError> {
        for i in 0..len {
            *out.get_mut(i as usize)
                .ok_or_else(|| boundary("Fortran_character_storage"))? = self.read_char()? as u8;
        }
        Ok(())
    }
    fn bytes<const N: usize>(&mut self) -> Result<[u8; N], TimingError> {
        let mut bytes = [0; N];
        if self.swap_byte == 1 {
            for b in bytes.iter_mut().rev() {
                *b = self.read_char()? as u8;
            }
        } else {
            for b in &mut bytes {
                *b = self.read_char()? as u8;
            }
        }
        Ok(bytes)
    }
    pub fn read_int(&mut self) -> Result<i32, TimingError> {
        Ok(i32::from_ne_bytes(self.bytes()?))
    }
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "complete read_fortran.h interface beyond the current FB caller"
        )
    )]
    pub fn read_float(&mut self) -> Result<f32, TimingError> {
        Ok(f32::from_ne_bytes(self.bytes()?))
    }
    pub fn read_double(&mut self) -> Result<f64, TimingError> {
        Ok(f64::from_ne_bytes(self.bytes()?))
    }
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "complete read_fortran.h interface beyond the current FB caller"
        )
    )]
    pub fn read_record_int(&mut self) -> Result<i32, TimingError> {
        let mut bytes = [0; 4];
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| boundary("Fortran_stream_lifetime"))?;
        // read_fortran.h:149-154 returns an uninitialized int after short fread.
        // Decode only a complete item, in native order (swapByte is irrelevant).
        if stream
            .read(&mut bytes, 4, 1)
            .map_err(|_| boundary("Fortran_record_read"))?
            != 1
        {
            return Err(boundary("Fortran_incomplete_record_int"));
        }
        Ok(i32::from_ne_bytes(bytes))
    }
    pub fn require_complete(&mut self) -> Result<(), TimingError> {
        let (_, eof, error) = self
            .stream
            .as_mut()
            .ok_or_else(|| boundary("Fortran_stream_lifetime"))?
            .status();
        if eof != 0 || error != 0 {
            return Err(boundary("FB_incomplete_record"));
        }
        Ok(())
    }
}
