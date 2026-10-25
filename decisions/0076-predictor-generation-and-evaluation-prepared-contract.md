# Predictor generation and evaluation: prepared Assignment AF

Central design decision, 25 October 2026. **Prepare AF to deliver TEMPO1 polyco and TEMPO2 Chebyshev generation, file exchange, evaluation and frequency plans. AF is not released.** Sequence it after AE's fitting implementation under 0065. Its timing dependency is AD; fitting is not silently inserted into predictor generation. Under 0074, release requires a reviewed predecessor implementation handover, a pinned baseline and explicit ownership closure/reassignment, rather than automatic acceptance of every inherited numerical gate. AD currently owns Rust under 0075; AE remains prepared under 0071.

AF will own `rust/` and new `validation/predictors/` as the sole Rust writer. Central retains design and milestone acceptance. Reuse the canonical state, parser, preprocessing, runtime, BAT and residual routines. Preserve existing accepted evidence under 0050, keep generated outputs ignored under 0039, and apply 0053 to small demonstrated source bugs with adjacent pinned source/cause/fix comments. The original 0008 document remains unchanged.

## Public capability

Provide safe Rust APIs and these CLI routes, freezing concrete signatures and authored runnable examples against the released baseline before implementation:

```text
tempo2-rust predictor cheby --par MODEL.par --runtime T2runtime --site SITE --start-mjd START --end-mjd END --freq-start-mhz LOW --freq-end-mhz HIGH --time-coeffs NT --freq-coeffs NF --segment-seconds SECONDS --output-dir DIR -- SOURCE_PREPROCESSING_ARGS
tempo2-rust predictor polyco --par MODEL.par --runtime T2runtime --site SITE --start-mjd START --end-mjd END --span-minutes MINUTES --coefficients N --max-hour-angle HOURS --frequency-mhz FREQ --output-dir DIR -- SOURCE_PREPROCESSING_ARGS
tempo2-rust predictor evaluate --input PREDICTOR_FILE --mjd MJD --frequency-mhz FREQ
tempo2-rust predictor plan --input PREDICTOR_FILE --start-mjd START --end-mjd END --step-seconds SECONDS --frequency-mhz FREQ
```

These commands are the proposed public contract, not currently implemented examples. Generation reads and preprocesses the PAR model, then synthesizes observations as the source does. `tempo2.C:359-378` skips TIM reading in predictor mode. Do not require a fabricated TIM file to reuse `bats::load_pairs`; reuse the existing lower-level canonical reader/lifecycle instead. Preserve preprocessing controls and model/resource provenance. Generation calls the shared BAT and uncentered residual path (`removeMean=0`), with no fit cycle. Direct construction APIs accept mutable canonical state because the ostensibly const C++ source modifies parameters and observations. Expose those source mutations; do not claim construction is a pure evaluation.

The CLI generates source-format artifacts in the requested output directory. Preserve the Chebyshev `t2pred.dat` and `pred.tim` responsibilities and both polyco formats (`polyco_new.dat`, `newpolyco.dat`) plus `polyco.tim`; expose the source filename-prefix behavior in the direct polyco operation. Reuse canonical TIM writing. Keep diagnostic output separate from structured command results, retain meaningful source warnings, and publish success only after generation and output completion. Any failure-publication improvement is an explicit adapter/correction, not a claim about original file atomicity.

Evaluation and plan commands need the predictor file, not the timing runtime. Their results identify predictor kind, PSR/site metadata, declared coverage, input MJD/frequency units, selected segment/out-of-range state, phase in turns and frequency in Hz. Plans include phase0, ordered segment endpoints, segment count and average frequencies; distinguish these from instantaneous frequency. Retain original bytes/hash and model-format provenance. None/empty or source-invalid backing produces a contextual recoverable error rather than an invented prediction.

Safe Rust equivalents of all defined `T2Predictor`, Cheby2D/model/set and T1Polyco/set operations are in scope, including direct construction/test callbacks, copy, insertion, keep, metadata and I/O. Source T1 insert/keep are explicitly unimplemented; preserve that classification rather than inventing an algorithm. Source Copy delegates to Insert, so its T1 behavior also needs explicit disposition. This stage does not claim C binary compatibility: the `libtempo2pred` exported ABI/header/build/consumer obligation remains part of final core integration, separate from plugin ABI. A Rust API alone must not be reported as an installed drop-in C library.

