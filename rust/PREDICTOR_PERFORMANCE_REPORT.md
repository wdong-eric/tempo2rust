# Assignment AO: predictor C evaluation performance

2 November 2026. Delivered for central review under handoff 41 and decision 0123.
On the measured macOS arm64 host, the complete installed C phase/frequency loop
is **2.69 times faster** after removing private scalar text conversion. Its
median loaded evaluation time decreases **62.9%**, with identical before/after
scientific outputs and state. Actual Linux execution remains pending. This is
not full migration, predictor-science or two-platform acceptance.

## Frozen implementation and scope

* Before: `68973d669af0e1180ad8532bb8b364452ca203d7`.
* Production: `0bd8e2cf2ef54e9da6feace122acd8aa96aa3fbe`.
* Validation/recipes: `b618519` (full identity in the delivery receipt).
* Native source: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, freshly compiled by
  the unchanged portable release qualifier with its recorded existing source
  corrections. No native timing engine is called by the Rust predictor.

All work is in `/private/tmp/tempo2-ao-predictor-20261102`. Main, AL/AN work,
public headers, build glue, dependencies, scientific leaves and historical
runners are unchanged. There is no publishing remote, push or workflow dispatch.

The independently exported before/after crate trees each have **600 files**,
all verified against their selected Git blobs. Exactly three files differ:
`src/predictor_ffi.rs`, `src/predictor_number.rs`, `native/predictor.c`.
The last file changes only private representation helpers. Its retained
ordinary event `expl`/`logl` helpers and old text helpers are unchanged.
They remain required by the event boundary and the transport regression oracle.

Each install uses its own Cargo target directory and the existing prepared
offline dependency cache. The host is macOS 26.7 arm64, Rust/Cargo 1.97.1,
Apple Clang 21.0.0, GCC/GFortran 16.2.0, Rug 1.30.0 and gmp-mpfr-sys 1.7.1.
A probe linked to the actual candidate archive reports **MPFR 4.2.2, TLS 1**.
The measured ordinary ABI is 8-byte/53-bit. Timing remains Rug128.

Input `t2pred.dat` is copied from AK's native `performance-v2/cheby-native-1-2`
product. Its SHA-256 is
`a8b4242d2600cc72198a7f40240b0e946d17d173804e6d10d7f5f31c9f8811ee`,
identical to AK's original evaluation input. It contains an 8-by-4 polynomial.
The unchanged C client evaluates separate phase and frequency calls, including
lookup, copying, reconstruction, canonical evaluation and result conversion.

## Measured cause and facility assessment

Five-second, 1 ms `sample` captures contain 4,226 before and 4,227 after stacks.
The actual loaded library UUIDs match the frozen installed binaries. Categories
below are **inclusive and overlap**; they are not additive exclusive timings.
The summarizer avoids counting nested allocator frames twice in one stack.

| Sampled responsibility | Before | After |
| --- | ---: | ---: |
| Scalar read, including conversion and allocation | 64.65% | 10.53% |
| Polynomial reconstruction, including coefficient reads | 51.70% | 11.17% |
| Native text formatting helper | 24.37% | 0% |
| Locale creation | 5.99% | 0% |
| Result write | 2.46% | 0.09% |
| Canonical `Cheby2D::evaluate`, including its allocations | 25.86% | 66.24% |
| Allocator subtrees across all callers | 39.26% | 54.53% |

The higher *fraction* of remaining allocator work does not mean more allocation:
the full loop is shorter and request counts decrease. Polynomial reconstruction
and canonical numeric allocations remain. No cache or leaf rewrite was added.

Before plain/sampled 400,000-call intervals were 9.657276/9.759483 s; after
plain/sampled 800,000-call intervals were 6.914468/7.111474 s. The observed
differences are 1.06%/2.85%; these single adjacent controls include run noise
and do not isolate profiler overhead statistically. Profiled timings are not
used for the speedup claim. Failed sandbox sampling attempts are retained.

Facilities considered against this bottleneck:

* Keeping `%La` plus a shared locale would remove only part of the measured
  cost. Hex/decimal formatting, parsing, string allocation and MPFR parsing
  would remain; a cached locale would also add a new lifetime owner.
