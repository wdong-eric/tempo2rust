# AE general fitting implementation checkpoint

AE remains active under 0079. **This is not the completed handover or numerical
acceptance report.** The remaining implementation and validation inventory below
is still required before central's 0074/0082 review. Updated 26 October 2026.
Implementation checkpoints are `8a16880` and `f75eb13`; subsequent work below
remains in progress.

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

The latest component campaign passes in debug and release: three DM/CM grid
cases, 42 solver/output/robust cases, ten kernel cases, 27 covariance-selector
cases, five spectral-session scenarios, the four-fit bootstrap plus 3,000 RNG
values, automatic covariance, seven legacy fits, 59 standalone callback fields
and six natural diagnostic-file cases. Numerical failures remain separate from
file checks. Receipts are under `build/component-campaign/` and each component's
profile directory. The extended solver observer also checks unscaled errors,
error multipliers and linearized residuals; all 42 cases pass both profiles.

The 58 public family recipes have matching debug/release outcomes: 43 pass,
six ELL1H cases have 19 whitened-data failures, four single-pulsar GW cases fail
in their near-null systems, and five stop at explicit source boundaries. Three
boundaries are the known unconstrained interpolation quotient overflow; two
are first-residual reads of unwritten SWGPSignal when TNsubtractSWGP=1. Separate
SWGP cases without that subtraction pass, including the mean transfer into
NE_SW. DM/CM polynomial transfer cases also pass. New postfit_value/error fields
record canonical state after these transfers, separately from immediate callback
updates. Comparator controls reject 49 mutants.

The affected compatibility runner uses accepted comparators with output paths
redirected into AE staging. M1a/M1b, resources and AC lifecycle pass both
profiles. All 92 selected AD comparisons pass (23 cases, both mean conventions,
both profiles). M2 retains exactly four historical raw-order failures across the
two profiles. Receipt: `build/regressions/receipt.json`.

Release DDH checks reproduce debug outcomes: the versioned historical GPS table
passes all 284 pre/postfit 1 ns ideal checks, while current/historical fidelity
reports retain 388/422 failures. Identical-input solver replay still has only
two raw-mode failures per DDH input owner. The new error-multiplier observable
adds one chromatic-log failure per input owner (difference about 3.08e-10,
relative gate 5e-14); it is not hidden by the passing chi-square gate. Exact
same-engine replay still passes. Receipts: `build/ddh/release.json` and
`build/pipeline-solvers/release.json`.


The twelve initial release public workflows checked spin fitting, independent
pulsars, shared globals with unequal rows/local columns, a nonzero parameter
prior, covariance weighting, H/B/W, and red/DM/chromatic/band noise. Eleven pass
the exported quantities; red has the six raw singular-value failures below.
Debug produced the same physical results; its initial independent-pulsar
comparison incorrectly compared raw sigma with MODE-0 effective unit weights.
The comparator now distinguishes these fields. The current release run of all 14 cases reproduces the six red raw-mode
failures and 16 chromatic-log noise-error failures; the other 12 pass. Further
API edits will require their affected checks.

The comparator enforces the frozen scaled gates, native raw mode order, rank,
selected rows, local/global output coordinates, constraints/targets, increments,
raw covariance, reported errors, spin updates, chi-square, noise arrays and
recomputed residual/phase/BBAT. It explicitly lists unchecked quantities:
physical unit mappings not yet exported, general-workflow kernel/Cholesky exports, full
robust schema integration and truth recovery. Standalone robust histories and
selected optional files now have separate native comparisons below. Passing these cases is not full
AE acceptance or a truth-recovery result.

The direct `fitting_components` example loads/times a real DMMODEL PAR/TIM pair.
Native comparisons pass for matrix-derived weights, automatic DM/CM grids, and
fixed-CM grid behavior in debug. The standalone interpolation callback exposed
a nested diagnostics mutex acquisition; the new callbacks now pass their
existing diagnostics borrow into the shared interpolation basis.

Focused Rust checks now pass 68 library tests (34 explicitly ignored). The
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

The comparator now rejects 37 focused mutations and checks exported native
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

The legacy global adapter now assembles the source global-first columns and
pulsar-local blocks. A focused native witness proves three original defects:
zero constraints trigger a zero-row allocation assertion; later pulsars use
pulsar-zero constraint labels; and ZERO-first constraint rows are written into
global-first coordinates. The corrected mapping uses the current pulsar's label
and explicitly remaps the existing legacy enumerator. Seven natural debug cases
pass for single, deprecated DCM/sigma and global wrappers, with diagonal and full
lower-triangular weighting. This retains the legacy sigma-multiplication convention.

