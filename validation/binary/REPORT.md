# Assignment G — binary source-reference evidence

23 October 2026, Australia/Melbourne. Released at central `2f75c3001778916ce5fa986c1637534b5208f6ac`. Owned output: `validation/binary/` only. **Reference preparation delivered for central review; no Rust implementation, Rust acceptance budget or milestone acceptance is claimed.**

## Delivered evidence

The suite contains **75 process-isolated cases**, with ordinary finite delays for **all 14 dispatches**: BT, BTJ, BTX, ELL1, DD, DDK, DDS, MSS, DDGR, T2, T2-PTA, DDH, ELL1H and ELL1k. Twelve unsorted epochs cover both sides of the reference epoch, negative integer orbits and several orbital phases. **54 finite integrated cases** export both `formResiduals(...,0)` and `formResiduals(...,1)`. `pks_dd` is an ordinary PKS propagation-to-binary example. No fitting is performed.

`source_inventory.json` identifies **187 model/derivative-label pairs**, **189 explicitly named model/update-label pairs**, all referenced parameters, native/wrapper precision, and **525 source if-conditions** with pinned source line numbers. T2-PTA also has a catch-all update. Every named derivative/update label has runnable evidence in `coverage.json`; some derivative returns are explicitly withheld for undefined-source paths. This is not a claim of exhaustive runtime branch coverage. [BRANCHES.md](BRANCHES.md) lists tested optional paths, companion/subindex limits and remaining gaps. [schema.json](schema.json) records storage units, parser conversions, fitting coordinates, phase conventions and outcome representations.

Full-decimal PAR/TIM files, explicit request sequences and exported JSON/logs are committed. Values are serialized as 36-significant-digit strings, with an exact binary128 parse-back assertion. A native double is promoted exactly for export; it is never recomputed at higher precision. Native DD output and independently invoked binary64 fitting-wrapper output remain separate. Direct delay output also includes its explicit binary64 narrowing; the fitting wrapper is not called with the invalid negative delay label.

## Reproduction

From the primary workspace:

```sh
cd /Users/Eric.Dong/Downloads/tempo2rust
python3 validation/binary/check_reference.py
python3 validation/binary/check_reference.py --regenerate
python3 validation/binary/check_upstream.py
```

The first command validates existing artifacts, coverage, source inventory and mutation contracts. The second builds the observer, reruns every case in disposable directories under `validation/binary/build/`, compares every exported JSON/log byte, and verifies exact fixture-generator regeneration. The third rebuilds and runs the unchanged upstream DDH ideal test with its original threshold. It treats the documented failure as an expected observation, not a pass of the ideal test.

To deliberately regenerate the committed references, or inspect one case in scratch space:

```sh
python3 validation/binary/prepare_fixtures.py
python3 validation/binary/run_reference.py
python3 validation/binary/run_reference.py --case dd_baseline --output validation/binary/build/one_case
```

`run_reference.py` verifies both independent repository pins/cleanliness, the A2 library and its dynamic dependencies, the actual model/dispatcher/parser source identities, and the exact approved patches before execution. Rebuilding A2 itself is outside this assignment; the scripts require the existing verified local build. Neither reference repository is written by these commands.

Actual target: **macOS 26.7 arm64**, Homebrew **G++ 16.2.0**, `-O2 -std=gnu++11 -DHAVE_CONFIG_H`, no fast-math. The inherited A2 `longdouble` is GCC binary128, **113 significand bits**; C++ `double` and native macOS `long double` have 53-bit significands. The observer links the existing TEMPO2 static library, bundled Fortran SOFA, GCC quadmath/gfortran/C++ runtime and system libraries. No new dependency is installed. Other platforms remain untested.

## Provenance and source preservation

