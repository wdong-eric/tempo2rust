# Public library interfaces and predictor C compatibility: prepared Assignment AJ

Central decision, 26 October 2026. **The user selected full Rust core/toolkit APIs plus compatibility with TEMPO2's standalone predictor C API.** This resolves the interface question in 0082. The scientific function/model scope is unchanged. The guarantee includes public Rust access to the full core/toolkit responsibilities and compatible external predictor callers; it does not include unchanged general core/toolkit C/C++ callers or legacy pulsar-structure ABI compatibility. Those callers must use the Rust interfaces or a separately requested future bridge. Plugin/GUI binary compatibility remains outside the existing scope.

This decision resolves the broad C-interface placeholders in 0076/0078/0080/0081/0082/0083. It is not permission to omit scientific functions, keep required Rust entry points inaccessible, or count private tests as evidence that external library callers can use them. The original source inventory and full source/body correspondence remain binding.

**Prepare AJ after AI; AJ is not released.** AH currently owns Rust under 0091; AF and AG have delivered and closed ownership, and AI remains prepared. The integration notes below were reconciled against the delivered predictor APIs on 26 October and still require the actual AI handover baseline. AJ requires a reviewed predecessor baseline, ownership closure and explicit central release under 0074/0082. Once released it owns `rust/` and new `validation/library_interfaces/` as sole implementation writer. Central retains shared design, numerical policy and acceptance. Policies 0053/0039/0050/0065 continue to apply; preserve original 0008.

## Usable deliverables

1. An installed/buildable Rust library surface for the full transferred core/toolkit APIs, exercised by external Cargo clients using canonical public owners and types. Include real input-to-timing/fitting, predictors, standalone numerical/analysis routines and source-defined direct APIs. Reuse the delivered engine and expose existing capabilities through coherent source-mapped interfaces; no second state model or scientific implementation is authorized.
2. A standalone predictor C-compatible library and headers, callable from separately compiled C and C++ clients. Preserve the source-defined predictor data/entry-point contracts on each actually qualified native ABI profile. Reuse AF's predictor engine and corrections.
3. The installed predictor-client workflow corresponding to `getPeriod.C`, with a distinct Rust development/install name such as `getPeriod-rust`, plus reproducible private-prefix build/link/run examples. Native-style generation remains AH's route through AF.

Build/install locally into an isolated prefix during qualification. Do not replace the user's canonical TEMPO2 installation. Freeze actual library target names, header paths, minimal build features and consumer recipes against the released predecessor. Static/shared artifacts must have explicit supported-linkage evidence; a Rust unit test or symbol listing alone does not establish external-call compatibility.

## Public Rust contract and source accounting

Use the full pinned source inventory and installed headers as the correspondence checklist. Each source public responsibility needs a named accessible Rust equivalent or an already approved source/no-op/unsupported-boundary disposition. Include core lifecycle/input/time/residual/fitting/output APIs, all toolkit/numerical/GW/spectrum families, predictor generation/evaluation and optional backend controls. Source-private helpers can remain internal when the complete public behavior reaches them; public routines must not disappear merely because current tests live inside the crate.

Trust canonical validated owners internally. Expose safe construction, resource/context ownership and meaningful direct operations using the established patterns. Do not publish raw pointers or duplicate validation throughout the engine solely to mimic the legacy structures. Callback-based Rust APIs must preserve source arguments, order, context and output semantics without requiring a C ABI for general core/toolkit callbacks. Frozen scientific quantities, errors and already documented departures remain visible to external callers.

The consumer matrix must include multiple pulsars/contexts, repeated calls and failure followed by valid processing; required local/global fitting and noise controls; source-direct numerical operations; model generation/evaluation and owned state lifetimes. Cover the public interface map completely through executable examples or focused direct callers. Broad scientific tests stay with their owning stage; reuse their evidence and run affected regressions when exposing or reorganizing code.

## Delivered predictor integration foundation

AF production `7420622`, validation `543760f` and report `7a94491` are reviewed under 0089; AG's handover is 0091. Read `rust/PREDICTOR_API.md` and `rust/PREDICTOR_SOURCE_MAP.md` alongside the native headers. The following integration points are grounded in central revision `b1a2067`. AH and AI may extend shared formatting, diagnostics or build configuration; reconcile their committed interfaces before AJ implementation rather than freezing an active worktree.

