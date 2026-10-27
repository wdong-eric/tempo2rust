# AH core CLI/output handover and AI backend release

Central decision, 27 October 2026. **Approve AH's implementation handover under 0074, close AH's Rust ownership, and release Assignment AI for enabled CALCEPH and BLAS/LAPACK backends under 0081.** The 0082 checkpoint is complete. This is functional handover; AH's twelve comparison groups and the inherited numerical ledger remain unaccepted.

Reviewed production: `0dbcf0842394eb9cfaf93cb31bfac428e2ef7d67`; original validation: `6ef441a1a46a964770352bebdc3395ef12045c10`; observer correction: `50f2bf33baaa3a0bfea0666c062ab5b59ba6b63b`; updated report: `9dec65cfe7696a150f131077494e487871b22fb7`. The main tree was clean at review. Canonical TEMPO2 and the independent reference are clean at their unchanged `fda6ad4a9753ad7dc5d291406f2f310a9161423f` and `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572` pins. Five formerly untracked canonical `.DS_Store` files are now absent; this housekeeping difference is recorded without attributing its cause. Original 0008 and its amendments remain intact.

## Delivered capability and central review

AH delivers the native-style core command entry point and complete assigned stateful text/PAR/auxiliary reporting, derived helpers, display, typed formatting, diagnostics and embedded-help responsibilities on the canonical Rust engine. Existing JSON subcommands remain. Source argument/alias order, one preprocessing pass, outer NITS loop, one `t2_fit` at each source fit point, `-pre`, writetim timing, predictor defaults and output mutations are retained. Direct APIs borrow streams and explicit destinations without changing cwd or environment. Completed writes and mutations survive later I/O failures as documented.

The public/API/type/unsafe/departure maps are `rust/CORE_OUTPUT_API.md`, `rust/CORE_OUTPUT_SOURCE_MAP.md` and `rust/CORE_OUTPUT_REPORT.md`. Six source bodies and 375 decision/loop/case sites are statically accounted for; this is not exhaustive dynamic or astrophysical validation. New unsafe calls are limited to the documented host CPU clock and private validated single-binary64 `snprintf` boundary. No new numerical dependency, production precision or scientific engine was introduced.

Central independently completed:

- Original and corrected delivery-index verification, committed-source correspondence, 27 compiled native source hashes, 33 native object hashes, accepted AF/reference dependency identities, and all five native executables. The old index accounts for 41 authored files, sixteen artifacts and seven executables; the correction adds three authored files and eight indexed artifacts. The 622 historical paths frozen before the observer correction are unchanged. In total, 2,454 indexed paths were verified, including shared resources through their recorded locations.
- A locked offline release build from the exact original validation archive, with all 156 production-source entries matching production `0dbcf08`. Only the subsequently committed `validation/core_output/` correction was overlaid; both archives and source identities are retained. Five CLI/output unit tests, seven behavior tests and four checker controls pass.
- A fresh complete release replay of the 32 paired real workflows and five direct groups: **25 exact groups, twelve failing groups, actual checker exit 1**, exactly the same comparison membership as the delivered debug/release receipts. Each profile's 267 recorded files across 78 executions retains its hash; there are zero artifact mismatches, unexpected statuses or changed inputs. The five direct groups include identical-state output/mutations, extended formatting, helpers, redirected diagnostics and actual stdout PTY behavior. The direct Rust evidence also exercises repeated reports, failed-write recovery, supplied-input prompting and PAR reload.
- A fresh UT1 snapshot and original/corrected native comparison: all 21 checks pass for clock modes 0/1/2. The complete 48-row before/after observation state is unchanged apart from the explicitly initialized UT1 diagnostic. Native BAT/residual/site geometry is unchanged, and the new values equal the independently composed applied clock plus EOP correction. All seventeen recorded small-bug checks and 51 indexed witness files were verified.

Owner evidence additionally records serial debug/release library suites with 91 passes and 36 ignored tests, debug/release behavior and analysis integration, formatting and Clippy. Central verified these logs and identities rather than repeating the unchanged whole suites. Actual Linux, installed ABI, concurrency/performance and the complete inherited corpus were not qualified by this review.

