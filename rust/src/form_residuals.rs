// Ported from TEMPO2 formResiduals.C and tempo2Util.C; GPL-3.0-or-later.
// Copyright (C) 2006-2009 George Hobbs, Russell Edwards and TEMPO2 contributors.
use crate::tempo2_util::{fortran_mod, fortran_nlong};
use crate::{
    EngineProvenance, PRECISION_BITS, Scalar, TimingError, TimingInput, TimingResult, ToaResult,
};
use rug::{Float, float::Round};

fn source_i32(value: &Float) -> Option<i32> {
    let integer = value.clone().trunc();
    (i32::MIN..=i32::MAX)
        .contains(&integer)
        .then(|| integer.to_integer_round(Round::Zero)?.0.to_i32())
        .flatten()
}

fn pulse_integer(value: &Float) -> Option<i64> {
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
    if psr.observations.iter().any(|o| o.original.site != "@") {
        return Err(TimingError::new(
            &psr.tim.path,
            None,
            "site",
            "use evaluate_terrestrial with a RuntimeContext for PKS",
        ));
    }
    let epochs: Vec<_> = psr.observations.iter().map(|o| o.sat_mjd.clone()).collect();
    form_residuals_at(psr, &epochs)
}

/// Shared residual kernel; epochs are BAT=BBAT for both declared isolated paths.
pub(crate) fn form_residuals_at(
    psr: &TimingInput,
    epochs: &[Scalar],
) -> Result<TimingResult, TimingError> {
    let f0 = &psr.parameters["F0"].value.0;
    let pepoch = &psr.parameters["PEPOCH"].value.0;
    let nf0 = source_i32(f0).ok_or_else(|| {
        TimingError::new(
            &psr.par.path,
            psr.parameters["F0"].original.as_ref().map(|p| p.line),
            "F0",
            "integer part exceeds the source i32 range",
        )
    })?;
    let pepoch_day = source_i32(pepoch).ok_or_else(|| {
        TimingError::new(
            &psr.par.path,
            psr.parameters["PEPOCH"].original.as_ref().map(|p| p.line),
            "PEPOCH",
            "integer MJD exceeds the source i32 range",
        )
    })?;
    let ff0 = f0.clone() - nf0;
    let mut phase_terms = Vec::with_capacity(psr.observations.len());
    for (o, epoch) in psr.observations.iter().zip(epochs) {
        let error = |message| {
            TimingError::new(
                &psr.tim.path,
                Some(o.original.line),
                &format!("observation {} ({})", o.original.index, o.original.name),
                message,
            )
        };
        let bbat = &epoch.0;
        let bbat_day =
            source_i32(bbat).ok_or_else(|| error("integer MJD exceeds the source i32 range"))?;
        let ntpd = bbat_day
            .checked_sub(pepoch_day)
            .ok_or_else(|| error("integer day difference exceeds the source i32 range"))?;
        // Reject the source's undefined signed-int product instead of wrapping it.
        let integral_product = nf0
            .checked_mul(ntpd)
            .ok_or_else(|| error("integer F0 * day difference exceeds the source i32 range"))?;
        let phaseint = Float::with_val(PRECISION_BITS, f64::from(integral_product) * 86400.0);
        let ftpd = (bbat.clone() - bbat_day) - (pepoch.clone() - pepoch_day);
        let delta_t = (bbat.clone() - pepoch) * 86400i32;
        // formResiduals.C:515-535, retaining every multiplication/addition boundary.
        // Complete the third product explicitly: adding Rug's borrowed product would fuse it.
        let phase2 = (Float::with_val(PRECISION_BITS, nf0) * &ftpd
            + Float::with_val(PRECISION_BITS, ntpd) * &ff0
            + Float::with_val(PRECISION_BITS, &ftpd * &ff0))
            * 86400i32;
        let mut arg = Float::with_val(PRECISION_BITS, &delta_t * &delta_t);
        let mut phase3 =
            (Float::with_val(PRECISION_BITS, 0.5) * &psr.parameters["F1"].value.0) * &arg;
        for (key, divisor) in [("F2", 6i32), ("F3", 24i32)] {
            arg *= &delta_t;
            if psr.parameters[key].set {
                phase3 += (psr.parameters[key].value.0.clone() / divisor) * &arg;
            }
        }
        let phase5 = phase2 + phase3;
        if !phase5.is_finite() {
            return Err(error("nonfinite spin phase"));
        }
        phase_terms.push((phaseint, phase5));
    }
    // Rug % uses a truncating quotient, matching the adopted fortran_mod semantics.
    let phas1 = fortran_mod(phase_terms[0].1.clone(), 1.0f64);
    let mut observations = Vec::with_capacity(phase_terms.len());
    for ((o, epoch), (phaseint, mut phase5)) in psr.observations.iter().zip(epochs).zip(phase_terms)
    {
        let error = |message| {
            TimingError::new(
                &psr.tim.path,
                Some(o.original.line),
                &format!("observation {} ({})", o.original.index, o.original.name),
                message,
            )
        };
        phase5 -= &phas1;
        let nphase = fortran_nlong(&phase5);
        let nphase_turn = pulse_integer(&nphase)
            .ok_or_else(|| error("reduced pulse count outside signed 64-bit range"))?;
        let pulse_n = pulse_integer(&(phaseint.clone() + &nphase))
            .ok_or_else(|| error("full pulse count outside signed 64-bit range"))?;
        let residual = (phase5.clone() - nphase) / f0;
        if !residual.is_finite() {
            return Err(error("nonfinite residual"));
        }
        observations.push(ToaResult {
            index: o.original.index,
            sat_mjd: o.sat_mjd.clone(),
            bat_mjd: epoch.clone(),
            bbat_mjd: epoch.clone(),
            phase_turn: Scalar(phaseint + phase5),
            nphase_turn,
            pulse_number: pulse_n,
            residual_s: Scalar(residual),
        });
    }
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
        reference_phase_turn: Scalar(phas1),
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
