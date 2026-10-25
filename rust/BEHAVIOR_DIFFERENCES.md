# Preservation and boundary ledger

Assignment E preserves defined TEMPO2 results, including deterministic bugs.
The pinned source is `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. Numerical
representation follows the already adopted Rug128 contract. The original E policy deferred fixes; decisions 0053/0058 authorize the
demonstrated W/Y corrections below. Central review owns acceptance.

| Behavior / source | Reference evidence | Rust behavior and adjacent explanation | Disposition |
|---|---|---|---|
| Square `n=p` rescaling, `TKfit.C` error multiplier | `reference/square_rescaled.json`, `timing_square_true.json`: NaN errors; unscaled pair finite | `tkfit.rs` keeps 0/0 and error multiplication; `FitNumber` emits `NaN` | Preserved source bug, no preemptive square-fit rejection |
| Error callback `error >= 0`, `t2fit_stdFitFuncs.C` | `callbacks.json`: NaN/-Infinity retain prior errors, +Infinity updates; timing square leaves F0 error zero despite supplied PAR uncertainty | `std_freq.rs` keeps the predicate; source current errors initialize to zero while supplied uncertainty stays metadata | Preserved; no finiteness substitution |
| ZERO error storage never initialized before a NaN callback | `timing_square_true.json`: exporter intentionally does not read `offset_e` | `t2fit.rs` exposes `offset_error_s=null` and `offset_error_state=undefined source storage; no value` | Explicit undefined-state adapter boundary; null is not NaN transport |
| ZERO estimate is assigned, not accumulated or applied as a new timing delay | Actual `t2Fit` cycles in all timing fixtures | `t2fit.rs` stores each estimate, recomputes timing solely from updated spin model and mean removal | Preserved |
| `n < p` returns without setting estimates, `TKfit.C:188` | `boundaries.json`: initialized sentinels remain untouched after source return | `tkfit.rs` returns contextual Err; no fabricated pseudoinverse/output | Undefined-output boundary, not an exact successful-result claim |
| Zero/tiny weights, source `TKsvd.C:65` can index `w[-1]` after nonfinite arithmetic | Isolated low-level calls SIGSEGV; timing calls eventually print 30-iteration exit after entering nonfinite SVD state (`boundaries.json`) | `tksvd.rs` stops at the invalid-index boundary and returns Err; debug/release library and CLI recovery recorded | Observable failure-stage/termination difference. No numerical source output exists to preserve |
| Effective iteration-30 termination despite `max_its=40`, `TKsvd.C:109` | Source timing-weight probes reach the message; preceding nonfinite state may already have undefined access | `tksvd.rs` retains iteration order and returns Err at the same explicit limit if reached safely | Library termination adapter. A finite matrix exhausting 30 iterations has not been demonstrated; do not claim that branch was independently numerically exercised |
| NaN matrix entry terminates, `t2fit.C::t2Fit_buildDesignMatrix` | Pinned source branch | `t2fit.rs` uses the same NaN predicate and contextual Err with row/cycle | Termination boundary; no clipping, finite floor or alternate solver |
| Defined covariance/increment narrowing overflow | `infinite_covariance`, `infinite_increment`, `infinite_rescaled_error` source fixtures | `tkfit.rs` preserves +Infinity covariance/errors/increments, -Infinity linearized residuals and NaN rescaled error; exact class comparison | Preserved; bounded public timing input may fail later at existing undefined phase/count boundaries |
| Strict `w < binary64(1e-27)*max(w)` cutoff and pseudoinverse | below/at/above controls, zero-column/all-zero/dependent matrices | Same strict test, zero-mode back substitution, unscaled covariance, source `n-p` | Preserved; no rank rejection or replacement DOF |
| Binary128 versus approved Rug128 changes ordering of dependent singular modes | Central decision 0010 and raw dependent-column artifacts | Production raw mode order unchanged; comparison pairs value/flag records, checks each engine's exact binary cutoff and records its permutation | 0010 defines paired diagnostics, but the latest direct user instruction keeps the exact raw-order gate unresolved; no algorithm/precision change |
| Native C++ long double, `TKsign_d`, `TKretMax_d` are binary64 on reference host | Central probe and source calls | `std_freq.rs` and `tksvd.rs` intentionally narrow those boundaries; main extended state remains Rug128 | Required accepted profile; Linux behavior unverified |
| Duplicate MODE/WEIGHT sequential assignment | `subset.par` includes three declarations | `read_parfile.rs` last declaration wins, all original declarations remain | Preserved |
| Unsupported controls/fit selections | MODE outside 0/1, nonpositive NITS, global or non-spin active flags | Explicit profile errors; fixed evaluation still accepts its pre-existing fit-flag metadata | Coverage boundary, not a claim that upstream rejects all these inputs |

Original M1 adapters outside JPL remain covered by their historical evidence and
F's fixed-baseline inventory. Assignment I changes only the released JPL scope;
M1 historical acceptance does not establish exact whole-source fidelity.
`M1A_REPORT.md`, `M1B_REPORT.md` and `validation/coverage/` remain unchanged.

Historical E verification used fresh runtime contexts. Under L, independent
fitting fixtures now use fresh processes, matching isolated reference processes. A preliminary verifier that reused a runtime from the
54000-centered fixture for the original seven-TOA fixture selected a different
cached clock chain at early epochs. Correcting the verifier's experiment setup
restored raw forward agreement. This observation is not proof of a newly found
Rust/source difference under the same call history; cross-profile history
comparison was untested at that delivery. L now restores process-lifetime
clock caches; the owned fitting driver isolates cases without a reset mode.

## Assignment I JPL dispositions (decision 0017)

| Source behavior | Current Rust disposition / evidence |
| --- | --- |
| U1 validate bytes then reopen path | Removed: runtime hashes/rewinds/transfers one File. Owned test replaces the path after hashing and proves original AU is consumed; reopen sees new AU. Digest observes a pass, not concurrent-write snapshot consistency. |
| U2/B29 packed AU/EMRAT getters | Decoded fields selected safely; isolated H getter commands return those values without unaligned access. |
| B03 cache before seek/read | Restored. Exact H raw cache bytes, statuses, I/O counts and retry observations pass; prior corrected-cache test and native patch removed. Fix deferred until overall transfer. |
| Unwritten Sun/recurrence state | Explicit absent Sun and initialized recurrence storage. First demand computes Sun. H control supplies its sentinels separately. Natural-init status is compared without invented C fields. |
| Transient ipt[15]/pv[14] pointer formation | Actual row/destination selected before indexing. Full calls match H host observations; C UB is not emulated. |
| Failed ordinary pleph reads unwritten local pv | Return source status and earlier mutations without a numerical vector. Timing adapter returns contextual error. |
| Position-only EMB copies unwritten velocity | Read only requested components; public source-cleared suffix remains zero. |
| Missing/unvisited private pleph body row | Explicit `pleph_unwritten_row` boundary; no fabricated zero scientific vector. Direct state retains caller storage for unvisited requests. |
| Optional direct demand flags | Preserve unsigned list assignment and signed interp flag: >1 requests velocity, exactly 3 also requests acceleration, negative flags request position. Optional rows skip AU scaling; 32 canonical host controls cover indices 10–13 and extreme signed flags. Planetary/Sun bounds use the source-width quantities*3 and actual storage. |
| nut[4] and direct acceleration demands | Initialized destination slices sized to actual dimension/demand. Four-slot libration rejects; nine-slot direct body/libration acceleration tested. |
| Partial fread elements | Raw initialized bytes retained, including old suffix; exact same-host byte comparison, not portable C numerical meaning. |
| NaN/tiny-step record conversion, bad coefficients/counts, title overflow, invalid selectors | Distinct operation-local Boundary errors. No broad low-level finite/AU/grid restrictions. Unsigned record-offset and kernel arithmetic retain source wrapping. |
| Defined nonfinite source arithmetic | Exact NaN/+Infinity/-Infinity classes preserved at low level; high-level nonfinite timing-vector rejection remains a documented adapter. |
| Global initialization code and live -7/-8 handles | Atomic global sequential status, startup -9, reset on init, unchanged by Drop. Live partial-init handles retain caller mutations and close normally. No C global-race equivalence claimed. |
| C pointer ABI and null/double close | Owned non-cloneable stream, bounded optional Sun view and field selectors. No packed Rust ABI, arbitrary byte-offset getters or raw handle API. |
| Pre430 or ncon400 unwritten temporary extension words | Preserve first-comparison mismatch clearing. If the computed row-12 end equals UINT32_MAX, reject with `unwritten_extended_header` before the second comparison consumes unread row-13/14 operands. Rust-only controls cover both routes and byte orders; valid small/large extended headers remain accepted. |
| Unterminated title, including all-whitespace H title_zero | Decision 0019 requires `title_conversion` when whitespace/sign/digit scanning reaches byte 84. Startup compares; open is a boundary; control/state/close remain explicitly accounted as unreachable. In-bounds no-digit zero and complete C-locale whitespace are preserved. |

All 12 implemented responsibilities and each H event are covered by the projection.
The ten isolated source cases are classified separately in I's evidence. No
sanitizer output is used as a post-failure numerical oracle. Platform limits and
untested malformed branches remain in `JPL_REPORT.md`; central owns acceptance.

F B06 documentation correction: `ifteph.C::IFTE_init` does not check its header
read counts. `IFTE_get_Vals` checks data seek/read outcomes and exits on a short
coefficient read; upper dates must not be described as unchecked data reads.
At that historical F follow-up only the source-description comment changed.
Y now removes the broad IFTE range/grid/image adapter and preserves the full
source lifecycle, with checked external-operation boundaries below.

## Assignment L clock/table dispositions (decision 0023 and user contraction approval)

| Source behavior | Current Rust disposition |
| --- | --- |
| Permissive numeric prefixes/hex/nonfinite input, failed rows, duplicates/decreasing rows | Preserved by typed libc scans and source chunk/order rules; stricter parser removed |
| Defined zero-width IEEE arithmetic and endpoint proxies | Preserved; no zero-width rejection. Final interpolation multiply/add alone uses user-authorized source-matched contraction |
| Directed binary32 weights/distances, unusual weights, strict ties, retry and reverse hints | Preserved; negative/finite guards removed; predecessor-cycle experiment remains unrun and algorithm unrepaired |
| Four-stage stored-chain fallback versus UTC-only accumulator | Preserved; exhausted lookup normally returns zero, earlier slot writes survive failed fills |
| Index/table/sequence, observatory and warning history lifetime | Process-wide synchronized session; new context/root does not reset. Header and sample identities retained separately |
| Darwin glob membership, path bytes and ordering | Decision 0029 corrects the original Rust mismatch: preserve full-string ordering, literal `^`, host bracket syntax, dot entries and repeated separators. Selected observatory paths/comparison use raw bytes too |
| Alias-list accumulation and SAT jump l/k mismatch/extra slot | Preserved deterministic source defects; no corrected variant |
| Normal source exit | Contextual `source_exit` with cause and defined diagnostic prefix; session/standalone table becomes terminal |
| Empty/singleton/header/basename/correction storage, invalid indices/unwritten fields/integer conversions | Operation-local named boundary before undefined operation, defined prefix retained; no guessed value or unsafe emulation |
| Source output and public JSON | Raw source-channel byte sink; CLI stderr routing on error; whole timing JSON only after success. Warning history is not reset |
| Historical L scope outside selected table/lookup dependencies | W later transfers EOP/weather/observatory state as detailed below; IF99/geometry/timing remain bounded adapters |

All nine withheld native clock experiments stay unrun. Eight have separate safe
Rust checks; the cycle has no execution result. Historical reports/reference
files and F's inventory remain unchanged. Central owns bounded acceptance.


## O parameter ownership and bounded frontend

Decision 0033 transfers source-indexed parameter allocation and labels with signed
flags/counts, independent logical/backing extents, and explicit unwritten prefit
and inactive-link entries. Caller 17/19 sentinels from K are never defaults.
`copyParam` maps to ordered `copy_param`, retaining links, backing cells and bytes
after the copied NUL. Undefined accesses yield operation-local named boundaries;
defined prefixes remain diagnostic and affected destinations become unavailable.
Overlapping buffers are excluded by safe ownership. Dropping/replacing an owner
frees its entire backing allocation instead of emulating source leaks or dangling
pointers. Full `Clone` is not source `copyParam`.

The strict frontend remains bounded: source values/fit/set flags live in one store;
original lines and supplied uncertainties remain separate metadata. Source `err`
starts at zero and fit callbacks assign it; prefit fields remain unwritten. This
is explicitly pending full PAR transfer. `parameters()` changes from a borrowed
map to a fresh owned snapshot, with unchanged JSON schema/membership/values.

Pinned `initialise.C:314–317` overwrites D_AOP with IPERHARM; lines 409–410 also
label param_iperharm IPERHARM. Both are preserved following the user's explicit
clarification in O's chat. This corrects the contract's prose assumption, not the
source defect or accepted K registry. General initialization, parser expansion,
linked-value arithmetic and full copyPSR remain deferred; M2 remains unaccepted.


## W observatory/EOP/meteorology dispositions

- Fixed observatory strings and MeteorologyFunction buffers retain initialized
  prefixes and tails. Aliases accumulate, warning state persists, and explicit
  observatory initialization appends. Read access cannot consume unwritten bytes.
- EOP caller loading appends without sorting or filtering. One process cache
  retains its first successful load across contexts and root/file changes.
  Checked output indices reproduce aliases and source write order. The upper
  proxy writes survive the contextual error before the invalid following-row
  read; a later valid query remains usable. Malformed bracket storage has no
  invented derivative or substitute sample.
- Shared weather initialization replaces the selected vectors in source order;
  lazy initialization happens once. Direct table objects remain independent.
  Missing/zero wet data gives zero; missing/zero pressure gives 101.325 kPa with
  source warning history. Warning-dependent glob NOMATCH that leads to a null
  native access returns a named safe error, with its defined prefix retained.
- The complete tropospheric driver preserves signed loop counts, reset and
  bypass order, per-pulsar warnings, SAT weather selection and clock-shifted UTC
  season. Only `troposphericDelay` changes in canonical source records. The public
  PKS frontend supplies its already selected geometry and clock-active adapter
  flag to the shared body; its input record remains unchanged.
- Native exits become contextual errors with diagnostics and observable prefixes.
  Independent resources/lifecycle diagnostics can run after a terminal clock
  failure. The existing terminal clock and direct-table fault policies remain.
- The NMF_wet upper c interpolation now reads the c table. This is an explicit
  0053 correction, with 10 affected U cases/31 wet-result fields in a separately
  pinned native experiment. Original U evidence is preserved. Source-matched
  binary64 contractions are explicit and bounded; no tolerance/backend change.

These checks establish source correspondence on the declared Darwin arm64 host,
not complete physical-model validation, concurrency behavior, or a general
PAR/TIM/site frontend. See W's report for native-invalid and unrun gaps.


## Y time-ephemeris dispositions (0058/0061)

| Original source behavior | Current disposition |
| --- | --- |
| IFTE init resets pc[1] but retains twot | Reset both to zero; retain other arrays/counters as mapped. Independently corrected native histories and rational zero-argument polynomial regression. |
| FB cache keys only by record across newly opened files | Actual consumed file SHA-256 plus record; changed content starts cold, unchanged bytes retain warm nr==1 history. Publish only a valid candidate. Same-path edits, identical-byte replacement and failure/recovery tested. |
| CALCEPH reads pulsar zero while evaluating another pulsar | Read the current observation; two-pulsar native witnesses and independent single/multiple equivalence. Supplied values only. |
| Source leaks/released FILE pointers and source exits | Unique RAII owners; failed readers release safely, null/released invalid operations return contextual errors. Four terminal native events explicitly project released ownership; original raw state remains available. |
| Partial IFTE header/constants, incomplete Fortran record int | Stop before consuming uninitialized source storage. These remain distinct from byte-oriented EOF conversion. No fabricated native golden. |
| Incomplete FB header/coefficient reads | Intentional small-bug correction under 0053/0065: read_fortran.h::read_double assigns all fgetc bytes, including host EOF 0xff. Original FB can return +0, NaN or a plausible finite correction; Rust rejects attempted incomplete reads as FB_incomplete_record. Direct primitive EOF semantics, complete-header nr==1 and signed count no-ops remain unchanged. |
| IFTE version/grid/count restrictions in old subset | Remove broad restrictions. Preserve zero/one/two coefficients, backed beyond-end dates, valid negative-step cache skip and source kind selection. Check actual conversions, seek arithmetic and storage accesses. |
| IFTE missing irec update; repeated obsTerm scaling | Preserve. No new record cache or derivative-formula correction. Dead second obsTerm assignment has no observable result. |
| Remainder implemented as percent | Evaluate a-(int)(a/p)*p at Rug128, checked i32 quotient, Rug divisor. Defined signed-zero/nonfinite outcomes retained; four 0049 conversion-boundary differences remain unresolved. |
| GCC O2 binary64 expression contraction | Explicit local mul_add only with compiled source and bounded witnesses for recurrence, derivative, reverse sum, record origin and dotproduct. No tolerance or production precision changes. |

Full original/corrected field deltas, source-invalid cases and lifecycle projection
are recorded in Y's contract and report. Historical V outputs/runners and all
accepted regression evidence remain unchanged. M2 remains unaccepted with its
same four raw-order diagnostics.


## AA observer-coordinate dispositions (0062/0066)

- All 133 original X histories retain exact ordered types/state, finite bits,
  nonfinite classes, signed zero, raw diagnostics and original six exit prefixes.
  Complete UT1 private arrays agree with X's separately audited observer.
- `get_obsCoord.C::remove_white:464,469,474` at fda6ad4 shifts potentially
  overlapping suffixes with `strcpy`. Rust moves the same initialized suffix/NUL
  with `copy_within`. Nineteen complete buffers match the central corrected
  memmove experiment; its seven original no-shift neighbors remain exact.
  Original overlapping calls are not executed or given fabricated native values.
- UT1 missing-file/row-limit source exits retain the written prefix and return a
  contextual error. An unsuccessful load remains unavailable for further UT1
  use, without a global resource fault. Empty/inconsistent backing, undefined
  integer conversions, unwritten input and missing string terminators stop before
  the source-invalid operation. The terminal per-table state is a safe continuation
  contract beyond a native exit; no successful native golden is invented.
- IMAG mutates the shared observatory xyz and retains cached geodetic values.
  Existing frontend runtimes now read that live record. Source satellite flag
  units, missing-X-epoch asymmetry, offset selectors and ecliptic selectors stay.
- Local `mul_add` sites reproduce the actual GCC16 O2 contractions with 18 bounded
  X histories and retained native object/assembly evidence. Rug128 and the four
  0049 precision-boundary findings are unchanged. Full public PAR/TIM/model
  integration remains separate; M2 still has its four raw-order findings.
