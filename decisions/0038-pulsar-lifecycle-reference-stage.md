# Pulsar lifecycle reference and Assignment S

Central decision, 24 October 2026. **Release Assignment S to establish complete pulsar state, initialization, allocation, destruction and copy evidence.** S owns only `validation/pulsar_lifecycle/`. R remains the sole Rust owner under 0037; P and Q retain their independent reference directories. This closes a reference dependency for the remaining pulsar-state Rust contract; it does not delegate that contract or accept the full input lifecycle.

## Source and scope

Use raw canonical revision `fda6ad4a9753ad7dc5d291406f2f310a9161423f`:

| Source | Raw SHA-256 |
| --- | --- |
| `tempo2.h` | `0313b6eaf9f9ae9d7e5cce340379ad4b7068394e5222a01f73949e3a2640de93` |
| `initialise.C` | `5da876556455e5fda635b30917ae9e745ef880b93ef513b47b6e4823458868c1` |
| `tempo2Util.C` | `1f8f7534c98404958d8839250d5107b2af90ad8d790547405a391c5494264765` |
| `TKmatrix.C` | `3817bedcd680e88be6b715b1165eacec028f942d3213eec1b0f02c097a01fb11` |
| `TKmatrix.h` | `b089efa5ddd1dda2c2615a41a119d015209b908858899713e4ec6bd47270ca84` |

Inventory all **349 named pulsar fields**, their source types/extents and nested `parameter`, `observation`, `clock_correction`, `storePrecision`, `FitInfo` and `FitOutput` state, relevant enums/constants and callback signatures. The released field manifest is checked against accepted M's complete field inventory. A type/ABI map is distinct from a claim that a callback, fitter or other consumer was executed.

The twelve primary routines are `initialise`, `initialiseOne`, `allocateMemory`, `destroyOne`, `destroyMemory`; `copyPSR`; and `malloc_blas`, `free_blas`, `get_blas_rows`, `get_blas_cols`, `malloc_uinv`, `free_uinv`. Other functions in compiled support units remain dependencies, not automatically covered modules. Preserve the actual allocator/accessor implementation and the complete initializer/copy functions; do not replace them with harness approximations.

K's bounded initialization/parameter evidence, M's copyPSR evidence and O's accepted parameter store are read-only context. Extend their missing whole-state and lifetime/caller histories rather than regenerating their goldens. R's evolving Rust code is not a native oracle. Build unchanged raw source and real dependencies with the measured GCC binary128/C-locale profile, exact source/config/header/compiler/backend/runtime/runner identities and no A2 corrected engine.

## Defined state and caller ownership

Write the complete field/type/initialization/read/write/retention map and case classifications before experiments. Separate three things: writes performed by source initialization, explicit caller seeds and genuinely unwritten storage. Do not infer a source default from calloc used by the harness, poison bytes, a lucky allocation or a previous case.

For natural initialization, project only fields/cells/bytes actually written by source or a declared allocator contract. Mark everything else unwritten without reading it. For retention experiments, initialize every field that will be observed with valid, distinguishable typed caller values before the native action. Do not use arbitrary byte patterns as floating, enum, pointer or callback values. All enums, callbacks and pointer targets must be valid for the intended operation. Classify pointer identity and alias relationships using declared symbolic owners/offsets rather than unstable addresses; never serialize or dereference freed/unwritten pointers as ordinary data.

Capture complete initialized scalar/array/string state, all tails, parameter backing, observation backing and nested fields, with counts and backing capacities separate. Preserve ordered action boundaries, initial state, full before/after deltas, diagnostics, return/exit/abort status and defined prefixes. Byte encodings may be sparse/deduplicated only if lossless and validated against retained raw execution. No scientific values, field membership or ordered state transitions may be normalized away.

Use small, explicit runtime MAX_PSR/MAX_OBSN values where they control caller allocations, while preserving compile-time capacities such as MAX_FIT=10000. Record the limits and actual allocation dimensions. Do not shrink source arrays or rewrite the covariance allocation for convenience. Resource exhaustion is not a reproducible ordinary golden; unsafe size arithmetic/access and allocator failure paths lacking safe observation remain classified and unrun unless a separately declared defined experiment establishes them.

## Required histories

1. **Natural initialization.** Cover every active assignment and relevant loop in initialiseOne, including all nonparameter fields, strings and suffix bytes, jump/FD arrays, constraint pointers, noise defaults, source enum/control defaults and the 2000-slot parameter allocation. Preserve the double IPERHARM labels already corrected in 0033. Confirm that observation data remain unwritten, prefit fields remain unwritten, and omitted controls such as ne_sw_ifuncN/nWhite_dm are not invented defaults. FullSetup is unused by this source; exercise distinct signed arguments without adding its intended conditional behavior.

2. **Wrapper and repeated calls.** Exercise initialise over zero/one/multiple declared pulsars and noWarnings/debug/CVS controls where defined. For repeated initialiseOne or allocateMemory calls, retain explicit handles to the prior allocations before they are overwritten. Observe source resets, retained nonparameter fields, string suffixes, replaced pointers and still-live prior allocations through valid caller aliases. The source does not free all old allocations before replacement. Record that behavior; harness cleanup after the observation is not a source repair. The realloc argument is unused. Do not duplicate the native leak indefinitely or imply that a future Rust ownership decision has already been made.

