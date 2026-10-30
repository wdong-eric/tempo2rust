//! 0107 diagnostic-only installed-library client; not the application CLI.
use std::{fs, path::PathBuf};
use tempo2_rust::Diagnostics;

fn main() {
    let out = PathBuf::from(std::env::args_os().nth(1).expect("output directory"));
    fs::create_dir(&out).unwrap();
    let mut rows = String::from(
        "role\tterminal\tbody_bytes\trendered_bytes\tvisible_body_bytes\tnewline\twarnings\tstored_bytes\trust_line\n",
    );
    for terminal in [false, true] {
        for size in (280..=320).chain([359, 360, 400]) {
            for role in ["old-prefix-control", "rust"] {
                let mut d = Diagnostics::default();
                d.log_outfile_is_terminal = terminal;
                let body = vec![b'x'; size];
                let mut line = 0;
                if role == "rust" {
                    line = line!() + 1;
                    d.rust_log(b'w', &body);
                } else {
                    // Synthetic pre-0107 prefix, not a genuine native execution.
                    d.log(b'w', "TKfit.C", 345, &body);
                }
                let bytes = &d.events[0].1;
                assert_eq!(d.warnings, 1);
                assert_eq!(d.warning_log[0], body[..size.min(359)]);
                let visible = bytes.iter().filter(|&&b| b == b'x').count();
                rows += &format!(
                    "{role}\t{terminal}\t{size}\t{}\t{visible}\t{}\t{}\t{}\t{line}\n",
                    bytes.len(),
                    bytes.ends_with(b"\n"),
                    d.warnings,
                    d.warning_log[0].len()
                );
                fs::write(out.join(format!("{role}-{terminal}-{size}.txt")), bytes).unwrap();
            }
        }
    }
    fs::write(out.join("boundaries.tsv"), rows).unwrap();
}
