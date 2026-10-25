//! Typed host text conversions for tabulatedfunction.C, clkcorr.C and observatory.C.
//! libc supplies only the source numeric grammar/rounding and fixed-format rendering.
use std::ffi::{c_char, c_double, c_int};

// Private libc ABI declarations only. The wrappers below own every buffer,
// restrict formats and argument types, and keep all pointers local to a call.
unsafe extern "C" {
    fn sscanf(input: *const c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(output: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn strtod(input: *const c_char, end: *mut *mut c_char) -> c_double;
}

pub(crate) fn float_prefix(bytes: &[u8]) -> Option<(f32, usize)> {
    let input = terminated(bytes);
    let (mut x, mut count) = (0.0f32, 0i32);
    // Owned terminated input and exactly typed, initialized %f/%n destinations.
    let n = unsafe { sscanf(input.as_ptr().cast(), c"%f%n".as_ptr(), &mut x, &mut count) };
    (n == 1).then_some((x, count as usize))
}

/// strtoflt128 and strtod share prefix grammar; only the end pointer is used.
pub(crate) fn extended_prefix_len(bytes: &[u8]) -> usize {
    let input = terminated(bytes);
    let start = input.as_ptr().cast::<c_char>();
    let mut end = std::ptr::null_mut();
    // libc points end into this owned NUL-terminated allocation, including the
    // start on failed conversion. No pointer survives the call or escapes.
    unsafe {
        strtod(start, &mut end);
        end.offset_from(start) as usize
    }
}

pub(crate) fn c_bytes(bytes: &[u8]) -> &[u8] {
    &bytes[..bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len())]
}
fn terminated(bytes: &[u8]) -> Vec<u8> {
    let mut result = c_bytes(bytes).to_vec();
    result.push(0);
    result
}
pub(crate) fn whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | b'\n' | 11 | 12)
}
pub(crate) fn tokens(bytes: &[u8]) -> impl Iterator<Item = &[u8]> {
    c_bytes(bytes)
        .split(|b| whitespace(*b))
        .filter(|s| !s.is_empty())
}
pub(crate) fn pair(bytes: &[u8]) -> Option<[f64; 2]> {
    let input = terminated(bytes);
    let (mut x, mut y): (c_double, c_double) = (0.0, 0.0);
    // Required for the host's numeric-prefix/hex/nonfinite grammar. Owned input is
    // NUL terminated; the literal format has exactly two aligned, initialized f64
    // destinations, valid exclusively for this call. No pointer escapes; only two
    // completed conversions publish output. libc's locale is never changed here.
    let n = unsafe { sscanf(input.as_ptr().cast(), c"%lf %lf".as_ptr(), &mut x, &mut y) };
    (n == 2).then_some([x, y])
}
pub(crate) fn number_prefix(bytes: &[u8]) -> Option<(f64, usize)> {
    let input = terminated(bytes);
    let (mut x, mut count): (c_double, c_int) = (0.0, 0);
    // Same owned-input/typed-output invariant as pair; %n writes an initialized
    // c_int, and is consumed only when the preceding %lf conversion completed.
    let n = unsafe { sscanf(input.as_ptr().cast(), c"%lf%n".as_ptr(), &mut x, &mut count) };
    (n == 1).then_some((x, count as usize))
}
pub(crate) fn header(bytes: &[u8]) -> Option<(Vec<u8>, Vec<u8>, f32)> {
    if bytes.first() != Some(&b'#') {
        return None;
    }
    let input = terminated(&bytes[1..]);
    let (mut from, mut to) = ([0u8; 64], [0u8; 64]);
    let mut weight: f32 = 1.0;
    // The source's %63s widths fit the 64-byte initialized arrays including NUL.
    // %f receives f32 (not promoted f64: these are destination pointers). All
    // pointers are aligned, uniquely borrowed, live for the call and cannot escape.
    // A failed third conversion keeps the source default, not a guessed weight.
    let n = unsafe {
        sscanf(
            input.as_ptr().cast(),
            c"%63s %63s %f".as_ptr(),
            from.as_mut_ptr().cast::<c_char>(),
            to.as_mut_ptr().cast::<c_char>(),
            &mut weight,
        )
    };
    (n >= 2).then(|| {
        (
            c_bytes(&from).to_vec(),
            c_bytes(&to).to_vec(),
            if n == 3 { weight } else { 1.0 },
        )
    })
}
#[derive(Clone, Copy)]
pub(crate) enum NumberFormat {
    Three,
    Nine,
    ScientificSix,
    One,
    Two,
    Four,
    Six,
    GeneralThree,
    GeneralSix,
    GeneralTen,
    GeneralFive,
    Five,
    Eight,
    Twelve,
    Fourteen,
    ScientificThree,
    WidthEightThree,
    WidthSevenTwo,
}
pub(crate) fn format_number(x: f64, style: NumberFormat) -> Vec<u8> {
    let format = match style {
        NumberFormat::Three => c"%.3lf",
        NumberFormat::Nine => c"%.9lf",
        NumberFormat::ScientificSix => c"%3e",
        NumberFormat::One => c"%.1f",
        NumberFormat::Two => c"%.2f",
        NumberFormat::Four => c"%.4lf",
        NumberFormat::Six => c"%lf",
        NumberFormat::GeneralThree => c"%.3g",
        NumberFormat::GeneralSix => c"%g",
        NumberFormat::GeneralTen => c"%.10g",
        NumberFormat::GeneralFive => c"%.5g",
        NumberFormat::Five => c"%.5f",
        NumberFormat::Eight => c"%.8f",
        NumberFormat::Twelve => c"%.12f",
        NumberFormat::Fourteen => c"%.14f",
        NumberFormat::ScientificThree => c"%.3e",
        NumberFormat::WidthEightThree => c"%8.3f",
        NumberFormat::WidthSevenTwo => c"%7.2f",
    };
    let mut out = [0u8; 512];
    // Fixed formats preserve C spelling/rounding (including signed zero). A
    // binary64 fixed representation fits 512 bytes; the bounded call receives
    // its exact capacity and one promoted c_double. No pointer escapes.
    unsafe {
        snprintf(
            out.as_mut_ptr().cast(),
            out.len(),
            format.as_ptr(),
            x as c_double,
        );
    }
    c_bytes(&out).to_vec()
}