## Source responsibilities

Use raw blobs from pinned TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; checkout Id expansion is not the source identity.

| Source | Required transfer |
| --- | --- |
| `polyco.C` | `polyco`, `atimfake`, `tzFit`, `chebpc`, `pcshft`: source artificial-TOA sampling, sidereal/day/half-hour conventions, 31-point sampling, coefficient transformations, trueDM mode, metadata, diagnostics, two file formats and retained state. |
| `cheby2d_int.C` | `chebyModelFunc`, model/set construct and test: TZR sample, time/frequency grid, apparent dispersion separation, canonical BAT/residual calls, segment assembly and source error summaries. |
| `cheby2d.c` | All defined polynomial/model/set lifecycle, construct/derivative/evaluate/test, parsing/writing, segment selection, insertion and keep routines. Map compiled test helpers to tests; do not count the disabled main as a production entry point. |
| `t1polyco.c` | Both readers and their helpers, fixed-format writer, phase/frequency evaluation, nearest selection and complete set lifecycle. Preserve legacy D exponents and the TEMPO2-marked input format. |
| `tempo2pred.c`, `tempo2pred.h`, `tempo2pred_int.h` | Generic kind dispatch, lifecycle, file/stream operations, metadata and both plan APIs. Account for header/definition disagreements without duplicating scientific bodies. |
| `tempo2.C:359-503`, relevant `getInputs.C` branches | PAR-only preparation and the two generation routes, source defaults and even-coefficient adjustment, no-fitting return, output files and generation diagnostics. Full legacy CLI integration remains a later owner. |
| `TKlongdouble.float128.h`, `TKlongdouble.C`, `Makefile.am` | Numeric dispatch/format boundaries and actual C/C++ library membership. Reuse existing numerical and formatting implementations; do not port a second backend. |

Implement source-mapped modules with a complete routine/body/branch map, not a replacement generic polynomial package. No remaining toolkit, GW/spectrum, core output, optional-backend or platform responsibility is removed by this assignment. AE may already supply dependencies; inspect its delivered inventory and reuse them.

## Numerical and scientific conventions

Keep the source's distinct numeric types. `polyco.C` timing `longdouble` remains the approved Rug128 representation under 0001. Predictor C structs and ordinary C/C++ `long double` expressions are a separate type. The accepted GCC 16.2.0 arm64 host has an 8-byte native long double with 53 significand bits and binary64 exponent range; the source timing typedef has 113 bits. A fresh compiler-macro check confirmed this during preparation. Follow the same native-type boundary policy as 0070: use the corresponding binary64 boundaries for this reference profile, and do not silently promote every predictor operation to Rug128 or globally narrow the timing engine. Other platform long-double formats require their own qualified disposition and evidence; a successful Linux build alone does not establish native numerical parity there.

Inspect compiled math resolution, including the float128 header's `cosl/sinl/floorl/fabsl/powl` remapping to quadmath, ordinary literals and intermediate conversions. A source variable declared native long double can consume a quadmath result before narrowing. Preserve operation grouping, source integer division, binary64 literals and any demonstrated contraction boundary. Existing helpers and explicit conversions should express this; no runtime precision switch or alternate timing engine is authorized.

Preserve the actual algorithms and conventions:

- The upstream generation CLI rounds odd Chebyshev coefficient counts upward and uses 10% segment overlap; direct library constructors consume their supplied values. Keep the distinction. Main's missing-TZR defaults (PEPOCH, 1400 MHz, source NULL-site rule) differ from the direct set constructor's start-MJD/start-frequency/empty-site defaults. Do not merge these caller contracts.
- Chebyshev sampling uses the source cosine grid, coefficient layout, accumulation order, Clenshaw recurrence and x-derivative recurrence. Keep the source dispersion-constant sample indices and subtraction/restoration. Use `tzrsite` for the reference observation, independent of the generated site. Preserve Hz/MHz and MJD/seconds conversions.
- Chebyshev selection considers inclusive time coverage, chooses nearest center and retains the first tie. No matching segment sets the source out-of-range state and leads to its defined -1 result; expose this status separately. T1 selection chooses nearest midpoint and extrapolates outside nominal span. Do not add a shared clamp or change both to one selection rule.
- T1 phase applies the source frequency-dependent dispersion expression. Its instantaneous-frequency routine does not use its observing-frequency argument. Preserve that behavior unless a demonstrated correction is separately justified under 0053; do not substitute a newly derived physical model.
- Plan construction uses source ceiling, shortened final interval and consecutive unwrapped phase differences divided by elapsed seconds. Do not replace it with instantaneous-frequency sampling or the future minimization mentioned in its comment.
- Polyco sampling preserves its sidereal constant, hour-angle and half-hour rounding, fractional-MJD truncation, source limits and custom coefficient conversion. `trueDM` preserves its actual propagation-delay calculation. Keep original observation/parameter mutations, tracking/EFAC settings, reference phase and diagnostic contents.

No numerical disposition of P, M2, 0049 or DDH is made here. Retain their actual identities and open checks. Predicate/integer differences are not covered by a continuous-error budget.

## Focused source defects to handle within implementation

These are source-identified issues/questions, not fresh numerical acceptance. Establish valid caller paths and focused original/corrected evidence under 0053. Correct the cause in the assigned implementation; preserve defined neighbors and source correspondence.

| Source | Required investigation |
| --- | --- |
| `tempo2pred.c:339-378` / `cheby2d.c:651-684` | GetPlan_Ext declares an uninitialized predictor and calls a reader that destroys prior model storage. Use valid initialization/ownership; do not execute arbitrary source memory to manufacture expected output. Include direct plan and read-fail/read-success histories. |
| `cheby2d.c:423-449` / `TKlongdouble.C::ld_vsprintf` | ChebyModel_Write passes ordinary native-long-double fields to the binary128 variadic formatter. Verify the built ABI: `%Lg` in the selected helper consumes the timing typedef, not native long double. Separate internal coefficient evidence from potentially invalid original serialized output; use a narrowly corrected native writer when needed and record its patch/identity. |
| `cheby2d.c` parser/derivative and `t1polyco.c` readers | Missing dimensions, nx below the derivative recurrence's required extent, excessive counts, incomplete rows and trailing malformed input need valid-input neighbors and explicit error/partial-read semantics. In particular, the source dimension predicate uses && and the T1 set reader accepts a completed prefix. Do not silently turn every parser behavior into a blanket strict policy. |
| `polyco.C::atimfake/tzFit` | Check the actual 800-element and coefficient-array bounds, including the off-by-one guard and one-based phase storage; distinguish documented storage safety corrections from changed sampling. Check direct-call initialization and file/date-buffer handling against real callers. |
| Predictor copy/insert/keep/read histories | Deep ownership, self-alias cases and changed/reused objects need a real sequence witness. Preserve defined append/order/selection semantics; do not introduce sorting or infer a bug merely from an unusual implementation. |

Formatting and stream-interface corrections must preserve represented scientific values, field meanings and interoperable source syntax. Keep original versus corrected oracles explicit. No corrupted/undefined native value is a scientific golden result.

## Verification and delivery

Freeze authored native inputs, observables/units and comparison rules before inspecting Rust results. Use the existing pinned reference archive/source with focused predictor callers; do not establish another independent reference-only prerequisite or modify accepted reference artifacts.

Required evidence has three distinct layers:

1. **Identical-input components:** analytic constant/linear/polynomial callbacks, source coefficient construction/derivatives, phase/frequency evaluation, segment ties/overlaps/endpoints/gaps, plan final intervals, metadata, both file grammars and owned copy/insert/keep/read histories. Preserve exact counts, kind/segment membership, statuses, signed classes and file-field order. Validate comparison rejection of changed coefficients, units, selected segments and plan counts.
2. **Public generation and exchange:** real PAR-only isolated and binary models through the canonical engine; terrestrial and barycentric sites, differing TZR/generation sites, dispersion and representative time/frame choices. Generate both predictor forms, read Rust outputs with native readers and native/corrected-native outputs with Rust, then evaluate them. Check constructed, serialized and reread stages separately so serialization loss cannot conceal an engine discrepancy. Validate source defaults and preprocessing, original/corrected cases, failed input/output followed by valid recovery and repeated runtime/context use.
3. **Approximation quality:** compare predictor evaluations with the full timing model on independent points beyond the construction grid, across time/frequency coverage and segment overlaps. Report unwrapped phase, frequency, source RMS/MAV and time-equivalent errors for native and Rust. A source approximation error is not the same quantity as a Rust/native implementation error. Preserve requested order/span rather than secretly increasing them until a test passes.

