# M2 fitting contract and Assignment E release

Central decision, 20 October 2026. **Release Assignment E for the bounded fitting implementation below. M2 is not accepted.** M0, M1a and M1b remain accepted. This decision authorizes implementation in a separate chat; it does not create that chat.

**Amended 23 October 2026 by [decision 0008](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0008-preservation-policy-and-stage-plan.md): preserve TEMPO2 bugs during the transfer and defer corrections.** Its result/failure policy supersedes the original guard requirements; the finite fitting equations and numerical gates remain unchanged. The contract therefore adopts the pinned source's fitting coordinates, arithmetic boundaries, SVD, update order, iteration lifecycle and uncertainty conventions. It does not substitute a different least-squares method or redefine the timing model.

## Starting identity and ownership

The previously accepted delivery is now recorded in these primary-repository commits:

| Commit | Scope |
| --- | --- |
| `3f08a8c` | `feat: add TEMPO2-compatible PKS observatory timing` |
| `31d55bf` | `test: add M1b fixtures and resource recovery evidence` |
| `87f3933` | `docs: record M1a and M1b acceptance decisions` |

The implementation baseline is `87f3933c6840a00e1f1aa2dd1154a0e810efedd6` plus this central documentation update. These commits preserve the accepted report and source-manifest hashes in decision 0006. Canonical TEMPO2 remains at `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; the independent A2 repository remains at `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`. Neither repository is changed.

E owns `rust/` and a new, root-tracked `validation/fitting/` directory. The latter holds fitting reference exports, their generator, immutable inputs, schema, comparator and provenance. It may read the accepted A2 build and create a disposable build under its own ignored `build/` directory. It must not edit A2's code, goldens, manifests or Git history. Central decisions, this evidence directory and milestone acceptance remain owned by this chat.

## Source inspection and executable evidence

The selected route is the modern, single-pulsar `t2Fit` path with no correlated-noise model, robust fit, global parameters or constraints. The legacy wrappers below `t2fit.C:2110` are not the entry-point specification.

| Source | Contract established |
| --- | --- |
| [tempo2.C:513](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/tempo2.C:513) | For each NITS iteration: form BATs and mean-subtracted residuals, fit and update once, then recompute BATs and mean-subtracted residuals. |
| [initialise.C:113](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/initialise.C:113), [readParfile.C:175](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/readParfile.C:175), [getInputs.C:160](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/getInputs.C:160) | NITS defaults to 1; MODE/WEIGHT defaults to 0; robust fitting is off; error rescaling defaults on; `-norescale` clears its flag. |
| [formResiduals.C:2364](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/formResiduals.C:2364) | Arithmetic, unweighted mean over undeleted observations, accumulated/subtracted in `longdouble`. |
| [t2fit.C:811](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/t2fit.C:811) | Observation selection, prefit residual snapshot, BBAT-relative days, binary64 fit data and uncertainty conversion. |
| [t2fit.C:166](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/t2fit.C:166), [cholesky.C:486](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/cholesky.C:486), [t2fit.C:250](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/t2fit.C:250) | MODE 0 replaces uncertainties by 1; MODE 1 divides data and matrix entries by uncertainties in seconds. |
| [t2fit.C:950](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/t2fit.C:950), [t2fit.C:2061](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/t2fit.C:2061) | Mandatory ZERO column, then supplied local fit parameters in source enumeration/subindex order. |
| [t2fit_stdFitFuncs.C:14](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/t2fit_stdFitFuncs.C:14), [t2fit.C:917](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/t2fit.C:917) | Spin columns, separate update functions and sequential parameter updates. |
| [t2fit.C:298](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/t2fit.C:298), [TKfit.C:325](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/TKfit.C:325) | SVD cutoff, raw covariance, chi-square, error scaling and `n - p` degrees of freedom. |
| [TKsvd.C:43](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/TKsvd.C:43), [T2toolkit.C:93](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/T2toolkit.C:93), [TKlongdouble.float128.h](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/TKlongdouble.float128.h) | SVD algorithm, convergence/iteration behavior, binary64 sign helper, and extended math overloads. |

The [central probe and reproduction instructions](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/m2/README.md) call the accepted static library directly. Its hash matches A2's build manifest. The probe confirms:

- Native C++ `long double` has a 53-bit significand on this reference host, while TEMPO2's `longdouble` abstraction uses 113-bit binary128. The spin-column function explicitly uses the former; the SVD primarily uses the latter. `TKsign_d` takes and returns binary64.
- With the same weighted matrix/data, toggling rescaling changes the reported errors and leaves both fitted increments and covariance identical.
- Identical columns produce the source pseudoinverse; diagonal singular values below the prescribed cutoff are dropped, while values at/above it are retained.
- A square exact fit with rescaling enabled returns nonfinite errors. Disabling rescaling gives finite parameter estimates, covariance and errors for that same matrix.
- The spin update callbacks use the newly updated F0 for higher derivative updates.

This is source-design evidence, not a Rust M2 implementation or end-to-end fitting validation. Native-type boundaries are pinned to the accepted macOS arm64 reference profile; Linux fitting is not yet verified.

## Supported fitting request

Support one isolated pulsar with the existing homogeneous `@` or PKS input profile. Retain the accepted time scales, phase reference, runtime configuration, propagation physics, source order and meteorology fallbacks. All input observations participate, in their original order.

Fit the mandatory ZERO offset and any supplied subset of F0–F3 having local `fit_flag = 1`. Preserve unset/fixed terms. Always include ZERO, including a request with no selected spin parameters. Column order is ZERO, F0, F1, F2, F3, omitting unselected terms. A selected but zero-valued derivative is still a fitted parameter.

Accept PAR `MODE 0`/`MODE 1` and its `WEIGHT` alias, plus positive integer `NITS`. Preserve raw spelling/tokens and the resolved defaults. For repeated MODE/WEIGHT declarations, preserve the source's sequential assignment: the last declaration wins, and the original source text remains available. Do not add TIM MODE or weighting directives in this assignment. Reject unsupported mode values and invalid iteration counts with input context.

The new fit operation rejects global fit flags (`2`) and active fit flags on parameters outside F0–F3. A fixed-parameter evaluation continues to preserve fit flags without performing a fit, as it does now. New fit restrictions must not retroactively reject previously supported fixed-evaluation inputs.

Additional derivatives, fitting DM/astrometry/binaries, BRAKE, TZR phase activation, pulse tracking, phase jumps, deleted-point/cut directives, START/FINISH selection, EFAC/EQUAD, robust/correlated-noise fitting, constraints, multiple pulsars, plugins and automatic convergence stopping are outside M2. Existing unsupported-keyword errors remain explicit. These are coverage limits, not claims that the upstream features are wrong.

## Iteration, residual and offset semantics

Perform exactly NITS update cycles; the default is one. Each cycle:

1. Evaluate the current model through the existing BAT and residual kernels. Retain the raw residuals as a separately identified diagnostic.
2. Accumulate their arithmetic mean in input order at extended precision and subtract it from each residual. This mean is unweighted even in MODE 1. Preserve the first-input-TOA phase reference and existing pulse-number calculation.
3. Construct fit data/matrix, solve, record the pre-update model and all fit outputs, then update the selected parameters in column order.
4. Re-evaluate the updated model and apply the same mean removal. This gives the cycle's recomputed postfit timing residuals. Refresh the prefit model/errors before the next cycle, matching the source lifecycle.

Do not add mean removal to `evaluate_barycentric`, `evaluate_terrestrial` or the existing `residuals` CLI. A fit result explicitly records `remove_mean = true` and `fit_performed = true`; its raw diagnostic is labeled accordingly.

ZERO estimates a fitted offset in seconds and has a reported uncertainty. TEMPO2 stores `offset = estimate`, rather than accumulating it. The selected `formResiduals` calculation does not subsequently subtract that stored offset as a new model parameter. Preserve this behavior. The offset is part of the linearized solution and its covariance; the recomputed postfit residuals come from the updated spin model and arithmetic mean removal.

Report these distinct quantities without relabeling one as another:

- Mean-subtracted prefit timing residual, used to form the binary64 data vector.
- Linearized postfit residual `y - A p`, with the source's reported binary64 increments and binary64 accumulation order.
- Recomputed, mean-subtracted timing residual after updating the model.

The source's fit chi-square comes from its solver calculation, not from substituting the recomputed residuals. Original SATs, TOA uncertainties, input identity and supplied metadata never change during fitting.

## Matrix, units and parameter updates

Let `x_i = f64(BBAT_i - PEPOCH)` in days, `y_i = f64(centered_residual_i)` in seconds, and `sigma_i = f64(toa_error_us_i * 1e-6)` in seconds. Subtract the epochs in extended precision before narrowing to binary64. MODE 0 uses a numerical denominator of 1; MODE 1 uses `sigma_i`. Divide each matrix entry and `y_i` by that denominator in binary64.

With no BRAKE term, the source expressions are:

| Column | Expression, preserving the written operation order |
| --- | --- |
| ZERO | `1` |
| F0 | `x * 24.0L * 3600.0L / f0` |
| F1 | `0.5L * x * x` |
| F2 | `1.0L / 6.0L * x * x * x / 1.0e9L` |
| F3 | `1.0L / 24.0L * x / 1.0e18L * x * x * x` |

Here `f0` and `L` expressions are native `long double`, hence binary64 for the accepted reference profile. Return binary64 columns. Do not evaluate these expressions entirely in Rug or silently replace them with powers of elapsed seconds. F1–F3 are source-scaled fitting coordinates, not directly increments in Hz/s, Hz/s² and Hz/s³.

Solve `A p ≈ y`. Apply the reported binary64 estimates/errors using `t2UpdateFunc_stdFreq`:

- F0: `F0 *= (1 - p_F0 / F0)` in the existing extended model type. Preserve this expression rather than algebraically simplifying it. Its physical reported error is the F0 fit-coordinate error.
- For `k = 1,2,3`, use `s_k = 1, 1e9, 1e18` and `D_k = pow(24.0 * 3600.0, k + 1)` evaluated as binary64. Update `F_k -= F0_current * (p_k / D_k) / s_k`. The quotient in parentheses is binary64 before promotion.
- The physical reported error is `((e_k / D_k) / s_k) * F0_current`. The first quotient is binary64; subsequent operations follow the source extended `scale` boundary. F0 has already been updated if it was selected; otherwise its fixed value is used.

Preserve the source sign, units, association, casts and update order. Do not use the original F0 for every derivative update or interpret the raw fit covariance as physical-parameter covariance. M2 reports the raw covariance with explicit fit-coordinate labels and separately reports the source's physical parameter errors. A transformed physical covariance is not required and must not replace the raw matrix.

## Solver and uncertainty contract

Use the nonaccelerated TEMPO2 SVD path from `TKsvd.C`/`TKfit.C`, ported to Rust using the existing fixed Rug128 representation for source `longdouble` operations. Keep intentional binary64 boundaries, including all calls equivalent to `TKsign_d(double,double)`; its sign test is `b >= 0`, not an unrestricted sign-bit copy. The running norm also passes through `TKretMax_d(double,double)` at `TKsvd.C:247`, narrowing both arguments and the return before assignment back to the extended norm. Preserve nonfused operation order and source math semantics.

A2 disables BLAS/LAPACK/ACCEL_LSQ. A new binary64 QR, normal-equations solver, column normalization, a different rank threshold or a selectable solver backend is not authorized. Reuse Rug and existing error/numeric facilities; do not add a new numerical dependency for this bounded port. Preserve source attribution and licensing.

Promote the already rounded binary64 whitened matrix and data into the solver. With singular values `w`, set a value to zero only when `w_j < tol * max(w)`; `tol` is the source binary64 literal `1e-27`, promoted for the comparison. Do not replace the strict comparison or use a machine-epsilon-based cutoff. Zero values contribute zero in back substitution and covariance.

Preserve the source pseudoinverse for rank-deficient matrices. Report singular values, retained/dropped modes and effective rank, with a warning when a mode is discarded. A zero variance along an unsupported mode is a pseudoinverse artifact, not evidence of an exactly known physical parameter. Do not reject an otherwise finite fit merely because rank is below the number of columns. Do not replace degrees of freedom by `n - rank`.

Covariance is `C_jk = sum_l V_jl V_kl / w_l²` over retained modes, accumulated at extended precision and narrowed to binary64. Implement the source `1 / w / w` operation order. The unscaled error is `sqrt(C_jj)` in binary64. The source error multiplier is:

```text
rescale_effective = (MODE == 0) || (MODE == 1 && rescale_err_chisq)
nu = n_observations - n_columns
error_multiplier = sqrt(fit_chi_square / nu) if rescale_effective, otherwise 1
```

**Multiply the error estimates, not the covariance matrix.** MODE 0 still rescales when the user supplies the no-rescale flag. Record requested and effective behavior. The covariance, fit-coordinate errors and mapped physical errors are separate output fields.

The solver computes chi-square using the extended solution before narrowing it to the reported binary64 increments. Its inner products and squared residual are extended; the running `double chisq` is promoted, added and narrowed after each row. Preserve those boundaries. In MODE 1 this is the dimensionless weighted statistic. In MODE 0 report it as the unweighted sum of squared residuals in seconds squared, rather than suggesting measurement uncertainties were used.

Preserve source convergence tests and the effective nonconvergence branch at iteration 30 (`max_its = 40` does not bypass that branch). Return a contextual error instead of terminating the process.

## Source outcomes and adapter boundaries

[Decision 0008](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0008-preservation-policy-and-stage-plan.md) replaces the original invalid-fit correction table. Preserve defined nonfinite outcomes, including square-fit rescaling, and the source update callback's `error >= 0` predicate. Fit diagnostics encode NaN and signed infinity explicitly. Do not turn these source calculations into different finite values or preemptive numerical corrections.

For upstream assertion/exit, uninitialized outputs or unsafe access, record the actual failure class and the Rust adapter's contextual error without claiming exact successful output. Preserve finite rank-deficient pseudoinverses. Keep caller input owned/unchanged and never publish a partially serialized result. Existing state/error differences remain visible in the compatibility ledger; defer bug fixes until after transfer.

## Minimal library and CLI

Keep the existing crate and explicit runtime ownership. Add conceptually:

```rust
FitOptions { rescale_err_chisq: bool } // default true; source -norescale control
fit_barycentric(input: &TimingInput, options: &FitOptions) -> Result<FitResult, TimingError>
fit_terrestrial(input: &TimingInput, runtime: &mut RuntimeContext,
                options: &FitOptions) -> Result<FitResult, TimingError>
