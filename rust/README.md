# TEMPO2 Rust timing core

One Rust library and JSON CLI provide general ordered PAR/TIM input, source
preprocessing, BAT/BBAT and fixed-parameter timing through all fourteen binary
families. The existing bounded residual and ZERO/F0–F3 fit commands retain their
input scopes through adapters to the shared timing routines. General fitting is
being extended through `timing --fit`; its current implementation and remaining
scope are recorded in [GENERAL_FITTING_REPORT.md](GENERAL_FITTING_REPORT.md).
See [TIMING_MODELS_REPORT.md](TIMING_MODELS_REPORT.md) for the preceding fixed
timing delivery and its unresolved acceptance gates.

## General fitting (AE work in progress)

```sh
rust/target/debug/tempo2-rust timing --fit \
  --par validation/general_fitting/inputs/global0.par --tim validation/fitting/inputs/bary.tim \
  --par validation/general_fitting/inputs/global1.par --tim validation/fitting/inputs/bary.tim \
  --runtime validation/reference/build/source/T2runtime
```

The general route uses canonical PAR/TIM parsing and preprocessing, then exactly
NITS cycles of BAT/residual evaluation, fitting and recomputation. PAR files own
MODE/WEIGHT, ROBUST and NITS. `--covariance MODEL`, `--norescale`,
`--force-always-fit-for-glitches` and `--acceleration INTEGER` map to
`general_fitting::FitOptions`; the baseline uses the source SVD. `--no-mean`
cannot be combined with `--fit`.

Call `general_fitting::fit` on a preprocessed `BatInput` for the complete cycle,
or `general_fitting::t2_fit` / `do_fit_all` on prepared canonical BAT/residual
state. The `tempo2-general-fit-v1` result records rows, local/global coordinates,
matrices, constraints and targets, raw solver outputs, physical updates and
noise projections. Covariance stays in source fit coordinates. These interfaces
do not imply that AE's full source scope or numerical acceptance is complete.

## General fixed-parameter timing

```sh
rust/target/release/tempo2-rust timing \
  --par validation/binary/fixtures/dd_baseline.par \
  --tim validation/binary/fixtures/barycentric.tim \
  --runtime validation/reference/build/source/T2runtime --no-mean
```

Run from the repository root after building the crate. Repeat ordered `--par` /
`--tim` pairs for multiple pulsars. The default removes the source arithmetic
residual mean; `--no-mean` keeps it. Neither choice fits parameters or changes
phase/count fields merely to match the residual mean. Preprocessing arguments
follow `--`, as for `bats`.

```rust,no_run
use std::path::Path;
use tempo2_rust::{bats::{self, InputPair}, timing, RuntimeContext, TimingError};
fn fixed_timing() -> Result<(), TimingError> {
    let root = Path::new("T2runtime");
    let mut input = bats::load_pairs(&[InputPair {
        par: "model.par".into(), tim: "arrival.tim".into(),
    }], root)?;
    bats::preprocess(&mut input, &[], root)?;
    let mut runtime = RuntimeContext::from_root(root)?;
    let result = timing::evaluate(&mut input, &mut runtime, true)?;
    assert!(result.remove_mean);
    Ok(())
}
```

`timing::evaluate` runs BAT preparation and full residual evaluation on the same
canonical context. `timing::form_residuals` operates directly on prepared BAT
state. `binary::{model, fit_function, update}` expose source delay/derivative and
update callbacks; `model` selector -1 requests delay and preserves DD's extended
return, while `fit_function` performs the source binary64 narrowing.

JSON includes AC provenance plus PET, binary delay, phase/nphase, exact pulse
numbers as decimal strings, residual/residualtn, original expanded TIM row index,
applicable averaged fields and parameter mutations. TZR is a separate record;
inapplicable averages are null. Signed nonfinite classes remain explicit. Errors
publish no success JSON and retain completed canonical writes; use fresh input
for recovery after a partially completed pipeline.

Stored noise signals must come from the caller's actual noise calculation/data.
The eccentric GW routine consumes a previously produced residual. The executable
[library example](examples/timing_lifecycle.rs) demonstrates these lifetimes and
explicit state switches. `AVERAGERES` writes source-format `avg.dat` in the working
directory; file failures are recoverable errors. The direct averaging functions
share the same routines. See [the runnable matrix](../validation/timing_models/EXAMPLES.md).

