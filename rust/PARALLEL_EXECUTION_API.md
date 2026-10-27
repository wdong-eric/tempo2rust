# Observation execution and concurrent callers

The default remains serial. The canonical timing, fitting and predictor paths
use the selected observation policy; fit matrices, solvers and reductions remain
sequential. No Cargo feature, environment variable or production CLI flag is
needed. Select a positive maximum on the calling thread:

```rust,no_run
use std::{num::NonZeroUsize, path::Path};
use tempo2_rust::{bats, execution, timing, RuntimeContext, TimingError};

fn calculate() -> Result<(), TimingError> {
    execution::with_workflow(|| {
        execution::with_observation_workers(NonZeroUsize::new(4).unwrap(), || {
            let root = Path::new("T2runtime");
            let pairs = [bats::InputPair {
                par: "model.par".into(), tim: "arrival.tim".into(),
            }];
            let mut input = bats::load_pairs(&pairs, root)?;
            bats::preprocess(&mut input, &[], root)?;
            let mut runtime = RuntimeContext::from_root(root)?;
            let result = timing::evaluate(&mut input, &mut runtime, true)?;
            // Result diagnostics belong to this complete admitted workflow.
            assert_eq!(result.pulsars.len(), 1);
            Ok(())
        })
    })
}
```

`with_observation_workers` restores the previous selection on return or panic.
`observation_workers()` reports the current maximum. A new caller thread starts
at one and selects its own maximum. Counts above one create scoped threads for
each pulsar, in outer pulsar order. Contiguous chunks have size
`ceil(n / min(maximum,n))`; empty and single-row cases need no extra thread.
Consequently five rows with a maximum of four use three workers. This is a
maximum, not a promise of a fixed pool or a speed improvement. Thread creation,
row scratch allocation and ordered publication cost time and memory.

## Complete workflows and process state

The safe Rust process-session entry points acquire workflow admission before
the existing session mutex. Complete `bats::evaluate`, `timing::evaluate`,
bounded fitting, general fitting and `core_cli::run` hold admission through
their result/diagnostic collection. Predictor constructors hold it throughout
construction. A workflow spanning multiple calls—loading, changing controls,
constructing predictors, writing output and draining diagnostics—uses an outer
`with_workflow`. All participating callers then serialize those complete
workflows. Scheduling chooses their order; separate RuntimeContexts do not
establish independent clock, warning, log, RNG or cache histories.

Admission can nest on its owning thread. The session mutex remains
non-reentrant: callbacks supplied to `clkcorr::with_process` or `with_resources`
must use the borrowed owner, never a wrapper that reacquires the session. Do
not spawn and wait for another process-owned workflow while holding admission.
Workers borrow immutable source data and resource roots and never acquire the
parent's session. They do not run user callbacks. Files, environment variables,
locale and provider configuration must remain stable during these workflows.

Pure owned numerical operations and explicitly owned `ToolkitRng` values can
run independently where their existing ownership contracts allow it. Shared
toolkit source controls still use the process session. Direct JPL initialization
and status calls participate individually; callers needing their paired meaning
place the pair in one outer workflow. CALCEPH descriptors remain uniquely owned
per pass and are closed before returning; they are not shared with observation
workers. SOFA/Fortran and accelerated fitting execute outside that observation
loop. Provider internal threading is a separate provider policy.

## Effects, failures and precision

All worker counts execute one observation routine with the same Rug128 and
native-width expression boundaries. Each worker owns observation scratch,
opened SWM files, provenance and diagnostic intents. The parent commits rows
and replays effects in the original observation order into the single source
history. DMMODEL suppression, log counters/backing, version history and all
message bytes retain their serial meaning.

A returned error publishes the successful prefix and the partially changed
failing row with its effects. Later speculative results are discarded. They
may already have read resources, but their provenance/messages are not
published. Every worker is joined and its files closed before return. A panic
joins and unwinds; it is neither retried nor turned into a scientific result.
Admission has no data to poison; the existing session poison and terminal
clock-error policies remain unchanged. Use the existing recovery contract for
the failed operation; no new automatic reset or rollback is promised.

## Standalone predictor C calls

The source ABI still exports process-wide `ChebyModelSet_OutOfRange` and
`tempo2_verbose`. **Callers provide process-wide exclusivity**, covering calls,
direct global access, callbacks and borrowed storage/streams/names/segments for
the duration of their use. Separate model allocations do not establish C
concurrency: Cheby lookup/evaluation can write OutOfRange. Direct locale changes
must obey the same exclusivity. The Rust workflow guard does not lock arbitrary
C accesses to exported data. ABI layout, native long double, fixed T1 backing,
partial reads and diagnostic behavior remain the AJ contract.

## Runnable driver and evidence

`cargo run --release --manifest-path rust/Cargo.toml --example parallel_execution
-- bats 4 3 RUNTIME OUTPUT.json - MODEL.par INPUT.tim` runs three evaluations.
Modes `timing` and `fit` exercise the corresponding complete public workflows;
the covariance argument is `-` or a model path. `raw-bats` times loaded
`form_bats_all` without scientific output projection and is only a measurement
mode. Its null output is not scientific evidence. Validation-only external CLI
wrappers read `AK_WORKERS`; production does not interpret that variable.

See [PARALLEL_EXECUTION_REPORT.md](PARALLEL_EXECUTION_REPORT.md) for measured
costs, executed boundaries and limits, and
[PARALLEL_EXECUTION_CONTRACT.md](PARALLEL_EXECUTION_CONTRACT.md) for the frozen
source/state map. Linux/provider ABI and two-platform release qualification
remain AL work.
