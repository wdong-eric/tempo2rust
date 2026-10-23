# M2 fitting implementation delivery

23 October 2026. Assignment E delivers the ZERO plus selected local F0–F3
library and JSON CLI for the accepted homogeneous barycentric `@` and PKS
profiles. **M2 is not accepted by this chat. The exact singular-mode order gate
remains unresolved, as explicitly requested by the user.**

The final full verifier exits **1** with four diagnostics: the dependent-column
fixture's raw retained-flag order and retained singular-value position each fail
in debug and release. All 15 end-to-end fixtures (20 fit cycles per build), the
remaining shared-input comparisons, source nonfinite classifications, failure
and recovery checks, and existing M1 regressions pass. Paired-mode diagnostics
explain the order difference but do not waive it.

## Delivery identity and reproduction

- Reference/export/comparator commit: `4ba0f208897fed5c5a46dc424d12e02bbc4b5526`.
- Production implementation/test commit: `e7efeac326f9f198f987194b7a533c3156ec8135`.
- This report and the final verification records are delivered in the following
  evidence commit; its identifier is supplied with the delivery message.
- Canonical TEMPO2: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
- Independent corrected A2 reference: `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`.
  Both independent repositories remain clean. E changed neither repository.
- A2 static-library SHA-256:
  `313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5`.
- Frozen input-manifest SHA-256:
  `f02fc4bc47efd5d488dbeebd34b4cc68081a72275501ed3d7676c60e76253e19`.
- Reference provenance-manifest SHA-256:
  `cf4ed1a4f055fe2d68816ddac2a6b2b37472dff30f38553c13f2971755702cc9`.
- Unchanged Cargo.lock SHA-256:
  `5847bf7fbf5be0042ad8e714b1d43c86820dda9ee0188ed081013c4ef5d1a90f`.

Executed on macOS 26.7 arm64 with Rust/Cargo 1.97.1, GCC/GFortran 16.2.0,
Rug 1.30.0 and gmp-mpfr-sys 1.7.1. No dependency was added. The reference uses
binary128 TEMPO2 `longdouble` (113 significand bits), native C++ `long double`
with 53 bits, and disabled BLAS/LAPACK/ACCEL_LSQ. Production retains the
approved Rug128 representation; the test asserts its 128-bit precision.

From the project root:

```sh
python3 validation/fitting/generate_reference.py
python3 rust/verify_m2.py
```

The second command currently returns 1 for the explicitly unresolved order
gate. It completes and saves the other checks before returning. Full command
vectors and return codes are in
[`commands.json`](verification/m2/commands.json), the nested
[`m1b/commands.json`](verification/m2/m1b/commands.json), and the reference
[`manifest.json`](../validation/fitting/reference/manifest.json). The full final
driver output is [`full_run.log`](verification/m2/full_run.log).

The verifier sets its existing offline Cargo/cache paths and validates frozen
inputs, exporter/source/library hashes and reference output hashes. It redirects
the unchanged M1b verifier's output to `verification/m2/m1b/`, preserving old
accepted records. [`source_hashes.json`](verification/m2/source_hashes.json)
identifies the executed Rust, comparison, fixture and verification sources.
The reference manifest records exact compiler/link commands, its executable
hash, source hashes and the accepted 80-resource manifest. Per-process
`.resources.tsv` files record actual opens, including absent meteorology files.

## Implemented behavior

`fit_barycentric` and `fit_terrestrial` borrow the original `TimingInput`, update
an owned model and return `FitResult`. `FitOptions` exposes only the source
error-rescaling flag. PAR MODE/WEIGHT and NITS use the existing parser; repeated
MODE/WEIGHT assignments retain source order and the last value wins. Defaults,
original tokens and original parameter uncertainty metadata remain visible.
The fit-only restrictions reject global and non-spin active flags. Existing
fixed evaluation and its JSON are unchanged.

`tempo2-rust fit --par PATH --tim PATH [--runtime PATH] [--norescale]` is a thin
library caller. PKS needs an explicit runtime. JSON calculation/serialization
finishes before stdout publication; an OS write failure is still a transport
failure, not a transactional pipe guarantee. `tempo2-fit-v1` separates original
metadata, fitted physical values/errors, raw fit-coordinate covariance, raw
timing diagnostics, unweighted mean-subtracted residuals, binary64 linearized
residuals and recomputed postfit timing. NaN and signed infinities have explicit
string encodings. Null denotes only undefined source ZERO-error storage.

The port retains source-scaled columns, binary64 whitening, both binary64 SVD
helper boundaries, source loop/arithmetic order, strict binary64-literal
`1e-27` cutoff, pseudoinverse, `n - p` degrees of freedom and sequential updates
using already-updated F0. It keeps covariance unscaled, errors rescaled, MODE 0
rescaling even with `--norescale`, exactly NITS cycles, and ZERO assignment
without adding an offset delay to recomputed timing. Defined source bugs are
preserved; no alternative solver, column normalization or precision mode exists.

