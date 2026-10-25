# Assignment AC: general input-to-BAT delivery

The general PAR/TIM → preprocessing → native-JPL → propagation → BAT/BBAT
workflow is implemented as a library and `bats` CLI. The existing bounded
load/residuals/fit frontends retain their validation and schemas, and their
terrestrial evaluation now uses the shared canonical calculation.

**Implementation handover is pending central review under 0074. This is not a
claim of milestone acceptance:** P has unresolved compatibility differences,
and the original AB expectations remain visibly different at three corrected
source defects. AD/full residual/binary/fitting work has not been substituted
with this delivery or started here.

Source is TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; accepted AA is
`ea9797236fae0f0dd29a32e5d3050e33eaf2fefe`, with production baseline `99cf714`.
The implementation and runnable checks are committed in `18c9065`. The contract
was committed in `4114aec`; its chronology is recorded in
[INPUT_TO_BAT_CONTRACT.md](INPUT_TO_BAT_CONTRACT.md). This report and its committed
implementation are the proposed AC handover. Central must pin that revision,
decide acceptance status, and close AC's Rust ownership before releasing AD.

## Public workflow and ownership

```sh
rust/target/release/tempo2-rust bats \
  --par validation/input_to_bat/fixtures/ordinary.par \
  --tim validation/input_to_bat/fixtures/ordinary.tim \
  --runtime validation/reference/build/source/T2runtime
```

The nth PAR is paired with the nth TIM. Library callers use `bats::load_pairs`,
`bats::preprocess`, `RuntimeContext::from_root`, and `bats::evaluate`. Direct
reader/helper/global operations, including `form_bats_all` and the narrower
`update_bats_all`, are also public. The README contains a library example.

One canonical `PulsarContext` owns scientific state. The wrapper records ordered
input bytes, paths, roles and SHA-256 identities. Runtime resources are identified
from consumed owners. Distinct process globals live in the existing session lock;
internal calls borrow that session instead of locking recursively. Source INFO,
warning masks, ecliptic obliquity and random history retain their own lifetimes.

Schema 1 returns ordered main observations plus a separate TZR observation,
byte-lossless identities/flags, deletion and source selectors, unitful components,
round-trip Rug128 MJD strings, explicit nonfinite classes and diagnostics. The CLI
serializes the whole result before writing stdout. An error retains source prefix
writes; it does not roll back an appended TZR row. Tested recovery creates a fresh
input context and reuses the same runtime. Runtime provenance accumulates consumed
resources across calls; use a new runtime for a standalone inventory.

## Executed verification

The tested host is aarch64-apple-darwin, Rust/Cargo 1.97.1, LLVM 22.1.6,
Homebrew GCC 16.2.0. Production extended precision remains Rug128. Test wire
projection to binary128 is a comparator operation, never a production mode.
Generated traces, receipts, binaries and native patches are ignored under
`validation/input_to_bat/build/`; no accepted reference or nested TEMPO2 source
was edited.

