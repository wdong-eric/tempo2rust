# Assignment T: canonical pulsar state and lifecycle

Implementation under decision 0045, with verification routing corrected by 0050 and recursive type comparison corrected by 0052, from release `d3507274e9ee9a7c4cf2bb3b2f9132d282694ebf`. Central retains design and acceptance. This report does not accept T or M2.

## Delivered behavior

`TimingInput` owns one `PulsarContext`. Its canonical source record contains all 349 released pulsar fields, nested FitInfo/FitOutput/storePrecision/TZR state, O's parameter store and R's complete observations. Static declarations are generated from the pre-edit source-checked inventory; the runtime model contains no string-keyed field map or JSON state. Scalar/cell/byte `Option` values preserve unwritten storage. Pointer slots distinguish unwritten, known null and context-qualified owners. Counts remain signed and separate from capacities.

The twelve released initialization/allocation/destruction/copy/matrix routines and ten packed-polynomial helpers are implemented. Initialization retains the raw source's omitted `ne_sw_ifuncN` write, interleaves `dm_series_type` at its label-section position, replaces dynamic parameter buffers while retaining every embedded link cell, and preserves old owners. Full covariance allocations are dense contiguous 10000 by 10000 binary64 matrices, initialized to positive zero. Accessors retain dimensions and exact row stride. No active-fit-size substitute is used.

`copy_psr` follows source field/string/read order, copies observation records forward one at a time, retains the destination covariance and parameter owners, and uses the destination's retained `nfdJumps` for the inclusive FD loop. Fixed strings copy through the first known NUL and retain suffixes. Unwritten semantic reads, invalid indices, missing terminators, overlapping self-string copies, invalid releases and invalid dimensions return local boundaries with completed writes retained. Lifecycle operations refuse terminal reentry; diagnostic state remains inspectable. Fresh contexts remain usable.

Opaque payloads retain byte capacity and unwritten suffixes. Callback slots carry typed Rust function signatures and stable logical identities, with distinct unwritten/null/callable states. Fixture callbacks establish identity, without invoking physical models or claiming a C ABI.

## Ownership and representation decisions

| Source storage/operation | Rust ownership and tested meaning |
| --- | --- |
| Observation pointer | Context-qualified owner plus record offset; shared and both overlapping histories retain one actual backing allocation. Handles remain valid across moves and indices are never recycled. |
| Replaced parameter buffers | `allocate_memory`/`initialise_one` return the actual retired `ParameterStore`; moving its backing retains allocation addresses. Embedded links are copied as values/knownness into the new current metadata. Dropping an unretained bundle reclaims the source leak. |
| Matrix pointers | Typed side metadata replaces the native hidden size_t header and row-pointer allocation. Full data backing is contiguous. Source copying retains destination identity. |
| Independent Clone | Clone `PulsarContext` or `TimingInput`, not an individual source record. The entire graph is independent, with internal aliases preserved. Matrix data may share immutable backing until a write detaches it. Released identities remain released; foreign handles are not remapped into valid local owners. |
| Constraint/opaque owners | One context owns their complete byte buffers. Source null assignments retain former allocations until explicit release or context Drop. Payloads need not be strings. |
| Destruction | Observation, covariance, all 2000 constraint slots, then parameters, in source logical order. Release traces observe Rust logical release, not native allocator calls. Grouped owned parameter cells reclaim otherwise unreachable label tails on Drop, continuing O's accepted departure. `ToAextraCovar` is not freed by `destroy_one`. |
| Diagnostics | Existing process sink and warning/version history are reused. Lifecycle logging does not reenter or clear a terminal clock calculation. Native layout/address messages have an explicit comparison projection. |

Raw `char` storage is retained as byte values; the currently transferred char controls use source equality tests. The owned model does not promise native object layout, arbitrary foreign-pointer compatibility, shallow C-struct ownership or plugin callback ABI.

## Current caller and module map

The strict frontend still parses files in the established order. Parsed temporary O/R values are consumed by canonical construction. The bounded caller supplies one pulsar, observation capacity equal to supplied records, ignored fullSetup=1 and noWarnings=1. Original lines/tokens, declarations, default provenance and uncertainty metadata remain presentation metadata. No parser directive or physical model was added.