The complete 23-file naming map and eight existing unsafe Rust sites are in
[`SOURCE_CORRESPONDENCE.md`](SOURCE_CORRESPONDENCE.md). New fitting code is safe
Rust. Retained native files, fixed goldens and prior M1b evidence are unchanged
relative to `ddc07b2`. The source-bug/adapter/coverage departure table, source
locations, adjacent explanations and regression evidence are in
[`BEHAVIOR_DIFFERENCES.md`](BEHAVIOR_DIFFERENCES.md).

## Independent evidence and measured coverage

The reference exporter invokes the actual source `t2Fit` and BAT/residual
lifecycle. Direct derivative/update callbacks and shared-input solver calls are
separate checks. Singular values come from a second source SVD call; rank,
cutoff labels, linearized residuals and error multipliers are explicitly named
postprocessing. No Rust fit equation serves as the source oracle.

The fixture construction, truth model, source-driven phase alignment and fixed
microsecond perturbations are documented in
[`validation/fitting/README.md`](../validation/fitting/README.md). The two
alignment bounds are approximately 2.67e-25 s (`@`) and 2.65e-25 s (PKS).
Inputs were frozen before comparison. Agreement is an implementation check,
not evidence of physical truth recovery, identifiability or broader timing
accuracy. Raw prefit predictions are independently gated before fit outputs.

Each build checks 14 completed standalone solver cases plus 20 shared-input
timing-cycle solves, 15 end-to-end fixtures and direct callbacks: 50 comparison
records and 10,666 numeric measurements. Shapes, modes, controls, row/column
order, counts, pulse numbers, rank and drop classifications have separate exact
checks. Forty-six JSON artifacts are byte-identical between debug and release.
There are 65 fitting comparator mutation checks per build, including just-over-
gate changes, paired-order rejection, cutoff equality, nonfinite classes and
physical output permutations. Each build also passes all 23 Rust unit/integration
tests and 17 fitting CLI checks. Source formatting and all-target Clippy with
denied warnings pass.

The full precision signed/absolute maxima, limits, fixture/cycle/row/column paths,
errors and diagnostic mode permutations are in
[`numerical_errors.json`](verification/m2/numerical_errors.json). The table below
lists every end-to-end cycle. Every listed cycle passes matrix, solve, covariance,
reported-error, physical-update, timing and exact-count gates in both builds.
Raw/centered columns include both prefit and postfit snapshots; all error columns
are seconds. The physical column is the prescribed timing-effect-scaled error.
[`coverage_by_cycle.json`](verification/m2/coverage_by_cycle.json) records the
per-cycle maxima derived from the saved debug artifacts; release artifacts are
byte-identical. Splitting this reporting view does not replace the full NITS and
iteration-count comparisons.

| Fixture / cycle | Columns | MODE / rescale requested | Raw residual | Centered | Linearized | Physical update |
|---|---|---|---:|---:|---:|---:|
| `zero` / 1 | ZERO | 0 / true | 4.385E-27 | 4.007E-27 | 4.362E-41 | 5.125E-28 |
| `f0` / 1 | ZERO/F0 | 1 / false | 4.385E-27 | 4.009E-27 | 3.388E-21 | 5.125E-28 |
| `f1` / 1 | ZERO/F1 | 0 / false | 4.385E-27 | 4.012E-27 | 3.388E-21 | 5.125E-28 |
| `f2` / 1 | ZERO/F2 | 1 / true | 4.385E-27 | 4.008E-27 | 1.694E-21 | 5.125E-28 |
| `f3` / 1 | ZERO/F3 | 1 / false | 4.386E-27 | 4.007E-27 | 4.947E-41 | 5.125E-28 |
| `subset` / 1 | ZERO/F0/F2 | 1 / true | 4.010E-27 | 4.009E-27 | 3.388E-21 | 5.125E-28 |
| `subset` / 2 | ZERO/F0/F2 | 1 / true | 4.062E-27 | 4.023E-27 | 4.547E-42 | 4.229E-28 |
| `all` / 1 | ZERO/F0/F1/F2/F3 | 1 / false | 4.385E-27 | 4.011E-27 | 6.776E-21 | 5.125E-28 |
| `all` / 2 | ZERO/F0/F1/F2/F3 | 1 / false | 5.713E-27 | 4.011E-27 | 4.400E-42 | 1.589E-27 |
| `all_mode0` / 1 | ZERO/F0/F1/F2/F3 | 0 / false | 4.254E-23 | 2.951E-23 | 3.388E-21 | 3.502E-23 |
| `all_mode0` / 2 | ZERO/F0/F1/F2/F3 | 0 / false | 4.254E-23 | 2.951E-23 | 4.987E-42 | 3.502E-23 |
| `pks_all` / 1 | ZERO/F0/F1/F2/F3 | 1 / true | 9.490E-14 | 8.743E-14 | 9.275E-14 | 3.061E-13 |
| `pks_all` / 2 | ZERO/F0/F1/F2/F3 | 1 / true | 9.490E-14 | 8.743E-14 | 9.275E-14 | 3.061E-13 |
| `pks_all_mode0` / 1 | ZERO/F0/F1/F2/F3 | 0 / false | 5.692E-14 | 6.317E-14 | 5.867E-14 | 2.660E-13 |
| `pks_all_mode0` / 2 | ZERO/F0/F1/F2/F3 | 0 / false | 5.692E-14 | 5.867E-14 | 5.867E-14 | 2.660E-13 |
| `pks_seven_mode0` / 1 | ZERO/F0 | 0 / false | 1.137E-13 | 6.191E-14 | 6.191E-14 | 1.329E-14 |
| `pks_seven_mode1` / 1 | ZERO/F0 | 1 / true | 1.137E-13 | 7.232E-14 | 8.729E-14 | 4.064E-14 |
| `zero_derivative` / 1 | ZERO/F3 | 1 / false | 0 | 0 | 0 | 0 |
| `timing_square_false` / 1 | ZERO/F0 | 1 / false | 0 | 0 | 0 | 0 |
| `timing_square_true` / 1 | ZERO/F0 | 1 / true | 0 | 0 | 0 | 0 |