## Corrections and retained failures

The bounded production corrections are approved under 0053: current-pulsar RMS cutoff ownership; unweighted CHI2R initialization from the actual fit; defined prior prefit metadata at first activation; nonglobal fit-count initialization/guard; the Rust empty-constraint DM/CM weight condition; and the modern terrestrial UT1 diagnostic producer. Owned arguments, buffers and typed formatting remove the demonstrated undefined operations with source/cause/fix explanations. The report distinguishes earlier Rust translation defects from upstream defects.

For UT1, retain the site-to-UTC correction actually applied by the existing timing path, add its selected EOP UT1-minus-UTC once in binary64, and promote to timing storage. Bypass clock modes contribute zero. This initializes the existing JSON diagnostic field on that producer path and preserves its schema and all other quantities. No new clock lookup, time model or fallback is adopted; barycentric/satellite paths without this producer receive no guessed value.

Central caught a separate observer defect: `controls/-h` and `controls/-H` identify the same directory on this filesystem, so extended help overwrote the earlier basic-help stdout. Original receipts retained the earlier hash and therefore could not prove its retained bytes. Correction `50f2bf3` uses distinct `help` and `help-extra` directories and rechecks every recorded execution directory before publication. A real overwrite after two successful commands now yields exit 1; distinct outputs yield zero. Old files/indexes remain historical evidence. The fresh central replay verifies the correction without changing any scientific comparison rule.

Both final profiles retain these twelve failures, with seventeen differing files and three missing files:

| Group | Open finding |
| --- | --- |
| `pre-list-bary-unwritten` | No source UT1 producer; explicit boundary, differing stdout/stderr and missing PAR. Undefined storage is not a numerical golden. |
| `pre-list-tempo1` | Displayed pulsar position/velocity signed-zero differences. |
| `pre-list-terrestrial` | Same signed-zero differences, including both NITS passes; modern list completes and PAR matches. |
| `dm-grid` | Ill-conditioned RMS/PAR digits; DM/CM weights and files match. |
| `red-noise` | One fitted F2 final PAR digit. |
| `ell1h-h4` | Offset/RMS display differences; PAR matches. |
| `waves` | Unfitted WAVE errors have no producer; explicit boundary and missing PAR. Fitted WAVE passes. |
| `wave-dm-scale` | Same unfitted-error boundary; fitted scale-1 workflow passes. |
| `global` | F2 PAR digits; global covariance file matches. |
| `cheby-odd-defaults` | Predictor coefficient digits; defaults, dimensions and command behavior match. |
| `cheby-ecliptic-tdb` | Predictor coefficient digits. |
| `polyco-prefix` | Coefficient digits in both literal-prefix files; warnings/prefix behavior match. AF's raw RMS gate remains separate. |

Absent DDH derived inputs and an all-deleted report span remain explicit unwritten-state boundaries. Every nonfinite/solver branch, full debug/allocation transcripts, telescope XYZ, fitted shapelets, WAVE scale 2, Jupiter-mass alternatives and all derived family branches are not dynamically qualified by AH. Source transfer does not imply their numerical acceptance.

The inherited 0091 ledger remains: P's 154 records including 32 SIMUL; M2's four raw-mode order failures; 0049's four discrete boundaries and three unconstrained interpolation cases; AE red's six raw modes; chromatic-log's sixteen errors and one multiplier; ELL1H's nineteen whitened-data differences; single-pulsar quadrupole/GW-single/memory/cosmic-string 770/750/448/468 failures; DDH's 401/433 current/historical fidelity failures and two shared-solve raw-order differences; two first-pass SWGP unwritten-state boundaries; and AF's seven raw polyco log10RMS failures. The historical 1 ns ideal pass and paired clock rule remain intact. Decisions 0077 and 0092 diagnose particular numerical differences without changing production precision or accepting these gates.

