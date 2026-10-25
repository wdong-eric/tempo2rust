# Enabled ephemeris and numerical backends: prepared Assignment AI

Central design decision, 25 October 2026. **Prepare AI for enabled CALCEPH and BLAS/LAPACK integration after AH. AI is not released.** AE currently owns Rust under 0079; AF, AG and AH are prepared successors. Release requires the reviewed predecessor baseline, ownership closure and explicit central authorization under 0074. Functional handover and scientific acceptance remain separate.

Once released, AI owns `rust/` and new `validation/native_backends/` as the sole Rust writer. Deliver the optional source backends through the canonical public input, timing, fitting and native-style CLI workflows. Reuse earlier state, matrices, diagnostics, resources, formatting and fitting implementations. Policies 0053/0039/0050/0065 continue to govern demonstrated small source fixes, ignored generated evidence, preservation and proportionate checks. Preserve original 0008.

## Functional scope

The deliverable is a real CALCEPH-enabled timing/fit workflow from PAR/TIM files using source `EPH_FILE`, and real accelerated matrix, Cholesky and QR fitting routes using a recorded native provider. Include callable equivalents of every scoped source body, even the currently unused `accel_uinv`. Source-disabled error/fallback routes remain executable. A stub, synthetic provider or build-only success does not establish enabled capability.

| Pinned source | Required implementation responsibility |
| --- | --- |
| `readEphemeris_calceph.C` | Complete enabled `readEphemeris_calceph` and `tt2tb_calceph`, disabled errors and shared `convertUnits`. Reuse the existing conversion helper and canonical observation/resource owners. |
| `global.C`, `tt2tdb.C`, `readParfile.C`, `readEphemeris.C` | Actual selectors, CALCEPH time/ephemeris pass ordering, per-observation state and source backend distinctions. Preserve the ordinary JPL workflow and existing parser behavior. |
| `T2accel.C`, `T2accel.h` | All five bodies: `accel_cholfac`, `accel_uinv`, `accel_lsq_qr`, `accel_multMatrixVec`, `accel_multMatrix`; source configuration and status conventions. |
| `TKmatrix.C/.h`, `TKfit.C`, `cholesky.C` | Enabled dispatch through the shared matrix/fitting/whitening engine, including constraints, covariance/error outputs, optional outputs and failure propagation. Reuse AE's completed routines and corrections. |
| `configure.ac`, `Makefile.am`, `tempo2.h` | Minimal Rust build/provider configuration, recorded macro/default equivalents, ABI/type boundaries and dependency provenance. Do not link the legacy TEMPO2 scientific core as an implementation shortcut. |

Freeze minimal build features, selected provider versions and runnable examples against the actual released predecessor. Use the same provider and configuration for the native and Rust comparisons. Include disabled, CALCEPH-only, BLAS-only where supported, LAPACK/BLAS and combined profiles; classify actual source configure/build limitations explicitly. Build availability, runtime acceleration and the selected solver are distinct facts. Local dependency autodetection must not silently change scientific defaults. The existing JSON adapters retain their explicit acceleration-zero baseline. The native-style profile must account for source `QR_DEFAULT` (initial value 2 when defined, otherwise 1), with its chosen configuration recorded. Runtime 0, 1 and 2 dispatch must follow the source.

## CALCEPH contract

