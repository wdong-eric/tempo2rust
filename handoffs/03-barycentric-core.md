# Rust barycentric timing implementation

**Assignment C is complete at `49e122c` and M1a is accepted in [decision 0003](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0003-m1a-acceptance.md).** The requirements and dispatch below are retained as the historical implementation brief. The [shared handoff](/Users/Eric.Dong/Downloads/tempo2rust/HANDOFF.md) records current gates. The reference remains accepted at `7795d53`, with its identity and three goldens pinned in [decision 0002](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0002-m0-acceptance-and-c-release.md). D requires a separate central release.

[Decision 0001](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0001-reference-and-m1a-contract.md) fixes Rug128, the barycentric conventions and budgets, the library/CLI contract, and Git layout. Do not repeat backend selection or wait for DDH diagnosis or terrestrial policy decisions; those are outside this assignment.

## Ownership

Own `/Users/Eric.Dong/Downloads/tempo2rust/rust/`, including its crate, tests, documentation, and `M1A_REPORT.md`. Other chats share the workspace; preserve their edits. The reference checkout, accepted fixtures, precision experiment, and central design documents are read-only inputs. Assignment D must not modify the crate concurrently.

Ownership also permits the explicitly defined primary-repository bootstrap and root `.gitignore` from decision 0001. Preserve the nested repositories and their histories. Copy only the needed small accepted inputs, three goldens, schema/comparator, and provenance into your test tree so the crate's checks are self-contained; do not change the source artifacts or commit another chat's unfinished work. Keep the copied contract unchanged and record its revision and hashes.

## Target behavior

Implement the declared subset of PAR and FORMAT 1 TIM loading, phase evaluation, and fixed-parameter residuals for a single isolated pulsar with barycentric TOAs. Use `test1.par` and `test1.tim` as the initial correspondence case and the additional cases accepted at M0 to exercise spin evolution and precision. Identify precisely which corrections the source applies or bypasses for the barycentric site marker.

Preserve input identity, precision-sensitive values, parameter presence and flags, units, phase origin, and mean-removal behavior. The supported subset is part of the public contract. Scientifically active unsupported fields or directives must produce an explicit error. Any informational fields accepted without calculation must be classified explicitly.

Use one library and a thin CLI with the accepted interface. Follow TEMPO2's calculations closely in safe Rust, with explicit owned state and returned errors. Ordinary bad input must not panic or terminate a library caller. A failed result must not appear complete.

## Work sequence

1. Reconfirm the accepted reference revision and fixture hashes, perform the authorized root Git bootstrap if still needed, and create the minimal crate. Use Rug 1.30.0 at a fixed 128-bit significand with B's pinned dependency versions. Avoid a selectable-backend framework.
2. Implement the necessary model and observation storage and the supported parsing path. Test direct decimal parsing, field identity, required parameters, and malformed values.
3. Port the relevant phase and residual logic from [formResiduals.C](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/formResiduals.C:396), including its split calculation, preserved binary64 boundaries, first-eligible-TOA reference, sign-dependent pulse rounding, and signed remainder. Keep full pulse counts as checked signed 64-bit integers and a concise map from Rust modules to original functions. Cover F0 through F3; exercise nonzero F3 and positive/negative half-pulse boundaries with independent expectations since the accepted reference fixtures alone do not cover them.
4. Compare parsed values, phase, pulse reference/count, and residuals for `barycentric`, `barycentric_4ns`, and `long_span` using the accepted `barycentric-v1` projection. Follow [basicBATs and subtractBATs](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/tests/testTempo2.C:360). Implement alignment and 4 ns perturbations in the test adapter at the declared stage epochs, keeping original input identity. Preserve separate exact-count, phase, and residual gates; do not add public scenario modes or fabricate unused legacy resources/diagnostics.
5. Expose the completed path through the CLI and document a reproducible example. Add meaningful failure checks for missing files, malformed epochs, missing required parameters, unsupported model features, and empty input. Check a failed library call followed by a valid call.
6. Run the focused checks and write the implementation report. Report unexplained numerical discrepancies before changing conventions or tolerances.

## Completion criteria

The library and CLI must calculate the accepted barycentric cases using Rust timing logic and agree at the declared comparison stages. Checks must assert the expected result or error, including the precision-sensitive perturbations, and preserve original inputs. Output must identify the code/configuration used and the declared support scope.

This milestone does not include terrestrial corrections, binary models, fitting, plugin hosting, or a new scientific formulation. Do not call the legacy engine to supply the Rust calculation result. Retained numerical dependencies are allowed only within the accepted design and must be listed.

Save `M1A_REPORT.md` with the shared return-report contents, a supported-parameter/directive table, source correspondence, and measured numerical differences. Request central decisions only for material contract changes. The central chat accepts M1a; release to D additionally requires its terrestrial contract.

## Historical dispatch prompt

```text
Implement Assignment C in /Users/Eric.Dong/Downloads/tempo2rust. M0 is accepted and C is released. Read /Users/Eric.Dong/Downloads/tempo2rust/HANDOFF.md, /Users/Eric.Dong/Downloads/tempo2rust/decisions/0001-reference-and-m1a-contract.md, /Users/Eric.Dong/Downloads/tempo2rust/decisions/0002-m0-acceptance-and-c-release.md, and /Users/Eric.Dong/Downloads/tempo2rust/handoffs/03-barycentric-core.md. Own the Rust barycentric core and the explicitly authorized root Git bootstrap. Use the accepted reference at 7795d53 and Rug128 design, preserve the independent repositories and other chats' changes, and deliver the runnable crate, focused checks, and rust/M1A_REPORT.md. Shared design changes and M1a acceptance remain with the central planning chat.
```
