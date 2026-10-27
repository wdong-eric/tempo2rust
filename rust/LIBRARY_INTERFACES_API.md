# Assignment AJ interface contract

Baseline: AI `20e368fe6e7d3e05565ae7f0132f4498dfa9069e`, correction
`e96b5898827cfd15f0890431922460860d05d349`, central release `841b944`.
Pinned TEMPO2: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
This contract implements 0084/0095; it accepts no numerical gate.

## Artifacts and ownership

* Cargo package `tempo2-rust`, import `tempo2_rust`: canonical safe owned state,
  resources, kernels and callbacks. Install its source with Cargo.lock, build.rs,
  native SOFA sources and notices under `share/tempo2-rust/rust`. External Cargo
  consumers use that path and the same locked dependencies. No Rust binary ABI
  is promised across compiler versions.
* `libtempo2pred_rust.a` and `libtempo2pred_rust.dylib` (Linux `.so`, execution
  deferred to AL); headers `include/tempo2pred.h`, `tempo2pred_int.h`. These use
  the actual native C structures, enum and ordinary long double calling ABI.
  No opaque handles are hidden in the source fields. The native shim owns only
  C allocations, scanning/formatting, dispatch and ABI marshalling. Arithmetic,
  polynomial construction/derivative/evaluation/selection and plans use Rust.
* `bin/getPeriod-rust`: source binary64 atof MJD, fixed 1400 MHz, binary64
  frequency/reciprocal, exact printf format; initialized owner and checked input.
  Canonical TEMPO2 installations are never replaced.

C allocations returned by Init/Read/Copy/Insert belong to their exposed owner;
Destroy releases them. Direct caller storage may be used without ownership
transfer for evaluation. Names and nearest-segment pointers borrow the exposed
backing until replacement, insertion/reallocation, keep or destruction. Callbacks
borrow native x/y/z arrays synchronously and retain caller info unchanged. FILE
arguments are borrowed; their actual scan positions and partial writes survive
failure. Only filename operations close the streams they open. Callers must
supply valid aligned live objects, correctly sized buffers and initialized cells
actually read by the selected source operation. C cannot validate dangling
pointers or discover allocation extents. Concurrent mutation is not supported.

## Frozen C source responsibilities

All twenty `T2Predictor_*` declarations in the pinned public header are included:
Init, Copy, Insert, Keep, Destroy, Read, FRead, Write, FWrite, GetPSRName,
GetSiteName, GetStartMJD, GetEndMJD, GetStartFreq, GetEndFreq, Kind, GetPhase,
GetFrequency, GetPlan, GetPlan_Ext.

The complete standalone lower-level surface is:

| Source | Exported definitions |
| --- | --- |
| cheby2d.c | parse_ld_cheby; Cheby2D_Init/Destroy/Copy/Construct/Construct_x_Derivative/Evaluate/Test; testFunc; testCheby2D; ChebyModel_Init/Copy/Destroy/GetPhase/GetFrequency/Write/Read; ChebyModelSet_GetNearestIndex/GetNearest/GetPhase/GetFrequency/Write/Read/Init/Insert/Keep/Destroy; ChebyModelSet_OutOfRange |
| t1polyco.c | T1P_grabString/LongDouble/Int; T1Polyco_GetPhase/GetFrequency/Write/Read/Read_NewFormat; T1PolycoSet_GetNearest/GetPhase/GetFrequency/Write/Read/Init/Destroy |
| tempo2pred.c | twenty generic functions above; unsigned tempo2_verbose initially 1 |

The installed internal header corrects the missing set argument in
T1PolycoSet_GetNearest and const declaration/definition mismatches, and declares
standalone definitions omitted by the original header. `T1P_grabInt` returns
long double as its actual source definition does. Core-dependent
ChebyModel_Construct/Test and ChebyModelSet_Construct/Test are Rust
`cheby2d_int::{construct_model,test_model,construct_set,test_set}`; they are not
standalone C library symbols. Disabled main is not an interface.

