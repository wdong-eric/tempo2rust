# AO next CPU assessment

2 November 2026, handoff 43 / decision 0128. **Recommend a bounded macOS
binary64 implementation of the temporary arithmetic inside the single canonical
Clenshaw recurrence, retaining the public `PredictorNumber(Float)` representation
and Linux Rug64 operations.** Central must explicitly release this private
arithmetic change before implementation. No production code, API, dependency,
precision, numerical gate or previous evidence changed during this assessment.
No replacement evaluator was built or benchmarked.

## Refreshed evidence

Reused exact accepted production `8716d1dc026641f3a51c2ba88850c5599293cbdd`,
installed under `build/evaluator-20261102/after-v1/prefix`, and the same 8-by-4
input. The library SHA-256 is
`7192bc8ee79ba8296450cdab82fbb712caa11c5d54e49cda310bbc4a44f08d1b`;
input SHA-256 is
`a8b4242d2600cc72198a7f40240b0e946d17d173804e6d10d7f5f31c9f8811ee`.
Sampled library UUID `A0298698-E52C-3F01-BC61-F60233821BB9` matches the installed
binary. The existing complete C witness produces the same **30,258 lines**
before and after the profile, also identical to accepted handoff 42 output.

The announced short window uses the unchanged `sample.py`: adjacent plain and
sampled runs of 800,000 phase/frequency pairs, with a five-second 1 ms capture.
Both checksums match. Loaded intervals are 8.530386 / 8.734351 s (2.39% apart);
this single adjacent observation does not isolate profiler overhead or establish
a new performance result. Central subsequently confirmed its local builds/tests
and AL's live controls had finished. Retained boundary snapshots contain no
identified compiler/test/predictor process; desktop media/Spotlight indexing was active.
The completed seven-sample timing campaign was not repeated.

There are **3,846 sampled stacks**. Fractions below are inclusive and overlap;
the evaluator column is an intersection with its call subtree, using the same
whole-process denominator. Inlined operations can be attributed to callers.

| Responsibility | All stacks | Within Cheby evaluation |
| --- | ---: | ---: |
| Canonical `Cheby2D::evaluate` | 54.84% | 54.84% |
| Native polynomial reconstruction | 16.67% | 0% |
| Private scalar reads | 16.12% | 0% |
| Allocator subtrees | 37.29% | 15.29% |
| MPFR get/set f64 | 18.69% | 16.38% |
| MPFR add/subtract/multiply/divide | 15.96% | 12.58% |
| Native range/subnormal boundary | 4.63% | 2.94% |

Of the 2,109 evaluator stacks, **1,521 contain no allocator frame**. This supports
targeting ordinary arithmetic and repeated Float/f64 conversion as well as
allocation. Existing accepted request counts remain 110 per direct polynomial
pair and 242 per full C pair. They were not remeasured or reinterpreted as live
memory here.

`profile_costs.py` also produces a disjoint partition by the *reported current
frame*: allocator 1,433; f64 conversion 380; arithmetic 573; range 178; init/clear
164; other MPFR 277; GMP 273; thread-local access 121; other 447. These sum to
3,846. This is sampled frame attribution, not precise source-level exclusive
CPU accounting. Its nested-stack self-check and conservation assertions pass;
the original summarizer independently agrees on the four shared inclusive
categories. No theoretical speedup is calculated from these percentages.

## Contract that bounds the next change

Current source is decisive:

* [`PredictorNumber`](../../rust/src/predictor_number.rs) exposes
  `as_float(&self) -> &Float`, `from_float`, conversions, formatting,
  serialization, comparisons and owned/borrowed operators. A caller can retain
  the shared Float borrow. Replacing the wrapper with bare f64 or returning a
  temporary Float would break that API/lifetime contract.
* [`predictor_ffi::write`](../../rust/src/predictor_ffi.rs) borrows the Float's
  sign, class, significand and exponent. Public native coefficient storage is
  reread on every call; caller mutation remains observable.