The earlier parallel library test failure is also retained. Central decoded both complete recorded assertion values: only `/cycles/0/postfit/warnings`, `/cycles/1/prefit/warnings` and `/cycles/1/postfit/warnings` differ; all other 6,716 scalar values and container structure/order match. This localizes that recorded failure to diagnostic collection. It neither reproduces the interleaving nor establishes general concurrency safety. AK must address it within its existing process-state/diagnostics contract; serial retries do not close that obligation.

## Major-stage checkpoint and ownership

The new usable capability is the native command/report workflow atop general timing, fitting, predictors and analysis. Enabled CALCEPH ephemeris/time processing and BLAS/LAPACK remain the next missing functional block. AI's prepared scope remains appropriate, followed by AJ's installed Rust/core-toolkit and standalone predictor C interfaces, AK's source parallel/concurrency/performance work, and AL's integrated qualification on actual macOS arm64 and Linux x86_64. Full migration remains incomplete; M0/M1a/M1b retain only their scoped acceptance and M2 remains unaccepted.

No new consequential user choice is needed. Source fixes fit the existing 0053 authorization; numerical findings remain open under 0074; diagnostics/concurrency already have an assigned stage. The later clap discussion requested assessment only and made no parser-refactor decision. Retaining AH's implementation for this handover does not prohibit a future measured simplification or claim that clap cannot express compatible behavior. The adopted 0090 weighted-spectrum warnings, 0084 interfaces, 0086 platforms and 0087 versioned DDH rule remain unchanged.

AH ownership is closed. Its chat must make no further Rust or validation edits after closure. AI becomes the sole Rust writer, owning `rust/` and new `validation/native_backends/`. Central retains design, numerical policy, cross-stage coordination and milestone acceptance. Other work and all accepted/historical evidence remain preserved; generated output remains ignored under 0039.

AI follows 0081 and handoff 35 against this exact AH baseline. Reconcile the delivered `core_cli::run`, ordered acceleration selectors, source default 1, existing JSON default 0, stateful report consumers, diagnostics and single-session locking. Preserve QR-specific augmented chi-square/rescaling and truthful algorithm metadata without inventing SVD outputs. Reuse the accepted current-pulsar CALCEPH correction and ordinary-long-double conversion boundary. Include 0093's demonstrated QR optional-output matrix/RHS correction and selected-provider Rust/native execution; the earlier syntax/Accelerate witnesses do not themselves qualify complete backends. Both disabled and enabled profiles, real TT-TDB data, failures followed by valid work, and the full open ledger remain required. No new tolerance or platform acceptance is granted by release.

## Review artifacts

Authored review scripts are under `decisions/evidence/core-output/`; copied raw evidence, archives, logs and executed binaries are ignored under `build/handover-20261027/`. The original build directory is `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-ah-committed-review-yddd4nmm`; Cargo caches, rlibs and the extracted workspace remain there. The review records its initial metadata-status stop, the actual help-output finding and a reviewer-only `/var` versus `/private/var` path-containment correction. No production change resulted from those bookkeeping corrections.

| Artifact | SHA-256 |
| --- | --- |
| Corrected AH delivery index | `8ab38dd6a78ecdad4b33d2b33ebc4785670067c5131231af1b08aa5944bda498` |
| Central handover receipt | `2509ae9a50b1df98f824dba7f9852ad94029a96d18c90f2eac6362f4d31bd167` |
| Fresh complete release matrix | `7ca7bbe2aa8263e40856f16ca49d5c786c43f942fa3801aa99c4cb34fac70c40` |
| Fresh UT1 receipt | `3efafca90fd7fdcc2c68c41e4449bb0c532f522383cb819ef3cec72806c9284b` |
| Retained parallel-warning diagnosis | `8c9de7930bbb13627e951f8a180c0107720d4d28c9e1cbf0272442579513766b` |
| Central preservation receipt | `57aaee103587afde0b5760b692852bdc08aeeaeb7aebabc796ef24c459b35332` |

Executed host: macOS 26.7 / Darwin 25.6 arm64, rustc 1.97.1 and GCC/GFortran 16.2.0. Native timing binary128 has 113 significant bits and production timing remains Rug128. These are provenance, not comparison allowances.
