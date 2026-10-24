# Assignment Y: complete time-ephemeris transfer

Delivery for central review under 0058/0061; acceptance remains central-owned.
The production implementation is commit `11165d1`. Verification generators
and the example error assertion are committed in `5343b1d`; this report and
current correspondence/behavior maps complete the authored delivery. The release base is `5920affdcb2de05df7b2101c6f53455e9ff31d1a`, after
accepted W `bd290cc` (production `eb2a589`). Native TEMPO2 remains
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`; A2 remains
`7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`. V is accepted `f179ce5`, authored
sources `573d667`, using only its corrected ordinary oracle as baseline.

## Transferred responsibilities

All 29 active routines are implemented: seven byte helpers, ten IFTE operations,
four tt2tdb operations and eight read_fortran.h operations. The complete original
routine/branch/field/format/caller map and explicit Y dispositions are frozen in
[`CONTRACT.md`](../validation/time_ephemeris_port/CONTRACT.md). The first owned
commit, `f4e4e0e`, predates production edits. No parallel immutable-image IFTE
implementation remains.

| Responsibility | Canonical implementation and owner |
| --- | --- |
| IFTE headers/constants, record reads, wrappers, overlapping output order | `ifteph.rs::IfteEphemeris`, owned by `ClockSession::ifte` |
| Polynomial arrays/counters and selective initialization | `InterpolationInfo`; distinct process and direct-caller instances |
| Fortran filename mutation, byte/numeric readers, native-order record int | `read_fortran.rs::FortranReader`, separate `ClockSession::fortran` stream |
| FB content identity, retained coefficients and record | `ClockSession::fb`; candidate cache committed only after successful computation |
| Full signed-count pulsar/observation driver | `tt2tdb::tt2tb` over canonical `PulsarContext` |
| Both existing frontend correction passes | `form_bats` seeds canonical fields and calls `tt2tdb::frontend`; full driver and frontend share `compute_observation` |
| Resource hashing and provenance | `runtime::open_resource` hashes, rewinds and transfers the same File; metadata export uses consumed resources, without a second getter |
| Source remainder and existing residual caller | `tempo2_util::fortran_mod`, checked i32 quotient with Rug divisor; `form_residuals` propagates the error |

`with_resources` takes one outer process mutex and passes mutable borrows; time
readers do not inherit an unrelated terminal clock fault. IFTE and Fortran
streams can be live together. Full tt2tb initialization/close occurs even for
zero/negative loop counts; exact STL and bypass retention, independent selectors,
units, dilation and once-per-call deltaT retention follow the source. The strict
public input scope and JSON schema remain unchanged.

## Corrections and exact comparison disposition

Three isolated native builds apply the corrections cumulatively. Each executes
all 97 histories, preserving the original V recording. `corrections.py` compares
the complete raw original and successive corrected observations with recursive
type/member checks. Path transport replaces only the known equal-length fixture
root, including its actual stored bytes and diagnostic bytes.

| Added correction | Changed cases | Changed raw leaves |
| --- | --- | ---: |
| Reset IFTE twot alongside pc[1] | ifte_big, ifte_little, ifte_reinit, ifte_negative_step_retained, ifte_replace | 244 |
| FB actual-content identity plus record | fb_cache, fb_count_tail, tt_repeat_fb | 216 |
| Current-pulsar supplied CALCEPH value | tt_multi_calc_1, tt_mixed_calceph_backing | 132 |

Raw leaf counts include redundant `.hex`, `.bits` and `.value` presentations;
they are not counts of independent physical quantities. Full exact paths and
before/after values are in `outputs/native/correction-deltas.json`. All other
fields, membership, statuses and streams agree at each correction step.

The Rust/native comparator preserves ordered state fields, scalar types, complete
arrays/tails, knownness, ownership, byte storage, diagnostic bytes and statuses.
Binary64 values compare exact typed IEEE bits/classes. Rug128 production outputs
are rounded only by the test exporter to IEEE binary128 (113-bit significand).
Redundant binary64 hexadecimal text and binary128 decimal text are removed in
both engines; no numerical field or tolerance is substituted. All 1,368 original
events remain present, including nine source exits per profile.

Exactly four terminal events have an explicit ownership projection:
`ifte_short`, `ifte_truncate`, `fb_before`, `fb_after`. Their native exit retains
a live stream; Rust releases the failed owner and reports `released-on-error`.
Only that terminal owner's position/eof/error observations disappear. Original
raw observations remain in the corrected-native artifacts. Saved old streams in
the probe are genuine caller ownership transfers and are closed explicitly.

Incomplete IFTE headers/constants and native incomplete record-int reads have no
valid source numerical oracle; Rust reports contextual boundaries. Bytewise EOF
continues to produce the recorded host all-ones classes. Actual conversion,
seek-product and slice bounds replace the old version/grid/file-length/minimum
coefficient restrictions. Backed beyond-end dates, small coefficient counts,
signed no-op counts and the defined negative-step cache skip are preserved.

## Arithmetic and focused evidence

The source remainder is now `a-(int)(a/p)*p` at Rug128 with a checked native-i32
quotient. Its tests cover ordinary results, positive-zero outcomes, infinite
divisor NaNs, undefined conversions and 0049's precision-boundary witnesses.
`fortran_nlong`, numerical backend, singular-mode order and numerical budgets
are unchanged. The four 0049 differences remain unresolved.

GCC16 O2 contracts specific binary64 expressions. `arithmetic.py` compiles
source-expression witnesses at O2 and with contraction disabled, compares 100
seeded inputs with explicit Rust mul_add, and records five actual separating
witnesses: polynomial recurrence, derivative recurrence, reverse sum, record
origin and dot product. Its saved assembly and full raw-source assembly inspection
identify the matching operand association. This is bounded expression evidence,
not a blanket FMA policy. Dot product's other current consumers pass the required
W and M1 regression gates.

Focused Rust checks cover a rational cubic reinitialization at zero argument
without clearing retained tails; same-path FB content edits; replacement by
identical bytes preserving warm nr==1; failed cache publication and recovery;
current-pulsar CALCEPH equivalence, including full-driver single/multiple runs
with recognized IF99 and FB90 selectors; all four incomplete record-int lengths;
IFTE byte orders, derivatives, failed headers,
actual record-conversion boundaries and recovery. The native controls include
all thirteen V behavior mutations: eleven unchanged, plus explicit reversions
of the FB content and CALCEPH corrections. V's original forced-FB-reload witness
is preserved; that mutation no longer separates the corrected fixture because
content changes already invalidate its cache.

## Ownership and unsafe inventory

The existing `stdio::Stream` is reused for all three reader families. Existing
fdopen/fread/fseek/fclose call blocks are byte-identical to the release. Y extends
the foreign declarations, adds fgetc and a grouped ftell/feof/ferror call block,
and adds a narrowly justified exclusive Send implementation. No Sync, cloneable
FILE, raw mathematical pointer kernel, packed cast or second FFI layer is added.
Each new unsafe site has adjacent necessity and ownership invariants.

The current production inventory has fifteen sites: seven blocks/declarations
plus Send in stdio, five clock-text sites and two SOFA sites. Actual isolated
thread handoff/read/EOF/status/close checks run in both profiles, alongside JPL's
existing descriptor/partial-item/fdopen-failure cleanup test. The staged R audit
checks unchanged old unsafe blocks and records the exact new stdio hash; it does
not pretend this newly extended file is unchanged. W's accepted safe observer
adapter is likewise distinguished from its unchanged SOFA call blocks.

## Reproduction and evidence ownership

All paths below are relative to the repository root. Generated artifacts remain
ignored beneath `validation/time_ephemeris_port/{build,outputs,work,w}`. To
reproduce on this host, retain the initial `build/freeze.json` and existing
accepted evidence; never regenerate the initial freeze after production edits.
Run these commands in dependency order:

```sh
python3 validation/time_ephemeris_port/freeze.py --check
python3 validation/time_ephemeris_port/arithmetic.py
python3 validation/time_ephemeris_port/native.py
python3 validation/time_ephemeris_port/corrections.py
python3 validation/time_ephemeris_port/mutations.py
python3 validation/time_ephemeris_port/stage_regressions.py
python3 validation/time_ephemeris_port/run.py --profile debug
python3 validation/time_ephemeris_port/run.py --profile release
```

Do not run native mutations and Rust V replay concurrently: both intentionally
use the same equal-length fixture paths. Original V/W/T/R runners and outputs
remain untouched. Staged routing diffs and source identities are in
`build/regressions/routing/`. Execute the staged W run.py for debug/release,
staged T run.py and supplement.py for debug/release, and staged R verify.py.
For example:

```sh
for profile in debug release; do
  python3 validation/time_ephemeris_port/build/regressions/routing/terrestrial_resources_port/run.py --profile "$profile"
  python3 validation/time_ephemeris_port/build/regressions/routing/pulsar_lifecycle_port/run.py --profile "$profile"
  python3 validation/time_ephemeris_port/build/regressions/routing/pulsar_lifecycle_port/supplement.py --profile "$profile"
