# General timing-model and residual implementation: prepared Assignment AD

Central design decision, 25 October 2026. **Prepare Assignment AD as the next functional implementation stage after AC. Do not start AD until AC is centrally accepted, its Rust ownership closes, and central records an explicit release at the accepted revision.** AC remains the sole Rust writer. AD will own `rust/` and new `validation/timing_models/`.

This follows priority 0065: use general input-to-BAT capability to deliver full fixed-parameter timing/residual evaluation, including all fourteen binary families. Reuse G's accepted binary evidence and the input/propagation/resource implementations. Additional native experiments belong to specific model, lifecycle or correction questions inside this implementation stage. No new independent reference-only prerequisite is created. This preparation accepts no implementation and changes no existing M2/0049 gate.

## Public capability and integration

Add the general `timing` library/CLI workflow on AC's canonical loader, preprocessing, runtime and BAT orchestration:

```text
tempo2-rust timing --par MODEL.par --tim INPUT.tim --runtime T2runtime
tempo2-rust timing --par P0.par --tim P0.tim --par P1.par --tim P1.tim --runtime T2runtime --no-mean -- SOURCE_PREPROCESSING_ARGS
```

Ordered pairing and preprocessing arguments follow AC. Execute full formBatsAll and then formResiduals once at the supplied fixed model; this command performs no fit. Default removeMean is 1, matching pinned tempo2.C:538; `--no-mean` selects 0, also needed by source predictor callers. Expose direct residual and binary derivative/update calls on canonical state without CLI parsing or a needless rerun of the BAT pipeline. Freeze concrete signatures and the small output schema against accepted AC before implementing consumers.

The general result includes each pulsar's ordered main observations and separate TZR state: SAT/BAT/BBAT/PET, binary delay, phase, nphase, exact pulse number, residual and residualtn, deletion/selection state, applicable averaged fields, source mutations, units/frames and actual input/resource provenance. Keep mean-removed residuals distinct from phase/count fields and the source's residualtn lifecycle. Preserve original row identity even when source preprocessing sorts or filters. Numeric-class and unwritten/inapplicable-field dispositions must be explicit; do not invent finite results or empty scientific values.

The existing bounded `load`, `residuals`, `evaluate_barycentric`, `evaluate_terrestrial` and `fit` APIs/schemas retain their accepted contracts through thin validation/output adapters. The general `timing` result represents multiple pulsars and source-mutating lifecycle explicitly. There is one authoritative scientific implementation per routine: extend/replace the old selected form_residuals body and route compatible callers through it. Reuse AC's canonical state and owned working-copy pattern where an existing immutable API needs one; do not create a second parameter store, alternate timing engine or configurable numerical backend. General fitting remains a later stage even though binary callback bodies transfer here.

## Source responsibilities and existing evidence

Use raw TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. Central inspected the complete formResiduals.C body, the eccentric-GW dependency, actual binary dispatch/wrappers and selected problematic model branches; existing G maps supply detailed model/label navigation. Central has not independently reviewed every binary expression in this preparation. At release, reuse those maps and freeze AD's actual interfaces/field/lifetime/branch dispositions against accepted AC, instead of repeating a whole-repository inventory.

| Component | Required transfer |
| --- | --- |
| `formResiduals.C` | All three defined routines: formResiduals, averageResiduals and averageDMResiduals. residualTracking is a declaration without a body/call here, not a fourth implemented routine. Include every active phase, tracking, noise-subtraction, averaging and output branch. |
| Fourteen binary files | All 42 defined routines, including delay/derivative selectors, all update functions and private/shared helpers. Families: BT, BTJ, BTX, ELL1, DD, DDK, DDS, MSS, DDGR, T2, T2-PTA, DDH, ELL1H and ELL1k. |
| `t2fit_stdFitFuncs.C` | Only t2FitFunc_binaryModels and t2UpdateFunc_binaryModels in this stage, using their actual selected-pulsar offset and wrapper narrowing. Other fit functions/driver responsibilities remain for general fitting. |
| `GWsim.C` / `GWsim.h` | The six-routine eccentric-source closure called by formResiduals: eccRes, Rs, dadt, dedt, dtdt and Fe. Remaining GW simulation/background/polarization/file APIs, including eccResWithEnergy, remain on the later core-API inventory. Do not claim the whole file transferred. |
| Existing dependencies | Reuse accepted AC's ifunc/shapelet, parameter links, full propagation and TZR lifecycle; AA/W/Y's geometry/time/resources; canonical integer/extended numeric utilities, diagnostics and byte scanners. Preserve source-specific helper variants where their formulas/types differ. |
| Public integration | General input-to-BAT followed by residual evaluation, source-compatible averaging output, library entry points, CLI serialization, examples and recovery. No enabled-CALCEPH, full upstream CLI/textOutput, fitting or predictor acceptance is implied. |

