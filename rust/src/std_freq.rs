//! TEMPO2 t2fit_stdFitFuncs.C: t2FitFunc_stdFreq / t2UpdateFunc_stdFreq.
// GPL-3.0-or-later; ordinary long double follows the qualified host profile.
use crate::{
    PRECISION_BITS, TimingInput, ordinary_long_double as ordinary,
    tempo2::{Label, Label::*, ParameterStore},
    tkfit::FitNumber,
    tksvd::ld,
};
use rug::{Float, ops::Pow};

pub(crate) const SPIN: [&str; 4] = ["F0", "F1", "F2", "F3"];

/// Canonical t2FitFunc_stdFreq for bounded and general fitting (source lines 25-115).
/// Ordinary assignments and L literals round before powq promotes into Rug128.
pub(crate) fn std_freq(params: &ParameterStore, x: f64, label: Label, k: usize) -> f64 {
    let x = ordinary::round(x);
    let t2 = ordinary::mul(&x, &ordinary::round(86400));
    let arg3 = ordinary::mul(&ordinary::mul(&t2, &t2), &t2);
    let arg4 = ordinary::mul(&arg3, &t2);
    let f0 = ordinary::round(&params.slots[param_f as usize].cells[0].val);
    let f1 = ordinary::round(&params.slots[param_f as usize].cells[1].val);
    // The pinned source deliberately leaves the stored braking index unused.
    let bindex = ordinary::round(0);
    if label == param_brake {
        let mut cubic = ordinary::mul(&f1, &f1);
        cubic = ordinary::div(&cubic, &f0);
        cubic = ordinary::mul(&cubic, &arg3);
        cubic = ordinary::div(&cubic, &ordinary::round(6));
        let mut quartic = ordinary::round(ordinary::mul(&ordinary::round(4), &bindex) - 1);
        for _ in 0..3 {
            quartic = ordinary::mul(&quartic, &f1);
        }
        for _ in 0..2 {
            quartic = ordinary::div(&quartic, &f0);
        }
        quartic = ordinary::mul(&quartic, &arg4);
        quartic = ordinary::div(&quartic, &ordinary::round(24));
        return ordinary::round(cubic + quartic).to_f64();
    }
    match k {
        0 | 1 => {
            // The source assigns to double v before the optional ordinary +=.
            let v = if k == 0 {
                ordinary::div(
                    &ordinary::mul(
                        &ordinary::mul(&x, &ordinary::round(24)),
                        &ordinary::round(3600),
                    ),
                    &f0,
                )
            } else {
                ordinary::mul(&ordinary::mul(&ordinary::round(0.5), &x), &x)
            }
            .to_f64();
            if params.slots[param_brake as usize].cells[0].param_set != 1 {
                return v;
            }
            // Source left-to-right products/divisions, including the 84000 literal.
            let mut cubic = if k == 0 {
                -bindex.clone()
            } else {
                ordinary::mul(&ordinary::round(2), &bindex)
            };
            for _ in 0..if k == 0 { 2 } else { 1 } {
                cubic = ordinary::mul(&cubic, &f1);
            }
            for _ in 0..if k == 0 { 2 } else { 1 } {
                cubic = ordinary::div(&cubic, &f0);
            }
            cubic = ordinary::mul(&cubic, &arg3);
            cubic = ordinary::div(&cubic, &ordinary::round(6));
            let mut quartic = ordinary::mul(&ordinary::round(if k == 0 { 2 } else { 3 }), &bindex);
            quartic = ordinary::mul(
                &quartic,
                &ordinary::round(ordinary::mul(&ordinary::round(2), &bindex) - 1),
            );
            for _ in 0..if k == 0 { 3 } else { 2 } {
                quartic = ordinary::mul(&quartic, &f1);
            }
            for _ in 0..if k == 0 { 3 } else { 2 } {
                quartic = ordinary::div(&quartic, &f0);
            }
            quartic = ordinary::mul(&quartic, &arg4);
            quartic = ordinary::div(&quartic, &ordinary::round(24));
            let mut correction = if k == 0 {
                ordinary::round(cubic - quartic)
            } else {
                ordinary::round(cubic + quartic)
            };
            correction = ordinary::mul(
                &correction,
                &ordinary::round(if k == 0 { 84000 } else { 86400 }),
            );
            if k == 1 {
                correction = ordinary::mul(&correction, &ordinary::round(86400));
            }
            correction = ordinary::div(&correction, &f0);
            ordinary::round(ordinary::round(v) + correction).to_f64()
        }
        2 | 4 => {
            let mut value = ordinary::div(
                &ordinary::round(1),
                &ordinary::round(if k == 2 { 6 } else { 120 }),
            );
            for _ in 0..k + 1 {
                value = ordinary::mul(&value, &x);
            }
            ordinary::div(&value, &ordinary::round(if k == 2 { 1e9 } else { 1e18 })).to_f64()
        }
        3 => {
            let mut value = ordinary::div(&ordinary::round(1), &ordinary::round(24));
            value = ordinary::mul(&value, &x);
            value = ordinary::div(&value, &ordinary::round(1e18));
            for _ in 0..3 {
                value = ordinary::mul(&value, &x);
            }
            value.to_f64()
        }
        _ => {
            let divisor = match k {
                5 => 720,
                6 => 5040,
                7 => 40320,
                8 => 362880,
                _ => 3628800,
            };
            let mut factor = ordinary::div(&ordinary::round(1), &ordinary::round(divisor));
            for divisor in 11..=if k <= 13 { k + 1 } else { 14 } {
                factor = ordinary::div(&factor, &ordinary::round(divisor));
            }
            let mut base = Float::with_val(PRECISION_BITS, &x);
            if k > 13 {
                base /= k;
            }
            // powl is powq in the preprocessed native source. Only its inputs,
            // the preceding factorial divisions and the L denominator are ordinary.
            let denominator =
                ordinary::round(Float::parse(if k < 10 { "1e18" } else { "1e23" }).unwrap());
            (Float::with_val(PRECISION_BITS, factor)
                * base.pow(&Float::with_val(PRECISION_BITS, k + 1))
                / Float::with_val(PRECISION_BITS, denominator))
            .to_f64()
        }
    }
}

