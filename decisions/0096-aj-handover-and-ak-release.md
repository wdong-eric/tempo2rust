# AJ interface handover and AK parallel-execution release

Central decision, 27 October 2026. **Approve AJ's implementation handover under 0074, close AJ's Rust ownership, and release Assignment AK for observation-parallel execution, the supported concurrency contract and representative performance measurements under 0085.** The 0082 checkpoint is complete. No new consequential user choice is required. This handover accepts neither the inherited numerical gates nor the full migration.

Reviewed production: `3e41497d5a86e627b0ce994d4882e4b2e77d3bf7`; qualification: `31c729079da44afe7155e42707edeac20d66bd2b`; delivery report: `f4b73f87102858a9fc1a7204b91464b6e1c65b29`. The last two commits change no production implementation. The main tree was clean before central's handover documents. Canonical TEMPO2 and the independent reference remain clean at unchanged pins `fda6ad4a9753ad7dc5d291406f2f310a9161423f` and `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`.

## Delivered capability and reviewed evidence

AJ supplies installed, buildable Rust core/toolkit source with the canonical public owners and workflows, the standalone predictor C interface in static/shared libraries, source-compatible headers and getPeriod-rust. Its public responsibility map covers all 93 scoped library translation units. Previously private resource, coordinate, time-ephemeris and stream operations now have external Rust routes. The predictor ABI exposes the native structs, fixed storage, ordinary long double, borrowed pointers, callbacks, streams and source globals. Its C layer handles representation, allocation, parsing/formatting and dispatch; canonical Rust leaves supply the scientific arithmetic.

The macOS arm64 profile uses ordinary long double with 53 significant bits and size/alignment 8/8. Timing remains Rug128. Installed notice/source completeness, source error/unsupported dispositions, header bytes and all 64 standalone function/data symbols are checked. Linux's wider ordinary-long-double ABI is implemented in the adapter's selected branch but has not been executed or qualified. API/source/unsafe/departure details remain in `rust/LIBRARY_INTERFACES_API.md`, `rust/LIBRARY_INTERFACES_SOURCE_MAP.md` and `rust/LIBRARY_INTERFACES_REPORT.md`.

Central independently completed:

- A fresh default-release install from an exact Git archive of production `3e41497`, with 577 committed Rust-tree files verified. Required authored notices, Cargo/build metadata and JPL source manifest are present and byte-identical. No earlier project target directory or installed predictor binary was reused; the existing offline dependency/native build cache was retained.
- Separately compiled C11/C++17 clients with static and shared linkage against that fresh prefix. All four combinations pass 44 ABI-layout rows, 68 value/state rows, 121 storage rows, 48 low-count rows and 26 pipe/locale/class rows each; each storage run also matches 270 phase diagnostic lines. Actual loaded artifacts, all 64 symbols, file exchange, unsupported subprocess exits, getPeriod behavior and failure/recovery pass. Twelve comparator corruptions are rejected without changing the AF gates.
- A separate release Cargo package using only the fresh installed tempo2-rust source dependency: all 30 supplied executables compile and 29 selected workflow invocations pass. This covers source resources, two-pulsar timing/fitting/output, recovery, toolkit/RNG, spectrum/GW and predictors. Other entry points remain compilation coverage. The thirteen frozen verification files equal committed `31c7290`.
- A read-only delivered-evidence audit: **2,467 unique files, zero identity or source-binding mismatches**, including the owner's 2,444-file audit, receipts and loaded-artifact records. All 577 production files and thirteen validation files match their Git blobs; both delivered 242-file installed source trees match the production commit. Report counts and named test logs were checked.

The owner's final campaigns additionally cover all eight C/C++ language/linkage/debug/release combinations, external Cargo default debug/release and combined CALCEPH/LAPACK release (29/29/65 invocations), and **59 installed-backend paired cases / 41,961 comparisons with zero failures** using the unchanged AI native oracle/comparator. Both serial crate profiles pass 91 tests with 36 ignored; formatting and all-target Clippy pass. Four clients pass C-boundary ASan/UBSan; Rust/MPFR/provider internals were not instrumented. Central checked these artifacts and logs without repeating unaffected whole suites. Retained upstream whitespace in byte-preserved/source-derived headers is documented, not relabeled as a clean unrestricted whitespace check.

## Correction closure and historical limits

The supported findings in `handoffs/36a-library-interface-review-corrections.md` are closed by committed implementation and the final independent replay: low-count C set reads retain complete prefixes and caller initialization opportunities; generic readers do not require a seekable stream; private scalar conversion preserves values and caller locale in both directions; required notices are installed. Existing owned Rust file constructors retain their 2..32 bound. getPeriod separately checks the backing its file-only evaluation actually needs.

The original bare-T1-pipe apparent success is not a compatibility oracle. AJ identified, and central confirmed, that the failed Cheby trial consumes the marker before an unsuccessful rewind leaves short lines for unchecked fixed-width conversion. That initial interpretation is withdrawn in committed 36a; original scripts/results remain historical diagnostics. Final T1 witnesses use defined direct and generic histories, check all assigned header/coefficient values and separately observe the failed-trial cursor without invoking undefined conversions. No accidental native success is reproduced as a requirement.

Other documented 0053 corrections cover active-union/plan initialization and cleanup, malformed storage/scan/allocation boundaries, self-copy/insert, native formatting type mismatches and previously approved T1 formatting/parsing fixes. Source/cause/fix and unsafe lifetime/extent comments are adjacent to the code. Numerical engines, precision, gates and source-facing locale conventions remain unchanged. Tested locale/nonfinite neighbors are bounded evidence, not all-locale or arbitrary-pointer qualification. Provisional owner/central results retain their original identities.

## Major-stage checkpoint and next contract

