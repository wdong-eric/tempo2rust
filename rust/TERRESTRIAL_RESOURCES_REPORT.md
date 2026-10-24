# Assignment W: terrestrial resources

24 October 2026. Implemented under 0056 and the correction policy in 0053.
Central retains acceptance of W and the complete core transfer. M2 remains
unaccepted. Final U checks pass in both profiles. The full regression run and the focused
verification after one final redundant-check removal are distinguished below.

## Delivered behavior and scope

All 22 active routines in pinned `observatory.C`, `eop.C` and `tropo.C` have Rust
counterparts. Observatory records, EOP and meteorology share the existing process
owner with clock diagnostics, with their own initialization flags and source
lifetimes. RuntimeContext's former EOP/weather caches and parsers are removed.
The complete tropospheric driver uses T/O/R's canonical pulsars and observation
owners. The bounded PKS frontend uses its same observation calculation body.

The public library/CLI input scope and JSON schema remain unchanged. Full internal
resource routines do not imply general PAR/TIM, arbitrary-site, other geometry,
IFTE/FB90, binary-model or additional fitting support. IF99/JPL algorithms and
Rug128 remain unchanged. No new unsafe, dependency or global arithmetic flag was
introduced. Concurrency stress, other platforms and performance are untested.

## Frozen source, fields, branches and callers

Raw pin: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.

| Raw file | SHA-256 |
| --- | --- |
| observatory.C | `fabfaabc78b75652590c5a81c1c2bb05f806550f99dbd7dd3f1ab43d056bf2c3` |
| eop.C | `ff90736cb57c96ce06411e4aa5dbb692b5c5ec0f4a9069fd9403eb7bb9d469a7` |
| tropo.C | `871d06bb10e647df0d563905db20b560cd659fcb4b372fed0c790937835a7fe6` |

Before production edits, commit `a8ef85c` froze the authored transfer contract and
generator. The immutable local `validation/terrestrial_resources_port/build/freeze.json`
(SHA-256 `1039cd60f60a1e2f659b28c723e5b6f0f9a4544af84b0012b9d9a68d8a6b06ef`)
contains U's complete bodies/branch/caller map, 130-case membership, old Rust
consumer sources and protected identities. The full authoritative field/type,
branch and native caller occurrences remain in U's
`outputs/frozen/source_map.json`; they are incorporated by exact identity rather
than replaced by a shorter handwritten inventory. The accompanying
[transfer contract](../validation/terrestrial_resources_port/CONTRACT.md) specifies
all fields, branch classes and ownership interfaces before implementation.

| Source routine | Rust counterpart | Caller/state contract |
| --- | --- | --- |
| readObservatoryFile | ObservatoryDatabase::read_file | Direct/init callers; append records, per-file local string tails |
| readAliases | ObservatoryDatabase::read_aliases | Direct/init callers; accumulate ordered alias lists and warning flags |
| initObservatories | ObservatoryDatabase::initialize | Explicit append; main before sorted extras before aliases |
| lookup_observatory_alias | ObservatoryDatabase::lookup_alias | Lazy initialization; first case-insensitive list; ordered output writes |
| getObservatory | ObservatoryDatabase::index/get | Clock, runtime and driver lookup; derived owned snapshot |
| fang | observatory::fang | All modes and geodetic diagnostic conversion |
| ITRF_to_GRS80 | observatory::itrf_to_grs80 | File loading and direct geometry |
| GRS80_to_ITRF | observatory::grs80_to_itrf | Geodetic error path and direct geometry |
| load_EOP | eop::load_eop | Append to explicit caller-owned sample vector |
| get_EOP | Eop::initialize/get_eop | One first-use process series; output slice plus four checked indices |
| MeteorologyFunction_load | MeteorologyFunction::load | Caller-owned direct record or shared vector construction |
| MeteorologyFunction_getValue | MeteorologyFunction::get_value | Existing table interpolation and diagnostics |
| MeteorologyFunction_getStartMJD | MeteorologyFunction::start | Existing table first sample |
| MeteorologyFunction_getEndMJD | MeteorologyFunction::end | Existing table last sample |
| initialize_meteorology_table | tropo::initialize_meteorology_table | Clear one vector; reusable local record and ordered glob paths |
| initialize_meteorology_tables | Meteorology::initialize | Replace zwd then sap; publish initialization flag |
| getMeteorologicalValue | Meteorology::get_value | Lazy shared initialization; first inclusive matching table |
| getZenithWetDelay | Meteorology::wet | Actual selected table or zero sentinel with TROP1 |
| getSurfaceAtmosphericPressure | Meteorology::pressure | Actual selected table or 101.325 kPa with TROP2 |
| NMF_hydrostatic | tropo::nmf_hydrostatic | Latitude/hemisphere/season/height branches |
| NMF_wet | tropo::nmf_wet | Latitude branches with the explicit correction below |
| compute_tropospheric_delays | tropo::compute_tropospheric_delays | Signed multi-pulsar/observation loops; shared compute_observation body |