| Check | Debug | Release | Scope and qualifications |
| --- | --- | --- | --- |
| K parameters | 304 pass | 304 pass | All 2000 slots/9616 cells, metadata, controls, links, cursor, exits and diagnostics. Seventeen records use only 0073's named continuous bounds; remaining comparisons stay exact. Two ambient-environment cases are inapplicable to the explicit-root API. |
| Q specialized PAR | 121 pass | 121 pass | Full typed/raw membership and diagnostics. Seven invalid historical TEL cases are withheld under 0072; safe-prefix/bounded-native and Rust 501/1000 controls are retained. |
| N TIM | 157 pass | 157 pass | Complete 95-field/7680-byte observations, caller tails, control/history, outputs and streams. Five JBO histories use separate corrected diagnostic expectations; all scientific state stays original. |
| P preprocessing | 296 executed; fail | 296 executed; fail | 154 state-record differences in both runs; zero diagnostic differences. Six external-plugin ABI cases are explicitly excluded. See the open ledger below. |
| Z planetary | 82 driver + 15 random pass | 82 driver + 15 random pass | One shared case gives 96 unique native cases. Complete driver caller/exit state and all random private histories; no call-mock equality claim. |
| AB original propagation | 324 executed; fail | 324 executed; fail | Both have 277 field/event differences, confined to three documented 0053 corrections; 321 original histories pass. Original failures are not suppressed. |
| AB corrected propagation | 3 pass | 3 pass | Separately rebuilt native refphs/troposphere corrections; original histories independently reproduced before generating corrected expectations. |
| Public file workflows | 18 pass | 18 pass | Fresh corrected-native execution for every case; exact membership/selectors/units and existing component gates; SAT/BAT/BBAT differences evaluated in extended seconds at 1e-12 s. |
| Library lifecycle and CLI failures | Pass | Pass | Repeat evaluation, narrow update without ephemeris reload, empty context, missing/corrupt JPL and unavailable-CALCEPH recovery; three failing CLI calls emit no success stdout. |
| Bounded terrestrial and 4 ns response | Pass | Pass | Existing immutable terrestrial-v1 contracts and same-runtime failure/recovery. |
| Standard Cargo tests | 64 pass, 26 ignored | 64 pass, 26 ignored | Ignored tests are separate corpus/resource drivers, not silently counted as passes. Includes the final signed-class regression, which also has its own native/debug/release witness. |
| Formatting / Clippy | Pass | — | `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `git diff --check`. |

The 18 public cases are `ordinary`, `ecliptic`, both ordered mean/TZR combinations,
`dispersion`, `chromx`, linked and numeric `dmmodel`, `shapelet`, `secular`, `fb90`,
`mixed_tropo_disabled`, `tempo1`, `tim_controls`, empty/nonempty DM tables, `epoch`,
and `solarwind`. They exercise non-PKS sites/aliases, terrestrial and barycentric
rows, PM/PX/ecliptic, units/time selectors, INCLUDE/flags/time/error/selection,
DM/CM/DMX/DMMODEL/shapelets, reference state and secular motion. The solarwind
current sheets are explicitly synthetic numerical fixtures, not calibration of
the physical model. Membership/unit negative controls reject all mutations in
both profiles. Final MJD agreement does not replace component comparisons.

The original K/Q/N/P records and Z/AB ordinary/observer records retain their own
reference hashes in receipts. Z checks complete public caller state, globals,
seeds, status and diagnostics; its random tests check private state. AB checks
3288 recursively expanded typed fields at every recorded begin/end/defined-exit
event, all parameter backing, complete observation/reference records, helper
results and streams. Additional native private call traces are not independently
projected into Rust mocks. Native AB NaN records supply class but not payload/sign;
that missing information is not represented as measured. P's typed raw projection
normalizes NaN payloads only, preserving signs, neighboring bytes and membership
under 0074. CPU elapsed seconds alone are normalized in N timing diagnostics.
One final parallel standard run exposed parser diagnostics entering an unrelated
fit-recovery result. The new stateful parser tests now run in fresh processes;
production session lifetime and the recovery assertion are unchanged. Both final
standard profiles pass. The initial failing log is retained separately.
Final AB verification runs complete independent case histories in four isolated
processes using the same unoptimized debug or optimized release test binary; every
case resets its source session. The interrupted serial debug attempt is retained
separately and is not counted as a completed run. Earlier component/public receipts
retain their actual execution identities; the subsequent negative-NaN-only repair
has focused native/both-profile evidence and final standard/AB verification.

## Localized corrections and original evidence

Each production correction has an adjacent source/cause explanation. Staged
native changes and compiler/object/executable hashes remain separate from original
records. `native_pipeline.py` links corrected primary units against the verified
accepted reference archive. Each public comparison removes stale output, launches
the native executable freshly, verifies its identity before/after, and records
the consumed input/output hashes. `--reuse-native` reuses only that verified binary.

The final public native executable SHA-256 is
`c9a7e04bb808e7d7452a2fc9a54532b1f3298b42f8b36ed28486ff758ef02a6e`, linked against
accepted archive `313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5`.
P's original corpus is `1e44d2446fa37ce22cbe0de36fbd1c2879b0005861c94819181b7e97a13999f2`;
AB's original main corpus is `0f43f863b19961842ece487680c4843882565aadfcc67b2581953e1bfb0f5a88`
and supplemental corpus is `3b05032ea23bdb9975da0a4cabb2c0746578e951855da69339504307fcd1deb1`.

| Correction | Cause, boundary and evidence |
| --- | --- |
| No-table ndm | Raw preProcess passes an unwritten count without `-setdm`; initialize the empty table count. Default, empty and nonempty public examples pass. |
| Fresh optional storage | Missing prefit uncertainty, empty chromatic/solar tables, bypass clock/time operands, rough site velocity and inactive planet operands can be read before a producer. Initialize only unwritten storage; retain defined caller values and inactive knownness elsewhere. |
| Shapelet first BAT | dm_delays tests BAT==0 to select SAT before the first BAT calculation. Supply the fresh zero sentinel only before the first write. The shapelet public fixture exercises it. |
| Numeric DMMODEL warning | dm_delays reads the first link slot even when nLinkTo==0. A fresh unwritten slot means no link; defined retained targets still follow source behavior. The numeric DMMODEL fixture exercises it. |
| Disabled troposphere | calculate_bclt clears psr[0] while processing p. Clear the current pulsar. Original `bclt_tropo_0` has 9 differences; the separately corrected history passes. |
| Reference cleanup | refphs_clean tests psr[0].refphs instead of psr[p].refphs. Correct the selector. Original `phase_cross_0_1`/`phase_cross_1_0` have 149/119 differences; corrected histories pass. |
| JBO aliasing | Int-pointer access to declared double storage yields compiler-dependent word diagnostics. Byte access preserves intended swaps and all scientific state. Separate original/corrected native proof covers five histories; only three histories' diagnostic words change. |
| Existing wet mapping | Carry W's corrected upper c coefficient; original sources remain untouched. |

Translation repairs also preserve actual compiled binary64 contractions in
planet-offset interpolation, Shapiro direction/norm, and solar terms. The source
header maps `powl` to `powq`; chromatic power uses extended arithmetic until the
source binary64 assignment. `inspect_native_numerics.py` records accepted object
instructions and the header macro. These are source correspondence repairs, not
new physical models or tolerances. The final signed-class check demonstrates that
native parsing/formatting retains `-nan`; the Rust parser/formatter now does too.

## Unresolved-check ledger

Every individual P difference retains its case, action, field, raw deltas and
available native/Rust numbers in `build/preprocessing/differences.json`. Counts
below are differing field/event records, not independent scientific parameters;
whole-record projections duplicate some typed differences. `open-checks.json`
indexes each input/check with the localization and owner below.

| ID / named input and check | Observed result and localization | Affected scope / owner |
| --- | --- | --- |
| AC-P-SIMUL: `simul_123,124,127,128,139,140,143,144` | 32 deleted/whole-record differences. The strict `<60` boundary evaluates false at native 113 bits and true at Rug128. The 113-bit replay matches native; 256/512 bits approach the Rug128 branch. No threshold/precision workaround is applied. | Built-in SIMUL selection/deletion at a representational boundary. Central numerical disposition; current Rust owner implements an approved change. |
| AC-P-UNITS: `units100`, both directions for `2↔1` and `2↔100` at `@`/`PKS`, `units_site_case` | 71 val/prefit/err/prefitErr record differences in `units::transform_units`, `scale_param`, `xform_mjd`; retained raw numeric evidence. | Unit conversion and downstream parameters. No P tolerance exists; central disposition and current Rust owner diagnosis. |
| AC-P-STAGE2: `stage2_order` | 2 SAT/whole-record differences in the source-ordered offset arithmetic in `pre_process_simple2`. | Preprocessed SAT; central disposition and current Rust owner diagnosis. |
| AC-P-LINK: `wrapper_link_chain` | 4 val/prefit record differences after linked-value/units preprocessing. | Linked parameter preprocessing; 0073's K-only allowance does not apply. Central disposition and current Rust owner diagnosis. |
| AC-P-EPOCH: `epoch_CENTER`, `CENTRE`, `RIGHT`, `58001.25suffix`, `sentinel`, `motion_0..15`, `fitwaves_binary`, `precision` | 45 val/prefit record differences localized to spin/DM/position epoch arithmetic in `update_epoch`. | Epoch transformations; central disposition and current Rust owner diagnosis. |
| AC-AB-ORIGINAL: three cases named above | Original full check remains failed with 277 differences. Separate corrected expectations preserve the original oracles and pass. | Review of the two 0053 source corrections; central acceptance owner. |
| Inherited M2 / 0049 / DDH | M2's exact raw SVD mode-order gate, four utility quotient/rounding boundaries, and DDH's original 1 ns check remain unresolved. No closure is inferred or rerun claimed here. | Cross-stage numerical acceptance; central retains ownership and assigns implementation to the current writer. |

The 122 continuous P records are localized arithmetic differences, **not a proved
blanket rounding equivalence**. Their original checks remain failed until a
specific explanation/correction or central comparison decision is evidenced.
The SIMUL diagnosis is stronger: `diagnose_precision.py` reproduces the exact
native SAT and evaluates the source expression at 113/128/256/512 bits. Neither
that diagnosis nor the passing public examples grants a P numerical allowance.

Enabled CALCEPH, external plugin ABI, full upstream CLI/text output, full
residual/binary/fitting, other platforms, concurrency stress, performance and
distribution qualification remain later full-core obligations. This delivery
does not claim physical validation of arbitrary scientific combinations.

## Reproduction and retained identities

Run from the repository root with:

```sh
export CARGO_HOME="$PWD/rust/.cargo-home"
export GMP_MPFR_SYS_CACHE="$PWD/rust/target/gmp-cache"
cargo test --manifest-path rust/Cargo.toml --locked --offline
cargo test --manifest-path rust/Cargo.toml --locked --offline --release
```

Run each `check_parameters.py`, `check_par.py`, `check_tim_corpus.py`,
`check_preprocessing.py`, `check_planetary.py`, `check_random.py`,
`check_propagation.py`, `check_pipeline.py`, and `check_lifecycle.py` under
`validation/input_to_bat/`, once without and once with `--release`.
P and original AB intentionally retain nonzero outcomes while the ledger is open.
First regenerate owned corrected proofs with `telescope_boundaries.py` and
`corrected_propagation.py` when their verified local artifacts are absent. The N
runner creates missing JBO correction evidence after recording its original
histories; `corrected_jbo.py` also regenerates that proof directly.
`check_propagation.py --corrected` checks the separate AB
proof. The public pipeline builds its disposable corrected native executable by
default. N's initial run records fresh native histories; `--reuse-native` validates
those identities for subsequent Rust-only comparisons.

`diagnose_precision.py`, `check_numeric_classes.py`,
`check_pipeline_comparator.py`, and `inspect_native_numerics.py` reproduce the
specific diagnoses/negative controls. `compare_terrestrial` plus the existing
`rust/tests/terrestrial/compare.py` reproduce both bounded regression contracts.
Generated receipt paths and hashes are indexed by `write_handover_receipt.py`;
that manifest distinguishes completed checks, qualified corrections and failures.
Old `build/tim_corpus/` evidence is explicitly invalidated by `INVALID.json`; the
valid compact N staging is `build/n/`. Earlier partial logs are not final receipts.