`fit_controls()` returns a detached owned snapshot from canonical `fitMode`/`nits`. Existing numeric and observation inspection is derived from the same context. `source_pulsar()` is read-only. The CLK/EPHEM profile checks read canonical values after declaration checks, and the supported TCB profile checks canonical units. Newly mapped PSRJ/CLK/EPHEM/TZRSITE strings enforce source byte capacities, including multibyte boundaries. Public result JSON membership and numerical expressions remain unchanged.

The owned ClockPulsar duplicate is removed. `toa2utc::ClockPulsarView` borrows canonical fields. The existing single-observation adapter explicitly supplies nobs=1, noWarnings=1, TT(TAI), and no SAT jumps, without allocating a pulsar per TOA. Direct fixtures seed their previous source values explicitly. Lazy string reads and early clock branches are retained.

| Source / responsibility | Rust counterpart |
| --- | --- |
| `tempo2.h` pulsar/nested types/constants | `src/pulsar.rs`; existing `src/tempo2.rs` O store and `src/observation.rs` R records |
| `initialise.C` capacities/labels | `src/initialise.rs`, reusing O's capacities and ordered labels |
| `initialise.C` whole-state order/lifetimes | `src/pulsar_lifecycle.rs` context methods and source-ordered writes |
| `tempo2Util.C::copyPSR` | `src/pulsar_lifecycle.rs::copy_psr`; `copyParam` remains `src/tempo2_util.rs` |
| `TKmatrix.C` twelve-routine subset members | `src/tk_matrix.rs` dense allocation/accessors; context allocation/release wrappers in `src/pulsar_lifecycle.rs` |
| ten `t2fit_nestlike.h` helpers | `src/t2fit_nestlike.rs` |
| strict loader and bounded consumers | `src/input.rs`, `src/lib.rs`, `src/read_parfile.rs`, `src/form_residuals.rs`, `src/form_bats.rs`, `src/t2fit.rs` |
| borrowed active clock bridge | `src/toa2utc.rs`; lifecycle diagnostic access in `src/clkcorr.rs` |
| complete typed evidence visitors | `src/pulsar_probe_fields.rs`, `src/pulsar_lifecycle_tests.rs`; generators under `validation/pulsar_lifecycle_port/` |

No new unsafe block, numerical dependency, tolerance, source expression, fusion policy, A2 initializer patch or SVD sorting workaround was introduced. Current module hashes and all unsafe locations are in the local `modules.json`; unchanged unsafe-containing files are checked against the pre-edit freeze.

## Reproduction and evidence

From the workspace root:

```sh
python3 validation/pulsar_lifecycle_port/verify.py
```

This runs formatting/Clippy, corrected S comparison and controls in debug/release, Q and supplemental native witnesses, the isolated diagnostic-state check, then T's `regressions.py`. That caller stages R's verifier beneath T and R dispatches O and its unchanged clock/glob/provenance, JPL, M1a/M1b and complete M2 gates. The runners use the existing crate-local Cargo/GMP caches, locked offline dependencies, C locale and serial ordinary tests.

Decision 0050 supersedes 0048's proposed exception. The live `validation/observation_storage_port/run.py` is restored byte-for-byte to accepted revision `04918d5`, SHA-256 `7d6424654a666d3852255bd9264ba0fbb88ba3c6e57f7bf717da4087a6b10529`. All 13 top-level R harness files are copied to T's ignored `routing/observation_storage_port/`. Only the copied run.py's ROOT and OUT assignments change: ROOT names this repository and OUT names T's `R/` output directory. Reversing those substitutions must recover the original bytes. All other copied files, including comparison code, fixtures, projection, membership and expectations, remain byte-identical. Exact accepted and staged identities, the unified routing diff and before/after preservation checks are recorded under `routing/`. No original R source or accepted evidence is written during verification.

The executed staged run.py has SHA-256 `7b0a951a4429b16ff594276a8c7978d8fa9f9ce837356094e91f186f8c2f32f8`, distinct from the restored original above. The one-line unified diff is `routing/run.diff`, SHA-256 `10d9b428e916fb4a6c2c17b3e327c9192c72d28713d694979aae245af59efb01`. R's copied `verify.py` and `native.py` remain identical to revision `04918d5`, respectively `e3e56140a9b06635e9d3fe07873e33c0a1bd87e6c1171b614e901aede9c29866` and `d10a35686b2a6cf0629b413d52027cf5608f7b91a326ee1fb51cb61f0680d290`. T checks these accepted identities before staging and checks every original and staged file again after the regression chain.

Individual gates:

