# Rust fitting implementation

**Assignment E is released under [decision 0007](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0007-m2-fitting-contract-and-e-release.md) as amended by [decision 0008](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0008-preservation-policy-and-stage-plan.md). M2 remains pending central acceptance.** This chat owns implementation and its evidence. The central planning chat retains scientific/design decisions, contract changes and milestone acceptance.

## Read first

Read [HANDOFF.md](/Users/Eric.Dong/Downloads/tempo2rust/HANDOFF.md), the complete decisions 0007 and 0008, [M1b acceptance](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0006-m1b-acceptance.md), [M1B_REPORT.md](/Users/Eric.Dong/Downloads/tempo2rust/rust/M1B_REPORT.md) and the [M2 source-probe evidence](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/m2/README.md). Decisions 0001–0004 continue to define the accepted numeric representation and timing profiles.

Work in `/Users/Eric.Dong/Downloads/tempo2rust`. The older `/Users/Eric.Dong/Downloads/tempo2-rust` spelling in historical reports is not the current workspace. The implementation baseline is primary commit `ddc07b2` plus the 23 October central policy update. The corrected M1b implementation, tests and acceptance documents have been committed separately; preserve those histories.

## Ownership and compatibility

Own `rust/` and the new `validation/fitting/` directory, including fitting reference generation, fixtures, comparisons, implementation, tests, verification records and `rust/M2_REPORT.md`. You are not alone in this workspace: preserve others' edits and adapt to accepted changes. Inspect Git status before editing or staging. Do not reset, clean, broadly stage or commit another chat's work.

Read canonical `tempo2/` at `fda6ad4a9753ad7dc5d291406f2f310a9161423f` and independent `validation/reference/` at `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572` without modifying either. Reuse A2's verified build read-only, or rebuild under your own disposable directory. Preserve A2's evidence and the existing M1a/M1b golden files and comparison rules. Record any required reference change for central review.

Preserve TEMPO2's existing behavior, including bugs; defer bug fixes until after the overall transfer. Decision 0008 supersedes the earlier instruction to correct demonstrated source failures. Preserve defined nonfinite fit outputs and record undefined-source/adapter differences explicitly. Every module must state its TEMPO2 counterpart (or identify Rust-only glue), and every necessary unsafe block must explain its necessity and safety invariants beside the operation. Use safe Rust for the new fitting path. Audit existing module headers and FFI comments within your ownership, preserving numerical behavior. F separately audits the fixed baseline in `validation/coverage/`; do not edit that directory.

Do not introduce a new solver backend, parameter-normalization policy, scientific correction, changed phase convention or relaxed tolerance. Present a concrete reproducer and recommendation to central review if one becomes necessary. Continue independent work while a material decision is unresolved.

## Target

Deliver Rust fitting of ZERO plus selected local F0–F3 terms for the accepted homogeneous barycentric `@` and PKS profiles. Support PAR MODE/WEIGHT 0 or 1, positive NITS with default 1, and the source error-rescaling flag. Add the library fit operations and thin JSON `fit` CLI specified in decision 0007.

Reuse existing loading, BAT and residual kernels. Keep fixed evaluations unchanged. Fit residuals use the source's unweighted mean removal; fitting must not conceal a baseline timing mismatch.

Preserve the source-scaled columns, mixed-precision SVD, `1e-27` strict cutoff, pseudoinverse, `n - p` degrees of freedom, sequential spin updates and unscaled covariance versus rescaled errors. Keep linearized residuals distinct from recomputed postfit residuals. Report original data separately from fitted model values/errors.

## Dependency-ordered work

