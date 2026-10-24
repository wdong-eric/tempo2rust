# Assignment U source and caller contract

Release `43001af`, decision 0046; raw source pin
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`. Only this directory is owned.
The frozen local `outputs/frozen/source_map.json` contains all 22 function
bodies with line ranges, every source decision/preprocessor line, all raw
repository call/declaration occurrences (including plugins), and S's full
349-pulsar/95-observation-field type map. `inventory.py` regenerates and checks
this map without executing native code. Generated evidence stays ignored.

## State, types, ownership and callers

| Source/routines | Reads and writes; lifetime and caller contract |
| --- | --- |
| observatory.C:68–147 readObservatoryFile | Appends `observatory`: six binary64 x/y/z/longitude/latitude/height cells; name[32], code[16], clock_name[16]. Local record survives between rows. Scan-written prefixes through NUL and retained earlier tails are known; never read natural unwritten suffixes. Five fields synthesize a truncated clock string. Geometry changes only the six numeric cells. |
| :150–192 readAliases | Local code[100], vector<string>, warned bool. Vector accumulates across lines, then copied into the process vector in file order; code tails survive earlier lines. A new call has a new temporary vector but appends global lists. |
| :197–243 initObservatories | Reads selected TEMPO2_ENVIRON root, main file before glob-sorted extras, then TEMPO2_ALIAS or aliases. Appends both vectors; sets initialized only after successful completion. No reset. |
| :245–283 lookup_observatory_alias | Lazily initializes; first case-insensitive match; one warning per matching list. Separate output copies original when unmatched; identical input/output preserves tails. Alias match copies from disjoint list storage. |
| :286–317 getObservatory | Alias lookup first; first code-or-name match returns borrowed vector-element pointer. Reacquire after any append; no saved pointer dereference after growth. Unknown code exits. Its second initialization branch is unreachable after successful alias lookup. |
| :368–381 fang | Two representable integer conversions; modes >2 multiply by 15; modes 1/3 convert fraction to radians; no normalized carry or sign repair. |
| :394–427 ITRF_to_GRS80 / GRS80_to_ITRF | Caller-owned initialized observatory, exact GRS80 constants and expression order. First writes height/latitude/longitude, second x/y/z. y>=0 longitude branch includes negative zero. IEEE exceptions are values, provided no downstream integer conversion occurs. |
| eop.C:55–135 load_EOP | Appends caller vector of four doubles mjd/xp/yp/dut1; resets local format each call. Fixed offsets require a fully initialized 50-byte line. All scans must succeed before append; xp/yp arcsec→rad. Duplicate/unordered/nonfinite rows are load-only. |
| :138–203 get_EOP | Private static first flag/vector, first successful load retained across filename/root/file changes. Writes derivative then xp/yp/dut1 after optional proxy writes. Four initialized output doubles may alias in declared histories. Valid bracket requires at least two ordered rows and query strictly below last time. |
| tropo.C:49–215 NMF_hydrostatic / NMF_wet | Double inputs/results; finite representable latitude integer conversion. Latitude interval/clamp, southern seasonal sign, source 53398 epoch/365.25 year, height term, and mixed wet b/c coefficient unchanged. Zero/nonfinite elevation, height or date may produce defined IEEE output; latitude remains conversion-safe. |
| :226–255 MeteorologyFunction wrappers | `TabulatedFunction` owns fileName[256], header_line[256], vector of x/y doubles; siteName[256]. Direct caller initializes character backing. Loader replaces samples but leaves unwritten/retained character tails; getValue uses real binary search, endpoints use real front/back. At least two samples for in-range interpolation. |
| :258–312 initialization pair | Clear/load each kind in glob order, zwd before sap; set shared initialized flag after both complete. Local `func` reused between files; logical strings/samples exported without unknown suffix reads. Explicit reinit replaces table vectors. |
| :314–363 lookup / wet / pressure | Lazy shared initialization; first case-insensitive site and inclusive time range wins. No match is +0. Wet zero emits TROP1 and stays zero; pressure zero emits TROP2 then 101.325. displayMsg warning key state persists process-wide. |
| :367–462 compute_tropospheric_delays | Signed npsr/nobs loops, per-pulsar/per-call warned=false. Always resets visited delay. First eligible zero zenith skips correction; later zero zenith can calculate. Exact STL_FBAT bypass changes warned even when correction flags are off. COE name bypass. Uses SAT for tables, SAT+real clock correction/SECDAY for hydrostatic season; source dot/height/asin, pressure/height/latitude expressions retained. |

Production callers are get_obsCoord.C, calculate_bclt.C, clkcorr.C,
preProcessSimple.C, readTimfile.C, polyco.C and selected plugins. The frozen
occurrence map distinguishes their locations from primary internal calls and
declarations. No production caller is accepted by merely linking its dependency.
The harness includes unchanged primary files to observe addressable statics;
it never exposes/resets get_EOP's private cache. Snapshots call no mutating helper.

Integrated callers use two value-initialized pulsars and twelve observations,
six owned slots per pulsar. No source initializer executes. Explicit field
assignments and zero initialization completely define logical inputs. Generated
typed equality checks cover every S/R field, excluding only the expected delay
write; they compare scalar representations, array elements and pointer identity,
never padding or pointee storage behind null pointers. Counts are bounded by
backing; negative/zero counts intentionally visit no elements.

## Pre-experiment safety and evidence classes

* **Native-invalid, unrun:** EOP final sample or beyond (proxy writes are a
  source-inspected prefix before samp[size]); empty/singleton/private malformed
  brackets; initial partial observatory numeric rows reading unwritten z;
  short fixed-format EOP rows; overflowing/unclosed scanner strings, table header
  >255 bytes, invalid NaN/infinite/out-of-range latitude/fang integer casts;
  meteorology GLOB_NOMATCH with nonzero warnings when host gl_pathv is null;
  stale observatory pointers; empty table access and singleton in-range query.
* **Defined ordinary:** fully initialized bounded fixtures, failed scans that exit
  before consuming unknown values, retained partial observatory rows after a
  safe full row, duplicate/nonfinite load-only EOP, initialized double output
  aliasing, IEEE results with conversion-safe inputs, signed empty loops.
* **Host-qualified diagnostics:** unchanged TKlog color stripping forms a
  before-buffer pointer when a leading ANSI code is removed. Keep raw measured
  output under the accepted J/M host qualification; do not call it portable C++
  semantics. Filesystem errno/glob/libSystem and locale are host observations.
* **Host-dependent / still-defined-unverified:** allocation/read failures,
  alternate platforms/locales, concurrent global state and arbitrary I/O races.
* **Inactive:** observatory test loop :125–139, convertGeocentric :321–366,
  tropo mapping test :381–398, table linear-search #if 0; never reactivated.

No nominal success is fabricated for an invalid prefix. Each terminating case
runs in its own process; a subsequent fresh success is not source recovery.
An ordinary caller `atexit` observer records the live global vectors and
caller-owned EOP/direct meteorology objects after a source exit, before their
destruction. It does not replace exit, reset native state or read native local
stack temporaries. Unreturned local scanner state remains source-inspected.

## Exact comparison and preservation

Finite binary64 values compare exact bits (including signed zero); nonfinite
values compare class. Binary128 inputs retain all 16 representation bytes.
Ordered events, fields, arrays, stream bytes and return codes are exact.
Raw streams are retained. Only declared source/build/resource root prefixes may
be projected, before exact comparison; no line, numeric, warning, substring or
general-address scrubbing is allowed. Any observed logger truncation stays visible.

Fresh builds establish actual compiler, flags, source/header/configuration,
dependency closure, runtime, caller, generated input/resource and executable
identities, checked again around execution. Ordinary verification cannot record
or replace its oracle. Every failure removes the success receipt. Mutation runs
must show behavioral differences independent of identity. Coverage runs compare
with ordinary results and report any differences without adding a tolerance.

`outputs/frozen/protected.json` excludes U, T's rust/ and
validation/pulsar_lifecycle_port/, V's validation/time_ephemeris/, central documents and volatile build caches.
It includes accepted source/projection/local evidence files; both independent
repositories retain their original HEAD and clean status. Sources/fixtures/
contracts/reports alone are committed using explicit owned paths.
