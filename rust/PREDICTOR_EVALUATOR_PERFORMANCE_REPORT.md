# AO continuation: canonical predictor owner reuse

2 November 2026. Delivered under handoff 42 and decision 0124 for central review.
On the measured macOS arm64 workload, reusing consumed numeric owners makes
the complete installed C phase/frequency loop **1.6234 times faster**, reducing
its median loaded time **38.4%**. Direct polynomial evaluation is **1.9940 times
faster**. Complete before/after results agree. Actual Linux execution remains
pending; this is a bounded performance result, not full predictor-science or
migration acceptance.

## Frozen pair and scope

* Accepted transport baseline: `0bd8e2cf2ef54e9da6feace122acd8aa96aa3fbe`.
* Owner-reuse production: `8716d1dc026641f3a51c2ba88850c5599293cbdd`.
* Continuation recipes: `2b963c39b7bbb580ede8256ac24404bd6694072b`.
* Native reference: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, with the unchanged
  installed qualifier's recorded source corrections.

Work remains in `/private/tmp/tempo2-ao-predictor-20261102`. The production commit
descends from delivered `f077292`; intervening commits added tests, recipes and
the transport report. The actual reused baseline install was built at `0bd8e2c`,
and is identified as such. Its 600 exported crate files and the candidate's
601 files differ in `src/predictor_number.rs`, `src/cheby2d.rs`, focused
`src/predictor_tests.rs`, and the intervening transport report. No other crate
content changes. The installed source selections also match their Git blobs:
262 baseline files and 263 candidate files, following the existing installer.

Main, completed transport evidence, public headers/ABI, C/FFI transport,
dependencies and `ordinary_long_double` are unchanged. There is no new unsafe
code, alternate evaluator, cache, configuration or persistent scratch owner.
T1 expressions and Cheby construction/selection/parsing are unchanged; effects
of their shared numeric operations are covered by the affected existing checks.
No push or workflow dispatch was made. AL's frozen Linux transport qualification
continues independently; central owns integration and acceptance.

The host is macOS 26.7 arm64, Rust/Cargo 1.97.1, Apple Clang 21.0.0,
GCC/GFortran 16.2.0, Rug 1.30.0 and gmp-mpfr-sys 1.7.1 / MPFR 4.2.2.
The measured ordinary profile is 53 bits; the Linux branch retains 64 bits and
separate source operations. Timing remains Rug128. Builds use distinct targets
and the existing prepared offline Cargo cache.

The unchanged 8-by-4 `t2pred.dat` input has SHA-256
`a8b4242d2600cc72198a7f40240b0e946d17d173804e6d10d7f5f31c9f8811ee`.
The full C loop makes separate phase and frequency calls at seven epochs
58000.010000 through 58000.010006 and 1400 MHz, including native storage
reconstruction, model selection, canonical evaluation and output transport.

## Measured cause and established facilities

The accepted transport profile already places canonical Cheby evaluation in
66.24% of sampled stacks and allocator subtrees in 54.53%. These are inclusive,
overlapping fractions from that prior execution, not new timings or an exclusive
cost decomposition. The continuation adds only the direct observations needed
to locate removable ownership costs. Before implementation, an 8-by-4 leaf
pair makes 472 malloc calls, a direct model pair makes 519, and the installed C
pair makes 639 allocation requests. Constructing `N` from f64 allocates twice;
one multiply with an owned operand, including its input clone, also allocates
twice. Source inspection identifies the redundant copies: owned operators
delegate to fresh borrowed results, and f64 construction creates then copies
a temporary Float.

This is allocation reuse within the existing Clenshaw recurrence. A replacement
polynomial method or numerical library is unnecessary. The pinned Rug source
and documentation provide the required safe operations:

* `AddAssignRound`, `SubAssignRound`, `MulAssignRound`, `DivAssignRound` mutate
  an existing owner and return the first rounding direction. Their `FromRound`
  counterparts preserve operand order when the right operand owns the storage,
  including subtraction and division.
