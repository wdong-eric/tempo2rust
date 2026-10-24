# Pulsar lifecycle reference acceptance

Central decision, 24 October 2026. **Accept Assignment S's corrected native reference at `58ddc07`**, with executed harness sources at `6cc0fd0`. This closes the two exporter findings in decision 0042 and S's ownership window. It establishes the reference dependency for the remaining whole-pulsar state/lifecycle contract; it does not accept a Rust lifecycle implementation, physical models or M2.

## Accepted behavior and correction

The reference uses unchanged raw TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, the measured GCC binary128 profile and the pinned configuration. It inventories all 349 pulsar fields, seven nested types, 95 observation fields and twelve initialization, allocation, destruction/accessor and copy routines. Source-defined writes, explicit caller seeds and unwritten storage remain separate. No A2 initializer correction supplies the ordinary source.

The corrected exporter independently tracks embedded-link and prefit-buffer knownness. Repeated allocation/initialization retains seeded linkTo/linkFrom arrays, resets their counts and replaces dynamic buffers with fresh unwritten prefit backing. Natural first allocation still marks unseeded links unwritten. The first fully seeded snapshot now includes its initialized extra-covariance backing.

Central independently reran the exact original-bundle audit: **24,000 current-owner link records** now expose their retained values, and **four initial extra-covariance records** were added. All other field membership/order, initialized values, streams and statuses remain unchanged. The original 132-file bundle is preserved with its frozen manifest. The correction changes the observation harness, not native behavior.

The reference establishes natural/seeded/repeated initialization, ignored signed fullSetup/realloc arguments, valid owner replacement and destruction, real matrix dimensions/zeros/strides, and copyPSR's partial field assignment. The ordinary jump loop remains inclusive after copying nJumps; the inclusive FD loop uses the destination's retained nfdJumps. Shared and overlapping initialized observation buffers preserve forward-assignment history, including source-visible changes. Params, covariance, fit metadata and other omitted fields are retained. copyPSR is not a full clone.

## Independent central verification

Central rebuilt an exact source copy and passed **30 cases / 133 ordered events / 5,329,662 field records**, all **25 integrity controls** and **14 compiled mutation/witness runs**. These represent twelve distinct mutations; the two link resets each have allocation and initialization witnesses. Every mutation was additionally checked for an actual native state difference after the profile event. Both known-to-unwritten controls reject the previously hidden retention error.

The fresh run reconstructs and validates raw state and all 103 complete snapshot transitions, reruns the correction audit, and reproduces the delivered delta files and summaries exactly. All 142 current S source/evidence files outside build/cache directories remain unchanged. The 760 protected files retain their hashes and both independent repositories remain clean at their pins. All thirteen runner/generated-input hashes, current native profiles and all delivered mutant executable identities were checked against their actual files.

The fresh executable SHA-256 is `23ab7b994d3e99d17fe3ce63e85e359a98af27b93130ac9943977a1f9beb047f`. The copied probe/exporter retain identical bytes; only their included-file paths are rebound to the disposable copies. Every other stable profile field matches, including the complete raw macro digest. No scientific or state normalization changes.

Central verified the separately executed O0/gcov profile, ordinary-case agreement and source-attributed coverage accounting: all twelve routines entered, **702/711 executable lines and 236/256 compiler branch outcomes**. Accessors now have 106 calls each because the corrected initial extra-covariance snapshot observes them. Central did not rerun the instrumented suite. Full natural covariance exports read every cell of the actual 10000-by-10000 allocation; lossless runs retain the entire zero range. Source free order is inspected, while successful destruction/coverage does not measure each individual free.

Tracked reproduction/check scripts and logs are in `decisions/evidence/pulsar-lifecycle/`. Complete receipts remain local and ignored under 0039. The central execution receipt SHA-256 is `4635c935863ed0e0859d348876fe2e2faf80d027886d40bd8dbc742625585681`. No generated JSON is committed.

## Limits and next ownership

Allocator exhaustion, unsafe size arithmetic, unwritten/freed reads, invalid/null/interior/double frees, same-pulsar or overlapping strcpy and unterminated/over-capacity strings remain unrun. Zero-dimension matrix assertions are observed in separate terminating processes; aborted processes do not flush gcov counters. Native metadata layout/type-punning and debug logger varargs/pointer handling retain their host/ABI qualifications. Callbacks and opaque payloads establish storage/retention identity only, not invocation or physical fitting behavior. Other platforms, numerical backends, sanitizers and concurrent lifetime behavior are not established.

S's corrected reference and preserved original bundle are read-only inputs. The next Rust stage must explicitly decide owned replacement/drop behavior, unwritten fields, matrix representation, callbacks/payloads and defined shared-observation views while preserving source copy order. No new Rust owner is released by this acceptance alone. O/R's accepted stores and K/N/P/Q's accepted reader/preprocessing references remain prerequisites. Rug128, source-defect preservation and the strict SVD gate remain unchanged; M2's four historical failures are still unresolved.
