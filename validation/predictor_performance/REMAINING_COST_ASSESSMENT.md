# AO remaining complete-call cost assessment

4 November 2026, decision 0139. **Recommend a bounded reuse slice within the
existing Rug path:** consume the already-dead selection FMA owner, replace the
seven-scalar model Vec with a fixed array, and use Rug's existing rounded
primitive-operand operations instead of allocating temporary N constants.
These preserve the canonical scientific expressions and public Float contract.
No production change or candidate speedup is implemented or claimed here.

The fresh profile places the recurrence in 7.20% of stacks and publication in
0.44%; remaining work is spread over scalar materialization, model arithmetic
and selection. A larger hardware model-arithmetic extension is a separate
option with substantially more semantic proof obligations. The pending release
budget is not resolved by this assessment.

## Immutable baseline and new profile

Use accepted production `a1851c55fcc0b1ac733b064e631bfbbd2e0118d5`, delivered
by `4f37bc378d51fac94b6bc3c12719e45131cfa612`. Its installed shared library is
`build/coefficient-view-20261104/after-v1/prefix/lib/libtempo2pred_rust.dylib`,
SHA-256 `9824a8f467b25a4b18abc2bdf4b395db794464ada24ffb9ebed316151a1a19bc`.
The sampled UUID `5D1245CD-5383-3A7A-BC0A-55B5EAED2E9E` agrees with its actual
load commands; the client UUID agrees as well. The unchanged one-segment,
8-by-4 predictor SHA-256 is
`a8b4242d2600cc72198a7f40240b0e946d17d173804e6d10d7f5f31c9f8811ee`.

After explicit central clearance and root/AL holds, the unchanged installed
complete-call client ran four million phase/frequency pairs plain, then sampled
for five seconds at 1 ms. The window was **10:00:47–10:01:03 UTC** (21:00
Melbourne). Loaded intervals are **8.328101 / 8.403810 s**, with matching
eight-million-call checksum `0x1.3d0a3259d95a9p+39`. Their 0.91% difference mixes
noise and instrumentation; it is not an isolated overhead measurement or a
new seven-sample performance campaign. Allocation observers were absent.
Boundary process snapshots contain no competing compiler/benchmark. Both
**30,258-line** output controls exactly match the accepted baseline.

There are **3,835 sampled stacks**. Percentages use this common denominator.
Inclusive rows overlap and must not be added or converted into a speedup forecast.

| Reported responsibility | Stacks | Percent |
| --- | ---: | ---: |
| Complete Rust model entry, `t2r_model` | 2,661 | 69.39% |
| Rust segment selection, `t2r_nearest` | 1,099 | 28.66% |
| Scalar vector construction, all `values` call trees | 1,039 | 27.09% |
| Scalar `read`, all callers | 703 | 18.33% |
| Native `t2r_ld_parts`, all callers | 125 | 3.26% |
| Model normalization including recurrence | 1,164 | 30.35% |
| Canonical recurrence, either storage specialization | 276 | 7.20% |
| Result publication, Rust `write` / native parts writer | 17 | 0.44% |

Of the scalar-vector stacks, 820 have the model-entry ancestor and 215 the
selection ancestor; scalar reads split 495 / 206. The remaining small counts
lack those reported ancestors. Subtracting nested call trees gives **890
normalization stacks outside recurrence (23.21%)**, **86 phase-helper stacks
outside normalization (2.24%)**, and **252 frequency-helper stacks outside
normalization (6.57%)**. These contain function-local arithmetic and cleanup;
they do not account for later destruction in the C entry. Two recurrence stacks
lack the reported normalization ancestor and are retained separately.

Named allocator subtrees occur in 1,831 stacks and MPFR init/clear in 1,660.
The disjoint partition by reported current frame is allocator 1,717; double
conversion 145; arithmetic 339; range 201; init/clear 176; other MPFR 123; GMP
259; thread-local access 84; other 791. These sum to 3,835. The allocator partition
also recognizes the allocator image, whereas the legacy inclusive matcher uses
symbol names. Inlining, tail calls and cleanup attribution limit interpretation.

`remaining_profile.py` reuses `costs` and `call_subtrees`; it adds current
model/selection/publication scopes and recognizes the generic C recurrence.
The unchanged `sample.py` summary's old evaluator pattern recognizes only the
owned symbol and is not used as the current recurrence total. Nested-root,
different-responsibility and conservation controls pass, and the new parser
agrees with the original summarizer's shared counters on both the saved prior
trace and this fresh trace. The saved-trace check is not another timing run.

## Ownership account and what the public API requires

The current accepted allocation observations are **60 mallocs + two reallocs
= 62 requests / 1,920 requested bytes per full C pair**. Direct owned leaf and
model pairs use two and 22 mallocs respectively. These are retained measurements
from the accepted delivery, not remeasured during this profile.

For this finite fixture with one containing segment, source inspection accounts
for the 62 requests as follows. The per-responsibility attribution is an
inference from source and the observed totals, not separately measured timings.

