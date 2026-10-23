# Binary model reference evidence

Assignment G is released by [decision 0009](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0009-binary-reference-stage.md). Read that contract, HANDOFF.md and decision 0008 before starting. The central chat owns model contracts and milestone acceptance; this chat supplies executable reference evidence for later implementation.

Work in `/Users/Eric.Dong/Downloads/tempo2rust`. Own only `validation/binary/`, including its scripts, inputs, reference outputs, schema/coverage manifest and `REPORT.md`. You are not alone: E writes `rust/` and `validation/fitting/`, and F writes `validation/coverage/`. Preserve their work. Canonical `tempo2/` and independent `validation/reference/` are read-only. Build or run only in your own disposable directories; do not overwrite prior manifests or goldens.

## Work sequence

1. Verify source/reference identities and inspect the actual binary dispatch, each of the 14 model entry points, reader units/defaults, fit derivatives and update callbacks. Record native precision, wrapper narrowing, required geometry/state, branch/parameter/subindex coverage and helper dependencies. Keep TEMPO2 notation and identify the counterpart for every new harness file.
2. Create independent source-driven probes and declared full-decimal fixtures for each model. Record native delay and fit-callback results separately, before/after state for update/mutation probes, and explicit defaults/presence flags. Cover ordinary finite cases and relevant phase/reference-epoch branches. Inventory unexercised optional paths instead of claiming complete model coverage from one case.
3. Export the integrated residual/phase path, including binary delay insertion, original-order phase reference, mean-removal settings and a PKS propagation example. Preserve source mutations and known bugs. Isolate failure probes and encode nonfinite results explicitly. Keep the existing DDH upstream failure distinct from port-compatibility evidence.
4. Verify reproducibility, fixture/source/resource hashes, model/label coverage and meaningful schema/export mutations. Write a report with commands, actual observations, precision/units/state maps and concrete remaining coverage. Do not invent Rust comparison results or decide acceptance thresholds.

Reuse the accepted A2 build read-only where it matches the selected source path. Record its initializer correction separately from the unmodified canonical target, and do not introduce new source patches. Reuse existing export patterns where useful, without linking production Rust to the legacy engine or changing other assignments' artifacts.

The user requires existing TEMPO2 behavior, including defects, to remain unchanged during transfer. Capture and document differences from analytic expectations; do not fix them. In particular, preserve SINI mutation, source-specific PB update units, binary precision boundaries and the actual BT EDOT behavior. Do not turn source-supported clamps/fallbacks into strict errors.

Commit explicit owned paths at coherent stages using the existing Conventional Commit convention. Return `validation/binary/REPORT.md`, reproduction commands, commit IDs and outstanding limitations in your final response for central inspection. Do not edit central decisions, implement Rust models or declare a milestone accepted.
