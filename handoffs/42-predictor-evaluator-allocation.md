# Assignment AO continuation: canonical predictor allocation cost

Released by central 0124 on 2 November 2026 to the existing AO chat
`01a19a86-1e28-7962-b3ff-d81c5cd16ad8`. Work in the same isolated clone
`/private/tmp/tempo2-ao-predictor-20261102`, initially at delivered `f077292`.
Transport production `0bd8e2c` is now integrated and frozen for Linux follow-through.
Read current 0124/HANDOFF.md in `/Users/Eric.Dong/Downloads/tempo2rust` first.
You are not alone: AL owns main and Linux qualification. Preserve all other
work, and do not modify main or the completed AO evidence.

## Purpose and evidence

The reviewed transport improvement reduces the installed C evaluation median
from 4.668811 s to 1.732665 s; native is 0.004530 s for the matched 100000-pair
workload. After transport, canonical Cheby evaluation occurs in about 66% of
sampled stacks. External requests remain 639 per phase/frequency pair. These
measurements justify investigating safe numeric-owner/scratch reuse; they do
not establish which allocations can be removed or the benefit of a proposal.

Deliver the next measured reduction using the same canonical Rust evaluator.
Confirm allocation sites and the direct Rust leaf versus full installed C route
before selecting a change. Reuse the completed profile where sufficient and
add only the observations needed to distinguish remaining cost. Do not repeat
the completed transport investigation or full timing/fitting campaign.

## Ownership and constraints

Own `rust/src/predictor_number.rs`, `rust/src/cheby2d.rs`, and focused predictor
tests. Confine leaf edits to equivalent ownership/allocation changes in evaluation
and directly necessary numeric operations. T1 arithmetic, construction algorithms,
parsers, model selection and public interfaces remain unchanged unless a measured
shared-operation effect is covered by the existing affected checks. Existing
transport remains the baseline; central must review any proposed further C/FFI
or public API change before it is made.

Own new authored continuation recipes/evidence under
`validation/predictor_performance/` and a new
`rust/PREDICTOR_EVALUATOR_PERFORMANCE_REPORT.md`. Keep the completed transport
report and raw evidence immutable. Extend/reuse its small runners rather than
create a general benchmark framework. A necessary shared runner correction
must preserve the original executed recipe and exact old evidence identity.

Assess the pinned safe Rug assignment/owned-operation facilities first. The
current owned arithmetic operators still delegate to borrowed operations that
allocate a new Float. Determine whether consuming/mutating existing owners or
local scratch storage reduces actual allocations. Preserve `ordinary_long_double`
rounding semantics and first-rounding direction at subnormal boundaries; that
shared timing/event module is outside your ownership. Any change there requires
central scope review before implementation.

Retain every scientific expression and operation order, source-matched macOS
FMA versus Linux separate operations, rounding points, ordinary 53/64-bit
precision, signed zero/nonfinite behavior, subnormal/overflow semantics and
Rug128 timing. No lower-precision intermediate, algebraic reassociation, blanket
fusion, second evaluator, C scientific replacement or assumed f64 substitution.
Do not add a persistent/global scratch cache or pointer-only coefficient cache;
callers can mutate exposed native storage. Preserve caller exclusivity, callbacks,
error/recovery, partial state, foreign-thread cleanup and native layout.

Prefer safe local ownership reuse. Any necessary new unsafe boundary needs a
measured reason and adjacent source/ownership/lifetime explanation; it is not
assumed necessary here. Changes to numerical policy, model assumptions, output
conventions or tolerances require a separate central decision.

## Execution and verification

1. Freeze the delivered transport baseline and full matched workload. Attribute
   remaining numeric/evaluator allocation cost with a bounded observation, then
   choose the smallest justified change. Show which existing library operations
   preserve the exact rounding contract and which would change it.
2. Implement one canonical path. Keep comments identifying the original TEMPO2
   counterpart, measured reason and preserved operation boundaries. Avoid new
   configuration/modes, speculative generic abstractions and unrelated cleanup.
3. Compare immutable before/after builds using the existing complete installed
   C transcript and all legacy ABI/storage/locale/callback/mutation gates. Cover
   direct Rust evaluation, affected shared T1 uses and the explicit subnormal,
   signed-zero, nonfinite and overflow neighbors. Reject actual ownership/rounding
   regressions; do not add tests that only mirror the new implementation.
4. Measure allocation requests and direct-leaf/full-C loaded work separately.
   Reuse fresh interleaved uninstrumented timing and retain samples/outliers.
   Coordinate sustained timing with central; no benchmark/compile overlap.
   Claim only observed platform/workload benefits. Preserve all inherited native
   failures and the original transport benchmark without relabeling executions.
5. Deliver Conventional Commits, the new report, exact source/input/binary
   identities, and a bounded Linux recipe. Keep generated JSON, binaries and
   traces ignored. No push, workflow dispatch or new chat is authorized here.
   Central owns integration and acceptance. Do not delay AL's frozen transport
   qualification for this next stage.

Completion is an implemented, measured equivalent improvement with focused
scientific/ownership verification, or a concrete measured negative result and
recommendation if no safe beneficial change exists within scope. A plan or
assumed allocation saving alone is not a completed stage.
