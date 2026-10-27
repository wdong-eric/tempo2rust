# AK handover and AL integrated qualification release

Central decision, 27 October 2026. **Approve AK's implementation handover under 0074, close AK's Rust ownership, and release Assignment AL under 0086/0097 and handoff 38.** At the 0082 checkpoint, the user explicitly selected **“Keep AL next”** after reviewing the substantial measured performance costs. Integrated Linux/macOS qualification therefore precedes profiling and optimization; performance remains work to address before production cutover. This decision accepts neither the inherited numerical gates nor the full migration.

Reviewed production: `81de1e6bb7d6f6cf1d17c075f402c301b87c9ce9`; qualification/API: `3926ae14375752a6ef9526e8141a2bad864097af`; final report: `eaa30c593b1871cf93edfb4cf23e384a039f47ab`. The latter commits do not change the production implementation. The main tree was clean before central's release documents. Canonical TEMPO2 and independent A2 remain clean at `fda6ad4a9753ad7dc5d291406f2f310a9161423f` and `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`.

## Delivered capability and reviewed evidence

AK supplies optional real observation workers through one canonical observation calculation, with the outer pulsar loop and each observation's scientific order preserved. The scoped positive worker limit defaults to one. Workers use disjoint row scratch and record effects for ordered publication by the existing process owner. Successful prefixes and the partially mutated failing row retain the documented error contract; later speculative reads may occur, but their unpublished results/effects are discarded. All workers join before return or unwind. No parallel fitting reduction, rollback guarantee or new unsafe sharing is introduced.

Complete supported Rust workflows now hold process admission through controls, work and diagnostic collection. Composed direct calls use the documented outer workflow scope. Existing process histories remain shared: RuntimeContext does not imply independent source state. Native predictor C callers retain caller exclusivity over calls, borrowed results, callbacks and exported globals. These bounded contracts are in `rust/PARALLEL_EXECUTION_CONTRACT.md`, `rust/PARALLEL_EXECUTION_API.md` and the final report.

Central independently completed:

- A source review of the observation extraction, row snapshots, worker/effect publication, diagnostic ownership, admission and connected callers, with no production correction required by that bounded review.
- A fresh isolated default-debug build from exact production `81de1e6`, verifying 623 exported Git blobs: 581 Rust-tree files and 42 fitting inputs. Five selected execution tests and the original terrestrial fit recovery test all pass with their test bodies explicitly enabled. These include actual worker assertions, full rows/effects, four concurrent complete callers, controls/unwind and failure/recovery. The existing offline dependency cache was reused; the project target directory was not.
- A final delivered-evidence audit at `eaa30c5`: **3,195 unique files, zero mismatches**, including the owner's 2,752-file audit, receipt runner/input identities, all 607 committed files under `rust/` and `validation/parallel_execution/`, and 209 installed source files bound to their Git contents. Both independent source pins remain clean.
- A freshly compiled external release Cargo client using only AK's installed source dependency, with four callers performing repeated correlated two-pulsar fits and predictors. Exact scientific equality, diagnostic ownership, failures and recovery pass. A separately compiled C11 shared-library client passes, and its runtime diagnostic confirms loading the installed predictor library. This reuses AK's audited private install; it is not another independent reinstall or a Linux run.

The audited owner campaigns provide 96 passing tests and 36 ignored tests in each default debug/release profile, formatting and all-target Clippy; 99 native/Rust/repeated invocations over 11 cases at one/two/four requested workers, with **484,314 quantity comparisons and zero failures**; 26 external Rust invocations; and the unchanged combined-provider comparison of **59 cases / 41,961 checks with zero failures**. Full Rust records and native scientific outputs agree across the selected worker counts. Defined native scheduler effects use the documented projection. Separate worker calibration establishes actual workers rather than assuming the requested maximum was reached.

The predictor C campaign runs four callers with three serialized workflows each in plain, ASan/UBSan and TSan clients. Instrumentation covers the C clients/shim, not the entire Rust/MPFR/Fortran/provider stack. MPFR 4.2.2 reports TLS enabled on the executed host. Provider internal worker counts are not measured; environment thread limits are requests. None of this establishes arbitrary concurrent API safety or an independent-context architecture.

