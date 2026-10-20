# M1a acceptance

Central decision, 20 October 2026. **M1a is accepted at project commit `49e122c3648e5cb91bae17233a062a444a10f9e4` for the declared macOS arm64 isolated-pulsar, TCB `@`, F0–F3 path. Assignment C is complete.** D remained gated when this acceptance was recorded.

Subsequent release: [decision 0004](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0004-m1b-contract-and-d-release.md) adopts the terrestrial contract and releases D under the user's instruction to preserve TEMPO2 decisions and comment justified departures at the changed code. The remaining-gate discussion below records the conditions at M1a acceptance; M1b itself remains pending.

The accepted submission is [C's report](/Users/Eric.Dong/Downloads/tempo2rust/rust/M1A_REPORT.md), including the user-requested TEMPO2 naming pass and initial project commit. [Decision 0001](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0001-reference-and-m1a-contract.md) continues to define the numerical representation and public interface; [decision 0002](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0002-m0-acceptance-and-c-release.md) pins the A2 reference, fixtures and comparison gates. No scientific convention or tolerance changes in this acceptance.

## Evidence and review

The central review inspected the submitted implementation, test adapter, behavioral tests, independent arithmetic diagnostics and recorded command results. The live workspace check then confirmed:

- The primary repository was clean at the full commit above before these planning-document edits.
- All 31 files in [source_hashes.json](/Users/Eric.Dong/Downloads/tempo2rust/rust/verification/source_hashes.json) match the passing run's recorded hashes, including source, lockfile, tests, verification script and copied contract.
- All 14 files in the [reference provenance record](/Users/Eric.Dong/Downloads/tempo2rust/rust/tests/reference/provenance.json) match both their recorded hashes and the independent A2 artifacts.
- The independent reference repository is clean at `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`; canonical TEMPO2 is clean at `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
- The unchanged comparator passes all three saved Rust projections. Recomputing the Decimal differences reproduces the recorded maxima across 177 scalar stage comparisons, 39 parameter comparisons and 16 residual rows. Identity, flags and full/reduced pulse counts match exactly. The saved F3 and 4 ns diagnostic summaries also reproduce and pass their separate gates.

The [recorded checks](/Users/Eric.Dong/Downloads/tempo2rust/rust/verification/commands.json) pass all nine Rust tests, the reference comparisons, formatting and Clippy. The naming-pass evidence records byte-identical numerical artifacts and identical exit status, stdout and stderr for 17 CLI cases. The central review checked the code and this evidence and reran the read-only comparisons above; it did not rebuild Rust or TEMPO2 or rerun the Rust test suite. This is sufficient for this bounded milestone.

The previous live-access failure was caused by the obsolete workspace path. The current project root is `/Users/Eric.Dong/Downloads/tempo2rust`. Central planning links now use this path. Submission-time paths and pending-acceptance wording in the implementation and reference evidence remain historical provenance; this decision records their subsequent central acceptance.

## Accepted implementation

The core uses Rug 1.30.0 `Float` at a fixed 128-bit significand, direct MJD values, source-split phase arithmetic, retained binary64 boundaries and checked pulse counts. The phase kernel preserves source operation order, including materializing multiplication before addition to avoid an unintended fused operation.

The supported readers retain original tokens, TOA identity and input order, validate the declared PAR/FORMAT 1 subset, and reject unsupported active inputs explicitly. The first eligible TOA supplies the signed fractional reduced-phase reference. There is no fit or mean subtraction. Source integer-range limits and signed 64-bit pulse-count overflow produce explicit errors.

The public `load` and `evaluate_barycentric` operations and JSON residuals CLI satisfy the adopted contract. Failed loading or evaluation is followed by successful processing in the same process without changed inputs or stale results. CLI calculation failures return contextual errors and empty stdout. The source-corresponding module and helper names are accepted; the public API and arithmetic behavior are unchanged by the naming pass.

The test adapter independently evaluates Rust outputs, including aligned and 4 ns perturbed epochs, while preserving the original observation identity. It uses the copied A2 contract without requiring a live reference engine. Nonzero F3, signed half-pulse boundaries, negative first-reference phase, decimal precision and overflow have focused checks beyond the three reference fixtures.

## Numerical acceptance

| Quantity | Largest measured difference | Adopted gate |
| --- | ---: | ---: |
| SAT/BAT/BBAT, epoch difference converted to seconds | `2.7647740800e-25 s` | `1e-12 s` |
| Full phase | `4.977176e-23 turns` | `1e-10 turns` |
| Residual | `3.76200923958827030e-25 s` | `1e-12 s` |
| Parsed nonzero extended parameters, relative | `3.47149e-35` | `1e-30` |
| Full/reduced pulse counts, identity and flags | Exact | Exact |

Expected-zero parameters remain exact. Observing-frequency and uncertainty differences are zero within their separate `1e-9 MHz` and `1e-12 microsecond` gates. These are compatibility results for the accepted cases, not physical-accuracy claims.

The separate nonzero-F3 diagnostic against a 512-bit unsplit polynomial has maximum phase and residual errors of approximately `2.03184e-32 turns` and `1.64579e-34 s`, within `1e-27 turns` and `1e-29 s`. Aligned residual magnitude and 4 ns response error are at most `2.66241e-30 s` and `2.49010e-30 s`, within their `1e-20 s` diagnostic gate.

## Git organization and remaining gates

The authorized primary-repository bootstrap is complete. Commit `49e122c` tracks the central documents, Rust implementation and precision study, with both independent reference repositories and build/cache directories excluded. This acceptance updates only central planning documents.

**M1b is pending and D is not released.** Before dispatch, the central chat must adopt the terrestrial fixture/resource configuration, active correction chain, retained astronomical dependencies, per-stage comparison budgets and explicit missing-meteorology policy. C's accepted barycentric path remains a regression requirement for D.

The DDH/DE414 ideal-fixture failure remains open under its original 1 ns threshold: first failure approximately `1.137585 ns`, maximum approximately `4.598955 ns`. It is outside the accepted isolated barycentric subset; reopen relevant acceptance if causal evidence implicates a shared component. The terrestrial reference's pressure and wet-delay defaults are not an approved production meteorology policy.

Linux and other targets, full upstream-suite coverage, packaging, cold-build cost, performance, terrestrial timing, binary models and fitting remain unvalidated or outside this milestone. M1a acceptance makes no claim about those paths.
