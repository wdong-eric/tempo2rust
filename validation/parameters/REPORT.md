# Assignment K: parameter state and shared PAR reference

Delivered for central review, 23 October 2026. This is bounded native reference evidence under decisions 0008/0020; it does not implement Rust, set tolerances, close F's input lifecycle batch, or accept a milestone. All writes are under `validation/parameters/`. The initial delivery preserved I/J work; the decision 0024 correction preserves L's concurrent Rust work and both independent repositories. Acceptance remains with central.

## Delivered result

The inventory covers **166 enum entries** (162 physical parameters plus the marker and three virtual values), **2,000 allocated slots / 9,616 cells**, all **15 primary functions**, and **339 ordered top-level dispatch branches / 559 nested if decisions**. The executable foundation contains **306 cases, 1,545 begin/end events and 1,299 explicitly traced checkLine tokens**. Nineteen cases reach declared source exit 1; the rest return normally. Additional dispatches inside the wrappers are captured by aggregate GCC coverage and post-reader state.

All ordinary cases regenerated exactly in fresh processes. A separate GCC coverage build reproduced every ordinary record exactly and entered every primary function. Eight compiled disposable source mutations were detected in parameter state or termination events, not merely changed log paths. Nine comparator/freshness mutations were rejected, including missing cases/events/fields/components and a no-output replacement executable with stale outputs present beforehand.

| Evidence | Contents |
| --- | --- |
| [REGISTRY.md](REGISTRY.md), [registry.json](registry.json) | Enum/index/array/label map, all allocated cells, initialized versus unwritten fields |
| [DISPATCH.md](DISPATCH.md), [inventory.json](inventory.json) | Every ordered primary branch, nested conditions/scans, function and source anchors, explicit deferred work |
| [cases.json](cases.json), [observations.json](observations.json) | Complete ordered input/action identities, controlled prior state, fixtures, returned status, stream positions/next tokens, change masks, warnings and exit stages |
| [profile.json](profile.json), [input_hashes.json](input_hashes.json) | Canonical sources/support/config/compiler/runtime identities, probe and executable hashes, G fixture and central/F evidence hashes |
| [regeneration.json](regeneration.json) | Fresh ordinary execution identity and complete exact comparison result |
| [verification.json](verification.json) | Comparator/launch challenge results, all eight source patches, compiled identities, observed mutant outputs and rejection paths |
| [line_coverage.json](line_coverage.json), [coverage.json](coverage.json) | Separate instrumented profile, raw GCC counts and bounded branch/function dispositions |
| [FINDINGS.md](FINDINGS.md) | Scientific/data semantics, source bugs, wrapper differences, invalid-state boundaries and deferred work |
| [diagnostics/fb20.json](diagnostics/fb20.json) | One unintended out-of-bounds FB20 trial, isolated from ordinary numerical evidence |

## Decision 0024: verifier identity correction

Central independently replayed the original 306 cases / 1,545 events, then reproduced a provenance defect: `--no-build` reused `profile.json` as the executed identity and could attribute a forwarding wrapper's launch to the frozen native executable. This correction removes that option completely. `run.py` has one fresh-build path; `profile.json` remains the unchanged native reference baseline, and is never substituted for the new build result.

`compile_probe` captures the current Python runner/build/fixture/inventory/check scripts and native `probe.cc` hashes and checks that they remain unchanged during compilation. The shared `check_identity` gate compares the frozen stable native profile with the fresh profile, then checks the actual executable path/bytes and current runner/harness files against that fresh build. Every subprocess launch checks those executable and runner/harness identities before and after execution. The complete ordinary run checks again before writing its receipt. A full run removes its preceding success receipt before starting, so a failed new run cannot leave that old receipt looking like its result.

