# Assignment C — M1a barycentric implementation

Delivered 20 October 2026 in `/Users/Eric.Dong/Downloads/tempo2-rust/rust/`.
**The runnable implementation and focused verification are complete. M1a remains
pending central acceptance; Assignment D has not started.** No central decision,
canonical TEMPO2 source, reference artifact, or precision-study file was edited.

The Rust library and CLI calculate the three accepted barycentric cases without
calling TEMPO2. All adopted comparison gates pass. The largest residual
difference is **3.76201e-25 s**, the largest full-phase difference is
**4.97718e-23 turns**, and both full and reduced pulse counts agree exactly.
There are no unresolved discrepancies within the declared subset and no shared
design change required to reproduce these results.

## TEMPO2 naming pass — 20 October 2026

Applied the requested naming pass after the initial delivery. The public
`load` and `evaluate_barycentric` signatures, public types/accessors, input
contracts, JSON fields, errors, numeric precision and arithmetic order are
unchanged. `evaluate_barycentric` is a direct re-export of the internal
`form_residuals::form_residuals` function; it adds no wrapper or alternate path.

| Previous organization/name | Current TEMPO2 correspondence |
| --- | --- |
| `phase` / internal `evaluate_barycentric` | `form_residuals::form_residuals`, matching `formResiduals.C` / `formResiduals` |
| Combined `parse_sources` | `read_parfile::read_parfile`, `read_parfile::read_value`, and `read_timfile::read_timfile`, called by the unchanged `load` operation in `input.rs` |
| `scalar` / decimal trait implementation | `longdouble::parse_longdouble`; the public `Scalar::from_str` delegates to it |
| Inline coordinate conversion | `tempo2_util::{hms_turn, dms_turn, turn_deg}`, preserving the existing binary64 operations on reader-validated components |
| Inline signed remainder / `nearest_pulse` | `tempo2_util::{fortran_mod, fortran_nlong}`; range checks remain explicit in the residual kernel |
| Kernel `input`, `epoch`, `delta`, `reduced`, `reference`, `rounded`, `pulse_number` locals | `psr`, `bbat`, `delta_t`, `phase5`, `phas1`, `nphase`, `pulse_n` |
| Existing source-like scientific locals | `nf0`, `ntpd`, `ff0`, `ftpd`, `phase2`, `phase3`, `phaseint`, `arg` retained; readers use `param`, `obsn`, `sat`, `freq`, `toa_err` |

The obsolete modules were removed. A private `ParFile` groups the validated
model contents while `load` assembles the same owned `TimingInput`; it does not
add a public model type or change serialization. File-read and validation order
are preserved, including the diagnostic when an invalid PAR is accompanied by a
missing TIM. The `longdouble` name denotes source correspondence only: this is
still fixed Rug128, not a C `long double` representation. Similarly,
`fortran_nlong` retains the rounded Float until the existing checked integer
conversion rather than introducing an unchecked C-style cast.

The focused suite was rerun: all nine Rust tests, all three accepted reference
comparisons, formatting and Clippy pass. The three actual fixture projections,
`numerical_errors.json`, the F3 diagnostic and the public CLI example are all
**byte-identical** to their snapshots before the naming pass. Seventeen CLI
cases also match exactly in exit status, stdout and stderr, covering valid
calculations, coordinate conversion, negative reference phase, parsing errors,
unsupported inputs, missing-file error precedence and pulse-count overflow.
`verification/naming_equivalence.json` records these comparisons and hashes.
The temporary baseline executable and source snapshot remain under ignored
`target/naming-before/`; normal reproducible acceptance checks remain `python3
verify.py`. `verification/source_hashes.json` now identifies the renamed source.

At completion of the naming pass, the existing Git index and read-only
design/reference/precision inputs were preserved; no staging or commit action
had yet been performed by this chat. The subsequent initial-commit review is
recorded below.

## Delivered files and identities