| Current source work per pair | Allocation requests |
| --- | ---: |
| Seven model scalar N owners per call | 14 |
| Selection MJD and two range-bound N owners per call | 6 |
| Model/recurrence arithmetic owners, including constants/results | 22 |
| Selection arithmetic owners | 12 |
| Two C range arrays and four Rust scalar vectors | 6 |
| Model scalar-vector growth | 2 |

Selection's twelve arithmetic owners include `best`, `start+end`, the clone
inside borrowed `source_mul_add`, the 0.5 operand, cloned/negated MJD and the
`abs_double` result, for each of the two calls. Model work has eight owners per
normalization call, one recurrence result, plus one phase correction owner or
three frequency-scaling owners: ten + twelve per pair. Nonfinite/range cases
and larger segment sets can have different counts.

Public `PredictorNumber(Float)` values, existing `&Float` references and the
owned coefficient slice require their established owners/lifetimes. A public
Rust evaluation result needs its independent N owner. Those are not candidates
for a global f64/enum/stack-storage replacement. Internal constants, intermediate
results and scalar containers are temporary implementation work. In a complete
C call even the final N is internal—the external result is a native scalar—but
eliminating it would change the evaluator/result boundary. Publication samples
do not isolate that owner's creation or later destruction. Keeping the current
output path is the bounded choice.

## Established facilities and alternatives