Ordinary native long double crosses pointers through exact hexadecimal text
and sufficient-digit decimal text, never binary64. Private conversion uses an
explicit C numeric locale and the complete-token Rust parser; source file
scanners/formatters retain their locale conventions. Rust's existing
PredictorNumber owns arithmetic; timing remains Rug128. Conversion allocates
and copies; AK measures performance. Initial measured target is macOS arm64
(53-bit native significand, size/alignment 8/8). Linux x86_64 marshalling must
retain its 64-bit significand, but actual qualification remains AL work.

T1's declared 0/1 counts retain fixed coeff[32] backing. Phase reads C0/C1,
frequency reads C1 and subsequent active coefficients only. Owned Rust file
constructors retain 2..32. Phase preserves all three stdout diagnostics and
frequency prints none. New-format scanf commits header members and coefficients
at their source points and leaves the failed token at the borrowed FILE cursor.
Cheby nearest-index clears OutOfRange even on miss; nearest-pointer sets it on
miss. T1 does not change that datum. tempo2_verbose does not guard phase prints.

## Errors and unsupported operations

Retain source integer reader statuses, -1 lookup sentinel, diagnostics and T1
Copy/Insert/Keep process-exit disposition (test in subprocesses). Generic file
read uses fresh initialized union storage to correct the known wrong-active-
union destruction, retains the previous owner on failure and attempts the source rewinds without
requiring seekability; actual stream consumption survives failed rewind.
Invalid extents, allocation failure and malformed previously undefined reads
produce explicit failure rather than uninitialized access. Numerical Rust
bridge errors are reported to stderr and mapped to -1 for fallible calls or
NaN for scalar calls; void operations return after reporting. Rust unwinding
must not cross C. Existing source file-write open failure exits 1. Output FILE
errors remain observable through ferror. Exact deviations are recorded beside
code and in the report.

## Rust source correspondence and sessions

Existing comprehensive responsibility maps remain normative:
GENERAL_FITTING_SOURCE_MAP.md, ANALYSIS_TOOLS_SOURCE_MAP.md,
PREDICTOR_SOURCE_MAP.md, CORE_OUTPUT_SOURCE_MAP.md, NATIVE_BACKENDS_API.md,
and the historical source inventory. Public core routes are `pulsar`,
`pulsar_lifecycle`, `tempo2`, `source_par`, `source_tim`, `preprocess`, `bats`,
`global`, `timing`, `binary`, `propagation`, `general_fitting`, `fit_callbacks`,
`constraints`, `legacy_fitting`, `bootstrap`, `core_output`, `core_cli`.
Numerical/analysis routes are `tkfit`, `tksvd`, `tk_matrix`, `tkrobust`,
`cholesky*`, `toolkit`, `spectrum`, `sigma_z`, `gw_sim`, `gw_file`, `ifunc`,
`shapelet`, `solar_wind`, `selection`, `sort_toas`, `update_epoch` and `units`.
Predictor routes remain the canonical AF public modules.

AJ additionally exposes source-direct resource owners and operations previously
reachable only within the crate: clock corrections/table interpolation,
observatories, EOP/UT1, Fortran stream readers, JPL/IFTE time ephemerides,
TT2TB, observer coordinates, meteorology/Niell functions, vector rotations,
parameter initialization and utility conversions. Public owners preserve Drop,
resource provenance, error classes and existing validated construction; exposing
an operation does not make arbitrary mutation of its internal cache safe.
The exact visibility additions and external direct calls are the companion
source map and consumer sources, not a claim that every scientific branch was
newly validated.

Process-owned operations acquire `clkcorr::with_process` or `with_resources`
once. Their closure uses borrowed-session functions only: calling an entry
that reacquires the same session inside it would deadlock. CALCEPH direct
wrappers and global drivers retain AI's ownership boundaries. Build features
calceph/blas/lapack/qr-default and runtime acceleration selection are independent.
QR rank/spectrum remain absent, augmented chi-square and covariance rescaling
remain distinct from SVD. See NATIVE_BACKENDS_API.md. 0090 positional spectral
errors retain warnings; 0087 historical-clock ideal and fidelity checks remain
separate. No frontend introduces new numerical conventions.
