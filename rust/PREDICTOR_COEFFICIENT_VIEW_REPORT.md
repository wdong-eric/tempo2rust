# AO call-local C coefficients: macOS qualification

4 November 2026, decision 0136. The complete installed C phase/frequency workload
is **2.2604 times faster** on the measured fixture: median loaded time falls
from **0.473858 s to 0.209635 s** per 100,000 pairs, a **55.76% reduction**.
Allocation requests fall from **134 to 62 per pair**, and requested bytes from
**6,784 to 1,920**. Complete before/after scientific and state outputs agree.
This is a macOS result for central review. Actual Linux execution and the
inherited native scientific discrepancies remain open.

## Frozen implementation and source scope

* Before: accepted recurrence `e24622d5210fdfc8abe10f9f58a21b7e4a2a1aca`.
* Production: `a1851c55fcc0b1ac733b064e631bfbbd2e0118d5`.
* Qualification recipe/witnesses: `ae9bf27130ab5b299de4f52e404b04279508c5c7`.
* Native reference: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, with the
  existing qualifier's recorded source corrections.

Work is confined to `/private/tmp/tempo2-ao-predictor-20261102`. The immutable
before install/direct clients/qualifier remain under
`validation/predictor_performance/build/recurrence-20261104/`; the candidate is
`build/coefficient-view-20261104/after-v1`. Five production files change:
`src/cheby2d.rs`, `src/cheby2d/evaluation.rs`, `src/predictor_ffi.rs`,
`native/predictor.c` and the directly affected `LIBRARY_INTERFACES_API.md`.
The exported crate also includes the intervening recurrence report. No other
crate content differs. Public headers, dependencies, T1, construction, parsing,
selection, ordinary-number implementation and Rug128 timing remain unchanged.

The before shared-library SHA-256 is
`6795f6bdb087263781f8d4d166b5f3f404a4e5819f7c8b97d543954360b99e75`;
the candidate is
`9824a8f467b25a4b18abc2bdf4b395db794464ada24ffb9ebed316151a1a19bc`.
The unchanged 8-by-4 input SHA-256 is
`a8b4242d2600cc72198a7f40240b0e946d17d173804e6d10d7f5f31c9f8811ee`.
The host is macOS 26.7 arm64, Rust/Cargo 1.97.1, Apple Clang 21.0.0,
GCC/GFortran 16.2.0, Rug 1.30.0 and gmp-mpfr-sys 1.7.1 / MPFR 4.2.2.
Builds use isolated targets and a private copy of the prepared offline cache.

## Design and safety boundary

The accepted [assessment](../validation/predictor_performance/C_RECONSTRUCTION_ASSESSMENT.md)
established the reconstruction cost and the applicability of standard immutable
slices and generic closures. The implementation extracts the existing source
Clenshaw loop and changes only its coefficient accessor. Owned Rust coefficients
and the macOS C slice use that one loop; no second scientific evaluator, public
view type, dependency, cache or persistent native borrow is introduced.

`evaluator` validates the existing positive checked C-int shape and null backing
before returning a private `FnOnce`. On macOS, the closure creates `&[f64]`,
evaluates synchronously, and returns an owned N result. The slice ends before
output publication. Compile-time C guards establish eight-byte size/alignment,
binary radix, 53-bit precision and binary64 exponent limits. The positive
C-int product times eight fits `isize::MAX` on this target. Existing caller
obligations supply initialized, aligned, live, single-allocation backing and
exclusivity for the call; the code cannot discover dangling pointers or actual
allocation lengths. It adds no heuristic pointer or duplicated shape checks.

Every call sees current coefficients. The accessor preserves Darwin's existing
positive, quiet NaN input convention, including negative/signaling input bit
patterns; it does not alter backing bytes or promise NaN payload preservation.
The existing recurrence retains operation order, half weights, signed-zero/NaN
rules, source FMA sites and separate rounding boundaries. Model normalization,
dispersion and frequency expressions remain shared and unchanged.

Polynomial validation still precedes x/y reads in `t2r_evaluate`; seven model
scalar reads still precede polynomial validation in `t2r_model`. Output writes,
diagnostics, panic containment and thread-local cache cleanup retain their
existing behavior. Linux uses the same accessor interface with an owned Rug64
polynomial and never creates this binary64 slice. Callback tests and derivatives
keep owned snapshots, preserving callback mutations and in-place derivative
behavior. Public `PredictorNumber(Float)`, live `&Float` borrows and owned
`Cheby2D::coefficients()` remain unchanged. API wording now describes the current
parts transport and bounded macOS view; event-libm text transport remains intact.

The actual optimized candidate has ten static FMA-family sites and five separate
`fadd` sites in each storage specialization. The C recurrence loads coefficient
doubles directly and has only two entry `mpfr_get_d` calls for x/y, followed by
one final result init/set. Its evaluation/model call chain no longer calls
`polynomial`; the model retains its seven-scalar vector. Full disassembly,
selected functions and the executed inspection assertions are retained. These
are static instruction observations, not per-call counts or a timing substitute.

## Fresh measurements

Central explicitly cleared the window after root and AL stopped heavy work.
The existing runners executed sequentially from **09:38:26 to 09:38:35 UTC**
(20:38 Melbourne), retaining all **28 direct and 63 C/native/startup records**.
Allocation observers were disabled; thread limits and locale match the earlier
campaign. Boundary process snapshots show no competing compiler or benchmark;
ordinary desktop processes remained. Complete C controls match before/after
timing, and all executable/input identities remain unchanged.

