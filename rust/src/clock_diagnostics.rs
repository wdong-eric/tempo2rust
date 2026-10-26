//! tempo2Util.C::displayMsg, selected TKlog.C/h and global.C::CVSdisplayVersion.
//! Byte events are a library presentation adapter; state belongs to the process session.
use crate::{TimingError, clock_text::c_bytes};
#[cfg(target_os = "macos")]
type ClockTicks = std::ffi::c_ulong;
#[cfg(not(target_os = "macos"))]
type ClockTicks = std::ffi::c_long;

#[derive(Default)]
pub struct Diagnostics {
    /// Terminal state of configured LOG_OUTFILE, independently of destination.
    pub log_outfile_is_terminal: bool,
    pub debug: bool,
    pub quiet: bool,
    pub display_version: bool,
    pub tcheck: i32,
    pub timer_clock: ClockTicks,
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
    pub fn timing(&mut self, file: &str, line: usize, body: &[u8]) {
        if self.tcheck == 0 {
            return;
        }
        unsafe extern "C" {
            fn clock() -> ClockTicks;
        }
        // POSIX clock() has no pointer arguments and returns process CPU ticks;
        // CLOCKS_PER_SEC is 1,000,000 on the supported POSIX hosts. TKlog.h uses
        // float division, then promotes that result for the printf conversion.
        let seconds = unsafe { clock() }.wrapping_sub(self.timer_clock) as f32 / 1_000_000.0f32;
        self.log(
            b't',
            file,
            line,
            &[
                b"T=".as_slice(),
                &crate::clock_text::format_number(
                    seconds as f64,
                    crate::clock_text::NumberFormat::Two,
                ),
                b" s: ",
                body,
            ]
            .concat(),
        );
    }
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
                let value = body[..body.len().min(359)].to_vec();
                if (*counter as usize) < stored.len() {
                    stored[*counter as usize] = value;
                } else {
                    stored.push(value);
                }
            }
            *counter = counter.wrapping_add(1);
        }
        let location = [
            self.source_directory.as_slice(),
            file.as_bytes(),
            format!(":{line}").as_bytes(),
        ]
        .concat();
        let rendered = match kind {
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
        self.check_log(false, &rendered);
    }
    /// TKlog.C::_TKchklog: truncate first, then strip ANSI only for redirected LOG_OUTFILE.
    pub fn check_log(&mut self, stderr: bool, body: &[u8]) {
        let body = &c_bytes(body)[..c_bytes(body).len().min(359)];
        let mut bytes = Vec::new();
        let mut code = false;
        for &b in body {
            if !self.log_outfile_is_terminal {
                if b == 27 {
                    code = true;
                }
                if !code {
                    bytes.push(b);
                }
                if b == b'm' {
                    code = false;
                }
            } else {
                bytes.push(b);
            }
        }
        self.events.push((stderr, bytes));
    }
    /// TKlog.C::logerr_check: warnings precede errors; reset counts, retain backing.
    pub fn check(&mut self) -> i32 {
        let count = self.errors;
        self.errors = 0;
        if self.warnings > 0 {
            self.check_log(false, format!("\x1b[0m\x1b[0;35mNotice:\x1b[0m There were {} warnings. Summaries are shown below, check logs for full details.\n", self.warnings).as_bytes());
            for i in 0..self.warnings.min(30) {
                let body = [
                    format!("\x1b[0m\x1b[1mWarning #{}:\x1b[0m ", i + 1).as_bytes(),
                    &self.warning_log[i as usize],
                    b"\n",
                ]
                .concat();
                self.check_log(false, &body);
            }
            if self.warnings > 30 {
                self.check_log(
                    false,
                    format!(
                        "\x1b[0m\x1b[0;35m   ... plus {} warnings not shown\n\x1b[0m",
                        self.warnings - 30
                    )
                    .as_bytes(),
                );
            }
            self.warnings = 0;
        }
        if count > 0 {
            self.check_log(false,format!("\n\n\x1b[0m\x1b[1;31mNotice:\x1b[0m There were {} errors! Summaries are shown below, check logs for full details.\n",count).as_bytes());
            for i in 0..count.min(30) {
                let body = [
                    format!("\x1b[0m\x1b[1;31mError #{}\x1b[0m = \x1b[0m\x1b[1m", i + 1).as_bytes(),
                    &self.error_log[i as usize],
                    b"\x1b[0m\n",
                ]
                .concat();
                self.check_log(false, &body);
            }
            if count > 30 {
                self.check_log(
                    false,
                    format!(
                        "\x1b[0m\x1b[1;31m  ... plus {} errors not shown\n\x1b[0m",
                        count - 30
                    )
                    .as_bytes(),
                );
            }
        }
        count as i32
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

/// TKlog/tempo2.C process CPU clock, shared by timing messages and final elapsed time.
pub(crate) fn cpu_clock() -> ClockTicks {
    unsafe extern "C" {
        fn clock() -> ClockTicks;
    }
    // No arguments, pointers or ownership; POSIX clock_t matches the host typedef.
    unsafe { clock() }
}
