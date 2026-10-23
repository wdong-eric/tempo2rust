# Assignment F — pinned core and fidelity inventory

Delivered 23 October 2026, Australia/Melbourne. Owned path: `validation/coverage/` only. **The source inventory and baseline evidence audit are complete; the full core transfer is not complete.** No implementation, reference, central policy, milestone acceptance or existing regression expectation is changed.

The expanded build contains **93 unique root-library translation units**, **121 bundled SOFA routines** and **44 unique associated headers**. At the fixed Rust baseline, 25 root units are partially represented, 67 have no implementation counterpart, and one (`jpleph.c`) is retained native with modifications. Of SOFA's 121 routines, 36 are retained unchanged and 85 are outside the retained closure. These are source-accounting counts, not percentages of scientific work or completed functionality. No root timing source is marked fully transferred merely because its filename has a Rust counterpart.

The audit identifies deterministic behavior changes that earlier acceptance did not resolve under the new preservation policy. A fresh replay confirms the JPL repeated-read status change; fresh table/parser probes establish several finite/nonfinite source outcomes that the strict Rust boundary rejects. All 19 production Rust files and eight unsafe sites are accounted for. JPL has an unenforced file-stability assumption and a retained packed-field alignment risk at its FFI boundary.

## Fixed identities and reading convention

| Role | Git identity |
| --- | --- |
| Unmodified canonical target, independent `tempo2/` repository | `fda6ad4a9753ad7dc5d291406f2f310a9161423f` |
| Primary Rust baseline, read through Git objects or its isolated snapshot | `ddc07b20fafc21fe92f2ecc1dd24c59db9df4142` |
| Central preservation policy / F release | `1322f8b` (full identity in inventory.json) |
| Corrected A2 reference, independent `validation/reference/` repository | `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572` |
| Original A evidence retained for initializer defect | `5289797` (full identity in inventory.json evidence list) |

Both independent repositories were clean at their stated identities at entry. Primary entry HEAD was the policy commit. E's subsequent live fitting changes and other concurrent paths are not F's baseline and were not edited or staged by F. All reproducible inventory reads use `git show`, not live Rust files. A temporary `git archive ddc07b2 rust` snapshot was used only for inspection. The verifier resolves every source/Rust/native reference and line against its declared Git blob.

In the accompanying reports, `S:file:line` denotes the canonical pin and `R:rust/file:line` denotes `ddc07b2`. Use these commands for exact text rather than assuming the live line is unchanged:

```sh
git -C tempo2 show fda6ad4a9753ad7dc5d291406f2f310a9161423f:FILE | nl -ba
git show ddc07b20fafc21fe92f2ecc1dd24c59db9df4142:rust/FILE | nl -ba
```

Hashes are over raw Git blobs. Some working/build headers have Git `$Id$` expansion; this does not move source line numbers. The table probe records and checks that difference explicitly for `tempo2.h` and compiles a raw pinned header. A2's initializer patch and resource instrumentation are distinguished from the canonical target throughout.

## Deliverables

| Artifact | Contents |
| --- | --- |
| [inventory.json](inventory.json) | Stable source keys, exact expanded library lists, source/Rust hashes and line anchors, generated-source recipes, associated headers, lexical parameter/directive/conditional indexes, all production module/unsafe findings, behavior ledger, dependency batches and historical evidence identities |
| [SOURCE_INVENTORY.md](SOURCE_INVENTORY.md) | Readable row for every root source, all 121 SOFA routines, all headers, separate programs and conditional dependencies |
| [CORRESPONDENCE.md](CORRESPONDENCE.md) | All 19 module header assessments, notation/counterpart findings, eight unsafe-site contracts and the JPL file-stability finding |
| [BEHAVIOR_DIFFERENCES.md](BEHAVIOR_DIFFERENCES.md) | 29 classified ledger entries, reproducers, source/Rust outcomes, existing tests that encode departures, evidence confidence and required next evidence |
| [STAGE_PLAN.md](STAGE_PLAN.md), [batches.json](batches.json) | Concrete candidate batches, dependency relationships and proof requirements; every source row remains open for final disposition |
| `source_map.tsv`, `module_map.tsv`, `unsafe_sites.tsv`, `behavior_ledger.tsv` | Small, reviewable curated inputs used to generate the JSON/readable tables |
| [inventory.py](inventory.py), [check_results.json](check_results.json) | Reproducible inventory/line/hash checks, independent GNU make expansion, retained SOFA closure verification and negative checks |
| [reproduce_native.py](reproduce_native.py), [native_replay.json](native_replay.json) | Exact original-versus-retained JPL short-read replay and layout-only alignment probe with hashes, commands and outputs |
| [table_probe.cc](table_probe.cc), [reproduce_tables.py](reproduce_tables.py), [table_replay.json](table_replay.json) | Defined table interpolation and decimal-parser source outcomes, exact fixtures, library/source hashes and commands |

