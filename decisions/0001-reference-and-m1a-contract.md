# Reference configuration and initial Rust contract

Central design decision, 19 October 2026, originally recorded after reviewing Assignment A at `5289797` and Assignment B. The decisions below settle the initial design. **M0 is now accepted and C released by [decision 0002](/Users/Eric.Dong/Downloads/tempo2-rust/decisions/0002-m0-acceptance-and-c-release.md)** after review of A2 at `7795d53`; that decision pins the corrected evidence and acceptance scope.

## Evidence reviewed

- [A report](/Users/Eric.Dong/Downloads/tempo2-rust/validation/reference/REPORT.md), manifests, comparison code, initializer reproducer, stage logs, and diagnostic test results. All six fixture output hashes and every artifact hash listed in A's manifest matched the files reviewed.
- [B report](/Users/Eric.Dong/Downloads/tempo2-rust/validation/precision/REPORT.md), source, operation audit, numerical TSV, dependency versions, and reference snapshot provenance. All 28 files listed in B's artifact manifest matched. Recomputing all 1,460 saved numerical verdicts with Decimal arithmetic reproduced the recorded decisions: 516 Rug128 passes, 324 high-precision convergence passes, 7 independent polynomial-identity passes, 92 MPFR113 replay passes, 425 assisted-qd passes and 16 failures, and 4 binary64-control passes and 76 failures.
- The B report reviewed has SHA-256 `fb6d91a8f25d9482195c70eabebdcbb8c4d4243bd270128f0e70a2f89ebbee5e`. Its 23 phase-replay observation/stage records include 16 barycentric records and 7 provisional terrestrial records. Replaying phase from exported BBAT does not validate the terrestrial correction chain.

This review checked saved evidence and relevant source; it did not rebuild either experiment or rerun timing calculations. Both studies executed only on macOS arm64.

## Reference configuration adopted

Adopt A's documented GCC 16.2.0 direct build with the existing TEMPO2 binary128 abstraction as the initial executable reference configuration on this host. Its measured significand is 113 bits. Keep the exact compiler options, linked dependencies, runtime hashes, and documented instrumentation in its provenance. This adoption does not assert that upstream autotools or the complete upstream test suite passes.

