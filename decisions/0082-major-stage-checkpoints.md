# Major-stage reassessment and user consultation

Central decision, 25 October 2026, implementing the user's updated goal: after an important implementation stage, pause to reassess whether the migration remains on track and whether a previously unconsidered issue has appeared; when one has, ask for the user's opinion with a recommendation and explanation.

## Handover checkpoint

Before releasing the successor to a major functional stage, central performs a short checkpoint alongside the 0074 handover review:

1. Identify the newly executable capability, exact delivered baseline and evidence, distinguishing implementation handover from numerical/scientific acceptance.
2. Reassess the remaining full-core scope and dependency order. Check that the next assignment still advances usable functionality and that retained dependencies, external interfaces, operations and unresolved scientific gates have not disappeared from the plan.
3. Identify new consequential issues discovered in the stage: changed scientific assumptions, public behavior or compatibility, unexpected dependencies, ownership/safety limits, or substantial changes to the next implementation path. Distinguish them from already decided limitations and known open numerical findings.
4. For a new issue requiring a choice, present the concrete alternatives, central's recommendation, reasons and effects on scope/sequence/acceptance. Ask the user before taking the dependent action. An unanswered question is not approval. Independent work may continue.
5. Record the checkpoint and its outcome in the handover/release decision. If no new choice needs the user, central may release the next stage after the reassessment and normal ownership closure. The user requested meaningful consultation about new issues, not a routine approval question for every commit.

Major stages are functional deliveries such as input-to-BAT, general timing, general fitting, prediction and remaining integration/release capabilities. Minor documentation, focused corrections and preparation commits do not each require a new ceremony. Previously settled questions are not repeatedly reopened without material new evidence. Functional priority 0065, numerical gates, one Rust writer, source correspondence, the separately approved small-bug policy 0053, generated-output policy 0039 and evidence preservation 0050 remain in force.

AE remains active under 0079. Its delivery will receive this checkpoint before AF can be released. No successor is released by this document, and no milestone is accepted here.

## New interface-scope question raised during preparation

The source installs three libraries and exposes C interfaces, including a standalone predictor interface in `tempo2pred.h`. The core header also exposes the native pulsar/observation structures. Rug-backed Rust state is not their native C memory representation. Full compatibility with unchanged general core/toolkit C callers therefore requires additional conversion/ownership work; complete scientific implementation in Rust does not establish that compatibility by itself.

Central has asked the user to choose between:

- **Recommended:** full Rust core/toolkit APIs plus compatibility with the standalone predictor C interface. This retains the scientific function scope and the installed predictor-client workflow while focusing compatibility work on a concrete standalone consumer.
- **Broader interface guarantee:** also preserve the general core/toolkit C calling interfaces for existing callers. This adds a substantial representation, ownership and callback compatibility layer and corresponding external-client evidence.

**The choice is pending.** Neither alternative is adopted or excluded by this record. Prior planning references to core/predictor/toolkit C-interface disposition remain open. Do not claim migration complete or release a dependent library-interface assignment by assuming the recommended answer. AE's scientific fitting work and the already prepared AF/AG/AH/AI contracts can continue independently; their eventual handovers still receive the checkpoint above. Plugin/GUI binary compatibility remains separately scoped under 0008.

The pinned `getPeriod.C` consumer reads into an uninitialized `T2Predictor`, omits an argument-count guard and ignores read status. Its normal calculations use parsed binary64 MJD, a fixed 1400 MHz observing frequency, binary64 frequency and reciprocal, and the source text format. A future consumer implementation must preserve those defined calculations and use 0053 for demonstrated initialization/error-boundary fixes, rather than preserve arbitrary uninitialized state.

## Bounded preparation evidence

A C11 and a C++17 probe compiled against raw pinned `tempo2pred.h` with Homebrew GCC/G++ 16.2.0 on the current macOS arm64 host. The two outputs agree on every recorded field offset/size, all exposed type sizes/alignments and predictor-kind values. Ordinary native `long double` is 53-bit, size/alignment 8/8; `T2Predictor` is 24 bytes with alignment 8. This is a header-layout observation, not proof that any Rust library currently implements the ABI.

The authored probe is `decisions/evidence/predictor-interface/layout.c`. Raw source identities, commands, compiler/executable identities, outputs and a read-only symbol listing of the existing A2 reference archive are retained in ignored `decisions/evidence/predictor-interface/build/preparation-20261025/`. The listing contains 57 predictor-related definitions/data entries; it is navigation evidence, not an exhaustive public-contract or implementation claim. No new scientific or external-library execution was performed.

Original preparation directory: `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-predictor-abi-plan-77b9p93m`. Receipt SHA-256: `3e344c772e0c8be89a2497654d7de58bcfcd13225a208012ae49b980b2ae193b`. Identical C/C++ layout-output SHA-256: `0211d885a1b62884d319a50cf59e1ad4dc15480c6b4ca0404e1b923cba380efe`. Generated JSON, source snapshots, binaries and symbol/layout dumps remain untracked under 0039.
