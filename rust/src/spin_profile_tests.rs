//! 0109: observe actual general/bounded callbacks before any arithmetic repair.
use crate::{
    PRECISION_BITS, Scalar, fit_callbacks, pulsar_lifecycle::PulsarContext, tempo2::Label::*,
};
use rug::Float;
use std::fmt::Write as _;

#[test]
#[ignore = "AL spin profile requires explicit input and output paths"]
fn source_profile() -> Result<(), Box<dyn std::error::Error>> {
    let mut output = format!(
        "profile\tordinary_bits\t{}\nprofile\ttiming_bits\t{PRECISION_BITS}\n",
        crate::ordinary_long_double::NATIVE_BITS
    );
    let mut context = PulsarContext::unwritten(1);
    context.initialise(1, 1, 0)?;
    context.pulsars[0].nobs = Some(1);
    for (k, value) in [
        "123.45678901234567890123456789",
        "-1.2345678901234567890123456789e-15",
    ]
    .iter()
    .enumerate()
    {
        context.pulsars[0].param.slots[param_f as usize].cells[k].val =
            value.parse::<Scalar>()?.as_float().clone();
    }
    for line in std::fs::read_to_string(std::env::var("AL_SPIN_CASES")?)?.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        let name = fields[0];
        let x = fields[1].parse::<f64>()?;
        let cell = &mut context.pulsars[0].param.slots[param_brake as usize].cells[0];
        cell.param_set = fields[2].parse()?;
        cell.val = Float::with_val(PRECISION_BITS, 3);
        for k in 0..=14 {
            let value = fit_callbacks::derivative(&mut context, 0, x, 0, param_f, k)?;
            writeln!(
                output,
                "{name}-F{k}\tcallback\t{}",
                Float::with_val(53, value).to_string_radix(10, Some(45))
            )?;
            if k < 4 && fields[2] == "0" {
                let bounded = crate::std_freq::std_freq(&context.pulsars[0].param, x, param_f, k);
                writeln!(
                    output,
                    "{name}-F{k}\tbounded\t{}",
                    Float::with_val(53, bounded).to_string_radix(10, Some(45))
                )?;
            }
        }
        let value = fit_callbacks::derivative(&mut context, 0, x, 0, param_brake, 0)?;
        writeln!(
            output,
            "{name}-brake\tcallback\t{}",
            Float::with_val(53, value).to_string_radix(10, Some(45))
        )?;
    }
    for k in [9, 10] {
        for (sign, value) in [("negative", -0.125), ("positive", 0.125)] {
            let params = &mut context.pulsars[0].param;
            params.slots[param_f as usize].cells[k].val = Float::with_val(PRECISION_BITS, 0);
            params.slots[param_f as usize].cells[k].err = Float::with_val(PRECISION_BITS, 0);
            crate::std_freq::update_spin(params, k, value, 0.25);
            let cell = &params.slots[param_f as usize].cells[k];
            for (key, value) in [("value", &cell.val), ("error", &cell.err)] {
                writeln!(
                    output,
                    "update{k}-{sign}\t{key}\t{}",
                    value.to_string_radix(10, Some(45))
                )?;
            }
        }
    }
    std::fs::write(std::env::var("AL_SPIN_OUTPUT")?, output)?;
    Ok(())
}
