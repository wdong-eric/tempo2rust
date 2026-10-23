# Assignment J: table, clock and active toa2utc reference

Delivered for central review on 23 October 2026 under release `67aa77c` and decision 0018. This is bounded source-reference evidence. It changes no Rust implementation, defines no Rust tolerance, accepts no milestone and does not close the broader core inventory.

The supported suite passes: **151 catalogued cases, 142 executed cases (122 ordinary and 20 terminating), 972 completed ordered actions**, exact fresh-process replay, **27 behavioral assertions, 19 rejected artifact mutations and 12 detected compiled source mutations**. The GCC coverage build executes all **32 primary function/constructor/lambda entries**, with per-case counts. Nine invalid-access/cycle probes remain **unrun**, as explained below; no successful vector or sanitizer classification is claimed for them.

## Ownership and commits

Only `validation/clocks/` was edited or staged. Existing Rust/JPL work and the newly appearing parameter-reference work were preserved. Both independent repositories, `tempo2/` and `validation/reference/`, were clean at entry and at the final check. No central document, F inventory, older reference artifact or Rust test was changed.

- `488743a` — `docs(clocks): pin Assignment J source profile and state contracts`.
- `d70b075` — `test(clocks): export pinned table routes cache and SAT jump behavior`.
- The final owned report/coverage commit follows these stages and is identified in the delivery message. It includes the measured function matrix, this report and removal of one trailing space in the runner; no numerical implementation changes follow the verified exporter.

## Reproduction and exact build

Run from `/Users/Eric.Dong/Downloads/tempo2rust`:

```sh
python3 validation/clocks/profile.py
python3 validation/clocks/run.py --record
python3 validation/clocks/coverage.py
python3 validation/clocks/run.py --skip-mutations
python3 -m py_compile validation/clocks/*.py
git diff --check -- validation/clocks
git -C tempo2 status --short
git -C validation/reference status --short
```

Those commands were executed. `--record` deliberately creates/replaces only J's checked artifacts; normal `python3 validation/clocks/run.py` checks them and runs all mutation checks. The final `--skip-mutations` invocation reran the complete source/fixture/resource identity gates and two fresh executions of each supported case after the final recording. Mutation checks had already passed against that same exporter and fixture set. Coverage is an additional instrumented run, not the numeric oracle. No Rust test or other assignment's suite was run.

For a single supported case, use e.g. `python3 validation/clocks/run.py --case pks_real`. This executes twice and checks the action schema and replay; only the full command compares with the committed whole-suite oracle. Case files are restored before each fresh run; within each run, ordered resource mutations and global state are retained. Withheld case IDs remain unrun even when selected individually.

The exact compiler argv is in `outputs/manifest.json:native.command`, each mutation's argv in `outputs/checks.json:source_mutations[].build.command`, and coverage argv in `outputs/coverage.json:build.command`. The ordinary command is:

```sh
/opt/homebrew/bin/g++-16 -O2 -std=gnu++11 -DHAVE_CONFIG_H \
  -I/Users/Eric.Dong/Downloads/tempo2rust/validation/clocks/build/source \
  /Users/Eric.Dong/Downloads/tempo2rust/validation/clocks/probe.cc \
  /Users/Eric.Dong/Downloads/tempo2rust/validation/clocks/build/source/observatory.C \
  /Users/Eric.Dong/Downloads/tempo2rust/validation/clocks/build/source/TKlog.C \
  /Users/Eric.Dong/Downloads/tempo2rust/validation/clocks/build/source/TKlongdouble.C \
  /Users/Eric.Dong/Downloads/tempo2rust/validation/clocks/build/source/tempo2Util.C \
  /Users/Eric.Dong/Downloads/tempo2rust/validation/clocks/build/source/global_data.C \
  -lquadmath \
  -Wl,-map,/Users/Eric.Dong/Downloads/tempo2rust/validation/clocks/build/probe.map \
  -o /Users/Eric.Dong/Downloads/tempo2rust/validation/clocks/build/probe
```

