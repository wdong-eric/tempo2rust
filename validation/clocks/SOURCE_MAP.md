# Assignment J source and state map

All line numbers refer to raw Git revision `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. `profile.json` pins every source/header, the A2 numeric configuration and support archive. Primary source bytes are compiled unchanged in the probe translation unit, permitting read-only snapshots of anonymous-namespace state. A preprocessing alias renames the raw table-value symbol to `J_Table_getValue`; a forwarding function records its actual binary64 input/result and returns the same double. This observation boundary changes symbol names only, not the table expression or clock arithmetic. The probe never calls `initialiseOne`. Zero-initialized pulsars/observations plus explicitly assigned fields and slot sentinels are experimental inputs, not changes to source initialization.

## Primary functions

| Source | Responsibility and observable contract | Fixture family |
| --- | --- | --- |
| tabulatedfunction.C:40 `TabulatedFunction_load` | Clears existing samples; fatal open/header/no-sample errors; permissive `%lf %lf`; keeps order and duplicate/nonfinite samples; warns on decreasing x; basename/header storage | `table_*`, `isolated_table_*` |
| :116 `TabulatedFunction_getValue` | Endpoint proxies; binary search (linear alternative literal-disabled); ordered binary64 arithmetic; zero-width NaN distinct from invalid index | `table_*`, `isolated_table_singleton` |
| :168 `TabulatedFunction_getStartX` | First sample x, requires nonempty vector | `table_*`, `isolated_table_empty_start` |
| :173 `TabulatedFunction_getEndX` | Last sample x, requires nonempty vector | `table_*`, `isolated_table_empty_end` |
| clkcorr.C:88 `ClockCorrectionFunction` | badness=1f, loaded=false, ranges=0; strings/vector constructed; table char arrays unwritten until load | every indexed graph |
| :101 `ClockSequence` | range=0; strings/vector constructed | explicit/cached routes |
| :110 `PathBuildResult` | source/target/reverse flags=false; vector empty | direct path cases |
| :117 `trim` | Trims space/tab/CR/LF, including all whitespace | `normalization` |
| :127 `normalizeClockName` | Trims then uppercase using unsigned char; C locale | `normalization` |
| :135 `clockNameEquals` | Normalized comparison | identity/chain/fallback |
| :140 `basenameFromPath` | Last slash only | `explicit_*` |
| :148 `parseClockHeaderLine` | Leading # mandatory; `%63s %63s %f`; failed/missing weight defaults to 1f; no sign/finiteness guard | `headers_*`, `weights_*` |
| :168 `addIndexedClockFile` | Reads header only, adds binary32 default offset, stores edge and path; exits on bad file/header | `index_*`, isolated index cases |
| :211 `globClockFiles` | Glob then explicit sorted order; GLOB_NOSPACE/GLOB_ABORTED exit | sorted/extra graphs; allocation/filesystem exhaustion unexecuted |
| :240 `ensureClockCorrectionsInitialized` | Extra before default, default weight +1 iff extra nonempty; default root required; process-global flag set only after full index | `index_*`, `lifetime_*`, isolated env/index |
| :296 `ensureFunctionLoaded` | First load snapshots samples/range; later calls return immediately | lifetime and retry |
| :307 `functionValidAtMJD` | Inclusive original query epoch range check after load | coverage and retry |
| :313 `sequenceMJDRange` | Intersects edge ranges with initial [0,1000000] | `range_*` |
| :331 `sequenceMatches` | Normalized keys and inclusive intersection range | range/cache precedence |
| :340 `findFunctionIndexFromToken` | Exact basename first, then token basename; supplied full path does not select a different duplicate | `explicit_duplicate_*` |
| :358 `buildDirectedPath` | Directed graph; float dist/weights; strict tie update; node insertion order; block invalid edges and retry; reverse hint only; no negative-cycle safeguard | `route_*`, `weights_*`, bounded cycle |
| :530 `cacheSequence` | Append, never replace; range determined during lazy loads | explicit/cache/range |
| :548 `getClockCorrectionSequenceInternal` | First matching cached sequence; otherwise path, CLK3/7 diagnostics, optional chain print, append | all clock graphs |
| :606 `fillCorrectionsFromSequence` | Stops at target; 30-entry limit before next write; mismatched edge false; shifted epoch; leaves prior writes on failure | capacity, sequential, controlled broken helper |
| :646 `accumulateCorrectionFromSequence` | Same shifted epoch/order; no 30-step limit; never writes observation | capacity, sequential, controlled broken helper |
| :681 `defineClockCorrectionSequence` | Whitespace tokens, basename resolution, nonempty directed chain required; caches in call order | `explicit_*`, isolated explicit |
| :728 `getClockCorrections` | Resets count only; null/empty source gets observatory name; normalized identity before init; original→target, UTC→target, original→TT(TAI), UTC→TT(TAI); exhausted ladder returns zero count; failed fill resets count but preserves array writes | fallback, null/empty, capacity, PKS |
| :810 `getCorrectionTT` | Binary64 ordered sum of counted slots only; empty=+0 | independent sum/capacity |
| :819 `getCorrection` | Identity=+0 before init; only original→target then UTC→target; no TT fallback; retains all observation fields | independent accumulator/fallback |
| toa2utc.C:60 `toa2utc` | p then observation order; clockCorr 0/2 bypass clocks; nested k then l SAT jumps test jumpSAT[l], use jumpVal[k]; increment then shift through new count (one extra slot); repeats preserve source behavior | `toa_*`, `pks_*`, isolated capacity |

## State, storage and defined snapshots

- `TabulatedFunctionSample`: two binary64 fields. `TabulatedFunction`: `fileName[256]`, `header_line[256]`, vector. Loader's header `fgets(...,1024,...)` exceeds destination capacity; normal fixtures keep complete header plus NUL within 256. Overflow cases are isolated. Body reads are 1024-byte chunks, not logical whole lines. A single x/y sample permits endpoint proxies but exact epoch/NaN query reaches index -1. Empty vector access is invalid, separately isolated. Reload clears samples without preserving old data after a failing load.
- `g_clockCorrectionsInitialized`, `g_clockCorrectionFunctions`, `g_clockCorrectionSequences` live for the entire process. Edge state includes full path, basename, source/destination spelling, binary32 badness, loaded flag, binary64 range. Snapshots never read unloaded table character arrays. Successful load initializes the two strings and all samples. Sequence cache keys exclude runtime root, extra path and resource identity; the first matching interval wins. No test resets these globals.
- `observation.sat` is binary128 (113 significand bits) in the ordinary profile. Routing narrows it to binary64. Each table query is `(double)sat + accumulated_seconds / 86400.0`; corrections and sum are binary64. `corrects_to[32]` stores truncated names. `MAX_CLK_CORR=30`; `nclock_correction` is independent of residual contents. Export all 30 initialized slots, full destination bytes and changed-byte masks, including slots outside count. A mask measures changed bytes, not stores of identical bytes.
- Controlled observation fields: sat, telID, clockCorr, nclock_correction, correctionsTT, obsNjump, jump[]. Controlled pulsar fields: nobs, obsn, clockFromOverride[64], clock, noWarnings, nJumps, jumpSAT[], jumpVal[], jumpStr[]. Other storage is explicitly zeroed and never asserted to reflect native initialization. No fields are fabricated after invalid access or termination.
- `toa2utc` uses `jumpSAT[l]`, not `jumpSAT[k]`; jump insertion writes new count itself. Initial count 28 plus one jump writes index 29 within storage (count 29). Initial count 29 plus one jump writes index 30 outside the array, despite a nominal count of 30. A disabled clock pass retains the previous count and slots before insertion. SAT labels use snprintf with size 31 (at most 30 text bytes); shifts use strncpy of 31 bytes and leave byte 31 untouched. Clock fills instead use the full 32-byte destination size. Full initialized destination bytes are compared.
- Logging: raw `TKlog.C/h` records counts and up to 30 strings; stdout non-TTY strips ANSI; raw `tempo2Util.C:362 displayMsg` suppresses repeated warning keys process-wide (DUP1 once). noWarnings=2 suppresses type 1/3, never type 2. Negative settings print type 1/3 directly. Snapshots include counts; complete stdout/stderr remains ordered per action. `displayCVSversion` is controlled separately.

## Dependencies and compiled profile

`observatory.C` is compiled raw: `getObservatory`, `lookup_observatory_alias`, `initObservatories`, `readObservatoryFile`, `readAliases` supply actual clock-name lookup, alias warning/state and input resources. Its Cartesian conversion runs when loading real/synthetic observatory files, but geometry accuracy, geodetic rejection and general observatory coverage are not claimed. Raw `TKlongdouble.C` supplies parse/display, `TKlog.C` actual diagnostics, `tempo2Util.C` actual displayMsg. `global.C` supplies CVSdisplayVersion. The data declarations at `global.C:1-64` and `CVSdisplayVersion` at `global.C:176-200` are concatenated verbatim into `global_data.C`, excluding unrelated timing wrappers; both slices and the resulting translation unit are hashed. No A2 archive object is linked. Its hash and symbols are checked solely as independent profile evidence; only its three-line generated config is reused. The A2 initializer correction and resource_fopen instrumentation are never used. Link maps, compiler/runtime-library hashes and exact dependency hashes are retained in build/ and the manifest. macOS libSystem comes from the system dyld cache and is identified by host/ABI version, not a standalone on-disk hash.

`toa2utc.C:35` unconditionally defines USE_NEW_CLK_CORR: the old wrapper/convertTOA/toa2utc_nist/linearInterpolate bodies are excluded. `tai2tt.C:36` and `utc2tai.C:37` use literal `#if 0`. `profile.py` checks preprocessed definitions and the accepted archive's defined symbols (declarations alone do not count). `readJBO_bat.C:44` is active, confirmed both ways, and deferred to the input-format stage. These inactive bodies are not reactivated. Table linear-scan `#if 0` is also excluded. No configurable source branch is labeled disabled merely because a fixture does not reach it.

