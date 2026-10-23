# Production correspondence and unsafe audit

All `S:path:line` references mean raw canonical Git blobs at `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; all `R:path:line` mean primary baseline `ddc07b20fafc21fe92f2ecc1dd24c59db9df4142`. Use `git -C tempo2 show COMMIT:path | nl -ba` or `git show COMMIT:path | nl -ba`. Live Rust files may already differ because E is active. Generated from the checked JSON/curated TSV; no milestone acceptance is implied.

The 19-file production Rust set includes build.rs, lib.rs and the CLI. resource_tests.rs is cfg(test); examples/tests are evidence, not production modules. There are eight unsafe sites: two extern declarations and six blocks, no unsafe fn/impl. Dependencies' internal Rust unsafe is outside this crate-site enumeration; retained C/Fortran obligations are described below.

| File | Header assessment | Actual source counterparts | Finding |
| --- | --- | --- | --- |
| `rust/build.rs` | missing | `S:Makefile.am:48`, `S:sofa/Makefile.am:3` | No header correspondence; build-only Rust glue should identify retained JPL/SOFA build responsibility and absence of standalone timing counterpart. The frecursive comment at 29 is behavioral/threading context, not module correspondence. |
| `rust/src/clkcorr.rs` | incomplete | `S:clkcorr.C:728`, `S:clkcorr.C:681` | Header names file only; add getClockCorrections/getCorrection and route/cache helpers, selected subset and omitted explicit chains. ClockStep describes source correctionsTT; ClockCorrections is Rust-owned process-state replacement. |
| `rust/src/eop.rs` | incomplete | `S:eop.C:55`, `S:eop.C:138` | Header names file only; name load_EOP/get_EOP and owned Eop replacement for static samples, with upper-bound departure. |
| `rust/src/form_bats.rs` | incomplete | `S:global.C:134`, `S:calculate_bclt.C:41`, `S:formBats.C:36`, `S:shapiro_delay.C:38`, `S:dm_delays.C:41`, `S:vectorPulsar.C:57`, `S:secularMotion.C:36` | Header names first three files; API doc names formBatsAll. Embedded Shapiro, DM, vector and zero secular branch counterparts are missing. evaluate_terrestrial also runs formResiduals; it is not identical in scope to formBatsAll. |
| `rust/src/form_residuals.rs` | incomplete | `S:formResiduals.C:396`, `S:tempo2Util.C:186`, `S:tempo2Util.C:211`, `S:formBats.C:36`, `S:secularMotion.C:36` | Header names residual/util files but no named routines; inline anchors explain split phase. @ entry additionally supplies zero BAT/BBAT path. source_i32/pulse_integer are checked Rust conversions, not full legacy functions. |
| `rust/src/get_obs_coord.rs` | incomplete | `S:get_obsCoord.C:55`, `S:sofa/c2t00b.for:1` | Header names get_obsCoord_IAU2000B and SOFA but omits source filename and named retained closure. Observatory/site geometry and opaque Fortran matrices have recognizable meanings. |
| `rust/src/ifteph.rs` | incomplete | `S:ifteph.C:128`, `S:ifteph.C:204`, `S:ifteph.C:387` | Header names file only; name IFTE_init/IFTE_get_Vals/IFTEinterp. coefficient_layout is source one-based IPT (documented at 26); descriptive coefficient/component/subinterval names preserve roles. split_ifte_epoch additionally supports tt2tdb.C epoch splitting. |
| `rust/src/input.rs` | missing | `S:readParfile.C:2542`, `S:readTimfile.C:56`, `S:initialise.C:56` | Header describes ownership/load only; explicitly identify Rust-only adapter and source parser/initialization responsibilities. SourceFile/provenance and immutable TimingInput have no standalone TEMPO2 routine. |
| `rust/src/lib.rs` | missing | `S:tempo2.h:1`, `S:initialise.C:56`, `S:global.C:134`, `S:formResiduals.C:396` | Header describes profile only; document owned projections of pulsar/observation/parameter, error/serialization glue and public reexports. No C ABI equivalence is implied by these types. |
| `rust/src/longdouble.rs` | misleading | `S:TKlongdouble.C:58`, `S:TKlongdouble.float128.h:1`, `S:TKlongdouble.ld.h:1`, `S:Makefile.am:164` | Header cites nonexistent longdouble.h. Actual generated header is TKlongdouble.h. Scalar/Rug128 is a finite 128-bit MPFR value, not source IEEE binary128 (113 bits), native C long double or ABI-compatible object. |
| `rust/src/main.rs` | missing | `S:tempo2.C:51`, `S:getInputs.C:52`, `S:textOutput.C:60` | No header correspondence; residuals CLI/JSON adapter has no standalone source counterpart and implements only bounded orchestration, not upstream argument/output contracts. |
| `rust/src/read_ephemeris.rs` | incomplete | `S:readEphemeris.C:56`, `S:jpleph.c:698`, `S:jpleph.c:135` | Header says pinned low-level JPL only; identify readEphemeris/read_ephemeris and jpl_init_ephemeris/jpl_pleph/close boundary. Inline call-order comment helps but does not identify full module scope. |
| `rust/src/read_parfile.rs` | adequate | `S:readParfile.C:2542`, `S:readParfile.C:2762` | Header explicitly names supported readParfile.C/readValue path; snake_case is clear. Future header expansion can point to checkLine/default subset, but present source/routine statement is usable. ParFile is owned glue. |
| `rust/src/read_timfile.rs` | incomplete | `S:readTimfile.C:56`, `S:readTimfile.C:119` | Header names file and FORMAT 1 but not readTim/readTimfile routines; reader implements only one format and deliberately rejects directives/flags. |
| `rust/src/runtime.rs` | missing | `S:observatory.C:286`, `S:observatory.C:394`, `S:clkcorr.C:728`, `S:tropo.C:301` | No module header. RuntimeContext/Resource are Rust ownership/provenance glue; get_observatory includes source getObservatory/ITRF_to_GRS80 and orchestrates IFTE/JPL/EOP/weather loaders. |
| `rust/src/tabulated_function.rs` | incomplete | `S:tabulatedfunction.C:40`, `S:tabulatedfunction.C:116` | Header names file only; name TabulatedFunction_load/getValue/start/end and safe owned table representation. |
| `rust/src/tempo2_util.rs` | incomplete | `S:tempo2Util.C:40`, `S:tempo2Util.C:121`, `S:tempo2Util.C:127`, `S:tempo2Util.C:149`, `S:tempo2Util.C:186`, `S:tempo2Util.C:211` | Header names file responsibility but not routines; dot_product maps dotproduct, conversions retain snake_case names; parse_binary64 is strict adapter parsing and signed remainder uses Rug. |
| `rust/src/tropo.rs` | incomplete | `S:tropo.C:49`, `S:tropo.C:160`, `S:tropo.C:314`, `S:tropo.C:367` | Header names file only; name meteorology lookup, NMF_hydrostatic/NMF_wet and compute_tropospheric_delays (singular Rust operation handles one TOA). Combined fallback API and Weather provenance are Rust adaptations. |
| `rust/src/tt2tdb.rs` | incomplete | `S:tt2tdb.C:65` | Header names file only; name tt2tb and restricted IF99 SI/TCB branch. correction_tt_teph, correction_tt_tb and einstein_rate match source fields. |

## U1: JPL file identity is an unenforced safety precondition

`R:rust/src/runtime.rs:215` reads/hashes bytes and passes them to preflight; `R:rust/src/read_ephemeris.rs:152` then calls native initialization which independently opens the path (`S:jpleph.c:704`). Neither the Rust borrow nor the mutex locks the resource file. The validated image need not equal the subsequently opened/native-read bytes if another actor changes it. Therefore the current structural-validation safety argument assumes stable file contents/identity. This is a source-inspected gap, not an executed race or proof that ordinary pinned fixtures fail. The later owner needs a concrete stable-input/native-boundary contract or implementation plus evidence; documenting a SAFETY label alone cannot enforce this invariant. F changes no implementation.

`NonNull<c_void>` has no unsafe Send/Sync implementation in the baseline, so JplEphemeris and RuntimeContext cannot be transferred/shared through safe Rust threads. Separate contexts can be constructed in separate threads; the mutex covers native init/global error access. &mut self serializes calls per handle. SOFA's -frecursive and immutable DATA tables support independent calls, but there is no concurrency stress result.

## U2: packed native AU getter lacks portable double alignment

`S:jpl_int.h:30` packs jpl_eph_data; its au member is at offset 28. `S:jpleph.c:75` casts base+field to double* and dereferences it; `R:rust/src/read_ephemeris.rs:160` requests field 28. F's layout-only probe reports double alignment 8 and struct size 648 on the actual host. A calloc-aligned base therefore does not align base+28 for this typed dereference. The wrapper cannot establish a portable C alignment contract merely from NonNull/owned allocation. This retained-source risk is separate from U1 and from ordinary successful fixture execution; F ran no sanitizer or deliberately unaligned getter probe and makes no crash claim. Record a central native-boundary disposition under preservation policy rather than silently repairing source. Evidence: native_replay.json native_layout and ledger B29.

## Unsafe site 1: R:rust/src/get_obs_coord.rs:4 (extern_declaration)

Native: `S:sofa/pom00.for:1`, `S:sofa/rxp.for:1`, `S:sofa/sxp.for:1`, `S:sofa/pxp.for:1`, `S:sofa/c2t00b.for:1`, `S:sofa/trxpv.for:1`, `S:sofa/trxp.for:1`.

Why unsafe / safe replacement: No existing safe wrapper for this retained pinned closure; scalar/vector code could be ported later but is not an already validated replacement.

Actual contract: GFortran underscore symbol names, pass-by-address f64 matching DOUBLE PRECISION; arrays use contiguous column-major 3x3, 3 and 3x2 storage; functions are synchronous and retain no pointers. ABI verified only by historical macOS arm64 builds.

Adjacent explanation: No adjacent safety or ABI contract; line 1 gives routine purpose only.

## Unsafe site 2: R:rust/src/get_obs_coord.rs:61 (block)

Native: `S:sofa/pom00.for:1`, `S:sofa/rxp.for:1`, `S:sofa/sxp.for:1`, `S:sofa/pxp.for:1`, `S:sofa/c2t00b.for:1`, `S:sofa/trxpv.for:1`, `S:sofa/trxp.for:1`.

Why unsafe / safe replacement: Unsafe required at foreign calls and pointer offsets with current native boundary; no safe wrapper exists.

Actual contract: All inputs/outputs are initialized aligned stack f64 arrays; matrices length 9, pv length 6 and vectors length 3. add(3) spans elements 3..6 of six-element arrays. PXP reads terrestrial_state[0..3] and writes disjoint [3..6]; TRXPV reads six and writes separate six. Borrowed Observatory survives calls; no pointer escapes, no heap cleanup. Matrix data remain in Fortran column order. frecursive build flag gives automatic mutable locals; DATA tables are retained constants, no concurrent stress verification.

Adjacent explanation: Line 60 mentions column-major layout only; missing unsafe necessity, lengths/alignment, nonoverlap, initialized output, pointer lifetime and threading rationale.

## Unsafe site 3: R:rust/src/read_ephemeris.rs:11 (extern_declaration)

Native: `S:jpleph.h:1`, `S:jpleph.c:75`, `S:jpleph.c:135`, `S:jpleph.c:666`, `S:jpleph.c:698`, `S:jpleph.c:884`.

Why unsafe / safe replacement: No safe wrapper already present; current boundary retains low-level native implementation.

Actual contract: Opaque native allocation never dereferenced by Rust; C int/char/void and double widths match declared ABI on tested host. names is passed null (native char[][6] storage never accessed). init/close are paired and native status/error code contracts matter.

Adjacent explanation: No adjacent declaration-level ABI and ownership contract.

## Unsafe site 4: R:rust/src/read_ephemeris.rs:152 (block)

Native: `S:jpleph.c:698`.

Why unsafe / safe replacement: No existing safe initialization wrapper.

Actual contract: CString is NUL-terminated and lives through synchronous fopen; null names/values are explicitly supported. JPL_INIT_LOCK covers init and error retrieval. NonNull rejects null; successful allocation+FILE owned solely by JplEphemeris, with Drop cleanup. Preflight bounds coefficients and record arithmetic but validates bytes separately from native fopen(path); stable file identity/content between read/preflight/open and subsequent reads is an unenforced assumption (U1).

Adjacent explanation: Nearby preflight/global-lock comments explain intent, but not full ownership, nullable arguments, filename lifetime, file stability or ABI. NonNull alone does not establish native safety.

## Unsafe site 5: R:rust/src/read_ephemeris.rs:156 (block)

Native: `S:jpleph.c:666`.

Why unsafe / safe replacement: No existing safe accessor; foreign global status read.

Actual contract: Called only on null return while the same JPL_INIT_LOCK guard is held; foreign initialization users in this crate all use that lock. Other externally linked native callers would also need synchronization. No pointer/output buffer involved.

Adjacent explanation: Global-lock explanation at 27 is relevant but remote; no adjacent necessity and held-lock explanation.

## Unsafe site 6: R:rust/src/read_ephemeris.rs:160 (block)

Native: `S:jpleph.c:75`, `S:jpleph.h:75`, `S:jpl_int.h:30`.

Why unsafe / safe replacement: No existing safe native field getter.

Actual contract: Successful raw handle is a live calloc allocation, field 28 is JPL_EPHEM_AU_IN_KM. However, native jpl_eph_data is packed: au offset=28 while double alignment=8 on this host. jpl_get_double casts base+28 to double* and dereferences it, so allocation alignment does not establish field-pointer alignment (U2). Layout probe does not execute unaligned access. Read only, no concurrent alias; native cleanup later owns the handle.

Adjacent explanation: No adjacent safety explanation; magic 28 hides a native alignment precondition that is not satisfied by packed layout. Passing macOS fixtures is not a portable alignment proof. Following comment explains scientific scaling only.

## Unsafe site 7: R:rust/src/read_ephemeris.rs:189 (block)

Native: `S:jpleph.c:135`, `S:jpleph.c:564`.

Why unsafe / safe replacement: No existing safe wrapper except this method; retaining kernel requires unsafe foreign call.

Actual contract: Exclusive &mut self serializes cache/FILE mutation per handle; NonNull<c_void> makes wrapper !Send and !Sync (no unsafe impl). Live handle is not cloned. Current callers use targets 2,3,5,6,7,8,11,13,14 and centres 3/12 with velocity=1. Initialized writable [f64;6] aligned output is disjoint from handle. Nutation writes four values and zero initialization defines remaining unused slots. Native does not retain output. Status must be checked; nonzero returns no published vector. Preflight/stable-file assumption U1 remains.

Adjacent explanation: After-call comment 199 explains status behavior, not pointer bounds, target set, aliasing, thread/lifetime or why FFI is necessary. pub(crate) method accepts arbitrary i32 targets; actual call sites supply valid constants.

## Unsafe site 8: R:rust/src/read_ephemeris.rs:230 (block)

Native: `S:jpleph.c:884`.

Why unsafe / safe replacement: No safe cleanup replacement for native calloc/fopen allocation.

Actual contract: Unique non-null handle from successful init, not Clone; Drop runs once, invokes fclose then free using native allocator. Exclusive drop with no concurrent access (!Send/!Sync and &mut calls); no output pointers retained. Failed construction before a handle is returned relies on native error cleanup.

Adjacent explanation: No adjacent cleanup safety explanation.

## Other misleading source descriptions

`R:rust/src/ifteph.rs:133` says upper dates reach unchecked reads. The source's header reads are unchecked (`S:ifteph.C:150`), but its coefficient read is checked and exits on a short read (`S:ifteph.C:266`). The malformed-grid conversion and upper-coverage adapter need distinct explanations. Native files preserve original headers; their provenance/corrections are recorded in native/upstream_manifest.json, departures.patch and README.md, rather than per-file Rust counterpart headers.
