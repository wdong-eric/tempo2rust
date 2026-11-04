# Linux baseline review and bounded Q/Z correction release

Central checkpoint, 4 November 2026. Accept AL's `0e60cde` as the current
completed-run evidence inventory and failure classification, subject to the
wording clarification below. This does not accept all Linux science or the
release. The 86 passing / 11 failing / zero blocked stages remain unchanged.

Central read the full [Linux review](../validation/release/LINUX_41392963741_REVIEW.md),
checked all sixteen proof-file bindings and both report bindings in its delivery
receipt, and verified the matrix candidate/counts. AL's complete 53,208-file
archive and 2,103-file Git/export verification are reused, preserving all 207
case-distinct path groups. Central did not repeat that archive campaign.
Delivery receipt SHA-256:
`93b2c5431c283ee3394da492777a655bf2ead61f45f42f7cffcf1073de8d4349`.

The proposed per-workload performance limits are recorded separately in 0132
and await the user's numerical-budget choice. The measured native failures
remain explicit; timing observations do not accept scientific discrepancies.

## Two demonstrated, scoped corrections

Release these to AL as the single compatibility stream in main. AL is not alone:
central owns `gui/`, central decisions and acceptance; AO owns its isolated
predictor specialization. Preserve their work and canonical TEMPO2.

1. **Q preparation ledger.** `validation/pulsar_lifecycle/coverage.py` was
   changed during native preparation only to replace the gcov executable path.
   Its historical hash `0503677b…` becomes the rejected hash `31f74085…` under
   exactly that substitution. The comparison staging omitted this already
   recorded preparation change. Repair the inheritance/composition of the
   existing adaptation ledger in the release harness. Require the expected
   original/intermediate/current identities; arbitrary source drift must still
   fail. Preserve historical validators and all original failed receipts.
2. **Z telescope-offset interpolation.** Pinned TEMPO2
   `readEphemeris.C:420–422`, `440–442`, `460–462` computes a slope, an intercept
   and an interpolated offset. `rust/src/read_ephemeris.rs` currently forces
   FMA in both latter expressions based on the macOS GCC16 object. The actual
   Linux GCC13 object uses separate binary64 multiply/subtract/add. AL's
   bound 28-intermediate replay reproduces the difference in
   `c076_absent_nutation`, including all three reported vector-state records.
   Preserve macOS's measured fused sites and use source-ordered separate
   operations on Linux. Keep Rug128, all conversions, range/selection logic,
   vector addition order and numerical gates unchanged.

This is established binary64 arithmetic with demonstrated compiler contraction
profiles, not a new numerical method. Reuse existing platform patterns, exact
bit observations and the existing Z runner. Put the original source location,
platform reason and evidence beside the corrected expressions. Do not infer a
correction for the Gaussian histories, U, AB, DDH or broader fits from this one
interpolation witness. No blanket nonfinite normalization is released.

## Dependency-ordered verification and delivery

1. Correct the report's phrase “Native nlong returns integer zero”: the adopted
   two targets are integer zero, while the native witness decisions were 1 and
   −1. The Rust helper retains its existing negative-zero Float representation.
2. Repair Q staging using the existing preservation helper/ledger. Add focused
   controls for the real inherited gcov adaptation, a subsequent valid staged
   edit, unrelated corruption and broken identity chains. Exercise the actual
   comparison staging when local retained inputs permit it; report platform
   execution separately.
3. Correct Z's platform operation boundary. Protect the actual c076 bit witness
   and neighboring/endpoint behavior through the production path or a minimal
   domain helper only if that makes an important regression test possible.
   Run the narrow macOS affected tests and retain all unresolved Z records.
4. Return scoped commits and updated recipes/reports. Prepare one meaningful
   combined Linux qualification candidate with 0127, owner reuse, 0131 after
   integration, these corrections and the GUI qualification slice when ready.
   Central retains candidate freeze, remote-state check, publication/dispatch
   and actual Linux acceptance. Do not dispatch or publish independently.

The documented native dependency inventory remains binding. The first GUI
frontend is a separate crate; no GUI dependency is added to the headless engine.
After this bounded batch, prioritize a concrete U/AB or material fitting/DDH
cause using saved evidence before launching another broad diagnosis campaign.