All generated builds are ignored beneath `validation/coverage/build/`. No new numerical dependency, whole-program port, repaired legacy code, or fixture-gate change is introduced.

## How the exhaustive source set was established

The source definition is the pinned `Makefile.am:37–44,78`, not the existing Rust test cases. `FITFUNCS`, `$(FITFUNCS:.C=.h)`, `headers` and `BUILT_SOURCES` are expanded. Python's expansion is independently checked by GNU make evaluating the same pinned assignments. Every expanded file occurs exactly once per target and exactly once in the union inventory, with its target memberships retained.

| Target | Expanded entries | Translation units | Associated headers |
| --- | ---: | ---: | ---: |
| `libtempo2` | 136 | 92 | 44 |
| `libt2toolkit` | 19 | 12 | 7 |
| `libtempo2pred` | 48 | 4 | 44 |
| bundled `libsofa` | 121 | 121 | 0 |

Library sets overlap. The union of the three root libraries has 93 units: `t2toolkit_global.C` adds the standalone toolkit state to the core set. The blanket `headers` variable contains generated **`enum_str.C`**, so the expanded prediction list also contains that compilation unit. Generated files are identified by pinned inputs/rules rather than assigned invented source hashes. The 13 variable-expanded fit/constraint sources all have rows. `choleskyRoutines.C` remains core despite the nearby historical `xtra_dist_sources` comment. `GWsim.C`, spectra and nestlike/constraint sources remain core even where plugin consumers are prominent.

The script also accounts for all other root C/C++ units: `tempo2.C` is a separately audited core executable; `getPeriod.C` is an installed predictor client; `convertxt.c` is a build helper; `age.C` is an unbuilt root source absent from these pinned targets. `plugin/`, `unsupported_plugins/`, `temponest`, test executables and Python wrappers are separate build/workflow surfaces. This separation does not remove embedded core fit/output dispatch, model routines or prediction APIs from the inventory.

SOFA is an explicitly retained numerical dependency, not hidden timing implementation. Its entire source list is present, and the verifier follows native `iau_*` dependencies to establish that the seven FFI entry routines have exactly the retained 36-file closure. All those files, unchanged JPL headers and native notices match the pinned native manifest. The 85 other SOFA files need a dependency/disposition review when their source branches are transferred; the count does not imply that 85 Rust rewrites are mandatory.

`Makefile.am:48–67`, `:91–94`, `:164–170` and `configure.ac` identify SOFA, OpenMP, BLAS/LAPACK, CALCEPH, QD/float128/native precision and ordinary numerical runtime alternatives. The reference disables OpenMP/BLAS/LAPACK/CALCEPH/QD; disabled source paths remain inventoried. CALCEPH's no-library failure stubs are also present. GSL detection is not treated as evidence that the root core links GSL. The tested Rust native build retains GCC/GFortran JPL/SOFA, while Rug uses GMP/MPFR; neither a current SOFA/ERFA replacement nor a new linear algebra backend is inferred.

## Core entry paths and actual coverage

The public/core path, with exact source anchors, is:

1. `tempo2.C:51`, `getInputs.C:52`, `initialise.C:40,56`: process options and pulsar state; then `tempo2.C:361,366,371` calls PAR input, TIM input and preprocessing.
2. `global.C:134` calls `refphs_init`, clock corrections, ephemeris routines, extra delays, `formBats`, `secularMotion`, then `refphs_clean`. `global.C:75,103` preserve rough/refined time and ephemeris passes. `global.C:163` exposes the partial-recalculation route.
3. `formResiduals.C:396` supplies the full phase/model machinery. `tempo2.C:512–538` drives NITS, pre/post cycles and mean-subtracted residuals; `tempo2.C:565` invokes modern `t2Fit`; `:573` emits text output.
4. `t2fit.C:811,888,904,917,950,1593,2061` connect selection, matrices, constraints, callbacks and updates. These include every generated fit/constraint file, not just spin derivatives. Toolkit/SVD/Cholesky and optional accelerated branches remain distinct dependencies.
5. `tempo2.C:379` selects polynomial/Chebyshev prediction; `polyco.C:90`, `cheby2d_int.C:150,178`, `tempo2pred.c:39,286,304,323` and `t1polyco.c:35,62,177` connect construction, persisted models, phase/frequency evaluation and planning.

Baseline Rust supports the homogeneous `@` or PKS FORMAT 1 subset; it has 13 numeric PAR keys and five text keys. The keys and source dispatch are recorded exactly. Missing supported F1–F3 terms become unset zero coefficients; metadata such as TZR fields is preserved without activating TZR reference phase. PKS fixes DE405, IF99, TT(TAI), TCB, IAU2000B and the declared propagation settings. It requires RAJ/DECJ/DM/POSEPOCH/DMEPOCH and expected resource labels. Binary parameters, proper motion, parallax, secular corrections, alternate units/time/site conventions, higher derivatives, directives, tracking and noise/model terms are rejected rather than partially applied.

The source-to-Rust map records those feature boundaries for every partial file. In particular, the spherical default solar-wind expression is part of **`dm_delays.C:303–327`**; it does not transfer **`sw_delay.C`**'s SWM=1 current-sheet model. Likewise, preserving TZR tokens is not a transfer of `refphs.C`, modern clock-chain conversion is not an implementation of every legacy clock routine, and static direction cosines are not a transfer of full astrometry.

At `ddc07b2`, fitting is absent. E's current changes cannot close any F row. Even successful delivery of its ZERO/F0–F3 subset would leave most branches of `t2fit.C`, `TKfit.C` and `t2fit_stdFitFuncs.C` open, alongside all additional callbacks, robust/correlated/global/constraint fits and prediction.

The lexical indexes preserve navigation to parameters, string dispatch, includes and compile-time branches, including inactive branches. They are **not** an AST, dynamic call graph, proof that every discovered function is public, or exhaustive branch-level runtime validation. Exhaustiveness here is checked source membership and a disposition for every file; all still-unproved features stay visible in the per-source remaining-work text and batch requirements.

## Fidelity findings requiring later disposition

The full ledger separates deterministic output/status differences, defined nonfinite floating stages, undefined source state/access, explicit API adapters, representation differences and unimplemented coverage. Most useful first reconciliation targets are:

| Ledger | Observed or established source behavior | Baseline difference and existing expectation |
| --- | --- | --- |
| B03 | Fresh original JPL probe: `0 -2 0 0`, recovered position delta 0 | Retained native: `0 -2 -2 0`, delta 0. `native_read_failure_recovers_without_stale_cached_record` requires the corrected second error. |
| B07 | Fresh raw source table probe skips `bad row`, retains two samples and returns 0 | `table_steps_endpoints_and_invalid_data` requires parse failure. |
| B08 | Fresh source interpolation returns NaN for all-duplicate and duplicate-final brackets | Existing test requires constructor or zero-width-interval error. Defined NaN is not uninitialized storage. |
| B09/B10 | Fresh source singleton and decreasing-table off-range proxies return 0; decreasing data warns | Rust rejects the tables globally. Exact-epoch singleton interpolation is a separate invalid-access case that F did not execute. |
| B12 | Source fallback exhaustion returns an empty clock chain, whose TT sum is zero (`clkcorr.C:781,810`) | Missing indispensable UTC→TT is an error. Existing clock/public recovery tests encode that changed outcome. Source result is inspected, not newly executed. |
| B14/B15 | Previous vectors and process-global resource caches affect source repeated-call inputs | Rust reinitializes rough-pass vectors and owns resource state. Tests require independent/recovered equality; paired canonical numerical deltas have not been established for every subsystem. |
| B16/B17 | Invalid geometry/overflow may produce defined nonfinite floating stages before later unsafe conversion | Rust preempts these with errors. Existing overflow-DM/geometry tests are error-boundary tests, not exact source-class equality. |
| B18/B19 | Sequential parser assignment and permissive decimal helper differ from strict input policy; fresh helper returns 1 for `1junk` and `0x1p0`, NaN for `NaN` | Duplicate/strict-decimal tests encode rejection. Full malformed PAR parser contexts need their own probes. |

