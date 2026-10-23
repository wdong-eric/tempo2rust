# Production correspondence and unsafe boundaries

Assignment E, TEMPO2 pin `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
All production Rust modules name their source files and relevant routines near
the top, or explicitly identify Rust-only glue. This maps selected responsibilities,
not complete source files or full feature coverage. The header review uses F's
validated `validation/coverage/module_map.tsv` and `CORRESPONDENCE.md`, plus the
pinned declarations. Test modules, examples and Python tools are verification glue.

| Rust file | TEMPO2 counterpart / responsibility |
|---|---|
| `src/lib.rs` | Rust-only owned projections of `tempo2.h` state and `initialise.C::initialiseOne`; API/JSON/error glue and reexports of `readParfile.C::readParfile`, `readTimfile.C::readTimfile`, `global.C::formBatsAll`, `formResiduals.C::formResiduals`, `t2fit.C::t2Fit`; no standalone counterpart or C ABI equivalence |
| `src/input.rs` | Rust-only file ownership/load glue for `readParfile.C::readParfile/readValue`, `readTimfile.C::readTimfile/readTim` and selected `initialise.C::initialiseOne` defaults |
| `src/read_parfile.rs` | `readParfile.C::readParfile/readValue`; MODE/WEIGHT sequential assignment and NITS; source text preserved |
| `src/read_timfile.rs` | `readTimfile.C::readTimfile/readTim` FORMAT 1 subset; directives and per-TOA flags omitted |
| `src/longdouble.rs` | Generated `TKlongdouble.h`, selected from `TKlongdouble.float128.h`; `TKlongdouble.C::parse_longdouble/print_longdouble` numeric roles. Finite MPFR128/strict decimal transport is the approved adapter, not source binary128, native long double or an ABI object |
| `src/tempo2_util.rs` | `tempo2Util.C::dotproduct/hms_turn/dms_turn/turn_deg/fortran_mod/fortran_nlong`; strict `parse_binary64` is Rust glue |
| `src/form_residuals.rs` | `formResiduals.C::formResiduals`, `tempo2Util.C::fortran_mod/fortran_nlong`; split spin phase, first-input reference and checked counts. The `@` entry supplies zero-delay `formBats.C::formBats` and `secularMotion.C::secularMotion` branches |
| `src/form_bats.rs` | Selected `global.C::formBatsAll`, `calculate_bclt.C::calculate_bclt`, `formBats.C::formBats`; fused `shapiro_delay.C::shapiro_delay`, `dm_delays.C::dm_delays` constant-DM/spherical-wind path, `vectorPulsar.C::vectorPulsar` fixed direction, and zero `secularMotion.C::secularMotion`. Also calls the Rust `formResiduals.C::formResiduals` counterpart |
| `src/runtime.rs` | Rust-owned resource/context glue; `observatory.C::getObservatory/ITRF_to_GRS80`, `tropo.C::initialize_meteorology_tables`; orchestrates `clkcorr.C::getClockCorrections`, `eop.C::load_EOP`, `ifteph.C::IFTE_init`, `jpleph.c::jpl_init_ephemeris` counterparts |
| `src/clkcorr.rs` | `clkcorr.C::getClockCorrections/getCorrection/getClockCorrectionSequenceInternal`; selected route/cache behavior and owned tables, without explicit source chains |
| `src/tabulated_function.rs` | `tabulatedfunction.C::TabulatedFunction_load/TabulatedFunction_getValue/TabulatedFunction_getStartX/TabulatedFunction_getEndX` in an owned table |
| `src/eop.rs` | `eop.C::load_EOP/get_EOP` |
| `src/ifteph.rs` | `ifteph.C::IFTE_init/IFTE_get_Vals/IFTEinterp` in owned storage; split-epoch responsibility from `tt2tdb.C::tt2tb` |
| `src/read_ephemeris.rs` | Selected `readEphemeris.C::readEphemeris`; retained `jpleph.c::jpl_init_ephemeris/jpl_pleph/jpl_get_double/jpl_init_error_code/jpl_close_ephemeris`, with Rust ownership/preflight |
| `src/get_obs_coord.rs` | Selected `get_obsCoord.C::get_obsCoord_IAU2000B`; retained `sofa/{pom00,rxp,sxp,pxp,c2t00b,trxpv,trxp}.for` routines `iau_POM00/RXP/SXP/PXP/C2T00B/TRXPV/TRXP` and their pinned closure |
| `src/tt2tdb.rs` | `tt2tdb.C::tt2tb`, accepted IF99 SI/TCB branch |
| `src/tropo.rs` | `tropo.C::getMeteorologicalValue/getZenithWetDelay/getSurfaceAtmosphericPressure/NMF_hydrostatic/NMF_wet/compute_tropospheric_delays`; combined lookup/provenance and single-TOA Rust operation |
| `src/std_freq.rs` | `t2fit_stdFitFuncs.C::t2FitFunc_stdFreq/t2UpdateFunc_stdFreq` |
| `src/tksvd.rs` | `TKsvd.C::TKbidiagonal/TKsingularValueDecomposition_lsq/TKbacksubstitution_svd/TKpythag`; `T2toolkit.C::TKsign_d/TKretMax_d` |
| `src/tkfit.rs` | `TKfit.C` nonaccelerated `TKleastSquares` route: covariance, back substitution, mixed-precision chi-square, error rescaling; `FitNumber` is Rust-only diagnostic transport |
| `src/t2fit.rs` | `t2fit.C::t2Fit/t2Fit_getFitData/t2Fit_fillFitInfo/t2Fit_buildDesignMatrix/t2Fit_updateParameters` local ZERO/F0–F3 subset and `tempo2.C::main` NITS lifecycle; owned fit results are Rust-only glue |
| `src/main.rs` | Rust-only command/JSON publication glue for `tempo2.C::main`, `getInputs.C::getInputs` and `textOutput.C::textOutput` responsibilities; not the full upstream CLI/output contract |
| `build.rs` | Rust-only Cargo build glue replacing selected `Makefile.am` and `sofa/Makefile.am` rules for retained JPL/SOFA; no standalone timing routine |

ZERO construction and storage remain in `t2fit.rs`; the frequency callback lives
in `std_freq.rs`, with `SPIN` fixing the source F0–F3 order. The SVD retains source
indices, `an`, `rv1`, `w`, `v`, `u`, `wt` and arithmetic association. Unused source
allocation of a separate `u` buffer is omitted: the source operates on the
modified design matrix as U. No algorithm, column normalization, fallback solver
or new numerical dependency is substituted.

`TKlongdouble.h` is generated by the pin's `Makefile.am` from the selected numeric
header; there is no `longdouble.h`. The IFTE comment now distinguishes unchecked
header reads in `ifteph.C::IFTE_init` from checked data seeks/short reads followed
by exit in `IFTE_get_Vals` (F B06). Neither correction changes code or dispositions
of the existing Rust parser/range/error adapters.

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
| `read_ephemeris.rs` AU query | C ephemeris metadata | Live non-null exclusive handle and source constant field offset 28; U2/B29 remains unresolved because the packed AU offset does not satisfy the getter's cast-double alignment requirement |
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

Separately, F's **U2/B29** records that canonical `jpl_int.h` packs AU at byte
offset 28, while `jpl_get_double` casts `base + field` to `double*` and dereferences
it. The host requires eight-byte double alignment; aligned allocation does not
establish alignment of `base + 28`. Successful timing fixtures do not prove a
portable alignment contract. F's [native layout evidence](../validation/coverage/native_replay.json)
at commit `15eee128aca18e4fd1f813a2084ba784b73570d0` measures offset 28,
alignment 8 and struct size 648 without executing the unaligned getter or running
a sanitizer. This retained-source risk is distinct from U1's file-identity gap.
The adjacent AU-call comment now states it; E changes no native behavior.
