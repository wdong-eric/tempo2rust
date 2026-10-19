# Assignment A2 — corrected reference return report

Prepared 19 October 2026, Australia/Melbourne. Owned directory: `/Users/Eric.Dong/Downloads/tempo2-rust/validation/reference/`. This report continues A at `5289797`, implements decision 0001, and returns A2 for central review. **M0 and M1b remain pending.** The revision accompanying this report is available with `git log -1 --oneline` in this independent repository.

## Result and intentional correction

The current executable reference is the adopted GCC 16.2.0 binary128 configuration plus the approved one-line initializer correction. All six exports were regenerated. Every exported scientific stage, parameter value/flag and original observation is **exactly unchanged from A's committed outputs**, including all three barycentric cases. Terrestrial outputs are now compatibility candidates with declared meteorology defaults, supported by initialized-storage regressions; they are not accepted M1b goldens.

`initializer-correction.patch` adds only this line in `initialiseOne`, immediately after `ifuncN = 0`:

```cpp
psr->ne_sw_ifuncN = 0;  /* No solar-wind interpolation samples by default */
```

This establishes the empty solar-wind interpolation collection before parsing. The parser can then use index zero for the first `_NE_SW` sample; the delay calculation sees a positive count only after samples were supplied. No whole-structure zero fill, post-parse reset, IPM/NE_SW/SWM change, or delay-formula change was made. Nonzero storage poisoning exists only in regressions, before the corrected initializer.

`build.py` refreshes a disposable pinned archive, restores the same raw source blobs used by A (avoiding Git archive keyword expansion), applies the correction, then adds instrumentation. The instrumentation patch is **byte-identical to A**. Obsolete harness-only zero-count executables and current defect-expected checks were removed. Original uncorrected output, logs, source and reproducer remain recoverable from `5289797`; no uncorrected executable is a current reference.

The canonical `/Users/Eric.Dong/Downloads/tempo2-rust/tempo2` remains clean at `fda6ad4a9753ad7dc5d291406f2f310a9161423f` (TEMPO2 2026.09.1). Central documents and the precision study were not edited. The independent reference Git history is preserved; no parent repository, remote, or push was created.

## Corrected reference identity

| Component | Identity |
| --- | --- |
| Base commit | `fda6ad4a9753ad7dc5d291406f2f310a9161423f` |
| Initializer correction SHA-256 | `9d86ce0571e33aa28d0167137f37ee0afe524579a7b5e16dc81fee0d033c7b1c` |
| Instrumentation SHA-256 | `32cc90af950a01d49f9faacf80f1edeb4d393ecbec477aa0bd19076a0a3479ed` |
| Build manifest SHA-256 | `a5daa986c6c054c7fa314271b5fa918cedc3f7412707a9cc7670ec3306d92dc8` |
| Exporter SHA-256 | `04f2df5b63b73990fb5640285ce8fa245a3f2377e0711824a4bbd59c31260c24` |

Every export records all four reference-identity components. `build_manifest.json` records compiler options, dynamic dependency hashes, static-library and binary hashes, generated configuration and probe results. `manifest.json` hashes code, schema, patches, fixture exports, regression input, and verification JSON. `checks/results.json` records both old and new fixture hashes. Hash changes in exports reflect schema/provenance additions; scientific equality is checked separately.

## Files delivered and reproduction

| Component | Files |
| --- | --- |
| Corrected build | `build.py`, `initializer-correction.patch`, unchanged `instrumentation.patch`, `build_manifest.json`, `build_commands.json` |
| Export and tracing | `export_reference.cc`, `run_reference.py`, unchanged `resource_trace.cc/.h` |
| Initializer regressions | `initialization_probe.cc`, `fixtures/solar_wind_samples.par`, `checks/initialization-empty.log`, `checks/initialization-samples.log` |
| Schema and checks | `schema.json`, `compare.py`, `check_reference.py`, `manifest.json` |
| DDH localization | `diagnose_reference.py`, `checks/ddh.json`, `checks/ddh-diagnostic.log`, original `checkIdeal` test log |
| Regenerated artifacts | `outputs/*.json`, associated logs, `checks/precision.json`, `checks/results.json`, `checks/upstream.json`, failure-publication evidence |
| Usage and report | `README.md`, `REPORT.md` |

Run from the independent reference repository:

```sh
cd /Users/Eric.Dong/Downloads/tempo2-rust/validation/reference
python3 build.py
python3 run_reference.py --upstream
python3 check_reference.py
```