Host: macOS arm64, Homebrew GCC 16.2.0, locale `C`. The runner explicitly removes `TEMPO2_ALIAS` from its child environment. `TEMPO2` and `tempo2_clock_path` are controlled by each fixture. The ordinary profile has binary128 `longdouble` (16 bytes, 113 significand bits), binary64 table/correction/epoch arithmetic (53 bits), binary32 badness/distance arithmetic (24 bits), and `MAX_CLK_CORR=30`. There is no fast-math flag. The O0 `--coverage` build retains the same numeric header and records reachability only.

## Executed source and instrumentation identity

Canonical source revision: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. Primary sources and required headers/dependencies are extracted from **raw Git blobs**, not the instrumented A2 checkout. Source preprocessing and the independent accepted archive's defined symbols confirm active `toa2utc`, modern clock routines and `readJBO_bat`; they exclude old `tai2tt`, `utc2tai`, `convertTOA`, `toa2utc_nist` and `linearInterpolate` definitions. Literal-disabled bodies were never reactivated. Active `readJBO_bat` remains later input-format work.

`probe.cc` includes the three raw primary sources so it can inspect initialized private clock state without resetting it. A macro renames only the raw `TabulatedFunction_getValue` symbol to `J_Table_getValue`. A forwarding function records actual binary64 arguments/results and returns the same double; it does not replace the interpolation expression or recompute a clock result. Full epoch/query ordering is therefore observed directly. Other primary function bodies and branches are unchanged.

Dependencies are raw `observatory.C`, `TKlog.C`, `TKlongdouble.C`, `tempo2Util.C`, plus a verbatim concatenation of `global.C:1-64` (declarations/data) and `global.C:176-200` (`CVSdisplayVersion`). This avoids linking unrelated timing wrappers. **No A2 archive object is linked**, no source initializer is called, and no approved initializer correction or A2 resource tracer runs. A2 supplies only its three-line minimal `config.h`; its archive hash/symbols are independent profile evidence. Controlled zero-initialized pulsars/observations and slot sentinels are fixture inputs, never presented as native initialization.

Important SHA-256 identities:

| Item | SHA-256 |
| --- | --- |
| `tabulatedfunction.C` | `0c3e90a2594e442c5135ca224f603a6f69e6278fa9f68382bc9e451bfe7b54c4` |
| `clkcorr.C` | `39d69133304ae2f92e9156f264db338cc103e7762f3f9b144ac0b17fdac0f163` |
| `toa2utc.C` | `9df5cb0d689b296c2d25637bbdab208b2f312842e268d7600bec823287afbb20` |
| `tempo2.h` | `0313b6eaf9f9ae9d7e5cce340379ad4b7068394e5222a01f73949e3a2640de93` |
| Selected `TKlongdouble.h` | `214911b81f82489996d698626cf8d30ab5bcc20099f6b5c5aa089715b85bc55e` |
| A2 minimal `config.h` | `470c61b68a61504558e76ef772639e46b565a1e4500115d6e3fb6a71286dd69e` |
| Extracted `global_data.C` | `4806f49b014b2174a8bb98a6a52064542f5d8d33f3102c3389c2c929b62608f7` |
| Exporter `probe.cc` | `74f47062c2ad2acd93a95e7a6b0910336ee4b5ff09416a139a41987ccf0f62a3` |
| Independently checked A2 archive | `313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5` |

`outputs/manifest.json` records every extracted source/header, all synthetic input bytes, all **75 real resources (72 clock files and three observatory resources)**, compiler binary hash, `libquadmath`, `libstdc++` and `libgcc_s` hashes, recorded executable hash, exact flags and link-map hash. Every real resource was compared with its pinned Git blob before recording and final verification. `outputs/last_execution.json` records the actual final check's executable/source identities; executable/link-map byte identity is not asserted across rebuilds. macOS libSystem is identified by host and dynamic-library ABI version; its dyld-cache image is not hashed as a standalone file.

## Findings and numerical scales

The fixture epochs are MJD, table corrections and SAT jumps are seconds, and badness values are dimensionless route penalties. Synthetic names A/B/C/etc. denote controlled clock nodes, not additional physical time-scale conventions. Input/action order is explicit in `cases.json`; there is no fitting, mean removal, ephemeris, phase reference or residual calculation in this stage.

