# Release the bounded macOS predictor recurrence optimization

Central implementation decision, 4 November 2026, under the user-approved
[0129 continuation](0129-qualified-release-continuation.md). Release AO's
[delivered assessment](../validation/predictor_performance/NEXT_CPU_ASSESSMENT.md)
as one implementation slice. This authorizes implementation and qualification;
it does not accept an unimplemented speedup or alter numerical policy.

## Reviewed basis and established facilities

Assessment commit `50d1b06dc49c68a26babfe95fb7a21fcec5a8dec` targets the existing
canonical Clenshaw recurrence. Central inspected `cheby2d.rs`,
`predictor_number.rs` and `ordinary_long_double.rs`, rebound its eighteen
evidence files, sixteen contract-source identities, five installed input/artifact
identities and original manifests, and replayed `profile_costs.py`. The saved
3,846-stack attribution reproduces exactly; all three retained complete-output
files agree on 30,258 lines. This is a saved-profile review, not a new timing run.
Receipt identities are recorded in [0130](0130-continuation-baseline-bounded-review.md).

Clenshaw evaluation is an established recurrence already implemented here. The
change should specialize temporary arithmetic, not introduce another evaluator.
The existing Rug assignment path remains the Linux implementation. Rug local
scratch/MiniFloat cannot remove all measured MPFR/conversion cost while directly
preserving the shared `as_float() -> &Float` contract. Standard Rust binary64
matches the adopted macOS ordinary format, and its documented
[`mul_add`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.mul_add)
supplies the required single rounding at the existing fused sites. Rust does
not promise arithmetic NaN signs; explicit existing sign/class rules and tested
equivalence are required. APFloat, C arithmetic and custom assembly are not
selected. No dependency or wider numeric-owner redesign is justified here.

## Authorized implementation and invariants

Keep a single canonical recurrence with a small private compile-time operation
adapter if needed. On macOS arm64, use f64 temporary values at the existing
53-bit rounding points. On Linux x86_64 retain Rug64, its wider exponent range
and separate operations. Preserve the public Float-backed `PredictorNumber`,
`as_float` lifetime, public signatures, source order, explicit contraction sites,
range/subnormal behavior, ties, signed zero, unary-negation signs and existing
operation-specific nonfinite results. Rug128 timing remains unchanged.

Conversion belongs at validated input/use and return boundaries. Construction,
differentiation, model normalization/dispersion, T1 arithmetic, parsing, public
formatting and C transport retain their canonical paths. No global Float-to-f64
switch, alternate evaluator, pointer/snapshot cache, scientific C kernel,
precision reduction or tolerance relaxation is authorized. C coefficient
mutation visibility remains required. Borrowing C coefficient storage may be
considered only after this change is independently measured and reviewed.

AO owns `rust/src/cheby2d.rs`, a strictly necessary private arithmetic adapter
and focused predictor tests, plus its performance recipes/report, in
`/private/tmp/tempo2-ao-predictor-20261102`. Preserve other work. Adjacent comments
must identify the TEMPO2 counterpart, arithmetic boundary and evidence-based
reason for the specialization. Minimize the adapter; do not duplicate validated
dimension/input checks or create public backend selection.

## Dependency-ordered acceptance plan

1. Implement and test primitive equivalence against frozen N operations:
   cancellation/ties, zero signs, normal/subnormal transitions, both overflow
   signs, NaNs/infinities and unary negation. Inspect optimized contraction
   sites and preserve the supported floating-point environment.
2. Reuse the exact 5,880-edge/28-finite direct and 30,258-line complete-C
   witnesses, mutation/restoration, affected construction/callback tests and
   installed `as_float` consumers. Keep the inherited T1 NaN-sign and polyco
   log10RMS failures explicit. Correctness failures reject the change.
3. Measure immutable before/after direct and installed-C routes with existing
   interleaved uninstrumented recipes in a coordinated quiet window. Count
   allocations separately; retain all samples, identities, outputs and RSS.
   Do not infer a speedup threshold from inclusive profile percentages.
4. Return scoped commits and a portable Linux unchanged-path qualification
   recipe. Central will review/integrate and AL will execute Linux qualification
   with a meaningful combined candidate. macOS evidence alone does not qualify
   Linux. No push/dispatch/public release is delegated to AO.

The release baseline's numerical performance budgets remain a separate central
checkpoint after the completed Linux artifact review. This slice must show a
substantial measured improvement without changing any adopted scientific gate;
an unsuccessful experiment is evidence to reassess, not grounds to weaken them.