The eight observatory routines are in `src/observatory.rs`, two EOP routines in
`src/eop.rs`, and twelve meteorology/Niell/driver routines in `src/tropo.rs`.
CamelCase source fields and recognizable routine names are retained where already
canonical. `xyz` maps x/y/z; longitude/latitude/height map the three GRS80 cells;
`clock` maps clock_name; table name/header/site storage maps the source
256-byte buffers. U's exact native type map supplies every field spelling and
extent, including all 349 pulsar and 95 observation fields.

Observatory records retain six binary64 values and fixed code[16], name[32],
clock_name[16] initializedness and tails. Alias code[100], list order and warned
state remain distinct. Accessors derive logical strings; there is no parallel raw
observatory record. Safe indices and owned snapshots prevent vector relocation
from invalidating caller references. EOP samples retain MJD, xp, yp and UT1-UTC;
load-only duplicate, unordered or nonfinite rows remain observable. Interpolation
checks its own bracket invariants, without rewriting the loaded series.
MeteorologyFunction retains filename/header/site known prefixes and tails through
repeated loads, plus the existing table samples and fault state. Coordinates and
height are metres, longitude/latitude radians. EOP epochs are MJD days, input
polar motion is converted from arcseconds to radians, and UT1-UTC is seconds;
the fourth output retains the source's fixed division by 86400 even for irregular
sample spacing. Pressure is kPa, zenith wet delay and final tropospheric delay
are seconds. Weather selection uses SAT and the Niell season uses corrected UTC.

Native callers outside the existing Rust frontend (preprocessing, TIM, polyco and
plugins) remain explicitly inventoried in U. Literal-disabled conversions, test
loops and linear table search remain inactive. They are not claimed transferred
as active production callers.

## Canonical lifetime and actual frontend path

`ClockSession` owns observatories, EOP, meteorology, clock tables/routes and the
one diagnostic registry. Separate initialization flags preserve source behavior:
explicit observatory initialization appends; aliases accumulate; EOP retains its
first successful load; explicit meteorology initialization replaces zwd then sap;
lazy weather use retains existing tables. A new context or root does not reset
those caches. Direct table objects and explicit EOP load vectors are independent.

Outer operations acquire one mutex and pass borrowed session state. Nested
lookups never reacquire it. `with_resources` allows independent geometry,
observatory/EOP/weather and lifecycle operations after a terminal clock fault;
clock calculations still go through the established terminal clock boundary.
No changed terminal scope is inferred from merely sharing a mutex.

The public path is `RuntimeContext::new` → canonical observatory/EOP/weather
initialization → `form_bats_all` → common EOP query → `get_obs_coord`'s existing
SOFA adapter → `tropo::compute_observation`. The full source driver uses that same
body through `PulsarContext::pulsar_observations_mut`, with scoped T owner checks.
Its `npsr`/`nobs` loops are signed; it resets delay before tests, retains per-pulsar
warning state, zero-zenith/STL_FBAT/COE order and writes only troposphericDelay.

The bounded frontend explicitly supplies its selected pulsar direction, warning
mode and enabled delay-correction flag on an owned observation copy. The original
input remains unchanged. Pressure/wet lookups occur once each in source order;
Weather provenance comes from those actual selections. Weather uses SAT; the
season uses SAT plus the binary64 UTC clock correction/86400, added in Rug128
before conversion to the NMF binary64 argument. That clock call is the actual
source calculation, not a second provenance lookup.

Resource digests derive from bytes loaded into the canonical records. Cached
resources retain their original path/hash across root changes, deletion or file
replacement. Neither resource export nor Weather provenance reopens a file to
pretend changed bytes supplied a cached value. JPL's existing read/hash/stream
semantics and IF99's context ownership remain unchanged.

## Source correction and arithmetic

