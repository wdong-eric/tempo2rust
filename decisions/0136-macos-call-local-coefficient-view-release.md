# Release a call-local macOS predictor coefficient view

Central implementation release, 4 November 2026, under [0129](0129-qualified-release-continuation.md)
and the completed [0134 assessment](0134-predictor-recurrence-acceptance-and-reconstruction-assessment.md).

Accept assessment `998cad58615ca79b20c2d1270476fe2ab089334c` and release its
[bounded design](../validation/predictor_performance/C_RECONSTRUCTION_ASSESSMENT.md).
This authorizes implementation and qualification, not an assumed speedup.
AO owns the affected predictor sources, focused witnesses and performance
recipes in its existing isolated clone; preserve all other work.

## Basis and independent review

The accepted recurrence baseline still reconstructs 64 coefficient owners and
two vectors per measured C phase/frequency pair. Central rebound all 21 new
evidence files, 48 external identities, 32 Git source bindings and four prior
manifests, replayed the new and previous trace attribution, and checked exact
30,258-line controls. Of 3,846 sampled stacks, 1,482 include polynomial
construction and 320 include recurrence evaluation. Inclusive percentages
overlap and do not predict the achievable speedup. The two plain/sampled runs
are profile support, not another seven-sample performance campaign.

Standard Rust immutable slices and a generic coefficient accessor are sufficient;
no new dependency, public view hierarchy or duplicated evaluator is needed.
The [Apple arm64 ABI](https://developer.apple.com/documentation/xcode/writing-arm64-code-for-apple-platforms?changes=_1)
defines long double as binary64, consistent with the compiled installed witness.
The [Rust slice safety contract](https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html)
requires initialized, aligned, non-null, single-allocation backing without
mutation for the borrow's lifetime. Existing C caller obligations and the
checked positive C-int shape supply these conditions. Linux's wider native
format does not support this view and keeps owned Rug64 reconstruction.

## Dependency-ordered implementation and gates

1. In `cheby2d.rs` and its private `evaluation.rs`, feed the one existing
   Clenshaw recurrence through a coefficient accessor. Retain loop/order/half
   weights, all rounding/FMA boundaries, public owned methods and Float borrows.
   Keep model normalization/dispersion/frequency expressions shared and intact.
2. In `predictor_ffi.rs`, create a macOS-only native binary64 slice after the
   existing shape/null gates, solely inside synchronous evaluate/model calls.
   Preserve error ordering and Darwin's positive-NaN input convention. End the
   borrow before output publication. Update obsolete adjacent ownership comments
   and the affected API description to match the actual bounded conversion.
   Keep test callbacks, derivatives, construction, parsing and T1 on their
   existing owning paths. No cache, persistent pointer or new concurrency policy.
3. Extend the existing macOS ABI assertions in `native/predictor.c` only as needed
   for size/alignment/format equivalence. Add a C/Rust representation witness
   against the unchanged parts-to-N conversion, including signed zeros,
   subnormal/normal neighbors, maxima, infinities and signed/signaling NaNs.
   No scientific C arithmetic is authorized.
4. Exercise caller-owned arrays and all coefficient positions, shape cases,
   mutation/restoration and replacement at repeated addresses, unchanged backing,
   invalid dimensions/null diagnostics, callback snapshots and derivative alias
   behavior. Reuse all retained direct/C/legacy/public-borrow witnesses,
   static/shared qualifiers and caller-exclusive sanitizer/cleanup checks.
   Inspect the optimized library for the preserved contraction sites and removal
   of the C-path coefficient Vec/N reconstruction.
5. Only after correctness, run the existing interleaved immutable-before/after
   direct and complete-C timing campaign in a centrally cleared quiet window;
   observe allocations separately. Return every sample, source/input/artifact
   identity, output comparison, RSS and portable Linux recipe. Actual Linux
   shared-loop/model testing remains required, including ordinary64 wide range.

No tolerance, native-sign policy, scientific assumption, public precision or
release-performance budget changes. Inherited native discrepancies remain open.
Central review is required before integration and any later publication.

The assessment and its external bindings are retained outside Git at
`validation/predictor_performance/build/reconstruction-reviewed-20261104/`.
Original manifest SHA-256:
`b2c63a2364b04e8e04e4aa9b3d45a77a6edb0c12815f5de4e8bc39ad3228b1c3`.
The central combined review receipt under
`decisions/evidence/release-qualification/build/continuation-next-review-20261104/`
is `9390ff81b81b9bcc2fd010546accad3c66f8c723fcac0ae834877acbafe80c2f`.
