# Source fidelity, measured Rust improvements and deferred covariance profiling

Central decision, 30 October 2026, recording the user's direct policy clarification and the allocation lead brought from a side conversation. **Retain AL-first sequencing under 0098. Investigate this lead in the optimization pass after resuming the goal; this record does not resume the paused goal, release implementation work or accept a milestone.** No allocation implementation change or new profiling run is part of this record.

## Implementation policy

Preserve TEMPO2's original algorithms, scientific expressions, operation order, conventions and defined behavior by default. Departures are permitted on either of these evidenced grounds:

1. **A demonstrated original bug.** Document the pinned TEMPO2 source file/routine/location, defect, consequences and correction beside the changed implementation. Retain focused original/corrected evidence and check affected valid and neighboring behavior. Decision 0053 continues to govern these corrections.
2. **A demonstrably better Rust implementation.** A change may improve safety, ownership, clarity, memory use or performance while preserving scientific meaning and required observable behavior. Explain the specific benefit and TEMPO2 counterpart beside the changed code, record the decision/departure in the assignment report, and verify the relevant behavior. Measure any claimed memory or performance improvement. Language preference or an assumed benefit alone is insufficient.

This second route does not require calling defined native behavior a bug. It also does not authorize a new scientific algorithm merely because it is faster. Changes to numerical precision, scientific assumptions, output conventions or acceptance tolerances still require an explicit central decision; so do changes to the established solver/scientific contract. Existing source correspondence, necessary-unsafe explanations, ownership boundaries and evidence preservation remain binding. Routine implementation choices within an assigned scope and this policy need no repeated permission; neither route enlarges that scope or reopens a closed assignment.

This clarification supplements 0053 and supersedes any older wording that categorically excludes justified Rust implementation improvements. It changes no existing numerical gate, precision setting, scientific convention or accepted reference artifact.

## Covariance allocation: verified source facts and unconfirmed cause

Central inspected primary revision `c14b1ee6fb703484036345c00453ba27c66c19df` and native TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. The four Rust source/report files below are unchanged from AK delivery `eaa30c5`.

| Responsibility | Source evidence |
| --- | --- |
| Native dimension and initialization | `tempo2/tempo2.h:80` defines `MAX_FIT` as 10000; `tempo2/initialise.C:105` assigns `covar = malloc_blas(MAX_FIT, MAX_FIT)` during pulsar initialization. |
| Rust dimension and initialization | `rust/src/pulsar.rs:10` defines the same constant; `rust/src/pulsar_lifecycle.rs:796` allocates the square matrix and assigns it to the pulsar's covariance owner. |
| Native storage | `tempo2/TKmatrix.C:177` uses `calloc((rows*cols+2), sizeof(double))`, plus a separate row-pointer allocation and two hidden dimension cells. |
| Rust storage | `rust/src/tk_matrix.rs:51` creates a `Vec<f64>`, calls `try_reserve_exact(len)`, then `resize(len, 0.0)` at line 55, held in `Arc<Vec<f64>>`; mutation uses copy-on-write at line 77. |
| Existing measured process peaks | `rust/PARALLEL_EXECUTION_REPORT.md:209` records 1578.1 MiB for the one-worker two-pulsar correlated fit; line 215 records 833.4 MiB for one-pulsar Chebyshev generation. These are medians of whole-process maximum RSS, not allocation-attributed measurements. |

Each matrix's cell payload is `10000 * 10000 * 8 = 800000000` bytes, or **762.939453125 MiB**; two payloads total **1525.87890625 MiB**. These figures exclude headers, row pointers, allocator metadata and other process storage. The cells are ordinary `f64`, so this payload size must not be attributed to Rug precision.

**Profiling hypothesis, not a confirmed allocation breakdown:** native `calloc` may obtain logically zero storage without physically backing/touching every unused page immediately, whereas Rust's reserve-then-resize may eagerly touch the entire payload. That mechanism would be consistent with AK's one- and two-pulsar process peaks. The source calls and size arithmetic do not prove actual allocator/compiler behavior, page residency, simultaneous buffer lifetimes or the fraction of RSS attributable to these buffers. No native bug or measured improvement is established here. The side conversation reported no implementation changes.

## Investigation order for the optimization pass

1. Freeze the actual post-AL candidate, compiler/allocator/platform, inputs, runtime resources and feature/worker settings. Use the native and Rust initialization paths with one and two initialized pulsars. Measure allocation sizes/counts/lifetimes, mapped versus resident memory and elapsed initialization time immediately around covariance allocation and pulsar initialization. Record supported page-residency/page-fault evidence so attribution is stronger than matching whole-process peaks. Avoid touching the whole buffer merely to inspect its values before the residency measurement; perform exhaustive value checks separately.
2. Only after establishing the baseline, evaluate an equivalent zero-initialized allocation strategy that avoids eager touching of unused pages where the measured platform supports it. Preserve full dimensions, contiguous cells and stride, positive-zero values, allocation/error boundaries, ownership, aliasing/copy-on-write behavior and cleanup. No uninitialized cell may be exposed as an initialized `f64`; any necessary unsafe allocation boundary needs local layout/lifetime/deallocation justification. This note selects no particular allocator or dependency.
3. Compare before/after peak RSS, allocation/page-residency observations and initialization time under the same conditions, with repeated samples. Re-run AK's matched one-pulsar Chebyshev and two-pulsar fitting workloads, retaining their input/output and measurement scopes. Report allocator/platform dependence and whether memory cost is removed or merely deferred to later writes; do not infer a general improvement from one process peak.
4. Compare matched scientific outputs and relevant lifecycle behavior: dimensions, positive-zero cells, row access/stride, mutation and independent-context clone behavior, errors and recovery, and release of storage. Preserve TEMPO2 calculations, expression/operation order and all existing numerical/discrete gates. Lower precision, smaller matrices, altered calculations or weakened checks are outside this allocation experiment.

Retain the original baseline and candidate evidence, including an experiment that fails to improve anything. Record the allocation decision and measured benefit beside the eventual code and in its report. Generated JSON, traces and binaries remain ignored under 0039; historical evidence/runners remain unchanged under 0050. Central reviews the result before accepting the claimed optimization. AL qualification remains ahead of this work.
