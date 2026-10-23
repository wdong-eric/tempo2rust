# Fitting preservation and boundary ledger

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

The original M1 guards, owned state, clock/resource cache behavior, repeat-call
policy, input handling, A2 solar-wind initializer correction and D's native
initialization/read-cache corrections are **not silently grandfathered as exact
legacy behavior**. E leaves them unchanged per the ownership handoff. Their
previous acceptance remains bounded by decisions 0002–0006; reconciliation
belongs to the separate stage after Assignment F's fixed-baseline inventory.
`M1A_REPORT.md`, `M1B_REPORT.md`, `native/departures.patch` and
`validation/coverage/` retain the detailed baseline evidence.

Independent fitting fixtures use fresh runtime contexts, matching isolated
reference processes. A preliminary verifier that reused a runtime from the
54000-centered fixture for the original seven-TOA fixture selected a different
cached clock chain at early epochs. Correcting the verifier's experiment setup
restored raw forward agreement. This observation is not proof of a newly found
Rust/source difference under the same call history; cross-profile history
comparison is untested and no cache behavior was changed.

JPL safety limitation identified by F/central: preflight validates supplied
bytes, but native initialization reopens the path. File identity/content must
remain unchanged between preflight and native use and during later native
reads; this is currently unenforced. Adjacent FFI comments explicitly state the
assumption. Preflight alone does not guarantee native memory bounds for a changed
file. A separate source-faithful JPL stage owns this issue; E does not fix it.
