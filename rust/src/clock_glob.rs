//! clkcorr.C::globClockFiles / observatory.C selected glob calls (flags = 0).
//! C-locale byte matching and traversal in safe Rust; explicit final sorting.
use std::{
    fs,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

fn character_class(name: &[u8], b: u8) -> bool {
    match name {
        b"alnum" => b.is_ascii_alphanumeric(),
        b"alpha" => b.is_ascii_alphabetic(),
        b"blank" => matches!(b, b' ' | b'\t'),
        b"cntrl" => b.is_ascii_control(),
        b"digit" => b.is_ascii_digit(),
        b"graph" => b.is_ascii_graphic(),
        b"lower" => b.is_ascii_lowercase(),
        b"print" => b.is_ascii_graphic() || b == b' ',
        b"punct" => b.is_ascii_punctuation(),
        b"space" => crate::clock_text::whitespace(b),
        b"upper" => b.is_ascii_uppercase(),
        b"xdigit" => b.is_ascii_hexdigit(),
        _ => false,
    }
}
fn bracket(pattern: &[u8], b: u8) -> Option<(bool, usize)> {
    let mut i = 1;
    let invert = matches!(pattern.get(i), Some(b'!' | b'^'));
    if invert {
        i += 1;
    }
    let first = i;
    let mut found = false;
    while i < pattern.len() {
        if pattern[i] == b']' && i > first {
            return Some((found != invert, i + 1));
        }
        if pattern.get(i..i + 2) == Some(b"[:") {
            let end = pattern[i + 2..].windows(2).position(|s| s == b":]")? + i + 2;
            found |= character_class(&pattern[i + 2..end], b);
            i = end + 2;
            continue;
        }
        // In the declared C locale, collating/equivalence symbols are bytes.
        if matches!(pattern.get(i..i + 2), Some(b"[." | b"[=")) {
            let end = pattern[i + 2..]
                .windows(2)
                .position(|s| s[0] == pattern[i + 1] && s[1] == b']')?
                + i
                + 2;
            found |= end == i + 3 && pattern[i + 2] == b;
            i = end + 2;
            continue;
        }
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
pub(crate) fn files(directory: &Path, extension: &[u8]) -> Vec<PathBuf> {
    let pattern = [directory.as_os_str().as_bytes(), b"/*.", extension].concat();
    let pattern = crate::clock_text::c_bytes(&pattern);
    let mut paths = vec![if pattern.starts_with(b"/") {
        PathBuf::from("/")
    } else {
        PathBuf::new()
    }];
    for component in pattern.split(|&b| b == b'/').filter(|c| !c.is_empty()) {
        let mut next = Vec::new();
        for prefix in paths {
            let scan = if prefix.as_os_str().is_empty() {
                Path::new(".")
            } else {
                &prefix
            };
            if component.iter().any(|b| matches!(b, b'*' | b'?' | b'[')) {
                if let Ok(entries) = fs::read_dir(scan) {
                    for entry in entries.flatten() {
                        let name = entry.file_name();
                        let bytes = name.as_bytes();
                        if bytes.starts_with(b".")
                            && !component.starts_with(b".")
                            && !component.starts_with(b"\\.")
                        {
                            continue;
                        }
                        if matches(component, bytes) {
                            next.push(prefix.join(name));
                        }
                    }
                }
            } else {
                let mut literal = Vec::new();
                let mut i = 0;
                while i < component.len() {
                    if component[i] == b'\\' {
                        i += 1;
                        if i == component.len() {
                            break;
                        }
                    }
                    literal.push(component[i]);
                    i += 1;
                }
                let path = prefix.join(std::ffi::OsStr::from_bytes(&literal));
                if fs::symlink_metadata(&path).is_ok() {
                    next.push(path);
                }
            }
        }
        paths = next;
    }
    paths.sort();
    paths
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn c_locale_glob_components() {
        for (p, n, yes) in [
            (b"*.clk".as_slice(), b"a.clk".as_slice(), true),
            (b"[[:digit:]][!x].clk", b"4y.clk", true),
            (b"[a-c]?.clk", b"b1.clk", true),
            (b"[a-c]?.clk", b"d1.clk", false),
            (b"a\\*.clk", b"a*.clk", true),
            (b"[.a.]", b"a", true),
        ] {
            assert_eq!(matches(p, n), yes);
        }
    }
}
