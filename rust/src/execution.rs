//! Execution policy around the existing source process owner; no scientific state.
use std::{
    cell::Cell,
    num::NonZeroUsize,
    sync::{Mutex, MutexGuard},
};

static WORKFLOW: Mutex<()> = Mutex::new(());
thread_local! {
    static ENTERED: Cell<bool> = const { Cell::new(false) };
    static WORKERS: Cell<NonZeroUsize> = const { Cell::new(NonZeroUsize::MIN) };
}

/// Serialize a complete source workflow, including controls and diagnostic
/// collection. Source caches, warnings and RNG retain process lifetime.
/// Nested calls on this thread are supported. Do not wait for another thread's
/// process-owned work inside this scope. ClockSession callbacks must still use
/// borrowed operations: this does not make the session mutex reentrant.
pub fn with_workflow<T>(operation: impl FnOnce() -> T) -> T {
    let _guard = enter();
    operation()
}

pub(crate) struct WorkflowGuard(Option<MutexGuard<'static, ()>>);
impl Drop for WorkflowGuard {
    fn drop(&mut self) {
        if self.0.is_some() {
            ENTERED.set(false);
        }
    }
}
pub(crate) fn enter() -> WorkflowGuard {
    if ENTERED.get() {
        return WorkflowGuard(None);
    }
    // Admission owns no scientific data. A panic releases it; the existing
    // ClockSession retains its own poison/error policy for interrupted mutation.
    let guard = WORKFLOW.lock().unwrap_or_else(|p| p.into_inner());
    ENTERED.set(true);
    WorkflowGuard(Some(guard))
}

/// Maximum observation workers in the current call scope; one is serial.
pub fn observation_workers() -> NonZeroUsize {
    WORKERS.get()
}

/// Select observation parallelism for timing, fit and predictor calls in this
/// closure. Default is serial; selection is restored on return or unwind.
/// New caller threads have their own default, and must select explicitly.
pub fn with_observation_workers<T>(workers: NonZeroUsize, operation: impl FnOnce() -> T) -> T {
    struct Restore(NonZeroUsize);
    impl Drop for Restore {
        fn drop(&mut self) {
            WORKERS.set(self.0);
        }
    }
    let _restore = Restore(WORKERS.replace(workers));
    operation()
}
