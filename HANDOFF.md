# TEMPO2 Rust implementation handoff

Prepared 19 October 2026; updated 23 October 2026 for H reference acceptance and the I/J/K stage releases. This workspace is modernizing TEMPO2's core in Rust while preserving its established scientific calculations. The central design chat owns shared design decisions, scientific conventions, scope changes, and milestone acceptance. Separate implementation chats own the bounded assignments below.

The active objective is to transfer the TEMPO2 core in staged implementation chats, with source behavior preserved and central acceptance. Existing isolated-pulsar milestones are intermediate coverage, not the full completion target. Kalman extensions, plugin binary compatibility, and a new scientific timing framework are outside the initial milestones.

## Workspace and current evidence

Workspace: `/Users/Eric.Dong/Downloads/tempo2rust`.

The reference checkout is `/Users/Eric.Dong/Downloads/tempo2rust/tempo2`, from `https://github.com/wdong-eric/tempo2.git`, pinned to `fda6ad4a9753ad7dc5d291406f2f310a9161423f` and still clean. A delivered the original binary128 reference at `5289797`; A2 delivered the corrected reference at `7795d53`; B delivered the precision study. C delivered the Rust barycentric library and CLI, including the TEMPO2 naming pass, at project commit `49e122c3648e5cb91bae17233a062a444a10f9e4`. **M0, M1a and M1b are accepted within their declared historical scope. E is delivered; M2 remains unaccepted at the exact SVD order gate. F/G/H evidence is accepted for planning/reference use. I is released for the JPL Rust transfer.** Later decisions govern current ownership and preservation requirements.

`tempo2/` and `validation/reference/` are independent Git repositories. The primary project repository tracks the central documents, Rust crate and precision study, and ignores those two nested repositories. E's new fitting-validation directory will also be tracked by the primary repository. The accepted D delivery is now committed as `3f08a8c` (implementation), `31d55bf` (validation) and `87f3933` (design/acceptance documentation), preserving the report/source-manifest hashes in decision 0006. Both independent repositories remain clean. Preserve existing histories and do not flatten or implicitly stage either nested repository. Inspect live Git status before starting new work.

The authoritative initial design is in [decision 0001](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0001-reference-and-m1a-contract.md). [M0 acceptance and C release](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0002-m0-acceptance-and-c-release.md) pins the reference evidence; [M1a acceptance](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0003-m1a-acceptance.md) records the accepted Rust implementation. [M1b contract and D release](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0004-m1b-contract-and-d-release.md) adopts the terrestrial contract and the user's compatibility rule. Later decisions supersede earlier pending-gate wording and historical submission status.

[M1b central review and bounded correction](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0005-m1b-review-and-resource-correction.md) records the original findings and required correction. [M1b acceptance](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0006-m1b-acceptance.md) closes both findings after debug/release checks and accepts the declared macOS arm64 PKS path. Scientific policy and tolerances are unchanged.

[M2 fitting contract and E release](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0007-m2-fitting-contract-and-e-release.md) defines ZERO/F0–F3 fitting, the source SVD and mixed-precision boundaries, weighting, uncertainty rescaling, iteration behavior, fitting fixtures, comparison gates and source result/failure classifications. The linked source probe supports these decisions. [Decision 0008](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0008-preservation-policy-and-stage-plan.md) supersedes its bug-correction policy, requires module counterparts and justified unsafe boundaries, and releases the core/fidelity inventory. [Decision 0009](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0009-binary-reference-stage.md) releases independent binary-model reference evidence while E owns the Rust crate.

[Decision 0010](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0010-svd-mode-comparison.md) establishes paired singular-mode diagnostics. The user's later answer, recorded in [decision 0012](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0012-retain-exact-svd-order-gate.md), retains the exact raw mode-order acceptance gate; M2 remains pending. [Decision 0011](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0011-jpl-reference-stage.md) releases JPL reference evidence before a later safe Rust replacement of the retained reader.

[Decision 0013](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0013-core-inventory-acceptance.md) accepts F's inventory at `15eee12` as planning evidence against baseline `ddc07b2`. It does not accept later implementation, close the behavior ledger or declare any source fully transferred. [Decision 0014](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0014-binary-reference-acceptance.md) accepts G's bounded binary reference evidence at `95d25c1`, `6027e11` and `f550a6e`; Rust model implementation remains separate work.