| Evidence | Observed source behavior |
| --- | --- |
| F's five exact table inputs | Sample counts 2, 2, 3, 1, 2. Skipped row, singleton off-epoch proxy and decreasing proxy return 0. All-equal and duplicate-final interpolation return NaN through defined floating arithmetic. |
| Parsing and line boundaries | Malformed rows are skipped; valid numeric prefixes and trailing tokens can be retained. A 255-byte header including newline fits with its NUL; 1023-byte body chunks are parsed as the source reads them. Reload replaces samples/header/name. A 255-byte basename and a much longer full path succeed. |
| Duplicate internal knot | At x=2 in the four-row repeated-knot case, source interpolation returns 30; the bracket mutation returns 20 and is rejected. |
| Nonfinite and signed zero | NaN/infinite x/y/query cases and signed-zero endpoint proxies retain explicit classes or finite bits. Equal infinite y endpoints yield NaN because the source subtracts infinities. Finite-to-infinite tables can yield signed infinite correction chains, also recorded. No NaN payload is compared. |
| Directed routes | Reverse-only routes return no conversion and report the reverse hint. Sorted file/node order and strict `<` relaxation preserve ties. Parsed badness and accumulation round in binary32; the `2^24` accumulation fixture selects the three-component correction 1+2+4=7. |
| Negative/nonfinite weights | The acyclic negative route returns 3 seconds. Negative infinity can select its edge; NaN, positive infinity and overflowing positive weights fail the strict distance comparison in the measured graphs. These observations do not prove general negative-weight shortest-path correctness. |
| Extra and explicit files | Extra files precede defaults and add one to default badness only if the extra directory is nonempty. A full-path explicit token still resolves by the first matching basename; the duplicate fixture chooses the extra file's 10 seconds, not the default file's 20 seconds. |
| Cache precedence | First matching cached sequence wins. An explicit chain defined after an automatic chain does not replace it. Intersections start at [0,1000000]; routes entirely outside this range can be rebuilt and appended on repeated calls without matching their cached interval. |
| Fallbacks | The stored-chain API tries original→target, UTC→target, original→TT(TAI), UTC→TT(TAI), in that order. Exhaustion returns count zero; `getCorrectionTT` returns +0. `getCorrection` has only the original/UTC-source attempts and does not inherit TT(TAI) fallback. |
| Shifted epochs | The synthetic first correction is 86400 seconds. Actual table queries are MJD 5 then 6; the second correction is 60 seconds and the sum is 86460. Removing the shift produces 50 and is rejected. A shifted query can leave the range validated at the original epoch and use an endpoint proxy. |
| Capacity | 29/30 steps store sums 435/465 seconds. For 31 steps, the first 30 slots are written, count is reset to zero and the stored sum is +0. The independent accumulator still returns 496 seconds and leaves the observation untouched. |
| Same-process resources | Before lazy load, changing the indexed file's header A→B to X→Y keeps the indexed A→B edge but loads new samples: result 77 seconds. After load, file overwrite, replacement, deletion, changed root and changed extra path retain the old 1-second value. Newly added files are absent from the fixed index. Removing a not-yet-loaded file terminates the process. |
| Active wrapper | clockCorr 0/2 bypasses clocks while preserving prior slots/count, then still applies SAT jumps. Other values, including 3 and -1, call clocks. The k-outer/l-inner loops test `jumpSAT[l]` and use `jumpVal[k]`; repeated passes and duplicate memberships retain their source behavior. |
| SAT shift | Starting with count 28, one jump produces count 29 and writes the extra slot 29 with the old slot 28 value 928. Starting at count zero, the extra slot 1 receives the old sentinel 900. Both are within allocated storage and exported, including full destination bytes and changed-byte masks. |
| Diagnostics | noWarnings=2 suppresses warning types but not errors. Duplicate warning keys are suppressed process-wide, with one DUP1 message. Full stdout/stderr order is retained separately, alongside action markers and warning/error counts. |

A changed-byte mask measures bytes that differ from the controlled prior state, not repeated stores of identical bytes. All 30 initialized slots are exported, including bytes outside the returned component count. Every `getCorrection` and `getCorrectionTT` observation snapshot is checked unchanged.

### Real pinned PKS route