A2's one-line `ne_sw_ifuncN=0` correction is B01. Original A's retained poisoned-storage probe observed **1431655765** through initialize/parse/preprocess; A2's assertions require zero and preserve supplied samples. Ordinary uninitialized storage has no portable expected value, so the ledger does not prescribe reproducing the poison pattern or unsafe access. A2 and original A's six exported scientific projections were unchanged, but that fact does not make the initializer source-faithful.

D's JPL header zero initialization is B02, distinct from its deterministic cache change. The exact decision-0005 malformed-step reproducers are retained: **DE405 byte 2668** and **IF99 byte 280**, big-endian `32.0 → 1e-300`. Historical evidence records pre-correction SIGBUS and Rust overflow panic, then constructor errors and same-process recovery in debug/release. These are not claimed as fresh F timing runs or as a complete classification of all malformed canonical inputs. Native integer/pointer hazards, valid-but-restricted layouts, checked exits and nonfinite arithmetic are separated.

The original DDH ideal failure remains open: 120/284 TOAs outside the original 1 ns criterion, maximum about **4.599 ns**, before fitting. Its existing snapshots/reproducer remain at A2's `checks/ddh.json`. No binary model is implemented at the baseline, and no source bug cause or remedy is inferred from that failure. The tautological `gracefulBadFit` assertion remains excluded as robustness evidence. M2's square-fit/nonfinite and underdetermined cases are listed as future-stage source evidence, not nonexistent baseline Rust tests.

Several suspicious expressions were already preserved: the repeated `obsTerm` scaling in `tt2tdb` and `NMF_wet`'s `b[j]` interpolation term. F records them without correcting or asserting a newly proved physical defect. Approved Rug128-versus-binary128 representation and decimal output differences also remain explicit; budgets do not authorize a different algorithm.

## Counterpart and unsafe findings

Of 19 production Rust files, **one header is adequate, 12 are incomplete, five are missing counterpart declarations, and one is misleading** under decision 0008's file-and-routine requirement. Many scientific filenames are recognizable, but file-only attribution does not identify all fused source routines. `longdouble.rs:1` cites nonexistent `longdouble.h`; the pinned generated header is `TKlongdouble.h`. `form_bats.rs` also implements pieces of `shapiro_delay.C`, `dm_delays.C`, `vectorPulsar.C` and the zero secular branch beyond its named counterparts. Rust ownership/serialization/build glue needs to say that it has no standalone source routine and identify the responsibility it supports.

The eight unsafe sites are two extern declarations, one SOFA-call block, and five JPL blocks (init, init-error, field read, interpolation, close). There are no unsafe functions or unsafe implementations. None has the complete adjacent necessity/invariant explanation requested by decision 0008. [CORRESPONDENCE.md](CORRESPONDENCE.md) records the actual array lengths/layout/alignment, disjoint pointer regions, initialized outputs, pointer lifetimes, native ownership/cleanup and threading constraints for each.

For JPL, `NonNull<c_void>` is not Send/Sync and the handle is not cloned; interpolation requires `&mut self`, initialization/error retrieval holds a mutex, and Drop calls the native allocator's close/free path. These facts support the existing boundary but do not enforce stable resource files. **U1:** Rust validates/hash-records one byte image and native init independently reopens the path. The file can differ between validation and native use; the current safety proof assumes unchanged contents/identity. F did not execute a race or modify the wrapper. This needs a bounded owner decision and evidence, not just a new SAFETY label.

**U2:** `jpl_int.h:30` packs the native struct; `jpleph.c:75` implements the generic getter by casting `base + field` to `double*` and dereferencing. Rust requests field 28 for AU. A fresh layout-only probe measures `offsetof(au)=28`, `alignof(double)=8`, `sizeof(jpl_eph_data)=648` on this host. Allocation alignment does not establish alignment of `base+28`. This is an unresolved portable C alignment contract in retained source, not a new Rust behavior correction. F did not run a sanitizer or deliberately execute the unaligned getter in that probe, and does not claim a crash. The complete invariant account must include this gap rather than claim all native field pointers are aligned.

