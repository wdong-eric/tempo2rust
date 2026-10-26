# Assignment AH: core CLI and stateful output handover

27 October 2026. AH delivers the native-style command entry point, the complete
assigned report/helper/display transfer and public formatting/logging APIs on
the canonical Rust engine. This is an implementation candidate for central
review under 0074/0080/0091. **Numerical acceptance remains pending:** each final
debug/release matrix has 25 exact comparison groups and 12 failing groups, and
the checker exits 1. No failed group is excluded or relabelled a pass.

## Revisions and entry points

| Role | Commit |
| --- | --- |
| Released AH baseline | `732fd0ea30de9b22f566065b26dd7642f536d7ea` |
| Contract freeze | `5ff9ae25f069a9cd28e3eaf3963f5b47236493bc` |
| AH production/API/source map | `0dbcf0842394eb9cfaf93cb31bfac428e2ef7d67` |
| AH validation sources | `6ef441a1a46a964770352bebdc3395ef12045c10` |
| Canonical TEMPO2 pin | `fda6ad4a9753ad7dc5d291406f2f310a9161423f` |
| Independent reference checkout | `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572` |

The reviewed predecessor is AG production `87099caf24089a9ca28091bb3072654adca9f040`,
validation `cf4dfeb8ceb21e335193797e8c8ef6f3ebea5ba6` and report
`6fcf5aac79ae490d15b049f1ea1fccfe53f3e9da`. Central documentation commits made
during AH are preserved and are not AH implementation changes.

The callable interfaces, native command examples and stream/destination contract
are in [CORE_OUTPUT_API.md](CORE_OUTPUT_API.md). The full routine, branch,
type, state, unsafe and departure map is in
[CORE_OUTPUT_SOURCE_MAP.md](CORE_OUTPUT_SOURCE_MAP.md). Executable replay commands
and native-build requirements are in
[the validation README](../validation/core_output/README.md); the comparison
rules are in [CONTRACT.md](../validation/core_output/CONTRACT.md).

`core_cli::run` receives argv including the program name, an explicit environment
snapshot and borrowed output/input streams. The executable dispatches native
root commands while retaining the existing explicit JSON subcommands. It reads
and preprocesses once, writes `out.tim` before fitting when requested, and uses
one `general_fitting::t2_fit` at each source loop point. Each outer NITS pass
performs BAT/residual evaluation, the fit, the second evaluation and stateful
output. `-pre` still fits and skips only the second evaluation. Later passes
copy the current values/errors into prefit state at the source point.

Native predictor commands reuse AF's PAR-only preparation and generation APIs.
Native defaults, odd Cheby dimension adjustment, seconds-to-days conversion,
overlap/order, literal filename prefixes and tempo1 warning placement are
preserved. Native acceleration defaults to 1; existing JSON defaults remain 0.
Help/version/no-argument native exits remain 1. Plugin discovery is implemented;
plugin execution remains explicitly unavailable. Owned allocation replaces the
source fixed-capacity arrays. The library does not change cwd or environment.

`core_output::text_output` operates on the mutable canonical context. The six
`textOutput.C` bodies, display routine and file/parameter mutations retain source
ordering. Direct APIs expose `calc_rms`, `m2`, `dglep`, `print_glitch`,
`display_parameters` and the source `record_precision` no-op. Empty PAR filenames
consume a token from the supplied input stream. Completed writes and state
changes survive later I/O errors, as at the source call points.

Typed `source_format` APIs preserve Rug timing values until an explicit source
cast, support the source numeric/string formatting boundaries and return zero
on successful `ld_*` calls. Logging extends the existing diagnostics owner,
including first-30 storage, full counters, summary/reset behavior, source
truncation and terminal-dependent ANSI handling. Canonical enum ordering and
the two byte-identical embedded help files supply generated-header/help duties.
No new numerical dependency, precision mode or alternate engine was introduced.

## Corrections and their evidence

The source map identifies the exact original sites and adjacent explanations.
The following changes are bounded source/implementation corrections, with
original behavior retained in ignored evidence rather than treated as goldens
when its state is undefined.

