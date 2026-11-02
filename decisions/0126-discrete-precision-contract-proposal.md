# Proposed contract for demonstrated discrete precision boundaries

The user subsequently adopted this proposal in [0127](0127-user-adopted-rug128-discrete-contract.md). The text below retains the reviewed proposal and its then-pending status.

**Proposed, awaiting the user's decision.** Central review, 2 November 2026.
No production precision, selection rule, comparison gate or accepted oracle is
changed. Running Linux candidate `9991e80` remains frozen. Independent AO
optimization continues under 0124/handoff 42.

Central reviewed [AL's disposition brief](../validation/release/DISCRETE_BOUNDARY_DISPOSITION.md),
verified all five cited evidence hashes, and independently reproduced its exact
rational calculations. The four 0049 cases receive identical exact operands.
The eight SIMUL histories in 0077 instead store the same decimal SAT differently
at native 113 versus Rug128 precision. Their separation computations are exact
at both precisions; the differing branch originates in input storage. Rounding
only the final separation cannot reproduce the native branch.

## Recommended decision

Retain canonical Rug128 storage and the unchanged source-ordered expressions.
Explicitly adopt their discrete outcomes for the demonstrated cases, documenting
the departure from native binary128 branches. This is a numerical-contract
choice, not a source-expression fix or a claim of native equivalence. Matching
native rounding through added precision/provenance machinery is the alternative.
There is no basis to add a new arithmetic library or another timing engine.

The immediate proposed acceptance changes are bounded:

- For the four identical-input utility witnesses, require exact target integers
  `0, 0` and signed nonzero remainders stated losslessly in the brief. Retain the
  native `1, -1, +0, +0` witnesses. Preserve signed-zero/nonfinite behavior,
  source integer-conversion domains and all unaffected existing cases. No
  end-to-end pulse-slip correction is claimed without a caller-level witness.
- For the eight named SIMUL histories, require the exact decisions determined
  by the declared Rug128 stored inputs and source predicate, plus the complete
  resulting observation state. Retain the original native deletion bits and
  duplicated whole-record manifestations. Protect the strict exactly representable
  threshold, neighboring inputs, reversed order and repeated-call behavior.
- Determine expectations from independent input/rounding evidence, not by copying
  current Rust output. Do not drop fields, add an epsilon, change `<` to `<=`,
  widen continuous tolerances, or automatically accept other discrete differences.
  New unexplained cases remain failures requiring diagnosis and review.

The recommendation is consistent with the already selected Rug128 backend and
avoids introducing a separate source-rounding history. For these specific
witnesses it follows the exact supplied-input decision: stored native operands
for 0049, and the exact decimal SAT token for SIMUL. This does not establish a
universal accuracy claim or a more accurate physical arrival time.

If the user requires native branch compatibility instead, prepare the precision
exception before implementing it. The remainder needs its full rounded source
expression, not only a rounded quotient. SIMUL needs source-compatible SAT values
before comparison; projecting an arbitrary already-transformed Rug128 SAT does
not recover the native history. The scope, representation and effect on the
chosen Rug128 policy must be explicit.

## Why this choice needs the user

0049/0077 left these exact discrete gates open, and the standing scientific-change
rules require clarification when a choice changes data semantics. SIMUL changes
which observation is retained; a tiny time discrepancy alone cannot waive that
outcome. The user-approved 0120 SVD pairing concerns mode association and does
not authorize different deletion or pulse decisions. Approval here would cover
only this declared contract and its bounded implementation/verification, not
the 122 continuous preprocessing records, DDH, other fit gates or full migration.

After a decision, AL implements the approved comments, exact contract checks and
report updates in a new coherent batch. The original evidence and current Linux
execution remain intact. Central reviews that delivery before accepting affected
milestones; this proposal itself grants no acceptance.
