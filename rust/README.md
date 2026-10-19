# TEMPO2 Rust barycentric core

One library and JSON CLI for fixed-parameter residuals of one isolated pulsar,
using TCB barycentric `@` TOAs and F0 through F3. M1a implementation is delivered
for central review; milestone acceptance belongs to the central planning chat.

## Run

The crate uses Rust edition 2024, Rug 1.30.0 and bundled GMP/MPFR through
gmp-mpfr-sys 1.7.1. Rust/Cargo 1.97.1 on macOS arm64 is the executed toolchain.
A native C build toolchain is needed for GMP/MPFR; Python 3 is needed for the
copied reference comparator. No TEMPO2 executable or runtime files are needed.

```sh
cd /Users/Eric.Dong/Downloads/tempo2-rust/rust
export CARGO_HOME="$PWD/.cargo-home"
export GMP_MPFR_SYS_CACHE="$PWD/target/gmp-cache"
# For a fresh checkout/cache; the delivered workspace already has these packages.
cargo fetch --locked
cargo run --locked --offline --bin tempo2-rust -- residuals \
  --par tests/reference/fixtures/original/test1.par \
  --tim tests/reference/fixtures/original/test1.tim
```

The three residuals are approximately 0, 0, and 0.4 seconds, with pulse counts
0, 864, and 3456. Decimal input rounding is retained in the full-precision JSON.
The command accepts `--par` and `--tim` in either order. Errors go to stderr and
exit nonzero. Calculation and JSON serialization finish before stdout is written.
An OS-level stdout write failure can still leave a partial transport stream;
the CLI does not claim atomic delivery through a pipe.

## Library

```rust,no_run
use std::path::Path;
use tempo2_rust::{load, evaluate_barycentric, TimingError};

fn residuals() -> Result<(), TimingError> {
    let input = load(Path::new("model.par"), Path::new("arrival.tim"))?;
    let result = evaluate_barycentric(&input)?;
    for row in &result.observations {
        println!("{} {} {}", row.index, row.pulse_number, row.residual_s);
    }
    Ok(())
}
```

`TimingInput` has read-only accessors for the validated values, conventions,
source paths, SHA-256 hashes, complete source text, and original observation
records. Evaluation borrows it and returns an owned result. There is no global
model or mutable result cache. `Scalar` permits strict decimal construction and
immutable access to its 128-bit MPFR value; there is no precision selector.

The public output is schema version 1, distinct from the reference fixture
adapter's schema 2. It includes engine version, precision, source correspondence,
resolved conventions, the owned input and ordered results. Epochs are MJD TCB,
phases are turns, and residuals are seconds. Extended scalars and both integer
pulse counts are JSON strings. Frequency and uncertainty retain binary64 input
semantics and their original tokens. Results include the first input TOA's signed
fractional reference phase, full phase, reduced nearest integer, and full count.

## Source correspondence

Internal modules and routines follow TEMPO2's names with Rust spelling:
`read_parfile::read_parfile` / `read_value`, `read_timfile::read_timfile`,
`form_residuals::form_residuals`, `longdouble::parse_longdouble`, and the
`tempo2_util` coordinate/pulse helpers. The residual kernel retains names such
as `phase2`, `phase3`, `phase5`, `phas1`, `nphase`, `phaseint` and `delta_t`.

The public `load` and `evaluate_barycentric` entry points are unchanged;
`evaluate_barycentric` re-exports the internal `form_residuals` function.
Public types, accessors and JSON fields retain their established names. See
[M1A_REPORT.md](M1A_REPORT.md#calculation-and-source-correspondence) for the
routine map and exact before/after naming-pass verification.

## Input scope

Required PAR entries are `PSRJ`, positive `F0`, and `PEPOCH`, plus explicit
`UNITS TCB` or the accepted omitted-UNITS rule with `EPHVER 5`. Optional `F1`,
`F2`, and `F3` default to zero with `set=false`. The supported inactive metadata
are `RAJ`, `DECJ`, `POSEPOCH`, `DM`, `DMEPOCH`, `CLK`, `EPHEM`, `TZRMJD`,
`TZRFREQ`/`TZRFRQ`, and `TZRSITE`. They are preserved when supplied; no absent
astrometry, DM or resource metadata is invented. The complete syntax and
classification table is in [M1A_REPORT.md](M1A_REPORT.md#supported-inputs).

TIM input requires one literal `FORMAT 1` header, then exactly five fields:
`name frequency_MHz MJD uncertainty_us @`. Blank lines and whole-line `#` or
`C` comments are accepted. All original TOAs remain in input order. Other
directives, flags, sites, binary models, higher spin derivatives, tracking,
glitches, noise terms, fitting and selection are unsupported and rejected.

The first TOA supplies the signed fractional phase reference. TZR metadata does
not activate `REFPHS_TZR`. There is no mean subtraction. `@` bypasses clock and
propagation corrections; the supported isolated model has `SAT = BAT = BBAT`.
See the report for the source integer-range guards and other limits.

## Verification

```sh
cd /Users/Eric.Dong/Downloads/tempo2-rust/rust
export CARGO_HOME="$PWD/.cargo-home"
export GMP_MPFR_SYS_CACHE="$PWD/target/gmp-cache"
python3 verify.py
```

This runs formatting, nine Rust tests, and Clippy, then records all three accepted
fixture comparisons and independent F3/4 ns diagnostics under `verification/`.
`cargo test --locked --offline` alone also checks the copied comparator and
fixture hashes. The fixture adapter is test-only; it is not a scenario mode of
the public CLI. To generate just its three projections:

```sh
cargo run --locked --offline --example compare_fixtures
python3 tests/reference/compare.py tests/reference/outputs/long_span.json \
  target/reference-comparison/long_span.json
```

`tests/reference/` contains unchanged A2 artifacts and provenance pinned to
`7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`. Tests do not read either independent
reference repository. The report separates implementation checks, numerical
agreement, and untested scientific scope. Linux execution, terrestrial timing,
binary timing, fitting, distribution packaging and performance remain open.

Ported routines derive from TEMPO2, copyright George Hobbs, Russell Edwards
and the TEMPO2 contributors, under GPL-3.0-or-later; see [LICENSE](LICENSE).
Rug/GMP/MPFR retain their own dependency licenses.