The owned `SpectralFitSession` now implements T2fitSpectra's repeated-call/redo
state and supplied/automatic corner, exponent, count, amplitude and beta choices.
Five native debug scenarios pass, including eight ordered calls. The observer
omits one diagnostic-only printf that otherwise reads uninitialized temporary
values when amplitude is supplied; those undefined bytes are not an oracle.
Standalone routine stdout presentation remains incomplete.

The direct covariance helper comparison now passes ten debug cases: nine kernel
families plus exact MJD masking, including boundary equality and the source's
endpoint extension. It checks daily samples, covariance/interpolation with a
zero constraint row, Cholesky/inverse, in-place decomposition, whitening and
diagonal determinant. The native witness first stops all four PSD entry paths
before reading the unwritten imaginary FFT DC element. The corrected reference
initializes only that omitted element, matching Rust's existing zeroed storage.
Receipts are `build/kernels/debug.json`; release remains pending. A caller audit
also corrected the automatic helper's shape arguments
after the shared kernel API began rejecting unknown shapes.

Twenty-seven natural covariance-selector cases now pass numerical and exact debug
file checks: all grammar kernel families, both legacy forms, direct/embedded DCF,
ECM, PSRJ, nested INCLUDE with PSR/MJD/CLEAR/END, numeric prefixes/hexadecimal
source numbers, ignored trailing text, and failed
calls followed by valid recovery on the same canonical context. Their receipt is
`build/covariance-models/debug.json`. The debug `newDCF`, `chol.covarMatrix` and
`chol.L` files exposed native contractions in interpolation, diagonal addition,
Matern 2.5 and red/pink PSD synthesis; recorded GCC16 arm64 disassembly and explicit
`mul_add` close those byte differences. Earlier files are preserved under
`before-contraction-debug/`. This also corrected an implementation dispatch bug:
the source `MODEL DM` structure-function kernel does not receive the extra
frequency scaling reserved for named `T2...DM` kernels. Automatic covariance was
rerun after the kernel-shape correction and still passed numerical/file checks.

## New numerical ledger, separate from inherited M2

### Public family and physical-update coverage

`check_families.py` now defines 52 authored cases from unchanged AC/AD model
fixtures, explicit fit flags and generated cadence files. Inputs still pass
through the real readers, preprocessing and timing. Initial observations are
preserved under `build/families/initial-debug/`; per-case current identities and
comparisons live under `build/families/debug/`. The suite is not yet a complete
debug/release campaign.

Passing debug cases now include event amplitudes, seven glitch coordinates,
jumps/FDJUMPs/FD, sinc interpolation, automatic interpolation constraints,
waves including the corrected WAVE_SCALE=1 storage path, shapelets, global
single-source/quadrupole/memory/cosmic-string GW fits, DM/DM1/CM/CM1/DMX/CHROMX,
solar-wind scalar/sinusoidal/interpolated fits, RAJ/DECJ, group noise,
ECORR/SECORR and a red spectral variant. Physical updates now compare the
source-defined fields, with source unit gains and exact unwritten-versus-defined
membership. Binary64 physical storage is round-tripped before decimal comparison,
so different shortest decimal spellings are not mistaken for different values.

The six ELL1H cases have 19 whitened-data failures in the initial debug runs;
their matrices, physical updates and timing pass. These are separate from DDH,
red raw order and chromatic-log errors. Seven redundant one-pulsar configurations
remain failures/boundaries: unconstrained linear/step IFUNC, both quadrupolar
interpolation polarizations, quadrupole, single-source GW, two-polarization memory
and cosmic strings. The IFUNC/quad interpolation fits produce offsets around
1e12 seconds and reach the existing source integer-conversion boundary. The
single-pulsar GW matrices retain near-null modes; their numerical failures are
preserved. Separate source-supported constrained interpolation and two-position
global GW neighbors pass. Neither the original cases nor any gate was removed.

The new source map is `GENERAL_FITTING_SOURCE_MAP.md`. Final full routine/API,
schema and diagnostic-presentation audits remain outstanding.

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
reproducible `check_pipeline_solvers.py` now confirms identical binary64 inputs,
exact same-engine pipeline reproduction and passing shared-solver gates for
both input owners. Those passing solver budgets do not close the downstream
16 noise-error failures. The receipt is `build/pipeline-solvers/debug.json`;
the earlier files remain `build/chrom-log-solver-{input.txt,native.txt,rust.json}`.

