# Remaining model/scalar cost assessment

4 November 2026, decision 0145. **Recommend one bounded model-scope slice using
the existing private evaluation representation.** Keep normalization,
dispersion, frequency scaling and the recurrence in that representation until
the final public N result; copy the seven macOS C inputs into it using the
already qualified binary64 storage contract. Keep selection and result
publication on their accepted paths. No implementation, alternate evaluator,
new arithmetic diagnostic or candidate performance run was made here.

The current profile and allocation account support this scope. It requires
explicit division NaN semantics and a public/private input bridge; it is more
than changing the recurrence's return type. Linux must continue through the
existing Rug64 Number alias without new input clones or f64 narrowing. No new
numeric backend, unsafe owner or public storage representation is needed.

## Current endpoint and fresh evidence

The accepted production endpoint is `e1d34d835377451dcc7822161afce96a87948005`,
delivered by `10e4b13319c260a5f0c55606ee3fb2f3c1fc1208`. The immutable install is
`build/temporary-reuse-20261104/after-v1/prefix`; its shared library SHA-256 is
`b9c3701d041aacc503d571339fc3ce696a60ff74e1180e4bf5831624c6ae473e`.
Actual load commands and sampled images agree for both library UUID
`9998D637-3A50-369A-8254-F8DF692C86C8` and client UUID
`D013CD34-6D0B-357C-B6C0-23728781FB1F`.

After a separate explicit central/AL clearance, the unchanged complete-call
client ran four million phase/frequency pairs plain and then sampled for five
seconds at 1 ms. The recorded window was **21:50:42.999–21:50:57.249 Melbourne**
on 4 November (**10:50:42.999–10:50:57.249 UTC**). Plain/sampled evaluation
intervals were **7.051005 / 7.042596 s**, both with eight-million-call checksum
`0x1.3d0a3259d95a9p+39`. This single pair mixes noise and instrumentation; it is
not a speedup or sampling-overhead estimate. No build or allocation observer ran.
Boundary snapshots show no competing compiler/benchmark. Both **30,258-line**
controls exactly match the accepted output, and input identities remain fixed.

This is a fresh profile of **e1d34d8**, distinct from the older coefficient-view
profile in `build/remaining-costs-20261104/` at **a1851c5**. The unchanged parser
reproduces that saved trace exactly and passes its nested-scope controls on the
new trace. Saved-trace replay is not an additional profiling run.

The new trace has **3,840 stacks**:

| Responsibility, inclusive unless stated | Stacks | Percent |
| --- | ---: | ---: |
| Complete Rust model entry | 2,476 | 64.48% |
| Rust segment selection | 1,268 | 33.02% |
| Scalar reads, all callers | 820 | 21.35% |
| Native scalar decomposition, all callers | 119 | 3.10% |
| Remaining scalar-vector builder, all callers | 237 | 6.17% |
| Model normalization including recurrence | 1,221 | 31.80% |
| Recurrence | 335 | 8.72% |
| Result publication | 23 | 0.60% |

There are **zero model-entry/vector-builder stacks**, versus 820 in the older
trace; 232 current vector stacks have the selection ancestor. Model/selection
read subtrees contain 554 / 252 stacks. Nested subtraction leaves **887
normalization stacks outside recurrence**, **102 phase-helper stacks outside
normalization**, and **209 frequency-helper stacks outside normalization**.
These regions include function-local cleanup. One recurrence stack lacks the
reported normalization ancestor; other small ancestry gaps remain unassigned.

Inclusive responsibilities overlap and must not be added. Their fractions are
compositional: changes from the older profile do not measure a responsibility's
absolute speedup. Recurrence also includes its current N-result construction;
publication does not capture every result owner's creation/destruction.

Named allocator subtrees occur in 1,638 stacks and MPFR init/clear in 1,713.
The disjoint reported-current-frame partition is allocator 1,611; double
conversion 196; arithmetic 397; range 178; init/clear 192; other MPFR 169; GMP
277; thread-local access 115; other 705. These sum to 3,840. Inlining and cleanup
attribution limit interpretation; these are not separately timed cost budgets.

