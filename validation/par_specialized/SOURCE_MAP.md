# Assignment Q source and dependency map

Canonical revision: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. `inventory.json` is the full machine-readable source/caller/dispatch/initialization map, derived from the released manifest and checked K inventory. All 116 deferred top-level IDs plus D029 retain their exact pinned source text, scans, field references and nested conditions. No accepted K result is rewritten or promoted to path completeness.

| Source | Role and observed boundary |
| --- | --- |
| readParfile.C | Primary checkLine ordered dispatcher; real simple/full/global readers; readValue and checkAllSet dependencies. Exact release hash `ecdbec472eff256ab1c7d875a4ea56536c146012599684408b03fe738ac7550e`. |
| tempo2.h | Actual types, capacities, parameter/constraint enums and dependency declarations. Hash `0313b6eaf9f9ae9d7e5cce340379ad4b7068394e5222a01f73949e3a2640de93`. |
| initialise.C | Unmodified allocation/initialization/destruction, including unwritten specialized fields and parameter prefit storage. Hash `5da876556455e5fda635b30917ae9e745ef880b93ef513b47b6e4823458868c1`. No A2 solar-wind correction. |
| t2fit_nestlike.h | Ten actual inline clamp/decode/encode/set helpers, called both through native PAR code and direct witnesses. Hash `50db4fb3e38bb16eab83dbd2ce4331ee9fbbbbb314e60b17fbb003997996a9af`. |
| TKlongdouble.C / TKlongdouble.float128.h | Real binary128 scanner/backend, measured 113-bit significand; no new arithmetic implementation. |
| tempo2Util.C / TKlog.C | Native parameter copy/readValue support and diagnostic history; accepted M host qualifications retained. |
| clkcorr.C / tabulatedfunction.C / observatory.C | Actual clock resources, indexing, cache, interpolation and route calls. J remains authority for broader graph behavior. |
| TKmatrix.C | Actual initializer matrix allocation/destruction support. |
| global.C | Verbatim data prefix before `extra_delays` and CVSdisplayVersion suffix. Exact byte extent/hash recorded; timing engine is not linked. |
| config.h | Exact configuration archived in K commit 38d62af9c183a0bb3294638999ca63cb431fc046 (originally A2), retrieved as configuration text only. All actual transitive headers, compiler backend and runtime library bytes are separately attested. |

`profile.py` builds raw Git blobs in the owned build directory. Only `fields.h` (source-derived observation/seed membership), numeric-header selection, configuration, and the declared global-data slices are generated. Native implementation sources are unchanged for the reference/coverage builds. Mutation builds have explicit disposable substitutions, distinct executable identities and no authority as reference goldens.

`profile.json` and each executed profile record the actual compiler driver/backend, flags, source/header/config identities, normalized and raw preprocessed hashes, sorted complete macro definitions digest and raw macro-dump digest, runtime dylibs, link map, runner/harness sources and executable. Stable comparison excludes executable/link-map bytes, which are re-attested for the actual current binary before/after every launch. Per-case raw receipts record a fresh nonce, process status and raw events/streams digests. Normal verification rebuilds; no `--no-build` mode exists.

Only absolute source/build/owned-directory prefixes are normalized in text after raw preservation. Raw numeric buffers containing scientific state are not text-normalized. No tolerance, numeric class substitution, timestamp rewrite, or downstream repair is applied. Repeated initial JSONL lines are stored once by SHA-256; raw receipts are verified after byte-exact reconstruction. Character matrices are projected per row to retain full initialized tails without duplicating every other row in each delta.

`protected.json` freezes the release commit ad6fbf6 membership/hashes of accepted evidence and both nested Git checkouts. Primary-repository membership comes from that historical Git tree, independently of current tracking. Missing local generated files are explicitly listed as archived-only and read from the pinned tree. Other-assignment `rust/`, `validation/observation_storage_port/`, `validation/preprocessing/`, `validation/pulsar_lifecycle/`, and R-authorized O verifier routing are outside that membership. P's preprocessing evidence is now accepted/read-only under 0040 and remains outside Q's original frozen membership. Rust and central files are not written, staged or committed by Q.

Input files and the exact generated actions stream have before/after hashes in every launch receipt, together with argv and the C-locale/runtime-root environment. Native EOF/error flags are preserved during observation: `pread` reads remaining bytes without altering the logical FILE position or flags.

Coverage compiles each unchanged native translation unit with `-O0 --coverage` and the observation caller with `-O2 --coverage`, then links their recorded objects. The caller includes direct inline-header witnesses at O2; native parser/initializer calls exercise the header at O0. This avoids timing out on serialization; it does not optimize or patch native parsing. Every per-unit compiler command is recorded.

Generated JSON/compressed evidence is local and ignored under the user's source-only tracking policy. The released branch manifest and K inventories/configuration are read from immutable primary-repository Git blobs; regenerating Q therefore does not require those generated files to remain tracked in the current tree. Hand-authored fixtures live in `fixtures.py`.
