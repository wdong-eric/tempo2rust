# AO C reconstruction assessment

4 November 2026, decision 0134. **Recommend a bounded macOS-only borrowed
coefficient slice for synchronous C evaluation/model calls, feeding the same
canonical recurrence. Keep Linux's current owned Rug64 reconstruction.**
The new profile directly attributes 38.53% of stacks to polynomial construction.
This is a concrete implementation proposal, not an implemented or measured
speedup. Production, public APIs and previous artifacts remain unchanged.

## Measured baseline and attribution

Reuse accepted production `e24622d5210fdfc8abe10f9f58a21b7e4a2a1aca`, installed
under `build/recurrence-20261104/after-v1/prefix`. Its shared-library SHA-256 is
`6795f6bdb087263781f8d4d166b5f3f404a4e5819f7c8b97d543954360b99e75`;
sampled UUID `010E9A4E-2B51-3B9A-9A7B-88123FE053C7` matches the installed image.
The unchanged 8-by-4 predictor SHA-256 is
`a8b4242d2600cc72198a7f40240b0e946d17d173804e6d10d7f5f31c9f8811ee`.
The complete C witness retains all **30,258 lines** before and after profiling,
identical to the accepted output, including mutation/restoration.

After central explicitly cleared root/AL activity, the unchanged `sample.py`
ran two million phase/frequency pairs plain, then sampled, with a five-second
capture at 1 ms intervals. Loaded intervals are **9.454360 / 9.486479 s**;
checksums match. Their 0.34% difference mixes noise and instrumentation, so it
does not isolate profiler overhead. Boundary snapshots show no competing
build/benchmark; ordinary desktop processes remained. No candidate evaluator
was run and the accepted seven-sample campaign was not repeated.

The trace contains **3,846 sampled stacks**. All percentages below use that
denominator. Rows are inclusive and overlap; do not add them or infer an
achievable speedup from them.

| Responsibility | Stacks | Percent |
| --- | ---: | ---: |
| `predictor_ffi::polynomial` construction | 1,482 | 38.53% |
| Scalar `read` inside that construction | 1,044 | 27.15% |
| Scalar `read` outside construction | 317 | 8.24% |
| Canonical recurrence | 320 | 8.32% |
| MPFR get/set-double within recurrence | 249 | 6.47% |
| Native `t2r_ld_parts`, all callers | 175 | 4.55% |
| Named allocator subtrees, all callers | 1,895 | 49.27% |
| MPFR init/clear subtrees, all callers | 1,636 | 42.54% |

Within construction, 647 stacks contain a named allocator frame. The trace
also shows GMP integer-to-Float conversion, exponent shifts and vector growth.
The disjoint partition by reported current frame is: allocator 1,792; double
conversion 176; arithmetic 209; range 69; init/clear 197; other MPFR 179; GMP
361; thread-local access 74; other 789. These sum to 3,846. The exclusive
allocator category also recognizes the allocator image, including anonymous
symbols; the inclusive legacy matcher uses symbol names. Inlining and cleanup
in the model caller limit source attribution. Construction excludes subsequent
coefficient destruction, and the recurrence's conversion count includes its
inputs/output as well as coefficients. Neither is an exact removable-time budget.

`reconstruction_profile.py` reuses the established `costs`/`summarize` parsers
and selects disjoint call subtrees. Nested/different-depth control roots,
count conservation and agreement with the original summarizer pass. It also
reproduces the prior 3,846-stack assessment's 641 construction stacks; that
saved-trace check is distinct from this new profile.

The accepted allocation observations remain 134 requests / 6,784 requested
bytes per full C pair, versus two mallocs for the direct leaf pair. They were
not remeasured here. Source inspection counts 64 reconstructed coefficient N
owners and two coefficient vectors per 8-by-4 phase/frequency pair. Eliminating
those owners targets a demonstrated cost; actual savings and remaining model,
selection, scalar and output costs require a later implemented-candidate run.

## Existing boundaries and established facilities

The decisive path is
`t2r_model -> values(v,7) -> polynomial -> shape + values(coeff,nx*ny) ->`
`Cheby2D::from_coefficients -> model normalization -> Cheby2D::evaluate -> write`.
`t2r_evaluate` reconstructs the same polynomial without model normalization.
Each current scalar read goes through C `t2r_ld_parts`, constructs an N owner,
and, on macOS, later converts its coefficient to f64 for evaluation.

