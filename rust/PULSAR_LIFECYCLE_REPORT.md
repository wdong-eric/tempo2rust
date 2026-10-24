# Assignment T: canonical pulsar state and lifecycle

Implementation under decision 0045, from release `d3507274e9ee9a7c4cf2bb3b2f9132d282694ebf`. Central retains design and acceptance. This report does not accept T or M2.

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

This runs formatting/Clippy, corrected S comparison and controls in debug/release, Q and supplemental native witnesses, the isolated diagnostic-state check, then R's verifier with `TEMPO2_OBSERVATION_VERIFICATION_ROOT` beneath T. R dispatches O and its unchanged clock/glob/provenance, JPL, M1a/M1b and complete M2 gates. Only R's output-root selection changed; its comparison, fixtures, projection and expectations are unchanged. Decision 0048 approves that exact runner identity, `5f0cb7bfa0c2ebc8606c789e157ab96b7d5fa248d54703e01f9c5c7495e0c4d6`, which remains unchanged at delivery. The runners use the existing crate-local Cargo/GMP caches, locked offline dependencies, C locale and serial ordinary tests.

Individual gates:

```sh
python3 validation/pulsar_lifecycle_port/run.py --profile debug
python3 validation/pulsar_lifecycle_port/run.py --profile release
python3 validation/pulsar_lifecycle_port/supplement.py --profile debug
python3 validation/pulsar_lifecycle_port/supplement.py --profile release
TEMPO2_OBSERVATION_VERIFICATION_ROOT="$PWD/rust/verification/pulsar_lifecycle/R" RUST_TEST_THREADS=1 python3 validation/observation_storage_port/verify.py
```

The original freeze is not regenerated. `freeze.py` refuses overwrite. It preserves complete field/type/extent/knownness/ownership and routine-body maps, current caller sites, S/Q membership, raw source hashes, baseline Rust hashes, both nested-repository identities and 8,141 protected-file hashes. U/V's active directories and concurrent central documents are outside its membership. Raw pinned Git blobs are authoritative; clean checkout bytes can differ because of Git `$Id$` expansion.

Local evidence root: `rust/verification/pulsar_lifecycle/`. Its `.gitignore` keeps all reproducible payloads, native staging and logs out of Git. The exact pre-edit maps are under `freeze/`; complete challenged per-case output is under `debug/` and `release/`; native helper/link runs and compiled mutations are under each profile's `native/`; required regressions are under `R/`. `commands.json`, `artifacts.json`, `modules.json` and `delivery.json` bind executed commands, source/compiler/runner/input/executable identities, artifact hashes and outcomes. No generated JSON/JSONL, compressed evidence, binaries or raw source staging is committed.

The S comparison retains all 30 cases and 133 ordered events. Only four native ABI profile fields (sizeof/layout/significand presentation) are separated from typed state comparison. All source state records, fixed tails, owner offsets, knownness and action boundaries remain checked. Every exposed full covariance visits all 100 million cells and all 10,000 actual row offsets. Finite binary128 representation is compared exactly, without decimal roundtrips or tolerance; additional Rust decoding/assignment checks cover subnormal values, signed zero and nonfinite classes.

Q coverage consists of its nine accepted helper actions and initial packed value, plus defined native witnesses for all 64 valid generic shifts across five packed inputs, clamp extremes and additional encode/setter inputs. A native witness seeds only two embedded link cells before allocation, then observes those cells after allocation and initialization. Two compiled reset-link mutations produce actual state differences at both observation points. Comparator controls reject missing/permuted membership, hidden knownness, lost tails, owner/alias changes, wrong FD bounds, callback/payload changes, full-matrix changes and stale identity/challenge/input evidence. Rust checks separately cover lifetime, boundaries, independent clones, canonical consumers and string capacity.

## Verification status and limits

The complete verifier exited zero on 24 October 2026. Its recorded repository HEAD is `a738a8e4fc782f0f008755fc9ff1e62125de2acb`; source-byte checks bind the run independently of concurrent central documentation commits. T's implementation commits are `08eeae3` (freeze/generator), `c6de1fc` (canonical state and consumers), `aa29269` (diagnostic-state isolation) and `2887220` (harness, native witnesses and R routing). This report and the README are documentation-only additions after that run.

