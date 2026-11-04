# 44: Qualified release — performance, science and minimal GUI

Approved continuation, 4 November 2026; authority:
[decision 0129](../decisions/0129-qualified-release-continuation.md) and the
user's explicit request to create this chat and begin the agreed goal.

## Goal and working role

Deliver a qualified TEMPO2-Rust release for macOS arm64 and Linux x86_64 that
supports the intended core/toolkit workflows, preserves agreed scientific
contracts and predictor C compatibility, meets explicit performance budgets
against original TEMPO2, and provides a minimal interactive timing workflow.
Keep native dependencies documented and all scientific computation owned by
the Rust engine.

You are the successor coordinator for design, implementation handoffs and
milestone acceptance. Start this goal in your new chat without a token budget;
none was requested. The old central goal is paused, not complete. Do not resume
it or reactivate its completed-run monitor. Act on the approved scope instead
of asking the user to approve this goal again.

The working repository is **`/Users/Eric.Dong/Downloads/tempo2rust`**. The older
`/Users/Eric.Dong/Downloads/tempo2-rust` spelling is stale. Project ID is
`94769334-d7b2-47e2-ac66-a8c112a53202`, label `rustify-tempo2`. Main was clean at
`483ced4b15a767d157d1b3ff40f964a78a35eaa9` before this handoff documentation
commit. Inspect actual HEAD/status before editing. Canonical TEMPO2 stays at
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`; use disposable source for native
correction experiments. Public repository: https://github.com/wdong-eric/tempo2rust.

Read this handoff first, then the linked decisions/reports for the work being
released. Historical HANDOFF.md paragraphs describe earlier checkpoints, not
the current run or acceptance state. Avoid rereading the entire archive before
making progress.

## First actions, in dependency order

1. Record the approved goal; verify the checkout and existing chat ownership.
   Finish the narrow pending reviews listed below. Existing successful checks
   should be reused when their source/artifact bindings still hold.
2. Obtain/review completed Linux run 41392963741's artifacts and consolidate
   current failures by root cause, affected workflow and release consequence.
   Preserve original failing evidence. Do not equate stage counts with bug
   counts or treat preparation as platform execution.
3. Review AO's delivered CPU assessment and release the smallest justified
   substantial optimization. Set matched runtime/RSS budgets from measured
   baselines. Continue one AL scientific/compatibility stream in parallel.
4. Perform a short GUI feasibility/build check and choose one frontend. Begin
   with loading a qualified PAR/TIM workflow, residual/error-bar display, zoom
   and TOA inspection. Progress to editing/refitting only through canonical
   Rust actions and proven CLI-equivalent scientific results.
5. Integrate coherent deliveries, run narrow affected checks, then qualify a
   meaningful complete candidate on both platforms. Review before publication
   or dispatch; reuse completed evidence for documentation-only changes.

Performance and scientific qualification have priority. A GUI feasibility
slice can proceed independently; a large visual redesign or plugin emulator
must not displace those priorities. Reassess dependencies at substantial stage
checkpoints and bring genuinely new consequential choices to the user with a
recommendation.

## Completion criteria

| Milestone | Observable completion |
| --- | --- |
| Current baseline | One implementation/acceptance matrix, classified failure ledger, native-dependency inventory and matched benchmark recipes; explicit per-workload runtime/RSS budgets recorded. |
| Performance | Measured predictor improvements followed by profiling-led timing/fitting work; accepted scientific/edge behavior retained; budget results recorded on each target. No universal speed-parity promise is implied. |
| Scientific qualification | Intended release workflows pass adopted contracts on both platforms, or have explicit scoped exclusions that do not masquerade as full coverage. General fitting, DDH and predictor discrepancies receive root-cause decisions. |
| Minimal GUI | Residual/error-bar plot, zoom and TOA inspection; then selection/exclusion, refit and save using the same engine. Stable TOA identity and GUI/CLI agreement verified; plotting precision never replaces scientific state. |
| Installed release | Installed Rust core/toolkit and predictor C clients plus representative CLI/GUI workflows work on recorded macOS arm64/Linux x86_64 environments, with dependency/linkage, coverage and limitations documented. |

The prior full-core scope remains the destination. Any proposed reduction in
that scope needs a clear central/user decision; shipping a small GUI does not
complete migration. Actual numeric performance budgets are a baseline-stage
decision, not a permission to reduce precision or weaken scientific gates.

## Binding implementation and scientific policy

- Follow TEMPO2 algorithms, expressions, operation order, notation, units,
  conventions and defined behavior by default. Identify every module's source
  counterpart. Preserve familiar human-readable commands/results; JSON is an
  explicit machine interface. Rust diagnostics identify Rust source, not fake
  native filenames.
- A demonstrated native bug may be fixed: explain the original source location,
  defect, consequence and correction beside the changed implementation; retain
  focused original/corrected evidence. A Rust implementation improvement is
  allowed when its safety, ownership, clarity, memory or speed benefit is
  demonstrated while scientific meaning and required observables are preserved.
- Precision, scientific assumptions, output conventions and acceptance
  tolerances still require explicit central decisions. Language preference or
  a predicted speedup is not evidence. Prefer mature methods/libraries after
  checking semantics and backend fit; distinguish documentation, testing and
  measured performance. Minimize unsafe and explain necessary ownership,
  lifetime and extent invariants next to it.
- Timing precision is **Rug128**; the timing reference is binary128 with a
  **113-bit significand**. Ordinary predictor profiles are **53-bit/8-byte on
  macOS arm64** and **64-bit-significand/16-byte-storage on Linux x86_64**, with
  Linux's wider exponent range. Linux ordinary arithmetic cannot become f64.
- Preserve the paired-mode SVD rule in **0120** (untouched raw arrays, matched
  modes/rank/cutoffs and all physical gates), bounded discrete rule **0127**,
  versioned DDH clocks **0087**, and source weighting with warnings **0090**.
  Their exceptions are bounded; unexplained new differences remain failures.
- Commit authored code/docs/recipes conventionally in scoped stages. Never
  commit reproducible validation JSON/JSONL, binaries or large traces. Keep
  recipes, hashes and concise reports; retain raw evidence in ignored paths.
  Preserve others' work and use explicit staging rather than `git add .`.

## Native dependency boundary and graphical interface

The product no longer needs the original TEMPO2 executable or its C core
checkout to process data. It still requires TEMPO2 runtime resources and native
components. Audit `rust/build.rs`, Cargo.lock and installed linkage; document
required versus optional, build versus runtime versus reference-only roles,
versions, packaging/licensing and platform verification.

| Component | Current role / boundary |
| --- | --- |
| Rug / GMP / MPFR | Native multiprecision implementation under Rust numeric types; not a delegated TEMPO2 timing engine. |
| Retained SOFA Fortran routines | 36 routines compiled by build.rs; document retained scientific support and Fortran compiler/runtime requirements. |
| `rust/native/predictor.c` | Standalone predictor C layout/storage/I/O and ordinary exp/log support; Rust evaluates predictors. Do not describe this as only a trivial FFI declaration. |
| Optional `native/backends.c` providers | CALCEPH and BLAS/LAPACK adapters; check actual enabled provider/linkage per build. |
| libc / owned FILE support | Native stream interoperability; JPL ephemeris processing itself is now in Rust (`jpleph.rs`). |
| Runtime data | Explicit `TEMPO2` resource tree with clocks, ephemerides and other required files, versioned in qualification evidence. |

Rust owns the scientific model/state and application workflow. Controlled native
numerical/graphics libraries are allowed. Removing every C/Fortran dependency
is outside this approved goal. Keep that boundary visible in user docs.

Existing predictor C compatibility is **not** a general timing/GUI C API.
Legacy `tempo2/plugin/plk_plug.C` takes a native `pulsar*` and calls native
read/preprocess/BAT/residual/fit/output functions (entry near line 335; calls
near 551 and 636–643). Reusing the entire plugin would require a much larger
state/function compatibility layer. General legacy plugin ABI and `-gr`,
`-output` or selection-plugin execution are not currently supported.

For the first GUI, evaluate a small `egui`/`egui_plot` frontend against a bounded
PGPLOT C rendering adapter only if the latter demonstrably saves work. Official
starting points: https://github.com/emilk/egui,
https://github.com/emilk/egui_plot, and
https://sites.astro.caltech.edu/~tjp/pgplot/index.html. Recheck current APIs and
actually build on the target platforms. None is selected or proven by this
handoff. Choose one canonical frontend, keep the headless engine independent,
and never put scientific computation in a second GUI implementation.

## Current delivery versus acceptance

Core implementation coverage is broad: general PAR/TIM-to-BAT (AC), residuals
and binary timing (AD), local/global/noise/robust fitting (AE), predictors (AF),
toolkit/statistics/GW (AG), native-style CLI/output (AH), optional providers
(AI), installed Rust/C interfaces (AJ), and concurrency/performance (AK) have
been delivered. These handovers do not mean all numerical contracts pass.

M0/M1a/M1b have scoped acceptance. **0122 accepts bounded M2 only**: ZERO/local
F0–F3 with the covered barycentric/PKS MODE/NITS/rescale cases. General/global/
binary fitting, broader scientific coverage, AL and full migration remain open.
Reports include `rust/RELEASE_QUALIFICATION_REPORT.md`, `rust/README.md`,
`docs/MIGRATION.md` and the corresponding stage reports/decisions. Nov4's
read-only assessment built current main, passed 105 default library tests with
40 ignored corpus/probe tests, and ran CLI examples. Those are build/API and
demonstration checks, not a fresh complete scientific qualification.

### Pending central work that must not be restarted blindly

**0127 bounded discrete implementation — formal entry still due.**
`ef4863008e1bffd2f40080cb55a34cad4f5b8a47` implements exact comparison targets;
`422c853333105ff5add3e2635ae53e094a016942` clarifies integer zero versus the
existing Rust Float's negative-zero representation. Rust production edits are
comments only; arithmetic/precision are unchanged. Four 0049 utility witnesses
and eight 0077 histories (`simul_123,124,127,128,139,140,143,144`) are the full
adopted scope. Original failures remain, with no tolerance relaxation or late
rounding that pretends to reconstruct native input history.

Before the pause, central passed the 16 utility targets, both saved 48-history
sets, 18 live histories, complete corruption controls, three unit tests, and a
fresh replay through the final committed comparator wrapper. Exactly 32 SIMUL
records are explained; remaining saved preprocessing records stay 122 on macOS
and 128 on the earlier Linux corpus (122 continuous plus six phase_divzero
NaN-sign records). This establishes no end-to-end pulse-slip correction.
Finish the bounded acceptance decision after checking the retained identities;
do not repeat the campaign without changed code/evidence.

- Retained review: `decisions/evidence/release-qualification/build/continuation-20261104/discrete-central/receipt.json`,
  SHA-256 `66180b6727ac8c6d7867fc80b64a9e940f835eb9ab3d5021d64f56cc1833c3d7`.
- Fresh final-wrapper receipt: `validation/release/build/discrete-0127-central-final-wiring/receipt.json`,
  SHA-256 `ed19a054df1ed03a11f0ce046af799762d99b2958406fe9a8e5d2c710a251c31`.
- Final controls receipt SHA-256:
  `af1bf18202751882751717c37df92aba877e0708963012f0ee9ebbf951b262f1`.
- AL's producer receipts and original failure bindings are indexed by
  `validation/release/build/discrete-0127-delivery/receipt.json` and its report.

**Completed Linux run — artifact/scientific review pending.**
Run https://github.com/wdong-eric/tempo2rust/actions/runs/41392963741,
attempt 1, SHA `9991e804d0e230248e5fb20974b7e63384d4efeb`, job `125108130146`,
completed with failure on **2 November, 07:52:04 UTC**. It is not still running.
The Nov4 saved log has **97 stages: 86 pass, 11 fail, 0 blocked**. Failed stages:
`corpora-comparison`, `fits`, `boundaries`, `ddh`, `event-source-profile`,
`predictors`, `output`, `allocation-verify`, `allocation-predictor-after`,
`allocation-predictor-before`, `predictor-pair-native-gate`. Some share causes.
This candidate predates 0127's comparator and AO's evaluator owner reuse.

Retained log/stage extraction is in the handover packet's `status-nov4/`.
The older `decisions/evidence/release-qualification/build/linux-run-41392963741-review-20261102/`
contains preflight and polls only through 06:05:47 UTC, not a completed-run
artifact review. Obtain and bind the actual artifacts before accepting platform
results. Preserve case-distinct Linux archive members when reviewing on macOS.

**Successor Linux preparation — review before dispatch.**
`483ced4b15a767d157d1b3ff40f964a78a35eaa9` queues exact evaluator pair
`0bd8e2cf2ef54e9da6feace122acd8aa96aa3fbe` →
`8716d1dc026641f3a51c2ba88850c5599293cbdd`, reusing the installed baseline and
bound 8×4 native workload. AL reports 19 routing/retention tests, shallow-fetch
HEAD preservation and 382 hash-retained products including 91 resource reports.
The native NaN-sign failure stays explicit. This preparation was neither pushed
nor dispatched by AL and has not had central acceptance. Inspect its changed
workflow/runner and recorded controls, then combine a meaningful candidate.
Do not launch an identical old candidate or relabel a prior run.

**AO next CPU assessment — delivered, not integrated or released.**
Isolated clone `/private/tmp/tempo2-ao-predictor-20261102` is at
`50d1b06dc49c68a26babfe95fb7a21fcec5a8dec`. Read its
`validation/predictor_performance/NEXT_CPU_ASSESSMENT.md` and `profile_costs.py`.
Copies and a Git patch are retained under the handover packet's `ao-assessment/`;
the original assessment receipt SHA-256 is
`19545358a383639ebcdb17e9ee360773b09ff6c2cf3c2260dfd34dacb524a7b6`.
Handoff 43 authorized assessment only. Central read it before pausing but did
not independently bind/replay it, integrate its commit or release production.

Its recommendation is private, compile-time **macOS binary64 temporaries in
one canonical Clenshaw recurrence**, preserving the public PredictorNumber
Float owner and `as_float(&self) -> &Float` API, and keeping Linux Rug64.
Explicit FMA/separate-operation, rounding, exponent/subnormal, zero and
nonfinite-sign behavior must all be preserved. Conversion only at validated
input/use and return boundaries; no second evaluator, global Float→f64 switch,
C scientific kernel or persistent pointer cache. Hardware NaN behavior alone
is insufficient. Consider borrowing current C coefficient storage into that
same recurrence only as a subsequent measured change, preserving visibility
of same-pointer mutations. APFloat x87 support was documented, not tested or
selected. Review this proposal rather than restarting open-ended profiling.

## Accepted performance evidence and remaining gap

All following acceptance is **macOS arm64** and workload-specific. Linux
execution/review must be distinguished for each pair.

| Workload/change | Before | After | Native / interpretation |
| --- | ---: | ---: | --- |
| AN Chebyshev generation | 0.459811 s / 833.469 MiB | 0.403618 s / 70.500 MiB | 0.130855 s / 161.875 MiB; five samples, exact scientific checks |
| AN two-pulsar correlated fit | 0.516534 s / 1578.125 MiB | 0.402634 s / 52.406 MiB | 0.040283 s / 161.297 MiB; five samples, exact scientific checks |
| AO scalar transport, loaded C 100k pairs | 4.668811 s | 1.732665 s | 2.6946× faster in that matched campaign |
| AO evaluator owner reuse, loaded C 100k pairs | 1.750129 s | 1.078071 s | Native 0.004528 s, so about 238× remains; seven interleaved samples |

AN's accepted change uses equivalent zero-initialized allocation for the full
10,000×10,000 f64 covariance buffer; dimensions, values, ownership and numerical
behavior remain unchanged. It is ordinary f64, not Rug storage. Production
`68973d6`, integration `6cc0b79`, acceptance **0123**; report
`rust/ALLOCATION_PROFILE_REPORT.md`, evidence
`validation/allocation_profile/build/macos-reviewed-20261102/`.

AO transport production `0bd8e2c`, integration `a44bda2`, acceptance **0124**;
evidence `validation/predictor_performance/build/macos-reviewed-20261102/`.
Owner reuse production `8716d1d`, integration `9b7a185`, acceptance **0128**;
report `rust/PREDICTOR_EVALUATOR_PERFORMANCE_REPORT.md`, recipe
`validation/predictor_performance/EVALUATOR.md`, evidence
`validation/predictor_performance/build/evaluator-reviewed-20261102/`.
Owner-reuse allocation requests fall 639→242/pair and requested bytes
14,864→8,512; neither is an RSS measurement. Central verified source/output
bindings and fresh predictor tests. One inherited negative-NaN T1 sign failure
and polyco log10RMS failures remain open; same-engine equality does not waive
independent native failures.

AO's later profile places 54.84% of sampled stacks inside evaluation,
16.67% in C polynomial reconstruction and 16.12% in scalar reads; allocation
frames account for 37.29% with overlap. Do not add inclusive categories or turn
them into a predicted speedup. Nov4's independent short profile corroborates
allocation/MPFR cost, but is not a fresh paired native benchmark. Its packet is
retained as `predictor-nov4/`; 30,258 output lines were preserved. Reuse matched
inputs/resources/solver settings and record host/toolchain/backend identities.

## Chats, ownership and operating instructions

| Chat | Role / current state |
| --- | --- |
| `01a153dd-5f85-7e42-a456-372a19347c85` | Previous central chat; goal paused, history/approvals only after handover. |
| `01a17c51-359e-7041-b04b-efebd2b2ec6e` | AL, compatibility/qualification in main; completed latest delivery and idle. |
| `01a19a86-1e28-7962-b3ff-d81c5cd16ad8` | AO, predictor optimization in the isolated clone; assessment delivered and idle. |
| `01a1a4dd-505b-7751-af42-f000604ebbc0` | Read-only Nov4 assessment supplied by the user; no tracked changes. |

Host is `local`. Read each referenced chat with `read_thread` before relying on
its detailed content. Use compact `wait_threads` for subsequent progress.
The user explicitly authorized coordination messages to implementation chats.
Reuse AL/AO with clear file/responsibility ownership; tell implementers they
are not alone and must preserve others' edits. Use isolated checkouts for
overlapping production work and coordinate sustained measurement windows.

0101 authorizes publication of **centrally reviewed AL qualification candidates
and in-scope fixes/reruns**. It does not authorize an unreviewed push, a public
release, or a scientific-policy change. Check current remote state before
publication. Keep the old `tempo2-linux-qualification-monitor` paused. Monitor
new authorized long runs in the active coordination chat roughly every fifteen
minutes while doing independent work; do not create duplicate monitors/runs.

Useful existing local build environment (verify tools, do not rename HOME):

```sh
CARGO_HOME=/Users/Eric.Dong/Downloads/tempo2rust/rust/.cargo-home \
GMP_MPFR_SYS_CACHE=/Users/Eric.Dong/Downloads/tempo2rust/rust/target/gmp-cache \
FC=gfortran-16 CC=clang \
cargo build --manifest-path rust/Cargo.toml --locked --offline --release --bin tempo2-rust
```

The populated cache made this work in the Nov4 assessment; a fresh environment
needs dependency setup. `-f PAR TIM` gives the familiar human report. The
general `timing` interface computes without fitting unless `--fit` is supplied;
older `residuals`/`fit` commands have narrower contracts. `-pre` still fits.
Use explicit resource roots and existing installers/qualification recipes.

The handover's ignored preservation packet is
`decisions/evidence/release-qualification/build/continuation-20261104/`.
Its manifest records original paths and unchanged bytes, including central
0127 review, final wrapper evidence, AO assessment, Nov4 status/profile outputs
and this preservation recipe. SHA-256: `02d0124aaec663b5aaa9f88e0372e753d3b5ec30727ba28ee1100007bedad463`.
This preserves evidence; it does not newly accept assessment claims. No raw
JSON, logs or traces belong in the documentation commit.

Key decisions to consult as needed: **0099** implementation policy; **0087**
versioned DDH; **0090** weighting; **0101** publication/qualification authority;
**0106–0108** familiar output/truthful attribution; **0120** SVD; **0122** bounded
M2; **0123/0124/0128** accepted macOS optimizations; **0127** adopted discrete
contract; **0129** this approved goal. Retain every material failure until it
is fixed, qualified under an explicit decision, or excluded transparently from
a deliberately scoped release claim.
