# Small upstream bug fixes during the transfer

Central decision, 24 October 2026, following the user's direct instruction:

> it's okay to fix small bugs along our rewrite, as long as we make clear statements in comments about what the bug is and where it corresponds to in original tempo2 source code and how it is fixed.

**Small, demonstrated TEMPO2 bugs may now be fixed during the Rust transfer.** This supersedes decision 0008's blanket requirement to preserve every deterministic defect until the entire transfer is finished, including inherited wording in earlier handoffs and the historical active-goal description. Ordinary source conventions, notation, algorithm choices and numerical contracts remain the default. This permission is not contingent on another user confirmation for each routine fix.

## What an implementation chat may fix

Within its assigned files and responsibilities, an implementation chat may correct a localized defect when the source behavior, cause and intended correction are established by source/caller evidence and a reproducer. Examples include an omitted initialization with an established empty-state invariant, a cache tag published before a failed read completes, or a wrong object/index contradicted by the surrounding caller contract. A suspicious expression, unusual fallback, slow path or disagreement with Rust is not sufficient evidence by itself. Undefined native operations are classified without executing an invalid access or inventing a native golden value.

For each correction:

1. Put an explanatory comment beside the changed Rust code. Name the original TEMPO2 file, routine and location at the pinned revision; state the original bug, why that behavior is wrong and how the Rust change fixes it. Retain the module's existing counterpart map. Give the regression-test or report reference when that makes the evidence easier to find.
2. Add a focused regression that exposes the failure and protects the intended correction. Record the original behavior or undefined-state classification, corrected behavior and affected valid/neighboring cases. Run the affected existing checks; distinguish implementation checks from scientific validation.
3. Record the intentional departure in the assignment report, with source revision, reproducer, changed cases/results and commit. Fix the cause; do not compensate downstream, silently substitute values, broadly initialize unrelated state or add a legacy-bug runtime mode without a real requirement.

A local fix can change the observed result; that is the point of this permission. The report must make that change explicit. Central still owns shared design, reference dispositions and milestone acceptance. Changes to physical conventions, units/frames, numerical precision, solver algorithms or comparison tolerances require a concrete central design decision. Implementation chats should return those specific questions with evidence while continuing independent work, rather than treating all bug fixes as approval requests.

## Reference and acceptance evidence

The canonical TEMPO2 checkout, independent A2 reference repository and accepted original outputs remain unchanged. They document what the pinned source does, including its defects. A corrected native experiment, if needed, uses a disposable copy with a recorded patch and separate outputs. Keep original and corrected identities distinct.

For an intentional correction, retain the original failing/native witness and add a separate explicit corrected regression expectation. Show the exact affected comparison cases and fields for central review. Do not rewrite an accepted oracle, silently skip a comparison, relax a tolerance or label a deliberate departure as exact source agreement. A verifier for the corrected contract may pass only after that contract explicitly accounts for the demonstrated correction; this is not permission to leave the old gate failing while reporting acceptance. No general compatibility mode or second production implementation is required.

Decision 0039 still excludes reproducible generated JSON and other large evidence from Git. Decision 0050 still requires the accepted R runner and its preservation membership to remain exact. Source code, generators, authored expectations and concise reports are committed with explicit-path Conventional Commits; local execution evidence remains reviewable by path and hash.

## Current work and unresolved findings

T remains the sole Rust owner. Its bounded 0052 comparator correction is a verifier bug, not a TEMPO2 scientific defect; it can finish that delivery without adding unrelated source fixes. V remains a native-reference assignment: its original IFTE/FB90/tt2tb observations stay source-faithful, and demonstrated bugs can inform the later Rust handoff. U's accepted reference and closed ownership remain unchanged. This policy does not reopen closed assignments or automatically modify already accepted implementations.

The known JPL cache-before-read behavior and the previously demonstrated solar-wind count initialization omission are candidates for bounded corrections when their code is next assigned. Their recorded source evidence remains useful; fixes no longer need to wait for the whole transfer. Each actual correction still needs the local comment, focused regression and reported reference disposition above.

The four historical M2 dependent-column mode-order/singular-index failures remain unresolved under 0012. The 0049 utility conversion-boundary differences remain unresolved under the fixed Rug128 design. Neither is established as an upstream bug merely because it obstructs exact comparison. This decision changes neither their gates nor their acceptance status. Full-core coverage, source correspondence, justified minimal unsafe and central milestone acceptance remain required.
