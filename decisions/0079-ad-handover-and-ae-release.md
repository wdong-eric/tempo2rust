# AD implementation handover and AE general-fitting release

Central decision, 25 October 2026. **Approve AD's implementation handover, close AD's Rust ownership and release AE for general fitting/noise under 0071 and handoff 31.** This applies 0074's separation of functional handover from scientific acceptance. The complete migration and inherited numerical gates remain unaccepted; this decision neither changes a numerical threshold nor selects another precision/backend.

## Released baseline

- AD production and validation: `a8e5178778f7eeb3a0fc65f8934a64a03f94208d`.
- AD documentation and report: `0f44180a021655074c387e2c03012d0c4253276e`; `rust/TIMING_MODELS_REPORT.md`.
- Canonical TEMPO2: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; independent A2 reference: `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`. Both independent checkouts were clean.
- AC input-to-BAT foundation remains the 0075-reviewed `18c9065` / report `d47ae51`, with pending P acceptance. AD's interface freeze is `8b1bdcb`.

AD delivers the public fixed-parameter `timing` pipeline from real PAR/TIM/resource inputs through general BAT and residual evaluation, canonical direct residual/averaging and binary APIs, all fourteen binary families (42 bodies and two fitting callbacks), all three residual/averaging routines and the six-body eccentric-GW closure. Old bounded adapters now use the shared residual implementation. This is a functional baseline for fitting; source counts alone are not its proof.

## Central review and evidence

Central inspected the public orchestration/projection, shared-adapter/source maps, correction recipes and adjacent explanations, binary companion boundary, cross-pulsar witness, comparison logic, report and final evidence index. This is a scoped handover review, not an independent review of every scientific expression or every parameter combination.

The read-only review is `decisions/evidence/timing-models/central-handover-review.py`. It ran against the clean delivered checkout and existing delivered binaries, without rebuilding or rewriting implementation evidence. It verified 1,748 distinct file identities, including committed source/fixtures, execution logs, native source/objects/executable, test executables, original G records, consumed inputs/resources, original outputs and corrected-comparator receipts. Original decision 0008 retains SHA-256 `2c5f81b82c43105fcd48b62139f1d78f2dc7c4089055ea162e9feef82ff4e12c`.

All 260 stored public comparisons (65 cases, two mean settings and two build profiles) pass the current comparator; debug/release scientific projections are identical. Central also made four fresh calls to final release executables: `pks_dd-mean0`, `t2-companions-mean1`, `two-cut-mean1` and `library-noise-mean0`. They pass against the identity-verified native outputs, preserve diagnostic events and, where produced, exact `avg.dat` bytes. No fresh native run or whole historical replay is claimed.

Maximum absolute differences in that stored public matrix are `2.5011104298755526543e-15 s` for AC fields expressed in seconds, `4.71844785188862101007789356e-16 s` for the added PET/binary-delay/residual fields, and `5.482582787613475492e-14 turns` for phase, below the unchanged `1e-12 s` / `1e-10 turn` gates. Exact count comparisons have zero difference. These are engineering comparisons on the declared cases, not physical calibration of every model. Averaged values are separately checked by the comparator; they are not included in those recorded maxima.

Central found one comparator gap during review: missing averaged fields or wrong units could pass. AD corrected the existing six-field/unit contract and added eleven rejecting mutation controls. All stored outputs were rechecked with separate new receipts; original executions and receipts remain intact. Central independently repeated the three original false-pass witnesses and confirmed rejection. This correction required no production arithmetic change or pipeline rerun.

The delivery records, in each profile, 4,684 original G checks (including 4,570 component/update records, 108 integrated calls and six source exits), with 432 undefined original records withheld; 506 separately corrected checks; normal and explicit lifecycle tests; CLI recovery; original M1a/M1b comparisons; affected resource/AC and M2 checks; formatting and Clippy. Central verified the named evidence identities and component success records and did not relabel these as newly executed central suites. M2 still exits with exactly its four historical raw-order differences.

The central execution receipt is retained byte-for-byte at `decisions/evidence/timing-models/build/handover-20261025/receipt.json`, SHA-256 `5e2f0bfb21c37aae5968dabb28b0f9e21e0f1af4b59337e2a72a52382e95c8ee`. Its original execution directory is `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-ad-handover-wznzkuh1`. The delivered manifest hash is `3a784985708e5c09680fe0d382c29df38a65be95373735744294d1bd155c9c7e`. Generated results, binaries and JSON remain untracked under 0039; only the authored reviewer and concise documents are committed.

## Corrections and unresolved acceptance

The documented narrow corrections meet 0053's source/cause/fix requirement: DDK's unused uninitialized derivative read; T2's capacity endpoint and optional companion flags; ELL1H/T2 intermediates restricted to consuming selectors; ELL1's ORBIFUNC count; current-pulsar phase reference, shapelet and averaging ownership; active-only noise subtraction after TZR removal; removal of unused newmean; and zero initialization of opt-in AverageDMResiduals and empty nWhite_dm. Original and corrected native witnesses remain separate. Undefined selectors receive no invented numerical golden. Source-defined update conventions, NHARM behavior, outer-loop tracking/coalescence lifetimes and defined nonfinite outcomes remain preserved. No new unsafe Rust block or native numerical dependency was introduced.

The ledger remains open: P's 154 field/event records (32 SIMUL, 71 units, two SAT offsets, four links, 45 epochs); M2's four raw-order comparisons; the four 0049 discrete boundaries; and the original DDH ideal check at 1 ns (first failure 1.137585 ns, maximum 4.598955 ns). 0077's disposable 113-bit diagnosis localizes P without changing Rug128. The original 277 AB differences remain preserved separately from the 0075-approved corrected behavior. No speculative rewrite, precision rounding, sorted SVD outputs or tolerance relaxation is authorized by this handover.

## AE release and ownership

AE now owns `rust/` and new `validation/general_fitting/` as the sole implementation writer. AD is closed. Central continues shared design, numerical-policy decisions, handover review and milestone acceptance. AF and AG remain prepared; later core output/entry points, C ABI packaging, optional backends and release/platform/performance/concurrency qualification remain on the full-migration inventory.

Start AE from the committed AD baseline above and follow 0071/0074/0065 and handoff 31. Its first concrete deliverable is the canonical public `timing --fit` path; complete built-in local/global fitting, remaining callbacks/updates, covariance/constraints, robust and timing-noise lifecycle and the scoped standalone APIs through that same engine. Reuse AD binary callbacks and current source state, existing E/G/AD evidence and authored examples. Toolkit/spectrum dependencies actually needed by fitting belong in AE's implementation closure; prepared AG is not a new prerequisite.

Freeze the concrete fit interfaces and new quantity-specific comparison contracts against this delivered state, then implement. Follow TEMPO2's choices except for demonstrated small corrections under 0053. Preserve source notation and counterparts, explain every necessary unsafe boundary, commit authored paths with Conventional Commits, and keep generated evidence ignored. Run affected checks; unchanged centrally owned findings do not justify repeated full histories or suspension of independent fitting work. Return `rust/GENERAL_FITTING_REPORT.md` with actual results and the inherited/new open ledger. M2 and full fitting acceptance remain pending until their mandatory gates are resolved.
