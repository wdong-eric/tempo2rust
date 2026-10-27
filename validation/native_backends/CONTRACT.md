# Assignment AI frozen implementation/comparison contract

Baseline: release 897041d; AH production 0dbcf08, corrected observer 50f2bf3,
report 9dec65c. Native source fda6ad4a9753ad7dc5d291406f2f310a9161423f;
reference archive 7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572. Canonical and
accepted evidence are read-only. All build products/resources go under build/.

Dependency order: provider/build and ABI; CALCEPH owner/global integration;
five T2accel helpers; canonical matrix/Cholesky/shared solver dispatch;
direct and public profile comparisons; source/type/unsafe/departure report.

## Configuration and APIs

Cargo default features remain empty. Explicit `calceph`, `blas`, `lapack`
(implies blas), and independent `qr-default` mirror source macros.
The local numerical provider is macOS Accelerate's legacy LP64 interface,
selected explicitly by TEMPO2_ACCELERATE=1. No ambient provider autodetection.
CALCEPH uses a separately installed static library/header at CALCEPH_DIR;
the selected local provider is 4.0.5, mirrored at commit
718e7945ccd27216b522243c3ff8aab63ce1d99d (the official release host timed out).
Real ephemeris: published INPOP17a_TDB_m100_p100_tt.dat from ftp.imcce.fr. Source/header/archive and loaded
Accelerate identities must be recorded by the executed recipe. An unmeasured
provider or ABI is not qualified by these tests. Linux qualification remains AL.

`core_cli::run` retains ordered -noaccel/-qrfit/-svdfit processing and native
default 1 (2 with qr-default). JSON FitOptions retains acceleration 0.
Direct helpers use canonical BlasMatrix. Scoped acceleration selection propagates
through existing matrix, whitening, robust and fit calls, restoring the previous
selection after success/failure; it adds no process-mutex recursion. This scope
is not concurrency qualification. CALCEPH direct entries retain their existing
signatures and take the existing outer session once; global calls borrow it.

Shared least-squares results identify the actual solver. SVD spectrum/retained
modes/rank remain populated for SVD and absent for QR. QR uses the real supplied
RHS even without parameter output, includes constraints in chi-square and uses
augmented rows minus columns for rescaling. No second solve supplies metadata.

## Comparison projection frozen before execution

All selectors, counts, row/column/body/owner identities, status/branch classes,
requested output membership, nonfinite/sign/structural-zero classes are exact.
Capture complete defined in-place mutations and retain unwritten state explicitly.
Native and Rust use the same provider and inputs. Failed native calls keep their
actual classifications; corrected-source observations are separate.

Use existing AE/0007 quantities and gates unchanged: timing 1e-12 seconds,
phase 1e-10 turns, positions 1e-12 light-seconds, velocity 1e-15 in units of c;
Cholesky L 5e-14 times native column maximum with exact structural zero;
covariance/errors/chi-square use AE/0007 identical-input or pipeline gates.
New dimensionless direct matrix products, inverse L, mutated QR work matrix/RHS
and triangular covariance use 5e-14 times their native column/vector maximum,
with structural zeros exact. This bounds binary64 provider correspondence and
does not widen timing gates. TT-TDB seconds use 1e-12 s. Selected-planet one-day
differences use their original vector units, without derivative normalization.
Full provider-equal direct cells will additionally be reported as bit matches.

Exercise disabled, CALCEPH-only, BLAS-only, BLAS/LAPACK and combined profiles;
runtime 0/1/2; actual target 16 data; distinct pulsars/counts; frames/units and
selected planets; direct optional outputs/constraints/rescaling; local/global,
correlated and robust fits; ordered CLI selectors; bad resource, factorization,
rank, dimensions, workspace and output-file boundaries followed by valid work.

All twelve AH groups and the full 0094 P/M2/0049/AE/DDH/AF ledger remain open.
Production Rug128 and existing gates do not change. AK owns the recorded warning
collection failure and concurrency/performance; AJ installed interfaces; AL actual
macOS/Linux integrated release qualification. AI does not self-accept milestones.