| Boundary | Existing evidence / obligation |
| --- | --- |
| Dimensions | `predictor_ffi::shape` requires positive C ints and checked signed multiplication. C Init has matching allocation checks. A view must reuse this gate. |
| Null coefficients | Positive dimensions reach `read`'s `null native scalar backing` error. Check that once before creating a slice, preserving diagnostic and error ordering. |
| Extent/lifetime | The public interface requires aligned live backing and initialized cells actually read. C cannot validate allocation lengths or dangling pointers. The recurrence reads every advertised coefficient. |
| Mutation | Callers provide process-wide exclusivity; concurrent mutation is unsupported. A fresh call-local borrow sees changes made between calls, including at the same pointer. |
| Native format | The installed C layout is long double size/alignment 8/8, 53 bits, exponent limits -1021/1024. Linux is 64 bits in 16-byte storage and must retain its wider range. |
| Public ownership | `PredictorNumber(Float)`, `as_float() -> &Float`, `Cheby2D::coefficients() -> &[N]`, Clone and serialization remain owned/public contracts. Native bytes cannot be treated as N objects. |

The ownership/exclusivity rules in `LIBRARY_INTERFACES_API.md` and
`PARALLEL_EXECUTION_API.md` apply. The former's historical text-marshalling
paragraph predates the accepted binary-parts transport; current source and
accepted receipts establish the actual conversion path.

