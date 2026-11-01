# Draft Assignment AN: covariance allocation profiling

**Prepared, not released.** This is the post-AL optimization handoff requested by
0099. It grants no present ownership and starts no profiling or implementation.
Central must first review the AL checkpoint, record outstanding gates, resolve
writer ownership and explicitly release a named post-AL candidate. Do not start
this work from a still-running qualification candidate. The user's AL-first
sequence, numerical policy and full migration objective remain unchanged.

## Goal and existing evidence

Determine whether eager touching of the full covariance storage explains a
measured part of Rust's process memory and initialization cost. If supported,
evaluate an equivalent zero-initialized allocation strategy with a demonstrated
benefit. The allocation lead is a hypothesis, not a diagnosed breakdown.
Read decisions 0099 and 0053, the released lifecycle contract, and the current
AL/AK reports. Preserve original evidence and the source behavior by default.

Central checked these sources at `fecbc18af95196a16db2046306099b6345a50976` on
1 November 2026; this is a source locator, not the future benchmark baseline:

| Responsibility | Current counterpart |
| --- | --- |
| Fixed covariance dimensions | `rust/src/pulsar.rs::MAX_FIT` and pinned `tempo2.h::MAX_FIT`, both 10000 |
| Per-pulsar creation | `PulsarContext::initialise_one` in `rust/src/pulsar_lifecycle.rs` calls `malloc_blas(MAX_FIT, MAX_FIT)` and assigns the owner; counterpart `initialise.C::initialiseOne` |
| Native allocation/free | Raw `fda6ad4a9753ad7dc5d291406f2f310a9161423f:TKmatrix.C::malloc_blas/free_blas`: `calloc` of cells plus two dimension cells and a separate row-pointer allocation |
| Rust allocation | `rust/src/tk_matrix.rs::BlasMatrix::malloc_blas`: fallible `Vec<f64>` reservation followed by `resize(len, 0.0)`, held in `Arc<Vec<f64>>` |
| Rust mutation/lifetime | `BlasMatrix::cells_mut` uses copy-on-write; `PulsarContext` tracks matrix owners and source aliases |
| Existing full-workflow benchmarks | `validation/parallel_execution/performance.py`, with portable staged execution in `validation/release/workers.py`; installed serial measurements in `validation/release/performance.py` |

The unchanged matrix source hash at this inspection is
`75534fee4b18e7634ce20e6c65f8df859adcd800d22e2aa5022f6c4ddab86857`.
Native `TKmatrix.C` is
`3817bedcd680e88be6b715b1165eacec028f942d3213eec1b0f02c097a01fb11`.
Rebind the actual candidate, build profiles and executables at release.

Each covariance payload is 800000000 bytes = 762.939453125 MiB; two payloads
are 1525.87890625 MiB. Headers, pointers and allocator/process storage are extra.
These are ordinary `f64` cells, not Rug allocations. AK's 833.4 MiB one-pulsar
Chebyshev and 1578.1 MiB two-pulsar fit figures are medians of whole-process
maximum RSS. Their proximity to the payload arithmetic does not prove residency,
actual allocation count, simultaneous lifetime or causation.

## Prospective ownership

At release, confirm a bounded owner for the shared matrix implementation and
its directly necessary lifecycle/tests/report changes, plus a new
`validation/allocation_profile/` evidence recipe. AL ownership must first be
closed or explicitly partitioned. Other writers' changes, canonical TEMPO2,
accepted historical runners and evidence must remain intact. Source experiments
use disposable snapshots; generated traces, JSON, binaries and archives stay
ignored. This draft does not allocate those paths to a live chat.

## Dependency-ordered work

1. **Freeze and measure the unchanged baseline.** Record exact native/Rust
   sources, executable hashes, compiler/linker/allocator and OS/page size,
   feature/provider/worker settings, inputs and runtime resources. Begin with
   the existing serial/default profile and one/two initialized pulsars. Observe
   both the matrix factory and the real full pulsar initialization path, since
   observations, parameter storage and retained owners also allocate. Record
   requested and actual allocation sizes/counts/ranges/lifetimes, initialization
   elapsed time, current process RSS, peak RSS, mappings and page-residency or
   page-fault information supported by the executing host. A difference of two
   peak-RSS counters cannot establish live allocation residency.
