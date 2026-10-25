# TEMPO2 Rust implementation handoff

Prepared 19 October 2026; updated 25 October 2026 for Y acceptance, AA implementation release and functional delivery priorities. This workspace is modernizing TEMPO2's core in Rust while preserving its established scientific calculations. The central design chat owns shared design decisions, scientific conventions, scope changes, and milestone acceptance. Separate implementation chats own the bounded assignments below.

The active objective is to transfer the TEMPO2 core in staged implementation chats, preserving established behavior except for documented, tested corrections of demonstrated bugs under decision 0053, with central acceptance. Existing isolated-pulsar milestones are intermediate coverage, not the full completion target. Kalman extensions, plugin binary compatibility, and a new scientific timing framework are outside the initial milestones.

## Full-migration readiness — 25 October 2026

The full TEMPO2 core migration remains incomplete. The usable public workflow is still the declared single-isolated-pulsar F0–F3 subset with barycentric `@` or PKS observations. M0, M1a and M1b have scoped acceptance; M2 remains unaccepted at its four historical raw SVD-order comparisons. Completing more reference assignments does not expand the supported Rust frontend by itself.

| Workstream | Current evidence and remaining transfer |
| --- | --- |
| Canonical state and lifecycle | Parameter storage, all 95 observation fields, all 349 pulsar fields and the released lifecycle routines have accepted Rust transfers. Full parser/preprocessing use of that state remains. |
| Resource and time infrastructure | JPL, clock/table and the released observatory/weather/EOP/tropospheric routines have accepted Rust transfers. Y's full 29-routine time-ephemeris transfer is accepted at `21dc956` (production `11165d1`) by 0066. |
| Astrometry, ephemeris and propagation | Existing frontend paths remain bounded. X's native observer-coordinate reference is accepted and AA's Rust implementation is released by 0066. Z's planetary reference at `d517acf` is accepted by 0064; AB delivered the propagation/dispersion reference at `cb2c41f` and awaits central review. Their full Rust transfers remain. General frame/site/unit, secular-motion and reference-phase integration must follow the complete contracts. |
| Inputs and preprocessing | K/Q/N/P provide accepted bounded native evidence. Full PAR/TIM grammars, directives, selection, preprocessing and multi-pulsar input lifecycle are not yet implemented in the public Rust workflow. |
| Residual and binary models | Full phase tracking, higher spin terms, glitches, jumps, waves, GW/noise contributions and the binary families still require implementation and integration. G's binary reference evidence is preparation, not Rust model acceptance. |
| Fitting | ZERO/F0–F3 fitting is delivered but M2 is unaccepted. Remaining derivatives/updates, correlated-noise and robust fitting, constraints and global fits remain larger implementation stages. |
| Toolkit, GW/spectrum and predictors | Selected helpers are present; the complete source toolkit APIs, GW/spectrum routines, polyco/Chebyshev predictors and predictor interface remain open. |
| Integration and release | Complete core entry-point/output behavior, broader real-data workflows, installation, platform coverage, performance and concurrency validation remain. Current checks principally establish the declared macOS arm64 scope. |

The full source inventory remains `validation/coverage/SOURCE_INVENTORY.md` and its dependency plan is `validation/coverage/STAGE_PLAN.md`. Their original Rust counts describe baseline `ddc07b2`, not current completion. File counts, routine counts and passing reference tests are not a defensible percentage of remaining implementation effort. The rows above group remaining work; they are not equally sized assignments or a calendar estimate.

Y and Z review is complete. Y ownership is closed and AA is released by 0066. Central reviews AB while observer-coordinate implementation proceeds. Planetary and propagation Rust handoffs then build on accepted references and the canonical state/resource interfaces. Full input/preprocessing, residual/binary, fitting and predictor stages remain on the dependency plan; no accepted subset removes them from the objective. The four 0049 discrete precision-boundary differences and the unchanged M2 gate remain explicit open decisions. The original DDH ideal-fixture discrepancy remains broader binary-validation work. Small demonstrated source bugs may be corrected under 0053, without changing numerical policy or silently replacing native evidence.

## Implementation priority

[Decision 0065](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0065-functional-coverage-and-review-priorities.md) prioritizes complete usable capabilities: general input-to-BAT processing, full timing/residual models, general fitting/noise, remaining core APIs and integrated release qualification. Reuse existing references and create new native experiments for concrete unresolved questions. Verification follows affected behavior; central does not repeat every historical suite for an isolated comment or report change. Keep exact prior evidence identities, full source scope and the user's numerical gates. A discrepancy outside a stage's supported capability stays visible without blocking independent implementation.

Immediate work: execute the released AA implementation, review AB while implementation proceeds, and assemble the next implementation handoff around planetary/propagation and public input/frontend integration. Each implementation brief must identify an executable user-visible deliverable; reference counts alone do not establish progress in supported functionality.

## Current source-bug policy

[Decision 0053](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0053-small-upstream-bug-fixes-during-transfer.md) implements the user's latest instruction: **small, demonstrated TEMPO2 bugs may be fixed during the rewrite.** It supersedes 0008's blanket deferral requirement wherever inherited by older handoffs or the historical goal text. Each fix needs an adjacent comment naming the original source file/routine/location, explaining the bug and correction, a focused regression and an explicit departure in the report. Follow normal TEMPO2 decisions otherwise. Preserve original reference evidence; central reviews the corrected contract and milestone acceptance. Routine fixes within an assignment do not need renewed user approval.

## Generated evidence policy

[Decision 0039](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0039-generated-validation-output-policy.md) implements the user's instruction: **do not commit reproducible validation JSON, compressed JSON or other large generated evidence.** Keep outputs local and preserve their paths/hashes for review. Commit sources, generators, authored fixtures/contracts and concise reports. This supersedes earlier instructions in any handoff to commit generated results; never force-add ignored outputs. Existing local accepted evidence remains immutable during unrelated work. [Validation artifact guidance](/Users/Eric.Dong/Downloads/tempo2rust/validation/README.md) explains recording and review.

