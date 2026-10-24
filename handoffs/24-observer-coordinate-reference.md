# Assignment X: TEMPO2 observer-coordinate reference

Work in `/Users/Eric.Dong/Downloads/tempo2rust`. Read HANDOFF.md, current bug policy 0053, generated-output policy 0039, numerical gate 0012, accepted J/M/S/U reference context and [contract 0057](../decisions/0057-observer-coordinate-reference-stage.md).

Own only new `validation/observer_coordinates/`. You are not alone in the workspace: W owns `rust/` and `validation/terrestrial_resources_port/`; central owns design documents and milestone acceptance. Preserve others' changes and all accepted evidence. Exclude W's evolving paths and central documents from immutable preservation membership.

Work in dependency order:

1. Inspect unchanged pinned get_obsCoord.C, vectorPulsar.C and tai2ut1.C. Freeze all nine routines, branches, callers, initialized/retained state, native-invalid boundaries and the actual SOFA/clock/EOP/utility dependency closure.
2. Build the original translation units and real dependencies with measured binary128 and pinned Fortran sources. Keep lmst's original public-overload visibility. Record actual source/configuration/toolchain/runtime/input/executable identities.
3. Establish direct helper, pulsar vector, UT1 reader/cache, full driver, satellite and IMAG histories. Preserve all initialized output tails, shared resources, diagnostics and failure prefixes. Classify undefined operations before executing fixtures.
4. Verify fresh replay with exact membership, nested types, state, streams and identity; add meaningful controls and compiled mutations with real witnesses. Measure coverage separately and expose differences and gaps.
5. Deliver `validation/observer_coordinates/REPORT.md`, maps, reproducible commands, local evidence paths/hashes, limitations and staged Conventional Commits.

The governing contract identifies required cases and known subtleties. Reuse established harness patterns; do not replace native dependencies with fake outputs or enlarge scope into Rust implementation. Original native behavior stays independently reviewable. Small demonstrated bugs may have separately identified corrected experiments under 0053; later Rust corrections require nearby comments naming the original source defect and fix, a focused regression and explicit comparison expectations.

**Do not commit reproducible JSON, compressed results, generated fixtures or other large evidence.** Keep them local and ignored under 0039. Do not edit canonical TEMPO2, the independent reference repository, accepted runners or W's code. Central owns scientific/numerical decisions and acceptance.

## Dispatch prompt

Implement Assignment X under decisions/0057-observer-coordinate-reference-stage.md and handoffs/24-observer-coordinate-reference.md in /Users/Eric.Dong/Downloads/tempo2rust. Own only validation/observer_coordinates/. Establish bounded original native evidence for all nine selected get_obsCoord.C/vectorPulsar.C/tai2ut1.C routines with actual pinned SOFA, clock, EOP, observatory and numeric dependencies. Freeze maps first; preserve translation-unit overload visibility, caller/retained state, UT1 cache histories, satellite flags/polynomials/offsets, IMAG shared mutations and exact diagnostics. Keep source-invalid operations unrun. Deliver fresh replay, actual identities, meaningful controls/compiled mutations and measured coverage with explicit limits. W remains sole Rust owner; do not alter its files or accepted evidence. Follow 0053 for separately documented bug-correction evidence and 0039 for local-only generated artifacts. Commit explicit owned paths with Conventional Commits and return REPORT.md for central acceptance.
