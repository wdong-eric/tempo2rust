# Predictor temporary reuse: implementation and measured result

4 November 2026, decision 0143. The combined macOS arm64 change reduces the
median complete-C interval from **0.210334 s to 0.177285 s per 100,000
phase/frequency pairs**: **1.186417-fold faster, 15.712628% less time**.
Allocation requests fall from **62 to 50 per pair**, and requested bytes from
**1,920 to 1,024**. These are observations of the combined candidate; no measured
contribution is assigned to an individual edit.

The candidate remains **40.246311 times slower than native** on this fixture.
Actual Linux qualification, inherited native discrepancies and the unanswered
release-budget proposal remain open. No push or dispatch was performed.

* Immutable before production: `a1851c55fcc0b1ac733b064e631bfbbd2e0118d5`.
* Candidate production and extended Rust witnesses:
  `e1d34d835377451dcc7822161afce96a87948005`.
* Portable qualification recipe: `a3fee87ccf40bd05d7b465aad13f2d2422bca471`,
  [TEMPORARY_REUSE.md](../validation/predictor_performance/TEMPORARY_REUSE.md).
* Native source: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, with the existing
  qualification script's recorded corrections.

## Bounded implementation

1. `cheby2d.rs` consumes `(start + end)` through the existing owned FMA helper.
   Operand order, macOS contraction / Linux separate operations, cloned/negated
   MJD, `abs_double`, inclusive endpoints, strict ties and the 1e6 sentinel stay
   intact. The temporary sum no longer needs another owner.
2. `predictor_ffi.rs::t2r_model` reads indices 0 through 6 in order into an array.
   The frequency call still reads the unused index-4 dispersion slot. Partial
   construction drops completed owners; scalar reads precede coefficient gates.
   The call-local coefficient view ends before output publication. General
   scalar vectors, callbacks, derivatives, T1 entry points and output transport
   retain their existing paths.
3. Only the two f64-RHS implementations in `predictor_number.rs` change. Owned
   left operands use Rug rounded assignment and retain the returned `Ordering`.
   Borrowed left operands construct a fresh Float from the mixed incomplete
   operation. Both reach the existing ordinary subnormal/range handling. Owned
   Add/Sub/Mul clear invalid-result NaN signs as before; Div clears input-NaN
   propagation while preserving its invalid-operation sign rule. No N/N
   implementation, public Float owner/borrow, Linux left precision/range or
   Rug128 timing arithmetic changes.

Rug 1.30.0 and its pinned MPFR already supply exact-double mixed operands; the
assessment bound those source files to their crate archives and Cargo.lock.
There is no new numeric storage, dependency, unsafe code, cache, native scientific
kernel, duplicated evaluator or tolerance. The shared model expressions remain
the canonical source path.

The extended ownership matrix uses unchanged borrowed N/N operations as its
oracle for all four mixed operations and both ownership forms. It covers signed
zeros/NaNs, infinities, maxima, normal/subnormal neighbors and live public Float
borrows. Selection adds sentinel and nonfinite cases to the existing ties,
overlaps, endpoints and gaps.

The additional first-rounding witness sets `e=2^-22`,
`a=(1-e+e^2) 2^emin` and the exact f64 operand `b=(1+e) 2^-p`. Their product is
`2^(emin-p) (1+2^-66)`, just above half the minimum subnormal. Both inputs fit
the p=53 and p=64 ordinary profiles. The first p-bit result lands on that
midpoint with `Ordering::Less`; losing the direction would round to zero.
Owned/borrowed multiplication must return the minimum subnormal with the correct
sign. This witness passed on macOS; the actual Linux branch still needs execution.

## Qualification results

The edits were checked in order: selection (one test), native representation
(two tests), then the operation/ownership and midpoint matrix (two tests).
The complete final predictor filter passes **all 18 tests in debug and release**.
Executed test binaries, stage diffs, source identities and logs are retained.

| Contract | Result |
| --- | --- |
| Direct edge / finite outputs | 5,880 / 28 exact before/after results |
| Complete C values, classes, signs, state and mutation/restoration | 30,258 exact lines, also equal to the saved accepted output |
| Retained native C/C++ qualification products | 38 legacy products exact |
| Coefficient, callback-snapshot and derivative-alias witness | 8,516 exact lines across ten C11/C++17 shared/static and ASan/UBSan profiles |
| Error ordering | All thirteen coefficient/model diagnostics exact |
| Installed consumers | Both existing consumers plus recurrence/public-borrow consumer agree; six executables retained |
| Caller-exclusive plain/ASan/TSan checks | Equal workflows and storage; explicit cache cleanup agrees with production cleanup |

The native qualifier ran freshly for the candidate; immutable before qualification
is reused. The coefficient witness executed both sides. Sanitizers instrument
C clients and the ABI shim, **not Rust/MPFR/Fortran**. No actual Linux run occurred.
The portable recipe retains Linux `1+2^-63`, `2^-16445`, `2^16382`, wide maxima
and the separately rounded source operations; a macOS cfg switch is not evidence.

Optimized installed-library inspection finds seven scalar reads and no `values`
call in `t2r_model`. Selection calls the existing owned helper with one `fmadd`
site. Both coefficient-storage specializations retain ten static FMA sites and
five separate additions, with direct native coefficient loads. These are static
instruction counts, not dynamic per-call counts.

The supplemental native transcript retains 1,430 differences: 1,428 phase and
one derivative difference lie below the unchanged AF absolute phase floor;
one T1 frequency negative-NaN-sign discrepancy remains. The separate polyco
log10RMS failure is also open. Neither failure is reclassified or waived.

## Allocation observations