| Item | Identity |
| --- | --- |
| Canonical TEMPO2 | `fda6ad4a9753ad7dc5d291406f2f310a9161423f` |
| Independent A2 reference | `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572` |
| A2 static library SHA-256 | `313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5` |
| A2 build manifest SHA-256 | `a5daa986c6c054c7fa314271b5fa918cedc3f7412707a9cc7670ec3306d92dc8` |
| Approved initializer correction SHA-256 | `9d86ce0571e33aa28d0167137f37ee0afe524579a7b5e16dc81fee0d033c7b1c` |
| Existing observational instrumentation SHA-256 | `32cc90af950a01d49f9faacf80f1edeb4d393ecbec477aa0bd19076a0a3479ed` |

A2 adds `psr->ne_sw_ifuncN = 0` in `initialiseOne`. It also records file opens and pipeline stages. **This correction remains an explicit difference from the unmodified canonical source** under decision 0008; it is not hidden or treated as permission for another correction. No new scientific or initializer patch is applied.

Compiled model sources use the raw pinned Git blobs; the checkout expands `$Id$` keywords. The verifier checks the expansion against the raw blob and records both hashes. It reconstructs the recorded A2 patches in G's scratch directory, verifies byte equality against the actual A2 source files, and verifies the generated binary128 header. Some diagnostics retain the old build-time `tempo2-rust` path; that is a string baked into A2, not a second source tree used for this execution.

Each case records input hashes, source/build/executable identities, actual runtime file-open events, file sizes and SHA-256 resource hashes. `outputs/manifest.json` binds the exports, case definitions and fixture hashes. `input_hashes.json` binds the committed decimal files. `upstream_ddh.json` separately binds the unmodified test, original inputs, rebuilt driver, runtime reads and historical A2 localization.

## State, precision and integrated conventions

The observer calls `initialise`, `readParfile`, `readTimfile` and `formBatsAll`. Synthetic cases deliberately omit preprocessing, sorting, cuts, tracking, TZR phase activation and fitting. Their UNITS are explicitly TCB, CLK is TT(TAI), EPHEM is DE405 and EPHVER is 5. The first eligible TOA **in original input order**, at orbital offset 0.37, is the phase reference. It is not the earliest observation.

After propagation, all initialized parameter value/error/set/fit arrays and observation storage are saved. Every request restores that state. Native and wrapper calls therefore receive the same parameter/geometry state. A `repeat > 1` request deliberately retains state between its calls. Before/after mutations include value, error, presence and fit flags; updates also export the requested slot explicitly, including no-op updates. PAR uncertainty initializes `prefitErr`; the source resets the update-facing `err` field to zero. We do not read uninitialized prefit fields. Source warning deduplication remains process-local and is recorded in complete case logs; it is not reset by parameter restoration.

`formResiduals` evidence retains BBAT, torb, PET, full phase, reduced nearest-integer phase, pulse counts and raw/centered residuals. `phase_turn` is the source full phase after fractional phase-reference removal. `reduced_phase_reconstructed_turn` is explicitly postprocessing: subtract the source integer-phase expression from `phase_turn`. No instrumentation patch is added to expose the local `phase5` variable. The sign contract remains: add torb to spin time/fractional day and set `pet = bbat - torb/86400`.

Geometry is produced by the actual source pipeline. Exported nonzero DE405 Earth vectors and normalized `psrPos` are retained even for `@` inputs; no zero-vector substitution is used. Fixtures are equatorial J2000. The PKS case uses the accepted DE405/IF99 time/resource profile and the actual PKS → GPS → GPST → UTC → TAI → TT(TAI) clock chain. Its epochs fall outside the supplied weather tables, so source TROP1/TROP2 warnings and the **standard-pressure/zero-wet-delay fallback** are preserved. This is compatibility evidence, not a claim about historical weather accuracy.

## Actual observations