Each long run evaluates 100,000 pairs; each entry below is the median of seven
fresh samples. Direct leaf inputs are dimensionless x/y and return phase plus
dphase/dx; direct model and complete C calls have different scopes. Their times
are not subtracted to infer a transport cost.

| Workload | Before loaded time (s) | After loaded time (s) |
| --- | ---: | ---: |
| Direct owned polynomial pair | 0.039474916 | 0.039527125 |
| Direct owned model pair | 0.104300042 | 0.103626333 |
| Complete installed C phase/frequency pair | 0.473858 | 0.209635 |

Direct differences are below 1%; this campaign establishes no useful direct
speedup. The native C median is **0.004158 s**, leaving the candidate **50.42
times slower** on this fixture. The full-C loaded ranges are 0.471876–0.478573 s
before and 0.208358–0.218678 s after. Whole-process wall medians are
0.477956583 / 0.213933542 s, CPU medians 0.47 / 0.21 s, and peak RSS medians
**2.0625 / 2.03125 MiB** (32 KiB lower). Direct leaf RSS is 1.828125 / 1.859375
MiB; direct model RSS is 1.84375 / 1.859375 MiB. Lower allocation volume is not
a claim of proportional RSS reduction.

| Separate allocation observation, per pair | Before malloc/realloc | After malloc/realloc | Before/after requested bytes |
| --- | ---: | ---: | ---: |
| Complete C | 126 / 8 | 60 / 2 | 6,784 / 1,920 |
| Direct leaf | 2 / 0 | 2 / 0 | 32.003 / 32.003 |
| Direct model | 22 / 0 | 22 / 0 | 352.003 / 352.003 |

Counts use the unchanged separate 1/1,001-iteration observers. The direct
0.003-byte delta retains the three-byte argv-length change divided by 1,000.
Full C removes 64 coefficient owners, two coefficient vector allocations and
their growth; scalar/model/output work remains. No allocation result is derived
from the earlier inclusive profile percentages.

getPeriod wall medians are 4.149 / 4.207 ms; one-pair process medians are
4.264 / 4.057 ms. Startup outliers of 0.398 / 0.462 s and the native 0.406 s
outlier remain in the raw records. Short CPU counters round to zero. These
observations do not establish a startup improvement or a release budget.

## Verification and limits

* All **18 focused predictor tests pass in debug and release**. The new 290-case
  representation witness compares native allocated storage against unchanged
  C-parts-to-N conversion, including both signs, boundaries, quiet/signaling
  NaNs and reproducible bit patterns. It checks unchanged backing bytes.
* The new C witness gives **8,516 exact lines across ten profiles**: before/after
  C11/C++17 static/shared clients and C ASan/UBSan clients/shims. It covers all
  coefficient positions in five shapes, both model polynomials, mutation,
  restoration, pointer/backing replacement, callback snapshots, derivative
  aliasing and all thirteen expected dimension/null diagnostics in order.
* All **5,880 direct edge results, 28 finite results, 30,258 C output/state
  lines and 38 legacy products** match exactly. The existing native qualifier
  passes its four C11/C++17 profiles, 64 symbols and twelve corruption controls.
* Both existing installed Rust consumers and the live public Float-borrow
  consumer agree exactly. Plain, ASan/UBSan and TSan caller campaigns each pass
  twelve exclusive workflows; production and explicit MPFR cleanup agree.
  Sanitizers instrument C clients and shim, not Rust/MPFR/Fortran internals.
* All **91 measurement records** reproduce their stored loop times, CPU and RSS
  from raw outputs/resource records; every sample is retained.

The native comparison still has 1,428 finite phase differences and one finite
derivative difference below the unchanged AF floor, plus the inherited negative-
NaN T1 frequency sign mismatch. Polyco log10RMS failures remain open. No gate,
tolerance, scientific assumption, public precision or native-sign policy changes.

The [portable recipe](../validation/predictor_performance/COEFFICIENT_VIEW.md)
pins both commits and includes actual Linux shared-loop/model qualification,
wide inputs `1+2^-63`, `2^-16445`, `2^16382`, first-rounding checks, the C view
witness's owned Linux route, consumers, sanitizers and separate timing/counting.
Linux execution has not occurred here. No push, dispatch, integration or release
performance acceptance is claimed; central retains those decisions.

## Retained evidence

Evidence root:
`/private/tmp/tempo2-ao-predictor-20261102/validation/predictor_performance/build/coefficient-view-20261104/`.
`delivery.json` SHA-256:
`b30c5b2d20a1f8670c74d57cce6d293db9ab6bd450f8cc94270327dd017cabc3`.
It binds **1,980 files / 304,270,888 bytes**, **61 external identities** and
**1,738 Git-to-export/installed-source bindings**. The executed `freeze.py`
rebinds all thirteen nested receipts, twelve immutable baseline products, five
earlier manifests and the unchanged input. Frozen sources, installed products,
raw checks, every measurement, process evidence and disassembly are retained.
Cargo caches/targets are excluded; six consumer executables and both executed
unit-test binaries are retained separately. This final report is bound by its
delivery Git commit rather than its own evidence manifest.
