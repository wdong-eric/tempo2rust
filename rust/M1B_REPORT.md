# Assignment D — isolated PKS terrestrial timing

Delivered 20 October 2026 in `/Users/Eric.Dong/Downloads/tempo2rust/rust/`.
**Implementation and the adopted compatibility checks are complete. M1b
acceptance remains with the central chat.** No central decision, canonical
TEMPO2 source, A2 artifact, or precision-study file was changed. Existing staged
planning changes were preserved; this assignment did not stage or commit files.

The two malformed-record-grid failures from central decision 0005 are corrected.
Both exact reproducers now return contextual errors in debug and release; valid
fixture outputs remain byte-identical. The correction evidence and its limits
are recorded in the final section below.

The library and CLI now calculate the declared seven-TOA PKS case from
observatory-clock SAT through clocks, time transformations, ephemerides, site
geometry, all active propagation corrections, TCB BAT/BBAT, phase and residuals.
Timing orchestration and IF99 interpolation execute in Rust. Only the adopted
SOFA rotation routines and low-level JPL reader/interpolator remain native.

Both terrestrial fixtures pass every adopted gate. The largest BAT difference
is **5.68435e-14 s**, the largest residual difference **1.13687e-13 s**, and the
largest 4 ns response discrepancy **1.41688e-13 s**, all below their `1e-12 s`
gates. Full and reduced pulse counts and applied clock destinations agree
exactly. The three accepted M1a comparisons still pass their unchanged comparator.

## Delivered files and source correspondence

All paths below are relative to this report's `rust/` directory.

| Pinned TEMPO2 component | Delivered implementation and responsibility |
| --- | --- |
| `global.C::formBatsAll` | `src/form_bats.rs::form_bats_all` (public re-export `evaluate_terrestrial`): initial time pass, first ephemeris pass, site geometry, refined time pass, refined ephemeris pass, atmosphere/propagation, BAT/BBAT, then residuals |
| `observatory.C`, runtime loaders | `src/runtime.rs`: explicit owned context, PKS Cartesian/GRS80 geometry, resource identity, coverage and meteorology loading |
| `clkcorr.C`, `tabulatedfunction.C` | `src/clkcorr.rs`, `src/tabulated_function.rs`: directed route selection, badness/ties, cached coverage, sequential clock-step epochs and interpolation |
| `ifteph.C`, `tt2tdb.C::tt2tb` | `src/ifteph.rs`, `src/tt2tdb.rs::tt2tb`: owned endian-aware IF99 image, source-order Chebyshev values/derivatives, rough/refined TT–Teph and TT–TCB |
| `readEphemeris.C`, `jpleph.c` | `src/read_ephemeris.rs`: owned JPL handle, source-order reads/conversions, recoverable statuses; `native/jpleph.c` retains the interpolation kernel |
| `eop.C`, `get_obsCoord.C` | `src/eop.rs`, `src/get_obs_coord.rs`: leap-aware EOP interpolation and IAU2000B orchestration; SOFA supplies the matrix/vector routines |
| `tropo.C` | `src/tropo.rs`: normal weather selection, Niell mapping, hydrostatic/wet corrections and source-compatible fallback |
| `calculate_bclt.C`, `formBats.C` | `src/form_bats.rs`: frequency dilation, geometric delay, six Shapiro terms, interstellar and spherical solar-wind dispersion, source iteration and final BAT/BBAT assembly |
| `formResiduals.C` / existing M1a port | `src/form_residuals.rs`: shared existing spin/phase/count arithmetic now accepts computed BBAT; the barycentric entry point remains available |
| Input/public interface | `src/input.rs`, `src/read_timfile.rs`, `src/lib.rs`, `src/main.rs`: homogeneous PKS input, explicit terrestrial API/runtime option, owned results and JSON |
| Retained numerical dependency build | `build.rs`, `native/`: source closure, original licenses/hashes and the two documented C changes |
| Independent terrestrial comparison | `tests/terrestrial/{contract.json,manifest.json,compare.py,test_contract.py,fixtures/,outputs/}`, `examples/compare_terrestrial.rs` |
| Failure/fallback verification | `src/resource_tests.rs`, `examples/check_terrestrial_resources.rs`, existing `tests/behavior.rs` |
| Reproduction and evidence | `verify_m1b.py`, `verification/m1b/`, `README.md`, this report |

