# AH core CLI and output contract

Baseline 732fd0ea30de9b22f566065b26dd7642f536d7ea; source fda6ad4a9753ad7dc5d291406f2f310a9161423f.

Native root commands coexist with explicit `bats`, `timing`, `residuals`, `fit`
and `predictor` JSON subcommands. Native options retain source case, order,
preprocessing controls and defaults. `-h`, `-H`, `-version`, `-v` and no arguments
retain source status 1. Identity explicitly names Rust and the native source pin.
Plugins are discovered but unavailable for execution; selection is an error.
The native route leads `--help`, README and installed examples. `core_cli::help()`
shares the pinned native option catalogue with short help, states the plugin
execution boundary, and corrects its obsolete CVS-version description (0106/0107).

`core_output::text_output(context, options, io)` accepts canonical mutable
PulsarContext, explicit global/output controls, and OutputIo owning borrowed
stdout, stdin and a destination directory. It never changes cwd. Files are
created at their source point with source names; previous successful writes and
state changes survive errors. Empty requested PAR name reads one token from the
supplied input stream. Direct helper APIs expose calc_rms (returns sum of weights),
m2 (including diagnostic and -1 sentinel), dglep, print_glitch, display_parameters
and the actual record_precision no-op. State remains the canonical owner.

`source_format` provides typed byte formatting and zero-success ld_fprintf,
ld_sprintf and ld_vsprintf equivalents. Rug values never pass through binary64
unless the pinned caller explicitly casts. Source parsing is separate from strict
Scalar parsing. TKlog extends existing Diagnostics, with explicit LOG_OUTFILE
terminal configuration, first-30 storage and counter/summary resets.

Native orchestration reads once, preprocesses once, immediately writes out.tim
when requested, and performs BAT/residual, one t2_fit, BAT/residual, text_output
per outer NITS pass. `-pre` omits the second evaluation only. Predictor routes
use canonical prepared PAR state and AF construction directly, retaining separate
CLI defaults. `-model`, old/newfit and dcm retain actual pinned consumers/no-ops.

## Runnable commands

From the repository root (write outputs in a disposable directory):

```sh
cargo build --manifest-path rust/Cargo.toml --release
export TEMPO2=/Users/Eric.Dong/Downloads/tempo2rust/validation/reference/build/source/T2runtime
mkdir -p validation/core_output/build/example
cd validation/core_output/build/example
/Users/Eric.Dong/Downloads/tempo2rust/rust/target/release/tempo2-rust -npsr 2 -nobs 100 -noaccel -f /Users/Eric.Dong/Downloads/tempo2rust/validation/general_fitting/inputs/robust-H.par /Users/Eric.Dong/Downloads/tempo2rust/validation/general_fitting/inputs/multi-frequency.tim -newpar -residuals
/Users/Eric.Dong/Downloads/tempo2rust/rust/target/release/tempo2-rust -f /Users/Eric.Dong/Downloads/tempo2rust/validation/predictors/fixtures/defaults.par -pred '@ 58000 58000.01 1000 2000 7 3 3600'
/Users/Eric.Dong/Downloads/tempo2rust/rust/target/release/tempo2-rust -f /Users/Eric.Dong/Downloads/tempo2rust/validation/predictors/fixtures/isolated.par -polyco '58000 58000.01 60 8 12 @ 1400' -polyco_file literal-
```

The predictor commands use native defaults, adjust odd Cheby dimensions and
write source filenames. Printed/raw coefficient mismatches remain failing AH
comparisons; successful execution is not numerical acceptance.

The direct command adapter is
`core_cli::run(argv_including_program, &Environment, &mut OutputIo) -> Result<i32, TimingError>`.
`Environment::from_process()` snapshots runtime/test-runtime, clock/plugin paths,
machine and stdout terminal status. A direct `-reminder` caller supplies
`reminder_date`; the executable obtains the source calendar text outside the
library. `get_inputs`, `set_plug_path`, `printplugs` are also public.
`OutputIo` borrows stdout, stderr, BufRead stdin and a destination directory.

`source_format::ld_printf` takes an explicit stdout writer just like ld_fprintf.
All successful ld_* calls return zero. Typed arguments remove undefined varargs;
unsupported/mismatched conversions fail. C integer widths/padding and L f/e/g
are explicit. The native single-l integer vararg mismatch is not a golden ABI.
`Diagnostics` is publicly reexported and can be owned independently; canonical
engine operations continue using the existing synchronized process session.

Under 0107, Rust emitters use `rust_log(kind, body)` and `rust_timing(body)`.
Compiler-derived basename/line metadata survives wrappers, observation transcripts
and draining. `log(kind, file, line, body)` and `timing(file, line, body)` remain
explicit caller-metadata APIs: their file/line must identify the actual emitter.
`source_directory` applies only to those explicit calls, never to Rust captures.
No user filename or native reference inside a message body is rewritten.

`version(native_file, native_function)` now clearly labels its arguments as the
TEMPO2 counterpart. It reports the Rust build/source pin and actual Rust call
site, while retaining the counterpart pair as its existing once-only key. This
preserves suppression when a source routine has more than one Rust owner.
The existing 359-byte full-line limit still precedes ANSI removal. A longer Rust
prefix can therefore shorten the visible body; warning/error backing storage,
counters, severity and routing retain their previous contracts.

Modern terrestrial output now defines site-TOA-to-UT1 seconds at the existing
clock/EOP producer, including modes that bypass clock correction. This also
makes the existing JSON `ut1` field initialized for that path; its schema and
other quantities are unchanged. Barycentric/satellite observations without the
producer retain an explicit unwritten boundary. Full source/types/unsafe and
unexercised branch dispositions are in CORE_OUTPUT_SOURCE_MAP.md.
