# Assignment AI: enabled native backend handoff

This delivers real CALCEPH time/position processing and the five accelerated
matrix, Cholesky and QR helpers through the canonical Rust APIs and native-style
CLI. Cargo's disabled default and the existing JSON default remain available.
This is an implementation handoff for central review, not migration acceptance.

The released baseline was `897041dd528cfb96db19dadb0844023e56814aa6`, following
AH production `0dbcf08`, corrected observer `50f2bf3` and report `9dec65c`.
Canonical TEMPO2 remains `fda6ad4a9753ad7dc5d291406f2f310a9161423f` and the
reference repository remains `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`.
Both repositories are clean. Only authored paths in `rust/` and the new
`validation/native_backends/` are committed; dependencies and generated evidence
remain ignored. Central documents and accepted/historical evidence are unchanged.

| Authored change | Commit |
| --- | --- |
| CALCEPH/Accelerate implementation, shared dispatch, metadata and feature banner | `4c0481172217d667990cc994170a7d3d20034a8e` |
| Public observers, profile recipes, comparator controls and API documentation | `1a9f06ff7f20c9c84cc29eeb8a486ef653f0295c` |
| QR diagnostic-order correction, focused failure/recovery test and final recipes | `e96b5898827cfd15f0890431922460860d05d349` |

## Delivered behavior and correspondence

[NATIVE_BACKENDS_API.md](NATIVE_BACKENDS_API.md) documents the public APIs,
configuration, source/type/unsafe map and each narrow departure. The implementation
reuses canonical context/matrix owners, the existing session lock, shared solver,
whitening, robust, local/global and command paths. No second timing engine or
metadata-only SVD is introduced. The retained 36-routine SOFA closure is unchanged.

CALCEPH preserves the source pass order, first-pulsar dispatch, current-pulsar
TT correction, ordinary-long-double time-JD versus Rug128 position-JD boundary,
two-part dates, target/centre IDs, units, frames, selected planet offsets, one-day
differences and mass corrections. Descriptors close on success and failure.
Compute failures become explicit errors instead of published failed output;
unwritten inactive planet state stays explicit. Earlier completed state survives
a later error, consistent with the existing operation boundary.

The five helpers preserve row/column-major interpretation, provider status,
defined matrix/RHS mutations, triangle conventions and source rescaling. The
source Cholesky diagonal retry is shared by observation and IFUNC covariance.
QR allocates its augmented matrix and copies its real RHS independently of
requested parameter output, reusing 0093's proven correction. It returns augmented
chi-square and scaled covariance even when parameters are absent; QR metadata
does not invent SVD rank/spectrum/retained modes. SVD behavior is retained.

Compile-time feature availability and runtime mode are separate. Native CLI
defaults to 1, or 2 with independent `qr-default`, and applies ordered selectors.
JSON remains 0. The no-argument banner uses the source feature-dependent text,
including its literal `mulitplication` spelling. Enabled failure never silently
falls back; unavailable QR uses the source warning/SVD path.

## Measured provider and build

The actual host is macOS 26.7 (25G229), arm64, SDK 27.0; Rust/Cargo 1.97.1,
Apple clang 21.0.0, GCC/GFortran 16.2.0, CMake 4.4.4. Rust and the native oracles
use the same installed static CALCEPH 4.0.5 and system Accelerate legacy LP64
interface. Integer/double/ordinary-long-double sizes are 4/8/8 bytes. The C shim
includes provider headers and asserts the ABI widths; native build identities
record the actual config, corrections, objects, archive and link commands.

CALCEPH's official host timed out; the built public-mirror tree matches all 533
files in immutable commit `718e7945ccd27216b522243c3ff8aab63ce1d99d`. This is
recorded mirror provenance, not an upstream signature claim. The real published
INPOP17A file spans JD 2414105..2488985 and includes target 16 (TT-TDB). At
2458000+0.125 the provider returns 0.0014399462350882192 seconds before source
negation. Missing target 16 is also exercised using the real position-only DE405
file. The sample is provider correspondence, not an external ephemeris calibration.

| Product | SHA256 |
| --- | --- |
| Immutable CALCEPH source tar | `b38f6ccc726bab51c5901bb74c655e13941753ca7c3d0054396a37b793040426` |
| Installed header | `17fbd05ae48449e1e0c27d70a153ddae07bde7f7722e0a75162ce71d9ff66417` |
| Installed static archive | `2228faa63fd734c6bf23046ef9fed15524c00015241f925a64d5678e2cb5f4e5` |
| Published INPOP17a time/position file | `32517ec82bfb298d53baeba4065750dec82143fab79357e4487f84fb439bc49c` |

Full OS/compiler/loaded-image evidence is in
`validation/native_backends/build/provider-identity.json`. Executed preparation,
build, comparison and failure recipes are in
[validation README](../validation/native_backends/README.md).

## Executed verification