## General input to BAT

```sh
cd /Users/Eric.Dong/Downloads/tempo2rust
export CARGO_HOME="$PWD/rust/.cargo-home"
export GMP_MPFR_SYS_CACHE="$PWD/rust/target/gmp-cache"
cargo run --manifest-path rust/Cargo.toml --locked --offline --release -- bats \
  --par validation/input_to_bat/fixtures/ordinary.par \
  --tim validation/input_to_bat/fixtures/ordinary.tim \
  --par validation/input_to_bat/fixtures/ecliptic.par \
  --tim validation/input_to_bat/fixtures/ordinary.tim \
  --runtime validation/reference/build/source/T2runtime
```

Each nth `--par` pairs with the nth `--tim`. Arguments after `--` are source
preprocessing arguments, for example `-- -epoch 55001`. The runtime directory
supplies the selected planetary ephemeris, clocks, observatories, EOP and time
resources. CALCEPH and external selection-plugin requests return explicit
unavailable-boundary errors. The CLI emits one complete JSON result or exits with
an error on stderr; it does not emit success JSON before the calculation finishes.

```rust,no_run
use std::path::Path;
use tempo2_rust::{RuntimeContext, TimingError, bats::{self, InputPair}};
fn barycentric_times() -> Result<(), TimingError> {
    let root = Path::new("T2runtime");
    let mut runtime = RuntimeContext::from_root(root)?;
    let mut input = bats::load_pairs(&[InputPair {
        par: "model.par".into(), tim: "arrival.tim".into(),
    }], root)?;
    bats::preprocess(&mut input, &[], root)?;
    let result = bats::evaluate(&mut input, &mut runtime)?;
    assert_eq!(result.schema_version, 1);
    Ok(())
}
```

The result preserves ordered pulsar/observation identity, byte strings, flags,
deletion state, SAT/BAT/BBAT, physical correction components, units and consumed-file
hashes. Extended scalars use round-trip strings; source nonfinite classes are
explicit. Main observations and the reference observation have separate membership.
The source-level readers and helpers are independently callable. See
[INPUT_TO_BAT_CONTRACT.md](INPUT_TO_BAT_CONTRACT.md) for ownership, retained process
state, narrower `update_bats_all` behavior, and error-prefix semantics.

The sections below document the existing bounded residual/fit interfaces.

## Run

The crate uses Rust edition 2024, Rug 1.30.0 and bundled GMP/MPFR through
gmp-mpfr-sys 1.7.1. Rust/Cargo 1.97.1 on macOS arm64 is the executed toolchain.
GFortran 16 compiles the vendored SOFA sources (`FC` overrides the compiler);
a C toolchain also builds GMP/MPFR. JPL parsing/interpolation executes in Rust
with a minimal owned libc stdio boundary. Python 3 runs the fixture
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
`get_clock_corrections`, `get_correction`, `get_sequence`, `toa2utc`, `load_eop`,
`get_eop`, `read_ephemeris`, `get_obs_coord::iau2000b` and `compute_tropospheric_delays`
(with a shared single-observation body for the bounded frontend).
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
fallback are retained. Tables skip unconverted rows, keep decreasing/duplicate/nonfinite
samples, and preserve endpoint proxies and defined IEEE results. Exhausted clock
fallbacks return zero; source exits become terminal-source errors. The approved
storage boundaries are listed in `CLOCK_REPORT.md`. Other indispensable resource
errors and existing timing guards remain in force. EOP at/after its last sample is an error because the source's derivative
access is out of bounds there. Assignment W corrects the demonstrable upper
coefficient typo in `NMF_wet`; its original witness and exact changed expectations
are preserved in [TERRESTRIAL_RESOURCES_REPORT.md](TERRESTRIAL_RESOURCES_REPORT.md).