`src/tropo.rs::nmf_wet` has an adjacent comment naming raw
`fda6ad4 tropo.C::NMF_wet:206`. The original upper c coefficient is read from the
b table. Interpolation of the c table must approach its own adjacent knot; the
original expression breaks that continuity. The correction reads `cs[ilat2]`.
Other source conventions—including the zero-weather fallback, hydrostatic
formula, units, season and constants—remain unchanged.

[CORRECTIONS.md](../validation/terrestrial_resources_port/CORRECTIONS.md)
enumerates all 31 changed exact wet-result fields in 10 U cases. A disposable
one-line native patch reruns the same 130 histories with U's O2 profile. All other
fields, streams, exits and integrated U delays remain exact. The original Rust
130-case agreement is also retained under `original_source_debug/` (its pilot
receipt records the separately explained central-document preservation drift).

| Witness | SHA-256 / result |
| --- | --- |
| Original U oracle | `d955d6b9ee4bf8e9259decff4b5c16cf25e97003034983def21c1895d00d94a2` |
| Separate corrected oracle | `6948f57785698ec65704652443f79146a26a92da6c4869a14d3b1e3b6eb0386e` |
| Corrected native experiment contract | `c4273b5fa79d3d2a748575a96d749a7ab90f60385995dc2339cb225906abf97a` |
| Changed-field inventory | `5738b2443b5d195658a9529327a0bb2e2bac9bf94510fd8b6998344c24b7d665` |
| Independent knot-continuity test | Original expression fails (exit 101); corrected expression passes (exit 0) |
| Accepted additional witness | U's actual compiled wet_mixed_coefficient mutation |

`correction_test.py` temporarily reinstates precisely the original typo in the
owned source, records the failing test and restores the exact source bytes in a
finally block before rebuilding and passing. This is independent evidence; the
ordinary direct comparator never records expectations or changes a source.

Initial exact comparisons exposed binary64 contraction differences. GCC16 O2
source-line assembly and a bounded fused/unfused experiment are recorded under
`validation/terrestrial_resources_port/build/arithmetic/`; four sampled scalars
change. Adjacent Rust comments mark only compiled contraction sites in fang,
both geometry directions, three EOP final writes, Niell coefficient interpolation
and the driver. `CORRECTIONS.md` lists the exact source locations. Explicit
mul_add reproduces those operations; no global fusion, fast-math, wider tolerance
or alternate precision mode was introduced. This is compiled-source agreement,
not independent validation of the physical approximations.

## Exact comparison and safe boundaries

The frozen projection retains all 130 cases, 1,128 ordered actions, 2,256 events
and 24 exit prefixes in each profile. The probe constructs U's actual
value-initialized two pulsars/twelve observations, independently of production
initializers. Generated assertions inspect every typed non-target field,
including inactive backing and fixed bytes, before/after the driver.

Finite binary64 bits (including signed zero), nonfinite classes, nested scalar
types, ordered case/action/event/field membership, buffer capacities, knownness,
retained tails, resource/cache/alias state and warning/error streams compare
exactly. Only four declared native ABI profile cells are omitted. Binary128 SAT
bytes are an explicit fixture serialization of the Rug value; production remains
Rug128. Exact source/build/case root substitutions follow U, with matching path
byte lengths so TKlog truncation is preserved. Raw streams remain available.
Native success/exit status 0/1 maps to library success/source_exit; actual test
process status is recorded separately. Native-invalid boundaries are never
manufactured into successful native goldens.

Focused Rust checks cover invalid numeric conversions, short/unwritten source
storage, invalid output indices, empty/singleton/duplicate/unordered EOP brackets,
all-four-output aliasing, upper-proxy prefix followed by valid in-process recovery,
warning-dependent NOMATCH followed by defined initialization, released observation
owners, and resource/lifecycle access after a terminal clock fault. Existing table
and clock boundaries remain their established contracts. Source exits retain
inspectable load/diagnostic prefixes; continued execution after a native exit is
a library adapter disposition, not a native equivalence claim.

28 W controls challenge real nested records and execution: missing/permuted
cases/actions/fields; bool/int/float distinctions; tails/knownness/capacity; cache
and alias histories; diagnostics and exit prefixes; non-target retention; original
wet results; stale sources/compiler/executable/nonce/input hash. A fresh unchanged
case is actually executed with a new challenge. U's 29 controls and thirteen
compiled source-mutation witnesses remain accepted immutable evidence.

## Verification and preservation

The full reproducible command is:

```sh
python3 validation/terrestrial_resources_port/verify.py
```

