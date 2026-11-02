# Assignment AO: predictor CPU optimization

Released by central decision 0123, 2 November 2026. Work only in isolated clone
`/private/tmp/tempo2-ao-predictor-20261102`, initially clean at
`68973d669af0e1180ad8532bb8b364452ca203d7`. Read this current handoff, 0123 and
HANDOFF.md from `/Users/Eric.Dong/Downloads/tempo2rust`; the isolated baseline
predates the release documents. AL owns main; AN finishes memory verification.
You are not alone in the codebase. Preserve all other work and do not edit main.

## Goal and contract

Deliver a measured reduction in standalone predictor C evaluation cost while
preserving the complete supported interface and scientific behavior. Start by
reproducing and profiling the real current route; if the suspected cost is not
confirmed, follow the measured dominant cost instead. Do not stop at a profiling
plan when an equivalent bounded improvement is supported. Central retains
scientific decisions, integration, publication and final acceptance.

Read 0084, 0096, 0098, 0099, 0109, 0117 and 0123 plus the library interface API/map,
parallel execution report and current predictor qualification scripts. Preserve
all 64 predictor symbols, public headers/layouts and source reader/FILE/callback,
locale, class/sign, error, ownership and caller-exclusivity semantics. Callers
can inspect and mutate their exposed structs and coefficient storage between
calls; caching solely by pointer identity is not valid. Existing process globals
and the foreign-thread MPFR cache owner remain part of the supported contract.

Timing remains Rug128. Ordinary predictor arithmetic retains the measured
macOS arm64 53-bit and Linux x86_64 64-bit semantics, operation order and rounding
boundaries. Keep one canonical Rust scientific implementation. Do not lower
precision, substitute a C scientific kernel, change output conventions or loosen
any numerical gate. A representation/allocation improvement needs evidence of
identical required behavior, source-counterpart comments and measured benefit.

## Ownership

Own `rust/src/predictor_ffi.rs`, directly related private representation helpers
in `rust/native/predictor.c`, and necessary representation-only changes in
`rust/src/predictor_number.rs`. The same C file contains ordinary event-libm
helpers: preserve those functions and their existing contract. Coordinate any
unavoidable coupling with central before editing their behavior. Public native
scientific routines, headers and unrelated timing/fitting modules are read-only.

Own `validation/predictor_performance/` and `rust/PREDICTOR_PERFORMANCE_REPORT.md`.
Reuse existing qualifier/benchmark functions; do not rewrite historical runners
or create a general profiling framework. Changes to canonical Cheby/T1 leaves,
build glue or dependencies need a concrete measured reason and a scoped central
ownership update first. Do not create a second evaluator or benchmark-only path.
Commit authored files in small Conventional Commits; never commit generated
JSON, binaries, traces or large profiling artifacts. No push, workflow dispatch
or independent messaging to other chats is authorized; report here for central.

## Work sequence and evidence

1. Freeze the native reference, current Rust source/executables, installed ABI,
   predictor file, toolchains, libraries and environment. Reuse
   `validation/parallel_execution/evaluation.c` and the portable release
   performance path. The existing client already accepts an iteration count;
   longer profiling runs need no changed arithmetic. Measure the complete
   caller-exclusive C phase/frequency loop and separately identify representation,
   polynomial construction, allocation and canonical-evaluation costs. Record
   profiler overhead and actual loaded code. A source suspicion or aggregate
   checksum alone does not establish the cause or scientific equivalence.
2. Assess existing Rug/MPFR/native and standard-library facilities against the
   measured bottleneck before custom conversion/storage infrastructure. Inspect
   pinned APIs and test critical exact conversion, subnormal, signed-zero,
   nonfinite and host-width requirements. Explain reuse versus new code. Avoid
   dependencies or unsafe code without a concrete necessity; document layout,
   initialization, aliasing, lifetime and deallocation beside any unsafe boundary.
3. Implement the smallest measured equivalent improvement. Preserve direct caller
   mutations, callback extents/partial states, short-lived borrows, cleanup and
   all error/publication behavior. Reuse the canonical numeric leaves. If a
   consequential precision/scientific/interface choice is necessary, present
   evidence and a recommendation to central before the dependent change; keep
   independent profiling and verification moving.
4. Use immutable before/after builds and fresh interleaved processes. Measure
   loaded phase/frequency evaluation and end-to-end installed clients separately,
   including initialization/parse costs, wall/CPU time, RSS and relevant allocation
   counts. Retain all samples/outliers and input identities. Coordinate quiet
   timing windows through central; compiler work or another benchmark can
   invalidate an apparent gain. No invented performance threshold or assumed
   speedup is part of acceptance.
5. Reuse the installed C11/C++17 static/shared ABI, storage, low-count, locale,
   pipe, callback, mutation and class/sign controls. Compare complete per-call
   scientific outputs and relevant state, including both Cheby and T1 callers
   where shared transport changes. Protect direct mutable coefficients and
   failure-then-valid recovery. Keep existing native/Rust gates and inherited
   failures explicit. Recheck affected Rust consumers and the existing scoped
   C sanitizer/thread-cleanup tests if their boundary changes. Avoid rerunning
   unrelated whole timing/fitting campaigns for a confined bridge improvement.
6. Deliver exact commits, executable recipes and the measured effect with its
   limits. macOS evidence may support integration while actual Linux follow-through
   is pending; it cannot establish a two-platform claim. Prepare a concrete Linux
   recipe that uses the caller's prepared cache and isolated build targets, and
   send central the minimal qualification wiring needed. Central owns GitHub
   publication/dispatch and will combine independent changes into coherent runs.

Completion requires a justified implemented improvement, preserved supported
behavior, measured before/after benefit and explicit platform evidence. If the
initial lead fails, return the measured result and continue to the next supported
cost within this scope rather than force a speculative optimization.