Each binary file contains its model and updater. Additional helpers are DDGR::mass2dd; ELL1H::calcDH; T2-PTA::computeU; and T2::{getParameter, calcGR, getKeplerian, addKeplerianJumps, getPostKeplerian, updateParameters, deriveKeplerian, derivePostKeplerian, KopeikinTerms, computeU, calcDH}. Keep different computeU/calcDH bodies in their corresponding modules unless equivalence is actually established. Use source-mapped Rust filenames and original domain notation, not a replacement generic orbital framework.

G was accepted under 0014 at `95d25c1`, `6027e11`, `f550a6e`: 75 isolated cases, 54 finite integrated cases, 187 named derivative-label pairs, 189 named update-label pairs and T2-PTA's catch-all update. These are bounded evidence, not exhaustive runtime branch coverage. G's source inventory hashes the checkout with expanded `$Id$`; central verified all fourteen against the raw pin by removing only that expansion. Preserve that distinction and the original source-specific newline bytes. G's schema assumes its equatorial TCB fixtures; do not copy those fixed units/frames into other general inputs.

Raw source hashes for this contract are:

| Source | SHA-256 |
| --- | --- |
| `formResiduals.C` | `63e604f6671703be82b141ab33883e2df440354c35121982be4a7a626e874a48` |
| `BTmodel.C` | `f5caa06a43f71a827088f7d3384a2c58f442c8f536b86e2322c3c0d1b3d0964d` |
| `BTJmodel.C` | `542e1ab0bd3a9f8fd9b305e890d3a5eb567b861e46104236e5a4ee1b0ed0ea1c` |
| `BTXmodel.C` | `c626d4a7b306a8954e50a73e03c22d844127068423d7632afc863a5101b14373` |
| `ELL1model.C` | `7367fb647915d27308bd758a9da42e80f9b010b5be380a6e0936a7aae11c5f79` |
| `DDmodel.C` | `370a501041babbf49712896a5ccda131872127918f3320d8da3f879c608cd275` |
| `DDKmodel.C` | `014259c06b8052b761f4848e0804e56b40fda8de35499679244950216c8be31d` |
| `DDSmodel.C` | `e17694dfb781d51f115d4b350c17c26d3bf7a48631b392f49acfd5648484fe60` |
| `MSSmodel.C` | `93774d815690c6eee904412d64be5c40c38dffe4307f1d0afae3f91858dbcccf` |
| `DDGRmodel.C` | `5d4f21758a2ed7bef1fc0a75a937e5269ee5da2c67c040ae52aa4c504c1010ce` |
| `T2model.C` | `23c6d978eb0e33882d78bed5265293cb887af9ce82325aad181326de4affb0e2` |
| `T2-PTAmodel.C` | `6589228c216efc55a21106cd82241d21b7f9d9190df6829cb044d9c0bb5d634c` |
| `DDHmodel.C` | `66711b926ca55647c06602abd5ad8ae9163d5a7a623f4207ff912ab8393581d8` |
| `ELL1Hmodel.C` | `47163ad9aa054845f7f5200b82c952d1a829ba6781574b3556c8f9b5c7401867` |
| `ELL1kmodel.C` | `b36ce31c1951c14a4ada4cc8842959a0183c69a35b7e25dd01d0b27bc9544661` |
| `GWsim.C` | `412d74045d9b4d8196c554e38d519a019c021eae02b3b6b1a418f3d21984296c` |
| `GWsim.h` | `e53d5868392d1f0d5dfdc4bd2a2dd2b73bc30b82abec41f7ddf9f7f4e499cc54` |
| `t2fit_stdFitFuncs.C` | `b2b4abb17f56fb058d376ff3987160d096908773be4e4384f2863ebf565fc923` |
| `TKlongdouble.float128.h` | `214911b81f82489996d698626cf8d30ab5bcc20099f6b5c5aa089715b85bc55e` |

