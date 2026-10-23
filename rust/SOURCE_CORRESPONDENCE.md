# Production correspondence and unsafe boundaries

Assignments E/I/L, TEMPO2 pin `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
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
| `src/clkcorr.rs` | `clkcorr.C::getClockCorrections/getCorrection/getClockCorrectionSequenceInternal`; all four public operations, explicit chains and all selected helpers/constructors; process session, directed routes, lazy tables and source fallbacks |
| `src/toa2utc.rs` | Active `toa2utc.C::toa2utc`, bypass and exact SAT jump mutation |
| `src/observatory.rs` | Selected `observatory.C` file/alias parsing, independent process initialization, alias lookup/accumulation and geometry |
| `src/clock_diagnostics.rs` | `tempo2Util.C::displayMsg`, selected `TKlog.C/h`, `global.C::CVSdisplayVersion`; byte-event sink |
| `src/clock_text.rs` | Private typed libc conversion/formatting for source fixed formats, C-string and whitespace rules |
| `src/clock_glob.rs` | Pinned Darwin glob flags-zero byte grammar, dot entries/leading-dot rule, raw path spelling and full-string ordering; 0029 supplemental helper/index/observatory evidence |
| `src/tabulated_function.rs` | `tabulatedfunction.C::TabulatedFunction_load/TabulatedFunction_getValue/TabulatedFunction_getStartX/TabulatedFunction_getEndX` in an owned table |
| `src/eop.rs` | `eop.C::load_EOP/get_EOP` |
| `src/ifteph.rs` | `ifteph.C::IFTE_init/IFTE_get_Vals/IFTEinterp` in owned storage; split-epoch responsibility from `tt2tdb.C::tt2tb` |
| `src/read_ephemeris.rs` | Selected `readEphemeris.C::readEphemeris` call order and Rug128 scaling; safe JPL calls, timing error/nonfinite adapter |
| `src/jpleph.rs` | All 12 implemented functions in `jpleph.c`, declared field meanings in `jpleph.h`, decoded `jpl_int.h` layout; full map in `../validation/jpl_port/PROJECTION.md` |
| `src/stdio.rs` | Rust ownership glue for JPL's libc I/O; no standalone scientific counterpart, no JPL algorithm |
| `src/get_obs_coord.rs` | Selected `get_obsCoord.C::get_obsCoord_IAU2000B`; retained `sofa/{pom00,rxp,sxp,pxp,c2t00b,trxpv,trxp}.for` routines `iau_POM00/RXP/SXP/PXP/C2T00B/TRXPV/TRXP` and their pinned closure |
| `src/tt2tdb.rs` | `tt2tdb.C::tt2tb`, accepted IF99 SI/TCB branch |
| `src/tropo.rs` | `tropo.C::getMeteorologicalValue/getZenithWetDelay/getSurfaceAtmosphericPressure/NMF_hydrostatic/NMF_wet/compute_tropospheric_delays`; combined lookup/provenance and single-TOA Rust operation |
| `src/std_freq.rs` | `t2fit_stdFitFuncs.C::t2FitFunc_stdFreq/t2UpdateFunc_stdFreq` |
| `src/tksvd.rs` | `TKsvd.C::TKbidiagonal/TKsingularValueDecomposition_lsq/TKbacksubstitution_svd/TKpythag`; `T2toolkit.C::TKsign_d/TKretMax_d` |
| `src/tkfit.rs` | `TKfit.C` nonaccelerated `TKleastSquares` route: covariance, back substitution, mixed-precision chi-square, error rescaling; `FitNumber` is Rust-only diagnostic transport |
| `src/t2fit.rs` | `t2fit.C::t2Fit/t2Fit_getFitData/t2Fit_fillFitInfo/t2Fit_buildDesignMatrix/t2Fit_updateParameters` local ZERO/F0–F3 subset and `tempo2.C::main` NITS lifecycle; owned fit results are Rust-only glue |
| `src/main.rs` | Rust-only command/JSON publication glue for `tempo2.C::main`, `getInputs.C::getInputs` and `textOutput.C::textOutput` responsibilities; not the full upstream CLI/output contract |
| `build.rs` | Rust-only Cargo build glue replacing selected `Makefile.am` and `sofa/Makefile.am` rules for retained SOFA; no standalone timing routine |

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

`native/upstream_manifest.json` identifies the 36 unchanged SOFA routines and
notices at their TEMPO2-relative paths. `native/README.md` describes the retained
closure. JPL C files/build/FFI and the historical cache-correction patch are
removed; `jpl_source_manifest.json` preserves source hashes and GPL attribution.
No numerical dependency or alternate JPL implementation was added.

## Unsafe audit

There are twelve production unsafe sites: five in `stdio.rs`, two existing
SOFA sites, and five in `clock_text.rs` (declarations plus four typed calls).
JPL and clock/table algorithms, storage, graph traversal and synchronization
remain safe Rust. Each site
has adjacent necessity and concrete invariants. No unsafe function or Send/Sync
implementation is present.

| Site | Necessity | Invariants |
| --- | --- | --- |
| `clock_text.rs` foreign declarations and pair/header/number-prefix calls | Complete host sscanf grammar, prefix and binary32 conversion | Owned NUL input, private literal formats, initialized correctly typed outputs, bounded 63-byte strings, completed-conversion reads only; no pointers escape |
| `clock_text.rs` formatting call | Fixed source printf spelling/truncation | One f64 argument, static enum-selected formats, initialized 512-byte bounded buffer; no pointers escape |
| `stdio.rs` foreign declarations | C stdio buffering is observable in H | Opaque FILE, bounded calls, no retained caller buffers |
| `stdio.rs` fdopen | Consume the already opened file without path reopen | File owns live fd until success; static NUL mode; failure drops File; success relinquishes fd exactly once |
| `stdio.rs` fread | Complete-item counts and partial-element writes | Checked size product fits initialized exclusive byte slice; no pointer escapes; live exclusive stream |
| `stdio.rs` fseek | Source seek/buffer transitions | Exclusive stream borrow, source-width offset/origin, no concurrent cache/position access |
| `stdio.rs` fclose | Matching destruction of libc buffer and fd | Non-cloneable stream, one Drop, all calls ended, no escaped pointer |
| `get_obs_coord.rs` extern block | Retained SOFA ABI | By-reference binary64 scalars and contiguous column-major arrays |
| `get_obs_coord.rs` call block | SOFA rotations | Aligned initialized arrays, exclusive outputs, disjoint cross-product slices, no retained pointers, automatic Fortran locals |

The test-only stdio observer duplicates a File descriptor only for metadata,
without reads/seeks. It adds no unsafe site and is absent from production builds.
The production stream is neither Clone nor Send/Sync. These are bounded ownership
checks on the declared host, not concurrency stress or other-platform validation.

## JPL boundary dispositions

U1: runtime provenance reads/hashes and rewinds one File, then consumes it into
stdio. Path replacement cannot substitute another object. In-place writes remain
visible according to stdio/cache behavior; the digest is not an immutable snapshot.

U2/B29: selectors read decoded AU/EMRAT fields; no packed typed dereference remains.
B03: canonical cache-before-read is restored, including stale retry success,
partial bytes and skipped byte swapping after failed reads. It is preserved as a
known source defect, not corrected downstream.

Decision 0017 supplies the remaining owned/undefined-state dispositions in
`../validation/jpl_port/PROJECTION.md` and `BEHAVIOR_DIFFERENCES.md`. H's full-call
values remain host observations subject to source undefined pointer formation.
Exact agreement does not make them portable C results or accept the full transfer.

## Clock/table scope and authorized contraction

`../validation/clocks_port/SOURCE_MAP.md` maps all 32 J entries and dependencies.
The user authorized only the final table interpolation `mul_add` to reproduce
J's pinned GCC `fmadd`; all preceding operations and every numerical budget stay
unchanged. `verification/clocks/interpolation_contract.json` records the evidence.
Process serialization, terminal-source errors, operation-local storage bounds,
byte diagnostics and JSON presentation are explicit adapters in `CLOCK_REPORT.md`.
