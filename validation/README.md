# Validation artifacts

Commit harness sources, generator scripts, authored fixture inputs, schemas, comparison contracts and concise reports. Keep generated JSON/JSONL, compressed copies, raw state dumps, coverage and execution receipts local. Root `.gitignore` enforces this policy; do not use `git add -f` for generated evidence. [Decision 0039](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0039-generated-validation-output-policy.md) supersedes historical requests to commit those outputs.

Existing generated files remain at their original paths. Removing them from Git tracking does not change an accepted local oracle. Normal comparison runs must still preserve the oracle and verify actual source, compiler, executable and runner identities. Put subsequent verification outputs in their assigned local output directory.

For a checkout missing generated reference data, use the matching stage report and pinned source/configuration. The native recording entry points include:

| Reference | Explicit native recording entry point |
| --- | --- |
| JPL | `python3 validation/jpl/run.py --record` |
| Clock/table | `python3 validation/clocks/run.py --record` |
| Parameter/PAR | `python3 validation/parameters/run.py --record` |
| Numeric utilities | `python3 validation/numeric_toolkit/run.py --record` |
| TIM I/O | `python3 validation/tim_io/run.py --record` |
| Preprocessing | `python3 validation/preprocessing/run.py --record` |

These are recording entry points, not a complete workspace bootstrap. Reports specify prerequisites, supporting inventories, coverage and verification commands. Historical freeze/provenance snapshots refer to their recorded revisions; rerunning a freeze against a later checkout does not reproduce the earlier acceptance state. Keep authored comparison rules separate from generated observations. Recording a new reference must never silently change the accepted numerical contract.

A delivery report records commands, source/toolchain identities, actual executable and artifact hashes, measured results, limitations and local evidence paths. Generated outputs do not need to be Git attachments for central review. Keep substantial data out of commits even when compressed; the central chat retains milestone acceptance.