The final campaign is `validation/native_backends/build/profiles-final-v3/`.
Each native/Rust executable is frozen in its own profile directory. The final
comparator checks exact membership, row order, kind, finite/nonfinite sign and
structural-zero classes before applying the unchanged frozen quantity gates.
Direct scalar/vector scales stay within their family; matrix scales stay within
the native column. Native/Rust use identical real inputs/providers.

| Profile | Paired cases | Compared cells | Failures |
| --- | ---: | ---: | ---: |
| `disabled-debug` | 22 | 32,508 | 0 |
| `disabled-release` | 22 | 32,508 | 0 |
| `blas-debug` | 22 | 32,514 | 0 |
| `lapack-debug` | 55 | 32,717 | 0 |
| `calceph-debug` | 26 | 41,752 | 0 |
| `combined-debug` | 59 | 41,961 | 0 |
| `combined-release` | 59 | 41,961 | 0 |
| **Total** | **265** | **255,921** | **0** |

The 21 public pipelines in each numerical profile exercise runtime 0/1/2 with
equatorial TCB, TDB, ecliptic, nonzero prior, Huber robust, two-pulsar global
(12/10 TOAs), and correlated covariance fits. Direct CALCEPH adds both frames and
units, two distinct pulsars with 2/3 TOAs, selected/mass planets, all defined
inactive arrays and repeated passes. Direct helper cells (45/45) and all four direct CALCEPH cases (2,170/2,170
each) also match as binary64 values in the final combined profile. Direct QR adds every combination of three
optional outputs, constraints and rescaling (32 cases). All five helpers include
complete defined success mutations; factorization/rank failure is followed by
valid work. Rectangular and incompatible output extents are exercised before FFI.

Each of eight builds has two matching no-argument banner/exit controls. Disabled,
combined and QR_DEFAULT command profiles additionally execute ten ordered
selector cases per engine and the independent JSON default: 21 commands each.
The public missing-resource/disabled-CALCEPH checks execute eight failure/recovery
commands on final binaries. The real-provider test repeats missing target,
out-of-range and missing-file failures followed by valid calls, checks distinct
pulsars and unchanged failed-call output, and verifies no descriptor-count growth.

| Check | Result |
| --- | --- |
| Disabled debug library, serial | 91 passed; 36 deliberately ignored historical probes |
| Disabled release library, serial | 91 passed; 36 ignored |
| Combined debug library, serial, before banner-only correction | 92 passed; 36 ignored |
| Combined release library, serial | 92 passed; 36 ignored |
| Focused tests for every final profile | 4 passed per build; the resource-dependent test is run explicitly below |
| Explicit real-provider debug/release recovery | 5 passed in each; includes the real-provider test and descriptor-count check |
| Independent `qr-default` without BLAS/LAPACK | 4 focused tests passed |
| Comparator mutation controls | 5 passed |
| Formatting and Clippy (`--all-targets -- -D warnings`), disabled/combined | Passed |

Library results are implementation regressions; serial success is not concurrency
qualification or scientific acceptance. The 36 ignored historical tests were not
replayed or counted as passing. The unchanged historical timing/model/output
corpora and numerical findings retain their accepted evidence and open gates.

Central independently built committed `e96b589` in an isolated Git archive and
reported 5 focused/recovery tests, 5 comparator controls, all 59 combined-release
cases / 41,961 comparisons, 21 CLI/JSON commands and 2 banner controls passing.
This handoff verified its completion and paired-receipt hashes; it is distinct
from the local campaign. The review directory is
`/private/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-ai-committed-review-8udj_tn7`.
`final-review-completion.json` has SHA256
`d9cdcb1c1914438e6db9ab1f5516274f77733572d74dddeff7c6918d13be4f3f`;
its paired receipt has SHA256
`d58d010718162c559ca174e52e9bb357f15d8cc39125a2f4fe320b5194b94812`.
Central explicitly retains overall acceptance and ownership closure as pending.

## Corrections and retained development evidence

0081's original BLAS declaration-guard witness and 0093's actual-provider QR
optional-output witness are reused; the new campaign executes BLAS-only and all
QR masks against the selected provider. The native builder applies those narrow
corrections plus the accepted current-pulsar and AE corrections explicitly and
records the complete patch.

The original CALCEPH out-of-range position witness initializes its buffers,
records provider errors, and still publishes zero because source ignores status.
The corrected Rust boundary returns an error and recovers; uninitialized native
time failure storage is never treated as a golden. Workspace size/status and
allocation are guarded before unsafe calls, but malformed provider workspace
responses and allocator exhaustion were not injected. Failed diagnostic writes
and subsequent valid solve are exercised, including an unwritable `awhite.res`,
retention of the earlier QR diagnostic and completed `adesign.matrix`, followed
by successful writing of the actual RHS after the destination is repaired. The measured native FMADD accumulation
is recorded in `build/accel-assembly.txt` and matched on macOS arm64.

Central review found two comparator defects: an in-gate finite sign reversal
could pass, and distinct QR vectors/scalars could borrow another family's scale.
Both were fixed without widening gates and have focused mutation controls.
An additional exact row-order control exposed different observer serialization
of otherwise matching CALCEPH and optional QR quantities; the Rust observer now
emits the same canonical order. Central also found the enabled no-argument banner
omission; the feature wiring and all-profile controls correct it.

