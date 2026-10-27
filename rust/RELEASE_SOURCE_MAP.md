# AL current correspondence and boundary map

Baseline `6acaf53d2e13291eb6d0b0988faf7dee7d33a635`, native pin
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`. This map separates implementation
presence, executed evidence and acceptance. Historical missing labels in the
older coverage inventory are not current implementation findings.

[The 93-row responsibility table](../validation/release/source_correspondence.tsv)
covers the union of the pinned core and standalone predictor libraries: 92 core
translation units plus `t2toolkit_global.C`. It names the current public route and
retains the detailed predecessor body/function maps. The executable source audit
checks that union, binds every current Rust body, indexes functions/exports and
unsafe/departure locations, and verifies all 36 retained SOFA source files against
the pinned blobs. These lexical indexes aid review; they are not an exhaustive
branch proof. AL's audited 155 Rust body files are byte-identical to the released
baseline. No new scientific implementation is substituted for a predecessor map.

| Responsibility | Current body/capability authority | AL execution |
| --- | --- | --- |
| PAR/TIM, lifecycle, clocks, ephemerides, terrestrial geometry, propagation | LIBRARY_INTERFACES_SOURCE_MAP; prior input/BAT, resource and timing reports | Outside installed consumers, default tests, 130 timing comparisons, P replay; early corpus histories otherwise retained |
| Fourteen binary models, phase/residuals, reference phase and epoch/unit handling | TIMING_MODELS_REPORT and source modules | All fourteen plus lifecycle cases in 130 comparisons |
| General/legacy/local/global/robust/noise fitting, callbacks, constraints, solvers | GENERAL_FITTING_SOURCE_MAP/API | 72 public/family cases, M2, DDH, provider solver matrix; failures retained |
| Statistics, RNG/bootstrap, GW generation/files, spectra, interpolation, sigma-z | ANALYSIS_TOOLS_SOURCE_MAP/API | Installed direct clients and retained AG evidence; fresh Lomb contraction and 0049 boundary witnesses |
| Timing-dependent predictors and standalone predictor leaves | PREDICTOR_SOURCE_MAP; LIBRARY_INTERFACES_SOURCE_MAP/API | AF generation/exchange, C11/C++17 static/shared all64 symbols, exact native-width comparator controls |
| Native entry point, formatted output, logging and help | CORE_OUTPUT_SOURCE_MAP/API | Full unchanged AH comparison, installed CLI; same twelve failing groups |
| Optional CALCEPH/BLAS/LAPACK and QR default | NATIVE_BACKENDS_API; native/backends.c | All six macOS profiles; actual Linux execution remains pending |
| Resource ownership and observation/concurrent workflows | PARALLEL_EXECUTION_CONTRACT/API; current source | Actual worker/diagnostic unit tests, installed concurrent Rust and serialized C callers; bounded fresh serial performance |

`tai2tt.C` and `utc2tai.C` remain entirely disabled source bodies at the pin;
active clock chains supply their responsibility. `storePrecision.C` remains the
source no-op. The full standalone `getPeriod.C` route is installed, and AL now
also installs `tempo2.C`'s Rust core CLI. Generated enums/help retain their pinned
source inputs. `convertxt.c` is a build tool; `age.C` is unbuilt. Plugins remain
outside the selected ABI contract. SOFA's retained 36 routines and notices are
kept; the other 85 routines are not newly promised public APIs.

## Unsafe, retained native and departure boundaries

All 63 lexical `unsafe` occurrences lie in these eight files, including foreign
blocks and exported-symbol attributes. They are not 63 independently unsafe
algorithms. Existing local invariants and owner contracts remain unchanged.

| Owner | Necessity and invariant | Evidence / limitation |
| --- | --- | --- |
| `calceph.rs` | Unique nonnull opaque descriptor; six-element outputs; Drop once; synchronous calls | Actual descriptor recovery/provider comparisons; no inferred Send/Sync |
| `clock_diagnostics.rs` | POSIX CPU clock, no pointers; host clock_t convention | Output tests; CPU values normalized only by existing output contract |
| `clock_text.rs` | Typed libc scanning/formatting, owned NUL strings and bounded buffers | Parsing/format/output controls; predictor locale conversion is separate |
| `get_obs_coord.rs` | SOFA by-reference scalars and disjoint 3/6/9-element arrays; recursive Fortran locals | Identical 36 sources; PIC addition; geometry and worker tests |
| `predictor_ffi.rs` | C owns layout/allocation; synchronous callbacks and advertised extents; unwind boundary | All64-symbol clients, storage/low-count/pipe/locale controls; caller exclusivity |
| `sort_toas.rs` | Source libc qsort order; whole unpinned owned records; no escaping callback borrows | Existing order semantics retained; host tie order is not generalized |
| `stdio.rs` | Unique owned FILE and source read-ahead, exclusive buffers, Drop closes once | Installed reader/resource clients and recovery tests |
| `t2accel.rs` | Validated contiguous matrices/workspace; distinct borrowed buffers; LP64 ABI | Actual macOS helpers and all profile cases; Linux CBLAS/LAPACK execution pending |

The native predictor shim and public headers retain the detailed AJ departure
ledger (union ownership, bounded reads, locale, partial state, low counts, pipe
cursor, self-copy/insert and declared unsupported operations). The actual C
caller/shim sanitizer scope excludes Rust/MPFR/Fortran/providers. AK's process
admission, ordered effects and native observation-worker correction remain the
recorded contract; no blanket concurrent API safety is inferred.

AL production changes are limited to explicit Linux OpenBLAS selection, a
header-checked CBLAS/LAPACK boundary, PIC Fortran linkage, and core CLI installation.
No Rust timing/fitting/toolkit body or numerical gate changes. Native construction
patches reproduce already reviewed corrections with byte provenance; their
observer hooks are not new scientific algorithms. The separate Linux declaration
patch supplies the real provider ABI without changing matrix expressions.

Full per-function branch execution, early-corpus Linux reconstruction and final
scientific acceptance are still open obligations. Presence and unchanged body
identity do not settle the inherited numerical ledger. See the qualification
report for actual pass/failure/unrun status rather than interpreting this map as
full migration acceptance.
