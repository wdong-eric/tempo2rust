//! Selected tempo2Util.C::dotproduct/hms_turn/dms_turn/turn_deg/fortran_mod/
//! fortran_nlong/copyParam. parse_binary64 is strict Rust adapter parsing; remainder and
//! integer selection use the approved Rug representation, not source ABI objects.
use crate::longdouble::normalized_decimal;
use rug::Float;

pub(crate) fn dot_product(a: [f64; 3], b: [f64; 3]) -> f64 {
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
pub(crate) fn hms_turn(hr: f64, min: f64, sec: f64) -> f64 {
    hr / 24.0 + min / 1440.0 + sec / 86400.0
}

pub(crate) fn dms_turn(deg: f64, min: f64, sec: f64, sign: f64) -> f64 {
    (deg.abs() / 360.0 + min / 21600.0 + sec / 1296000.0) * sign
}

pub(crate) fn turn_deg(turn: f64) -> f64 {
    turn * 360.0
}

pub(crate) fn fortran_mod(a: Float, p: &Float) -> Result<Float, crate::TimingError> {
    // tempo2Util.C::fortran_mod:186-191 (fda6ad4): a-(int)(a/p)*p.
    // The former % helper differed for signed zero, infinite divisors and
    // rounded quotients (0049). Preserve the expression at Rug128 and reject
    // undefined native i32 conversions; 113/128-bit boundary differences remain.
    let quotient = Float::with_val(crate::PRECISION_BITS, &a / p);
    let n = crate::ifteph::extended_i32(&quotient, "fortran_mod_quotient_int")?;
    Ok(a - Float::with_val(crate::PRECISION_BITS, p * n))
}

pub(crate) fn fortran_nlong(x: &Float) -> Float {
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
            dst.prefit = Some(
                src.prefit
                    .as_ref()
                    .ok_or_else(|| error("prefit", "unwritten_value"))?
                    .clone(),
            );
            dst.prefit_err = Some(
                src.prefit_err
                    .as_ref()
                    .ok_or_else(|| error("prefitErr", "unwritten_value"))?
                    .clone(),
            );
        }
        destination.a_size = source.a_size;
        Ok(())
    })();
    if let Err(error) = &result {
        destination.boundary = Some(error.clone());
    }
    result
}
