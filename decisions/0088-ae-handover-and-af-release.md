# AE implementation handover and AF predictor release

Central decision, 26 October 2026. **Approve AE's implementation handover, close AE's Rust ownership and release AF for predictor generation/evaluation under 0076 and handoff 32.** This applies 0074 after the major-stage reassessment below. It does not accept the unresolved scientific/numerical gates or the full migration.

## Released baseline and capability

- AE implementation: `ddff7f82cfe05bc152e55ddbdb33f9bfc4d06bc9`, preceded by `8a16880`, `f75eb13` and `1209f31`.
- AE report/API/source map: `ecf41ed1ada8fbaec2dda8a143d11347b13bcd28`; `rust/GENERAL_FITTING_REPORT.md`, `GENERAL_FITTING_API.md` and `GENERAL_FITTING_SOURCE_MAP.md`.
- Underlying AD implementation/report: `a8e5178` / `0f44180`; AC implementation/report: `18c9065` / `d47ae51`, with their pending acceptance explicitly retained.
- Canonical TEMPO2: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; independent A2: `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`. Both independent checkouts were clean at review. Original 0008 remains unchanged.

AE delivers executable canonical `timing --fit` local/global/NITS workflows, direct preparation/fit/postfit APIs, derivative/update and constraint families, covariance/whitening and robust/TN processing, legacy wrappers, automatic covariance, seeded bootstrap and the necessary SVD/matrix/spectral helpers. The existing bounded adapters share the same canonical state and solver. API/source maps distinguish later toolkit, predictor, output, enabled-backend and ABI responsibilities from delivered helpers. This is the functional foundation required for a successor, with the reported limitations retained.

## Central review and executed evidence

Central inspected the contract, routine/header/source maps, main lifecycle, global layout/update extraction, typed constraint ownership, bootstrap scratch ownership, legacy interfaces and comparator membership/unit/class/physical-update checks. No new production unsafe operation or FFI declaration was added in the AE diff; the formatter extensions reuse the existing fixed-format binary64 boundary. The reported focused corrections retain source/cause/fix comments and separate original/corrected witnesses under 0053. This bounded review is not an exhaustive new body-by-body scientific validation.

The delivery index SHA-256 is `971f96f9ca9c8aafb519756235343d8bb369ec24a8842edf4f174797cfe6a7c9`. Central independently matched all 61 receipts, 710 comparison/export/identity files and 245 authored files, plus the two still-present native executable/archive identities. The 30 indexed Rust executables were no longer present. Their absence is recorded, not silently replaced with new hashes or described as changed source evidence.

Central therefore built the committed release binary and direct `fitting_ideal` example with locked offline dependencies in an isolated temporary target directory. Actual review host: macOS 26.7 (25G229), Darwin 25.6.0 arm64; rustc 1.97.1; retained Fortran compiler GCC 16.2.0. This is a fresh build identity, not a recreation claim for the missing binaries.

Fresh independent native/Rust runs passed **9,061 checked quantities across four public workflows**: global, nonzero parameter constraint, robust H and covariance weighting. **25 additional comparator mutations were rejected**, covering missing/duplicate membership, cycle substitution, error scaling, postfit physical values, global update ownership and constraint targets. Separate fresh Rust DDH runs reproduced the delivered 401/433 current/historical fidelity failures against the immutable native traces, and all 284 historical-clock observations passed the unchanged 1 ns ideal check both before and after fitting.

AE's existing debug/release evidence remains attributed to its recorded identities: 12/14 public cases and 43/58 family cases pass; 42 solver/output cases, standalone callbacks, covariance/spectrum, bootstrap/legacy/history/recovery and correction witnesses are recorded separately. The report records formatting, Clippy, 68 passing library tests, 34 explicitly ignored tests and 92 selected AD regressions. Central did not rerun those complete campaigns or claim the ignored corpus passed. Linux and enabled acceleration are unqualified.

Authored reviewer: `decisions/evidence/general-fitting/central-handover-review.py`, SHA-256 `f91d1e3eb64af6fdd1bf7cfc566350ca19db748d0745845386b6abc570914ad8`. Run it with explicit `--root`, fresh `--out`, `--rust` and `--ideal` paths after building the declared committed profile. It preserves accepted evidence and requires a new output directory.

Preserved ignored evidence: `decisions/evidence/general-fitting/build/handover-20261026/`; receipt SHA-256 `be7e3d8ec26a195a8ab99eb989bd78fe694c1da321301210703289c11b3ada55`. Environment record SHA-256: `32ed3a25c316d367eea29ff245fad11dcd4dce926a0c384312df4d11d1595850`. The original execution root is `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-ae-central-sdxbqqhr`. Only authored source and concise documentation are committed; generated JSON, traces and binaries remain ignored under 0039.

## Unresolved ledger inherited by AF

