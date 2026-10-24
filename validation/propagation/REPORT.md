# Assignment AB: original native propagation reference

Delivered for central review under decision 0063 and handoff 28, released at
`b8d6263`. This reference covers all 25 selected routines. It does not declare
milestone acceptance or change Rust, Rug128, M2, a numerical gate, or a physical
convention. Ownership is confined to `validation/propagation/`.

## Source, build and state boundary

The original source is raw Git revision
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`. The ten primary file hashes match
decision 0063. Both `ifunc` overloads count as separate routines. See
[SOURCE_MAP.md](SOURCE_MAP.md) for all routines, raw locations, callers and
dependencies, [CONTRACT.md](CONTRACT.md) for the frozen scientific/state contract,
and [GAPS.md](GAPS.md) for native-safety and coverage limits.

Every primary compiles as a separate unchanged translation unit. Real pinned
`clkcorr`, `tropo`, `observatory`, `tabulatedfunction`, `tempo2Util`, `TKlog` and
`TKlongdouble` sources, plus verbatim global data/CVS slices, supply the closure.
There is no opaque archive, initializer, stub or substituted numerical helper.
Accepted A2 supplies only its minimal config, SHA-256
`470c61b68a61504558e76ef772639e46b565a1e4500115d6e3fb6a71286dd69e`.
W `bd290cc` is fixed Rust context; native troposphere retains the original U
expression, including its wet mapping. No corrected native profile was recorded.

The ordinary profile is serial `-O2 -std=gnu++11 -DHAVE_CONFIG_H`, Homebrew GCC
16.2.0, macOS 26.7 (25G229), arm64, C locale. Measured representations are:

| Native type | Bytes | Significand bits |
| --- | ---: | ---: |
| `double` | 8 | 53 |
| built-in `long double` | 8 | 53 |
| configured `longdouble` | 16 | 113 |

Thus `cmval`/`powl` and L-suffixed secular-motion literals retain the actual
built-in type; they are not silently promoted to binary128. Build profiles record
commands, macros, compiler/backend/assembler/linker, included headers, dynamic
libraries, link map and executed binary. Available runtime files are hashed;
macOS shared-cache libSystem, loader and kernel remain a stated host boundary.

The explicit caller owns two pulsars, six observation cells each, and sixteen
initialized values/flags for each of 26 selected parameter slots per pulsar.
All 349 pulsar fields, 95 observation fields, all 2,000 parameter records and
nested typed records are serialized, including initialized tails. Primitive runs
are losslessly compressed; pointers are logical owner tags or explicit nulls.
No struct padding or uninitialized automatic storage enters the evidence.
Geometry, clock steps, frames and ephemeris inputs are supplied caller state.
Synthetic current-sheet/observatory resources exercise the real source readers.

## Verification results

The canonical fresh run is recorded in `outputs/verification.json`; per-phase
logs are in `outputs/pipeline/`. It rejects source changes during verification,
clears earlier phase-success receipts at entry, and checks that both immutable
original oracles remain unchanged.

| Check | Result |
| --- | --- |
| Primary exact replay | 311 cases, 3,917 action begin/end events; two fresh executions per case |
| Separate supplemental exact replay | 13 cases, 255 events; two fresh executions per case |
| Combined terminal outcomes | 314 successful returns, seven source exits (1), two assertion aborts (-6), one isolated timeout |
| Independent arithmetic/structure checks | 167 supporting checks |
| Separate observer audit | All 324 cases run twice; complete ordinary projection exactly equal |
| Evidence/launch controls | 32 controls, including positive controls and expected rejections |
| Compiled behavior mutations | 43 core plus five additional mutations, each with an actual state/numerical/diagnostic witness |
| Compiler reachability | 25/25 routines, 581/585 lines, 388/400 branch outcomes |
| Coverage-profile equivalence | All 324 cases exactly equal to ordinary O2; no measured differences |

Ordinary recording and verification require exact ordered case/action/field
membership, recursive value types, finite floating bits, signed zero, nonfinite
classes, statuses and diagnostic streams. Actual executable, dependency, input
and resource identities are checked at launch. A fresh unpredictable launch
nonce rejects absent execution and stale event files. Raw streams remain saved;
only declared exact source/resource roots are projected. The non-consuming sheet
scan is explicitly killed after three seconds and compared as `timeout`.

Independent checks cover typed structure/extents, factorial/Hermite recurrences,
shapelet normalization/Gaussian limits, interpolation/sinc, calendar values,
solar-density sums, vector normalization, DM/CM dimensions, retained fields and
full phase-reference copies. Their diagnostic bounds do not define new Rust
tolerances and do not replace the exact native oracle.

The observer records complete initialized observations at each iteration, the
initial zero, real TT/Shapiro/DM call order, the 100-iteration boundary, the
veryFast first pass, both TT sums in BAT assembly, the process-static DMMODEL
warning mask, successful current-sheet reads, full static diagnostic buffers and
initialized mcl2 outputs at defined exits. Its only added public trace material
is a private record and normal-source-exit snapshots; removing those produces
the complete ordinary result. Instrumentation patches, derived caller source,
build commands and actual binary identity are recorded separately. Raw `$Id$`
anchors are resolved in this separate profile without changing ordinary source.

The thirteen supplemental cases were frozen before recording into a separate
oracle. They cover diagnostic/storage saturation, defined mcl2 loop/year exits,
the safe NaN interpolation assertion, additional calendar/rotation paths and
other measured gaps. They leave the first corpus and manifest unchanged.
One additional finite PMRV sensitivity input is frozen before its original and
mutant executions because the nominal short-span contribution rounds away.
That input is a verifier test, not a physical calibration case.

## Original behavior and correction boundary

Controlled multi-pulsar witnesses retain the original first-pulsar accesses in
tropospheric zeroing, DM series/warning/shapelet selection, secular-motion gates
and phase cleanup. Full retained observations and repeated/composed histories
make cross-owner writes visible. Source mutations demonstrate that the verifier
detects changing those accesses and consequential bypass, iteration, frequency,
DMX/CM, solar, interpolation, shapelet and phase-lifetime behavior.

Every mutation is labelled a deliberate control, not a correction. Neither
suspicious source behavior nor a successful mutation test establishes the
intended scientific fix. Policy 0053 and central review still govern any future
correction, with separate expectations and adjacent source/bug/fix comments.
Original TEMPO2 and the ordinary oracles remain unchanged.

The staged `git diff --cached --check` reports one trailing space at `run.py:65`.
It is retained in this delivery because that runner's exact bytes are already
part of both frozen manifests and the successful execution identities. Native
verification passed; the whitespace check did not. The warning has no execution
effect, and the recorded code and oracle identities remain intact.

## Reproduction and local evidence

From the repository root, verify the existing frozen local oracles:

```sh
python3 validation/propagation/verify.py
```

The canonical command runs ordinary replay, supplemental replay, independent
checks, observer audit, controls, additional mutations and coverage in dependency
order, then refreshes the field cross-reference and preservation checks.

For a genuinely new output directory, establish the explicit original references
once, then run the same verification command:

```sh
python3 validation/propagation/prepare.py
python3 validation/propagation/run.py --record
python3 validation/propagation/supplemental.py --record
python3 validation/propagation/verify.py
```

Recording refuses an existing oracle or manifest; preparation refuses an existing
freeze. Preserve existing evidence rather than deleting it to bypass a failure.
Prerequisites are the pinned independent TEMPO2 repository, release `b8d6263`,
W `bd290cc`, accepted S's local `validation/pulsar_lifecycle/inventory.json`,
accepted A2's `validation/reference/build/source/config.h`, Python 3, GCC/gcov 16
at `/opt/homebrew/bin`, and the recorded macOS toolchain/runtime. Exact replay
against this host's existing manifest is intentionally host-qualified.

| Local evidence | Contents |
| --- | --- |
| `outputs/frozen/` | Pre-execution raw source/caller/type/branch maps, authored primary cases and preservation identities |
| `outputs/reference.json.gz`, `outputs/manifest.json` | Immutable 311-case original oracle and source/build contract |
| `outputs/record_identity.json`, `outputs/record_execution_receipts.json.gz` | Preserved initial ordinary recording identities |
| `outputs/supplemental/` | Frozen additional cases, separate oracle/manifest, raw streams and fresh receipts |
| `outputs/execution_receipts.json.gz`, `outputs/last_execution.json` | Latest primary fresh launch/replay identities |
| `outputs/observer_audit.json`, `outputs/observer_snapshots.json.gz` | Complete ordinary projection audit and initialized private histories |
| `outputs/independent.json` | Supporting arithmetic and typed-state assertions |
| `outputs/controls.json`, `outputs/mutations.json.gz` | Controls, exact patches, executed profiles and changed fields |
| `outputs/branch_controls.json`, `outputs/branch_mutations.json.gz` | Five additional compiled behavior witnesses |
| `outputs/coverage.json`, `outputs/coverage_receipts.json.gz` | Every primary compiler line/edge, dependencies, zero-count dispositions and exact comparisons |
| `outputs/field_map.json` | Active field/type/parameter cross-reference and measured precision trace |
| `outputs/verification.json` | Final source/oracle/phase-receipt identities and canonical PASS |
| `build/*/profile.json`, raw-output directories | Actual commands, dependencies, executable hashes and unprojected streams |

Generated JSON/JSONL, compressed traces, binary resources and build products stay
local and ignored under 0039. Only sources, generators, authored contracts and
concise reports belong in the commit.

The implementation revision is the commit containing this report; the final
verification receipt pins every Python source, `probe.cc` and the frozen contract
by content hash.

## Delivery identities

SHA-256 identities for the frozen inputs, original oracles and freshly executed
ordinary binaries are:

| Artifact | SHA-256 |
| --- | --- |
| Primary oracle | `0f43f863b19961842ece487680c4843882565aadfcc67b2581953e1bfb0f5a88` |
| Primary manifest | `ef41a2cdc141b0abf444b44853b23f7344c93178c4e03538146a250cba8dda66` |
| Supplemental oracle | `3b05032ea23bdb9975da0a4cabb2c0746578e951855da69339504307fcd1deb1` |
| Supplemental manifest | `4fe017e7db30bd111cc8ae35136ed42424fb10ee7c5fbb282a4e9f3dc99fe4d8` |
| Primary executed binary | `e144ef2828e0d3a99c8f59261255f2398e1a0825795d257ae9f4eac6d8c39e2e` |
| Supplemental executed binary | `7eb1e55ed7bd3102c91828afe259f81305c42429519775ddc28beb9423e98f94` |
| Observer executed binary | `f0950f63b9486fea5ea8a3b953e3ee7f71b35a3b849b893d30abd37cd3e7a788` |
| Coverage executed binary | `b6164f3e38abc60048b0e69ff803ce20d3f0b29faf4947dceca79b96bf7e5963` |
| Frozen source/caller/type map | `62885bc40b23ee864c04e6d51942ad1e506c3778b29d9511350a849e46665cdf` |
| Frozen primary case file | `b687b887d856ef12f6510a43c13b34bb571562889b735115ffcd9f3c0cd987c4` |
| Frozen supplemental case file | `071825636c932772c509ef7a8871bf6b17315659b4a0dcf09c0321f8d94c9da0` |
| Accepted-file preservation manifest | `77a974697c680e04977acd7dde377891dc9a92532cac7c6ef5f0bc6917c8f1e4` |
| Complete verification receipt | `df4ffad35228f2f1ac3bbd1ae21c361a05b5a99b78a79a5fb62abbd25d143fe7` |
| Coverage receipt | `554976527d0a6ab14d2f862622c17b3e43e33bbaacf3be844cbefb807b183f6b` |
| Observer audit receipt | `14fca6a1949401eb8f28a861ef2f7d215ad2da5a3d8a3a27be48aaa2febe08b5` |
| Field cross-reference | `5a38fd710f88f5fe1ff3ee966713dcd5c006dcd8260bbe5d5ece513c4cae6f7e` |

The final canonical run passed every phase. Its receipt binds all seven phase
receipts and both unchanged oracles; the phase artifacts retain the actual
profiles and executed input/dependency identities. The primary profile has 354
dependencies. All final source, oracle and phase-receipt hashes were checked again
before staging. A later fresh replay produces new nonce-bearing receipt hashes;
the hashes above identify this delivery, not an instruction to overwrite it.

## Limits and preservation

All four unhit lines and twelve unhit compiler outcomes are individually retained
and classified in [GAPS.md](GAPS.md). Some are terminal counter loss; others are
explicit finite-domain gaps or native-invalid paths. None is silently treated as
covered. Full global/formBatsAll, CALCEPH, residual/binary/fitting integration,
physical calibration, allocation/I/O fault injection, OpenMP, concurrency and
other hosts are outside this reference. Logging/formatter results retain M's
native ABI/host qualifications. No Rust tests were run because no Rust changed.

The preservation gate checks 8,306 frozen accepted files under the reference,
pulsar-lifecycle, terrestrial-resource and observer-coordinate directories, plus
clean unchanged HEAD/status for both independent repositories. M/R contracts
were read without edits; this file-hash set is not a claim to cover every accepted
directory. Evolving central/Y/Z/AA paths are excluded from immutable membership.
The independent HEADs are TEMPO2
`fda6ad4a9753ad7dc5d291406f2f310a9161423f` and the accepted reference repository
`7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`; both retain clean status.