1. **`validation/fitting/`: establish reference evidence.** Verify pinned Git/build identities, reproduce the central probe, and create deterministic fitting fixtures/exports through the actual TEMPO2 fit lifecycle. Record inputs, all controls, source/numeric boundaries, resources and provenance. Export the column/matrix, solver, update and pre/post residual fields required by decision 0007. Freeze inputs before comparing Rust. Separate pure solver cases from full timing cases; preserve old goldens.
2. **Rust fit controls and source derivatives.** Extend the existing PAR reader minimally for MODE/WEIGHT and NITS, retaining raw tokens and validated defaults. Add selected-column construction with the mandatory ZERO term. Compare direct callback outputs, column order and physical-update mappings against the reference before adding a solver. Reject unsupported fit requests at the fit boundary; preserve fixed-evaluation fit-flag semantics.
3. **Rust `tkfit`/`tksvd` counterpart.** Port the selected source SVD and back substitution using existing Rug128 and the documented binary64 boundaries. Preserve convergence, cutoff, covariance, chi-square and error-rescaling order. Verify identical-input solves, rank deficiency, cutoff controls and square-fit behavior. Preserve source nonfinite outcomes and callback predicates; record contextual adapter failures only for source termination/undefined-state boundaries, with nearby explanations.
4. **Rust `t2fit` orchestration.** Reuse the current timing kernels, apply fit-only arithmetic mean subtraction, update an owned working model, and execute exactly NITS cycles. Compare every iteration, including model snapshots, offset, linearized residuals and recomputed residuals. Do not mutate caller input or return a partial earlier iteration after a later failure.
5. **Public library/CLI and output.** Expose the two explicit fit operations and `FitOptions` described in decision 0007. Add the `fit` command with `--norescale`; retain the existing `residuals` behavior. Serialize one complete, versioned fit result with source-sensitive decimal transport, labeled coordinate systems, original metadata and resource/rank warnings.
6. **Verification and report.** Run every prescribed numerical gate and meaningful comparator mutation, failure/recovery check and existing M1a/M1b regression. Check formatting and Clippy. Save source hashes, exact commands, outputs and measured errors; write `rust/M2_REPORT.md` and update the crate's usage/support documentation.

Each step owns a tightly related component. Reuse current functions, types and verification patterns; avoid a generic optimizer framework, parallel parsers or redundant internal validation. If a useful shared refactor is necessary, keep it small and prove the old fixed outputs remain unchanged.

## Required evidence

The reference fixture matrix and comparison table in decision 0007 are authoritative. Cover the mandatory offset, each spin term, a nonconsecutive subset and the full set; both weighting modes, heterogeneous errors, both rescale settings, NITS 2 and input order; PKS and resource-free inputs; rank/cutoff behavior; invalid-fit errors and recovery.

Test comparator mutations that would detect a sign or update-order error, column/row permutation, changed weights, centered versus uncentered data, covariance incorrectly rescaled, a dropped or extra fit column, altered rank/DOF, missing iteration, mislabeled residual stage, nonfinite values and a numerical error just over each gate. Reuse the comparator's declared units/scales; do not compare only printed parameter digits or use a fit to reduce a failing fixed-model discrepancy.

Demonstrate the source's defined nonfinite cases as well as the explicitly labeled library failure boundaries and later valid processing. Include debug and release checks for those cases. Do not add numerical guards that replace reproducible upstream bug behavior. Keep Rust numerical validation separate from reference-only diagnostics, implementation tests and untested platform claims.

The report must include:

- Commit/source identity, dependency/configuration and fixture/resource hashes, plus exact reproduction commands.
- A TEMPO2-to-Rust naming map and a departure table with adjacent comments, evidence and regression locations.
- Coverage by fixture and iteration: matrix, solve, covariance/errors, physical updates, residuals and exact counts, with maximum errors and limits.
- Failure and recovery results, unchanged fixed-evaluation evidence, and known unresolved/out-of-scope cases.
- A clear statement that implementation is delivered for central review and **M2 is not accepted by this chat**.

No broader binary/DDH, Linux, performance, concurrency or packaging claim follows from this assignment. Do not mark central decisions complete or message other chats unless the user authorizes it.

## Dispatch prompt

```text
Implement Assignment E in /Users/Eric.Dong/Downloads/tempo2rust. Read HANDOFF.md, decisions/0007-m2-fitting-contract-and-e-release.md, decisions/0008-preservation-policy-and-stage-plan.md, handoffs/05-fitting-core.md, decisions/evidence/m2/README.md and rust/M1B_REPORT.md. Own rust/ and new validation/fitting/ only. M0, M1a and M1b are accepted; M2 remains pending. Implement the prescribed ZERO/F0-F3 fitting path, source-scaled derivatives, mixed-precision TEMPO2 SVD, weighting, error rescaling and NITS lifecycle. Preserve TEMPO2 behavior including known bugs and defer fixes until after the transfer. Apply decision 0008's nonfinite-result amendment, add explicit module counterparts and adjacent necessity/safety comments for unavoidable unsafe, and preserve the two independent reference repositories and old fixed-parameter evidence. Establish fitting reference fixtures first, run all prescribed numerical/error/recovery and M1 regression checks, and deliver runnable artifacts plus rust/M2_REPORT.md. You may commit your owned work in stages using existing Conventional Commit messages. Shared design decisions, contract changes and milestone acceptance remain with the central planning chat.
```