At binary128 input MJD `58000.12345678901234567890123456789`, with real observatory lookup, target `TT(TAI)` and clock corrections enabled, the route is:

| File | Destination | Correction (seconds, binary64 round-trip decimal) |
| --- | --- | --- |
| `pks2gps.clk` | UTC(GPS) | `-5.0349515655284256e-07` |
| `gps2gps_usno.clk` | UTC(GPS_USNO) | `0` |
| `gps_usno2utc.clk` | UTC | `-1.2111111101054121e-09` |
| `utc2tai.clk` | TAI | `37` |
| `tai2tt_tai.clk` | TT(TAI) | `32.183999999999997` |

Ordered sum: **`69.183999495293733 s`**. The independent PKS→UTC call returns `-5.04706267662948e-07 s`. Repeated active toa2utc passes reproduce the same five components when no jumps are selected.

The recorded binary64 epoch spacing near MJD 58000 is about **0.629 microseconds**. The particular binary128→binary64 narrowing changes this input by about **+81.5 ns**. The first approximately -0.503 microsecond correction changes the next table's epoch by one binary64 ULP. These are observed source storage/arithmetic scales, not proposed error budgets. Synthetic day-sized corrections intentionally make sequential evaluation visible; they are not an astronomical clock model.

The five real route resource hashes are:

```text
pks2gps.clk       4592e421f977f1d90be0ba0f3bbe7fc7a004c06189226cae4436ff12e6f73198
gps2gps_usno.clk  55e806258932e74f0f0cb3dab24707f3911930abea86269c7f81cf42d65385fb
gps_usno2utc.clk  ab8f0a4955594d906b66cf357aa99fa15867613fe5a65f2483cb065dd7ba4c26
utc2tai.clk       cade8c0b65ad37682e8e265a10e7587454faf9c5a1d77009458daacf940847bf
tai2tt_tai.clk    46161bbb915b3b4fa4e910f327833795df3950410b18e01cd6118648dff8699a
```

## Coverage and comparison integrity

`SOURCE_MAP.md` maps every primary routine, constructor, field/storage contract, private cache, diagnostic dependency and active/disabled disposition. `outputs/coverage.json` verifies actual case membership through gcov rather than comment scanning: four table routines, four public clock routines, 18 helpers, three constructors, two lambdas and active toa2utc all execute. Read-only snapshots add getter calls, so counts are not a workload benchmark. The observed line counters are 62/62 table, 433/444 clock and 19/19 active wrapper lines; line coverage does not imply all branches or invalid cases are tested.

Unexecuted clock locations include glob allocation/read errors (219–226), an invalid-predecessor safeguard (497–498), public handling of a broken cached accumulator chain (866–867), and a literal UTC-versus-TT(TAI) identity return (777). The broken helper itself is executed with controlled arguments. Compiler-associated closing/return locations 133 and 238 have no execution counters. These gaps remain visible in the matrix. Observatory geometry and general logging/BAT input coverage are not claimed merely because some dependencies execute.

The comparator checks recursive types, exact object-field sets, full array lengths/order, case/action IDs, actual query lists, slot counts/destinations/byte masks, finite bits and decimal text, nonfinite classes, return codes and complete diagnostic streams. It rejects missing/extra cases, events and fields before indexed comparison. Absolute workspace/build paths alone are normalized; diagnostic text, line numbers and spacing remain exact for the recorded host. Native compiler/source/config/runtime hashes and full resource identity are checked separately. Default verification never rewrites the ordinary/isolated goldens.

The 19 artifact mutations cover missing/extra/reordered cases and events, fields, component count/destination order, binary64 bits, nonfinite class, binary32 weights, shifted epoch, loaded state, write mask, exit status/last stage and an unrun→passed misclassification.

The 12 compiled disposable-source mutations change numeric-prefix parsing, duplicate brackets, edge direction, tie relaxation, exhausted-fallback count, shifted epoch, loaded-table cache, index lifetime, SAT jump index, SAT extra shifted slot, capacity and exit status. All are detected. Mutation comparison excludes executable path and stdout/stderr source paths, so a filename or warning-line change alone cannot pass the sensitivity check. Mutants run only defined ordinary/normal-exit fixtures; they never execute the withheld invalid-access/cycle probes. Canonical source and checked goldens remain unchanged.

