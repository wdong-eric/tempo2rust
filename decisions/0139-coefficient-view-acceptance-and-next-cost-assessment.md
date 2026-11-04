# Accept the macOS coefficient view; assess remaining predictor cost

Central review, 4 November 2026, under [0136](0136-macos-call-local-coefficient-view-release.md).

Accept production `a1851c55fcc0b1ac733b064e631bfbbd2e0118d5`, witnesses
`ae9bf27130ab5b299de4f52e404b04279508c5c7` and delivery
`4f37bc378d51fac94b6bc3c12719e45131cfa612`, integrated by
`612073e7a88f20a6be0a7c76d715419613f9cd93`. This accepts the bounded macOS
implementation and its measured improvement. Linux qualification, inherited
native discrepancies and the release performance requirement remain open.

## Reviewed contract and evidence

Central inspected the shared Clenshaw recurrence, generic coefficient accessor,
private synchronous evaluator and native ABI assertions. The macOS slice is
created only after the existing shape/null gates and ends before output writes.
Every call sees current caller-owned storage. Callback and derivative paths
retain owned snapshots; Linux retains owned Rug64 values and wide exponents.
The shared model expressions, source rounding/FMA sites, Darwin NaN input
convention, public Float owners/borrows and error ordering are preserved.

Independent review rebound 1,980 files (304,270,888 bytes), 61 external identities,
1,738 Git source/installed-source bindings, five prior manifests and twelve
immutable baseline products. It replayed eight native C/C++ profiles and 24
corruption controls, exact 30,258-line C output, 5,880 direct edge results,
28 finite results and 38 legacy products. The new coefficient witness agrees
across ten profiles and 8,516 lines, including thirteen ordered diagnostics.
Public borrow, callback snapshot, derivative alias, caller-exclusive sanitizer
and cleanup evidence remains intact. All 18 focused predictor tests also pass
freshly in the integrated tree; saved debug/release runs each pass the same 18.

All 91 raw measurement records were reparsed. Complete C phase/frequency pairs
take 0.473858 s before and 0.209635 s after per 100,000 pairs, a 2.2604-fold
improvement. Allocation requests fall from 134 to 62 per pair; requested bytes
fall from 6,784 to 1,920. Median peak RSS is 2.0625/2.03125 MiB. Direct owned
leaf/model timings differ by less than 1% and establish no useful speedup.
Native takes 0.004158 s, so this candidate remains 50.42 times slower on the
measured fixture. This is not acceptance of a release budget. The unanswered
numeric budget proposal in [0132](0132-release-performance-budget-proposal.md)
remains pending.

Reviewed evidence is retained outside Git at
`validation/predictor_performance/build/coefficient-view-reviewed-20261104/`.
Central scripts, receipts and fresh test log are under
`decisions/evidence/release-qualification/build/ao-coefficient-central-20261104/`.
Delivery SHA-256:
`b30c5b2d20a1f8670c74d57cce6d293db9ab6bd450f8cc94270327dd017cabc3`.
Independent review receipt:
`03a014a36f2157c7ec6bfd403e204ff509348d8124b04b1e956c9dc9934fa2c6`.
Retention receipt:
`91c39ffe6f44f0908aed53c7ba814b226a1f8b1cc9e87d347d174203fe6020d9`.
These distinguish saved-producer evidence from fresh integration checks and
do not claim new Linux execution or closure of T1/polyco differences.

## Next bounded assessment

Release AO to profile and assess the remaining complete C evaluation cost on
this immutable installed baseline. AO retains ownership in its isolated clone
and must preserve other work. This authorizes diagnosis, a concrete design and
qualification recipes, not further production changes.

1. Reuse the established complete-call profiler and output controls. Identify
   remaining scalar transport, model normalization/dispersion/frequency,
   recurrence and result-publication costs from actual stacks or a narrowly
   justified diagnostic. Inclusive stack counts overlap; do not add them or
   predict a speedup from them. Coordinate any timed window centrally.
2. Check existing numeric owners, conversion methods and private platform
   arithmetic before proposing custom machinery. Explain which allocations and
   conversions are required by the public Float contract and which are temporary
   internal work. Compare the smallest reuse-based alternatives, retaining one
   canonical source expression path, with their rounding and ownership costs.
3. Return a bounded proposal with affected files, exact operation/rounding/class
   semantics, C lifetime/error behavior, output controls and portable Linux
   checks. Preserve macOS53/Linux64 range, public owners/borrows, T1 and Rug128
   timing. No cache, persistent native borrow, duplicated scientific evaluator,
   new scientific C kernel or changed tolerance is released by this assessment.

AL's current U diagnosis and recurrence workflow preparation remain separately
owned. The next Linux candidate should also qualify this accepted coefficient
pair through the existing integrated runner after central review of the concrete
wiring. No push or dispatch is authorized by this entry alone. Native GUI
verification remains deferred while the desktop is locked.
