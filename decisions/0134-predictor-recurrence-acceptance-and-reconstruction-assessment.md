# Accept the macOS recurrence slice; assess C reconstruction next

Central review, 4 November 2026, under [0129](0129-qualified-release-continuation.md)
and the bounded [0131 release](0131-macos-predictor-recurrence-release.md).

Accept AO production `e24622d5210fdfc8abe10f9f58a21b7e4a2a1aca`, recipe
`f5924bb10bcd6cd2da7880c52ac6f24f3e29ed76` and delivery
`7a3fb85bc2ab0c80cd5ab3b63786fcae4296ca0e` as the macOS recurrence improvement.
They are integrated by merge `a978215dc9be8189633c3579ef4badacbc3146b1`.
This is bounded acceptance of the change, not full predictor/native equivalence,
Linux qualification or completion of the release performance requirement.

## Independent evidence and limits

Central inspected the single Clenshaw recurrence and private arithmetic adapter.
The macOS path preserves explicit FMA sites, separately rounded doubling and
coefficient additions, and operation-specific NaN/zero signs. Public Float
owners and borrowing remain unchanged. Linux delegates to the existing Rug64
operators without converting through binary64. The installed assembly retains
separate additions and contractions. Timing remains Rug128.

Saved evidence was independently rebound: 1,882 files (210,775,135 bytes), 41
external identities, 1,732 Git source/installed-source bindings, six earlier
manifests and twelve baseline products. The review replayed eight native C/C++
profiles and 24 corruption controls, exact before/after 30,258-line C output,
5,880 edge results, 28 direct finite results, 38 legacy files, live public
borrows, mutation/restoration and sanitizer/concurrency results. All 91 raw
measurement records reproduce the reported medians and allocation counts.
Seventeen focused predictor tests also pass freshly in the integrated worktree.

The same macOS workload takes 1.052714 s before and 0.472748 s after for 100,000
complete C phase/frequency pairs (2.2268 times faster); native takes 0.004160 s.
Direct leaf and model pairs improve by 15.4620 and 6.3346 times respectively.
Complete C allocation requests fall from 242 to 134 per pair; requested bytes
fall from 8,512 to 6,784. Median peak RSS is 2.016 MiB before and 2.062 MiB after.
These are measurements of this fixture, not general performance guarantees.
The inherited native finite-format differences, T1 NaN-sign discrepancy and
polyco log10RMS failures remain open. Linux execution remains pending.

Evidence is retained outside Git at
`validation/predictor_performance/build/recurrence-reviewed-20261104/` and
`decisions/evidence/release-qualification/build/ao-recurrence-central-20261104/`.
The original delivery SHA-256 is
`08591e74c16e2b71e95fb20f5a8430458250d5aaed9c5d49a6421899cf3f98ff`;
central review receipt is
`0071f633e78663ec176c5bc9c4395647cc69cbf6bfefde80293d8fe45465854b`;
retention receipt is
`e137648dd914c14e9d6c9f9dd3bc363f10c2f0466da58423813d226553710b7b`.
The review distinguishes saved-producer evidence from the fresh focused tests.

## Next bounded assessment

Release AO to assess the remaining C evaluation reconstruction cost using the
new immutable installed baseline. AO retains ownership in its isolated clone;
other work must be preserved. This step authorizes profiling, read-only code
analysis, qualification recipes and a proposed design, not production changes.

Measure attributable reconstruction/conversion cost before proposing a borrowed
view of C coefficients. Check established Rust borrowing/slice patterns and
existing validated C adapter boundaries before adding abstractions. Identify
where dimensions, pointer lifetimes and layout are already validated, how every
call sees caller mutations, and how the same canonical recurrence can consume
the data without copying or a second evaluator. Preserve macOS53/Linux64 range,
source ordering, Float-backed public APIs, T1 and all existing C failure behavior.
Do not introduce caches, persistent C borrows or scientific C kernels.

Return the smallest justified design, affected files, safety/scientific proof
obligations, output witnesses, performance measurements and a portable Linux
recipe. Coordinate any timed window before measurement. Central will review
that concrete assessment before releasing implementation. AL's separate Q/Z
work and later combined Linux qualification continue independently.
