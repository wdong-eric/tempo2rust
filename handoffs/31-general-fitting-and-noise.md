# Assignment AE: general fitting and noise

**Released by decision 0079 on AD production `a8e5178` / report `0f44180`.** AD implementation handover is approved and its ownership is closed under 0074; inherited numerical acceptance remains pending. AE is the sole Rust writer. This is the functional implementation assignment under 0071.

Work in `/Users/Eric.Dong/Downloads/tempo2rust`. Read HANDOFF.md, `decisions/0071-general-fitting-and-noise-prepared-contract.md`, 0074 handover/acceptance rule, 0065 priorities, 0053 small-bug policy, 0039 generated-output policy, 0050 evidence preservation, 0007/0012/0015 fitting contracts and 0079's AD baseline release. Read `rust/TIMING_MODELS_REPORT.md` and retain its open ledger, including 0077's P diagnosis. Preserve the original 0008 document and apply its later amendments.

Own `rust/` and new `validation/general_fitting/` as the sole Rust writer. You are not alone in the shared repository: preserve other chats' changes, central documents and all accepted reference/verification artifacts. Central retains design and milestone acceptance.

Deliver general local/global fitting from real PAR/TIM/covariance inputs through the released AC/AD canonical pipeline. Extend `timing` with the explicit `--fit` workflow specified in 0071, retaining the existing bounded fit adapters and schemas. Include the remaining source derivative/update families, fit-info/selection and iteration lifecycle, whitening/covariance, constraints/priors, robust and timing-noise processing, legacy wrappers and the scoped standalone fitting APIs. Do not silently insert bootstrap/automatic covariance into a source route that does not call them. Optional native backend and plugin ABI boundaries retain the explicit dispositions in 0071.

Proceed in coherent internal stages:

1. Freeze the canonical API, general fit result, physical/fit-coordinate map and authored public examples against the released AD baseline. Map complete source routines/dispatch and reuse existing E/G/AD evidence. Document remaining standalone source responsibilities.
2. Extend the shared fit-info, callbacks/updates and row/column/global lifecycle. Reuse AD's binary derivatives and the canonical SVD/matrix state. Preserve source precision, operation order, selector mutations and exact output identities.
3. Connect covariance parsing/kernels/whitening, constraints, robust fitting and all scoped TN prefit/postfit processing. Handle actual repeated/context-switch state. Port scoped legacy/standalone helpers and numerical dependencies without a second scientific engine.
4. Connect the full public fit cycles, source side effects, failure/recovery and compatibility adapters. Resolve demonstrated small source bugs under 0053 with adjacent source/cause/fix comments, focused original/corrected evidence and affected neighbors.
5. Verify identical-input components and independent public workflows in both profiles, then affected regressions and required checks. Keep the unchanged M2 raw-order and 0049 findings visible; do not sort modes, change precision/algorithms, relax gates or report a failed gate as accepted.
6. Commit explicit authored owned paths with Conventional Commits and return `rust/GENERAL_FITTING_REPORT.md`, runnable examples and actual evidence identities for central acceptance. Generated large/reproducible outputs remain local and ignored.

Focused source questions already identified in 0071 include nonzero constraint targets in the SVD branch, robust wrapper forwarding, covariance sort/mapping order, constraint/derivative lookup, TN polynomial-cache invalidation and scoped initialization/allocation faults. Establish each cause before correcting it; continue independent implementation while a broader scientific decision needs central disposition.

## Dispatch prompt

Implement Assignment AE under decisions/0071-general-fitting-and-noise-prepared-contract.md, decisions/0074-implementation-handover-and-acceptance.md and handoffs/31-general-fitting-and-noise.md from the AD implementation revision named by central's release. Own rust/ and validation/general_fitting/ as sole Rust writer. Deliver the general public timing --fit workflow, complete built-in local/global fitting and its derivative/update, covariance, constraint, robust and timing-noise lifecycle, plus the scoped standalone APIs. Use the released canonical input/timing engine and numerical boundaries. Reuse existing reference evidence, apply policy 0053 to demonstrated small bugs, preserve original/corrected identities and exact numerical gates, minimize and explain unsafe, commit authored paths only, and return rust/GENERAL_FITTING_REPORT.md for central review.