Representative global maxima follow. These are largest absolute errors within
each field group; the quoted gate belongs to that same comparison. Variable
scales and covariance units make them unsuitable for comparing conditioning
between fixtures. Full per-case maxima remain in the JSON evidence.

| Family / quantity | Maximum absolute difference | Gate at that point | Fixture / location |
|---|---:|---:|---|
| shared / increment_effect_s | 3.057E-23 | 1.000E-13 | `all_mode0/1` / `shared.solver.increments[3]` |
| shared / modeled_difference_s | 3.057E-23 | 1.000E-14 | `all_mode0/1` / `shared.modeled_difference[15]` |
| shared / linearized_s | 2.776E-17 | 1.000E-12 | `pks_seven_mode0/1` / `shared.solver.linearized_s[6]` |
| end-to-end / x_days | 8.674E-19 | 3.469E-18 | `pks_seven_mode0` / `cycles[0].x_days[0]` |
| end-to-end / matrix | 1.137E-13 | 2.592E-5 | `pks_seven_mode0` / `cycles[0].matrix[0,1]` |
| end-to-end / white_matrix | 1.192E-7 | 2.592E+1 | `pks_seven_mode1` / `cycles[0].white_matrix[0,1]` |
| end-to-end / increment_effect_s | 3.061E-13 | 1.000E-11 | `pks_all` / `cycles[0].solver.increments[4]` |
| end-to-end / physical_update_effect_s | 3.061E-13 | 1.000E-11 | `pks_all` / `cycles[1].after_values[3]` |
| end-to-end / errors | 7.267E-14 | 4.594E-11 | `pks_all` / `cycles[0].solver.errors[4]` |
| end-to-end / physical_error_effect_s | 7.267E-14 | 4.594E-11 | `pks_all` / `cycles[1].after_errors[3]` |
| end-to-end / covariance | 3.815E-6 | 1.310E+1 | `pks_all_mode0` / `cycles[1].solver.covariance[0,4]` |
| end-to-end / chi_square | 4.429E-3 | 3.069E+4 | `pks_seven_mode1` / `cycles[0].solver.chi_square` |
| end-to-end / mean_s | 5.684E-14 | 1.000E-12 | `pks_seven_mode0` / `cycles[0].prefit.mean_s` |
| end-to-end / residual_s | 1.137E-13 | 1.000E-12 | `pks_seven_mode0` / `cycles[0].prefit.observations[1].residual_s` |
| end-to-end / centered_s | 8.743E-14 | 1.000E-12 | `pks_all` / `cycles[0].postfit.centered_s[4]` |
| end-to-end / linearized_s | 9.275E-14 | 1.000E-12 | `pks_all` / `cycles[0].solver.linearized_s[4]` |
| end-to-end / bbat_mjd | 5.684E-14 | 1.000E-12 | `pks_all` / `cycles[0].prefit.observations[6].bbat_mjd` |
| end-to-end / phase_turn | 9.607E-13 | 1.000E-10 | `pks_all` / `cycles[0].postfit.observations[4].phase_turn` |