## Residual semantics and state

Preserve native expression grouping and the actual numeric boundaries. Source `longdouble` is binary128 in the reference and remains Rug128 in production. Native C++ `long double`, float, double and integer locals are separate types; on the accepted host the first two floating distinctions include native-long-double 53 bits and float 24 bits. The float128 header maps cosl/sinl/floorl/fabsl/powl to quadmath, while other suffixed calls and C++ overloads require their actual compiled resolution. An extended-looking function name does not establish its precision. Retain f64/f32 narrowing, constants and promotions, including averaging powf and timestep. No global contraction/reassociation or temporary 113-bit production mode is authorized. Any local FMA needs an actual compiled-source witness and adjacent explanation.

Binary dispatch occurs only when PB[0] or FB[0] has paramSet exactly 1; BINARY text alone does not activate it. Preserve exact model spelling and the unknown-model exit. Set torb, add it to deltaT and the fractional-day spin expression, and set PET = BBAT - torb/86400 with the source signs. Geometry-dependent models consume the actual AC Earth/site/pulsar vectors and frames, never fabricated zero vectors.

Complete the split integer/fractional phase expression through F12, state switches, braking, glitches and all three recoveries, nudot transitions, exponential/Gaussian events, jumps and FD/FDJUMP terms, red shapelets, WAVE/WAV_DM, quadrupole/single/continuous/eccentric GW, memory/cosmic-string/burst terms, clock offsets, scalar/quadrupole interpolation, and radd/padd flags. Keep each source choice of SAT/BAT/BBAT and frequency units. Do not combine source phase terms into a mathematically cleaner but differently rounded expression. Preserve exact selectors, epoch inequalities, source-written geometry caches and retained non-target fields.

REFPHS_TZR reinstates the stored extra observation before phase construction, copies its complete record, includes the source's intermediate operations, stores it back and removes it at the declared point before shifting main residuals. Use actual canonical row capacity/ownership and retain failure prefixes; do not conflate this phase lifecycle with AC's earlier refphs_init/clean BAT lifecycle.

The phase reference is the first eligible observation in source order, with the source START/FINISH fit-flag selectors, SAT/BAT distinction and START_FINISH_DELTA. Deleted-point predicates are not uniformly interchangeable. Preserve IPERHARM's fractional adjustment, signed fortran_mod/fortran_nlong, all TRACK modes, pn/pnadd ordering, phase-jump corrections and the exact count conversions. Source-valid counts remain exact i32/native-long/i64 operations at their declared boundaries; invalid floating conversion, signed overflow or backing is a contextual error, not wrapping or saturation. Do not sort inside residual evaluation to conceal a source tracking-order error. The four 0049 utility-boundary findings remain explicit; no arithmetic redesign is implied here.

Arithmetic mean removal uses source membership and write order; phase, pulse number and residualtn do not automatically receive the same transformation. Noise subtraction consumes the stored source TNRedSignal/TNDMSignal/TNChromSignal/SWGPSignal and CM interpolation. This does not implement stochastic inference or produce fitted noise signals. Trace their natural reader/initializer/fitting ownership before public use; no harness-only zero seeding is a production default. Repeated calls, zero/nonpositive counts, multiple pulsars, deletion/selection, failed processing and later valid work need explicit lifecycle evidence.

Preserve averaging's actual flag matching/order, inverse-variance aggregation, ECORR/SECORR selection, units, float narrowing, output fields and source avg.dat formatting/write order. Use scoped Rust ownership for all arrays, including AverageFreq, and recoverable I/O errors. Exercise file-producing helpers in owned temporary working directories. Output JSON is published only after the requested calculation and serialization complete; source diagnostics remain distinct from scientific output. Do not silently convert an averaging failure or missing field into a successful zero-filled result.

