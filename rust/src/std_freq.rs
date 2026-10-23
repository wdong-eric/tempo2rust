//! TEMPO2 t2fit_stdFitFuncs.C: t2FitFunc_stdFreq / t2UpdateFunc_stdFreq.
// GPL-3.0-or-later; source-native long double is binary64 on the accepted arm64 host.
use crate::{TimingInput, tkfit::FitNumber, tksvd::ld};
use rug::Float;

pub(crate) const SPIN: [&str; 4] = ["F0", "F1", "F2", "F3"];

pub(crate) fn std_freq(x: f64, f0: f64, k: usize) -> f64 {
    match k {
        0 => x * 24.0 * 3600.0 / f0,
        1 => 0.5 * x * x,
        2 => 1.0 / 6.0 * x * x * x / 1.0e9,
        3 => 1.0 / 24.0 * x / 1.0e18 * x * x * x,
        _ => unreachable!("selected F0-F3 only"),
    }
}

pub(crate) fn update_std_freq(psr: &mut TimingInput, k: usize, val: f64, error: f64) {
    let f0 = psr.parameter_cell("F0").val.clone();
    let value = &mut psr.parameter_cell_mut(SPIN[k]).val;
    if k == 0 {
        *value *= ld(1.0) - ld(val) / &f0;
        // Preserve the source predicate: NaN leaves the existing physical error unchanged.
        if error >= 0.0 {
            psr.parameter_cell_mut(SPIN[k]).err = ld(error);
        }
    } else {
        let scale = [1.0, 1.0, 1.0e9, 1.0e18][k];
        let divisor = (24.0f64 * 3600.0).powf((k + 1) as f64);
        *value -= f0.clone() * (val / divisor) / scale;
        if error >= 0.0 {
            psr.parameter_cell_mut(SPIN[k]).err = ld(error / divisor) / scale * &f0;
        }
    }
}

pub(crate) fn spin_values(psr: &TimingInput) -> [Float; 4] {
    SPIN.map(|key| psr.parameter_cell(key).val.clone())
}

pub(crate) fn spin_errors(psr: &TimingInput) -> [FitNumber; 4] {
    SPIN.map(|key| FitNumber(psr.parameter_cell(key).err.clone()))
}