Standalone solver coverage includes weighted rescaled/unscaled and unweighted
controls; below/equal/above cutoff; a zero column; an all-zero matrix; square
rescaled/unscaled fits; three narrowing-overflow cases; and exactly dependent
columns. All pass their physical-coordinate and nonfinite checks. The dependent
case alone fails its raw mode-order checks. Shared matrices/data are identical
after their binary64 boundaries. Decimal transport spelling can leave tiny
nonzero reported differences for identical binary64 results; finite gates use
the declared Decimal comparison, while exact binary64 inputs are decoded first.

## Unresolved exact-order gate

The exactly dependent two-column matrix produces:

| Engine | Raw singular values, approximately | Raw retained flags |
|---|---|---|
| TEMPO2 binary128 | `[3.46410161513775459438, 3.045162619e-34]` | `[true, false]` |
| Rust Rug128 | `[9.293098813e-39, 3.46410161513775459438]` | `[false, true]` |

Solutions, raw covariance, errors, chi-square and rank agree within their gates.
Paired spectra and each engine's strict cutoff flags also agree. A diagnostic
113-bit run reproduced the source order; production was restored to and remains
128 bits. That diagnostic is not a replacement implementation or acceptance run.

Central decision 0010 permits pairing modes, but the subsequent direct user
answer was **“Keep exact order gate and report it unresolved.”** This delivery
enforces that latest instruction. It preserves both engines' raw output, checks
raw retention order and retained singular-value positions, and records paired
analysis only as diagnostic evidence. No tolerance is relaxed. The four exact
failure messages are in
[`open_gate_failures.json`](verification/m2/open_gate_failures.json).

## Failure, recovery and unchanged fixed evaluations

- Square rescaling retains source NaN errors and the callback's `error >= 0`
  predicate; supplied PAR uncertainty remains metadata while current source
  physical errors initialize to zero. +Infinity updates errors; NaN/-Infinity
  leave prior errors. Undefined first ZERO-error storage is never read by the
  exporter and has an explicit null/state representation in Rust.
- Overflow fixtures preserve positive infinity in covariance/errors/increments,
  negative infinity in linearized residuals and NaN rescaled errors, with exact
  class checks. Finite rank deficiency is solved by the source pseudoinverse.
- The source `n < p` probe returns untouched output sentinels. Zero/tiny-weight
  standalone calls SIGSEGV; full timing calls print the 30-iteration termination
  after entering nonfinite SVD state that can first access `w[-1]`. Rust returns
  contextual errors at the invalid-index boundary. These are explicit
  termination/undefined-state adapters, not exact successful-result matches.
- Unsupported/global selections, invalid modes/NITS, empty/underdetermined data,
  invalid weights and missing runtime resources produce no CLI JSON. Subsequent
  valid fits succeed. Barycentric and PKS library recovery occurs in the same
  process; PKS reuses the same runtime. An injected failure after a completed
  earlier cycle returns no partial fit and leaves caller input unchanged.
- The unchanged M1 verifier passes the two terrestrial goldens, all stage gates,
  4 ns response, 80 resource identities, both builds' resource/CLI recovery,
  existing comparator mutations and M1a regressions. Evidence is isolated under
  [`verification/m2/m1b/`](verification/m2/m1b/). No accepted M1 goldens,
  numerical rules, native files or old verification artifacts were overwritten.

Reference failure return codes/logs are preserved in
[`boundaries.json`](../validation/fitting/reference/boundaries.json). Rust test
results, CLI records, callback results and recovery artifacts are under
[`verification/m2/`](verification/m2/).

## Remaining limits

The raw dependent-mode order gate is unfinished acceptance work. The explicit
iteration-30 branch is ported, but no finite matrix independently demonstrated
safe exhaustion of that branch; source nonfinite termination probes do not prove
finite nonconvergence equivalence. The JSON schema is supplied as documentation;
structural/numerical comparator checks ran, but a separate JSON Schema validator
was not installed or run.

JPL preflight validates inspected bytes, while native initialization reopens the
path. File identity/content remaining unchanged through native use is an
**unenforced assumption**. Preflight does not guarantee that native code consumes
the validated bytes or establish memory bounds for a changed file. This existing
limitation is stated next to the FFI calls and in both ledgers. E does not expand
into JPL initialization, caching or reader changes; the separate central stage
owns that work.

Independent PKS fixtures use fresh contexts matching isolated reference
processes. Within a fit the runtime is reused. Cross-profile clock-cache history
equivalence remains untested. Existing M1 behavioral departures are retained and
listed for the separate fixed-baseline reconciliation stage. Linux, broader
binary/DDH models and the existing DDH 1 ns issue, other observatories, performance,
concurrency stress and distribution packaging remain outside this assignment.

Implementation and evidence are delivered for central review. This report does
not grant M2 acceptance or resolve the outstanding fidelity and safety limits.
