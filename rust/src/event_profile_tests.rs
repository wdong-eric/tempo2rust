//! Direct event callback/ordinary-type probe; no fitting can hide a timing delta.
use crate::ordinary_long_double as ordinary;
use crate::{
    PRECISION_BITS, Scalar, fit_callbacks, pulsar_lifecycle::PulsarContext, tempo2::Label::*,
};
use rug::{Float, ops::Pow};
use std::fmt::Write as _;

thread_local! {
    static RETURN_OBSERVATION: std::cell::RefCell<Option<(String, String)>> = const { std::cell::RefCell::new(None) };
}

// Test-only observation of the actual production return boundary. Store text
// before any Float clone/assignment can discard an internal NaN sign.
pub(crate) fn observe_return(before: &Float, after: f64) {
    RETURN_OBSERVATION.with_borrow_mut(|slot| {
        *slot = Some((
            before.to_string_radix(10, Some(45)),
            crate::FitNumber::from(after)
                .as_float()
                .to_string_radix(10, Some(45)),
        ))
    });
}

#[test]
#[ignore = "AL event profile probe requires explicit input and output paths"]
fn source_profile() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::var("AL_EVENT_CASES")?)?;
    let mut output = String::new();
    let mut returns = String::new();
    writeln!(
        &mut output,
        "profile\tordinary_bits\t{}",
        ordinary::NATIVE_BITS
    )?;
    writeln!(&mut output, "profile\ttiming_bits\t{PRECISION_BITS}")?;
    for token in ["nan", "-nan", "0", "-0", "inf", "-inf", "-1", "-0.75"] {
        let value = ordinary::round(Float::parse(token)?);
        for (key, number) in [
            ("exp", ordinary::exp(&value)),
            ("expl", ordinary::exp(&value)),
            ("log", ordinary::ln(&value)),
        ] {
            writeln!(
                output,
                "boundary_{token}\t{key}\t{}",
                number.to_string_radix(10, Some(45))
            )?;
        }
        writeln!(
            output,
            "boundary_{token}\tinput\t{}",
            value.to_string_radix(10, Some(45))
        )?;
        writeln!(
            output,
            "boundary_{token}\tcallback_transport\t{}",
            crate::FitNumber::from(ordinary::to_f64(&value))
                .as_float()
                .to_string_radix(10, Some(45))
        )?;
    }
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
            RETURN_OBSERVATION.with_borrow_mut(|slot| *slot = None);
            let value = fit_callbacks::derivative(&mut context, 0, 0., 0, label, 0)?;
            let observed = RETURN_OBSERVATION.with_borrow_mut(Option::take);
            if let Some((before, after)) = observed {
                writeln!(returns, "{name}\t{}\t{before}\t{after}", label as i32)?;
            }
            writeln!(
                &mut output,
                "{name}\tcallback_{}\t{}",
                label as i32,
                crate::FitNumber::from(value)
                    .as_float()
                    .to_string_radix(10, Some(45))
            )?;
        }
    }
    std::fs::write(std::env::var("AL_EVENT_PROFILE_OUTPUT")?, output)?;
    std::fs::write(
        format!("{}.returns.tsv", std::env::var("AL_EVENT_PROFILE_OUTPUT")?),
        returns,
    )?;
    Ok(())
}

