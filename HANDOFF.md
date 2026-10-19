# TEMPO2 Rust implementation handoff

Prepared 19 October 2026. This workspace is modernizing TEMPO2's core in Rust while preserving its established scientific calculations. The central design chat owns shared design decisions, scientific conventions, scope changes, and milestone acceptance. Separate implementation chats own the bounded assignments below.

The immediate objective is one supported calculation path with demonstrated numerical agreement and useful failure behavior. Kalman extensions, plugin binary compatibility, and a new scientific timing framework are outside the initial milestones.

## Workspace and current evidence

Workspace: `/Users/Eric.Dong/Downloads/tempo2-rust`.

The reference checkout is `/Users/Eric.Dong/Downloads/tempo2-rust/tempo2`, from `https://github.com/wdong-eric/tempo2.git`, pinned to `fda6ad4a9753ad7dc5d291406f2f310a9161423f` and still clean. A delivered the original binary128 reference at `5289797`; A2 delivered the corrected reference at `7795d53`; B delivered the precision study. Their reports and saved evidence have been reviewed. **M0 is accepted and C is released for implementation**, as recorded in decision 0002. No Rust timing core exists yet; M1a and M1b remain pending.

`tempo2/` and `validation/reference/` are independent Git repositories. The adopted plan is a primary project repository at this workspace root, ignoring those two nested repositories and tracking the central documents, Rust crate, and precision study. C may now perform that bootstrap. Preserve existing histories and do not flatten or implicitly stage either nested repository.

The authoritative design is in [Reference configuration and initial Rust contract](/Users/Eric.Dong/Downloads/tempo2-rust/decisions/0001-reference-and-m1a-contract.md); [M0 acceptance and C release](/Users/Eric.Dong/Downloads/tempo2-rust/decisions/0002-m0-acceptance-and-c-release.md) pins the accepted evidence and current gate status. These decisions supersede the original candidate-selection and pending-gate wording in historical assignment reports.