| Component | Files under `rust/` |
| --- | --- |
| Crate and dependency lock | `Cargo.toml`, `Cargo.lock`, `LICENSE` |
| Fixed precision and source-named decimal parsing | `src/longdouble.rs` |
| Owned input/result, provenance and contextual error types | `src/lib.rs` |
| Public loading and source-corresponding readers | `src/input.rs`, `src/read_parfile.rs`, `src/read_timfile.rs` |
| Sexagesimal conversion and Fortran-style pulse helpers | `src/tempo2_util.rs` |
| Source-order phase and residual kernel | `src/form_residuals.rs` |
| JSON residuals command | `src/main.rs` |
| Behavioral, precision, F3, boundary and recovery checks | `tests/behavior.rs`, internal tests in `src/form_residuals.rs` |
| Unchanged reference snapshot and hash verification | `tests/reference/`, `tests/reference_contract.rs` |
| Test-only scientific projection and alignment adapter | `tests/support/projection.rs`, `examples/compare_fixtures.rs` |
| Reproduction and measured outputs | `verify.py`, `verification/*.json`, `verification/*.log`, `README.md`, this report |

The accepted reference is independent revision
`7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`, based on canonical TEMPO2
`fda6ad4a9753ad7dc5d291406f2f310a9161423f` plus the approved initializer patch.
Before copying, the three fixture hashes, schema, comparator, build manifest,
artifact manifest, and A2 report were checked against decision 0002. The copied
schema and comparator remain byte-identical; their historical submission-time
M0 wording was preserved.

| Copied contract artifact | SHA-256 |
| --- | --- |
| `outputs/barycentric.json` | `a9bac02d3ebeb4a65935f208b6e1ca8a58362d604d66a1fb55b9ea2f788a675e` |
| `outputs/barycentric_4ns.json` | `44a2dfbbb294e932fc66657f2ca8b47f866d17bb26b337c11e6ebe8786e4d9af` |
| `outputs/long_span.json` | `d62cc4cfd2a702a255f0c0d3b392c4cf9f93a2ba10feb84964feabca14069b44` |
| `schema.json` | `39de24bd60d7d33ea88a776771e0932fca1182913c3973b95a6f3708e9a46043` |
| `compare.py` | `67fc9cbcdc6e5185ca06789b1b2b638c51746b774864b5044e1715213d476679` |

`tests/reference/provenance.json` records all 14 copied files, original paths,
hashes and reference revisions. Copies include the four PAR/TIM inputs, three
goldens, contract, reference manifests, patches and A2 report. Tests use only
these copies. `verification/source_hashes.json` identifies this delivered crate's
source, lockfile, test inputs, adapter and verification script.

The root was not a Git repository at entry. The authorized bootstrap initialized
`main` and added root `.gitignore` rules for `tempo2/`, `validation/reference/`,
Cargo caches, native build trees and Python caches. Both independent repositories
remain clean with their histories intact. During the original delivery check,
other workspace activity had populated the root index with the project files;
that staging was preserved, and the root still had no commits. The central
documents and precision study contents remain untouched. No remote or push was
created by this chat.

### Initial project commit review — 20 October 2026

At the user's subsequent request, the staged project snapshot was reviewed for
the initial commit, including the naming pass. The snapshot contains the central
documents, Rust crate and copied test evidence, and the delivered precision study.
Neither independent repository, any Git link, nor build/cache directories are
included. Both independent repositories are clean at the revisions recorded above.

All 31 staged files in `verification/source_hashes.json` match the inputs of the
passing focused run. The 14 copied A2 artifact hashes and all 28 recorded
precision-study artifact hashes also match. The six naming-pass output artifacts
remain byte-identical to their baseline snapshots. Review found no blocking
implementation issue; the only subsequent edit was this report's Git-history
clarification, so the numerical checks were not repeated. The evidence whitespace
exceptions documented below are retained. This commit does not confer M1a
acceptance or release Assignment D.

## Reproduction and platform

Executed with Rust **1.97.1** (`8bab26f4f68e0e26f0bb7960be334d5b520ea452`),
Cargo **1.97.1**, LLVM **22.1.6**, target **aarch64-apple-darwin**,
platform **macOS-26.7-arm64-arm-64bit**. Full command records and toolchain
details are in `verification/commands.json` and `verification/toolchain.log`.