## Current owners and bounded opportunity

Reuse the accepted allocation observations, not a new observation in this
assessment: **50 mallocs, zero per-pair calloc/realloc, 1,024 requested bytes**
per complete C pair. Direct owned leaf/model pairs use **two / sixteen** mallocs.
For the finite, one-containing-segment fixture, source inspection accounts for
the complete-call count as follows; individual rows are source attribution.

| Current source work per pair | Requests |
| --- | ---: |
| Seven model input N owners per call | 14 |
| Model arithmetic and recurrence-result owners | 16 |
| Selection MJD and two range-bound N owners per call | 6 |
| Selection arithmetic owners | 10 |
| Two native range arrays and two remaining Rust scalar vectors | 4 |
| Total | 50 |

Each model normalization currently needs six owners. One recurrence result and
one phase-square or frequency-extent owner bring each call to eight. Its result
owner is reused by later arithmetic. A private model path could replace the
fourteen input owners and fourteen internal arithmetic owners while keeping two
final N results: **50 → 22 requests is a testable source hypothesis**, not a
measurement or performance forecast. Retaining the same observed finite-owner
sizes suggests 576 requested bytes, also unmeasured. Selection's twenty
requests remain. Nonfinite cases and larger segment sets may differ.

Public N/Float owners, coefficient storage and live `&Float` borrows remain
required at public Rust boundaries. The C model's intermediate owners are
internal transport/arithmetic work. Retaining its final N keeps the current
writer and error/publication boundary simple; removing those last two owners
is outside this proposal. No predicted speedup follows from the profile shares
or removable-owner count.

## Reuse choice and arithmetic contract

The existing platform module already provides macOS `Number(f64)`, source FMA,
addition, unary negation and native coefficient inputs; Linux aliases Number
to N. Extend that module with only the model's needed Sub/Mul/Div operand forms.
Retain one model expression path and one Clenshaw loop. Do not build another
backend trait, runtime numeric enum, model kernel or C scientific implementation.

| Alternative | Assessment |
| --- | --- |
| Share the local N(-1) constant between the two normalization coordinates, using existing borrowed-left owner reuse | A smaller source hypothesis of two fewer requests per pair, but leaves almost all model input/arithmetic ownership intact |
| Reuse the private Number for model arithmetic, retain current scalar N reads | Useful implementation checkpoint; removes fourteen internal requests by source account while leaving fourteen input owners |
| Reuse private model arithmetic plus seven copied macOS scalar inputs | Recommended bounded final scope; targets the current model work without changing selection or publication |
| Change public N storage or arithmetic globally | Broader T1/public contracts; unnecessary for this scope |
| Introduce MiniFloat/unsafe Float views or another numeric dependency | Adds ownership/range machinery already avoided by the private representation |
| Optimize selection or write f64 directly to C output in the same slice | Separate boundaries and witnesses; defer |

Rug's rounded primitives remain the Linux implementation and independent N
oracle. The Float-oriented ordinary helpers do not remove temporary Float
ownership by themselves. For macOS public-input conversion, the existing
`ordinary_long_double::to_f64` can preserve NaN sign where needed; ordinary
`N::to_f64` uses MPFR get_d, which canonicalizes NaN sign. Neither conversion
may be used to narrow a Linux input.

The pinned MPFR `div.c` checks input NaNs before setting the quotient sign.
For non-NaN operands it sets the sign to the operand-sign XOR before handling
0/0 or infinity/infinity. Combined with N's fresh-result/owner-reuse rules, this
requires the following private semantics:

| Operation or boundary | Required rule |
| --- | --- |
| Add/Sub/Mul | Preserve finite rounding and signed zeros; NaN result has positive sign, as in the N oracle |
| Div with a NaN input | Positive NaN |
| Div with non-NaN inputs producing NaN (0/0 or infinity/infinity) | NaN sign is the XOR of input signs |
| Other Div results | Normal IEEE finite, infinity and signed-zero result under the qualified environment |
| Unary negation | Flip sign, including zero and NaN |
| Explicit source FMA | Keep the current platform-specific helper and its positive-NaN result rule |
| macOS native input copy | Apply the existing Darwin positive/quiet NaN input convention |
| Private result → public N | Keep the existing conversion; the current complete leaf/model expressions guarantee a positive NaN at this boundary |

Number's existing `From<f64>` cannot implement every division result: it
canonicalizes NaNs positive. Private division must distinguish input NaNs from
invalid division and enforce the sign rule after hardware arithmetic.

Publication has a narrower established invariant. The leaf ends in a positive-
NaN FMA; phase ends in a positive-NaN addition; frequency ends in division by
finite nonzero 86400, so any NaN there is propagated positively. Thus no negative
NaN reaches the current final boundary. Keep the existing N::from output
conversion and document/test these complete expressions. MPFR `set_d.c` would
discard a negative NaN sign, but adding a general output sign-restoration branch
is unnecessary for these call paths. No NaN payload or signaling-exception
contract is introduced.

