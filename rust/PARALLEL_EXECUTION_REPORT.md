# Assignment AK: observation execution, concurrent workflows and measurements

AK delivers optional real observation workers through the canonical engine,
ordered scientific/state/effect publication, and complete workflow admission
around the existing process owner. Default execution remains serial. This is
an implementation/qualification handover for central's 0082 checkpoint, not
numerical acceptance or two-platform release acceptance. The measured workloads
show no consistent end-to-end benefit from four Rust workers; memory and the
standalone C bridge remain material performance costs.

## Identity and scope

Frozen contract commit: `a8180f1`. Production:
`81de1e6bb7d6f6cf1d17c075f402c301b87c9ce9`.
Qualification, API and reproducible recipes: `3926ae14375752a6ef9526e8141a2bad864097af`.
The production commit follows released AJ `3e41497` / `31c7290` / `f4b73f8`
and central's `16c516f` dispatch. The final report is a separate documentation commit listed in the delivery
message; qualification/documentation do not change the scientific implementation.

Canonical TEMPO2 remains clean at
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`; the independent reference remains
clean at `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`. A2 archive SHA-256:
`313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5`.
Existing runners, numerical gates and historical evidence were not edited.
Central's simultaneous changes remain outside AK ownership.

Executed host: macOS 26.7 (25G229), arm64; Rust/Cargo 1.97.1, Apple Clang
21.0.0, Homebrew GCC/GFortran 16.2.0. Rust uses Rug 1.30.0 with 128-bit
significands; the actual static MPFR reports 4.2.2 and TLS enabled. Native
timing uses the existing float128/quadmath profile (113-bit significand),
retaining mixed source binary64 operations. Ordinary predictor C long double
is the host's 53-bit ABI. No precision or compiler-contraction policy changed.
Actual CALCEPH 4.0.5/INPOP17a and Apple Accelerate are exercised in the combined
profile. The baseline timing benchmarks use the disabled-provider profile.

## Implementation, source and unsafe map

| Source responsibility | Delivered location and behavior |
| --- | --- |
| calculate_bclt.C outer pulsar and observation loops | `src/propagation.rs`: serial outer pulsars/troposphere; one extracted observation routine shared by serial and scoped workers; immutable source pulsars and disjoint row scratch |
| Source observation assignment | `src/observation.rs::execution_snapshot`: clone one row and preserve all Rug scalar signs, including NaN; Rug's ordinary Clone resets NaN sign, so this local correction preserves the existing stored state |
| dm_delays.C static warning mask, TKlog/global.C histories | `src/clock_diagnostics.rs`: tagged observation intents, ordered parent replay into the existing mask/counters/log backing/version history; no per-worker source session or reset |
| sw_delay.C | `src/solar_wind.rs`: borrowed immutable runtime root, owned file/resource records and borrowed diagnostics; identical arithmetic and resource selection |
| Complete caller ownership | `src/execution.rs`: admission before existing ClockSession; scoped positive maximum workers; no scientific data in admission; no new unsafe code |
| Session and collection | `src/clkcorr.rs`: all process-session access participates; existing non-reentrant mutex, terminal error/poison semantics and cache lifetimes remain |
| Public complete operations | `bats`, `timing`, `form_bats`, `t2fit`, `general_fitting`, `core_cli`, `cheby2d_int`, `polyco`, `predictor`, `runtime`: hold admission across the relevant complete call; composed caller workflows use the outer scope |
| JPL process status | `jpleph`, `read_ephemeris`: admission before the existing init/status pair lock; direct multi-call pairing needs an outer scope |
| CALCEPH | Existing `calceph.rs` uniquely owned descriptor, no Send/Sync or sharing added; open/compute/close remains outside observation workers |
| SOFA/Fortran and Accelerate | Existing native boundaries and `-frecursive` profile; no parallel fitting/reduction; caller acceleration selection explicitly inherited by observation workers |
| Owned streams | Existing `stdio.rs` unsafe Send only: unique FILE ownership, mutable operations, no escaped pointer, exactly-once close; session serialization remains; no new unsafe sharing |
| Predictor C ABI | Unchanged `native/predictor.c` / Rust scalar bridge: caller storage, callbacks, streams and exported process globals retain AJ meanings and lifetime/extent obligations |

The new observation implementation contains no unsafe block or unsafe trait
implementation. Existing Rug Send/Sync bounds are checked by scoped-thread
compilation; the executed MPFR TLS probe additionally identifies this host's
dependency build. Existing libc parsing/stream, MPFR, native numerical and C
ABI unsafe boundaries remain those documented by AJ/AI. C sanitizer evidence
below does not instrument those entire dependency stacks.

Workers never reacquire the parent's process session. Success commits in
pulsar/observation order. An error publishes the successful prefix plus the
partially mutated failed row and its effects; speculative later reads can have
occurred, but their results/provenance/messages are discarded. All threads and
files finish before return. Panic joins and unwinds. There is no rollback,
retry, silent fallback, new parameter convention or parallel floating reduction.

See [PARALLEL_EXECUTION_API.md](PARALLEL_EXECUTION_API.md) for runnable usage,
complete-call composition, nested admission, callback restrictions, direct
JPL pairing, stable external resources and C caller exclusivity. In particular,
independent RuntimeContexts do not imply independent source histories, and
separate C models do not isolate OutOfRange. C callers serialize calls and
direct global access through the lifetime of borrowed results/callbacks.

## Concrete diagnostic correction and source departures

The unchanged predecessor full debug suite reproduced the AH fit recovery
failure: 90 passed, one failed, 36 ignored. AK's extraction found only the same
three warning-vector paths; 6,718 other scalars matched in this newer baseline.
The historical AH observation remains three paths / 6,716 scalars and retains
its original receipt. Both are ownership evidence, not a numerical discrepancy.
`predecessor-debug.log` and `predecessor-warning-review.json` are retained.

Admission now covers the complete supported workflow including controls and
destructive diagnostic collection. The original fit test retains its warning
comparisons and its existing first-prefit history qualification; AK did not
erase vectors or weaken equality. The added concurrent tests actually overlap
four callers, alternate controls, retain unique diagnostic markers and ensure
each result receives exactly its own marker with no remaining foreign events.
Fresh-process helpers establish source initialization histories; serialization
of the complete supported operation is the correction, not an isolated retry.

Two native defects are isolated in disposable copies under 0053:

* Original `libtempo2_la_calculate_bclt_CXXFLAGS` is not an Automake per-target
  flag variable. Real bootstrap/configure/make omits both OpenMP flags and the
  object has no GOMP_parallel reference. A commented convenience-library target
  receives `@TEMPO2_OPENMP_CXXFLAGS@` and links into libtempo2. Its actual compile
  has `-fopenmp -DHAVE_OPENMP`, and its object references GOMP_parallel.
* `dm_delays.C:160–195` has a shared static warning mask and once-only logging
  inside the source OpenMP loop. All eight original-racy probes produced
  differing diagnostic multisets. Eight corrected probes match serial values
  and messages exactly; only the mask checks/logging groups are serialized.
  This limited native correction does not assert that arbitrary source debug
  logging is safe. Native racy output is never a golden.

The already accepted selected-pulsar troposphere correction is retained in
those native builds. Its canonical source and accepted evidence are untouched.
Rust ordered transcript replay supplies a deterministic serial presentation
instead of reproducing scheduler interleavings. Native qualification explicitly
uses a message multiset plus observation/event-tagged SWM provenance/branches;
all Rust state/message/provenance comparisons retain order.

## Executed qualification

| Evidence under `validation/parallel_execution/build/` | Result and scope |
| --- | --- |
| `qualify-release-v2` | 11 cases × native/Rust counts 1/2/4 plus repeated Rust histories; 99 invocations; 484,314 unchanged AC quantity comparisons, zero failures |
| Same campaign | Exact complete Rust JSON across counts, including diagnostics/resources/parameters; exact native scientific TSV and tagged SWM events across counts |
| `rust-worker-calibration` and five execution tests | Actual thread identities/one visit per row; 259/17 unequal populations, uneven partitions, delay-disabled/deleted rows, veryFast 0/1, DMMODEL log/version/mask history, failed-prefix recovery, SWM reads/missing-file recovery, scalar signs/unwritten fields and control unwind |
| `native-v3`, `native-v4`, `native-warning-v1` | Actual calculate_bclt workers and observation/event identity; source race retained separately; full timing rather than only bypass rows |
| `consumers-v2` | 26 separately compiled debug/release invocations: core timing/fitting/output/recovery, RNG/toolkit, predictor workflows, repeated correlated global fits at 1/2/4, and concurrent fit/predictor callers |
| `providers-v1/paired` | Unmodified AI oracle/comparator: 59 combined-provider cases, 41,961 comparisons, zero failures and no changed artifacts |
| `providers-v2` | Exact full TSV across 1/2/4 at acceleration modes 0/1/2, real CALCEPH timing/correlated fitting, four complete concurrent provider fit/predictor callers and failure/recovery |
| `c-concurrency-v2` | Plain, ASan/UBSan and TSan each pass four callers × three complete caller-exclusive workflows; original C assertions, callback/stream/storage/global/locale histories retained |
| `timed-control-v1` | 12 complete native product-set comparisons show that removing observers preserves small/unequal/large BAT, correlated global fit, Cheby and polyco outputs at 1/4 workers |
| `installed-v1` | Fresh private release install; all installed source/native/header/data files checked against current authored files; separate Cargo package using only the installed dependency passes four concurrent repeated fit/predictor callers; separately linked shared C consumer passes |
| `library-debug-final.log`, `library-release-v1.log` | Each 96 passed, zero failed, 36 intentionally ignored; original fit recovery test passes in the full suite |
| `clippy-final.log`, formatting | All-target Clippy with `-D warnings` and `cargo fmt --check` pass |

The provider continuation corrects only an omitted TEMPO2 environment in the
added runner command after the 59 paired cases had passed. The C first attempt
hit AJ's test-local static callback-count assumption when invoking its standalone
main repeatedly; the generated AK copy initializes that counter per invocation
and retains every assertion. The original files and failed attempts remain.
The first consumer fit comparison differed only in four explicit working-directory
provenance values; final comparison normalizes only that generated caller path.
No scientific value or diagnostic vector was removed.

The C instrumentation covers clients and the C shim; Rust/MPFR, providers and
Fortran are not sanitizer-instrumented. Its caller mutex intentionally enforces
the documented exclusivity contract; it does not claim unsupported independent
C models can run concurrently. Rust tests exercise process admission and real
internal observation workers. Provider-internal thread counts are not measured;
the recorded VECLIB/OpenBLAS limits are requests, not proof of provider internals.

Central independently rebuilt production 81de1e6 in a fresh default debug
directory and replayed the five execution tests plus the original fit failure/
recovery test, including actual worker-visit selectors. Its evidence is
`/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-ak-committed-review-q98ue_ox`.
This independent bounded replay is not central handover acceptance.

## Measurement contract and results

Final `performance-v2` ran from 2026-10-27 06:59:36 to 07:00:52 UTC, after
central's rebuild and AK's other compilation/qualification processes completed.
Three shuffled fresh-process samples cover each workload/engine/count: 108
processes. Every raw BAT process evaluates five times after loading. “Cold”
means a fresh process/source session, not a flushed OS cache. All samples,
including a 3.686 s Rust four-worker correlated-fit outlier, remain in ranges.
This was an ordinary desktop host, not a reserved performance machine.

Native timings use `native-timed-v1`, whose observation objects contain no
observer calls or per-row observer allocations. The earlier `performance-v1`
includes native instrumentation and is provisional; it supports no final timing
claim. Actual worker counts were measured in the separate calibration; timing
runs record requested counts. Native and Rust serial do not execute an OpenMP/
Rust worker region. Counts above one are proven by the calibration, not inferred
solely from a build flag or environment variable.

Direct raw comparison uses the matched `formBatsAll` / `global::form_bats_all`
evaluation interval, excluding output projection/publication. Native writes a
complete TSV afterward; Rust writes null markers. Their total process wall/CPU/
RSS therefore describe separate output workloads and are not a matched
end-to-end comparison. The receipt retains them with that explicit label.
The CLI cases request identical PAR/TIM, NITS/covariance, predictor requests,
capacity settings and generated files for matched whole-process measurements.
Within each engine, generated file membership/bytes are exact across all counts
and samples. Existing native/Rust numerical gates are not replaced by these
same-engine checks.

Small uses 16 supplied terrestrial rows; unequal uses 259/17 and large 4099/2051
rows by repeating validated epochs/frequencies and changing only row names.
No extrapolated ephemeris or new physical model was introduced. No general
fit/reduction parallelism, persistent worker pool or optimization threshold is
implied by these results. Scoped-thread setup, row copies, ordered publication
and the sequential timing stages can dominate the selected loop.

The following tables are generated directly from the final receipt. Times are
seconds. Evaluation tables show median [min, max] over twelve repeated loaded
evaluations (four per process); CLI tables show three whole-process samples.
CPU is user+system and RSS is the median macOS maximum resident set in MiB.

| Loaded BAT workload | Engine | 1 worker | 2 workers | 4 workers |
| --- | --- | --- | --- | --- |
| small | native | 0.000757 [0.000719, 0.000774] | 0.000608 [0.000548, 0.000691] | 0.000660 [0.000555, 0.000727] |
| small | rust | 0.049640 [0.048559, 0.050427] | 0.049616 [0.049249, 0.049794] | 0.049388 [0.046981, 0.051475] |
| unequal | native | 0.009110 [0.008811, 0.011722] | 0.010874 [0.008473, 0.010977] | 0.010329 [0.008298, 0.010949] |
| unequal | rust | 0.113299 [0.112683, 0.115166] | 0.112708 [0.112311, 0.114858] | 0.115085 [0.108500, 0.117550] |
| large | native | 0.229822 [0.206540, 0.259183] | 0.204942 [0.196074, 0.258536] | 0.238849 [0.194326, 0.258798] |
| large | rust | 0.664666 [0.654529, 0.681744] | 0.682004 [0.667174, 0.755437] | 0.668493 [0.655108, 0.730986] |

Fresh-process load / first evaluation medians at one worker: native small 0.008843 / 0.023123, unequal 0.017517 / 0.031727, large 0.024799 / 0.254069; Rust small 0.040229 / 0.223770, unequal 0.080866 / 0.289737, large 0.113171 / 0.842194. Every count and sample remains in the receipt.

| Matched CLI workload | Engine | Requested workers | Wall median [min, max] | CPU median | Peak RSS median MiB |
| --- | --- | ---: | --- | ---: | ---: |
| correlated-global | native | 1 | 0.040261 [0.040204, 0.073498] | 0.020 | 161.4 |
| correlated-global | native | 2 | 0.039605 [0.036726, 0.041142] | 0.020 | 161.7 |
| correlated-global | native | 4 | 0.040126 [0.039484, 0.040135] | 0.020 | 161.7 |
| correlated-global | rust | 1 | 0.462281 [0.461457, 0.514997] | 0.440 | 1578.1 |
| correlated-global | rust | 2 | 0.462286 [0.462160, 0.512196] | 0.430 | 1578.4 |
| correlated-global | rust | 4 | 0.517326 [0.509564, 3.685708] | 0.460 | 1578.7 |
| cheby | native | 1 | 0.132337 [0.128277, 0.132520] | 0.060 | 161.9 |
| cheby | native | 2 | 0.129134 [0.128493, 0.132128] | 0.070 | 162.1 |
| cheby | native | 4 | 0.129371 [0.127744, 0.132024] | 0.070 | 162.2 |
| cheby | rust | 1 | 0.462000 [0.458603, 0.462038] | 0.400 | 833.4 |
| cheby | rust | 2 | 0.460534 [0.448362, 0.460824] | 0.410 | 847.1 |
| cheby | rust | 4 | 0.455128 [0.449140, 0.462124] | 0.410 | 856.1 |
| polyco | native | 1 | 0.128480 [0.125463, 0.130866] | 0.080 | 162.1 |
| polyco | native | 2 | 0.132075 [0.131684, 0.132136] | 0.080 | 162.3 |
| polyco | native | 4 | 0.131472 [0.128470, 0.131631] | 0.080 | 162.3 |
| polyco | rust | 1 | 0.407057 [0.397343, 0.407199] | 0.350 | 844.1 |
| polyco | rust | 2 | 0.395010 [0.393769, 0.396540] | 0.360 | 858.1 |
| polyco | rust | 4 | 0.407128 [0.404638, 0.407511] | 0.370 | 865.8 |

For the separately labeled raw Rust large processes, peak RSS medians were 1710.7 / 1953.9 / 1837.4 MiB at 1/2/4 workers; process CPU medians were 3.61 / 3.95 / 4.12 s. These include five evaluations and the raw marker output. The nonmonotonic memory and wall variability is retained; it is not evidence of a memory scaling law.

Standalone C Cheby evaluation (`evaluation-v1`): three samples of 10,000 phase plus 10,000 frequency calls on the identical native-generated predictor file. Native loop median 0.000346 s [0.000343, 0.003922]; Rust C route 0.469391 s [0.463081, 0.469557]. Aggregate checksums match exactly. This measures the full C route including marshalling, copying and canonical evaluation, not an isolated conversion primitive. Native/Rust process CPU medians were 0.00 / 0.46 s at the coarse resource-counter resolution, RSS 8.5625 / 2.28125 MiB. This is serial caller-exclusive evaluation, with no observation-worker claim.


## Remaining acceptance ledger

This handover closes no inherited numerical acceptance item. M0/M1a/M1b retain
their scoped acceptances; M2 remains unaccepted. The complete 0096 ledger is:

* P: 154 records including 32 SIMUL; M2: four raw SVD-order failures;
  0049: four discrete boundaries and three unconstrained interpolation cases.
* AE red: six raw modes; chromatic-log: sixteen errors and one multiplier;
  ELL1H: nineteen whitened-data differences; single-pulsar quadrupole/GW-single/
  memory/cosmic-string: 770/750/448/468 failures.
* DDH: 401/433 current/historical fidelity failures and two shared-solve raw-order
  differences; two first-pass SWGP unwritten-state boundaries; AF: seven raw
  polyco log10RMS failures. Historical versioned 1 ns ideal passes remain separate
  from fidelity acceptance.
* AH twelve groups: `pre-list-bary-unwritten` (unproduced UT1/missing PAR),
  `pre-list-tempo1` and `pre-list-terrestrial` (position/velocity signed zeros),
  `dm-grid` (ill-conditioned RMS/PAR digits), `red-noise` (F2 digit), `ell1h-h4`
  (offset/RMS), `waves` and `wave-dm-scale` (unfitted errors/missing PAR),
  `global` (F2 digits), `cheby-odd-defaults`, `cheby-ecliptic-tdb` and
  `polyco-prefix` (coefficient digits). Absent DDH derived inputs and all-deleted
  span boundaries remain.
* AH's three warning-vector ownership failure is reproduced and corrected within
  the tested supported complete-workflow contract; this is submitted to central
  for review, not used to imply arbitrary concurrent API safety.

0090's positional source-weighted errors/warnings and 0087's paired current/
historical clock semantics remain unchanged. AL retains actual Linux provider
implementation/execution, 64-bit-significand ordinary-long-double ABI/conversion,
compiler contraction measurements, integrated installed consumers on both
platforms, both clock versions, broader resource/scientific coverage and every
inherited numerical gate. Decision 0097's GitHub Actions preparation does not
transfer those obligations to this macOS result. Full migration/distribution
acceptance remains open.

## Evidence preservation

All generated products remain ignored. The exact original qualification
executable was reproduced with SHA-256
`666af72fe643dba9df55c3a1de9605a575c1d4bfe9773e795fb025d52da26585`
and frozen in `build/source-snapshots/parallel_execution-qualification` after
adding the raw measurement mode to the example. The relocation receipt preserves
the original path/hash relationship. Final timing copies its binaries and
executed recipe before running. Historical runner failures and source revisions
are retained, not relabeled as current successful evidence.

| Final artifact (under build/) | SHA-256 |
| --- | --- |
| `qualify-release-v2/receipt.json` | `8be1e4593c181b4e011546ca0ce4c07dcb9034a52c0975438329eefc8fd73106` |
| `makefile-v3/receipt.json` | `d8a79cf2d0cefa0fed94c48d2fb462606512552df67349eb4bc0f9f59dc899a5` |
| `native-v4/identity.json` | `7338af6b39e8f238c9649f528a348f8348db7fb6a880bb36a7db78b4bcdce343` |
| `native-timed-v1/identity.json` | `081f8946651d02b09bd78986c0c58e983c7257b8ab319685aca2f4de961ae8a6` |
| `native-warning-v1/receipt.json` | `96780448593b7497a205e7e945855cc6c9a416cfedc96293f9adfe9fcf4af75b` |
| `timed-control-v1/receipt.json` | `66f15845aaae1dcc71ae1e4bba61d440cd59bde9d808527e76614f0f34f42edc` |
| `consumers-v2/receipt.json` | `d48331eadbd793ac4b86531c7c2f6c3a1cfec203cc66237ebbfa7060f43c5357` |
| `providers-v1/paired/receipt.json` | `95d3cab6b0e4e28c6afeabfd6274585340d4ef80edd577eb57f8743aa90db4e9` |
| `providers-v2/receipt.json` | `e1354dae7838bf4bc84b7725531b8e30e7ff82dd9e07e4288ca6cd098c10f131` |
| `c-concurrency-v2/receipt.json` | `9f284a420af7c22df9ef23232f3108c347bbd304be0f7837bf261bf52a625d4e` |
| `performance-v2/receipt.json` | `c9169bfd16d6847bb92a88ca44960a63845dfca3b574dea09fc263a569c51418` |
| `evaluation-v1/receipt.json` | `5a9691b10c2ae71842ceace0e3e570647962a14cbd22ffcd3657513a8f1214c1` |

Final artifact/source audit: **2,752 unique files, zero mismatches** (`build/audit-v1/receipt.json`). It verifies the selected receipts, installed source, generated Makefile/object witnesses, all 21 production-commit source files and both clean pins. The qualification executable relocation is explicit. This is a delivery identity audit, not additional numerical validation. The fresh installed source check covers 209 code/native/header/data and build/license/manifest files; it preceded publication of this final report.

`build/installed-v1/receipt.json`: `06d549813bbe92e6ff0089c74f28d5d6e629ed868b8f40abd62d93854c90cf9a`.

`build/rust-worker-calibration/receipt.json`: `444006a469525439bfaefb7016db370010e1e4c58de9855f163efda9d83d5053`.

`build/mpfr-profile/receipt.json`: `e232c8b1853504469228e28c3a7217c111b69c973b8d548031f1f150213db292`.

`build/audit-v1/receipt.json`: `c5190ac3ac3f0ead16041e2378aadf2ba474a6887404bf7494814575ef7eeb6b`.