| Delivered Rust responsibility | Predictor C adapter obligation |
| --- | --- |
| `predictor_number::PredictorNumber` | Reuse the ordinary-long-double arithmetic owner and its subnormal/overflow rounding. It wraps Rug storage and is not a native C layout. Native values wider than binary64 must cross the measured ABI without narrowing through f64. |
| `Cheby2D` construction, evaluation, x derivative and test callbacks | Adapt source scalar arrays, coefficient order, callback arguments and caller `info` to the existing kernel. Keep synchronous borrow lifetimes and source writes explicit; do not retain caller scratch pointers beyond the source call. |
| `ChebyModel::{phase,frequency}` and `ChebyModelSet::nearest_index` | Use the appropriate leaf operation for each separate C entry. The owned generic `T2Predictor::evaluate` computes both results and metadata; it is not automatically the contract for a single legacy call. Preserve which backing each call actually reads and its diagnostics/global-state effects. |
| `T1Polyco::{phase,frequency,read,read_new,write}` and `T1PolycoSet` | Reuse the canonical formulas, formats and nearest-midpoint behavior. Phase and frequency remain distinct operations; phase has source stdout diagnostics while frequency does not. The ABI adapter must account for that observable difference. |
| `T2Predictor::{copy_from,insert,keep,metadata}` | Preserve generic dispatch and source-unimplemented T1 operations. Rust `Clone` is an owned convenience, not permission to make the legacy T1 Copy/Insert/Keep entry points newly supported. Name/nearest pointers refer to the exposed C backing with their source validity period, not temporary cloned Rust metadata. |
| `T2Predictor::{read_stream,read_into,write_stream}`, `get_plan`, `get_plan_ext` | Adapt caller-owned FILE streams and source cursor/error behavior to canonical parsing/serialization. Existing Read+Seek and whole-buffer provenance behavior does not establish a FILE adapter by itself. Retain actual consumed positions and partial/publication semantics, without closing the borrowed stream. Reuse the existing plan engine and exact interval/count rules. |

The exposed native T1 structure has a fixed `coeff[32]` array and a separate declared `ncoeff`; the current owned file model derives its active count from a validated 2–32-element vector. Freeze C preconditions against the actual source reads and initialized backing, rather than assuming a file-constructor check proves the complete direct-C contract. Source phase/frequency read coeff[1] unconditionally. Likewise, native fixed char arrays have byte capacities and terminators; current String-based Rust metadata is not authorization for lossy decoding, changed byte widths or temporary name-pointer lifetimes. These are adapter representation questions to resolve with concrete caller witnesses, not new scientific formulas.

Source `ChebyModelSet_GetNearestIndex` resets `ChebyModelSet_OutOfRange` to zero even when it returns -1. `ChebyModelSet_GetNearest` then sets it to one on a miss; set phase/frequency calls go through that pointer-returning routine. Preserve this distinction at each exported operation, including success after a miss and source first-segment ties. Mapping every Rust `None` to an identical global update would change the lower-level C interface. A T1 call does not acquire Cheby's out-of-range meaning merely because Rust's combined result contains an `out_of_range` field.

Account for the standalone source datum `tempo2_verbose` as well as `ChebyModelSet_OutOfRange`; the pin defines it as unsigned with initial value one, although the inspected standalone bodies do not use it as a guard for T1's three phase diagnostics. Preserve the actual source print controls rather than inferring them from the variable's name. Existing Rust status/diagnostic adapters and the exported legacy C data remain distinct responsibilities. AK/AL qualify concurrency; an owned Rust internal result does not make unsynchronized caller access to an exported mutable C datum thread-safe.

## Predictor C contract

The actual standalone source target is `tempo2pred.c`, `cheby2d.c` and `t1polyco.c` in pinned `Makefile.am`. `tempo2pred.h` defines the public predictor types and twenty generic entry points; installed `tempo2pred_int.h` also exposes lower-level declarations, including some core-side constructors. Account for the defined standalone C-linkage routines/data and each header/definition mismatch explicitly. `cheby2d_int.C`'s timing-core constructors remain available through the full Rust core API; their presence in an internal header does not make them standalone predictor-library definitions.

Preserve `T2PredictorKind`, the exposed structs/unions, scalar/array types, layout and native calling convention for the qualified profile. These structs contain ordinary C `long double`, not the timing typedef or an MPFR object. The current arm64 probe measures a 53-bit native significand and size/alignment 8/8; this is not a portable assertion about Linux or other architectures. Measure sizeof/alignment/offsets, enum representation, argument/return conventions and numeric conversions on each additional qualified target.

