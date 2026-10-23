//! clkcorr.C::globClockFiles / observatory.C selected glob calls (flags = 0).
//! Pinned Darwin flags-zero byte grammar; safe traversal and full-string sorting.
use std::{
    fs,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

fn bracket(pattern: &[u8], b: u8) -> Option<(bool, usize)> {
    let mut i = 1;
    let invert = pattern.get(i) == Some(&b'!');
    if invert {
        i += 1;
    }
    let first = i;
    let mut found = false;
    while i < pattern.len() {
        if pattern[i] == b']' && i > first {
            return Some((found != invert, i + 1));
        }
        // The pinned Darwin glob treats nested [: :], [. .] and [= =]
        // syntax as ordinary bracket bytes; only ! negates a set.
        let lo = if pattern[i] == b'\\' {
            i += 1;
            *pattern.get(i)?
        } else {
            pattern[i]
        };
        i += 1;
        if pattern.get(i) == Some(&b'-') && pattern.get(i + 1).is_some_and(|x| *x != b']') {
            i += 1;
            let hi = if pattern[i] == b'\\' {
                i += 1;
                *pattern.get(i)?
            } else {
                pattern[i]
            };
            i += 1;
            found |= lo <= b && b <= hi;
        } else {
            found |= lo == b;
        }
    }
    None
}
fn matches(pattern: &[u8], name: &[u8]) -> bool {
    if pattern.is_empty() {
        return name.is_empty();
    }
    match pattern[0] {
        b'*' => {
            let rest = &pattern[pattern.iter().take_while(|&&b| b == b'*').count()..];
            (0..=name.len()).any(|i| matches(rest, &name[i..]))
        }
        b'?' => !name.is_empty() && matches(&pattern[1..], &name[1..]),
        b'[' if !name.is_empty() => {
            if let Some((yes, end)) = bracket(pattern, name[0]) {
                yes && matches(&pattern[end..], &name[1..])
            } else {
                name[0] == b'[' && matches(&pattern[1..], &name[1..])
            }
        }
        b'\\' => {
            pattern.len() > 1
                && !name.is_empty()
                && pattern[1] == name[0]
                && matches(&pattern[2..], &name[1..])
        }
        b => !name.is_empty() && b == name[0] && matches(&pattern[1..], &name[1..]),
    }
}
pub(crate) fn files(pattern: &[u8]) -> Vec<PathBuf> {
    let pattern = crate::clock_text::c_bytes(pattern);
    let mut paths = vec![Vec::<u8>::new()];
    // Keep separators and dot components as bytes. Path joining/comparison
    // normalizes them, changing source diagnostics and equal-cost edge order.
    for (part, component) in pattern.split(|&b| b == b'/').enumerate() {
        let mut next = Vec::new();
        for mut prefix in paths {
            if part > 0 {
                prefix.push(b'/');
            }
            if component.is_empty() {
                next.push(prefix);
                continue;
            }
            if component.iter().any(|b| matches!(b, b'*' | b'?' | b'[')) {
                let scan = if prefix.is_empty() {
                    b"."
                } else {
                    prefix.as_slice()
                };
                if let Ok(entries) = fs::read_dir(Path::new(std::ffi::OsStr::from_bytes(scan))) {
                    // readdir supplies these two entries to native glob; Rust's
                    // read_dir intentionally omits them.
                    let names = [b".".to_vec(), b"..".to_vec()]
                        .into_iter()
                        .chain(entries.flatten().map(|e| e.file_name().as_bytes().to_vec()));
                    for name in names {
                        if name.starts_with(b".")
                            && !component.starts_with(b".")
                            && !component.starts_with(b"\\.")
                        {
                            continue;
                        }
                        if matches(component, &name) {
                            next.push([prefix.as_slice(), &name].concat());
                        }
                    }
                }
            } else {
                let mut i = 0;
                while i < component.len() {
                    if component[i] == b'\\' {
                        i += 1;
                        if i == component.len() {
                            break;
                        }
                    }
                    prefix.push(component[i]);
                    i += 1;
                }
                if fs::symlink_metadata(Path::new(std::ffi::OsStr::from_bytes(&prefix))).is_ok() {
                    next.push(prefix);
                }
            }
        }
        paths = next;
    }
    paths.sort(); // std::string byte order, not PathBuf component order
    paths
        .into_iter()
        .map(|p| PathBuf::from(std::ffi::OsStr::from_bytes(&p)))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn c_locale_glob_components() {
        for (p, n, yes) in [
            (b"*.clk".as_slice(), b"a.clk".as_slice(), true),
            (b"[[:digit:]][!x].clk", b"4y.clk", false),
            (b"[[:digit:]][!x].clk", b"d]y.clk", true),
            (b"[^x]", b"x", true),
            (b"[^x]", b"a", false),
            (b"[a-c]?.clk", b"b1.clk", true),
            (b"[a-c]?.clk", b"d1.clk", false),
            (b"a\\*.clk", b"a*.clk", true),
            (b"[.a.]", b"a", true),
        ] {
            assert_eq!(matches(p, n), yes);
        }
    }
}