## Review findings closed within the delivered scope

The AH diagnostic ownership failure is closed for the tested supported complete-workflow contract. The predecessor failure was reproduced with the same three warning-vector paths and 6,718 other matching scalar values in that newer baseline; the original AH evidence remains three paths and 6,716 scalars. Admission includes controls and destructive collection, and new overlapping-caller tests retain private markers and warning comparisons. The original recovery test's existing first-prefit history normalization is unchanged. An isolated retry is not the basis for this closure.

AK retains original/corrected evidence for the source OpenMP build-wiring omission and the DMMODEL warning race, with adjacent source/cause/fix comments under 0053. The corrected native profile supplies a defined comparison; the original race is not adopted as required behavior. The small row-snapshot correction preserves Rug scalar signs, including NaN. Scientific expressions, Rug128 production precision and existing gates remain unchanged.

Central's draft benchmark finding is closed by `native-timed-v1`, `timed-control-v1` and `performance-v2`. Timed native binaries contain no worker instrumentation; twelve controls reproduce the instrumented products. Actual workers are measured separately. Raw native TSV publication differs from Rust's output, so only matched load/evaluation intervals are compared directly there. Full-process wall/CPU/RSS comparisons are reserved for the explicitly matched CLI input/output workloads. The original provisional benchmark and review snapshot remain immutable. No numerical rerun or new tolerance was substituted for this measurement correction.

## Performance checkpoint and the user's sequence decision

AK measured 108 process runs with three shuffled samples per selected engine/workload/worker combination. These were ordinary desktop measurements after the build campaigns, not reserved-host capacity or a production service-level qualification. A fresh process does not imply a cold filesystem cache. Outliers remain visible.

The large raw workload's median loaded evaluation is about 0.665 s with one Rust worker and 0.668 s with four, versus native 0.230 s and 0.239 s. The matched one-worker correlated-global CLI workload takes about 0.462 s and 1,578 MiB peak RSS in Rust, versus native 0.0403 s and 161 MiB. There is no consistent measured four-worker benefit. These cases establish costs, not a general scaling law.

For 20,000 standalone predictor phase/frequency calls on the same native-generated predictor file, the complete Rust C route takes a median 0.469 s versus native 0.000346 s; the aggregate checksum agrees exactly. The interval excludes parsing/publication but includes the complete bridge and canonical evaluation. Attribution to conversion, allocation or arithmetic requires profiling and is not inferred from that timing alone.

Central presented optimization-first versus AL-first. The user chose **AL next**. Actual Linux ABI/provider execution and a clean installed candidate remain the next missing release capability; profiling follows on an identified, integrated candidate before production cutover. Default serial execution stays unchanged. No performance threshold, precision concession or claim of performance parity is introduced.

## AL release and ownership

**AK ownership is closed. AL is the sole Rust writer**, owning `rust/`, new `validation/release/` and the new `.github/workflows/tempo2-rust.yml` if needed, starting from `eaa30c5` plus this central release. Central retains shared design, numerical policy, cross-stage coordination and milestone acceptance. Predecessor source, runners and evidence are read-only; AL stages portable successors without rewriting them. Generated JSON, binaries, traces and archives stay ignored under 0039.

Follow 0086, 0097 and handoff 38 in dependency order. Freeze the candidate and requirement-to-evidence map; prepare the portable pinned native/reference/resource bootstrap and Linux provider boundary; deliver clean private-prefix core CLI, Rust and static/shared C consumers; execute matched platform profiles and integrated scientific/operational checks; complete the source/unsafe/departure map and preserve every open numerical gate. Reuse immutable evidence where its identity and applicability hold, repeating affected checks when code, compiler, ABI, provider or linkage changes.