### DDH paired checks under 0087

`check_ddh.py` executes the original test3 PAR/TIM inputs through simple
preprocessing, first-TOA phase reference, no mean removal and one direct t2Fit
cycle. It uses the current runtime and a separate overlay adding only the
historical GPS table (SHA-256 `30727fdfaaec0f1775543804210d057ffe62076594bb8884d8f8ccafafb676be`).
All 284 native prefit residuals reproduce central's corresponding exports. Input
bytes/order and within-engine SAT/frequency/uncertainty metadata are unchanged;
pulse counts and nphase agree exactly between engines. The SAT comparison uses
0007's existing 1 ps epoch gate. An initial comparator mistakenly required exact
decimal SAT equality between native113 and Rug128; its failed receipts are retained
as `comparison-before-sat-gate-fix.json`, not reclassified numerical failures.

| Clock resources | Rust max prefit residual | Rust max postfit residual | 1 ns ideal result |
| --- | --- | --- | --- |
| Current | 4.598955163858 ns | 6.510126901421 ns | 120 prefit and 197 postfit failures, same membership as native |
| Historical GPS only | 0.465895042335 ns | 0.416568355665 ns | All 284 observations pass both checks |

The historical ideal requirement passes in debug. **The paired fidelity
requirement remains open.** Current-clock fitting has 388 failed checks:
223 whitened-data entries, 155 covariance entries, four increments plus their
four duplicated update records, and two raw singular indices. Historical fitting
has 422: 283 whitened-data entries, 128 covariance entries, the same eight
increment/update records, two raw singular indices and chi-square. These are new
AE observations, separate from inherited M2 and the red/chromatic ledgers.
Timing itself passes: current max native/Rust residual differences are 0.028422 ps
prefit and 0.049258 ps postfit; historical maxima are 0.028422 ps and 0.100663 ps.
No gate, clock default, DDH formula or precision was changed.

The two independent pipelines have small binary64 input differences. Replaying
each pipeline's inputs through both standalone solvers reproduces each original
engine's parameters/errors/covariance/chi-square exactly as binary64. All shared
solver gates then pass except raw indices 13 and 14, which remain permuted.
Thus the physical fitting differences arise from differing pipeline inputs;
the separate raw-order issue survives identical inputs. This bounded localization
does not assert a general precision cause or numerical acceptance. Receipts are
`build/ddh/debug.json` and `build/pipeline-solvers/debug.json`; the current release
replay and newly added multiplier failure are recorded above.

## Focused source corrections

Each correction preserves separate original/corrected native observations in
AE-owned staging. The canonical source and accepted reference archive are unchanged.

| Source defect | Witness / correction |
| --- | --- |
| SVD ignores nonzero constraint RHS | `check_solver.py`: target 4 gives original 0 versus corrected 8/7; target-zero neighbor unchanged. |
| TKrobustLeastSquares discards its robust code | Same probe: W wrapper originally returns 34.333333333333336 versus delegated/corrected 4.8784378310184371. |
| Covariance sort follows row capture | `check_order.py`: three wrong pairings originally, none when sorted before capture; sorted neighbor unchanged. Receipt: `build/order/receipt.json`. |
| Polynomial cache omits interior BBAT/frequency dependencies | `check_cache.py`: changed interior epochs and frequency match fresh calls after removing stale shared storage. |
| Planet callback uses pulsar zero | `check_callbacks.py`: two-pulsar 10/10 becomes 10/40. |
| WAVDM uses achromatic harmonic count | `check_callbacks.py`: independent DM harmonic count controls modulo. |
| IFUNC covariance uses ifuncN as a column | Safe padded-row witness in `check_callbacks.py`: original sentinel 99/99 becomes weights 3/4. |
| DM/chromatic logarithmic priors repeat sine | `check_fitinfo.py`: original 57/57 and 55/55 become 57/58 and 55/56; harmonic neighbors unchanged. |
| IFUNC expanded rows read another configured constraint label | `check_fitinfo.py` includes a real second parameter constraint and checks repeated IFUNC labels. |
| Bootstrap places a 125 MiB result array on the stack | `check_bootstrap_stack.py` records a 131,104,480-byte original frame, failure before entry under the 8,372,224-byte stack limit, and a corrected 32,480-byte frame completing four natural IBOOT=2 fits. Broader capacity/history coverage remains pending. |
| copyPSR reads reserved jump/FDJUMP slot zero | `check_copy.py` leaves reserved destination slots untouched and preserves active-slot values. Rust copies active one-based slots. |
| copyParam evaluates unused unwritten prefit cells | Rust clones initialized/unwritten ownership; native correction copies representation bytes. Focused initialized/unwritten Rust neighbor passes; complete affected lifecycle regression comparison remains required. |
| Constant-only TN projection consumes unwritten white data | `check_source_boundaries.py` observes zero initialized entries and stops before the solve; corrected omission leaves the fitted neighbor unchanged. |
| Automatic covariance allocates double buffers with sizeof(float) | `check_source_boundaries.py` stops before original write 10 beyond 80 bytes; the corrected 160-byte buffer admits all 16 natural rows. |
| Three covariance-spectrum builders leave imaginary FFT DC unwritten | `check_kernels.py` guards all four PSD entry paths before the invalid read; corrected native sets only p_i[0]=0, matching the real PSD and existing Rust storage. |
| Quadrupole expansion omits three column labels | `check_fitinfo.py` seeds defined sentinel labels, observes three missing assignments, and checks all four corrected labels. The natural original workflow assertion is preserved. |
| Cosmic-string derivative consumes unwritten pre-event prefactor | `check_gw_boundary.py` stops before the original invalid read; the corrected zero agrees with the residual model and six defined neighboring epochs are unchanged. |
| WAVE_SCALE=1 update selects the wrong coefficient array | `check_callbacks.py` initializes both arrays to distinct values: original changes the unused DM array; corrected changes the parsed/timed WAVE array. WAVE_SCALE=0/2 neighbors are unchanged; the corrected public fit passes. |

