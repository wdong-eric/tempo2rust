# Moving a TEMPO2 workflow to Rust

Start with the main `tempo2-rust` executable and your existing PAR/TIM inputs.
The native-style `-f` route produces a human-readable fitting report. Choose an
explicit JSON command when a script needs structured results or when you want
fixed-parameter evaluation.

The code delivers core timing/fitting/toolkit and predictor interfaces, but full
migration acceptance is pending on macOS arm64 and Linux x86_64. Check the
[qualification report](../rust/RELEASE_QUALIFICATION_REPORT.md) and
[central status](../HANDOFF.md) before relying on a capability's scientific or
platform qualification. This guide's demonstrations are operational examples.

## Build or install

Run build commands from the repository root. The recorded release toolchain is
Rust/Cargo **1.97.1**, with Rust edition 2024; no lower minimum is established here.
You need a C compiler, `ar`, GFortran for the retained SOFA sources, and Python 3
for the installer. Bundled GMP/MPFR also needs the usual native build tools:
macOS command-line developer tools, or Linux `diffutils`, `make`, `m4` and a C
compiler. Use checkout/build/prefix paths without spaces, as required by the
pinned GMP/MPFR build. A TEMPO2 executable is unnecessary for a product build.

Select compilers actually installed on your machine. The existing profiles use:

| Host | Compiler selection before building |
| --- | --- |
| macOS arm64 | `export FC=gfortran-16 CC=clang` |
| Linux x86_64 | `export FC=gfortran-13 CC=clang` |

`build.rs` defaults to `gfortran-16` and `clang`; `FC` and `CC` override them.
These are reproduction settings, not a claim that the Linux release has passed.
Fetch the locked Cargo dependencies once; subsequent commands use that cache:

```sh
cargo fetch --locked --manifest-path rust/Cargo.toml
```

For the CLI, predictor C libraries/headers, client and complete Rust source
package, install into a **new, nonexistent** private prefix:

```sh
T2R_PREFIX="$HOME/.local/opt/tempo2-rust-trial"
python3 rust/install.py --prefix "$T2R_PREFIX"
T2R_BIN="$T2R_PREFIX/bin/tempo2-rust"
"$T2R_BIN" --help
```

The installer defaults to release, builds offline with the lockfile, and refuses
an existing prefix. Choose another name for a later installation. It records
commands, file hashes and link requirements in `${T2R_PREFIX}-build/install.json`.
Installed products are `bin/tempo2-rust`, `bin/getPeriod-rust`,
`lib/libtempo2pred_rust.a`, the shared `.dylib` (macOS) or `.so` (Linux),
`include/tempo2pred.h`, `include/tempo2pred_int.h`, and source under
`share/tempo2-rust/rust`. Runtime resources are separate.

For a source-tree CLI instead, after fetching dependencies:

```sh
cargo build --manifest-path rust/Cargo.toml --locked --offline --release --bin tempo2-rust
T2R_BIN="$PWD/rust/target/release/tempo2-rust"
"$T2R_BIN" --help
```

That executable path assumes Cargo's default target directory; adjust it if you
set `CARGO_TARGET_DIR`. Binaries under `target/*/examples/` are library or
diagnostic clients, with their own output contracts. They are not the main CLI.

The default build has no CALCEPH/BLAS/LAPACK features. If your model selects
`EPH_FILE`, follow the [CALCEPH build contract](../rust/NATIVE_BACKENDS_API.md)
and supply the selected provider/data. The same document explains explicit
BLAS/LAPACK providers, QR and runtime solver selection; the installer accepts the
same feature names through `--features`.
Native commands start in acceleration mode 1 (2 with `qr-default`); the JSON
general-fit adapter defaults to mode 0. Preserve the intended solver/provider
when comparing workflows; changing output routes is not a promise of identical
defaults. Production timing precision remains Rug128, with source-defined
ordinary-width boundaries and no user precision selector.

## Keep inputs and runtime choices explicit

Supply an existing TEMPO2 runtime containing the clocks, ephemerides,
observatories, Earth-orientation and time-conversion resources your inputs need:

```sh
export TEMPO2=/absolute/path/to/T2runtime
```

