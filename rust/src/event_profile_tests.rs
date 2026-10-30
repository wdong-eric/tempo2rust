//! Direct event callback/ordinary-type probe; no fitting can hide a timing delta.
use crate::ordinary_long_double as ordinary;
use crate::{
    PRECISION_BITS, Scalar, fit_callbacks, pulsar_lifecycle::PulsarContext, tempo2::Label::*,
};
use rug::{Float, ops::Pow};
use std::fmt::Write as _;

#[test]
#[ignore = "AL event profile probe requires explicit input and output paths"]
fn source_profile() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::var("AL_EVENT_CASES")?)?;
    let mut output = String::new();
    writeln!(
        &mut output,
        "profile\tordinary_bits\t{}",
        ordinary::NATIVE_BITS
    )?;
    writeln!(&mut output, "profile\ttiming_bits\t{PRECISION_BITS}")?;
    let mut context = PulsarContext::unwritten(1);
    context.initialise(1, 1, 0)?;
    context.pulsars[0].nobs = Some(1);
    for line in input.lines() {
        let fields: Vec<_> = line.split('\t').collect();
        let name = fields[0];
        let wide = |i: usize| -> Result<Float, Box<dyn std::error::Error>> {
            Ok(fields[i].parse::<Scalar>()?.as_float().clone())
        };
        let bbat = wide(1)?;
        let epoch = wide(2)?;
        let amplitude = wide(3)?;
        let width = wide(4)?;
        let gamma = wide(5)?;
        let set = fields[6].parse::<i32>()?;
        let frequency = fields[7].parse::<f64>()?;
        let (p, rows) = context.pulsar_observations_mut(0)?;
        rows[0].bbat = Some(bbat.clone());
        rows[0].freqSSB = Some(frequency);
        for label in [param_expep, param_gausep] {
            p.param.slots[label as usize].cells[0].val = epoch.clone();
        }
        for label in [param_expph, param_gausamp] {
            p.param.slots[label as usize].cells[0].val = amplitude.clone();
        }
        for label in [param_exptau, param_gaussig] {
            p.param.slots[label as usize].cells[0].val = width.clone();
        }
        for label in [param_expindex, param_gausindex] {
            let cell = &mut p.param.slots[label as usize].cells[0];
            cell.val = gamma.clone();
            cell.param_set = set;
        }
        let dt = ordinary::round(bbat - epoch);
        let amp = ordinary::round(amplitude);
        let width = ordinary::round(width);
        let freq = ordinary::round(frequency / 1.4e9);
        let exp_arg = ordinary::div(&(-dt.clone()), &width);
        let gaus_arg = ordinary::mul(&(-dt.clone()), &dt);
        let gaus_arg = ordinary::div(&gaus_arg, &ordinary::round(2));
        let gaus_arg = ordinary::div(&gaus_arg, &width);
        let gaus_arg = ordinary::div(&gaus_arg, &width);
        let gamma = if set == 1 {
            ordinary::round(gamma)
        } else {
            ordinary::round(0)
        };
        let val = ordinary::round(
            Float::with_val(PRECISION_BITS, &amp)
                * Float::with_val(PRECISION_BITS, &freq)
                    .pow(&Float::with_val(PRECISION_BITS, &gamma))
                * Float::with_val(PRECISION_BITS, ordinary::exp(&gaus_arg)),
        );
        for (key, value) in [
            ("freq", freq.clone()),
            ("dt", dt),
            ("amp", amp),
            ("width", width),
            ("exp", ordinary::exp(&exp_arg)),
            ("expl", ordinary::exp(&exp_arg)),
            ("exp_arg", exp_arg),
            ("log", ordinary::ln(&freq)),
            ("gaus_exp", ordinary::exp(&gaus_arg)),
            ("gaus_arg", gaus_arg),
            ("gaus_val", val),
        ] {
            writeln!(
                &mut output,
                "{name}\t{key}\t{}",
                value.to_string_radix(10, Some(45))
            )?;
        }
        for label in [
            param_expph,
            param_exptau,
            param_expindex,
            param_expep,
            param_gausamp,
            param_gausep,
            param_gaussig,
            param_gausindex,
        ] {
            let value = fit_callbacks::derivative(&mut context, 0, 0., 0, label, 0)?;
            writeln!(
                &mut output,
                "{name}\tcallback_{}\t{}",
                label as i32,
                Float::with_val(53, value).to_string_radix(10, Some(45))
            )?;
        }
    }
    std::fs::write(std::env::var("AL_EVENT_PROFILE_OUTPUT")?, output)?;
    Ok(())
}