/// `%d%n` grammar without native integer-overflow behavior. The scanner advances
/// through the decimal prefix only; conversion overflow is an explicit boundary.
pub(crate) fn integer_prefix(bytes: &[u8]) -> Result<Option<(i32, usize)>, crate::TimingError> {
    let bytes = c_bytes(bytes);
    let mut end = 0;
    while end < bytes.len() && whitespace(bytes[end]) {
        end += 1;
    }
    let start = end;
    if end < bytes.len() && matches!(bytes[end], b'+' | b'-') {
        end += 1;
    }
    let digits = end;
    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }
    if end == digits {
        return Ok(None);
    }
    let text = std::str::from_utf8(&bytes[start..end]).unwrap();
    let value = text
        .parse()
        .map_err(|_| crate::clock_diagnostics::boundary("scan_integer_range"))?;
    Ok(Some((value, end)))
}

pub(crate) fn format_coordinate(a: i32, b: i32, sec: f64, sign: Option<u8>) -> Vec<u8> {
    let mut out = [0u8; 128];
    // Fixed source format and exact promoted variadic argument types. The two
    // signed i32 fields and one binary64 field fit the bounded owned buffer.
    unsafe {
        if let Some(sign) = sign {
            snprintf(
                out.as_mut_ptr().cast(),
                out.len(),
                c"%c%02d:%02d:%08.5f".as_ptr(),
                sign as c_int,
                a as c_int,
                b as c_int,
                sec as c_double,
            );
        } else {
            snprintf(
                out.as_mut_ptr().cast(),
                out.len(),
                c" %02d:%02d:%010.7f".as_ptr(),
                a as c_int,
                b as c_int,
                sec as c_double,
            );
        }
    }
    c_bytes(&out).to_vec()
}

/// ld_printf's %Lg with default precision six: round the extended value before
/// choosing fixed/scientific notation, preserving signed zeros and classes.
pub(crate) fn extended_general_six(value: &rug::Float) -> Vec<u8> {
    if !value.is_finite() || value.is_zero() {
        return format_number(value.to_f64(), NumberFormat::GeneralSix);
    }
    let (negative, mut digits, exponent) = value.to_sign_string_exp(10, Some(6));
    let exponent = exponent.expect("finite nonzero value has an exponent");
    while digits.ends_with('0') {
        digits.pop();
    }
    let mut text = if negative {
        "-".to_owned()
    } else {
        String::new()
    };
    if !(-3..=6).contains(&exponent) {
        text.push_str(&digits[..1]);
        if digits.len() > 1 {
            text.push('.');
            text.push_str(&digits[1..]);
        }
        text.push_str(&format!("e{:+03}", exponent - 1));
    } else if exponent <= 0 {
        text.push_str("0.");
        text.push_str(&"0".repeat((-exponent) as usize));
        text.push_str(&digits);
    } else {
        let whole = exponent as usize;
        if digits.len() <= whole {
            text.push_str(&digits);
            text.push_str(&"0".repeat(whole - digits.len()));
        } else {
            text.push_str(&digits[..whole]);
            text.push('.');
            text.push_str(&digits[whole..]);
        }
    }
    text.into_bytes()
}
