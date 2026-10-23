# Preservation and boundary ledger

Assignment E preserves defined TEMPO2 results, including deterministic bugs.
The pinned source is `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. Numerical
representation follows the already adopted Rug128 contract. Fixes are deferred
until the overall transfer is complete; central review owns dispositions.

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

Independent fitting fixtures use fresh runtime contexts, matching isolated
reference processes. A preliminary verifier that reused a runtime from the
54000-centered fixture for the original seven-TOA fixture selected a different
cached clock chain at early epochs. Correcting the verifier's experiment setup
restored raw forward agreement. This observation is not proof of a newly found
Rust/source difference under the same call history; cross-profile history
comparison is untested and no cache behavior was changed.

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
| nut[4] and direct acceleration demands | Initialized destination slices sized to actual dimension/demand. Four-slot libration rejects; nine-slot direct body/libration acceleration tested. |
| Partial fread elements | Raw initialized bytes retained, including old suffix; exact same-host byte comparison, not portable C numerical meaning. |
| NaN/tiny-step record conversion, bad coefficients/counts, title overflow, invalid selectors | Distinct operation-local Boundary errors. No broad low-level finite/AU/grid restrictions. Unsigned record-offset and kernel arithmetic retain source wrapping. |
| Defined nonfinite source arithmetic | Exact NaN/+Infinity/-Infinity classes preserved at low level; high-level nonfinite timing-vector rejection remains a documented adapter. |
| Global initialization code and live -7/-8 handles | Atomic global sequential status, startup -9, reset on init, unchanged by Drop. Live partial-init handles retain caller mutations and close normally. No C global-race equivalence claimed. |
| C pointer ABI and null/double close | Owned non-cloneable stream, bounded optional Sun view and field selectors. No packed Rust ABI, arbitrary byte-offset getters or raw handle API. |
| Pre430 unwritten temporary extension words | Source invalid marker clears optional rows; Rust avoids reading/swapping unwritten temporary words. Malformed accidental-valid marker case returns an explicit boundary. |
| Unterminated no-digit title | Declared H title_zero maps to version zero; overflowing or unterminated numeric token rejects rather than scanning beyond the consumed title. |

All 12 implemented responsibilities and each H event are covered by the projection.
The ten isolated source cases are classified separately in I's evidence. No
sanitizer output is used as a post-failure numerical oracle. Platform limits and
untested malformed branches remain in `JPL_REPORT.md`; central owns acceptance.

F B06 documentation correction: `ifteph.C::IFTE_init` does not check its header
read counts. `IFTE_get_Vals` checks data seek/read outcomes and exits on a short
coefficient read; upper dates must not be described as unchecked data reads.
The Rust range/record guards and owned storage remain existing adapter departures.
Only their source-description comment changed in this review follow-up.
