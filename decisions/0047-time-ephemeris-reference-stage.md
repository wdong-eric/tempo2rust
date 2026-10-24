# Time ephemeris reference and Assignment V

Central decision, 24 October 2026. **Release Assignment V for complete bounded native time-ephemeris and TT-to-barycentric-time evidence.** V owns only new `validation/time_ephemeris/`. T remains the sole Rust owner; U independently owns `validation/terrestrial_resources/`. Central retains later implementation contracts, numerical policy and acceptance.

The existing Rust IF99/SI subset and historical M1b evidence do not establish the complete IFTE file/state API, FB90 reader/cache or all tt2tb branches. This reference supplies that dependency without changing the accepted isolated/PKS gates or choosing a new Rust backend.

## Source and selected membership

Use unchanged raw TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`:

| Source | Raw SHA-256 |
| --- | --- |
| ifteph.C | `ddf1f64bbff6ce3b10cefc8c7aefdf9fe42f38936b89f90359b9c6da04ed40d4` |
| ifteph.h | `ed3a13ca76b0fa395a19b38fc86f99ba3da68204268385f8eafc5aa52b0a3a70` |
| tt2tdb.C | `c62c6c599f8101703ed511a80653bc53d09ff7a4af89ddd98b8b4d936a222426` |
| read_fortran.h | `b20d74d5ed9b832267bd07ea633081c3c5c780c65fe6d9cc37605677dca48e46` |

The **29 selected routines** are:

- Seven IFTE byte helpers: IFTswap4, IFTswapInt, IFTswapInts, IFTswap8, IFTswapDouble, IFTswap8N, IFTswapDoubles.
- Ten IFTE operations: IFTE_init, IFTE_close_file, IFTE_get_Vals, IFTE_get_DeltaT_DeltaTDot, IFTE_DeltaT, IFTE_DeltaTDot, IFTE_get_vE_vEDot, IFTE_get_vE, IFTE_get_vEDot, IFTEinterp.
- Four conversion routines: tt2tb, init_ifte, IF_deltaT, FB_deltaT.
- Eight read_fortran.h functions: open_file, close_file, read_character, read_char, read_int, read_float, read_double, read_record_int.

Inventory the complete source functions, header constants, static/global/local retained state, buffer dimensions, read/write order, runtime branches and preprocessing dispositions before experiments. read_fortran.h is distinct from N's read_fortran2.h; do not transfer coverage between their similar names. Inactive code and compiled but unentered dependencies remain explicit.

Build unchanged primary files and actual clock/numeric/diagnostic dependencies under the measured GCC binary128/C-locale profile. Reuse accepted J/M/N/S preparation and observation patterns where useful, while recording new actual identities. Verbatim global-data/CVS slices must be bounded and hashed; no fake getCorrectionTT, vector helper, IFTE/FB result or initializer is permitted. No A2 correction or resource instrumentation may execute. Including raw source files in the caller translation unit for read-only static-state access is allowed; rewriting their operations or adding reset hooks is not.

Pin raw sources/headers, generated configuration, numeric selection, compiler/flags, runtime libraries, fixtures/resources, caller/runner and executed binaries. Record native widths, signed char behavior, integer representations and header union/byte-decoding host qualifications. Only source-defined/declared-host observations become ordinary evidence.

## Experiments and state projection

Use fresh processes for independent cases and explicit ordered actions for shared histories. Seed every caller field that an executed branch reads, with meaningful distinct values and retained tails. Synthetic zero/seed initialization is caller state, not native initialiseOne. The complete R/S field maps support observation/pulsar projection; a full covariance allocation is unnecessary for a caller that never reads it.

Record all initialized file/header/cache/interpolation fields, logical live/released owners, file position while the handle is live, output membership and retained cells, streams and return/exit classes. Separate source static-zero initialization, explicit IFTE_init writes and retained cells. The IFTE title is not populated by the header read; the local header buffer does not establish a title assignment. Known bytes must not disappear behind an unwritten marker. Do not serialize padding, read a released FILE object or fabricate private-local accessors.

Retain raw bytes and lossless numeric representations. Compare binary64/selected binary128 values, signed zero and established nonfinite classes; native object padding or pointer addresses are not portable state. Observational snapshots must not mutate caches or consume input bytes before the action under test. Any additional instrumentation needs its own identity and comparison with the unchanged ordinary execution.

Keep invalid storage, uninitialized reads, out-of-range integer casts, arithmetic/pointer overflow and released-handle operations unrun. Prove bounds for each unusual fixture before execution; do not infer validity from a process returning. Record source-inspected prefixes separately. Defined source exits may be followed by a valid fresh process, but this is not in-process recovery.

## IFTE binary format, lifetime and interpolation

Generate small deterministic IFTE fixtures in both byte orders from an explicit byte-layout specification. Cover complete headers/constants, both coefficient groups, multiple records/subintervals, varying valid coefficient counts through the 18-cell interpolation limit, and the 322-double read-buffer constraint. Account for the record-length expression, six descriptor integers, ephver's floor conversion, actual constants record and retained unused fields. Verify layout independently against decoded bytes and the real pinned IF99 file; do not use the Rust reader as the generator or oracle.

Exercise all byte helpers with initialized spans and sentinel tails, zero/negative counts where loops are defined, and round trips without interpreting invalid scalar representations. Direct header-helper branch coverage must distinguish the host's natural byte order from an explicitly seeded caller control.

IFTE_init leaves selected interpolation cells and twot untouched while resetting np/nv/pc[0]/pc[1]/vc[1]/irec. Cover first initialization and reinitialization after different interpolation histories, especially a next normalized time of zero. Save and observe an older still-open file through a declared caller owner before replacement; source replacement does not close it. Clean up retained old handles outside the measured action with provenance. IFTE_close_file accepts an initially null handle but leaves a closed non-null pointer afterward. Double close and getters after close stay unrun; a later successful init establishes a new live owner.

IFTE_get_Vals normalizes a two-part epoch, checks the first normalized component against startJD, computes a record and decrements it when that component equals endJD. Cover equivalent split epochs, negative fractions/carries, record/subinterval boundaries, start/end and safely backed extrapolation cases. Do not add an upper-date check absent from source. Distinguish defined short-record exits from unsafe header reads or interpolation offsets.

**The source never updates ifte.irec after reading a record.** Preserve and demonstrate that state, repeated reads and their observable file-history consequences. A later optimized cache is a behavior change. File replacement/truncation observations must record actual open-handle/stdio behavior on this host, not assume every underlying edit is immediately visible through a buffered FILE.

Exercise direct IFTEinterp with valid caller-owned coefficient/cache/output storage: one/three and other defined component counts, positions-only and derivative branches, repeated/changed normalized times, increasing/decreasing polynomial counts, subinterval selection and source reverse summation order. Preserve derivative scaling by the whole interval, cache writes and output/unused-tail retention. Test defined aliased output histories in the convenience wrappers, accounting for their interleaved assignment order. Invalid coefficient extents, cache lengths and count paths remain unrun.

Include simple independently calculated polynomial values/derivatives as supporting checks. They do not replace the exact source oracle or set a future Rust tolerance. Use a bounded real IF99 smoke history to bind the synthetic format to the actual resource.

## Fortran reader and FB90 histories

Exercise every read_fortran.h function with controlled initialized files and mutable filename buffers. open_file chooses swapByte from the host and truncates the filename at its first space; retain the suffix and actual opened prefix. It returns 1 on a missing file, leaving no usable stream. Call the failure helper directly, without subsequent read/close through null. read_record_int performs its own native-order fread and does not apply the other readers' byte swap.

Cover complete integer/float/double/character reads, count/no-op boundaries, EOF classes only where conversion/object representation is defined for the recorded host, and source stream position. read_record_int's incomplete read can expose an uninitialized local and stays unrun. Preserve the distinction from the byte readers that assign every representation byte through fgetc. Union decoding/EOF char conversions remain host-qualified rather than a cross-platform ABI promise.

Build explicit small FB90 file fixtures from the actual header/read sequence and record traversal; independently verify their bytes. Exercise jda/jdb splitting around the half-day boundary, valid epochs near range/record boundaries, positive record widths, valid coefficient counts bounded by buf[16]/pc[18], and defined zero/negative-count cases only after full source bounds analysis. Preserve the actual nr range condition, sequential record-reading loop, normalized time and reverse Chebyshev sum.

**FB_deltaT retains tdbnrl and buf across file opens.** Test same/different record numbers, root/file changes with the same record number, return to earlier records, nr==1's no-read behavior when defined, and repeated tt2tb calls using FB90. It still opens and reads each new header before deciding whether to reuse coefficients. Do not key that cache by file identity or clear it between actions.

FB_deltaT ignores open_file's status; missing-file calls through that wrapper would continue into invalid reads and are not ordinary exit goldens. Keep direct missing-open evidence and the wrapper's source-inspected invalid path distinct. Also classify malformed/truncated headers, zero divisors, excessive coefficient counts, path overflow and invalid date-to-int conversions before experiments. Do not hide them behind a blanket native error result.

## Full tt2tb caller behavior

Cover all runtime branches using declared pulsars/observations and actual IFTE/FB files. init_ifte runs even for zero/negative npsr or observations whose time correction is bypassed, and tt2tb closes the IFTE stream at its end. Test this resource requirement and repeated-call lifecycle explicitly.

Exercise clockCorr zero/nonzero, exact STL spelling, supplied correctionTT values through the real getCorrectionTT helper, IF99/FB90 selection, SI/TDB and other source-defined integer-control paths, dilateFreq equal to one versus other values, multi-pulsar/count histories and retained fields. The bypass writes correctionTT_TB only; retain correctionTT_Teph/einsteinRate where the source does. deltaT starts once per tt2tb call and can retain its previous value when neither timeEphemeris comparison matches.

For useCalceph==1, **the source reads psr[0].obsn[i].correctionTT_calcEph** even while processing another pulsar. Give pulsar zero enough initialized observation backing and distinct values to expose that behavior. These are declared caller-supplied values; they do not establish a CALCEPH library implementation or its ephemeris calculations.

Preserve native double/longdouble conversion points, integer epoch splitting, constants including extended IFTE_K construction, vector roles and operation order. Test the second scaling of obsTerm rather than obsTermDot, unit-dependent drift/rate expressions, and dependence on IFTE derivatives even when the selected delay uses FB90 or the supplied CALCEPH correction. Never replace those expressions with a corrected physical derivation.

Compare full initialized output/retained state after every action and preserve source exits at their actual point. Direct helper cases plus integrated histories must expose required resource selection, state reuse and numerical type boundaries. No broader BAT/residual or live CALCEPH coverage follows from these inputs.

## Verification, outputs and ownership

Fresh normal verification checks actual source/compiler/runtime/caller/runner/input/executable identities before and after launch, challenges each case, removes stale success receipts and never rewrites its ordinary oracle. Require exact ordered case/action/field membership, state, numeric representations/classes, streams and statuses. Permit only a narrow declared diagnostic path/address projection and keep raw streams; no path-dependent truncation or failed stage may be normalized away.

Include a positive control and meaningful membership/order/value/knownness/lifetime/cache/stream/status/identity controls. Defined compiled mutations need real numerical, state or status witnesses: updating IFTE's record cache, clearing retained interpolation state, changing half-day/subinterval behavior, correcting FB's cross-file cache, altering bypass retention, changing the pulsar-zero read and scaling obsTermDot. A hash difference alone is not a behavioral witness.

Measure all 29 selected routines and source branches in a separate coverage profile. Compare its observations with the ordinary executable and expose compiler/numerical differences for central disposition. Classify unhit paths as inactive, host-dependent, native-invalid or still-defined/unverified. Coverage counters do not imply exhaustive input or platform validation.

Deliver `validation/time_ephemeris/REPORT.md`, complete source/branch/caller/lifetime/format maps, reproducible commands, actual identities, raw/local replay/control/mutation/coverage evidence, limitations and explicit-path Conventional Commits. Under **0039**, generated JSON/JSONL/compressed outputs, binary fixtures produced by generators, raw archives and other large generated evidence stay local in ignored output/build paths. Commit sources, generators, authored inputs/contracts and concise reports only; never force-add outputs.

Protect accepted references and both independent repositories. T's active `rust/` and `validation/pulsar_lifecycle_port/`, U's `validation/terrestrial_resources/`, and central documents are concurrent work outside V's immutable membership. Use accepted Git revisions for fixed Rust context rather than treating T's working files as an oracle.

No Rust edits, source repairs, new numerical dependencies/backends/tolerances or milestone decisions are delegated. General planetary ephemeris/CALCEPH implementations, astrometry/site transformations, BAT/propagation assembly, broader file-fault/platform/concurrency behavior and the unresolved M2 gate remain separate work. Return concrete out-of-contract findings to central while continuing independent cases.

## Dispatch

Created **Assignment V: TEMPO2 time ephemeris reference**, chat `01a16bdb-8e82-71a0-b823-67399b391ae0`, in the local `rustify-tempo2` project from release commit `8e56df0`. An immediate snapshot confirms an active reference turn. T and U received V's separate ownership boundary; T remains the sole Rust owner. The dispatch preserves policy 0039 and central acceptance authority. This is active reference work, not accepted evidence.