## Termination evidence, withheld probes and remaining limits

All 20 normal source-exit fixtures terminate with code 1 at their declared action, and replay with identical last stage/stdout/stderr. They cover missing/empty/bad-header/no-sample tables; missing indexed files; absent default root files even with extra files; unset/empty environment; invalid clock headers; failed lazy load; empty/null/missing/broken explicit chains; and deletion before lazy load. Their later scripted actions are not executed or presented as recovery. No numerical output from the terminating action is fabricated.

The central chat reported that an earlier turn was flagged for a possible cybersecurity risk, without naming the triggering experiment, and directed continuation of permitted work without bypassing safeguards. In response, the following nine cases were left **unrun**, with explicit reasons in `outputs/isolated.json`:

- `table_singleton_exact`, `table_singleton_nan`: index-before-beginning at the exact/special query.
- `table_empty_value`, `table_empty_start`, `table_empty_end`: empty sample storage accesses.
- `table_header_256`, `table_header_800`: header buffer overflow boundaries.
- `toa_capacity_29`: incremented count 30 causes the source to write slot 30 outside the 30-slot array.
- `weights_cycle`: bounded predecessor-cycle/nontermination experiment.

No sanitizer result, observed signal, timeout result or stable diagnostic class is asserted for these cases. No alternative tool, compiler configuration or execution path was used to retry them. This is a real open coverage gap, not an accepted safe-adapter disposition. The harness's initial compile/protocol mistakes were fixed before recording; no failed harness execution is classified as a canonical source result.

A basename longer than 255 bytes cannot be created on this host; the maximal supported basename and a long full path are measured, but a larger-name invalid-copy experiment is unavailable. Allocation exhaustion, glob filesystem failures and arbitrary malformed graphs are not exhaustively tested. Outputs reflect this macOS/GCC profile, not Linux or another C library. Dependency initialization beyond selected observatory/logging roles, active BAT parsing, EOP B11, weather/IFTE/TT-to-TDB state, full timing, fitting and full-core fidelity remain outside J.

## Central-review decisions still required

This run strengthens F ledger B07–B10, B12, B13 and the clock portion of B15 without modifying or closing that ledger. B11 remains unmeasured. Deterministic fallbacks, permissive tables, directed/tie selection, cache lifetimes and the SAT indexing/extra-slot behavior are explicit future compatibility targets under decision 0008.

Central still owns the later Rust contract: query-specific dispositions for undefined table/jump operations, safe adapters for process exits, source-global versus owned-context behavior, warning/public-clock controls and integration scope. The nine withheld probes require a separate approved resolution; J does not infer or fill in their results. No numerical budget, milestone gate, M1/M2 criterion or unresolved SVD mode-order decision changes here.

## Artifact navigation and integrity

- `cases.json`, `fixtures.py`: complete declarative inputs/actions, expected termination locations and explicit unrun membership.
- `probe.cc`, `run.py`: canonical-source exporter, resource setup, exact regeneration and comparison.
- `verify.py`, `outputs/checks.json`: behavior assertions and artifact/compiled-source mutations.
- `coverage.py`, `outputs/coverage.json`: measured function/case/dependency matrix and unexecuted locations.
- `profile.py`, `profile.json`, `outputs/manifest.json`, `outputs/last_execution.json`: raw source/configuration, active-profile, compiler/runtime/resource and actual execution identities.
- `outputs/reference.json`: complete ordered ordinary/terminating records; `outputs/isolated.json`: explicit unrun records.

Recorded artifact SHA-256:

```text
reference.json  9f189fd0765acaa9ee3eda0299993eb7d3fb1d13bf1f103aa688fa1ea165ccaa
isolated.json   742718f00e94aa576d3837283c80647e0bbd4aa8e33e70de0708e3d959d789a9
manifest.json   551df45df59ba8837c307d1bebc15a0d69e786dbdefed0bc655ae66f8ec31eb8
checks.json     d46eda1ba21830276b9b4bd7157fea8eca9659174cfe110921363b85642147ed
coverage.json   4521f0e7921019eb1f16f600779fe559ce1c1cc3dd808f69053a243b434a8672
```
