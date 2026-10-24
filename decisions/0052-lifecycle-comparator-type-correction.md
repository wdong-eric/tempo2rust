# Lifecycle comparator type correction

Central review, 24 October 2026. **Return one bounded verifier correction to Assignment T before acceptance.** Its production state/lifecycle implementation is not shown wrong by this finding. T retains ownership under 0045/0050; no new Rust owner is released.

## Completed central checks

Central freshly built both debug and release in an independent Cargo target. Each passed all 30 corrected S cases / 133 events / 5,329,542 typed records, 31 existing comparator controls, Q and supplemental helpers, the two compiled link-reset mutations, focused lifecycle tests and the isolated diagnostic-state check. The delivery audit also verified all 531 current indexed artifacts, all 518 preserved prior artifacts, original/staged R identities and the complete reported regression chain. R is restored exactly; staging changes only its two path assignments. The four historical M2 failures remain unchanged.

The fresh execution receipt is `3c3d20129aee66cc803d5204dec5e974b440de0afc7ea0a1b0389c669975e2b8`; the delivery audit receipt is `04bd6b7698ccbe7579ef8e518a3f3217061f3a00e59c6efd5bee2f34ffa23c2e`. They remain local in `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-t-central-cpq5rxy2/`. Those successful checks establish the tested values, but do not close the additional type-integrity finding below.

## Reproduced finding

At delivery `4225f48`, `validation/pulsar_lifecycle_port/run.py::compare` returns immediately for `a == b` before checking types. Python considers `1`, `True` and `1.0` equal, including inside lists and dictionaries. An outer-container equality shortcut therefore bypasses the later recursive type assertions.

Central used the actual accepted `matrix_1_1` expected output after T's declared projection. Changing the retained profile `MAX_OBSN` record's integer count from 1 to `true`, changing it to `1.0`, or changing its first run-length count to `true` all pass the current exact comparator. These are malformed typed counts; the scientific source values and accepted oracle were not changed. The affected runner SHA-256 is `877e106731865417a0a8e9616c090ee72563b87c6c8212df26658323f5a93fee`; the source case SHA-256 is `461e77b0d9032cf999641abca44c5903fd31346dff97c2c7bc8fa4b5b0b9256b`.

The executed reproducer is `decisions/evidence/pulsar-lifecycle-port/type-equality-reproducer.py`. It reads the existing oracle, mutates copies in memory and reports the comparator outcomes. Its generated result stays local and ignored.

## Required correction and verification

T must make scalar-type checking recursive through every compared container. Merely moving a type check before the current equality shortcut is insufficient when the containers themselves have matching types. Preserve existing field/case/event coverage, ordered state records, byte values, ABI/diagnostic projection and exact numerical comparisons. Keep a positive unchanged-output control and add realistic nested mutations for boolean versus integer counts, float versus integer counts and a nested run-length count. Use the actual projected case; the tests must demonstrate rejection by the normal comparator.

Preserve the pre-correction delivery and central evidence before replacing output receipts. R's accepted runner remains read-only under 0050. This correction belongs in T's harness and controls; it does not authorize editing accepted reference JSON, R's comparer, production Rust, arithmetic, precision, source behavior or M2 expectations.

Rerun T's affected debug/release S and helper comparisons, the new controls and final delivery verification. Report unchanged production/executable identities if they remain unchanged, the precise new control results and the updated report/commit. Keep generated evidence ignored under 0039. Central will independently challenge the corrected comparator and review the final delivery before deciding acceptance.
