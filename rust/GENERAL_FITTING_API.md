# AE routine, header and branch inventory

This inventory accompanies `GENERAL_FITTING_SOURCE_MAP.md`. Names refer to
TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. A public route means the
body is callable through the indicated Rust operation; it does not imply every
parameter combination has a passing numerical comparison. Evidence and open
gates are in `GENERAL_FITTING_REPORT.md`.

## Driver and callback interfaces

| Source bodies / declarations | Canonical Rust route | Branch disposition |
| --- | --- | --- |
| `t2Fit`, `doFitAll`, main NITS loop | `general_fitting::{t2_fit,do_fit_all,fit}` | Independent/local and combined/global systems; MODE 0/1; rescale; full recomputation before/after each cycle; source first-pulsar NITS |
| `t2Fit_getFitData` | `get_fit_data`, `fit_observation_selected` | Deleted rows; SAT/BAT START/FINISH; source endpoint margin; row/input identity; unrestricted endpoint mutation |
| `t2Fit_fillGlobalFitInfo`, `t2Fit_fillFitInfo`, `t2Fit_fillFitInfo_INNER`, `t2fit_fillOneParameterFitInfo` | `global_fit_columns`, `fill_fit_columns`, `fill_constraint_info`; internal `parameter_columns` | Global definitions from pulsar zero; local ZERO/jitter/jumps/parameters/globals; vector expansion; source event exclusions; TN temporary parameter enable/disable |
| `getDerivFunction`, `t2Fit_getParamDeriv`, built-in `getParamDeriv` | `fit_callbacks::derivative`; installed `FitInfo` callbacks | Source family selection. Missing/unimplemented callbacks return explicit errors; unknown fitted labels retain source log-and-omit behavior. No fabricated function pointer |
| `t2Fit_buildDesignMatrix`, `t2Fit_buildConstraintsMatrix`, `t2Fit_updateParameters`, `t2Fit_getParamMatrixRow` | `design_matrix_row`, `constraint_matrix_row`, `update_parameters`, `get_param_matrix_row` | Installed canonical callbacks and owned constraint payloads; local then global updates; global coordinates match each pulsar |
| `t2fit_prefit`, `t2fit_postfit`, `subtractTNPoly`, `subtractSWGPMeanSW` | `prefit`, `postfit`, internal `project_noise` | Eight noise accumulators; separate raw callback and postfit state; TN polynomial transfer; SWGP weighted mean transfer and optional files |
| `callFitFuncPlugin` | Explicit `do_fit_all` plugin boundary | Dynamic plugin ABI and host integration remain later scope |

All callback families below are reached by `fit_callbacks::{derivative,update}`
unless a separate public route is named. Source helper/dispatch distinctions
are retained; exposing a standalone body does not add a new default fit label.

