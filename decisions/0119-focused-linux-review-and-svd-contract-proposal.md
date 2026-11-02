# Focused Linux review and SVD contract proposal

Central review, 2 November 2026. The 0117 correction run completed and its
artifact is verified. Release the bounded follow-through below; AL, M2 and full
migration remain pending. The final complete 69-stage qualification remains due.
The SVD proposal in this document is **not adopted** without the user's answer.

## Actual Linux result

Run `41312212554`, job `124864766765`, attempt 1 executed candidate
`0d968bf48f4f5b48cd12e3d2256d2fb60f95e967` and finished at
2026-11-01 14:21:29 UTC. All 26 partial stages executed: 23 passed, three failed,
none blocked. Collection/upload succeeded; this was not a timeout.

The local monitor recorded its initial 13:40:47 check only. On resumption its
process handle was missing; central inspected the same completed GitHub run and
collected it without redispatch. The promised later local polls are not evidenced.
Future active runs still require roughly fifteen-minute checks and a live monitor;
a completed run must never be restarted because its observer disappeared.

Artifact `13087172369` has 233084538 bytes and SHA-256
`928a0796bac120200cbbb4b0f794c80d6e3dd7f1d6526e3bbbb35aded78939e2`.
The fixed collector verified all 14121 manifest files / 2314760936 bytes directly
inside the ZIP. No case-distinct filename groups occur in this artifact.
Manifest SHA-256:
`bff7fe6f4f74d385eccd88bd355340f55d63c4244438a22c5855930076aeafc0`.
Matrix receipt SHA-256:
`aeb9719a0b697bf11f170162e122424750f2290a626b6d982a6185f010b830de`.
Collection completed at 2026-11-02 03:19:38 UTC. Original logs, artifacts and
receipts remain under `build/linux-run-41312212554-review-20261102/` in the
release-qualification evidence directory; generated evidence remains ignored.

Central additionally verified all 2063 exported candidate files against Git,
recomputed all 249 event-profile comparisons, and reviewed sixteen selected
members. Review receipt SHA-256:
`be6aa5c566137dcf23c5cfd1d65052ebc55be184540e961411377a8877c3b310`.
The initial central review assertion incorrectly expected every W fixture to
exit zero; negative-resource fixtures intentionally preserve nonzero exits.
The corrected analysis retains those exits and verifies the actual comparison
and preservation gates. No producer or historical result was changed.

| Scope | Reviewed result |
| --- | --- |
| Spin source profile | All 288 callback/bounded observations pass; four wide-update differences remain diagnostic |
| Selected public spin fit | Pass |
| Event timing and diagnostic | Both mean modes pass; six event/no-event/Gaussian controls retain neutral Rust observers |
| Event source profile | 236/249 rows pass; all finite comparisons and forty direct boundary controls pass; thirteen negative-native/positive-Rust NaN-sign differences remain |
| Event return transport | All 76 observed ordinary-to-f64 conversions preserve the observed value/class/sign; the remaining sign divergence is upstream of this boundary |
| M2 | Fifty records; exactly the two existing dependent-column raw-order/value failures remain; spin callback failures close |
| K parameters | Native-matched preparation and Rust comparison pass, including the truthful source-site correction |
| Q specialized PAR | Earlier undefined ROOT is repaired; execution now reaches an unbound historical Git blob lookup and fails before comparison |
| W terrestrial | All 130 histories and 28 downstream controls pass, with the native controls and corrected oracle |
| Observer coordinates | All 133 corrected histories pass, with 35 controls, nineteen supporting checks and fifteen native mutation witnesses; original comparison retains eighteen correctionUT1 differences from the approved producer correction |
| Contraction source profile / harness / formatting / Clippy | Pass within the executed partial scope |

This accepts the released spin, finite event evaluation, measured contraction,
W diagnostic rendering and K-site fixes within their actual two-platform checks.
It does not accept remaining nonfinite event behavior, Q, the raw SVD gate,
unrun fit/timing/output families or full platform/migration qualification. The
fourth-run unresolved ledger in 0117 remains authoritative for unrun scope.

## Independent SVD causal review

AL's disposable diagnosis is complete and centrally reviewed for the existing
six-by-two all-ones dependent-column fixture. Central verified sixteen source
bindings, three native libraries, three retained original inputs, 37 diagnostic
artifacts and all six executable identities. The native source matches the pin;
the Rust core matches the published candidate. Each engine's observed and plain
executions have identical eighteen U/W/V entries. Native binary113 and diagnostic
Rug113 match all 82 selected arithmetic/branch observations and all outputs.
Only the precision constant differs between the disposable Rug113/Rug128 copies.

