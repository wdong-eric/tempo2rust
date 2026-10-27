# Parallel observation execution and concurrency: prepared Assignment AK

Central preparation, 26 October 2026. **AK implements the pinned core's optional observation-parallel capability and qualifies the delivered library's concurrency boundaries, with representative performance measurements.** This advances the OpenMP/operational responsibility retained in 0065/0081/0083/0084. It does not replace serial scientific behavior, introduce parallel fitting algorithms or accept unresolved numerical gates.

**Prepared after AJ; not released.** As of 27 October, AJ is the sole Rust writer under 0095; AI implementation handover is approved and its ownership is closed. AK requires AJ's reviewed committed baseline, ownership closure, the 0082 checkpoint and an explicit release under 0074. Once released it owns `rust/` and new `validation/parallel_execution/`. Central owns shared design, numerical policy and acceptance. Preserve 0008, its amendments, accepted evidence and other chats' changes. Apply 0053 to demonstrated small source bugs and 0039 to generated products.

## Source boundary and executable deliverable

The 137 authored root C/C++ source/header files contain one OpenMP parallel region: `calculate_bclt.C:62-64` parallelizes the observations of the currently selected pulsar. The outer pulsar loop remains sequential; tropospheric computation occurs before the parallel region. Each observation retains its complete geometric/Shapiro/dispersion iteration, the `veryFast` exit, deleted-row condition, strict `1e-10` convergence test and 100-iteration limit. Preserve the established expression order, source type boundaries, units and existing corrected behavior, including AC's documented selected-pulsar troposphere fix.

Deliver an explicitly selectable and actually executed parallel observation path through the canonical Rust library and public timing/fit/predictor workflows that call it. Reuse the serial observation algorithm and existing canonical owners. Use safe disjoint observation access and scoped immutable inputs; avoid a second scientific engine, broad unsafe sharing or a copy of the entire pulsar context per observation. Serial execution remains available for comparison and the established serial profile remains unchanged. Freeze the minimal public/build selection and worker-limit contract against the released predecessor before implementing it; this preparation does not add an unreviewed option to current interfaces.

The compatibility obligation is the source observation-parallel capability and its defined scientific behavior. It does not require a Rust implementation to expose the OpenMP runtime ABI, copy every vendor runtime environment variable, parallelize the outer pulsar loop or alter SVD/reduction order. Record actual worker use, provider/runtime controls and any serialized regions. A build flag or multiple launched threads with all numerical work serialized does not establish this deliverable.

## Shared state and diagnostics

The loop reaches `shapiro_delay`, `dm_delays`, interpolation/parameter helpers and, for SWM=1, the solar-wind resource reader. Source inspection identifies concrete shared mutations: `dm_delays.C:160-195` uses a static warn-once bit mask; `TKlog.h` updates global diagnostic counts and indexed log buffers; `global.C::CVSdisplayVersion` updates static name arrays and a counter. SWM=1 opens current-sheet resources and prints diagnostic records during observation work. These are relevant call-path responsibilities, not evidence that concurrent source execution is safe.

Preserve the serial process-lifetime meaning of warning suppression, resource selection/cache histories, RNG state, version reporting and source global controls. Do not silently reset them per worker or per RuntimeContext to simplify parallel execution. Existing Rust drivers borrow one process session under an outer lock; nested internal operations must borrow the scoped owner rather than reacquire its non-reentrant mutex. Worker operations must not block on a lock held by their waiting parent.

Use safe synchronization or an ordered owner-side update for shared effects; identify and test the chosen boundary. Keep per-observation numerical work independent and preserve its operation order. Where native parallel diagnostics have no deterministic global order or contain a demonstrated race, retain the original source witness and document a narrow correction under 0053. Define how messages, counts, warning suppression, resource provenance and errors are attributed and published before comparison. Do not waive a defined scientific field, aggregate away missing events or claim an arbitrary racy native trace is the required result.

A fresh process with serial execution remains the deterministic reference for defined message/state histories. Parallel native traces need a separate explicit projection for scheduler-dependent worker identities/interleaving; it must retain observation/event identity and all invariant contents. Serial fixtures keep their existing diagnostic gates. Document partial-state and output-publication behavior on worker failure, join/cleanup obligations and subsequent valid processing; preserve existing public error contracts unless central approves a consequential change.

