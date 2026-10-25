# AE canonical fitting interface and comparison freeze

Baseline: AD production a8e5178778f7eeb3a0fc65f8934a64a03f94208d,
report 0f44180a021655074c387e2c03012d0c4253276e; release ae03c87.
Source fda6ad4a9753ad7dc5d291406f2f310a9161423f. This is an implementation
contract, not numerical acceptance.

## API and execution order

`general_fitting::fit(&mut bats::BatInput, &mut RuntimeContext, &FitOptions)`
executes source NITS from pulsar zero, copying parameter values/errors to prefit
storage on later iterations and recomputing BATs and mean-subtracted residuals
before and after each solve. Input parsing/preprocessing happens once, outside
this operation. `general_fitting::t2_fit(&mut PulsarContext, &FitOptions)`
executes the built-in fit on prepared BAT/residual state, returning owned row,
column, constraint, whitening and solver inspection results. `do_fit_all` is the
built-in source adapter. Derivative/update APIs use canonical context, pulsar,
observation, source Label and subindex. The source's callback selection remains
separate from standalone callback availability. Dynamic native plugins are an
explicit unsupported host ABI boundary.

FitOptions carries `covariance: Option<String>`, `rescale_err_chisq: bool`,
`force_always_fit_for_glitches: bool`, `acceleration: i32`, and optional owned `tkfit::FitDiagnostics`
(`directory`, source `write_residuals` bit mask, `debug`). Covariance paths
resolve against the actual working directory; included paths and consumed bytes
are recorded. CLI mappings are `--covariance`, `--norescale`,
`--force-always-fit-for-glitches`, `--acceleration`. Without enabled acceleration,
the source fallback uses SVD and reports the fallback. `--fit-diagnostics DIR`
selects source writeResiduals=7 in an existing output directory. The direct Rust
API exposes other bit combinations and debug covariance output. Files preserve
source names, text formats and overwrite order; I/O failures return an error.
For correlated fits, debug output includes `newDCF` for the applicable synthesis
kernel and `chol.covarMatrix`/`chol.L`; source overwrites during INCLUDE/multiple
models are retained. `cholesky::read_model` exposes the same grammar on supplied
coordinates. `get_cholesky_matrix` takes the trailing constraint count and optional
`FitDiagnostics`, returning owned covariance/lower matrices and written-file paths.
The diagonal helper retains its distinct determinant convention.
Robust results include the ordered scale/final solve history. The first solve
writes prefit/design files; later solves retain only bit 4, and the final files
reflect the final weighted solve before the source robust covariance overwrite.
No process-global diagnostic mask is modified. MODE/WEIGHT, ROBUST and
NITS remain PAR-owned. `timing --fit --no-mean` is an error.

The new `tempo2-general-fit-v1` result contains supplied controls, per-cycle
prefit/postfit full timing projections, each solve's selected observation and
original input identities, source-local columns and combined global coordinates,
physical before/after values/errors, matrices/data/constraints/targets, raw solver
mode order/rank/covariance/errors/chi-square/dof, and canonical noise projections.
Global coordinates precede pulsar-local blocks in the solve; each pulsar updates
locals followed by globals. Counts and membership are exact. Numbers retain
extended decimal or explicit nonfinite class; unwritten/inapplicable values are
null, never manufactured zeros. Bounded fit adapters retain their v1 schemas and
reuse the same solver/callback arithmetic.

Each physical-update record additionally exposes `postfit_value` and
`postfit_error` after the source TN polynomial/SWGP transfers. `after_value` and
`after_error` remain the immediate callback result. This preserves the distinction
between a fitted increment and the deterministic state used by the final timing
evaluation. The same physical-coordinate gates apply to both snapshots.

## Prospective numerical comparisons

All 0007, 0012 and 0071 gates remain unchanged. Matrices and whitening columns
use 5e-14 times the reference column maximum, exact structural zeros. Increments,
raw covariance, errors and chi-square retain the applicable identical-input or
independent-pipeline 0007 gate. Timing is 1e-12 s; phase is 1e-10 turn; all row,
column, mutation, count, mode/drop and class identities are exact.