The first normalization of `1/6` rounds below the exact value at binary113 and
above it at Rug128 (`TKsvd.C:192` / `tksvd.rs:60`). The first Householder update
leaves opposite-signed small values. In the first QR shift, native `f` is about
`-3.21e-34` and Rug128 `f` about `+9.80e-39`. The source `TKsign_d(g,f)` branch
(`TKsvd.C:113` / `tksvd.rs:252`) selects opposite signs; the subsequent rotation
places the large singular value in different raw slots. The later convergence
branch also differs. This is a demonstrated precision-sensitive path for this
fixture, with no independent port indexing/sorting defect found in it; it is
not a proof of general solver equivalence.

Both original raw-order failures remain. Prior physical-fit agreement is retained
evidence, not a new end-to-end fitting run. Delivery receipt SHA-256:
`83ed5a15746f39e40f7343876e458c14fcdf12aa90117adff9993689def7aed5`.
Central review receipt SHA-256:
`9760d234c9925a211229ee1ca8fe9998661923dda745ad94c7f7b97280c4819a`.
The complete report, recipes, traces and bound products are retained in the
ignored `build/focused-result-and-svd-review-20261102/` packet. No production
solver, ordering, precision or acceptance check changed.

## SVD proposal for the user's decision — not adopted

The new branch trace gives a concrete reason to reconsider the user's 0012
exact raw-order requirement. Central recommends retaining Rug128 and each
engine's untouched raw order, while permitting a pure singular-mode permutation
to satisfy the ordering comparison through the existing 0010 paired-mode method.

The proposed contract requires an exact one-to-one mode count, matching paired
singular values within the existing numerical budgets, matching retained/dropped
classification, each engine's original cutoff checks, and every unchanged rank,
physical-parameter, covariance, fit and timing gate. Wrong values, cutoff/rank
changes, missing/extra modes, nonfinite mismatches or physical-output failures
still fail. Raw arrays and the original exact-order diagnostic stay visible;
production output is never sorted or rewritten. This proposal changes only the
acceptance interpretation of mode position, not scientific precision, algorithms,
physical coordinate ordering or numerical tolerances. Broader null-space/basis
rotation allowances are not proposed.

The alternatives are to retain 0012's pending exact gate or separately investigate
source-precision solver arithmetic, which would require an explicit numerical
policy decision and broader qualification. Until the user answers, **0012 remains
fully binding**. No comparator or solver change is authorized by this proposal.

## Bounded AL follow-through released

AL resumes sole implementation ownership of the directly affected Rust and
release-harness files in the main checkout. Central retains shared decisions,
handoffs, publication, monitoring and acceptance. Preserve other authors' changes,
canonical TEMPO2, historical runners, all failed evidence and AM documentation.

1. Repair Q's staged historical lookup. The actual failing command requests
   `ad6fbf6:decisions/evidence/par-specialized/released-branches.json` from a
   comparison copy without `.git`. Reuse the existing bound history/staging
   mechanism or exact immutable blob adapter; preserve pinned revisions and
   protection checks. Verify the actual staged Q execution, including recovery
   and rejection controls, rather than only evaluating its injected expressions.
2. Localize the thirteen remaining event-sign rows to their first source operation.
   Ten are callback results and three are the Gaussian argument/exp/value at zero
   width/onset. The direct native-math/transport controls and all 76 return
   conversions pass, so do not undo those fixes or canonicalize signs. Correct
   only demonstrated internal generation/propagation differences with adjacent
   source explanations and focused signed NaN/zero/infinity controls. Preserve
   finite source operations, host ordinary precision and Rug128 timing semantics.
   Distinguish newly generated invalid-operation NaNs from propagated operand
   NaNs. If a host behavior is not yet evidenced, prepare the minimal actual
   Linux probe instead of inferring it from architecture or nominal precision.
3. Return separate clean commits, exact before/after evidence and the updated
   unresolved ledger for central review. The unchanged macOS/full W campaigns
   need no repetition solely for a harness or unrelated nonfinite change.
   Central will select the next full or focused execution after reviewing the
   concrete candidate; the complete 69-stage requirement remains due.

Do not wait for the SVD policy answer to complete these independent corrections.
No SVD, precision-policy, global arithmetic rewrite, tolerance change or covariance
optimization is released. AL-first sequencing and the deferred profiling handoff
remain unchanged.