* [`cheby2d_int`](../../rust/src/cheby2d_int.rs) borrows native predictors into
  Rug128 timing expressions; `quad_cos` deliberately computes at 113 bits and
  narrows afterward. [`t1polyco`](../../rust/src/t1polyco.rs) uses the Float
  borrow for source formatting. These are real boundaries, not replaceable by
  global f64 conversion. The [public predictor contract](../../rust/PREDICTOR_API.md)
  retains 53-bit macOS / 64-bit Linux ordinary values and Rug128 timing.
* [`Cheby2D::evaluate`](../../rust/src/cheby2d.rs) needs only ordered multiply,
  add, unary negation, constants and its existing source FMA operation. Model
  normalization/dispersion/division, parsing, construction, differentiation,
  sqrt, transcendental work and T1 arithmetic can retain their current owners.
* Existing operations explicitly preserve signed zeros, range/subnormal
  rounding and operation-specific NaN signs. `to_f64` follows MPFR's NaN
  conversion, and the macOS FMA result re-enters the fresh-result NaN convention.
  Hardware NaN signs alone cannot stand in for those rules.

## Established facilities assessed

| Facility | Verified fit and limitation | Disposition |
| --- | --- | --- |
| Existing safe Rug assignment and local Float scratch | Returns the rounding direction needed by the native subnormal boundary; reusable owners can reduce remaining allocation. MPFR arithmetic and three Float-to-f64 conversions plus assignment at each source FMA remain. | Smaller risk, but addresses only part of the observed evaluator cost. No assumed speedup. |
| Rug 1.30.0 `MiniFloat` | Inline limbs, primitive-dependent precision, safe `borrow()` returns `BorrowFloat`; `borrow_excl()` needs `&mut self`. Mutating Float access is unsafe and forbids precision changes/swaps. It does not directly provide the current shared `&Float` return contract. | Useful for small constants, not a drop-in numeric owner. Do not use deprecated `SmallFloat`, which now allocates after a soundness fix. |
| Standard Rust f64 | Binary64 matches the macOS ordinary significand/range; `mul_add` has specified fused rounding. It does not represent the Linux ordinary 64-bit significand/range. Rust does not guarantee arithmetic NaN signs. | Suitable candidate for private macOS recurrence temporaries, with explicit existing class/sign semantics and exact qualification. |
| `rustc_apfloat` 0.2.3+llvm-462a31f5a5ab | LLVM-derived software arithmetic includes `X87DoubleExtended`, the needed leaf operations, explicit rounding and allocation-free arithmetic. This is software, not hardware x87 execution; its API is explicitly unstable and its NaN rules differ from this MPFR wrapper. | Credible later Linux facility to assess, not a tested backend or new dependency recommendation now. |
| C long double / custom x87 assembly / binary128 | C/assembly adds a new foreign or unsafe scientific boundary. Binary128 has 113 significand bits and different rounding points from ordinary Linux long double. | Outside this bounded proposal; neither a transparent f64 widening nor a precision-preserving shortcut. |

