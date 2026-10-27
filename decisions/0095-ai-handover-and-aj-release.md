# AI native backend handover and AJ library-interface release

Central decision, 27 October 2026. **Approve AI's implementation handover under 0074, close AI's Rust ownership, and release Assignment AJ for installed Rust core/toolkit APIs and standalone predictor C compatibility under 0084.** The 0082 checkpoint is complete. No new consequential user choice is required. This handover does not accept the inherited numerical gates or the full migration.

Reviewed implementation: `4c0481172217d667990cc994170a7d3d20034a8e`; validation/API documentation: `1a9f06ff7f20c9c84cc29eeb8a486ef653f0295c`; final QR diagnostic correction: `e96b5898827cfd15f0890431922460860d05d349`; delivery report/audit: `20e368fe6e7d3e05565ae7f0132f4498dfa9069e`. The final report commit changes no production or exercised observer bytes. The main tree was clean at review. Canonical TEMPO2 and the independent reference are clean at unchanged pins `fda6ad4a9753ad7dc5d291406f2f310a9161423f` and `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`.

## Delivered capability and evidence

AI delivers real CALCEPH time/position processing and all five scoped BLAS/LAPACK helpers through the canonical direct, timing, fitting and native CLI routes. Owned CALCEPH descriptors close after success and failure. The source two-part dates, units/frames, first-pulsar dispatch, accepted current-pulsar time correction, mass/selected-planet operations and ordinary-long-double versus Rug128 boundaries remain explicit. Shared matrix, Cholesky, robust, constrained and local/global fitting use the same owners and session rules.

Cargo's disabled default, independent `qr-default`, native command defaults/ordered selectors and JSON default zero remain distinct. Enabled provider errors return errors. QR retains its augmented chi-square, covariance scaling and optional-output behavior; SVD retains its own data-row statistic and unscaled covariance. Solver metadata describes the actual algorithm: QR has no invented SVD rank, spectrum or retained modes. Public/source/type/unsafe/departure details are in `rust/NATIVE_BACKENDS_API.md` and `rust/NATIVE_BACKENDS_REPORT.md`.

The executed provider profile is macOS 26.7 arm64, SDK 27.0, Rust/Cargo 1.97.1, GCC/GFortran 16.2.0, Apple clang 21.0.0, CALCEPH 4.0.5 and Accelerate legacy LP64. CALCEPH's official host timed out; the report identifies an immutable public-mirror commit and a matching 533-file source tree, without claiming an upstream signature. Actual INPOP17A TT-TDB data and a position-only DE405 failure neighbor were used. This is a measured provider correspondence profile, not external ephemeris calibration or Linux qualification.

Central independently completed:

- A locked offline combined-release build from the committed source archive, followed by the exact `e96b589` correction. The only production change from the initial `4c04811` build was `rust/src/tkfit.rs`. The initial source archive, successful build and binary are preserved separately. All 201 production and exercised verification files at delivered `20e368f` equal the final fresh review bytes.
- Five focused backend/recovery tests with the real-provider case explicitly enabled, five comparator controls, all **59 paired combined-release cases / 41,961 comparisons**, 21 ordered-selector/JSON commands and two feature-banner controls. All pass; all 375 recorded paired output files retain their hashes. Native executables were independently hash-checked against the recorded source/config/object/archive identities; provider/input bytes were also checked.
- A read-only audit of the complete final delivery: **3,205 unique files, zero hash or membership mismatches**. This covers eight native build profiles, seven numerical matrices, five CLI profiles, eight banner profiles, provider identities, test logs, failure/recovery evidence, final authored validation and raw pinned source correspondence. The final owner matrices contain **265 paired cases / 255,921 compared cells, zero failures**, 105 selector/JSON commands, sixteen banner commands and eight public failure/recovery commands. Original source hashes and the independent reference archive remain bound to each recorded native build.

Owner logs additionally record disabled debug/release library suites (91 passed, 36 ignored), combined release (92 passed, 36 ignored), an earlier combined debug suite explicitly identified in the report, final focused builds and explicit real-provider debug/release recovery, formatting and disabled/combined Clippy. Central verified those identities and logs instead of repeating unaffected whole suites. Ignored historical tests are not passes. These serial checks do not qualify concurrent sessions, every model/provider/nonfinite branch, allocator exhaustion or malformed provider workspace responses.

## Corrections and preserved history

The bounded corrections follow existing 0053/0066/0081/0093 decisions: BLAS declarations under their actual guard; QR matrix/RHS preparation independent of optional parameter output; current-pulsar CALCEPH correction; explicit failed-compute errors and owned cleanup; guarded workspace/ABI/extent/allocation boundaries; and recoverable diagnostic-file writes. Source/cause/fix comments and unsafe necessity/extent/lifetime explanations remain adjacent to the implementation. Native failure witnesses are separate from corrected comparisons; unwritten native buffers are not numerical goldens.

Central found two verifier false acceptances: a small finite sign reversal could pass, and one QR quantity could borrow another family's tolerance scale. Focused controls now reject both without widening a gate. Exact row-order checking also exposed an observer serialization mismatch; the final observer uses source order. The enabled no-argument banner omission and the placement of the QR diagnostic before a later failed `awhite.res` write were corrected. Recovery preserves the earlier diagnostic and completed design file, then writes the real RHS when the destination is repaired.

The final campaign is explicitly `profiles-final-v3`; earlier campaigns, their actual comparator/order limitations and the interrupted v2 run remain historical. Native binaries reused from v2 retain verified identities; final Rust binaries and comparisons were rebuilt/re-executed. Central's earlier probes, initial build and final replay likewise remain separately identified. No historical receipt was relabeled as a final pass.

## Major-stage checkpoint and next contract

