# AE general fitting implementation delivery

Updated 26 October 2026. This is the implementation delivery for central's
0074 handover and 0082 assessment. **Numerical/scientific acceptance remains
pending.** Central must review this delivery and close AE ownership before
releasing a successor; this report does neither.

Source pin: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. AD implementation baseline:
`a8e5178778f7eeb3a0fc65f8934a64a03f94208d`; AD report:
`0f44180a021655074c387e2c03012d0c4253276e`; AE release: `ae03c87`.
AE implementation commits: `8a16880`, `f75eb13`, `1209f31`, and `ddff7f82cfe05bc152e55ddbdb33f9bfc4d06bc9`.
The evidence index and tested executable identities are recorded below. The
canonical TEMPO2 checkout, reference archive and accepted runners are unchanged.

## Delivered capability and public interfaces

`timing --fit` executes the canonical PAR/TIM, preprocessing, BAT/residual and
NITS fitting lifecycle for independent and shared-global pulsars. The library
exports `general_fitting::{fit,t2_fit,do_fit_all}`, separate preparation,
design/constraint-row, update and prefit/postfit operations, and inspectable
`tempo2-general-fit-v1` results. Local/global ordering, row identities,
constraint targets, solver outputs, physical before/after/postfit state,
noise arrays and recomputed timing are exposed. The bounded fit adapters share
the same state, timing and SVD routines.

The completed routine/header inventory is
[GENERAL_FITTING_API.md](GENERAL_FITTING_API.md); module, ownership and unsafe
mapping is [GENERAL_FITTING_SOURCE_MAP.md](GENERAL_FITTING_SOURCE_MAP.md).
These cover the released fitting callbacks, constraints and automatic DM/CM
operations, robust/legacy/optional-output solvers, public SVD and matrix helpers,
covariance grammar/kernels/whitening, automatic covariance and required spectral
closure, seeded bootstrap and state recovery. Standalone source bodies remain
callable even where default source dispatch does not select them. Explicit
source exits and undefined-memory boundaries remain errors.

No new unsafe operation, FFI declaration, numerical dependency, precision mode
or alternate solver is introduced. Rug128 and source binary64/binary32
boundaries remain fixed. Typed constraint ownership survives context cloning
and destruction of the original. Recovery preserves the source's completed
mutations; it is not a general transactional rollback promise.

## Executed verification

Unless noted, every row below was executed in both debug and release. Receipt
paths are relative to `validation/general_fitting/`. A successful runner exit
is not a numerical pass when its receipt contains failed comparisons.

| Campaign | Observed result | Evidence |
| --- | --- | --- |
| Public workflows | 14 cases: 12 pass; red has six raw-mode failures; chromatic-log has 17 failures | `build/public/{debug,release}/receipt.json` |
| Family workflows | 58 cases: 43 pass, ten numerical nonpasses, five explicit boundaries | `build/families/{debug,release}/receipt.json` |
| Shared solver and robust/output contract | All 42 cases pass, including independently optional outputs, H/B/W/fallback histories and files | `build/solver-contract/{debug,release}/receipt.json` |
| Standalone SVD | 69 quantities pass: decomposition, bidiagonalization, extended RHS, dropped mode, norm/maximum, signed ties and NaN-first semantics | `build/svd-api/{debug,release}/receipt.json` |
| Scalar/telescope callbacks | 1,240 derivative/update/error fields pass; FDDI, GLEP and clock-offset expected source exits verified by diagnostic; GWECC null callback checked | `build/callback-grid/{debug,release}/receipt.json` |
| Standalone callbacks | 88 fields pass, including DM/CM covariance callbacks and cutoff-QP | `build/standalone-callbacks/{debug,release}/receipt.json` |
| Constraint payload parsing | Five natural prefix/hex/trailing-text parameter and solar payload cases pass | `build/constraint-payloads/{debug,release}/receipt.json` |
| Public preparation and recovery | Three exact same-engine preparation/apply/postfit cases; five failure/recovery pairs; two automatic-covariance failure histories followed by valid fresh-context/file equality | `build/api-lifecycle/{debug,release}/receipt.json` |
| DM/CM components | Three automatic-grid/matrix-weight cases pass | `build/components/{debug,release}/receipt.json` |
| Covariance kernels/selectors | Ten kernel/mask cases and 27 grammar/selector/file/recovery cases pass | `build/kernels/{debug,release}.json`, `build/covariance-models/{debug,release}.json` |
| Spectral session | Five scenarios with eight ordered calls pass | `build/spectral-session/{debug,release}.json` |
| Automatic covariance | All exported controls, covariance samples, lower factor and four exact files pass | `build/automatic/{debug,release}/receipt.json` |
| Bootstrap | Four-fit baseline and 3,000 RNG values pass; 0/1/4,096 fits and guarded over-capacity boundary pass; one/two-pulsar partial-copy/output histories pass | `build/bootstrap{,-capacity,-history}/{debug,release}/receipt.json` |
| Legacy fitting | Seven natural single/global/DCM cases pass | `build/legacy/{debug,release}/receipt.json` |
| Public diagnostic files | Six cases pass exact bytes; numerical results remain separately classified | `build/public-files/{debug,release}/receipt.json` |
| Comparator negative controls | 52 spin, 66 robust-H and 58 covariance mutations rejected (176 total) | `build/comparator-controls/{spin,robust-H,covariance}/receipt.json` |
| Affected regressions | M1a/M1b, resource and AC lifecycle checks pass; 92 selected AD comparisons pass; exactly four inherited M2 failures remain | `build/regressions/run-pj8g3xgs/receipt.json` |
| Rust checks | Formatting and all-target Clippy with warnings denied pass; debug library: 68 pass, 34 explicitly ignored; runtime constraint snapshot passes separately in both profiles | Commands below |