The calculations preserve the pinned constants, source expression ordering,
binary64 versus extended-precision boundaries, initial/refined passes and
100-iteration limit. Extended arithmetic remains the accepted fixed Rug128
backend. There is no C `long double`/binary128 FFI boundary.

The native closure contains 36 unchanged bundled SOFA Fortran routines, reached
from `iau_c2t00b`, `iau_pom00`, `iau_rxp`, `iau_sxp`, `iau_pxp`, `iau_trxpv`
and `iau_trxp`, plus `jpleph.c`, `jpleph.h` and `jpl_int.h`. Original hashes
are in `native/upstream_manifest.json`; changes are in `native/departures.patch`.
The normal build uses only this crate's sources, without linking TEMPO2,
`ifteph.C`, a C++ timing wrapper or the reference executable.

## Pinned identities and reproducibility

| Identity | Revision or SHA-256 |
| --- | --- |
| Accepted starting M1a | `49e122c3648e5cb91bae17233a062a444a10f9e4` |
| Canonical TEMPO2 | `fda6ad4a9753ad7dc5d291406f2f310a9161423f` |
| Accepted A2 reference | `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572` |
| Immutable `terrestrial` export | `c19aac714e38521c5dd309dbf50f815f49430171430206d7af969fc790043d83` |
| Immutable `terrestrial_4ns` export | `5455b5dc4d387ec8bea42a43665dffe0b3d7a39ced47e0f131395d003adb62eb` |

`tests/terrestrial/manifest.json` records the copied fixtures and all 80 pinned
A2 runtime identities. `verification/m1b/source_hashes.json` records the
delivered implementation, native dependencies, tests, examples and verification
script. The existing `tests/reference/` goldens, schema and comparator are
unchanged. The new comparator declares `terrestrial-v1`; it never relabels
terrestrial data as `barycentric-v1`.

Executed platform: **aarch64-apple-darwin, Rust/Cargo 1.97.1, LLVM 22.1.6,
GCC/GFortran 16.2.0**. Rug 1.30.0 and gmp-mpfr-sys 1.7.1 remain locked.
`verification/m1b/{rustc,cargo,cc,fc,dependencies}.log` preserve exact versions.
Native compilation uses `-O2`; SOFA additionally uses
`-fno-second-underscore -frecursive`. `CC` and `FC` may override the compiler
commands, but other compiler/platform combinations have not been validated.

Reproduce all adopted checks using the delivered dependency cache:

```sh
cd /Users/Eric.Dong/Downloads/tempo2rust/rust
export CARGO_HOME="$PWD/.cargo-home"
export GMP_MPFR_SYS_CACHE="$PWD/target/gmp-cache"
python3 verify_m1b.py --runtime ../tempo2/T2runtime
```

A fresh cache first needs `cargo fetch --locked` and the documented C/Fortran
toolchains. The verifier checks all 80 pinned runtime hashes before running,
uses offline locked Cargo operations, and writes its evidence only beneath
`rust/`. Large ephemerides/runtime files remain explicit external inputs.

Public CLI:

```sh
cargo run --locked --offline --bin tempo2-rust -- residuals \
  --par tests/terrestrial/fixtures/test1.par \
  --tim tests/terrestrial/fixtures/test2.tim \
  --runtime ../tempo2/T2runtime
```

Public library:

```rust
let input = tempo2_rust::load(par_path, tim_path)?;
let mut runtime = tempo2_rust::RuntimeContext::new(runtime_path)?;
let result = tempo2_rust::evaluate_terrestrial(&input, &mut runtime)?;
```

`load` and `evaluate_barycentric` remain usable without runtime data. Evaluation
borrows the immutable input; an owned terrestrial result is returned only after
every TOA and the residual stage succeed. The CLI completes calculation and
serialization before writing stdout; warnings use stderr. This does not promise
atomic delivery if an OS-level stdout write fails.

## Supported configuration and resource provenance

