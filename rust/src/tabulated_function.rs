//! tabulatedfunction.C::TabulatedFunction_load/getValue/getStartX/getEndX.
//! Source byte/chunk rules and expression order; invalid source storage has named
//! operation-local boundaries (0023). No sorting, finite gate or interval repair.
use crate::{
    TimingError,
    clock_diagnostics::{Diagnostics, boundary, terminal},
    clock_text::{NumberFormat, c_bytes, format_number, pair},
};
use sha2::{Digest, Sha256};
use std::{
    cell::RefCell,
    fs,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

#[derive(Default, Debug)]
pub(crate) struct TabulatedFunction {
    pub path: PathBuf,
    pub name: Vec<u8>,
    pub header: Vec<u8>,
    pub header_storage: Vec<u8>, // initialized prefix; may lack NUL at a boundary
    pub samples: Vec<[f64; 2]>,
    pub sha256: String,
    pub size_bytes: usize,
    fault: RefCell<Option<TimingError>>,
}
/// fgets(...,1024): each chunk includes its newline, unless capacity comes first.
pub(crate) fn chunks(mut data: &[u8]) -> impl Iterator<Item = &[u8]> {
    std::iter::from_fn(move || {
        if data.is_empty() {
            return None;
        }
        let limit = data.len().min(1023);
        let end = data[..limit]
            .iter()
            .position(|&b| b == b'\n')
            .map_or(limit, |i| i + 1);
        let (line, tail) = data.split_at(end);
        data = tail;
        Some(line)
    })
}
pub(crate) fn os_error(error: &std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error ").next().unwrap_or(&text).to_owned()
}
impl TabulatedFunction {
    pub fn load(&mut self, path: &Path, diagnostics: &mut Diagnostics) -> Result<(), TimingError> {
        self.check()?;
        diagnostics.log(
            b'd',
            "tabulatedfunction.C",
            47,
            &[
                b"Loading tabulated function from ".as_slice(),
                path.as_os_str().as_bytes(),
            ]
            .concat(),
        );
        self.samples.clear();
        self.path = path.to_owned();
        let result = match fs::read(path) {
            Ok(data) => self.read_bytes(path, &data, diagnostics),
            Err(e) => {
                diagnostics.stderr(
                    &[
                        b"Fatal Error: Unable to open file ".as_slice(),
                        path.as_os_str().as_bytes(),
                        b" for reading: ",
                        os_error(&e).as_bytes(),
                        b"\n",
                    ]
                    .concat(),
                );
                Err(terminal(path, "table_open"))
            }
        };
        self.finish(result)
    }
    pub fn from_bytes(
        path: &Path,
        data: &[u8],
        diagnostics: &mut Diagnostics,
    ) -> Result<Self, TimingError> {
        let mut table = Self {
            path: path.into(),
            ..Self::default()
        };
        diagnostics.log(
            b'd',
            "tabulatedfunction.C",
            47,
            &[
                b"Loading tabulated function from ".as_slice(),
                path.as_os_str().as_bytes(),
            ]
            .concat(),
        );
        let result = table.read_bytes(path, data, diagnostics);
        table.finish(result)?;
        Ok(table)
    }
    fn read_bytes(
        &mut self,
        path: &Path,
        data: &[u8],
        diagnostics: &mut Diagnostics,
    ) -> Result<(), TimingError> {
        self.sha256 = format!("{:x}", Sha256::digest(data));
        self.size_bytes = data.len();
        let path_bytes = path.as_os_str().as_bytes();
        let mut lines = chunks(data);
        let Some(header) = lines.next() else {
            diagnostics.stderr(
                &[
                    b"Error parsing file ".as_slice(),
                    path_bytes,
                    b": file appears empty or corrupted.\n",
                ]
                .concat(),
            );
            return Err(terminal(path, "table_empty_file"));
        };
        // Count bytes written by fgets even if an embedded NUL hides the suffix.
        if header.len() > 255 {
            self.header_storage = header[..256].to_vec();
            self.header = c_bytes(&self.header_storage).to_vec();
            return Err(boundary("table_header_storage"));
        }
        self.header_storage = [header, &[0]].concat();
        self.header = c_bytes(header).to_vec();
        if self.header.first() != Some(&b'#') {
            diagnostics.stderr(
                &[
                    b"Error parsing file ".as_slice(),
                    path_bytes,
                    b": first line must begin with #\n",
                ]
                .concat(),
            );
            return Err(terminal(path, "table_header"));
        }
        for line in lines {
            if line.first() == Some(&b'#') {
                continue;
            }
            if let Some(sample) = pair(line) {
                if let Some(last) = self.samples.last()
                    && sample[0] < last[0]
                {
                    let index = i32::try_from(self.samples.len())
                        .map_err(|_| boundary("table_sample_count"))?;
                    diagnostics.log(
                        b'w',
                        "tabulatedfunction.C",
                        85,
                        &[
                            b"Error parsing file ".as_slice(),
                            path_bytes,
                            format!(": Entry {index} out of order ").as_bytes(),
                            &format_number(sample[0], NumberFormat::Six),
                            b" < ",
                            &format_number(last[0], NumberFormat::Six),
                        ]
                        .concat(),
                    );
                }
                if self.samples.len() == i32::MAX as usize {
                    return Err(boundary("table_sample_count"));
                }
                self.samples.push(sample);
            }
        }
        diagnostics.log(
            b'd',
            "tabulatedfunction.C",
            95,
            &[
                format!("Loaded {} samples from ", self.samples.len()).as_bytes(),
                path_bytes,
            ]
            .concat(),
        );
        let name = c_bytes(path_bytes)
            .rsplit(|&b| b == b'/')
            .next()
            .unwrap_or_default();
        if name.len() >= 256 {
            return Err(boundary("table_filename_storage"));
        }
        self.name = name.to_vec();
        if self.samples.is_empty() {
            diagnostics.stderr(
                &[
                    b"Error parsing file ".as_slice(),
                    path_bytes,
                    b": no tabulated samples found.\n",
                ]
                .concat(),
            );
            return Err(terminal(path, "table_no_samples"));
        }
        Ok(())
    }
    fn check(&self) -> Result<(), TimingError> {
        self.fault.borrow().clone().map_or(Ok(()), Err)
    }
    fn finish<T>(&self, result: Result<T, TimingError>) -> Result<T, TimingError> {
        if let Err(error) = &result {
            *self.fault.borrow_mut() = Some(error.clone());
        }
        result
    }
    pub fn start(&self) -> Result<f64, TimingError> {
        self.check()?;
        self.finish(
            self.samples
                .first()
                .map(|s| s[0])
                .ok_or_else(|| boundary("table_sample_storage")),
        )
    }
    pub fn end(&self) -> Result<f64, TimingError> {
        self.check()?;
        self.finish(
            self.samples
                .last()
                .map(|s| s[0])
                .ok_or_else(|| boundary("table_sample_storage")),
        )
    }
    pub fn covers(&self, x: f64) -> Result<bool, TimingError> {
        Ok(self.start()? <= x && self.end()? >= x)
    }
    pub fn get_value(&self, x: f64, diagnostics: &mut Diagnostics) -> Result<f64, TimingError> {
        self.check()?;
        let result = (|| {
            if self.start()? > x {
                diagnostics.display(
                    1,
                    b"TAB1",
                    &[
                        b"requested value before available data! (".as_slice(),
                        &self.name,
                        b" @",
                    ]
                    .concat(),
                    &[format_number(x, NumberFormat::One), b")".to_vec()].concat(),
                    1,
                )?;
                return Ok(self.samples[0][1]);
            }
            if self.end()? < x {
                diagnostics.display(
                    1,
                    b"TAB2",
                    &[
                        b"requested value after available data! (".as_slice(),
                        &self.name,
                        b" @",
                    ]
                    .concat(),
                    &[format_number(x, NumberFormat::One), b")".to_vec()].concat(),
                    1,
                )?;
                return Ok(self.samples.last().unwrap()[1]);
            }
            let mut lo: i32 = 0;
            let mut hi =
                i32::try_from(self.samples.len()).map_err(|_| boundary("table_sample_count"))? - 1;
            loop {
                let mid = lo
                    .checked_add(hi)
                    .ok_or_else(|| boundary("table_search_integer"))?
                    / 2;
                if self.samples[mid as usize][0] > x {
                    hi = mid;
                } else {
                    lo = mid;
                }
                if hi <= lo + 1 {
                    break;
                }
            }
            if hi < 1 {
                return Err(boundary("table_bracket_storage"));
            }
            let [x0, y0] = self.samples[hi as usize - 1];
            let [x1, y1] = self.samples[hi as usize];
            // User-authorized host correspondence: J's pinned GCC -O2 emits
            // fmadd for this final multiply/add. Earlier subtraction/division
            // remain separate. This preserves the measured one-ULP knot case.
            Ok(((x - x0) / (x1 - x0)).mul_add(y1 - y0, y0))
        })();
        self.finish(result)
    }
}