| Demonstrated cause | Delivered correction and evidence |
| --- | --- |
| `calcRMS` uses pulsar zero's cutoffs for another pulsar | Read the current pulsar's cutoffs. The two-pulsar witness changes only the affected owner; first-pulsar and other helper results remain equal. |
| Unweighted CHI2R reads an unassigned local | Use the current fit's `fitChisq`; original defined-poison and corrected outputs, plus a real unweighted workflow, are retained. |
| First activation of START/FINISH, default EPHVER, `-last`, TZR/TRES leaves prefit metadata unwritten | Snapshot defined prior values/errors at activation; preserve supplied PAR metadata. Poison witnesses distinguish the original missing write from the corrected snapshot. |
| Nonglobal `globalNfit` has no producer | Reset at prefit, allow the global fit to overwrite it, and guard global covariance by `nGlobal`. Poison and actual quadrupole/global output witnesses cover the consumers. |
| Existing Rust calculated DM/CM weights for an empty constraint matrix | Use the source nonempty-matrix condition. The resulting DM/CM files and weights match native exactly. |
| Modern terrestrial observation coordinates use clock/EOP components but omit `correctionUT1` | Store the actually applied site-to-UTC seconds plus the existing EOP UT1-UTC value, once in binary64, then promote to timing storage. No new lookup, model or fallback. |
| Formatter varargs cleanup/empty-conversion scan and unsafe argument/buffer/shell behavior | Typed owned formatting, bounded argument/alias errors and byte-preserving suffix pair discovery. Narrow native corrections and bounded original failure prefixes/statuses remain recorded. |

The UT1 correction was made at the canonical producer after central's focused
review. Clock modes 0/2 apply no site-to-UTC correction, so their first addend is
zero; mode 1 retains the correction already used by the existing timing path.
The existing JSON `ut1` field is now initialized on this modern terrestrial
path. Its schema and other values are unchanged. Barycentric/satellite paths
without that producer receive no invented value.

`ut1-check-v1` passes all 21 checks across modes 0/1/2. The 48 before/after Rust
observation rows are identical after removing only the newly initialized UT1
field. Native SAT/BAT/BBAT/PET/residual/phase and site geometry are unchanged;
the new native and Rust UT1 values exactly match an independent composition of
the applied clock and EOP components. `ut1-before` is the retained pre-edit
snapshot, not a snapshot regenerated from current code. The native original
producer retains defined poison, proving the missing write.

AH's earlier covariance AST classification and omitted diagnostic formatting
were implementation defects corrected before delivery. Their failed historical
outputs remain available; they are not classified as upstream bugs. Central
also caught an outdated AF dependency selection and the checker's failure to
propagate comparison failures. The final builder uses accepted
`predictors/build/native-corrected`, including the minimum-width Doppler reader
fix. The final checker enforces statuses, input stability, file membership and
bytes; its control tests exercise actual nonzero subprocess results.

## Executed verification

All paths in this table are relative to `validation/core_output/build/`.

| Check | Actual final result | Evidence |
| --- | --- | --- |
| Debug library suite, serial | 91 passed, 0 failed, 36 ignored | `lib-debug-ut1.log` |
| Release library suite, serial | 91 passed, 0 failed, 36 ignored | `lib-release-ut1.log` |
| Debug behavior and analysis-tools integration | 7 + 3 passed | `integration-debug.log` |
| Release behavior and analysis-tools integration | 7 + 3 passed | `integration-release.log` |
| Clippy, all targets, warnings denied | Passed | `clippy-ut1.log` |
| Cargo formatting check | Passed | `fmt-delivery.log` |
| Python compilation of validation sources | Passed; final translator whitespace edit rechecked | Executed `py_compile` |
| Checker controls | 3 tests passed, including omitted/reordered records, changed units/flags/digits, missing files, unexpected exit/timeout and subsequent valid recovery | `checker-tests-final.log` |
| Bounded source-bug witnesses | 17 checks passed | `bugs-v2/receipt.json` |
| UT1 correction and unchanged-state witnesses | 21 checks passed | `ut1-check-v1/receipt.json` |
| Debug final native/Rust matrix | 37 groups: 25 exact, 12 failing; actual exit 1 | `debug-final/receipt.json` |
| Release final native/Rust matrix | 37 groups: 25 exact, 12 failing; actual exit 1 | `release-final/receipt.json` |
| Static source accounting | Six bodies and 375 decision/loop/case sites | `source-inventory.json` |

Each matrix contains 32 paired real workflows plus five direct comparison
groups. Its receipt contains 78 execution entries and one source/runtime
identity entry. Both final summaries report 17 differing files, three missing
files, zero extra files, zero unexpected statuses and zero changed inputs.
Known unwritten-state error statuses are recorded explicitly and do not turn
their differing/missing output files into passes. Final live source/runtime,
checker, contract and executable hashes were checked against both receipts.

The five exact direct groups cover extended formatting (70 format/value rows
plus two mixed rows), derived helpers, redirected logging, a real stdout PTY,
and the identical-state report snapshot. All six snapshot products match:
stdout, stderr, `snapshot.par`, `residuals.dat`, `global_covar.cvm` and `state.tsv`.
The Rust direct-API test invoked by the runner additionally checks repeated
stateful reports, a failing stdout followed by successful output on the same
context, the supplied-input filename prompt and actual fitted-PAR reload.