```sh
python3 validation/pulsar_lifecycle_port/run.py --profile debug
python3 validation/pulsar_lifecycle_port/run.py --profile release
python3 validation/pulsar_lifecycle_port/supplement.py --profile debug
python3 validation/pulsar_lifecycle_port/supplement.py --profile release
python3 validation/pulsar_lifecycle_port/regressions.py
```

The original freeze is not regenerated. `freeze.py` refuses overwrite. It preserves complete field/type/extent/knownness/ownership and routine-body maps, current caller sites, S/Q membership, raw source hashes, baseline Rust hashes, both nested-repository identities and 8,141 protected-file hashes. U/V directories and concurrent central documents added after the freeze are outside its original membership. Raw pinned Git blobs are authoritative; clean checkout bytes can differ because of Git `$Id$` expansion.

Local evidence root: `rust/verification/pulsar_lifecycle/`. Its `.gitignore` keeps all reproducible payloads, native staging and logs out of Git. The exact pre-edit maps are under `freeze/`; complete challenged per-case output is under `debug/` and `release/`; native helper/link runs and compiled mutations are under each profile's `native/`; required regressions are under `R/`. `commands.json`, `artifacts.json`, `modules.json` and `delivery.json` bind executed commands, source/compiler/runner/input/executable identities, artifact hashes and outcomes. No generated JSON/JSONL, compressed evidence, binaries or raw source staging is committed.

The entire completed run using the superseded 0048 routing is retained under `prior_0048/`. All 518 indexed artifacts preserved their hashes during relocation, and its historical delivery receipt remains `a08aecb21ff6470d2523fddd33bbd2c1e62c02fbf7df017c75d238e9e5d85c98`. Absolute paths inside those receipts describe their original execution locations. That bundle is historical evidence, excluded from the current artifact index; the original pre-production freeze remains unchanged.

The pre-0052 delivery is separately retained under `prior_0052/`, with all 531 indexed artifact hashes unchanged and delivery SHA-256 `ac7b19e79ad308f978ace5f785371224dc3e0b6a66211ce37926b7a1a78708f5`. Its added reproducer receipt records all three malformed nested counts being accepted before correction. Central's independent pre-correction evidence remains untouched under `decisions/evidence/pulsar-lifecycle-port/build/pre0052/`. Both historical bundles are excluded from the current artifact index.

The first corrected invocation passed both S/Q profiles and all 35 controls, then stopped at R's preservation gate because central's 0053 notice changed historical decision 0008. The failed commands/logs and restoration evidence remain under `failed_0053_policy_gate/`. Central restored 0008's original SHA-256 `2c5f81b82c43105fcd48b62139f1d78f2dc7c4089055ea162e9feef82ff4e12c`; neither R's manifest nor its checker changed. The 0053 policy remains effective separately. An independent O/regression attempt then reached the expected numerical results but failed its final immutable-output check because T added `policy_restoration.json` during that run. Its complete output and exact added-file diagnosis are also retained there. These were preservation failures, not production or numerical failures; the final standard-verifier run passed sequentially with no concurrent evidence annotations.

The S comparison retains all 30 cases and 133 ordered events. Only four native ABI profile fields (sizeof/layout/significand presentation) are separated from typed state comparison. All source state records, fixed tails, owner offsets, knownness and action boundaries remain checked. Every exposed full covariance visits all 100 million cells and all 10,000 actual row offsets. Finite binary128 representation is compared exactly, without decimal roundtrips or tolerance; additional Rust decoding/assignment checks cover subnormal values, signed zero and nonfinite classes.

Under 0052, `compare` visits every dictionary/list element and checks each scalar's exact Python type before value equality. The former container-equality shortcut could accept nested `1`, `true` and `1.0` interchangeably. The normal controls now include the unchanged projected `matrix_1_1` case and three mutations to its profile `MAX_OBSN` record: `count_boolean`, `count_float` and `run_count_boolean`. The independent central reproducer rejects the first two at `$[0].state[2].count` and the third at `$[0].state[2].runs[0][0]`. The source case remains SHA-256 `461e77b0d9032cf999641abca44c5903fd31346dff97c2c7bc8fa4b5b0b9256b`; no scientific value, projection or accepted reference changes. The corrected runner SHA-256 is `552357415e5fd31b081c65315bd18e5c3cc48b8e251ba3edeac816955a28887d`; `type-equality-reproducer.json` records the independent normal-comparator rejections.