`regeneration.json` now contains both `executed_profile` and independently checked `execution_identity`; each compiled source mutation records the same execution metadata in `verification.json`. This correction changes four Python files (`profile.py`, `run.py`, `verify.py`, `coverage.py`), receipt metadata, this report and delivery hashes. It changes **no native source/harness bytes, source pins, generated configuration, cases, observations, scientific values, statuses, numerical budgets, registry, dispatch inventory or diagnostics**. In particular, no undefined diagnostic was executed.

The seven focused identity controls use the same normal gate: a fresh build is accepted (and its numeric case matches); deliberately wrong compiler, canonical source, runner and harness hashes are rejected; a same-path forwarding wrapper is rejected by its actual executable digest; and invoking the removed `--no-build` option exits 2 without changing the existing receipt's bytes or modification time. The normal complete regeneration, all nine existing comparator/freshness controls, and all eight compiled source mutations pass again. Exact commands are `python3 validation/parameters/run.py` followed by `python3 validation/parameters/verify.py`.

The measured coverage is **reused unchanged**, not relabeled as a new coverage run. `verification.json::coverage_reuse` checks its native source/header/generated-config/harness/compiler/runtime identities and the compile/link command, allowing only coverage instrumentation and output/map filenames to differ. `coverage.py` was updated to pass fresh identity data to the shared launch gate for any future rerun; its historical `line_coverage.json` is byte-for-byte unchanged. The normal execution metadata identifies the current Python files; historical coverage metadata retains its original meaning.

## Version-control stages

Harness and frozen observations: `474ce01cb9fdf2ee690fd98de5c56d4c1480412c` (`test: add canonical parameter and PAR reference harness`). The original report/evidence commit is `6d135816a4f57aa8019b011882a7e0d58b51bad6` (`docs: record parameter reference coverage and limitations`). Decision 0024 implementation and executed receipts: `ccdf55c8143d0986c5af478d74db329286f57354` (`fix: verify freshly built parameter reference identities`). The subsequent report/delivery metadata commit ID is returned in the delivery chat. Every stage uses explicit owned paths.

## Reproduce

From `/Users/Eric.Dong/Downloads/tempo2rust`, with Python 3 and the installed GCC 16 toolchain:

```sh
python3 validation/parameters/run.py
python3 validation/parameters/verify.py
python3 validation/parameters/coverage.py
python3 validation/parameters/catalog.py
```

For the decision 0024 correction only the first two commands were rerun; the coverage and catalog artifacts were retained unchanged after the native-identity reuse checks. Run these sequentially: they deliberately recreate the same per-case working directories. Each suite internally uses four independent processes; this does not exercise shared-state concurrency. No network, new package, Rust build or timing-resource installation is needed. A2's generated minimal `config.h` is read and hashed; **no A2 object or archive is linked**.

Initial generation used `python3 validation/parameters/run.py --record`. That explicit option replaces **only K** fixtures/goldens/profile. Ordinary `run.py` rebuilds the native probe, regenerates cases/inventory, compares all ordered records and stable build identities, and writes the regeneration receipt. `--select CASE_ID` is a diagnostic subset and never writes a full-suite success receipt. Builds, compiler logs, action files and temporary outputs are ignored under `build/`.

The build copies raw pinned Git blobs, not working-tree primary routines. Canonical `initialise.C`, `readParfile.C`, `TKlongdouble.C`, `tempo2Util.C`, `TKmatrix.C`, `TKlog.C`, `clkcorr.C`, `tabulatedfunction.C` and `observatory.C` compile directly. `global.C` contributes only its verbatim data prefix and CVS routine. Actual logging, matrix allocation and clock support are retained. The compile command and link-map digest identify the boundary; the A2 initializer correction and resource instrumentation cannot resolve into this executable.

## Executed identities

Canonical source: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. Independent A2 reference HEAD remains `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`. F's evidence remains its `15eee12` delivery, describing the fixed `ddc07b2` implementation baseline. K does not infer anything about the concurrently changing Rust implementation.

The ordinary recorded and regenerated executable SHA-256 is:

`ddbe600183cca59ccbddbe99f08229572eb16b77c56b4fe160341a3974df6fa2`