/// Decision 0119 diagnostic: source-local invalid operations versus NaN operands.
/// This extends the unchanged callback observations; it changes no production path.
#[test]
#[ignore = "AL internal event NaN probe requires explicit input and output paths"]
fn nan_arithmetic_profile() -> Result<(), Box<dyn std::error::Error>> {
    source_profile()?;
    let mut output = String::new();
    let mut emit = |case: &str, key: &str, value: &Float| {
        writeln!(
            output,
            "{case}\t{key}\t{}",
            value.to_string_radix(10, Some(45))
        )
        .unwrap();
    };
    let tokens = ["nan", "-nan", "0", "-0", "inf", "-inf", "1", "-1"];
    for left in tokens {
        let a = ordinary::round(Float::parse(left)?);
        let aw = Float::with_val(PRECISION_BITS, Float::parse(left)?);
        let name = format!("assign_{left}");
        emit(
            &name,
            "ordinary_to_wide",
            &Float::with_val(PRECISION_BITS, &a),
        );
        emit(&name, "wide_to_ordinary", &ordinary::round(&aw));
        for right in tokens {
            let b = ordinary::round(Float::parse(right)?);
            let bw = Float::with_val(PRECISION_BITS, Float::parse(right)?);
            let name = format!("arithmetic_{left}_{right}");
            emit(&name, "ordinary_mul", &ordinary::mul(&a, &b));
            emit(&name, "ordinary_div", &ordinary::div(&a, &b));
            emit(
                &name,
                "wide_mul",
                &Float::with_val(PRECISION_BITS, &aw * &bw),
            );
            emit(
                &name,
                "wide_div",
                &Float::with_val(PRECISION_BITS, &aw / &bw),
            );
        }
    }
    let input = std::fs::read_to_string(std::env::var("AL_EVENT_CASES")?)?;
    for line in input.lines() {
        let fields: Vec<_> = line.split('\t').collect();
        let name = fields[0];
        if ![
            "zero_amplitude",
            "negative_zero_amplitude",
            "zero_width",
            "zero_width_onset",
        ]
        .contains(&name)
        {
            continue;
        }
        let wide = |i: usize| -> Result<Float, Box<dyn std::error::Error>> {
            Ok(Float::with_val(PRECISION_BITS, Float::parse(fields[i])?))
        };
        let dt = ordinary::round(wide(1)? - wide(2)?);
        let amp = ordinary::round(wide(3)?);
        let width = ordinary::round(wide(4)?);
        let freq = ordinary::round(fields[7].parse::<f64>()? / 1.4e9);
        let set = fields[6] == "1";
        let gamma = if set {
            ordinary::round(wide(5)?)
        } else {
            ordinary::round(0)
        };
        let gm = ordinary::mul(&(-dt.clone()), &dt);
        let half = ordinary::div(&gm, &ordinary::round(2));
        let div1 = ordinary::div(&half, &width);
        let arg = ordinary::div(&div1, &width);
        let ap = Float::with_val(PRECISION_BITS, &amp)
            * Float::with_val(PRECISION_BITS, &freq).pow(&Float::with_val(PRECISION_BITS, &gamma));
        emit(name, "trace_gaus_amp_power", &ap);
        let product = ap * Float::with_val(PRECISION_BITS, ordinary::exp(&arg));
        emit(name, "trace_gaus_wide", &product);
        let val = ordinary::round(product);
        for (key, value) in [
            ("trace_gaus_mul", &gm),
            ("trace_gaus_half", &half),
            ("trace_gaus_div1", &div1),
            ("trace_gaus_arg", &arg),
            ("trace_gaus_val", &val),
        ] {
            emit(name, key, value);
        }
        emit(name, "trace_gaus_amp_return", &ordinary::div(&val, &amp));
        let epoch = ordinary::div(&ordinary::div(&(-dt.clone()), &width), &width);
        let factor = ordinary::div(
            &ordinary::div(
                &ordinary::div(
                    &ordinary::div(&ordinary::mul(&dt, &dt), &ordinary::round(6)),
                    &width,
                ),
                &width,
            ),
            &width,
        );
        emit(name, "trace_gaus_epoch_factor", &epoch);
        emit(
            name,
            "trace_gaus_epoch_return",
            &ordinary::mul(&val, &epoch),
        );
        emit(name, "trace_gaus_width_factor", &factor);
        emit(
            name,
            "trace_gaus_width_return",
            &ordinary::mul(&val, &factor),
        );
        emit(
            name,
            "trace_gaus_index_return",
            &ordinary::mul(&val, &ordinary::ln(&freq)),
        );
        if dt > 0 {
            let gamma = if set {
                ordinary::round(wide(5)?)
            } else {
                ordinary::round(-2)
            };
            let base = Float::with_val(PRECISION_BITS, &amp)
                * Float::with_val(PRECISION_BITS, &freq)
                    .pow(&Float::with_val(PRECISION_BITS, &gamma))
                * Float::with_val(
                    PRECISION_BITS,
                    ordinary::exp(&ordinary::div(&(-dt.clone()), &width)),
                );
            emit(name, "trace_exp_base", &base);
            let epoch = Float::with_val(
                PRECISION_BITS,
                &base / Float::with_val(PRECISION_BITS, &width),
            );
            let numerator = base * Float::with_val(PRECISION_BITS, &dt);
            emit(name, "trace_exp_tau_numerator", &numerator);
            let first = numerator / Float::with_val(PRECISION_BITS, &width);
            emit(name, "trace_exp_tau_div1", &first);
            let second = first / Float::with_val(PRECISION_BITS, &width);
            emit(name, "trace_exp_tau_div2", &second);
            emit(name, "trace_exp_epoch_return", &epoch);
        }
    }
    let path = std::env::var("AL_EVENT_PROFILE_OUTPUT")?;
    let original = std::fs::read_to_string(&path)?;
    std::fs::write(path, original + &output)?;
    Ok(())
}
