# Independent M2 fitting evidence

This directory is owned by Assignment E. TEMPO2 is read from pinned commit
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`; the corrected A2 library is read
from independent reference commit `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`.
Neither repository is changed. The library SHA-256 is
`313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5`.

The reference fixtures were established before the Rust fit comparisons. The
later three overflow cases extend nonfinite classification coverage; they do
not change any original input or a numerical tolerance. `input_hashes.json`
freezes every supplied decimal input. `cases.json` identifies the selected
parameters, mode, iteration count, no-rescale option and truth model through
those input files. `reference/manifest.json` records compiler/configuration,
source/library/executable/input/output hashes, exact commands and the accepted
runtime-resource manifest. Per-process `.resources.tsv` files record actual
resource opens, including unsuccessful meteorology lookup. The native type
profile is macOS arm64, GCC 16.2.0, binary128 TEMPO2 `longdouble`, binary64
native C++ `long double`, and no BLAS/LAPACK/ACCEL_LSQ.

Run from the project root:

```sh
python3 validation/fitting/generate_reference.py
python3 rust/verify_m2.py
```

The first command verifies frozen inputs and links `export_reference.cc` to
the existing A2 library. It writes only this directory. Reconstruct inputs
only deliberately with `--create-inputs`; the script contains every seed,
permutation, perturbation and solver row. The second command verifies reference
identity, runs both Rust build modes and all M1 checks, and writes new evidence
under `rust/verification/m2/`. It does not overwrite accepted M1 artifacts.
Before shared-input comparisons, the verifier requires the exact shared artifact
set and an array with exactly one solver cycle or the declared timing NITS cycles
for each case. Missing, empty, truncated, extra and non-array artifacts fail
verification. Prior shared outputs are cleared before generation and copying so
stale files cannot fill a missing case. `rust/test_verify_m2.py` exercises these
checks with one- and two-cycle cases; per-build coverage records list all 29
completed shared cases and 34 cycles.

## What the exporter actually observes

For each iteration the exporter runs `formBatsAll`, observes raw diagnostics
with `formResiduals(...,0)`, then calls `formResiduals(...,1)`. It calls the
source fit-data and design-matrix builders to observe the inputs, invokes the
actual `t2Fit`, reads its initialized estimates/errors/covariance and physical
parameter values/errors, and recomputes BATs/residuals. Iteration >1 refreshes
`prefit`/`prefitErr`, following `tempo2.C`. No timing, derivative, update or
solver equation is reimplemented as an oracle.

The diagnostic singular spectrum comes from a separate call to the original
`TKsingularValueDecomposition_lsq` on the same rounded white matrix. Rank and
cutoff labels, the error multiplier and the named `y - A p` residual are
postprocessing. This does not expose or compare nonunique SVD vectors. The
unscaled errors are `sqrt` of the actual raw covariance diagonal. The exporter
adds no source patch or instrumentation beyond A2's existing resource tracing.
No uninitialized `offset_e` is read: when every ZERO callback error so far is
NaN, its output is null with an explicit undefined-storage meaning in the
Rust contract. Null is never used to encode a floating-point NaN.

## Fixtures and scientific conventions

All models are isolated TCB. `@` inputs are barycentric TCB; PKS SATs use the
accepted TT(TAI), IF99, DE405, IAU2000B, dispersion/solar-wind/Shapiro and
meteorology-fallback profile. Every observation participates in its original
order; no cuts, tracking, TZR activation, constraints or noise model are added.
The first input TOA sets the signed reduced-phase reference. Mean removal is
unweighted even for MODE 1. TOA uncertainties are microseconds in TIM and
seconds in the solver. ZERO and F1-F3 retain source fitting coordinates.

- `zero`, `f0`, `f1`, `f2`, `f3`, `subset`, `all`, `all_mode0`: the mandatory
  offset, every individual spin term, nonconsecutive F0/F2 and the full set;
  nonzero initial derivatives, heterogeneous errors, both modes/rescale
  settings, defaults, repeated MODE/WEIGHT controls, NITS 2 and unsorted data.
- `pks_seven_mode{0,1}`: a copy of the accepted seven-TOA input, with selected
  F0 and deterministic heterogeneous uncertainties. No phase-connection or
  truth-recovery assertion is made for that original long-span input.
- `pks_all{,_mode0}`: 16 phase-connected epochs spanning both sides of PEPOCH,
  all four spin terms, both modes and two iterations.
- `zero_derivative`: all BBATs equal PEPOCH, selected zero-valued F3 and a zero
  derivative column. `timing_square_{false,true}` preserves the finite/norescale
  versus NaN-rescaled square fit and the source callback predicate.
- Solver controls include the central six-row matrices, exact dependence, a
  zero column/all-zero matrix, diagonal modes below/above/exactly at the cutoff,
  square pairs and three defined overflow cases. Central `source_probe.cc` is
  independently compiled and its output must match the saved central result
  byte for byte.

`truth.par` is distinct from all starting models. Seed epochs and perturbations
are explicit in `generate_reference.py`. Twelve source-driven SAT iterations
subtract the raw residual/86400 for rows after the reference TOA. The final
alignment maxima are `2.67310712701473348214475563397010884e-25 s` for `@` and
`2.64906911143225024448815540606761652e-25 s` for PKS. A declared deterministic
0,+3,-2,+4,+1,-5,+6,-3,+2,-1,+5,-4,+1,+2,-2,+3 microsecond sequence is then added.
These inputs are frozen before fitting. The comparator checks raw prefit
predictions before accepting fit outputs. Agreement with TEMPO2 does not prove
recovery of the truth coefficients or validate the physical model.

Each independent end-to-end run gets a fresh Rust runtime, matching the
reference's isolated process. Reusing a clock-path cache across different
fixture histories is not the same experiment. Within a fit, the runtime is
reused through every pre/post evaluation and iteration, following the source
lifecycle. Cross-profile cache-history equivalence remains broader coverage.

## Comparison and source failure boundaries

`compare.py` implements decisions 0007/0008 using 90-digit Decimal arithmetic,
source column scales, physical timing-effect scales, normalized raw covariance,
exact shapes/counts/controls and exact nonfinite classes. Decimal strings at
binary64 boundaries are decoded back to binary64 for exact identity checks;
different round-trip decimal spellings alone are not numerical differences.
All finite gates are those adopted prospectively by decision 0007.
`test_comparator.py` injects sign/order/weight/centering/covariance/shape/rank/DOF,
iteration, stage, identity, nonfinite and just-above-gate mutations.

`reference/boundaries.json` records isolated zero/tiny-weight processes and the
underconstrained source call. `n < p` leaves sentinel estimates untouched;
there is no numerical solution to export. Zero/tiny weights cause a low-level
SIGSEGV or an eventual 30-iteration exit. Inspection shows nonfinite SVD state
can first reach the invalid `w[-1]` access. Rust stops at that explicit unsafe
source boundary. A termination after undefined access is not a defined
numerical outcome to emulate.

The exact dependent-column fixture exposes a mode-order difference between
Rug128 and binary128. Decision 0010 defines paired-mode comparison, but the
user's latest direct answer was "Keep exact order gate and report it unresolved".
The verifier therefore retains the strict raw-order gate. Paired-mode analysis
is recorded as diagnostic evidence only, including each engine's exact binary
cutoff/flag validation and the comparison permutation. Production arrays and
physical-coordinate checks are unchanged. The full verifier intentionally exits
nonzero for this unresolved gate; it does not label M2 accepted or all checks
passed. See `rust/M2_REPORT.md` and `verification/m2/open_gate_failures.json`.