The earlier component campaign is retained at `build/component-campaign/` and
belongs to the tested source/binary identities in its receipts. The final pass
reran public/family, solver, standalone callback, bootstrap, automatic covariance,
DDH, API/recovery and affected regression checks after their respective changes.
Unchanged kernel/spectrum/legacy numerical bodies were not repeatedly rerun.
No historical receipt is retroactively attributed to a newer binary.

Final Rust checks used `CARGO_HOME="$PWD/rust/.cargo-home"` and
`--manifest-path rust/Cargo.toml`: `cargo fmt --check`,
`cargo clippy --locked --offline --all-targets -- -D warnings`,
`cargo test --locked --offline --lib`, and the explicitly ignored
`canonical_constraint_callbacks_survive_context_snapshot` test in debug/release.
The 34 ignored library tests are not claimed as a full historical corpus pass.

The public comparator checks exact shapes, membership, numeric classes/signs,
effective MODE/ROBUST/NITS/rescale controls, row/column identity, raw mode
order/rank, matrix/constraint quantities, full covariance/lower factors, increments/errors/covariance,
unscaled errors, error multiplier, linearized residuals and ordered robust
histories. It checks physical values in source callback units and separates
raw updates from postfit transfers. Frozen gates are in `CONTRACT.md`; kernel
samples and optional files use their separate native checks. Synthetic truth
recovery is not claimed. Central's earlier four false-pass findings are closed
by its receipt SHA-256
`537b699c2dedba3c0b2903981bd8c39f5843533cddaee6a0e3cd9678c94e8f63`;
final schema/coverage review remains central's responsibility.

Selected maxima across passing public/family cases (both profiles), expressed
in their checked units: pre/post residual difference `6.4947e-17 s`, phase
`9.1073e-15 turn`, column-scaled fitted/physical update difference
`1.9468e-13 s`, and linearized residual difference `1.3566e-15 s`.
These are observed differences, not new acceptance budgets.

## Open AE numerical ledger

Central owns numerical-policy disposition; until ownership transfer, AE owns
correction of demonstrated translation/integration defects. These observations
are separate from the inherited P/M2/0049 ledger. No failure is classified as
expected success, no mode is sorted and no tolerance is widened.