F ledger mapping: B07/B08/B09/B10 table fixtures; B12 fallback/empty sum; B13 weight/route fixtures; B15 clock index/table/sequence lifetime and observatory/logging dependencies. B11's EOP indexing is outside this table/clock stage: no EOP result is measured or closed. B15's weather/IFTE/TT-to-TDB states remain outside scope.

## Execution evidence and open boundaries

`outputs/coverage.json` is generated by `coverage.py` with an otherwise identical GCC binary128 profile at O0 plus gcov. It records per-case execution counts for all 32 primary entries: four table routines, four public clock routines, 18 helpers, three constructors, two lambdas and active toa2utc. Counts include read-only harness snapshots (notably table start/end getters); they establish actual function/case membership, not complete branch or scientific validation. The O2 build remains the numerical oracle. Unexecuted source lines and dependency-function counts are retained rather than hidden.

Nine invalid-access/cycle fixtures remain catalogued but unrun following the central message reporting an automated possible-cybersecurity-risk flag, without identifying the triggering experiment. No sanitizer, alternate numeric configuration, or alternate execution path was used to retry them. `outputs/isolated.json` states this disposition explicitly; it contains no numerical or sanitizer result. Ordinary process-exit fixtures are executed in separate subprocesses, preserve the exact last action, exit code and stdout/stderr, and cannot be treated as same-process recovery.

Long full paths and the host's maximum 255-byte basename are measured separately. A larger basename cannot be created on this host; no fabricated file API or source repair is used to bypass that filesystem limit. Allocation exhaustion, glob read failures and malformed inputs beyond the catalog remain source-inspected or unexecuted. B11 EOP invalid access is not exercised by this assignment.

The coverage build leaves clkcorr.C lines 219–226 (glob allocation/read errors), 497–498 (invalid predecessor safeguard), 866–867 (public accumulator rejection of a broken cached chain) unexecuted. The broken helper itself is exercised with controlled arguments. Line 777 compares the literal fallback names UTC and TT(TAI), so its return is unreachable in this profile. Lines 133 and 238 are compiler-associated closing/return locations without executed counters, not additional APIs. Full line execution of table/toa2utc does not cover their invalid-access branches; those remain withheld.
