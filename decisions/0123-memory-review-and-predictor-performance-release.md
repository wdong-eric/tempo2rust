# Memory stage review and predictor performance release

Central checkpoint, 2 November 2026. Accept AN production `68973d669af0e1180ad8532bb8b364452ca203d7`
and its measured **macOS arm64** allocation improvement for integration. AN's
complete two-platform qualification remains open until actual Linux execution.
Release independent Assignment AO under handoff 41 so that CPU work advances
while AL prepares the next integrated Linux candidate. M2 stays accepted under
0122; AL, general fitting and full migration remain unaccepted.

## Measured improvement

The unchanged factory touches every page in the 10000-by-10000 f64 covariance
payload during `resize`; the zeroed global allocation leaves unused pages
unresident on this measured host. Native TEMPO2's `TKmatrix.C::malloc_blas` uses
`calloc`. The change retains every cell, positive-zero initialization, dimensions,
stride/alignment, Arc/Vec ownership, COW and recoverable payload allocation errors.
It changes no scientific expression, precision, model or acceptance tolerance.

Central reviewed the two necessary unsafe operations: allocation uses the checked
nonzero f64 layout and the global allocator; the initialized, aligned, uniquely
owned allocation transfers once to Vec with matching length/capacity/deallocator.
Adjacent comments identify the native counterpart and safety obligations. The
existing safe fallible reserve/resize path causes the measured eager touching;
the pinned standard-library assessment and bounded failure control are retained.
No dependency, custom allocator or speculative matrix abstraction was introduced.

Five interleaved fresh-process samples per selected workload show:

| Workload | Before peak RSS median | After peak RSS median | Before/after wall median |
| --- | ---: | ---: | ---: |
| Chebyshev generation | 833.469 MiB | 70.500 MiB | 0.460 / 0.404 s |
| Two-pulsar correlated fit | 1578.125 MiB | 52.406 MiB | 0.517 / 0.403 s |

These are bounded macOS results, not universal performance guarantees. Original
outliers and host-load observations remain recorded. Full reads/writes and COW
expose the deferred page cost; cells are not omitted. Logical release and retained
allocator RSS are distinguished. Native timings still show substantial remaining
CPU gaps, so the memory result does not establish performance completion.

Central independently verified 1200 frozen source bindings, ten executables,
208 runtime files, all 210 process/resource records and raw counters, exact CLI
products, full scientific JSON and all nine scientific files. Eleven lifecycle
histories are byte-identical before/after. The native correlated-fit gate has
4555 checks and zero failures. A fresh small injected-allocation-failure test
also passes. Only `tk_matrix.rs` and its lifecycle test differ between frozen
crate exports. AN retains the unchanged seeded-reinitialization reference
mismatch and predictor log10RMS failure; these are not allocation regressions
or newly waived gates.

Central evidence is retained under ignored
`decisions/evidence/release-qualification/build/an-central-20261102/`.
Review receipt SHA-256:
`2542957337a1d6a7bbe000535058adbae76aca003230716cdf4525d3c69f84ba`.
The executed original directory is `/private/tmp/tempo2-central-an-review-20261102`.
AN is finishing the report and Linux recipe. Its build helper must explicitly
isolate before/after target directories and use the prepared Cargo cache; no
repeat of completed scientific measurements is needed for that recipe correction.

## Next substantial stage

AK's dated standalone C predictor loop measured 0.469391 s in the Rust route
versus 0.000346 s natively for 10000 phase plus 10000 frequency calls. This was
the complete bridge/evaluator route, not a causal conversion profile. Current
source rereads and reconstructs each polynomial through `predictor_ffi.rs`, with
per-scalar locale creation and string conversion in private C transport. Those
are concrete profiling leads, not yet attributed costs or an approved replacement.

AO starts from the already reviewed memory source `68973d6` in isolated clone
`/private/tmp/tempo2-ao-predictor-20261102`. Measure the current C route and Rust
leaf, identify the dominant cost, then implement the smallest justified equivalent
improvement within handoff 41. Keep the canonical Rust scientific calculation,
source operation/rounding order, native ABI and caller-owned mutable storage.
Ordinary predictor precision remains the qualified 53/64-bit profile; timing
remains Rug128. No source pointer cache, lower precision or alternate scientific
engine is assumed to be acceptable.

The major-stage reassessment finds no new scientific-policy choice: the measured
allocation improvement satisfies 0099/0121, and AO must preserve the existing
contract. AL retains the single compatibility stream, now preparing truthful
propagation profile reporting and actual-object/intermediate evidence for the
remaining Linux timing differences. Minor diagnostics do not block AO. Central
coordinates integration, affected checks, quiet timing windows, publication and
all milestone acceptance. The next Linux run should batch reviewed changes;
there is no full migration or production-cutover approval here.


## Integration and dispatch completed

AN final verification/recipe commit `497168b4fc6dfed02046d8eecb1055cb4f055585`
is integrated with its original production and observer commits through merge
`6cc0b79b1836ace952c7c25b3cfaf5651adff1dd`. Central freshly passed the real Cargo
build-path control under an inherited shared target. AN's implementation ownership
is closed; AL owns the integrated Linux follow-through. No Linux improvement is
claimed before that execution.

Central copied and independently hashed 2068 evidence files (74521716 bytes) to
ignored `validation/allocation_profile/build/macos-reviewed-20261102/`, excluding
rebuildable target/source trees and dependency caches. The copy manifest retains
original paths and does not relabel executions. Original Git source revisions
remain reachable in main, and the isolated clone remains intact.

AO is active as **Assignment AO: Predictor CPU optimization**, thread
`01a19a86-1e28-7962-b3ff-d81c5cd16ad8` on the local host. Its isolated baseline
contains the reviewed allocator; central verified its active profiling turn.
This startup does not imply delivery or acceptance of a predictor improvement.

Central also accepts AL `febb650` as diagnostic preparation: 229 bound inputs and
recipes, seven actual-object disassemblies, five neutral source histories and
28 exact intermediate rows were independently verified; four focused harness
tests pass. Actual Linux behavior remains untested. The proof packet and independent
Cargo routing check are retained in `an-central-20261102/integration-closure.json`.
AL is now adding AN's source-bound before/after execution and existing scientific
checks to the full Linux matrix, keeping inherited native failures explicit.