Q coverage consists of its nine accepted helper actions and initial packed value, plus defined native witnesses for all 64 valid generic shifts across five packed inputs, clamp extremes and additional encode/setter inputs. A native witness seeds only two embedded link cells before allocation, then observes those cells after allocation and initialization. Two compiled reset-link mutations produce actual state differences at both observation points. Comparator controls reject missing/permuted membership, hidden knownness, lost tails, owner/alias changes, wrong FD bounds, callback/payload changes, full-matrix changes and stale identity/challenge/input evidence. Rust checks separately cover lifetime, boundaries, independent clones, canonical consumers and string capacity.

## Verification status and limits

The complete sequential verifier exited zero on 24 October 2026 after the 0052 correction, using the restored accepted R sources and T-owned staging. Its recorded repository HEAD is `ded9e2bbbdc31d30ed78ac47ab7bd0f237a4da99`; source-byte checks bind the run independently of concurrent commits. T's implementation commits are `08eeae3` (freeze/generator), `c6de1fc` (canonical state and consumers), `aa29269` (diagnostic-state isolation), `2887220` (harness and native witnesses), `8daa4aa` (0050 restoration and disposable regression staging) and `804d9fd` (0052 recursive scalar-type comparison and actual-case controls). The original 0048 routing in `2887220` is reversed by `8daa4aa`. All 51 Rust source/build inputs and both Rust test executables are byte-identical to the preserved pre-0052 run. This report update is documentation-only.

| Check | Final observed result |
| --- | --- |
| Formatting / Clippy | Pass; Clippy all targets with warnings denied. |
| Ordinary Rust tests, each profile | 37 library tests, seven CLI tests and one numeric test passed; ten explicit probe/process tests ignored by the ordinary suite. Required ignored probes are launched separately by the harnesses. |
| Corrected S, each profile | All 30 cases, 133 ordered events and 5,329,542 typed records match exactly; 35 controls pass (two positive controls and 33 required rejections), including all three nested scalar-type mutations. |
| Q / supplemental witnesses, each profile | Nine accepted Q actions plus the initial value, 18 helper rows, 320 valid-shift rows and eight clamp rows pass. Both compiled link-reset mutations change observed allocation and initialization state. |
| Independent diagnostic state, each profile | Explicit isolated test passes after a terminal clock fault. |
| R, each profile | Three direct cases, 95 observation fields, four additional histories, 15 boundaries and 39 controls pass. |
| O, each profile | Ten cases, 32 events and 30 controls pass. |
| Clock / glob / provenance / JPL | All required gates pass. Clock comparison has zero finite difference; JPL and comparator mutation gates pass. |
| M1a / M1b | Pass, with M1b exercised through the complete M2 run. |
| M2 | Completes with exit 1 and exactly the four historical failure strings; no new failure or acceptance. |
| Preservation | All 8,141 frozen protected files unchanged. The restored accepted R runners and all 13 original/staged harness files pass exact identity checks. Both independent repositories remain clean at their pinned HEADs. All three existing unsafe-containing Rust files are byte-identical to the freeze. |

Rust toolchain: `rustc 1.97.1 (8bab26f4f 2026-07-14)`, host `aarch64-apple-darwin`, LLVM 22.1.6. Compiler SHA-256 is `210df6794001b73ec3d453878707fa1e0bdcb63c427024a6e6574bbe5615a4da`; Cargo SHA-256 is `7672ead309d505577c018fff2cafb3433601f073e38cbe87359ac1f7b944bbf5`. Supplemental native builds use `/opt/homebrew/Cellar/gcc/16.2.0/bin/g++-16`, SHA-256 `b1f98e3089a1f165872b09e8628e4c92a315f5bc6e9a61ef85aa0c1c2887f363`; its recorded compiler-backend SHA-256 is `9d8bd8a5840699af210fd5f46e743031005a7a5bb8bc00080b99c02fbcd0a62b`.

| Executed artifact | SHA-256 |
| --- | --- |
| `target/debug/deps/tempo2_rust-ea61be09d25d8f48` | `e0e6558a0bf3dc1edfea7f794736f9279260bd2beeb07a0fc1320316cb710d08` |
| `target/release/deps/tempo2_rust-de1e77b2b99ef1f6` | `a14c927ef82f6000449959c928726d90e176917c21d9635237097d0f1587146c` |
| `verification/pulsar_lifecycle/debug/native/ordinary/probe` | `eabceb4b31a7c38e81ece1591b24cefcf6468cbbd90b9a3f223e8f67d204e86a` |
| `verification/pulsar_lifecycle/release/native/ordinary/probe` | `5257404de978c85b29a9a39e472773f67f60b73f981768ab9c3bc3dc4bda7fb1` |