- `EPH_FILE` sets `useCalceph=1` and the ephemeris path; `EPHEM_FILE` only sets the path. Retain both parser orderings that enforce IAU2000B when CALCEPH and TEMPO T2CMETHOD meet. Follow actual code rather than treating the keywords as aliases.
- The global outer dispatcher uses the first pulsar's selector. Clock processing calls CALCEPH time correction before `tt2tb`; ephemeris processing calls CALCEPH positions, observer coordinates, `tt2tb`, CALCEPH time correction and CALCEPH positions again. Preserve this ordering and all existing time-resource dependencies. Any change requires demonstrated source-bug evidence, not a guessed cleaner ordering.
- Position JD arithmetic uses the timing `longdouble` type before splitting at the C integer cast into two doubles. The time-correction routine instead assigns the expression to ordinary C++ `long double` before splitting. Preserve that separate rounding boundary: the current macOS arm64 reference has a 53-bit ordinary long-double significand, distinct from the source timing typedef's 113 bits and production Rug128. Measure and record native types for every additional platform; never apply a blanket binary64 conversion to the position path.
- Use the source body/center IDs without the NAIF flag, `KM|SEC` for positions/velocities, and the existing conversion into light-seconds and light-seconds/second with SI scaling and coordinate rotation. Capture all written vectors and untouched state; the header comment incorrectly promises some legacy-JPL writes not present in this body.
- Selected planets use the source `paramSet == 1` condition. Their next state is evaluated at `jd0 + 1`, a full day later; the differences are stored without a new derivative normalization. Preserve the later truthy mass-flag condition and subtraction across all six Earth components. Do not replace the actual call with the commented JPL increment.
- Time correction calls `calceph_compute` with target 16 and center 0, then writes the negated first result into `correctionTT_calcEph`. That result is seconds and only the first output element is defined for this target; no extra day conversion or six-element consumption is valid. Use ephemeris data that actually supply this time correction. A position-only file cannot prove the full workflow. These provider semantics are documented in the [CALCEPH 3.5.5 C manual](https://www.imcce.fr/content/medias/recherche/equipes/asd/calceph/html/c/calceph.multiple.html); verify them against the selected build's headers/manual as well.
- Own and close each descriptor on success and every failure. Start with the source per-pulsar/per-call opening lifecycle; add no speculative cache. Provider calls report failure explicitly. Same-descriptor sharing has provider conditions; do not add unsafe Send/Sync based merely on an opaque pointer. Preserve resource identity and failure-followed-by-valid-call behavior.

## Accelerated numerical contract

Reuse canonical contiguous matrix ownership. Preserve all source f64 boundaries, layout, leading dimensions, transposes, optional outputs and in-place mutations. Keep necessary FFI small and document adjacent pointer validity, extents, aliasing, ownership and actual provider ABI. Verify symbol spelling, integer width and character calling convention from the chosen provider rather than assuming every BLAS/LAPACK has the same ABI. A small binding shim is permitted where required for ABI safety; it must not reimplement the timing or fitting engine.

`accel_cholfac` calls DPOTRF with upper column-major storage, which represents the lower triangle in TEMPO2's row-major array. `cholesky_formL` copies the source matrix before that call and retains the other triangle; compare the complete defined mutation, not an invented zeroed matrix. `accel_uinv` uses DPOTF2 and packed DTPTRI, then explicitly unpacks the lower row-major inverse and zeroes the upper triangle. Preserve their distinct results and nonzero status handling. The provider contracts are [DPOTRF](https://netlib.org/lapack/explore-html/d2/d09/group__potrf_ga84e90859b02139934b166e579dd211d4.html) and [DTPTRI](https://www.netlib.org/lapack/explore-html/d4/d92/group__tptri_ga5f8af5d725a5c1cd3423b5ae8c882171.html).

`accel_lsq_qr` passes the row-major design as a transposed system to DGELS, queries workspace, mutates both matrix and data, copies the first parameter entries when requested, and sums the remaining data entries for chi-square. The covariance path packs/inverts the triangular factor and forms the source product using DTRMM. Retain the source rescaling and optional covariance behavior. DGELS assumes full rank and detects exact zero triangular diagonals; do not silently add an SVD rank cutoff or promise detection of every nearly singular matrix. See the [DGELS contract](https://netlib.org/lapack/explore-html/d8/d83/group__gels_gaa65298f8ef218a625e40d0da3c95803c.html).

TKfit passes `ndata + nconstraints` to the QR routine, including constraint targets in the augmented RHS. Consequently this route's returned chi-square and rescaling denominator include the augmented rows. Preserve this source branch-specific definition and distinguish it from the SVD data-row statistic and model fit degrees of freedom. Reuse AE's demonstrated nonzero-target SVD correction; retain original and corrected evidence separately. Unavailable QR follows the source warning/SVD fallback; a failed enabled QR follows its error path and must not silently switch algorithms.

Matrix-vector multiplication uses the source DGEMV transpose and matrix multiplication reverses the operand order in DGEMM to account for row-major storage. Verify rectangular and nonsymmetric inputs as well as full fitting callers. Runtime mode 1 can enable matrix/Cholesky acceleration while retaining SVD; mode 2 selects the source QR branch when compiled. Optional native acceleration does not replace Rug128 production timing arithmetic or resolve the inherited raw SVD-order gate.

## Bounded source-defect questions

Use 0053's original/corrected witness, adjacent source/cause/fix comment and focused regression if a realistic call demonstrates these defects. These are preparation findings, not completed corrections or new goldens:

1. CALCEPH compute return codes are ignored, permitting unwritten/stale values after a missing target or out-of-range time. Unselected planet differences can also read state not populated by this call. Establish actual initialization/producers and preserve defined input state; do not invent a scientific result for missing data.
2. `tt2tdb.C::tt2tb` uses `psr[0].obsn[i].correctionTT_calcEph` inside the current-pulsar loop. Check two distinct pulsars and observation counts to demonstrate any cross-pulsar error before a local ownership correction. Keep the separate first-pulsar outer dispatch decision explicit.
3. BLAS declarations/macros in `T2accel.C` are nested under HAVE_LAPACK while BLAS bodies are guarded by HAVE_BLAS. A BLAS-only compile witness determines whether a narrow declaration-guard correction is needed; do not claim that profile tested from a combined-provider build.
4. TKfit's QR branch allocates the augmented matrix only when `computeParam` is true but fills it unconditionally. Exercise valid covariance/error-only requests, actual dimension/workspace boundaries and failed output-file creation. Correct proven absent-buffer or unchecked-query handling at its source while preserving successful scientific behavior.

Existing deliberate source choices remain the default. New physics, precision, tolerances, or broad multi-pulsar/backend policy changes require central disposition while independent implementation continues.

## Verification and handover

Implement the enabled workflows directly using existing AC/AD/AE/AH inputs and evidence; no separate reference-only assignment precedes AI. Add focused native wrappers and an enabled disposable native build inside AI's owned directory. Record provider/library/header/compiler identities, native configuration, resource hashes and exact runtime settings. Never modify canonical TEMPO2 or accepted reference runners/results. Downloaded/generated ephemerides, libraries, binaries and JSON stay ignored; commit authored recipes, fixtures and concise reports.

Before comparison, freeze the public examples and quantity-specific projection against the released interfaces. Require real CALCEPH timing and fitting, repeated calls and multiple pulsars; frame/time-unit and selected-planet branches; enabled and disabled profiles; full-rank QR with/without covariance rescaling; constraint and global/local fits; correlated-noise whitening; mode 0/1/2 dispatch; rank/factorization/provider failures followed by valid work. Exercise all five accelerated wrappers directly and check complete defined mutations, statuses, row/column identity and output membership. Singular/invalid original calls receive their actual source classification, not a fabricated finite oracle.

Use exact identities, selectors, counts, ordering, classes and branch/failure comparisons. Reuse timing `1e-12 s`, phase `1e-10 turn` and existing fitting/covariance/Cholesky contracts for the same quantities. Freeze new quantities with units, native types, justified scales and structural-zero rules before comparison. Compare QR to the same native QR route, not to a reordered SVD spectrum; provider changes require separately identified evidence. The P preprocessing differences, M2 raw-order failures, 0049 discrete boundaries and DDH ideal gate remain open. No new backend or passing public subset waives them.

Run affected debug/release components and representative public workflows, disabled-profile regressions, resource/CLI recovery, formatting and Clippy. Hand over authored Conventional Commits, executable recipes, source/type/unsafe/departure maps and `rust/NATIVE_BACKENDS_REPORT.md`. Under 0074, localized numerical-policy findings may remain explicitly open at implementation handover; all applicable gates remain mandatory for full acceptance.

OpenMP/parallel execution and concurrency/performance qualification, remaining SOFA dependency disposition, core/predictor/toolkit C interfaces and installation/platform/release qualification remain explicit later obligations. AI does not equate a serial backend test with thread safety or completion of those responsibilities.

## Preparation evidence

Central inspected the enabled CALCEPH bodies, global/time/parser call sites, all five acceleration bodies, matrix/fit/Cholesky dispatch and native configuration, and checked the cited primary provider contracts. Fifteen source files were read from raw `git show` at TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; hashes below avoid checkout Id/line-ending transformations. This is source preparation only. No enabled provider build, new Rust execution, platform qualification or scientific acceptance is claimed.

| Source | SHA-256 |
| --- | --- |
| `readEphemeris_calceph.C` | `387ce174dd7108ac335bab4794d1efcc78ad592323a33a11c4a5c6b9bde05b1d` |
| `readEphemeris.C` | `822678396155955e8f55f307b373d1666b4def988d0b21e0762bf6fd5e53ce3a` |
| `tt2tdb.C` | `c62c6c599f8101703ed511a80653bc53d09ff7a4af89ddd98b8b4d936a222426` |
| `global.C` | `e61fc1a83a9adf0b6d905d2c5a58cd764a1fd0261c276fccb33daadb6788a2a7` |
| `T2accel.C` | `e90d592067b5d0ab273262eb488fecc5771ef7d612cf276d7ed78c446b923a0f` |
| `T2accel.h` | `3604c0967d53c64e0f611943809524a7d79ff3b8face432912323f67f8c8cc00` |
| `TKmatrix.C` | `3817bedcd680e88be6b715b1165eacec028f942d3213eec1b0f02c097a01fb11` |
| `TKmatrix.h` | `b089efa5ddd1dda2c2615a41a119d015209b908858899713e4ec6bd47270ca84` |
| `TKfit.C` | `18c4124563edceeea3f19694aec1256288fa1e744f69bea97c2070c2c59f4abc` |
| `calculate_bclt.C` | `899c58042bc053efeae5b25eacc46de9b4ca1bb0943c995a846e0affb8d597e7` |
| `configure.ac` | `1be741a57a7cfd2001da6f98327ff7b6a34e5657be6e13db72de912960dd8aa9` |
| `Makefile.am` | `172ec480482f0261f0a845e9aface382c1a441f4e80d53132574bf8ff912bd7a` |
| `tempo2.h` | `0313b6eaf9f9ae9d7e5cce340379ad4b7068394e5222a01f73949e3a2640de93` |
| `cholesky.C` | `1c3b572adb0a751ab1d9c5c8bd9d9222c5434ff49ad211483df6b6ead0aff1c5` |
| `readParfile.C` | `ecdbec472eff256ab1c7d875a4ea56536c146012599684408b03fe738ac7550e` |