Rug findings come from the pinned installed `float/mini.rs`, `small.rs`,
`borrow.rs` and arithmetic sources, retained by hash in the evidence receipt;
the [Rug documentation](https://docs.rs/rug/latest/rug/float/index.html) identifies
these facilities. Standard-library claims are documented by
[f64](https://doc.rust-lang.org/stable/core/primitive.f64.html),
[mul_add](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.mul_add),
and [f128](https://doc.rust-lang.org/stable/core/primitive.f128.html).
The Rust reference lists [stable floating types](https://doc.rust-lang.org/stable/reference/types/numeric.html#floating-point-types),
with no x87 primitive. APFloat's [implementation contract](https://docs.rs/rustc_apfloat/latest/rustc_apfloat/),
[format list](https://docs.rs/rustc_apfloat/latest/rustc_apfloat/ieee/index.html)
and [operation contract](https://docs.rs/rustc_apfloat/latest/rustc_apfloat/trait.Float.html)
support the assessment above. Documentation establishes capabilities, not
equivalence or performance on this repository. No APFloat code was installed.

## One proposed implementation release

Authorize an internal, compile-time macOS arithmetic specialization of the
**existing single recurrence**, using a small private operation adapter rather
than duplicating its loop. Convert each already-validated coefficient/input
at its use boundary and return an ordinary `PredictorNumber` at the end.
Retain the existing N-based instantiation on Linux with separate multiply/add.
The public `Cheby2D::evaluate` signature and `PredictorNumber(Float)` stay intact;
there is no public backend selector, pointer cache, coefficient snapshot cache
or alternate evaluator. Expected production scope is `cheby2d.rs` and a small
private arithmetic adapter, plus focused tests. C transport, models, T1,
Rug128, constructors and public borrowing/formatting remain unchanged.

This removes repeated MPFR-backed temporary arithmetic and conversion from the
macOS recurrence while preserving its mathematical method and explicit source
contraction sites. It is a proposal for a substantial measured target, **not an
implemented or proven speedup**. Reconstruction and model costs remain; it will
not by itself close the full native gap. Linux receives no claimed performance
gain. A cross-platform native-width replacement would need a separate decision
and actual 64-bit-profile evidence.

Required qualification before acceptance:

1. Compare the private arithmetic against frozen N operations at every existing
   rounding point: ties/cancellation, signed zero, normal/subnormal transitions,
   both overflow signs and nonfinite inputs. Preserve unary-negation signs and
   source FMA NaN conversion explicitly; retain the supported floating-point
   environment and gradual underflow. Inspect optimized contraction sites;
   do not add blanket fusion or relax signs.
2. Run the accepted exact 5,880-edge / 28-finite direct witnesses and 30,258-line
   full C witness, native gates, mutation/restoration, affected construction/test
   callbacks and installed `as_float` consumers. Test unchanged Linux Rug64
   behavior on Linux, including its wide-range and subnormal midpoint cases.
   Keep inherited T1 NaN and polyco log10RMS failures visible.
3. Measure the final canonical direct leaf and full installed C route with the
   existing fresh interleaved uninstrumented recipe; observe allocations
   separately. No speedup threshold is inferred from this profile. Reject an
   equivalence regression regardless of apparent speed.

The consequential central decision is whether to release this **private macOS
hardware arithmetic specialization with a shared recurrence**, while preserving
the public Float API and Linux implementation. The current assessment grants
no such authorization.

## Reproduction and retained evidence

New ignored root: `build/next-cpu-20261102/`. It contains the complete before/after
controls, process snapshots, original stack trace, plain/sample outputs,
`costs.json`, input hashes, loaded UUID, pinned contract-source identities and
an evidence manifest. Earlier manifests retain their original hashes.

`receipt.json` binds 18 new evidence files / 2,943,035 bytes and the inspected
source/recipe identities. Its SHA-256 is
`19545358a383639ebcdb17e9ee360773b09ff6c2cf3c2260dfd34dacb524a7b6`;
the unchanged sampling runner's `profile/receipt.json` hashes to
`883d3ae058e58a81342b5e32c1fffd6b4d57c0058712f978b5db2e74024bba1f`.

Replay only after the short measurement window is clear, using a fresh output path:

```sh
export AO_NEXT=/path/to/fresh/next-profile
export AO_BASE=validation/predictor_performance/build/evaluator-20261102
export AO_INPUT=validation/predictor_performance/build/t2pred.dat
mkdir -p "$AO_NEXT"
"$AO_BASE/verify/after-outputs" "$AO_INPUT" > "$AO_NEXT/control-before.stdout"
cmp "$AO_NEXT/control-before.stdout" "$AO_BASE/verify/after-outputs.stdout"
python3 validation/predictor_performance/sample.py \
  --executable "$AO_BASE/verify/after-evaluation" --predictor "$AO_INPUT" \
  --out "$AO_NEXT/profile" --iterations 800000
"$AO_BASE/verify/after-outputs" "$AO_INPUT" > "$AO_NEXT/control-after.stdout"
cmp "$AO_NEXT/control-before.stdout" "$AO_NEXT/control-after.stdout"
python3 validation/predictor_performance/profile_costs.py \
  "$AO_NEXT/profile/sample.txt" > "$AO_NEXT/costs.json"
```

Only this assessment and the small trace-attribution recipe are authored changes.
The profile parser's assertions and fresh complete output checks pass. There is
no new production implementation to test, and no new Linux execution or full
timing/fitting campaign is claimed.