[Decision 0015](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0015-m2-review-and-correction-closure.md) closes E's requested verifier/correspondence corrections at `75b9e93` and `90b52bb`. E's implementation window is complete; M2 remains unaccepted because the exact raw mode-order gate still fails. Any next Rust-writing assignment requires a separate central release.

[Decision 0016](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0016-jpl-reference-acceptance.md) accepts H's bounded JPL reference evidence and diagnostic-comparison correction at `1df4d37`. Full state/pleph values remain host observations subject to the documented source undefined behavior. The observed stdio buffering must be preserved by the next reader implementation.

[Decision 0017](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0017-jpl-rust-contract-and-i-release.md) releases I to transfer JPL parsing/state/interpolation into Rust while retaining a minimal owned C-stdio boundary. It defines safe dispositions for undefined source operations, exact declared host comparisons, the restoration of canonical cache-before-read behavior and unchanged M1/M2 regression requirements. I owns the Rust implementation window; central retains all acceptance decisions.

[Decision 0018](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0018-clock-table-reference-stage.md) releases J's independent table/clock/active `toa2utc` reference evidence while I works on Rust. It targets remaining deterministic table, fallback and cache departures, and distinguishes literal-disabled legacy clock bodies from active core routines. No new port or milestone is accepted by that release.

The latest scope clarification is in [Estimate Rust Rewrite Effort](chatgpt-conversation://6ad5ede9-5268-83ec-a623-52058d4a2959). All execution requirements needed for the initial tasks are included here and in the linked briefs.

## Assignments and execution order

| Assignment | Owned directory | Start condition | Required report |
| --- | --- | --- | --- |
| [A Reference baseline](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/01-reference-baseline.md) | `/Users/Eric.Dong/Downloads/tempo2rust/validation/reference/` | Delivered and reviewed at `5289797`; see A2 | `REPORT.md` inside the owned directory |
| [B Precision study](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/02-precision-study.md) | `/Users/Eric.Dong/Downloads/tempo2rust/validation/precision/` | Delivered and reviewed; Rug128 selected | `REPORT.md` inside the owned directory |
| [A2 Reference correction](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/01a-reference-correction.md) | `/Users/Eric.Dong/Downloads/tempo2rust/validation/reference/` | Delivered and accepted for M0 at `7795d53` | Updated `REPORT.md` and new revision |
| [C Barycentric timing](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/03-barycentric-core.md) | `/Users/Eric.Dong/Downloads/tempo2rust/rust/` | Delivered at `49e122c`; M1a accepted in decision 0003 | `M1A_REPORT.md` inside the owned directory |
| [D Observatory timing](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/04-observatory-core.md) | `/Users/Eric.Dong/Downloads/tempo2rust/rust/` | Complete; corrected delivery accepted for M1b in decision 0006 | `M1B_REPORT.md` and correction evidence |
| [E Fitting](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/05-fitting-core.md) | `rust/` and new `validation/fitting/` under this workspace | Delivered and corrections reviewed in decision 0015; M2 exact-order gate remains open | `rust/M2_REPORT.md` and fitting reference/verification evidence |
| [F Core and fidelity inventory](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/06-core-fidelity-inventory.md) | `validation/coverage/` only | Delivered at `15eee12`; inventory accepted by decision 0013 | Coverage inventory, behavior-difference ledger and `REPORT.md` |
| [G Binary reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/07-binary-reference.md) | `validation/binary/` only | Delivered; bounded reference evidence accepted by decision 0014 | All 14 binary dispatches, derivative/update/mutation evidence and `REPORT.md` |
| [H JPL reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/08-jpl-reference.md) | `validation/jpl/` only | Delivered and corrected at `1df4d37`; reference accepted by decision 0016 | Kernel/layout/state/file-lifetime evidence and `REPORT.md` |
| [I JPL Rust transfer](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/09-jpl-rust.md) | `rust/` and optional new `validation/jpl_port/`; historical verification immutable | Released by decision 0017 after E ownership closure and H acceptance | `rust/JPL_REPORT.md`, complete source mapping, host comparisons, ownership/unsafe evidence and M1/M2 regressions |
| [J Clock/table reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/10-clock-table-reference.md) | `validation/clocks/` only; delivered evidence now read-only | Executed evidence accepted by decision 0021 at `360cc03`; nine unrun cases remain open | `REPORT.md`, table/route/fallback/cache/toa2utc source evidence and active-profile inventory |
| [K Parameter/PAR reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/11-parameter-par-reference.md) | `validation/parameters/` only | Released by decision 0020; independent of I/J | Complete parameter/dispatch inventory and bounded initialization/shared PAR reference; specialized records and TIM/preprocessing remain later work |

A through H and J have delivered their assigned evidence or implementation. E's M2 gate remains unresolved. I follows E as the Rust owner, with its initial delivery under final boundary correction review; K prepares parameter/PAR evidence. I stores new checks separately. Decision 0008's later preservation policy requires removing the earlier JPL cache correction while retaining the source's deterministic behavior; historical M1b acceptance remains scoped to its recorded revision. Accepted reference and precision directories remain read-only inputs; report needed changes rather than silently changing comparison evidence.

Starting an assignment means opening an implementation chat in this workspace and giving it the appropriate brief. This document does not create or message chats automatically.

**Dispatched 23 October 2026:** E and F started from policy commit `1322f8b` in the local `rustify-tempo2` project. G started from reference-stage contract commit `2f75c30`; H started from `173f0fb`. I started from `d499e94` after E/H ownership closure; J started from `67aa77c`; K started from `9f974ef`. I, J and K have separate write ownership. Acceptance is recorded per assignment in the decisions above; M2 remains pending.

| Chat | Thread ID | Responsibility |
| --- | --- | --- |
| Assignment E: Rust TEMPO2 fitting | `01a16734-4da3-7810-979a-4dad2f647ecd` | Delivered amended M2 implementation and corrections; ownership window closed, acceptance gate pending |
| Assignment F: TEMPO2 core and fidelity inventory | `01a16734-9835-7f53-8a56-79602dc5e2af` | Audit baseline `ddc07b2`; write only `validation/coverage/` |
| Assignment G: TEMPO2 binary reference | `01a1673f-3754-7641-83bd-1d0b03ad06c3` | Prepare all-model source evidence; write only `validation/binary/` |
| Assignment H: TEMPO2 JPL reference | `01a1675e-84aa-7080-b480-1d4cc9919e08` | Delivered and accepted JPL kernel/layout/state/file-lifetime reference; ownership window closed |
| Assignment I: Rust JPL transfer | `01a1678a-ed2a-78c0-819d-a2b914a221bb` | Transfer JPL logic under decision 0017; current bounded owner of `rust/` and optional `validation/jpl_port/` |
| Assignment J: TEMPO2 clock and table reference | `01a16796-3234-7a41-aebf-0943867d9d04` | Delivered reference accepted within its executed scope by decision 0021; ownership window closed, nine cases remain unrun |
| Assignment K: TEMPO2 parameter and PAR reference | `01a167ad-4f08-7061-ae55-4f25abc12a7f` | Prepare parameter state and shared PAR reference under decision 0020; write only `validation/parameters/` |

## Shared working design

Use one Rust crate with a library and a small CLI initially. Owned model and observation values hold the input; fallible operations return results with contextual errors. The initial barycentric operation needs no observatory resources. D introduces an explicit runtime context when the terrestrial path needs resources. Preserve recognizable correspondence with TEMPO2 routines and the original calculation order, including the accepted source-corresponding module and helper names. Retain established numerical libraries behind small interfaces when justified by the selected path.

Preserve existing TEMPO2 behavior, including known bugs, throughout the transfer. Record defects and reproducers for correction after the overall transfer; do not fix them during a port. Decision 0008 supersedes earlier permission to change behavior after demonstrating a bug. It also requires every production module to name its TEMPO2 counterpart and every necessary unsafe boundary to explain why it is needed and which invariants make it valid. F inventories earlier deviations; their previous acceptance does not establish exact source fidelity under the new goal.

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
| Preservation and full-core coverage | Decision 0008 adopted; F released | Deterministic bugs retained, baseline differences reconciled, every core module and unsafe boundary accounted for |
| Binary reference preparation | Decision 0009 adopted; G released | Native and wrapper precision, model-specific updates, source mutations and integrated residual evidence; implementation gates await central review |
| M2 acceptance | Pending exact mode-order gate; E corrections closed in decision 0015 | All other reviewed comparisons pass; keep the four raw-order diagnostics unresolved under decision 0012 |
| SVD diagnostic comparison | Decision 0010 diagnostic only; decision 0012 governs acceptance | Keep exact raw mode-order gate; paired singular modes do not replace it. Numerical budgets and source cutoff controls remain unchanged. |
| JPL reference evidence | Accepted by decision 0016 at corrected H delivery `1df4d37` | Exact host regeneration, source/helper/cache/lifetime evidence and stable isolated failure classifications; no port acceptance |
| JPL Rust contract | I released by decision 0017; title boundary clarified by 0019 | Same-open-file C stdio, safe Rust source logic, explicit undefined-source dispositions, exact host observations and unchanged timing gates; all-whitespace title classified without editing H |
| Clock/table reference | J's executed evidence accepted by decision 0021 at `360cc03` | 142 exact case replays, measured function membership and mutation controls; nine unrun cases and later port dispositions remain open |
| Parameter/PAR reference | K released by decision 0020 | Initialized versus unwritten state, shared scalar grammar, links/defaults and distinct reader lifecycles; explicit later parser coverage |

The central chat updates this table when a decision is made. Implementation chats can make routine choices inside their accepted contract without requesting approval again. For a choice that changes scientific meaning, time scale, phase convention, shared public output, dependency architecture, or acceptance tolerances, provide the concrete alternatives and evidence for central review. Continue independent work while that decision is unresolved.

## Numerical and behavioral contracts

- Preserve the pinned source's scientific conventions, defaults, supported fallbacks and deterministic bugs. Record known defects for later correction; do not add a behavior change as part of the transfer. Identify undefined-source cases and observable adapter differences explicitly under decision 0008.
- Parse precision-sensitive decimal inputs directly into the selected representation. Preserve precision through arithmetic, serialization, comparison, and display. Converting an already-rounded `f64` into an extended type does not restore input digits.
- State each fixture's time scale, units, ephemeris, clock realization, reference-phase rule, mean-removal setting, selections, and input ordering. A fit must never be used to conceal a fixed-parameter discrepancy.
- Compare numerical errors using the magnitude of the difference between actual and expected values, with explicit units and any relative scaling defined. Compare identifiers and discrete flags exactly. Document tolerance rationale before accepting results; discrepancies trigger investigation rather than automatic relaxation.
- Preserve TOA identity and original input fields. Reject unsupported scientifically active parameters and directives explicitly. Defaults must be identified and recorded; missing metadata must not be guessed.
- Preserve source result and failure classifications, including defined nonfinite fit outputs. Record explicit fallible-library boundary differences where upstream terminates or produces undefined state; do not invent values to simulate it. Preserve source fallbacks, including missing-weather behavior, with warnings and provenance. Historical recoverable-error guards and state changes remain compatibility inventory items until reviewed under decision 0008.
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

M2's amended brief is available as Assignment E. Assignment F inventories the entire core, baseline departures, module correspondence and unsafe sites to support the remaining transfer. Assignment G prepares source-driven binary references, without changing Rust or accepting model coverage. M3 and M4 receive detailed implementation briefs after the preceding interfaces and coverage are known. Earlier time and token estimates are planning hypotheses; use delivered assignments' measured effort to update them.

## Required return report

Each owner saves the report named in the assignment table and returns its absolute path in that implementation chat. Include:

1. What was delivered, changed file paths, and the source or dependency versions used.
2. Exact commands needed to reproduce the result and actual target architecture/toolchain.
3. Checks run, numerical differences with units and tolerances, and separate existing failures.
4. Supported and unsupported inputs, retained dependencies, and material limitations.
5. Any shared decision needed, with a recommended option and the evidence behind it.
6. Which milestone criteria are satisfied and which remain open.

The user can bring that report back to the central chat for review. On 23 October 2026 the user explicitly authorized the central chat to coordinate implementation chats, including relaying central decisions and correction briefs. Central may therefore read delivery reports and send those follow-ups directly. This does not authorize implementation chats to message arbitrary chats or accept milestones. Do not mark central decisions or milestone acceptance complete from an implementation chat.

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

E/F/G/H dispatch prompts remain historical in their handoffs. I follows its [JPL transfer prompt](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/09-jpl-rust.md) under decision 0017; J follows its [clock/table reference prompt](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/10-clock-table-reference.md) under decision 0018. Preserve E's unresolved M2 gate, all accepted reference evidence and the central acceptance boundary.
