# Assignment AN: covariance allocation and memory optimization

**Delivery update, 2 November 2026:** Central 0123 accepts the measured macOS
change and merged AN through `6cc0b79`; implementation ownership is closed.
AL owns actual Linux follow-through. This does not claim two-platform acceptance.

**Released under decision 0121, 2 November 2026.** The user asked central to
prioritize substantial migration progress and put small differences in one chat.
AL continues compatibility/qualification work; AN now owns this independent
performance stage. This supersedes the earlier draft's post-AL waiting condition,
without accepting AL or any outstanding scientific gate.

The exact baseline is `dc435f53a64e4dd5c25cae6319841a28bd981fcb`. Work only in the isolated clone
`/private/tmp/tempo2-an-allocation-20261102`; the main checkout remains AL's work
area. Read the current released handoff and decision 0121 from the main checkout,
not the older draft present in the baseline snapshot. This is a full authorized
measurement/implementation task within the bounds below: do not stop at a plan
or request another release merely to implement a demonstrated equivalent allocator.
Central retains numerical policy, integration, publication and acceptance.

Active chat: **Assignment AN: Covariance memory optimization**, thread
`01a19a64-27ab-77c3-a36e-416defad0906` on the local host. Central verified its
initial work is active; report and acceptance remain pending.

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
Bind the released baseline, every implementation revision, build profile and executable in the actual measurement receipts.

Each covariance payload is 800000000 bytes = 762.939453125 MiB; two payloads
are 1525.87890625 MiB. Headers, pointers and allocator/process storage are extra.
These are ordinary `f64` cells, not Rug allocations. AK's 833.4 MiB one-pulsar
Chebyshev and 1578.1 MiB two-pulsar fit figures are medians of whole-process
maximum RSS. Their proximity to the payload arithmetic does not prove residency,
actual allocation count, simultaneous lifetime or causation.

## Released ownership

AN owns `rust/src/tk_matrix.rs`, directly necessary matrix/lifecycle controls
in `rust/src/pulsar_lifecycle_tests.rs`, a small observer example if needed,
`validation/allocation_profile/`, and `rust/ALLOCATION_PROFILE_REPORT.md` in its
isolated clone. The lifecycle implementation is read-only unless a directly
necessary allocation/lifetime change is justified and coordinated with central.
Cargo manifests/build glue may change only for a demonstrated required dependency
or observer integration; prefer existing facilities. No fitting/event/precision,
CLI presentation, source-comparison or qualification-workflow changes belong here.

You are not alone in this project. Preserve AL's main-checkout changes and all
other authors' work. Commit only AN-owned authored paths in the isolated clone;
return commit identities for central integration. Do not push, dispatch workflows,
change canonical TEMPO2, rewrite historical runners, or commit generated evidence.
Freeze each before/after build from an exact commit/export. Avoid simultaneous
heavy runs when measuring timing; record host load and coordinate a quiet window
through central when needed. Report in this chat; central will read and relay it.

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
   release targets before making a two-platform improvement claim. No speed or memory threshold is invented; report the measured effect and its limits.
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
output convention or numerical/discrete gate changes. Preserve the user-adopted paired-mode contract in 0120 and all other inherited
scientific/discrete gates; no precision or output adjustment is permitted. Broad predictor marshalling, fitting
parallelism, worker-pool changes and unrelated performance work require their
own evidence and scope. This pass tests the covariance allocation lead first;
it does not promise to remove all measured Rust overhead.

Return the first measured allocation/residency diagnosis promptly, then continue
the authorized equivalent implementation and focused verification when supported.
If this lead does not explain the memory cost, report the evidence and next
measured allocation target instead of forcing a change. Final delivery must state
what improved, by how much, what stayed scientifically identical, and which
platform/execution remains unverified. Do not wait for every AL mismatch to close.
