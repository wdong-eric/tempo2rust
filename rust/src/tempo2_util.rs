//! Selected tempo2Util.C::dotproduct/hms_turn/dms_turn/turn_deg/fortran_mod/
//! fortran_nlong/copyParam. parse_binary64 is strict Rust adapter parsing; remainder and
//! integer selection use the approved Rug representation, not source ABI objects.
use crate::longdouble::normalized_decimal;
use rug::Float;

pub fn dot_product(a: [f64; 3], b: [f64; 3]) -> f64 {
    // tempo2Util.C::dotproduct at fda6ad4: GCC16 O2 contracts in this
    // operand order (second product rounded first). Y arithmetic.py records
    // actual assembly and 100 bounded native/Rust expression comparisons.
    a[2].mul_add(b[2], a[0].mul_add(b[0], a[1] * b[1]))
}

pub(crate) fn parse_binary64(token: &str) -> Result<f64, String> {
    let normalized = normalized_decimal(token)?;
    let value: f64 = normalized.parse().map_err(|e| format!("{token:?}: {e}"))?;
    let nonzero = normalized
        .split('e')
        .next()
        .unwrap()
        .bytes()
        .any(|c| matches!(c, b'1'..=b'9'));
    if value.is_finite() && (value != 0.0 || !nonzero) {
        Ok(value)
    } else {
        Err(format!(
            "{token:?} is outside the finite binary64 range (including underflow)"
        ))
    }
}

// The reader validates sexagesimal components before these source-order conversions.
pub fn hms_turn(hr: f64, min: f64, sec: f64) -> f64 {
    hr / 24.0 + min / 1440.0 + sec / 86400.0
}

pub fn dms_turn(deg: f64, min: f64, sec: f64, sign: f64) -> f64 {
    (deg.abs() / 360.0 + min / 21600.0 + sec / 1296000.0) * sign
}

pub fn turn_deg(turn: f64) -> f64 {
    turn * 360.0
}

pub fn fortran_mod(a: Float, p: &Float) -> Result<Float, crate::TimingError> {
    // tempo2Util.C::fortran_mod:186-191 (fda6ad4): a-(int)(a/p)*p.
    // The former % helper differed for signed zero, infinite divisors and
    // rounded quotients (0049). Preserve the expression at Rug128 and reject
    // undefined native i32 conversions; 113/128-bit boundary differences remain.
    let quotient = Float::with_val(crate::PRECISION_BITS, &a / p);
    let n = crate::ifteph::extended_i32(&quotient, "fortran_mod_quotient_int")?;
    Ok(a - Float::with_val(crate::PRECISION_BITS, p * n))
}

pub fn fortran_nlong(x: &Float) -> Float {
    // fortran_nlong: signed half addition then truncation, not ties-to-even.
    (x.clone() + if x > &0 { 0.5f64 } else { -0.5f64 }).trunc()
}

