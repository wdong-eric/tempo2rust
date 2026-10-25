# AE general fitting implementation checkpoint

AE remains active under 0079. **This is not the completed handover or numerical
acceptance report.** The remaining implementation and validation inventory below
is still required before central's 0074/0082 review. Updated 26 October 2026.

## Executable capability

`timing --fit` now reaches canonical local/global fitting through real PAR/TIM
loading, preprocessing, BATs and residuals. `general_fitting::fit` performs NITS;
`t2_fit` and `do_fit_all` operate on prepared canonical state. The bounded spin
adapter shares the generalized SVD and spin updates. General output remains
`tempo2-general-fit-v1`; see `validation/general_fitting/CONTRACT.md` and
`EXAMPLES.md`.

Implemented components include source column/row selection, binary and remaining
callback families, constraints and targets, typed constraint payload ownership,
H/B/W robust fitting, covariance model grammar and lower-triangular whitening,
TN bases/projections, polynomial fitting helpers, and standalone DM/CM weights
and automatic grids. Standalone bootstrap/RNG, automatic covariance estimation,
spectrum/FFT dependencies, single-pulsar legacy adapters and independently optional
solver outputs are now implemented. Their presence is not evidence of complete
branch coverage.

## Current evidence

The twelve initial release public workflows checked spin fitting, independent
pulsars, shared globals with unequal rows/local columns, a nonzero parameter
prior, covariance weighting, H/B/W, and red/DM/chromatic/band noise. Eleven pass
the exported quantities; red has the six raw singular-value failures below.
Debug produced the same physical results; its initial independent-pulsar
comparison incorrectly compared raw sigma with MODE-0 effective unit weights.
The comparator now distinguishes these fields. Final runs of both profiles are
still required after the subsequent ownership and helper edits.

The comparator enforces the frozen scaled gates, native raw mode order, rank,
selected rows, local/global output coordinates, constraints/targets, increments,
raw covariance, reported errors, spin updates, chi-square, noise arrays and
recomputed residual/phase/BBAT. It explicitly lists unchecked quantities:
physical updates outside spin, general-workflow kernel/Cholesky exports, full
robust schema integration and truth recovery. Standalone robust histories and
selected optional files now have separate native comparisons below. Passing these cases is not full
AE acceptance or a truth-recovery result.

The direct `fitting_components` example loads/times a real DMMODEL PAR/TIM pair.
Native comparisons pass for matrix-derived weights, automatic DM/CM grids, and
fixed-CM grid behavior in debug. The standalone interpolation callback exposed
a nested diagnostics mutex acquisition; the new callbacks now pass their
existing diagnostics borrow into the shared interpolation basis.

Focused Rust checks now pass 65 library tests (34 explicitly ignored). The
runtime-dependent constraint snapshot test was separately run and passed. It
exercises parameter-prior and TN callbacks after cloning the canonical context
and dropping the original. Clippy passed after the covariance implementation;
both profiles and complete affected regressions remain pending.

Bootstrap's debug example loads a real PAR/TIM pair with IBOOT 2 and performs
four resampled fits. Native comparisons pass for sample membership, fitted F0
history, final errors and exact `testout.tim` bytes. The RNG matches all 3,000
binary64 values checked across seeds -999, -1 and +1. Original bootstrap stack
failure and corrected heap ownership are recorded separately; broader capacity,
copy and state-history evidence is still required.

The debug automatic-covariance example passes all exported controls, covariance
samples and Cholesky entries, plus exact bytes of `comp.dat`, `compchol.dat`,
`cholSpec.dat` and `automatic.dcf`. The first run had 180 covariance-file lines
differing in final printed digits despite passing numerical gates. The archive's
FFT disassembly showed four contracted operations; explicit Rust `mul_add`
matched every binary64 output for both directions at sizes 2, 8, 128 and 1024,
and closed the file discrepancy. The pre-correction outputs remain under
`build/automatic/pre-fft-contraction/`; current receipts are `build/fft/debug.json`
and `build/automatic/debug/receipt.json`. The supplied `.cov` branch of automatic
estimation reads uninitialized source model values and remains an explicit error
boundary, with no invented oracle.

The comparator now rejects 34 focused mutations and checks exported native
MODE/ROBUST/NITS/rescale controls, declared shapes, exact membership, numeric
classes/signs, spin update metadata and fit-coordinate updates. Central's separate
recheck closes its four identified false passes only; final schema and quantity
coverage review remains required. Fresh debug spin, independent, prior and DM-log
cases pass; chromatic-log has the separate findings below.