| Source file and bodies | Route / interpretation | Evidence class |
| --- | --- | --- |
| `t2fit_stdFitFuncs`: zero, stdFreq, binaryModels, planet, stdDm, stdCm, simpleAdd, simpleMinus, telPos_delta, telPos, jump, fdjump and notImplemented derivative/update bodies | Source ZERO/spin/jump/DM/CM/telescope/planet dispatch; binary bodies call accepted `binary` APIs; spin update shared with bounded adapter | Public workflows; scalar callback grid; inherited E/G/AD binary cases; source correction witnesses |
| `t2fit_position`: stdPosition derivative/update | RAJ/DECJ, proper motions and second derivatives, PX, PMRV, DSHK with source angular/time units | Public RAJ/DECJ; direct scalar grid |
| `t2fit_glitch`: stdGlitch derivative/update, expdip, gausdip | Default update selection preserved; standalone `std_glitch_update` exposes the non-default body | Natural event/glitch fits and scalar grid; standalone update fields; GLEP source assertion |
| `t2fit_dmother`: dmx, chromx, fddc, fd, dmsinusoids | Frequency and interval selectors; FDDI remains source-not-implemented | Public dispersion/FD fits and scalar grid; FDDI source exit |
| `t2fit_dmmodel`: dmmodelDM/dmmodelCM derivative/update | Source concatenated DM then CM coefficient index | Natural DM/CM component/grid and legacy examples |
| `t2fit_fitwaves`: fitwaves derivative/update | Separate achromatic/DM harmonic counts and WAVE_SCALE 0/1/2 storage | Public fits; original/corrected count/storage witnesses |
| `t2fit_ifunc`: sifunc, ifunc, update_ifunc | IFUNC/clock/quadrupolar interpolation; standalone `sinc_derivative` | Natural constrained/unconstrained fits; standalone all three sinc families; clock-offset source assertion |
| `t2fit_gw`: gwm_amp, gwb_amp, gwcs_amp, quad_om and gwsingle derivatives; quad_om/gwsingle updates | Standalone `gwb_amplitude`; GWECC null fitting callback is an explicit boundary | Single/global natural fits, standalone GWB, pre-event source witness; null-callback assertion |
| `t2fit_solarwind`: ne_sw_sin, ne_sw, ne_sw_ifunc derivatives; ne_sw/ne_sw_ifunc updates; constraint_ne_sw_ifunc_function | Source scalar/sinusoidal/interpolated solar wind; typed `ConstraintSpecial::Error` | Natural fits, SWGP mean transfer, sigma/sinusoidal constraint payloads |
| `t2fit_nestlike`: legendre, computePolySubtractCoeffs; red/red_dm/red_chrom/swgp/jitter/band/group/shape_red/shape_dm derivatives and their updates | `t2fit_nestlike::{legendre,polynomial_coefficients,derivative,update,update_metadata}` and public dispatcher | Natural TN/ECORR/SECORR/shapelet fits, context-cache witness; source polynomial contractions |
| `t2fit_nestlike.h` clamp, encode/decode flags/orders and set-order inline bodies | Public source-named functions in `t2fit_nestlike` | Reused canonical bitfield implementation; invalid shifts explicit |

The telescope polynomial and mode -1 grid uses terrestrial PKS inputs; its
non-STL zero selector is covered, but a nonzero STL ephemeris branch is not
claimed by that grid.

## Constraints and numerical toolkit