Complete hashes of the executed primary/support sources, generated config, numeric header, compiler, GCC runtime libraries and harness are in `profile.json` and `regeneration.json`. Each compiled mutant has its own executed profile in `verification.json`; coverage has its own profile in `line_coverage.json`. `delivery.json` records final tracked artifact digests and the two independent repository identities/statuses.

System libSystem is identified by the macOS platform and link dependency; its shared-cache binary is not extracted/hashed.

The numeric profile is GCC `__float128` (113 significand bits, 16 bytes) with source binary64 coordinate/scalar boundaries. Runtime capacities are `MAX_PSR=2`, `MAX_OBSN=2`; compile-time array sizes, enum ordering, constants and `MAX_FIT=10000` are unchanged. Locale is C. No fast-math or source expression rewriting is used.

## Primary function disposition

The counts below are the separate coverage build's function executions, not branch/path-completeness claims.

| Source function | Calls | Evidence |
| --- | ---: | --- |
| initialise.C:40 `initialise` | 1 | Two real pulsars, caller capacities, matching destruction |
| :56 `initialiseOne` | 306 | Natural parameter registry/defaults and explicit parser caller state; both fullSetup values |
| :777 `allocateMemory` | 308 | All slots/labels/counts; both realloc values on fresh lifetimes |
| :849 `destroyOne` | 287 | Matching successful init lifetimes; source exit processes terminate separately |
| :866 `destroyMemory` | 289 | Matching allocated-only and initialiseOne lifetimes |
| readParfile.C:47 `setupParameterFileDefaults` | 35 | Present/missing environment, partial write and return status |
| :69 `readSimpleParfile` | 6 | Repeated state, complete/partial ecliptic input, missing epoch |
| :132 `readParfileGlobal` | 6 | Repeated state, two pulsars/tpar[0], missing file, no epoch requirement |
| :159 `checkLine` | 1,364 | Scalar aliases, safe indexed families, controls, G's 14 models' PAR inputs, ordered stream behavior |
| :2381 private `checkBinaryParameterConsistency` | 50 | DDGR/FB rejection and ELL1 precedence through real wrappers/finalization |
| :2413 `checkAllSet` | 50 | Epoch metadata copying, conditional waves, ecliptic flags/errors, legacy clock remapping |
| :2542 `readParfile` | 13 | Repeated/multi-pulsar state, missing files/env, fixed header and HEAD/TOAS/noread behavior |
| :2762 `readValue` | 1,041 | Token forms, duplicates, precision/classes, coordinate/period partial returns, five links |
| :2948 `getValue` | 26 | Bounded fixed fields, P0/P1/P2, JD/MJD, coordinates, D-prefix difference, empty field |
| :3005 `removeCR` | 10 | LF, CRLF, CR and no-newline cases; empty string remains undefined/unrun |

Selected helpers `parse_longdouble`, `turn_deg`, `hms_turn`, `dms_turn`, `copyParam` and `getParameterValue` execute in their real source units. This does not close those whole support modules. In particular, clock chain graph/cache semantics remain J's assignment.

## Comparison and mutation integrity

The comparison projection is explicit: all recorded JSON types/key sets/list lengths/order/values, raw stdout/stderr text and warning order, and status/action stages are exact. Scientific numbers are never normalized. Only workspace/build path prefixes are replaced with named markers. Fresh UUID launch challenges are validated then omitted from the comparison; addresses, padding and unwritten values are not serialized. Linker temporary object names make link-map digests execution identities, not a stable equality gate. Stable source/config/compiler/runtime/harness identities are compared on regeneration. The actual launch executable digest and current runner/harness hashes must match the fresh build before/after each process; they are recorded separately from the unchanged reference profile.

Every invocation removes its case directory before creating action and output files. A random launch challenge must appear in the first probe event. The no-op-executable test rejects stale or absent outputs. Case directory names include an identity hash because this filesystem aliases names differing only in case; an early attempted run correctly failed the launch check on that collision. The corrected complete suite passed. Natural initializer fields and controlled prior parser fields remain separate; the change mask records observed value changes, with same-value write sets explained from source in FINDINGS.md.

