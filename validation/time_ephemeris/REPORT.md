# Assignment V: native time ephemeris reference

Delivered for central review from release `8e56df0`, under decisions 0008,
0012, 0039, 0047 and the restoration decision 0050 (superseding 0048). V owns
only `validation/time_ephemeris/`. Central owns acceptance and the later Rust
contract. No Rust, raw TEMPO2, accepted oracle, clock/numeric implementation,
physical convention or tolerance was changed by V.

## Result and scope

The ordinary reference contains **97 cases and 1,368 ordered events**, including
**nine expected native exits with status 1**. All other cases return zero.
There are 75 cases classified as defined behavior and 22 with explicit host
qualifications (13 byte-helper, two stdio, three union/EOF, two native-record
and two diagnostic cases). An expected exit is recorded separately from the
case's portability classification. No native-invalid fixture was executed.

Thirty normal verifier controls, 36 independent polynomial/endian checks and
thirteen compiled mutations establish comparison sensitivity. Every mutation
has an actual state, numerical result or helper-result difference; executable
hashes alone are not witnesses. Separate O0/gcov results and the final
preservation/replay identities are recorded below.

The complete **29-routine source/branch/caller/state/lifetime/format map** is
in [SOURCE_MAP.md](SOURCE_MAP.md), committed initially as `0a86b42` before native
experiments. Its later negative-step/backing extension was analyzed and added
before those cases ran. `inventory.py` binds every selected definition and
source condition to its raw source hash. The final corrected oracle contains that
inventory and exact ordered case/action/input membership.

| Map or reproducible component | Responsibility |
|---|---|
| SOURCE_MAP.md | All 17 IFTE, four conversion and eight Fortran routines; branch lines, callers, constants, byte layouts, fields, ownership, unsafe and unverified paths |
| inventory.py | Raw-definition/condition inventory, never inferred from coverage counters |
| profile.py | Raw Git blobs, actual headers/configuration, compiler/backend/runtime, preprocessed units, link map and executed binary identities |
| probe.cc | Explicit initialized caller and exact ordered field/state/result export; raw IFTE and tt2tdb inclusions for read-only IFTE static access |
| fixtures.py | Authored cases, binary generators, independent layout decoding and bounded real-resource inputs |
| run.py | Fresh build/identity checks, per-launch challenge, exact replay and explicit first recording only |
| verify.py | Positive/negative controls, simple analytic checks and thirteen defined compiled mutations |
| coverage.py | Separate O0/gcov execution, per-case calls, raw compiler edges, every unhit disposition and exact O0/O2 differences |
| preservation.py | Exact release-anchored preservation, decision-0050 restoration, rejecting controls and final ordinary replay wrapper |

## Source profile and dependencies

