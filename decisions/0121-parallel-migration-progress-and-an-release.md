# Parallel migration progress and AN release

Central decision, 2 November 2026. The user asked when major implementation last
advanced, instructed central to prioritize substantial progress, and delegated
how to organize it. Keep small differences with one implementation owner while
moving independent major work forward. This changes sequencing and ownership;
it does not waive scientific acceptance or declare migration complete.

## Evidence-based checkpoint

The last major user-facing implementation stage was AK's observation execution,
commit `81de1e6bb7d6f6cf1d17c075f402c301b87c9ce9`, authored 27 October 2026 at
17:25:44 +11:00. The same day also delivered native backend routes, familiar CLI
output, Rust APIs and the predictor C installation. General fitting, predictors
and analysis-toolkit stages landed on 26 October. Later feature commits mainly
added release builds/matrices, including full corpus/worker qualification on
30 October. Recent source fixes are useful scientific/platform corrections, but
are not a new large functional stage.

The broad port now includes PAR/TIM loading, barycentric timing, binary models,
local/global fitting, analysis/toolkit APIs, predictor generation/evaluation,
Rust and predictor-C consumers, native providers and optional observation workers.
This is implementation coverage, not proof of every source branch or release
readiness. There is no basis to invent another large feature merely to show
activity. The substantial remaining work is scientific qualification and usable
performance, followed by a fully qualified migration candidate.

AK measured about 1578 MiB peak RSS and 0.462 s in the selected Rust two-pulsar
fit, versus 161 MiB and 0.0403 s natively. Its standalone predictor C workload
also showed a large cost gap. Those are dated, scoped desktop measurements and
must be remeasured before attributing causes or claiming improvements. The
covariance-buffer hypothesis is concrete and still untested: each 10000-by-10000
f64 payload is about 763 MiB, and Rust may eagerly touch it. Dimensions, values
and scientific behavior must remain intact.

Actual Linux execution now exists: the complete fourth run executed all 69
stages, and the reviewed correction run passed 23/26 targeted stages. AL and
remaining scientific gates are unaccepted, but waiting for all small discrepancies
before profiling is no longer the user's preferred sequence. The latest request
supersedes the earlier post-AL waiting condition in 0098/0099 and draft handoff 40.
The practical AL-first purpose—obtaining actual cross-platform evidence—has been
served; qualification and independent optimization now proceed concurrently.

## Work ownership and priorities

1. **AN: memory optimization, starting now.** Release handoff 40 from exact
   baseline `dc435f53a64e4dd5c25cae6319841a28bd981fcb` in isolated clone
   `/private/tmp/tempo2-an-allocation-20261102`. Measure the covariance allocation
   and real pulsar initialization, then implement an equivalent allocation
   strategy only if evidence supports it. Preserve zero initialization, full
   dimensions, layout, ownership/COW, failure behavior and scientific outputs.
   Deliver matched before/after RSS, residency and timing, with both-platform
   qualification before a two-platform claim. The handoff grants the complete
   bounded measurement/implementation task; central will not require another
   permission checkpoint for an equivalent measured optimization.
2. **AL: one compatibility and qualification stream.** Retain the current Q,
   event-sign and adopted SVD-comparator work, plus the remaining fidelity ledger.
   Prioritize wrong scientific outputs, pulse/deletion/rank decisions, failed
   valid workflows, safety and artifact loss. Batch minor diagnostic, presentation
   and harness differences into coherent candidates. These remain tracked and
   may block final acceptance where required, but do not block unrelated AN work.
   AL stays in main and must not edit AN's matrix allocation ownership without
   coordination. AN does not edit AL's active modules.
3. **Next major performance stage.** After the allocation result, use current
   workload profiles to select CPU work, particularly the measured predictor C
   conversion/evaluation path and full fitting/initialization costs. This is the
   next planned substantial deliverable, not present authority for a speculative
   backend rewrite. Preserve source semantics and measure claimed gains.
4. **Central: integration and release milestones.** Review coherent deliveries,
   integrate disjoint commits and run the complete matrix on frozen candidates.
   Reuse valid unaffected evidence; do not trigger a full campaign for each minor
   wording or bookkeeping edit. Review actual scientific/operational/performance
   outcomes rather than document or commit counts. Monitor active Linux runs
   about every fifteen minutes, preserving the same run and failed evidence.

AN's isolated clone prevents main's in-flight fixes from contaminating its
before/after baseline. Exact commit/source/resource identities and quiet timing
windows remain necessary; isolation alone does not eliminate host-load effects.
Central reviews and integrates AN's commits. Neither owner publishes or dispatches
independently. Any needed publication authorization is assessed for a concrete
reviewed candidate, not sought speculatively before the work exists.

## Acceptance remains explicit

This is not a declaration that remaining differences are all minor. The prior
preprocessing deletion decisions, discrete utility boundaries and unresolved
fit/timing values are material scientific questions until evidence resolves them.
Keep the user-adopted 0120 paired SVD rule, Rug128, source algorithms/expressions,
existing tolerances, resource conventions, interfaces and selected platforms.
No scientific policy is relaxed to create apparent progress. Minor-output work
has one owner; major performance progress now has an independent active owner.

Observable next milestones are: a measured memory diagnosis and justified
implementation; qualified before/after benefit with unchanged products; a measured
CPU-priority decision; and a complete integrated migration candidate with explicit
remaining blockers. Full migration still requires the original scientific,
interface, platform and operational scope to be verified.


## Dispatch completed

Central created **Assignment AN: Covariance memory optimization**, local thread
`01a19a64-27ab-77c3-a36e-416defad0906`, and verified its active first turn. The
isolated clone is clean at the named baseline and has no publishing remote.
AN has begun source/baseline and residency-observer preparation. No allocation
improvement or measured benefit is claimed yet. AL was informed of the revised
priorities and partition; its in-flight main-checkout changes remain intact.