For SOFA, 9-element matrices, 6-element position/velocity arrays and 3-element vectors are initialized, aligned, synchronous and nonescaping. `add(3)` remains within a six-element array; the cross-product input/output regions are disjoint. Column-major storage stays opaque. No existing validated safe replacement is present. `-frecursive` addresses mutable local storage but sequential fixture tests are not concurrency stress tests.

One documentation correction is also due: `ifteph.rs:133` describes upper-date reads as unchecked, whereas source `ifteph.C:266` checks short data reads and exits. Source header reads at `:150` are unchecked. The report distinguishes those paths instead of inheriting the misleading description.

## Verification performed

Run from the workspace root:

```sh
python3 validation/coverage/inventory.py
python3 validation/coverage/reproduce_native.py
python3 validation/coverage/reproduce_tables.py
git diff --check -- validation/coverage
```

To intentionally regenerate checked machine/readable reports after reviewing curated changes:

```sh
python3 validation/coverage/inventory.py --write > validation/coverage/check_results.json
```

Actual F execution: macOS 26.7 arm64, Python 3.11.2, GNU make, GCC/G++ 16.2.0. The probes use `-O2`, with C++ `-std=gnu++11 -DHAVE_CONFIG_H`; no fast-math. Native replay compiles both exact Git blobs, uses an owned copy of pinned DE405 truncated by 4000 bytes, and matches D's recorded source hashes/statuses. Table replay compiles the raw canonical table source ahead of the verified A2 support library, checks parser/header identity and library SHA-256 `313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5`, and records exact fixtures/classifications. It does not call A2's corrected initializer or timing engine.

The inventory checker passes:

- Exact source/header memberships, unique stable keys, all FITFUNCS substitutions and independent GNU make expansion.
- Every Rust/native source mapping resolves to the declared Git blob and line; all production Rust files and unsafe tokens are accounted for.
- All 36 retained SOFA files and their transitive closure; retained native provenance hashes, including unmodified headers/notices.
- All 96 entries of the baseline M1b source manifest. All ten historical correction-equivalence hashes match preserved outputs; three ignored target outputs are verified through their identically hashed tracked `rust/verification/` copies, explicitly recorded in JSON.
- Readable reports regenerate exactly from the checked inventory; omitted-source, duplicate-source and dangling-Rust-mapping mutations are rejected.
- Fresh JPL statuses/recovery and the five table outcomes/three decimal-helper outcomes described above.
- A non-dereferencing native layout probe confirms the AU offset/alignment discrepancy; this is layout evidence, not runtime memory-safety validation.

These checks establish inventory consistency and the narrowly executed source behavior. **F did not rerun Cargo tests, fit comparisons, the full timing verifier or DDH diagnosis.** Existing M1a/M1b results are historical, identified and source-hash-checked: maximum M1a residual difference approximately `3.76201e-25 s`; maximum M1b BAT/residual differences `5.684341885448086137600e-14 s` / `1.13686836941064559950026880243e-13 s`, under the unchanged 1 ps gates. They establish only their declared fixed-parameter fixtures. No new platform, concurrency, performance, packaging or scientific-accuracy claim is made.

## Remaining work and handoff

[STAGE_PLAN.md](STAGE_PLAN.md) supplies 17 candidate batches: departure reconciliation; input/default/preprocessing lifecycle; numerical helpers; resources; astrometry/time; propagation; dispersion/solar wind; residual models; Keplerian binaries; relativistic binaries; standalone toolkit; bounded E fitting; extended derivatives/updates; noise/global/constraints; GW/spectra; prediction; output/integration. Their dependency edges and per-batch proof requirements are machine-readable. Every source row also retains its specific unimplemented/partial responsibilities and proof requirement; a passing fixture batch does not erase uncovered rows.

Recommended central next step: inspect the deterministic ledger targets and U1/U2, then assign bounded reconciliation after E's ownership window. Review E independently against the amended fitting contract. Choose subsequent batches from all open source rows, including toolkit and prediction. Require separate classifications for undefined source cases and API adapters rather than inventing results or treating historical approval as full fidelity.

Assignment F's requested deliverables and evidence are complete. **M2 acceptance and full transfer acceptance remain central decisions.** No deterministic departure is grandfathered, and none is corrected by this assignment.