2. **Separate untouched and touched controls.** Capture residency before reading
   or writing the payload. Exhaustive zero checks, forced reads, forced writes
   and copy-on-write checks belong in separate, explicitly identified phases or
   processes so inspection does not contaminate the untouched baseline. Include
   one/two live matrices and observed clone/reinitialization/drop behavior.
   Distinguish logical deallocation from allocator retention and OS reclamation.
   Prefer existing OS/profiler facilities and small source-bound observers;
   establish output neutrality and observer overhead before attributing costs.
   Do not replace the baseline with an instrumented run's wall-time claim.
3. **Choose an allocation approach from evidence.** First assess the actual
   pinned Rust standard-library APIs and mature maintained alternatives for
   initialized positive-zero `f64` storage, fallible allocation, contiguous
   layout, ownership/copy behavior and the selected targets. Distinguish an API
   guarantee from measured lazy page residency. Keep the existing allocation
   if the lead is not supported or alternatives do not improve it. Preserve
   dimensions, stride, alignment and allocator/deallocator compatibility, all
   initialized values, checked size/capacity boundaries and recoverable errors.
   A new dependency or unsafe boundary requires a concrete need; explain the
   source counterpart, initialization, layout, aliasing, lifetime and freeing
   obligations beside any necessary unsafe code. Exposing uninitialized cells
   as `f64`, shrinking matrices or lowering precision is outside this work.
4. **Measure the proposed change under matched conditions.** Use separate
   immutable before/after builds and fresh processes with repeated interleaved
   samples. Reuse AK's defined one-pulsar `cheby` and two-pulsar
   `correlated-global` CLI workloads and actual generated products. Preserve
   their PAR/TIM, capacities, NITS, covariance model, runtime, serial settings,
   output files and measurement scopes. Report current/peak RSS, allocation and
   residency observations and initialization/full-workflow wall and CPU time,
   including every failed or outlying sample. Establish whether memory work is
   removed, shifted to first use or moved to another owner. Recheck both selected
   release targets before making a two-platform improvement claim. No new speed
   or memory acceptance threshold is invented by this draft.
5. **Protect affected behavior.** The factory is shared by covariance storage
   and ordinary matrix/BLAS paths. Reuse the existing lifecycle and matrix
   controls for full dimensions, positive-zero bit patterns, row access/stride,
   mutation, aliases inside a context, independence after context cloning,
   allocation/error recovery and destruction. Check enabled provider consumers
   when the owner/layout or shared constructor changes. Use bounded evidence for
   allocation failures rather than exhausting the host. Require unchanged
   same-engine scientific results, file membership/products and state under
   the existing explicit metadata projections. Re-run the relevant existing
   native/Rust gates without changing them; inherited failures remain visible.
   Do not substitute rounded terminal values for complete scientific products.
6. **Return evidence and a scoped implementation decision.** Deliver the
   source-bound baseline, alternatives considered, original and changed
   executions, a causal account with remaining uncertainty, and a report such
   as `rust/ALLOCATION_PROFILE_REPORT.md`. A failed optimization is a valid
   investigation result and must remain recorded. Commit authored scoped work
   only. Central reviews measured benefit, unchanged behavior and any material
   platform dependence before accepting or publishing the optimization.

## Explicit limits

No scientific algorithm, expression order, precision policy, runtime resource,
output convention or numerical/discrete gate changes. Keep the exact SVD-order
obligation and other inherited failures. Broad predictor marshalling, fitting
parallelism, worker-pool changes and unrelated performance work require their
own evidence and scope. This pass tests the covariance allocation lead first;
it does not promise to remove all measured Rust overhead.

There is deliberately no executable dispatch prompt or active chat until
central releases the post-AL ownership and exact baseline.