Raw TEMPO2 revision is `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
The ordinary source hashes are:

| Raw source | SHA-256 |
|---|---|
| ifteph.C | `ddf1f64bbff6ce3b10cefc8c7aefdf9fe42f38936b89f90359b9c6da04ed40d4` |
| ifteph.h | `ed3a13ca76b0fa395a19b38fc86f99ba3da68204268385f8eafc5aa52b0a3a70` |
| tt2tdb.C | `c62c6c599f8101703ed511a80653bc53d09ff7a4af89ddd98b8b4d936a222426` |
| read_fortran.h | `b20d74d5ed9b832267bd07ea633081c3c5c780c65fe6d9cc37605677dca48e46` |

The actual profile is Homebrew GCC **16.2.0**, macOS **26.7 arm64**, C locale,
`-O2 -std=gnu++11 -DHAVE_CONFIG_H`, with the raw binary128 header. The caller
measures a **113-bit significand** by arithmetic; it also checks 32-bit int,
32/64-bit IEEE float/double, signed char and the host's all-ones negative-int
representation. The native C long double has 64 storage bits/53 significant
bits; selected TEMPO2 longdouble has 128 storage bits. Scalar exports retain
binary32/64/128 object bytes, signed zero and classifications where applicable.
No structure padding or raw pointer address is serialized.

Full raw `TKlongdouble.C`, `tempo2Util.C`, `TKlog.C`, `clkcorr.C`,
`tabulatedfunction.C` and `observatory.C` are compiled. The real
`getCorrectionTT`, `dotproduct`, `vectorscale` and `fortran_mod` execute.
Only the global.C data prefix before `extra_delays` and `CVSdisplayVersion`
through EOF are sliced verbatim, with byte boundaries and digests recorded.
The A2 three-line config has SHA-256
`470c61b68a61504558e76ef772639e46b565a1e4500115d6e3fb6a71286dd69e`.
The corrected A2 initializer and reference archive are never linked or executed.

Git's `ident` attribute expands `$Id$` in clean checkout files. For example,
checkout tt2tdb.C hashes to
`03ddc98c028a8c1f5ea24f3a0c2dea5b198ee6a733c6a3611aa0223713331009`,
while its raw blob has the decision-0047 hash above. The build uses raw blobs;
the profile separately records and freshly checks both identities. The initial
identity gate exposed this distinction before a native case was launched.

Every build receipt includes actual compiler/backend and GCC dylib paths and
hashes, every discovered header dependency, selected/generated source bytes,
preprocessed units, flags, link dependencies/map, runner/caller identities and
executable identity. macOS frameworks/dyld are bounded by the recorded OS;
this is not a hermetic cross-platform runtime claim.

## Behavioral evidence

| Required behavior | Minimal history / observation |
|---|---|
| IFTE record omission | `ifte_big`, `ifte_little`: successful reads leave `irec=-1`. The cache-update mutant changes the observed state to 2. |
| Defined negative-step cache hit | `ifte_negative_step_cold` and `ifte_negative_step_retained`: complete header step=-4, query start+12 gives irec=-1, skips reading and uses initialized zero/retained buf. No negative seek occurs. Cold derivatives expose signed zero. This malformed-header behavior has no physical ephemeris interpretation. |
| Reinitialization retention | `ifte_reinit`: after nonzero normalized time, save old owner, initialize another file, then query tc=0. Retained twot/cells survive; observed delay is `0x1.cp-25`. Clearing the cache changes actual state and subsequent values. Old FILE remains live until explicit caller cleanup. |
| Endian and format fidelity | Both orders produce exactly equal arithmetic/cache/output arrays. Counts 0,1,2,3,4,7,18, multiple subintervals/records, floor(ephver), constant/header state, reverse sums and retained tails are explicit. title remains static zero despite a nonzero file title. |
| Epoch and interpolation boundaries | Equivalent split epochs, negative fractions/carries, start/end, end plus bounded fraction, record/subinterval transitions, backed dates beyond end, direct bounded extrapolation, component/derivative counts and zero/negative interval arithmetic are recorded. |
| Wrapper assignment order | Scalar exact alias and vector exact/shifted overlap histories preserve the interleaved writes. A two-pass wrapper mutant changes an actual overlapping output cell. |
| File history | `ifte_replace` keeps the old open inode until reinit; `ifte_truncate` retains host stdio buffering and eventually exits after reading 50 of 66 coefficients. Raw streams and live ftell/eof/error remain available. |
| Fortran helpers | All eight run with initialized storage; filename first-space truncation retains its suffix; count no-ops retain tails. Forced swap controls cover both primitive branches; natural host detection remains little-endian only. read_record_int is native order (big-endian test bytes return 67305985, not 16909060). EOF byte-reader observations are host qualified; partial read_record_int is unrun. |
| FB cache across files | `fb_cache`: A record 2 returns `-0x1p-9`; B at the same record still returns `-0x1p-9`. A fresh-read mutant returns `-0x1.cp-7`. Switching record then returning to A retains B's record values. nr==1 cold and warm no-read histories, changing count/tails, half-day boundaries and counts -1/0..16 are covered. |
| tt2tb bypass and resource lifecycle | Bypass sets TB=0 while Teph=210 and rate=310 remain in the first slot. Exact STL overrides the bypass; lowercase and longer spellings do not. Initialization is required even for zero/negative pulsar counts; repeated successful calls reopen and close IFTE. |
| Selection, units and clock corrections | IF99/FB90/unmatched integer selectors, SI/TDB/other units, dilation 1 versus 0/-1/2, real clock-chain accumulation, zero/negative observation/count histories and retained tails are exercised. Unmatched selectors retain deltaT across pulsars within one call. |
| Pulsar-zero CALCEPH-value read | Distinct initialized caller values expose the read from psr[0] while processing psr[1]. The mixed FB90/CALCEPH history uses p0.nobs=1 and p1.nobs=4 with four valid p0 backing slots. These are supplied values, not a CALCEPH library implementation. |
| Second obsTerm scaling | Source inspection establishes that the second assignment targets obsTerm, whose later value is unused. Retargeting it to obsTermDot changes the measured rate from `0x1.0000012c45849p+0` to `0x1.0000012c45847p+0`. Ordinary output does not observe the dead local store itself. |

The private function-static FB `tdbnrl` and `buf[16]` are not accessible through
unchanged C++ source. No fabricated accessor or reset hook is used. Their
initialization, lifetime and complete write ranges are mapped by inspection;
ordered file/count histories and exact results expose their consequences.
The directly accessible IFTE global is fully exported, including all 322 buf
and both 18-cell interpolation arrays. Caller-defined ownership markers are
separate from source values. Local FILE leaks on terminal initialization
errors are source-inspected prefixes; no dead/local FILE is dereferenced.

Independent checks evaluate closed-form constant/linear/quadratic polynomials
and derivatives at exact dyadic points, plus simple FB quadratics and exact
IFTE endian agreement. They support the format/arithmetic interpretation and
establish no Rust tolerance. Real IF99/FB90 smoke cases use the pinned resource
bytes at MJD 58000 and nearby dates, with integrated mixed-unit tt2tb calls.
They establish no broader planetary ephemeris, BAT, residual or live CALCEPH
coverage.

## Reproduction and immutable evidence

Run from `/Users/Eric.Dong/Downloads/tempo2rust`:

```sh
python3 validation/time_ephemeris/run.py
python3 validation/time_ephemeris/verify.py --mutations
python3 validation/time_ephemeris/coverage.py
python3 validation/time_ephemeris/preservation.py
```

The final wrapper checks preservation, runs fresh ordinary controls and then a
complete fresh ordinary replay, and checks preservation again. Its corrected checker is
separately pinned by `preservation-checker.sha256`; the earlier checker hash
remains untouched in `outputs/preservation-frozen.json`. It does not alter
the original scientific oracle or the original runner identity contract.

`run.py` always compiles fresh. `--no-build` is rejected and stale success
receipts are removed first. Every case receives a fresh challenge; actual
input bytes, resources, command, controlled environment, executable, raw events,
stdout/stderr, ordered snapshots and status are recorded. Sources, headers,
compiler/backend, runtimes and runners are checked before and after launch.
Exact comparison retains case/action/field order, initialized values/tails,
knownness, owners, file history, streams and exit classes. No diagnostic path,
address or numerical normalization is used.

On a fresh evidence installation, `run.py --record` creates an oracle explicitly
and refuses to overwrite an existing one. The preserved first 93-case oracle
is `outputs/reference-initial/`; the initial 97-case oracle is `outputs/reference/`. Both are preserved.
The authoritative corrected 97-case oracle is `outputs/reference-corrected/`.
The initial no-op cache-control defect was corrected, the four bounds-analyzed
cases added and command/input provenance expanded before the final explicit
recording. A later measured coverage comparison exposed a second harness defect:
case-insensitive macOS filenames collapsed the STL and stl case IDs. Distinct
artifact names, a case-folded uniqueness guard and a post-save identity/hash
check correct this without changing source calculations. The corrected oracle
was explicitly recorded in a new directory; neither previous oracle was edited
or moved after the preservation freeze. Those earlier directories are historical
defective recordings, not complete authoritative case evidence. Ordinary replay
never generates or updates its oracle. Preservation
freezing is likewise explicit (`preservation.py --freeze`) and cannot overwrite
an existing manifest.

## Exact preservation after decision 0050

V's original oracle did not include R's runner; it has no earlier failed R
preservation receipt to reinterpret. Following 0048, a separate manifest was
added without changing the original oracles. It protects **1,466 files**,
including **633 tracked members anchored to release 8e56df0**, existing local
accepted evidence, both historical V oracles and the reference config/archive.
Pre-existing generated files were frozen at that explicit preservation check;
this is not retrospective hash certification of their initial turn state.
T/U working directories, evolving Rust and central documents are excluded.

Decision **0050**, commit `66e602c`, then superseded 0048 because the direct
human instruction was “Keep gate blocked until restored.” The provisional
exception was removed. The original manifest, its hash and previous checker
bytes remain intact; `outputs/preservation-checker-0048.py` preserves that
checker. The corrected checker has an authored SHA-256 pin and requires exact
membership and hashes. No exception, baseline replacement or membership removal
is used to pass preservation.

Actual R restoration was independently read from disk:
`validation/observation_storage_port/run.py` again has SHA-256
`7d6424654a666d3852255bd9264ba0fbb88ba3c6e57f7bf717da4087a6b10529`,
matching the original release-anchored manifest. V did not edit or restore R.
The three controls reject further runner drift, unrelated protected source
drift and deletion of R's protected membership. Both independent repositories
must retain their pinned HEADs and clean statuses. The final exact result is
reported below. The separate corrected scientific recording addresses the
case-filename collision; it is unrelated to the restoration and does not alter
the frozen preservation baseline.

## Limits

All native-invalid paths in SOURCE_MAP.md remain unrun: unchecked incomplete
IFTE headers/constants, null/released streams, double close, overflowing path,
record or pointer arithmetic, invalid numeric-to-int conversions, excessive
coefficient/cache/output extents, incomplete record-int reads, and FB's
missing-open wrapper continuation. A process returning would not validate them.

The regular-file suite does not induce IFTE fseek failure. Other platforms,
locales, concurrency, resource exhaustion and unconstrained malformed inputs
remain outside the measured evidence. The frozen caller uses initialized
synthetic state, not native initialiseOne. Dependencies compiled but unentered
are not claimed as newly validated. The existing Rust contract and the unresolved
M2 exact-order gate are unchanged.

Generated JSON/JSONL, compressed raw dumps, binary fixtures, profiles, coverage,
inventories and receipts are local and ignored under 0039. Generated COVERAGE.md
is also ignored. Only authored source/generators/contracts and this concise
review report are committed. No output is force-added.

## Final measured coverage and profile differences

All **29 selected routines** were entered: **327/330 executable lines** and
**196/268 compiler edges**. Every one of the 75 unhit line/edge entries has
a disposition in local `COVERAGE.md` and `outputs/coverage.json`: 62 compiler
exception edges were not forced; six debug-logger edges remain host-qualified
and unentered; natural big-endian detection accounts for one line/one edge;
the regular-file fseek-failure path accounts for two lines/three edges.
All twenty IFTEinterp edges were hit. The negative-step fixtures cover the
cache-skip arm without a native reset hook. Literal-disabled branches and
native-invalid paths retain their separate source-inspection classifications.

The corrected O0/O2 comparison has **30 differing cases / 576 differing leaf
representations**, all confined to numeric `.hex`, `.bits` or `.value` fields.
They affect interpolation caches, FB results and TT correction values. Case/
action/field membership, owners, resources, status and raw streams agree.
The raw numerical differences are exposed for central disposition; they are
not normalized away and are not a Rust tolerance. O2 remains the ordinary
reference profile. The earlier case-collision comparison is retained locally
as `outputs/coverage-before-casefix.json`.

## Current executed identities and local artifacts

Authored harness/contract/preservation revision: **`573d667`**. Initial map
revision: **`0a86b42`**. The final command sequence completed successfully:

```sh
python3 validation/time_ephemeris/verify.py --mutations
cp validation/time_ephemeris/outputs/verification.json validation/time_ephemeris/outputs/verification-mutations-final.json
python3 validation/time_ephemeris/coverage.py
python3 validation/time_ephemeris/preservation.py
```

The wrapper's last ordinary run matched **97 cases / 1,368 events**. Its fresh
controls passed **30 rejection/positive controls and 36 independent checks**.
The separately retained mutation receipt records all **13 behavioral witnesses**.
Exact preservation passed before and after that run for **1,466 files**, with
**zero exceptions** and **three rejecting controls**. Both independent
repositories remain clean at their pinned revisions.

| Executed profile | Actual executable SHA-256 |
|---|---|
| O2 `build/probe` | `e789033c6284b2eea4844d46b6cdf74f22cc4a462b29890dc3b08d8dd0d15494` |
| O0/gcov `build/coverage` | `098ae30650f302698204cbf1d80d4faba53b55c41735c8df8290076515a83417` |
| ifte_record_cache | `ec77649dd7fd8a002ccc55c32baf9b5c593d2198c4c90b4d841e60106e404f90` |
| ifte_reinit_clear | `118efc550da2c5244109c7df8c40585b7a5031ffcd3a559a11e9edb683596d6b` |
| ifte_subinterval | `804d58e53f2df5408a284a07ae12d5ac9acf06be17b73a5fdf9c90e1bec6ae46` |
| ifte_derivative_scale | `5ecde21cf732524394047eeb5a13db8a235a1123711b1f293a1fa5067a3ec5fa` |
| fb_halfday | `4cd8c31d957ec581ac195eaa1ee87af5baec4fcc067339b36239602effb1d315` |
| fb_file_cache | `4ea43c1aafc7e8e46ff95e1cd879639a6e596774e13c31b63acab44b9e16b0f5` |
| fb_nr1 | `b348a9e6a64831d369c38a6b919a1f5f9907aaaccac55be8b0d1a520ebde5d64` |
| bypass_retention | `b47559b18153c0f3d3c88138a76f10408a722ab890a0eb9b4cd66117b2435c72` |
| calceph_pulsar | `dd50a9e397c917afb7f304ede1a3d018dd70230be6081b0a093c6943d0a48c9e` |
| obstermdot_scaling | `db2424df476e8d54e1c67f794e9810e6de93b5c7c13bf1ee7d8cb4369d5e81d4` |
| deltaT_retention | `886fdf3f4d8b238ff064bf811bd0ce4cf433fea39436421efbeb509f6f79ad73` |
| record_swap | `1878760560c2e29a226e0a9705d550f4cf20c712bbfa8fcb6e6246b3ca8cd1a6` |
| wrapper_alias | `0a983fb70bb92e810023b6a0b418c0ce55a3bf04c4d01cb5d7a89f2fdb7bc328` |

Each row is checked against its actual executable in the final evidence audit;
full commands, source/header/config/compiler/runtime/caller/runner/input
identities are in the corresponding receipt. The GCC runtime identities are:

| Runtime | SHA-256 |
|---|---|
| `/opt/homebrew/Cellar/gcc/16.2.0/lib/gcc/current/libgcc_s.1.1.dylib` | `5b741a6f933be1fea16db7361121d178bb88b81fc8c567846d1d5593bd0cc146` |
| `/opt/homebrew/Cellar/gcc/16.2.0/lib/gcc/current/libquadmath.0.dylib` | `6f885df6ad8277edf614f0f5f7fbc6f8564a95b64679b88fb860382c9d1c0e9a` |
| `/opt/homebrew/Cellar/gcc/16.2.0/lib/gcc/current/libstdc++.6.dylib` | `6b6012c574ec8d680aaadeaed32a488449b7b75fafcd7bc45348bafdc1d8400c` |

Pinned real-resource hashes (unchanged raw Git blobs):

| Resource | SHA-256 |
|---|---|
| `TIMEEPH_short.te405` | `a455fe86dd66c173ebb5a58a6fe12f65b3e9c21c2000be235b669f84d5561565` |
| `TDB.1950.2050` | `efde871622dc01df4d36df50421f0a5f6cec67707ebe57654c12e2abc63802b7` |

All paths below are relative to `validation/time_ephemeris/`; they are local
and ignored. Receipts enumerate per-case raw archive paths and hashes.

| Artifact | SHA-256 |
|---|---|
| `outputs/reference-corrected/oracle.json` | `14479d4ccd454ef0f0976b887b9054acf2e0283639570aa465ea9dfbde3708a0` |
| `outputs/recording.json` | `2c0d5986baf0efa476aaf768f3945fdf8ddd8d4d335b45313b146e809f0203e7` |
| `outputs/last_execution.json` | `91002fa66a07fe7b0d02ba5a931cf20fdb5a215aa0a01b94884b4b96d927d224` |
| `outputs/verification.json` | `c1e442bb090527d6eea0af6bb9198bef7f772b8c75451adc761af1ff04e1d654` |
| `outputs/verification-mutations-final.json` | `9e4276522b04a974adcabfc022b846e8ae628ad549f651686fc15e5767d0d75e` |
| `outputs/coverage.json` | `4713d29847e2ce9fca320995e9463950e3ef532e3046fa14161e04c52ee4e436` |
| `outputs/inventory.json` | `645e1019dd8d87299b43486bcef8d4ce9ab0d3fbbd755d3f39a3f7606a30df78` |
| `outputs/preservation-frozen.json` | `f2805159485b919e400c89c1d6ab9882fc6c435d80d9eecf0ef5c80f51539cdc` |
| `outputs/preservation-final.json` | `078d31dd7126f9e1feb401713805c9ba869f45734bea34941b7eec8ab94a6d6f` |

`outputs/reference-corrected/*.json.gz` are the 97 complete raw reference
records; `outputs/replay/*.json.gz` are the latest exact replay;
`outputs/mutations/*.json.gz` and `outputs/coverage/*.json.gz` retain their
independent raw executions. Generated binary inputs and expanded action files
remain beneath `build/cases/`. The original `reference-initial/`, `reference/`,
pre-correction coverage and provisional 0048 checker are historical artifacts
kept intact; no earlier defective recording is silently presented as complete.

No generated output is tracked. Central acceptance, a later Rust implementation
contract, other hosts and unrun native-invalid paths remain outside this delivery.
