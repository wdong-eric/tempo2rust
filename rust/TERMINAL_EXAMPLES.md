# Native terminal examples

Use the installed main application with the same input files and runtime you
intend to analyse. Replace `PREFIX` and the runtime path with your actual paths:

```sh
export TEMPO2=/path/to/T2runtime
PREFIX/bin/tempo2-rust -f model.par arrivals.tim
PREFIX/bin/tempo2-rust --help
```

The native root command prints the familiar TEMPO2 report and fits according to
the PAR controls. `-pre` still fits; it is not a switch to evaluation-only timing.
`--help` starts with this workflow and shares the native option catalogue.
Runtime data and scientific input files are supplied separately from installation.

From this repository root, the existing robust-H example is:

```sh
export TEMPO2="$PWD/validation/release/build/integrated-v1/a2/runtime"
PREFIX/bin/tempo2-rust -npsr 1 -nobs 100 -noaccel \
  -f validation/general_fitting/inputs/robust-H.par \
     validation/general_fitting/inputs/multi-frequency.tim
```

That runtime path names the retained qualification build; use an available
equivalent runtime if it has not been built locally. Run in a separate working
directory with absolute input paths when using options that write result files.
The output contains PSR, pre/post-fit RMS, chi-square, parameter/error/fit-flag,
derived-quantity and timing-convention sections. Diagnostics name Rust emit sites,
such as `general_fitting.rs`; exact line numbers follow the installed source.
This is a runnable product example, not an independent scientific-equivalence test.

For scripts, choose an explicit machine interface instead:

```sh
PREFIX/bin/tempo2-rust timing --par model.par --tim arrivals.tim \
  --runtime "$TEMPO2"
```

This JSON command evaluates without fitting; add `--fit` only when a fit is
intended. `residuals` and `fit` have narrower input/model contracts. Predictor
commands generate or evaluate predictors without a timing fit. See the installed
README and API documents for each contract. Executables in `target/*/examples/`
are diagnostic/library clients and may deliberately emit JSON or probe records.
