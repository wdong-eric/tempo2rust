# Integrated performance candidate and full Linux execution

Central checkpoint, 2 November 2026. Publish and qualify reviewed candidate
`9991e804d0e230248e5fb20974b7e63384d4efeb`. The full Linux run is
[41392963741](https://github.com/wdong-eric/tempo2rust/actions/runs/41392963741),
attempt 1, job `125108130146`, created at **05:15:39 UTC**. The first actual
central observation at 05:16:27 UTC confirms that the native/installed matrix
is executing. Results remain pending; no Linux performance or full migration
acceptance is implied.

## Reviewed content and authority

The candidate combines the accepted AN covariance allocation and AO scalar
transport implementations (0123/0124), user-approved paired-mode comparison
(0120/0122), Q staging, and reviewed Linux source diagnostics. AL's final wiring
commits are `d46c658`, lifecycle correction `f511ced`, and final `9991e80`.
The new evaluator optimization under handoff 42 remains isolated and is excluded.

The user's 0101 authorization covers centrally reviewed AL candidates and reruns.
The later major-progress instruction and 0121 replace the former post-AL-only
optimization sequence. This publication is qualification of the now reviewed
integrated implementation on the already selected platform and public repository.
It introduces no further scientific-policy, precision, convention or tolerance
change. Central retains publication, review and milestone acceptance.

Two frozen comparisons accompany the combined-candidate matrix:

- AN: `dc435f53a64e4dd5c25cae6319841a28bd981fcb` to
  `68973d669af0e1180ad8532bb8b364452ca203d7`; only the allocator and its lifecycle
  test differ in the crate. Execute residency, complete science/state, native
  fitting/predictor, debug lifecycle and provider checks.
- AO: `68973d669af0e1180ad8532bb8b364452ca203d7` to
  `0bd8e2cf2ef54e9da6feace122acd8aa96aa3fbe`; only the three reviewed transport
  files differ. Execute installed C/C++ static/shared qualification, full
  before/after outputs, native failure accounting, Rust consumers, predictor
  regressions, scoped sanitizer/thread cleanup and matched measurements.

All source commits remain reachable in main and are explicitly fetched for the
shallow workflow. Prepared Cargo/GMP caches and separate target directories keep
these comparisons distinct from the combined candidate. The AO benchmark uses
AK's actual native 8x4 `cheby-native-1-2/t2pred.dat`, bound to the successful
case record, isolated PAR and product hash. The separate ABI fixture remains 4x2.
No missing benchmark is replaced with a different workload.

## Review findings closed

AN's raw original S mismatches remain immutable. The integrated gate separately
checks every state field against the existing approved corrected-native lifecycle
proof, including its original/corrected source identities. This follows 0068,
0079 and 0103; it introduces neither a field omission nor a new numerical waiver.
An unrelated state regression shared by both engines is still rejected. Missing
or failed proof remains explicit. Original fitting/predictor failures are retained.
AO's supplemental native sign mismatch likewise remains a separate failing gate
when present, even if before/after output equality and performance checks pass.

Central independently passed 14 focused wiring/collector tests. It checked all
30 saved Linux corrected-reference pairs and 61 bindings, replayed the complete
corrected state for all 22 saved AN outputs, and verified all 143 retained AO
products including 63 raw resource records. The saved native 8x4 workload binding
also passes. These are harness and saved-evidence checks, not a new Linux run.
The differing original identities of two historical lifecycle campaigns remain
explicit; their content replay does not substitute for the live identity gate.

The source audit and current unsafe/source map are refreshed. Historical report
counts remain dated. Collection retains the new matrix products, predictor
transcripts and raw usage records while excluding rebuildable caches/source
exports. The previous complete run took approximately 153 minutes; the job limit
is now 240 minutes to accommodate the added builds/checks and artifact collection.
This is execution headroom, not a scientific threshold change.

## Publication and monitoring evidence

Central verified a clean worktree, native tracked source at unchanged
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`, and fast-forward publication from
`0d968bf48f4f5b48cd12e3d2256d2fb60f95e967`. There are 107 newly introduced Git
blobs; the largest is 121248 bytes. No generated validation JSON/JSONL or build
artifacts are added. Existing untracked native `.DS_Store` files are untouched.

The publication preflight receipt SHA-256 is
`0907b72d7f5f7b21ac45b2da5fe4e9756e9033b673f13cda20f7d37304793711`.
The exact workflow, source diff, receipt, initial GitHub response and actual
poll log are retained under ignored
`decisions/evidence/release-qualification/build/linux-run-41392963741-review-20261102/`.
The executed preflight recipe also remains in
`/private/tmp/tempo2-central-linux-candidate-0125-20261102/`.

Central pushed the exact candidate, verified that no matching run existed, and
dispatched the full matrix once. The persistent app monitor
`tempo2-linux-qualification-monitor` is attached to this central chat every
fifteen minutes and bound to the exact run/attempt/SHA. It stays quiet while
unchanged, records actual checks, and reports meaningful changes or completion.
Local scheduled execution requires the app/computer to remain running; missed
polls must never be described as completed. Pause it after artifact collection
and handback to central review.

While Linux runs, AO continues the independently released allocation/evaluator
work in its isolated clone. AL remains the single compatibility owner, with
central review required before changing this frozen candidate or numerical
policy. M2 remains accepted only within 0122's bounds; general fitting, AL and
full migration remain open.
