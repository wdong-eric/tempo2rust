# AF interface and comparison freeze

Frozen before AF numerical comparisons, 26 October 2026, on release a002e6f
(dispatch 480ef2d), AE ddff7f8 / ecf41ed. Source fda6ad4. Decisions
0076/0088 govern scope; 0053 governs demonstrated corrections. This is an
implementation contract, not central acceptance or a change to existing gates.

## Dependency-ordered work

1. This contract and authored examples; source and quantity mapping.
2. `predictor_number`, `cheby2d`, `t1polyco`, `tempo2pred`: owned numeric
   values, callback polynomials, models/sets, grammars, metadata, selection, plans.
3. `cheby2d_int`, `polyco`, `predictor`: canonical PAR-only preparation,
   synthesized observations, shared BAT/residual calls and source file products.
4. CLI, direct consumers, native component and generation witnesses, exchange,
   independent approximation and failure/recovery histories in this directory.
5. Debug/release, affected regressions, formatting/Clippy, source/departure maps
   and report; explicit-path Conventional Commits.

## Public API freeze

`PredictorNumber` owns ordinary native-long-double arithmetic, distinct from
Rug128 timing `Scalar`/`Float`. Its compile-target representation is 53 significand
bits on macOS arm64 and 64 on Linux x86_64 (x87 extended); no runtime switch.
Linux native/ABI qualification remains AJ/AL and is not implied by implementation.
Source explicit double conversions stay binary64 on both targets. The quadmath
cosl/powl macro result is rounded at its actual native assignment boundary.

`Cheby2D::{new,from_coefficients,construct,evaluate,x_derivative,test}` use
native values and callback slices in y-major/x-minor order. Dimensions and owned
coefficients are validated on construction. `ChebyModel` retains metadata,
ranges, dispersion constant, phase and derivative polynomials. `ChebyModelSet`
owns ordered segments and exposes nearest-index, insert, keep, read/write.
Clone/Drop replace defined deep-copy/destruction. Source aliases that destroy
their own storage are corrected through safe ownership, not invalid reads.

`T1Polyco` exposes source metadata, coefficients, phase/frequency, fixed/new
readers and fixed writer. `T1PolycoSet` retains nearest-midpoint extrapolation
and successful-prefix reading, with trailing-input diagnostics. Generic
`T2Predictor::{read,read_into,write,evaluate,metadata,insert,copy_from,keep,plan}`
dispatches through those bodies. Empty/none backing is an error; Cheby no-match
is an explicit out-of-range result with source -1 phase/frequency. T1 insert,
keep and source Copy remain errors because the source does not implement them.
`get_plan`/`get_plan_ext` read owned initialized predictors and share `plan`.
File results retain bytes, SHA-256, format and warnings; no runtime is required.

`predictor::load_par` initializes one canonical pulsar, reads PAR and runs
preprocessing without reading TIM. `cli_defaults` applies PEPOCH/1400/NULL-site
rules. Direct Cheby set construction applies start/start-frequency/empty-site
defaults. `cheby2d_int::{cheby_model_func,construct_model,test_model,
construct_set,test_set}` mutate canonical context and runtime. `polyco::{polyco,
atimfake,tz_fit,chebpc,pcshft}` preserve source sampling and timing arithmetic;
direct polyco has explicit true-DM and filename-prefix arguments. Timing calls
are formBatsAll then formResiduals(removeMean=false), never general fitting.

CLI routes are exactly those in 0076. Generation writes `t2pred.dat`/`pred.tim`
or `polyco_new.dat`/`newpolyco.dat`/`polyco.tim` beneath the explicit output
directory. A JSON success is emitted only after all required output completes.
Generation diagnostics include requested/effective dimensions, approximation
statistics, consumed PAR/resources and retained source warnings.

## Frozen quantities and comparisons

All comparison receipts check membership, shape, units, kind, segment/order,
count, status, signed zero and nonfinite classes before finite tolerances.
No sorting of segments/modes. Mutations of coefficient, units, selection and
plan count must be rejected. Original and corrected-native records are separate.

* Canonical engine on identical observations: existing 1e-12 s epoch/delay and
  1e-10 turn phase gates remain unchanged. Generated grids also compare epochs
  at 1e-12 s; a changed grid is reported separately from engine disagreement.
* Native arithmetic components: epsilon = 2^(1-p), p = target significand bits.
  The bound for a coefficient (turns) is `16*epsilon*nx*ny*S`, where S is the
  maximum absolute callback phase, including cancellation scale. Derivative
  coefficient bound is this times `2*nx*nx` (turn per normalized x).
  These accumulation bounds apply only to identical callback inputs. An exactly
  zero callback has exact zero coefficients; analytically zero coefficients
  from nonzero callbacks retain the stated cancellation bound.
* Predictor phase (turns): `1e-10 + 32*epsilon*A`, A the sum of absolute
  evaluated polynomial terms plus reference/linear/dispersion terms. Frequency
  (Hz): `1e-12 + 32*epsilon*B`, B the absolute derivative-term sum in Hz.
  Generated coefficients are additionally checked by their maximum phase
  contribution over the declared span, using the same phase gate. A failed
  coefficient gate is not waived by a passing sampled evaluation.
* Plans: exact ceiling/count/endpoints/selection; phase0 uses the phase gate.
  Average-Hz bound propagates the two endpoint phase bounds divided by the
  actual interval seconds, plus `8*epsilon*abs(reference average Hz)`.
* Serialization: check field order/dimensions and parsed rounding cells at
  each source precision (Cheby 34/25 significant digits; legacy/new polyco
  field-specific precision in polyco.C/t1polyco.c). Constructed versus reread
  differences are reported separately. Do not add format loss to engine gates.
* Approximation: independent non-node time/frequency grids and overlap points;
  report unwrapped phase, Hz, source RMS/MAV and phase/F0 seconds for both engines.
  Authored isolated and binary examples target 1 microsecond maximum phase/F0
  error at the requested span/order; failures remain failures. This target is
  an approximation diagnostic, never a Rust/native fidelity allowance.

## Inherited unresolved ledger

P: 154 field/event differences, including 32 SIMUL; M2: four raw-order failures;
0049: four quotient/rounding boundaries and three unconstrained interpolation
inputs reaching that boundary near 2.5e12 s. AE red: six raw-mode failures over
two cycles at rank nine. Chromatic-log: 16 reconstructed errors plus one
multiplier, ratio about 1.05e26. ELL1H A1/EPS1/EPS2/TASC/H3/PB: 19 whitened-data
differences. Single-pulsar quadrupole/GW single/memory/cosmic-string:
770/750/448/468 failures; supported global cases are separate. DDH: historical
1 ns ideal passes, current/historical fidelity retains 401/433 failures;
shared solves retain two raw-order differences. Two TNsubtractSWGP=1 first
passes consume unwritten SWGPSignal. Exact identities remain in immutable AE
index 971f96f9ca9c8aafb519756235343d8bb369ec24a8842edf4f174797cfe6a7c9 and
0088. None is accepted or changed by AF.
