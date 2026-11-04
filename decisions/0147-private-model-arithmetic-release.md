# Release private predictor model arithmetic

Central implementation release, 4 November 2026, under
[0145](0145-temporary-reuse-acceptance-and-final-linux-wiring.md).
Accept AO's read-only assessment `1347538e732b1c15dabcd1bb9a8d7fbace666469`,
integrated as `fc2ff69`, and its
[bounded proposal](../validation/predictor_performance/MODEL_SCALAR_ASSESSMENT.md).
This releases implementation and qualification in AO's existing isolated clone.
It does not change the separately prepared AL Linux candidate or accept a
performance prediction, numerical budget or scientific discrepancy.

## Reviewed evidence and design

Central rebound 27 files / 3,148,981 bytes, 76 external identities, forty Git
source bindings and fourteen dependency-source bindings. Pinned dependency
files agree with their crate archives and lockfile checksums. Eight prior
manifests and eight accepted products remain unchanged. Independent parser
replay reproduces the current and previous saved profiles and their controls.
Actual load commands, sampled library/client UUIDs and both 30,258-line output
controls agree with accepted `e1d34d8`.

The cleared 10:50:42.999–10:50:57.249 UTC window contains the fresh plain and
sampled runs only. The current trace contains 3,840 stacks: model entry 2,476,
selection 1,268, scalar reads 820, recurrence 335 and publication 23. These
inclusive counts overlap; no timing contribution is inferred from their sum.
The 7.051005/7.042596-second observations describe the profiling workload,
not a speedup. No model-entry vector-builder frames remain.

Reuse the existing private macOS binary64 Number and Linux N alias through
model normalization, recurrence and final scaling. This targets the observed
model work while preserving one expression path and one recurrence. Reuse
existing Rug operations as the unchanged Linux implementation and test oracle.
Changing public N storage, adding a numeric backend or optimizing selection
would add independent contracts and is unnecessary for this bounded slice.

The source account suggests 50 to 22 allocation requests per complete C pair,
retaining final N results and the current writer. That is unmeasured. The
accepted allocation observations remain 50 requests / 1,024 bytes per pair.

Assessment receipt SHA-256:
`678bbdf8bd2507952d37e3e2d2cbd0e38d67ecf38fbf0751fa2ac82ad0f523a1`.
Retained packet:
`validation/predictor_performance/build/model-scalar-assessment-reviewed-20261104/`.
Central receipt under the qualification evidence directory's
`build/ao-model-assessment-central-20261104/`:
`c5486fc900e967edf6e5b5b9ef3eb179bebfe7afae8a1632f875aa852e0ea5cb`.

## Dependency-ordered implementation and gates

1. In `cheby2d/evaluation.rs`, add only the Sub/Mul/Div operand forms needed by
   the existing model expressions. Preserve signed zeros and finite rounding.
   Add/Sub/Mul and propagated NaNs keep the N oracle's positive NaN sign;
   invalid division of non-NaN inputs keeps the operand-sign XOR. Unary minus
   and explicit source FMA retain their existing semantics. A synchronous input
   bridge may remove repeated platform conversion: stack copies on macOS,
   original borrowed N inputs on Linux, with no reference escaping.
2. In `cheby2d.rs`, carry the private Number through the existing model helpers
   and same recurrence. Publish one N at the public result boundary. Preserve
   every subtraction, separate multiply/divide/add, coefficient half weight and
   explicit FMA. Keep the repeated frequency extent subtraction and division
   by 86400; do not combine denominators or substitute reciprocals. Linux must
   add no input clones, binary64 narrowing or extra rounding pass.
3. In `predictor_ffi.rs`, first qualify model arithmetic with existing N reads.
   Then use the already qualified macOS ABI to copy seven ordered native inputs
   into the private representation. This permits only the necessary bounded,
   call-local typed pointer reads, with adjacent alignment/extent/lifetime
   invariants. Keep the unused frequency DM slot and scalar validation before
   coefficient validation. Apply the existing native NaN-input convention.
   Linux retains its N reader. Finish scalar copies and coefficient borrows
   before the unchanged output writer; preserve all error precedence and
   untouched outputs on failure. No unsafe owner, cached pointer or C scientific
   arithmetic is authorized.
4. Extend the existing private arithmetic, model/coefficient and public-borrow
   witnesses. Compare raw private division signs against unchanged N, including
   every sign combination of zero/zero and infinity/infinity, input NaNs,
   subnormal midpoints, overflow, cancellation and anti-contraction values.
   Check complete expressions separately: leaf ends in FMA, phase in addition,
   frequency in division by finite 86400. Their positive-NaN result invariant
   permits the existing output conversion without a general repair branch.
5. Vary all seven C model scalars, both polynomials, degenerate and adjacent
   extents and coupled cancellation. Include output aliases into scalar or
   coefficient backing, mutation/restoration, replacement and ordered errors.
   Keep live public Float borrows, callback/derivative snapshots and T1 checks.
   Run the full existing temporary-reuse recipe against immutable `e1d34d8`,
   adding these witnesses. Inspect actual installed machine code/IR for the
   required separate operations and source FMA sites. Actual Linux wide-range
   and midpoint executions in both profiles remain necessary for acceptance.
6. After correctness, observe allocations separately. Request one central quiet
   window for the unchanged seven-sample direct/full-C workload. Retain every
   sample and identity; report combined effects without per-edit attribution
   unless separately measured. Return scoped commits and bound evidence for
   central integration review.

The pinned MPFR sources establish the oracle's division and conversion sign
behavior. [Rust's floating-point reference](https://doc.rust-lang.org/std/primitive.f64.html)
does not guarantee arithmetic NaN signs, so hardware observations alone cannot
implement that policy. [MPFR's range documentation](https://mpfr.org/mpfr-current/mpfr.html)
explains the previous rounding direction needed to avoid subnormal double
rounding. These are documented obligations; new implementation correctness and
performance still require the stated tests on the actual toolchains. Preserve
the qualified nearest-even/gradual-underflow environment without fast-math or
control-register changes.

AO owns the three named production files and narrowly required predictor tests,
private-boundary documentation and validation recipes in its isolated clone.
It is not alone: preserve AL's Gaussian/U qualification work and root GUI/event
work. `predictor_number.rs`, ordinary arithmetic, public types, selection,
general scalar transport and Rug128 timing remain unchanged. No push, dispatch,
new cache, precision/tolerance change or inherited-failure waiver is released.
