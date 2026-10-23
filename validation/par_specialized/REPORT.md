# Assignment Q: specialized PAR native reference

Delivered for central review on 24 October 2026. Harness, fixtures, source/caller contracts and verifier sources are committed at **b61896f3a3274b08d7ab9c1b770a23a17b92977e**. Execution source hashes match that commit. Q owns only `validation/par_specialized/`; no Rust API, numerical backend, tolerance or milestone decision is changed. M2 and its four unresolved exact-order failures were not rerun or reclassified.

The governing release is ad6fbf692751b39c67713fd5a446c675a51db665 / decision 0035. Decision 0039 supersedes its generated-evidence commit requirement: **all generated JSON, compressed state, profiles, coverage, raw receipts and mutation outputs remain local and ignored**. Only source, authored fixture definitions, contracts and this report enter Git. Other assignments and both nested repositories are preserved.

The corpus reconciles K's 339 top-level / 559 condition entries to the 116 released deferred branches plus D029's FD subtree/context: 117 entries, 207 selected condition entries, 308 projected fixed fields, all allocated parameter metadata/buffers, and ten subtraction helpers. `inventory.py` reads the released manifest and accepted K metadata from immutable Git blobs, verifies the release hashes, and derives the local complete inventory. The actual native routines and dependencies are unchanged raw blobs at **fda6ad4a9753ad7dc5d291406f2f310a9161423f**; no A2 engine or initializer correction is linked.

| Executed check | Result |
| --- | --- |
| Fresh normal replay | **107 cases / 182 declared actions / 2,252 ordered events, exact** |
| Declared classes | 67 defined controlled-state observations; 31 conservative host-diagnostic observations; 9 normal source exits |
| Integrity controls | 16 identity/build controls and 11 comparator/launch controls pass, including fresh positive build, changed executable/forwarder, actual changed source/config/runner/resource, stale/missing launch and stale success removal |
| Compiled mutations | **14/14 detected through native state, stream/event or status witnesses**, independently of identity rejection |
| Separate coverage replay | **107/107 complete observations equal the normal build** |
| Semantic/raw archive checks | 17 independent semantic witnesses pass; both raw reference and replay archives reconstruct byte-exactly against their launch receipts |
| Protected evidence | 572 historical primary-repository files plus 852 canonical-source and 58 independent-reference tracked files retain their hashes; both nested repositories are clean |

The direct histories and real simple/full/global readers cover repetition and two pulsars, exact token/EOF/error/remaining-stream state at direct boundaries, field precision, initialized string tails and retained slots. Notable observed requirements are ordinary JUMP retaining SAT/error state, FD zero-index nonconsumption and PINT-format effects, lowercase substring precedence, independent DM/CM/SW counts, 11 constraints per DMMODEL_DM1, IFUNC_COV's overwritten terminator and unwritten final allocation byte, subtraction packing, RN logarithmic conversion, and shapelet warnings leaving caller coefficients intact. Full readers reset ordinary jumps/phase jumps/links while retaining FD and append counts. A later clock-chain definition with the same endpoints appends another cache entry; the first eligible cached chain remains selected across pulsars (`clkcorr.C:530–564,681–722`). Reverse definition order is also witnessed.

`PROJECTION.md`, `SOURCE_MAP.md` and `UNRUN.md` describe caller initialization, types/capacities, ownership, dependency and source-definedness boundaries. Caller seeds are explicit inputs, including naturally unwritten controls and prefit fields. No unwritten native bytes or invalid accesses are exported. IFUNC_COV is represented by its known initialized prefix and allocation/write extent, never strlen of the returned allocation. Raw finite representations and numeric classes are preserved; byte-exact native replay does not create a new Rust NaN-payload acceptance rule.

Measured coverage is source-attributed per case using gcov16. All 117 selected entries have a positive first-scan witness. Their regions reach **623/633 executable lines** and **812/1,184 compiler branch outcomes**. The ten unexecuted selected lines are 1182–1183, 1186–1187, 1202–1203, 1219–1220 and 1236–1237: DM/CM/SW error statements reachable only after a preceding invalid append. Those paths remain unrun. Entire readParfile.C coverage is 1,082/1,755 lines, 1,538/3,659 compiler outcomes and 9/10 emitted routines; fixed-format getValue is not entered. The helper header reaches 31/31 lines and 8/12 compiler outcomes; nine emitted helper functions are entered, with the tenth declared helper (flags decoder) fully inlined in direct witnesses. Compiler outcomes, exceptional edges and line hits are not path completeness.

An initial all-O0 coverage attempt exceeded the 90-second caller limit during the long scalar case. The final coverage build compiles unchanged native translation units at `-O0 --coverage` and the serialization caller, including direct inline witnesses, at `-O2 --coverage`; all per-unit commands are retained. The normal scientific goldens stayed byte-identical through this build refinement and the later tracking-policy adjustment (`coverage_build_refinement.json`, `tracking_refinement.json`).

Reproduce from this workspace with both nested Git checkouts, their histories, Python 3, Homebrew GCC/gcov16, libquadmath and the recorded SDK/runtime available:

```sh
cd /Users/Eric.Dong/Downloads/tempo2rust
python3 validation/par_specialized/protected.py
python3 validation/par_specialized/verify.py
python3 validation/par_specialized/coverage.py
python3 validation/par_specialized/run.py
python3 validation/par_specialized/analyze.py
```

Normal verification builds fresh and never rewrites the oracle. `verify.py` deliberately tests a failed normal run and removes its success receipt; the final `run.py` restores only a genuinely successful receipt. For a workspace missing **Q's local generated outputs**, explicitly run the following first, then check the deterministic artifact hashes below and run the verification sequence. This records native reference data and grants no Rust acceptance or tolerance change:

```sh
python3 validation/par_specialized/protected.py --record
python3 validation/par_specialized/run.py --record
```

Released K inventories/configuration come from commit 38d62af9c183a0bb3294638999ca63cb431fc046. The archived configuration text has SHA-256 `470c61b68a61504558e76ef772639e46b565a1e4500115d6e3fb6a71286dd69e`. Protection membership is anchored to the Q release tree, independent of current tracking; missing local historical generated artifacts are explicitly marked archived-only. Active sibling directories are outside that frozen membership; P's now-accepted preprocessing data also stays read-only and outside it.

Actual final profile: **g++-16 (Homebrew GCC 16.2.0) 16.2.0**, **macOS-26.7-arm64-arm-64bit**, C locale, measured 113-bit binary128 significand. Full source, transitive-header, compiler/backend, macro, runtime library, generated configuration, runner, command and executable identities are retained in local profiles and receipts. Every launch records its nonce, argv, input-file/action-stream hashes, process status and raw output hashes; input hashes are checked before/after use. Python used here is 3.11.2 (v3.11.2:878ead1ac1, Feb  7 2023, 10:02:41) [Clang 13.0.0 (clang-1300.0.29.30)].

| Executed identity | SHA-256 |
| --- | --- |
| Final normal executable | `b8038808babc3781d28c94f43566663b686f3d80e063a24b7b53eeb42826d3bb` |
| Final coverage executable | `af5e2e94b946dea0f50d99bd6ec31ca97cb1d02729bd2e275be5589184295fa7` |
| Final verifier executable | `7b51f5fbd59ae5dff7a3604d646f65aeb250a4ae372cf87f575561610d998099` |
| Compiler driver | `b1f98e3089a1f165872b09e8628e4c92a315f5bc6e9a61ef85aa0c1c2887f363` |
| Compiler backend cc1plus | `9d8bd8a5840699af210fd5f46e743031005a7a5bb8bc00080b99c02fbcd0a62b` |
| Actual libquadmath | `6f885df6ad8277edf614f0f5f7fbc6f8564a95b64679b88fb860382c9d1c0e9a` |

Deterministic local reference artifacts (relative to this directory):

| Artifact | SHA-256 |
| --- | --- |
| `cases.json` | `3d433a75a1d2946115e19975bb893b37f98ea32124c21c1773857cf7d3d89d31` |
| `inventory.json` | `d2f9a6a63b3bef11da3812cac9a02bfc9e95a08d14431739a2f2c3696d87aaf9` |
| `initial_state.json.gz` | `df3e00e4e3cf1e16f9e8a7747b6766f27ea1ab984cbe26cb91f6bd9a5995b52f` |
| `observations.json.gz` | `beb6122689951013cc26db58093d89d1fac51cca9cddc76faff65e29f48e5420` |

Current local execution records are `regeneration.json` (fresh exact replay), `verification.json` plus `mutations/` (all 14 distinct executed mutant identities and witnesses), `coverage.json`, `branch_coverage.json`, `entry_coverage.json`, `analysis.json`, and `delivery.json`. `reference.raw.json.gz` / `replay.raw.json.gz` index the corresponding `reference_raw/` / `replay_raw/` payloads and content-addressed `raw_initial/` lines. These remain on disk, with exact reconstruction checked before delivery. Runtime resources, generated headers, preprocessing output, compiler logs, link maps and binaries are under ignored `build/`.

Remaining limits are explicit in `UNRUN.md` and each selected decision's coverage ledger. Uninitialized optional jump tokens, failed local index/order/string scans, natural unwritten counter use, OOB writes, allocation/I/O failures and nonterminated-string overreads stay unrun. Ordered-shadowed alternatives and compiler exception edges are distinct from untested-defined variants. Host logger qualification from M is retained, including the diagnostic-bearing `clock_empty` and `shape_invalid_scale` exits. Full/global reader-owned FILE positions inside the loop are not instrumented; their boundary state and source loop behavior are complemented by the exact direct-dispatch traces. Special allocated constraint prefixes are directly witnessed, while mixed-reader files use ordinary constraints. Other hosts/locales/backends, arbitrary malformed data, exhaustive paths, physical GW/noise models, full preprocessing/TIM/fixed-format parsing, sanitizers and numerical timing/fitting validation are not established. System libSystem identity is represented by the recorded OS/dependency profile, not a separate dyld-cache file hash.

No new central decision blocks this reference delivery. A later Rust parser contract still needs source-backed safe dispositions for undefined accesses/local reads and unterminated constraint storage, while retaining deterministic parser defects. This report does not grant milestone acceptance.