| Input/check, each profile | Observed result and localization | Affected scope |
| --- | --- | --- |
| Public `red`, cycles 1/2 | Six raw singular-mode mismatches (indices 4/5/6); rank nine, all modes retained; physical fitting/timing gates pass | Exact source SVD ordering; separate from historical M2 |
| `chromatic-log-noise` | 16 reconstructed chromatic-noise error failures at 1 ps plus one error-multiplier failure; identical binary64 solver inputs reproduce it | Ill-conditioned chromatic logarithmic basis; singular-value ratio about 1.05e26; no general precision-cause claim |
| ELL1H A1/EPS1/EPS2/TASC/H3/PB | 5/2/4/3/3/2 whitened-data failures (19 total); matrix, physical-update and timing checks pass | Six naturally timed single-coordinate family cases |
| Single-pulsar quadrupole/GW single/memory polarizations/cosmic string | 770/750/448/468 failures with near-null systems | Redundant single-pulsar geometry; separate supported global geometries pass |
| DDH paired run | 401 current-clock and 433 historical-clock fidelity failures; historical 1 ns ideal gate passes | Current-source fidelity remains open under 0087; detail below |

The three unconstrained IFUNC linear/step/quadrupolar-interpolation cases produce
offsets around `2.5e12 s` and hit the inherited 0049 integer-quotient boundary
in `fortran_mod`. Two `TNsubtractSWGP=1` cases attempt to read source-unwritten
SWGPSignal during the first residual calculation, before prefit initializes it.
These are the five family execution boundaries, not five numerical passes.
Constrained interpolation and SWGP without that first-pass subtraction pass;
SWGP mean transfer into NE_SW and DM/chromatic polynomial transfers are checked.

### Raw red modes

| Cycle | Raw index | Native value | Rust value |
| --- | --- | --- | --- |
| 1 | 4 | `1582228.8481050432247550208302007491` | `64628148.00393346917465173728681352563465` |
| 1 | 5 | `64628148.0039334691746517372868135433` | `64615519.18737586179490550319009043125306` |
| 1 | 6 | `64615519.18737586179490550319008748` | `1582228.848105043224755020830200748660248` |
| 2 | 4 | `1582228.84810504319747018527259814042` | `64628148.00393347071752585680509107856768` |
| 2 | 5 | `64628148.0039334707175258568050910579` | `64615519.18737585020845599598349834618584` |
| 2 | 6 | `64615519.1873758502084559959835035022` | `1582228.848105043197470185272598140818129` |

The earlier checkpoint's raw values and original failed artifacts remain in
version control and `build/before-api-observer-completion/`. Current per-case
identities and comparisons are under `build/public/{debug,release}/red/`.

### Shared-input localization

`check_pipeline_solvers.py` replays both native-owned and Rust-owned inputs
through both standalone solvers. Same-engine reproduction is exact as binary64.
DDH retains only two raw-mode failures (indices 13/14) for each clock/input owner;
physical solver gates pass on shared input. The other DDH fitting discrepancies
are therefore sensitive to differing pipeline inputs, without establishing a
universal precision explanation.

Chromatic-log inputs are identical between pipelines. Its native chi-square is
`23584532.5236933045089244842529296875`; Rust is
`23584532.5237045027315616607666015625`. The error multiplier differs by about
`3.08e-10`, above its `6.49e-11` gate, for either input owner. The passing
chi-square budget does not excuse that failure or the 16 downstream noise-error
failures. Receipts: `build/pipeline-solvers/{debug,release}.json`.

### DDH paired checks under 0087

The original test3 PAR/TIM bytes, order, simple preprocessing, first-TOA phase
reference and removeMean=0 are preserved. The separate historical runtime overlay
adds only GPS revision `0759584ebf8315bbbe6a6d0b41c08ebbcfe527f2`, SHA-256
`30727fdfaaec0f1775543804210d057ffe62076594bb8884d8f8ccafafb676be`.
All 284 native prefit residuals reproduce central's corresponding exports.
Within-engine SAT/frequency/uncertainty and original metadata are unchanged;
pulse counts/nphase agree exactly. Cross-engine SAT uses the existing 1 ps gate.

| Clock resources | Rust max prefit | Rust max postfit | Unchanged 1 ns ideal result, both profiles |
| --- | --- | --- | --- |
| Current | 4.598955163858 ns | 6.510126901421 ns | 120 prefit and 197 postfit failures, same membership as native |
| Historical GPS only | 0.465895042335 ns | 0.416568355665 ns | All 284 observations pass prefit and postfit |