* MPFR supplies `mpfr_set_ld`/`mpfr_get_ld`; its own documentation notes NaN
  sign loss in assignment. Rug's raw-pointer methods are available, but direct
  use here would require an additional C MPFR header/build interface or manual
  ABI declarations and raw-object borrowing. These are unnecessary for the
  two existing 53/64-bit native profiles. See the
  [MPFR 4.2.2 interface](https://mpfr.org/mpfr-current/mpfr.html).
* The pinned Rug 1.30.0 `Float::get_significand` API supplies a **borrowed,
  positive, allocation-free** significand and documents its whole-limb padding.
  Both supported precisions fit in u64, including padding. `get_exp`,
  `Integer::to_u64`, `Float::with_val` and binary exponent shifts provide the
  remaining safe Rust operations. Their pinned implementation/documentation
  was inspected; the native boundary cases were then executed on macOS.
* C `frexpl`/`ldexpl` decompose/reconstruct the full ordinary value without
  decoding native padding, narrowing a finite scalar through f64, or inspecting
  GMP/MPFR layout. No new dependency or allocator is needed.

## Representation change and preserved behavior

The C shim copies the current native cell into an unsigned significand, binary
exponent and signed class tag. Finite inputs already obey the native range and
precision, so the private Rust constructor imports them exactly without another
rounding operation. On output, the borrowed Rug significand and exponent rebuild
an already rounded native value. Zero, infinity and NaN classes have explicit
sign handling. Darwin's previous `%La` input convention discarded the NaN sign;
the new read deliberately retains that existing behavior. Linux retains its
sign-bearing convention and needs actual host verification.

All polynomial/model calculations, arithmetic order, contraction sites, rounding
boundaries and 53/64-bit precision selection remain canonical Rust code. There
is no pointer cache: coefficients and scalar fields are reread on every call.
Caller-owned layout/storage, partial readers, FILE positions, callbacks and
opaque context, process globals, publication/error paths, null checks and panic
containment are preserved. Source public text I/O keeps its locale conventions;
private binary conversion does not access either caller locale. The existing
foreign-thread MPFR cache owner remains unchanged.

The new unsafe calls borrow live native cells and exclusive local output
integers synchronously. C alone dereferences native `long double`; no Rust
numeric reference aliases its backing. The borrowed Rug significand never
crosses C or outlives its owner. Native callback allocation/free and ownership
are unchanged. These constraints and the source counterparts are documented
beside the code. No new unsafe allocator, MPFR layout or public ABI is introduced.

## Uninstrumented measurements

Seven samples per engine/workload, shuffled with seed 123: **63 fresh processes**.
The sustained window was announced after compilation and qualification. Host
snapshots show ordinary desktop processes; this is not a reserved or idle host.
The client interval excludes initial parse, while process wall/CPU/RSS includes
load, initialization, parsing and cleanup. All samples and outliers are retained.

For 100,000 iterations = 100,000 phase plus 100,000 frequency calls:

| Engine | Loaded evaluation median (min–max), s | Process wall median, s | Process CPU median, s | Peak RSS median, MiB |
| --- | ---: | ---: | ---: | ---: |
| Native | 0.004530 (0.004476–0.004663) | 0.008483 | 0.00 | 1.422 |
| Before | 4.668811 (4.469706–4.804604) | 4.674084 | 4.65 | 2.281 |
| After | 1.732665 (1.727397–1.781757) | 1.737295 | 1.73 | 2.094 |

Native remains about 382 times faster in this loaded workload. The representation
improvement does not close the remaining Rug/canonical-evaluation CPU gap.

Short installed getPeriod processes have wall medians **5.189 ms before /
5.243 ms after**; no startup speedup is established. Native getPeriod is
13.090 ms with its separately recorded GCC/quadmath dependencies. One-iteration
evaluation processes have 5.052/4.612 ms medians; first-run loader outliers reach
0.423/0.360 s and are retained. Short CPU records round to 0.00 s. These startup
measurements must not be used to infer microsecond parse costs or a broad win.

The separate allocation observer subtracts a one-iteration process from a
1,001-iteration process, retaining both raw records. Per phase/frequency pair:

| Observed request | Before | After |
| --- | ---: | ---: |
| malloc | 975 | 631 |
| realloc | 10 | 8 |
| free | 1,059 | 631 |
| Requested bytes, including realloc sizes | 145,663 | 14,864 |

Allocation requests fall from 985 to 639 (35.1%); requested bytes fall 89.8%.
These are external allocator calls/requested volume, not live bytes or RSS;
hidden libc allocator calls are not exhaustively counted. The instrumented
observer is excluded from performance timings. Linux's static `--wrap` observer
has a documented different interception scope and has not yet run.

## Verification and open results

* 13 focused predictor tests pass in debug and release, including the new
  975-case text-versus-binary transport test: random significands/exponents,
  subnormal boundaries, native maxima, signed zero, both infinities/NaNs,
  and null failure. Linux-specific wide cases execute meaningfully only there.
* The unchanged installed native qualifier passes C11/C++17, static/shared:
  each profile has 44 layout, 68 value/state, 121 storage, 48 low-count and
  26 boundary rows; 270 T1 phase diagnostic lines match per storage run.
  All 64 public symbols, source files, file exchange, getPeriod, locale/pipe,
  callback, ownership and error/recovery checks pass. Its twelve corruption
  controls remain effective; no numerical gate was changed.
* 38 legacy transcript/product files are byte-identical before/after.
  The additional **30,258-line** complete per-call transcript is byte-identical,
  including 10,000 phase and frequency outputs, state, in-place coefficient
  mutation/restoration, miss/recovery, T1 counts 0/1/2/6/32, unused-state NaNs
  and class/sign neighbors. Mutation controls explicitly observe changed results
  at the same pointers and their restoration.
* Supplemental native comparison retains 1,428 one-ULP phase differences
  (maximum `2.910383045673370361328125e-11` turns) and one derivative difference
  (`8.2718061255302767487140869e-25`). All 1,429 are below even AF's absolute
  phase floor. **One negative-NaN T1 frequency sign mismatch remains against
  native**, identically present in before and after; it is exposed, not waived
  or corrected by this representation optimization. The original qualifier's
  gates still pass. Broader inherited predictor log10RMS failures remain open
  and were not rerun as if this bridge change could close them.
* Plain, ASan/UBSan and TSan caller campaigns each complete twelve exclusive
  workflows; complete workflow/storage products agree, including the explicit
  MPFR-cleanup control. Instrumentation covers C clients/shim, not all Rust,
  MPFR, Fortran or provider internals.
* Two separately compiled installed Rust consumers (`predictor_components`
  and `predictor_subnormal`) produce identical complete outputs before/after.
  Formatting and all-target Clippy with warnings denied pass.

No unrelated full timing/fitting campaign was repeated. Actual Linux x86_64
64-bit scalar transport, allocator counts, sanitizer cleanup and performance
remain due. The executable [Linux recipe](../validation/predictor_performance/README.md)
uses the caller's prepared cache and isolated targets. Central can wire it into
the existing installed predictor qualification stages; no new dependency,
workflow dispatch or scientific-policy decision is required by this delivery.

## Retained evidence

Evidence root:
`/private/tmp/tempo2-ao-predictor-20261102/validation/predictor_performance/build`.
`delivery.json` binds **3,188 files / 274,591,921 bytes**, excluding Cargo target
caches and Python bytecode, and verifies the 1,200 frozen Rust source bindings.
Its SHA-256 is
`e181ff4c83e2cca582bc281cb059e81f05d4ac9116b9d3f1e9c662ff682a28f6`.
Failed harness/preparation attempts are retained and described in the recipe.

| Evidence | SHA-256 |
| --- | --- |
| Final complete comparisons, `verify-v4/receipt.json` | `a252d6cf6899e92795b357ce25146467015728fbb8a1dc382df9c67ab061ce94` |
| All timing samples, `measure-v2/receipt.json` | `5ac3fa8fedf6e460a270f7add46797dbb14f2afd2c38ab6fee92dc243b8a44c7` |
| Candidate installed qualifier | `43608b475afae041066751588f9875642337394a3538c80669844f8c20da0b87` |
| Caller/sanitizer campaign | `4caa2515a1c3fc09e6b1515d469dcdc2d0cbb335ce05c0b7915728a2359fd4cc` |
| Installed Rust consumers | `d97379fd760ed36cd46d0298d73d714b84a4c6c6bd26078b79a74c2d9ef9f813` |
| Before shared library | `003677f890095dac8d2e9bde6d9948009705b47466164b2d6f8a7a3269a64288` |
| After shared library | `acb1d0461bdcb911a01b2cad0032cf82bfd390c161e24f3cafdd392022d1532b` |

Before/after sampled library UUIDs are
`5F18C007-3126-3583-B9EC-23E2CEFB92D1` /
`0F5CE5C9-E03C-3550-840B-68BE3B6B4D47`; the raw stack reports and `dwarfdump`
observations agree. Complete receipts retain static archives, native corrections,
loaded dependencies, toolchains, original stdout/stderr, resource counters,
input identities and all generated scientific products. Generated evidence is
ignored; authored implementation, tests, recipes and this report are committed.
