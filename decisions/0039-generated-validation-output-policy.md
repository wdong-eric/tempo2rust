# Generated validation output stays outside Git

Central decision, 24 October 2026, following the user's instruction to stop committing reproducible validation JSON. **Generated execution evidence stays local.** This supersedes earlier handoffs requiring generated payloads to be committed. It changes artifact tracking, not scientific behavior, comparison gates or acceptance authority.

## Tracked inputs and local outputs

Keep implementation and harness sources, generator scripts, authored fixtures, schemas, comparison policies, source pins, concise reports and central decisions in Git. Generated case catalogs, native/Rust state dumps, numerical outputs, profiles, coverage, raw archives, inventories and execution receipts are local artifacts. This includes compressed JSON; compression is not a reason to commit a generated payload. Do not substitute another large generated format or force-add ignored evidence.

Root ignore rules cover JSON/JSONL and gzip forms under `validation/`, `rust/verification/` and `decisions/evidence/`. Thirteen explicit exceptions retain small schemas, reviewed comparison contracts and release field/branch specifications used as source inputs. Existing small Rust test fixtures and their contracts remain tracked. A new exception must be an actual source input, not an execution result.

Implementation chats Q, R, P and S have received this rule. Future handoffs inherit it. A delivery report identifies source and implementation revisions, commands, toolchain, actual executable/input hashes, results, limits and local evidence paths. Central reviews source identities and locally retained execution evidence; a generated payload's presence in the current Git tree is no longer an acceptance requirement. Historical statements about artifacts committed at earlier delivery revisions remain historical records.

Generate missing native reference outputs explicitly from the pinned source using the existing harness and its report's prerequisites. Ordinary verification still must not silently overwrite its oracle. A `--record` run establishes local reference data; it does not authorize accepting altered Rust behavior, relaxing a tolerance, or hiding a failed comparison. Keep accepted local evidence unchanged during unrelated work and use separate output locations for subsequent runs.

## Applied tracking change

The audit found 1,261 tracked JSON-family files totaling 541,445,493 bytes in the three evidence areas. Of these, **1,248 generated files totaling 539,959,393 bytes** were removed from the primary repository's index. All remain on disk at their original paths, with every size and SHA-256 checked before and after untracking. The thirteen retained source-input exceptions total 1,486,100 bytes. The 55.6 MB TIM observation dump and all three preprocessing archives above 33 MB are among the untracked outputs.

Verification checked complete ignore-rule membership, the exact staged removal list, unchanged local payload hashes and preservation of the retained inputs. No production code or scientific data changed. No numerical suite was rerun for this tracking-only change. The independently versioned canonical TEMPO2 and reference repositories remain untouched.

This commit does not remove blobs from existing Git history or reduce the space required by that history. Rewriting published or shared history requires a separate, explicit decision. No history rewrite, garbage collection, local artifact deletion or external upload is performed here.
