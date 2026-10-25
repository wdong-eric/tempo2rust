# AE source correspondence and ownership

Source pin: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. This map describes
the current implementation; final routine/branch and public-API inventory remains
part of AE's outstanding acceptance work. Passing representative cases does not
make an unexercised branch covered.

| Source / headers | Rust implementation and public route | Numerical and ownership boundary |
| --- | --- | --- |
| `tempo2.C` fit iteration; built-in `doFit.C` | `general_fitting::{fit,t2_fit,do_fit_all}`, `main` | One AC input context; NITS from pulsar zero; full BAT/residual evaluations around every solve; source mean convention |
| `t2fit.C` modern preparation, selection, column layout and updates | `general_fitting`, `fit_callbacks` | SAT/BAT cuts precede binary64 fit-array projection; global-first combined coordinates; local-then-global per-pulsar updates; no separate authoritative parameters |
| `t2fit.C` legacy matrix/fit wrappers | `legacy_fitting` | Callback-based canonical context; source diagonal multiplication versus modern sigma division; global constraint remapping |
| `t2fit_stdFitFuncs.C` | `t2fit_stdFitFuncs`, `fit_callbacks`, shared `spin_update`, AD `binary` | Source spin powers/narrowing, ZERO, jumps, FDJUMPs, DM/CM, telescope and planet paths; default binary dispatch reuses AD |
| `t2fit_position.C`, `t2fit_glitch.C`, `t2fit_dmother.C` | Corresponding private modules via `fit_callbacks::{derivative,update}` | Preserve angular/time/unit conversions, event selection and default callback choices; FDDI remains source-not-implemented |
| `t2fit_dmmodel.C`, `t2fit_fitwaves.C`, `t2fit_ifunc.C`, `t2fit_solarwind.C` | Corresponding modules through the same callback route | Owned grid/coefficient arrays; initialized fields only; sinc uses knots 0, 1 and k; waveform update storage follows the timed coefficient family |
| `t2fit_gw.C` | `t2fit_gw`, `fit_callbacks::{derivative,update,gwb_amplitude}` | Default and standalone GWB dispatch remain distinct; GWECC fit's null callback is an explicit boundary; GW parameter degeneracy is not removed by changing SVD |
| `t2fit_nestlike.C` and TN pre/postfit in `t2fit.C` | `t2fit_nestlike`, `general_fitting::postfit` | Source basis/constraint/noise sums, polynomial projection and SWGP mean; per-call coefficient ownership replaces stale static cache |
| `constraints.C` | `constraints` | Classical/annual/automatic constraints, source weights and DM/CM grid operations; preserve ordering and current-pulsar rules |
| `constraints_param.C`, `constraints_covar.C`, `constraints_nestlike.C` | `constraints::{ConstraintSpecial,build,derivative}`, `constraints_nestlike` | Typed owned prior/weight rows replace void pointers; callback clones retain payload ownership; covariance constraints retain source lower-L convention |
| `TKfit.C`, `TKrobust.C` | `tkfit`, `tkrobust` | Binary64 matrix/data to Rug128 SVD boundary; raw modes, optional outputs, data-only chi-square, augmented dof and source robust covariance/error distinctions |
| `TKsvd.C` | Existing `tksvd` shared by bounded/general solvers | Same raw order, cutoff and arithmetic; no alternate solver, sorting, or precision mode |
| `TKmatrix.C` | `tk_matrix`, existing typed matrix storage | Owned rows/dimensions replace hidden headers; forward substitution and matrix product source accumulation order; native allocation ABI is not exported |
| `cholesky.C` | `cholesky` | Grammar/INCLUDE/PSR/MJD and extra covariance; binary64 kernels/FFT samples; extended subtraction inside decomposition; source diagonal/full determinant conventions |
| `choleskyRoutines.C` | `cholesky_routines`, `cholesky` | Explicit `SpectralFitSession` answers/history replace interactive scanf state; residual/smoothing/interpolation/spectrum/PSD dependency closure |
| `choleskyAutomatic.C` | `cholesky_automatic` | Standalone canonical timed context; explicit output directory; preserve source estimation sequence and files; supplied-covariance branch with unwritten source model values is an error |
| Spectrum/FFT dependencies | `cholesky_spectrum`, `cholesky::fft`, existing toolkit helpers | Dependency closure only; unrelated standalone spectrum/GW APIs remain later scope |
| `bootstrap.C` | `bootstrap` | Owned scratch pulsar, heap result vectors and exact seeded RNG state; retain source partial-copy and observation mutation history; no automatic bootstrap in normal fits |
| `T2accel.C` callers | Existing baseline SVD/fallback controls | Enabled optional native acceleration belongs to AI under 0081; no enabled build is claimed here |

`t2fit*.h`, constraint headers, `TKfit.h`, `TKrobust.h`, `TKmatrix.h` and
Cholesky headers are represented by the routes/types above, not a C-struct ABI.
The final API audit must still enumerate any private standalone body that is not
reachable through those public routes, rather than treating body translation as
public usability. Dynamic plugin loading and full text output remain later work.

## Memory, files and unsafe disposition

The canonical `PulsarContext` owns scientific state. Fit results are inspection
snapshots, and typed constraint payloads survive snapshot ownership changes.
Scratch matrices, covariance inputs and bootstrap storage are owned Rust values.
Source-unwritten state remains `Option::None`; defined nonfinite values retain
their class and sign. Errors preserve already completed source mutations and do
not promise transactional rollback.

No unsafe block was added to the AE numerical/callback modules. Diagnostic
format extensions reuse `clock_text`'s existing bounded, typed libc formatter;
no new FFI or native numerical dependency was added. Diagnostic files use explicit
owned destinations and propagate I/O failures. Consumed covariance/INCLUDE files
carry their paths, bytes and hashes. Final whole-change unsafe review remains due.

## Evidence and limitations

`validation/general_fitting/CONTRACT.md` freezes interfaces and quantity gates;
`GENERAL_FITTING_REPORT.md` retains separate inherited/new failures and incomplete
coverage. Authored `check_*.py` builders retain original/corrected source witnesses
under ignored AE staging. In particular, the newer family cases expose physical
updates, nullable storage, automatic constraints, global GW geometry, chromatic/
solar callbacks, and group/ECORR/SECORR noise through actual PAR/TIM input.

Native floating-point contractions are observations of GCC16 on macOS arm64.
FFT, TN polynomial and covariance contractions use explicit local `mul_add` only
where corresponding native evidence exists. Linux qualification is later work
under 0086. Raw SVD failures and the DDH paired gate under 0087 remain binding.
