# AL frozen-source correspondence and boundary map

## Current integrated source accounting — 2 November 2026

This refresh binds Rust source at `f511cede0851cc375ceed263ad97e40cae25a0a0`,
including the exact reviewed AN history through `497168b` and AO history through
`f077292` (central merge `a44bda2`). M2 is accepted
under 0122; AN's allocation improvement is accepted for measured macOS behavior
under 0123. Actual Linux allocation qualification, AL and full migration remain
open. AO's completed scalar transport is included under 0124; its next evaluator
stage remains isolated and is not part of this snapshot.

Against the unchanged `6acaf53d2e13291eb6d0b0988faf7dee7d33a635` baseline, the 158
Rust source files comprise 101 unchanged bodies, 54 modified existing files and
three additions, with no removals. The audit indexes 73 lines containing `unsafe`
in ten files, 1,735 function/export lines and 119 source-departure-pattern lines.
These are lexical navigation counts, not exhaustive safety or scientific checks.
The AN-only refresh had 69 unsafe lexical lines: four ordinary-long-double
transport lines and two allocator operations beyond the dated 63-line snapshot
below. AO changes predictor transport and its test oracle, bringing the lexical
total to 73; that net count is not a count of new production pointer operations.

| Current boundary | Necessity and invariant | Reviewed evidence / remaining limit |
| --- | --- | --- |
| `tk_matrix.rs:59`, `alloc_zeroed` | Checked nonzero f64 array layout; global allocation; null maps to the established recoverable allocation error. Native `TKmatrix.C::malloc_blas` uses calloc. | 0123 source review and macOS allocation/failure controls; actual Linux residency and allocation controls pending. |
| `tk_matrix.rs:67`, `Vec::from_raw_parts` | All bytes are initialized positive-zero f64; aligned unique storage transfers once with matching length/capacity and global deallocator. Existing Arc ownership/COW remains. | 0123 full positive-zero/layout, lifecycle, complete science and COW evidence; no deferred-cell semantics or custom allocator. |
| `ordinary_long_double.rs:13,88,97,101` | Existing synchronous native math transport uses owned NUL-terminated text and a bounded output buffer, with source ordinary precision retained. | 0117/0119 ordinary-exp/log qualification; separate from allocator changes and AO predictor profiling. |
| `predictor_ffi.rs:43,80` | Synchronous native scalar reads/writes use exclusive local significand/exponent/class values; current public cells are reread, no pointers escape and ordinary 53/64-bit precision remains. | 0124 review and macOS scalar/native/consumer evidence; Linux transport and matched 8x4 performance follow-through pending. Test-only text transport remains an independent oracle. |

The fresh identity/navigation audit is retained at
`validation/release/build/source-map-ao-0124/receipt.json`. It checks all
93 responsibility rows and 36 retained SOFA sources. The allocation-only Linux
pair remains `dc435f5` / `68973d6`; the predictor pair is `68973d6` / `0bd8e2c`.
The ordinary integrated matrix separately
qualifies the merged candidate. See [qualification status](RELEASE_QUALIFICATION_REPORT.md)
and the measured [AN](ALLOCATION_PROFILE_REPORT.md) and
[AO](PREDICTOR_PERFORMANCE_REPORT.md) reports.

## Dated frozen-source snapshot — 1 November 2026

The following counts and pending statements describe their named historical
candidate; they are retained without reinterpreting old evidence.

