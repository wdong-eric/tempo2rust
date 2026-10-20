// Ported from TEMPO2 ifteph.C; GPL-3.0-or-later.
use crate::{PRECISION_BITS, TimingError};
use rug::Float;
use std::path::{Path, PathBuf};

pub(crate) const IFTE_LC: f64 = 1.48082686742e-8;
pub(crate) const IFTE_KM1: f64 = 1.55051979176e-8;
pub(crate) const IFTE_MJD0: f64 = 43144.0003725;
pub(crate) const IFTE_TEPH0: f64 = -65.564518e-6;

pub(crate) fn split_ifte_epoch(mjd: &Float) -> (f64, f64) {
    let day = mjd.clone().trunc();
    (
        2400000.0 + day.to_f64(),
        (Float::with_val(PRECISION_BITS, mjd - &day) + 0.5f64).to_f64(),
    )
}

pub(crate) struct IfteEphemeris {
    path: PathBuf,
    data: Vec<f64>,
    pub start_jd: f64,
    pub end_jd: f64,
    step_days: f64,
    coefficients_per_record: usize,
    // Source IPT: one-based offset, coefficients/component, subinterval count.
    coefficient_layout: [[usize; 3]; 2],
}

impl IfteEphemeris {
    pub fn init(path: &Path, bytes: &[u8]) -> Result<Self, TimingError> {
        let error = |message| TimingError::new(path, None, "IF99", message);
        // IFTE_init/get_Vals use shared static storage, unchecked reads and exit().
        // Own a validated immutable image per context so failure is recoverable
        // and another model/context can never inherit partial or closed-file state.
        if bytes.len() < 316 {
            return Err(error("truncated IF99 header"));
        }
        let big_endian = match &bytes[288..292] {
            [0, 0, 0, 2] => true,
            [2, 0, 0, 0] => false,
            _ => return Err(error("unrecognized IF99 byte order / constant count")),
        };
        let read_f64 = |offset| {
            let b = bytes[offset..offset + 8].try_into().unwrap();
            if big_endian {
                f64::from_be_bytes(b)
            } else {
                f64::from_le_bytes(b)
            }
        };
        let read_u32 = |offset| {
            let b = bytes[offset..offset + 4].try_into().unwrap();
            (if big_endian {
                u32::from_be_bytes(b)
            } else {
                u32::from_le_bytes(b)
            }) as usize
        };
        let (start_jd, end_jd, step_days) = (read_f64(264), read_f64(272), read_f64(280));
        let coefficient_layout = [
            [read_u32(292), read_u32(296), read_u32(300)],
            [read_u32(304), read_u32(308), read_u32(312)],
        ];
        if !(start_jd.is_finite()
            && end_jd.is_finite()
            && step_days.is_finite()
            && start_jd < end_jd
            && step_days > 0.0)
            || coefficient_layout
                .iter()
                .any(|p| p[0] < 1 || !(3..=18).contains(&p[1]) || !(1..=32).contains(&p[2]))
        {
            return Err(error("invalid IF99 coverage or interpolation dimensions"));
        }
        let coefficients_per_record =
            coefficient_layout[1][0] - 1 + 3 * coefficient_layout[1][1] * coefficient_layout[1][2];
        if coefficients_per_record > 322
            || coefficient_layout[0][0] - 1 + coefficient_layout[0][1] * coefficient_layout[0][2]
                > coefficients_per_record
            || !bytes.len().is_multiple_of(coefficients_per_record * 8)
            || bytes.len() < coefficients_per_record * 8 * 3
        {
            return Err(error("invalid IF99 record layout"));
        }
        // Decision 0005: a positive step=1e-300 passed initialization, then
        // record selection saturated to usize::MAX and overflowed on +2.
        // The owned image must contain the complete integral header grid.
        // Reject impossible metadata here instead of wrapping/panicking later.
        let data_records = bytes.len() / (coefficients_per_record * 8) - 2;
        let grid_records = (end_jd - start_jd) / step_days;
        if !grid_records.is_finite()
            || grid_records < 1.0
            || grid_records.fract() != 0.0
            || grid_records != data_records as f64
        {
            return Err(error("invalid IF99 record grid for available data"));
        }
        // Constants are decoded as upstream even though tt2tdb uses ifteph.h constants.
        if read_f64(coefficients_per_record * 8).floor() != 405.0
            || !read_f64(coefficients_per_record * 8 + 8).is_finite()
        {
            return Err(error("unsupported IF99 ephemeris constants"));
        }
        let data = (0..bytes.len()).step_by(8).map(read_f64).collect();
        Ok(Self {
            path: path.into(),
            data,
            start_jd,
            end_jd,
            step_days,
            coefficients_per_record,
            coefficient_layout,
        })
    }

