# Numerical precision study

Assignment B is delivered and reviewed. This is the historical study brief; [decision 0001](/Users/Eric.Dong/Downloads/tempo2-rust/decisions/0001-reference-and-m1a-contract.md) selects Rug128 for the initial core using its evidence. Read the [shared handoff](/Users/Eric.Dong/Downloads/tempo2-rust/HANDOFF.md) for the current milestone gate. The original assignment below records the question and experiment scope.

## Ownership

Write only within `/Users/Eric.Dong/Downloads/tempo2-rust/validation/precision/`. Own a small isolated experiment, its dependency lockfile and reproduction instructions, comparison results, and `REPORT.md`. Other chats share this workspace; preserve their edits. Do not create or alter the production Rust crate, reference outputs, or central decision table.

This study can run alongside A. Start with exact decimal inputs and independent high-precision calculations. Consume A's reference outputs when available, while keeping independent arithmetic accuracy and TEMPO2 behavioral agreement separate.

## Candidate selection

Assess the current Rust package `qd` associated with [sarah-quinones/qd](https://github.com/sarah-quinones/qd) first. Its documented approach is double-double arithmetic. This can serve the role of an extended scalar without being IEEE binary128 or a C `long double` ABI type. Its suitability depends on the required operations and measured behavior, not its name.

Do not confuse this package with older `barandis/qd` documentation or the C++ QD library. Pin the exact package version tested and inspect that version's source. Cached documentation can resolve to an older release. Check the actual decimal parser, formatting, rounding, remainder, and transcendental APIs rather than assuming the existence of a complete float interface.

The alternatives have different purposes:

| Candidate | Purpose and constraint |
| --- | --- |
| Rust `qd` | First candidate for the production scalar; measure operation coverage and numerical behavior |
| `rug::Float` backed by MPFR | Independent high-precision arithmetic reference; also a possible implementation candidate if warranted by results and packaging cost |
| Rust built-in `f128` | Feasibility check only initially; official documentation still marks it experimental and platform support incomplete |
| The separate `f128` crate backed by GCC quadmath | Low-priority feasibility check; its current repository README advises avoiding it because of incomplete support and performance/linking issues |

Sources checked during planning: [qd documentation](https://docs.rs/qd), [Rug documentation](https://docs.rs/rug/latest/rug/), [built-in f128 documentation](https://doc.rust-lang.org/std/primitive.f128.html), and [f128 crate repository](https://github.com/jkarns275/f128). Refresh facts against the versions actually evaluated. Do not spend the study reviving all alternatives; prototype `qd` and the independent reference first, and reject unsuitable candidates with concrete evidence.

TEMPO2 itself contains an older, inactive `dd_real` alternative in [TKlongdouble.ld.h](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/TKlongdouble.ld.h:13). This is evidence that double-double was contemplated, not proof that the branch works in the pinned build or that the Rust crate has equivalent semantics.

## Work sequence

1. Inventory precision-sensitive operations for M1a and the additional operations needed for M1b and early binary models. Start from the original parsing, residual, time-conversion, and binary routines. Distinguish operations already performed in binary64 upstream from operations requiring extended precision.
2. Verify candidate build support and features on the actual macOS arm64 host. Check Linux x86_64 when an execution environment is available. Report compile-only and runtime evidence separately; unavailable targets do not prevent completion of the local study.
3. Implement a small executable comparison using decimal strings as the source of truth. Use a high-precision reference such as MPFR at 256 bits, with a higher-precision check where needed to establish that the reference error is negligible. Keep that reference dependency confined to the experiment unless the central chat later selects it for production.
4. Exercise the contracts below. Measure error in the relevant units and record package/toolchain versions. Check the implementation of any trait method relied on by the experiment; the existence of a trait implementation alone is not evidence that a conversion works.
5. Measure representative scalar workload cost after correctness checks, including parsing and serialization where relevant. Report operations and data sizes; do not infer whole-engine speed from a scalar benchmark.
6. Recommend a single production representation and minimal API needs. If the preferred candidate lacks required functionality, report the gap and assess an established alternative before proposing local mathematical implementations.

## Required numerical cases

- Parse and serialize representative long decimal MJD values directly, including the precision-sensitive epochs in `test1.tim` and `test2.tim`. Test exponent forms required by the selected PAR/TIM subset. Malformed and non-finite inputs must have explicit outcomes.
- Subtract nearby large epochs and add positive and negative delays of 1 ns and 4 ns. Show the recovered differences in seconds. Include day-boundary cases.
- Evaluate the source-matched spin phase terms for a millisecond pulsar over long time spans. Assess absolute phase, fractional phase, pulse counting, and conversion to residual seconds separately. Preserve the original rounding/tie behavior; include values near pulse boundaries.
- Test each needed arithmetic, rounding, remainder, and transcendental operation over its relevant domain. M1a passing basic arithmetic does not establish coverage for binary models that use extended trigonometric functions.
- Exercise serialization followed by parsing and the full comparison path. No intermediate conversion to binary64 may erase the low-order information being tested.
- Compare against A's pinned reference when available. Explain differences caused by reference precision, rounding, constants, or conventions separately from candidate defects.

Write down the error budget and units before accepting results. The existing 4 ns regression is a useful timing contract, not a universal tolerance for every quantity. A double-double value also cannot be passed to a foreign `long double` or `__float128` parameter by assuming identical memory layout.

## Deliverables and decision boundary

Deliver the runnable experiment and lockfile, an operation-support table, observed error and timing results, platform/build notes, and `REPORT.md` using the shared report format. Describe any parser/formatter or math-function gap that would become project maintenance work. Recommend either an established scalar backend or a justified epoch/phase representation, including the smallest interface the initial core needs.

The central chat selects the backend and representation after reviewing this report and A's reference evidence. Do not add a production backend selector, generic numeric hierarchy, or unreviewed precision downgrade. If no candidate meets the initial contracts, a reproducible explanation and bounded next option are a valid study result; M0 remains open.