Rust documents that arithmetic NaN signs are not stable, so an observed hardware
sign is not a portable policy ([f64 reference](https://doc.rust-lang.org/std/primitive.f64.html)).
The retained default environment is nearest-even with gradual underflow, with
no fast-math, extra contraction, reciprocal substitution or control-register
change ([LLVM environment](https://llvm.org/docs/LangRef.html#floating-point-environment)).
Linux keeps the actual returned rounding direction through N's existing range
handling; MPFR documents why subnormal rounding needs that direction
([MPFR manual](https://mpfr.org/mpfr-current/mpfr.html)). These document support
and obligations, not tested correctness of the proposed extension. The live
Rust documentation is newer than the accepted rustc 1.97.1 build; qualification
must inspect and execute the actual candidate toolchain.

Keep the precise source operation graph:

* Normalize `x = -1 + ((mjd - start) * 2) / (end - start)` and the analogous
  frequency coordinate, with every multiply/divide/add rounded separately.
* Run the same recurrence with its existing half weights, separate doubling,
  coefficient additions and explicit FMA sites.
* Phase is `P + D / (freq * freq)`, preserving the square, divide, then add.
* Frequency is `((P_x * 2) / (end - start)) / 86400`. Keep the second extent
  subtraction and final division; do not hoist, combine denominators or replace
  division with a reciprocal multiply.

Coordinates, phase and frequency retain their existing units and meaning.
The proposal changes temporary representation, not the accepted 53/64-bit
profiles, model assumptions or source rounding sites.

## Dependency-ordered implementation proposal

This section is for a later central implementation release.

1. **`cheby2d/evaluation.rs`: extend existing private primitives and bridges.**
   Add the Sub/Mul/Div ownership forms actually required by the shared model
   expressions, without a general operator framework. A small synchronous
   `with_inputs` bridge can serve the existing two-, six- and seven-input public
   call sites: stack Number copies on macOS, borrowed original N values on Linux.
   Its closure returns the complete leaf/model result; no input reference
   escapes. Keep coefficient Input distinct and unchanged. Retain the existing
   output conversion and its positive-NaN precondition.
2. **`cheby2d.rs`: carry Number through the existing model helpers and loop.**
   Give the same recurrence a private Number result; public leaf/model wrappers
   publish N once at their boundary. Use a private entry to that same loop so
   model calls do not rematerialize a public N between recurrence and scaling.
   Public method signatures, owners and coefficient borrows remain unchanged.
   On Linux the alias/bridge must reproduce existing N expressions without new
   clones, f64 conversions or a second rounding pass.
3. **`predictor_ffi.rs`: copy only model scalar inputs into the private type.**
   First qualify the shared model with existing N reads as a checkpoint. Then
   use the existing macOS ABI assertions and input-NaN normalizer for seven
   ordered typed copies; Linux continues through the existing N reader. The
   narrow raw reads need documented alignment/extent/lifetime preconditions,
   not an unsafe owner, pointer cache or new C arithmetic accessor. Keep the
   general reader and C leaf scalar transport on their accepted paths. Adapt
   the synchronous evaluator boundary to the private result and call the
   existing writer only after conversion to N.
4. **Focused witnesses and API/recipe documentation.** Extend the existing
   private primitive matrix, public-borrow consumer and coefficient/model
   witness. Keep `predictor_number.rs` and ordinary arithmetic unchanged as
   the reference. Update only the documented private model boundary and replay
   recipe. Use one final combined qualification/timing campaign after gates.

For C, retain all seven reads, including the unused frequency dispersion slot,
before coefficient validation. Preserve null/dimension/error precedence and
unchanged output on failure. Scalar copies must finish before computation;
coefficient borrows must end before output writes. Include output aliasing a
scalar slot or coefficient cell, mutation/restoration and pointer replacement.
Callbacks/derivatives keep their owning snapshots. T1, selection, general
numeric transport and the separate Rug128 timing type remain outside the slice.

## Required evidence before accepting implementation

* Compare every new private operation and ownership form against unchanged N
  operations over signed boundaries, zeros, infinities, both NaN signs and
  reproducible finite bit patterns. Explicitly cover all sign combinations of
  0/0 and infinity/infinity and propagated NaNs. Check raw private division
  results separately from publication after the complete leaf/model expression.
  Keep the
  first-rounding midpoint, overflow, underflow, cancellation and anti-contraction
  witnesses. Finite equality alone does not establish class/sign equivalence.
* Extend C model controls to vary each of seven scalar fields, both selected
  polynomials, degenerate/adjacent extents and coupled cancellation cases.
  Existing coefficient-heavy controls alone do not exercise all these inputs.
  Compare complete values/classes/signs and diagnostics against immutable e1d34d8.
* Extend the installed public-borrow consumer through phase/frequency evaluation,
  keeping input, metadata, coefficient and prior-result Float references live.
  Reuse callback/derivative alias controls and caller-exclusive sanitizer/cleanup
  checks. Sanitizer scope remains C clients/shim, not all Rust/MPFR code.
* Run the complete [temporary-reuse recipe](TEMPORARY_REUSE.md) with e1d34d8 as
  before and the exact future candidate as after, using fresh output directories.
  Preserve all 5,880 direct edges, 28 finite outputs, 30,258 C lines, 38 legacy
  products, 8,516 existing coefficient lines and thirteen ordered diagnostics;
  add the new model witnesses rather than replacing those controls.
* Execute on actual Linux x86_64, with `1+2^-63`, `2^-16445`, `2^16382`, maximum
  range and both debug/release midpoint tests. Verify tests actually ran. Inspect
  the actual installed machine code/IR for separate model operations, unchanged
  source FMA sites and no forbidden narrowing. No macOS cfg substitution qualifies
  Linux. After correctness, separately observe allocations and use one centrally
  cleared unchanged seven-sample direct/full-C campaign; report all samples.

The accepted e1d34d8 Linux wiring remains AL's separate work. This assessment
does not claim a future candidate result or close inherited T1 NaN-sign,
polyco log10RMS or release-performance-budget issues.

## Retained assessment packet

`build/model-scalar-assessment-20261104/receipt.json` SHA-256:
`678bbdf8bd2507952d37e3e2d2cbd0e38d67ecf38fbf0751fa2ac82ad0f523a1`.
It binds **27 files / 3,148,981 bytes**, **76 external identities**, **40 Git
source bindings** and **14 dependency-source bindings**. Pinned crate archives
match Cargo.lock; inspected dependency files match their archive entries.
Eight prior manifests and eight selected accepted products remain unchanged.
The packet retains the new raw profile, old-trace parser replay, process
snapshots, controls, loaded identities, explicit clearance and start/end times.
The manifest excludes itself; this assessment is bound by its delivery commit.