The native route reads `TEMPO2`; general JSON routes require `--runtime`.
Retain your PAR/TIM files, included-file layout, `UNITS`/`EPHVER`, clock and
ephemeris selections, observatory identifiers, fit flags, weighting, `ROBUST`
and `NITS`. Record resource versions/hashes, compiler/features and commands with
the results. Preserve intentional `TEMPO2_CLOCK_DIR` or `-clkdir` overrides when
using the native route. A new runtime with the same directory name can contain
different science inputs; do not swap clocks, ephemerides or units merely to
make two outputs agree. Review emitted warnings and their documented resource
defaults as part of the result.

Keep resources stable during a run. Structured results record consumed inputs
and resources; those hashes describe observed bytes, not a snapshot against
concurrent modification. See the [input/BAT contract](../rust/INPUT_TO_BAT_CONTRACT.md).
Qualification build directories mentioned in older examples are retained local
evidence, not data shipped in a fresh checkout.

## Choose the calculation and output

| Existing task/interface | Rust route | Contract |
| --- | --- | --- |
| `tempo2 -f model.par arrivals.tim` | `tempo2-rust -f model.par arrivals.tim` | Native-style fitting and text output, with PAR controls and source iteration order. |
| Native `-pre` | The same native option | Fits, then skips the second BAT/residual evaluation in each outer pass. It is not evaluation-only. |
| Fixed-parameter timing | `tempo2-rust timing --par … --tim … --runtime …` | JSON; no parameter fit. Default arithmetic residual-mean removal; `--no-mean` preserves the mean. |
| General fitting for a script | `tempo2-rust timing --fit --par … --tim … --runtime …` | JSON; NITS evaluation/fit/recomputation cycles. `--no-mean` is incompatible with fitting. |
| Input/preprocessing to BAT/BBAT | `tempo2-rust bats --par … --tim … --runtime …` | JSON; no residual fit. Ordered pairs and preprocessing options after `--`. |
| Native `-pred` / `-polyco` | The same native options | PAR-only predictor generation and source-format files; no timing fit. Explicit `predictor` subcommands provide JSON. |
| Core/toolkit C/C++ consumers | Public `tempo2_rust` APIs | Port the caller to canonical Rust owners; legacy `pulsar` structure ABI is not provided. |
| Standalone predictor C consumers | Installed predictor headers/libraries | Source-defined C surface and platform ABI limits in the [interface contract](../rust/LIBRARY_INTERFACES_API.md). |