The latest scope clarification is in [Estimate Rust Rewrite Effort](chatgpt-conversation://6ad5ede9-5268-83ec-a623-52058d4a2959). All execution requirements needed for the initial tasks are included here and in the linked briefs.

## Assignments and execution order

| Assignment | Owned directory | Start condition | Required report |
| --- | --- | --- | --- |
| [A Reference baseline](/Users/Eric.Dong/Downloads/tempo2-rust/handoffs/01-reference-baseline.md) | `/Users/Eric.Dong/Downloads/tempo2-rust/validation/reference/` | Delivered and reviewed at `5289797`; see A2 | `REPORT.md` inside the owned directory |
| [B Precision study](/Users/Eric.Dong/Downloads/tempo2-rust/handoffs/02-precision-study.md) | `/Users/Eric.Dong/Downloads/tempo2-rust/validation/precision/` | Delivered and reviewed; Rug128 selected | `REPORT.md` inside the owned directory |
| [A2 Reference correction](/Users/Eric.Dong/Downloads/tempo2-rust/handoffs/01a-reference-correction.md) | `/Users/Eric.Dong/Downloads/tempo2-rust/validation/reference/` | Delivered and accepted for M0 at `7795d53` | Updated `REPORT.md` and new revision |
| [C Barycentric timing](/Users/Eric.Dong/Downloads/tempo2-rust/handoffs/03-barycentric-core.md) | `/Users/Eric.Dong/Downloads/tempo2-rust/rust/` | Released; M0 accepted in decision 0002 | `M1A_REPORT.md` inside the owned directory |
| [D Observatory timing](/Users/Eric.Dong/Downloads/tempo2-rust/handoffs/04-observatory-core.md) | `/Users/Eric.Dong/Downloads/tempo2-rust/rust/` | C has finished; central chat accepts M1a and fixes the terrestrial fixture contract | `M1B_REPORT.md` inside the owned directory |

A, A2, and B have delivered. C is the next implementation assignment; D remains gated. C and D are sequential owners of the same Rust crate. The reference and precision directories are inputs to C and D; those chats should report needed changes to their owners rather than silently changing the evidence they are being tested against.

Starting an assignment means opening an implementation chat in this workspace and giving it the appropriate brief. This document does not create or message chats automatically.

## Shared working design

Use one Rust crate with a library and a small CLI initially. Owned model and observation values hold the input; fallible operations return results with contextual errors. The initial barycentric operation needs no observatory resources. D introduces an explicit runtime context when the terrestrial path needs resources. Preserve recognizable correspondence with TEMPO2 routines and the original calculation order. Retain established numerical libraries behind small interfaces when justified by the selected path.

The reference executable is a separate process used for comparison. Invoking the old engine from Rust is useful reference plumbing, but does not complete a Rust timing milestone. The relevant timing logic must execute in the new core; retained external numerical dependencies must be identified.

Use existing functions and established packages before writing replacements. Add only the abstractions required by the current milestone. In particular, the precision experiment must not turn into a production framework with interchangeable numeric backends.

Implementers are not alone in this workspace. Preserve others' edits, stay within ownership, and adapt to accepted changes. Keep the canonical reference source unchanged; A may build a disposable copy in its own directory if the build system writes into the source tree. Do not reset, clean, or broadly stage another chat's files.

## Decisions owned by the central chat

| Decision | Current state | Evidence needed |
| --- | --- | --- |
| Project scope | Faithful core modernization, safer ownership, useful errors | User's scope clarification |
| Reference configuration | Accept A2 `7795d53`, GCC 16.2.0 binary128 on this host, approved initializer correction | Decision 0002 pins the corrected identity; canonical source unchanged |
| Numerical backend and epoch representation | Select Rug 1.30.0, 128-bit significand, direct MJD with source-split phase | B's reviewed source/results; Linux remains untested |
| Fixture schema, conventions, and tolerances | Accept schema 2 `barycentric-v1` and three goldens in decision 0002 | Adopted barycentric gates unchanged; terrestrial budgets remain proposed |
| Initial library and CLI contract | Adopt `load` and `evaluate_barycentric`, thin JSON residuals CLI | Decision 0001 defines semantics and boundaries |
| Version-control organization | Primary root repo; preserve and ignore the two independent reference repos | C may now perform the defined bootstrap |
| M0 acceptance | Accepted 19 October 2026 for the declared initial path | Reviewed A2 identity, regressions, unchanged science, schema/projection; B decision retained |
| M1a acceptance | Pending | C's report and reproducible comparisons |
| M1b fixture scope and acceptance | Pending | A's terrestrial evidence followed by D's implementation report |

The central chat updates this table when a decision is made. Implementation chats can make routine choices inside their accepted contract without requesting approval again. For a choice that changes scientific meaning, time scale, phase convention, shared public output, dependency architecture, or acceptance tolerances, provide the concrete alternatives and evidence for central review. Continue independent work while that decision is unresolved.

## Numerical and behavioral contracts

- Preserve the pinned source's scientific conventions for supported valid inputs. Record intentional corrections separately with a reproducer, the reason the original result is wrong, and the new expectation.
- Parse precision-sensitive decimal inputs directly into the selected representation. Preserve precision through arithmetic, serialization, comparison, and display. Converting an already-rounded `f64` into an extended type does not restore input digits.
- State each fixture's time scale, units, ephemeris, clock realization, reference-phase rule, mean-removal setting, selections, and input ordering. A fit must never be used to conceal a fixed-parameter discrepancy.
- Compare numerical errors using the magnitude of the difference between actual and expected values, with explicit units and any relative scaling defined. Compare identifiers and discrete flags exactly. Document tolerance rationale before accepting results; discrepancies trigger investigation rather than automatic relaxation.
- Preserve TOA identity and original input fields. Reject unsupported scientifically active parameters and directives explicitly. Defaults must be identified and recorded; missing metadata must not be guessed.
- Treat malformed input, unavailable resources, unsupported configurations, and invalid fits as defined failures. A failed calculation must not expose stale or apparently complete results. Test failure followed by valid processing when the corresponding library path exists.
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

M2 through M4 receive detailed implementation briefs after the preceding interfaces and coverage are known. Earlier time and token estimates are planning hypotheses; use A through D's measured effort to update them.

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
Implement assignment A in /Users/Eric.Dong/Downloads/tempo2-rust. Read /Users/Eric.Dong/Downloads/tempo2-rust/HANDOFF.md and /Users/Eric.Dong/Downloads/tempo2-rust/handoffs/01-reference-baseline.md. Follow their ownership and acceptance requirements. This chat owns implementation of the reference baseline; shared design decisions remain in the central planning chat. Deliver the runnable artifacts and required report.
```

For B, historical initial dispatch; its study is now delivered:

```text
Implement assignment B in /Users/Eric.Dong/Downloads/tempo2-rust. Read /Users/Eric.Dong/Downloads/tempo2-rust/HANDOFF.md and /Users/Eric.Dong/Downloads/tempo2-rust/handoffs/02-precision-study.md. Evaluate qd and the specified alternatives with executable evidence. This chat owns the precision study; recommend a backend for central review without installing it into the production core. Deliver the experiment and required report.
```

The completed A2 dispatch remains in [its historical brief](/Users/Eric.Dong/Downloads/tempo2-rust/handoffs/01a-reference-correction.md).

For C, released now:

```text
Implement Assignment C in /Users/Eric.Dong/Downloads/tempo2-rust. M0 is accepted and C is released. Read /Users/Eric.Dong/Downloads/tempo2-rust/HANDOFF.md, /Users/Eric.Dong/Downloads/tempo2-rust/decisions/0001-reference-and-m1a-contract.md, /Users/Eric.Dong/Downloads/tempo2-rust/decisions/0002-m0-acceptance-and-c-release.md, and /Users/Eric.Dong/Downloads/tempo2-rust/handoffs/03-barycentric-core.md. Own the Rust barycentric core and the explicitly authorized root Git bootstrap. Use the accepted reference at 7795d53 and Rug128 design, preserve the independent repositories and other chats' changes, and deliver the runnable crate, focused checks, and rust/M1A_REPORT.md. Shared design changes and M1a acceptance remain with the central planning chat.
```

For D, after its start condition is satisfied:

```text
Implement assignment D in /Users/Eric.Dong/Downloads/tempo2-rust. Read /Users/Eric.Dong/Downloads/tempo2-rust/HANDOFF.md and /Users/Eric.Dong/Downloads/tempo2-rust/handoffs/04-observatory-core.md, including the accepted M1a report and terrestrial fixture contract. This chat owns the isolated-pulsar observatory path. Preserve the existing barycentric path and deliver stage comparisons, resource-error checks, and the required report.
```