* `AssignRound` reuses storage for the existing macOS f64 `mul_add` result.
  The Linux branch still performs separately rounded multiply and add. Rug's
  fused multiply/add would change that boundary and is not substituted there.
* Ordinary `+=`/`*=` convenience operations do not expose the direction needed
  at the subsequent native subnormal boundary. Discarding it or replacing it
  with `Equal` can turn a value just above half the minimum subnormal into zero.
  Assignment methods with an explicit rounding result preserve that contract.
* A single f64-to-Float construction suffices because both supported ordinary
  profiles represent every finite f64 value. It removes a copy without narrowing
  any general predictor scalar or changing precision.

These facilities avoid a new MPFR ABI/raw-pointer interface or custom allocator.
Documented API support is distinct from the executed macOS correctness and
performance checks below. The exact pinned Rug/MPFR source identities are bound
in the delivery receipt, including MPFR's special-case arithmetic.

## Preserved arithmetic and ownership

Owned arithmetic now consumes either available owner through the safe Rug
assignment API. Borrowed/borrowed arithmetic retains the original fresh-result
path. Both feed the unchanged native range/subnormal function with the actual
first rounding direction. Cheby evaluation consumes only already-dead
temporaries at the existing source multiply/add sites. Every scientific
expression, operand order and rounding/contraction boundary is retained.
Coefficient storage remains borrowed, and native cells are reread on each C call.

One actual aliasing trap required explicit treatment. MPFR can set a destination
to NaN without changing its old sign, whereas a fresh result starts positive.
Consumed-owner operations restore the existing fresh-result sign in those
cases. Division has a different source rule: after excluding NaN inputs,
MPFR sets the quotient sign before producing NaN for zero/zero or infinity/
infinity. Those signs must remain. The first expanded test caught an incorrect
blanket sign reset; the final implementation retains division's explicit sign
and resets only propagated input NaNs there. The failed test log is retained.
This preserves baseline output conventions; it does not adopt a new NaN policy.

Signed zero, infinities, overflow and subnormal boundaries remain covered.
The targeted midpoint regression constructs an exact product just above half
the minimum subnormal whose first p-bit result is exactly the midpoint; it
requires the smallest signed subnormal, in both ownership directions. The test
is parameterized for p=53/64 and their respective minimum exponents. The 64-bit
execution remains an explicit Linux obligation.

## Allocation observations

Whole-process instrumented runs at 1 and 1,001 iterations are subtracted.
Both raw records are retained; allocation observers are excluded from timings.

| Scope, per iteration | Before malloc | After malloc | Realloc before/after |
| --- | ---: | ---: | ---: |
| f64 construction | 2 | 1 | 0 / 0 |
| Owned multiply, including input clone | 2 | 1 | 0 / 0 |
| Two direct polynomial evaluations | 472 | 110 | 0 / 0 |
| Direct model phase/frequency pair | 519 | 130 | 0 / 0 |
| Complete installed C phase/frequency pair | 631 | 234 | 8 / 8 |

The direct leaf pair removes 76.7% of allocation requests; the full C pair
removes 62.1%, from 639 to 242. Full C requested bytes fall from 14,864 to 8,512
per pair, a 42.7% reduction. Free-call deltas match malloc deltas in each scope;
calloc deltas are zero. Direct leaf requested-byte deltas are 7552.003 and
1760.003 per pair, and direct model deltas are 8304.003 and 2080.003. The .003
comes from the three-byte argv-length difference between 1 and 1001; it remains
visible rather than being silently corrected.

These are external allocation requests and requested volume, not live memory
or peak RSS. macOS dyld interposition does not see private allocator calls hidden
inside libc. The prepared Linux static `--wrap` route has a different observation
scope and has not been executed for this continuation.

## Uninstrumented timings