`RuntimeContext` owns IF99 and JPL resources. Observatory records/aliases, EOP,
meteorology tables, clock tables/index/sequences and diagnostic history have
synchronized process lifetime. A new context, changed root or changed file does
not clear those caches. The source counterparts retain explicit observatory
append and meteorology replacement operations; the public constructor initializes
EOP/weather on first use. Provenance records the actual cached bytes and table
selection, including resources retained from an earlier root.
A failed evaluation publishes no partial result. A clock source-exit or storage
boundary latches the shared session; later clock operations return that terminal
state until a fresh process. Independent observatory, EOP, meteorology and
lifecycle operations remain accessible; they do not inherit the clock fault.
Recoverable EOP query boundaries and JPL/IF99 recovery retain their scope.
`take_clock_diagnostics()` drains source-channel byte events without resetting
history or terminal state; the CLI routes pending error-prefix bytes to stderr.
Native dependencies are described in `native/README.md` and `CLOCK_REPORT.md`.

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
in release mode. Under Assignment I, JPL's `step_days=1e-300` conversion is
rejected at use. Y likewise rejects an IFTE conversion only when it is used. Both leave CLI stdout empty
and permit valid processing afterward. JPL's canonical cache-before-read and
partial-read behavior are now tested against H, including stale retry success;
IFTE retains checked native conversion, seek and coefficient access boundaries. Release terrestrial exports
must be byte-identical to debug exports.

Assignment I evidence is separate in `verification/jpl/`; see `JPL_REPORT.md`.
Run `python3 ../validation/jpl_port/run.py --profile debug` and `--profile release`
for the 55-scenario exact H gate. `../validation/jpl_port/verify.py` runs the
complete I verification and redirects existing M1/M2 verifiers into that directory.
M2's four raw SVD-order diagnostics remain unresolved; the verifier explicitly
checks that exact failure set. Historical verification directories are immutable.



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
`input.fit_controls()`. This returns a detached owned snapshot derived from the
canonical source `fitMode` and `nits`; its declarations and default provenance
remain frontend metadata. There are no competing CLI mode/iteration overrides.
`input.source_pulsar()` provides read-only inspection of all 349 source fields;
`parameter_store()` and `source_observations()` refer to that same owned context.
Clone `TimingInput` for an independent owner graph. The full 10000 by 10000 source
covariance uses copy-on-write between independent clones; no active-fit-size
substitution is made.
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

## Parameter storage foundation (Assignment O)

`TimingInput` owns the source-indexed `tempo2::ParameterStore`: 166 enum entries,
2,000 allocated slots and 9,616 backing cells. `param_LAST` is a marker. Source
integer flags/counts and logical sizes remain signed; backing sizes are independent.
The selected initializer preserves all labels, including both `param_daop` and
`param_iperharm` labeled `IPERHARM`, blank cells, and bytes after terminating NULs.
Unwritten prefit/link entries are `None`, not zero or experimental caller seeds.

Loading, fixed evaluation and fit callbacks share that one store. The current
strict PAR frontend writes `val`, `fitFlag` and `paramSet`; supplied uncertainty
and original lines remain separate frontend metadata. It does not implement full
source `readValue`, prefit initialization, linked getters, or general PAR/TIM input.
Source `err` is initialized to zero and subsequently assigned by fit callbacks.

`parameters()` now returns an **owned** `BTreeMap<String, Parameter>` derived on
each call, instead of a borrowed map. Indexing/iteration still works; callers that
need a reference should bind the snapshot first. Mutating the snapshot has no
effect on `TimingInput`. The five historical default keys, optional-key membership,
and CLI/JSON fields and values are unchanged. `Parameter` remains the scalar
compatibility view; `tempo2::SourceParameter` is the array-valued source structure.
`parameter_store()` offers read-only inspection of this bounded bridge state.

`copy_param` preserves source field order, complete destination tails and links,
negative/zero copy counts, and logical-size assignment last. It returns a named
boundary on a missing cell/string byte or unwritten value. Earlier writes remain
available through explicitly diagnostic access, while `cell()` and later copy
calls reject the affected destination. Rust ownership excludes overlapping buffers.
Whole-object `Clone` copies ownership and is distinct from `copy_param`.

Run the complete O evidence with:

```sh
python3 validation/parameter_storage_port/verify.py
```

