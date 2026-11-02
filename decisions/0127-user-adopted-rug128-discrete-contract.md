# User-adopted bounded Rug128 discrete contract

Central decision, 2 November 2026. The user explicitly selected **“Adopt bounded
Rug128 contract (Recommended)”** for the four 0049 utility boundaries and eight
0077 SIMUL histories in [proposal 0126](0126-discrete-precision-contract-proposal.md).
This adopts that bounded contract. It does not accept a completed verifier,
affected milestone, other mismatches or the full migration.

Keep canonical Rug128 storage, source-ordered expressions and the strict SIMUL
threshold. The approved outcomes differ from native binary128 decisions at the
demonstrated boundaries. Describe that intentional numerical-contract departure
explicitly; do not label it native equivalence, a translation fix or universal
accuracy improvement. User approval supersedes the pending disposition in 0049
and 0077 only for this specified work. The remaining gates stay unchanged.

## Exact scope

- The four identical-input utility witnesses require `nlong` results `0, 0` and
  the positive/negative nonzero remainders given losslessly in AL's
  [brief](../validation/release/DISCRETE_BOUNDARY_DISPOSITION.md). Retain native
  outcomes `1, -1, +0, +0` as the original evidence. Preserve all twelve unaffected
  cases, signed-zero/nonfinite behavior and defined conversion-domain controls.
- The eight SIMUL histories are `simul_123`, `simul_124`, `simul_127`, `simul_128`,
  `simul_139`, `simul_140`, `simul_143`, `simul_144`. Require exact decisions
  from their declared Rug128 stored inputs and source predicate, with complete
  observation state, ordering, repeat behavior and diagnostics checked. Retain
  the original sixteen scalar deletion differences and their sixteen duplicated
  whole-record manifestations. The same rule is not a blanket allowance for
  other SIMUL discrepancies.
- Derive target expectations independently from bound input/rounding evidence;
  never record current Rust output as the expected answer. Source expressions,
  rounding sequence, threshold values and integer-conversion meanings stay
  unchanged. No epsilon, field omission, `<` to `<=` change or relaxed continuous
  tolerance is authorized. New unexplained or corrupted outcomes must fail.
- A caller-level pulse-count witness is needed before claiming a timing-path
  correction. This decision establishes no observed end-to-end pulse slip.
  The 122 continuous P records, DDH and other fitting/discrete gates remain open.

## Implementation release to AL

AL continues in its existing compatibility chat and main checkout. The frozen
Linux candidate `9991e80` and run `41392963741` remain untouched. Prepare one
coherent successor batch; do not publish or dispatch it independently.

Ownership is limited to adjacent explanatory comments in
`rust/src/tempo2_util.rs` and `rust/src/selection.rs`, the relevant current release
comparators/adapters and focused controls under `validation/release/`, and the
current behavior/qualification report. Reuse the existing 0049/0077 data and
source-comparison functions; historical source runners, references and receipts
remain immutable. If another production arithmetic change appears necessary,
return the demonstrated reason before editing it. AO's numeric/evaluator files
and ongoing isolated work are outside this ownership.

The problem is explicit floating-point semantics at discrete boundaries.
Established tools already suffice: the pinned Rug operations define production
rounding; lossless native decoding and exact rational/directed rounding witnesses
establish the expected cases. Assess and reuse those facilities rather than add
a custom numerical backend, shadow timing engine or generic tolerance system.

Adjacent comments must identify the pinned TEMPO2 source counterpart, explain
113-bit versus Rug128 rounding/storage and the adopted outcome, and cite 0127.
For SIMUL distinguish initial SAT quantization from arithmetic rounding: the
saved separation fits exactly at each precision, so late rounding is no remedy.

Verification must show both the retained original failed comparison and the
new declared-contract result. Reject wrong decisions, missing/extra/reordered
state, changed thresholds, class/sign errors and unrelated corruptions through
the actual comparator. Reuse existing exact-threshold, neighboring, reverse and
repeat controls; add only missing controls that protect plausible regressions.
Run the affected Rust utility/selection tests and replay the applicable saved
macOS/Linux evidence with source identities preserved. A changed checker is a
new comparison receipt, never a relabeled old success. Do not rerun an unchanged
full engine campaign solely for this contract update.

Commit authored changes conventionally and leave generated evidence ignored.
Return the exact commits, complete scope/results and any surviving failures for
central review. Central retains numerical decisions and milestone acceptance.