The eccentric-GW closure retains its incoming previous-orbit state, coalescence flags, source integration count/cap and RK4 expressions, residual-dependent epoch, constants and source 1e9 factor. It must not be replaced by a different physical model or integrator. Branches with no integration still follow actual source writes. The remaining GWsim APIs are not dependencies of this closure and stay explicitly untransferred.

## Binary conventions and corrections

Reuse G's model-specific coordinate map. Native DD returns extended precision; its fitting wrapper narrows to binary64. Other models/helpers retain their own types and casts. Preserve negative exact-integer orbit behavior, source parameter links, conditional defaults, clamping writes, warnings, derivative zero fallbacks and callback/update selectors. A shared parameter label does not imply a shared physical update: notably PB increments, FB powers, OMDOT, M2, BPJOM/BPJPB and MSS second derivatives differ by model. Compare actual increments and errors as well as the final parameter values. Preserve ELL1H's additive error behavior and source-defined NaN/infinity outcomes unless a separately demonstrated correction changes them.

Policy 0053 supersedes old G instructions to defer every bug. Demonstrated small bugs may be corrected within AD with the original file/routine/location, cause and fix next to the code; a focused regression; and original/corrected evidence kept separately. No per-fix permission round is needed. A source-invalid callback does not acquire a numerical golden merely because one native build returned a number.

The following concrete sites need focused disposition during implementation, not a separate reference campaign:

- T2model.C::getParameter:634–651 tests `k > aSize` before indexing, admitting `k == aSize`. G already demonstrates the two-companion boundary. A narrow range correction must preserve the source's defined in-range and beyond-range behavior; verify natural multi-companion inputs and defined neighbors rather than manufacturing scalar backing.
- DDKmodel.C:267 reads dtdpx before initialization on derivative paths; its current value has no return use in the inspected body. Remove or correctly scope a dead invalid calculation only with source/data-flow evidence and corrected-native callback neighbors; do not assign an arbitrary scientific parallax value.
- ELL1Hmodel.C and T2model.C evaluate orthometric intermediates outside the modes that define their inputs. Scope calculations to the modes/selectors that actually consume them where that is sufficient. G's H3-only/H4 records remain original invalid-source evidence; new corrected expectations must be explicit. Changing NHARM mode selection or physical derivatives requires its own demonstrated cause/fix, not an assumed cleanup.
- ELL1model.C's step ORBIFUNC branch uses ifuncN and can leave ival unset; formResiduals.C's step IFUNC branch can also lack an assignment. Do not choose an extrapolation/default value by guess. Establish the exact corrected range/selector contract and source-safe neighbors before accepting that public option.
- formResiduals.C:2411–2454 iterates through `i <= nobs` after TZR removal. Show whether the extra row is active, retained TZR or unwritten on the actual caller path; a correction must protect active membership and noise-mean semantics with a sentinel/spare-row and TZR regression.
- formResiduals.C's reference selection, red shapelet access and averaging calls mix the current pulsar with psr[0]/psr->; tracking/coalescence locals also span the outer pulsar loop. Trace each lifetime and prove any cross-pulsar correction with distinct two-pulsar inputs and unchanged single-pulsar neighbors. Do not reset every variable or rewrite all cross-pulsar accesses merely because they look suspicious.

Other real initialization, scanner, allocation or indexing defects follow the same rule. Trust validated canonical invariants; avoid redundant guards for fabricated internal states. Retain defined oddities such as BT EDOT/ELL1k EPSDOT delay behavior and model-specific update units until evidence establishes an actual defect. Unexplained numerical, scientific-model or shared-interface differences remain central decisions while independent work continues.

## Verification and acceptance target

Freeze authored inputs, intended branches, comparison projections and applicable original/corrected classifications before reading Rust results. Reuse all applicable G records in debug/release, including all fourteen model families and named callback/update membership; withheld invalid values stay withheld unless a separately evidenced correction supplies a new oracle. Add only the missing focused model/lifecycle examples needed to demonstrate the released full routines and public combinations. Original G/A2 files, runners, schemas and membership remain immutable; route all successor work into AD's owned staging.

