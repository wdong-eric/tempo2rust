# Assignment AB: TEMPO2 propagation reference

Released by [decision 0063](../decisions/0063-propagation-dispersion-reference-stage.md). Work in `/Users/Eric.Dong/Downloads/tempo2rust`; own only new `validation/propagation/`. Y is the sole Rust writer, Z owns `validation/planetary_ephemeris/`, and AA is prepared but unreleased. You are not alone in the workspace: preserve their edits, both independent repositories and accepted evidence. Central owns shared design and acceptance.

Read HANDOFF.md, 0063, current bug policy 0053 and generated-evidence policy 0039. Use raw pinned source and accepted field/dependency contracts. The full scope is 25 routines across calculate_bclt.C, shapiro_delay.C, dm_delays.C, formBats.C, secularMotion.C, refphs.C, sw_delay.C, ifunc.C, shapelet.C and storePrecision.C. Do not substitute the existing isolated/PKS subset for this scope.

Proceed in dependency order:

1. Freeze complete source/routine/branch/caller/field/precision/lifetime and native-safety maps, with the actual original build profile. Use accepted W `bd290cc` only for fixed Rust context.
2. Build unchanged original primaries with real dependencies and explicitly initialized caller backing. Freeze authored cases before recording; keep source-initializer omissions and ordinary versus corrected dependencies explicit.
3. Exercise all primary helpers and drivers, full solar-wind resources, DM/CM/DMX/DMMODEL/shapelet paths, convergence and retained-state histories, BAT/secular assembly and phase-reference lifecycle. Include controlled multi-pulsar witnesses for the source's first-pulsar accesses.
4. Audit any separate observer against the complete ordinary corpus. Add mathematical/structural checks, actual execution/identity/type controls, compiled behavior mutations and measured coverage with all gaps/profile differences retained.
5. Deliver `validation/propagation/REPORT.md`, full contracts/maps, reproducible commands, identities and precise limitations. Commit explicit owned source/generator/report paths with Conventional Commits.

Use the original source oracle for reference. Small demonstrated corrections may have separately identified experiments under 0053; never overwrite original evidence or treat an unexplained difference as a bug. No Rust implementation, changed precision/tolerance, broader physics convention or milestone acceptance is delegated. Native-invalid operations must be classified before execution; isolate any deliberate nontermination probe with a finite timeout.

Trust validated internal contracts when designing later production recommendations. Native preflight and verifier corruption cases do not justify duplicate kernel checks. Do not freeze evolving central or other assignment work as immutable evidence; do not duplicate large accepted output trees. Keep all reproducible JSON/JSONL/compressed traces, binary fixtures, archives and build products local and ignored under 0039.

## Dispatch prompt

Implement Assignment AB under decisions/0063-propagation-dispersion-reference-stage.md and handoffs/28-propagation-reference.md in /Users/Eric.Dong/Downloads/tempo2rust. Own only validation/propagation/. Freeze and execute the complete 25-routine native propagation/dispersion/solar-wind/BAT/secular/phase-reference contract, with real pinned dependencies and explicit typed caller state. Preserve original source and accepted evidence, witness multi-pulsar and retained-state behavior, audit observers, and deliver exact fresh replays, controls, behavior mutations, coverage and limitations. Y owns Rust; Z owns its separate reference; AA is not released. Keep generated evidence local/ignored and commit explicit owned source/contract/report paths with Conventional Commits. Return validation/propagation/REPORT.md for central review; central retains design and milestone acceptance.
