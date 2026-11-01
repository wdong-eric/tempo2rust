# TEMPO2 Rust

`tempo2-rust` brings TEMPO2-style timing commands and reports to an owned Rust
core. It includes general PAR/TIM loading, barycentric timing, binary models,
local/global fitting, numerical and analysis APIs, and predictor generation and
evaluation. External consumers use the Rust library or the standalone predictor
C interface.

**Release qualification is incomplete.** The first release targets macOS arm64
and Linux x86_64. Implemented capabilities and scoped passing checks do not yet
establish full scientific or platform acceptance; AL, M2 and inherited numerical
gates remain open. See the [qualification report](rust/RELEASE_QUALIFICATION_REPORT.md)
and [central status](HANDOFF.md) for the evidence and current disposition.

## Start with a familiar timing report

Follow the [migration guide](docs/MIGRATION.md) for prerequisites, building,
private-prefix installation and runtime setup. After installation, use your
chosen runtime and input files in a separate working directory:

```sh
export TEMPO2=/absolute/path/to/T2runtime
/path/to/private-prefix/bin/tempo2-rust -f model.par arrivals.tim
/path/to/private-prefix/bin/tempo2-rust --help
```

Replace the example paths with your own. The `-f` workflow fits according to PAR
controls and prints the familiar PSR, RMS, chi-square, parameter/error/fit-flag
and timing-convention report. **`-pre` still fits.** Fixed-parameter evaluation is
the explicit `timing` JSON command without `--fit`; see
[choosing a workflow](docs/MIGRATION.md#choose-the-calculation-and-output).
Rust diagnostics name actual Rust files and call sites.

Runtime clocks, ephemerides, observatory and other required data are supplied
separately. Keep the resource versions and scientific conventions appropriate to
your analysis. The installer uses distinct Rust product names and requires a
fresh prefix. General legacy `pulsar` C/C++ ABI compatibility and TEMPO2
plugin/GUI execution are outside the interface promise.

## Documentation

| Task | Start here |
| --- | --- |
| Move an existing TEMPO2 workflow | [Migration guide](docs/MIGRATION.md) |
| Run the main application and interpret output | [Terminal examples](rust/TERMINAL_EXAMPLES.md), [core CLI/output contract](rust/CORE_OUTPUT_API.md) |
| Use the Rust library or predictor C libraries | [Library interfaces](rust/LIBRARY_INTERFACES_API.md) |
| Generate, exchange or evaluate predictors | [Predictor API](rust/PREDICTOR_API.md) |
| Select CALCEPH, BLAS/LAPACK or QR | [Native backends](rust/NATIVE_BACKENDS_API.md) |
| Understand worker/process-state limits and measured costs | [Execution contract](rust/PARALLEL_EXECUTION_API.md), [measurements](rust/PARALLEL_EXECUTION_REPORT.md) |
| Inspect build/qualification evidence | [Release recipes](validation/release/README.md), [migration documentation checks](docs/MIGRATION_REPORT.md) |

The Cargo package is in [`rust/`](rust/README.md). The port derives from TEMPO2
source `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. See the
[project license](rust/LICENSE) and the retained [native dependency notices](rust/native/README.md),
including [SOFA's license](rust/native/sofa/copyr.lis).
