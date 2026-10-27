# Optional native backends

Cargo keeps its disabled default. Enable `calceph`, `blas`, `lapack` (which
includes `blas`), or the independent `qr-default` feature explicitly:

```sh
CALCEPH_DIR="$PWD/validation/native_backends/build/provider" \
TEMPO2_ACCELERATE=1 cargo build --offline --manifest-path rust/Cargo.toml \
  --release --features calceph,lapack
```

`CALCEPH_DIR` must contain `include/calceph.h` and `lib/libcalceph.a`. The
BLAS/LAPACK binding selects macOS Accelerate's legacy LP64 ABI with
`TEMPO2_ACCELERATE=1`, or Linux OpenBLAS LP64 with `TEMPO2_OPENBLAS=1`.
Linux requires `cblas.h`, `lapack.h`, and OpenBLAS development libraries; the
header macros supply the Fortran character-length arguments. Both integer
types must be four bytes. Selection is explicit. Linux execution is pending;
see [RELEASE_QUALIFICATION_REPORT.md](RELEASE_QUALIFICATION_REPORT.md).
The measured provider and reproduction commands are in
[`../validation/native_backends/README.md`](../validation/native_backends/README.md).
The retained SOFA dependency and FC selection are unchanged.

Native-style `core_cli::run` and `tempo2-rust -f MODEL.par TOAS.tim` start in
mode 1, or mode 2 with `qr-default`. Argument order is significant:
`-noaccel` sets 0, `-qrfit` sets 2, and `-svdfit` sets 1 only if the current
mode is nonzero. The JSON timing/fit adapter and `FitOptions::default()` retain
mode 0. Source configuration and runtime selection are distinct:

| Mode | Matrix/Cholesky dispatch | Shared solve |
| --- | --- | --- |
| 0 | Existing Rust loops/decomposition | Existing Rug128 SVD |
| 1 | Enabled BLAS and LAPACK helpers where compiled | Existing Rug128 SVD |
| 2 | Enabled BLAS and LAPACK helpers where compiled | LAPACK QR if compiled; source warning and SVD otherwise |

An enabled provider failure returns an error. It does not switch algorithms.
`qr-default` alone is valid and retains the source unavailable-QR warning/fallback.
The crate's default differs from a native configure invocation that defines
`QR_DEFAULT`; the matched default-1 native oracle explicitly undefines it.

## Canonical public calls

`calceph::read_ephemeris_calceph(&mut PulsarContext)` and
`calceph::tt2tb_calceph(&mut PulsarContext)` keep their existing signatures.
Each acquires the existing process session once; the global driver uses the
borrowed-session implementations to avoid recursive locking. `EPH_FILE` in the
canonical PAR parser selects CALCEPH through the normal timing/CLI path.
Each pulsar/pass owns and closes a separate descriptor, including error exits.
Timing results record the observed resource hash; the provider's independent
filename open is not a snapshot against concurrent file modification.

`t2accel` exposes the five source helpers on `tk_matrix::BlasMatrix`:

| Rust call | Result and mutation |
| --- | --- |
| `accel_mult_matrix_vec(a, x)` | New output vector; both inputs borrowed |
| `accel_mult_matrix(a, b)` | New canonical contiguous output matrix |
| `accel_cholfac(&mut a)` | Provider INFO; lower row-major factor, original upper triangle retained |
| `accel_uinv(&mut a)` | Provider INFO; source lower inverse and zero upper triangle |
| `accel_lsq_qr(&mut a, &mut rhs, parameters, covariance, rescale)` | Chi-square; DGELS mutates A and the real RHS; parameter/covariance destinations are independently optional |

Direct helpers explicitly require their compiled feature. For canonical shared
dispatch, use `t2accel::with_acceleration(mode, || operation())`; it restores the
previous thread selection after normal return, error or unwinding. This is a
selection scope, not a new process lock or a concurrency qualification.
`general_fitting::t2_fit` scopes its existing `FitOptions::acceleration`, and
the native command scopes all later preprocessing/timing/fitting/predictor work.

Shared `tkfit::least_squares_outputs` reports `solver: "SVD"` or `"QR"`.
Its `rank` is now `Option<usize>`: `Some` for SVD, `None` for QR. QR JSON omits
rank, singular values and retained modes; no second SVD is run to manufacture
metadata. QR unscaled errors are empty because that intermediate was not
computed. Requested QR errors come from the actual returned covariance.
Unrequested parameter/error/covariance outputs remain empty.

QR chi-square includes augmented constraint rows even without parameter output;
rescaling uses augmented rows minus columns and scales covariance in DTRMM.
SVD keeps its original data-row statistic and unscaled covariance; absent
parameter output leaves its statistic zero and can zero rescaled errors.
`general_fitting` records the actual solver engine. The historical SVD engine
label describes the SVD itself, even when whitening uses an enabled helper.

