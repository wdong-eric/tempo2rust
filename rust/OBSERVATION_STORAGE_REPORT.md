# Assignment R: complete source observation storage

Delivered 24 October 2026 under decision 0037 and handoff 18. **The released
storage/integration checks pass; central acceptance remains pending. M2 remains
unaccepted with exactly its four historical failures.** This is an observation
storage foundation, not the complete input lifecycle or the complete TEMPO2 port.

## Implementation and source correspondence

The pre-edit freeze is `624c1bf`; production and focused tests are `b7a4d17`;
the exact verifier and authorized O output routing are `c1498e2`. Concurrent
central/reference commits advanced shared HEAD during execution. The final run
records shared HEAD `a70db2683c418fa655ff1fa0084e8468c2c26f70`; executed-source hashes, not that shared
commit label alone, identify every compiled implementation.

Canonical raw source: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
Raw tempo2.h SHA-256:
`0313b6eaf9f9ae9d7e5cce340379ad4b7068394e5222a01f73949e3a2640de93`.
The nested worktree expands the Git $Id$ keyword. Native builds use raw pinned
Git blobs, consistent with M/N/O; the filtered worktree header is not substituted.

| Source responsibility | Rust counterpart / evidence |
| --- | --- |
| tempo2.h:409–513, all 95 observation fields | `src/observation.rs::SourceObservation`; reexported by `tempo2.rs`; complete ordered names/types/extents in `validation/observation_storage_port/fields.json` |
| tempo2.h:399–404, clock_correction | `SourceClockCorrection`: optional binary64 correction and 32 individually optional bytes |
| initialise.C:68 allocation; inactive loop at 87–94 | `SourceObservation::default`: every logical scalar/cell/byte unwritten |
| tempo2Util.C:293 whole observation assignment | Safe owned Clone/clone_from; M observation-only projection and supplemental native member assignment |
| Existing strict FORMAT 1 bridge | `read_timfile.rs`: sat/freq/toaErr/fname/telID only; provenance returned separately |
| Existing input profile selection / snapshots | `input.rs`, `lib.rs`, CLI and comparison example consume canonical site and derived snapshots |
| Existing barycentric / terrestrial inputs | `form_residuals.rs`, `form_bats.rs`: canonical SAT, frequency and telID; original line/name/index for provenance only |
| Existing fitting input weights | `t2fit.rs`: source toaErr with unchanged binary64 multiplication by 1e-6 |
| clkcorr.C public operations/helpers | `clkcorr.rs`: same record, operation-local optional reads and unchanged calculation/routing/session logic |
| Active toa2utc.C:76–114 | `toa2utc.rs`: signed obsNjump + fixed jump backing; source SAT-jump index mismatch, extra shifted slot and partial writes retained |

The record contains 20 Rug128 cells, 354 binary64 cells (including 30 nested
corrections), 87 signed i32 cells, one signed i64 pulseN and 4,121 byte cells:
4,583 individually represented logical cells. Arrays retain fixed dimensions:
501 filename bytes, 100 telescope bytes, 40 jumps/flags, 32 bytes per flag
ID/value, 30 correction entries with 32 destination bytes, and every header
vector/matrix dimension (including 9 by 6 planetary arrays). Counts are signed
and independent of capacity. No C ABI/pointer/padding equivalence is asserted.

`None` is the sole unwritten representation. No sentinel value or allocator
pattern implies initialization. Scalars, isolated bytes, holes, written prefixes
through NUL and retained initialized suffixes survive whole-owner cloning.
Rug values retain 128-bit precision, signed zero and nonfinite classes. All
production reads occur at the consuming operation. Byte reads distinguish an
unwritten byte from a missing terminator; writes retain completed prefixes at
capacity boundaries. The existing process clock session retains terminal failure
and reentry behavior. No unsafe block was added.

## Canonical integration and API

TimingInput owns one vector of complete source records and a separate vector of
original text/line/index provenance. There is no mutable numeric mirror.
`source_observations()` returns a read-only slice of those records;
`observation_site(index)` reads source telID. Both evaluators and fitting weights
read source cells. Input BAT/residual fields remain unwritten even when an
immutable timing result contains BAT/residual outputs.

**Authorized API change:** `observations()` now returns a fresh owned
`Vec<Observation>` instead of `&[Observation]`. Mutating its values or provenance
does not change the input. JSON continues to serialize the same keys, membership,
number values and original text; source storage and initializedness are not
added to output JSON. Repository callers have been adapted where required.