Formats are selected explicitly; redirecting stdout does not change a command
from text to JSON or from fitting to evaluation. `residuals` and `fit` are
**narrower adapters**: the bounded isolated `@`/PKS input profile and ZERO/F0–F3
fitting contract in the [crate README](../rust/README.md#input-scope) apply.
Use `timing` / `timing --fit` for general timing/fitting. Native help retains
the source option catalogue, including unavailable plugins: `-gr`, `-output`
and selection-plugin execution are not supported. Plugin/GUI compatibility is
not inferred from an option appearing in help.

## Run and read a human report

Set `T2R_BIN` using one of the build/install routes above. Use absolute input
paths and a fresh working directory so generated files have a clear destination:

```sh
T2R_PAR=/absolute/path/to/model.par
T2R_TIM=/absolute/path/to/arrivals.tim
mkdir tempo2-rust-run
cd tempo2-rust-run
"$T2R_BIN" -f "$T2R_PAR" "$T2R_TIM"
```

The normal native cycle reads/preprocesses once, then performs BAT/residual
evaluation, one fit, recomputation and text output per outer NITS pass. The report
contains PSR identity, pre/post-fit RMS in microseconds, chi-square/degrees of freedom
where applicable, parameter values/errors/fit flags, derived quantities and
timing conventions. Read the uncertainty-rescaling notice and warnings alongside
the numbers. A completed report does not by itself establish scientific agreement.

For the repository's small robust-H demonstration, capture the repository root
before entering a fresh run directory and use your available runtime:

```sh
# Start in the repository root; TEMPO2 and T2R_BIN are already set.
T2R_REPO="$PWD"
mkdir -p docs/build
mkdir docs/build/robust-example
cd docs/build/robust-example
"$T2R_BIN" -npsr 1 -nobs 100 -noaccel \
  -f "$T2R_REPO/validation/general_fitting/inputs/robust-H.par" \
     "$T2R_REPO/validation/general_fitting/inputs/multi-frequency.tim"
```

This fixture has sixteen barycentric `@` TOAs, an F0 fit flag, `MODE 1`,
`ROBUST H` and `NITS 1`. It exercises the report/fitting interface; it is not a
terrestrial-resource or real-data validation. The source and installed-example
identities actually exercised for this guide are in [the check report](MIGRATION_REPORT.md).

Rust-owned diagnostic prefixes identify real emitters, for example
`general_fitting.rs`, `tkrobust.rs`, `tkfit.rs` and `core_cli.rs`; line numbers
depend on the build. `-displayVersion` adds actual Rust emit sites and explicitly
labelled TEMPO2 counterparts. `-v`/`-version` print Rust/package/source identity.
Native `-h`, `-H`, `-v`, `-version` and no-argument help retain exit status 1;
`--help` returns 0. The native help route needs `TEMPO2` set. These historical
statuses matter when invoking help from a script.

## Know where output is written

Native output uses the process working directory unless an output path is
explicitly supplied. Existing destinations can be replaced; completed files
and state changes can survive a later error. Important examples are:

| Request/active model | Files |
| --- | --- |
| `-newpar` or `-outpar PATH` | `new.par` or the requested path; multiple pulsars append `_1`, `_2`, etc. |
| `-residuals` | `residuals.dat` |
| `-writetim` | `out.tim`, written after preprocessing |
| `AVERAGERES` | `avg.dat` |
| Native `-pred` | `t2pred.dat`, `pred.tim` |
| Native `-polyco` | `polyco_new.dat`, `newpolyco.dat`, `polyco.tim`, subject to `-polyco_file` prefix |

Active models, global fits and debugging can write further products such as
`clockOffset.dat`, `ifunc.dat`, `telescopeXYZ.dat`, `global_covar.cvm` and fit
diagnostics. Output is not transactional. Inspect the
[core output contract](../rust/CORE_OUTPUT_API.md) and
[predictor contract](../rust/PREDICTOR_API.md) for the selected operation.
The isolated-directory practice also applies to JSON calculations with file
side effects, such as averaging or explicitly requested fit diagnostics.

## Evaluate without fitting, or automate a fit

With the same executable, runtime and absolute input paths:

```sh
"$T2R_BIN" timing --par "$T2R_PAR" --tim "$T2R_TIM" \
  --runtime "$TEMPO2" > timing.json
```

This evaluates the supplied model without fitting even if PAR fit flags are set.
Add `--no-mean` only when you intend to retain the arithmetic residual mean;
mean removal does not alter phase/count fields just to match that subtraction.
Evaluation still performs the source-defined timing state updates. For a general
fit instead, use `timing --fit` with those same named inputs. See the
[fitting API](../rust/GENERAL_FITTING_API.md) for covariance, constraints and
local/global parameter semantics. Repeat matching `--par`/`--tim` pairs for
multiple pulsars; preprocessing arguments follow `--`.

Read JSON as structured data: extended scalars and exact pulse counts use decimal
strings, and nonfinite/unwritten values have explicit representations. Preserve
units and precision rather than coercing every field to a binary64 number. Check
the exit status and stderr; a failed calculation publishes no success JSON,
although shell redirection can leave an empty destination and an output write
failure can leave a partial stream. The text report is not a machine schema.

## Integrate a library or predictor

Use Cargo package `tempo2-rust` (import `tempo2_rust`) from the checkout or the
installed `share/tempo2-rust/rust` source. The
[Rust/C interface contract](../rust/LIBRARY_INTERFACES_API.md) defines ownership,
errors and external consumers; Rust binary ABI across compiler versions and
legacy general-core C/C++ ABI are not promised. Predictor construction uses
PAR/runtime inputs without a timing fit. Existing predictor evaluation and plans
need no timing runtime; see the [predictor API](../rust/PREDICTOR_API.md) for
commands, file formats and units. The installed `getPeriod-rust` client retains
its source-defined fixed 1400 MHz/binary64 interface.

Serial execution is the default. Before adding workers or concurrent callers,
read the [execution contract](../rust/PARALLEL_EXECUTION_API.md): separate
contexts do not isolate all process state, and C predictor callers provide
process-wide exclusivity. The [measurements](../rust/PARALLEL_EXECUTION_REPORT.md)
record workload-specific timing and memory costs; this guide promises no general
speedup or memory reduction. Keep scientific acceptance separate from a working
build, successful command or performance measurement.