Documentation refresh, 1 November 2026, under
[0114](../decisions/0114-independent-work-during-qualification.md). Current
source means frozen `fecbc18af95196a16db2046306099b6345a50976`, the candidate
published and dispatched by central under
[0112](../decisions/0112-fourth-linux-candidate-review-and-execution.md).
Its 158 Rust source files match the clean implementation export `91309fd`.
The comparison baseline is `6acaf53d2e13291eb6d0b0988faf7dee7d33a635`
(27 October); the native pin remains `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
Source presence, correspondence, lexical indexes, executed branches and scientific
acceptance are separate claims. Historical missing labels in older inventories
are not findings about this candidate.

Central owns review of Linux run `41294531402`, job `124812485584`, attempt 1
on that exact candidate. This documentation check neither executes the engine
nor interprets an unfinished run. Actual promoted Linux qualification, AL, M2
and full migration acceptance remain pending. The pending 0113 comparator and
covariance investigation are not released by this refresh.

[The 93-row responsibility table](../validation/release/source_correspondence.tsv)
covers the union of the pinned core and standalone predictor libraries: 92 core
translation units plus `t2toolkit_global.C`. It names the current public route and
retains the detailed predecessor body/function maps. The existing
[source audit](../validation/release/source_audit.py) checks that union, binds
the Rust source bytes, indexes functions/exports and unsafe/comment locations,
and compares all 36 retained SOFA source files with the pinned blobs. Its fresh
read-only execution passes; this is identity and navigation evidence.

| Source snapshot / date | Identical to baseline | Changed entries, including additions | Added / removed |
| --- | --- | --- | --- |
| Historical `5c8c0aa`, 31 October; second follow-through | 148 | 9 | 2 / 0 |
| Historical presentation production `801bc94`, 31 October; retained presentation audit | 117 | 40 | 2 / 0 |
| Frozen `fecbc18`, checked 1 November | 107 | 51 | 3 / 0 |

The historical nine comprise seven modified files (`cholesky_spectrum`,
`fitting_tests`, `lib`, `predictor_number`, `predictor_tests`, `residual_source`,
`t2fit_glitch`) and two additions (`ordinary_long_double`, `event_profile_tests`).
The frozen candidate has 48 modified files and adds `spin_profile_tests` as the
third addition. These are byte-level file counts, including tests, diagnostic
attribution and comments; they are not counts of changed scientific algorithms.
The original [second follow-through](RELEASE_SECOND_LINUX_FOLLOWUP.md) and
[presentation report](RELEASE_PRESENTATION_REPORT.md) retain their own evidence
and dates. The [third follow-through](RELEASE_THIRD_LINUX_FOLLOWUP.md) records
the current correction bundle and remaining failures.

The execution column below dates retained campaigns; none is a fresh aggregate
qualification of `fecbc18`. Earlier source correspondence remains useful where
the body identity is unchanged; a later source change needs its own evidence.

| Responsibility | Current body/capability authority | Dated execution evidence / limit |
| --- | --- | --- |
| PAR/TIM, lifecycle, clocks, ephemerides, terrestrial geometry, propagation | [Library source map](LIBRARY_INTERFACES_SOURCE_MAP.md); prior input/BAT, resource and timing reports | 30–31 October predecessor corpora retain separate ordinary/observer/gcov identities. 1 November focused macOS checks include corrected 30 lifecycle and 133 observer histories, full clock/time/planetary/propagation comparisons; original failures and three strict terrestrial stdout failures remain. |
| Fourteen binary models, phase/residuals, reference phase and epoch/unit handling | [Timing report](TIMING_MODELS_REPORT.md) and source modules | Initial macOS campaign: 130 comparisons pass. The 30 October first Linux campaign retains an event failure; 0109's third Linux event residual still exceeds its gate. The 1 November macOS event checks do not establish Linux correction. |
| General/legacy/local/global/robust/noise fitting, callbacks, constraints, solvers | [Fitting source map](GENERAL_FITTING_SOURCE_MAP.md) and [API](GENERAL_FITTING_API.md) | 30–31 October campaigns cover 72 public/family cases, M2, DDH and provider solvers. Third Linux fit/discrete/DDH, SVD-order and spin-callback failures remain in the 0109 ledger. |
| Statistics, RNG/bootstrap, GW generation/files, spectra, interpolation, sigma-z | [Analysis source map](ANALYSIS_TOOLS_SOURCE_MAP.md) and [API](ANALYSIS_TOOLS_API.md) | Retained AG and 30–31 October installed clients; 0104's bounded Lomb contraction and 0049 witnesses retain their specific candidate/compiler identities. |
| Timing-dependent predictors and standalone predictor leaves | [Predictor source map](PREDICTOR_SOURCE_MAP.md); [library API](LIBRARY_INTERFACES_API.md) | Retained AF generation/exchange; 30–31 October C11/C++17 static/shared clients cover all 64 symbols. Revised observation controls retain raw RMS failures; 1 November installed cache checks are separately scoped below. |
| Native entry point, formatted output, logging and help | [Output source map](CORE_OUTPUT_SOURCE_MAP.md) and [API](CORE_OUTPUT_API.md) | Historical AH layout/scientific comparisons retain twelve macOS and seventeen first-Linux failing groups. 0108 accepts the 31 October presentation correction; 1 November output still has twelve failing cases, seventeen differing and three missing files. |
| Optional CALCEPH/BLAS/LAPACK and QR default | [Backend API](NATIVE_BACKENDS_API.md); [native boundary](native/backends.c) | Six paired profiles and provider ABI pass in named 30–31 October campaigns on both hosts. Those results do not qualify a changed candidate or a different provider ABI. |
| Resource ownership and observation/concurrent workflows | [Parallel contract](PARALLEL_EXECUTION_CONTRACT.md) and [API](PARALLEL_EXECUTION_API.md); frozen source | Second Linux worker campaign: 99 processes, 33 comparisons, 484,314 quantities, 108 timing samples and twelve product controls. The historical foreign-thread leak, private intervention and promoted production guard are distinct records below. |

`tai2tt.C` and `utc2tai.C` remain entirely disabled source bodies at the pin;
active clock chains supply their responsibility. `storePrecision.C` remains the
source no-op. The full standalone `getPeriod.C` route is installed, and AL now
also installs `tempo2.C`'s Rust core CLI. Generated enums/help retain their pinned
source inputs. `convertxt.c` is a build tool; `age.C` is unbuilt. Plugins remain
outside the selected ABI contract. SOFA's retained 36 routines and notices are
kept; the other 85 routines are not newly promised public APIs.

## Unsafe, retained native and departure boundaries

At frozen `fecbc18`, the source audit matches 63 lines containing the word
`unsafe` in these eight files, including foreign blocks and exported-symbol
attributes. It also indexes 1,721 function/export lines and 118 matches of its
source-pin/0053/source-bug comment patterns. These are lexical navigation counts,
not a complete departure ledger, safety proof or branch-coverage measurement.
The safe Rug cache release below adds no `unsafe` site; the unchanged lexical
count does not imply unchanged ownership behavior.

| Owner | Necessity and invariant | Evidence / limitation |
| --- | --- | --- |
| `calceph.rs` | Unique nonnull opaque descriptor; six-element outputs; Drop once; synchronous calls | Actual descriptor recovery/provider comparisons; no inferred Send/Sync |
| `clock_diagnostics.rs` | POSIX CPU clock, no pointers; host clock_t convention | Output tests; CPU values normalized only by existing output contract |
| `clock_text.rs` | Typed libc scanning/formatting, owned NUL strings and bounded buffers | Parsing/format/output controls; predictor locale conversion is separate |
| `get_obs_coord.rs` | SOFA by-reference scalars and disjoint 3/6/9-element arrays; recursive Fortran locals | Identical 36 sources; PIC addition; geometry and worker tests |
| `predictor_ffi.rs` | C owns layout/allocation; synchronous callbacks and advertised extents; unwind boundary; calling-thread Rug cache owner | Retained all-64-symbol/storage/low-count/pipe/locale controls; 1 November installed cache evidence below; caller exclusivity remains binding |
| `sort_toas.rs` | Source libc qsort order; whole unpinned owned records; no escaping callback borrows | Existing order semantics retained; host tie order is not generalized |
| `stdio.rs` | Unique owned FILE and source read-ahead, exclusive buffers, Drop closes once | Installed reader/resource clients and recovery tests |
| `t2accel.rs` | Validated contiguous matrices/workspace; distinct borrowed buffers; LP64 ABI | Actual macOS helpers and second-Linux LP64 provider execution; current candidate replay pending |

The native predictor shim and public headers retain the detailed AJ departure
ledger (union ownership, bounded reads, locale, partial state, low counts, pipe
cursor, self-copy/insert and declared unsupported operations). The actual C
caller/shim sanitizer scope excludes Rust/MPFR/Fortran/providers. AK's process
admission, ordered effects and native observation-worker correction remain the
recorded contract; no blanket concurrent API safety is inferred.

## Production changes and cache evidence

AL production changes include explicit Linux OpenBLAS selection, a header-checked
CBLAS/LAPACK boundary, PIC Fortran linkage and core CLI installation. Decision 0104
additionally authorizes measured host contraction in Lomb and ordinary arithmetic
in EXP/GAUS residuals and eight callbacks. `ordinary_long_double.rs` shares the
existing predictor rounding kernel without predictor-specific operations; the
timing role stays Rug128. Subsequent source/status changes are:

- **0107/0108, presentation:** actual Rust call-site attribution, preserved
  through wrappers/replay, replaces fictional native emitter labels. Pinned
  counterparts remain beside the callers. The existing 359-byte colored-render
  cap and all three strict terrestrial failures remain; the independent cap
  proof does not implement 0113's future paired comparator.
- **0109, production cache owner:** `fd185ec` promotes the measured guard into
  [predictor_ffi.rs](src/predictor_ffi.rs). `finish()` registers the thread-local
  `ForeignThreadCache` inside the existing panic boundary; its destructor calls
  Rug's safe `float::free_cache(FreeCache::Local)` at calling-thread exit. It
  releases local caches, not caller-owned C storage or a global cache. The named
  native counterparts are `tempo2pred.c`, `cheby2d.c` and `t1polyco.c`; this fixes
  a Rust/Rug integration lifetime, not a native scientific algorithm.
- **0110/0111, measured platform arithmetic:** [clock interpolation](src/tabulated_function.rs)
  and [LMST](src/get_obs_coord.rs) preserve the accepted macOS fused operations
  and the measured Linux source grouping. Source-site rationale and original
  mismatches remain explicit. The [contraction inventory](RELEASE_CONTRACTION_INVENTORY.md)
  is diagnostic preparation; other unverified contractions and production spin
  arithmetic remain unchanged.

The cache history must be read in order:

| Record | Source identity and evidence | What it establishes |
| --- | --- | --- |
| Historical private experiment: 31 October preparation; 1 November 0109 review | Frozen `a35104a` library plus guard-only private copy; original ASan/LSan reports 544 bytes in sixteen allocations. The actual Linux private guard and explicit-cleanup variants pass with twelve keyed workflows and twelve storage products in each configuration. | Evidence supporting promotion; it did not execute the promoted production installation. The original recipe remains in [archive/cache_lifetime.py](../validation/release/archive/cache_lifetime.py), with its [historical scope](../validation/release/archive/README.md). |
| Promoted production source, 1 November | `fd185ec` is included in clean `a8ba6f1`, implementation export `91309fd` and frozen `fecbc18`. The guard is in the shipped source, not only a diagnostic copy. | Source correspondence and ownership implementation; presence alone is not a leak or concurrency test. |
| Scoped accepted macOS evidence, 1 November | Clean `a8ba6f1` default/combined installations; plain, ASan/UBSan, TSan and explicit caller-cleanup comparisons. Central `2bbdd46` and 0112 independently verify twelve complete workflows and twelve TSV products per variant, with exact agreement. | Tested installed behavior on macOS. Sanitizer instrumentation covers C clients/shim, not all Rust/MPFR/Fortran/providers; no blanket concurrent-API safety claim. |
| Promoted Linux qualification | Central's fourth run uses frozen `fecbc18`; its completed evidence and acceptance are pending. | The private Linux intervention and macOS production results cannot substitute for actual reviewed Linux installation checks. |

Other ordinary sites remain [proposals](RELEASE_ORDINARY_TYPE_PROPOSALS.md).
Native construction patches retain byte provenance. Native timing remains
binary128 and Rust timing Rug128; no numerical gate, frequency convention,
production clock resource or global precision policy changed.

## Identity receipt and remaining obligations

The fresh audit is retained under
`validation/release/build/source-map-fecbc18-20261101-v1/audit/`; receipt SHA-256
is `49857535a7772c2d921f2a3c9fe28adcde9b12f39ae3009ee172cac895f82c7e`.
It ran the existing recipe against source bytes first checked equal to `fecbc18`.
The companion `receipt.json` binds the final two documents, the unchanged
AM README, checked links, saved predecessor receipts and all 332 inspected
source/test/native/interface/manifest/recipe/workflow files. The audit's map
hashes describe the pre-edit documents; the companion receipt binds this refresh.
No engine execution, compilation or numerical comparison was performed.

Full per-function branch execution, early-corpus Linux reconstruction and final
scientific acceptance are still open obligations. Presence and unchanged body
identity do not settle the inherited numerical ledger. See the qualification
report for actual pass/failure/unrun status rather than interpreting this map as
full migration acceptance.

The successor [requirements.tsv](../validation/release/requirements.tsv) groups
all 93 units into 26 requirement families. `evidence_closure.py` binds each family
to its canonical Rust route, authored function/branch rows with source hashes and
line numbers, actual native function/edge records and explicitly selected
numerical receipts. It retains unhit edges, static dispositions and missing fresh
measurements. Authored correspondence may be reused for unchanged bodies; it
does not become a fresh compiler/ABI or branch witness. The generated ledger
therefore reports qualification incomplete even when accounting succeeds.