| Compiled mutation | Witness | Gate that rejected it |
| --- | --- | --- |
| DM selects CM | alias_DM | Changed field membership |
| FB index forced to zero | index_FB | Changed field membership |
| prior uncertainty reset before parsing | value_forms | Lost duplicate/prior-state fields |
| RA uncertainty denominator doubled | coordinate_duplicate | Exact binary128 uncertainty value |
| linked PX reciprocal removed | link_DSHK_after | DSHK prefit 2 versus 0.5 |
| POSEPOCH copy omitted | final_defaults | Finalized metadata membership |
| full-reader nJumps reset changed | repeat_full | Count 0 versus 1 |
| missing-PEPOCH exit changed to return | final_no_pepoch | Event/termination stage |

Comparator mutations additionally remove/add/reorder cases, remove events/fields/components, alter a numerical class or action, and substitute a successful executable that produces no probe launch. Every one rejects. This establishes the reference gates reach the selected source decisions; it is not a Rust conformance result.

## Findings that matter for the next contract

- Canonical `prefit`/`prefitErr`, `ne_sw_ifuncN` and `nchromx` are unwritten. An allocator's convenient bytes must not become defaults. The 17/19 numerical sentinels and counter markers are declared caller inputs only.
- Numeric parsing is permissive and layer-specific. Repeated coordinate declarations can rescale a prior uncertainty twice. Negative RA and P1 before P0 return after different partial writes; dispatch ignores their returned error status.
- Links are directional and nonrecursive. DSHK stores PX but evaluates `1/PX`; repeated links can exit in different actions. Ordinary numeric assignment does not clear links.
- Full/simple/global reader lifecycles are distinct. Global reads tpar[0] for every object and skips ordinary finalization. Full's noread persists between pulsars: fixed-header input can suppress the next object's parsing; HEAD can make later ordinary input stop at TOAS.
- Scalar aliases, prefix/substring dispatch, strict index guards, legacy unit heuristics, label collisions and shadowed aliases remain source behavior. All 14 G input models parse through this source, but K does not evaluate any binary model.

## Remaining scope and proposed later batches

**116 top-level branches are explicitly deferred specialized/control branches.** Other rows have bounded scalar/index evidence, not exhaustive branch validation; the FD branch also contains deferred FDJUMP syntax. See every concrete line/scan in DISPATCH.md and the limitations in FINDINGS.md.

1. Jump and selector grammar: JUMP/SATJUMP, FDJUMP variants/scales, PHASE, TOFFSET and selection flags; complete/partial records, capacities, counts, unit/error semantics.
2. Sample tables and interpolation: wave/DM/CM/SW/quad/orbital/telescope samples, associated epochs and counters; distinguish source-unwritten counts before adding any samples.
3. Constraints/noise/GW compound records: constraints and special allocations; packed noise controls, event coefficients, groups/bands; full GW geometry and model-specific record fields.
4. Reader invalid-state dispositions and TIM/preprocessing: explicit central decisions for global scratch pointers, missing metadata/unchecked scans/index hazards; then real TIM grammar, preprocessing and integration. J's clock chain evidence remains a dependency.

The ordinary profile does **not** execute source-uninitialized/invalid-memory paths. Global ecliptic (including relevant PMRA/PMDEC guards), empty removeCR, unsafe/incomplete scans, overflowing indices and repeated invalid allocation lifetimes are source-classified. The accidental FB20 diagnostic terminated SIGBUS and has no portable successful result; its early executable hash and raw events are retained, but its pre-freeze harness source was not separately frozen. No safety tool blocked a requested experiment. No sanitizer, leak check, alternate platform, numerical/scientific convergence study or Rust comparison was run.

No canonical source, Rust representation, acceptance tolerance or historical evidence was changed. Central retains design, release and milestone acceptance.