All builds and checks finished before the announced sustained timing window.
The two campaigns ran sequentially: 28 fresh direct-client processes shuffled
with seed 124, then 63 fresh C/native/startup processes shuffled with seed 123.
Each table entry has seven samples. All outliers remain in the evidence.
Host snapshots show ordinary desktop activity; this is not a reserved idle host.
No local compiler or competing benchmark appears in the recorded boundary
snapshots. Client intervals exclude setup/parse; wall/CPU/RSS cover the process.

For 100,000 iterations of each scope:

| Scope / engine | Loaded median (min–max), s | Wall median, s | CPU median, s | Peak RSS median, MiB |
| --- | ---: | ---: | ---: | ---: |
| Direct leaf pair, before | 1.238879 (1.210040–1.486718) | 1.245398 | 1.23 | 1.844 |
| Direct leaf pair, after | 0.621293 (0.606492–0.700261) | 0.626368 | 0.62 | 1.844 |
| Direct model pair, before | 1.346498 (1.343031–1.444235) | 1.352015 | 1.34 | 1.844 |
| Direct model pair, after | 0.687565 (0.669520–0.709102) | 0.692443 | 0.68 | 1.828 |
| Complete C pair, before | 1.750129 (1.735351–1.776965) | 1.755126 | 1.74 | 2.094 |
| Complete C pair, after | 1.078071 (1.065014–1.087698) | 1.082947 | 1.07 | 2.016 |
| Complete native C pair | 0.004528 (0.004450–0.004652) | 0.008649 | 0.00 | 1.422 |

The direct leaf/model speedups are 1.9940x / 1.9584x, with 49.9% / 48.9% less
loaded time. The full C speedup is 1.6234x. Native remains about **238x faster**
than the candidate on this workload. No universal or Linux speedup is inferred.
These are new paired measurements; the previous transport campaign's
1.732665 s median and raw evidence retain their original identity.

Direct leaf inputs are seven dimensionless x values from -0.75 through 0,
with y=0.25. The second polynomial result is dphase/dx, not frequency in Hz.
Direct model inputs use preconstructed epochs and 1400 MHz. These scopes are
not interchangeable with the complete C route, so their timings are not
subtracted to claim an exact FFI cost.

Short getPeriod wall medians are 5.093 ms before and 4.820 ms after; one-pair
process medians are 4.768 ms and 5.007 ms. Loader outliers reach 0.415/0.427 s
and are retained. These observations do not establish a startup improvement.
Short CPU counters round to zero. Lower requested allocation volume is also
not evidence of a proportional RSS decrease.

## Verification and remaining limits

* All **15 focused predictor tests pass in debug and release**. The new
  ownership matrix makes 12,584 comparisons against the unchanged borrowed
  operations and previous source multiply/add boundary, covering operand
  order, precision, finite values, signs and nonfinite classes. The two new
  tests also expose the real MPFR aliasing trap and protect first-rounding
  direction at the subnormal midpoint. The existing 975-case native transport
  regression remains passing.
* The unchanged installed native qualifier passes all four C11/C++17
  static/shared profiles, all 64 symbols, locale/pipe, file exchange, callbacks,
  storage/layout, error/recovery and its twelve rejected corruption controls.
  All **38 legacy transcript/product files** match the accepted baseline.
* The complete **30,258-line C result/state transcript** matches exactly,
  including same-pointer coefficient mutation/restoration, model miss/recovery,
  T1 count boundaries and class/sign neighbors. The observer explicitly checks
  that mutation changes the result and restoration restores it.
* A separate public Rust client compares **5,880 complete edge results** across
  five polynomial shapes, signed zeros, infinities/NaNs, minimum subnormals,
  minimum normals and maxima. All **28 finite polynomial/model results** match
  across every direct observation and timing run. Neither witness implements
  a second scientific evaluator.