The declared profile is an isolated pulsar with homogeneous literal `pks`
(case-insensitive) FORMAT 1 observations. Common M1a PAR requirements remain:
PSRJ, positive F0, PEPOCH and the adopted TCB convention; omitted F1–F3 are
zero with `set=false`. Terrestrial evaluation additionally requires RAJ, DECJ,
DM, POSEPOCH, DMEPOCH, `CLK TT(TAI)` and `EPHEM DE405`. Fit flags are retained
as metadata; no fit is performed.

The profile fixes IF99, IAU2000B, frequency dilation, solar/planetary Shapiro,
interstellar DM, IPM=1, NE_SW=4, SWM=0 and enabled troposphere. SAT remains
observatory-clock MJD; BAT/BBAT and model epochs are TCB. Original input order
and tokens are retained. The first TOA supplies the signed fractional phase
reference; there is no mean subtraction or TZR reference activation.

Aliases such as `7`, mixed `@`/PKS input, other sites, geodetic observatory-file
coordinates, alternative ephemerides/time conventions, proper motion/parallax,
binaries, fitting, glitches, noise, tracking, higher spin derivatives and
unrecognized active parameters are outside this profile and rejected.

The public result adds per-TOA corrections, ordered TT/UTC clock chains,
effective weather and fallback reasons, resource hashes/sizes/ranges, runtime
root, warnings and retained dependency identities. Keep runtime files unchanged
while a context is alive; create a new context to adopt changed resources.

The accepted run reports 79 resources: 62 indexed unused clocks, 10 applied
clock files, two observatory definition files, two weather tables, IF99, DE405
and EOP. A2's 80th resource is its alias file, which the literal-PKS path does
not read or claim to use. The comparator enforces the 16 required identities:
main observatory definition, EOP, DE405, IF99, both PKS weather tables, and the
ten applied clocks. Other scans are provenance rather than cross-engine
scientific equality requirements.

| Runtime data | Recorded MJD range / role |
| --- | --- |
| `ephemeris/DE405.1950.2050` | 33264–69808; planetary states |
| `ephemeris/TIMEEPH_short.te405` | 38736–69808; IF99 time ephemeris |
| `earth/eopc04_IAU2000.62-now` | 39491–61163 samples; interpolation requires a following sample, so the final epoch is excluded |
| `atmosphere/pks.sap`, `pks.zwd` | 53000–53001; every adopted fixture epoch is outside coverage |
| `pks2tid.clk`, `tid2nist.clk` | 0–50844.72917 |
| `nist2utc.clk` | 45989–58599 |
| `pks2gps.clk` | 0.00521–60644.08347 |
| `gps2gpst.clk` | 48988–55559 |
| `gpst2utc.clk` | 48988–99999 |
| `gps2gps_usno.clk`, `gps_usno2utc.clk` | 55559–99999 |
| `utc2tai.clk` | 37300–99999 |
| `tai2tt_tai.clk` | 0–1000000 |

Clock paths, in original zero-based TOA order:

| TOAs | Applied TT chain |
| --- | --- |
| 0–3, around MJD 50000 | UTC(PKS) → UTC(TID) → UTC(NIST) → UTC → TAI → TT(TAI) |
| 4, MJD 54000 | UTC(PKS) → UTC(GPS) → UTC(GPST) → UTC → TAI → TT(TAI) |
| 5–6, around MJD 56000 | UTC(PKS) → UTC(GPS) → UTC(GPS_USNO) → UTC → TAI → TT(TAI) |

`verification/m1b/cli-equivalent.json` records every applied step/value and
resource identity. Clock route caching, float32 badness/ties, lexical file
ordering, inclusive route coverage, sequential step epochs and the source's
fallback from an unavailable site clock to UTC are retained.

Normal meteorology handling is active, with one calculation path. Missing,
out-of-range or zero-valued data use standard pressure `101.325 kPa` and zero
zenith wet delay with TROP1/TROP2 warnings and the actual reason. Missing
weather is not fatal and requires no mode or opt-in. Valid table data are
interpolated and applied. These assumptions reproduce TEMPO2; the fixtures do
not establish historical weather accuracy.

## Numerical agreement

