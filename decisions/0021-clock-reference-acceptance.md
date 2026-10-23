# Clock and table reference review

Central decision, 23 October 2026. **Accept Assignment J's executed reference evidence at `360cc038221b4bbc42a63d91ef07d7bc457d69de` for the declared macOS arm64 profile. Close J's current ownership window.** The nine unrun cases and the other limits below remain open coverage. This accepts a reference foundation, not a Rust clock transfer, complete validation of these source files, M2, or the overall goal.

## Evidence reviewed

The canonical pin remains `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; the independent A2 repository remains `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`. Both were checked clean. J compiled raw primary and dependency sources and used the A2 archive only for hash/symbol profile checks. Its corrected initializer and timing instrumentation did not execute. The sole primary observation change renames and forwards the table-value symbol to record actual arguments/results; its expression remains unchanged.

Central independently rebuilt J's numerical snapshot `d70b075c81fb9b5bf8aeaffce8abaf0783377d5d` in temporary storage. All **142 executed cases** (122 ordinary, 20 normal source exits) reproduced their complete records on two fresh executions each: **1,964 begin/end events**, comprising 972 completed actions and 20 terminating-action begins. All **19 artifact mutations** and **12 compiled source mutations** were detected, and the 27 behavioral checks passed. All nine withheld cases remained unrun. See [central replay](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/clocks/central-review.json); [the reproducer](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/clocks/central-review.py) writes only temporary build/evidence files and does not run withheld cases.

The final delivery adds measured coverage, the report and the reusable stable-identity gate; its numerical exporter, fixtures and goldens are unchanged. Central inspected that diff and directly exercised `run.compare_manifest` and all three native-identity controls. Changed compiler, libquadmath and observatory-dependency hashes reject. Rebuilt executable/link-map bytes remain recorded per execution without an inappropriate identical-binary requirement. The final `last_execution.json` is a successful check with current runner/verifier hashes and an executable hash matching the built probe. [Final acceptance checks](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/clocks/acceptance-checks.json) pin the reviewed artifacts and these checks.

The gcov evidence maps all 142 executed case identities to calls in **32 primary entries**: four table functions, four public clock functions, 18 helpers, three constructors, two lambdas and active `toa2utc`. Central checked the complete case membership and nonempty call maps. Read-only snapshots contribute some getter calls. The O0 instrumented build demonstrates reachability; the O2 build supplies numerical observations. Neither function nor line coverage proves all paths.

| Accepted artifact | SHA-256 |
| --- | --- |
| `REPORT.md` | `aef614d49f9673b546539823dc03b0d17995606282d84cb4e5bb9333c4047ce4` |
| `outputs/reference.json` | `9f189fd0765acaa9ee3eda0299993eb7d3fb1d13bf1f103aa688fa1ea165ccaa` |
| `outputs/isolated.json` | `742718f00e94aa576d3837283c80647e0bbd4aa8e33e70de0708e3d959d789a9` |
| `outputs/manifest.json` | `551df45df59ba8837c307d1bebc15a0d69e786dbdefed0bc655ae66f8ec31eb8` |
| `outputs/checks.json` | `d46eda1ba21830276b9b4bd7157fea8eca9659174cfe110921363b85642147ed` |
| `outputs/coverage.json` | `4521f0e7921019eb1f16f600779fe559ce1c1cc3dd808f69053a243b434a8672` |
| `outputs/identity_checks.json` | `2112cfd8e7edefc2975c60ec43fb5b68114ba3c33ca5d88f018629ad1c818f73` |
| `outputs/last_execution.json` | `183d03cc93cc6e057cefdcb8c06eb813037817eee5543875528ab5f64a750dcf` |

## What this establishes for the later transfer

Preserve the measured source behavior: skipped malformed table rows, retained order/duplicates/nonfinite values, endpoint proxies, duplicate-knot arithmetic, binary32 route weights and tie decisions, directed edges, explicit-chain basename precedence, fallback ladders, shifted binary64 query epochs and all initialized slot mutations. Do not restore the current Rust parser's stricter guards as fidelity criteria.

Clock index, loaded tables, cached sequences and warning history have process lifetime. A changed runtime root or file does not reset them. A file changed before lazy load can supply new samples while its original indexed edge remains. A loaded table survives later replacement/removal. Active `toa2utc` retains the `jumpSAT[l]` versus `jumpVal[k]` choice and its extra in-bounds shifted slot. These are compatibility requirements under decision 0008, not proposed fixes.

Keep the source's distinct string rules: clock-name `trim` removes only space, tab, carriage return and newline; numeric/header scanning follows its own C-locale rules. Do not infer one universal whitespace or Unicode-normalization policy from these fixtures. Input SAT is binary128, while routes and accumulated table queries narrow to binary64 as recorded. The observed epoch narrowing scales are not new numerical budgets.

The real PKS route has five components and ordered total `69.183999495293733 s` at the declared input. Resource hashes, component order and actual query epochs are explicit. This is a clock-stage observation, not a new full timing validation.

## Coverage that remains open

The eight invalid-access fixtures (`table_singleton_exact`, `table_singleton_nan`, three empty-table accessors, two overflowing headers and `toa_capacity_29`) and the bounded `weights_cycle` experiment were not run after the earlier automated risk flag. They have no observed sanitizer, signal, timeout or numerical result. This acceptance does not waive them, retry blocked experiments elsewhere, or silently approve a Rust disposition. Later safe bounds require explicit central source-based decisions; deterministic cycle behavior must not be replaced with a new routing algorithm.

Glob allocation/read failures, the invalid-predecessor safeguard and the public broken-cached-accumulator path remain unexecuted. A basename beyond the host's 255-byte limit was unavailable. General observatory/geometry/logging coverage is not established by selected dependency calls. EOP, weather, IFTE and other process-state differences remain open; F's historical inventory is unchanged.

Literal-disabled `tai2tt`, `utc2tai` and old `toa2utc` bodies remain inventoried as inactive in this pin/profile. Active `readJBO_bat` belongs to later input-format work. No source has been dropped from the overall transfer merely because it was not called here. Linux, other C libraries, concurrency and arbitrary malformed graphs remain unverified.

## Next ownership boundary

J's accepted evidence is now read-only unless central issues a specific correction. I retains the Rust window while addressing the final legacy-header boundary review; K continues parameter/PAR reference work. Central will release the clock/table Rust contract separately, including state lifetime, diagnostics, process-exit adapters, safe bounds and integration. M2's exact-order gate and all existing scientific budgets remain unchanged.
