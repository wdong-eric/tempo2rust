# Assignment U: terrestrial resource reference

Delivered for central review on 24 October 2026, from release `43001af` under
0046. **Native evidence is complete for the declared bounded catalogue.** The
user required the preservation gate to remain blocked until
`validation/observation_storage_port/run.py` was restored. Its original bytes
have now been restored by the owning chat; U has neither changed that file nor
excluded it from membership. All four final verification commands now pass
their exact before/after preservation checks.
No Rust implementation, tolerance, scientific correction or milestone acceptance
is claimed.

## Results and preservation

The suite records **130 fresh-process cases: 106 ordinary completions and 24
source exits**, with **1,128 ordered actions / 2,256 events** (1,104 completed
actions and 24 observed exit prefixes). Explicit recording and subsequent normal
verification each replay every case in another fresh process. All ordered state,
initialized bytes, finite bits/nonfinite classes, streams and statuses agree.

All **29 controls** pass (three positive and 26 rejected corruptions), as do
**13 defined compiled mutations with native state/numerical witnesses**.
The separate coverage profile enters **22/22 routines**, with **336/341 lines and
217/252 compiler branch outcomes**; all 130 results agree exactly with ordinary
execution. The exit-observer audit matches all 130 executions after removing only
the added exit-state event and verifies six initialized failure prefixes.

The final ordinary replay, controls, coverage and observer-audit commands each
return **exit 0**, with `status: PASS`, `native_status: PASS` and empty
`preservation_before` / `preservation_after` differences.

Before restoration, the four overall commands returned **exit 1**, deliberately,
with `status: PRESERVATION_DRIFT` and `native_status: PASS`. Their only mismatched
member was the concurrent one-line output-root override in the accepted
observation verifier:

| File | SHA-256 |
| --- | --- |
| Frozen validation/observation_storage_port/run.py | `7d6424654a666d3852255bd9264ba0fbb88ba3c6e57f7bf717da4087a6b10529` |
| Prior routed version | `5f0cb7bfa0c2ebc8606c789e157ab96b7d5fa248d54703e01f9c5c7495e0c4d6` |
| Restored live version | `7d6424654a666d3852255bd9264ba0fbb88ba3c6e57f7bf717da4087a6b10529` |

The prior diff added `TEMPO2_OBSERVATION_VERIFICATION_ROOT` selection. All 1,274
original frozen files now match, and both independent repositories remain clean
at their original pins. Restoration was verified from actual bytes with U's
unchanged checker. No baseline, oracle or protected membership was rewritten.

Central's proposed exception in decision 0048 was never applied by U. The user's
direct instruction **"Keep gate blocked until restored"** instead governs
[decision 0050](../../decisions/0050-restore-accepted-observation-verifier.md),
committed as `66e602c`, which supersedes that exception. The original failed
receipts, logs, raw outputs and executed profiles/binaries are retained under
`outputs/preservation_drift_before_restore/`; its 2,133-file inventory is
`snapshot_sha256.json`, SHA-256
`93d545f4d8dd5c90275dc82618cc33359bf40a55975a9f5641937b4dddf5380b`.
Those historical failures remain failures. Subsequent successful verification
uses the original exact membership check after actual restoration.

## Reproduction

From `/Users/Eric.Dong/Downloads/tempo2rust`:

```sh
python3 validation/terrestrial_resources/run.py
python3 validation/terrestrial_resources/controls.py
python3 validation/terrestrial_resources/coverage.py
python3 validation/terrestrial_resources/observer_audit.py
python3 -m py_compile validation/terrestrial_resources/*.py
git diff --check -- validation/terrestrial_resources
```

All were run successfully after restoration. Python syntax and whitespace
checks also pass. Full command statuses and raw logs are local in
`outputs/commands.json` and `outputs/*.log`.
No Rust regression suite was run by U.

For an environment missing this local evidence, `inventory.py` creates a one-shot
source/protection freeze, then `run.py --record` explicitly creates the native
oracle. The existing frozen membership must be preserved for this delivery's
audit; a fresh snapshot of a different workspace is not the original preservation
baseline. Recording refuses an existing oracle. Ordinary verification always
freshly builds, verifies the frozen case/harness/contract membership and actual
launch identities, and never writes the oracle. Preliminary development runs and
their executed sources are retained under `outputs/pilot*` and
`outputs/prerelease_record/`; they are not the delivery oracle.

Prerequisites are the pinned TEMPO2 Git repository, accepted A2 archive/config,
accepted S type inventory, Python 3.11, Homebrew GCC/gcov 16 and macOS command-line
tools. No network, new dependency installation or A2 object linkage is needed.

## Actual execution profile

Raw TEMPO2 pin: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
Independent A2 pin: `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`.
Host: macOS 26.7 build 25G229, arm64, C locale; GCC 16.2.0.
Measured `longdouble` is 16-byte binary128 with 113 significand bits;
resource/geometry/mapping arithmetic is binary64 with 53 bits.

