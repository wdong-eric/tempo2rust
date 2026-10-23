# TEMPO2 Rust timing core

One library and JSON CLI for fixed-parameter residuals and ZERO/F0–F3 fitting of one isolated pulsar,
using TCB barycentric `@` TOAs or the declared PKS terrestrial path and F0
through F3. M1a and M1b are accepted. M2 evidence is in `M2_REPORT.md`; milestone
acceptance belongs to the central planning chat.

## Run

The crate uses Rust edition 2024, Rug 1.30.0 and bundled GMP/MPFR through
gmp-mpfr-sys 1.7.1. Rust/Cargo 1.97.1 on macOS arm64 is the executed toolchain.
GCC/GFortran 16 compile the vendored JPL/SOFA sources (`CC`/`FC` override the
compiler commands); a C toolchain also builds GMP/MPFR. Python 3 runs the fixture
comparators. The normal build uses only sources inside this crate. No TEMPO2
executable is needed. Barycentric evaluation requires no runtime data.

```sh
cd /Users/Eric.Dong/Downloads/tempo2rust/rust
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
resolved conventions, the owned input and ordered results. For `@`, epochs are MJD TCB,
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

The terrestrial implementation follows the same convention:
`form_bats::form_bats_all` (re-exported as `evaluate_terrestrial`),
`get_clock_corrections`, `get_clock_correction_sequence`, `load_eop`,
`get_eop`, `read_ephemeris`, `get_obs_coord` and `compute_tropospheric_delay`.
`tt2tdb.rs` matches the original filename and retains its routine name `tt2tb`.
The shared `dot_product` lives in `tempo2_util`, matching `tempo2Util.C`.

Descriptive names replace ambiguous shorthand: `sun_ssb`, `earth_ssb`,
`planets_earth`, `observatory_earth`, `correction_tt_tb`, `correction_tt_teph`
and `einstein_rate` identify the frames or quantities. IF99 uses explicit
coefficient/component/subinterval counts and Chebyshev value/derivative arrays.
Established astronomical abbreviations (SAT, BAT, TCB, IFTE, EOP) and useful
mathematical indices remain. The naming map and byte-identical output evidence
are recorded in [M1B_REPORT.md](M1B_REPORT.md#naming-pass--20-october-2026).

## Input scope

Required PAR entries are `PSRJ`, positive `F0`, and `PEPOCH`, plus explicit
`UNITS TCB` or the accepted omitted-UNITS rule with `EPHVER 5`. Optional `F1`,
`F2`, and `F3` default to zero with `set=false`. For `@`, the supported inactive metadata
are `RAJ`, `DECJ`, `POSEPOCH`, `DM`, `DMEPOCH`, `CLK`, `EPHEM`, `TZRMJD`,
`TZRFREQ`/`TZRFRQ`, and `TZRSITE`. They are preserved when supplied; no absent
astrometry, DM or resource metadata is invented. The complete syntax and
classification table is in [M1A_REPORT.md](M1A_REPORT.md#supported-inputs).

TIM input requires one literal `FORMAT 1` header, then exactly five fields:
`name frequency_MHz MJD uncertainty_us @`. Blank lines and whole-line `#` or
`C` comments are accepted. All original TOAs remain in input order. Other
directives, flags, sites other than PKS (below), binary models, higher spin derivatives, tracking,
glitches, noise terms and selection are unsupported and rejected. The bounded
fit operation below accepts local F0–F3 fit flags.

The first TOA supplies the signed fractional phase reference. TZR metadata does
not activate `REFPHS_TZR`. Fixed evaluation performs no mean subtraction. `@` bypasses clock and
propagation corrections; the supported isolated model has `SAT = BAT = BBAT`.
See the report for the source integer-range guards and other limits.

## Verification

```sh
cd /Users/Eric.Dong/Downloads/tempo2rust/rust
export CARGO_HOME="$PWD/.cargo-home"
export GMP_MPFR_SYS_CACHE="$PWD/target/gmp-cache"
python3 verify.py
```

This runs formatting, the Rust tests, and Clippy, then records all three accepted
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
agreement, and untested scientific scope. The fitting tests additionally read
root-tracked `validation/fitting/`, never an independent reference repository.
Linux execution, broader terrestrial coverage, binary timing, broader fitting,
distribution packaging and performance remain open.

Ported routines derive from TEMPO2, copyright George Hobbs, Russell Edwards
and the TEMPO2 contributors, under GPL-3.0-or-later; see [LICENSE](LICENSE).
Rug/GMP/MPFR retain their own dependency licenses. The pinned SOFA source has
its separate bundled license, including a commercial-use restriction; see
`native/sofa/copyr.lis`. JPL retains its GPL-2.0-or-later notices.

## PKS terrestrial evaluation (M1b)

```sh
cargo run --locked --offline -- residuals \
  --par tests/terrestrial/fixtures/test1.par \
  --tim tests/terrestrial/fixtures/test2.tim \
  --runtime /absolute/path/to/T2runtime
```

The explicit runtime root contains observatory definitions, directed clock tables,
DE405, IF99 and EOP. Weather files under `atmosphere/` are optional. Missing or
out-of-range weather uses the source's 101.325 kPa pressure and zero zenith wet
delay. A zero table value has the same source behavior. Warnings go to stderr and
also appear in JSON; effective values and substitution reasons accompany every
TOA. There is one normal weather path and no compatibility switch.

```rust,no_run
use std::path::Path;
use tempo2_rust::{load, evaluate_terrestrial, RuntimeContext, TimingError};
fn pks() -> Result<(), TimingError> {
    let input = load(Path::new("model.par"), Path::new("pks.tim"))?;
    let mut runtime = RuntimeContext::new(Path::new("T2runtime"))?;
    let result = evaluate_terrestrial(&input, &mut runtime)?;
    println!("{}", result.timing.observations[0].residual_s);
    Ok(())
}
```