    pub fn get_values(
        &self,
        jd0: f64,
        jd1: f64,
        earth_velocity: bool,
    ) -> Result<Vec<f64>, TimingError> {
        let mut jd0_integer = (jd0 - 0.5).floor();
        let mut jd0_fraction = jd0 - 0.5 - jd0_integer;
        let mut jd1_integer = jd1.floor();
        let jd1_fraction = jd1 - jd1_integer;
        jd0_integer += jd1_integer + 0.5;
        jd0_fraction += jd1_fraction;
        jd1_integer = jd0_fraction.floor();
        let jd1_fraction = jd0_fraction - jd1_integer;
        jd0_integer += jd1_integer;
        let error = |message| TimingError::new(&self.path, None, "IF99", message);
        // Source only checks the lower bound then seeks/reads unchecked upper dates.
        // Reject uncovered dates before indexing; interpolation inside coverage is unchanged.
        if jd0_integer < self.start_jd
            || jd0_integer + jd1_fraction > self.end_jd
            || !jd0_integer.is_finite()
        {
            return Err(error("epoch outside IF99 coverage"));
        }
        // Bound the floating record number before conversion or offset
        // arithmetic; a later slice check cannot recover from integer overflow.
        // Keep the source's final-epoch selection of the preceding record.
        let record_number = ((jd0_integer - self.start_jd) / self.step_days).floor();
        let data_records = self.data.len() / self.coefficients_per_record - 2;
        if !record_number.is_finite() || record_number < 0.0 || record_number > data_records as f64
        {
            return Err(error("invalid IF99 record index"));
        }
        let mut record_index = (record_number as usize)
            .checked_add(2)
            .ok_or_else(|| error("IF99 record index overflow"))?;
        if jd0_integer == self.end_jd {
            record_index -= 1;
        }
        let record_fraction = (jd0_integer
            - (self.start_jd + self.step_days * (record_index - 2) as f64)
            + jd1_fraction)
            / self.step_days;
        let [offset, coefficients_per_component, subinterval_count] =
            self.coefficient_layout[usize::from(earth_velocity)];
        let component_count = if earth_velocity { 3 } else { 1 };
        let temp = subinterval_count as f64 * record_fraction;
        let subinterval_number = temp - record_fraction.trunc();
        if !subinterval_number.is_finite()
            || subinterval_number < 0.0
            || subinterval_number >= subinterval_count as f64
        {
            return Err(error("invalid IF99 coefficient record or subinterval"));
        }
        let subinterval_index = subinterval_number as usize;
        let chebyshev_argument = 2.0 * (temp.fract() + record_fraction.trunc()) - 1.0;
        let begin = record_index
            .checked_mul(self.coefficients_per_record)
            .and_then(|base| base.checked_add(offset - 1))
            .ok_or_else(|| error("IF99 coefficient offset overflow"))?;
        let coefficient_end = begin
            .checked_add(coefficients_per_component * component_count * subinterval_count)
            .ok_or_else(|| error("IF99 coefficient offset overflow"))?;
        let coefficients = self
            .data
            .get(begin..coefficient_end)
            .ok_or_else(|| error("truncated IF99 coefficient record"))?;
        if coefficients
            .iter()
            .any(|coefficient| !coefficient.is_finite())
        {
            return Err(error("invalid IF99 coefficient record or subinterval"));
        }
        let mut chebyshev_values = [0.0; 18];
        let mut chebyshev_derivatives = [0.0; 18];
        chebyshev_values[0] = 1.0;
        chebyshev_values[1] = chebyshev_argument;
        let twice_argument = chebyshev_argument + chebyshev_argument;
        for i in 2..coefficients_per_component {
            chebyshev_values[i] =
                twice_argument * chebyshev_values[i - 1] - chebyshev_values[i - 2];
        }
        chebyshev_derivatives[1] = 1.0;
        chebyshev_derivatives[2] = twice_argument + twice_argument;
        for i in 3..coefficients_per_component {
            chebyshev_derivatives[i] = twice_argument * chebyshev_derivatives[i - 1]
                + chebyshev_values[i - 1]
                + chebyshev_values[i - 1]
                - chebyshev_derivatives[i - 2];
        }
        let derivative_scale =
            (subinterval_count as f64 + subinterval_count as f64) / self.step_days;
        let mut values_and_derivatives = vec![0.0; component_count * 2];
        for i in 0..component_count {
            let mut derivative = 0.0;
            for j in (0..coefficients_per_component).rev() {
                let c = coefficients
                    [coefficients_per_component * (i + subinterval_index * component_count) + j];
                values_and_derivatives[i] += chebyshev_values[j] * c;
                derivative += chebyshev_derivatives[j] * c;
            }
            values_and_derivatives[i + component_count] = derivative * derivative_scale;
        }
        Ok(values_and_derivatives)
    }
}
