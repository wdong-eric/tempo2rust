# Assignment S: pulsar lifecycle reference

Delivered for central review on 24 October 2026 under decision 0038. The native reference covers the complete **349-field pulsar map, seven nested types, 95 observation fields and twelve primary routines**. Fresh exact replay passed **30 cases / 133 ordered events / 5,329,658 field records**. Twenty-eight cases return normally; two zero-dimension allocations terminate with SIGABRT at the source assert. All **23 integrity controls and ten defined compiled mutations** pass. A separate O0/gcov build matches all 30 ordinary projections exactly and enters all twelve routines.

This is bounded native reference evidence, not a Rust transfer, physical validation or milestone acceptance. R/P/Q, Rust, accepted evidence and central documents were not edited. Both independent repositories remain clean. No A2 engine or patched native function supplies ordinary results. Central retains ownership/API, callback ABI, numerical and acceptance decisions; M2 remains unresolved.

## Reproduce

From the project root, with the pinned local source/reference repositories, accepted M inventory, Python 3 and installed Homebrew GCC/gcov 16:

```sh
# Rebuild and check the existing local frozen bundle; does not rewrite goldens.
python3 validation/pulsar_lifecycle/verify.py
python3 validation/pulsar_lifecycle/coverage.py
python3 validation/pulsar_lifecycle/evidence.py
```

To regenerate the local bundle from the source pins and committed scripts:

```sh
python3 validation/pulsar_lifecycle/inventory.py
python3 validation/pulsar_lifecycle/fixtures.py
python3 validation/pulsar_lifecycle/generate.py
python3 validation/pulsar_lifecycle/run.py --record
python3 validation/pulsar_lifecycle/verify.py
python3 validation/pulsar_lifecycle/coverage.py
python3 validation/pulsar_lifecycle/evidence.py
```

Run sequentially; suites reuse owned build paths. `--record` deliberately creates/replaces local reference artifacts and is not verification. Ordinary `run.py` always builds fresh, verifies current identities and compares all cases. No cached-build bypass exists. Inventory generation also snapshots the declared accepted-file protection membership; do not use it to bless changes to accepted evidence. The native build uses only the raw pinned source, the selected binary128 header and A2's hashed minimal configuration, never A2 objects or archives.

Per the user's later instruction, generated JSON/gzip, raw dumps, profiles, receipts, deltas, coverage, generated exporter header and binaries remain local and ignored. The earlier map-freeze commits predate that instruction; central has untracked their generated outputs under decision 0039 at `3848f5a`, preserving local bytes. No generated payload is added in the delivery commits.

## Source, caller and lifetime map

`inventory.py` mechanically reads the pinned header, cross-checks all pulsar/observation names against accepted M, and emits `inventory.json`: field types/extents, natural-write classifications and source anchors, copy/retention anchors, nested declarations, enums/constants/callback signatures, twelve function bodies and every selected branch site. `CONTRACT.md` and `fixtures.py` fix the caller and ownership rules. The pre-experiment freeze is `a70db26`; `ba07fe5` corrects classification of `char *constraint_special[]` as a pointer array before any native execution. Executed harness/scripts are committed at `428a050`; the subsequent documentation commit contains this report.

Natural initialization has **219 entirely unwritten top-level members**, 116 complete scalar/array writes, eleven string-prefix writes, and the three special allocation-bearing members `param`, `covar`, `obsn`. Unwritten members are reported without reading storage. Source writes within a member and allocator writes to its backing are distinct:

| State | Natural initialization and copy behavior |
| --- | --- |
| `parameter[2000]` | Actual allocation gives 9,616 cells. aSize/link counts, val/err/fit/set and complete calloc label buffers are written. Link arrays and prefit/prefitErr backing remain unwritten. copyPSR retains all parameter state. Both IPERHARM labels and retained label suffix bytes are exported. |
| `observation`, including nested clock corrections | Initializer malloc backing is unwritten. Seeded copies export every field of every record, including unused backing. Only the nobs prefix is assigned; tzrobs is retained. |
| Strings | Natural writes are exported only through their terminating NUL; suffixes are marked unwritten. Retention callers initialize every byte, so strcpy's destination suffix retention is fully observed. |
| `FitInfo`, `FitOutput`, `storePrecision`, TZR, constraints | Unwritten/retained unless an initializer anchor says otherwise. Seeded snapshots cover every nested member. Callback fixtures have valid typed function identities and are never called. Opaque payloads have declared fixture owners/bytes. |
| Covariance | Actual MAX_FIT=10000 dimensions, all 10,000 row offsets and all 100,000,000 double element representations are read. Every element is observed as positive zero. The lossless equal-unit run covers the full range, not a sample. |
| Constraint pointers | Natural initialization writes all 2,000 pointers to null. Retention callers use independently owned, distinct terminated payloads in every slot. Copy retains them; reinitialization overwrites them while the saved old owner remains live. |