## Library concurrency boundary

Inventory permitted concurrent calls after AJ: full timing/fitting/predictor workflows, direct toolkit operations, retained SOFA/native providers, JPL initialization/status pairs, diagnostics, resource roots and C predictor global state. Separate memory safety, deadlock freedom, source process-state semantics and independent-context behavior. A mutex, Send/Sync implementation or successful concurrent launch alone establishes none of the other properties.

Preserve source process-scoped state rather than promising automatic isolation between two RuntimeContexts. For each public operation, document the actual exclusivity/serialization contract and enforce it through the safe Rust ownership/API boundary where needed. Exercise complete supported logical workflows, including diagnostic/result collection, rather than testing isolated driver locks only. Where caller serialization is part of the C predictor contract, state and test that contract; the exported legacy out-of-range datum does not become thread-local under 0084.

Use existing owners and minimal changes at the actual shared-state boundary. Do not introduce a new process service, per-context scientific state model, broad cache redesign or unchecked unsafe Send/Sync solely to satisfy a stress harness. If a complete supported workflow needs a consequential public ownership/error change, present the source-backed alternatives and recommendation to central for the 0082 user checkpoint while continuing independent work.

Retained SOFA's 36-routine closure and `-frecursive` are existing dispositions, not concurrency proof. Verify the called native profiles and provider handles actually used after AI/AJ. Record provider internal threading and oversubscription controls in measurements; do not assume BLAS, CALCEPH, Fortran or predictor C state is safe merely because the Rust call site is safe.

## Native enabled-build evidence

A2 is a serial reference profile. For a parallel native comparison, use disposable pinned source and existing reference builders with explicit per-object compiler/link commands, the OpenMP macro/runtime identity and evidence of more than one actual worker. Retain an otherwise matched serial build. Do not change canonical TEMPO2 or accepted reference evidence.