Use a small C binding shim where the native long-double ABI cannot be expressed directly and safely in Rust. It may marshal ownership and representation; scientific algorithms remain in the canonical Rust implementation. Never reinterpret a Rug Float as native long double, narrow a wider native profile through binary64, or hide an opaque Rust handle inside fields while claiming compatibility with the original exposed layout. Preserve permitted caller writes to the exposed C data and the source lifetime/aliasing contracts. Record conversion costs for later performance review.

Implement complete generic kind dispatch, initialization/copy/insertion/keep/destruction, file/stream operations, names/ranges/kind, phase/frequency and both plan APIs, plus the defined standalone lower-level C routines/data. Preserve file grammar, selection/overlap/tie behavior, plan interval rules, source native-type boundaries and AF's original/corrected distinctions. T1 operations that the source leaves unimplemented require their declared disposition; compatibility is not a reason to invent missing scientific behavior.

Borrowed name pointers and nearest-segment pointers must retain the source validity periods. FILE* APIs consume caller-owned streams according to the source and do not silently close them or replace their position with an unrelated file read. Match allocations with the corresponding destruction/free contract, include deep-copy and legitimate repeated/alias histories, and prevent unwinding across the foreign boundary. Document actual caller preconditions instead of pretending arbitrary invalid C pointers can be validated safely.

Preserve `ChebyModelSet_OutOfRange` as an observable legacy C-interface datum with its source update semantics. An owned Rust status alone does not provide that exported datum. Qualify the supported concurrency behavior explicitly; a global legacy interface is not made thread-safe by claiming the Rust internals are owned. Keep any necessary unsafe block/function/implementation small with adjacent necessity, layout, extent, initialization, aliasing, lifetime, threading and cleanup explanations.

The internal header's T1 nearest-function declaration omits an argument present in the definition, and const/declaration coverage also needs reconciliation. Use focused compile/link evidence and 0053 for narrow corrected declarations, preserving the original header witness. Do not manufacture a callable function from a declaration with no implementation. Existing AF fixes for initialization, reader bounds and the binary128/native-long-double variadic mismatch carry forward; no invalid native bytes become expected scientific output. Freeze how each fallible Rust outcome maps to the legacy C status/diagnostic/state contract, especially void-returning APIs, before validating it.

## getPeriod client

The pinned client reads a predictor filename from argv[1], parses argv[2] through atof into binary64 MJD, queries frequency at 1400 MHz, narrows the returned frequency to binary64 and prints MJD, frequency and reciprocal period using `%lf %24.20lf %24.20lf\n`. Preserve those defined numeric and formatting choices, including the source parse convention and extra-argument behavior. It does not perform a fresh timing fit.

The source fails to initialize the predictor, checks neither argument count nor read status, and omits destruction. Apply 0053 to the demonstrated initialization/ownership and failure boundaries: include missing/malformed/read-failure neighbors and a valid call, with adjacent source/cause/fix comments. Reuse the source-compatible formatter/scanner and predictor APIs rather than route this client through subprocess calls to legacy TEMPO2.

## Verification and handover

First freeze the source/API/symbol/type/ownership map, public signatures, error/diagnostic mapping, runnable consumers and exact qualified build profile. Use existing AF/AH examples and evidence. No independent reference-only stage precedes implementation.

Compile C11 and C++ consumers against the installed headers and link them to the Rust-backed predictor artifact. Exercise both T1 and Chebyshev files, all generic entries, relevant standalone lower-level APIs/callbacks, direct caller-owned structs, field/layout round trips, names/segment lifetimes, stream positions, out-of-range state, copy/insert/keep/read histories, plan outputs and failure recovery. Run source-native and Rust-backed consumers in separate processes; show which artifact was actually loaded. Validate direct result values and state as well as source file exchange. A name-compatible symbol with the wrong ABI must not pass based on successful linking alone.

Use the existing AF quantity-specific numerical/formatting contracts and exact identities, counts, ordering, enum/field layouts, classes and signed-zero rules. Do not apply one tolerance to every foreign value or treat display rounding as permission to widen engine gates. Predictor read/evaluate consumers must operate without TEMPO2 runtime tables or a legacy TEMPO2 executable/library. Record actual build/link/runtime dependencies; qualify the isolated prefix rather than relying on checkout-relative include/library/resource paths.

Run external Rust clients and affected debug/release tests, static/shared consumer profiles actually promised, memory/ownership error controls appropriate to the new boundary, formatting and Clippy. Additional platforms require actual execution; an arm64 layout probe cannot accept another ABI. Keep original inputs/results immutable and generated JSON, binary products and install prefixes ignored. Only authored bindings/headers/recipes/fixtures and concise reports are committed.