* Both existing installed Rust consumers, `predictor_components` and
  `predictor_subnormal`, agree exactly. Plain, ASan/UBSan and TSan C caller
  campaigns each pass twelve exclusive workflows, including the production
  versus explicit MPFR cleanup comparison. Instrumentation covers C clients
  and shim, not all Rust/MPFR/Fortran/provider internals. Formatting and
  all-target Clippy with warnings denied pass.

The supplemental native transcript still has 1,428 one-ULP phase differences
and one finite derivative difference, all below the existing conservative AF
floor. **One negative-NaN T1 frequency sign mismatch against native remains**,
unchanged before/after. No gate or tolerance is relaxed. Inherited polyco
log10RMS failures remain open; this focused continuation does not rerun the
unrelated full timing/fitting campaign or claim to close them.

The next qualification is actual Linux x86_64 execution of the same owned
operation, wide-range, separate-operation, installed-interface and cleanup
checks, plus its own allocation/timing observations. The standalone
[replay recipe](../validation/predictor_performance/EVALUATOR.md) uses the
existing portable runners and prepared cache. No scientific-policy choice or
new implementation mode is required by this delivery.

## Evidence and reproducibility

New evidence root:
`/private/tmp/tempo2-ao-predictor-20261102/validation/predictor_performance/build/evaluator-20261102`.
Its `delivery.json` binds **1,895 files / 172,106,780 bytes**, with **1,726**
Git-to-export/installed source bindings. SHA-256:
`0ec8a587d02e02517b195f874bbe76bb19fb86a49ad392a3ecfe9ee912af7621`.
It excludes Cargo target caches and Python bytecode; the four installed consumer
executables are copied separately. External references bind the accepted
baseline binaries/receipts, input, prior profile and pinned library sources.
The executed `freeze.py` and its initial installer-selection check failure are
retained. That preparation check was corrected to the existing installer's
declared source selection; no installed content was changed.

| Evidence | SHA-256 |
| --- | --- |
| C complete comparison, `verify/receipt.json` | `4bc8f4ed985ecabb4ecfbc01ea0ec731229af86857f9ddf9010cf08d9db8879b` |
| Direct comparison/counts, `direct-verify/receipt.json` | `e9f39908f5ae591b80f4acdde56d33432a26cb2311476ebc091b3ef7f1c64757` |
| Full C timing, `measure/receipt.json` | `0774c56567f6aea8b3e01b8bdd42602a2226888933382e9419dab444f135c09d` |
| Direct timing, `direct-measure/receipt.json` | `d182dbfe6afbf6a1e97e70c1e62cc8c9d6f0db4b1d9f6056bfd0691fdb4961c1` |
| Installed qualifier, `qualify-after/receipt.json` | `2f7e26f60598aba55069abcfe58aa9c4ef4246f432329eb69fe0d17c8a8049c3` |
| Caller/sanitizer campaign, `concurrency/receipt.json` | `4cb36007a2350c9f9adb25ace2a73cf87832e4139e28cf4bb2c2f500ababded0` |
| Installed Rust consumers, `consumers/receipt.json` | `ab9c69cdb687854af3eaf26626878408257c76ee8654b6ab8a12053dc716f3be` |
| Baseline shared library | `acb1d0461bdcb911a01b2cad0032cf82bfd390c161e24f3cafdd392022d1532b` |
| Candidate shared library | `7192bc8ee79ba8296450cdab82fbb712caa11c5d54e49cda310bbc4a44f08d1b` |

The initial attribution build, expanded-test NaN-sign failure, zero-test filter
attempt and final successful debug/release logs are retained. Only completed
meaningful runs count as passed. The old transport report, recipes and delivery
manifest remain unchanged; the latter still hashes to
`e181ff4c83e2cca582bc281cb059e81f05d4ac9116b9d3f1e9c662ff682a28f6`.
Generated exports, JSON, binaries and traces stay ignored. Production, focused
tests, continuation recipes and this report are committed for central review.
