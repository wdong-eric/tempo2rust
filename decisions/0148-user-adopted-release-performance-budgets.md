# User-adopted release performance budgets

Approved by the user, 4 November 2026, in response to the explicit numerical
choice in [0132](0132-release-performance-budget-proposal.md):

> Require ≤2× native for all three workloads

This adopts the stricter alternative in that proposal. The proposed peak RSS
caps and measurement protocol remain the same. These limits now govern release
acceptance independently on macOS arm64 and Linux x86_64.

| Fixed workload | Maximum Rust/native median runtime | Maximum sampled process peak RSS |
| --- | ---: | ---: |
| Loaded 8×4 C predictor, 100,000 phase/frequency pairs | 2× | 16 MiB |
| CLI Chebyshev generation | 2× | 128 MiB |
| CLI two-pulsar correlated fit | 2× | 128 MiB |

Use the matched native reference on the same host, unchanged inputs/resources
and declared compiler/provider profile. Loaded-predictor time is the client's
evaluation interval; CLI time is complete subprocess wall time. Preserve seven
interleaved predictor samples and five samples per CLI workload, with compilation
finished and provider threads constrained to one. RSS is the largest retained
uninstrumented process peak, in MiB (2^20 bytes). Observe allocations separately.
Retain every sample and actual binary/source/resource/toolchain identity.

The originally recommended 10×/4×/5× limits are not the acceptance criteria.
This decision closes the pending budget choice. It does not retroactively pass
an old run, promise a speedup or waive any scientific gate. The accepted macOS
temporary-reuse predictor result in 0145 remains 40.246311× native on its fixed
fixture; it does not meet the newly adopted limit. Older CLI and Linux baselines
remain dated observations at their identified implementation stages.

Continue the bounded predictor work released in 0147 and the separate scientific
qualification stream. Review the next exact Linux candidate's measurements
against this decision. Further timing/fitting work must address measured costs
without changing precision, physical assumptions, operation-order contracts,
output conventions or numerical tolerances. Native parity on arbitrary
workloads, GUI latency and other toolkit performance are not implied by these
three fixed-workload limits.