| Check | Final observed result |
| --- | --- |
| Formatting / Clippy | Pass; Clippy all targets with warnings denied. |
| Ordinary Rust tests, each profile | 37 library tests, seven CLI tests and one numeric test passed; ten explicit probe/process tests ignored by the ordinary suite. Required ignored probes are launched separately by the harnesses. |
| Corrected S, each profile | All 30 cases, 133 ordered events and 5,329,542 typed records match exactly; 31 controls pass (one positive control and 30 required rejections). |
| Q / supplemental witnesses, each profile | Nine accepted Q actions plus the initial value, 18 helper rows, 320 valid-shift rows and eight clamp rows pass. Both compiled link-reset mutations change observed allocation and initialization state. |
| Independent diagnostic state, each profile | Explicit isolated test passes after a terminal clock fault. |
| R, each profile | Three direct cases, 95 observation fields, four additional histories, 15 boundaries and 39 controls pass. |
| O, each profile | Ten cases, 32 events and 30 controls pass. |
| Clock / glob / provenance / JPL | All required gates pass. Clock comparison has zero finite difference; JPL and comparator mutation gates pass. |
| M1a / M1b | Pass, with M1b exercised through the complete M2 run. |
| M2 | Completes with exit 1 and exactly the four historical failure strings; no new failure or acceptance. |
| Preservation | All 8,141 frozen protected files unchanged. Both independent repositories remain clean at their pinned HEADs. All three existing unsafe-containing Rust files are byte-identical to the freeze. |

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
| `debug/direct.json` | `0689a88eaf9885e035b374b009913e5cd807a8a588dcb0b4904f3a9df43539d6` |
| `release/direct.json` | `7d946685b2b16bc91cca778c4dc2cc3c2cfd3b99df5022aa7c50f690d2b088bb` |
| `debug/helpers/receipt.json` | `ad8819a60b87caa518651fafc1e466c3c989ec139742794938d1da9e11007a11` |
| `release/helpers/receipt.json` | `5de02774b019d12dc63c39228b73e7acedf9a05ba99494e84e0cbb138646f49f` |
| `R/delivery_checks.json` | `d2010268a5e3ea26c4f03d1c2a2c57edfbb1ed37c67dd877f77caeb8a4451d1f` |
| `commands.json` | `9ac56d32c411130fd663185557229d5d564dcbfa8970a1193f56b41ec31ba175` |
| `modules.json` | `19cca83788ff7073c9388e72e456278e0133ccac0a26afad6dfcbd41dc4a6d1c` |
| `artifacts.json` | `8990d69bf7b4fbf30410fb19c4ae7c53e754481d41212961f667543738624d6b` |
| `delivery.json` | `a08aecb21ff6470d2523fddd33bbd2c1e62c02fbf7df017c75d238e9e5d85c98` |

M2 remains unaccepted. The complete strings in `R/O/regressions/m2/open_gate_failures.json` and `delivery.json` compare equal to the original `rust/verification/m2/open_gate_failures.json`: debug and release each retain the dependent-column raw retention-order mismatch `[True, False] != [False, True]` and the singular-value-index-zero difference of `3.464101615137754594384695270973822809990706901186851371815329282255643829069923` against threshold `3.46410161513775459438469527097382281E-10`. No ordering repair or tolerance waiver is part of T. Central acceptance of T remains pending.

This is storage/lifecycle implementation evidence, not validation of deferred physical callbacks, full native PAR/TIM/preprocessing, bootstrap, additional fitting/noise algorithms, another platform, allocator exhaustion, concurrency stress or performance. Fallible reservation and dimension arithmetic do not establish recovery from every OS OOM. Native invalid-pointer/unwritten-read experiments remain unrun; Rust boundaries are not native goldens. Native successful destruction and Rust logical release traces do not measure the same allocator event stream.
