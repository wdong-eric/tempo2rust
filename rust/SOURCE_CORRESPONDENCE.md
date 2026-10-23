# Production correspondence and unsafe boundaries

Assignment E, TEMPO2 pin `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
All production Rust modules name their source responsibility near the top.
Test modules, examples and Python evidence tools are verification glue.

| Rust file | TEMPO2 counterpart / responsibility |
|---|---|
| `src/lib.rs` | Rust-only owned API/types and serialization for `tempo2.h`, readers, timing and `t2fit.C`; no standalone counterpart |
| `src/input.rs` | Rust-only file ownership/load glue for `readParfile.C`, `readTimfile.C` |
| `src/read_parfile.rs` | `readParfile.C::readParfile/readValue`; MODE/WEIGHT sequential assignment and NITS; source text preserved |
| `src/read_timfile.rs` | `readTimfile.C` FORMAT 1 subset |
| `src/longdouble.rs` | `longdouble.h` / `TKlongdouble.float128.h`; approved fixed Rug128 representation and decimal transport |
| `src/tempo2_util.rs` | `tempo2Util.C` coordinate, dot product and pulse-selection helpers |
| `src/form_residuals.rs` | `formResiduals.C::formResiduals`; source-split phase, first-input reference, integer counts |
| `src/form_bats.rs` | `global.C::formBatsAll`, `calculate_bclt.C`, `formBats.C` isolated PKS correction orchestration |
| `src/runtime.rs` | Rust-owned resource/context glue; `observatory.C::getObservatory`, clock/EOP/ephemeris resources |
| `src/clkcorr.rs` | `clkcorr.C` clock tables, sequence search and interpolation |
| `src/tabulated_function.rs` | `tabulatedfunction.C::TabulatedFunction_getValue` |
| `src/eop.rs` | `eop.C::load_EOP/get_EOP` |
| `src/ifteph.rs` | `ifteph.C::IFTE_init/IFTE_get_Vals`, IF99 interpolation |
| `src/read_ephemeris.rs` | `readEphemeris.C`; retained `jpleph.c` reader/interpolator boundary |
| `src/get_obs_coord.rs` | `get_obsCoord.C::get_obsCoord_IAU2000B`; retained SOFA calls |
| `src/tt2tdb.rs` | `tt2tdb.C::tt2tb` |
| `src/tropo.rs` | `tropo.C`, meteorology and Niell mapping functions |
| `src/std_freq.rs` | `t2fit_stdFitFuncs.C::t2FitFunc_stdFreq/t2UpdateFunc_stdFreq` |
| `src/tksvd.rs` | `TKsvd.C::TKbidiagonal/TKsingularValueDecomposition_lsq/TKbacksubstitution_svd/TKpythag`; `T2toolkit.C::TKsign_d/TKretMax_d` |
| `src/tkfit.rs` | `TKfit.C` nonaccelerated `TKleastSquares` route: covariance, back substitution, mixed-precision chi-square, error rescaling; `FitNumber` is Rust-only diagnostic transport |
| `src/t2fit.rs` | `t2fit.C` local fit data/matrix/whitening/update orchestration and `tempo2.C` NITS lifecycle; owned fit results are Rust-only glue |
| `src/main.rs` | Rust-only command/JSON publication glue for `tempo2.C` entry points |
| `build.rs` | Rust-only Cargo build glue replacing native dependency Makefile rules |

ZERO construction and storage remain in `t2fit.rs`; the frequency callback lives
in `std_freq.rs`, with `SPIN` fixing the source F0–F3 order. The SVD retains source
indices, `an`, `rv1`, `w`, `v`, `u`, `wt` and arithmetic association. Unused source
allocation of a separate `u` buffer is omitted: the source operates on the
modified design matrix as U. No algorithm, column normalization, fallback solver
or new numerical dependency is substituted.

## Retained dependencies

`native/upstream_manifest.json` names every retained native file at the same
TEMPO2-relative path, including all 36 SOFA routines and the JPL C/header files.
Those files remain byte-identical to accepted M1b, including the previously
accepted JPL patch. `native/README.md` and `native/departures.patch` identify the
native responsibilities and the existing changes. They are established retained
numerical dependencies, not hidden TEMPO2 fitting calls. Their original source
headers, routine names and licenses are preserved. SOFA packaging restrictions
remain a separate distribution limitation.

## Unsafe audit

There is no unsafe code in the new controls, derivatives, SVD, fit orchestration,
serializer or CLI. The only unsafe Rust sites remain the eight pre-existing
FFI declaration/call blocks below. Each now has an adjacent necessity explanation
and concrete invariants; no new FFI is added.

| Site | Why native entry is necessary | Invariants at the boundary |
|---|---|---|
| `get_obs_coord.rs` extern block | Retained SOFA ABI | By-reference f64 scalars and contiguous column-major 3/6/9-element arrays; no retained pointers |
| `get_obs_coord.rs` call block | SOFA rotations | Live aligned initialized arrays, exclusive outputs, disjoint position/velocity slices for cross product, no escaping pointers, `-frecursive` locals |
| `read_ephemeris.rs` extern block | Retained JPL ABI | Valid native handles, NUL-terminated path and six-element f64 output contract |
| `read_ephemeris.rs` initializer | C-owned reader allocation | Live CString, ABI-permitted null optional arrays, global init/status lock; native file matching preflight is an unenforced assumption |
| `read_ephemeris.rs` init-status query | C error slot | Same lock held; no pointers |
| `read_ephemeris.rs` AU query | C ephemeris metadata | Live non-null exclusive handle and source constant field offset 28 |
| `read_ephemeris.rs` interpolation | C numerical interpolation | Exclusive mutable handle/cache access, finite epoch, aligned initialized six-slot output, source body identifiers, no retained pointers; native bounds assume unchanged file contents |
| `read_ephemeris.rs` Drop | Matching C deallocator | Unique non-cloneable successful handle, exactly one cleanup, no outstanding call or escaped pointer |

The handle is not made `Send` or `Sync` by an unsafe implementation. This audit
explains the existing boundaries; it is not concurrency stress testing, an
exhaustive native-memory proof or approval of historical behavior differences.
Those differences remain visible in `BEHAVIOR_DIFFERENCES.md` and Assignment F's
independent baseline inventory.

The JPL preflight reads and validates file bytes, then native initialization
reopens the path. There is no enforced identity/content pin between those
operations or throughout native reads. Preflight therefore does **not** prove
the native reader consumes the validated bytes. This is an existing safety
limitation assigned to a separate central JPL stage; E changes no native reader
or cache behavior.