The source baseline stays `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. A2 produced a separately identified corrected reference from that baseline plus one recorded initialization patch, accepted in decision 0002. Its identity includes the base commit, correction hash, instrumentation hash, and build metadata. Preserve the uncorrected evidence through A's existing Git history. The canonical TEMPO2 checkout stays unchanged.

## Initialization correction approved

Approve setting `psr->ne_sw_ifuncN = 0` in `initialiseOne`, next to the analogous interpolation counts. This restores the empty-collection invariant before parsing. It is supported by three independent observations:

1. [initialiseOne](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/initialise.C:119) initializes adjacent counts but omits this one.
2. [The parser](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/readParfile.C:1226) uses the count as the next `_NE_SW` array index and increments it.
3. [The delay calculation](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/dm_delays.C:320) treats a positive count as evidence of interpolation data. A's poisoned-storage reproducer and isolated correction demonstrate the failure and its removal.

Apply the correction at initialization, not after parsing, so supplied solar-wind samples remain active. Retain IPM, NE_SW, SWM, and existing delay formulas. Do not substitute zero solar-wind density, disable IPM, or blanket-zero the pulsar structure to conceal the omission. A2 implemented the correction in its disposable reference source, recorded the source patch separately from instrumentation, and supplied both empty-state and supplied-samples regressions.

## Remaining ideal-fixture failure

Keep the DDH ideal fixture as a failing upstream regression against the original 1 ns threshold. A2 records a first failing residual of approximately 1.13759 ns and a maximum absolute residual of 4.59896 ns; 120 of 284 TOAs fail. The fixture is [test3.par](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/tests/test_data/test3.par:13), which uses DDH, DE414, and a broader model than the initial isolated-pulsar path. The assertion fails before fitting.

The cause remains unknown; the fixture's binary model does not establish that the binary calculation is at fault. Record this as an open broader-coverage issue. It does not by itself reject the independently checked barycentric fixtures or require a looser Rust agreement tolerance. A2 preserved the failing TOAs and stage values and completed the bounded localization recorded in decision 0002. Complete causal diagnosis is required before claiming validation of that broader path, not before beginning the declared M1a path. If evidence implicates a component shared with the selected fixtures, reconsider their acceptance explicitly.

The bundled meteorology fallback is a separate issue: all minimal terrestrial epochs lie outside the tiny bundled weather tables. Those outputs can become compatibility fixtures only if the standard-pressure and zero-wet-delay behavior is declared explicitly. They are not validated historical meteorological observations. The eventual production policy for missing meteorology remains a separate M1b decision; do not infer permission for a silent fallback.

## Numerical representation selected

Use **Rug 1.30.0 `Float` with a fixed 128-bit significand**, initially with the dependency versions pinned by B. Use round-to-nearest arithmetic and explicit construction at that precision. This is an MPFR precision policy, not IEEE binary128 or a foreign `long double` representation.

Use it directly for precision-sensitive decimal model values, MJD epochs, and phase arithmetic. Preserve TEMPO2's split integer/fractional spin expression and its original binary64 constants and operations at identified boundaries. Keep complete pulse counts as checked signed 64-bit integers; an out-of-range result is an error. Pulse rounding follows the source's sign-dependent half addition and truncation, separately from MPFR arithmetic rounding. Signed remainder must retain the source's truncating-quotient semantics.

Parse full finite decimal tokens, with explicit E/e/D/d exponent handling, and preserve original tokens. Serialize with sufficient round-trip precision. A minimal domain type may enforce the fixed-precision invariant; a backend hierarchy, public precision switch, custom decimal arithmetic, and custom transcendental functions are unnecessary.

Do not select qd 0.8.0 for the initial core. Its supplied I/O and trigonometric support is incomplete, and B reproduced pulse-rounding defects with a one-turn consequence. Successful assisted arithmetic does not remove those defects. Neither native experimental f128 nor the separate quadmath crate is needed in the production dependency path. Rug packaging and Linux execution remain future validation work; scalar timings do not establish whole-engine performance.

## Initial fixture contract adopted

Adopt A's decimal-string transport, explicit field units, stable observation identity, parameter presence/fit flags, and initialized-stage output approach. Separate engine provenance from scientific comparison: compiler, allocation policy, diagnostic verbosity, file-open traces, and harness scenario names must remain truthful for each engine and are not fields that Rust must imitate. Do not fabricate legacy state or perform unused resource reads just to match instrumentation metadata.

The initial accepted conventions for the barycentric fixture family are:

- One isolated pulsar; TCB barycentric `@` epochs; the effective settings resolved from the original test1 files are retained. An omitted UNITS entry in this legacy fixture resolves through the verified EPHVER/default rule, and that resolution is recorded.
- Original input order, all declared TOAs, no sorting, cuts, fit, pulse-tracking directives, or residual-mean subtraction.
- Fractional phase of the first eligible input TOA supplies the reference. The TZR fields in these fixtures do not activate REFPHS_TZR.
- Fixed-parameter `residual_s` is captured after residual formation. The uninitialized no-fit `prefitResidual` field is not an output.
- Barycentric, aligned/4 ns, and long-span fixtures are the initial comparison cases. A2 confirmed their numerical values remain unchanged under the initializer correction; decision 0002 accepts their hashes and comparison projection. Explicit F0 through F3 support is sufficient for C; unsupported supplied higher derivatives must be rejected.

Approve these cross-implementation gates for that family:

| Quantity | Gate |
| --- | --- |
| TOA identity, ordering, original tokens, relevant presence/fit flags, pulse counts | Exact |
| SAT/BAT/BBAT epoch difference | Absolute difference multiplied by 86400, at most `1e-12 s` |
| Fixed-parameter residual | Absolute difference at most `1e-12 s` |
| Full phase | Absolute difference at most `1e-10 turns`, with the residual and exact-count gates enforced independently |
| Parsed extended parameters in the declared subset | Relative difference at most `1e-30`; expected zero remains exact |
| Parsed observing frequency | At most `1e-9 MHz`, preserving upstream binary64 parsing and original token |
| Parsed TOA uncertainty | At most `1e-12 microseconds`, preserving upstream binary64 parsing and original token |

These are engineering comparison budgets for the specified cases, not claims of physical accuracy or universal thresholds. Retain the much tighter analytic checks as independent diagnostics. Terrestrial stage budgets remain proposals pending a separate M1b decision. Comparison adapters must use the supported scientific projection accepted in decision 0002; inactive unsupported terms cannot become silently accepted input features.

A2's schema 2 clarifies the time-scale descriptions: observatory SAT is in the relevant observatory clock, while BAT/BBAT are TCB for these terrestrial fixtures; `@` SAT is already barycentric TCB. Preserve this distinction in public types and output descriptions.

## Minimal M1a library and CLI contract

One crate and one CLI are sufficient. The initial public operations are conceptually:

```rust
load(par_path: &Path, tim_path: &Path) -> Result<TimingInput, TimingError>
evaluate_barycentric(input: &TimingInput) -> Result<TimingResult, TimingError>
```

`TimingInput` owns validated model values, ordered observations, original input identity, and resolved conventions. Its initial supported PAR/TIM subset is the fixture fields plus F0 through F3; C must document the exact keyword and directive table. Recognized astrometric/DM/resource fields that are inactive for `@` are explicitly classified and preserved. Other scientifically active unsupported settings are rejected. Fit flags are preserved but never trigger a fit in this operation.

`TimingResult` owns ordered per-TOA phase, full pulse count, reduced integer phase where needed for comparison, residual seconds, and the applicable input/configuration provenance. Evaluation borrows input without partially mutating it. Scalar precision is an invariant at construction, not a repeatedly revalidated internal mode.

Use `tempo2-rust residuals --par PATH --tim PATH` as the initial command. Successful output is structured JSON on stdout with extended values represented as decimal strings and counts represented losslessly. Errors go to stderr with a nonzero status and file/line/parameter or observation context. Publish results only after calculation and serialization succeed. No output-file option or format selector is required initially.

The supported barycentric path requires no observatory resource context. It may avoid the legacy wrapper's unused ephemeris/EOP file opens; this is an approved operational difference. D will introduce the explicit runtime context at the terrestrial stage that needs it. Preserve source correspondence for the actual calculation instead of duplicating incidental legacy I/O.

## Workspace Git organization

Adopt one primary project repository at `/Users/Eric.Dong/Downloads/tempo2-rust` for the central documents, the Rust crate, and the precision study. Preserve the independent repositories at `tempo2/` and `validation/reference/`; ignore those two directories in the parent repository. Their revisions and artifact identities are recorded, and relevant small golden fixtures/provenance are copied into Rust tests for self-contained M1a checks. Do not flatten, move, or implicitly stage either nested repository.

C may initialize the parent repository and its root `.gitignore` after M0 is accepted, if no parent project repository has since been created. Keep Cargo caches, native build trees, and generated large binaries out. Use explicit staging paths and inspect the dry run; do not commit another chat's in-progress changes. Creating remotes, pushing, and reorganizing the independent reference history are outside this bootstrap. The existing A commit remains intact. The central documents remain local until that bootstrap occurs.

## Gate and next assignment

[A2 Reference correction and regeneration](/Users/Eric.Dong/Downloads/tempo2-rust/handoffs/01a-reference-correction.md) is complete and accepted for M0 at `7795d53`. The three barycentric fixtures are numerically unchanged, so B's backend decision remains supported without another experiment.

[Decision 0002](/Users/Eric.Dong/Downloads/tempo2-rust/decisions/0002-m0-acceptance-and-c-release.md) releases C. The open DDH causal diagnosis, broader model coverage, Linux execution, terrestrial stage budgets, and production meteorology policy remain explicitly scoped future work.