```sh
cd /Users/Eric.Dong/Downloads/tempo2-rust/rust
export CARGO_HOME="$PWD/.cargo-home"
export GMP_MPFR_SYS_CACHE="$PWD/target/gmp-cache"
# Needed only when filling a fresh Cargo cache:
cargo fetch --locked

python3 verify.py

cargo run --locked --offline --bin tempo2-rust -- residuals \
  --par tests/reference/fixtures/original/test1.par \
  --tim tests/reference/fixtures/original/test1.tim
```

`verify.py` runs `cargo fmt --check`, `cargo test --locked --offline`, and
`cargo clippy --all-targets --locked --offline -- -D warnings`. It then uses
the unchanged Python comparator and 90-digit Decimal differences to publish
`verification/numerical_errors.json` and the actual three scientific projections.
The nine Rust tests include reference verification. Python 3 is required for
the comparator; it is not a runtime dependency of the library or CLI.

For a standalone projection regeneration and comparison:

```sh
cargo run --locked --offline --example compare_fixtures
python3 tests/reference/compare.py tests/reference/outputs/barycentric.json \
  target/reference-comparison/barycentric.json
python3 tests/reference/compare.py tests/reference/outputs/barycentric_4ns.json \
  target/reference-comparison/barycentric_4ns.json
python3 tests/reference/compare.py tests/reference/outputs/long_span.json \
  target/reference-comparison/long_span.json
```

The first default offline Cargo invocation found no Rug in the user's default
cache. Available registry packages and B's GMP/MPFR cache were copied into C's
ignored local caches, without modifying B. All subsequent builds and execution
were offline. No package installation or network fetch was needed in this run.
This was a cache-availability issue, not a numerical or dependency-build failure.
Cold native compilation was not benchmarked.

Retained dependencies are Rug **1.30.0**, gmp-mpfr-sys **1.7.1** (bundled
GMP **6.3.0**, MPFR **4.2.2**), az **1.3.0**, libc **0.2.191**, and libm
**0.2.16**, retaining B's numerical dependency versions. Serde **1.0.229**,
serde_json **1.0.151** and sha2 **0.10.9** provide structured JSON and input
identity. `Cargo.lock` pins all transitive versions and checksums;
`verification/dependencies.log` records the graph. `otool -L` on the executed
debug CLI lists only macOS libSystem and libiconv, consistent with static bundled
GMP/MPFR. Distribution must retain the applicable licenses; no packaging claim
is made here. The port carries the upstream GPL-3.0-or-later license.

## Supported inputs

PAR keywords are case-insensitive, but the accepted time-scale value is literal
`TCB`, EPHVER is literal `5`, and the TIM header is literal `FORMAT 1`.
Duplicate PAR keywords, including duplicate TZRFREQ/TZRFRQ aliases, are errors.
Whole-line `#` comments, uppercase `C` comments, and blank lines are accepted in
both files. Inline comments and surplus tokens are rejected.

