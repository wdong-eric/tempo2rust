# Implementation handover and milestone acceptance

Central decision, 25 October 2026. Separate release of a successor implementation assignment from scientific acceptance of its predecessor. This applies the user's functional-coverage priority and permits temporary, explicitly tracked gaps while the full migration proceeds. Full core scope, numerical conventions, required gates and the final completion audit remain unchanged.

This supersedes only the requirement in prepared decisions 0070/0071 and handoffs 30/31 that AC/AD must be formally accepted before AD/AE can start. It does not itself release AD or AE, close AC ownership, approve a numerical mismatch, or accept a milestone. Central must still issue a separate release naming the actual baseline and its acceptance status.

## Conditions for a successor implementation release

1. The predecessor delivers its in-scope implementation, stable canonical interfaces and runnable public workflow examples at a committed revision. The next stage must have a real functional foundation; a reference-only delivery or a proposed API is insufficient.
2. Its affected verification has been executed and reported truthfully, with source/build identities, pass/failure distinctions and reproducible evidence. Correct identified translation, ownership and integration errors within the existing contract. An unchanged numerical-policy finding does not require repeated runs while awaiting central disposition.
3. Every unresolved gate has a named input/check, observed result, current localization, affected scope and responsible owner. Central determines whether the successor can proceed meaningfully with those limitations. A missing dependency, unsafe ownership boundary or unbounded failure affecting the next stage can still prevent release.
4. Central reviews the delivery for this implementation handover, pins the exact revision, records whether acceptance is pending and closes the predecessor's Rust ownership before assigning the next writer. There is one Rust writer. Passing a handover review does not imply passing numerical or scientific acceptance.
5. The successor inherits the explicit open-check ledger, retains the reproductions and reports new failures separately. Do not delete checks, widen tolerances, make all similar failures expected, relabel failures as passes or claim an unaccepted dependency is accepted. Central owns cross-stage numerical decisions; after ownership transfer, implementation of a central correction belongs to the current Rust owner.

The full migration remains incomplete until every required capability and acceptance condition has an evidenced resolution. M2's exact raw SVD gate, the four 0049 utility boundaries, DDH's original 1 ns check and new in-scope compatibility gaps remain obligations. Future policy changes require explicit central decisions and supporting evidence; this sequencing decision supplies none.

## Current application to AC

AC remains active and owns Rust. It should finish the general input-to-BAT implementation and public/debug/release checks, correct translation defects, and deliver `rust/INPUT_TO_BAT_REPORT.md` with an explicit unresolved-check ledger. It may deliver while a localized central numerical decision remains pending. Central will then separately decide implementation handover, milestone acceptance and AD release. AD's full residual/binary implementation must not be replaced by another independent reference-only assignment.

AC's P preprocessing diagnostic demonstrates another precision-dependent discrete boundary. For SATs `58000` and `58000.0006944444444444444444444444444444`, the source expression `fabs(SAT_i-SAT_j)*SECDAY < 60` in `preProcess.C::processSimultaneous` selects different branches:

| Evaluation precision | Separation in seconds | Strictly below 60 |
| --- | --- | --- |
| Native-compatible 113 bits | `60.0000000000000000000000001696366491` | false |
| Production 128 bits | `59.99999999999999999999999999999149509337` | true |
| 256/512-bit diagnosis | approaches `59.99999999999999999999999999999616` | true |

The 113-bit diagnostic reproduces the SAT bits in the accepted native record; the source ordering and comparison were inspected centrally. This changes deletion state in the SIMUL cases and is an unresolved compatibility gap. It is not covered by 0073's continuous rounding bounds. Do not adjust the threshold, silently round production values to 113 bits or describe this as an exact native match. Its final numerical disposition remains central, alongside 0049. Full P/Rust verification and AC acceptance are still pending; no independent full native replay is claimed for this review.

Evidence: original `validation/preprocessing/observations.json.gz`, SHA-256 `1e44d2446fa37ce22cbe0de36fbd1c2879b0005861c94819181b7e97a13999f2`; AC's extracted `build/preprocessing/simul-native.json`, SHA-256 `ee11dfdb8758a601c30c2a4c4d66b7bb05e365a839e51784f9c9e6364172d5a0`; and `validation/input_to_bat/build/preprocessing/simul-precision-diagnosis.json`, SHA-256 `7187511076f94cdb238316b42b064f79ea0f901b6f1117f028696917a0f0f0d3`. The diagnostic is `p_precision_diagnosis` in AC's `rust/src/preprocessing_corpus_tests.rs`. Generated evidence remains untracked under 0039.

## Floating-class projection clarification

Under 0068's numerical-class requirement, compare NaN class/sign, infinity sign and signed zero explicitly. MPFR does not retain native hardware NaN payload bits. Where P's typed fields also appear in raw parameter/observation projections, normalize only the identified NaN payloads in those typed locations, retaining every surrounding byte, field and membership check. Preserve the original native records and disclose the transport normalization. This is not a finite-value allowance, a reason to omit a whole record or permission to hide a changed discrete field. It does not change native self-regeneration or the existing finite numerical gates.