Use homogeneous `pks` observations (case-insensitive); aliases such as `7` and
mixed `@`/PKS files are outside this profile. In addition to the common input
requirements, supply RAJ, DECJ, DM, POSEPOCH, DMEPOCH, `EPHEM DE405` and
`CLK TT(TAI)`. Other active parameters remain rejected. F0–F3 are supported;
no fit runs even when fit flags are present. SAT is observatory-clock MJD;
BAT and BBAT are TCB. The public result retains the existing timing fields and
adds per-TOA corrections, applied TT and UTC clock chains, weather provenance,
resource hashes/ranges, retained numerical dependencies and warnings.

The fixed profile uses IF99, IAU2000B, frequency dilation, Sun/planet Shapiro,
interstellar DM, IPM=1, NE_SW=4, SWM=0 and troposphere. Source table selection,
clock-route caching/ties, sequential step epochs, lower EOP proxy and clock UTC
fallback are retained. Missing indispensable resources and malformed data are
errors. EOP at/after its last sample is an error because the source's derivative
access is out of bounds there. No scientific coefficient correction is included.

`RuntimeContext` owns resource state. Evaluations require exclusive mutable access;
independent contexts do not share clock/IF99 caches. Keep runtime files unchanged
while a context is alive, and construct a new context to adopt a new resource set.
A failed evaluation returns no partial result; a later valid evaluation is supported.
Native dependency sources and intentional changes are described in `native/README.md`.

```sh
python3 verify_m1b.py --runtime /absolute/path/to/T2runtime
```

This checks the 80 pinned A2 resource identities, both terrestrial goldens, all
stage gates, the 4 ns response against the local BAT derivative, comparator
mutations, resource/weather/error recovery, CLI publication, and unchanged M1a
comparisons. Evidence goes to `verification/m1b/`; the separate contract and
immutable terrestrial goldens live under `tests/terrestrial/`. It neither changes
nor relabels `tests/reference/`. Full runtime tests copy resources to a temporary
owned directory; the supplied runtime remains read-only. Large runtime files are
explicit external test inputs, not bundled in the crate.

The verifier also runs the resource unit tests and public resource/CLI checks
in release mode. Both central-review `step_days=1e-300` reproducers must return
contextual constructor errors, leave CLI stdout empty, and permit valid
processing afterward in the same library host. DE405 grids are bounded by
native record/seek limits; IF99 grids must match the owned records and use
checked index/offset arithmetic. Release terrestrial exports must be
byte-identical to debug exports. The JPL repeated-short-read/cache-recovery
regression remains active in both builds.


## ZERO and spin fitting (M2 delivery; central acceptance pending)

```sh
cargo run --locked --offline -- fit \
  --par ../validation/fitting/inputs/all.par \
  --tim ../validation/fitting/inputs/bary.tim --norescale
# PKS uses the same command with --runtime ../tempo2/T2runtime.
python3 verify_m2.py
```

Use `fit_barycentric(&input, &FitOptions::default())`, or
`fit_terrestrial(&input, &mut runtime, &options)`. `FitOptions` contains only
`rescale_err_chisq` (default true). PAR `MODE`/`WEIGHT` 0 or 1 and positive
source-range integer `NITS` control execution; defaults are 0 and 1. The last
MODE/WEIGHT declaration wins, and every original declaration is retained in
`input.fit_controls()`. There are no competing CLI mode/iteration overrides.
Fixed-evaluation JSON for previously supported inputs remains unchanged.

The fit always includes ZERO, then supplied F0–F3 with local flag 1 in source
order. Global flags and active flags on other parameters fail at the fit
boundary; fixed evaluation still preserves them. Each cycle evaluates raw
residuals, subtracts their unweighted mean, constructs source-scaled columns,
whitens in binary64, solves with the source SVD and strict `1e-27` cutoff,
updates selected parameters sequentially, and recomputes centered residuals.
F1–F3 use the newly updated F0. ZERO is stored for that cycle, not accumulated
or subsequently subtracted from the timing model. Exactly NITS cycles run.

The versioned [`tempo2-fit-v1` schema](../validation/fitting/schema.json)
separates original input, fitted values, source physical errors, raw covariance,
unscaled/rescaled coordinate errors, raw/centered/linearized/recomputed residuals,
row/column mapping and full per-cycle timing/resource provenance. Covariance is
never rescaled. MODE 0 rescales errors even with `--norescale`. The source DOF
is `n - p`, including rank-deficient pseudoinverse results. Rank warnings do not
assert physical identifiability.

Source nonfinite fit diagnostics use `NaN`, `+Infinity` and `-Infinity` strings.
A square rescaled fit may complete with `status=completed_with_source_nonfinite`;
NaN callback errors retain preceding physical errors. Supplied PAR uncertainties
remain original metadata; source current errors begin at zero. An uninitialized
source `offset_e` has no numerical counterpart and is explicitly labeled with
`offset_error_s=null`. Source unsafe-access/termination/uninitialized-output
boundaries return contextual errors, with no partial fit result. Input `Scalar`
remains finite. See [BEHAVIOR_DIFFERENCES.md](BEHAVIOR_DIFFERENCES.md).

The new fitting path is safe Rust and adds no numerical dependency. All production
Rust modules and existing FFI sites are mapped in
[SOURCE_CORRESPONDENCE.md](SOURCE_CORRESPONDENCE.md). Retained native sources
remain byte-identical to the accepted M1b delivery; their original names and
paths are listed in `native/upstream_manifest.json`.
