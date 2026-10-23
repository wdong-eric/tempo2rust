//! Typed host text conversions for tabulatedfunction.C, clkcorr.C and observatory.C.
//! libc supplies only the source numeric grammar/rounding and fixed-format rendering.
use std::ffi::{c_char, c_double, c_int};

// Private libc ABI declarations only. The wrappers below own every buffer,
// restrict formats and argument types, and keep all pointers local to a call.
unsafe extern "C" {
    fn sscanf(input: *const c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(output: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
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
    One,
    Four,
    Six,
    GeneralThree,
}
pub(crate) fn format_number(x: f64, style: NumberFormat) -> Vec<u8> {
    let format = match style {
        NumberFormat::One => c"%.1f",
        NumberFormat::Four => c"%.4lf",
        NumberFormat::Six => c"%lf",
        NumberFormat::GeneralThree => c"%.3g",
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
