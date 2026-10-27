# Assignment AK: parallel observation execution and concurrency

**Released by 0096 after AJ's reviewed handover, ownership closure and the 0082 checkpoint.** Follow 0085/0074 from production `3e41497`, qualification `31c7290`, report `f4b73f8` and the central release commit. Work in `/Users/Eric.Dong/Downloads/tempo2rust`; read HANDOFF.md, decisions 0096/0095/0094/0090/0087/0086/0085/0084/0082/0074/0065/0053/0039/0050, `rust/LIBRARY_INTERFACES_API.md`, `rust/LIBRARY_INTERFACES_SOURCE_MAP.md`, `rust/LIBRARY_INTERFACES_REPORT.md` and the native-backend/analysis reports they reference.

Own `rust/` and new `validation/parallel_execution/` as sole Rust writer. AJ ownership is closed and its evidence is read-only. You are not alone in this repository: preserve other chats' changes, central files, canonical TEMPO2 and accepted evidence. Central retains shared design, numerical policy and acceptance. Preserve original 0008 and its amendments.

Deliver the source's optional per-observation calculate_bclt parallel capability through the canonical engine, a documented and tested library concurrency contract, and representative performance measurements. Keep the outer pulsar order, each observation's algorithm, process-state lifetimes, precision and scientific gates. General fitting/reduction parallelism and a new process-state architecture are not part of this assignment.

Proceed in dependency order:

1. Freeze the source/state/lock map, minimal public selection/worker controls, permitted concurrent calls, diagnostic/error projection and focused cases against the released predecessor.
2. Reuse the serial observation algorithm with safe disjoint observation work and scoped shared effects. Address concrete warning/version/log/resource dependencies, parent/worker lock ownership and failure cleanup. Explain any necessary unsafe locally.
3. Qualify supported complete concurrent library workflows, native provider/FFI boundaries and the predictor C caller-exclusivity contract without changing source process-lifetime semantics incidentally.
4. Build a disposable native enabled profile with proven actual workers; verify the Makefile flag-wiring concern and retain original/corrected evidence for any small fix. Reuse existing native builders and fixtures.
5. Execute serial/parallel scientific and state comparisons, affected debug/release and external-client checks, bounded stress/error/recovery checks, and representative measured workloads with exact build/input/backend/worker identities; run formatting and Clippy.
6. Commit explicit authored owned paths with Conventional Commits and return `rust/PARALLEL_EXECUTION_REPORT.md`, checked concurrency guarantees, measured limitations and the complete source/unsafe/departure/open-gate ledger for central's 0082 checkpoint.

Keep generated evidence ignored. A real parallel workload and measured results are required; multiple spawned threads, a mutex or a build flag alone are not evidence of parallel capability or complete safety. Escalate consequential new public/scientific choices to central with alternatives and a recommendation; continue independent work.

## Delivered-backend preparation

Read both 27 October supplements in 0085: the AI `20e368f` process map/native witness and the release reconciliation with AJ `f4b73f8`. Reconcile the actual public owner methods, C globals and supported complete-call boundaries before implementation. It maps the process session, complete timing chain, destructive diagnostic collection, DMMODEL/SWM resource effects, thread-local acceleration and CALCEPH descriptor/session ownership. A bounded native probe executes the actual `calculate_bclt` loop with one/two/four workers and passes 4,662 exact serial/parallel comparisons for 259 defined `delayCorr=0` observations. Reuse that concrete worker-observation recipe where helpful; it does not replace the full corrected timing/shared-state/failure/performance cases required here. Original Makefile OpenMP wiring, Rust parallel execution and concurrency remain unqualified. The complete 0096 ledger remains open. AK now owns implementation; the bounded preparation grants no concurrency or scientific acceptance.

## Concrete predecessor diagnostic finding

0094 carries the AH parallel library failure `fitting_tests::terrestrial_fit_failure_recovers_with_the_same_runtime` at `rust/src/fitting_tests.rs:302`. The retained `validation/core_output/build/tests-debug.log` has two complete assertion values. Central's reproducible `decisions/evidence/core-output/central-warning-review.py` finds only three differing vectors: `/cycles/0/postfit/warnings`, `/cycles/1/prefit/warnings`, `/cycles/1/postfit/warnings`; all other 6,716 scalar values and container structure/order match. Receipt `decisions/evidence/core-output/build/handover-20261027/warning-review.json` has SHA-256 `8c9de7930bbb13627e951f8a180c0107720d4d28c9e1cbf0272442579513766b`.

This is localization of an actual recorded failure, not a reproduced race or concurrency qualification. Reproduce/address diagnostic ownership and log-control interleavings within the declared supported-call contract; retain the original failure. Do not erase warning comparisons or treat isolated/serial retries as a concurrency fix. This evidence remains part of AK's released 0096 scope; it does not establish a resolved race or concurrency guarantee.

## Dispatch prompt

Implement Assignment AK under decisions/0085-parallel-execution-and-concurrency-prepared-contract.md and handoffs/37-parallel-execution-and-concurrency.md from AJ delivery `f4b73f8` plus central decision 0096 and its release commit. Own rust/ and validation/parallel_execution/ as sole Rust writer. Deliver actual safe observation-parallel timing through the canonical engine, supported concurrency boundaries and matched performance measurements. Preserve source state/algorithms, existing gates and Rug128; use 0053 for demonstrated small bugs, justify necessary unsafe, verify affected workflows and return committed authored work with rust/PARALLEL_EXECUTION_REPORT.md for the 0082 checkpoint.
