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


## Final audit and installed-client replay, 27 October 2026

Decision 0098 accepts the scoped AK handover and closes the benchmark finding
above. `native-timed-v1` removes worker instrumentation from timed binaries;
twelve `timed-control-v1` product controls pass, and `performance-v2` distinguishes
matched evaluation intervals from different raw publication work. The corrected
measurements retain the material performance costs and no consistent four-worker
benefit. The user chose AL qualification next, with profiling before cutover.

`central-handover-review.py` is the exact executed final review recipe. It reruns
the owner's read-only audit and supplements receipt runner/input identities,
the entire committed Rust/AK validation trees, installed-source Git bindings and
the two clean reference pins. At `eaa30c5`, it checks 3,195 unique files with zero
mismatches, including 607 committed files and 209 installed source files. It
checks the 99 qualification invocations / 484,314 comparisons, 108 performance
records, twelve output-equivalence controls and uninstrumented native symbols.
This is an identity/count audit, not another scientific comparison campaign.

A fresh external release Cargo client built from the audited installed source
passes four concurrent complete callers with repeated correlated fits,
predictors, diagnostics and failure/recovery. A newly compiled C11 shared-library
consumer passes and reports the installed library as actually loaded. The Rust
target directory was fresh; the existing offline dependencies and AK private
install were reused. There was no fresh reinstall, benchmark, Linux run or
inherited numerical acceptance in this replay.

Original run: `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-ak-handover-review-_v5v511v`.
The byte-preserving copy under ignored `build/handover-review-20261027/` contains
53 selected files (17,002,285 bytes), plus its preservation
manifest. Original receipt paths remain unchanged. It includes the selected
executed binaries, recipes, external client source, logs and products, excluding
the rest of the Cargo target cache. No generated evidence is committed.

| Evidence | SHA-256 |
| --- | --- |
| Central audit | `84a6c12ff443233e2ed234c53585b50a8027de96ee727a07f0c52155d4498222` |
| Completion | `e0dc5a83e849f4f8d5a86d24d4e8b26643bc6d63108f7d792feb6a7dcf7e9c83` |
| Commands | `2d7d829528a7164a6613f77b01ae0bdd9115972e3a2aaf640eea506a73b4ae70` |
| Executed recipe | `0c753dd38d7b683a9c0a5d17635deed61a78bb0fe8402ba8b6c05d08b603c4e0` |
| Preservation | `5f9392e9397d3395aefd5939dbdf9efb03f37002d4edf19fb6156faea3fb159d` |