Signed differences below mean **Rust minus A2**. The table gives maxima over
both fixtures, including the shifted diagnostic stages. TOA indices are
zero-based; the full evidence also preserves original identities/tokens.

| Quantity / representative maximum field | Signed difference | Unit | Gate | Stage / TOA |
| --- | ---: | --- | ---: | --- |
| BAT/BBAT epoch | −5.68434188545e-14 | s | 1e-12 | bats / 4 |
| TT–TB | +1.29223e-33 | s | 1e-12 | offset_4ns / 6 |
| Earth/SSB component 0 | −4.878e-34 | light-second | 1e-12 | slope_plus / 3 |
| Roemer | −5.68434188608e-14 | s | 1e-12 | bats / 0 |
| Largest stored Shapiro difference (Sun) | −1.69406589451e-20 | s | 1e-12 | slope_minus / 6 |
| Troposphere | +4.96308367532e-24 | s | 1e-12 | offset_4ns / 6 |
| Solar dispersion | +1.69406589451e-20 | s | 1e-12 | offset_4ns / 2 |
| Full phase | +1.13686836941e-13 | turns | 1e-10 | residuals / 1 |
| Residual | +1.13686836941e-13 | s | 1e-12 | residuals / 1 |
| SSB frequency | 0 | Hz | 1e-5 | all |

Clock steps/total, TT–Teph, interstellar dispersion, observatory vectors,
combined BAT correction and all other declared fields also pass; their
individual maxima are in `verification/m1b/numerical_errors.json`. Original
frequency uses `1e-9 MHz`, uncertainty `1e-12 microsecond`, and extended
parameters `1e-30` relative with expected zero exact. No gate was relaxed.
Very small differences below binary64 resolution arise from decimal export
representations. The largest Roemer difference is one binary64 ULP at these
values; a compiler-specific instruction-level cause has not been established.

Representative unshifted results:

| TOA | BAT difference (s) | Residual difference (s) |
| --- | ---: | ---: |
| 0 | −5.68434180868e-14 | 0 |
| 1 | +5.68434188543e-14 | +1.13686836941e-13 |
| 4 | −5.68434188545e-14 | −7.67665637578e-22 |
| 6 | +5.68434188078e-14 | +1.13686836895e-13 |

The comparator checks 490 scalars for `terrestrial` and 1,162 for
`terrestrial_4ns`. It independently checks exact configuration, parameter
presence/fit flags, observation identity/order/tokens, sites, correction flags,
clock destinations, required resource hashes and full/reduced pulse counts.
It compares only lifecycle-defined quantities at parsed, bats, residuals,
offset_4ns, slope_minus and slope_plus stages. Original observation identity
remains unchanged in shifted evaluations.

For each TOA the separate response check uses
`[BAT(SAT+1 ms) − BAT(SAT−1 ms)] / 0.002 s` as the local derivative, then
compares the 4 ns BAT response with that derivative times `4e-9 s`. The maximum
discrepancy is at TOA 5: derivative `0.99996457817099508052951531080000`,
response `4.00000000021758490039862400e-9 s`, signed discrepancy
`+1.416875336045782805627568e-13 s`, gate `1e-12 s`. A unit derivative was not
assumed.

`verification/m1b/{terrestrial,terrestrial_4ns}_signed_errors.json` retains
every field difference with stage, TOA, unit and gate. There are no failing
TOAs. Comparator mutation checks pass 92 cases per fixture (184 total):
in-budget numeric perturbations pass; out-of-budget values, changed counts,
configuration/resources/flags and missing stages fail.

## Intentional departures and focused evidence

These changes implement the already authorized owned-state, initialized-state,
recoverable-error and invalid-access contracts. They do not change accepted
scientific coefficients or relax comparison budgets. Local comments explain
the source behavior and intended difference.