| Check/input | Observed result and current localization | Acceptance disposition |
| --- | --- | --- |
| P preprocessing | 154 field/event differences, including 32 SIMUL records; 0077 localizes the existing precision boundary | Pending; Rug128 and exact discrete semantics unchanged |
| Historical M2 | Four raw SVD-order failures | Pending under the user's 0012 exact-order decision |
| 0049 utilities | Four quotient/rounding boundaries; three unconstrained interpolation family inputs reach the existing fortran_mod boundary after offsets around 2.5e12 s | Pending; no new clipping, threshold or precision mode |
| AE public red | Six raw-mode failures across two cycles, rank nine and all modes retained; physical fitting/timing gates pass | Pending exact order, separate from M2 |
| AE chromatic-log noise | 16 reconstructed-error failures plus one error-multiplier failure on identical binary64 solver inputs; singular-value ratio about 1.05e26 | Pending; ill-conditioning localizes sensitivity without proving a universal precision cause |
| ELL1H family fits | 19 whitened-data differences across A1/EPS1/EPS2/TASC/H3/PB; matrix, update and timing checks pass | Pending pipeline comparison; retain the named six cases |
| Single-pulsar GW geometries | Quadrupole/GW single/memory/cosmic-string cases have 770/750/448/468 failed quantities in near-null systems; separately tested supported global geometries pass | Pending; no blanket claim of a correct or usable single-pulsar solution |
| DDH under 0087 | Historical ideal check passes, but current/historical fidelity retains 401/433 failures. Shared-input solves retain only two raw-order differences; remaining differences depend on pipeline inputs | Paired acceptance remains pending; original current-clock failure and production clocks preserved |
| SWGP subtraction before first fit | Two TNsubtractSWGP=1 inputs encounter source-unwritten SWGPSignal before prefit initialization | Explicit boundary, not a numerical pass or invented zero default |

The exact cases, fields, failed comparisons, source exits and optional-output/unwritten boundaries remain in AE's report and indexed evidence. The five family execution boundaries are not successful cases. Central owns numerical-policy disposition; after ownership transfer, any authorized production correction belongs to the current Rust writer. No unexplained translation defect is accepted as a numerical-policy allowance. New evidence of one returns to that writer for correction within the unchanged contract.

## 0082 major-stage reassessment

The new executable capability is general fitting, including correlated/global/noise and standalone workflows. The migration remains on the dependency plan: predictors next, then remaining toolkit/GW/spectrum, core CLI/output, enabled native backends, library/C predictor interfaces, concurrency/performance and integrated release qualification. Full scientific scope, source correspondence, necessary-unsafe explanations, macOS arm64 plus Linux x86_64 execution and every inherited/new gate remain required.

The new numerical findings extend the unresolved ledger; they do not require choosing a new solver, precision, tolerance, physical convention or public interface to implement predictors. Predictor generation uses the canonical timing model and its own prescribed construction fit; it does not run the general timing-parameter fit. Therefore the scoped AE findings do not remove AF's functional foundation. Independent predictor approximation error and Rust/native implementation error remain separate obligations.

The DDH acceptance choice was consequential and was explicitly approved by the user in 0087. Interface scope and first-release platforms were already selected in 0084/0086. This checkpoint identifies no additional unresolved choice that must precede AF, so no repeat approval is requested. A new consequential issue during AF returns to the user with alternatives and a recommendation before the dependent action.

## AF release and ownership

AE's implementation ownership is closed. AF now owns `rust/` and new `validation/predictors/` as the sole Rust writer, starting from the exact delivered baseline above plus this central release. Existing `validation/general_fitting/` delivery evidence remains read-only; affected successor checks use AF staging. Central retains design, cross-stage coordination and milestone acceptance. AG through AL remain prepared and unreleased.

Deliver all 0076 predictor responsibilities through the existing core: PAR-only polyco/Chebyshev construction, both file grammars and cross-reader exchange, phase/frequency evaluation, model/set lifecycle and plans. Preserve original versus corrected oracles for demonstrated initialization/format/storage bugs. Do not substitute a native timing-engine wrapper, reduced model set or independent reference-only assignment for the Rust implementation.

Apply 0084/0086 while freezing AF's types: ordinary native long double is binary64 on the measured macOS profile, distinct from the binary128 reference timing typedef and production Rug128. Preserve those source boundaries explicitly; macOS's 53-bit layout is not a portable Linux definition. Subsequent platform/ABI qualification must follow the actual native type and cannot silently narrow a wider Linux value through f64. No new runtime precision switch is approved. AJ retains standalone predictor C packaging/consumer qualification; AF supplies the canonical engine it will expose.

AF must retain this ledger and the 0074 handover/acceptance distinction, use 0053 for demonstrated small bugs, preserve accepted artifacts under 0050 and keep generated outputs ignored. Return committed authored work and `rust/PREDICTOR_REPORT.md` for the next central checkpoint. Full migration remains incomplete.