Execution requires the local pre-edit freeze and the separately recorded native
correction experiment; it refuses missing/stale artifacts rather than recording
an oracle. `corrected_native.py`, `arithmetic.py`, `correction_test.py` and
`generate_fixture.py` are explicit evidence/fixture generators. Generated outputs,
binaries and receipts stay ignored under W's two output roots.

The full regression sequence completed at recorded HEAD `7946aeee6e7e105fc276b311f1c36a4288767d0d`
with the source identities in
[the preserved full receipt](verification/terrestrial_resources/before_guard_review/delivery.json)
(SHA-256 `c4a1b54094e73e2f6b78d5baca4f2965ec0e8ec93082a8a2335f68f105b3638b`).

| Full run, before the final guard removal | Result |
| --- | --- |
| U, debug and release | 130 cases / 1,128 actions / 2,256 events / 24 exits per profile; exact corrected contract; 28 controls per profile |
| Formatting / Clippy | PASS, including all targets with warnings denied |
| Ordinary tests, debug and release | PASS; 38 library tests, with 12 probes separately ignored by ordinary cargo test |
| T, each profile | 30 cases / 133 events / 5,329,542 typed records; 35 controls; 18 helpers, 320 shifts, eight clamps and two native link mutants |
| R, each profile | Three direct cases, four M histories and 39 controls; exact state |
| O, each profile | Ten direct cases and 30 controls; exact state |
| Clock/glob/provenance, both profiles | 142 exact cases, eight safe boundaries; 18 glob patterns, three integrated cases, nine mutations; provenance PASS; one native cycle remains unrun |
| JPL, both profiles | Accepted direct, isolated/header/boundary and comparator checks PASS |
| M1a / M1b | PASS with their existing comparison budgets |
| Complete M2 | Ran to completion; expected exit 1 with exactly the four unchanged historical raw mode-order diagnostics |
| Public resource examples, both profiles | PASS: cache/provenance, weather, input retention, CLI and in-process recovery |

After that run, the updated standing rules prompted one isolated production
change: remove `site.len() >= 256` after successful table loading. The loader
already rejects headers longer than 255 bytes and requires '#'; its remaining
token can be at most 254 bytes. No hypothetical invalid object is used to justify
a duplicate check. EOP queries still validate brackets because their loader
intentionally accepts unordered/duplicate rows. Scoped lifecycle access checks
revocable owner liveness. Canonical source storage can be partly initialized; it
is not a fully validated TimingInput.

`final_review.py` proves the sole production delta, checks unchanged dependent
algorithms/examples and verifies the retained pre-change binaries. It ran
formatting, Clippy, the existing table-boundary and meteorology tests, and **all
130 U histories plus 28 controls in each profile** on the final sources: PASS.
The unrelated full regression chains were not rerun just to replace a source
stamp. Their original receipts remain pre-change evidence. The focused run was
recorded against base HEAD `bdda5d57dbc879b46024935b5d1a5ad2e1da2bee` plus the explicit delta, later
committed as `eb2a589`.

The final delivery record is
[final_scope.json](verification/terrestrial_resources/final_scope.json), SHA-256
`16830551f1c915aa827bdbb05ac520da14a5c8d5ff3ffddb831321f6c0b77718`.
The exact tropo.rs delta is
`04ee953a46be4e824136189ec6f4a06ef491c9bb6e66e1e2d726087178c8db1f` →
`4fb466adbf346f1660b843636fed84a38ce9941a7e1ae19c2a0cd3cfbcd39b52`.
This is a removed unreachable check, with unchanged scientific calculations.

| Final identity | SHA-256 |
| --- | --- |
| Rust compiler | `210df6794001b73ec3d453878707fa1e0bdcb63c427024a6e6574bbe5615a4da` |
| Debug U executable | `589e0540e5bb7f1c6b83c73db700bdff01b6d162953629a54a8b87aeed70d1c2` |
| Release U executable | `c754828c3e046f803fdbf82827fbe92c989c9511eb918877ee869292f7f3b49b` |
| Patched native executable | `dec332f168ab938700efc5a2c167ac9a0aa79ed806e5bd534160a6160fdfb45c` |
| Final fixture generator | `f688f543776e22745936bf4731e40fd723292c80b6904fa939c8d21e9b8e7f53` |