- All 14 defined baseline delay sequences are finite. Their maximum absolute delays range from about **1.23464 s to 1.32727 s**; exact per-model values are in `observations.json`.
- Native DD versus binary64 narrowing reaches **`1.0988742727273791033e-16 s`** in direct delay exports. The largest exported derivative narrowing is **`3.3968085627450286294e-11`**, for EDOT's source fitting coordinate (seconds squared), not a delay error. Every defined independently invoked wrapper equals the exported native narrowing exactly.
- The same binary64 PB increment (`1.24999999999999994343513978235782336e-7`) is added directly to stored days by **BT/BTJ/T2**; ELL1 and the other PB-update models divide it by 86400. BTX uses FB coordinates, with `(1e7)^(k+1)` scaling. OM/OMDOT, M2, BPJOM/BPJPB and MSS second derivatives have additional model-specific mappings documented in the schema. No shared-unit guess replaces them.
- Changing BT EDOT to zero produces **exactly zero delay difference**, while the nonzero derivative and additive update remain exported. The update-then-delay sequence also preserves that behavior. ELL1k ignores the supplied EPS1DOT/EPS2DOT values in its delay while exposing their derivative/update routes; its paired delay difference is exactly zero.
- DD, ELL1, ELL1k and T2 SINI clamps mutate source values. T2 also clamps negative SINI and invalid ECC. A tiny SINI excess is rounded away before ELL1's double comparison. At exact low-eccentricity conjunction, SINI=1 can produce **-Infinity**. These direct outcomes are retained, without calling integrated pulse conversion on them.
- DDGR `param=-2` writes SINI, OMDOT, GAMMA, PBDOT, DR and DTHETA; the second identical call makes no further parameter changes. ELL1H repeated A1DOT updates **add** errors. Binary update functions preserve unconditional negative/NaN error assignment or accumulation; M2's standard-frequency error guard is not imported into them.
- The direct exports contain **3018 finite**, **6 +Infinity**, **12 -Infinity** and **12 NaN** return records. Defined nonfinite arithmetic is not turned into an error or a finite substitute. Seven isolated source failures exit with code 1 at their observed stage. There were no observed signals or timeouts in the final suite.
- **432 native/wrapper return records are withheld** because their executed source path has undefined behavior. These are not numerical expectations, even where the selected compiler happens to return a finite number.

The important deferred defects/boundaries are:

| Source behavior | Reproducer/evidence | Disposition |
| --- | --- | --- |
| DDK derivative evaluates `dtdpx = dtdpx - daopx_dpx` before initializing dtdpx | `DDKmodel.C:267`, `ddk_derivatives_undefined` | Execute in isolated case; no numerical return golden. Baseline delay and updates remain separate. |
| T2 `getParameter` tests `k > aSize`; two-companion calls reach `k == aSize` for scalar arrays | `T2model.C:636`, `initialise.C:788`, `t2_companions` | Undefined-source model values withheld; valid indexed updates run in separate `t2_companion_updates` process. |
| ELL1H H3-only/H4 paths, and T2 H4 path, read uninitialized locals | `ELL1Hmodel.C:204`, `T2model.C:417`, named isolated cases | Values withheld. No initialization repair. |
| NHARM mode selection is overwritten by `mode=2` | `ELL1Hmodel.C:104`, `T2model.C:360`, H4/NHARM cases | Retained; no assertion that model-3 derivatives were computed. |
| ELL1 no-interpolation branch loops over ifuncN and may use unset ival | `ELL1model.C:190`, `ell1_orbifunc_step_undefined` | Isolated; no numerical golden. |
| BT EDOT and ELL1k EPSDOT exposed but ignored by delay | paired zero-input and callback fixtures | Deterministic behavior preserved for later porting. |
| MSS/DAOP and jump coordinate labels/conversions differ or conflict | schema unit map and source lines in inventory | Record executable conversion; defer scientific/unit correction. |

No Rust counterpart result exists for these rows yet. Corrections are deferred until after the overall transfer, under central direction. A successful run of undefined-source code cannot establish portable behavior or justify copying unsafe accesses into Rust.

## Verification and separate upstream failure