The new usable capability is optional ephemeris/time and numerical-provider integration across the delivered Rust engine. The next missing capability is external use: installed Rust core/toolkit consumers, standalone predictor C callers and the getPeriod-equivalent client. AJ's prepared 0084 scope remains appropriate. AK follows for source parallel execution, concurrency and performance; AL follows for integrated installed qualification on actual macOS arm64 and Linux x86_64.

No new user choice is needed. Interface scope (0084), platforms (0086), paired DDH clocks (0087), source spectrum weighting with warnings (0090), precision and numerical gates remain unchanged. The provider profile and source fixes fit the already released AI contract. Linux BLAS/LAPACK support remains actual implementation work: AI's current Accelerate binding rejects non-macOS builds. AL must implement/select a measured compatible provider boundary and execute it, including the native QR accumulation/contraction behavior, rather than treating cross-compilation or macOS results as Linux evidence. The measured arm64 QR FMADD and AG Lomb contraction are profile-specific obligations, not universal rounding permissions.

AI ownership is closed. Its chat must make no further Rust or validation edits after closure. **AJ becomes the sole Rust writer**, owning `rust/` and new `validation/library_interfaces/` against delivered baseline `20e368f` plus this central release. Other chats' changes, canonical TEMPO2, accepted runners and historical evidence remain preserved. Central retains shared design, numerical policy, cross-stage coordination and milestone acceptance. Generated evidence stays ignored under 0039.

AJ follows 0084 and handoff 36. Reconcile the actual AI public APIs, feature/link requirements, scoped acceleration/session ownership, optional QR metadata and solver-specific covariance semantics. Qualify complete source-mapped external Rust consumers and the standalone C predictor contract using the existing scientific kernels; no general legacy pulsar-layout C bridge is required. Preserve 0084's measured native long-double boundary, fixed T1 storage versus declared count, separate phase/frequency reads, names/nearest-pointer lifetimes, source diagnostics/global state and partial supplied-object reads with borrowed-stream positions. The 0/1-count and partial-failure witnesses remain binding; they are not evidence of a completed Rust C ABI. Static/shared linkage, C11/C++ clients, actual loaded artifacts, private-prefix installation, getPeriod behavior and failure recovery all need executable evidence. Keep the complete source scope and all unresolved gates visible.

## Unresolved acceptance ledger

M0, M1a and M1b retain their scoped acceptances; M2 remains unaccepted. The complete 0094 ledger carries forward:

- P's 154 records including 32 SIMUL; M2's four raw SVD-order failures; 0049's four discrete boundaries and three unconstrained interpolation cases.
- AE red's six raw modes; chromatic-log's sixteen errors and one multiplier; ELL1H's nineteen whitened-data differences; single-pulsar quadrupole/GW-single/memory/cosmic-string 770/750/448/468 failures.
- DDH's 401/433 current/historical fidelity failures and two shared-solve raw-order differences; two first-pass SWGP unwritten-state boundaries; AF's seven raw polyco log10RMS failures. The versioned historical 1 ns ideal passes remain separate from fidelity acceptance.
- AH's twelve groups: `pre-list-bary-unwritten` (unproduced UT1/missing PAR); `pre-list-tempo1` and `pre-list-terrestrial` (position/velocity signed zeros); `dm-grid` (ill-conditioned RMS/PAR digits); `red-noise` (F2 digit); `ell1h-h4` (offset/RMS); `waves` and `wave-dm-scale` (unfitted errors/missing PAR); `global` (F2 digits); `cheby-odd-defaults`, `cheby-ecliptic-tdb` and `polyco-prefix` (coefficient digits). AH's absent DDH derived inputs and all-deleted span boundaries remain.
- The recorded parallel failure in three warning vectors remains AK's obligation; the other 6,716 scalars matching in that observation and serial success do not prove concurrency safety.

Installed interfaces, Linux/provider ABI implementation and execution, concurrency, performance, broader scientific/resource coverage and distribution remain unqualified obligations of AJ/AK/AL. Production Rug128, exact raw-mode rules and existing quantity-specific gates are unchanged. Functional handover closes no item merely because a successor can proceed.

## Review artifacts

Authored executed review scripts and their scope notes are in `decisions/evidence/native-backends/`. Byte-preserving copies of 469 selected central files (archives, receipts, logs, outputs, original probes and binaries) are ignored under `build/handover-20261027/`. Original receipt paths remain intact; the extracted workspace and Cargo caches remain at `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-ai-committed-review-8udj_tn7`. The initial archive/build and the final correction are separately identified; the review scripts record the actual session-specific paths and are not a newly executed clean-bootstrap recipe.

| Artifact | SHA-256 |
| --- | --- |
| AI final delivery index | `ceb102a64449abefdc02c1557a6e12c2269550150d69a3ef8e7c08f9f60decc3` |
| AI final profile matrix | `6c0e583a3a0467e090cc6696ea5e08dbb7f96db9fcd3fe6e555c81940466c4df` |
| Central fresh-review receipt | `d9cdcb1c1914438e6db9ab1f5516274f77733572d74dddeff7c6918d13be4f3f` |
| Central fresh paired matrix | `d58d010718162c559ca174e52e9bb357f15d8cc39125a2f4fe320b5194b94812` |
| Central final evidence audit | `7506f79c4f51c15b0b0f00b87177d2f0cea22bfbebe94103a38abdcf61c9f13f` |
| Central preservation receipt | `09d7a9ce98439b18160ba4b05350dd0b9c0d25e6aeb88b412fa128623b0a287d` |

The owner report retains the provider, source archive, native header/archive and real input hashes. No generated JSON, archive, binary, large comparison dump or provider distribution is committed by this decision.