All three commands completed successfully. `--upstream` records individual failures rather than treating the entire legacy suite as passed. `check_reference.py` requires the corrected precision/residual/BAT tests to pass and retains the observed DDH failure separately. Its old/new comparison requires A's Git revision `5289797`. To regenerate separately and exercise the CLI comparator:

```sh
python3 run_reference.py --output build/regenerated
python3 compare.py outputs/barycentric.json build/regenerated/barycentric.json
python3 compare.py outputs/long_span.json build/regenerated/long_span.json
python3 diagnose_reference.py
```

Actual target: `macOS-26.7-arm64-arm-64bit`, Darwin arm64, SDK `27.0`, Python `3.11.2`, Homebrew GCC/G++/GFortran **16.2.0**, bundled Google Test **1.7.0**. C/C++ use `-O2`, C++ `-std=gnu++11 -DHAVE_CONFIG_H`; Fortran uses `-O2 -fno-second-underscore`. No fast-math is enabled. The direct build reads upstream core/FITFUNCS/SOFA source lists and generates enum/help/config files locally. OpenMP, BLAS, LAPACK, CALCEPH, QD and plugins are disabled. Autotools success is not claimed.

Retained numerical dependencies are the full TEMPO2 timing core, bundled Fortran SOFA, upstream JPL ephemeris reader/interpolation, GCC libquadmath/libgfortran/libstdc++, and macOS libSystem. Exact link paths/hashes are recorded. Build trees, source copies, runtime snapshots and binaries remain local under ignored `build/`; no downloads or global installations were required. Only macOS arm64 was executed.

## Initializer regressions and upstream tests

The regression starts from a whole `pulsar` filled with **0x55**, not allocator-provided zeros. Both cases require the corrected initializer to overwrite the count with zero before invoking the parser.

- Empty case: count stays **0** after parsing, `preProcessSimple`, `formBatsAll` and `formResiduals`. Seven terrestrial TOAs remain finite; solar-wind delay matches the no-sample expression exactly.
- Supplied case: `solar_wind_samples.par` adds `(MJD, value, uncertainty)` samples **(49000, 2, 0.25)** and **(51000, 6, 0.5)** to test1. They occupy indices 0 and 1; count stays **2** and times/values/uncertainties survive preprocessing and delay evaluation. The interpolated contribution is positive and active for all seven TOAs. Independently evaluating the piecewise linear sample contribution gives maximum delay difference **3.3881317890172014e-21 s**, within the regression's **1e-20 s** gate.
- The two terrestrial export pipelines also reproduce exactly when the solar-wind count and arrays are filled with **0x55** or **0xaa** before initialization. These checks establish independence from those storage patterns for the corrected collection. They are not a broad audit of all legacy uninitialized state or memory safety.

| Corrected-library upstream test | Result |
| --- | --- |
| `testLongDouble.precision` | pass |
| `testLongDouble.printAndParse` | pass |
| `testFormResiduals.basicBATs` | pass |
| `testFormResiduals.subtractBATs` | pass |
| `testFormResiduals.subtractSATs` | pass |
| `testFormBats.offsetSATs` | pass |
| `testEndToEnd.checkIdeal` | **fail**, exit 1, original threshold preserved |
| `testEndToEnd.gracefulBadFit` | pass, **excluded from acceptance evidence** because its final assertion is tautological |

Both original upstream test source files were verified byte-identical to the pin. The initializer source was checked against the raw pinned blob plus exactly the approved line. The canonical checkout, preserved original PAR/TIM copies, instrumentation and every manifest hash were verified independently before commit.

## Numerical and implementation checks

The probe still measures **113 significand bits**, 16-byte `__float128`, and 33 decimal precision digits. Native `long double` has 53 bits. The direct MJD-50000 4 ns arithmetic error remains **1.2084308783029334269794713885472227e-25 s**. Every scientific scalar is serialized with 36 significant digits and checked by exact parse-back equality to its binary128 value; comparisons use 90-digit Decimal arithmetic.

| Numerical diagnostic | Maximum absolute difference | Diagnostic gate |
| --- | --- | --- |
| Basic barycentric 0 / 0.4 s residual | `3.2000000189240046988e-17 s` | `1e-12 s` |
| Aligned subtractBATs residual | `1.5267298415285374e-25 s` | `1e-20 s` |
| 4 ns barycentric response | `5.541180890764174e-26 s` | `1e-20 s` |
| Long-span analytic phase | `4.977252493668111e-23 turns` | `1e-18 turns` |
| Long-span analytic residual | `3.762066354700268e-25 s` | `1e-20 s` |
| Terrestrial 4 ns response vs local BAT slope | `1.416875336047296e-13 s` | `1e-12 s`, proposed diagnostic |

