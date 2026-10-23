# Source behavior and limitations

Assignment K, canonical `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. These are compatibility findings, not proposed corrections. Rust was neither changed nor evaluated here. F's B01/B18/B19/B20 remain historical baseline findings; A2's solar-wind initializer fix is not part of this build.

## State boundary

`initialise.C:777–846` allocates **all 2,000 parameter slots**, including enum-marker/virtual and unnamed slots. It sets sizes, zero link counts, zero value/error/set/fit arrays and zero-filled label buffers. It does not write any `prefit` or `prefitErr` array cell. `initialiseOne` fills labels and ordinary scalar defaults but does not write `ne_sw_ifuncN` or `nchromx`. The original commented-out observation-vector initialization remains inactive. No observation payload or padding is compared.

The initializer experiments supply only two declared caller markers (`ne_sw_ifuncN=117`, `nchromx=119`) before calling the source. Their retention proves a write-mask fact, not a default. Natural parameter snapshots omit unwritten fields. Both `fullSetup` values and both `realloc` values are exercised with valid matching lifetimes; their argument names do not imply conditional allocation. The bounded global `initialise` experiment sets the real `MAX_PSR=2`, `MAX_OBSN=2` and initializes both pulsars. The compile-time `MAX_FIT=10000` remains unchanged; real `malloc_blas` allocates its covariance storage. No successful allocation stub is used.

Parser experiments explicitly seed allocated `prefit=17`, `prefitErr=19`, coordinate strings, clock/ephemeris strings, `ne_sw_ifuncN=117`, `nchromx=0`, `waveScale=23` and telescope-velocity selector fields to 7 after canonical initialization. Other fields retain source defaults. Every action sequence and subsequent prior override is in `cases.json`. These are **controlled parser experiments**, not a claim that a newly malloc'd ordinary pulsar has those values. Reader scratch allocations are the source's own, and defined ecliptic fixtures supply value, fit flag and uncertainty to avoid reading unwritten scratch fields.

Snapshots compare every parameter cell for the complete enum plus selected initialized/caller-supplied scalar fields, strings, links and real TK logging counters. Initialization additionally exports every allocated slot. `changes` is a before/after **value-change mask**; it cannot distinguish a store of an equal value from no store. The source write sets below disambiguate those cases. No bytewise comparison of objects, addresses or padding is used. In the separate `ecliptic_changes` namespace, the reused serializer roots `param_raj` and `param_decj` denote scratch `elong` and `elat`, respectively; these are not writes to the pulsar RA/DEC arrays. Non-parameter initialized arrays are source-inventoried rather than exhaustively serialized.

Matching `destroyOne` and `destroyMemory` calls return successfully in the probes. This is not a leak-sanitizer result. Source inspection: `readSimpleParfile` leaks its temporary ecliptic arrays/labels; `readParfile` frees its scratch; repeated `initialiseOne` or `allocateMemory` on live allocations overwrites pointers and leaks storage. Destroy functions do not null freed pointers, so double destruction/reusing the freed arrays is invalid. Allocation failures and invalid lifetimes were not executed.

## Numeric grammar and partial writes

`TKlongdouble.C:58–74` uses `strtoflt128(str,NULL)` in this build. Numeric prefixes, hexadecimal input, whitespace, signed zero, NaN/Infinity, underflow and overflow are source behavior. No whole-token validation is added. `1D3` is 1 at this layer; `readValue` changes D/d to e in the value and the selected uncertainty token first. An invalid numeric prefix such as `junk` yields zero at this helper layer; this does not make every malformed PAR record safe.

`readParfile.C:2762–2944` calls `fgets`, then scans at most three whitespace tokens. Surplus tokens are consumed with the rest of the line. One-token input leaves prior `prefitErr`; two-token exact `0`, `1`, `2` sets a flag and also leaves prior `prefitErr`. Spellings `1.0`, `+1`, `01` are uncertainties. Three-token input scans an integer prefix from token two after resetting `fitFlag` to zero; `7junk` gives 7, while a failed integer scan leaves zero. Negative uncertainty is retained. `err` is reset to zero independently of `prefitErr`.

Coordinates use **binary64** `hms_turn`/`dms_turn`, `turn_deg`, `M_PI`, and arithmetic before assignment into binary128. `hms_turn` replaces colons in its input buffer; `dms_turn` copies into a 40-byte local buffer and removes the first minus sign wherever it appears. Partial conversions and the one-subtraction degree wrap are preserved. Blank input gives zero because the scan returns EOF, whereas a failed nonblank conversion returns one turn. Fixtures are bounded below the local buffer capacity.

RA uncertainty is divided by `12*60*60/M_PI`; DEC uncertainty by `180*60*60/M_PI`. A duplicate coordinate without an uncertainty divides the **previous, already scaled** uncertainty again. A negative RA writes the coordinate strings and `val`, then returns `-1` before resetting flags/errors or assigning `paramSet`/`prefit`. `checkLine` ignores this returned status. No attempt was made to execute the unreachable wrap-around code after the return.

P/P0 inverts the value, but the ordinary reset `err=0` makes its error transformation zero for finite values; `prefitErr` is not transformed. P1/Pdot before F0/P0 writes `val`, resets fit/error state, may set `prefitErr`, then returns `-1` without setting `paramSet`/`prefit`. With F0 present, expression order is preserved, including the binary64 literal `1e-15` and the unusual squared error expression. The controlled nonzero F0-error experiment observes it. P0=0 produces Infinity and NaN through defined floating arithmetic. EPHVER accepts only exact case-insensitive `2`, `5`, `TEMPO1`, `TEMPO2`; unknown `3` exits 1 at that action.

Finite binary128 output uses 36 significant decimal digits and asserts a same-backend round trip including zero sign. Binary32/binary64 serializers retain bits and class. This scalar/coordinate case set actually uses binary128 and binary64; binary32 compound directives remain deferred. NaN payload bits are not exported for binary128. No Rust representation or tolerance decision is implied.

## Dispatch and stream contracts

`inventory.json` lists the entire ordered top-level chain (339 branches), all nested if conditions, scan expressions, source anchors and parameter mentions. `DISPATCH.md` assigns every top-level branch a disposition. `line_coverage.json` gives the actual GCC condition edges/line counts; it is not path completeness. A keyword appearing in the catalog or as an attempted token does not establish that its later branch executed.

Every exact-match scalar `readValue` alias is attempted in a fresh controlled case, including aliases behind earlier branches. All safe indexed families have valid first/last cells and, where a guard exists, a rejected upper-bound token. Each stream token records its pre/post stream positions, the next token and changed parameter/index. `fscanf` controls may leave same-line tokens for dispatch; `readValue` consumes the line. Unknown tokens consume only their keyword and warn; subsequent tokens are dispatched independently. Uppercase first-character C/I comments consume the rest of the line; lowercase c/i does not get that fallback. Hash comments are separate. EOF without a final newline is exercised for a value line. `removeCR` removes only one terminal LF, retaining CR; empty input would access before the array and is not executed.

Selected source-specific distinctions:

- Higher F derivatives use a broad first-character F/f test that excludes only uppercase B in position 1. Thus lowercase `fb1` is intercepted before the later uppercase-only FB branch and does not assign FB1; `fB1` also does not reach the uppercase-only FB branch. `F1junk` accepts integer-prefix 1.
- Glitch/exp/Gauss families often use `strstr` for all-uppercase or all-lowercase spellings, then scan at a fixed offset from the **start**. They are not general case-insensitive prefixes. Their `gval < aSize`, followed by index `gval-1`, excludes the final allocated element. No lower-bound guard is provided.
- GLF0D2/GLF0D3 and GLTD2/GLTD3 mistakenly reuse the lower-case first-family spelling. Those alternatives are shadowed by earlier branches; `glf0d2_1` is not equivalent to `GLF0D2_1`.
- NUdot families use case-sensitive substring tests and fixed offsets 10/11/12 for AMP/TIME/EPOCH. Their strict `<32` guard accepts index 31 (array cell 30) but excludes the final allocated cell. These are cataloged separately from the more permissive case handling of other families.
- FD uses the **frequency** array size as its upper guard. FD1 and FD9 are valid; FD10–12 can access beyond the nine-element FD allocation. FD13 is rejected. FDJUMP's nested grammar is a separate deferred feature.
- FB has **no** range or successful-scan check. FB and FB0 select zero; FB19 is the last valid array element. FB20 is not an ignored upper boundary. An initially over-broad test attempted it and ended in SIGBUS; the preserved diagnostic is excluded from ordinary goldens. No portable numerical value is assigned to it.
- `A2DOT` writes `param_a1dot[1]`; `X2DOT` is handled earlier and writes `param_a2dot[0]`. The later X2DOT alias in the A2DOT branch is shadowed. PBDOT/XPBDOT/A1DOT/XDOT/EPS1DOT/EPS2DOT/EDOT convert large-magnitude values with strict source thresholds; uncertainties are not subjected to that value conversion.
- TEMPO1 assigns multiple controls in order. UNITS sets `setUnits` even for an unrecognized unit. EPHEM uses case-sensitive value branches for DE414/INPOP06 while the keyword is case-insensitive. EPH_FILE selects CALCEPH and can force the T2C method. No ephemeris is opened by these parser-only tests.
- The canonical label of `param_daop` is overwritten to IPERHARM. A registry must not infer parameter identity solely from its label.

## Links and defaulting

`readValue` creates directional, count-limited link records. `getParameterValue` (`tempo2Util.C:426–472`) evaluates only the specified stored source values; it is not recursive. All five pairs are probed before/after their targets, including repeat links and the five-entry array capacity for SINI. More than one link causes exit 1 when evaluated. Repeating DSHK reaches that exit inside `readValue` itself because it immediately asks for the linked prefit.

| Link | Source write set and evaluation |
| --- | --- |
| DSHK PX | Append DSHK's linkTo and PX's linkFrom; copy current PX into stored DSHK `val`; set DSHK `prefit = 1/PX`; set flag. Linking before PX yields stored 0 and prefit Infinity; setting PX later makes the getter return 2 for PX=0.5 without updating that stored state. |
| SINI KIN | Append links and set paramSet only. Getter uses `sin(stored KIN/180*M_PI)`. It does not recursively follow KIN STIG. |
| KIN STIG | Append links and set paramSet only. Getter uses `asin(2*STIG/(1+STIG*STIG))*180/M_PI`, in source order and overloads. |
| D_AOP PBDOT | Append links and set paramSet only. Getter uses PBDOT, PB, PMRA, PMDEC, SPEED_LIGHT/SECDAY/MASYR2RADS/PCM exactly as written. Inputs are explicit. |
| DMMODEL DM | Append links and set paramSet. Exactly two tokens with exact flag 0/1/2 also set fitFlag. Getter reads stored DM. The dispatch resets DM/CM sample counts. |

Ordinary numeric assignments after links do not automatically remove links. No general dependency graph was invented. `copyParam` copies arrays, label strings and `aSize`, but does not copy link arrays/counts. The direct copy case and epoch-finalization cases preserve complete source metadata, including fit flag and uncertainty.

`checkAllSet` requires PEPOCH. Missing POSEPOCH and DMEPOCH copy PEPOCH metadata and relabel the target. Missing wave epochs copy PEPOCH only when the corresponding wave parameter is set. Explicit epochs survive. Complete ecliptic coordinates become RA/DEC radians using source arithmetic, but their errors are not copied; if either input fit flag equals 1, both output fit flags become 1. Flag 2 alone does not trigger this rule. A single ecliptic coordinate exits 1; tested full/simple wrappers use complete or deliberately partial records with initialized scratch fit/uncertainty inputs.

Clock remapping is case-sensitive: UNCORR also sets `clockFromOverride=UTC`; UTC(NIST), UTC(BIPM), PTB and AT1 have separate mappings. Lowercase versions fall through to TT(TAI), whereas any string beginning with uppercase TT bypasses the non-TT warning. Actual warning history is retained across repeated actions; duplicate suppression is visible, including BINFB1. DDGR with any FB set exits 1. For ELL1 with FB1 and PBDOT, PBDOT's set and fit flags become zero while stored numerical fields remain.

## Wrapper and fixed-header distinctions

| Routine | Source behavior observed |
| --- | --- |
| `setupParameterFileDefaults` | Writes clock TT(TAI) before checking TEMPO2; missing environment returns -1, leaving previous ephemeris strings. Success constructs the DE405 path. |
| `readParfile` | Resets phase-jump/jump and link counts per pulsar. Does not erase all previous parameter values/set flags. Resets ecliptic scratch flags per pulsar. Normal text input installs clock/ephemeris defaults, dispatches, then finalizes. Missing PAR can fall back to TIM headers. |
| `readSimpleParfile` | Uses the caller's FILE and existing defaults/state; does not perform full-reader resets/default setup. Dispatches and finalizes. Complete ecliptic input works for the bounded records despite its stale source comment. Leaks local scratch storage. |
| `readParfileGlobal` | Reopens **tpar[0] for every pulsar**; only performs binary consistency after parsing, without ordinary epoch/clock finalization or full-reader resets. Missing file exits 1. No safe global ecliptic/PMRA/PMDEC claim: temporary ecliptic parameter pointers are uninitialized. |

Repeated full/simple/global parsing and two-pulsar full/global calls are recorded. The global fixture deliberately supplies different second-file text; both objects receive the first file. No caller wrapper clears warning globals between actions.

Full reader `noread` is outside the pulsar loop. After a fixed-format first pulsar sets it to 1, a valid second PAR is opened then skipped: its F0/PEPOCH remain unset. After a HEAD first pulsar sets it to -1, a later ordinary PAR also terminates on TOAS. The TOAS token is still dispatched to `checkLine` before the outer loop ends. These deterministic cross-pulsar effects are preserved.

Fixed headers use bounded source-width strings, fit bits, position fields, P0/P1/P2, JD-to-MJD, proper motion, parallax and DM. The implemented fixed path sets DE200 path, fixedFormat=4 and POSEPOCH, then skips ordinary finalization. Binary values after the fixed header are explicitly not implemented by this source path. `getValue` uses its own fixed substring offsets and does **not** replace D exponents; direct fixed `1D3` gives 1. Blank fixed fields leave state unchanged. All these observations use supplied prior metadata; no missing fixed-header metadata is fabricated.

## Undefined, deferred and unrun cases

| Case / precondition | Disposition |
| --- | --- |
| Fresh `prefit`/`prefitErr`; fresh `ne_sw_ifuncN`, `nchromx`, `nWhite_dm`, waveScale and other absent scalar writes | Source-unwritten. Canonical registry avoids reading them. Controlled markers are named separately. Natural first coordinate with no uncertainty, or PEPOCH default copying an unwritten uncertainty, has no portable numeric golden. |
| Global reader ecliptic keyword, or PMRA/PMDEC guard reaching temporary elat/elong pointers | Invalid/uninitialized-pointer boundary, source-inspected; not executed. |
| Empty `removeCR` | Before-buffer access, source-inspected; not executed. |
| readValue missing value, exhausted fgets, overlong token; fixed header too short; coordinate local buffer overflow | Incomplete scan/invalid access boundary; not executed. Input lengths here are bounded. |
| FB20 | One initial diagnostic run terminated SIGBUS; raw partial events and executable/source hashes preserved in `diagnostics/fb20.json`. No successful output, portable exit guarantee, or sanitizer diagnosis claimed. |
| FD10–12, negative/zero one-based indices, unchecked FB parse failure, overflowing integer scans, link capacity >5 | Invalid index/indeterminate/overflow risks, source-inspected; not executed. |
| Clock-chain directives, sampled DM/CM/SW/wave tables, jump/FDJUMP/phase selectors, constraints, noise events, compound GW/telescope/orbital records | Explicit deferred rows in DISPATCH.md/coverage.json. Real support is linked; no successful no-op replaces it. J owns clock graph/cache reference. |
| TIM grammar and preprocessing, epoch updates, sorting, fitting, timing/model evaluation | Outside K. G fixture reuse validates parser inputs only. |
| Linux/other CPU/other longdouble backend, leak/ASan/MSan runs, allocation failure, long-run resource exhaustion, concurrent shared-pulsar use | Not run. The four-worker runner uses independent processes; it is not a source thread-safety test. |

No experiment was blocked by tool safety review. Undefined operations were generally classified instead of executed; the one unintended FB20 crash is explicitly retained. An exit ends that process: later cases start fresh processes and do not demonstrate same-process recovery.
