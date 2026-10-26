# AH core CLI and output contract

Baseline 732fd0ea30de9b22f566065b26dd7642f536d7ea; source fda6ad4a9753ad7dc5d291406f2f310a9161423f.

Native root commands coexist with explicit `bats`, `timing`, `residuals`, `fit`
and `predictor` JSON subcommands. Native options retain source case, order,
preprocessing controls and defaults. `-h`, `-H`, `-version`, `-v` and no arguments
retain source status 1. Identity explicitly names Rust and the native source pin.
Plugins are discovered but unavailable for execution; selection is an error.

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