Source numerical conventions such as Welsch's expression, diagonal/full detL,
source IFUNC covariance's use of lower L, and band/group error accumulation are
retained. GCC16 arm64 fused arithmetic in TN polynomial projection is explicitly
represented; the unfused native diagnostic reproduced the earlier Rust matrix
and localized a 24 ps downstream discrepancy. `check_contraction.py` now records
14 native contracted instructions versus zero with `-ffp-contract=off`. The
counterfactual changes 41 first-cycle red matrix entries and postfit residuals
by up to 23.622976 ps. Current Rust has only the existing six raw-mode failures
against the contracted native run, versus 310 against the counterfactual.
Unprojected DM/chromatic neighbors pass both. Receipt: `build/contraction/debug.json`.

## Remaining required work

1. Complete remaining optional covariance/output paths; extend the passing legacy,
   pointer/file/robust checks to remaining boundary paths and integrate their
   schema checks.
2. Complete bootstrap capacity/copy/failure histories and retained release comparisons;
   preserve the passing debug RNG, resampling and writeTim observations.
3. Complete automatic covariance helper/public API coverage and spectrum failure
   and context histories; complete failure/recovery evidence.
4. Finish callback/constraint/noise family and history coverage: binary/astrometry,
   events/jumps/interpolation, solar wind, group noise, ECORR/SECORR, shapelets,
   spectral variants, automatic constraints, full covariance selectors/masking,
   nonfinite/error/recovery and optional output files.
5. Complete remaining bootstrap capacity/copy histories and investigate further
   demonstrated local defects under 0053 without using undefined native memory
   as a numerical oracle. Stack, sorting and TN-contraction builders now exist.
6. Independent identical-input solver checks for the new workflows, complete
   observable/schema checks, debug/release public/component runs, affected
   AC/AD/M1/M2 regressions, formatting/Clippy, and final source/unsafe maps.
7. Rewrite this checkpoint as the final report with exact delivered commit and
   evidence identities, complete remaining source inventory and 0082 assessment.

Inherited open gates remain P 154 including SIMUL 32; the four historical M2 raw
mode-order checks; four 0049 utility boundaries; and original DDH 1 ns ideal-fixture
gate. Central decision 0087 now localizes the DDH native failure to GPS-clock
resource version: current clocks retain the 4.598955 ns maximum; the single-file
historical clock overlay gives 0.465895 ns and passes the full unchanged native
ideal test. This is central native evidence, not an AE Rust run. The user adopted
the paired acceptance rule in 0087: Rust must match current-clock native behavior
under existing gates and pass the unchanged 1 ns pre/post-fit ideal check with
the versioned historical GPS table. Debug passes the historical ideal check;
current-clock fitting fidelity remains unresolved as detailed
above. Current
production clocks and the original failed native observation remain unchanged.
Enabled native acceleration
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