## Workspace and current evidence

Workspace: `/Users/Eric.Dong/Downloads/tempo2rust`.

The reference checkout is `/Users/Eric.Dong/Downloads/tempo2rust/tempo2`, from `https://github.com/wdong-eric/tempo2.git`, pinned to `fda6ad4a9753ad7dc5d291406f2f310a9161423f` and still clean. A delivered the original binary128 reference at `5289797`; A2 delivered the corrected reference at `7795d53`; B delivered the precision study. C delivered the Rust barycentric library and CLI, including the TEMPO2 naming pass, at project commit `49e122c3648e5cb91bae17233a062a444a10f9e4`. **M0, M1a and M1b are accepted within their declared historical scope. E is delivered; M2 remains unaccepted at the exact SVD order gate. F/G/H/J/K/M/N/P/Q/S/U/V/X/Z evidence is accepted within its declared planning/reference scope. I's JPL, L's clock/table, O's parameter-storage, R's observation-storage, T's pulsar-state/lifecycle and W's terrestrial-resource Rust transfers are accepted within their bounded scopes.** Later decisions govern current ownership and preservation requirements.

`tempo2/` and `validation/reference/` are independent Git repositories. The primary project repository tracks the central documents, Rust crate and precision study, and ignores those two nested repositories. E's new fitting-validation directory will also be tracked by the primary repository. The accepted D delivery is now committed as `3f08a8c` (implementation), `31d55bf` (validation) and `87f3933` (design/acceptance documentation), preserving the report/source-manifest hashes in decision 0006. Both independent repositories remain clean. Preserve existing histories and do not flatten or implicitly stage either nested repository. Inspect live Git status before starting new work.

The authoritative initial design is in [decision 0001](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0001-reference-and-m1a-contract.md). [M0 acceptance and C release](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0002-m0-acceptance-and-c-release.md) pins the reference evidence; [M1a acceptance](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0003-m1a-acceptance.md) records the accepted Rust implementation. [M1b contract and D release](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0004-m1b-contract-and-d-release.md) adopts the terrestrial contract and the user's compatibility rule. Later decisions supersede earlier pending-gate wording and historical submission status.

[M1b central review and bounded correction](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0005-m1b-review-and-resource-correction.md) records the original findings and required correction. [M1b acceptance](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0006-m1b-acceptance.md) closes both findings after debug/release checks and accepts the declared macOS arm64 PKS path. Scientific policy and tolerances are unchanged.

[M2 fitting contract and E release](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0007-m2-fitting-contract-and-e-release.md) defines ZERO/F0–F3 fitting, the source SVD and mixed-precision boundaries, weighting, uncertainty rescaling, iteration behavior, fitting fixtures, comparison gates and source result/failure classifications. The linked source probe supports these decisions. [Decision 0008](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0008-preservation-policy-and-stage-plan.md) historically deferred bug fixes and still requires module counterparts, justified unsafe boundaries and the core/fidelity inventory; decision 0053 now supersedes its blanket bug-fix prohibition. [Decision 0009](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0009-binary-reference-stage.md) releases independent binary-model reference evidence while E owns the Rust crate.

[Decision 0010](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0010-svd-mode-comparison.md) establishes paired singular-mode diagnostics. The user's later answer, recorded in [decision 0012](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0012-retain-exact-svd-order-gate.md), retains the exact raw mode-order acceptance gate; M2 remains pending. [Decision 0011](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0011-jpl-reference-stage.md) releases JPL reference evidence before a later safe Rust replacement of the retained reader.

[Decision 0013](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0013-core-inventory-acceptance.md) accepts F's inventory at `15eee12` as planning evidence against baseline `ddc07b2`. It does not accept later implementation, close the behavior ledger or declare any source fully transferred. [Decision 0014](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0014-binary-reference-acceptance.md) accepts G's bounded binary reference evidence at `95d25c1`, `6027e11` and `f550a6e`; Rust model implementation remains separate work.

[Decision 0015](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0015-m2-review-and-correction-closure.md) closes E's requested verifier/correspondence corrections at `75b9e93` and `90b52bb`. E's implementation window is complete; M2 remains unaccepted because the exact raw mode-order gate still fails. Any next Rust-writing assignment requires a separate central release.

[Decision 0016](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0016-jpl-reference-acceptance.md) accepts H's bounded JPL reference evidence and diagnostic-comparison correction at `1df4d37`. Full state/pleph values remain host observations subject to the documented source undefined behavior. The observed stdio buffering must be preserved by the next reader implementation.

[Decision 0017](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0017-jpl-rust-contract-and-i-release.md) releases I to transfer JPL parsing/state/interpolation into Rust while retaining a minimal owned C-stdio boundary. It defines safe dispositions for undefined source operations, exact declared host comparisons, the restoration of canonical cache-before-read behavior and unchanged M1/M2 regression requirements. I owns the Rust implementation window; central retains all acceptance decisions.

[Decision 0018](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0018-clock-table-reference-stage.md) releases J's independent table/clock/active `toa2utc` reference evidence while I works on Rust. It targets remaining deterministic table, fallback and cache departures, and distinguishes literal-disabled legacy clock bodies from active core routines. No new port or milestone is accepted by that release.