Return authored Conventional Commits and `rust/LIBRARY_INTERFACES_REPORT.md` with the complete interface/source/unsafe/departure map, runnable installed consumers, exact source/build/loaded-artifact identities, checked guarantees and remaining limitations. Receive the 0082 checkpoint before any successor release. The complete inherited ledger in 0091 remains open, including AE fitting/noise/timing and AF's seven raw polyco log10RMS failures. Preserve 0087's adopted paired DDH clock rule and the bounded historical ideal pass separately from unresolved fidelity comparisons; interface completion does not accept the full migration. OpenMP/parallel execution, broader platform/installation qualification, concurrency/performance and integrated scientific release acceptance remain on the full plan.

## Preparation evidence

Central inspected the pinned predictor types/declarations, definitions, source-library membership, installed client and core/toolkit interface boundaries. The existing C/C++ header probe and symbol observation are preserved under 0082 in `decisions/evidence/predictor-interface/build/preparation-20261025/`; receipt SHA-256 `3e344c772e0c8be89a2497654d7de58bcfcd13225a208012ae49b980b2ae193b`. Its 57 observed predictor-related entries are navigation evidence, not proof of an implemented Rust ABI. That initial header/symbol preparation executed no external Rust/C predictor consumer or new scientific computation; the separate 26 October bounded state witness is recorded below.

Raw pinned source identities used during preparation:

| Source | SHA-256 |
| --- | --- |
| `tempo2pred.h` | `fb9b0a192e14bbb8ee9c4525807406b01b7e45846ea2b5ad1bc774e2693d0a76` |
| `tempo2pred_int.h` | `55b49ac77241bb7b4d0a8bc55b5a5ca955dbe21c061ab0434997032abfceb1ad` |
| `tempo2pred.c` | `bdce274251187b0348a9ef2232ef7872a107a57323a9c6f86c5fc65cfc122e00` |
| `cheby2d.c` | `173947867f2e4b168e03d43186793b6b5118380daff61fcc9e0facc022d4c488` |
| `t1polyco.c` | `e3be7ac166cfd25c0327b2d40c70f42adc636ca43f0d1abd56282da397b05c04` |
| `cheby2d_int.C` | `4893b478174ec5d3845be7793721eb649c558d1b76adbb2b992d3c028dbfef00` |
| `getPeriod.C` | `3f1c19bb982b567dae1ce3fadfded54c2c80ee327887d217ea7406c49f4bee2d` |
| `Makefile.am` | `172ec480482f0261f0a845e9aface382c1a441f4e80d53132574bf8ff912bd7a` |
| `tempo2.h` | `0313b6eaf9f9ae9d7e5cce340379ad4b7068394e5222a01f73949e3a2640de93` |
| `TKlongdouble.float128.h` | `214911b81f82489996d698626cf8d30ab5bcc20099f6b5c5aa089715b85bc55e` |
| `TKlongdouble.ld.h` | `b9acbfa5a68e3427f83ed3a8ec9551d9fb202d8b73502be219205ad1269a713d` |
| `sofa/Makefile.am` | `2d0f7ba86115bcb0bd53ba5b6a8ad3d2dca2e716be94097c1937efb353e259d4` |

## 26 October source-state preparation

The authored `decisions/evidence/predictor-interface/central-state-preparation.py` extracts four exact pinned source bodies into a disposable C11 probe with the pinned public header: Cheby nearest-index/nearest-pointer and T1 phase/frequency. Fully initialized caller storage demonstrates index miss `(-1, flag 0)`, pointer miss `(NULL, flag 1)`, first-segment tie followed by flag zero and a successful later segment. The T1 phase call emits the source's three diagnostic lines; the frequency call emits none. This is bounded native state/diagnostic evidence, not a complete standalone library, Rust C ABI, Linux or general numerical qualification. No scientific function was reimplemented in the probe.

Ignored bundle: `decisions/evidence/predictor-interface/build/state-preparation-20261026/`; native receipt SHA-256 `af36a9af9761e8a3ca63cb52d0911c2c56f1aa0062dbf068b904ffa71ee4873e`; preservation receipt `28435f2ee532954d6cbb7bcd74a890c9e08ff9d9b05cb218c141af8c64eac78d`; authored reproducer `a0cfae4f0c8b5132f65936ee48a8f0b189005c1e475669b194b10d0acce4ec33`. The pinned eight-file Rust/API integration identity is `6139a53315a678e534eb302bf931f189a25be66063c050ef1517be89870b9537`. Raw snapshots, extracted bodies, command, compiler identity, executable and output remain local. No production Rust, canonical TEMPO2 or accepted reference artifact was changed. AJ remains unreleased.