The FORMAT 1 grammar, numeric parsers, supported directives/sites and ordering
are unchanged. The filename storage boundary is 500 UTF-8 bytes plus NUL. Tests
cover fitting/non-fitting ASCII lengths 500/501 and multibyte lengths 500/501,
with the original source line in overflow errors. This is a safe storage
boundary, not a new native parsing/overflow interpretation.

The separate ClockObservation/ClockCorrection owners and obsolete clock byte
copy helper are removed. Terrestrial evaluation clones the canonical source
record and explicitly supplies L's clockCorr=1, count=0, no-members and zeroed
correction buffers. Those caller seeds are not native observation defaults.
Direct L tests explicitly retain their historical zero/sentinel inputs. The
clock source-name identity branches return before SAT/site reads where the
source does. `toa2utc` uses signed active membership, assigns each shifted
correction before its destination bytes, reads at most 31 bytes for strncpy,
pads only after a known NUL and retains byte 31. Unused fields/tails are preserved.

## Exact direct evidence

`validation/observation_storage_port/PROJECTION.md`, `fields.json`,
`baseline.json`, `m_projection.json` and `protected_hashes.json` were frozen
before production edits. The M projection retains all 95 fields in all three
backing observations for copypsr_0/1/2/3, including the destination's uncopied
suffix. It excludes unrelated pulsar/parameter fields and native offsets/padding.
N's complete inventory and explicit caller-state/source map provide context;
no N output is presented as a Rust TIM-reader result.

The supplemental native caller compiles against unchanged pinned headers.
Every field and array position receives a controlled initialized value; full
byte buffers include embedded NULs and nonzero tails. It measures native field
widths/extents and signed int/long-long types and performs the source's ordinary
whole-record assignment. Three cases cover finite distinct values (including
exact binary128-only values), signed integer extremes, signed zero, infinities
and NaN classes, and independent destination mutation. No natural native
unwritten memory or C object padding is read as a golden.

Each debug/release direct run passes:

- Three complete native before/after assignment cases, including unchanged source.
- Four M observation assignment/retention histories with exact ordered fields.
- Fifteen Rust boundary/prefix cases: byte holes, through-NUL reads, write capacity,
  missing NUL, clock identity without SAT/site, negative/oversized counts,
  unwritten active corrections, disabled clock, strncpy holes/no-NUL/early-NUL,
  jump capacity/unwritten membership/negative membership and terminal reentry.
- Thirty-nine controls: one positive and 38 rejected mutations/failures, including
  omission/order/type/extent/count/sign/class/tail/initialization/prefix changes,
  native ABI/signedness, Rust/native source/compiler/executable identities,
  stale tokens/input hashes, nonzero processes and missing outputs.
- 22,449 exact finite-number comparisons, 103,938 field visits and 439,192 scalar
  leaf comparisons. These overlap and include repeated states; they are not
  distinct physical cell counts.

Finite native IEEE values are decoded to exact rationals; Rust integer/exponent
exports are compared exactly, with signed zero separately retained. Nonfinite
classes are exact; NaN payloads are outside the contract. There is no numerical
tolerance, mode sorting or scientific-number approximation in the direct gate.
Debug/release state projections are identical after removing only fresh token
and input-file identity fields.

Five focused Rust tests cover whole-record ownership/classes/precision,
operation-local partial initialization, byte-capacity frontend boundaries,
canonical SAT/uncertainty/snapshot/JSON/barycentric integration and canonical
frequency/SAT/site changes reaching terrestrial evaluation. Ordinary test runs
pass in both profiles: 28 library tests, seven behavior tests and one reference
contract test. Seven fresh-process probes are ignored by the ordinary harness;
the required direct probes are explicitly launched by their verifiers.

## Execution identities and full regressions

Host: macOS arm64; rustc 1.97.1 (8bab26f4f, LLVM 22.1.6). The native compiler is
GCC 16 with the pinned binary128 header and libquadmath. Native direct debug uses
-O0 and release uses -O2. Complete compiler/backend/header/library/runner/source
hashes and actual command arguments are in each direct receipt and compile log.

| Executed artifact | SHA-256 |
| --- | --- |
| debug Rust test executable | `a633689e4b650d0edd30fb55558f516015b45fb6f2b01b7f1267f2ab065aaa04` |
| debug supplemental native executable | `2c70a3f6408bf2370476c72b17d7daec8c92aa674363114f1d1d60792bbc196a` |
| release Rust test executable | `71623cbf4e17d52b9f7c718fe33ee3c1f0a41861c964f62ab4869ca590604d74` |
| release supplemental native executable | `403f6823ffc9d55094a47477189e95914817e8dd82488caa771998f7a69de7a0` |
| debug regression CLI | `11923da288cbb6532bfd09a55f54246a9ee05d816446a02affc3ff24a71d56fa` |
| release regression CLI | `a35e70701fb929829c4c42e3da6d172bd9021b327bc58f079a67900f8c2971a6` |