The ordinary flags are `-O2 -std=gnu++11 -DHAVE_CONFIG_H -I<raw-source>`.
Compile dependency files use `-MD -MF`; linkage uses `-lquadmath` and a saved link
map. Coverage adds `--coverage -fno-inline` and never supplies ordinary values.
Full argv, 334 actual file identities, macro digest, compiler/backend/assembler/
linker/Python identities and linked-library versions are in the manifest and
actual execution profiles. System headers are hashed from the compiler dependency
closure. libSystem is qualified by the measured host and dyld ABI, not presented
as a separately hashed dyld-cache image.

The caller includes unchanged observatory.C/eop.C/tropo.C, tabulatedfunction.C and
clkcorr.C. Separate unchanged dependencies are TKlog.C, TKlongdouble.C and
tempo2Util.C. Only global.C:1–64 and :176–200 are copied verbatim for global data
and CVSdisplayVersion. The checked A2 archive is context only; no archive object,
corrected initializer, fake clock/lookup, native reset hook or source repair runs.

| Item | SHA-256 |
| --- | --- |
| Actual ordinary executable | `c3aa6025e19246687b37be834d520e08efa2da638057527b35c49e2ff53e24f4` |
| observatory.C | `fabfaabc78b75652590c5a81c1c2bb05f806550f99dbd7dd3f1ab43d056bf2c3` |
| eop.C | `ff90736cb57c96ce06411e4aa5dbb692b5c5ec0f4a9069fd9403eb7bb9d469a7` |
| tropo.C | `871d06bb10e647df0d563905db20b560cd659fcb4b372fed0c790937835a7fe6` |
| tempo2.h | `0313b6eaf9f9ae9d7e5cce340379ad4b7068394e5222a01f73949e3a2640de93` |
| Selected TKlongdouble.h | `214911b81f82489996d698626cf8d30ab5bcc20099f6b5c5aa089715b85bc55e` |
| config.h | `470c61b68a61504558e76ef772639e46b565a1e4500115d6e3fb6a71286dd69e` |
| global_data.C | `4806f49b014b2174a8bb98a6a52064542f5d8d33f3102c3389c2c929b62608f7` |
| Context-only A2 archive | `313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5` |

Every execution checks executable, compiler/runtime/header/source, generated
field checker, runner and frozen input identities before and after launch. Actual
resource bytes and encoded action input are checked before launch; resource
before/after hashes record deliberate same-process file changes. Rebuilt binary
and link-map hashes are recorded individually; their byte identity across linker
runs is not assumed. No stale `--no-build` route exists.

## Maps, observations and bounded findings

[CONTRACT.md](CONTRACT.md) is the pre-experiment source/type/state/ownership and
safety map. The immutable local `outputs/frozen/source_map.json` contains all
22 routine bodies with source ranges and decision lines, preprocessor branches,
all raw repository caller/declaration occurrences (including plugins), and the
349-pulsar/95-observation-field type inventory. `inventory.py` rechecks it against
raw Git blobs. [GAPS.md](GAPS.md) classifies every unhit coverage outcome, including
optimized-counter attribution that remains unresolved.

| Area | Measured evidence and preserved behavior |
| --- | --- |
| Observatory parser and ownership | Main-before-glob ordering; five/six-field rows and numeric prefixes; bounded scanner tokens/chunks; 15-byte default-clock truncation; comments/ignored rows; complete-row then partial-row failure prefixes; geodetic diagnostic before count rejection; direct reads, lazy lookup, explicit repeat initialization, root/file changes and both environment selectors. Borrowed pointers are reacquired after append. |
| Aliases | Accumulating per-call alias vectors, append across calls, case-insensitive code/name/alias lookup, first-match ordering, code-only/comment lines, once-per-list warnings and in-place/separate output buffers. Known retained string tails are exported; unwritten suffixes are excluded explicitly. |
| Geometry | Every fang mode branch, signs/component boundaries, north/south/equator/poles, both longitude branches, negative/zero/large heights and defined IEEE outcomes. Round-trip values support inspection; they establish no new tolerance or geometric algorithm. |
| EOP | Default/old/both modern formats and transitions, year bounds, numeric-prefix scans, arcsecond conversion, append/order/duplicate/nonfinite load-only rows, defined parse exits, below-first/first/interior knots, nonuniform spacing, positive/negative/no leaps, strict ±0.5 thresholds, output alias assignment order, filename/file/root changes after private first use. Derivative remains divided by 86,400 rather than sample spacing. |
| Meteorology | Four direct wrappers, retained seeded byte tails, header/site/basename limits, accepted/skipped/duplicate/decreasing rows, safe endpoints and singleton off-epoch proxies, glob ordering, per-kind clear/load, explicit/lazy histories, partial initialization exit, first matching inclusive-range table, case-insensitive sites, absent/out-of-range/measured-zero behavior, TROP1/TROP2/DUP1 histories and warning/quiet suppression. Pressure zero becomes 101.325; wet zero remains zero. |
| Niell mappings | Every latitude interval/clamp, north/south season, source epoch/year, elevation/height cases, signed zero and defined nonfinite results. The wet c coefficient retains its mixed b/c expression. |
| Integrated delay | Two pulsars with six observations each, repeated calls, signed counts and correction flags, unconditional reset, zero-zenith warning order, exact STL_FBAT and COE bypasses, alias lookup, real clock-to-UTC conversion, SAT weather versus shifted seasonal date, table-backed/fallback values, negative/nonfinite outputs. All other typed pulsar/observation fields are checked unchanged without reading padding. |