3. **Covariance allocation.** malloc_blas uses calloc for its element block, stores size_t dimensions before the exposed double data, allocates row pointers and establishes contiguous row strides. Matrix cells are initialized zeros, unlike observation malloc storage. Cover rectangular/square shapes, rows/columns through the actual accessors, first/last row/element positions, source wrappers and valid matching destruction. Record the real MAX_FIT-by-MAX_FIT initializer covariance extent without dumping it redundantly in every case. State exactly which full ranges or bounded samples were observed. Keep zero-dimension assert termination distinct from undefined accesses and allocation exhaustion. Host-sized metadata, type-punning/layout and debug logger limitations must retain explicit classifications.

4. **Destruction.** Inventory the exact order and ownership assumptions in destroyMemory/destroyOne: observation pointer check, covariance free, all MAX_PARAMS constraint-special entries independently of nconstraints, then parameter buffers/labels. Exercise valid matching lifetimes, null observation where the rest of the owner is valid, and separately allocated constraint strings at early/late slots. Do not call double-free, free an interior/foreign pointer, dereference released storage or treat a successful return as proof of every deallocation. Record what is source-inspected versus actually measured. Any optional allocation instrumentation must be a separately identified profile, preserve real allocation semantics, and be compared with the ordinary execution; it is not permission to patch core functions.

5. **copyPSR complete retention.** Cover every copied and retained pulsar field, the complete observation records and unused backing. Preserve strings through NUL with suffix retention; source name/binary model/fitMode/nobs/nits first; forward observation assignment; position/velocity; ecliptic state; inclusive ordinary jump loop after copying nJumps; the FD loop using the destination's existing nfdJumps; and the remaining scalar/string assignments in source order. nfdJumps, FD indices, jump SAT/scaled controls, parameters, covariance, fitinfo and other omitted fields remain retained. Seed omitted fields distinctly so accidental zeroing/copying would be detected. Include zero/negative signed counts and last valid inclusive entries where the source remains defined. Do not generalize this partial copy into a full pulsar clone.

6. **Caller alias and capacity history.** Use two distinct pulsar objects for defined string-copy cases. Where their observation buffers share or overlap complete initialized records, exercise source forward-assignment history with explicit shared-owner/offset metadata and valid backing. Source observations may then change through aliases; do not assert source immutability universally. Manage that shared allocation once in the harness; do not pass interior/shared pointers to incompatible destruction paths. Same-pulsar strcpy, overlapping string buffers, missing terminators, unwritten reads and over-capacity loops are unrun. Record these cases for a later central Rust ownership/API decision rather than silently excluding defined alias behavior or inventing safe-adapter semantics now.

7. **Embedded and opaque state.** Account for every FitInfo/FitOutput/storePrecision/TZR/constraint field even when initialization and copyPSR leave it untouched. Valid explicit caller values can establish retention. Function pointers and opaque constraint payloads may be declared fixture inputs with identity tags; do not invoke them as substitutes for actual core routines, infer null defaults for unwritten fields, or claim fitter/plugin ABI coverage. No physical timing model is evaluated merely by copying its fields.

## Verification and delivery

Normal verification must freshly build and identify the executable and current dependencies before and after use, challenge launches, recreate case outputs and remove stale success receipts. Retain raw streams and complete state separately from narrow declared diagnostic path/address presentation normalization. Match exact values/classes/bytes, case/action/field membership, order, ownership relationships and status; no floating tolerance is authorized.

Include meaningful comparator/identity controls and disposable compiled mutations with observable witnesses: initializer value/reset or omitted-field write, string suffix handling, allocation dimensions/row stride, observation copy order, inclusive jump bounds, destination versus source FD count, and accidentally copying an omitted field. Mutations must remain defined for their witnesses. Do not execute an uninitialized-memory mutation merely to test whether it happens to differ. Measure function/line/branch coverage separately, compare its observations with the ordinary build and classify every unhit selected path. Source-inspected, host/ABI, inactive, undefined/unrun and untested defined behavior remain distinct.

Deliver `validation/pulsar_lifecycle/REPORT.md`, complete field/caller/lifetime maps, declared fixtures/classifications, source identities, raw and lossless comparison evidence, fresh replay/control/mutation commands and results, measured coverage, explicit limitations and Conventional Commits. A future Rust state/lifetime/API contract requires central review of this evidence. S may not change Rust, existing evidence, numerical policies, source defects or milestone status.

## Dispatch

Created **Assignment S: TEMPO2 pulsar lifecycle reference**, chat `01a16912-7fd8-7392-9c1e-79436775676e`, in the local `rustify-tempo2` project from release commit `3ec1e18`. An immediate snapshot confirms an active implementation turn. P, Q and R received the separate ownership boundary. R remains the sole Rust owner; S's dispatch does not imply completed or accepted evidence.