| PAR input | Status and meaning |
| --- | --- |
| `PSRJ name` | Required single pulsar identity; text preserved |
| `F0 value` | Required positive spin frequency in s^-1; directly parsed into Rug128 |
| `F1`, `F2`, `F3` | Optional spin derivatives in s^-2, s^-3, s^-4; absent values are zero with `set=false`, fit flag 0 |
| `PEPOCH value` | Required MJD TCB spin epoch |
| `UNITS TCB` | Explicit TCB convention; no unit transformation |
| `EPHVER 5` | Accepted TEMPO2 setting. If UNITS is omitted, retain `initialiseOne`'s SI_UNITS/TCB default and record this resolution. If neither is supplied, reject unresolved metadata |
| `RAJ hh:mm:ss`, `DECJ [+-]dd:mm:ss` | Optional inactive astrometry. Source-order binary64 sexagesimal conversion to radians, then exact promotion into Rug128. RA is in [0,24 h); DEC is in [-90,90 deg]. Minutes and seconds are in [0,60); hour/degree and minute fields are integral. Negative-zero DEC is retained |
| `POSEPOCH`, `DM`, `DMEPOCH` | Optional inactive metadata; supplied decimal values/presence/flags preserved. No dispersion or astrometric calculation for this path |
| `CLK label`, `EPHEM label` | Optional inactive resource declarations, preserved verbatim. No resource lookup or validation is performed |
| `TZRMJD`, `TZRFREQ` / `TZRFRQ`, `TZRSITE` | Optional inactive reference metadata. Alias maps to canonical `TZRFRQ`. No `REFPHS_TZR` activation |
| Numeric fit/uncertainty suffixes | `value`, `value flag`, `value uncertainty`, or `value flag uncertainty`. Flags are 0/1/2; a lone suffix exactly 0/1/2 is a flag as in `readValue`. Nonnegative uncertainty is metadata in original units. All tokens and raw lines are retained; no fit runs |
| Everything else | Rejected, including supplied F4–F12, TRACK (even zero), BINARY (including NONE), PM/DSHK, DM derivatives, glitches, jumps, noise, selection and REFPHS |

Missing inactive metadata stays absent in the public model. It is not filled
with guessed positions, dispersion measures, resource labels or a physical zero.
Only missing F1–F3 receive the defined zero spin coefficients. This does not
claim to reproduce unrelated `checkAllSet` defaults for a terrestrial model.

| TIM input | Status and meaning |
| --- | --- |
| `FORMAT 1` | Required once, before every observation |
| `name frequency_MHz MJD uncertainty_us @` | Exactly five fields, at least one observation; all observations are eligible and preserved in original order |
| `MJD` | Complete finite decimal, parsed directly to Rug128; already barycentric TCB |
| Frequency and uncertainty | Finite nonnegative binary64 values with original decimal tokens retained; zero is accepted as no fit/weighting is performed |
| Other sites or `bat` alias | Rejected; the declared marker is exactly `@` |
| Extra flags or directives | Rejected, including pulse-number flags, INCLUDE, TIME, PHASE, MODE, EFAC/EQUAD and selection directives. Known directive names are reserved even in a five-token line |

Decimal syntax is signed decimal digits with an optional dot and E/e/D/d
exponent. At least one mantissa digit is required. Hexadecimal syntax,
underscores, trailing junk, NaN/infinity, overflow and parse underflow are errors.
Malformed-input errors include file, line and field; calculation errors include
the model parameter or observation index/name and TIM line.

## Calculation and source correspondence

| Rust code | Canonical TEMPO2 correspondence and preserved boundary |
| --- | --- |
| `src/longdouble.rs::parse_longdouble` | `longdouble.h` decimal-parsing entry point; fixed Rug128 and strict decimal grammar retained |
| `src/read_parfile.rs::{read_parfile, read_value}` | `readParfile.C`, including `readValue` at line 2762; the declared supported subset and error handling are unchanged |
| `src/tempo2_util.rs::{hms_turn, dms_turn, turn_deg}` | `tempo2Util.C:121-177`, preserving binary64 RA/DEC conversion order |
| `src/read_timfile.rs::read_timfile` and PAR conventions | `readTimfile.C:435-439`: `@` sets clockCorr=delayCorr=0. `initialise.C:117`, `preProcessSimple.C:88-123`: the accepted omitted-UNITS/EPHVER rule |
| BAT/BBAT values in `src/form_residuals.rs` | `formBats.C:52-53` assigns BAT=SAT; `secularMotion.C:51-65` has zero secular delay in this supported model and assigns BBAT=BAT |
| `src/form_residuals.rs::form_residuals` | `formResiduals.C:515-535`, then `:2098`: separate integer/fractional F0/day terms and F1–F3 accumulation |
| `tempo2_util::{fortran_mod, fortran_nlong}` and `phas1`/`nphase` | `formResiduals.C:2160-2208`, `tempo2Util.C:179-221`: first eligible TOA's signed fractional reduced phase; signed half addition then truncation |
| Results | `formResiduals.C:2330`, `:2349-2355`: residual/F0, restored integral phase, full pulse count |

