# AO macOS Clenshaw recurrence qualification

4 November 2026, decision 0131. Private macOS binary64 recurrence temporaries
make the measured complete installed C phase/frequency loop **2.2268 times
faster**, reducing median loaded time **55.1%**. Direct polynomial evaluation
is **15.4620 times faster**. Required before/after outputs agree exactly.
This is bounded macOS arm64 evidence; Linux execution and release-budget
acceptance remain separate central checkpoints.

## Frozen implementation and arithmetic

* Baseline: `8716d1dc026641f3a51c2ba88850c5599293cbdd` (accepted owner reuse).
* Production: `e24622d5210fdfc8abe10f9f58a21b7e4a2a1aca`.
* Recipe and installed-borrow witness:
  `f5924bb10bcd6cd2da7880c52ac6f24f3e29ed76`.
* Native reference: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, with the existing
  qualifier's unchanged recorded corrections.

`Cheby2D::evaluate` still implements one Clenshaw recurrence. A private
compile-time operation adapter stores macOS arm64 temporary values in binary64.
It converts validated x/y once, reads coefficients when used, and constructs
the public Float-backed result on return. Every finite macOS N value is already
representable in binary64, including subnormals and signed zero. This changes
temporary storage and implementation cost, not the adopted 53-bit profile.

The existing recurrence is an established method; no replacement numerical
algorithm or library is needed. The prior measured assessment found substantial
MPFR conversion/arithmetic cost even in stacks without allocation frames.
Rug MiniFloat cannot directly preserve the public shared `&Float` interface;
standard binary64 supplies the selected private arithmetic without a new
dependency. [Rust's `mul_add` documentation](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.mul_add)
specifies the single rounding required at the existing source FMA sites.

Rust arithmetic does not promise a NaN sign. The adapter explicitly makes
add/doubling/FMA NaNs positive, matching unchanged N operations. Unary minus
preserves its separate sign-flipping rule for zeros and NaNs. The final j=0
source FMA has already restored the public result's positive NaN convention.
No NaN payload contract is added. Rust documents the distinction between
arithmetic NaNs and bit-preserving sign operations in its
[floating-point reference](https://doc.rust-lang.org/std/primitive.f32.html#nan-bit-patterns).

The `x*2` and `y*2` rounding points, coefficient additions, half weights and
explicit source contractions remain distinct. Linux's private adapter borrows
the existing N inputs and delegates to the unchanged Rug64 operators, including
separate multiplication/addition and the wider exponent range. It does not
convert through binary64. Public `PredictorNumber`, `as_float` lifetime, model
normalization/dispersion, construction, derivatives, T1, parsing, formatting,
C transport and Rug128 timing are unchanged.

The actual optimized installed macOS library was disassembled. The retained
`assembly-review.json` binds the library, full disassembly, symbol table and
recurrence excerpt. Source FMA sites compile to FMADD/FNMSUB; doubling compiles
to separately rounded FADD of a value with itself; coefficient and outer-row
additions remain separate FADD instructions. FABS/FCMP/FCSEL retain the NaN
rule. Only boundary MPFR reads and one returned owner construction remain in
the evaluator. The nx=1 specialization reduces a zero-product FMA to FMUL;
the complete edge witness covers its zeros and nonfinite inputs.

Qualification uses the default nearest-even environment with gradual underflow,
as required by the existing Rust hardware-FMA path. No control register,
fast-math setting or exception policy changes. LLVM documents the relevant
[default floating-point environment assumptions](https://llvm.org/docs/LangRef.html#floating-point-environment).
Alternate rounding modes, subnormal flushing and observable exception flags
are not newly supported or claimed by this delivery.

## Executed correctness checks

All 17 focused predictor tests pass in debug and release. The new primitive
matrix uses 98 signed boundary/random values and seven third operands to
compare private operations with the unchanged N oracle. It checks addition in
both recurrence forms, doubling, both FMA entry points and unary signs.
Separate regressions protect subnormal rounding, cancellation at a fused site,
overflow before a subsequent FMA and finite cancellation inside a single FMA.
The existing native-transport, owner-reuse and first-rounding-direction tests
remain passing. Formatting and all-target Clippy with warnings denied pass.

The same immutable baseline and candidate installed builds pass the existing
C11/C++17 static/shared qualifier, all 64 symbols, locale/pipe, exchange,
callback, storage/layout, recovery and twelve corruption controls. All
**38 legacy products** and the entire **30,258-line C transcript** agree exactly,
including same-pointer coefficient mutation and restoration.

The unchanged external direct client returns **5,880 exact edge results** and
**28 exact finite results**. The two established installed Rust consumers agree
exactly. A third installed consumer keeps input, coefficient and result
`&Float` borrows alive across later polynomial/derivative evaluations, checks
owner identity and values, and preserves a negative-zero input. Its before/after
output agrees exactly. These consumers use the installed public API.

Plain, ASan/UBSan and TSan C campaigns each pass twelve caller-exclusive
workflows; the explicit MPFR-cleanup control remains passing. Sanitizer coverage
is scoped to clients and the C shim, not every Rust/native dependency.

Native comparison still records 1,429 finite differences below the adopted AF
absolute floor and **one inherited negative-NaN T1 frequency sign mismatch**.
Exact before/after results do not waive that native failure. Inherited polyco
log10RMS failures remain open; this slice does not rerun unrelated full timing
or fitting campaigns and changes no acceptance tolerance.

## Separate allocation observations

The existing observers subtract whole-process counts at 1 from counts at
1,001 iterations. Both raw records are retained; these clients are excluded
from runtime measurement.

| Scope per iteration | Before malloc | After malloc | Realloc before/after | Requested bytes before/after |
| --- | ---: | ---: | ---: | ---: |
| f64 construction | 1 | 1 | 0 / 0 | 16.003 / 16.003 |
| Owned multiply, including input clone | 1 | 1 | 0 / 0 | 16.003 / 16.003 |
| Two direct polynomial evaluations | 110 | 2 | 0 / 0 | 1760.003 / 32.003 |
| Direct model phase/frequency pair | 130 | 22 | 0 / 0 | 2080.003 / 352.003 |
| Complete installed C phase/frequency pair | 234 | 126 | 8 / 8 | 8512 / 6784 |

The leaf pair now allocates only its two returned public owners. The full C
pair falls from 242 to 134 allocation requests (44.6% fewer), with 20.3% fewer
requested bytes. Free deltas match malloc deltas; calloc deltas are zero.
The direct 0.003-byte delta reflects the three-byte argv-length difference and
is retained. External allocation requests/volume are not live memory or RSS;
dyld interposition does not observe hidden libc internals.

## Uninstrumented runtime

All builds and correctness checks finished before central explicitly cleared
the timing window. The 28 direct processes (seed 124) ran first, followed by
63 full-C/native/startup processes (seed 123), with seven samples per group.
Boundary snapshots show no competing compiler or benchmark; ordinary desktop
activity remained. Every sample, including startup outliers, is retained.
Allocation observers were removed from both campaigns. Loaded intervals exclude
setup/parsing; wall/CPU/RSS cover the complete process.

For 100,000 iterations of each scope:

| Scope / engine | Loaded median (min–max), s | Wall median, s | CPU median, s | Peak RSS median, MiB |
| --- | ---: | ---: | ---: | ---: |
| Direct leaf pair, before | 0.603102 (0.597489–0.605827) | 0.607604 | 0.60 | 1.844 |
| Direct leaf pair, after | 0.039006 (0.037810–0.039511) | 0.042875 | 0.03 | 1.828 |
| Direct model pair, before | 0.662396 (0.651705–0.673576) | 0.666871 | 0.66 | 1.844 |
| Direct model pair, after | 0.104569 (0.102845–0.104767) | 0.108391 | 0.10 | 1.828 |
| Complete C pair, before | 1.052714 (1.049723–1.066365) | 1.057172 | 1.05 | 2.016 |
| Complete C pair, after | 0.472748 (0.469795–0.478632) | 0.476904 | 0.47 | 2.062 |
| Complete native C pair | 0.004160 (0.004146–0.004165) | 0.007773 | 0.00 | 1.422 |

The direct leaf/model speedups are 15.4620x / 6.3346x (93.5% / 84.2% less
loaded time). The complete C speedup is 2.2268x; native remains about **114x
faster** on this workload. The full C route still reconstructs coefficient
owners and performs scalar transport/model work. Borrowing C coefficient
storage or further representation changes are not part of this delivery.

The measured direct leaf evaluates two 8-by-4 polynomials at seven dimensionless
x values, with y=0.25; the second result is dphase/dx, not frequency in Hz.
Direct model calls use seven preconstructed epochs and 1400 MHz. The complete
C route includes native storage reconstruction and model/result transport.
These are different workloads; subtracting their intervals does not establish
an exact transport overhead. The new baseline median is a fresh measurement
of the unchanged old binary, not a relabeled prior campaign.

Short getPeriod wall medians are 4.063 ms before / 4.254 ms after; one-pair
process medians are 4.008 ms / 4.089 ms. One-pair startup outliers reach
0.466 s / 0.490 s (native 0.397 s). No startup improvement is established.
Short CPU counters round to zero. Full-C median peak RSS rises by 48 KiB;
the lower allocation request count does not imply a peak-RSS reduction.

## Evidence and remaining qualification

The host is macOS 26.7 arm64, Rust/Cargo 1.97.1, Apple Clang 21.0.0,
GCC/GFortran 16.2.0, Rug 1.30.0 and gmp-mpfr-sys 1.7.1 / MPFR 4.2.2.
The immutable input `t2pred.dat` has SHA-256
`a8b4242d2600cc72198a7f40240b0e946d17d173804e6d10d7f5f31c9f8811ee`.
The full C loop uses epochs 58000.010000 through 58000.010006 and 1400 MHz.

New evidence root:
`/private/tmp/tempo2-ao-predictor-20261102/validation/predictor_performance/build/recurrence-20261104`.
Its `delivery.json` binds **1,882 files / 210,775,135 bytes** and **1,732**
Git-to-export/installed source identities. SHA-256:
`08591e74c16e2b71e95fb20f5a8430458250d5aaed9c5d49a6421899cf3f98ff`.
Cargo target caches and Python bytecode are excluded; all six installed-consumer
executables are copied separately. The production source exports contain
601 baseline / 603 candidate files; their installed selections contain
263 / 265 files. The only crate differences are the recurrence, its private
adapter and the intervening owner-reuse report. Public numeric-owner source
and all other production code match the baseline.

The manifest rebinds the reused baseline's actual executables/libraries and
six runner receipts' input/product hashes. It also retains primitive-test logs,
optimized disassembly, every correctness/allocation/timing result, host snapshots,
the explicit timing clearance, summary statistics and executed binding script.
An initial manifest-preparation error named the comparator's directory wrongly;
the failed script and correction note remain. No scientific or timing check
failed in this campaign.

The installed candidate shared-library SHA-256 is
`6795f6bdb087263781f8d4d166b5f3f404a4e5819f7c8b97d543954360b99e75`.
Earlier transport, owner-reuse and CPU-assessment manifests retain their
original hashes; their evidence was not overwritten or relabeled.

The [portable recipe](../validation/predictor_performance/RECURRENCE.md) records
the actual production pair, existing correctness/timing runners and installed
borrow witness. Actual Linux x86_64 execution remains AL's qualification task.
Its focused test includes a 64-significand-bit value, `2^-16445` and `2^16382`;
macOS results cannot qualify that branch.

All work is isolated in `/private/tmp/tempo2-ao-predictor-20261102`. No main
checkout, public header, dependency, scientific C implementation or older
evidence was changed. Central owns integration, performance-budget acceptance,
publication and workflow dispatch.