done
python3 validation/time_ephemeris_port/build/regressions/routing/observation_storage_port/verify.py
```

R dispatches the unchanged O, clock/table, JPL, M1a, M1b and M2 comparisons with
outputs routed under Y. Then run `python3 validation/time_ephemeris_port/verify.py`
for focused ownership/boundary checks and the final identity/preservation audit.
Cargo uses the existing `rust/.cargo-home` and `rust/target/gmp-cache`, locked and
offline. Each native launch verifies compiler/backend/runtime/header/source and
actual executable identity; each Rust launch verifies source/compiler/executable,
nonce and input hash before and after execution. CLI shortcut rejection removes
stale success pointers before argument parsing. Final direct receipts publish
only after controls finish.

The initial freeze protects 23,666 accepted files. Its SHA-256 is
`4eb65dd099a661dade3fd5e04d108396787e3ca35393dcbeb65a5ebb3a1def6e`.
Central, Z planetary and newly released AB propagation paths are evolving and
excluded; AB was absent when the original membership was frozen. X is protected;
AA remains unreleased. The two independent repositories retain their pinned
HEADs and clean status. No historical receipt is relabeled as a final-source run.

## Final execution results

All required checks completed with the final production sources and current
example assertion. Rust is 1.97.1 (8bab26f4f), aarch64-apple-darwin; native is
Homebrew GCC 16.2.0, `-O2 -std=gnu++11 -DHAVE_CONFIG_H`, binary128 profile.
Actual command, input, source, header, runtime, compiler and executable hashes
remain in each launch/build receipt. Final direct run directories are:

- debug: `outputs/debug/runs/2e37cc32ef2e3bcc`; executable SHA-256 `3d9e0b916a27e62837c586643b9602ee7be4256ebf8a924b469a3a5d8561b285`.
- release: `outputs/release/runs/9cd52b442b41cf25`; executable SHA-256 `1641417e41e29bca510890e06f1544e183ae883d57aac28ad968d853cc2a236e`.

Corrected native executable SHA-256: `381154e02ad36adfe2a3e9dcecca88924586cd16754551aa309f8357d3821914`.
Rust compiler SHA-256: `210df6794001b73ec3d453878707fa1e0bdcb63c427024a6e6574bbe5615a4da`.
Native compiler SHA-256: `b1f98e3089a1f165872b09e8628e4c92a315f5bc6e9a61ef85aa0c1c2887f363`.

| Gate | Observed result |
| --- | --- |
| Full V transfer, debug and release | 97 cases / 1,368 events / 9 native exits each; zero projected mismatches |
| Controls and independent checks | 50 controls and 36 analytical checks each; recursive type/order/knownness/tail/lifetime/status and actual identity challenges |
| Compiled behavioral mutations | All 13 rejected; actual fresh corrected-native mutation executables |
| W terrestrial resources | 130 cases / 2,256 events and 28 controls each profile |
| T pulsar lifecycle | 30 cases / 133 events / 5,329,542 records and 35 controls each profile; Q/helper/link supplements and two compiled mutations each pass |
| R and O | Exact storage/state gates and existing boundary/recovery checks pass in both profiles |
| Clock/table, glob/provenance and JPL | Existing direct and mutation gates pass; clock finite differences zero |
| M1a and M1b | Original gates, 4 ns response, all resource/CLI failure-recovery and debug/release byte-equivalence pass |
| M2 | Expected exit 1; exact four historical raw-order diagnostics, no new failure class |
| Focused ownership/boundaries, fmt, Clippy | Both-profile focused tests and isolated stream/lifecycle checks pass; all-targets Clippy with warnings denied |
| Preservation | All 23,666 files unchanged; independent native/A2 repositories still pinned and clean |

For M1b's unchanged A2 comparison, the largest absolute BAT error across the two
fixtures is `2.064668463849600e-20` days; TT-to-TB error is `1.29223e-33` seconds;
residual error is `2.07260262619050637216e-20` seconds. Every signed measurement,
original budget and 4 ns response result remains in
`build/regressions/R/O/regressions/m2/m1b/`. These existing tolerance-based frontend
gates are separate from Y's exact projected direct comparison.

A preliminary broad run stopped on the example's obsolete `IF99` error-label
assertion. The example now checks the actual `source_exit` / `IFTE_short_record`
cause and resource path, and the entire affected chain was rerun successfully.
A control transport missing its case ID was repaired before the final replays.
Development runs interrupted by harness edits were rejected by their source
identity checks; their success pointers were not published. The final receipts
below identify only the completed executions.

| Artifact (relative to Y directory) | SHA-256 |
| --- | --- |
| `outputs/delivery.json` | `b0c19132bf54bb3bea2bfb53a19b22ac1ec73c1b5e0ff0e93dab83ff634a9d5c` |
| `outputs/native/receipt.json` | `0d31eb92d436693c260d036dcb8ff45f7b8bbfa9cf428e0f71735014dd59971b` |
| `outputs/native/correction-deltas.json` | `feb44ec93f712436f62cdaed8a48b336701e461bf52806585d749f96f3dcba76` |
| `outputs/mutations/receipt.json` | `b30af661c456871447a0324051ee8aa26593efe98ff00578b929889bca566de7` |
| `outputs/checks/commands.json` | `29228b2f46acadeff42bc3cc06e2fb3053676b9e8b338544b833e9eb9f6a092b` |
| `build/regressions/R/delivery_checks.json` | `9a57202db9bb2b38cfbe003cad72272d617d2834d49f3e1a9da31e144e1b53c2` |

The delivery receipt also hashes the module/unsafe inventory, initial freeze,
arithmetic proof, staging diffs and downstream gate receipts. Native-stage
authored-source identities describe their actual earlier generator revisions;
they are not relabeled as the later final comparator revision.

## Limits and open gates

This is a bounded same-host transfer: macOS 26.7 arm64, GCC 16.2 O2/binary128 and
Rug128 Rust. Bytewise EOF narrowing, stdio visibility and native diagnostics are
host-qualified. Other platforms, concurrent writers during hashing, resource
exhaustion, broad concurrency and full scientific ephemeris validation remain
outside the evidence. Hashing consumes one opened file but does not promise an
atomic snapshot against concurrent in-place modification.

Native-invalid operations are classified in the frozen map; they are not run to
invent numerical goldens. The regular-file corpus does not independently provoke
fseek failure. CALCEPH values are supplied inputs, not a CALCEPH library transfer.
The IFTE irec-update omission remains, as does the source obsTermDot convention;
the dead repeated obsTerm scaling is not retargeted. General planetary transfer,
observer-coordinate transfer and broader PAR/TIM support remain separate.

M2 remains unaccepted with exactly its four original raw SVD retention/value-order
diagnostics. The two fortran_nlong and two fortran_mod 113-versus-128 boundary
findings under 0049 also remain unresolved. No sorting, tolerance widening,
precision switch or waiver is introduced.
