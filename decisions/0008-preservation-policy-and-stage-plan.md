# Preservation policy and continuing stage plan

**Current-policy notice (24 October 2026):** [Decision 0053](0053-small-upstream-bug-fixes-during-transfer.md), based on the user's later instruction, supersedes this document's blanket prohibition on fixing demonstrated bugs during transfer. The original decision below is retained as history. Its source correspondence, unsafe, coverage and central-acceptance requirements remain applicable; independently decided numerical gates are unchanged.

Central decision, 23 October 2026, implementing the user's active goal. **Preserve TEMPO2 behavior during the transfer, including known bugs. Defer their correction until the overall transfer is finished.** This supersedes earlier permission to fix a newly demonstrated TEMPO2 bug while porting. Central design and milestone acceptance remain in this chat; separate implementation chats deliver evidence for review.

## Required fidelity

Preserve source notation, physical conventions, units, coordinate/time frames, constants, operation order, numerical boundaries, defaults, selection rules, warnings/fallbacks and repeat-call state behavior. The approved Rug128 representation and declared comparison budgets remain in force; they do not authorize changing the underlying algorithm. A surprising source expression or reproducible defective result is a compatibility target, not a correction opportunity.

Record each discovered defect with the pinned source location, a minimal reproducer, observed legacy result, Rust result and a deferred-fix note. The port must reproduce deterministic source behavior where it can be represented. Do not silently convert a finite, nonfinite or stale source result into an apparently improved result. Do not relax a comparison gate to excuse a difference.

C/C++ undefined behavior has no single portable result to copy. Uninitialized output, invalid memory access, overflow with undefined semantics and process termination must be identified separately from defined floating-point arithmetic. Do not invent numerical values or add unsafe Rust to imitate uninitialized storage. Existing owned-state and fallible-library boundaries remain operational boundaries, but every observable difference is recorded as a compatibility limitation; they are not evidence of exact legacy behavior on those paths. Do not call the overall transfer complete while such dispositions remain unreviewed.

The prior M0/M1a/M1b acceptances are historical, bounded findings under their recorded contracts. They do not establish full compatibility under this new goal. Preserve those artifacts and histories. In particular, A2's solar-wind initializer correction, D's JPL cache/read-error correction, initialization policy, input guards and repeat-call state changes must appear in the baseline difference inventory. Deterministic departures require a source-faithful implementation before final transfer acceptance; a previous acceptance is not a blanket exception to the new instruction. Implementation changes follow a separate bounded handoff after the inventory is reviewed, to avoid conflicting edits to E's crate.

## Module correspondence and unsafe

Every production file/module must state its TEMPO2 counterpart(s) near its top, naming the source file and relevant routines. When a module only supplies Rust ownership, parsing glue or serialization, say that it has no standalone TEMPO2 counterpart and name the source responsibility it supports. Keep recognizable scientific names; explain unavoidable Rust naming changes in a correspondence table. Do not rename source concepts merely for style.

Prefer safe Rust. New parsing, fitting, linear algebra and orchestration must use safe code unless an actual requirement makes that impossible. Retained numerical FFI can require `unsafe`; keep the boundary small. Each necessary unsafe block, unsafe function or unsafe implementation requires an adjacent explanation of why safe Rust cannot perform that operation and the invariants that make the operation valid: ownership/lifetime, pointer length/alignment, initialized output, aliasing, threading and cleanup as applicable. An isolated `SAFETY` label without the reason and invariant is insufficient. Existing FFI sites need this audit too.

## M2 amendment

Decision 0007 still specifies the finite fitting calculation, selected columns, SVD, covariance, updates, weighting, NITS lifecycle and numerical gates. Its formerly mandated bug corrections are replaced as follows:

- Preserve the source square-fit calculation when `n = p` and error rescaling divides by zero. The source probe already demonstrates NaN errors. Do not reject the calculation merely to avoid those errors. Preserve the update callbacks' `if (error >= 0)` behavior: NaN does not satisfy this condition, so a physical parameter's existing error can remain unchanged even when the raw fit-coordinate error is NaN.
- Preserve each nonfinite value's classification in fit diagnostics. Use explicit string spellings `NaN`, `+Infinity` and `-Infinity` in the fit output and comparator; do not coerce them to zero, a finite floor, or an unexplained null. Do not weaken the existing finite `Scalar` input invariant. Keep the representation local to fit diagnostics. A run that completes with source nonfinite errors is identified as such, rather than mislabeled a finite valid estimate or converted into a different numerical result.
- A reduced chi-square at zero degrees of freedom follows the source arithmetic if evaluated; distinguish it from a quantity the source never computes. Do not invent a finite value. With no rescaling, preserve the finite square-fit estimates/covariance/errors.
- Duplicate MODE/WEIGHT controls follow the parser's sequential assignment: the last declaration wins, with all original source text retained. Positive NITS remains the initial M2 execution profile; other counts remain explicit coverage gaps, not a claim that upstream rejects them.
- Probe zero weights and other nonfinite arithmetic in isolated reference processes. Preserve defined calculations and their nonfinite outcomes. A source assert/exit, uninitialized-output return or unsafe access is recorded as that failure class; the Rust adapter may return contextual failure without fabricating outputs. Report that adapter difference. In particular, `n < p` upstream returns without initializing estimates; no exact successful numerical solution can be claimed for that path.
- Keep finite rank-deficient pseudoinverses, the strict SVD cutoff, unscaled covariance, source-rescaled errors and the effective convergence limit. No fallback solver, clipping or error-floor policy is introduced.

Reference fixtures for these cases compare source values/classes and the exact stage reached. The finite comparison gates from decision 0007 remain unchanged. Nonfinite classifications are compared exactly without applying finite tolerances or comparing NaN payload bits. The old central probe remains historical evidence; E's new exports use the explicit classifications above. Mark observable boundary differences separately in the report.

## Stages and ownership

The full objective is the transfer of the TEMPO2 core, not just the presently supported isolated-pulsar fixtures. The pinned build's library source lists, core entry points and their active model/directive paths anchor the coverage inventory. Retained established numerical dependencies must have explicit dispositions and boundaries; they cannot hide missing timing implementations. Plugins and GUI compatibility are not silently added to core scope, and neither are core routines silently excluded because they are difficult to test.

| Stage | Owner and output | Release / acceptance |
| --- | --- | --- |
| E: fitting | Separate implementation chat; owns `rust/` and `validation/fitting/`, returns `rust/M2_REPORT.md` | Release under decisions 0007 and 0008. M2 remains pending central review. |
| F: core and fidelity inventory | Separate evidence chat; owns only `validation/coverage/`; maps pinned core sources, baseline Rust modules, unsafe sites and earlier departures | Release now. Read-only with respect to implementation and central decisions. |
| G: binary reference | Separate evidence chat; owns only `validation/binary/`; probes all 14 source model dispatches and their derivatives, updates and mutations | Released by decision 0009. Independent of E/F; does not implement Rust or accept binary coverage. |
| Existing-departure reconciliation | A subsequent implementation handoff after F's evidence and E's ownership window | Central review assigns concrete changes and source-observed regression targets; no silent grandfathering of deterministic fixes. |
| Remaining timing/model/input/fitting/prediction stages | Separate bounded implementation chats, selected from F's complete inventory | Central contracts specify dependencies, reference fixtures and acceptance. A completed stage does not remove uncovered rows. |
| Integration and transfer acceptance | Implementation checks plus central acceptance | Every core row has a reviewed disposition and evidence; correspondence, unsafe justifications, regression behavior and remaining limitations are audited. |

F uses a Git snapshot of primary commit `ddc07b2` for its baseline Rust audit so E can work concurrently without moving the target. It may also list later files as unreviewed additions but must not claim to have verified them. E audits the modules it changes and adds the missing explanations at existing unsafe boundaries within its ownership. Their output directories do not overlap.

Stage commits are authorized by the user. Use existing Conventional Commit prefixes, stage explicit owned paths, preserve others' edits and never rewrite the independent reference histories. The central chat may inspect completed reports and send follow-up implementation work when user-authorized coordination permits it; it does not delegate milestone acceptance.

## Completion evidence

Keep the goal active until the source inventory has no unreviewed core gaps, every released implementation stage has delivered and been centrally accepted, all deterministic behavior differences have been reconciled or explicitly resolved by the user, and every production module/unsafe boundary has the required explanation. Verify the actual code and checks behind each claim. M2, the inventory, a green fixture subset or the creation of chats alone cannot establish completion.
