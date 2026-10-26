# Assignment AG: public analysis tools delivery

26 October 2026. Released baseline 1310dba; AF production 7420622, validation
543760f and report 7a94491. Raw TEMPO2 pin
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`. Decisions 0078/0089 define scope;
0090 fixes weighted-transform policy; handoff 33a identifies the extrema comparator
correction. This report is an implementation/evidence handover, not central milestone
acceptance or a claim that the inherited scientific ledger has passed.

## Delivered capability

Public safe Rust now provides every defined toolkit/statistics/RNG, spectral,
sigma-z and GW body in the AG source map. The three requested executable consumers
are `toolkit`, `spectrum`, and `gw_simulation`. The latter two load actual PAR/TIM,
preprocess through the canonical runtime and evaluate timing before analysis.
Spectrum uses prefit residuals with removeMean=true and SAT−PEPOCH0 in days;
GW uses that origin in seconds and RAJ/DECJ, prepares geometry before summing,
and leaves TOAs untouched. These are executable recipes in ANALYSIS_TOOLS_API.md.

Owned ToolkitRng/SpectralSession/SigmaZ/GwSession state exposes serial histories;
MT default 5489, reseeds, twist transitions, rejection draws and lack of Gaussian
caching are retained. Source helpers share the canonical toolkit, FFT, Lomb,
weighted-LS, covariance/SVD/robust and eccentric implementations. No added crate,
unsafe block, FFI boundary or alternate production precision is present. MT19937's
license notice is retained. `TK_fitSine` is removed upstream and `sineFunc` lacks a
body; no guessed implementation was added.

The driver retains source transform order, float-scratch differencing, 4096-point
spline grid, all five estimator branches and normalizations, written extent versus
reported nSpec−1, and early diagnostic/white-noise returns. The result explicitly
marks whether the reported-count pointer would have been written. Estimator 5's
additional source normalization is named by its actual compound units. Sigma-z
uses every active row, including deleted observations, and preserves its source
weight choices and repeat-call history. Failed analysis invalidates helper access
until valid initialization; subsequent valid work is tested.

GW covers tensor and all four general families, isotropic/anisotropic/dipole
populations, both frequency samplings, harmonic/CDF/grid doubling, negative-PDF
warnings, tensor/general geometry and real/imaginary responses, Earth-only and
finite-distance branches. Angular frequencies are rad/s; flo/fhi are Hz. Distinct
source speed constants remain distinct. Eccentric wrappers reuse AD's equations
and expose the energy variant's writes: source residuals are nanoseconds and
`eOut` is a binary32 W/m^2 estimate, not a validated total radiated energy.

Native file exchange is explicit little-endian i32 headers with binary128 values,
9/17 scalar prefixes, and separate geometry setup. Multiple IDs, counts, cursors,
normal/subnormal boundaries, signed zeros, infinities and signed NaNs are preserved.
NaN payloads are canonicalized. The codec borrows prefixes because Rug 1.30
Float::clone resets NaN signs. Missing/truncated/negative-count inputs error;
complete prior rows and cursor remain observable, and valid processing can resume.
Nonserialized caller geometry/metadata is preserved, never claimed to be imported.

## Selected scientific policy and bounded source fixes

0090 preserves supplied positional errors after averaging, spline interpolation,
Hann smoothing and first/second differencing. At an actually reached weighted
estimator the warning names all applied transforms and states that original
positional errors are retained without propagated uncertainty/covariance. Earlier
returns consume no weights and produce no fictitious weighted warning. Genuine
insufficient backing errors instead of inventing errors. This compatibility
choice is not transformed-noise covariance propagation.

The source/departure map records adjacent source locations and the original/corrected
witnesses. Local 0053 corrections comprise associated error sorting; actual spline
scratch allocation; the missing cross factors in weighted two-column elimination;
valid diagonal-sigma lower-matrix backing for TK_fitSinusoids; current resy instead
of sigma-z globals for diagnostic files; safe optional robust output writes in the
native t2spectrum witness; sigma-z one-based backing/no-pivot handling; the general
linear vector-family assignment typo; and initialized exact-alignment imaginary
response. Original undefined memory is classified, not executed as numerical truth.
File read checks and actual realization-ID search are explicit bounded corrections.

The original weighted witness returns cosine/sine coefficients approximately
2.5562991596823932/2.0580551236511742 for an exact 3/2 input. Corrected native and Rust
recover the independent normal equations. Original reversed distinct-error rows
change weighted output; corrected outputs are permutation invariant. Initialized
linear-vector sentinels 0/7 expose inactive-polarization contamination. Original
sigma-z data sentinels 0/17 change prefit diagnostic files without changing the
whitened solve; corrected native and Rust files match byte-for-byte.

GCC 16.2.0 arm64 default `-O2` contracts Lomb multiply/add pairs. The equal-spacing
Nyquist example differs substantially without those contractions; explicit mul_add
now reproduces the observed source arithmetic. Retained assembly is
`validation/analysis_tools/build/debug/spectrum.s`. Central independently reproduced
the default/fast versus off behavior. No threshold was widened and no runtime
numerical mode was introduced. Linux contraction remains an AL qualification item.

## Verification

Debug and release each pass **21,520 checks** across these fresh public/native
projections, with **zero new comparison failures**:

| Case | Checks per profile | Failures |
| --- | ---: | ---: |
| toolkit | 7195 | 0 |
| components-spectrum | 11003 | 0 |
| components-sigma | 123 | 0 |
| components-gw | 817 | 0 |
| actual-spectrum-1 | 174 | 0 |
| actual-spectrum-2 | 142 | 0 |
| actual-spectrum-3 | 140 | 0 |
| actual-spectrum-4 | 154 | 0 |
| actual-spectrum-5 | 142 | 0 |
| actual-gw-0 | 86 | 0 |
| actual-gw-1 | 86 | 0 |
| eccentric.par | 43 | 0 |
| eccentric-coalescence.par | 43 | 0 |
| special-wire | 112 | 0 |
| rust-0-wire | 624 | 0 |
| rust-1-wire | 624 | 0 |
| diagnostic-0 | 6 | 0 |
| diagnostic-17 | 6 | 0 |

The component sweep covers 75 estimator/output/differencing combinations, averaging
and Hann weighted/unweighted neighbors, both weighted/unweighted spline routes with
genuine 4096 error backing, helper boundary cases, three FFT direction selectors,
all sigma-z weight selectors in reused state, general family order/grid doubling,
and repeated mixed log/linear RNG consumption. Actual spectrum examples reach all
five estimators plus N/T/I covariance callbacks and sigma-z. The largest actual
GW residual discrepancy is 3.51769882432720394457551e−18 seconds, below 1 ps.

Each profile rejects 12 deliberate corruptions. The four one-representable-step
min/max mutations each produce exactly one failure. The initial comparator had
mistakenly allowed arithmetic tolerance for these fields; handoff 33a retained that
false-acceptance evidence. Final rebuilt emitters use f64_exact/f32_exact and compare
source-width bits. Native anisotropy warning counts are measured from output,
replacing the preliminary hard-coded count. TKcmonot is included in the source map.

Affected AE checks use fresh AG destinations and unchanged scientific gates:
automatic covariance passes; all 42 optional-output/robust/diagnostic solver cases
pass; seven legacy single/DCM/sigma/global diagonal/full routes pass; 69 standalone
SVD quantities pass, in both profiles. The old legacy observer link needed three
local no-op diagnostic callbacks for later AE objects; accepted runners/files were
not edited. Each profile passes 86 library tests and 3 new behavioral tests; 34 corpus tests
remain explicitly ignored. `cargo fmt --check`, all-target Clippy with `-D warnings`,
and the staged whitespace check pass. The final tests include missing/truncated
file recovery, retained nonserialized metadata and signed-NaN wire round trips.

The original native diagnostic crash from optional TKrobust outputs, failed initial
Lomb/fixture/NaN-sign attempts, and initial legacy link failure remain in ignored
build/debug evidence. Final results supersede those attempts without deleting them.
This is implementation and native-conformance evidence, not astrophysical model
validation or scientific acceptance of every inherited timing/fitting family.

## Evidence identities and replay

Full commands, input hashes, numerical maxima, failures, executable identities,
measured warnings and generated artifact hashes are in the per-profile receipts.
`validation/analysis_tools/README.md` gives fresh-directory replay commands.

| Evidence | SHA-256 |
| --- | --- |
| outputs/debug-v2/receipt.json | 8392a218c483be57d38764c95ee5ee890124b4e5510e53381b6feac52fac0016 |
| outputs/release-v1/receipt.json | 398a826db552be92ffa6edc4287f847af19c57fa486daf8dd4ebc76ac16b7818 |
| build/native-final/identity.json | c591e0c11675f37a3497a6499f146fc72d166636d2a0d867b411e8a80b31035a |
| build/native-original/identity.json | 7f46bc014a0615f4aa795b6d639379c6ca92d2e14f470431b62ee47a88460ad7 |

Paths in this table are under validation/analysis_tools. Native identities include
raw toolkit/spectrum/GW snapshots, the executed corrections, accepted AE/AD objects
and archive hashes. TKrobust is based on the existing AE observed source, with its
additional optional-output guards in the corrected build. Controlled TKsetSeed
uses a source-local clock shim at 1793000000 seconds, not a guessed live-clock value.
The native layout witness preceded codec implementation. Generated evidence stays
ignored; no JSON/binary/output was committed.

The primary checkout's final authored scope is rust/ and validation/analysis_tools/.
Canonical TEMPO2 had five pre-existing untracked .DS_Store paths; these are preserved.
The independent reference source checkout remains clean. Concurrent central decision
and handoff updates are outside this implementation's commits.

## Inherited unresolved ledger and later work

Unchanged: P's 154 records including 32 SIMUL; M2's four raw-mode order failures;
0049's four discrete boundaries and three unconstrained interpolation cases; AE
red's six raw modes; chromatic-log's 16 errors plus one multiplier; ELL1H's 19
whitened-data differences; single-pulsar quadrupole/GW-single/memory/cosmic-string
770/750/448/468 failures; DDH's 401/433 current/historical fidelity failures plus two
shared-solve raw-order differences; and two first-pass SWGP unwritten states. The
paired DDH clock rule and historical 1 ns ideal pass remain intact. AF's seven raw
polyco log10RMS failures remain open. None was deleted, accepted, clipped, reordered
or relabeled a pass by AG.

Installed Rust/predictor packaging, actual Linux x86_64 native comparison,
performance/concurrency and integrated full-migration qualification remain later
AJ/AL work. Weighted uncertainty propagation remains a separate scientific design
choice. Central retains milestone and scientific acceptance.

## Authored commits and final verification record

* `1d3c760` — freeze APIs, source map and prospective comparisons.
* `87099ca` — complete public implementation, examples and focused tests.
* `cf4dfeb` — native replay, rejection controls, affected regression tooling,
  completed source/departure map and 0090 public/validation contracts.

The 21,520-check receipts were produced before committing the implementation.
Subsequent production-file edits were explanatory comments, test-only recovery/NaN
coverage and license whitespace; the compared numerical bodies are those committed
in 87099ca. Final debug/release library and behavioral tests compile that delivery.
The report commit follows these three commits and changes documentation only.

Final log/receipt identities and authored-source hashes are indexed in the ignored
`validation/analysis_tools/build/delivery-identity.json`. Detailed regression roots:
`build/regression-debug` (automatic/solver), `build/regression-debug-rest`
(legacy/SVD), and `build/regression-release` (all four), under analysis_tools.
No repeated historical full campaign or platform qualification is claimed.
