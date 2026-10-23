# JPL source reference evidence

Assignment H is released by [decision 0011](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0011-jpl-reference-stage.md). Read it, HANDOFF.md and decision 0008. Work in `/Users/Eric.Dong/Downloads/tempo2rust` and own only `validation/jpl/`, including disposable builds, probes, declared inputs, reference outputs, coverage manifest and `REPORT.md`.

You are not alone. E owns `rust/` and `validation/fitting/`, F owns `validation/coverage/`, and G owns `validation/binary/`. Preserve their edits and commits. Canonical `tempo2/` and independent `validation/reference/` are read-only. Do not modify the retained C reader, Rust code, central decisions or another assignment's evidence.

Work in dependency order:

1. Verify pinned source identities; map all implemented JPL entry points/helpers, header layouts, types, units, output mutations and caches. Identify declarations without implementations and undefined state without filling it in.
2. Build source-driven probes for the real DE405 file and declared synthetic layouts. Cover ordinary and boundary interpolation, targets/centres, optional quantities, direct state lists, getters/constants, both byte orders and stateful query sequences as specified in decision 0011.
3. Extend the existing cache-failure evidence to defined output bytes/numbers and file lifetime/mutation behavior. Run unsafe or terminating source cases in isolated processes; export only defined values/classes and exact statuses. Preserve the source's cache-before-read defect and any other observed deterministic quirk.
4. Verify regeneration, coverage and meaningful artifact mutations. Return a report with actual commands, precise correspondence, observations, remaining coverage and decisions needed for a later safe Rust port.

The user requires preserving TEMPO2 behavior, including defects, until the overall transfer is finished. Do not repair source bugs, use a cleaner interpolation algorithm, silently initialize canonical unwritten state or turn defined nonfinite results into successful finite values. Clearly label controlled initialized-state kernel probes so they are not mistaken for evidence about the canonical initializer.

Commit explicit owned paths at coherent stages with Conventional Commit prefixes. This is reference preparation; a later central release assigns Rust ownership and numerical acceptance gates. Do not declare the JPL port, M2 or the overall transfer accepted.
