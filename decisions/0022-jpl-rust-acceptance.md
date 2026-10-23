# JPL Rust transfer acceptance

Central decision, 23 October 2026. **Accept Assignment I's bounded JPL transfer at delivery `902fbba`, with executed implementation `27c83840295fd0f1309cf54b3477d033cf0d4896`, for the declared macOS arm64 profile. Close I's Rust ownership window.** Decisions 0017 and 0019 govern this acceptance. M2 remains unaccepted, and the complete TEMPO2 transfer remains unfinished.

## Evidence and final correction

Central reviewed the source correspondence, owned stdio boundary, removal of retained JPL C, comparison projection and final [report](/Users/Eric.Dong/Downloads/tempo2rust/rust/JPL_REPORT.md). All **127 executed-input hashes** and **233 protected-file hashes** match the delivered manifests. Canonical TEMPO2 remains clean at `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; the independent A2 repository remains clean at `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`.

The last review found that a legacy header could reach a comparison of source-unwritten extended IPT fields. Rust's zero-initialized storage had made that branch silently clear the rows and return a handle. Commit `27c8384` retains the source's first comparison and rejects with `unwritten_extended_header` before the second comparison consumes unread operands. This implements 0017's operation-local undefined-state boundary, without assigning a numerical result to the undefined C branch.

Central reran its independent Rust-only reproducer against the corrected source. The valid input (`fc0b68c8cf258557124b3797e0642f3275cfbb1c7d33340be5b2b435737b9e53`) still returns version 405 and 751 coefficients. The malformed input (`156d0e0ebbfbf5d29bae06a8080e4bd29a851ea28f5e78a1fa373221ee8ae073`) now returns the required boundary. The invalid native comparison was not executed. I's 12 additional controls cover normal/malformed pre-430 and ncon400 headers in both byte orders and valid small/large extended headers. Their Rust observations agree between debug and release; all four malformed cases stop after two header reads and before allocation.

[Central acceptance checks](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/jpl-rust/acceptance-checks.json) pin the reviewed report, evidence, current source hashes, independent before/after observations and profile comparisons. The focused [reproducer](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/jpl-rust/legacy-header-review.py) compiles copied Rust modules in temporary storage only. Central previously ran the seven comparator test methods independently; the final correction does not alter that comparator.

## Accepted results

Both profiles account for all **55 H scenarios and 1,443 original events**: 1,439 events compare exactly, one reaches the approved title boundary, and three remain explicitly unreachable. There are 43,310 finite, 96 nonfinite-class and 17,098 discrete comparisons per profile; maximum finite difference is zero. All 576 independent dyadic polynomial/derivative components agree exactly. H's ten isolated dispositions, eight supplemental title cases, 32 optional-demand cases, ownership checks and the final header controls pass their declared gates.

The recorded full verification passes 17 Rust library tests, seven behavior tests and one reference-contract test per profile, with the three ignored probe/ownership entry points executed explicitly. Formatting, Clippy, fresh-output/comparator controls and M1 regressions pass. M1a and M1b budgets are unchanged; M1b's maximum residual discrepancy is approximately `1.13687e-13 s`, and its 4 ns response error is `1.4168753360457828e-13 s`, below `1e-12 s`.

The M2 verifier still returns **1**, with exactly the protected four diagnostics: raw retained-mode order and raw singular-value index zero for `dependent_columns`, in debug and release. The outer I verifier recognizes only this exact existing failure set; it does not accept M2 or suppress new failures. Decision 0012 remains binding.

## Preservation, ownership and limits

All 12 implemented JPL functions and the Sun-view macro have Rust counterparts. Source cache-before-read behavior, partial bytes, retry behavior, defined unusual demand flags and the selected call order/units remain intact. Earlier corrected-cache behavior is removed from active implementation. Same-open-file hashing, rewind and ownership transfer avoid a path reopen. The libc stdio boundary remains necessary for the observed buffering semantics. Seven production unsafe sites remain: five stdio sites and two retained SOFA sites, each with adjacent rationale and invariants; the JPL logic itself is safe Rust. No retained JPL C entry points remain in the newly verified main binaries.

This accepts the reviewed host observations and safe boundaries, not portable meaning for undefined C accesses. The explicit title, unwritten-header, buffer, pointer and conversion dispositions remain documented adapters. High-level timing finite/error guards and other core behavior differences remain open where outside I's scope. Linux, Windows, 32-bit hosts, big-endian hardware, alternate libc, exhaustive malformed inputs, allocation exhaustion and concurrency stress remain unverified. The deliberately retained cache defect remains for the post-transfer correction phase.

I's delivered evidence is now read-only unless central issues a specific correction. J's clock/table reference is accepted by 0021, and K continues its separate parameter/PAR reference work. A new Rust owner requires a separate release; this decision does not itself start the clock/table implementation.