| Departure and concrete reason | Adjacent comment | Regression / evidence |
| --- | --- | --- |
| Context-owned clock and IF99 state replaces process-global flags, files and caches; failed or independent evaluations must not inherit partial state | `src/clkcorr.rs:89`, `src/ifteph.rs:33` | Clock/IF99 synthetic recovery tests; public context recovery and repeated baseline equality |
| Initial geometry vectors are zeroed for each evaluation; legacy repeat calls can inherit vectors from a previous call | `src/form_bats.rs:111` | Both stage goldens and ±1 ms/4 ns diagnostics; failed then valid evaluation matches a fresh result |
| Resource I/O/parser failures return contextual errors instead of process exit, ignored conversion failures or partial results | `src/runtime.rs:27`, `src/ifteph.rs:33`, `src/read_ephemeris.rs:27` | Missing and malformed observatory, DE405, IF99, EOP and indispensable clock files each followed by restored success |
| Malformed, nonfinite or decreasing clock/weather rows are rejected instead of skipped/warned through; singleton/degenerate brackets cannot index absent samples or divide by zero. Valid interior duplicate clock epochs remain supported | `src/tabulated_function.rs:34`, `src/tabulated_function.rs:42` | `table_steps_endpoints_and_invalid_data` |
| Invalid negative/nonfinite clock badness is rejected because Dijkstra requires finite nonnegative weights and can otherwise construct cyclic/invalid paths | `src/clkcorr.rs:71` | `clock_routes_coverage_ties_fallback_and_recovery` |
| Failure to find even an indispensable UTC→TT chain is an error instead of an empty apparent-success chain; the source's site→UTC fallback remains | `src/clkcorr.rs:245` | Clock synthetic fallback/recovery test; missing `utc2tai.clk` public check |
| Malformed/unordered/insufficient EOP data fail before interpolation. At/after the final sample, `get_EOP` would read `samp[size]` for its derivative; return an error. The lower-end proxy and warning remain | `src/eop.rs:42`, `src/eop.rs:64`, `src/eop.rs:92` | `eop_upper_boundary_recovers_and_leap_is_not_interpolated`; public malformed-resource recovery |
| IF99 owns a validated immutable image and checks records, coefficients and upper coverage instead of unchecked seeks/reads and process exits | `src/ifteph.rs:33`, `src/ifteph.rs:133` | `ifte_endianness_derivative_boundaries_and_recovery`, both endiannesses and analytic derivative; full pinned fixture |
| DE405 preflight protects native fixed polynomial arrays, offsets and unterminated title parsing rather than allowing asserts/invalid access | `src/read_ephemeris.rs:73`, `src/read_ephemeris.rs:79` | `native_read_failure_recovers_without_stale_cached_record`, malformed-header cases |
| DE405 grids now bound the native record conversion and u32/C-long seek arithmetic; nonfinite epochs are rejected before the FFI, preventing the decision 0005 SIGBUS | `src/read_ephemeris.rs:132`, `src/read_ephemeris.rs:178` | `jpl_record_grid_bounds_and_endpoints_recover`; exact pinned-file mutation through library/CLI in both builds |
| IF99 grids must match the owned complete records; record/subinterval conversion and offset arithmetic are bounded before indexing, preventing the decision 0005 overflow panic | `src/ifteph.rs:86`, `src/ifteph.rs:141` | `ifte_record_grid_rejects_impossible_steps_and_recovers`; exact pinned-file mutation through library/CLI in both builds |
| Native initialization/error retrieval is serialized because upstream uses one global error slot; the handle itself has an owned lifetime | `src/read_ephemeris.rs:27` | Independent handles and failure/recovery tests; no parallel stress-test claim |
| Unused pre-DE430 header slots are zero-initialized before native swapping/copying, replacing reads of uninitialized stack state | `native/jpleph.c:710` | Native DE405/synthetic initialization and independent-context test; stage goldens; recorded patch |
| JPL statuses are checked instead of publishing stale vectors after failed reads/range errors. Failed reads invalidate the record cache, and success commits it only after complete I/O | `src/read_ephemeris.rs:199`, `native/jpleph.c:594`, `native/jpleph.c:603` | Native repeated-short-read/range/recovery test; direct original-versus-retained reproducer below |
| Invalid GRS80 geometry, zero zenith norm and nonfinite propagation are returned as errors rather than complete-looking NaN output | `src/runtime.rs:161`, `src/form_bats.rs:240` | Public zero-coordinate resource rejection; overflowing DM error then valid reuse in `check_terrestrial_resources` |
| Fortran locals use `-frecursive` to avoid mutable local storage shared across independent contexts | `build.rs:29` | Numerical stage gates for compiled closure; concurrency performance/stress testing remains unclaimed |
| Literal-PKS scope does not load aliases or claim that unused alias metadata was applied | `src/runtime.rs:91` | Explicit site parser rejection and exact required-resource/clock comparison; no alias-support claim |
| A former M1a negative test used PKS as an unsupported site; it now uses an actually unsupported site because PKS is the new declared input | `tests/behavior.rs` beside that case | Existing behavioral suite; all M1a numerical artifacts/comparator remain unchanged |