Caller seeds use finite typed values, valid enum constants, terminated strings with distinct suffixes, real allocations and registered symbolic owner/offset identities. Counts consumed by copy are explicitly bounded. Other retained numeric metadata is fixture state, not a fitted physical model. There is no poison-byte initialization of floating values, enums, pointers or callbacks. Struct padding and pointer addresses are not serialized as field values.

The histories cover signed fullSetup values -7/0/1/9 and repeated signed arguments; wrapper counts 0/1/2, noWarnings values and debug/CVS controls; direct repeated allocation with realloc -3/0/1/5; and two successive initialiseOne calls after a fully seeded owner. Prior observation, covariance, parameter and constraint allocations are observed through saved live owners and then cleaned separately. Native overwrites/leaks remain observations; harness cleanup does not repair the native functions.

Small matrices cover 1x1, 2x3, 3x2, 4x4 and malloc_uinv(3), actual accessors, every row offset/cell, first/last writes and matching free_blas/free_uinv. Destruction includes normal owners, a null observation pointer and independent early/last constraint payloads while nconstraints=0. Exact source free order is inventoried; successful returns and coverage do **not** measure individual free calls or prove allocator internals. No allocation interception was used.

Copy cases cover negative/zero counts, partial prefixes, the last valid inclusive jump and FD entries (index 1999), repeated copies, shared observation storage and both overlap directions. Source order is retained: name/binaryModel/control counts, forward observations, position/velocity, ordinary inclusive jumps after copying nJumps, inclusive FD loop using the **destination's retained nfdJumps**, then remaining assignments. nfdJumps, FD indices, jump SAT/scaled flags, parameters, covariance and omitted state retain distinguishable destination values.

With destination offset +1, forward assignment propagates the first source observation through the overlapped destination prefix; source-visible observations consequently change. Shared/interior observation pointers are detached before destroyOne and the shared backing is freed once by the harness. Distinct pulsar objects keep all strcpy operations disjoint.

## Evidence and execution identity

Local `raw/*.json.gz` retains the native event bytes, stdout/stderr bytes, fresh launch nonce and process status. `expected/*.json.gz` contains complete exact comparison projections; `observations.json` indexes them. `deltas/*.json.gz` contains all added/removed/changed records across **103 snapshot transitions**, including both values. These are snapshot differences, not instrumentation of identical-value stores or intra-function writes. `raw_validation.json` checks field encoding and raw state equality. `mutations/` retains complete mutant projections and raw bundles.

The only diagnostic normalization replaces staged source-path prefixes and, in the explicitly classified debug case, rendered addresses. Stream positions in the comparison refer to that presentation; original byte positions and streams remain in raw bundles. All scientific/state bytes, member/order/count/owner relationships, return status and diagnostics otherwise compare exactly. No floating tolerance is applied.

Host: macOS 26.7 arm64, C locale, Homebrew GCC 16.2.0. Selected longdouble is measured 16-byte binary128 with 113 significand bits; double and size_t are eight bytes. Runtime MAX_OBSN=4 and wrapper MAX_PSR=0/1/2; compile-time capacities are unchanged.