The following paths are relative to `rust/verification/pulsar_lifecycle/`. These receipts carry exact source/runner hashes, case membership, fresh challenged input/output hashes, compiler/runtime dependencies and actual executable identities. Native mutation identities and their changed state witnesses are retained in the helper receipts. The artifact index binds comparison, Rust-output and regression receipts; native staging/build directories are excluded from that index and retain their own identity and launch records. Maps and receipts remain ignored local artifacts.

| Local artifact | SHA-256 |
| --- | --- |
| `freeze/field-and-operation-map.json` | `2884e63bae35a3159052055f91c13b960c0fa1605279b5b27950ff83799dfe5f` |
| `freeze/caller-map.json` | `c2ea0c421be772e4f0fb379698d5ef3d4b2f21bb6ca62fc019f2400a3626a6f0` |
| `freeze/baseline.json` | `40b10e54ab938b3af655c3bcef315a6c1032cc67833b2af36c074d2d77bdb004` |
| `freeze/membership.json` | `44e73ce0f8c91386da7ab3719f78db935451eba8db4ff98cee75e9d0fca7f7e8` |
| `freeze/protected.json` | `c726022712e180bcd3e830db3781b348b7d2e0b13f15cdfe9e266ffbb807bc8b` |
| `debug/direct.json` | `fb93bcf5b2dab225bb6c495641f0c89fabae7018530152051f09737f5b917648` |
| `release/direct.json` | `dd5c58a9d968d65785d96ce2d48436cb1a2ac7077cb281bcc4786be3945a2660` |
| `debug/helpers/receipt.json` | `0b4b083b934edb4f6725699e0bbefc9ff9399615a5c0ba8428fdd7aece070630` |
| `release/helpers/receipt.json` | `0ab531cd03f58274cdd965015dddeb21340506ba545d8ee0d49ec47a231eb4ae` |
| `R/delivery_checks.json` | `ecaf60eaf094560351b4517054b2b601bc959dcb012539c438839501fd586b95` |
| `routing/identity.json` | `32ac144e647a44c2c8c03df0a84d560f8c48de520fa9072517178cca8819af17` |
| `routing/run.diff` | `10d9b428e916fb4a6c2c17b3e327c9192c72d28713d694979aae245af59efb01` |
| `routing/receipt.json` | `5e05f6bd127ab1d3e03d9c6716da107466a748a8a6af588e380ce95f1cf2d609` |
| `type-equality-reproducer.json` | `aae23f155ae3ddaba87b5143cf1baf282a9b812b7742f0405ef8f5e5d2bd2a71` |
| `commands.json` | `f6cdfb807724d88e80f0607df9d031f61f06c11357c530a8f971de52b19046f0` |
| `modules.json` | `19cca83788ff7073c9388e72e456278e0133ccac0a26afad6dfcbd41dc4a6d1c` |
| `artifacts.json` | `0a017b36e387c6e1a521e36b6147e77642b8ed3542324dcbdf8cfb5439b718d6` |
| `delivery.json` | `5ba8eae0869f9bae74e07cf25e4dc0b6a97e9f9f60fa8392b4c358e43fd719d3` |

M2 remains unaccepted. The complete strings in `R/O/regressions/m2/open_gate_failures.json` and `delivery.json` compare equal to the original `rust/verification/m2/open_gate_failures.json`: debug and release each retain the dependent-column raw retention-order mismatch `[True, False] != [False, True]` and the singular-value-index-zero difference of `3.464101615137754594384695270973822809990706901186851371815329282255643829069923` against threshold `3.46410161513775459438469527097382281E-10`. No ordering repair or tolerance waiver is part of T. Central acceptance of T remains pending.

This is storage/lifecycle implementation evidence, not validation of deferred physical callbacks, full native PAR/TIM/preprocessing, bootstrap, additional fitting/noise algorithms, another platform, allocator exhaustion, concurrency stress or performance. Fallible reservation and dimension arithmetic do not establish recovery from every OS OOM. Native invalid-pointer/unwritten-read experiments remain unrun; Rust boundaries are not native goldens. Native successful destruction and Rust logical release traces do not measure the same allocator event stream.
