// TEMPO2 eop.C::load_EOP/get_EOP subset; Eop owns the source's static samples.
// The checked upper-date boundary is a documented adapter departure. GPL-3.0-or-later.
use crate::TimingError;
use std::path::{Path, PathBuf};
pub(crate) struct Eop {
    path: PathBuf,
    samples: Vec<[f64; 4]>,
}
impl Eop {
    pub fn coverage(&self) -> [f64; 2] {
        [self.samples[0][0], self.samples.last().unwrap()[0]]
    }
    pub fn load_eop(path: &Path, text: &str) -> Result<Self, TimingError> {
        let mut modern = false;
        let mut samples: Vec<[f64; 4]> = Vec::new();
        for (i, line) in text.lines().enumerate() {
            let error = |m| TimingError::new(path, Some(i + 1), "EOP", m);
            let t: Vec<_> = line.split_whitespace().collect();
            if t.first()
                .and_then(|s| s.parse::<i32>().ok())
                .is_some_and(|y| y > 1966 && y < 3000)
            {
                let fields = if modern {
                    t.get(3..7)
                        .ok_or_else(|| error("truncated EOP row"))?
                        .to_vec()
                } else {
                    [(17, 22), (22, 31), (31, 40), (40, 50)]
                        .iter()
                        .map(|&(a, b)| {
                            line.get(a..b)
                                .ok_or_else(|| error("truncated fixed-width EOP row"))
                        })
                        .collect::<Result<Vec<_>, _>>()?
                };
                let mut row = [0.0; 4];
                for j in 0..4 {
                    row[j] = fields[j]
                        .trim()
                        .parse::<f64>()
                        .map_err(|_| error("malformed EOP number"))?;
                }
                // The source assumes usable ordered samples when dividing by
                // interval widths. Reject corrupt data rather than return NaN
                // or use an invalid interpolation bracket.
                if row.iter().any(|x| !x.is_finite())
                    || samples.last().is_some_and(|last| row[0] <= last[0])
                {
                    return Err(error("nonfinite or unordered EOP sample"));
                }
                row[1] *= std::f64::consts::PI / (180.0 * 60.0 * 60.0);
                row[2] *= std::f64::consts::PI / (180.0 * 60.0 * 60.0);
                samples.push(row);
            } else if line
                .contains("FORMAT(3(I4),I7,2(F11.6),2(F12.7),2(F11.6),2(F11.6),2(F11.7),2F12.6)")
                || line.contains(
                    "FORMAT(3(I4),I7,2(F11.6),2(F12.7),2(F11.6),2(F11.6),2(F11.7),2(F12.6))",
                )
            {
                modern = true;
            } else if line.contains("FORMAT(2X,I4,2X,A4,I3,2X,I5,2F9.6,F10.7,2X,F10.7,2X,2F9.6)") {
                modern = false;
            }
        }
        // get_EOP needs two samples for its derivative; the source otherwise
        // indexes absent entries. Treat an incomplete resource as recoverable.
        if samples.len() < 2 {
            return Err(TimingError::new(
                path,
                None,
                "EOP",
                "need at least two usable samples",
            ));
        }
        Ok(Self {
            path: path.into(),
            samples,
        })
    }
    pub fn get_eop(
        &self,
        mut mjd: f64,
        warnings: &mut Vec<String>,
    ) -> Result<[f64; 4], TimingError> {
        if mjd < self.samples[0][0] {
            // Preserve the source's lower-end proxy, including first-interval slope.
            warnings.push(format!(
                "EPH1: no EOP at {mjd}; using {}",
                self.samples[0][0]
            ));
            mjd = self.samples[0][0];
        }
        // get_EOP sets the upper proxy then reads samp[size] for dut1dot.
        // There is no defined source value at/after the final sample: return an
        // error instead of out-of-bounds access; see eop_upper_boundary_recovers.
        if mjd >= self.samples.last().unwrap()[0] {
            return Err(TimingError::new(
                &self.path,
                None,
                "EOP",
                format!("MJD {mjd} has no following sample for dut1dot"),
            ));
        }
        let i = self.samples.partition_point(|s| s[0] <= mjd);
        let a = self.samples[i - 1];
        let b = self.samples[i];
        let leap = if b[3] - a[3] > 0.5 {
            1.0
        } else if b[3] - a[3] < -0.5 {
            -1.0
        } else {
            0.0
        };
        let dut1dot = (b[3] - leap - a[3]) / 86400.0;
        let f = (mjd - a[0]) / (b[0] - a[0]);
        Ok([
            a[1] + f * (b[1] - a[1]),
            a[2] + f * (b[2] - a[2]),
            a[3] + f * (b[3] - leap - a[3]),
            dut1dot,
        ])
    }
}