The long-span oracle uses original decimal PAR/TIM tokens and an independent F0/F1/F2 polynomial, subtracts the first TOA's fractional phase, and agrees exactly on pulse counts around ±7.8×10¹⁰. These diagnostic gates remain distinct from the adopted cross-implementation budgets.

All six regenerated exports compare exactly in values, scientific and engine configuration, original identity, and file-open/resource provenance for the same build. All six also retain exactly A's scientific values; old engine configuration fields are moved out of the scientific projection and the newly defined zero sample count is recorded as an engine diagnostic. This confirms that A2 does not require repeating B's backend experiment for the unchanged barycentric fixtures.

Comparator checks reject a 2 ns residual error, a one-pulse error, changed clock convention or original token, unsupported parameters/observation state, failed calculations, provisional/smoke data, and status-only promotion of old uncorrected evidence. Half-budget perturbations pass and double-budget perturbations fail for epochs, phase, residuals, frequency and uncertainty; parsed-parameter relative and exact-zero violations fail. A reduced independent-engine projection passes without legacy display labels, unused zero fields, runtime reads, compiler information or allocation policy.

A missing runtime exits 1 and publishes no numerical stages; the next valid child succeeds. A malformed TIM directive removes stale same-name results. These are process-isolation/publication guarantees of the Python driver, not same-process library recovery from TEMPO2 exits/aborts. The low-level C++ TSV is staging data and is not a public result on failure.

The original `offsetSATs` source already takes an absolute difference, but its shared threshold is `1e-9 days = 86.4 microseconds`. The tighter diagnostic compares BAT changes in seconds against 4 ns times a ±1 ms finite-difference slope; it does not assume a unit derivative or certify terrestrial physical accuracy.

## Schema 2 and the M1a projection

Decision 0001's barycentric gates are implemented: exact identity/order/original tokens, relevant parameter presence/fit flags and pulse counts; **1e-12 s** for SAT/BAT/BBAT differences after multiplying days by 86400 and for residuals; **1e-10 turns** for full phase; **1e-30 relative** parsed-parameter difference with exact expected zero; **1e-9 MHz** for frequency and **1e-12 microseconds** for uncertainty. Terrestrial field budgets are labeled proposals, not adopted universal thresholds.

`schema.json` enumerates the C adapter projection. It includes scientific conventions, F0–F3 and the fixture parameters, original ordered TOA records, parsed fields and initialized timing stages. Astrometric/DM/resource/TZR settings inactive for `@` are preserved explicitly. Missing F4–F12 and TRACK state need not be fabricated; any supplied such parameter is rejected, even zero. Known zero binary/secular delays can be omitted; nonzero values or unknown scientific fields are rejected. The original input parser remains C's responsibility; this fixture comparator is not a general PAR/TIM validator.

Engine provenance is separately retained and ignored for cross-engine equality: compiler/dependencies, memory policy, harness scenario name, diagnostic verbosity, legacy fit mode, diagnostic sample count, resource paths and file-open events. Scientific configuration and original TOA identity remain exact. C should map its own results to the projection and evaluate the declared stage epochs for aligned/4 ns tests; it need not imitate unused legacy I/O. This is a fixture adapter, not a new public Rust interface.

Time-scale meanings are explicit: **pks/7 SAT is observatory-clock time; `@` SAT is already barycentric TCB; BAT and BBAT are TCB for these fixtures**. Test1 omits UNITS, retains `initialiseOne`'s `SI_UNITS`/TCB default, and supplies EPHVER=5; the optional preprocessing rule does not activate TEMPO1 emulation at that value. Long-span explicitly supplies UNITS TCB. Minimal test pipelines preserve the original omission of preprocessing; example1 uses it.

The phase reference remains the fractional phase of the first eligible TOA in original order, with `formResiduals(...,0)`, no residual-mean subtraction, sorting, cuts, pulse tracking or fit. TZR metadata does not activate REFPHS_TZR. Fixed-parameter `residual_s` is exported; `prefitResidual` is omitted because fitting has not initialized it. Terrestrial-only stage fields are omitted for `@` where their lifecycle does not define them. The schema also describes Shapiro terms as signed stored corrections, rather than incorrectly implying all terms are positive.

## Bounded DDH failure record

