// TEMPO2 tabulatedfunction.C::TabulatedFunction_load/TabulatedFunction_getValue/
// TabulatedFunction_getStartX/TabulatedFunction_getEndX in an owned table.
// Selected interpolation/boundary behavior; GPL-3.0-or-later.
use crate::TimingError;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct TabulatedFunction {
    pub path: PathBuf,
    pub header: String,
    pub samples: Vec<[f64; 2]>,
}

impl TabulatedFunction {
    pub fn parse(path: &Path, text: &str) -> Result<Self, TimingError> {
        let error = |line, message| TimingError::new(path, line, "table", message);
        let mut lines = text.lines();
        let header = lines
            .next()
            .filter(|s| s.starts_with('#'))
            .ok_or_else(|| error(Some(1), "first line must begin with #"))?;
        let mut samples: Vec<[f64; 2]> = Vec::new();
        for (i, line) in lines.enumerate() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }
            let mut tokens = line.split_whitespace();
            let (Some(x), Some(y)) = (tokens.next(), tokens.next()) else {
                return Err(error(Some(i + 2), "expected MJD and value"));
            };
            let parse = |s: &str| {
                crate::tempo2_util::parse_binary64(s)
                    .map_err(|e| TimingError::new(path, Some(i + 2), "table", e))
            };
            let sample = [parse(x)?, parse(y)?];
            // TabulatedFunction_load silently skips malformed rows and merely warns
            // on decreasing epochs, leaving binary search unreliable. Reject those
            // resources before interpolation; duplicate epochs (clock steps) remain valid.
            if samples.last().is_some_and(|last| sample[0] < last[0]) {
                return Err(error(Some(i + 2), "decreasing table epoch"));
            }
            samples.push(sample);
        }
        // The source indexes isamp-1 even for a singleton and can divide by zero
        // at an all-duplicate endpoint. Report invalid resource data, never invalid access.
        if samples.len() < 2 || samples[0][0] == samples.last().unwrap()[0] {
            return Err(error(None, "need at least two distinct table epochs"));
        }
        Ok(Self {
            path: path.into(),
            header: header.into(),
            samples,
        })
    }
    pub fn start(&self) -> f64 {
        self.samples[0][0]
    }
    pub fn end(&self) -> f64 {
        self.samples.last().unwrap()[0]
    }
    pub fn covers(&self, x: f64) -> bool {
        self.start() <= x && x <= self.end()
    }
    pub fn get_value(&self, x: f64, warnings: &mut Vec<String>) -> Result<f64, TimingError> {
        if x < self.start() || x > self.end() {
            let row = if x < self.start() {
                &self.samples[0]
            } else {
                self.samples.last().unwrap()
            };
            warnings.push(format!(
                "TAB: {} @ {x}: using endpoint {}",
                self.path.display(),
                row[0]
            ));
            return Ok(row[1]);
        }
        let (mut lo, mut hi) = (0, self.samples.len() - 1);
        while hi > lo + 1 {
            let mid = (lo + hi) / 2;
            if self.samples[mid][0] > x {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        let [x0, y0] = self.samples[hi - 1];
        let [x1, y1] = self.samples[hi];
        if x0 == x1 {
            return Err(TimingError::new(
                &self.path,
                None,
                "table",
                "zero-width interpolation interval",
            ));
        }
        Ok((x - x0) / (x1 - x0) * (y1 - y0) + y0)
    }
}