The shared-input solver contract now passes 42 debug cases: all eight optional
parameter/error/covariance combinations with present/absent data and rescaling
on/off, robust H/B/W and unsupported-code fallback histories, and optional output
files. Native masks and files are exact; each scale/final solve records matrices,
data, residuals, scales, phi/dphi, raw modes and fitted outputs. General CLI
`--fit-diagnostics DIR` now emits the source mask-7 files in an existing directory;
its API also exposes the mask and debug correlation matrix. Natural spin, global,
robust-H and red file checks pass byte-for-byte (red's six numerical raw-order
findings remain). DM/chromatic files pass with explicitly supplied DM1/DM2. Without
those supplied values, their source metadata callback reads unwritten prefit
storage; Rust returns an explicit boundary instead of inventing zeros. This is a
new optional-output limitation, not a failure of the default numerical fit.

`check_source_boundaries.py` establishes two safe original/corrected witnesses.
On naturally loaded 16-row timing state with MAX_OBSN=20, the original automatic
covariance buffers reserve 80 bytes and the eleventh double would overflow;
corrected buffers reserve 160 bytes and all 16 rows fit. Instrumentation stops
before the invalid write. The constant-only TN projection likewise reaches the
solver with zero of 16 white-data entries initialized; instrumentation stops
before the invalid read. The corrected unused-solve omission preserves the
fitted-F0 neighbor exactly and restores writeResiduals=7.

## New numerical ledger, separate from inherited M2

Case `red`, pulsar 0, cycles 1 and 2, has rank 9 in both engines with every mode
retained. The raw indices 4, 5 and 6 are permuted. No modes are sorted, no gate is
relaxed and no alternate precision mode is introduced. Fit increments,
covariance, reported errors, physical spin updates, chi-square, noise projections
and final timing checks pass their stated gates in this case.

| Cycle | Raw index | Native value | Rust value |
| --- | --- | --- | --- |
| 1 | 4 | `1582228.8481050432247550208302007491` | `64628148.00393346917465173728681352563465` |
| 1 | 5 | `64628148.0039334691746517372868135433` | `64615519.18737586179490550319009043125306` |
| 1 | 6 | `64615519.18737586179490550319008748` | `1582228.848105043224755020830200748660248` |
| 2 | 4 | `1582228.84810504319747018527259814042` | `64628148.00393347071752585680509107856768` |
| 2 | 5 | `64628148.0039334707175258568050910579` | `64615519.18737585020845599598349834618584` |
| 2 | 6 | `64615519.1873758502084559959835035022` | `1582228.848105043197470185272598140818129` |

Selected release red-case maxima (absolute differences, not acceptance budgets):

- `postfit.residual`: `9.77805383323903252612E-26`.
- `postfit.phase`: `8.484236E-25`.
- `matrix`: `2.500E-30`.
- `white_matrix`: `5.000E-29`.
- `covariance`: `2.77555756156289135103250E-17`.
- `physical_spin`: `2.0521581312953236007201022632342592640742490196896E-25`.

Recorded red-case executable hashes:

- Rust: `df22c211a96f07ad780a165118ff557e98cc9b240e673a297228e3c6275b6994`.
- Native: `6f1ec139751cd525e87704ee3a3544ce7f84c25c747f3c7b19c8661e7cfa627f`.
- Comparison: `7416c5555137e3e079344ff83e2d3c9783386a7da3afc2e0acb1d63512cb2e3d`.

Exact per-case identities and all values are under ignored
`validation/general_fitting/build/public/{debug,release}/CASE/`. These hashes
identify the tested binaries; later edits are not retroactively covered.

**Recommendation:** retain these six checks as a new open raw-order ledger under
0074 while finishing AE's independent scope. Central's 0082 checkpoint should
keep them distinct from the four historical M2 checks. No new precision diagnosis
or numerical-policy change is needed to continue implementation.

### Chromatic logarithmic noise

The debug `chromatic-log-noise` case has 16 reconstructed-noise error differences above the
1 ps gate. Its design matrix, whitened matrix/data and constraints are identical
binary64 inputs in both engines. The standalone shared-input solver reproduces
the native chi-square `23584532.5236933045089244842529296875` versus Rust
`23584532.52370450273156166076660156250000`. Raw singular extremes are
23551.616110315696 and 2.465271177816473e30 (ratio about 1.05e26). This localizes
the discrepancy to the ill-conditioned solver, without proving a precision cause
or changing Rug128. Preserve it separately from the red raw-order ledger. The
reproducible shared-input receipt builder and final quantity assessment remain
pending; current files are `build/chrom-log-solver-{input.txt,native.txt,rust.json}`.

## Focused source corrections

Each correction preserves separate original/corrected native observations in
AE-owned staging. The canonical source and accepted reference archive are unchanged.