Executed host/toolchains: aarch64-apple-darwin, Rust/Cargo 1.97.1
(`8bab26f4f68e0e26f0bb7960be334d5b520ea452`, LLVM 22.1.6), GCC/GFortran 16.2.0;
native U uses its original O2 profile. Every selected Rust/harness source,
compiler, executable, challenged input and before/after resource hash is in
`debug|release/<case>/launch.json` and `receipt.json`; raw events and both byte
streams are retained. For example, final debug real_pks_smoke input SHA-256 is
`31c6d8cc517f61693007519241efb9a16b52b9f5376efbead8b10d5ed2519b7c`;
its loaded fixture smoke.sap and smoke.zwd bytes hash to
`253b5c27614a20d52828950878c15e082d99a1a4f871dcf33415425b5120b0d0`
and `d113300302ef6b5501199ea390dc7f2197200d1544685f71bf81e179bb9baa28`.
Native dependency/compiler/runtime identities remain in corrected_native's
contract; full command lines and statuses are in commands.json and
final_commands.json. Final source identity is in final_scope.json; the older
module inventory remains explicitly tied to the full pre-change run.

T runs its original direct and supplemental callers with recorded ROOT/OUT
routing. R's original runner SHA remains
`7d6424654a666d3852255bd9264ba0fbb88ba3c6e57f7bf717da4087a6b10529`.
The disposable R verifier updates only the obsolete whole-file get_obs_coord
hash expectation after proving its two unsafe blocks byte-identical to a8ef85c.
The five clock_text unsafe blocks are likewise byte-identical. Safe EOP argument
plumbing and the safe integer scanner account for those file changes. Exact
original/staged hashes, diffs and the unsafe audit are in `routing/`; numerical
comparisons and O/clock/JPL/M1/M2 logic are unchanged.

The first downstream run failed the old fitting recovery assertion solely at
`cycles[0].prefit.warnings`: initial load/TROP/DUP messages versus an empty later
batch. The complete recursive diff confirms every other field identical. The
owned test now excludes only that first warning batch, with all numerical,
resource and later-stage fields still compared exactly. U still checks warning
histories exactly. `failed_attempt_1/` preserves the original failure and diff.
No fitting algorithm or numerical expectation was changed.

A final generator reproduction check detected an extra standalone rustfmt call
on an include file that cargo fmt leaves raw. The tested fixture bytes were
restored exactly (`b0f1d0bf2ab18aa975b6aaf199ff7e61cc17d25a294a19bab891dd1df1fd4c2b`),
and the generator now reproduces them. `fixture_repro/receipt.json` records that
check. Prior aggregate receipts remain in `before_generator_fix/`; the complete
verification sequence is freshly executed for the final generator identity.
The old receipts were not relabeled as executions of the newer generator.

The original frozen manifest retains 12,366 historical entries. Following
central's explicit 0057 clarification, 81 evolving central Markdown documents
are exempt from the live equality check; all 12,285 accepted evidence members
still match. New X observer-coordinate work is outside W's membership. The
initial HANDOFF drift failure remains recorded, with the original freeze intact.
Both nested repositories remain at their frozen pins and clean:
`tempo2=fda6ad4a9753ad7dc5d291406f2f310a9161423f`,
`validation/reference=7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`.

## Remaining limits

U's coverage measured 22/22 routines, 336/341 executable lines and 217/252
compiler branch outcomes. W replays that complete catalogue; it does not claim
new native coverage. U's reviewed 35 unhit outcomes remain 23 host-dependent,
seven still-defined-unverified, two inactive and three native-invalid. These
include allocation/unwind and glob failures, warning/error capacity histories,
selected quiet/debug combinations and unresolved gcov attribution. Full details
remain in U's unchanged GAPS.md. Invalid native pointers, uninitialized scans,
overflows and malformed native brackets were not executed. No concurrency,
locale/other-platform or arbitrary I/O failure campaign was run.

All earlier scientific/numerical budgets stay in force. The four M2 raw mode-order
diagnostics remain open; neither this transfer nor exact resource agreement
accepts M2, the complete timing core, or the physical adequacy of the source's
atmospheric model.


## Owned stage commits

- `a8ef85c` — docs: freeze terrestrial resource transfer contract.
- `7df6234` — feat: complete terrestrial resource routines and shared state.
- `a5cc8c2` — test: verify terrestrial resources against explicit native contracts.
- `eb2a589` — refactor: trust validated meteorology header bounds.

The final documentation commit contains this report, README, correspondence and
behavior-disposition updates. All commits use explicit owned paths; generated
JSON/JSONL, raw/compressed evidence and binaries are ignored and unstaged.