| Source bodies | Rust route | Branch disposition |
| --- | --- | --- |
| Both `get_constraint_name` overloads | `Constraint::from_index`, `constraints::name` | Source names and unknown index boundary |
| `computeConstraintWeights`, `matrixDMConstraintWeights`, `autosetDMCM`, `autoConstraints` | `compute_weights`, `matrix_dm_weights`, `auto_set_dm_cm`, `auto_constraints` | Baseline uniform-weight build; matrix-derived standalone weights; fixed/free CM grid and automatic constraint selection |
| `consFunc_dmmodel_mean`, `dm1`, `cw`, `cw_year`; `consFunc_tel_dx/dy/dz`; `consFunc_ifunc_X0`, `ifunc`, `ifunc_year`; `consFunc_qifunc_p_year/c_year`, `quad_ifunc_p/c` | `constraints::standard_constraint` and its source branches | Polynomial, annual and astrometric factors; canonical knot weights and epochs |
| `getConstraintDeriv`, `standardConstraintFunctions`, `CONSTRAINTfuncs` | `constraints::{derivative,standard_constraint,legacy_constraint}` | Modern typed payload and legacy enumeration remain distinct |
| `constraint_param_function`, `constraints_covar_ifunc`, `constraints_covar_dmmodel_dm/cm`, `constraint_ne_sw_ifunc_function` | `ConstraintSpecial::{Parameter,CovarianceRow,DmCovarianceRow,CmCovarianceRow,Error}` through `derivative` | DM/CM covariance callbacks are standalone source bodies, absent from default selector construction; owned rows preserve clone lifetime |
| `constraints_nestlike_red`, `red_dm`, `red_chrom`, `swgp`, `jitter`, `band`, `group` | `constraints::derivative` | Harmonic/logarithmic prior coordinates; source noise amplitudes/index/band/group/epoch membership |
| `qp_term_cutoff` | `constraints::qp_term_cutoff` | Separate cutoff-QP prior helper, distinct from covariance `qp_term` |
| `TKleastSquares`, `TKconstrainedLeastSquares`, `TKrobustLeastSquares`, `TKrobustConstrainedLeastSquares`, `TKrobustDefConstrainedLeastSquares`, `TKrobust` | `tkfit::{tk_least_squares,constrained_least_squares,least_squares_outputs}`, `tkrobust::{least_squares,robust_least_squares}` | Zero/nonzero targets, independent optional outputs, rectangular/degenerate cases, H/B/W/fallback, rescaling, robust covariance/error conventions |
| `TKrobust_reweight_huber/bisquare/welsch` | `tkrobust::reweight` | In-place source weight/residual order and Welsch formula retained |
| `TKleastSquares_svd`, `_noErr`, `TKfitPoly`, `TKfindPoly_d`, `TKremovePoly_d/f`, `TKfindMax_Ld` | `tkfit::{fit_basis,polynomial_basis,fit_polynomial,remove_polynomial,remove_polynomial_f32,find_max}` | Source fixed wrapper cutoff, binary32 scratch path, signed ties/NaN comparisons |
| `TKleastSquares_single_pulsar`, `_global_pulsar`, `TKfit_getPulsarDesignMatrix`, `TKleastSquares_svd_psr_dcm`, `_psr`, `_passN` | `legacy_fitting::{single_pulsar,global_pulsar,design_matrix,svd_psr_dcm,svd_psr,svd_pass_n}` | Legacy diagonal multiplication; corrected global constraint layout; passN source-abandoned path returns explicit error |
| `TKsingularValueDecomposition_lsq`, `TKbacksubstitution_svd`, `TKbidiagonal`, `TKpythag` | `tksvd::{decompose,bidiagonal,pythag}`, `Decomposition::back_substitute` | Same Rug128 engine as all fits; raw mode order; extended RHS; caller-zeroed modes; source unused output pointer is not exposed |
| `TKforwardSubVec`, `TKforwardSub`, `TKmultMatrix`, `_sq`, `TKmultMatrixVec`, `_sq` | `tk_matrix::{forward_sub_vec,forward_sub,mult_matrix,mult_matrix_vec}` | Square wrappers use the same rectangular operations; source accumulation order |
| `malloc_blas`, `malloc_uinv`, `get_blas_rows/cols`, `free_blas/uinv`, `malloc/free_2dLL`, `malloc/free_2df` | `BlasMatrix` methods and owned `Vec<Vec<FitNumber/f32>>` | Dimensions/Drop replace pointer headers and native allocation ABI; no duplicate allocator ABI |

## Covariance, spectrum and bootstrap

