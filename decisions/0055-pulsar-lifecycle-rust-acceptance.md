# Pulsar state and lifecycle Rust acceptance

Central decision, 24 October 2026. **Accept Assignment T at report commit `6c8cf68`, with comparator correction `804d9fd` and restored-runner staging `8daa4aa`.** This closes T's ownership of `rust/` and `validation/pulsar_lifecycle_port/`. The next Rust-writing assignment requires a separate central release. M2 remains unaccepted.

## Accepted implementation and scope

The 0045 transfer provides canonical typed ownership for all 349 pulsar fields and the mapped nested structures, using the accepted O parameter and R observation stores. It transfers the twelve released lifecycle/copy/matrix routines and ten packed-polynomial helpers; distinguishes source partial copy from owned Clone; retains explicit old allocation owners, shared/overlapping observation views, fixed backing/tails and source-known versus unwritten state; and integrates existing load, evaluation, fitting and clock consumers. The duplicate owned ClockPulsar representation is removed in favor of a borrowed canonical view with declared single-observation caller inputs.

Central previously reviewed the complete source/field/order/ownership maps and implementation. The final identity audit confirms the same 51 Rust source/build inputs and the same debug/release test executable bytes as before 0052. No new production unsafe was introduced; the three existing unsafe-containing files and native/dependency build inputs remain unchanged. Existing fallible storage/string/lifetime boundaries remain explicit representation differences, not claims to reproduce native undefined memory access. Physical callbacks, full PAR/TIM/preprocessing and further timing models remain future work.

## Corrections and direct checks

0050 is satisfied: R's original runner is byte-identical to accepted `04918d5`, SHA-256 `7d6424654a666d3852255bd9264ba0fbb88ba3c6e57f7bf717da4087a6b10529`. T copies all thirteen R harness files to its own disposable staging. Only the copied run.py's ROOT/OUT path assignments differ; all original/staged identities and the reversible diff were audited. No R comparison, native expectation or preservation member changed.

0052 is satisfied: the comparator checks scalar types recursively through every dictionary/list. Central's three original actual-case witnesses now reject integer-to-boolean, integer-to-float and nested run-count-to-boolean substitutions at their scalar paths. The unchanged source case hash remains `461e77b0d9032cf999641abca44c5903fd31346dff97c2c7bc8fa4b5b0b9256b`. The corrected runner hash is `552357415e5fd31b081c65315bd18e5c3cc48b8e251ba3edeac816955a28887d`.

Central freshly built independent debug/release Rust executables before 0052 and passed the complete S/Q/helper/native-mutation/focused-lifecycle checks. After the harness correction, it verified exact production/compiler/executable identity and reused those independently built binaries for another fresh challenged S replay. Each profile passed **30 cases / 133 events / 5,329,542 typed records** and all **35 controls** (two positive, 33 required rejections). This includes all three independent malformed-type witnesses. All 8,141 protected files remained unchanged.

## Final delivery and regression evidence

T's complete sequential verifier exited zero with the corrected comparator. Central audited all 1,030 current indexed artifacts, 531 preserved pre-0052 artifacts, 518 earlier preserved artifacts and 31 current commands through the R/O/clock/JPL/M1/M2 chain. Required formatting, Clippy, ordinary tests, S/Q/supplemental helpers, link-reset mutation witnesses, isolated diagnostic-state checks, R/O storage and comparison controls, clock/glob/provenance, JPL and M1a/M1b checks pass. The full wider regression chain was audited from actual identities/logs/receipts, not independently rerun by central after 0052; central's separate fresh checks are described above.

M2 completes with exit 1 and exactly its four historical debug/release dependent-column retained-mode-order and singular-value-index-zero diagnostics. Its failures match the original receipt exactly. This acceptance neither repairs nor waives that gate.

Both failed corrected-verifier attempts remain recorded. The first encountered central's temporary notice in historical decision 0008; those original bytes were restored exactly. The independent O attempt encountered a newly added restoration receipt during its immutable-output check. These preservation failures were not scientific failures. The final standard run completed sequentially with no such concurrent annotations. No original manifest or failure was rewritten to pass a gate.

Final delivered receipt SHA-256: `5ba8eae0869f9bae74e07cf25e4dc0b6a97e9f9f60fa8392b4c358e43fd719d3`. Reviewed report SHA-256: `2f6d8af2b64f1934bee41dd7c7ad2f9a5ba752e4f6b0ba797e3e90f36b4fe135`.

The central sources and ignored evidence are under `decisions/evidence/pulsar-lifecycle-port/`. Pre-correction execution/audit receipts are `3c3d20129aee66cc803d5204dec5e974b440de0afc7ea0a1b0389c669975e2b8` and `04bd6b7698ccbe7579ef8e518a3f3217061f3a00e59c6efd5bee2f34ffa23c2e`. Post-correction execution/audit receipts are `97ba6aeef40a91f8bfbe5e193052e1268102c15b387f227f2ba80cb4f46d2ba7` and `60bbbaf2faa8f51d6aa4e10f0d06b591a3ae35a9203b9040f0a054b14fc77106`. Generated evidence remains local and ignored under 0039.

## Remaining work and current policy

This is the released state/lifecycle transfer, not full TEMPO2-core acceptance. Other platforms, concurrency stress, performance, OS allocation exhaustion, later physical callbacks, parsers, timing models and algorithms remain outside the tested scope. The storage model does not promise native layout or plugin callback ABI. M2 and the 0049 numeric-boundary questions remain unresolved.

T's accepted baseline preserves the source omissions/defects required by its original contract. Decision 0053 now permits small demonstrated corrections with adjacent original-source explanations, focused regressions and explicit corrected comparison contracts. That permission does not retroactively change this delivered baseline or its original reference observations. Both independent reference repositories and accepted evidence remain unchanged; no new bug fix is released by this acceptance.
