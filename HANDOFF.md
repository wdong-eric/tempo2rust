# TEMPO2 Rust implementation handoff

Prepared 19 October 2026; updated 20 October 2026 for the M2 fitting contract and Assignment E release. This workspace is modernizing TEMPO2's core in Rust while preserving its established scientific calculations. The central design chat owns shared design decisions, scientific conventions, scope changes, and milestone acceptance. Separate implementation chats own the bounded assignments below.

The immediate objective is one supported calculation path with demonstrated numerical agreement and useful failure behavior. Kalman extensions, plugin binary compatibility, and a new scientific timing framework are outside the initial milestones.

## Workspace and current evidence

Workspace: `/Users/Eric.Dong/Downloads/tempo2rust`.

The reference checkout is `/Users/Eric.Dong/Downloads/tempo2rust/tempo2`, from `https://github.com/wdong-eric/tempo2.git`, pinned to `fda6ad4a9753ad7dc5d291406f2f310a9161423f` and still clean. A delivered the original binary128 reference at `5289797`; A2 delivered the corrected reference at `7795d53`; B delivered the precision study. C delivered the Rust barycentric library and CLI, including the TEMPO2 naming pass, at project commit `49e122c3648e5cb91bae17233a062a444a10f9e4`. **M0, M1a and M1b are accepted; Assignment D is complete. Assignment E is released for M2 implementation; M2 acceptance remains pending.** Decision 0006 closes D's resource findings; decision 0007 adopts the fitting contract and prepares dispatch.

`tempo2/` and `validation/reference/` are independent Git repositories. The primary project repository tracks the central documents, Rust crate and precision study, and ignores those two nested repositories. E's new fitting-validation directory will also be tracked by the primary repository. The accepted D delivery is now committed as `3f08a8c` (implementation), `31d55bf` (validation) and `87f3933` (design/acceptance documentation), preserving the report/source-manifest hashes in decision 0006. Both independent repositories remain clean. Preserve existing histories and do not flatten or implicitly stage either nested repository. Inspect live Git status before starting new work.

The authoritative initial design is in [decision 0001](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0001-reference-and-m1a-contract.md). [M0 acceptance and C release](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0002-m0-acceptance-and-c-release.md) pins the reference evidence; [M1a acceptance](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0003-m1a-acceptance.md) records the accepted Rust implementation. [M1b contract and D release](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0004-m1b-contract-and-d-release.md) adopts the terrestrial contract and the user's compatibility rule. Later decisions supersede earlier pending-gate wording and historical submission status.

[M1b central review and bounded correction](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0005-m1b-review-and-resource-correction.md) records the original findings and required correction. [M1b acceptance](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0006-m1b-acceptance.md) closes both findings after debug/release checks and accepts the declared macOS arm64 PKS path. Scientific policy and tolerances are unchanged.

[M2 fitting contract and E release](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0007-m2-fitting-contract-and-e-release.md) defines ZERO/F0–F3 fitting, the source SVD and mixed-precision boundaries, weighting, uncertainty rescaling, iteration behavior, fitting fixtures, comparison gates and invalid-fit errors. The linked source probe supports these decisions. No separate implementation chat has been created by that document.

