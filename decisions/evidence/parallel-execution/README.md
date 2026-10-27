# Central parallel-execution evidence

The earlier native-worker preparation is described in decision 0085 and remains
under ignored `build/worker-preparation-20261027/`. It is distinct from the
implementation review below.

## Bounded implementation replay, 27 October 2026

Central inspected committed AK production
`81de1e6bb7d6f6cf1d17c075f402c301b87c9ce9`, including the observation routine's
extraction, worker scratch/publication path, diagnostic transcripts, process
admission and connected workflow callers. No production correction arose from
that bounded inspection. AK was still qualifying its full delivery; this is
not handover or numerical acceptance.

`central-implementation-review.py` exports that revision into a new temporary
workspace and verifies all 623 exported Git blobs (581 Rust-tree files and 42
fitting inputs). It builds a fresh default debug test executable using the
existing offline Cargo/native dependency cache, without reusing a project
`target` directory. Canonical TEMPO2 resources are read through a symlink at the
unchanged pinned commit. This is the existing macOS arm64 host, not Linux.

The exact test executable then runs six selected contracts directly, with each
isolated-test selector supplied so its body actually executes:

- `execution_tests::observation_workers_preserve_full_rows_effects_and_failed_prefix`
- `execution_tests::solar_workers_preserve_resource_history_and_failure_cleanup`
- `execution_tests::concurrent_complete_workflows_own_controls_and_diagnostic_collection`
- `execution_tests::execution_controls_restore_on_unwind`
- `execution_tests::observation_scratch_retains_unwritten_and_float_classes`
- `fitting_tests::terrestrial_fit_failure_recovers_with_the_same_runtime`

All six pass. The first two retain actual observation-worker visit assertions
and full-row/effect/resource comparisons; the third exercises four concurrent
complete callers with private diagnostic markers. The scoped unwind test emits
its expected caught-panic diagnostic. The original recovery test retains its
pre-existing first-prefit cache-warning normalization; no additional warning
comparison was removed. All exported files retain their original hashes after
execution.

Original run: `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-ak-committed-review-q98ue_ox`.
Ignored preserved bundle: `build/implementation-review-20261027/`. It includes
the source archive, executed script, test executable, full logs and receipts;
the receipts retain their original paths. The JSON key `production_files` in
the completion receipt counts all 623 exported files, including fixtures and
documents, rather than a count of production code modules.

| Evidence | SHA-256 |
| --- | --- |
| Completion | `e0d7f14cf5af24d1a5be2e8c5e3a054e955418173510744a3e7400b0c45e7cbb` |
| Source/environment identity | `c07a76675650384e470b7558fa1baf110f6e456c8c22ac74a871ed9d7b446eb8` |
| Commands and results | `2374ac5f5c871bf1bbdf12803ac8564c9399db211eaed4b101dbe907ea4b9df8` |
| Test executable | `4d53e159a59beaa88a4b10e425642da1653a3af71dd6458aecdd573018a88d87` |
| Source archive | `cd7a581f158534fd3847e1fb5acb4003b2fa607d5fa15d24c15338ac4c56c458` |

This replay does not establish native parallel fidelity, the installed/external
interface matrix, provider concurrency, performance, arbitrary panic recovery,
Linux qualification or resolution of any inherited numerical gate. Those stay
in AK's full delivery and AL's released-platform obligations. Central informed
AK when the build/check processes ended so its benchmark samples could be kept
separate.

## Benchmark review during qualification

Central reviewed AK's in-progress benchmark scripts before final delivery. The
observed native `AK_EVALUATE` interval includes `ak_begin`/`ak_visit`/`ak_end`,
which allocate/fill worker-observation storage and print summaries. The Rust raw
path does not perform that instrumentation. The raw native process also writes
a complete TSV after evaluation, while Rust writes null placeholders. Direct
whole-process comparisons would therefore include different output work. This
is a measurement-scope finding, not an observed numerical or production failure.

Central sent AK a bounded correction: retain worker calibration separately, use
an uninstrumented or explicitly calibrated timing profile, and match publication
for whole-process comparisons or label those as different workloads while using
matched evaluation intervals for the direct comparison. Distinguish observed
worker counts in calibration from requested timing-run counts. Existing scientific
and worker qualification remains applicable; refresh only affected benchmarks
and a focused output-equivalence control. The final report must resolve this
finding before central accepts its performance claims.

The in-progress source snapshot is ignored under
`build/benchmark-review-20261027/`; it is not a committed implementation baseline.
Snapshot receipt SHA-256: `dbafc4a3450eb68ea13949202adee3730232156f610d7c4ebaf8d2b7d7b9fdaa`.