| Source bodies | Public Rust route | Branch disposition |
| --- | --- | --- |
| `getCholeskyMatrix`, `getCholeskyDiagonals`, `cholesky_formL/Linv`, `T2cholDecomposition` | `cholesky::{get_cholesky_matrix,diagonals,form_l,form_linv,decompose}` | Diagonal/full paths; source determinant offset; baseline nonpositive-pivot error. LAPACK return-code diagonal retry belongs to enabled acceleration |
| `cholesky_readT2CholModel`, `_R`, `cholesky_readT2Model1/2`, `cholesky_readFromCovarianceFunction`, `cholesky_ecm` | `read_model`, `read_covariance_function`, `read_extra_covariance` | File/inline/PSRJ, recursive INCLUDE, PSR/MJD/CLEAR/END, legacy numeric-prefix/hex scanner, DCF/ECM. Source extra-ECM constraint capacity error is explicit |
| `cholesky_covarFunc2matrix`, `addCovar`, `cholesky_cov2dm` | `covar_function_to_matrix`, `add_covariance`, `covariance_to_dm` | Daily interpolation, endpoint extension, masks, zero constraint covariance and frequency scaling |
| `cholesky_dmModelCovarParam`, `cholesky_dmModel`, `cholesky_sqexp_kernel`, `_matern15_kernel`, `_matern25_kernel` | `dm_covariance_parameters`, `dm_covariance`, `squared_exponential`, `matern15`, `matern25` plus common matrix interpolation | Every selector represented; MODEL DM retains its distinct scaling |
| `cholesky_powerlawModel`, `_withBeta`, `_powerlawQPModel`, `_powerlawRedPinkModel`; `T2calculateCovarFunc`, `_with_QP`, `_redpink`, `qp_term` | `powerlaw_covariance`, `qp_term` and common interpolation | Shape discriminator selects the source PSD body; corrected imaginary DC initialization; fixed source FFT normalization |
| `T2obtainTimingResiduals`, `T2getWhiteNoiseLevel`, `T2cubicFit`, `T2findSmoothCurve`, `T2interpolate`, `T2getHighFreqRes`, `T2guess_vals`, `T2fitSpectra`, `T2calculateCholesky`, `T2getWhiteRes`, `T2calculateDailyCovariance` | `cholesky_routines::{timing_residuals,white_noise_level,cubic_fit,smooth_curve,interpolate,high_frequency_residuals,guess_spectrum,fit_spectrum,calculate_cholesky,daily_covariance}`, `SpectralFitSession::fit`, `tk_matrix::forward_sub_vec` | Explicit answers/session replace stdin/static choice history; source weighted/unweighted rules; cutoff failure and subsequent valid call; optional beta/amplitude |
| `T2calculateSpectra`, `calcSpectra`, `calcSpectraErr`, `calcSpectraErr_complex`, `TKlomb_d`, `TK_weightLS`, `TK_fft` | `cholesky_spectrum::{periodogram,calculate,lomb,weighted_lomb}`, `cholesky::fft` | Required spectrum closure, including prewhitening's binary32 first difference, source reported-count convention and complex/error outputs |
| `T2get_covFunc_automatic`, `T2writeCovarFuncModel` | `cholesky_automatic::{automatic,write_model}` | Canonical residual input and explicit output directory; numeric and exact file comparison. Supplied-covariance branch reads unwritten native model controls and returns an explicit boundary |
| `bootstrap`, `random` | `bootstrap::bootstrap`, `BootstrapRng::random` | Exact seed/history; caller pulsar and scratch ownership; source partial copy and pulsar-zero output; 0/1/4096 fits and over-capacity boundary; failure cleanup |

The alternate `T2calculateCovarFunc(..., fitVar)` FFTW implementation is inside
a source comment, not a compiled overload. No API is invented for it.

The remaining standalone `TKspectrum` preprocessing/plotting interface,
`TK_fitSinusoids`, `TK_dft`, point averaging/boxcar/Hann, monotonic/spline helpers,
`TKcalcSigmaz` and its `fit4/mat20/getprtj/getweights/readin/indexx8` closure,
and unrelated toolkit/GW functions remain AG's scope. Required sorting, median
and first-difference operations are included locally in the released dependency
closure; this does not claim the whole toolkit was delivered by AE.

## Optional builds and ownership

`T2accel` declares `accel_cholfac`, `accel_lsq_qr`, `accel_multMatrixVec` and
`accel_multMatrix` under LAPACK/BLAS macros. AE implements the reference SVD and
nonaccelerated Cholesky/matrix paths and the source unavailable-acceleration
fallback. Enabled native wrappers/build/ABI qualification is AI under 0081.
No accelerated profile or Linux qualification is claimed.

The relevant t2fit, constraint, TKfit/TKrobust/TKsvd/TKmatrix and Cholesky headers
map to these Rust APIs/types. Native `pulsar`, `FitInfo`, `void*`, contiguous
matrix headers, allocator ownership and plugin function-pointer ABI are not a
core C-struct compatibility guarantee under 0084. Canonical state owns callbacks,
constraint rows, matrices and scratch pulsars; unwritten fields stay explicit.
No new unsafe block or native numerical dependency is introduced by AE.