`verification.json` records byte-exact regeneration of every case JSON/log and the output manifest, exact fixture-generator regeneration, source/resource/build checks and **11 rejected deliberate mutations**: missing model, missing derivative label, changed PB units, omitted SINI mutation, changed NaN class, omitted result, manufactured undefined-source value, omitted update mutation, changed pulse count, changed wrapper value and fabricated zero geometry.

Decimal consistency replays are diagnostics, not Rust acceptance gates or independent binary physics validation. Their observed maximum discrepancies are **`2.8010597056e-25 s`** for the PET relation, **`5e-29 turns`** for reduced-phase reconstruction and **`4.8466666666666666666666666666666666666666666666666666667e-37 s`** for mean subtraction. Loose diagnostic bounds (respectively 1e-23 s, 1e-23 turns and 1e-30 s) allow serialization and source rounding in those field identities; they do not authorize any port discrepancy. Exact regeneration is the actual repeatability check, with maximum numerical difference **zero**.

The original `testEndToEnd.checkIdeal` remains **failed**, exit 1, with **`abs(residual) < 1e-9 s` unchanged**. Only its driver was rebuilt from the unchanged source to relocate DATDIR; the A2 timing library is reused. The live run again fails at the first residual around **1.13759 ns**. The separately hashed A2 localization records index **35**, **120/284** residuals outside threshold and maximum **4.59895516385995339247551543745087149 ns**. Those aggregate localization values are identified as reused historical A2 evidence, not newly recomputed by the assertion driver. The test stops at the first failure before fitting. Its failure is distinct from port-compatibility evidence and is not explained or corrected here.

The unfiltered whitespace check reports upstream trailing spaces in the two byte-preserved original `test3` inputs and in raw source diagnostic logs. Those bytes are intentionally retained. The scoped check excluding those input copies and `outputs/*.log` passes for the authored harness, synthetic fixtures, JSON and reports. No source input or diagnostic is trimmed to make the check pass.

## Files, commits and remaining work

| File | Responsibility / TEMPO2 counterpart |
| --- | --- |
| `export_binary.cc` | Model native/update calls, fitting wrappers, PAR/TIM input, formBatsAll/formResiduals observation |
| `prepare_fixtures.py` | No standalone counterpart; declared fixtures and inventory of model/reader source |
| `run_reference.py` | No standalone counterpart; isolated execution, A2 verification and resource tracing |
| `check_reference.py` | No standalone counterpart; evidence schema, callback/state/phase contracts and mutation checks |
| `check_upstream.py` | Unchanged `tests/testEndToEnd.C::checkIdeal` at 1 ns |
| `fixtures/`, `cases.json`, `input_hashes.json` | Original decimal declarations and request ordering |
| `outputs/`, `upstream_ddh.json` | Scientific return/state/phase evidence, diagnostics and provenance |
| `source_inventory.json`, `coverage.json`, `BRANCHES.md`, `schema.json` | Source conditions, callback/subindex/presence coverage and semantics |
| `observations.json`, `verification.json` | Observed numerical scales and actual validation results |

Source/fixture stage commit: **`95d25c1`** (`test(binary): add pinned source probes and declared fixtures`). Evidence/report stage commit: **`6027e11`** (`test(binary): record model behavior and reproducible reference evidence`). A subsequent report-only clarification records the intentionally retained whitespace in raw diagnostics. Use `git log --oneline -- validation/binary` for the complete delivery history.

Remaining work is explicit in BRANCHES.md: runtime branch instrumentation/exhaustive combinations, FB3–19, jump indices 2–4, third/fourth companions and unsafe companion boundaries, unsupported link combinations, legacy parser scaling/aliases beyond the declared inputs, ecliptic and non-TCB cases, singular/extreme parameters, other platforms, binary-fitting integration and the eventual Rust port. Central review must choose the implementation sequence and finite acceptance budgets after inspecting this evidence. No design decision or milestone is accepted by this report.
