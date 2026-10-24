# Approved observation-verifier output routing

Central decision, 24 October 2026. Clarify the concurrent ownership in decisions 0045–0047. T's already authorized routing change to R's runner is compatible with U/V's preservation gate. This decision approves one exact file transition; it does not remove that runner from protected membership or permit scientific changes.

## Reviewed transition

Decision 0045 explicitly allows minimal output/build-root routing in `validation/observation_storage_port/{run,native,verify}.py` so T can verify R without overwriting R's accepted evidence. The U/V preservation paragraphs omitted those narrowly shared runner paths. U correctly retained its original freeze and reported the resulting mismatch.

Central inspected the complete diff from accepted R revision `04918d5` to current revision `2887220`. Only `validation/observation_storage_port/run.py` changed, in the definition of `OUT`: it now reads `TEMPO2_OBSERVATION_VERIFICATION_ROOT`, with the original `rust/verification/observation_storage` directory as its default. The existing `os` import is reused. There are no changes to comparison code, fixtures, projection, protected membership or scientific expectations; `native.py` and `verify.py` remain unchanged. The change is committed in `2887220` (`test(lifecycle): verify complete source histories and native witnesses`).

The exact reviewed SHA-256 transition is:

- Original frozen runner: `7d6424654a666d3852255bd9264ba0fbb88ba3c6e57f7bf717da4087a6b10529`.
- Approved routed runner: `5f0cb7bfa0c2ebc8606c789e157ab96b7d5fa248d54703e01f9c5c7495e0c4d6`.

Central independently evaluated U's preservation checker: this is its only current mismatched member. Both independent repositories still satisfy that check's pinned HEAD and clean-status assertions.

## Applying the clarification

U and V must retain their original frozen manifests and prior failed receipts as historical evidence. For this path only, a checker whose frozen hash is the original hash may recognize the exact approved hash above, explicitly recording decision 0048 and the old/new identities in its preservation result. A checker frozen after the routing change can continue its existing exact-hash check. Do not overwrite the baseline, remove membership, accept arbitrary changes under this path, or turn unrelated drift into a warning.

Keep an integrity control that rejects a further change to this runner and a control that rejects drift in another protected member. The approved transition may be a small explicit policy entry; no general waiver mechanism is needed. Source-map, fixture, ordinary-oracle and independent-repository protections stay in force. Fresh ordinary replay and controls should establish each delivery's final status after applying this clarification. Generated receipts and JSON remain local under decision 0039.

T's production and validation scope is unchanged. Any additional runner change must remain within 0045 and receive its own central review before U/V treat its identity as approved. This is coordination of an existing authorization, not acceptance of T, U, V or any numerical milestone.