The real PKS smoke copies pinned observatory/alias/clock resources into an isolated
case root and uses bounded authored weather tables. It is a resource integration
smoke, not full pulsar timing or validation of actual weather measurements.
Snapshots never query mutating helpers. A separate helper history reproduces the
integrated delay exactly; a non-equatorial clock-season fixture avoids the zero
seasonal amplitudes of the equatorial synthetic site.

All primary source bodies remain unchanged. The caller's `atexit` callback only
observes live initialized global and caller-owned storage; native local scanner
temporaries remain source-inspected. The callback-off audit retains all source
streams/statuses/previous events exactly. Finite doubles compare bitwise,
binary128 input representations are complete, nonfinite values retain class, and
ordered fields/events cannot be omitted or permuted. Projection replaces only
declared exact source/case/workspace root prefixes; no numeric, line, warning,
general address or truncation scrubbing occurs. Raw streams remain available.

The 13 source mutations witness alias accumulation, observatory append, EOP
cache/leap thresholds/derivative spacing, meteorology precedence/cache/zero
fallback, mixed wet coefficient, warning-dependent delay order, clock-season
input and unconditional reset. Each saved mutant includes its actual source,
executable, profile, input/resource identities and changed native fields.
Preliminary ineffective EOP-append and equatorial-season mutations are retained
as pilot evidence; they are not counted among successful behavioral controls.

## Local evidence and limits

All paths below are relative to this directory's ignored `outputs/`. The source
freeze and oracle are preserved; normal verification did not rewrite either.

| Local artifact | SHA-256 |
| --- | --- |
| frozen/source_map.json | `f303e2b0df9b8070d1b8cca1591c8558d25f7ae0982e860e70610821d56c344a` |
| frozen/protected.json | `e483e13c44edbd047b151c0a02c13d220fde684e2052fb91458b3d3f67e1a9da` |
| frozen/cases.json | `cca1633b08f302075135751d1e49742dfa8f61159acf49f7792802fdd77c5020` |
| reference.json.gz | `d955d6b9ee4bf8e9259decff4b5c16cf25e97003034983def21c1895d00d94a2` |
| manifest.json | `37f5d106e77d7f201988fca10d267317e36c7460577ebea9ec3089e9e07ac2a6` |
| last_execution.json | `aad020dea01b6459133ee1d55faccffa061687bca312afafa14962ee7dc04b84` |
| execution_receipts.json.gz | `3da4e670dfd2462058fc5a143457676e6385dfa1f2a5ae09e464d67f3809481d` |
| controls.json | `59260d22096904c6904939bdf8ff6e67ad7d298c18f77d2dc1e29b09c7c1d8e1` |
| mutations.json.gz | `8b4a7dc9f1a51082e49a3f3ec5bce86da0abe853856c6065e17e3896246ba323` |
| coverage.json | `69b60d28f933de0255c86fb7393488faf14d2a3ee980858196856fa11d48dc47` |
| observer_audit.json | `b2c503327b79725f90ca8ecd75ea049cf69c211e1beab8ecf64ca54a1830bdde` |

Raw stdout/stderr and losslessly compressed event streams live in `record_raw/`,
`check_raw/`, `replay_raw/`, `mutations/`, `coverage_raw/` and
`observer_audit_raw/`. Their individual hashes are in the corresponding execution
receipts. Actual compiler inputs, executable files, link maps, dependency lists,
macros and gcov output are under ignored `build/ordinary`, `build/mutant00` through
`build/mutant12`, `build/coverage` and `build/observer`.

Known native-invalid paths remain unrun: final-sample EOP access, malformed private
brackets, unwritten partial-row reads, overflowing strings, invalid float-to-int
conversions, null glob iteration, invalid table backing and stale pointers.
TKlog's existing before-buffer-pointer limitation remains host-qualified rather
than portable semantics. Allocation/read failures, other platforms/locales,
concurrency, full physical timing, IFTE/TT-to-TDB and other propagation models are
outside this evidence. Coverage does not close those gaps or the M2 exact-order
gate. Central determines whether the bounded native evidence and its declared
gaps meet the wider release criteria.

## Ownership and commits

Only `validation/terrestrial_resources/` is authored or staged by U. T's evolving
Rust/lifecycle-port work, V's time-ephemeris directory and central documents remain
outside U's immutable membership. Neither independent repository is modified.
Policy 0039 is enforced by the local ignore file: no generated JSON/JSONL,
compressed evidence, raw archive, profile, coverage, receipt or large output is
committed or force-added.

The pre-experiment contract stage is `99c0771`; the native harness stage is
`2cd8c91`. The report commit ID is returned with this delivery; central owns
acceptance.