The direct native reproducer copies pinned DE405 into owned scratch space,
truncates its last 4,000 bytes, then requests a valid epoch, the same unreadable
epoch twice and the valid epoch again. Original JPL returns
`0, -2, 0, 0`: the second failed record read falsely succeeds. The retained
reader returns `0, -2, -2, 0`; the recovered position equals the initial one.
`verification/m1b/native_source_probe.json` preserves the complete probe source,
construction instructions, exact compiler commands, both source hashes and
outputs. The canonical source was compiled read-only and not patched.

Two unusual source expressions were deliberately preserved: `tt2tb`'s second
scaling of `obsTerm` rather than `obsTermDot`, and `NMF_wet`'s use of `b[j]`
inside its `c` interpolation. Comments are beside them in `src/tt2tdb.rs` and
`src/tropo.rs`. Neither is silently scientifically corrected. A focused Niell
test uses values generated from the unchanged pinned C functions:
hydrostatic `6.3735950766954428`, wet `6.5370360886066781` for MJD 50000,
latitude −0.575959 rad, height 415 m and elevation 0.15 rad, checked to `2e-14`.
This is source-compatibility evidence, not independent atmospheric validation.

## Verification, limits and return to central review

The corrected run passes formatting, all 18 Rust tests, Clippy with warnings
denied, both terrestrial stage contracts, 184 comparator mutations, the 4 ns
response check, public resource recovery, eight original CLI failure cases and a
subsequent successful CLI call with weather warnings on stderr. CLI failures
publish no JSON stdout. A failure after one successfully processed TOA returns
no partial library result and leaves the borrowed input unchanged.
The two new malformed-grid CLI cases and same-process library recovery also
pass in debug and release, alongside all eight resource unit tests in release.

Weather tests separately cover valid interpolation, zero values, absent files
and out-of-range data. Public missing/zero weather runs reproduce baseline
residuals while retaining different truthful fallback reasons; nonzero valid
tables change the atmospheric correction as expected.

Evidence is recorded in `verification/m1b/{commands.json,tests.log,clippy.log,
resource_checks.json,recovery.json,cli_checks.json,numerical_errors.json}` and
the actual exports, signed errors and mutation logs alongside them. These are
implementation and pinned numerical-compatibility checks, not validation of
all physical models or historical meteorology.

There are no outstanding failures within the released profile. The known
broader DDH discrepancy was neither rerun nor addressed. Linux execution,
additional observatories/conventions, binary timing, fitting/design matrices,
performance and distribution packaging remain outside this assignment. The
historical SOFA license is preserved and restricts commercial use; packaging
requires its own review. No broad parallel-execution validation is claimed.

The next fitting milestone can consume the owned deterministic forward result
and provenance, but still needs its own parameter-variation, derivative,
weighting, fit lifecycle and broader-model contracts.

All Assignment D implementation criteria are satisfied for the adopted
isolated PKS fixtures. No reference-exporter change, tolerance change or new
scientific convention was required. The recommended central action is to
review this evidence and accept M1b for that declared scope. **Central
acceptance and the next milestone's release remain open.**

## Naming pass — 20 October 2026

Reviewed the Rust names against the pinned TEMPO2 source and applied the user's
requested naming convention. Source-corresponding routines use Rust snake_case;
ambiguous shorthand is expanded. Public API signatures, exported types, CLI
options, JSON field names, numerical operations/order, native sources, resource
policy and error behavior are unchanged.