The user-selected GitHub Actions route remains **preparation authorization**. Produce the exact reviewable candidate/workflow/publication diff before requesting publication approval. Do not push or dispatch as part of this release. The initial Linux runner is explicit `ubuntu-24.04` x64; record the actual image/toolchain/provider identities when executed. Both selected platforms require actual execution. The ordinary-long-double conversion/comparator must preserve the measured Linux precision rather than collapse it to binary64. Any necessary numerical-policy choice returns to central.

AL should return `rust/RELEASE_QUALIFICATION_REPORT.md`, authored executable recipes, per-requirement evidence and the full unresolved ledger. A runnable candidate with open gates remains a handover, not full migration acceptance. No further stage-order question is needed for the already answered AL-first choice.

## Unresolved acceptance ledger

M0/M1a/M1b retain scoped acceptance; M2 remains unaccepted. The numerical ledger from 0096 is unchanged:

- P: 154 records including 32 SIMUL; M2: four raw SVD-order failures; 0049: four discrete boundaries and three unconstrained interpolation cases.
- AE red: six raw modes; chromatic-log: sixteen errors and one multiplier; ELL1H: nineteen whitened-data differences; single-pulsar quadrupole/GW-single/memory/cosmic-string: 770/750/448/468 failures.
- DDH: 401/433 current/historical fidelity failures and two shared-solve raw-order differences; two first-pass SWGP unwritten-state boundaries; AF: seven raw polyco log10RMS failures. Versioned historical 1 ns ideal passes remain separate from fidelity acceptance.
- AH's twelve groups: `pre-list-bary-unwritten` (unproduced UT1/missing PAR), `pre-list-tempo1` and `pre-list-terrestrial` (position/velocity signed zeros), `dm-grid` (ill-conditioned RMS/PAR digits), `red-noise` (F2 digit), `ell1h-h4` (offset/RMS), `waves` and `wave-dm-scale` (unfitted errors/missing PAR), `global` (F2 digits), `cheby-odd-defaults`, `cheby-ecliptic-tdb` and `polyco-prefix` (coefficient digits). Absent DDH derived inputs and all-deleted span boundaries remain.

Linux provider/ABI implementation and execution, measured predictor long-double conversion, Lomb and QR compiler-contraction behavior, integrated installed consumers on both platforms, both clock versions, complete source/capability closure and broader scientific/resource coverage remain AL obligations. The scoped diagnostic correction above does not close those items. Preserve 0087 clocks, 0090 weighting with warnings, Rug128, 0012 raw-order comparisons and the limited 0073 allowances. No sorting of raw modes, weakened tolerance or reclassification of a failing check is authorized.

## Central review artifacts

Authored executed recipes and explanations are under `decisions/evidence/parallel-execution/`. The earlier implementation replay and provisional benchmark snapshots retain their identities. The final audit/client replay is preserved separately under ignored `build/handover-review-20261027/`, with original receipt paths unchanged and an explicit preservation manifest. Cargo target caches are excluded except for the selected executed consumer binary. The original final replay is `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-ak-handover-review-_v5v511v`.

| Artifact | SHA-256 |
| --- | --- |
| Earlier six-test completion | `e0d7f14cf5af24d1a5be2e8c5e3a054e955418173510744a3e7400b0c45e7cbb` |
| Final central audit | `84a6c12ff443233e2ed234c53585b50a8027de96ee727a07f0c52155d4498222` |
| Final central completion | `e0dc5a83e849f4f8d5a86d24d4e8b26643bc6d63108f7d792feb6a7dcf7e9c83` |
| Final central commands | `2d7d829528a7164a6613f77b01ae0bdd9115972e3a2aaf640eea506a73b4ae70` |
| Executed central review script | `0c753dd38d7b683a9c0a5d17635deed61a78bb0fe8402ba8b6c05d08b603c4e0` |
| Fresh installed Rust client | `c1baf4d8eca6de752fe3596c72b4269574a794de4d43185d104d332eeb9cae6d` |
| Fresh C client | `8c181be0359e16c2d4d9a946f0cc8362bf959ce8537e77291da3721855960c0f` |

The owner report retains final qualification, corrected native, performance, provider, install and sanitizer receipt identities. No generated evidence is committed by this decision.