## Source, type and unsafe map

All native correspondences refer to pinned TEMPO2
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`.

| Source | Rust path / preserved boundary |
| --- | --- |
| `readEphemeris_calceph.C::readEphemeris_calceph` | `src/calceph.rs::enabled::driver`: timing-longdouble/Rug128 position JD, C-int truncation, then two binary64 JD parts; KM/SEC body IDs; TCB scale and source ecliptic rotation; selected planets, one-day differences, all six mass-correction components |
| `readEphemeris_calceph.C::tt2tb_calceph` | Same driver time branch: existing `PredictorNumber` ordinary-long-double rounding before JD split; target 16/centre 0; negate component 0 and promote to Rug128 |
| `global.C::{clock_corrections,ephemeris_routines}` | `src/global.rs`: first-pulsar CALCEPH dispatch; time before TT2TB in clocks; positions, observatory coordinates, TT2TB, CALCEPH time, positions in ephemeris pass |
| `tt2tdb.C::tt2tb` | Existing accepted current-pulsar correction is reused, never pulsar 0's correction for another pulsar |
| `T2accel.C::accel_multMatrix[Vec]` | `src/t2accel.rs` + `native/backends.c`: row-major reinterpretation and source DGEMM operand reversal/DGEMV transpose |
| `T2accel.C::accel_cholfac` | DPOTRF U column-major gives lower row-major; INFO is retained |
| `T2accel.C::accel_uinv` | DPOTF2 L, lower packed DTPTRI, source unpack and upper zeroing, including the unused public routine |
| `T2accel.C::accel_lsq_qr` | DGELS T, queried workspace, source overwritten matrix/RHS, packed upper inversion, DTRMM R/U/T/N, binary64 chi-square and rescaling |
| `TKmatrix.C::{multMatrix,multMatrixVec}` | `src/tk_matrix.rs`: canonical source layouts, shared feature/runtime dispatch and existing fallback |
| `cholesky.C::{formL,getCholeskyMatrix}` | `src/cholesky.rs`: enabled decomposition and one source retry multiplying covariance diagonals by 1.000001; observation and IFUNC callers share it |
| `TKfit.C::TKrobustDefConstrainedLeastSquares` | `src/tkfit.rs`: shared augmented system and optional destinations, existing robust wrapper, truthful solver metadata and source diagnostic order |
| `t2fit.C` / `tempo2.C` | `src/general_fitting.rs`, `src/t2fit.rs`, `src/core_cli.rs`: scoped runtime state, local/global/noise/covariance callers and ordered native CLI defaults |

All new unsafe sites are in `src/t2accel.rs` and `src/calceph.rs`, with adjacent
extent/lifetime/alias explanations. Canonical `BlasMatrix` owns contiguous
binary64 storage and validates positive dimensions; public adapters check
cross-buffer extents and LP64 conversion before FFI. Mutable provider buffers
have unique borrows. The C shim includes the selected headers and asserts
integer/double sizes. CALCEPH's opaque nonnull pointer has a unique Drop owner,
no Send/Sync implementation, and a live NUL-terminated filename for open.
The timing core is not linked into Rust. Existing Rug128 production precision
and retained SOFA inventory are unchanged.

## Defined corrections and departures

* 0081's demonstrated BLAS declaration-guard defect is corrected by independent
  guards; an actual BLAS-only native/Rust execution now supplements its syntax witness.
* 0093's actual-provider optional-output witness is reused. The shared QR matrix
  allocation and real RHS copy are independent of parameter-output requests.
* CALCEPH source ignores compute failure and consumes output. The original
  defined-buffer, out-of-range witness emits provider errors yet publishes a
  zero position with this provider. Rust checks status before using that call's
  output, returns an error, and closes the descriptor. Earlier completed state
  is retained; the operation is not transactional.
* Source subtracts inactive planet storage without a producer. Supplied values
  retain their subtraction; absent values leave `None`. The native direct
  observer supplies defined inactive values and never treats native unwritten
  bytes as a numerical golden. Unwritten `sun_earth`/TEL fields are not invented.
* LP64 workspace query status, finite positive integral size/range and allocation
  are checked before casting/allocating. This protects the unsafe boundary;
  no invalid provider workspace result or allocator exhaustion was injected.
* Source QR diagnostics checks one FILE but dereferences another. Existing
  fallible owned writes report either destination error. Completed earlier files
  survive later failure, preserving the source operation order.
* The selected GCC16 arm64 oracle contracts the QR residual-square accumulation
  to FMADD. The Rust macOS arm64 branch uses `mul_add` for that measured rounding
  (recorded disassembly); this does not establish another compiler/target profile.

These follow 0053's existing authorization and keep disabled behavior separate
from enabled failure. See [NATIVE_BACKENDS_REPORT.md](NATIVE_BACKENDS_REPORT.md)
for executed coverage and unresolved acceptance obligations.