The latest scope clarification is in [Estimate Rust Rewrite Effort](chatgpt-conversation://6ad5ede9-5268-83ec-a623-52058d4a2959). All execution requirements needed for the initial tasks are included here and in the linked briefs.

[Decision 0021](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0021-clock-reference-acceptance.md) accepts J's executed clock/table evidence at `360cc03`; nine unrun cases remain open. [Decision 0022](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0022-jpl-rust-acceptance.md) accepts I's bounded Rust JPL delivery at `902fbba`, executed implementation `27c8384`, and closes its ownership window. These supersede the earlier dispatch status above. A later Rust owner requires a separate central release.

[Decision 0023](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0023-clock-table-rust-contract-and-l-release.md) releases L as the next bounded Rust owner. It requires process-lifetime clock/selected lookup/diagnostic state, source table and routing behavior, active SAT-jump semantics, explicit safe boundaries and unchanged numerical gates. K's parameter/PAR reference delivery is now accepted by 0026 after its identity correction; no Rust parser stage is released yet.

[Decision 0024](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0024-parameter-reference-review-correction.md) records central's exact replay of K's 306 cases and returns a verifier identity defect for correction. The `--no-build` shortcut can attribute a changed executable to the frozen reference hash. Decision 0026 closes this finding after correction; reference values are unchanged.

[Decision 0025](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0025-numeric-utility-reference-stage.md) releases M's independent TKlongdouble/tempo2Util reference stage. It covers shared numeric, text, conversion, copy/link and diagnostic responsibilities while L owns Rust. It changes no numerical representation or acceptance gate.

[Decision 0026](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0026-parameter-reference-acceptance.md) accepts K's bounded parameter/PAR reference at corrected delivery `38d62af`. Central's independent replay and focused identity controls close 0024 without changing scientific evidence. K's ownership window is closed; full parser/TIM implementation and M2 remain separate.

[Decision 0027](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0027-clock-interpolation-contraction.md) records the user's approval in L's chat for one final interpolation `mul_add`, matching the pinned GCC `fmadd`. It supersedes 0023's no-fusion clause only at that expression; exact gates and final regression requirements remain unchanged.

[Decision 0028](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0028-tim-io-reference-stage.md) releases N's independent TIM reading/writing, JBO/Fortran I/O and observation-ordering reference. It preserves include/repeat-call history and distinguishes host observations from defined results. General preprocessing and a Rust input-lifecycle contract remain later work.

[Decision 0029](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0029-clock-glob-review-correction.md) returns L's delivered `189fed3` for a glob fidelity correction. Central reproduced eight membership/order/path differences against the raw native helper. Decision 0031 closes the correction after independent verification of the corrected delivery.

[Decision 0030](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0030-numeric-reference-acceptance.md) accepts M's bounded numeric reference at `a01d428`, final evidence `e754581`. Central independently rebuilt all 350 cases / 1,207 events and 35 normal verifier controls. M's ownership window is closed; its ABI/unrun limitations remain explicit.

[Decision 0031](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0031-clock-rust-acceptance.md) accepts L's bounded clock/table Rust transfer at corrected delivery `84b6b86`. Central independently rebuilt the exact J and supplemental comparisons in both profiles, checked final source/artifact identities and closed 0029. L's ownership window is closed; M2 and broader core coverage remain open.

[Decision 0032](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0032-tim-reference-acceptance.md) accepts N's bounded TIM I/O reference at `68de89a`. Central independently rebuilt all 157 cases / 906 events and 24 normal verifier controls, verified the actual binary/artifact identities and retained the JBO diagnostic/other host limits. N's ownership window is closed; a Rust input-lifecycle contract remains separate.

[Decision 0033](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0033-parameter-storage-contract-and-o-release.md) releases O as the next Rust owner. It transfers source-indexed parameter storage, allocation/labels and copy semantics, then integrates one canonical store into existing loading/evaluation/fitting. The strict frontend, Rug128, CLI/JSON schema and numerical gates remain unchanged; `parameters()` may return an owned compatibility snapshot. Full PAR/TIM and linked-value arithmetic remain later work.

[Decision 0034](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0034-preprocessing-reference-stage.md) releases P's independent preprocessing, selection/noise, unit and epoch reference across 15 primary routines. It requires explicit caller histories, source defect preservation and separate undefined/host classifications, including the main wrapper's uninitialized no-setdm argument. P owns only its new reference directory; O retains sole Rust ownership.

[Decision 0035](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0035-specialized-par-reference-stage.md) releases Q's independent specialized PAR reference over K's 116 deferred top-level branches and the nested FD-jump subtree. It preserves ordered token consumption, retained state and deterministic parser defects, with explicit undefined-path classifications. Q owns only its new reference directory; O and P retain their existing ownership.

[Decision 0036](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0036-parameter-storage-acceptance.md) accepts O's parameter-storage/copy foundation and canonical-store integration at `8d2be6f`, after independent fresh debug/release direct comparisons and committed regression-evidence checks. O's ownership window is closed. Rug128, the strict frontend and numerical gates remain unchanged; M2 remains pending.

[Decision 0037](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0037-observation-storage-contract-and-r-release.md) releases R as the next sole Rust owner. It transfers all 95 source observation fields, explicit unwritten state and fixed backing, integrates existing input/evaluation/fitting access and consolidates the clock observation representation. The owned observations snapshot is authorized; strict parsing, JSON output, Rug128 and numerical gates remain unchanged. Full pulsar state and source readers remain subsequent input-lifecycle work.

[Decision 0038](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0038-pulsar-lifecycle-reference-stage.md) releases S's independent whole-pulsar state/lifecycle reference: all 349 fields and twelve initialization, allocation/accessor, destruction and copy routines. It distinguishes native covariance zeros from unwritten observations and records retained, repeated and alias-dependent behavior before a later Rust ownership contract. S owns only its new reference directory; R retains sole Rust ownership.

[Decision 0040](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0040-preprocessing-reference-acceptance.md) accepts P's bounded preprocessing reference at `2038c41`, with report-only policy clarification `b05538e`. Central independently rebuilt all 302 cases / 3,451 events and 22 controls, and checked executed identities, mutation sensitivity and coverage accounting. P's ownership window is closed; its defined coverage gaps and unsafe/host boundaries remain open. Generated receipts stay local under 0039.

[Decision 0041](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0041-observation-storage-acceptance.md) accepts R's full observation-storage foundation and canonical consumer integration at `04918d5`. Central freshly replayed both profiles, all 39 controls and five focused tests per profile, and checked the full regression/source evidence. R's ownership window is closed. At that decision Q and S awaited review; decision 0043 now accepts Q. S remains pending and no subsequent Rust owner is released.

[Decision 0042](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0042-pulsar-lifecycle-export-correction.md) returns two exporter corrections to S: retained parameter link arrays are incorrectly hidden after repeated allocation/initialization, and the first seeded extra-covariance backing is omitted. Central demonstrated that a defined reset-link mutation passed the original exact comparison. S delivered corrections at `6cc0fd0` / `58ddc07`; central review remains pending and no production Rust stage is released.

[Decision 0043](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0043-specialized-par-reference-acceptance.md) accepts Q's bounded specialized PAR reference at `bb7e629`, after fresh central replay of all 107 cases / 2,252 events, 27 controls and fourteen compiled mutations. Q's ownership window is closed. Its defined coverage gaps and unsafe/host boundaries remain open; S's corrected lifecycle evidence is next for review.

[Decision 0044](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0044-pulsar-lifecycle-reference-acceptance.md) accepts S's corrected lifecycle reference at `58ddc07`, closes both 0042 findings and closes S's ownership. Central freshly passed all 30 cases / 133 events / 5,329,662 field records, 25 controls and fourteen mutation/witness runs, plus raw/delta and correction audits. A separate whole-pulsar Rust contract is still required.

[Decision 0045](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0045-pulsar-lifecycle-contract-and-t-release.md) releases T as sole Rust owner for complete pulsar state, twelve lifecycle/copy/matrix routines, ten packed-polynomial helpers and canonical integration of current consumers. It fixes ownership/alias, unwritten-state, matrix, clone, callback and frontend boundaries. Accepted S/Q evidence is read-only; generated outputs stay local under 0039. M2 remains unaccepted.

[Decision 0046](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0046-terrestrial-resource-reference-stage.md) releases U for independent native observatory/EOP/meteorology/troposphere evidence across all 22 active primary routines. U owns only `validation/terrestrial_resources/`; T remains sole Rust owner. Source defects, native-invalid boundaries and generated-output policy 0039 remain explicit.

[Decision 0047](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0047-time-ephemeris-reference-stage.md) releases V for independent native IFTE/FB90/tt2tb and Fortran-reader evidence across 29 selected routines. V owns only `validation/time_ephemeris/`; T retains Rust and U retains its separate resource reference. Exact source state, native-invalid classifications, current numerical gates and policy 0039 remain unchanged.

[Decision 0049](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0049-numeric-utility-boundary-evidence.md) records fresh native/Rug evidence that source half-addition and rounded-quotient boundaries can differ at Rug128, and that `%` is not the full source remainder expression. These fidelity gaps remain unresolved; precision and existing gates are unchanged.

[Decision 0050](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0050-restore-accepted-observation-verifier.md) follows the user's direct restoration instruction and supersedes 0048's proposed exception. R's accepted runner is restored; T keeps output isolation in its own staging. Exact original preservation membership remains in force.

[Decision 0051](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0051-terrestrial-resource-reference-acceptance.md) accepts U at `051d2ea` with sources `2cd8c91`, after central fresh replay of 130 cases / 2,256 events, 29 controls and thirteen compiled mutations, plus coverage/observer/history audits. U's ownership closes. Its native-invalid, host and defined coverage limits remain open.

[Decision 0052](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0052-lifecycle-comparator-type-correction.md) returned a bounded comparator correction to T. The original equality shortcut accepted nested boolean/float substitutions for integer counts. Decision 0055 closes this finding after recursive type checking and independent rejection controls.

[Decision 0054](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0054-time-ephemeris-reference-acceptance.md) accepts V at `f179ce5`, with sources `573d667`, after central fresh replay of all 97 cases, thirty controls, 36 independent checks and thirteen compiled mutations. The O0 coverage audit retains all 30 cases / 576 numeric representation differences from O2. V ownership closes; the ordinary O2 reference remains bounded to its declared host profile.

[Decision 0055](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0055-pulsar-lifecycle-rust-acceptance.md) accepts T at `6c8cf68`, with recursive comparator correction `804d9fd`. Fresh central debug/release replay passes all 30 cases and 35 controls; the final source/field/artifact/regression audit passes. T ownership closes. M2 retains exactly its four historical failures.

[Decision 0056](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0056-terrestrial-resources-rust-contract-and-w-release.md) releases W as the next sole Rust owner after T acceptance. W transfers all 22 observatory/EOP/meteorology/troposphere routines from U's accepted reference, integrates canonical resource state into existing consumers, and follows 0053 for documented small fixes.

[Decision 0057](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0057-observer-coordinate-reference-stage.md) releases X for independent native observer-coordinate, pulsar-vector and UT1 evidence across nine routines. X owns only `validation/observer_coordinates/`; W remains the sole Rust owner. Current bug policy 0053 and generated-output policy 0039 apply.

[Decision 0058](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0058-time-ephemeris-rust-contract.md) prepares Y for the full 29-routine time-ephemeris Rust transfer, including demonstrated IFTE/FB/CALCEPH corrections and the required remainder dependency. Y is not released: W must first be accepted and close Rust ownership. The minimal IFTE reset has fresh central causal evidence; no production change is accepted by this preparation.

[Decision 0059](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0059-observer-coordinate-reference-acceptance.md) accepts X at `02234cd`: central freshly rebuilt and replayed all 133 cases twice, 35 controls, fifteen compiled mutations and the complete private-state observer. Audited coverage enters all nine routines, with 327/333 lines and 207/214 branches; limits remain explicit. X ownership closes. W retains Rust, and Y remains prepared until W acceptance.

[Decision 0060](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0060-planetary-ephemeris-reference-stage.md) releases Z for the complete three-routine native readEphemeris.C driver/random-helper reference, using real pinned JPL dependencies. It owns only new `validation/planetary_ephemeris/`; W retains Rust until accepted and Y remains prepared. Existing accepted references are read-only.

[Decision 0061](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0061-terrestrial-resources-acceptance-and-y-release.md) accepts W at `bd290cc`, with final production `eb2a589`, after central exact debug/release replay of all 130 cases and 28 controls per profile and review of the corrected native/full regression evidence. W ownership closes. Y is released as sole Rust owner under 0058; Z keeps its separate native reference. M2 remains unresolved.

[Decision 0062](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0062-observer-coordinate-rust-contract.md) prepares AA for the full nine-routine observer-coordinate Rust transfer, shared IMAG/UT1 state and the demonstrated overlapping-copy correction. AA is not released: Y must first be accepted and close Rust ownership. The central whitespace probe preserves all seven defined neighbors and passes nineteen corrected cases; no production or milestone acceptance is implied.

[Decision 0063](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0063-propagation-dispersion-reference-stage.md) releases AB for the complete 25-routine native propagation/dispersion/solar-wind/BAT/secular/phase-reference reference. It owns only new `validation/propagation/`, independently of Y/Z; AA remains unreleased. The reference must expose original cross-pulsar and retained-state behavior with real dependencies and explicit native-safety limits.

[Decision 0064](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0064-planetary-ephemeris-reference-acceptance.md) accepts Z at `d517acf` after fresh central ordinary/observer replay of all 96 cases, 31 controls, eighteen compiled mutations, supporting checks and a raw coverage audit. Z ownership closes; native JPL values retain their host-qualified limits. Y and AB deliveries await central review; AA remains prepared and unreleased.

[Decision 0066](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0066-time-ephemeris-acceptance-and-aa-release.md) accepts Y at `21dc956`, closes its ownership and releases AA as sole Rust writer. Central audited and recomputed the delivered comparisons, verified the focused EOF follow-up and retained all earlier result identities under 0065. The public frontend and unresolved M2/0049 gates remain bounded and explicit. AA implements the observer-coordinate dependency of the general input-to-BAT workstream while central reviews AB.

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
| [I JPL Rust transfer](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/09-jpl-rust.md) | Delivered evidence read-only; ownership window closed | Accepted by decision 0022 at `902fbba`, executed implementation `27c8384` | `rust/JPL_REPORT.md`, complete source mapping, host comparisons, ownership/unsafe evidence and M1/M2 regressions |
| [J Clock/table reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/10-clock-table-reference.md) | `validation/clocks/` only; delivered evidence now read-only | Executed evidence accepted by decision 0021 at `360cc03`; nine unrun cases remain open | `REPORT.md`, table/route/fallback/cache/toa2utc source evidence and active-profile inventory |
| [K Parameter/PAR reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/11-parameter-par-reference.md) | Delivered evidence read-only; ownership window closed | Accepted by decision 0026 at `38d62af`, including correction `ccdf55c` | Complete parameter/dispatch inventory and bounded initialization/shared PAR reference; specialized records and TIM/preprocessing remain later work |
| [L Clock/table Rust transfer](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/12-clock-table-rust.md) | Delivered evidence read-only; ownership window closed | Corrected delivery `84b6b86` accepted by decision 0031 | `rust/CLOCK_REPORT.md`, complete projection, process-state and safe-boundary evidence, unchanged JPL/M1/M2 gates |
| [M Numeric utility reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/13-numeric-utility-reference.md) | Delivered evidence read-only; ownership window closed | Accepted by decision 0030 at `a01d428`, final evidence `e754581` | `REPORT.md`, complete selected-function/header inventory, bounded numeric/text/state evidence and actual execution-identity controls |
| [N TIM I/O reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/14-tim-io-reference.md) | Delivered evidence read-only; ownership window closed | Accepted by decision 0032 at `68de89a` | `REPORT.md`, complete primary-file inventory, caller-state/history projection, raw TIM/JBO/order/I/O evidence and current execution controls |
| [O Parameter storage Rust foundation](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/15-parameter-storage-rust.md) | Delivered evidence read-only; ownership window closed | Delivery `8d2be6f` accepted by decision 0036; production `8bad0be` | `rust/PARAMETER_STORAGE_REPORT.md`, complete storage/copy projection, integrated canonical state and unchanged regressions |
| [P Preprocessing reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/16-preprocessing-reference.md) | Delivered local evidence read-only; ownership window closed | Accepted by decision 0040 at `2038c41` | `REPORT.md`, complete primary routine/branch/caller maps and exact native preprocessing/unit/epoch history evidence |
| [Q Specialized PAR reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/17-specialized-par-reference.md) | `validation/par_specialized/` only | Accepted in decision 0043 at `bb7e629`; ownership closed | `REPORT.md`, complete released branch/caller maps and exact native specialized PAR state/stream evidence |
| [R Observation storage Rust foundation](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/18-observation-storage-rust.md) | Delivered local evidence read-only; ownership window closed | Accepted by decision 0041 at `04918d5`; production `b7a4d17` | `rust/OBSERVATION_STORAGE_REPORT.md`, complete 95-field/caller projection, consolidated state and unchanged regressions |
| [S Pulsar lifecycle reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/19-pulsar-lifecycle-reference.md) | `validation/pulsar_lifecycle/` only | Accepted in decision 0044 at `58ddc07`; ownership closed | `REPORT.md`, complete pulsar/nested-state and native initialization/copy/lifetime evidence |
| [T Pulsar state and lifecycle Rust transfer](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/20-pulsar-lifecycle-rust.md) | `rust/` and `validation/pulsar_lifecycle_port/`; R output routing only in T-owned staging under 0050 | Accepted at `6c8cf68` by decision 0055; ownership closed | `rust/PULSAR_LIFECYCLE_REPORT.md`, complete state/lifetime/caller projection and unchanged regressions |
| [U Terrestrial resource reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/21-terrestrial-resource-reference.md) | `validation/terrestrial_resources/` only | Accepted at `051d2ea` by decision 0051; ownership closed | `REPORT.md`, complete primary source/caller/state maps, exact native histories and verification |
| [V Time ephemeris reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/22-time-ephemeris-reference.md) | Delivered native evidence read-only; ownership closed | Accepted at `f179ce5` by decision 0054 | `REPORT.md`, complete source/caller/format/lifetime maps and exact native IFTE/FB90/conversion histories |
| [W Terrestrial resources Rust transfer](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/23-terrestrial-resources-rust.md) | Delivered evidence read-only; ownership closed | Accepted at `bd290cc` by decision 0061 | `rust/TERRESTRIAL_RESOURCES_REPORT.md`, complete 22-routine transfer, integrated resource state and direct/regression evidence |
| [X Observer-coordinate reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/24-observer-coordinate-reference.md) | Delivered reference read-only; ownership closed | Accepted at `02234cd` by decision 0059 | `REPORT.md`, complete nine-routine/source/state maps, native histories and verification |
| [Y Time ephemeris Rust transfer](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/25-time-ephemeris-rust.md) | Delivered evidence read-only; ownership closed | Accepted at `21dc956` by 0066 (production `11165d1`) | `rust/TIME_EPHEMERIS_REPORT.md`, full 29-routine transfer, corrected contracts and integrated verification |
| [Z Planetary ephemeris reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/26-planetary-ephemeris-reference.md) | Delivered reference read-only; ownership closed | Accepted at `d517acf` by decision 0064 | `REPORT.md`, complete three-routine state/caller map, native driver/random histories and verification |
| [AA Observer-coordinate Rust transfer](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/27-observer-coordinate-rust.md) | `rust/` and new `validation/observer_coordinates_port/`; sole Rust owner | Released by 0066 from accepted Y `21dc956` | `rust/OBSERVER_COORDINATES_REPORT.md`, complete nine-routine transfer, shared state and corrected/native comparisons |
| [AB Propagation and dispersion reference](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/28-propagation-reference.md) | `validation/propagation/` only | Delivered at `cb2c41f`; central reference acceptance pending under 0063 | `REPORT.md`, full 25-routine source/state/caller maps, original native histories, controls and coverage |

A through Z and AB have delivered their assigned evidence or implementation. E's M2 gate remains unresolved. I/L/O/R/T/W's bounded Rust transfers and the accepted native references, including X, have closed ownership windows. Y is accepted at `21dc956` by 0066 and its ownership is closed. Z's reference is accepted by 0064 and its ownership is closed; AB's reference awaits central acceptance. AA is released under 0062/0066 as sole Rust writer. Original reference evidence, accepted outputs and runners remain read-only; successor verification uses owned staging with explicit applicability. Historical acceptances remain scoped to their recorded revisions, and current small-bug policy 0053 governs subsequent corrections.

Starting an assignment means opening an implementation chat in this workspace and giving it the appropriate brief. This document does not create or message chats automatically.

**Dispatched 23 October 2026:** E and F started from policy commit `1322f8b` in the local `rustify-tempo2` project. G started from reference-stage contract commit `2f75c30`; H started from `173f0fb`. I started from `d499e94` after E/H ownership closure; J started from `67aa77c`; K started from `9f974ef`. I, J and K ownership windows are now closed. Acceptance is recorded per assignment in the decisions above; M2 remains pending.

| Chat | Thread ID | Responsibility |
| --- | --- | --- |
| Assignment E: Rust TEMPO2 fitting | `01a16734-4da3-7810-979a-4dad2f647ecd` | Delivered amended M2 implementation and corrections; ownership window closed, acceptance gate pending |
| Assignment F: TEMPO2 core and fidelity inventory | `01a16734-9835-7f53-8a56-79602dc5e2af` | Audit baseline `ddc07b2`; write only `validation/coverage/` |
| Assignment G: TEMPO2 binary reference | `01a1673f-3754-7641-83bd-1d0b03ad06c3` | Prepare all-model source evidence; write only `validation/binary/` |
| Assignment H: TEMPO2 JPL reference | `01a1675e-84aa-7080-b480-1d4cc9919e08` | Delivered and accepted JPL kernel/layout/state/file-lifetime reference; ownership window closed |
| Assignment I: Rust JPL transfer | `01a1678a-ed2a-78c0-819d-a2b914a221bb` | Bounded transfer accepted by decision 0022; ownership window closed |
| Assignment J: TEMPO2 clock and table reference | `01a16796-3234-7a41-aebf-0943867d9d04` | Delivered reference accepted within its executed scope by decision 0021; ownership window closed, nine cases remain unrun |
| Assignment K: TEMPO2 parameter and PAR reference | `01a167ad-4f08-7061-ae55-4f25abc12a7f` | Corrected bounded reference accepted by decision 0026 at `38d62af`; ownership window closed |
| Assignment L: Rust clock and table transfer | `01a167e7-780c-7163-9ed3-e1af7cb9ec33` | Corrected bounded transfer accepted by decision 0031 at `84b6b86`; ownership window closed |
| Assignment M: TEMPO2 numeric utility reference | `01a167fd-6e4d-7b31-ae87-f1fafd486df3` | Bounded reference accepted by decision 0030 at `a01d428`; ownership window closed |
| Assignment N: TEMPO2 TIM I/O reference | `01a1681a-64c8-7350-b6ae-006587b2b2d9` | Bounded reference accepted by decision 0032 at `68de89a`; ownership window closed |
| Assignment O: Rust parameter storage foundation | `01a168ce-9a6e-7a20-b8ff-f5df761fa336` | Delivery `8d2be6f` accepted by decision 0036; ownership window closed |
| Assignment P: TEMPO2 preprocessing reference | `01a168d8-48fa-7722-82db-2d2b6ec8916f` | Bounded reference accepted by decision 0040 at `2038c41`; ownership window closed |
| Assignment Q: TEMPO2 specialized PAR reference | `01a168e7-6a90-7d63-b906-362363a4c15f` | Accepted at `bb7e629` under decision 0043; ownership closed |
| Assignment R: Rust observation storage foundation | `01a16903-61ef-7651-a9c9-41e6c8943e41` | Delivery `04918d5` accepted by decision 0041; ownership window closed |
| Assignment S: TEMPO2 pulsar lifecycle reference | `01a16912-7fd8-7392-9c1e-79436775676e` | Accepted at `58ddc07` under decision 0044; ownership closed |
| Assignment T: Rust pulsar state and lifecycle | `01a16bca-89a0-7cd0-a371-9555df226d7d` | Accepted at `6c8cf68` by decision 0055; ownership closed |
| Assignment U: TEMPO2 terrestrial resource reference | `01a16bd2-a746-7871-9dfb-52a76ae73c64` | Accepted at `051d2ea` under decision 0051; ownership closed |
| Assignment V: TEMPO2 time ephemeris reference | `01a16bdb-8e82-71a0-b823-67399b391ae0` | Accepted at `f179ce5` by decision 0054; ownership closed |
| Assignment W: Rust terrestrial resources | `01a16c39-0b79-75f2-9014-0b8dffc5c63c` | Accepted at `bd290cc` by 0061; ownership closed |
| Assignment X: TEMPO2 observer coordinates | `01a16c44-a1ca-7e43-a83a-1cdce98d44e8` | Accepted at `02234cd` by decision 0059; ownership closed |
| Assignment Y: Rust time ephemerides | `01a16c81-884a-70b0-9791-627bb0b21c0c` | Accepted at `21dc956` by 0066; ownership closed |
| Assignment Z: TEMPO2 planetary ephemeris | `01a16c74-090a-70a3-a4a8-29ecb51fe5de` | Accepted at `d517acf` by 0064; ownership closed |
| Assignment AA: Rust observer coordinates | `01a17180-88f2-72a3-a6c3-609640aacd7b` | Dispatched from `4498cf1` under 0062/0066; sole Rust writer, plus `validation/observer_coordinates_port/` |
| Assignment AB: TEMPO2 propagation reference | `01a16c97-34ca-70f0-a77c-d251fa5c9f3e` | Dispatched from `b8d6263` under 0063; owns only `validation/propagation/` |

**Dispatched 24 October 2026:** T started from `d350727` after R/S ownership closure. The implementation chat is active and explicitly inherits the generated-output policy. Central retains design decisions and milestone acceptance. U started from `43001af` for independent terrestrial-resource reference work; its active status and separate ownership have been confirmed and relayed to T. V started from `8e56df0` for independent time-ephemeris reference work; its active status is confirmed and T/U received its ownership boundary.

**W dispatched 24 October 2026:** The separate implementation chat started from contract commit `17fc107` in the local `rustify-tempo2` project after T acceptance. It inherits 0053's documented small-bug-fix permission and 0039's generated-output rule. Central retains shared design and milestone acceptance.

**X dispatched 24 October 2026:** The independent native observer-coordinate reference chat started from `64cab7f` under 0057. Its active turn is confirmed; W received the separate ownership boundary. Generated evidence remains local under 0039, and current bug policy 0053 applies.

**Z dispatched 24 October 2026:** The independent native planetary-ephemeris reference chat started from `7946aee` under 0060. Its active turn is confirmed and W received the separate ownership boundary. X is closed; W remains sole Rust owner while its final verification/report is completed. Y remains prepared until central acceptance of W.

**Y dispatched 24 October 2026:** The separate Rust time-ephemeris chat started from `5920aff` after W acceptance at `bd290cc`. Its active turn is confirmed. Y owns `rust/` and `validation/time_ephemeris_port/`; Z received the independent ownership boundary. The released 0058/0061 contract and current bug/validation policies apply. Central retains design and milestone acceptance.

**AB dispatched 24 October 2026:** The independent native propagation/dispersion reference chat started from `b8d6263` under 0063. Its active turn is confirmed; Y and Z received the ownership boundary. AB owns only `validation/propagation/`, Y remains the sole Rust writer, and AA remains prepared until Y acceptance. Current bug and generated-evidence policies apply; central retains design and milestone acceptance.

## Shared working design

Production checks must address realistic failures in intended call paths. Trust canonical validated objects and dependency guarantees; deliberate contract violations in verifier tests do not justify duplicate internal validation. This applies the user’s replacement standing rules of 24 October 2026.

Use one Rust crate with a library and a small CLI initially. Owned model and observation values hold the input; fallible operations return results with contextual errors. The initial barycentric operation needs no observatory resources. D introduces an explicit runtime context when the terrestrial path needs resources. Preserve recognizable correspondence with TEMPO2 routines and the original calculation order, including the accepted source-corresponding module and helper names. Retain established numerical libraries behind small interfaces when justified by the selected path.

Follow established TEMPO2 behavior unless a small, demonstrated bug is corrected under decision 0053. Explain the original bug, pinned source counterpart and correction beside the changed code, protect it with a focused regression and report the intentional departure. Keep original reference evidence intact. Decision 0008's module-counterpart and necessary-unsafe explanations still apply. F's earlier deviation inventory remains evidence for review; historical acceptance alone does not establish current source agreement or validate a correction.

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
| Source behavior and full-core coverage | Decision 0053 permits small demonstrated bug fixes; 0008 correspondence and coverage requirements remain | Each fix has a source-mapped comment, regression and reported departure; every core module and unsafe boundary is accounted for |
| Binary reference preparation | Decision 0009 adopted; G released | Native and wrapper precision, model-specific updates, source mutations and integrated residual evidence; implementation gates await central review |
| M2 acceptance | Pending exact mode-order gate; E corrections closed in decision 0015 | All other reviewed comparisons pass; keep the four raw-order diagnostics unresolved under decision 0012 |
| SVD diagnostic comparison | Decision 0010 diagnostic only; decision 0012 governs acceptance | Keep exact raw mode-order gate; paired singular modes do not replace it. Numerical budgets and source cutoff controls remain unchanged. |
| JPL reference evidence | Accepted by decision 0016 at corrected H delivery `1df4d37` | Exact host regeneration, source/helper/cache/lifetime evidence and stable isolated failure classifications; no port acceptance |
| JPL Rust contract | I released by decision 0017; title boundary clarified by 0019 | Same-open-file C stdio, safe Rust source logic, explicit undefined-source dispositions, exact host observations and unchanged timing gates; all-whitespace title classified without editing H |
| JPL Rust acceptance | Accepted by decision 0022 at `902fbba`; I ownership closed | Corrected legacy-header boundary, exact H host observations, owned stdio and unchanged M1/M2 regression outcomes |
| Clock/table reference | J's executed evidence accepted by decision 0021 at `360cc03` | 142 exact case replays, measured function membership and mutation controls; nine unrun cases and later port dispositions remain open |
| Parameter/PAR reference | K accepted by decision 0026 at `38d62af`; ownership closed | 306 exact case replays, actual executable/runner identity, controlled-state conventions, measured coverage and mutation controls; specialized records/TIM remain later work |
| Clock/table Rust contract | L accepted by 0031; 0029 closed; 0027 contraction retained | Process lifetime, complete selected source behavior, byte parsing, explicit safe/terminal adapters, exact J comparisons and unchanged JPL/M1/M2 gates |
| Numeric utility reference | M accepted by decision 0030 at `a01d428`; ownership closed | 350 exact cases, 35 controls, 22 compiled mutations and measured 24-routine/11-wrapper coverage; host/undefined limits retained |
| TIM I/O reference | N accepted by decision 0032 at `68de89a` | Complete source inventory, formats/directives/includes, writing and ordering, JBO/primitive I/O, declared host/undefined boundaries and fresh execution evidence |
| Preprocessing reference | P accepted by decision 0040 at `2038c41`; ownership closed | 302 exact fresh cases, 22 central controls, preserved identities; stated coverage gaps remain open |
| Observation storage Rust | R accepted by decision 0041 at `04918d5`; ownership closed | Complete 95-field foundation, fresh exact direct gates and focused tests, reviewed unchanged regressions; full lifecycle remains later work |
| Terrestrial resource reference | U accepted by decision 0051 at `051d2ea`; ownership closed | 130 exact central cases, 29 controls, thirteen compiled mutations; audited 22-routine coverage and restored exact preservation |
| Time ephemeris reference | V accepted by decision 0054 at `f179ce5`; ownership closed | 97 exact central cases, thirty controls, 36 analytic checks, thirteen mutations; audited 29-routine O0 coverage and explicit O0/O2 numeric differences |
| Pulsar state/lifecycle Rust transfer | T accepted by decision 0055 at `6c8cf68`; ownership closed | 349-field canonical state and released routines; exact S/Q checks, 35 controls per profile, restored R staging and audited final regressions |
| Observer-coordinate reference | X accepted by decision 0059 at `02234cd`; ownership closed | 133 exact central fresh/replay cases, 35 controls, fifteen mutations and observer equivalence; reviewed nine-routine coverage and explicit gaps |
| Terrestrial-resource Rust transfer | W accepted by 0061 at `bd290cc`; ownership closed | Full 22-routine integration, explicit wet-coefficient correction, 130 exact cases and 28 controls per profile; scoped final-code and preserved broader regressions |
| Planetary-ephemeris reference | Z accepted by 0064 at `d517acf`; ownership closed | 96 exact central ordinary/observer cases, 31 controls, eighteen compiled mutations, supporting checks and audited 207/207 lines and 106/108 branches; JPL host limits retained |
| Observer-coordinate Rust contract | AA prepared under 0062; not released | Accepted X, canonical UT1/IMAG ownership, shared SOFA/frontend calculations and documented whitespace correction; Y acceptance required |
| Propagation/dispersion reference | AB delivered at `cb2c41f` under 0063; central review pending | Complete 25-routine original source, real solar-wind/shapelet/ifunc dependencies, typed state, cross-pulsar histories and measured coverage |
| Generated validation output | Decision 0039 adopted at `3848f5a` | Local generated evidence; tracked source/contracts/reports; no history rewrite |

The central chat updates this table when a decision is made. Implementation chats can make routine choices inside their accepted contract without requesting approval again. For a choice that changes scientific meaning, time scale, phase convention, shared public output, dependency architecture, or acceptance tolerances, provide the concrete alternatives and evidence for central review. Continue independent work while that decision is unresolved.

## Numerical and behavioral contracts

- Preserve the pinned source's scientific conventions, defaults and supported fallbacks. Small demonstrated bugs may be corrected under 0053 with adjacent source-mapped comments, focused regressions and reported departures. Identify undefined-source cases and observable adapter differences explicitly; preserve original reference evidence.
- Parse precision-sensitive decimal inputs directly into the selected representation. Preserve precision through arithmetic, serialization, comparison, and display. Converting an already-rounded `f64` into an extended type does not restore input digits.
- State each fixture's time scale, units, ephemeris, clock realization, reference-phase rule, mean-removal setting, selections, and input ordering. A fit must never be used to conceal a fixed-parameter discrepancy.
- Compare numerical errors using the magnitude of the difference between actual and expected values, with explicit units and any relative scaling defined. Compare identifiers and discrete flags exactly. Document tolerance rationale before accepting results; discrepancies trigger investigation rather than automatic relaxation.
- Preserve TOA identity and original input fields. Reject unsupported scientifically active parameters and directives explicitly. Defaults must be identified and recorded; missing metadata must not be guessed.
- Preserve source result and failure classifications, including defined nonfinite fit outputs. Record explicit fallible-library boundary differences where upstream terminates or produces undefined state; do not invent values to simulate it. Preserve source fallbacks, including missing-weather behavior, with warnings and provenance. Historical recoverable-error guards and state changes remain inventory items until reviewed under the evidence and correction policy in 0053.
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

**AA dispatched 25 October 2026:** The separate observer-coordinate implementation chat started from `4498cf1` after Y acceptance at `21dc956`. Its active turn is confirmed. AA owns `rust/` and `validation/observer_coordinates_port/`; central continues AB review and the following planetary/propagation plus input/frontend plan. Functional priority 0065, small-bug policy 0053, generated-output policy 0039 and accepted-evidence preservation remain binding. Central retains design and milestone acceptance.