| Source defect | Witness / correction |
| --- | --- |
| SVD ignores nonzero constraint RHS | `check_solver.py`: target 4 gives original 0 versus corrected 8/7; target-zero neighbor unchanged. |
| TKrobustLeastSquares discards its robust code | Same probe: W wrapper originally returns 34.333333333333336 versus delegated/corrected 4.8784378310184371. |
| Covariance sort follows row capture | `order_probe.cc`: three wrong pairings originally, none when sorted before capture; sorted neighbor unchanged. Reproducible receipt builder still pending. |
| Polynomial cache omits interior BBAT/frequency dependencies | `check_cache.py`: changed interior epochs and frequency match fresh calls after removing stale shared storage. |
| Planet callback uses pulsar zero | `check_callbacks.py`: two-pulsar 10/10 becomes 10/40. |
| WAVDM uses achromatic harmonic count | `check_callbacks.py`: independent DM harmonic count controls modulo. |
| IFUNC covariance uses ifuncN as a column | Safe padded-row witness in `check_callbacks.py`: original sentinel 99/99 becomes weights 3/4. |
| DM/chromatic logarithmic priors repeat sine | `check_fitinfo.py`: original 57/57 and 55/55 become 57/58 and 55/56; harmonic neighbors unchanged. |
| IFUNC expanded rows read another configured constraint label | `check_fitinfo.py` includes a real second parameter constraint and checks repeated IFUNC labels. |
| Bootstrap places a 128 MiB result array on the stack | Source fails before its first statement on the default stack; corrected native uses heap storage and Rust owns vectors. Final original/corrected capacity receipt remains pending. |
| copyPSR reads reserved jump/FDJUMP slot zero | `check_copy.py` leaves reserved destination slots untouched and preserves active-slot values. Rust copies active one-based slots. |
| copyParam evaluates unused unwritten prefit cells | Rust clones initialized/unwritten ownership; native correction copies representation bytes. Focused initialized/unwritten Rust neighbor passes; complete affected lifecycle regression comparison remains required. |
| Constant-only TN projection consumes unwritten white data | `check_source_boundaries.py` observes zero initialized entries and stops before the solve; corrected omission leaves the fitted neighbor unchanged. |
| Automatic covariance allocates double buffers with sizeof(float) | `check_source_boundaries.py` stops before original write 10 beyond 80 bytes; the corrected 160-byte buffer admits all 16 natural rows. |

Source numerical conventions such as Welsch's expression, diagonal/full detL,
source IFUNC covariance's use of lower L, and band/group error accumulation are
retained. GCC16 arm64 fused arithmetic in TN polynomial projection is explicitly
represented; the unfused native diagnostic reproduced the earlier Rust matrix
and localized a 24 ps downstream discrepancy. A reproducible contraction witness
builder/receipt remains to be authored before final delivery.

## Remaining required work

1. Complete the legacy global adapter and remaining optional covariance/output
   paths; extend the passing pointer/file/robust checks to boundary and release
   profiles and integrate their schema checks.
2. Complete bootstrap capacity/copy/failure histories and release comparisons;
   preserve the passing debug RNG, resampling and writeTim observations.
3. Complete automatic covariance helper/public API coverage and interactive
   spectrum state; run release comparisons.
4. Finish callback/constraint/noise family and history coverage: binary/astrometry,
   events/jumps/interpolation, solar wind, group noise, ECORR/SECORR, shapelets,
   spectral variants, automatic constraints, full covariance selectors/masking,
   nonfinite/error/recovery and optional output files.
5. Finish the bootstrap stack/copy and contraction/order witness receipts, and
   investigate any further demonstrated local defects under 0053 without using
   undefined native memory as a numerical oracle.
6. Independent identical-input solver checks for the new workflows, complete
   observable/schema checks, debug/release public/component runs, affected
   AC/AD/M1/M2 regressions, formatting/Clippy, and final source/unsafe maps.
7. Rewrite this checkpoint as the final report with exact delivered commit and
   evidence identities, complete remaining source inventory and 0082 assessment.

Inherited open gates remain P 154 including SIMUL 32; the four historical M2 raw
mode-order checks; four 0049 utility boundaries; and original DDH 1 ns ideal-fixture
gate. None is reclassified by this implementation. Enabled native acceleration
is assigned to AI under 0081; baseline SVD/fallback behavior remains AE's task.
Decision 0084 records the user's selected full Rust core/toolkit APIs plus
standalone predictor C compatibility. General core/toolkit C-structure ABI
compatibility is outside that guarantee. AE's scientific and Rust API scope is
unchanged; typed payload ownership makes no native ABI-layout claim.

## Qualified profile and later platform work

Current AE native observations use macOS arm64 and GCC/G++ 16 with the pinned
reference archive. Timing `longdouble` is `__float128`; ordinary C/C++ `long double`
on this host has binary64 precision. Rust retains Rug128 for timing operations
and explicit source binary64/binary32 boundaries. The FFT and TN polynomial
contractions are observed compiler arithmetic on this profile, not assumed
portable properties. Decision 0086 selects macOS arm64 and Linux x86_64 for the
first release; actual Linux toolchain/native execution belongs to later platform
qualification and is not claimed by this AE checkpoint.