The latest scope clarification is in [Estimate Rust Rewrite Effort](chatgpt-conversation://6ad5ede9-5268-83ec-a623-52058d4a2959). All execution requirements needed for the initial tasks are included here and in the linked briefs.

## Assignments and execution order

| Assignment | Owned directory | Start condition | Required report |
| --- | --- | --- | --- |
| [A Reference baseline](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/01-reference-baseline.md) | `/Users/Eric.Dong/Downloads/tempo2rust/validation/reference/` | Delivered and reviewed at `5289797`; see A2 | `REPORT.md` inside the owned directory |
| [B Precision study](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/02-precision-study.md) | `/Users/Eric.Dong/Downloads/tempo2rust/validation/precision/` | Delivered and reviewed; Rug128 selected | `REPORT.md` inside the owned directory |
| [A2 Reference correction](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/01a-reference-correction.md) | `/Users/Eric.Dong/Downloads/tempo2rust/validation/reference/` | Delivered and accepted for M0 at `7795d53` | Updated `REPORT.md` and new revision |
| [C Barycentric timing](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/03-barycentric-core.md) | `/Users/Eric.Dong/Downloads/tempo2rust/rust/` | Delivered at `49e122c`; M1a accepted in decision 0003 | `M1A_REPORT.md` inside the owned directory |
| [D Observatory timing](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/04-observatory-core.md) | `/Users/Eric.Dong/Downloads/tempo2rust/rust/` | Complete; corrected delivery accepted for M1b in decision 0006 | `M1B_REPORT.md` and correction evidence |
| [E Fitting](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/05-fitting-core.md) | `rust/` and new `validation/fitting/` under this workspace | Released by decision 0007; handoff ready for a separate chat | `rust/M2_REPORT.md` and fitting reference/verification evidence |

A, A2, B, C and D have delivered. D's bounded DE405/IF99 correction is complete and accepted, with TEMPO2's meteorology fallback, valid numerical behavior and comparison gates preserved. E follows C and D as the next owner of the Rust crate and creates fitting evidence in its separate directory. The accepted reference and precision directories remain read-only inputs to implementation work; report needed changes rather than silently changing comparison evidence.

Starting an assignment means opening an implementation chat in this workspace and giving it the appropriate brief. This document does not create or message chats automatically.

## Shared working design

Use one Rust crate with a library and a small CLI initially. Owned model and observation values hold the input; fallible operations return results with contextual errors. The initial barycentric operation needs no observatory resources. D introduces an explicit runtime context when the terrestrial path needs resources. Preserve recognizable correspondence with TEMPO2 routines and the original calculation order, including the accepted source-corresponding module and helper names. Retain established numerical libraries behind small interfaces when justified by the selected path.

Follow existing TEMPO2 decisions unless evidence demonstrates they are wrong or prone to errors. Preserve defaults and supported fallbacks; do not replace them solely because a different policy seems stricter or cleaner. For every intentional behavioral departure, put explanatory comments beside the changed code identifying the original behavior, the concrete problem and the intended difference. Record the evidence and regression in the report; material scientific changes still belong to central review.

The reference executable is a separate process used for comparison. Invoking the old engine from Rust is useful reference plumbing, but does not complete a Rust timing milestone. The relevant timing logic must execute in the new core; retained external numerical dependencies must be identified.

Use existing functions and established packages before writing replacements. Add only the abstractions required by the current milestone. In particular, the precision experiment must not turn into a production framework with interchangeable numeric backends.

Implementers are not alone in this workspace. Preserve others' edits, stay within ownership, and adapt to accepted changes. Keep the canonical reference source unchanged; A may build a disposable copy in its own directory if the build system writes into the source tree. Do not reset, clean, or broadly stage another chat's files.

## Decisions owned by the central chat

| Decision | Current state | Evidence needed |
| --- | --- | --- |
| Project scope | Faithful core modernization, safer ownership, useful errors | User's scope clarification |
| Reference configuration | Accept A2 `7795d53`, GCC 16.2.0 binary128 on this host, approved initializer correction | Decision 0002 pins the corrected identity; canonical source unchanged |
| Numerical backend and epoch representation | Select Rug 1.30.0, 128-bit significand, direct MJD with source-split phase | B's reviewed source/results; Linux remains untested |
| Fixture schema, conventions, and tolerances | Accept M1a contract in decision 0002 and terrestrial fixtures/gates in decision 0004 | M1a unchanged; D implements a separate terrestrial projection against immutable A2 exports |
| Initial library and CLI contract | Adopt `load` and `evaluate_barycentric`, thin JSON residuals CLI | Decision 0001 defines semantics and boundaries |
| Version-control organization | Primary root repo; preserve and ignore the two independent reference repos | Bootstrap and initial commit `49e122c` complete and verified |
| M0 acceptance | Accepted 19 October 2026 for the declared initial path | Reviewed A2 identity, regressions, unchanged science, schema/projection; B decision retained |
| M1a acceptance | Accepted 20 October 2026 at `49e122c` for the declared macOS arm64 path | Decision 0003 records source review, artifact identity, comparisons and coverage limits |
| M1b fixture scope and release | Adopted PKS path; D released by decision 0004 | A2 identity/configuration; source-compatible meteorology; SOFA/JPL boundary; stage budgets |
| M1b acceptance | Accepted 20 October 2026 for the declared macOS arm64 PKS path | Decision 0006 pins the corrected delivery; both resource findings closed, numerical outputs unchanged, debug/release recovery verified |
| M2 contract and release | Adopted; E released by decision 0007 | Pinned source inspection and executable probe; ZERO/F0–F3, source SVD, MODE/NITS, raw covariance/rescaled errors, separate fit fixtures and gates |
| M2 acceptance | Pending implementation and central review | E's source, identical-input solver checks, end-to-end fitting comparisons, invalid-fit recovery and unchanged M1a/M1b behavior |

The central chat updates this table when a decision is made. Implementation chats can make routine choices inside their accepted contract without requesting approval again. For a choice that changes scientific meaning, time scale, phase convention, shared public output, dependency architecture, or acceptance tolerances, provide the concrete alternatives and evidence for central review. Continue independent work while that decision is unresolved.

## Numerical and behavioral contracts

- Preserve the pinned source's scientific conventions, defaults and supported fallbacks. Justify intentional departures with a demonstrated error or concrete failure risk, explanatory comments at the changed code, a focused regression and the new expectation.
- Parse precision-sensitive decimal inputs directly into the selected representation. Preserve precision through arithmetic, serialization, comparison, and display. Converting an already-rounded `f64` into an extended type does not restore input digits.
- State each fixture's time scale, units, ephemeris, clock realization, reference-phase rule, mean-removal setting, selections, and input ordering. A fit must never be used to conceal a fixed-parameter discrepancy.
- Compare numerical errors using the magnitude of the difference between actual and expected values, with explicit units and any relative scaling defined. Compare identifiers and discrete flags exactly. Document tolerance rationale before accepting results; discrepancies trigger investigation rather than automatic relaxation.
- Preserve TOA identity and original input fields. Reject unsupported scientifically active parameters and directives explicitly. Defaults must be identified and recorded; missing metadata must not be guessed.
- Return defined errors for malformed input, unavailable indispensable resources, unsupported configurations and invalid fits. Preserve adopted TEMPO2 fallbacks, including missing-weather behavior, with warnings and provenance. A failed calculation must not expose stale or apparently complete results. Test failure followed by valid processing when the corresponding library path exists.
- Separate implementation checks, numerical agreement, and scientific corrections in reports. An untested platform remains untested even if it cross-compiles.

## Milestones

| Milestone | Acceptance target |
| --- | --- |
| M0 | A reproducible reference and usable fixture exports, plus an evidence-based choice of numerical representation and a minimal shared interface |
| M1a | A supported PAR and FORMAT 1 TIM subset produces barycentric fixed-parameter phase and residuals in Rust, with numerical and input-error tests |
| M1b | A declared isolated-pulsar observatory path agrees at correction stages and final residuals and handles its resource errors |
| M2 | Selected derivatives and weighted least-squares fitting agree in design matrix, parameter updates, covariance, and residuals, with defined invalid-fit behavior |
| M3 | Additional models and directives are supported in fixture-driven batches with an explicit compatibility table |
| M4 | Supported workflows install reproducibly and have documented library and CLI behavior, provenance, robustness tests, and declared platform coverage |

M2's detailed brief is now available as Assignment E. M3 and M4 receive detailed briefs after the preceding interfaces and coverage are known. Earlier time and token estimates are planning hypotheses; use delivered assignments' measured effort to update them.

## Required return report

Each owner saves the report named in the assignment table and returns its absolute path in that implementation chat. Include:

1. What was delivered, changed file paths, and the source or dependency versions used.
2. Exact commands needed to reproduce the result and actual target architecture/toolchain.
3. Checks run, numerical differences with units and tolerances, and separate existing failures.
4. Supported and unsupported inputs, retained dependencies, and material limitations.
5. Any shared decision needed, with a recommended option and the evidence behind it.
6. Which milestone criteria are satisfied and which remain open.

The user can bring that report back to the central chat for review. Do not assume permission to send messages to other chats. Do not mark central decisions or milestone acceptance complete from an implementation chat.

## Prompts for dispatch

For A, historical initial dispatch; A and A2 have delivered:

```text
Implement assignment A in /Users/Eric.Dong/Downloads/tempo2rust. Read /Users/Eric.Dong/Downloads/tempo2rust/HANDOFF.md and /Users/Eric.Dong/Downloads/tempo2rust/handoffs/01-reference-baseline.md. Follow their ownership and acceptance requirements. This chat owns implementation of the reference baseline; shared design decisions remain in the central planning chat. Deliver the runnable artifacts and required report.
```

For B, historical initial dispatch; its study is now delivered:

```text
Implement assignment B in /Users/Eric.Dong/Downloads/tempo2rust. Read /Users/Eric.Dong/Downloads/tempo2rust/HANDOFF.md and /Users/Eric.Dong/Downloads/tempo2rust/handoffs/02-precision-study.md. Evaluate qd and the specified alternatives with executable evidence. This chat owns the precision study; recommend a backend for central review without installing it into the production core. Deliver the experiment and required report.
```

The completed A2 dispatch remains in [its historical brief](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/01a-reference-correction.md).

For C, historical dispatch; implementation and M1a acceptance are complete:

```text
Implement Assignment C in /Users/Eric.Dong/Downloads/tempo2rust. M0 is accepted and C is released. Read /Users/Eric.Dong/Downloads/tempo2rust/HANDOFF.md, /Users/Eric.Dong/Downloads/tempo2rust/decisions/0001-reference-and-m1a-contract.md, /Users/Eric.Dong/Downloads/tempo2rust/decisions/0002-m0-acceptance-and-c-release.md, and /Users/Eric.Dong/Downloads/tempo2rust/handoffs/03-barycentric-core.md. Own the Rust barycentric core and the explicitly authorized root Git bootstrap. Use the accepted reference at 7795d53 and Rug128 design, preserve the independent repositories and other chats' changes, and deliver the runnable crate, focused checks, and rust/M1A_REPORT.md. Shared design changes and M1a acceptance remain with the central planning chat.
```

For D, historical initial dispatch under decision 0004; its bounded correction is complete and M1b is accepted in [decision 0006](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0006-m1b-acceptance.md):

```text
Implement Assignment D in /Users/Eric.Dong/Downloads/tempo2rust. D is released by /Users/Eric.Dong/Downloads/tempo2rust/decisions/0004-m1b-contract-and-d-release.md. Read that contract, /Users/Eric.Dong/Downloads/tempo2rust/HANDOFF.md, /Users/Eric.Dong/Downloads/tempo2rust/handoffs/04-observatory-core.md, and rust/M1A_REPORT.md. Own the Rust PKS observatory path. Follow existing TEMPO2 decisions unless demonstrated wrong or prone to errors, and explain each intentional departure in comments beside the changed code. Retain the normal meteorology fallback with warnings and provenance; do not introduce the rejected strict-weather policy or an opt-in switch. Preserve the accepted barycentric path, staged planning changes and independent reference repositories. Deliver stage comparisons, resource/fallback/recovery checks and rust/M1B_REPORT.md. Shared design changes and M1b acceptance remain in the central planning chat.
```

For E, ready for a separate implementation chat: use the [complete dispatch prompt](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/05-fitting-core.md) in its handoff. Decision 0007 is the adopted contract; E delivers evidence for central M2 acceptance.
