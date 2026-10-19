# Assignment A — reference baseline report

Prepared 19 October 2026, Australia/Melbourne. Owner: `validation/reference/` only. This report proposes a reference configuration and fixture contract; it does not accept M0 or change shared design decisions.

## Delivered result

A runnable TEMPO2 reference, process-isolated exporter, six machine-readable fixture exports, resource/input provenance, Decimal comparator, and focused verification are delivered. The three barycentric exports are usable candidates for M1a. The terrestrial exports are reproducible in the tested fresh-process allocation pattern, but remain **provisional** because the pinned source reads an uninitialized solar-wind interpolation count. The comparator refuses to use them as accepted goldens.

The canonical `../../tempo2` repository remains clean at `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, TEMPO2 2026.09.1. Its existing installed executable was inspected for inventory only and was never used to produce reference values. No runtime files were downloaded or updated.

All paths below are relative to `/Users/Eric.Dong/Downloads/tempo2-rust/validation/reference/`:

| Deliverable | Files |
| --- | --- |
| Build and provenance | `build.py`, `build_manifest.json`, `build_commands.json`, `instrumentation.patch` |
| Export and file-open tracing | `export_reference.cc`, `resource_trace.h`, `resource_trace.cc`, `run_reference.py` |
| Precision and defect reproducers | `precision_probe.cc`, `initialization_probe.cc`, `diagnose_reference.cc` |
| Fixture contract and comparisons | `manifest.json`, `schema.json`, `compare.py`, `check_reference.py` |
| Preserved inputs | `fixtures/original/test1.par`, `test1.tim`, `test2.tim`, `example1.par`, `example1.tim` |
| Synthetic inputs | `fixtures/long_span.par`, `fixtures/long_span.tim` |
| Results | `outputs/{barycentric,barycentric_4ns,long_span,terrestrial,terrestrial_4ns,example1}.json` and corresponding `.log` files |
| Verification evidence | `checks/results.json`, `checks/precision.json`, `checks/upstream.json`, `checks/*.log`, `checks/missing-runtime.json` |
| Instructions | `README.md`, this `REPORT.md`, `.gitignore` |

Generated executable, static-library, object, source-copy, and runtime artifacts live under `build/`. They are intentionally excluded from the assignment Git repository and rebuilt from the pinned source. No parent Git repository or canonical source repository was reinitialized. The local assignment commit contains the scripts, fixture exports, manifests, and reports; workspace-wide Git organization remains a central decision.

## Reproduction and actual build

```sh
cd /Users/Eric.Dong/Downloads/tempo2-rust/validation/reference
python3 build.py
python3 run_reference.py --upstream
python3 check_reference.py
```

The last command exits successfully for the declared implementation and numerical checks. Upstream failures are recorded separately by `--upstream`; they are not silently counted as passes. To regenerate into a separate directory:

```sh
python3 run_reference.py --output build/regenerated
python3 compare.py outputs/barycentric.json build/regenerated/barycentric.json
python3 compare.py outputs/long_span.json build/regenerated/long_span.json
```

Actual host reports `macOS-26.7-arm64-arm-64bit` and `Darwin BatBook-Pro.local 25.6.0 Darwin Kernel Version 25.6.0: Tue Aug 18 17:49:53 PDT 2026; root:xnu-12377.161.15.700.19~2/RELEASE_ARM64_T6030 arm64`; SDK `27.0`. Python `3.11.2`; Homebrew GCC/G++/GFortran **16.2.0**; bundled Google Test **1.7.0**. The build uses C/C++ `-O2`, C++ `-std=gnu++11`, `-DHAVE_CONFIG_H`, and Fortran `-O2 -fno-second-underscore`. It does not enable fast-math; ordinary compiler contraction behavior remains that of this GCC target. Exact per-file commands are saved in the build manifests.

Autoconf/automake were absent. The direct build reads the upstream `libtempo2_la_SOURCES`, `FITFUNCS`, and SOFA source lists, invokes the upstream enum/help generators in a disposable `git archive` copy, and writes a minimal `config.h`. It selects the unchanged `TKlongdouble.float128.h`. OpenMP, BLAS, LAPACK, CALCEPH, QD and plugins are disabled. Consequently the selected tests actually execute using the same GCC float128 ABI; the upstream macOS/OpenMP test-disable branch and SDK clang override are not used. This is a documented direct build, not a claim that upstream `configure && make check` passed.

Retained numerical code: the full upstream timing core, bundled Fortran SOFA, upstream JPL ephemeris reader and interpolation routines. Dynamic dependencies include GCC libquadmath, libgfortran, libstdc++, and macOS libSystem. Paths, dylib versions, dependency hashes, static-library hash, and executable hashes are in `build_manifest.json`. Only Darwin arm64 was run.

The probe measures **113 significand bits**, 16-byte `__float128`, and 33 decimal precision digits. Native `long double` is only **53 bits** here. At MJD 50000 the direct 4 ns epoch arithmetic probe has absolute error `1.2084308783029334269794713885472227e-25 s`. This supports the claimed reference arithmetic resolution; it does not upgrade binary64 delay, clock, and ephemeris subroutines to 113 bits.

## Fixture and phase conventions

| Fixture | Pipeline and purpose | Availability |
| --- | --- | --- |
| `barycentric` | Original test1 PAR/TIM; read inputs → `formBatsAll` → `formResiduals(...,0)` | Candidate reference |
| `barycentric_4ns` | Upstream `subtractBATs` sequence: BAT=BBAT=SAT, subtract residuals, then add decimal 4 ns to all but the first TOA | Candidate reference |
| `long_span` | Four @ TOAs across about 40 years; F0/F1/F2 spin polynomial, fine decimal epochs, negative and positive epoch offsets | Candidate reference |
| `terrestrial` | Original test1 PAR/test2 TIM; original test pipeline without preprocessing | Provisional |
| `terrestrial_4ns` | Original terrestrial path, then +4 ns SAT; ±1 ms runs estimate local BAT response | Provisional diagnostic |
| `example1` | Bundled example1 PAR/TIM with `preProcessSimple`, then fixed-parameter evaluation | Smoke only |

Original input files are retained byte-for-byte. `long_span.par` replaces F0 with `123.456789123456789123456789 Hz`, adds F1 `-1.23456789123456789e-15 Hz/s` and F2 `3.456789123456789e-27 Hz/s²`, and appends explicit `UNITS TCB`. Its TIM file introduces four distinct names, epochs on either side of PEPOCH, and a late pair separated by 4 ns. These changes exercise decimal parsing, long-baseline cancellation and pulse counts. No fitted parameters are changed after parsing.

The minimal files resolve to **TCB**, **TT(TAI)**, **DE405**, IF99 time ephemeris, IAU2000B terrestrial-to-celestial conversion, frequency dilation, planetary Shapiro, tropospheric correction, IPM=1, NE_SW=4, SWM=0, isolated pulsar, and REFPHS enum 0. All effective exported settings and parsed parameter flags appear in each JSON. Barycentric @ times bypass site clock and delay corrections. The source's existing solar-wind and atmospheric defaults are recorded, not replaced.

`formResiduals(...,0)` references the fractional phase of the **first eligible TOA in input order**. It does **not** remove the residual mean. TZRMJD/TZRFREQ/TZRSITE in test1 do not activate REFPHS_TZR. No sorting, selection cuts, pulse tracking directive, fitting, or error rescaling is introduced. Repeated filename `test` is not a unique identifier: fixture plus original line/index is the identity. Original rows, decimal tokens, frequency, uncertainty, site, ordering, and input hashes are retained.

`prefitResidual` is deliberately omitted: `t2Fit_getFitData` copies `residual` into it at `t2fit.C:855`, when fitting begins. It is not an initialized no-fit output after just `formResiduals`. The exported `residual_s` is captured directly after the no-fit calculation. `batCorr`, site corrections, and related fields are omitted for @ observations where that lifecycle does not initialize them.

## Runtime provenance and coverage

Each run explicitly sets `TEMPO2=build/source/T2runtime` and starts a new process in a temporary directory. Source instrumentation only wraps `fopen` and labels stages, recording successful/failed opens, errno, caller and path. It delegates the original call and preserves errno. `instrumentation.patch` contains every such source change. No timing formula or initializer is changed in the reference library.

Every successfully opened input/runtime file is hashed, including clock files whose headers are indexed but whose tables are not used for a correction. The caller distinguishes `clkcorr.C` indexing from `tabulatedfunction.C` loading; applied clock destinations and values are recorded per TOA. The manifests include operational first/last table samples and out-of-order counts. These bounds describe the upstream files; they are not independently validated historical coverage. Some old clock files contain near-zero initial samples, which are preserved.

DE405 header coverage is MJD **33264–69808**; IF99 time ephemeris **38736–69808**. The loader's usable EOP samples cover **39491–61163**. All declared epochs requiring these resources lie within those ranges. The long-span @ fixture does not apply EOP/site corrections, although upstream `formBatsAll` still opens planetary/time ephemerides.

For test2, the early clock path is PKS→TID→NIST→UTC→TAI→TT(TAI); the 54000 epoch uses PKS→GPS→GPST→UTC→TAI→TT(TAI); the 56000 epochs use PKS→GPS→GPS_USNO→UTC→TAI→TT(TAI). Exact files and hashes are in each output. The bundled PKS atmospheric tables cover only **53000–53001**. All seven minimal terrestrial epochs lie outside that range, so upstream assumes **101.325 kPa surface pressure and zero zenith wet delay**. The TROP1/TROP2 warnings are retained. These artificial test observations are regression inputs, not independently validated observations.

## Checks and numerical results

| Check | Measured maximum absolute error | Check tolerance |
| --- | --- | --- |
| Basic barycentric residuals 0 and 0.4 s | `3.2000000189240046988E-17 s` | `1e-12 s` |
| Aligned subtractBATs residual | `1.52672984152853740772164299610569316E-25 s` | `1e-20 s` |
| 4 ns response | `5.541180890764173978E-26 s` | `1e-20 s` |
| Long-span phase against independent Decimal polynomial | `4.97725249366811095854039951333242215239808544908793000664E-23 turns` | `1e-18 turns` |
| Long-span residual against polynomial | `3.7620663547002679664849392631064956230350702865208544670505006735471E-25 s` | `1e-20 s` |
| Terrestrial 4 ns response versus local transformation slope | `1.416875336047296112492800e-13 s` | `1e-12 s`, provisional |

The long-span oracle evaluates the original decimal PAR/TIM tokens with 90-digit Decimal arithmetic and the F0/F1/F2 polynomial, then applies the documented first-TOA reference and nearest-pulse rule. Pulse counts agree exactly, including magnitudes around 7.8×10¹⁰. The basic 0.4 s comparison includes the slight decimal approximation already present in test1.tim.

All six exports regenerate exactly in scientific values, configurations, TOA identity, and resource provenance. Every exported scalar checks its own 36-significant-digit decimal round trip back to the identical `__float128` value. Python comparisons do not pass through binary64. Deliberately changing a residual by 2 ns or a pulse count by one is rejected. A missing time-ephemeris runtime returns exit 1 and publishes a failed manifest with no numerical stages; the next valid child succeeds. A malformed/unsupported TIM directive removes any same-name previous output. These are process-isolation checks, not proof of same-process library recovery: TEMPO2 can exit/abort.

Source correction to the brief: at this pin, `offsetSATs` already puts the expected offset **inside** `fabsl`. Its important defect is the unit of the threshold: `1e-9 days = 86.4 microseconds`, not 1 ns. The new diagnostic compares `abs(measured BAT change - 4 ns × estimated local slope)` in seconds. The slope ranges approximately 0.9999646–1.0000229; binary64 subroutines limit the response resolved by a 4 ns perturbation. This diagnostic does not certify a complete terrestrial physical model.

Latest unmodified narrow upstream test snapshot:

| Test | Result | Process return code |
| --- | --- | --- |
| `testLongDouble.precision` | pass | 0 |
| `testLongDouble.printAndParse` | pass | 0 |
| `testFormResiduals.basicBATs` | pass | 0 |
| `testFormResiduals.subtractBATs` | pass | 0 |
| `testFormResiduals.subtractSATs` | fail | -11 |
| `testFormBats.offsetSATs` | pass | 0 |
| `testEndToEnd.checkIdeal` | fail | 1 |
| `testEndToEnd.gracefulBadFit` | fail | 1 |

None of these tests is disabled. Signals and failures remain failures. `gracefulBadFit` is excluded from acceptance evidence regardless of its exit status because its final assertion is tautological. The full upstream suite was not run.

## Localized upstream failures and limits

`initialiseOne` does not initialize `pulsar::ne_sw_ifuncN`. `dm_delays.C:319–324` reads it and may call `ifunc` on absent solar-wind data. `initialization_probe.cc` fills storage with 0x55 and proves that the value **1431655765** survives initialization, parsing and `preProcessSimple`. The stage reproducer reaches finite clock/ephemeris quantities and then faults in `dm_delays` during the delay stage. This explains why stack-based original tests can fail or accidentally pass depending on allocation/layout, while fresh heap allocations can happen to work. Fresh-process repeatability is insufficient to remove this limitation.

The separately named diagnostic programs set **only** that count to zero. All four original residual/BAT tests then pass. This experiment is explicit and is not the reference library, a silent fallback, or an accepted correction. Even after that diagnostic initialization, end-to-end `checkIdeal` fails its **pre-fit** assertion at about **1.13759 ns versus a 1 ns threshold**. No fit is used to conceal that disagreement, and its cause is not yet established. The logarithmic stage evidence, original tests and diagnostic results are saved under `checks/`.

The harness supports the declared isolated-pulsar PAR/FORMAT 1 inputs and its named perturbation experiments. It rejects additional PAR keywords, TIM directives/flags, unsupported sites and nonfinite numeric tokens. It is not a general TEMPO2 CLI compatibility layer. No binary models, selections, INCLUDE/EFAC/TIME directives, fits, derivatives, plugin ABI, multi-pulsar execution, or Rust timing logic are certified. Some underlying upstream state remains uninitialized, and there has been no broad memory-safety audit. Only the fields with a traced initialization point are exported.

## Proposed contract and central decisions

1. Adopt the **GCC 16 binary128 build on this arm64 host** as the current executable reference candidate. Native long double on this host is inadequate. Keep production Rust precision/backend selection with assignment B and central review.
2. Adopt the simple JSON structure in `schema.json`: original input rows and hashes, resolved configuration/parameter flags, per-stage observation maps, exact integer pulse counts, decimal scientific values, and resource provenance. A minimal future interface needs explicit runtime resources, model and ordered TOAs as input; fixed-parameter phase/residual outputs and contextual failures. No backend abstraction is required by A.
3. Proposed cross-implementation budgets: **1 ps absolute** for BAT/BBAT differences after multiplying days by 86400, residuals, delay terms and vector light-times; **1e-10 turns absolute** for full phase; exact identifiers/flags/counts. Parsed parameter values use a provisional `1e-30` relative budget with exact zero. Frequency/uncertainty budgets and all field definitions are in `schema.json`. These proposals are deliberately distinct from the much tighter analytic self-check thresholds above, and remain subject to B's results and central adoption.
4. Approve a targeted source correction initializing `ne_sw_ifuncN=0` in `initialiseOne`, with a new recorded reference identity, then regenerate/review terrestrial fixtures. Keeping the pin unchanged leaves terrestrial outputs provisional; using a named `IPM 0` derivative would change the modeled physics and needs a scope decision. The diagnostic gives concrete evidence for the targeted initialization correction.
5. Decide how to handle the remaining 1.13759 ns original `checkIdeal` discrepancy and the bundled meteorology defaults before adopting the M1b terrestrial contract. Do not relax an acceptance tolerance automatically. Wider workspace Git organization remains open; this assignment's repository is scoped to its owned directory.

A's runnable build/export, provenance, serialization, comparison, barycentric precision and failure-isolation deliverables are satisfied. Terrestrial diagnosis and provisional artifacts are delivered. **A stable accepted terrestrial baseline, shared fixture/tolerance approval, production numerical representation, shared interfaces, and M0 acceptance remain open with the central chat.**
