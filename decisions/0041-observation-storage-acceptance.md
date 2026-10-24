# Observation storage Rust acceptance

Central decision, 24 October 2026. **Accept Assignment R's complete observation-storage foundation and integration at delivery `04918d5`; close R's Rust ownership window.** Production and focused tests are `b7a4d17`, the verifier and authorized O routing changes are `c1498e2`, and pre-edit projection freeze is `624c1bf`. This accepts contract 0037. It does not accept full pulsar state, source readers/preprocessing, M2 or the complete core transfer.

## Accepted source behavior and integration

Central independently checked all 95 fields against raw `tempo2.h` at `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, including names, order, source types and every fixed dimension. The canonical record contains 20 Rug cells, 354 binary64 cells, 87 signed i32 cells, one signed i64 cell and 4,121 byte cells, including nested clock corrections: 4,583 logical cells. Source signed counts remain separate from capacity. Rust layout is not presented as the native ABI.

Native observation allocation is unwritten. `None` records that state per scalar, array cell and byte. Owned cloning retains initializedness, precision, signed zero/nonfinite classes and complete tails. Through-NUL reads, byte holes, missing terminators and capacity failures remain distinct. Source-operation reads validate only the cells actually consumed and retain completed writes at the declared safe boundary; no fabricated native default is introduced.

TimingInput owns one canonical observation vector and separate original-text provenance. The bounded frontend supplies only SAT, frequency, uncertainty, filename and site. Evaluators, fitting weights and site selection use source fields; immutable timing results do not populate unrelated input BAT/residual cells. `observations()` returns the authorized detached snapshot, with existing JSON membership and values preserved. Filename capacity is enforced in bytes, including the tested 500/501-byte ASCII and multibyte boundaries.

The duplicate clock observation/correction owners and obsolete byte-copy helper are removed. Existing timing and direct-clock callers explicitly seed their previous correction buffers and controls. The accepted route/cache/calculation behavior remains intact. Early clock identity returns do not read irrelevant SAT/site state. Active toa2utc retains the source jump-index mismatch, count increment, extra shifted slot and scalar-before-byte writes. Its strncpy path consumes at most 31 bytes, pads only after an initialized NUL and retains byte 31. Signed negative counts preserve the source's empty-loop behavior; terminal-session failures retain the observed partial state.

R introduces no unsafe block, dependency, arithmetic expression, numeric parser, precision/fusion policy or tolerance change. Central checked all twelve existing unsafe sites in stdio.rs, clock_text.rs and get_obs_coord.rs. stdio.rs and get_obs_coord.rs are byte-identical to the baseline. All existing unsafe bodies are unchanged; clock_text's sole change is deleting its obsolete safe copy helper. Cargo.toml, Cargo.lock and build.rs are unchanged.

## Independent verification and reviewed regressions

Central ran the delivered exact direct gate freshly in **debug and release**, relocating only output/native-build roots to a temporary directory. No fixture, comparison rule, expected result, scientific value, source routine or tolerance changed. Each profile passes:

- Three complete native observation assignment cases and four accepted M observation-only histories.
- Fifteen safe boundary/prefix cases, including partial initialization, through-NUL behavior, count/capacity limits and terminal reentry.
- All **39 controls**: one accepted positive and 38 rejected structural, numerical, initialization, identity and launch/process failures.
- **22,449 exact finite comparisons**, 103,938 field visits and 439,192 scalar leaf comparisons; these overlapping traversal counts are not distinct physical cells.
- Five focused Rust tests for ownership/precision, operation-local boundaries, frontend capacity, canonical-state/serialization/fitting integration and terrestrial field consumption.

Every fresh Rust and native state equals the delivered data after removing only the launch token and input-file hash. The Rust test executable hashes match the delivery exactly: debug `a633689e4b650d0edd30fb55558f516015b45fb6f2b01b7f1267f2ab065aaa04`, release `71623cbf4e17d52b9f7c718fe33ee3c1f0a41861c964f62ab4869ca590604d74`. Independently relocated native binaries also match: debug `2c70a3f6408bf2370476c72b17d7daec8c92aa674363114f1d1d60792bbc196a`, release `403f6823ffc9d55094a47477189e95914817e8dd82488caa771998f7a69de7a0`.

Central verified 246 sealed artifacts, fifteen source/projection hashes, all 62 direct executed-source identities, 39 module hashes and 1,964 protected local files. Both nested repositories remain clean at their pins. The authorized O verifier differences change only output/build roots; O's fixtures, comparator and controls are unchanged.

Formatting, Clippy, ordinary debug/release tests, O's exact storage gate, clock/glob/provenance, JPL and M1a/M1b pass in R's full run. Central checked their source/artifact identities, command exit codes and logs, including 28 library, seven behavior and one reference-contract tests per ordinary profile. Current regression CLI hashes agree with R's report. These full suites were reviewed from their execution receipts rather than independently repeated by central.

M2 completed with exit 1 and **exactly the same four complete historical failure strings**: debug/release raw retention order and singular-value index zero for dependent columns. No ordering workaround or tolerance waiver is accepted. Decision 0012 remains in force.

The [central reproducer](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/observation-storage/central-review.py), log and [source/receipt checks](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/observation-storage/acceptance-checks.py) are tracked. The complete review receipt remains [local and ignored](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/observation-storage/central-review.json), SHA-256 `4b86b8c8d0f62d7be9e64178fc004a38113f74986d300266660020eae60b1fb8`, under decision 0039. Generated payloads are not re-added to Git.

## Scope and next ownership

This acceptance covers observation storage and its existing bounded consumers. It does not establish general PAR/TIM parsing, preprocessing, complete copyPSR, whole-pulsar initialization/destruction, additional timing models, linked arithmetic, source numeric parsing, allocation failure, concurrency stress, performance or another platform.

R's delivery, projections and local verification evidence are now read-only inputs. A subsequent Rust owner needs a new central contract; no new production assignment is released here. S's lifecycle reference at `16737d2` and Q's specialized-PAR reference at `bb7e629` are delivered but unaccepted. Review S next to define the remaining pulsar-state foundation, then use accepted K/N/P/Q evidence for source readers and preprocessing. Central retains shared design and milestone acceptance.