```

MODE/WEIGHT and NITS come from validated PAR controls, with defaults recorded. Do not create parallel CLI/API overrides for those controls. Preserve original tokens and keep the existing fixed-evaluation JSON unchanged for all previously supported inputs. A small typed accessor/state for validated fit controls is reasonable; avoid a second parser or generic fitting framework.

Add `tempo2-rust fit --par PATH --tim PATH [--runtime PATH] [--norescale]`. It uses the library operations and follows existing atomic stdout/error conventions. `--runtime` is required for PKS and unnecessary for `@`. No PAR-file rewrite, output-file mode, plotting or interactive fitting interface is required.

Use a distinct versioned fit JSON contract. Include original input identity; resolved controls/defaults; source/numerical/runtime provenance; column labels/scales; per-cycle pre/post model values; original-observation row mapping; BBAT-relative x, raw and centered residuals, uncertainties, matrix and whitened matrix/data; singular values/rank/drop diagnostics; fit increments; raw covariance; raw/rescaled fit-coordinate and physical errors; offset; chi-square/degrees of freedom; linearized and recomputed postfit residuals; and final timing/resource warnings. Source-sensitive numbers use round-trip decimal strings and counts remain lossless.

Reuse existing timing/result components where useful, but label original supplied parameters/uncertainties separately from fitted values/errors. Do not overwrite the supplied uncertainty metadata or imply that the original PAR text contains the fitted solution. Keep TEMPO2-corresponding names such as `t2fit`, `tkfit`, `tksvd` and `std_freq` where appropriate; document the naming map.

## Reference fixtures and comparison gates

E first builds the fitting fixture family in `validation/fitting/`, before judging the Rust implementation against it. Read or reuse the verified A2 static library; a missing build may be reconstructed in E's own disposable directory from the pinned source and the exact approved initializer patch. Record compiler/configuration, precision, instrumentation changes, executable/source/library/input/resource hashes and commands. Any added instrumentation must observe initialized values without changing science. Do not rerun A2 scripts that overwrite its tracked evidence.

Export through the actual `t2Fit` call and the specified pre/post `formBatsAll`/`formResiduals(...,1)` lifecycle. Compare derivative callbacks and low-level solver cases separately. Pure postprocessing is allowed to compute the named linearized residual and rank diagnostics; identify it explicitly. Replaying only Rust's own equations in the exporter is not a reference.

Required families, with every row's complete decimal input committed:

1. **Solver controls:** deterministic full-rank matrices with heterogeneous errors; rescaling on/off; exact dependent columns; one zero column; singular values on both sides of the `1e-27` cutoff and the central probe's exactly represented diagonal equality case; the square-fit nonfinite-rescaling/unscaled-finite pair. Include the central probe cases as independently generated checks. No random data or tolerance-driven fixture selection.
2. **Barycentric fitting:** ZERO only; each of F0, F1, F2 and F3 separately; a nonconsecutive subset; all four together. Include nonzero initial derivatives, heterogeneous uncertainties, MODE 0 and MODE 1, both rescale settings, an unsorted input, and NITS 2. Cases may cover several requirements together.
3. **PKS fitting:** a copy of the accepted seven-TOA case with selected spin fit flags and heterogeneous positive errors, under both weighting modes; plus a controlled, phase-connected small-perturbation case with at least 12 distinct epochs spanning both sides of PEPOCH and all F0–F3 selected. Use the accepted resource/time/meteorology profile. Do not change the original files.
4. **Failure and regression:** unsupported/global fit requests, invalid controls/weights/counts, nonconvergence/error path, caller immutability, same-process recovery, CLI failure publication and all accepted M1a/M1b checks.

For controlled timing fixtures, choose exact declared epochs and deterministic perturbations, record a truth model separately from the starting fit model, and keep every relevant phase away from half-turn rounding boundaries. Record any reference alignment/root-finding used to construct TOAs, with its final error bound. Freeze and hash inputs before Rust fitting comparisons. Compare fixed-parameter predictions first: fitting must not hide a pre-existing timing discrepancy. Do not claim recovery of a truth model merely from agreement with TEMPO2.

The following are prospective engineering gates, not measured M2 performance. They are fixed before implementation; failures require localization and central review, not automatic relaxation. Use decimal/extended comparisons, check shape and numeric classification before arithmetic; compare source nonfinite classes exactly under decision 0008 and apply finite budgets only to finite pairs, and report maximum signed/absolute differences with fixture/cycle/row/column identity.

Let `a_j = max_i |A_ref[i,j]|` for unwhitened columns. For a zero reference column require an exactly zero candidate column, zero fitted increment/error and zero covariance row/column; its physical parameter remains unchanged from the input. Covariance normalization uses `sqrt(C_ref[j,j] * C_ref[k,k])` computed without binary64 underflow in the comparator. In the physical-update gate, `delta Fk` is the difference between engines' updated values and `Delta Fk_ref` is the reference's change from its prefit value in that cycle.

| Quantity | Gate |
| --- | --- |
| Inputs, row mapping/order, flags, column order, controls, iteration count, degrees of freedom, effective rank/drop decisions, pulse counts | Exact for the prescribed fixtures. Match singular modes together with their drop flags as specified in decision 0010; their raw index is not a physical column identity. Do not compare nonunique SVD vector signs as scientific outputs. |
| `x` after its binary64 boundary | At most 4 binary64 ULPs of the reference; structural zero is exact. |
| Matrix entries | `|delta A_ij| <= 5e-14 * a_j`; apply the same rule with the corresponding whitened column maximum to the whitened matrix. Structural zeros remain exact. |
| Singular values | Match the finite spectrum and associated flags under decision 0010, preserving raw order in output. Retained values agree relatively within `1e-10`; discarded values satisfy the specified cutoff and exact drop classification. Preserve the exactly represented equality control. |
| Identical-input solver increments | `a_j * |delta p_j| <= 1e-13 s + 1e-11 * a_j * |p_ref,j|`; additionally `max_i |sum_j A_ref,ij * delta p_j| <= 1e-14 s`. |
| Raw covariance, identical-input solve or end-to-end fit | `|delta C_jk| <= 1e-10 * sqrt(C_ref,jj * C_ref,kk)`; a structural zero-variance row/column remains exactly zero. |
| Fit-coordinate/physical errors, identical-input solve | Relative difference at most `1e-10`; an analytically exact zero remains zero. |
| Solver chi-square, identical input | Relative `1e-10` plus absolute `1e-12` for MODE 1 or `1e-24 s²` for MODE 0. |
| End-to-end increments and mapped physical updates | Fit-coordinate comparison above with an absolute floor of `1e-11 s`. For physical Fk compare the difference using `b_k = max_i |t_i|^(k+1) / ((k+1)! * |F0_prefit|)`, `t_i = (BBAT_i - PEPOCH)*86400`, and gate `b_k*|delta Fk| <= 1e-11 s + 1e-10*b_k*|Delta Fk_ref|`. ZERO is compared in seconds with the same absolute floor. |
| End-to-end reported errors | `a_j*|delta e_j| <= 1e-11 s + 1e-6*a_j*|e_ref,j|` in fit coordinates; use `b_k` for physical errors. An exactly zero discarded-mode error remains zero. |
| End-to-end chi-square | Relative `1e-6` plus absolute `1e-12` for MODE 1 or `1e-24 s²` for MODE 0. |
| Mean, centered prefit residuals, linearized postfit residuals, recomputed postfit residuals, SAT/BAT/BBAT | At most `1e-12 s`, converting epoch differences to seconds before comparison. |
| Full phase | At most `1e-10 turns`, with residual and exact pulse-count gates independently enforced. |
| Existing fixed-parameter M1a/M1b fixtures | All original gates and scientific outputs remain unchanged. |

For a zero `b_k` (all epochs at PEPOCH), enforce the zero-column/unchanged-parameter rule rather than dividing by zero. Do not infer truth recovery or identifiability from a finite pseudoinverse result. Apart from the exactly represented diagonal equality control, cutoff tests must be deliberately separated from the threshold; a precision-sensitive borderline case is diagnostic and must not be silently reclassified.

The separate end-to-end error/chi-square allowance accounts for propagation of the `1e-12 s` timing budget into fixtures with microsecond-scale residual structure. It does not relax the identical-input solver check. Every comparison also enforces the final residual gate. Acceptance requires both a shared-input solver check and complete independent end-to-end reference/Rust runs; neither alone suffices.

## Delivery and central acceptance

Follow [Assignment E's handoff](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/05-fitting-core.md). Deliver reproducible fitting exports/comparison tools, the Rust implementation, a naming/departure map, source hashes, verification records and `rust/M2_REPORT.md`. Include individual case maxima, failure diagnostics and all known limits.

The original DDH ideal-fixture failure remains open at its unchanged 1 ns threshold; fitting this subset does not resolve it. Broader observatories/models, Linux, performance, concurrency stress and packaging remain separate coverage work. This decision approves the contract and dispatch readiness only. M2 acceptance requires central review of E's implementation and evidence.