The new usable capability is external consumption of the transferred engine and its standalone predictor interface. The next missing operational capability is source observation-parallel execution plus a tested concurrency contract and measured performance. AK's prepared 0085 scope remains appropriate; AL follows for integrated release qualification on actual macOS arm64 and Linux x86_64.

No new user choice is needed. The corrected interface histories apply 0084/0053, and the invalid T1 oracle changes no scientific convention. Interface scope, selected platforms, 0087 paired clocks, 0090 source weighting with warnings, Rug128 and all numerical gates remain settled. Source process-lifetime state is still preserved; neither exposed Rust owners nor a successful C install promises automatic context isolation or concurrent C use. Performance has no newly invented threshold or precision concession.

**AJ ownership is closed. AK becomes the sole Rust writer**, owning `rust/` and new `validation/parallel_execution/` from `f4b73f8` plus this central release. AJ must make no further Rust or validation edits after closure. Central retains shared design, numerical policy, cross-stage coordination and milestone acceptance. Canonical TEMPO2, accepted runners, all historical evidence and other chats' changes remain preserved. Generated artifacts stay ignored under 0039.

AK follows 0085 and handoff 37. Reconcile the existing process-session/borrowed-owner map with AJ's actual public APIs and native C globals. Reuse the canonical observation calculation, preserve outer pulsar order and per-observation expressions, and define shared-effect/error publication without a second scientific engine. Address the recorded AH diagnostic-collection failure with complete supported workflow tests, including collection and controls. Prove actual workers and deterministic Rust serial/parallel scientific outputs on affected paths; retain a separate defined native comparison and original/corrected evidence for small source races or build defects. Execute representative measured workloads, supported concurrent callers, cleanup/recovery and affected external consumers. A mutex, build flag or serial retry does not close those obligations. Consequential new ownership/scientific/public choices return to central with a recommendation under 0082.

AL remains prepared and unreleased. It still owes actual Linux provider implementation and target execution, the 64-bit-significand predictor ABI/conversion, compiler contraction measurements, integrated installed consumers, both clock versions and every inherited numerical gate. macOS serial interface results do not discharge those obligations.

## Unresolved acceptance ledger

M0, M1a and M1b retain their scoped acceptances; M2 remains unaccepted. The complete 0095 ledger carries forward:

- P's 154 records including 32 SIMUL; M2's four raw SVD-order failures; 0049's four discrete boundaries and three unconstrained interpolation cases.
- AE red's six raw modes; chromatic-log's sixteen errors and one multiplier; ELL1H's nineteen whitened-data differences; single-pulsar quadrupole/GW-single/memory/cosmic-string 770/750/448/468 failures.
- DDH's 401/433 current/historical fidelity failures and two shared-solve raw-order differences; two first-pass SWGP unwritten-state boundaries; AF's seven raw polyco log10RMS failures. The versioned historical 1 ns ideal passes remain separate from fidelity acceptance.
- AH's twelve groups: `pre-list-bary-unwritten` (unproduced UT1/missing PAR); `pre-list-tempo1` and `pre-list-terrestrial` (position/velocity signed zeros); `dm-grid` (ill-conditioned RMS/PAR digits); `red-noise` (F2 digit); `ell1h-h4` (offset/RMS); `waves` and `wave-dm-scale` (unfitted errors/missing PAR); `global` (F2 digits); `cheby-odd-defaults`, `cheby-ecliptic-tdb` and `polyco-prefix` (coefficient digits). AH's absent DDH derived inputs and all-deleted span boundaries remain.
- The recorded parallel failure in three warning vectors remains AK's obligation; the other 6,716 scalars matching in that observation and serial success do not prove concurrency safety.

AJ now has the bounded macOS installed-interface evidence above. Linux/provider ABI implementation and execution, concurrency, performance, broader scientific/resource coverage and distribution remain unfinished AK/AL obligations. Functional handover closes no numerical item merely because a successor can proceed.

## Review artifacts

Authored executed central recipes are in `decisions/evidence/library-interfaces/`; its README distinguishes the original probes, withdrawn T1 interpretation, fresh install/replay and delivered-evidence audit. Byte-preserving copies of 795 selected files (114,885,002 bytes) are ignored under `build/handover-20261027/`. Original receipt paths remain unchanged. Cargo caches and most external Rust executables remain at `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-aj-committed-review-a4ra8qic`; the copied bundle includes the installed libraries/client/source, complete C campaign and two Rust executables. No generated JSON, binary, archive or large comparison output is committed.

| Artifact | SHA-256 |
| --- | --- |
| Fresh install completion | `b54d45c387b05928f7c0baecaa67ff35dd586f0572c5c4ed908017c3dc86aef5` |
| Central replay completion | `1a0c044ae20e127adb0cc64743fd9f232ba6c8cf04d3beaaeca4a6479abba63d` |
| Fresh C/C++ campaign | `79d6b35b95e65d3f0c4d7d81bd499e3f9cb0af7287f30369b6d79eb9103e98e8` |
| Fresh external Cargo campaign | `d0334e1527da1b52e155d8c4b6824d424937dcfc28bf185eb0695c68e4f509c0` |
| Central delivered-evidence audit | `e33142ab99fd57b0e6fd5959203334e02eddb78bc44b0c252dd52d911a28785e` |
| Owner final audit | `c121d391099aecee9cbdba32cf3c8aa252fdbf2016fa51e63f096e3c7d36cf35` |
| Central preservation | `aa9744e3b43753acd4d935826bb142d4497c86451e96228fbd4ee44281d4d317` |

The delivery report retains the final debug/release library/client hashes, native corrected-source identities, provider/input hashes and all nine final receipts. The fresh central library is a separately built artifact, not relabeled as the owner's binary.
