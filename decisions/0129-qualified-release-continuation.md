# Qualified release goal and continuation

Approved by the user, 4 November 2026:

> Agreed. Create the new chat with this goal and a complete handoff. Keep native
> dependencies documented, prioritize performance and scientific qualification,
> and start with a minimal GUI.

## Approved objective

Deliver a qualified TEMPO2-Rust release for macOS arm64 and Linux x86_64 that
supports the intended core/toolkit workflows, preserves agreed scientific
contracts and predictor C compatibility, meets explicit performance budgets
against original TEMPO2, and provides a minimal interactive timing workflow.
Keep native dependencies documented and all scientific computation owned by the
Rust engine.

Rust owns scientific model/state logic and the public workflow. Documented
native numerical libraries, the existing predictor ABI adapter and a bounded
graphics adapter are allowed behind controlled interfaces. This goal does not
require removing every C/Fortran dependency or recreating the legacy `pulsar`
structure/plugin ABI. Existing SOFA/GMP/MPFR and optional backend use must be
described honestly; it is not evidence of a fully native-code-free build.

## Delivery sequence

1. Establish one current implementation/acceptance baseline, a scientific
   failure ledger and matched native/Rust performance baselines. Set explicit
   workload-specific runtime and memory budgets at that checkpoint; numerical
   budget values have not yet been agreed.
2. Prioritize measured predictor arithmetic/reconstruction improvements, then
   timing/fitting bottlenecks. Preserve precision and source operation order;
   independently verify outputs and measure each claimed improvement.
3. Continue scientific qualification in parallel through one compatibility
   stream. Material discrepancies block the affected release claims. Small
   presentation differences must not serialize unrelated implementation.
4. Start a minimal GUI with residuals, error bars, zoom and TOA inspection;
   then add selection/exclusion, refitting and saving through the same Rust
   engine. Choose one small frontend after checking mature options and actual
   platform support. No GUI framework has been selected by this approval.
5. Qualify installed workflows on both target platforms with documented
   dependencies, coverage, limitations and benchmark results.

The source-faithful policy in 0099 remains binding: demonstrated native bugs
and demonstrably better Rust implementations may be corrected/improved with
adjacent source references, rationale and focused evidence. Precision,
scientific assumptions, output conventions and acceptance tolerances still
require explicit central decisions. User-adopted 0120 and 0127 remain binding.

## Handover boundary

[Handoff 44](../handoffs/44-qualified-release-continuation.md) is the entry point
for the new continuation chat and supersedes earlier live-status descriptions
in HANDOFF.md. It inherits coordination, design and acceptance responsibility
for this goal, using separately scoped implementation assignments. Existing
AL/AO ownership must be respected when releasing further work.

The previous central goal in chat `01a153dd-5f85-7e42-a456-372a19347c85` remains
paused and unfinished; its old objective's blanket bug-preservation wording is
superseded by the user's later policy. Leave its monitor paused. The successor
starts this approved goal in its own chat without a token budget.

At handover, main is clean at `483ced4b15a767d157d1b3ff40f964a78a35eaa9`, before
this documentation commit. Linux run 41392963741 at `9991e80` has completed
with failure: the saved log has 86 passing and 11 failing stages, none blocked.
Its full artifact/acceptance review remains pending. It did not execute the
later owner-reuse continuation. The 0127 central checks passed before the pause,
but the formal bounded acceptance entry is still due; 483ced4's successor
Linux preparation and AO assessment 50d1b06 also await central review/release.
This decision accepts the new goal, not those pending deliverables or full
migration.

Existing permission covers coordination with implementation chats and later
centrally reviewed AL publication/reruns within 0101's qualification scope.
It is not blanket authorization for public release or scientific-policy
changes. Conventional scoped commits remain authorized. Generated validation
JSON, logs, binaries and large traces remain outside Git.

Temporary evidence retained for this handover is indexed under ignored
`decisions/evidence/release-qualification/build/continuation-20261104/`.
The preservation manifest SHA-256 is `02d0124aaec663b5aaa9f88e0372e753d3b5ec30727ba28ee1100007bedad463`.
Copy/hash verification preserves provenance; it is not a new scientific run or
independent acceptance of the supplied assessment.