Physical updates use each source callback's units. For callbacks that add or
subtract the fit value directly, divide the physical difference by unit gain 1
and multiply by the reference design-column maximum. ELL1H PB has gain
1/86400 (days per fitted second); its A1/EPS1/EPS2/TASC/H3/H4/STIG callbacks have
gain 1. Spin retains its time-power/frequency conversion. Compare before/after
values with 1e-11 s plus 1e-10 times the reference update's equivalent seconds;
physical errors use 1e-11 s plus relative 1e-6, with structural zeros exact.
These extend the existing spin checks without widening their budgets. Native
observers explicitly identify unwritten first-cycle error/offset storage and do
not evaluate it. Unsupported physical unit mappings remain listed as unchecked.

New identical-input quantities, with arithmetic performed in the comparator at
extended decimal precision:

| Quantity | Unit and reference scale | Gate |
| --- | --- | --- |
| Covariance kernel/matrix | s^2; sqrt(Cii*Cjj) for matrix, maximum magnitude for sampled kernel | 5e-14 times reference scale; structural zero exact |
| Cholesky L | s; reference column maximum | 5e-14 times scale; structural zero exact |
| Whitened data | dimensionless; reference maximum magnitude | 5e-14 times scale; structural zero exact |
| Prior/constraint matrix and RHS | source fit-coordinate weight and dimensionless RHS, respectively; reference column/RHS maximum | 5e-14 times scale; structural zero exact |
| log determinant | natural log of source dimensioned numeric convention; max(1, abs(reference)) | 5e-14 times scale |
| Reconstructed noise/error | s | 1e-12 s, exact structural zero |
| Robust scale and weights | source normalized residual and dimensionless weight; max magnitude | 5e-14 times scale; exact branch/iteration membership |
| Seeded bootstrap RNG | source binary64 | exact bits and seed history |
| Standalone spectral density | source declared density units; maximum reference magnitude | 5e-14 times scale, structural zero exact |
| DM/CM normalized grid weights | dimensionless, derived from source covariance errors | relative 1e-10, zero exact; grid membership/epochs exact binary64 |
| Standalone callback derivatives | source residual seconds per fit coordinate; separate column maximum | 5e-14 times reference column scale, structural zero exact |
| Standalone glitch update on identical supplied increments | stored GLF0 (Hz) / GLF2 (Hz/s^2), each value/error separately | relative 5e-14, structural zero exact; no shared scale between different units |

Ill-conditioned or precision-boundary failures are reported, not accommodated by
changing these gates. An unchecked field is not accepted. Original and corrected
native observations are separate and never overwrite accepted evidence.

## Dependency-ordered implementation

1. Canonical general fit rows, columns, callbacks and NITS public workflow.
2. Shared solver constraint RHS, robust fitting and matrix operations.
3. Remaining source callbacks, constraints, covariance and timing-noise lifecycle.
4. Legacy/standalone dependency closure, recovery and optional side effects.
5. Independent public/component checks, affected AC/AD/M1/M2 regressions,
   formatting/Clippy, evidence identities and final report.

Inherited open ledger: P 154 (including SIMUL 32), M2 four exact raw mode-order
failures, four 0049 utility boundaries. DDH now follows 0087's approved paired
current-clock fidelity and versioned historical-clock 1 ns pre/post-fit checks;
the original current-clock native assertion failure remains a preserved record.
Production remains Rug128; no sorted modes or compatibility precision switch.

The identical-input solver observer exports errors before rescaling, the applied
error multiplier (including the robust wrapper's override), and the source
binary64 unweighted linearized residual. Unscaled errors use the existing
relative `1e-10` error gate, multiplier uses relative `5e-14`, and linearized
residuals use `1e-12 s`. Structural zeros and numeric classes/signs remain exact.
Public result shape requires all unscaled errors and one linearized residual per
selected observation; standalone observations validate the shared implementation.
