# Clock interpolation contraction on the qualified platforms

Central decision, 1 November 2026. Release one bounded AL source-profile
correction in addition to 0109. This does not accept Linux, AL or a scientific
milestone and changes no tolerance, clock resource or numerical precision.

Decision 0027 approved the final multiply/add contraction in
`tabulatedfunction.C:163-164` to match the observed macOS arm64 GCC binary. Its
scope explicitly excluded a claim of cross-platform equivalence. Rust currently
uses that `mul_add` on both selected platforms. The completed Linux run now
provides the missing host evidence.

## Evidence and localization

The frozen a35104a Linux artifact binds native clock executable
`15ebe5ff451fc520775a2060ee886e377a1a6a78b5a94ccf98332f142ed3a217`
to GCC 13.3.0, x86_64, `-O2 -std=gnu++11`, and pinned source
`tabulatedfunction.C` SHA-256
`0c3e90a2594e442c5135ca224f603a6f69e6278fa9f68382bc9e451bfe7b54c4`.
At the `table_internal_duplicate` query immediately below 2:

| Evaluation | Binary64 bits | Value |
| --- | --- | --- |
| Recorded Linux native | `4034000000000000` | `20.0` |
| Recorded Rust on Linux | `4033ffffffffffff` | `19.999999999999996` |
| Separate binary64 multiplication/addition witness | `4034000000000000` | `20.0` |
| One-rounding product-plus-addition witness | `4033ffffffffffff` | `19.999999999999996` |
| Accepted macOS native in 0027 | `4033ffffffffffff` | `19.999999999999996` |

Central independently checked the original Linux fixture, all five query
results, compiler/source identities and captured Rust failure. Separate
operations match all five Linux results; fused arithmetic differs at exactly
this query. The one-rounding result is computed from exact rational binary64
operands and independently agrees with `math.fma`. The existing 0027 evidence
binds the same source to the macOS `fmadd` instruction sequence. This establishes
a source-profile mismatch at the existing explicit Rust contraction. It is not
an inference from a nominal precision or from a relaxed numerical tolerance.

The review uses saved Linux outputs; it does not claim a newly executed Linux
counterfactual, disassembly or corrected Rust program. AL must verify the actual
corrected program. The scope is this one source expression, not fusion policy
for other observer, timing or toolkit functions.

## Released implementation and verification

Retain 0027's fused final multiply/add on the qualified macOS arm64 profile.
Use the source's separate multiplication and addition on the qualified Linux
x86_64 profile. Preserve the preceding subtractions/division, sample selection,
endpoint behavior, nonfinite results and all observable diagnostic/state rules.
Use the existing measured-platform approach already present in
`cholesky_spectrum.rs`; this single call site needs no new generic arithmetic
abstraction, configuration switch, dependency or unsafe block. Keep a nearby
comment identifying `tabulatedfunction.C:163-164`, the two measured compiler
profiles and why their contraction differs. This corrects an overly broad Rust
platform assumption, not a TEMPO2 algorithm bug.

Retain the original failed Linux evidence. Verify the duplicate-knot query and
its exact/neighboring knots through the real table API, then the complete
applicable clock/table comparison with 0107's attribution adapter on both
selected platforms. Keep the original exact bit gates, declared safe-boundary
checks and unresolved native cases. Recheck affected installed clock/timing
workflows in the next integrated candidate; a five-query witness alone is not
full clock or migration acceptance. Report any new differences separately.
AL owns implementation and its scoped commits. Central retains candidate review,
publication/dispatch and milestone acceptance under 0101/0109. All other 0109
work remains active, including the measured cache correction and event/spin
source evidence. The covariance optimization remains deferred.

## Retained review

The exact executed recipe is
`decisions/evidence/release-qualification/central-clock-contraction-review.py`.
It reads the already verified third-run ZIP by exact case-sensitive member name
and verifies each selected member against its manifest. Original output is
`/tmp/tempo2-central-clock-contraction-review-20261101/`, copied byte-for-byte to
ignored `decisions/evidence/release-qualification/build/clock-contraction-review-20261101/`.

- Review receipt: `1df02b2601fc8574959a16e469446411d68f0cdccb24b87e6eea298ba483e89f`.
- Executed recipe: `a43349ca41787b4accaefac3577a935fc18c801c59337094e181fa281f6af91b`.
- Existing accepted macOS evidence: `96637acc9e67ed84997921bc2055896a17193a6fd0a7bc47cac817550e2fcf6b`.

Generated review JSON remains ignored. The existing 0027 evidence and historical
runners are unchanged.
