# Proposed workload-specific release budgets

Central baseline proposal, 4 November 2026. **Choice resolved:** the user adopted
the stricter **2× for all three runtime limits** alternative, with the same RSS
caps and protocol, in [0148](0148-user-adopted-release-performance-budgets.md).
The recommended limits below record the original proposal, not the adopted
acceptance criteria. Scientific precision, conventions and tolerances are
unchanged; the qualified release still needs both scientific and performance
acceptance.

## Recommended limits

Require all three on **each** target, against the matching native reference on
the same host with the same inputs/resources and declared compiler/provider
profile. Ratios use median runtime; RSS limits use the largest process peak
among the retained uninstrumented samples, in MiB (2²⁰ bytes).

| Fixed workload | Proposed Rust/native median runtime limit | Proposed maximum peak RSS |
| --- | ---: | ---: |
| Loaded 8×4 C predictor, 100,000 phase/frequency pairs | **10×** | **16 MiB** |
| CLI Chebyshev generation | **4×** | **128 MiB** |
| CLI two-pulsar correlated fit | **5×** | **128 MiB** |

This asks for substantial remaining predictor progress on both hosts and about
another twofold fit improvement on macOS, while preserving demonstrated memory
gains. It does not promise native parity for every workload or allow a scientific
failure to pass because it is fast. A tighter alternative is 2× for all three
runtime limits with the same memory limits; that is a materially larger project.

## Measured baseline, before the new 0131 optimization

| Platform / workload | Native runtime | Latest measured Rust runtime | Ratio | Rust median RSS |
| --- | ---: | ---: | ---: | ---: |
| macOS, loaded C predictor (0128) | 0.004528 s | 1.078071 s | 238.09× | 2.016 MiB |
| Linux, loaded C predictor (0124 transport only) | 0.036750 s | 2.627960 s | 71.51× | 4.391 MiB |
| macOS, Cheby generation (0123) | 0.130855 s | 0.403618 s | 3.08× | 70.500 MiB |
| Linux, Cheby generation | 0.214602 s | 0.364997 s | 1.70× | 49.605 MiB |
| macOS, correlated fit (0123) | 0.040283 s | 0.402634 s | 9.995× | 52.406 MiB |
| Linux, correlated fit | 0.114357 s | 0.214623 s | 1.88× | 50.074 MiB |

The platforms' predictor rows intentionally refer to different accepted
implementation stages; they are not a cross-platform speed comparison. The
Linux run did not execute owner reuse. The corresponding latest measured Linux
ratios cannot be relabeled as 0128 or 0131 results. Median RSS above describes
the old observations; the proposed gate is the maximum sampled process peak.

Sources are [0123](0123-memory-review-and-predictor-performance-release.md),
[0128](0128-predictor-evaluator-review-and-cpu-assessment.md) and the
[completed Linux review](../validation/release/LINUX_41392963741_REVIEW.md).
The macOS predictor RSS was read from the seven bound 100,000-pair records in
`validation/predictor_performance/build/evaluator-reviewed-20261102/measure/receipt.json`;
its maximum also equals 2.016 MiB. These are existing evidence, not new runs.

## Measurement and scientific conditions

Reuse the AN/AO recipes named in the Linux review. Loaded predictor time is the
client's evaluation interval, excluding load/startup. CLI time is the harness's
complete subprocess wall interval; do not substitute GNU time's coarser elapsed
value. Use seven interleaved samples for predictor and five for each CLI workload,
with compilation completed and provider threads constrained to one. Keep every
sample and record native/Rust binary, source, input, resource, toolchain and
host identities. Instrumented allocation counts are separate from runtime/RSS.

The native baseline is the pinned TEMPO2 source/profile with every approved
source correction identified. Original failing native evidence remains. Exact
Rust before/after equivalence and independent adopted scientific gates remain
necessary: current native whitening, polyco and T1-sign failures are not waived.
If samples are unstable or host/resource identities change, diagnose that
condition before judging a threshold; do not discard inconvenient samples.

GUI load/interaction and other toolkit workloads need their own representative
qualification; these three budgets do not silently cover them. The first GUI
slice remains a fixed-parameter viewer until selection/refit/save contracts are
implemented and tested through the same Rust engine.