Apple documents that arm64 `long double` has the same binary64 representation
as `double` in its [ABI guide](https://developer.apple.com/documentation/xcode/writing-arm64-code-for-apple-platforms?changes=_1).
The existing compiled size/alignment/profile witness agrees. This is the
platform-specific basis for a macOS `&[f64]`; it gives no basis for a Linux cast.
An implementation should strengthen the existing macOS compile-time ABI checks
with the necessary double/long-double size, alignment and exponent identities,
and add a focused C/Rust representation witness. No new scientific C arithmetic
or native backend dependency is needed.

Standard [`slice::from_raw_parts`](https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html)
provides the view with non-null, alignment, initialization, single-allocation,
byte-extent and lifetime obligations. The checked positive C-int product is at
most `INT_MAX`; multiplied by eight it is below `isize::MAX` on the qualified
64-bit target. Existing caller extent/alignment/exclusivity obligations supply
the remaining conditions. Do not duplicate dimension validation inside the
recurrence or add heuristic pointer validation. End the borrow before publishing
the scalar result; retain no pointer or slice after the call.

[`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html) supports an immutable
coefficient accessor capturing the existing owned slice or the native slice.
A generic closure needs no trait object or heap allocation. `Cow<[N]>` and
`AsRef<[N]>` cannot borrow binary64 C storage as Float-backed N values; using
them would still require conversion/copying. A new public generic polynomial,
view hierarchy or mutable cache is unnecessary.

## Minimal proposed implementation

1. In `cheby2d.rs`, extract the existing loop into one private routine taking
   validated dimensions and a coefficient accessor returning the existing
   platform input type. The public `Cheby2D::evaluate` supplies
   `input(&self.coeff[i])`; the macOS C path supplies its native slice element.
   This is one source recurrence with two storage accessors, not two evaluators.
2. In `cheby2d/evaluation.rs`, allow coefficient addition to consume that input
   type. Keep Linux `&N` operations unchanged. Preserve every loop index,
   half weight, operand order, separate rounding point and existing source FMA.
   The native getter must retain Darwin's existing positive-NaN input convention;
   unary and result NaN/zero rules remain those of the accepted adapter.
3. In the private model helpers, accept an evaluator closure called once with
   the existing normalized N inputs. Keep the normalization, dispersion and
   frequency expressions exactly where they are; owned Rust and C callers
   supply their respective accessors to that same recurrence. Public model
   methods and their outputs do not change.
4. In `predictor_ffi.rs`, borrow a checked macOS coefficient slice only inside
   `t2r_evaluate` and `t2r_model`. Keep model scalar reads and output transport,
   `finish`/panic containment, foreign-thread cache ownership and failure behavior.
   Linux continues through `polynomial` and owned Rug64. Preserve current gate
   order: polynomial validation precedes x/y reads in `t2r_evaluate`; model
   scalar reads precede polynomial validation in `t2r_model`.

Expected production scope is those three Rust files plus strictly necessary
ABI assertions beside the existing guards in `native/predictor.c`. This is a
proposed dependency-ordered slice; no production patch has been written.

Keep `polynomial()` for derivative/test paths and Linux. In particular,
`t2r_test` takes an owned snapshot before invoking the user callback. A borrowed
view spanning that callback could change snapshot behavior or permit aliasing
with caller mutations. Derivative output likewise remains on the existing
owning path. Construction, parsing, T1, public Float borrows and Rug128 timing
retain their current implementations. There is no global scalar conversion
change, C scientific kernel, persistent borrow, pointer cache or new concurrency
permission.

## Proof and acceptance obligations

The per-coefficient equivalence to prove on macOS is the existing
`native long double -> parts -> N -> to_f64` value/class/sign versus the checked
native binary64 getter. Cover both signs of zero, subnormal/normal neighbors,
maxima, infinities and NaNs, including signed/signaling bit patterns accepted
by the native platform. The public contract does not add NaN payload observables.
Default nearest-even and gradual-underflow assumptions remain unchanged.

Retain exact 5,880-edge / 28-finite direct results, 30,258 C output/state lines,
38 legacy products, installed Float-borrow consumers, callback/derivative tests,
static/shared C11/C++17 qualifiers and caller-exclusive sanitizer/cleanup checks.
Add a focused C coefficient-view witness: direct caller-owned arrays, all
coefficient positions, constant/multi-axis shapes, mutation/restoration and
replacement between calls, unchanged backing, nonfinite/range neighbors, and
the existing invalid-dimension/null diagnostic cases. The direct Rust edge
client alone cannot qualify the new C slice path. Preserve owned test-callback
snapshot and derivative alias behavior rather than extending borrowing there.

Inspect optimized contraction sites and confirm the C evaluation path no longer
builds a coefficient Vec/N array on macOS. Then run the existing interleaved
plain direct/full-C timing recipes and separate allocation observers. No exact
future allocation count, speedup or native-parity threshold is asserted from
this profile. The inherited native T1 NaN-sign and polyco log10RMS failures stay
explicit; unchanged same-engine results cannot waive them.

## Portable Linux qualification after implementation release

There is no Linux execution in this assessment. The proposed Linux route still
requires its owned Rug64 coefficients, separate operations, wide exponents,
subnormal midpoint behavior and exact input signs. A macOS-only cfg must prevent
creation of any Linux `&[f64]` view. Shared-loop/model plumbing still needs actual
Linux tests; unchanged intentions are not platform qualification.

Use the existing [complete recurrence recipe](RECURRENCE.md), substituting
the following explicit pair in its two build commands; subsequent qualifiers,
direct/C comparisons, consumers, sanitizers and debug/release predictor tests
use the same before/after directory layout:

```sh
export CARGO_HOME=/path/to/prepared/linux-cargo-cache
export AO_OUT=/path/to/fresh/reconstruction-linux
export CC=clang FC=gfortran NATIVE_CC=gcc NATIVE_CXX=g++
# AO_CANDIDATE must be the exact centrally released implementation commit.
python3 validation/predictor_performance/build.py \
  --revision e24622d5210fdfc8abe10f9f58a21b7e4a2a1aca \
  --out "$AO_OUT/before" --cargo-home "$CARGO_HOME"
python3 validation/predictor_performance/build.py \
  --revision "$AO_CANDIDATE" \
  --out "$AO_OUT/after" --cargo-home "$CARGO_HOME"
```

Keep the existing ordinary64 `1+2^-63`, `2^-16445`, `2^16382`, first-rounding
and class/sign witnesses, plus the new C-view witness's unchanged Linux route.
Use the unchanged Linux `--wrap` allocation observers and interleaved
`evaluator_measure.py`/`measure.py` only in a cleared host window. Darwin's
`sample.py` is not a Linux profiler and must not be presented as one.

## Assessment reproduction and evidence

New ignored root: `build/reconstruction-20261104/`. The completed profile used:

```sh
export AO_BASE=validation/predictor_performance/build/recurrence-20261104
export AO_INPUT=validation/predictor_performance/build/t2pred.dat
export AO_PROFILE=/path/to/fresh/reconstruction-profile
mkdir -p "$AO_PROFILE"
"$AO_BASE/verify/after-outputs" "$AO_INPUT" > "$AO_PROFILE/control-before.stdout"
cmp "$AO_PROFILE/control-before.stdout" "$AO_BASE/verify/after-outputs.stdout"
# Only after explicit quiet-window clearance; remove allocation observers.
env -u DYLD_INSERT_LIBRARIES -u LD_PRELOAD LC_ALL=C \
  VECLIB_MAXIMUM_THREADS=1 OPENBLAS_NUM_THREADS=1 \
  python3 validation/predictor_performance/sample.py \
  --executable "$AO_BASE/verify/after-evaluation" --predictor "$AO_INPUT" \
  --out "$AO_PROFILE/profile" --iterations 2000000
"$AO_BASE/verify/after-outputs" "$AO_INPUT" > "$AO_PROFILE/control-after.stdout"
cmp "$AO_PROFILE/control-before.stdout" "$AO_PROFILE/control-after.stdout"
python3 validation/predictor_performance/reconstruction_profile.py \
  "$AO_PROFILE/profile/sample.txt" > "$AO_PROFILE/costs.json"
```

The assessment's `build/reconstruction-20261104/receipt.json` SHA-256 is
`b2c63a2364b04e8e04e4aa9b3d45a77a6edb0c12815f5de4e8bc39ad3228b1c3`.
It binds 21 files (3,090,545 bytes), 48 external identities and 32 Git source
bindings: raw trace/outputs, process snapshots, clearance, loaded UUID, baseline
artifacts, exact source contracts and parser inputs. The executed `freeze.py`
rebinds eight selected accepted-baseline products and verifies all four earlier
manifests remain unchanged. This Markdown is bound by its delivery commit,
outside its own evidence manifest.

No production, Linux, candidate-speedup or full scientific qualification is
claimed here. Central must explicitly release the proposed implementation
before it begins.