Observers execute separately from timing at 1 and 1,001 iterations. Counts below
are the differences divided by 1,000; constructor and owned-multiply controls
each perform one operation, while leaf/model/C workloads perform one pair.

| Workload | Requests before / after | Requested bytes before / after |
| --- | ---: | ---: |
| N-from-f64 control | 1 / 1 | 16.003 / 16.003 |
| Clone then owned N/N multiply control | 1 / 1 | 16.003 / 16.003 |
| Direct owned leaf pair | 2 / 2 | 32.003 / 32.003 |
| Direct model pair | 22 / 16 | 352.003 / 256.003 |
| Complete C pair | 62 / 50 | 1,920 / 1,024 |

The direct observer retains a three-byte process-level difference between the
1- and 1,001-iteration runs, hence the displayed 0.003-byte contribution. The
complete C observation is 60 mallocs + two reallocs before, versus 50 mallocs
and no reallocs after; both have zero per-pair calloc calls. The measured total
agrees with the assessment's twelve-request hypothesis without proving a timing
contribution for each edit. Requested allocation bytes are not peak RSS.

## Cleared timing campaign and every sample

Central and AL explicitly held compilation, tests, hashing/copying and workloads.
The recorded window is **21:33:18.949–21:33:25.637 Melbourne time** on 4 November
2026 (**10:33:18.949–10:33:25.637 UTC**). No build or allocation observer ran in
the window. Boundary process snapshots show no competing compiler/benchmark;
both complete-output controls and every input identity agree.

The unchanged runners interleave seven samples, retain startup outliers and
record **28 direct + 63 complete-C/native/startup processes**. All 91 raw records
were reparsed, including their CPU and RSS fields. The fixture is the unchanged
one-segment, 8-by-4 predictor with SHA-256
`a8b4242d2600cc72198a7f40240b0e946d17d173804e6d10d7f5f31c9f8811ee`.

| 100,000-pair workload | Before median (s) | After median (s) | Before / after | Peak RSS before / after (MiB) |
| --- | ---: | ---: | ---: | ---: |
| Direct leaf | 0.039374667 | 0.039575375 | 0.994928 | 1.859375 / 1.859375 |
| Direct model | 0.103471833 | 0.093283917 | 1.109214 | 1.859375 / 1.859375 |
| Complete C | 0.210334 | 0.177285 | 1.186417 | 2.031250 / 2.015625 |

Direct leaf differs by about 0.51% and establishes no useful improvement. Direct
model time falls 9.846077%. The complete-C native median is 0.004405 s, with
1.4375 MiB peak RSS. Complete-C process-wall medians are 0.214687292 /
0.181266542 s; CPU medians are 0.21 / 0.17 s at the tool's coarse resolution.
The RSS values are process peaks, not per-pair storage savings.

Every loaded evaluation interval in seconds, in sample order:

| Workload / engine | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Leaf before | .039396917 | .039323250 | .039863834 | .039457459 | .039374667 | .039311417 | .039246292 |
| Leaf after | .039279916 | .039408625 | .039575375 | .039720833 | .039817333 | .039940708 | .039414125 |
| Model before | .103101209 | .103207667 | .103346125 | .103520500 | .103645708 | .103471833 | .103999667 |
| Model after | .093090084 | .093351459 | .093271625 | .094273541 | .093283791 | .093800959 | .093283917 |
| Complete C before | .210135 | .210508 | .211386 | .210334 | .210329 | .209255 | .211035 |
| Complete C after | .175769 | .177285 | .175701 | .178875 | .179415 | .178003 | .176699 |
| Complete C native | .004174 | .004405 | .004460 | .004382 | .004447 | .004446 | .004139 |

The other 42 records are startup/single-pair diagnostics. Their process-wall
intervals in seconds are shown here, including the roughly 0.4-second outliers;
none was dropped. All internal intervals, wall intervals, CPU and RSS samples
remain in `summary.json` and the raw measurement receipts.

| Diagnostic / engine | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Startup before | .004663000 | .004035125 | .004220000 | .004204125 | .009783292 | .004309541 | .004696792 |
| Startup after | .004020666 | .004068666 | .004791875 | .004338209 | .004432708 | .004660375 | .004411334 |
| Startup native | .011601625 | .012383959 | .012290833 | .025296500 | .016747542 | .011525750 | .011548209 |
| One pair before | .004149667 | .004171000 | .004888542 | .397691791 | .004138167 | .003895958 | .010090209 |
| One pair after | .441266583 | .004264333 | .004433042 | .005182166 | .004130500 | .003941208 | .004237417 |
| One pair native | .003784250 | .004001750 | .003488291 | .474474041 | .003602959 | .004216667 | .003677750 |

These short-process observations do not establish a startup improvement. The
performance conclusions apply to the loaded fixture and this combined macOS
candidate; they do not establish a general speedup or a release budget.

## Retained delivery

Evidence is under
`validation/predictor_performance/build/temporary-reuse-20261104/`. Its
`delivery.json` SHA-256 is
`4dce3cd292c650af9ef85364a8afac556260e7c126ad3adeb390f419461da1ee`.
It binds **2,005 files / 407,336,950 bytes**, **68 external identities** and
**1,742 Git source/installed-source bindings**. Seven previous manifests and the
predictor input remain unchanged; fourteen selected immutable baseline products
were rebound to the accepted coefficient delivery. Raw logs, test/consumer
binaries, source trees, qualification receipts, allocation observations,
all samples, controls, assembly and clearance/start/end records are retained.
Cargo caches/targets and Python bytecode are excluded. This report is bound by
its delivery commit rather than by a self-referential manifest.