Current/historical native maxima are respectively
`4.598955163860 / 6.510121115575 ns` and
`0.465895042333 / 0.416555983776 ns` (prefit/postfit).
Native/Rust timing differences remain below the frozen gates: current pre/post
maxima `0.028422 / 0.049258 ps`; historical postfit `0.100663 ps`.
The final fidelity counts are 401/433 per profile. Current clocks have 223
whitened-data, 155 covariance, eight increment/update, two raw-mode, four
unscaled-error, one multiplier and eight physical-value failures. Historical
clocks have 283 whitened-data, 128 covariance, eight increment/update, two
raw-mode, two unscaled-error, one multiplier, one chi-square and eight
physical-value failures. PB, T0, OM and OMDOT fail at both immediate and
postfit stages. New observations are counted explicitly; earlier
388/422 and 393/425 snapshots remain preserved, not overwritten as passes.
No DDH formula, production clock default, precision or gate changed.
Receipts: `build/ddh/{debug,release}.json`.

## Focused source corrections

Each correction has adjacent source/cause/fix comments and separate original/
corrected native evidence in AE staging. Undefined source memory is stopped
before use and is not assigned a fabricated numerical oracle.

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
| Bootstrap places a 125 MiB result array on the stack | `check_bootstrap_stack.py` records a 131,104,480-byte original frame, failure before entry under the 8,372,224-byte stack limit, and a corrected 32,480-byte frame completing four natural IBOOT=2 fits. Capacity and one/two-pulsar histories now pass both profiles. |
| copyPSR reads reserved jump/FDJUMP slot zero | `check_copy.py` leaves reserved destination slots untouched and preserves active-slot values. Rust copies active one-based slots. |
| copyParam evaluates unused unwritten prefit cells | Rust clones initialized/unwritten ownership; native correction copies representation bytes. Focused initialized/unwritten Rust neighbor and affected lifecycle regressions pass. |
| Constant-only TN projection consumes unwritten white data | `check_source_boundaries.py` observes zero initialized entries and stops before the solve; corrected omission leaves the fitted neighbor unchanged. |
| Automatic covariance allocates double buffers with sizeof(float) | `check_source_boundaries.py` stops before original write 10 beyond 80 bytes; the corrected 160-byte buffer admits all 16 natural rows. |
| Three covariance-spectrum builders leave imaginary FFT DC unwritten | `check_kernels.py` guards all four PSD entry paths before the invalid read; corrected native sets only p_i[0]=0, matching the real PSD and existing Rust storage. |
| Quadrupole expansion omits three column labels | `check_fitinfo.py` seeds defined sentinel labels, observes three missing assignments, and checks all four corrected labels. The natural original workflow assertion is preserved. |
| Cosmic-string derivative consumes unwritten pre-event prefactor | `check_gw_boundary.py` stops before the original invalid read; the corrected zero agrees with the residual model and six defined neighboring epochs are unchanged. |
| WAVE_SCALE=1 update selects the wrong coefficient array | `check_callbacks.py` initializes both arrays to distinct values: original changes the unused DM array; corrected changes the parsed/timed WAVE array. WAVE_SCALE=0/2 neighbors are unchanged; the corrected public fit passes. |
| Legacy global constraints use the wrong pulsar/layout | `check_legacy_layout.py`: zero-row allocation, pulsar-zero labels and ZERO-first rows fail in the original; corrected current-pulsar/global-first mapping passes seven natural legacy cases. |
| Solar IFUNC sigma/sinusoid expansion labels only the first row | `check_fitinfo.py`: source-zeroed labels `[65,0]` / `[66,0]` become `[65,65]` / `[66,66]`. Assign each expanded row its configured constraint label; five natural payload grammar cases pass both profiles. |

Source numerical conventions such as Welsch's expression, diagonal/full detL,
IFUNC covariance's lower-L use and band/group error accumulation are retained.
GCC16 arm64 contraction is explicit only where native evidence establishes it.
`check_contraction.py` records 14 contracted instructions versus zero with
`-ffp-contract=off`; the counterfactual changes 41 red matrix entries and postfit
residuals by up to 23.622976 ps. Current Rust has six raw-order failures against
the contracted run versus 310 against the counterfactual. Unprojected DM/chromatic
neighbors are unchanged. FFT contraction closes earlier exact-file differences;
original files remain under `build/automatic/pre-fft-contraction/`.