| Identity | SHA-256 or revision |
| --- | --- |
| Canonical TEMPO2 | `fda6ad4a9753ad7dc5d291406f2f310a9161423f` |
| Independent A2 checkout, config only | `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572` |
| Ordinary O2 executable | `6922529daf9e3660687a1c9bab44adf7eddee672724025f2022ffd3318d42eff` |
| Separate O0/gcov executable | `7de4d5e7bd0a5426cddc9a54872920e01bcf9e5bf66bff6bf92c2daad824aa08` |
| Native harness | `b7e69ae438a2957f0114d87c154a72546e14ae462bf2d4e7e0ef56d8f904de1b` |
| Frozen inventory | `b71beaeb418345784444dd572834776a6fa4a6d1c275c3b725d6f6ef0346c321` |
| Ordinary profile | `ae1ae27fbd79f0dc11e75818ff9ab8fed8a573a716320c9e0817fea90806d657` |
| Observation index | `d57ce24d54eda485eda64cf707b819f437f08c29162297f803b29fd8a7c73ddf` |
| Fresh replay receipt | `b2e2532e3e7316069913bd3ababc4737b99b63763f637997d3ec352866daef8e` |
| Controls/mutations receipt | `4107ac74f0e7053d675be3661cc557f756ebb7f6e660dad2e917f57f9a1add2f` |
| Coverage receipt | `e53e858c475d50c7c79fd1e27cba52610e387a9cdf193417934f1706f85c5718` |

The local profiles retain commands, executable/link-map digests, raw and normalized preprocessing, macros, compiler and cc1plus hashes, runtime library identities, 246 included source/header dependencies and every runner/exporter hash. Every launch checks actual executable, current runner/header/source/compiler/backend/runtime bytes before and after execution. A failed normal attempt removes its preceding success receipt. System libSystem/shared-cache, loader and kernel are identified by the OS/link boundary, not separately hashed; this is not a hermetic build.

## Controls, measured coverage and gaps

The 23 controls cover missing/extra/reordered events/fields, status/stream/value/run-length changes, alias identity, compiler/backend/source/config/header/preprocessing/runtime/macro/runner identity, same-path executable replacement, stale success receipt, no-output replacement with old case files present, and stale nonce. Ten disposable mutants are rejected by actual state differences independently of their changed identities: nits initialization, omitted-control write, string suffix copying, matrix columns, row stride, reverse observation assignment, exclusive ordinary jump bound, source FD bound, copying omitted nfdJumps and nonzero calloc element replacement. All witnesses remain initialized and within capacity; canonical source is unchanged.

Gcov measures **702/711 executable selected lines and 236/256 branches**, including compiler-generated edges. Call counts are: initialise 4; initialiseOne 12; allocateMemory 37; destroyOne 29; destroyMemory 37; copyPSR 9; malloc_blas/free_blas 51 each; both accessors 105 each; malloc_uinv/free_uinv 1 each. Aborting processes do not flush counters, so these are accumulated coverage counts, not all process calls.

Every unhit selected line/branch has the following disposition; exact indices remain in local coverage.json:

| Unhit location | Disposition |
| --- | --- |
| initialise.C:70–71, 95, 97–101 | Observation allocation failure and its diagnostic/exit path: unrun. Branches at 97–100 include unentered normal diagnostic edges and compiler exception edges. |
| initialise.C:105 exception edge | Compiler-modeled exception from the allocation call; untested, no exception injection. |
| TKmatrix.C:171 assert failure edge | Both zero-row and zero-column cases actually abort and match raw evidence; absent from flushed gcov counters. |
| TKmatrix.C:179–182 | Element-block/row-pointer allocation failure and nested diagnostic branches: unrun. |
| TKmatrix.C:203–204 inner debug false edges | Enclosed by if(debugFlag); false is unreachable with the declared single-threaded, unchanged flag history. |
| TKmatrix.C:173–176 sizeof guard | False for this compiled ABI and eliminated from measured executable paths; another platform is untested. |

Other source-inspected/unrun classes remain explicit: allocator exhaustion and size overflow; unwritten/freed reads; null/interior/foreign/double frees; same-pulsar or overlapping strcpy; missing terminators and over-capacity loops. No such experiment supplies a golden. Commented observation zeroing, intended fullSetup/realloc behavior and commented copy allocation are inactive source, not defaults.

Matrix metadata uses native size_t/type-punned layout and is evidence for this host. Debug logging uses source varargs formats and TKlog's known before-buffer pointer formation; the debug/CVS case is a **host/ABI diagnostic observation**, not portable defined C++ evidence. Other backends/platforms/locales, callback invocation/ABI, physical timing/fitting consumers, allocator interception and sanitizers are not covered.

Central's later contract must decide whole-pulsar ownership and safe treatment of leaked/replaced owners, unwritten state and allocation failure; whether/how to expose defined shared/overlapping observation histories; and the Rust representations of covariance metadata, callbacks and opaque payloads. This delivery supplies evidence without choosing those APIs or changing numerical policy.