The exact real workflows include robust/unweighted fits, NITS and NITS with
`-pre`, writetim, parameter prior, DM/band/chromatic noise, T2, jumps/FD jumps,
glitches, IFUNC, clock offsets, quadrupole interpolation, GW memory, fitted WAVE
and fitted WAVE_SCALE=1, standalone TIM and PAR reload. This is correspondence
evidence, not independent astrophysical validation.

An earlier parallel library run in `tests-debug.log` had 90 passes, one failure
and 35 ignored tests: `terrestrial_fit_failure_recovers_with_the_same_runtime`.
Its isolated retry passed (`recovery-isolated.log`), as did the subsequent serial
suite (`lib-debug-serial.log`) and both final serial suites above. The failure is
retained. This does not establish concurrent process-diagnostics correctness;
AK's concurrency obligation remains open. The 36 ignored tests were not all
rerun as a historical corpus campaign; selected AH evidence tests were invoked
explicitly by their runners.

The whitespace check reports deliberate upstream spaces/final blank lines in
the two verbatim help files and one trailing space inside `native_build.py`'s
exact source-match literal. These bytes are retained. The ordinary check passes
for every other authored path. No generated binaries, raw outputs or receipts
are committed.

## Final failing comparisons

These same 12 groups fail in both profiles. Data files are compared exactly.
Only stdout/stderr source-location prefixes and the final CPU-time line are
normalized by the runner; no scientific digits, signs, units or records are
removed. The inherited timing/phase/fitting gates are not replaced by this
display comparison, and production precision is unchanged.

| Group | Remaining observed difference / disposition |
| --- | --- |
| `pre-list-bary-unwritten` | Barycentric `correctionUT1` has no source producer. Rust reports the boundary; stdout/stderr differ and `new.par` is missing. Undefined native storage is not a golden. |
| `pre-list-tempo1` | Inherited signed-zero differences in displayed pulsar position/velocity; stdout fails exact comparison. |
| `pre-list-terrestrial` | Modern terrestrial list now completes and PAR matches. Displayed pulsar position/velocity still have inherited signed-zero differences, including both NITS passes. |
| `dm-grid` | RMS stdout and fitted PAR digits differ on the ill-conditioned fixture. Raw differences remain; DM/CM weight files match exactly. |
| `red-noise` | One fitted F2 final digit in `new.par`; other compared files match. |
| `ell1h-h4` | Small offset/RMS stdout differences; fitted PAR matches. |
| `waves` | Unfitted WAVE error arrays are unwritten; stdout/stderr differ and `new.par` is missing. The fitted WAVE workflow passes. |
| `wave-dm-scale` | Same unfitted-error boundary; stdout/stderr differ and `new.par` is missing. The fitted scale-1 workflow passes. |
| `global` | F2 digits in `fitted.par_1`; global covariance output matches. |
| `cheby-odd-defaults` | `t2pred.dat` coefficient digits differ. Native defaults, odd dimension adjustment, command output and filenames match. |
| `cheby-ecliptic-tdb` | `t2pred.dat` coefficient digits differ. |
| `polyco-prefix` | Coefficient digits in `literal-newpolyco.dat` and `literal-polyco_new.dat`; stdout warnings and literal prefix behavior match. AF's separate raw log10RMS gate remains open. |

Absent DDH derived inputs and an all-deleted report span also retain explicit
unwritten-state boundaries. Full native debug/allocation transcripts, every
nonfinite/solver branch, telescope XYZ, fitted shapelets, WAVE scale 2, Jupiter
mass alternatives and all family-specific derived branches were not dynamically
qualified by AH. Their source transfer and individual predicates are mapped;
static accounting is not exhaustive dynamic coverage.

## Native profile and reproducible identities

The executed host is macOS 26.7 / Darwin 25.6 arm64, rustc 1.97.1 and GCC/GFortran
16.2.0. Native GCC uses the recorded `-O2` profile, timing binary128 with 113
significant bits and ordinary long double with 53 significant bits. Rust timing
remains Rug128. These type differences are provenance, never a comparison
allowance. Native sources, patches, linked objects, commands and executables
are indexed in `native-v9/identity.json`.

The native build reuses hash-verified accepted AD/AF objects and reference
archive `313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5`.
The AF accepted executable identity is
`3bf82ca766e45599c23a575868d6977d97c2f25c46428c02c877ccca0276a27f`.
Original/poison probes mean the scoped original output/producer bodies with
these dependencies; they are not pristine whole-upstream builds. The separate
`tempo2-bounded` driver changes only default MAX_PSR/MAX_OBSN to 2/100 to exercise
the source argc=2 grammar without its large fixed allocation. Original argument
failure witnesses retain bounded prefixes and statuses with a two-second time
limit, 1-MiB file limit and disabled core dumps.