Use identical fully specified canonical state for direct component comparisons, then independently run authored files through the public native/Rust pipelines. Source-defined field writes, indices, owner identities, row/phase selection, counts, diagnostics, nonfinite classes and mutation membership are exact. An early error, omitted callback or unexecuted branch is not a passing model comparison.

Prospective finite gates reuse the existing timing and matrix budgets:

| Quantity | Gate |
| --- | --- |
| SAT/BAT/BBAT/PET and binary-delay/residual/mean/response values in seconds | Absolute difference at most 1e-12 s; convert MJD differences in extended arithmetic. Apply to each relevant component and final result. |
| Full phase | At most 1e-10 turns, with the independent residual and exact pulse-count gates. |
| Binary derivative columns on identical state | For each model/label/subindex, at most 5e-14 times the maximum absolute reference column value, as in 0007. Structural zero columns and nonfinite classes remain exact. Keep native DD and narrowed wrapper records separate. |
| Stored extended values/errors after a fixed binary update | Relative difference at most 1e-30 for the same supplied state/increment; structural zero exact, following the existing extended-value gate. Also compare the applied increment, source update coordinate and post-update timing response; a large unchanged base value cannot hide a wrong small update. |
| Existing M1/M2 and accepted AC fixtures | Their original gates remain unchanged. M2's four raw-order findings remain open under 0012. |

These are engineering comparison budgets, not independent validation of the physical approximations. Subtraction/increment comparisons must account explicitly for the finite precision of the recorded before/after values; freeze that projection from the source arithmetic before comparisons, not from the observed Rust error. If a new field or ill-conditioned derivative needs a distinct unit-aware gate, present the concrete source and fixture evidence centrally before accepting it. Do not infer permission for a blanket relative tolerance, precision switch, looser pulse count or compensating delay error.

Public examples must include all fourteen baseline binary families; higher spin/state-switch/braking/glitch terms; phase and frequency-dependent jumps/events; wave/interpolation/shapelet terms; representative GW branches including the eccentric closure; source tracking and explicit pulse numbers; deleted/unsorted/cut data and both mean settings; TZR; averaging and stored-noise subtraction; multiple pulsars; and more than one supported site/frame/time choice inherited from AC. Add model-specific index/optional-route cases from G's open list where needed for the complete transferred routines, including higher FB/jump/companion indices. Demonstrate repeated evaluation and failure followed by valid work. No public success may depend on a private probe supplying missing physical state.

Keep the original DDH ideal fixture and its 1 ns threshold unchanged. Reuse its bounded localization, compare the newly executable native/Rust stages, and investigate a concrete shared-component hypothesis when evidence supports it. Matching native DDH is source-fidelity evidence, not a pass of the ideal test. The known first 1.137585 ns / maximum 4.598955 ns discrepancy remains explicit until causally resolved; do not use fitting or a wider port tolerance to hide it, or suspend independent model implementation while it is investigated.

Run affected AC/core/M1/M2 regressions, owned I/O/resource recovery, debug/release comparisons, formatting and Clippy. Review effort follows 0065; a report-only change does not require another native or historical full replay. Record actual source/input/compiler/executable identities and prior-versus-current applicability. Larger fitting, stochastic inference, remaining GW/toolkit/predictor/output APIs, enabled CALCEPH, cross-platform, performance, concurrency and distribution remain later full-core obligations.

## Delivery and release

After explicit release, work through coherent commits: freeze the AC integration/API and authored examples; transfer binary families and their callbacks; complete phase/residual lifecycle and non-binary contributions; integrate public timing/averaging and shared legacy adapters; verify affected components and the full workflow; deliver `rust/TIMING_MODELS_REPORT.md` with source/unsafe/behavior maps and remaining evidence limits. These are internal dependency steps of one functional assignment, not separate reference-only milestones.

Only `rust/` and `validation/timing_models/` become AD-owned. AC must be accepted and closed first. Use explicit-path Conventional Commits. All reproducible JSON/JSONL/compressed traces, generated binary fixtures and large outputs stay ignored under 0039. Source-mapped comments explain every necessary unsafe boundary and demonstrated correction. Central owns shared design, release and milestone acceptance. No AD chat is created or implementation released by this preparation.
