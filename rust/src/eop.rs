//! eop.C::load_EOP/get_EOP at fda6ad4: append loading and process-first-use series.
//! Checked output indices retain source alias/write order without native pointers.
use crate::{
    TimingError,
    clock_diagnostics::{Diagnostics, boundary, terminal},
    clock_text::{self, c_bytes},
    observatory::resource,
    runtime::Resource,
    tabulated_function::{chunks, os_error},
};
use std::{
    fs,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

pub(crate) fn load_eop(
    samples: &mut Vec<[f64; 4]>,
    root: Option<&Path>,
    filename: &[u8],
    diagnostics: &mut Diagnostics,
) -> Result<Resource, TimingError> {
    diagnostics.version("eop.C", "load_EOP()");
    let root = root.ok_or_else(|| boundary("eop_environment_storage"))?;
    let name = [root.as_os_str().as_bytes(), b"/", c_bytes(filename)].concat();
    if name.len() >= 1024 {
        return Err(boundary("eop_path_storage"));
    }
    let path = PathBuf::from(std::ffi::OsStr::from_bytes(&name));
    let data = fs::read(&path).map_err(|e| {
        diagnostics.stderr(
            &[
                b"Fatal Error: Unable to open file ".as_slice(),
                &name,
                b" for reading: ",
                os_error(&e).as_bytes(),
                b"\n",
            ]
            .concat(),
        );
        terminal(&path, "eop_open")
    })?;
    let mut modern = false;
    for (i, line) in chunks(&data).enumerate() {
        let year = clock_text::integer_prefix(line)?;
        if year.is_some_and(|(y, _)| y > 1966 && y < 3000) {
            let mut row = [0.0; 4];
            let parsed = if modern {
                let mut tail = c_bytes(line);
                let mut valid = true;
                for _ in 0..3 {
                    if let Some((_, n)) = clock_text::integer_prefix(tail)? {
                        tail = &tail[n..];
                    } else {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    for cell in &mut row {
                        if let Some((v, n)) = clock_text::number_prefix(tail) {
                            *cell = v;
                            tail = &tail[n..];
                        } else {
                            valid = false;
                            break;
                        }
                    }
                }
                valid
            } else {
                if line.len() < 50 {
                    return Err(boundary("eop_fixed_line_storage"));
                }
                let mut valid = true;
                for (cell, (a, b)) in row.iter_mut().zip([(17, 22), (22, 31), (31, 40), (40, 50)]) {
                    if let Some((v, _)) = clock_text::number_prefix(&line[a..b]) {
                        *cell = v;
                    } else {
                        valid = false;
                        break;
                    }
                }
                valid
            };
            if !parsed {
                diagnostics.stderr(
                    &[
                        format!("Error parsing line {} of ", i + 1).as_bytes(),
                        &name,
                    ]
                    .concat(),
                );
                return Err(terminal(&path, "eop_row"));
            }
            row[1] *= std::f64::consts::PI / (180.0 * 60.0 * 60.0);
            row[2] *= std::f64::consts::PI / (180.0 * 60.0 * 60.0);
            samples.push(row);
        } else {
            let contains = |needle: &[u8]| c_bytes(line).windows(needle.len()).any(|w| w == needle);
            if contains(b"FORMAT(2X,I4,2X,A4,I3,2X,I5,2F9.6,F10.7,2X,F10.7,2X,2F9.6)") {
                modern = false;
                diagnostics.stdout(b"Warning: using old format for $TEMPO2/eopc04_IAU2000.62-now - should update\n");
            } else if contains(
                b"FORMAT(3(I4),I7,2(F11.6),2(F12.7),2(F11.6),2(F11.6),2(F11.7),2F12.6)",
            ) {
                modern = true;
                diagnostics.log(
                    b'd',
                    "eop.C",
                    124,
                    b"Using new format for $TEMPO2/eopc04_IAU2000.62-now",
                );
            } else if contains(
                b"FORMAT(3(I4),I7,2(F11.6),2(F12.7),2(F11.6),2(F11.6),2(F11.7),2(F12.6))",
            ) {
                modern = true;
                diagnostics.log(
                    b'd',
                    "eop.C",
                    130,
                    b"Using new format for $TEMPO2/eopc04_IAU2000.62-now",
                );
            }
        }
    }
    Ok(resource(
        &path,
        &data,
        "Earth orientation",
        samples
            .first()
            .zip(samples.last())
            .map(|(a, b)| [a[0], b[0]]),
    ))
}
#[derive(Default)]
pub(crate) struct Eop {
    pub initialized: bool,
    pub samples: Vec<[f64; 4]>,
    pub resource: Option<Resource>,
}
impl Eop {
    pub fn initialize(
        &mut self,
        root: Option<&Path>,
        filename: &[u8],
        diagnostics: &mut Diagnostics,
    ) -> Result<(), TimingError> {
        if !self.initialized {
            self.resource = Some(load_eop(&mut self.samples, root, filename, diagnostics)?);
            self.initialized = true;
        }
        Ok(())
    }
    pub fn get_eop(
        &self,
        mut mjd: f64,
        outputs: &mut [f64],
        indices: [usize; 4],
        warnings: i32,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), TimingError> {
        if indices.iter().any(|&i| i >= outputs.len()) {
            return Err(boundary("eop_output_storage"));
        }
        let [xp, yp, dut1, dot] = indices;
        let samples = &self.samples;
        if samples.len() < 2
            || !mjd.is_finite()
            || samples.iter().any(|r| !r[0].is_finite())
            || samples.windows(2).any(|w| w[0][0] >= w[1][0])
        {
            return Err(boundary("eop_bracket_storage"));
        }
        let proxy = if samples[0][0] > mjd {
            Some(samples[0])
        } else if samples.last().unwrap()[0] <= mjd {
            Some(*samples.last().unwrap())
        } else {
            None
        };
        if let Some(row) = proxy {
            outputs[xp] = row[1];
            outputs[yp] = row[2];
            outputs[dut1] = row[3];
            if warnings != 2 {
                diagnostics.display(
                    1,
                    b"EPH1",
                    b"No Earth orientation parameters for MJD ",
                    format!("{mjd:.2}. Using {:.1} instead", row[0]).as_bytes(),
                    warnings,
                )?;
            }
            mjd = row[0];
        }
        let i = samples.partition_point(|s| s[0] <= mjd);
        // eop.C:179-199 reads samp[size] at/after the final row. Preserve the
        // initialized proxy prefix, then return the established recoverable error.
        if i == 0 || i == samples.len() {
            return Err(boundary("eop_following_sample"));
        }
        let a = samples[i - 1];
        let b = samples[i];
        let leap = if b[3] - a[3] > 0.5 {
            1.0
        } else if b[3] - a[3] < -0.5 {
            -1.0
        } else {
            0.0
        };
        outputs[dot] = (b[3] - leap - a[3]) / 86400.0;
        let f = (mjd - a[0]) / (b[0] - a[0]);
        // eop.C:197-199: three GCC16 -O2 fmadd sites; subtraction and
        // division remain separate (W arithmetic evidence).
        outputs[xp] = f.mul_add(b[1] - a[1], a[1]);
        outputs[yp] = f.mul_add(b[2] - a[2], a[2]);
        outputs[dut1] = f.mul_add(b[3] - leap - a[3], a[3]);
        Ok(())
    }
}