| Original TEMPO2 name or role | Rust name after this pass |
| --- | --- |
| `formBatsAll` and the supported residual operation | `form_bats::form_bats_all`, directly re-exported as the existing `evaluate_terrestrial`; no wrapper or extra execution path |
| `ClockCorrectionFunction`, `ClockSequence` | Same type names, owned by `ClockCorrections`; `clock_from`, `clock_to`, `start_mjd`, `end_mjd`, `function_indices` |
| `getClockCorrections`, `getClockCorrectionSequenceInternal` | `get_clock_corrections`, `get_clock_correction_sequence` |
| `TabulatedFunction_getValue`, `load_EOP`, `get_EOP` | `TabulatedFunction::get_value`, `Eop::load_eop`, `Eop::get_eop` |
| `getObservatory`, `MeteorologyFunction`, `getMeteorologicalValue` | `get_observatory`, `MeteorologyFunction`, `get_meteorological_value`; the last retains the already combined lookup/fallback implementation |
| `compute_tropospheric_delays`, `NMF_hydrostatic`, `NMF_wet` | `compute_tropospheric_delay` for one TOA, `nmf_hydrostatic`, `nmf_wet` |
| `tt2tdb.C::tt2tb` | `tt2tdb.rs::tt2tb`; corrects the earlier report's mistaken source filename `tt2tb.C` |
| `correctionTT_Teph`, `correctionTT_TB`, `einsteinRate` | `correction_tt_teph`, `correction_tt_tb`, `einstein_rate`, replacing `teph`, `tb`, `rate` |
| `sun_ssb`, `earth_ssb`, planet/Earth and observatory/Earth states | `sun_ssb`, `earth_ssb`, `planets_earth`, `observatory_earth`, `site_velocity` |
| `rca`, `rsa`, `vobs`, `tdis1`, `tdis2` | Descriptive `observatory_ssb`, `observatory_sun`, `observatory_velocity_ssb`, `dispersion`, `solar_dispersion`; ambiguous source shorthand is not copied |
| `IFTE_init`, `IFTE_get_Vals` | `IfteEphemeris::init`, `IfteEphemeris::get_values`; unchanged constants retain their `IFTE_` prefixes |
| IFTE `ncf`, `ncm`, `na`, `pc`, `vc`, `ipt` | `coefficients_per_component`, `component_count`, `subinterval_count`, `chebyshev_values`, `chebyshev_derivatives`, documented `coefficient_layout` |
| `jpl_pleph` wrapper | `JplEphemeris::interpolate_state`; native ABI names are unchanged |
| `tempo2Util.C::dotproduct` | `tempo2_util::dot_product`, moved from the generic `form_bats::dot` location |
| IFTE epoch splitting | Existing helper moved from `get_obs_coord` to `ifteph::split_ifte_epoch` |

M1a's established source-corresponding reader/residual names remain. Standard
astronomical names (IFTE, EOP, SAT, BAT, TCB, RA/Dec) and short mathematical
indices/coefficient names remain where their meaning is clear.

The full `verify_m1b.py` workflow was rerun after the naming pass. All 16 Rust
tests, both terrestrial contracts, 184 comparator mutations, recovery checks,
CLI checks, unchanged M1a comparisons, formatting and Clippy pass. The ten
snapshotted numerical/provenance outputs are byte-identical: both terrestrial
exports, public CLI-equivalent result, numerical maxima, both signed-error
files, recovery result and three M1a projections.
`verification/m1b/naming_equivalence.json` records their before/after hashes;
`source_hashes.json` identifies the final source names. No new scientific or
behavioral departure was introduced.

## Decision 0005 correction — 20 October 2026

Central review found two failures despite passing valid-data comparisons:
changing only the pinned DE405 big-endian step at byte 2668 to `1e-300`
caused native SIGBUS, and changing IF99's step at byte 280 to the same value
caused a Rust integer-overflow panic. The earlier malformed-header tests did
not cover these valid-looking headers with impossible grids. This correction
closes those two demonstrated gaps; it does not claim exhaustive malformed-file
or platform coverage.

