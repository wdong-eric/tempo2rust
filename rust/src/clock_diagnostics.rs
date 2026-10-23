//! tempo2Util.C::displayMsg, selected TKlog.C/h and global.C::CVSdisplayVersion.
//! Byte events are a library presentation adapter; state belongs to the process session.
use crate::{TimingError, clock_text::c_bytes};

#[derive(Default)]
pub(crate) struct Diagnostics {
    pub debug: bool,
    pub quiet: bool,
    pub display_version: bool,
    pub source_directory: Vec<u8>,
    pub events: Vec<(bool, Vec<u8>)>, // false stdout, true stderr
    pub warnings: u32,
    pub errors: u32,
    pub warning_log: Vec<Vec<u8>>,
    pub error_log: Vec<Vec<u8>>,
    pub keys: Vec<Vec<u8>>,
    pub warned_multiple: bool,
    versions: Vec<(&'static str, &'static str)>,
}
impl Diagnostics {
    pub fn stdout(&mut self, bytes: &[u8]) {
        self.events.push((false, c_bytes(bytes).to_vec()));
    }
    pub fn stderr(&mut self, bytes: &[u8]) {
        self.events.push((true, c_bytes(bytes).to_vec()));
    }
    pub fn log(&mut self, kind: u8, file: &str, line: usize, body: &[u8]) {
        if (kind == b'd' && !self.debug) || (kind == b'm' && self.quiet) {
            return;
        }
        let body = c_bytes(body);
        let (counter, stored) = match kind {
            b'w' => (Some(&mut self.warnings), &mut self.warning_log),
            b'e' => (Some(&mut self.errors), &mut self.error_log),
            _ => (None, &mut self.error_log),
        };
        if let Some(counter) = counter {
            if *counter < 30 {
                stored.push(body[..body.len().min(359)].to_vec());
            }
            *counter = counter.wrapping_add(1);
        }
        let location = [
            self.source_directory.as_slice(),
            file.as_bytes(),
            format!(":{line}").as_bytes(),
        ]
        .concat();
        let mut rendered = match kind {
            b'e' => [
                b"\x1b[0m\x1b[1;31m[".as_slice(),
                &location,
                b"] ERROR:\x1b[0m",
                body,
                b"\n",
            ]
            .concat(),
            b'w' => [
                b"\x1b[0m\x1b[1m[".as_slice(),
                &location,
                b"] \x1b[0m\x1b[0;35mWarning: \x1b[0m",
                body,
                b"\n",
            ]
            .concat(),
            _ => [b"[".as_slice(), &location, b"] ", body, b"\n"].concat(),
        };
        rendered.truncate(359); // vsnprintf before non-TTY ANSI removal
        let mut in_code = false;
        let mut stripped = Vec::new();
        for b in rendered {
            if b == 27 {
                in_code = true;
            }
            if !in_code {
                stripped.push(b);
            }
            if b == b'm' {
                in_code = false;
            }
        }
        self.stdout(&stripped);
    }
    pub fn display(
        &mut self,
        kind: i32,
        key: &[u8],
        search: &[u8],
        variable: &[u8],
        no_warnings: i32,
    ) -> Result<(), TimingError> {
        if kind == 1 || kind == 3 {
            if no_warnings == 2 {
                return Ok(());
            }
            if no_warnings < 0 {
                self.stdout(&[b"[", key, b"] * ", search, b" ", variable, b"\n"].concat());
            } else if self.keys.iter().any(|k| k.eq_ignore_ascii_case(key)) {
                if !self.warned_multiple {
                    self.log(
                        b'w',
                        "tempo2Util.C",
                        401,
                        b"[DUP1] duplicated warnings have been suppressed.",
                    );
                    self.warned_multiple = true;
                }
            } else {
                if self.keys.len() == 50 {
                    self.log(b'e', "tempo2Util.C", 390, b"Too many warnings!");
                    return Ok(());
                }
                if self.keys.len() > 50 {
                    return Ok(());
                }
                if c_bytes(key).len() >= 10 {
                    return Err(boundary("diagnostic_key_storage"));
                }
                self.keys.push(c_bytes(key).to_vec());
                let body = [b"[", key, b"] ", search, b" ", variable].concat();
                self.log(
                    if kind == 1 { b'w' } else { b'm' },
                    "tempo2Util.C",
                    if kind == 1 { 396 } else { 397 },
                    &body,
                );
            }
        } else {
            self.log(
                b'e',
                "tempo2Util.C",
                411,
                &[b"[", key, b"]: ", search, b" ", variable].concat(),
            );
        }
        Ok(())
    }
    pub fn version(&mut self, file: &'static str, function: &'static str) {
        if !self.display_version || self.versions.contains(&(file, function)) {
            return;
        }
        if self.versions.is_empty() {
            self.stdout(b"[TEMPO2 VERSION:] [tempo2.h            ] [                    ] [2026.09.1           ]\n");
        }
        self.stdout(
            format!(
                "[TEMPO2 VERSION:] [{file:<20.20}] [{function:<20.20}] [$Id$                ]\n"
            )
            .as_bytes(),
        );
        self.versions.push((file, function));
    }
}
pub(crate) fn boundary(name: &str) -> TimingError {
    TimingError::new(
        "",
        None,
        name,
        "source operation would access invalid or unwritten storage",
    )
}
pub(crate) fn terminal(path: &std::path::Path, cause: &str) -> TimingError {
    TimingError::new(path, None, "source_exit", cause)
}

/// Source-channel bytes captured by the clock/selected dependency library sink.
/// Reading these events does not reset any source diagnostic state.
#[derive(Clone, Debug)]
pub struct ClockDiagnostic {
    pub source_stderr: bool,
    pub bytes: Vec<u8>,
}