/// tempo2Util.C::copyParam: ordered field writes, with links and backing retained.
/// A failed destination is terminal for scientific use; its prefix is diagnostic.
/// Owned disjoint source/destination borrows exclude overlapping strcpy buffers.
pub fn copy_param(
    source: &crate::tempo2::SourceParameter,
    destination: &mut crate::tempo2::SourceParameter,
) -> Result<(), crate::tempo2::ParameterBoundary> {
    use crate::tempo2::ParameterBoundary;
    if let Some(error) = &destination.boundary {
        return Err(error.clone());
    }
    let result = (|| {
        if let Some(error) = &source.boundary {
            return Err(error.clone());
        }
        let count = source.a_size.ok_or(ParameterBoundary {
            cell: 0,
            field: "aSize",
            kind: "unwritten_value",
        })?;
        for index in 0..count {
            let i = index as usize;
            let error = |field, kind| ParameterBoundary {
                cell: i,
                field,
                kind,
            };
            let src = source
                .cells
                .get(i)
                .ok_or_else(|| error("label", "missing_source_cell"))?;
            let dst = destination
                .cells
                .get_mut(i)
                .ok_or_else(|| error("label", "missing_destination_cell"))?;
            for (field, src, dst) in [
                ("label", &src.label, &mut dst.label),
                ("shortlabel", &src.shortlabel, &mut dst.shortlabel),
            ] {
                let mut j = 0;
                loop {
                    let byte = *src
                        .get(j)
                        .ok_or_else(|| error(field, "missing_string_terminator"))?;
                    *dst.get_mut(j)
                        .ok_or_else(|| error(field, "insufficient_label_storage"))? = byte;
                    if byte == 0 {
                        break;
                    }
                    j += 1;
                }
            }
            dst.val = src.val.clone();
            dst.err = src.err.clone();
            dst.fit_flag = src.fit_flag;
            dst.param_set = src.param_set;
            // copyParam reads every prefit cell, including unset higher-order
            // cells left uninitialized by readParfile. AE bootstrap exposes it.
            // The corrected native copy uses memcpy for these two buffers;
            // retain knownness here without interpreting unwritten numbers.
            dst.prefit = src.prefit.clone();
            dst.prefit_err = src.prefit_err.clone();
        }
        destination.a_size = source.a_size;
        Ok(())
    })();
    if let Err(error) = &result {
        destination.boundary = Some(error.clone());
    }
    result
}

/// Linked parameters are evaluated in caller order, without recursive resolution.
pub fn get_parameter_value(
    p: &crate::pulsar::SourcePulsar,
    label: usize,
    index: usize,
) -> Result<Float, crate::TimingError> {
    crate::clkcorr::with_process(|s| get_parameter_value_inner(p, label, index, &mut s.diagnostics))
}
pub(crate) fn get_parameter_value_inner(
    p: &crate::pulsar::SourcePulsar,
    label: usize,
    index: usize,
    diagnostics: &mut crate::clock_diagnostics::Diagnostics,
) -> Result<Float, crate::TimingError> {
    use crate::{PRECISION_BITS, observation::read, preprocess::value, tempo2::Label::*};
    let par = &p.param.slots[label];
    if *read(&par.n_link_to, "nLinkTo")? > 0 {
        if par.n_link_to.unwrap() > 1 {
            diagnostics.stdout(b"Sorry, can currently only link two parameters together\n");
            return Err(crate::clock_diagnostics::boundary(
                "multiple_parameter_links",
            ));
        }
        let target = *read(&par.link_to[0], "linkTo")?;
        if label == param_daop as usize && target == param_pbdot as i32 {
            #[allow(clippy::excessive_precision)] // Pinned binary64 source constant.
            let a = value(p, param_pmra, 0) * 1.53628185004416045e-16f64;
            #[allow(clippy::excessive_precision)] // Pinned binary64 source constant.
            let b = value(p, param_pmdec, 0) * 1.53628185004416045e-16f64;
            return Ok((value(p, param_pbdot, 0) * 299792458.0f64
                / (value(p, param_pb, 0) * 86400.0f64 * (a.square() + b.square())))
                / 3.08568025e16f64);
        }
        if label == param_kin as usize && target == param_stig as i32 {
            let s = value(p, param_stig, 0);
            return Ok(
                (s.clone() * 2.0f64 / (Float::with_val(PRECISION_BITS, 1) + s.square())).asin()
                    * 180.0f64
                    / std::f64::consts::PI,
            );
        }
        if label == param_sini as usize && target == param_kin as i32 {
            return Ok((value(p, param_kin, 0) / 180.0f64 * std::f64::consts::PI).sin());
        }
        if label == param_dshk as usize && target == param_px as i32 {
            return Ok(Float::with_val(PRECISION_BITS, 1) / value(p, param_px, 0));
        }
        if label == param_dmmodel as usize && target == param_dm as i32 {
            return Ok(value(p, param_dm, 0));
        }
    }
    Ok(par
        .cells
        .get(index)
        .ok_or_else(|| crate::clock_diagnostics::boundary("parameter_cell_index"))?
        .val
        .clone())
}