Earlier `paired-debug-v1`, `paired-combined-debug`, `cli-combined-debug`, and
`profiles-published*`/`profiles-final*` artifacts retain their actual identities.
The v2 campaign was interrupted for the diagnostic-order correction; its partial
results and interrupted commands are not counted as final passes. Native binaries
from that build were hash-verified and reused; all final Rust binaries were rebuilt
and all final runtime comparisons were executed again.
The earlier 59-case comparison had zero numeric failures, but its comparator
did not enforce complete stream ordering. `historical-order-audit.json` records
that distinction. Developmental disabled-profile test failures came from wrongly
expecting SVD optional-output error/covariance scaling to match QR; tests were
corrected from the source, without changing production SVD behavior. Final
evidence is the explicitly named fresh campaign, not relabeled earlier files.

## Open ledger and owners

All twelve AH groups remain open: `pre-list-bary-unwritten` (unproduced UT1 and
missing PAR); `pre-list-tempo1` and `pre-list-terrestrial` (position/velocity
signed zeros); `dm-grid` (ill-conditioned RMS/PAR digits); `red-noise` (F2 digit);
`ell1h-h4` (offset/RMS); `waves` and `wave-dm-scale` (unfitted-error producer
boundaries and missing PAR); `global` (F2 digits); `cheby-odd-defaults` and
`cheby-ecliptic-tdb` (coefficients); `polyco-prefix` (coefficients).
AH's absent DDH derived-input and all-deleted report-span boundaries also remain.

The complete 0094 inherited ledger also remains: P's 154 records including
32 SIMUL; M2's four raw-mode order failures; 0049's four discrete boundaries and
three unconstrained interpolation cases; AE red's six raw modes; chromatic-log's
16 errors and one multiplier; ELL1H's 19 whitened-data differences; single-pulsar
quadrupole/GW-single/memory/cosmic-string 770/750/448/468 failures; DDH's 401/433
current/historical fidelity failures and two shared-solve raw-order differences;
two first-pass SWGP unwritten-state boundaries; AF's seven raw polyco log10RMS
failures. The historical 1 ns ideal pass and paired-clock rule remain separate
from unresolved fidelity. Production Rug128 and the numerical gates are unchanged.

AK owns the recorded three warning-vector assertions and full concurrency,
parallel execution and performance; these serial passes do not close that work.
AJ owns installed consumers and standalone predictor C compatibility. AL owns
integrated macOS arm64/Linux x86_64 qualification and retained-dependency release
linkage/distribution. This binding currently rejects non-macOS BLAS/LAPACK builds;
Linux/provider ABI support needs actual implementation and qualification, not
inference from this host. These tests do not qualify every provider, ephemeris,
nonfinite branch, model, noise family, predictor workflow or concurrent session.

The final retained-file audit completed with no mismatches or missing/extra
paired outputs. `delivery-identity.json` binds all final profile receipts, native
source/config/object/archive identities, compiler/provider evidence, test logs,
original failure witnesses and authored validation hashes. Individual paired
receipts also bind every observed production source, input and output. The audit
verified the canonical/reference pins above and current production bytes.

| Evidence index / receipt (under `validation/native_backends/build/`) | SHA256 |
| --- | --- |
| `delivery-identity.json` | `ceb102a64449abefdc02c1557a6e12c2269550150d69a3ef8e7c08f9f60decc3` |
| `profiles-final-v3/receipt.json` | `6c0e583a3a0467e090cc6696ea5e08dbb7f96db9fcd3fe6e555c81940466c4df` |
| `provider-identity.json` | `50f90ced713feeca675f5dad4d5198819d2929f68130e50b9a87a4cd2f4af83a` |
| `final-checks-v3/receipt.json` | `6669be5514f1b8efd269792140694267ee666a55fa9fe66613b321e4dc06a8fd` |
| `public-boundaries-final-v3/receipt.json` | `508d19cc8ce5ee9a668ee7db30c9c4fae275d6c4694f17fe273694e1005ab430` |
| `profiles-final-v3/combined-debug/paired/receipt.json` | `ea3baeb45b908ce975c6bae58fdd329cda3a1d219cc128d0b9f2bb925d8cc81c` |
| `profiles-final-v3/combined-release/paired/receipt.json` | `965afdfe8c07bd8e0b5f25fab2e2813ddf451273244b7d8775bce31a861214a7` |

The final comparator SHA256 is `b1f4c0ea6c04057ab92385a5ca1e69fa7bc06f6ba69e1ab0b2ecf535259f8461`.
All 265 paired cases / 255,921 cells, 105 selector/JSON commands and 16 feature
banner commands passed. Eight public failure/recovery commands and both explicit
real-provider suites passed separately. No new numerical failure is accepted or
hidden by these results; the inherited ledger and later owners remain unchanged.
