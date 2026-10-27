# Assignment AK: parallel observation execution and concurrency

**Prepared only; not released.** Follow decision 0085 after AJ's reviewed committed handover, ownership closure, the 0082 checkpoint and explicit central release under 0074. Consult HANDOFF.md for current Rust ownership; this prepared brief grants no ownership. Work in `/Users/Eric.Dong/Downloads/tempo2rust`; read HANDOFF.md, decisions 0085/0084/0082/0074/0065/0053/0039/0050 and the predecessor reports named at release.

Once released, own `rust/` and new `validation/parallel_execution/` as sole Rust writer. You are not alone in this repository: preserve other chats' changes, central files, canonical TEMPO2 and accepted evidence. Central retains shared design, numerical policy and acceptance. Preserve original 0008 and its amendments.

Deliver the source's optional per-observation calculate_bclt parallel capability through the canonical engine, a documented and tested library concurrency contract, and representative performance measurements. Keep the outer pulsar order, each observation's algorithm, process-state lifetimes, precision and scientific gates. General fitting/reduction parallelism and a new process-state architecture are not part of this assignment.

Proceed in dependency order:

1. Freeze the source/state/lock map, minimal public selection/worker controls, permitted concurrent calls, diagnostic/error projection and focused cases against the released predecessor.
2. Reuse the serial observation algorithm with safe disjoint observation work and scoped shared effects. Address concrete warning/version/log/resource dependencies, parent/worker lock ownership and failure cleanup. Explain any necessary unsafe locally.
3. Qualify supported complete concurrent library workflows, native provider/FFI boundaries and the predictor C caller-exclusivity contract without changing source process-lifetime semantics incidentally.
4. Build a disposable native enabled profile with proven actual workers; verify the Makefile flag-wiring concern and retain original/corrected evidence for any small fix. Reuse existing native builders and fixtures.
5. Execute serial/parallel scientific and state comparisons, affected debug/release and external-client checks, bounded stress/error/recovery checks, and representative measured workloads with exact build/input/backend/worker identities; run formatting and Clippy.
6. Commit explicit authored owned paths with Conventional Commits and return `rust/PARALLEL_EXECUTION_REPORT.md`, checked concurrency guarantees, measured limitations and the complete source/unsafe/departure/open-gate ledger for central's 0082 checkpoint.

Keep generated evidence ignored. A real parallel workload and measured results are required; multiple spawned threads, a mutex or a build flag alone are not evidence of parallel capability or complete safety. Escalate consequential new public/scientific choices to central with alternatives and a recommendation; continue independent work.

## Concrete predecessor diagnostic finding

0094 carries the AH parallel library failure `fitting_tests::terrestrial_fit_failure_recovers_with_the_same_runtime` at `rust/src/fitting_tests.rs:302`. The retained `validation/core_output/build/tests-debug.log` has two complete assertion values. Central's reproducible `decisions/evidence/core-output/central-warning-review.py` finds only three differing vectors: `/cycles/0/postfit/warnings`, `/cycles/1/prefit/warnings`, `/cycles/1/postfit/warnings`; all other 6,716 scalar values and container structure/order match. Receipt `decisions/evidence/core-output/build/handover-20261027/warning-review.json` has SHA-256 `8c9de7930bbb13627e951f8a180c0107720d4d28c9e1cbf0272442579513766b`.

This is localization of an actual recorded failure, not a reproduced race or concurrency qualification. Reproduce/address diagnostic ownership and log-control interleavings within the declared supported-call contract; retain the original failure. Do not erase warning comparisons or treat isolated/serial retries as a concurrency fix. This adds evidence to AK's existing scope and grants no early ownership.

## Dispatch prompt after release

Implement Assignment AK under decisions/0085-parallel-execution-and-concurrency-prepared-contract.md and handoffs/37-parallel-execution-and-concurrency.md from the baseline explicitly released by central. Own rust/ and validation/parallel_execution/ as sole Rust writer. Deliver actual safe observation-parallel timing through the canonical engine, supported concurrency boundaries and matched performance measurements. Preserve source state/algorithms, existing gates and Rug128; use 0053 for demonstrated small bugs, justify necessary unsafe, verify affected workflows and return committed authored work with rust/PARALLEL_EXECUTION_REPORT.md for the 0082 checkpoint.
