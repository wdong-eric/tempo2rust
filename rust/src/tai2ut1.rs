//! tai2ut1.C::tai2ut1/ut1red at fda6ad4. Separate process-lifetime UT1 table.
use crate::{
    TimingError,
    clkcorr::ClockSession,
    clock_diagnostics::{Diagnostics, boundary, terminal},
    clock_text,
    observation::{read, read_string},
    observatory::resource,
    pulsar_lifecycle::PulsarContext,
    runtime::Resource,
};
use std::{
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

pub(crate) struct Ut1 {
    pub read: bool,
    pub mjd1: f64,
    pub mjd2: f64,
    pub mjd_val: [i32; 1000],
    pub entry: [i32; 10000],
    pub count: usize,
    pub count2: usize,
    pub resource: Option<(PathBuf, Resource)>,
    pub fault: Option<TimingError>,
    #[cfg(test)]
    pub observed: bool,
}
impl Default for Ut1 {
    fn default() -> Self {
        Self {
            read: false,
            mjd1: 0.0,
            mjd2: 0.0,
            mjd_val: [0; 1000],
            entry: [0; 10000],
            count: 0,
            count2: 0,
            resource: None,
            fault: None,
            #[cfg(test)]
            observed: false,
        }
    }
}
impl Ut1 {
    pub fn red(
        &mut self,
        root: Option<&Path>,
        mjd: f64,
        warnings: i32,
        diagnostics: &mut Diagnostics,
    ) -> Result<f64, TimingError> {
        #[cfg(test)]
        {
            self.observed = true;
        }
        if let Some(e) = &self.fault {
            return Err(e.clone());
        }
        let result = self.calculate(root, mjd, warnings, diagnostics);
        // A native exit has no continuation. Preserve its prefix for inspection;
        // reject reuse of this failed table, without poisoning other resources.
        if let Err(e) = &result
            && !self.read
        {
            self.fault = Some(e.clone());
        }
        result
    }
    fn calculate(
        &mut self,
        root: Option<&Path>,
        mjd: f64,
        warnings: i32,
        d: &mut Diagnostics,
    ) -> Result<f64, TimingError> {
        if !self.read {
            let path = root
                .ok_or_else(|| boundary("ut1_environment"))?
                .join("clock/ut1.dat");
            let data = std::fs::read(&path).map_err(|_| {
                d.stderr(
                    &[
                        b"Could not open UT1 file: ".as_slice(),
                        path.as_os_str().as_bytes(),
                        b"\n",
                    ]
                    .concat(),
                );
                terminal(&path, "ut1_open")
            })?;
            self.resource = Some((
                path.clone(),
                resource(&path, &data, "legacy UT1 table", None),
            ));
            let mut remaining = data.as_slice();
            // Source fgets(...,1000), independently of the 1024-byte table reader.
            let lines = std::iter::from_fn(|| {
                if remaining.is_empty() {
                    return None;
                }
                let limit = remaining.len().min(999);
                let end = remaining[..limit]
                    .iter()
                    .position(|&b| b == b'\n')
                    .map_or(limit, |n| n + 1);
                let (line, tail) = remaining.split_at(end);
                remaining = tail;
                Some(line)
            });
            for line in lines.skip(2) {
                let mut rest = &line[..line.len().min(57)];
                let mut nread = 0;
                for i in 0..7 {
                    let Some((v, n)) = clock_text::integer_prefix(rest)? else {
                        break;
                    };
                    if i == 0 {
                        self.mjd_val[self.count] = v;
                    } else {
                        self.entry[self.count2 + i - 1] = v;
                    }
                    rest = &rest[n..];
                    nread += 1;
                }
                let mut more = nread > 1;
                if more {
                    self.count += 1;
                    for _ in 0..nread - 1 {
                        if self.entry[self.count2] != 0 {
                            self.count2 += 1;
                        } else {
                            more = false;
                        }
                    }
                }
                if self.count > 999 {
                    d.stdout(
                        b"ERROR: Too many lines in ut1 file -- increase array size in tai2tdb\n",
                    );
                    return Err(terminal(&path, "ut1_row_limit"));
                }
                if !more {
                    break;
                }
            }
            if self.count == 0 {
                return Err(boundary("ut1_empty_table"));
            }
            self.mjd1 = self.mjd_val[0]
                .checked_add(5)
                .ok_or_else(|| boundary("ut1_date_integer"))? as f64;
            self.mjd2 = self.mjd_val[self.count - 1]
                .checked_sub(5)
                .ok_or_else(|| boundary("ut1_date_integer"))? as f64;
            self.read = true;
        }
        let ut1 = if mjd > self.mjd1 && mjd < self.mjd2 {
            let t = (mjd - self.mjd1) / 5.0;
            let it = crate::ifteph::native_i32(t, "ut1_index_integer")?;
            let tab = self
                .entry
                .get(it as usize..(it as usize).saturating_add(4))
                .ok_or_else(|| boundary("ut1_interpolation_backing"))?;
            let t = t - it as f64;
            let s = 1.0 - t;
            let mut y1 = [0.0; 2];
            let mut y2 = [0.0; 2];
            for i in 0..2 {
                // tai2ut1.C::ut1red:203-205,210: GCC16 O2 contracts the
                // neighbor sum into both coefficients and each interpolation term.
                // AA c017/c023 and the retained assembly witness these exact sites.
                let nr = i + 1;
                let sum = tab[nr + 1] as f64 + tab[nr - 1] as f64;
                y1[i] = (-sum).mul_add(1.0 / 6.0, 4.0 / 3.0 * tab[nr] as f64);
                y2[i] = sum.mul_add(1.0 / 6.0, -1.0 / 3.0 * tab[nr] as f64);
            }
            ((t * t).mul_add(y2[1], y1[1])).mul_add(t, s * (s * s).mul_add(y2[0], y1[0])) * 1.0e-4
        } else {
            d.display(
                1,
                b"CLK10",
                b"MJD is outside of UT1 table range ",
                format!("(MJD {mjd:.6})").as_bytes(),
                warnings,
            )?;
            let i = if mjd <= self.mjd1 {
                0
            } else {
                self.count2
                    .checked_sub(1)
                    .ok_or_else(|| boundary("ut1_empty_extrapolation"))?
            };
            self.entry[i] as f64 * 1.0e-4
        };
        Ok(-ut1)
    }
}
pub(crate) fn tai2ut1(
    s: &mut ClockSession,
    c: &mut PulsarContext,
    npsr: i32,
) -> Result<(), TimingError> {
    s.diagnostics.version("tai2ut1.C", "tai2ut1()");
    for p in 0..npsr {
        let psr = c
            .pulsars
            .get(p as usize)
            .ok_or_else(|| boundary("tai2ut1_pulsar_storage"))?;
        if *read(&psr.t2cMethod, "t2cMethod")? != 2 {
            continue;
        }
        let (psr, rows) = c.pulsar_observations_mut(p as usize)?;
        for i in 0..*read(&psr.nobs, "nobs")? {
            let o = rows
                .get_mut(i as usize)
                .ok_or_else(|| boundary("tai2ut1_observation_storage"))?;
            o.correctionUT1 = Some(rug::Float::with_val(
                128,
                if *read(&o.clockCorr, "clockCorr")? != 0 {
                    let ut = s.ut1.red(
                        s.root.as_deref(),
                        read(&o.sat, "sat")?.to_f64(),
                        *read(&psr.noWarnings, "noWarnings")?,
                        &mut s.diagnostics,
                    )?;
                    let from = read_string(&psr.clockFromOverride[..], "clockFromOverride")?;
                    ut + s.run(|s| {
                        s.get_correction(
                            o,
                            Some(&from),
                            Some(b"TAI"),
                            *read(&psr.noWarnings, "noWarnings")?,
                        )
                    })?
                } else {
                    0.0
                },
            ));
        }
    }
    Ok(())
}