The original upstream assertion is still **abs(residual) < 1e-9 s** and still fails before fitting. `checks/ddh.json` preserves full original `test3.par` / `test3.tim` text and hashes, parsed model, all 284 residuals, resource hashes/coverage, and full parsed/BAT/residual snapshots for the phase reference, first failure, and largest failure. The PAR NTOA metadata says 294; the actual TIM file and parsed output contain **284** observations.

| Item | Recorded value |
| --- | --- |
| First failing zero-based index / TIM line | **35 / 39** |
| Original SAT token | `55594.45971978297532701` |
| Original frequency / site | `1369.24600000 MHz` / `7` (PKS) |
| First failing residual | `1.13758502732523367544302697283684274e-09 s` |
| BAT = BBAT at first failure | `55594.4607074746065559806423713149974 MJD TCB` |
| Stored binary delay | `4.82969626520771733169112849282100797 s` |
| Maximum absolute residual, index 38 / line 42 | `4.59895516385995339247551543745087149e-09 s` |
| Number outside the unchanged threshold | **120 / 284** |

The model uses **DDH, DE414, TT(TAI), IF99**, nonzero proper motion/parallax and fitted-flag metadata, with MODE/EFAC/backend flags and zero-valued jumps. No fit is performed. DE414 header coverage is MJD **36912–58864**; IF99 is **38736–69808**. The actual opened files and clock chains are recorded, alongside the declared atmospheric defaults. The diagnostic does not change the threshold, switch binary models, suppress active solar-wind physics or replace runtime resources.

Clock, ephemeris, propagation and binary-delay stages are finite. A 90-digit Decimal replay of the downstream spin polynomial from **exported BBAT and binary delay** reproduces phase within `5.2797830426893648224463511148179844365863084751413017437E-24 turns` and residual within `7.037787904184198639495872887471559271134924521960606284447187088150081601E-27 s`, with exact pulse counts. Thus extended spin arithmetic after those supplied inputs does not explain the nanosecond discrepancy. The replay does **not** independently validate BBAT, the DDH delay, their reference origin, or any correction component. The cause remains unknown; the presence of DDH alone is not evidence that DDH is wrong.

No evidence from this bounded localization overturns the independently checked isolated barycentric fixtures. Full causal diagnosis and validation of this broader DDH/DE414 path remain a later coverage task, as decision 0001 permits. Reconsider the selected fixtures if later evidence implicates a component they share.

## Terrestrial availability and remaining decisions

Recommend reviewing `terrestrial` and `terrestrial_4ns` as **corrected compatibility candidates with declared meteorology defaults**. They use the unchanged isolated model: DE405, IF99, IAU2000B, frequency dilation, planetary Shapiro, IPM=1, NE_SW=4, SWM=0, troposphere enabled. DE405 coverage is MJD 33264–69808; usable EOP samples are 39491–61163. Applied clock paths and values, indexed-but-unused clock files and their hashes are preserved separately.

The PKS weather tables cover only MJD **53000–53001**. All seven minimal terrestrial epochs are outside those tables, so the legacy calculation uses **101.325 kPa surface pressure and zero zenith wet delay**. TROP1/TROP2 warnings remain in logs/JSON. These are compatibility assumptions, not validated historical weather. The table ranges describe operational first/last samples, not independently certified coverage.

A2's source correction, regressions, corrected identity, regenerated evidence, unchanged barycentric values, schema reconciliation and bounded DDH record are delivered. The central chat should review this revision before accepting **M0** or releasing C. Existing adopted Rug128/interface/Git decisions need no new experiment from A2. Central review must separately settle terrestrial fixture acceptance/budgets and eventual production missing-meteorology policy before M1b; broad DDH diagnosis and Linux execution remain open. No milestone is marked accepted here.

## Current fixture artifact hashes

| Fixture | SHA-256 |
| --- | --- |
| `barycentric` | `a9bac02d3ebeb4a65935f208b6e1ca8a58362d604d66a1fb55b9ea2f788a675e` |
| `barycentric_4ns` | `44a2dfbbb294e932fc66657f2ca8b47f866d17bb26b337c11e6ebe8786e4d9af` |
| `long_span` | `d62cc4cfd2a702a255f0c0d3b392c4cf9f93a2ba10feb84964feabca14069b44` |
| `terrestrial` | `c19aac714e38521c5dd309dbf50f815f49430171430206d7af969fc790043d83` |
| `terrestrial_4ns` | `5455b5dc4d387ec8bea42a43665dffe0b3d7a39ced47e0f131395d003adb62eb` |
| `example1` | `d9c46d81a4a18a5d0a14fad81986cad005ba77828b2b670e4e6eebae95e9756d` |