All following evidence paths are beneath `validation/core_output/build/` unless
otherwise specified. The delivery index also hashes authored files, all five
native executables and final logs, and records protected repository status.
It is frozen before this report and deliberately has no report self-hash.

| Artifact | SHA-256 |
| --- | --- |
| `delivery-identity.json` | `7c63ad15e0ab1a0fcc3b7a48991f3c28c273f4b2b2a036f6e3db3366bd7dacc7` |
| `native-v9/identity.json` | `63e88ca4eed916986f755d4741b5bf35acde8230719117020ca7f252427a2de8` |
| `native-v9/tempo2` | `b8dbab993d07cc291f95800d05018aac718f55ab3a81d9329a3d8e18ce160012` |
| `../native_build.py` | `f8d88590ae796b8f89109f84d8410c58761ee1c8eb41d17d94e3f75e564fdf11` |
| `../check.py` | `3e0c5d09760e58c5ba5b05963c9a5b4f2c23d5d8057e63885d8f9fc6ab0c5ad0` |
| `../CONTRACT.md` | `c2561cb244ff13f26f0be24a4408932454d0b7cb800eec921e9b466cdb9b4a4b` |
| `debug-final/receipt.json` | `2d227a4c810ef016e650acc178e2605705a59993a5b93b78878064e4417c312e` |
| `release-final/receipt.json` | `8fb5c428158fd35271dca873c648b2170cf443d91bb0bc8679535321ef6337f2` |
| `bugs-v2/receipt.json` | `1f26484c03607d405d61c21846fe82a8527288a6304246ab9fb768e7c56cd46e` |
| `ut1-check-v1/receipt.json` | `a1e4ef4fedea208802c7ed32054712663f95271ff06ab3d3078cba81ff97d885` |
| `source-inventory.json` | `583d9d77b79104de2d4cafeb4272bc98a894f20aa56beba0c6d2f8d133369438` |
| Repository `rust/target/debug/tempo2-rust` | `895521893d6826845a8d17041b12e9d2c0f19fe3de7e20b69cd2c019befcc8cc` |
| Repository `rust/target/release/tempo2-rust` | `340c3d5eb5cede14dd96ffddd248692db67cc238d91fd10d6ac63e01577eb7bf` |

The accepted evidence and all AH attempts remain in place. Native v1-v6 used the
superseded AF dependency selection and are not delivery oracles; v7-v8 and older
debug/release/snapshot attempts are historical. The final names above identify
the actual completed runs. The development translator uses pycparser 2.23;
Cargo consumes committed Rust and has no Python-parser dependency.

Canonical TEMPO2 retains its pin and unchanged tracked source, with exactly the
five pre-existing untracked `.DS_Store` files in the root, T2runtime, python,
tests and unsupported_plugins directories. The independent reference checkout
is clean at its recorded pin. AH commits contain only explicit authored paths
under `rust/` and `validation/core_output/`.

## Remaining acceptance and release scope

The 0091 ledger is inherited unchanged: P's 154 records including 32 SIMUL;
M2's four raw-mode order failures; 0049's four discrete boundaries and three
unconstrained interpolation cases; AE red's six raw modes; chromatic-log's
sixteen errors and one multiplier; ELL1H's nineteen whitened-data differences;
single-pulsar quadrupole/GW-single/memory/cosmic-string 770/750/448/468 failures;
DDH's 401/433 current/historical fidelity failures and two shared-solve raw-order
differences; two first-pass SWGP unwritten-state boundaries; and AF's seven raw
polyco log10RMS failures. The paired DDH clock rule and historical 1 ns ideal
pass remain intact. AH has neither rerun nor accepted that entire ledger.

Central's 0092 diagnosis retains the same seven failures among 2,247 generation
comparisons at each of 128 and 113 bits, with all fourteen raw diagnostic values
unchanged. Its Rømer/BAT and BT orbital-delay localization is not a complete
arithmetic cause or grounds to change precision. Central's 0093 optional QR
output correction belongs to the prepared backend stage; AH does not activate
that backend or change its defaults.

New unsafe Rust is limited to the CPU-clock boundary and a private `snprintf`
call for one validated binary64 conversion, documented in the source map. Existing scanner/SOFA
FFI remains. The full Rust core/toolkit plus standalone predictor C guarantee,
actual Linux x86_64 qualification, installed ABI, optional backends, final SOFA
disposition, concurrency/performance and integrated release acceptance remain
separate obligations. This macOS implementation handover does not close them.