This writes only `rust/verification/parameter_storage/` and build caches. Existing
clock/JPL/M1/M2 runners retain their projections and tolerances. M2 must still exit
nonzero with its four historical raw-mode-order diagnostics. See
`PARAMETER_STORAGE_REPORT.md` for executed identities and scope.

## Source observation storage (Assignment R)

`TimingInput` now owns one complete `tempo2::SourceObservation` per input row.
`source_observations()` exposes these records read-only. Fields keep the pinned
header's names, signed integer types and fixed array capacities. `None` means
source-unwritten for each scalar, array cell and byte; it does not mean zero.
`SourceObservation::default()` is wholly unwritten. `Clone` copies full backing,
including initializedness and inactive tails, into independent owned storage.
This is whole-record assignment support, not a port of `copyPSR` or C layout.

The strict FORMAT 1 adapter supplies only `sat`, `freq`, `toaErr`, `fname` and
`telID`. The filename capacity is 501 bytes including NUL; the last fitting
UTF-8 filename is 500 bytes. Oversized names return a storage error with the
source line. Numeric parsing and the supported input grammar are unchanged.
Original lines, indices and tokens are separate provenance. Evaluators use the
source SAT/frequency/site; fitting weights use source `toaErr`. Output BAT and
residual records do not implicitly initialize the input record.

**API change:** `observations()` returns a fresh owned `Vec<Observation>`,
previously `&[Observation]`. Bind the returned vector when retaining references
to its entries. Editing it cannot modify the input. Existing JSON keys, values
and original text remain unchanged. `observation_site(index)` reads the source
site bytes without substituting historical provenance.

Clock operations use the same source record. The bounded timing caller clones
it and explicitly seeds its historical zero correction buffers/count and no
SAT jumps; those caller inputs are not allocation defaults. Source reads are
checked only when executed, preserving identity-clock early returns, signed
counts, partial writes, destination suffix bytes and existing terminal sessions.

Reproduce R's exact native assignment and M projection comparisons and all
required regressions with `python3 ../validation/observation_storage_port/verify.py`
from this directory. All new evidence is under `verification/observation_storage/`.
See `OBSERVATION_STORAGE_REPORT.md` for identities, source maps and remaining scope.


## Assignment W terrestrial resources

All 22 active observatory/EOP/meteorology/Niell/tropospheric routines now use the
canonical process owner and T/O/R records. Full internal source routines and the
bounded PKS public frontend share the tropospheric calculation. The frontend's
supported sites and JSON schema remain unchanged. See
[TERRESTRIAL_RESOURCES_REPORT.md](TERRESTRIAL_RESOURCES_REPORT.md) for exact U
coverage, source corrections, safe boundaries and remaining limits.

```sh
python3 ../validation/terrestrial_resources_port/verify.py
```

This compares all 130 U histories in both profiles and runs the accepted T/O/R,
clock/glob/provenance, JPL and M1/M2 checks into ignored
`verification/terrestrial_resources/`. Native correction artifacts and the
pre-edit freeze must already exist; ordinary verification does not record new
expectations. Central retains acceptance; M2's four historical mode-order
diagnostics remain unresolved.


## Complete time-ephemeris lifecycle (Y; acceptance pending)

The 29 IFTE, tt2tdb and read_fortran.h routines share canonical process ownership
and one observation calculation with the existing strict frontend. See
[TIME_EPHEMERIS_REPORT.md](TIME_EPHEMERIS_REPORT.md) for the three authorized
corrections, exact V comparison projection, reproduction commands and remaining
M2/0049 limitations. General PAR/TIM acceptance and CALCEPH library execution
remain outside this transfer. Generated Y evidence is local and ignored under
`../validation/time_ephemeris_port/`.


AA adds the complete internal observer-coordinate, vector and legacy UT1 driver
on canonical state. Run `python3 ../validation/observer_coordinates_port/examples.py`
from this crate for terrestrial, satellite and vector/UT1 examples (or add
`--profile release`). These explicitly initialized examples exercise source
routines; they do not expand the public PAR/TIM grammar. See
[OBSERVER_COORDINATES_REPORT.md](OBSERVER_COORDINATES_REPORT.md) for coverage,
shared frontend behavior, exact evidence identities and remaining integration.