Observer corrections are distinct from scientific fixes: MODE-0 effective
weights were initially compared to raw sigma; a switch fallthrough gave several
physical coordinates GLF2's unit gain; and DDH/position unit maps were initially
absent. Current comparators use the appropriate fields and source mappings.
An initial exact-decimal SAT test was corrected to 0007's existing epoch gate.
An earlier regression rerun reused a lifecycle scratch path and hit
`AlreadyExists`; each run now gets a fresh staging directory. These harness
failures and their outputs remain retained.

## Assessment, inherited gates and coverage limits

AE now supplies the callable general-fitting foundation required by 0071.
The implementation can be reviewed for handover under 0074 independently of
unresolved numerical acceptance. New red/chromatic/ELL1H/GW/DDH observations
remain named, reproducible and distinct from inherited findings. The final
observer exposes more failed quantities; that is additional measurement, not a
change to the scientific contract. No new implementation choice currently
requires changing the accepted solver, precision, interface or dependency order.
Central should review the ledger and schema, then decide whether to close AE
ownership and release the next functional stage under 0082.

Inherited obligations remain P's 154 records (including 32 SIMUL), the four
historical M2 raw-order comparisons, and four 0049 utility quotient/rounding
boundaries. No repeat 113-bit diagnosis was performed. DDH's historical ideal
portion now passes; its required current-clock fidelity remains open. None of
these findings is removed by implementation handover.

Coverage is finite. The telescope polynomial and mode -1 grid uses PKS and checks
the non-STL zero selector, not a nonzero STL ephemeris path. Existing E/G/AD
binary evidence is reused; the new general family suite does not exhaust every
binary parameter combination. Public preparation replay is a wiring/ownership
check, not an independent numerical oracle. Source FDDI/GLEP/clock fitting exits,
GWECC's null callback, the abandoned legacy passN path, automatic covariance's
supplied-model uninitialized branch and optional DM/chromatic metadata with
unwritten prefit coefficients stay explicit. Supplying those coefficients makes
the defined optional-file path testable; no zero defaults are invented.

Enabled LAPACK/BLAS builds belong to AI under 0081. Full text output, plugin/host
integration, unrelated standalone toolkit/spectrum/GW functions and prediction
remain later assignments; their inventory is in `GENERAL_FITTING_API.md`.
Decision 0084 guarantees full Rust core/toolkit APIs plus standalone predictor C
compatibility, not native core C-struct layout. Those future interfaces are not
claimed by this delivery.

Native observations use macOS arm64 with GCC/G++ 16 and A2's pinned archive;
`longdouble` is `__float128`, while ordinary host `long double` is binary64.
Rust retains Rug128. Decision 0086 includes Linux x86_64 in the first release,
but Linux, accelerated builds, performance and full release/concurrency
qualification remain later work. No claim of whole-migration or scientific
acceptance is made here.

## Evidence identity

Implementation revision: `ddff7f82cfe05bc152e55ddbdb33f9bfc4d06bc9`.
Index: `validation/general_fitting/build/delivery/index-ddff7f82cfe0.json`.
SHA-256: `971f96f9ca9c8aafb519756235343d8bb369ec24a8842edf4f174797cfe6a7c9`.
It contains 61 receipts, 710 comparison/identity/raw export files, 245 authored
source files and 32 available executable/archive identities. Generate a new
snapshot after a future committed revision with:

```sh
python3 validation/general_fitting/write_evidence_index.py \
  --regressions validation/general_fitting/build/regressions/run-pj8g3xgs
```

The latest native observer link adds DDH/proper-motion physical unit maps;
DDH was rerun with it. Other campaign receipts retain their actual earlier
observer/executable hashes. Those new maps do not change the earlier cases'
observed numerical bodies or the Rust implementation.

Authored builders and fixtures are committed. Generated JSON/TSV, binaries,
source snapshots and large outputs remain ignored under 0039/0050. The index
hashes receipts and their retained comparison/identity files, plus authored
source and available executables; each run's own receipt identifies its tested
binary. `validation/general_fitting/EXAMPLES.md` provides reproduction entry
points. Earlier snapshots remain separately preserved.