pub(crate) fn update_std_freq(psr: &mut TimingInput, k: usize, val: f64, error: f64) {
    update_spin(&mut psr.source.pulsars[0].param, k, val, error);
}

/// Canonical t2UpdateFunc_stdFreq arithmetic shared by bounded and general fits.
pub(crate) fn update_spin(
    params: &mut crate::tempo2::ParameterStore,
    k: usize,
    val: f64,
    error: f64,
) {
    let cells = &mut params.slots[crate::tempo2::Label::param_f as usize].cells;
    let f0 = cells[0].val.clone();
    if k == 0 {
        cells[k].val *= ld(1.0) - ld(val) / &f0;
        if error >= 0.0 {
            cells[k].err = ld(error);
        }
    } else {
        let literal = if k == 2 {
            "1e9"
        } else if k > 2 && k < 10 {
            "1e18"
        } else if k > 9 {
            "1e23"
        } else {
            "1"
        };
        let scale = Float::with_val(
            PRECISION_BITS,
            ordinary::round(Float::parse(literal).unwrap()),
        );
        let divisor = (24.0f64 * 3600.0).powf((k + 1) as f64);
        cells[k].val -= f0.clone() * (val / divisor) / &scale;
        if error >= 0.0 {
            cells[k].err = ld(error / divisor) / &scale * &f0;
        }
    }
}

pub(crate) fn spin_values(psr: &TimingInput) -> [Float; 4] {
    SPIN.map(|key| psr.parameter_cell(key).val.clone())
}

pub(crate) fn spin_errors(psr: &TimingInput) -> [FitNumber; 4] {
    SPIN.map(|key| FitNumber(psr.parameter_cell(key).err.clone()))
}