The existing source-engine 1e-12 s timing and 1e-10 turn phase gates retain their scopes. Freeze explicit coefficient, predictor phase/frequency, plan and formatting budgets with units, reference scale, structural-zero rules and native-type rounding analysis before new comparisons. Final predictor errors must be reported in turns and, where F0 is valid, their time-equivalent units. Do not apply a seconds tolerance directly to coefficients with different powers/units, or use an approximation envelope as a waiver of a failed implementation comparison. A new budget changing an existing requirement or scientific assumption returns to central; a new field without a checked gate is not accepted. Independently chosen per-case approximation targets must be recorded from source/order requirements before observing Rust output.

Run the affected checks in debug/release, relevant inherited timing/fitting regressions, formatting and Clippy. Preserve actual run/source/build identities and unrun-platform limitations. Under 0074, a localized pending numerical decision can coexist with a functional handover; it remains unaccepted and reproducible. Do not repeatedly replay unchanged failures as a prerequisite to unrelated predictor work.

Deliver coherent Conventional Commits, `rust/PREDICTOR_REPORT.md`, concrete public examples, source/unsafe/departure maps, component and full-workflow results, cross-reader evidence, approximation diagnostics and the complete unresolved ledger. Generated JSON, predictor products, native/Rust binaries and large traces remain local and ignored. Acceptance remains central; this document alone releases no implementation chat or Rust writer.

## Preparation evidence

Preparation inspected the five implementation files, public/internal headers, generation driver and build/type boundaries. It ran only a GCC type/version macro check (16.2.0; native long-double mantissa 53, size 8, MIN_EXP=-1021 and MAX_EXP=1024; float128 mantissa 113). No predictor numerical comparison, Rust implementation, corrected native build or C ABI compatibility test is claimed.

Raw pinned source identities:

| Source | SHA-256 |
| --- | --- |
| `tempo2pred.h` | `fb9b0a192e14bbb8ee9c4525807406b01b7e45846ea2b5ad1bc774e2693d0a76` |
| `tempo2pred_int.h` | `55b49ac77241bb7b4d0a8bc55b5a5ca955dbe21c061ab0434997032abfceb1ad` |
| `tempo2pred.c` | `bdce274251187b0348a9ef2232ef7872a107a57323a9c6f86c5fc65cfc122e00` |
| `t1polyco.c` | `e3be7ac166cfd25c0327b2d40c70f42adc636ca43f0d1abd56282da397b05c04` |
| `cheby2d.c` | `173947867f2e4b168e03d43186793b6b5118380daff61fcc9e0facc022d4c488` |
| `cheby2d_int.C` | `4893b478174ec5d3845be7793721eb649c558d1b76adbb2b992d3c028dbfef00` |
| `polyco.C` | `f2d0cf24fae2fedde10602ef4bc88273304e193842943981b7dc937586cdb932` |
| `tempo2.C` | `aa8df00d26c914dd0c56aaf7ebe15ddd415cc8c18c839b92b259c9b3da0a33ad` |
| `Makefile.am` | `172ec480482f0261f0a845e9aface382c1a441f4e80d53132574bf8ff912bd7a` |
| `TKlongdouble.float128.h` | `214911b81f82489996d698626cf8d30ab5bcc20099f6b5c5aa089715b85bc55e` |
| `TKlongdouble.C` | `4e8da8ed6c9fd6d60a671e330f09f975e32caba7aacd0c522e8c9a5a0d75a05c` |
| `getInputs.C` | `c59797f69cc09696869fa712d1537c5ea9fa1be08da08936ce86861aecc05d28` |
