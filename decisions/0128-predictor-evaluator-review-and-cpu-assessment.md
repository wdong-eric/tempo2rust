# Predictor evaluator acceptance and remaining CPU work

Central checkpoint, 2 November 2026. Accept AO handoff 42's measured **macOS
arm64** owner-reuse improvement. Production `8716d1dc026641f3a51c2ba88850c5599293cbdd`,
validation `2b963c39b7bbb580ede8256ac24404bd6694072b` and report
`e475cba0dbc24a935ee81eeb176031d3e6e9a0f8` are integrated with their original
history through `9b7a185ab5a5836581fbabf442321aff4e74b073`. Linux x86_64 execution
of this continuation remains due. This accepts the scoped optimization, not
all predictor science, AL, general fitting or full migration.

## Preserved contract and verified improvement

The existing canonical Clenshaw evaluator consumes already-dead numeric owners.
Safe pinned Rug assignment operations reuse their storage, retain operand order
and pass the actual first-rounding direction to the unchanged ordinary range
and subnormal function. A redundant f64 construction/copy is removed. Existing
macOS fused and Linux separate source operations remain distinct. The explicit
NaN handling preserves fresh-result behavior, including division's separate
sign rule. The implementation comments explain the TEMPO2 counterpart and
measured ownership reason. No new unsafe operation, public API, C transport,
dependency, numerical precision or acceptance tolerance is introduced.

Central reviewed these operation/rounding boundaries, independently verified
all **1,895 files / 172,106,780 bytes**, 40 external identities and **1,726**
Git-to-export/installed source bindings, replayed eight C/C++ static/shared
native value/storage profiles and 24 corruption controls, and compared all
30,258 C result/state lines, 38 legacy files and 5,880 direct edge results.
All 28 finite direct results agree in the observation and timing runs.
Allocation deltas and all **91** raw process/resource records were recomputed.
Saved installed-consumer and scoped caller/sanitizer results are bound and
checked. A fresh central debug execution passes all **15** predictor tests;
AO's saved release execution and formatting/Clippy logs are retained. No new
full timing/fitting campaign was needed for this bounded change.

Seven interleaved samples per scope, each with 100,000 iterations, give:

| Loaded scope | Before, s | After, s | Speedup |
| --- | ---: | ---: | ---: |
| Direct polynomial pair | 1.238879 | 0.621293 | 1.9940x |
| Direct model pair | 1.346498 | 0.687565 | 1.9584x |
| Complete installed C phase/frequency pair | 1.750129 | 1.078071 | 1.6234x |

Complete C allocation requests fall from **639 to 242** per pair and requested
bytes from **14,864 to 8,512**. Counts/volume are not live-memory or RSS savings.
All samples and startup outliers remain; no startup gain is established. These
are measured macOS workload results, not Linux or universal guarantees. Direct
and C workloads differ and cannot be subtracted into an exact FFI cost.

The original supplemental native comparison remains unchanged: 1,429 finite
differences fall within its existing AF floor, and one negative-NaN T1 frequency
sign mismatch remains a failure. Inherited polyco log10RMS failures remain open.
The unchanged installed qualifier passes. Same-engine equality does not waive
these native gates.

Central saved-review receipt SHA-256:
`40a59fa3523608dfaf194c8d58ad564d29366120b561e5cf71cbce5fde698c68`.
The delivered manifest remains
`0ec8a587d02e02517b195f874bbe76bb19fb86a49ad392a3ecfe9ee912af7621`.
All declared artifacts and external files are copied and hash-checked under
ignored `validation/predictor_performance/build/evaluator-reviewed-20261102/`.
The central executed review, fresh test log and copy mapping are retained under
ignored `decisions/evidence/release-qualification/build/ao-evaluator-central-20261102/`;
retention receipt SHA-256 is
`a89805aa53a9c18ec4769e842f8f9601b238252ad344689670ea136186b2e715`.
Original transport and continuation identities remain intact. Generated JSON,
binaries and traces are not committed.

## Checkpoint and next dependency

The installed native loop still takes 0.004528 s: native remains about **238x
faster** than this Rust candidate on the measured workload. The two accepted
predictor improvements and AN memory improvement are substantial progress, but
performance and scientific/platform qualification still prevent full migration.
Further allocation reuse alone must not be assumed to close that gap.

AL keeps the single compatibility/qualification stream and implements the
user-adopted bounded Rug128 contract under 0127. Queue actual Linux owner-reuse
qualification with the next reviewed successor, retaining the exact baseline
`0bd8e2c` and candidate `8716d1d`, 64-bit ordinary rounding/edge checks, full
installed-interface outputs and independent native failures. The currently
running Linux run **41392963741** remains frozen at **9991e80**; this integration
does not publish, restart or relabel it. Its fifteen-minute monitor continues.

Release the bounded [next CPU assessment](../handoffs/43-predictor-cpu-next-assessment.md)
to the existing AO chat. Refresh the post-change profile and assess established
facilities against the actual ordinary-profile/public-API contract before central
chooses the next implementation. A hardware-width backend, public API change,
new scientific kernel or precision change is not authorized by that assessment.
Central retains that design decision. There is no new scientific-policy choice
needed to accept the completed owner-reuse stage.
