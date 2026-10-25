# Retained SOFA dependency disposition

Central decision, 26 October 2026. **The existing 36-routine retained SOFA closure covers the numerical dependency needs of the complete pinned TEMPO2 core. The other 85 bundled routines are not reached by that core.** This resolves their core-dependency classification under 0013/0062; it is not a new scientific implementation, a full SOFA-library validation, or a migration-acceptance claim. AE remains the sole active Rust writer, and the interface choice raised in 0082 remains pending.

## Source and implementation disposition

The pinned three-library build contains 93 root translation units. Central expanded their membership using the existing inventory helpers and GNU make, then inspected all 137 authored root C/C++ source/header files, the generated enum translation unit and the 121 bundled SOFA units. The only direct caller is `get_obsCoord.C`, with eight entries: `CP`, `POM00`, `RXP`, `SXP`, `PXP`, `C2T00B`, `TRXPV` and `TRXP`. `C2T00A` is declared but never called in the pinned core. This includes both legacy and IAU observer-coordinate paths; a declaration alone is not a missing scientific model branch.

Rust's canonical `get_obs_coord.rs` calls seven of those entries and implements the direct CP operation with a safe slice copy, as already reviewed in 0062/0069. The eight native roots and seven Rust roots reach exactly the same 36 SOFA routines. Those 36 remain byte-identical to the pin, including the two associated notice files recorded in `rust/native/upstream_manifest.json`. No extra or missing `.for` file was found in the retained directory.

The per-file disposition is precise:

- Each of the 36 files in the manifest's `.for` membership is a **retained native numerical dependency**, mapped to its original SOFA routine and the canonical Rust FFI boundary. It does not require another Rust scientific implementation to satisfy the approved dependency policy.
- Each of the other 85 entries in pinned `sofa/Makefile.am` is **not reached by the pinned TEMPO2 core**. Its source identity and dependencies remain listed in the review's 121-row disposition table. It is not a missing TEMPO2 scientific routine, and no new transfer assignment is needed merely because the bundled collection contains it.
- Standalone SOFA export/installation obligations, if required by the eventual library-interface/distribution choice, remain separate packaging work. This core-call-graph disposition neither excludes those exports from a future compatibility guarantee nor claims they are already shipped or tested.

This follows 0013's explicit distinction between complete source accounting and a requirement to rewrite every external numerical routine. The original historical inventory remains intact; this decision supplies the later dependency disposition rather than rewriting its baseline counts. The unused `C2T00A` declaration does not authorize replacing source IAU2000B calculations with IAU2000A.

## Evidence and limits

The read-only reviewer is `decisions/evidence/sofa-dependencies/central-review.py`. It reuses Assignment F's make-expansion helpers without invoking that historical inventory's regeneration. It verifies the pinned graph, current retained-source and manifest identities, current Rust foreign entries, the original 0008 hash and 0080's generated enum evidence. It also inspects immutable local copies of two existing archives. The Rust native archive defines all 36 required SOFA routines and contains exactly their 36 objects plus the Darwin archive symbol index. The A2 core archive's SOFA references occur only in `get_obsCoord.C.o` and match the eight native roots. These are source/linkage observations; neither archive was rebuilt and no new timing numerical run is claimed.

The existing build retains `-frecursive` and the adjacent FFI ownership/layout explanations. Native DATA declarations occur in `nut00b.for` and `s00.for`; this review does not equate automatic-local compilation or a complete call graph with concurrency stress-test evidence. Keep platform/ABI checks, build/runtime dependency checks, installation provenance/notices, concurrency and performance on the release plan. The separate general core/toolkit C-interface choice remains with the user under 0082.

Generated receipts, symbol listings, archive copies and the disposition TSV are preserved locally under `decisions/evidence/sofa-dependencies/build/review-20261026/` and remain ignored under 0039. The original successful review ran in `/private/tmp/tempo2-sofa-review-20261026`; its receipt records the actual input paths, source/compiled identities and command. An initial reviewer attempt counted the Darwin `__.SYMDEF SORTED` index as a source object; the corrected reviewer distinguishes that observed archive metadata and the successful result above is from the corrected run. This was a review-script parsing issue, not a production or scientific failure.

Receipt SHA-256: `fcef7c9d67ed57d2dcb1042d3d047af1db0c7c6239e6daf25d13ffd7b6d39a82`. Disposition TSV SHA-256: `2db964b910609b2b3af7babcaaf1a4f5e99ae3a57bc6169a474bf514b2e16363`.

## Effect on the next stages

AE continues general fitting; AF/AG/AH/AI retain their prepared scope and release conditions. Later references to an unresolved SOFA numerical dependency inventory are satisfied by this reviewed closure; SOFA distribution/interface and operational qualification remain open. Every major functional handover still receives the 0082 reassessment. The unresolved P, M2, 0049 and DDH numerical gates are unchanged, and the full migration remains incomplete.
