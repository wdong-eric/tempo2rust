# AK execution contract (frozen before implementation)

Baseline: AJ production `3e41497d5a86e627b0ce994d4882e4b2e77d3bf7`,
qualification `31c729079da44afe7155e42707edeac20d66bd2b`, report `f4b73f8`;
central release `3c90d1e51d6315edeab7e6e296c45a1d2ea419a6` and dispatch
`16c516f`. Governing decisions: 0085/0096, 0053, 0039, 0074, 0082.

## Public selection and complete workflows

`execution::with_observation_workers(NonZeroUsize, closure)` selects a scoped
maximum worker count on the calling thread. Default and count one execute the
existing serial path. Counts above one use standard-library scoped threads;
there is no build feature, environment inference, new numerical backend or
fitting reduction. The scope restores selection on return and unwind. Outer
pulsars stay sequential; actual work is limited by the observation population.
New caller threads select their own controls. Worker numerical code uses the
same observation routine, Rug128 and source expression/type boundaries.

`execution::with_workflow(closure)` holds an outer process admission guard
through loading, controls, calculation, output and diagnostic collection. Safe
process-session entry points participate in that guard. Nested entry on the
same caller thread does not reacquire admission; the existing ClockSession
mutex remains non-reentrant and borrowed-session callbacks must not call its
locking wrappers. No source state is reset, copied into a RuntimeContext or
made thread-local. Complete result-producing timing/fitting and native CLI
operations hold admission themselves. Composed multi-call library workflows,
including predictor generation followed by diagnostic collection, use the
explicit outer scope. Other callers wait; independent-context histories are
not promised. Do not launch and wait for another process-owned workflow while
holding the scope. Pure owned numerical operations need no admission.

## Source/state/lock map

| Boundary | Owner and order |
| --- | --- |
| Complete logical workflow | Admission, then existing process session; nested admission only on owning thread |
| Clock/EOP/weather/IFTE/Fortran/FB/UT1 resources | Existing ClockSession; borrowed owner operations; original lazy/cache/terminal lifetimes |
| calculate_bclt.C: outer pulsars and troposphere | Parent session, original source order |
| calculate_bclt.C: observation loop | Shared immutable pulsars, separate observation scratch rows; no session lock acquired by workers |
| dm_delays.C static warning mask | Existing SourceState mask; ordered replay of tagged once-only diagnostic groups |
| TKlog/global.C version bookkeeping | Existing Diagnostics counters/backing/keys/version history; observation-local transcripts replayed in pulsar/observation/call order |
| sw_delay.C SWM=1 | Borrowed immutable root; worker-owned opened files and resource records; ordered provenance publication |
| JPL initialization/status | Existing atomic source status plus paired-init lock; direct init/status composition requires workflow admission |
| CALCEPH | Existing owned descriptors per pass, closed on success/error; outside observation workers |
| SOFA/Fortran | Existing recursive native profile, outside observation workers; complete process workflows serialize |
| Accelerate | Existing thread-local caller selection and owned matrix buffers; no new parallel fitting; propagate selection explicitly to observation workers |
| Toolkit RNG | Existing explicit RNG owners and process SourceState, no reset |
| Predictor C | Caller-owned storage/streams/callbacks plus process globals; caller serialization across all potentially global-writing operations, including Cheby lookup/evaluation |

The C exported OutOfRange/tempo2_verbose data retain their ABI and lifetime.
Separate models do not establish C concurrency. Callers serialize accesses to
these data and calls that can touch them, including callbacks and borrowed
streams/names/segments until their use ends. Native long-double conversion,
caller locale, separate phase/frequency reads, fixed T1 storage/counts and
partial-read semantics remain AJ's contract.

## Effects and failure projection

Worker transcripts are presentation intents, not independent source sessions.
They contain stdout/stderr bytes, log calls, version requests and tagged
DMMODEL once-only groups. Parent replay updates the one real source history.
Per-observation scientific values and discrete choices compare exactly between
Rust worker counts. Serial diagnostic bytes, ordering, counts, suppression,
resources and version histories remain the reference; CPU elapsed values and
instrumented worker identities are separately identified measurements.

Rows are calculated in scratch storage (one observation copy, never a pulsar
or session copy). Publication follows source observation order. On a returned
error publish the successful prefix and partially mutated failing row with its
effects/resources, then stop; later speculative row results are discarded.
All scoped threads join and their opened resources close before return. Later
rows may already have read resources, but do not publish their provenance or
diagnostics or write external output. Serial clock terminal faults retain their
existing meaning. A panic joins workers and unwinds; it is not silently retried
or converted into a scientific result. Resource files/environment/locale must
not be changed concurrently by callers.

## Dependency-ordered implementation and checks

1. `execution.rs`, `clkcorr.rs` and complete workflow callers: admission and
   minimal scoped worker controls. Reproduce AH's warning ownership failure,
   then test collection/control interleavings and failure/recovery with bounded
   concurrent callers, retaining every original warning comparison.
2. `clock_diagnostics.rs`, `solar_wind.rs`, `propagation.rs`: extract borrowed
   effects and one canonical observation routine; ordered parallel publication.
   Compare full rows, state, diagnostics/resources at 1/2/4 workers; instrument
   actual observation thread identities. Cover unequal pulsars, uneven chunks,
   bypass/deleted/veryFast, DMMODEL, SWM=1, logs/version and failed-prefix recovery.
3. Public docs and external consumers: exercise complete timing, correlated
   fitting, predictors, resource/provider and caller-serialized C histories;
   preserve ABI and all AJ scientific contracts. Use available sanitizers with
   explicit instrumentation limits, not an all-platform safety claim.
4. `validation/parallel_execution/`: disposable pinned native serial/OpenMP
   builds, real compile/link/worker evidence and original Makefile wiring
   diagnosis. Native shared-state races are evidence, never numerical goldens.
5. Matched cold/repeated timing, correlated fitting, predictor work and larger
   validated observations: wall/process time, memory, run variability, actual
   worker/provider counts and precise source/input identities. No speed gate.
6. Affected debug/release, installed/external consumers, fmt and Clippy; explicit
   authored-path Conventional Commits and final report with source/unsafe/
   departure map and the complete unchanged 0096 numerical ledger. AL retains
   actual Linux/provider implementation and two-platform release qualification.
