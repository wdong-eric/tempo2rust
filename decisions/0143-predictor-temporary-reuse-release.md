# Release bounded predictor temporary reuse

Central implementation release, 4 November 2026, under
[0139](0139-coefficient-view-acceptance-and-next-cost-assessment.md).

Accept AO assessment `99448181a8fba74c19a7998a0a413f649b1d1949` and its
[three-file proposal](../validation/predictor_performance/REMAINING_COST_ASSESSMENT.md).
AO owns those predictor changes and their focused tests/recipes in its existing
isolated clone; preserve main's U/GUI work. This releases implementation and
qualification, not a speedup prediction or performance-budget acceptance.

## Evidence and choice

Central rebound 27 files (3,150,913 bytes), 64 external identities, 37 Git
bindings and eight dependency-source bindings. The inspected Rug/MPFR files
match their crate archives, whose hashes match the pinned lockfile. Fresh
parser replay reproduces both saved traces and controls; loaded-image UUIDs
match the actual accepted library/client. Both 30,258-line controls match the
accepted coefficient-view output. Six prior manifests and six selected products
remain unchanged.

The profile has 3,835 stacks: recurrence appears in 276 and result publication
in 17; model entry and segment selection appear in 2,661 and 1,099 respectively.
Inclusive counts overlap. The plain/sampled 8.328101/8.403810 s observations
support profiling, not a new performance comparison. Source accounting suggests
twelve removable requests from the accepted 62-request C pair, but this has
not been measured and is not an expected acceptance result.

Use mature Rug rounded primitive-operand operations. The pinned implementation
forwards to MPFR's stack-backed exact-double operands, so no new numeric owner,
unsafe layout, dependency or scientific evaluator is needed. Destination
precision, nearest-even mode and returned rounding direction remain explicit.
A broader hardware model-arithmetic path has additional Sub/Mul/Div class,
range and lifetime obligations and is outside this release.

Evidence is retained outside Git at
`validation/predictor_performance/build/remaining-costs-reviewed-20261104/`.
Original receipt SHA-256:
`d1d871940fa880d5ef3f953a3ac2aa99a48b600ef7db03857de3dc9ab8e3c8e9`.
Central review receipt under
`decisions/evidence/release-qualification/build/ao-remaining-central-20261104/`:
`781c69b90bae5ce3653b9142c19aa6c1a2d71ae95805299a205dba577e307e31`.

## Dependency-ordered change and gates

1. In `cheby2d.rs`, consume the dead `(start + end)` result through the existing
   `source_mul_add_owned`. Preserve selection operand order, platform contraction,
   `abs_double`, inclusive endpoints, strict tie comparison and the 1e6 sentinel.
2. In `predictor_ffi.rs`, replace only `t2r_model`'s seven-scalar Vec with an
   explicitly ordered `[N; 7]` using the existing fallible reads. Keep the unused
   dispersion read on frequency calls, partial-owner cleanup and all error
   ordering. Keep general `values`, coefficient borrowing, callbacks,
   derivatives, T1 and publication on their current paths.
3. In `predictor_number.rs`, change only the two `f64` RHS implementations in
   the existing arithmetic macro. Owned left operands reuse their Float through
   the rounded assign methods; borrowed left operands construct the result
   through the corresponding mixed incomplete operation and `with_val_round`.
   Retain the actual returned `Ordering` through existing range/subnormal
   handling. Preserve Add/Sub/Mul fresh-result NaN signs and Div's distinct
   input-NaN versus invalid-operation rule. Keep all N/N variants, public Float
   owners/borrows, Linux64 left operands and Rug128 timing unchanged.
4. Extend the existing operation/ownership tests against unchanged N/N oracles:
   all four operations, owned/borrowed inputs, live borrows, signed zeros/NaNs,
   infinities, maxima, normal/subnormal neighbors and first-rounding midpoints.
   Include actual Linux wide-significand/exponent checks in the portable recipe.
   Reuse selection edge cases and all retained C coefficient/error/callback/
   derivative and T1 count/class/sign/parser controls. Run narrow checks after
   each edit; one final combined qualification campaign is sufficient unless a
   failure or a per-edit performance claim requires additional execution.
5. Require exact retained direct/C/legacy outputs, installed consumers and
   caller-exclusive sanitizer/cleanup controls. After correctness and a centrally
   cleared window, run the unchanged seven-sample direct and full-C campaigns,
   observing allocations separately. Report actual counts and every sample;
   do not infer each edit's measured contribution from source accounting alone.

Return committed production, witnesses, report and retained evidence for central
integration review. Actual Linux execution remains required; a macOS cfg switch
is not a substitute. No new unsafe code, cache, persistent pointer, native C
arithmetic, duplicated model path, tolerance or NaN-policy change is authorized.
Inherited native discrepancies and the pending numeric budget proposal remain
open. Coordinate later combined Linux wiring with AL after the final candidate
is reviewed. No push or dispatch is authorized by this entry.