The pinned Rug 1.30.0 `float/arith.rs` implements rounded primitive `f64` operands
for all four operations. Its `PrimOps<f64>` forwards to MPFR `add_d`, `sub_d`,
`mul_d` and `div_d`. The bound MPFR 4.2.2 `*_d.c` sources use a stack temporary
for the exact double operand, call the corresponding MPFR operation and retain
the rounding result. No new numeric library or hand-built MPFR layout is needed.
Rug documents the rounding-direction result and direct primitive-operand support
in its [Float API](https://docs.rs/rug/latest/rug/struct.Float.html) and
[rounded assignment API](https://docs.rs/rug/latest/rug/ops/trait.MulAssignRound.html).
The [MPFR 4.2.2 manual](https://mpfr.org/mpfr-current/mpfr.html) documents the
mixed arithmetic operations. Pinned local source identities are the version
evidence; documented support is not qualification of a future N implementation.

| Alternative | Benefit and boundary | Assessment |
| --- | --- | --- |
| Existing consumed-owner FMA | Removes a clone of the dead `start+end` result without changing selection expressions | Include |
| Fixed seven-element N array | Removes model container allocation/growth; keeps scalar reads and owners | Include |
| Direct Rug `f64` right operands | Removes temporary constant owners in owned operations; retains MPFR arithmetic, range logic and direction | Include |
| Direct native scalar-to-N conversion | Still needs each N owner; changes more transport paths and mostly avoids the smaller parts-conversion component | Defer |
| `MiniFloat` throughout internal arithmetic | Provides stack-backed operands, but mutating its Float view requires non-reallocation guarantees and a compatible range/ownership design | Defer |
| Extend the private hardware adapter to all model arithmetic | Could remove many private N temporaries; requires Sub/Mul/Div class rules, revised input/output boundaries and shared model plumbing | Separate design/release |

`MiniFloat` was checked in pinned `float/mini.rs`: `borrow()` returns a temporary
`BorrowFloat`; `borrow_excl()` requires mutable access; unsafe
`as_nonreallocating_float()` forbids precision changes and swaps. The existing
range helper consumes an owned Float and may replace it on overflow. A MiniFloat
substitution therefore cannot simply reuse that ownership path or the public
`as_float(&self) -> &Float` contract. Direct Rug primitive operands already supply
the useful stack-operand behavior without introducing that machinery.

The current private recurrence Number has Add/Neg/FMA, not model Sub/Mul/Div.
Extending it would need separately rounded normalization and scaling, no new
contractions, and MPFR-compatible division NaN signs: input-NaN propagation
differs from invalid 0/0 or infinity/infinity. A blanket positive-NaN wrapper
would not implement every primitive's contract. Hardware subnormal and signed-
zero proof plus actual Linux separation would also be required. Publication
optimization or another recurrence rewrite is not justified as the first step.

## Proposed dependency-ordered slice and proof obligations

This is a proposal for central release, with no production patch written.

1. **`cheby2d.rs`: consume the existing selection sum.** Change the already-dead
   `(start + end)` owner to the existing `source_mul_add_owned` call. Retain
   operand order, the explicit FMA/separate Linux boundary, `-mjd.clone()`,
   `abs_double`, the 1e6 initial best distance, inclusive endpoints and strict
   tie comparison. Do not replace or duplicate selection logic.
2. **`predictor_ffi.rs`: use a fixed `[N; 7]` in `t2r_model`.** Read indices
   0 through 6 explicitly with the existing fallible `read` calls. Retain the
   frequency path's unused index-4 dispersion slot, current failure ordering and cleanup
   of partially constructed owners. Scalar reads still precede coefficient
   validation; the coefficient slice remains call-local and ends before output.
   Keep general `values`, T1, callbacks, derivatives and publication unchanged.
3. **`predictor_number.rs`: replace the two `f64` RHS implementations in the
   existing arithmetic macro.** Owned N calls the existing Rug rounded assign
   operation directly with the f64 operand and then `rounded_reused`; borrowed N
   uses the corresponding incomplete mixed operation with `Float::with_val_round`
   and `rounded`. Keep one macro and all N/N variants. Borrowed arithmetic still
   needs its result owner; it must not mutate a live input borrow.

For step 3, every finite f64 operand is exactly representable in both qualified ordinary
profiles. The operation's exact inputs, destination precision and nearest-even
mode therefore stay the same without the temporary N. Feed the actual returned
Ordering into the existing `subnormalize_round`/overflow path; never substitute
Equal, narrow a Linux left operand to f64, or use default rounded assignments
that lose the direction. Preserve the existing clear-NaN decision: Add/Sub/Mul
fresh-result signs, and Div's input-NaN propagation versus invalid-operation
sign. Use `rhs.is_nan()` for the primitive operand, with the unchanged left-owner
check. Do not infer class/sign equivalence solely from finite value equality.

Source accounting identifies two selection clones, six owned constant operands
in the model expressions, and four model-container requests per pair as the
target of these edits. Removal of twelve requests is a testable source hypothesis,
not a measured allocation count or speedup. No result is presumed from the
47.74% inclusive allocator fraction. Actual timing may show little benefit.

Only those three production files and focused existing tests/recipes are expected
to change. No new unsafe code, C kernel, backend trait, cache, persistent borrow,
public precision/ownership change or native-sign-policy change is proposed.
The separate Rug128 timing type and its arithmetic remain outside this slice.

## Required witnesses and portable replay

Extend the existing mixed-operand ownership matrix in `predictor_tests.rs` using
the unchanged N/N borrowed operation as oracle. Cover owned and borrowed left
operands, all four operations, signed zero, signed NaNs, infinities, maxima,
normal/subnormal neighbors and the first-direction midpoint. Preserve the Linux
left values requiring 64 significand bits and wide exponents. Keep live Float
borrows across the new mixed operations. Selection tests must include ties,
overlaps, endpoints, gaps, sentinel-distance limits and nonfinite metadata.

Reuse the coefficient witness, thirteen ordered diagnostics, callback snapshot,
derivative alias and complete C mutation/restoration controls. All 5,880 direct
edge results, 28 finite results, 30,258 C lines and 38 legacy products must still
match the accepted baseline. Since the numeric macro is shared, retained T1
count/class/sign/reader/plan checks remain mandatory even though T1 expressions
are untouched. Preserve installed consumers and caller-exclusive sanitizer/cache
cleanup checks. Sanitizer scope remains C clients/shim, not all Rust/MPFR code.

For both macOS and actual Linux x86_64, use the complete
[coefficient-view recipe](COEFFICIENT_VIEW.md), substituting:

```sh
# Fresh directories and the existing prepared offline cache/toolchain.
python3 validation/predictor_performance/build.py \
  --revision a1851c55fcc0b1ac733b064e631bfbbd2e0118d5 \
  --out "$AO_OUT/before" --cargo-home "$CARGO_HOME"
python3 validation/predictor_performance/build.py \
  --revision "$AO_CANDIDATE" \
  --out "$AO_OUT/after" --cargo-home "$CARGO_HOME"
```

`AO_CANDIDATE` must be the exact later centrally released implementation commit.
Run the recipe's complete direct/C/consumer/debug/release/sanitizer checks, with
Linux `1+2^-63`, `2^-16445`, `2^16382` and separate source operations. Then, in a
cleared window, use the unchanged seven-sample interleaved timing runners and
separate allocation observers. Compare each small edit against the immutable
baseline before interpreting a combined gain. No cfg-swapped macOS test stands
in for Linux execution. No candidate or Linux execution occurred in this assessment.

## Retained assessment evidence

New ignored root: `build/remaining-costs-20261104/`. It retains the raw trace,
plain/sampled outputs, full controls, process snapshots, explicit clearance,
loaded-image identities, source/primitive contracts, parser controls and prior
baseline bindings. Its `receipt.json` SHA-256 is
`d1d871940fa880d5ef3f953a3ac2aa99a48b600ef7db03857de3dc9ab8e3c8e9`:
27 files / 3,150,913 bytes, 64 external identities, 37 Git source bindings and
eight dependency-source bindings. Both crate archives match Cargo.lock checksums,
and the inspected dependency files match their archive entries. Six previous
manifests and six selected accepted products remain unchanged. The manifest
excludes itself; this Markdown is bound by its delivery commit.

The native T1 NaN-sign mismatch and polyco log10RMS failures remain open.
Production and previous evidence are unchanged; central must release any
implementation. No push or dispatch was performed.
