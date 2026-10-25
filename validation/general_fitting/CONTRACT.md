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
`force_always_fit_for_glitches: bool`, and `acceleration: i32`. Covariance paths
resolve against the actual working directory; included paths and consumed bytes
are recorded. CLI mappings are `--covariance`, `--norescale`,
`--force-always-fit-for-glitches`, `--acceleration`. Without enabled acceleration,
the source fallback uses SVD and reports the fallback. MODE/WEIGHT, ROBUST and
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

## Prospective numerical comparisons

All 0007, 0012 and 0071 gates remain unchanged. Matrices and whitening columns
use 5e-14 times the reference column maximum, exact structural zeros. Increments,
raw covariance, errors and chi-square retain the applicable identical-input or
independent-pipeline 0007 gate. Timing is 1e-12 s; phase is 1e-10 turn; all row,
column, mutation, count, mode/drop and class identities are exact.

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
failures, four 0049 utility boundaries, original DDH 1 ns ideal-fixture gate.
Production remains Rug128; no sorted modes or compatibility precision switch.