The `@` branch bypasses the combined clock, TT-to-barycentric, Roemer, Shapiro,
tropospheric, interstellar and solar-wind delay correction used by `formBats`.
`calculate_bclt.C:80` and `dm_delays.C:52` show the no-delay branches. The legacy
wrapper still performs some unused resource reads and position work; the approved
Rust path omits those. Supplied secular-motion or binary parameters are rejected,
so these omitted calculations cannot silently become active.

For MJD `t`, PEPOCH `t0`, let `n=trunc(F0)`, `d=trunc(t)-trunc(t0)`,
`f=F0-n`, `u=(t-trunc(t))-(t0-trunc(t0))`, and `dt=(t-t0)*86400`.
The implementation follows the source's operation order:

```text
phase2 = ((n*u + d*f) + u*f) * 86400
arg = dt*dt
phase3 = (0.5*F1)*arg
arg *= dt; if F2 supplied: phase3 += (F2/6)*arg
arg *= dt; if F3 supplied: phase3 += (F3/24)*arg
phase5 = phase2 + phase3
phas1 = fortran_mod(phase5[first], 1)   # truncating quotient, signed fraction
phase5 -= phas1
nphase = fortran_nlong(phase5)          # signed half addition, then truncation
phaseint = binary64(n*d) * 86400.0      # source integer-product/binary64 boundary
phase = phaseint + phase5
pulse_n = checked_i64(phaseint + nphase)
residual_seconds = (phase5 - nphase) / F0
```

All extended arithmetic is nearest rounding at **128 significand bits**.
This is MPFR policy, not IEEE binary128. A borrowed Rug multiplication can fuse
when added to another Float; the third p2 product is therefore explicitly
completed before addition to preserve the source rounding boundary. No custom
transcendentals, selectable numeric backend, TDB conversion or fitting is used.

Integer F0, integer MJD, day difference and `n*d` must fit the source's signed
32-bit intermediates. Violations are returned errors rather than reproducing
undefined C/C++ casts or overflow. Within this range, the source's integral
binary64 product is exactly representable. Reduced and full pulse counts are
independently checked against signed 64-bit limits. Bounds are checked before
allocating a Rug Integer, including for very large finite input exponents.
The signed-remainder semantic contract uses Rug `%`; it does not reproduce
undefined out-of-range integer conversions inside legacy `fortran_mod`.

## Numerical agreement

The unchanged `barycentric-v1` comparator checks scientific configuration,
parameter flags/values, complete original observation identity, and required
stage fields. Engine provenance records Rust's actual implementation. The
adapter supplies source-default inactive configuration for these fixture inputs;
it does not claim that Rust applied those corrections or opened legacy resources.

For the aligned fixture, the test adapter computes `SAT -= residual/86400`
using Rug128, reloads those epochs through the same production parser, then adds
decimal `4e-9/86400` to every aligned epoch except the first and reevaluates.
The projection retains the original TOA identity throughout. There is no public
scenario option. These stage epochs agree with A2 within the adopted epoch gate.

The following maxima include all required stages (177 scalar stage comparisons,
39 projected parameter comparisons and independent exact identity/count checks;
16 residual-stage rows):

| Fixture | Max SAT/BAT/BBAT error (s) | Max phase error (turn) | Max residual error (s) | Max parameter relative error |
| --- | ---: | ---: | ---: | ---: |
| barycentric | 1.90087e-25 | 1.89247e-25 | 1.89247e-25 | 1.54852e-36 |
| barycentric_4ns | 1.90087e-25 | 1.89247e-25 | 1.89247e-25 | 1.54852e-36 |
| long_span | 2.76478e-25 | 4.97718e-23 | 3.76201e-25 | 3.47149e-35 |
| Adopted gate | 1e-12 | 1e-10 | 1e-12 | 1e-30, exact expected zero |

