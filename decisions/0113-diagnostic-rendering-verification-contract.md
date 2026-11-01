# Verify stored messages and each engine's prescribed rendering

Central design decision, 1 November 2026. This defines the next bounded
comparison contract for the source-attribution effect already accepted in
0107/0108. **Implementation remains frozen during run 41294531402.** Release
AL's work explicitly after reviewing that run; this document does not itself
start implementation or change any current result. AL, M2 and full migration
remain pending.

**Scheduling update:** [0116](0116-isolated-diagnostic-comparator-work.md)
records the isolated implementation/saved-evidence release and its reviewed
follow-through into fresh macOS W verification. The main candidate remains
frozen; Linux replay and integration still await central review of the current
run. All requirements below remain.

## Evidence and policy

The three terrestrial cases `alias_bounded_tokens`,
`observatory_exit_read_alias` and `observatory_missing_alias` store identical
complete native/Rust messages, including raw filenames. Their corresponding
state, counters, event sequence, return/exit status and all other projected
fields agree. The native source prefix is 125 bytes; the Rust basename prefix
is twenty. The source logger formats ANSI bytes, caps the result at 359 bytes,
then removes ANSI for redirected LOG_OUTFILE. This exactly explains the
336/344/344 native visible bytes versus 282/240/256 Rust bytes. The lost native
tails are independently present in native backing storage; they are not inferred
from Rust output. All six emissions are reproduced by the bounded witness.

The source-bound proof is `validation/release/diagnostic_cap_review.py` at
`fecbc18`. Its receipt is
`d68ec56eb523df4679f610681ce478aa747fb7f3a406fa7a0731fa443a1d006b`, with twenty-two
bound inputs and separate raw native/Rust event streams. Central independently
checked the inputs, stored-message equality and source formatting definitions;
0112 records the publication boundary. The original three strict comparisons
remain failed, and their downstream controls remain unrun in that execution.

Truthful Rust attribution and the existing logger behavior remain the product
contract. A successor comparison must verify the complete logical messages and
each engine's exact prescribed rendering. Different prefixes can change the
visible suffix under the shared cap. The new check preserves truthful
attribution and the logger's source behavior. This decision changes the
diagnostic verification method only; it introduces no numerical tolerance, scientific
projection, public output change or exception based solely on a case name.

## Required paired comparison

1. Keep the existing attribution-only adapter and its body-preserving checks.
   Its raw-input validation must still reject internal comparison tags,
   fictional Rust files/lines and native labels used for Rust emitters. A string
   pair without independent message data continues to use the strict comparison.
2. A diagnostic rendering difference may be accepted through the additional
   proof only when the caller supplies independently recorded native and Rust
   message bodies. For the initial delivery, use the three demonstrated W
   warning/error histories, with a single unambiguous emission associated with
   its event and storage slot. For a qualified history, verify the prescribed
   rendering even when the initial strict text comparison is equal: deleting
   Rust's additional visible tail can otherwise manufacture that equality.
   Record the strict result separately. Missing required backing evidence still
   fails; other string-only/unsupported histories retain the strict contract.
   Keep the complete inventory; do not whitelist case identifiers as passes.
3. Verify all existing schema, membership/order, scientific state, counters,
   severity, stream routing, outcome and file/input/source identities first.
   Compare the complete backing messages exactly. Apply only the pre-existing,
   explicitly bound fixture-path projection where that contract requires it;
   reconstruct each side from its own raw message and raw paths. The three
   current witnesses have byte-identical raw filenames. An unavailable, already
   truncated or ambiguous backing message cannot establish this proof.
4. Validate the real native/Rust emitter pair using the source-derived mapping
   and native compiled-source identity. Build each expected rendering from that
   side's verified prefix, severity and independent message. Apply the exact
   TKlog.h WHEREWARN/WHEREERR formatting and TKlog.C::_TKchklog order, including
   C-string semantics, the 359-byte colored-buffer cap and LOG_OUTFILE routing.
   Reuse the existing logger/witness patterns. A general ANSI-stripping library
   does not establish equivalence to this source byte-processing rule.
5. Require exact equality of each observed raw emission to its independently
   expected bytes. Compare all bytes outside those bounded emissions exactly,
   including order, adjacent action markers and newlines. Determine bounds from
   the verified rendering/event association; do not split arbitrary message data
   on marker-like substrings or accept only a common visible prefix.
6. Missing evidence, a wrong site, a changed backing message or any unexplained
   rendered byte remains a failure. The initial scope is the demonstrated
   redirected warning/error path. Broader modes, multiple ambiguous emissions,
   incomplete captured messages and other corpora need their own bound evidence;
   there is no automatic global fallback.

Keep raw outputs and the original strict-comparison differences in the receipt.
Report the additional result explicitly as verified source-attribution
rendering, with both raw identities, complete-message binding, expected/observed
render hashes and visible tails. A later full case result may pass only when
all its original non-rendering checks and this complete paired proof pass.
Do not overwrite or relabel earlier failed receipts.

## Bounded implementation and verification handoff

After central releases implementation, AL owns the work in dependency order:

1. Reuse `diagnostic_attribution.py` and the existing cap witness for one small
   paired verification entry point. Retain `Attribution.stream`'s strict contract
   and its existing long-line/internal-tag controls. Keep the source formatter
   counterpart and the approved attribution rationale beside the implementation.
2. Wire only the qualified terrestrial comparison through the AL-owned staging
   adapter in `corpus_compare.py`, preserving immutable historical runners and
   binding exact staged edits. The actual corpus and controls must call the same
   verification entry point. No production Rust change or new runtime option is
   needed for this contract.
3. Add meaningful controls using the saved complete pairs. A changed visible
   byte, numeric/message token, filename, source site, severity, channel, order,
   counter or case membership must fail. Specifically reject a change confined
   to the Rust tail that native truncation hides, appending/dropping a newline,
   truncating Rust to the common prefix, removing a diagnostic and passing an
   internal comparison tag as raw output. Test missing/truncated message evidence
   and retain the original adapter's strict behavior when no proof is supplied.
4. Replay the bound saved pairs through the actual entry point, then execute the
   complete terrestrial comparison and the previously unrun downstream controls
   from a named clean candidate. Use isolated build targets and retain all
   original failures. Qualify the same contract with actual Linux inputs/results;
   macOS prefix byte counts are not constants for another build or platform.

Return the scoped commits, exact identities and separate original/successor
results in the AL report. The current Linux run must finish under its frozen
rules. No SVD ordering waiver, precision change, numerical gate change, broader
message normalization or covariance optimization is released here.