`Makefile.am` declares `libtempo2_la_calculate_bclt_CXXFLAGS`, while the declared library target is `libtempo2.la`. GNU Automake documents flags per program/library and says per-object flags need a separate convenience target ([official manual](https://www.gnu.org/software/automake/manual/html_node/Per_002dObject-Flags.html)). This is a build-wiring concern inferred from source and that documented rule, not an executed configure/build failure. Do not assume the declaration enabled the region: inspect the generated compile command or an explicitly scoped reference-builder command and prove worker execution. A necessary small build correction requires its original/corrected witness under 0053. No autotools regeneration or enabled OpenMP build was run during central preparation.

## Verification and measurements

Freeze the implementation plan, source/state/lock map, public selection, error/diagnostic projection and executable cases against the actual predecessor. Reuse AC/AD/AE/AF fixtures and accepted references; no new independent reference-only assignment is needed. Add focused cases only for new parallel interactions or realistic gaps.

Compare Rust serial with Rust parallel at one and multiple actual workers, then source serial/parallel on defined native paths. Check all affected observation outputs, discrete selections/counts/order, model parameters, mutation boundaries, diagnostics/provenance and classes. Since the observation algorithm has no cross-observation numerical reduction, a Rust worker-count change should not change its deterministic numerical outputs; do not hide differences behind the wider native/Rug tolerance. Native-versus-Rust quantities retain their existing gates and known named failures.

Include unequal pulsar observation counts, non-divisible work partitions, disabled corrections, deleted observations and early exits; DMMODEL warning histories, SWM=1 resource calls, debug/version/warning modes, and failure followed by valid processing. Exercise repeated supported calls and actual concurrent callers with bounded deadlock detection and ownership/error controls. Use available memory/race tooling where it can test the retained boundary; report tool limitations and exact scope. Do not infer all-platform thread safety from one host or from a clean sanitizer run.

Measure usable workloads with identical inputs, selected models, fit cycles, precision profiles, backend choices and output work: cold load plus evaluation, repeated evaluation, general correlated fitting, predictor generation/evaluation and source-sized larger observation sets. Use already validated inputs and declared deterministic generated data. Report wall/process time, memory, actual worker count, build/provider identities and run variability for source serial, Rust serial and Rust parallel as applicable. Separate startup/resource/serialization costs from numerical work. Keep Rug128 unchanged and show source precision differences; no performance threshold or precision downgrade is authorized here. Measure first; any consequential optimization or revised readiness target goes to the major-stage checkpoint with evidence.

Run affected debug/release checks, external callers required by the changed boundary, formatting and Clippy. A performance run is not a scientific validation. Keep generated JSON, traces, binaries and profiles ignored; commit authored code, focused fixtures/recipes and concise reports only.

Return Conventional Commits and `rust/PARALLEL_EXECUTION_REPORT.md` with the executable parallel deliverable, exact source/build identities, source/lock/unsafe/departure map, supported concurrency contract, measured results and unresolved gates. Central performs the 0082 reassessment before a successor release. Final installation/platform qualification and integrated numerical acceptance still follow; P, M2, 0049 and DDH remain open.

## Preparation evidence

Central read the raw pinned sources below and recorded the OpenMP sites in all 137 authored root C/C++ files. For current ownership context, nine committed Rust/build files were snapshotted at central revision `d795b9d`; AE's uncommitted implementation was not used as a stable acceptance baseline. Snapshot receipt SHA-256: `2b8a40646dcb113a1ec261d0f39208b4b5231dca57a36cb3fcf010dd27ef25de`. Evidence is ignored under `decisions/evidence/parallel-execution/build/preparation-20261026/`. This is source preparation only: no new native/Rust numerical execution, parallel build, stress run or performance claim.

| Pinned source | SHA-256 |
| --- | --- |
| `calculate_bclt.C` | `899c58042bc053efeae5b25eacc46de9b4ca1bb0943c995a846e0affb8d597e7` |
| `dm_delays.C` | `43e563cbdc570be9d836e0fe26576ad8512261494e5267c28bee99c59a84c7a8` |
| `shapiro_delay.C` | `9ebbfb8af3a5309bf7ad852f1cf5e11408f12190a6902e6706142e53619238d2` |
| `sw_delay.C` | `f71de099d59cc6e1adb71d34bf72ed4fe1649248c84a143f2420d21b211647c2` |
| `tropo.C` | `871d06bb10e647df0d563905db20b560cd659fcb4b372fed0c790937835a7fe6` |
| `global.C` | `e61fc1a83a9adf0b6d905d2c5a58cd764a1fd0261c276fccb33daadb6788a2a7` |
| `TKlog.C` | `696904f67674dcfa51a485b0943d10bd39358daeb14649fe4c5d728d667f1d90` |
| `TKlog.h` | `977dc204af66a0e61fe3b6288c630e3a3343ebc9dc08d5e1f2e535b5ea518e3c` |
| `T2toolkit.C` | `70a6f7c0f24e5f55e244a02edbe49f7cbcc56068dc364e8e7cb7efe2c9cdd6c6` |
| `Makefile.am` | `172ec480482f0261f0a845e9aface382c1a441f4e80d53132574bf8ff912bd7a` |
| `configure.ac` | `1be741a57a7cfd2001da6f98327ff7b6a34e5657be6e13db72de912960dd8aa9` |
| `autoconf.boot/ax_openmp.m4` | `2d1e61ed0ee747cb9a637aa55425dc0e72a396584f520d4797f930145b75c4be` |

## 27 October preparation against the delivered AI baseline

Central inspected twelve committed Rust files at AI delivery `20e368fe6e7d3e05565ae7f0132f4498dfa9069e`; those exact bytes are snapshotted with this preparation. AJ's ongoing interface edits are not frozen or accepted by it. Reconcile this map with the completed AJ handover before releasing AK.

| Delivered boundary | Consequence for AK |
| --- | --- |
| `clkcorr::with_process`, `with_resources`, `with_diagnostics` and `lifecycle_diagnostics` lock one process `ClockSession` | Internal observation work must use the already scoped owner/effects rather than reenter these public wrappers from workers while the parent holds the session. |
| `global::form_bats_all` and `update_bats_all` hold the session through their internal timing chain | Preserve the actual outer lifetime and source pulsar order when extracting the observation work; do not split the public timing operation into uncoordinated worker acquisitions. |
| `propagation::calculate` uses immutable source pulsars plus mutable observation rows, session diagnostics, `source.very_fast`, DMMODEL suppression and resource records | Reuse one observation calculation with disjoint row ownership. Shared-effect publication needs its own explicit order and failure boundary; a clone of each entire session/pulsar would change process history and add avoidable work. |
| `solar_wind::model` reads the selected session root, appends resource provenance and emits diagnostics | Include these actual effects in the observation contract; pure geometric cases alone do not establish SWM=1 concurrency. |
| `take_clock_diagnostics` drains all pending process events under a separate acquisition; `bats` collects diagnostics after the timing driver returns | Per-driver mutual exclusion does not prove that a complete concurrent operation collects its own diagnostics. Preserve the original AH failure and test workflow/collection ownership, counters and controls together. This source observation is not itself a reproduction of that failure. |
| AI `t2accel::with_acceleration` scopes a thread-local selection, while fitting and the native CLI establish their own outer scopes | A newly spawned thread does not inherit the caller's selection automatically. Keep selection explicit wherever a worker-reachable call uses it, distinguish native provider internal threading, and avoid changing algorithm selection accidentally during concurrency work. |
| AI CALCEPH public wrappers acquire the process session once; global timing uses borrowed-session calls and separate owned descriptors per pulsar/pass | Retain descriptor ownership and error cleanup. Do not use raw-handle sharing, recursive locking or a broad unsafe Send/Sync implementation to add concurrency. Actual concurrent provider behavior still needs verification. |

The future predictor C boundary remains AJ work. Its exported source data and caller-owned structures have the exclusivity/alias/lifetime contracts in 0084; no current native worker probe establishes that ABI's concurrency. Production Rug128, source process-lifetime semantics and all 0095 numerical gates remain unchanged.

### Executed native worker witness

Central compiled the actual pinned `calculate_bclt.C` in disposable source, with one observation-entry callback recording the worker ID in that observation's own slot. The callback changes no source scientific expression; original and instrumented source, the exact patch, copied A2 headers, the pinned A2 archive identity and all compiler/link commands are retained. The control compiles without the OpenMP source macro/compiler flag; the enabled object uses both `-DHAVE_OPENMP` and `-fopenmp`, linked with GCC's OpenMP runtime. No canonical or accepted reference file was changed.

One initialized pulsar has 259 distinct observations with `delayCorr=0`, disabled troposphere and fully supplied position/velocity/epoch/frequency fields. Debug/version/timing diagnostics are disabled. Each observation is visited once. The serial control observes worker 0 even with four requested threads; enabled runs observe exactly one, two and four worker IDs at the corresponding requests. GCC 16.2 reports `_OPENMP=202111` for the enabled translation unit and zero for the control. Six recorded output quantities per observation agree byte for byte with the serial control: **4,662 exact comparisons, zero differences**. The common output SHA-256 is `97b3c1c6c2b3abe68711ac79c85b0435133035ecd0d4a4bd15ac131b89459ae4`.

This proves actual worker participation in the pinned loop on this host for that supplied bypass state. It does not qualify full timing corrections, unequal pulsar populations, shared warning/version/resource paths, failure publication, race/deadlock freedom, Rust parallelism, performance or Linux. The source Makefile flag-wiring concern above remains an inference: Autoconf/Automake tools are unavailable in the inspected standard/local tool paths, and no generated Makefile command was exercised. The explicit object build must not be described as proof that the original Makefile enables OpenMP.

Authored recipe: `decisions/evidence/parallel-execution/central-worker-preparation.py`, executed as `/tmp/tempo2-central-openmp-preparation.py`. It creates a new temporary directory, records exact inputs/outputs and asserts actual workers and equality. Original directory: `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-central-openmp-7a6wagv1`. The 94 copied evidence files remain ignored under `decisions/evidence/parallel-execution/build/worker-preparation-20261027/`; copied receipts keep their original paths. Receipt SHA-256: `10be1c1f325d0f7d5706d72541ed51f9b0b03c37ba299dc392aac5e2ac957b37`; preservation SHA-256: `df805920181b8c1b1f6883837aef2e47486399473ea2cd8327ef529260a23a9b`; executed script SHA-256: `65f9e5221a5695515a46fb21edb1c83e09e859f9eee40ddfe910e4bb5df673c6`.

This adds bounded preparation and actual host capability evidence. AJ retains sole Rust ownership; AK remains unreleased and still owes its complete source/parallel/concurrency/performance deliverable and 0082 checkpoint.