Observing frequency and uncertainty differences are zero for these fixtures,
inside their respective 1e-9 MHz and 1e-12 microsecond gates. Full pulse count,
reduced integer phase, parameter presence/fit flags, original tokens, ordering
and identity match exactly. No tolerance was changed. The tiny differences are
compatible with decimal input evaluation at 128 bits versus the accepted
113-bit reference and its 36-digit export; they are not a physical accuracy claim.

Independent diagnostics, separate from the adopted reference comparison:

| Diagnostic | Measured maximum | Diagnostic gate |
| --- | ---: | ---: |
| Nonzero F0–F3 phase vs 512-bit unsplit polynomial | 2.03184e-32 turn | 1e-27 turn |
| Same F3 residual comparison | 1.64579e-34 s | 1e-29 s |
| Aligned residual magnitude | 2.66241e-30 s | 1e-20 s |
| Perturbed residual minus 4 ns | 2.49010e-30 s | 1e-20 s |
| Signed 1/4 ns MJD arithmetic and serialization | All four cases below 2e-29 s | 2e-29 s |

The F3 oracle starts from the original decimal model/epochs at 512 bits and
evaluates the unsplit polynomial for four ordered epochs around PEPOCH, including
negative dt and a nonzero first reference. F3=2.4e-18 s^-4 is active. Counts are
exact. This is an independent arithmetic diagnostic, not a new TEMPO2 golden.
Signed half-pulse tests cover both sides and exact ties near zero and ±1e11
turns, end-to-end ±337.5 turns, and a negative first fractional reference.

## Failure behavior and verification result

All **nine Rust tests**, the copied Python comparisons, formatting and Clippy
pass; logs are saved. Tests cover missing files, malformed decimal epochs,
missing required parameters or units metadata, duplicate fields, unsupported
scientific parameters/directives/sites/flags, empty observations, invalid
uncertainties, nonpositive F0, source integer-range errors, and both reduced
and restored-full pulse-count overflow.

A failed `load` and failed `evaluate_barycentric` are followed by a valid
calculation in the same process. The valid result is byte-identical before and
after the failures, and failed evaluation leaves its input unchanged. CLI input
and evaluation failures produce nonzero status, contextual stderr and empty
stdout. A later valid CLI call produces complete JSON with decimal-string
extended values and lossless counts. There is no partially published library
result or stale result cache. OS-level write failure is a transport limitation,
as documented in the README.

During development, a metadata test initially expected nearest-integer phase
instead of the source's signed fractional reference; its expectation was
corrected. A huge-exponent guard test initially used a value beyond MPFR's
finite exponent range; it was moved inside that range to exercise the intended
integer-allocation guard. Both are resolved test-construction issues. Final
code and numerical verification contains no failing check.

A separate whole-index `git diff --cached --check` flags whitespace in the
unchanged copied reference `instrumentation.patch` and captured command logs,
including two pre-existing precision-study logs. These were preserved as evidence.
The scoped check of C's code, tests, manifest, script and documentation passes;
the artifact whitespace is not a Rust implementation or numerical failure.

## Limits and milestone disposition

Execution and numerical agreement cover **macOS arm64 only**. Linux, other
targets, release-package installation, cold build cost and whole-engine
performance are untested. Arbitrary PAR/TIM compatibility, TDB, terrestrial
corrections, binary models, higher derivatives, fitting, tracking, noise models,
derivatives/design matrices and plugins remain outside this crate's support.
Synthetic diagnostics do not establish physical accuracy over arbitrary models.

The existing DDH/DE414 ideal-fixture failure and missing-meteorology policy remain
outside C. They were not rerun, reclassified or repaired. The accepted reference
reports retain their original findings; no failure is hidden by fitting or a
relaxed comparison budget. GMP/MPFR remain retained numerical dependencies;
the legacy TEMPO2 engine is not a runtime dependency.

The declared C implementation criteria are demonstrated: strict owned input,
source-corresponding F0–F3 calculations, all three accepted fixture comparisons,
independent boundary/F3 tests, checked pulse counts, useful recoverable errors,
minimal library and JSON CLI, and reproducible artifacts. **Central review and
M1a acceptance remain open.** There is no request to release D or change central
scientific conventions, interfaces or tolerances.