`src/read_ephemeris.rs::JplEphemeris::open` now requires a finite positive
integral data-record count. Its maximum is derived from the native record size,
`u32` multiplication in `(nr + 2) * recsize`, and the platform's C `long`
seek-offset limit. This bounds the entire declared grid before native
initialization/evaluation, including the final record's start offset. The
wrapper also rejects nonfinite interpolation epochs before the FFI; NaN would
otherwise bypass the native range comparisons. Valid record selection,
including the source's preceding-record choice at exact boundaries, remains
inside unchanged native code. Short files still reach the native checked-read
path, preserving the approved repeated-short-read/cache-recovery regression.

`src/ifteph.rs::IfteEphemeris::init` now checks that the finite positive integral
header grid matches the complete records in its owned immutable image.
`get_values` bounds the floating record and subinterval numbers before integer
conversion, uses checked record-index addition and coefficient-offset
arithmetic, and retains the source's final-epoch rule and interpolation
expressions. Invalid metadata returns `TimingError`; there is no panic catching,
clipping, replacement resource, subprocess workaround or new fallback mode.

| Regression/evidence | Result |
| --- | --- |
| `ifte_record_grid_rejects_impossible_steps_and_recovers` | Both byte orders; tiny steps, nonintegral/inconsistent grids, restored final-endpoint values |
| `jpl_record_grid_bounds_and_endpoints_recover` | Tiny/nonintegral grids, integral grid exceeding native byte-offset limits, nonfinite epochs, valid initial/internal/final endpoints and restored states |
| Existing `native_read_failure_recovers_without_stale_cached_record` | Repeated native read errors and later successful recovery still pass |
| All Rust tests, debug | 18 passed |
| Resource unit tests, release | 8 passed, including both new regressions and the native cache regression |
| Exact pinned-file mutations, public library, debug and release | Both rejected during context construction with the resource path, `JPL`/`IF99` field and `record grid` message; subsequent valid evaluation in the same host process with a new context reproduces the entire baseline result |
| Exact pinned-file mutations, public CLI, debug and release | Both exit 1, empty stdout, contextual stderr; restored CLI stdout is byte-identical to baseline |
| Full adopted verification | Both terrestrial fixtures, 184 mutations, 4 ns response, resource/weather/CLI recovery, all M1a comparisons, formatting and Clippy pass |
| Valid-data equivalence | All ten reviewed numerical/provenance outputs unchanged; four release terrestrial/result/recovery exports byte-identical to debug |

`examples/check_terrestrial_resources.rs` reproduces both exact byte mutations
in disposable copies of the pinned runtime. Its library checks execute directly
in one process; launching the normal CLI is an additional interface check, not
isolation of the library failure. Construction failures are ordinary `Err`
values, so the executable continues to restore resources and evaluate again.

Reproduce the complete corrected delivery:

```sh
cd /Users/Eric.Dong/Downloads/tempo2rust/rust
export CARGO_HOME="$PWD/.cargo-home"
export GMP_MPFR_SYS_CACHE="$PWD/target/gmp-cache"
python3 verify_m1b.py --runtime ../tempo2/T2runtime
```

The verifier now records both build modes. Evidence is in
`verification/m1b/resource_checks.json`,
`verification/m1b/resource_checks_release.json`,
`verification/m1b/release_resource_tests.log`,
`verification/m1b/release_equivalence.json` and the command/build logs.
`verification/m1b/resource_correction_equivalence.json` records the ten
before/after reviewed-output hashes; `source_hashes.json` identifies the
corrected implementation and tests. The maximum BAT, residual and 4 ns response
differences remain `5.684341885448086137600e-14 s`,
`1.13686836941064559950026880243e-13 s` and
`1.416875336045782805627568e-13 s`, respectively.

The retained native source and its already approved cache/initialization patch
are unchanged. Central documents, the staged index, both independent source
repositories, runtime resources, immutable goldens and acceptance budgets were
preserved. This delivery is identified by the source-manifest/report hashes
returned to central review, without a new commit.

No issue remains from the two bounded correction requests. M1b acceptance is
still central; Linux, broader observatories/models, binaries/DDH, fitting,
performance, concurrency stress testing and distribution packaging remain
outside verified scope. The broader DDH discrepancy remains open.
