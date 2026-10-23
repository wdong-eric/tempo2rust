# Accept the bounded binary reference evidence

Central review, 23 October 2026. Accept Assignment G's source-reference deliverable at `95d25c1`, `6027e11` and `f550a6e` under decision 0009. This establishes reference evidence for subsequent implementation contracts; it accepts no Rust binary model, fitting integration, numerical budget or complete branch coverage.

## Evidence and central checks

The [report](/Users/Eric.Dong/Downloads/tempo2rust/validation/binary/REPORT.md) covers 75 isolated cases across all 14 binary model dispatches, including 54 finite integrated cases with raw and mean-subtracted residuals. The source/coordinate inventory identifies 187 named derivative-label pairs and 189 named update-label pairs, plus T2-PTA's catch-all update. Every named callback has a recorded invocation; a callback with undefined source execution has no numerical return golden. The 525 source condition records are lexical navigation, not runtime branch coverage.

The canonical pin is `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; the reused A2 library is at independent revision `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`, SHA-256 `313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5`. A2's initializer correction and observational instrumentation remain explicit differences. The output-manifest SHA-256 is `5e8cfcd0f45485607f0b9902d778420529735378e10fb3553ef6e84aa3a7c2f3`; the coverage SHA-256 is `2a25488a0e4e5a6301d5dd533f9a9b33f4f45a37ff7db71a9778a2aea6090dd2`.

Central inspected the exporter, request generation, source/patch verification and comparison checks. It reran the inventory/schema/case checks and all 11 mutations without rewriting reports, then rebuilt the exporter and reran all cases into a temporary output directory: all 151 JSON/log files matched byte for byte. The input generator, redirected to a temporary directory, reproduced all 233 fixture/inventory files exactly. These are repeatability checks on the recorded macOS arm64 compiler/library profile, not cross-platform or physical-model validation.

Central independently rebuilt the unchanged DDH ideal-test driver in a temporary directory. It returned 1 at the original `1e-9 s` threshold, with the first displayed failure about `1.13759 ns`. The 120/284 out-of-threshold count, index 35 and maximum `4.59895516385995339247551543745087149 ns` remain separately hashed historical A2 localization; central did not newly recompute that full series. This failure is preserved and does not become a future port tolerance.

No tracked G artifact changed during these checks, and neither independent reference repository was changed.

## Contract implications

Future ports must preserve native versus wrapper precision, source expression order, negative-orbit conventions and per-model update coordinates. A shared parameter name is insufficient to choose an update unit: BT/BTJ/T2 add PB increments directly to stored days, whereas several other models divide by 86400. Likewise, BT's hardwired zero EDOT in the delay remains compatible behavior even though derivative/update routes exist. Source parameter mutations, conditional defaults, warnings and unconditional or additive error updates are part of the evidence.

Binary residual integration must preserve the actual `formResiduals` dispatch condition, original-order phase reference and signs: add `torb` to spin time/fractional day and set `pet = bbat - torb/86400`. Geometry-dependent models need the actual source frame/state; the current resource-free `@` adapter cannot silently supply zero Earth vectors for them. The report's explicit native and wrapper outputs and both mean settings remain distinct comparison targets.

Defined NaNs and signed infinities are source results. Uninitialized locals, invalid companion indexing and later nonfinite-to-integer conversion are separate undefined-source boundaries; withheld results must never become fabricated finite goldens. A source process exit is not a successful timing solution. Preserve these classifications and record any fallible-library adapter.

[BRANCHES.md](/Users/Eric.Dong/Downloads/tempo2rust/validation/binary/BRANCHES.md) remains the open coverage list: higher FB/jump/companion indices, untested links and parser branches, other frames/time scales, singular inputs, binary-fitting integration and other platforms are not accepted here. A next implementation contract must name its exact models/branches, ownership, source-to-Rust numeric boundaries, fixed-input and integrated gates, and regression responsibilities before dispatch. E retains Rust ownership while its bounded corrections finish; H's separate JPL reference assignment continues.