Formatting, Clippy with `-D warnings`, ordinary debug/release tests, O's exact
storage/comparator controls, clock/glob/provenance, JPL and M1a/M1b all pass.
O retains its ten cases/32 ordered events, 30 controls and 96,906 exact finite
comparisons per profile. Clock comparison retains 142 exact cases, eight safe
Rust boundaries and one unrun cycle, with zero finite differences. The glob
supplement retains 18 helper patterns, three integrated cases and nine controls.
JPL gates and their comparator controls are unchanged.

M2 ran to completion and exited **1**. Its entire failure list equals the
historical `rust/verification/m2/open_gate_failures.json`: debug and release
dependent-column raw retention-order mismatches plus singular-value index-zero
mismatches. No expression, SVD order, backend, precision, rounding/fusion policy
or tolerance changed. M2 acceptance remains unresolved under decision 0012.

All new results are beneath `rust/verification/observation_storage/`. The only
O verifier edits are run.py's output-root environment override and native.py's
corresponding build-root routing; fixtures, projections, comparison, controls,
protected membership and expected results are unchanged. O/verify.py is unchanged.
The inherited nested verifier status strings retain their historical assignment
labels; R's top-level receipt reports this execution. Their immutable snapshots
record their execution-time view; `final_artifacts.json` seals the final outer
receipts after parent runners finish writing them.

## Reproduction and receipts

Run from the workspace root:

```sh
python3 validation/observation_storage_port/verify.py
```

Individual fresh direct runs:

```sh
python3 validation/observation_storage_port/run.py --profile debug
python3 validation/observation_storage_port/run.py --profile release
TEMPO2_PARAMETER_VERIFICATION_ROOT="$PWD/rust/verification/observation_storage/O" python3 validation/parameter_storage_port/verify.py
```

The runners set the crate-local Cargo/GMP caches and C locale. Native source and
build files are staged only beneath R. Commands, stdout/stderr, launch tokens,
input/output hashes and every executed identity are retained. The test visitor
and native supplemental caller can be regenerated with
`python3 validation/observation_storage_port/generate_probe.py`, followed by
`cargo fmt --manifest-path rust/Cargo.toml`; production storage is independent.

Key artifacts:

- `verification/observation_storage/delivery_checks.json`: full run result.
- `debug_direct.json`, `release_direct.json`, corresponding `*_raw.json.gz` and
  `*_native.json.gz`: exact evidence, identities and controls.
- `O/delivery_checks.json` and `O/regressions/delivery_checks.json`: unchanged
  storage and clock/JPL/M1/M2 gates, with logs and complete raw outputs below them.
- `module_inventory.json`: 39 Rust module/test-source files and unsafe-site lines.
- `final_artifacts.json`: final source/report/artifact membership and hashes.

## Preservation, unsafe audit and remaining scope

All 1,964 frozen protected files retain their hashes. Both independent nested
repositories are clean at fda6ad4 (tempo2) and 7795d53 (validation/reference).
P preprocessing, Q par_specialized and S pulsar_lifecycle remain outside R's
immutable membership as directed; their work was not staged by R. Central's
live HANDOFF.md advanced concurrently. `membership_note.json` preserves its
baseline/observed identities and records its removal from the immutable set;
no accepted scientific reference/projection/output was removed. The R freeze
remains a pre-production record of the fields and caller contract.

All twelve existing unsafe declarations/blocks are retained in stdio.rs,
clock_text.rs and get_obs_coord.rs. stdio.rs/get_obs_coord.rs are byte-identical
to the baseline. clock_text.rs differs only by deleting the now-unused safe
copy_string helper after its unsafe bodies. Cargo.toml/Cargo.lock/build.rs and
native numerical sources are unchanged. No new unsafe boundary or dependency.

There is no new source-backed scientific/API decision outside 0037 to request.
Full pulsar allocation/initialization/destruction, complete copyPSR, source
PAR/TIM/preprocessing, source numeric utilities, linked values, additional
models, source numeric parsing versus Rug/binary128, allocation failure,
concurrency stress, performance and other-platform claims remain outside R.
Natural native unwritten values have no golden. This stage represents and safely
consumes the released storage; it does not close these later transfer gaps.
Central alone accepts R and releases subsequent Rust work.
