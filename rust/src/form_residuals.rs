// Bounded immutable-input adapter over the complete formResiduals routine.
// The @ entry supplies the declared zero-delay BAT=BBAT=SAT branch.
// Checked integer conversions are Rust adapters. GPL-3.0-or-later.
// Copyright (C) 2006-2009 George Hobbs, Russell Edwards and TEMPO2 contributors.
#[cfg(test)]
use crate::tempo2_util::fortran_nlong;
use crate::{
    EngineProvenance, PRECISION_BITS, Scalar, TimingError, TimingInput, TimingResult, ToaResult,
};
use rug::{Float, float::Round};

pub(crate) fn pulse_integer(value: &Float) -> Option<i64> {
    // Bound the Float before allocating an arbitrary-size Integer for untrusted input.
    let integer = value.clone().trunc();
    (i64::MIN..=i64::MAX)
        .contains(&integer)
        .then(|| integer.to_integer_round(Round::Zero)?.0.to_i64())
        .flatten()
}

/// Evaluate without mutating input or exposing partially calculated observations.
/// All input observations are eligible; the first one's signed fractional phase
/// sets the reference. No fit, tracking, sorting, cuts or mean subtraction occurs.
pub fn form_residuals(psr: &TimingInput) -> Result<TimingResult, TimingError> {
    psr.require_tcb()?;
    for i in 0..psr.source_observations().len() {
        if psr.observation_site(i)? != b"@" {
            return Err(TimingError::new(
                &psr.tim.path,
                None,
                "site",
                "use evaluate_terrestrial with a RuntimeContext for PKS",
            ));
        }
    }
    let epochs: Vec<_> = psr
        .source_observations()
        .iter()
        .map(|o| crate::observation::read(&o.sat, "sat").map(|v| Scalar(v.clone())))
        .collect::<Result<_, _>>()?;
    form_residuals_at(psr, &epochs)
}

/// Shared residual kernel; epochs are BAT=BBAT for both declared isolated paths.
pub(crate) fn form_residuals_at(
    psr: &TimingInput,
    epochs: &[Scalar],
) -> Result<TimingResult, TimingError> {
    use crate::observation::read;
    let mut context = psr.source.clone();
    let (_, rows) = context.pulsar_observations_mut(0)?;
    for (o, epoch) in rows.iter_mut().zip(epochs) {
        o.bat = Some(epoch.0.clone());
        o.bbat = Some(epoch.0.clone());
        // The strict five-column TIM contract excludes flags, deletion and jumps.
        o.deleted = Some(0);
        o.nFlags = Some(0);
        o.phaseOffset = Some(0.0);
        o.obsNjump = Some(0);
        o.obsNfdjump = Some(0);
    }
    let phases = crate::clkcorr::with_process(|state| {
        crate::residual_source::form_residuals_inner(
            &mut context,
            0,
            state.source.max_observations,
            &mut state.diagnostics,
        )
    })?;
    let (_, rows) = context.pulsar_observations_mut(0)?;
    let observations = rows
        .iter()
        .take(psr.source_observations().len())
        .zip(&psr.observation_metadata)
        .map(|(o, original)| {
            let error = |message| {
                TimingError::new(
                    &psr.tim.path,
                    Some(original.line),
                    &format!("observation {} ({})", original.index, original.name),
                    message,
                )
            };
            let phase = read(&o.phase, "phase")?;
            let residual = read(&o.residual, "residual")?;
            if !phase.is_finite() {
                return Err(error("nonfinite spin phase"));
            }
            if !residual.is_finite() {
                return Err(error("nonfinite residual"));
            }
            Ok(ToaResult {
                index: original.index,
                sat_mjd: Scalar(read(&o.sat, "sat")?.clone()),
                bat_mjd: Scalar(read(&o.bat, "bat")?.clone()),
                bbat_mjd: Scalar(read(&o.bbat, "bbat")?.clone()),
                phase_turn: Scalar(phase.clone()),
                nphase_turn: pulse_integer(read(&o.nphase, "nphase")?)
                    .ok_or_else(|| error("reduced pulse count outside signed 64-bit range"))?,
                pulse_number: *read(&o.pulseN, "pulseN")?,
                residual_s: Scalar(residual.clone()),
            })
        })
        .collect::<Result<Vec<_>, TimingError>>()?;
    Ok(TimingResult {
        schema_version: 1,
        status: "completed",
        engine: EngineProvenance {
            name: "tempo2-rust",
            version: env!("CARGO_PKG_VERSION"),
            numeric_backend: "Rug 1.30.0 / MPFR",
            significand_bits: PRECISION_BITS,
            arithmetic_rounding: "nearest, ties to even; pulse selection uses signed half then truncation",
            support_profile: "M1a isolated TCB @ F0-F3",
            source_correspondence: "TEMPO2 fda6ad4a9753ad7dc5d291406f2f310a9161423f; accepted reference 7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572",
        },
        input: psr.clone(),
        reference_phase_turn: Scalar(phases[0].clone()),
        observations,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_half_pulses_and_checked_counts() {
        for (token, expected) in [
            ("0.49999999999999999999", 0),
            ("0.5", 1),
            ("0.50000000000000000001", 1),
            ("-0.49999999999999999999", 0),
            ("-0.5", -1),
            ("-0.50000000000000000001", -1),
            ("100000000000.49999999999999999999", 100000000000),
            ("-100000000000.49999999999999999999", -100000000000),
            ("100000000000.5", 100000000001),
            ("-100000000000.5", -100000000001),
        ] {
            let value = token.parse::<Scalar>().unwrap();
            assert_eq!(
                pulse_integer(&fortran_nlong(&value.0)),
                Some(expected),
                "{token}"
            );
        }
        for token in ["9223372036854775808", "-9223372036854775809", "1e100000000"] {
            assert_eq!(pulse_integer(&token.parse::<Scalar>().unwrap().0), None);
        }
        for value in [i64::MIN, i64::MAX] {
            assert_eq!(
                pulse_integer(&Float::with_val(PRECISION_BITS, value)),
                Some(value)
            );
        }
    }
}
