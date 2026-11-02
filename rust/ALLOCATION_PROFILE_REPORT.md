# Assignment AN: covariance allocation

Released by decision 0121; measured on 2 November 2026 in the isolated clone
`/private/tmp/tempo2-an-allocation-20261102`. Main, canonical TEMPO2, historical
runners and accepted evidence were not modified. Central retains integration,
Linux dispatch and milestone acceptance.

The eager-zeroing hypothesis is confirmed on the measured macOS host. Requesting
zeroed storage through the standard global allocator removes approximately one
763 MiB resident payload per unused covariance matrix. Dimensions, zero values,
layout, ownership and calculations remain unchanged. Linux benefit is **not yet
measured**. This report does not claim that the remaining CPU gap is solved.

## Immutable sources and builds

| Role | Revision |
| --- | --- |
| Released production baseline | `dc435f53a64e4dd5c25cae6319841a28bd981fcb` |
| Standalone observers and paired measurement runner | `9aaeb032c0944eb7dd8a22b67e187365bc98f911` |
| Production allocator and focused unit control | `68973d669af0e1180ad8532bb8b364452ca203d7` |
| Native TEMPO2 source pin | `fda6ad4a9753ad7dc5d291406f2f310a9161423f` |

Only `src/tk_matrix.rs` and `src/pulsar_lifecycle_tests.rs` change in the production
commit. The lifecycle implementation, Cargo manifest/lockfile, providers, fitting,
precision, CLI and qualification/comparison policies are unchanged. Before/after
crate exports are in `validation/allocation_profile/build/{before-v1,after-v1}`.
Their tracked source files and copied executables are read-only; hashes are checked
before/after measurement and verification. Test artifacts and resource aliases
added later are outside each frozen source manifest. The first diagnostic build
and runs remain separately under `before/` and `diagnosis-{before,native}/`.

`tk_matrix.rs` SHA-256 changes from
`75534fee4b18e7634ce20e6c65f8df859adcd800d22e2aa5022f6c4ddab86857` to
`666ec85bf65a9bcfa8184da8cc757d2a8295349cff28db0ec7031827a00fd705`.
Raw native `TKmatrix.C` is
`3817bedcd680e88be6b715b1165eacec028f942d3213eec1b0f02c097a01fb11`.

| Executed artifact | SHA-256 |
| --- | --- |
| Before AK CLI | `78722d75c3fbf31a6450039bdfff4302c4c09324ea9bd40bcd7260a251b7fadb` |
| After AK CLI | `328c8aab0f4be6d44842fbbc002f2c5eac7fd77918422f748bec67b0f6b8095d` |
| Before ordinary CLI / JSON interfaces | `63e514052447035e8475b4d251c7ec3fd8d8d03d63968414f6f5aabbaa3dfb12` |
| After ordinary CLI / JSON interfaces | `01aff6ca5dd44fa30079ce625e3d4d5a2446bd0734166b8f9a744317856d9bfc` |
| Before residency observer | `91917aa461af2886249e4f70584976f6b6cf5ab23e933ed2adf16273db7e1d99` |
| After residency observer | `bb7e58b0c14e40ea1f5ae824a20fe2bcf67790187c1fc04f39c24f72a23dd689` |
| Native residency observer, identical in both builds | `e8ab4106b32a486e0163813257054e2ca3c3a5999b83a45493c23bde0b95f59d` |
| Native disabled-provider CLI | `b6b0e3493858fe6c77b53b8da3650a195cbce6a8007f282aa2b411001024a5a3` |

The native library and runtime are read-only inputs from
`/Users/Eric.Dong/Downloads/tempo2rust/validation/release/build/integrated-v1/a2`.
The A2 receipt SHA-256 is
`9a02b11d502fefb7a422da9a6d90e3a235756316c041f8f91f4cca543d68e7ea`;
the archive is `54d0c7fa33cba466ae334c9449eb7400eff6851272d9ba28420607406a9604cc`.
All 208 runtime-file hashes are checked before and after the paired campaign.
PAR/TIM/model hashes, exact argv, executable identities, time outputs and load
averages are retained in its receipt. The AK wrapper and workload arguments are
reused; no old performance runner is rewritten.

Host: Mac15,6, arm64, 18 GiB RAM, 11 CPUs, macOS 26.7 / 25G229, Darwin 25.6.0,
16,384-byte pages. Rust 1.97.1 (`8bab26f4f`, LLVM 22.1.6), Cargo 1.97.1,
Apple Clang 21.0.0, Apple ld 27037.1, GNU C++/Fortran 16.2.0. Production and
residency binaries use Rust's default System allocator / macOS libSystem; there
is no replacement allocator. Library linkage and compiler hashes are retained.
The separate failure-control executable intentionally supplies a rejecting System
wrapper and is excluded from performance measurements.

## Causal measurement and scope

`observe.rs` calls the actual factory and actual `PulsarContext::initialise_one`,
with 10,000 observation slots for each initialized pulsar. `observe_native.cc`
links A2's unchanged `malloc_blas`, `initialiseOne` and `destroyOne`. C++ pulsar
records start zeroed as in the A2 observer convention; native observation storage
is not equated to Rust's initialized canonical records.

The C observer queries `mincore` without reading payload bytes. It records each
live range, requested payload length, allocator usable size, the VM region
containing the start, current RSS, peak RSS, user/system CPU and page faults.
The recorded first VM region can be smaller than a large allocation; successful
`mincore` covers the entire requested range. These are observed covariance
allocations and owner histories, **not** a census of all process allocations.
Repeated pointers in clone snapshots denote shared backing, not two allocations.

Each Rust payload is 800,000,000 bytes; native also requests 16 bytes of hidden
dimensions and a separate 80,000-byte row table. Both cell allocations have
800,014,336 allocator-usable bytes on this host. Full dimensions remain 10000 by
10000, stride 10000, and `f64` alignment. No Rug allocation is involved in these
payloads. Native's one initially resident payload page contains its dimension
header; Rust has typed side metadata instead.

Five fresh process samples were taken for every control, alternating engine
order. Matrix and context creation precede any cell inspection. Full reads,
exhaustive positive-zero validation, full writes, matrix/context COW and context
reinitialization run in distinct processes. Below are median allocation/init
intervals and current RSS immediately after the last allocation. Initialization
time for two owners sums the two calls; context construction is observed separately.

| Scope | Before time / RSS | After time / RSS | Native time / RSS |
| --- | --- | --- | --- |
| One matrix | 38.036 ms / 764.52 MiB | 0.006 ms / 1.58 MiB | 0.020 ms / 2.55 MiB |
| Two live matrices | 76.285 ms / 1527.55 MiB | 0.009 ms / 1.66 MiB | 0.032 ms / 2.69 MiB |
| One real initialized pulsar | 55.854 ms / 926.81 MiB | 17.428 ms / 163.88 MiB | 1.169 ms / 6.44 MiB |
| Two real initialized pulsars | 110.916 ms / 1851.38 MiB | 34.600 ms / 325.45 MiB | 2.353 ms / 10.25 MiB |

In every single-matrix untouched sample, before has 48,829/48,829 resident pages,
after has 0/48,829, and native has 1/48,829. Before adds 48,832 minor faults between
start and allocation snapshots; after adds four. The real context observations
show the same payload distinction. One first-sample two-matrix baseline lost some
residency during observation (45,637 pages in its older range); it is retained,
not removed as an outlier. This illustrates why peak RSS alone is insufficient.

The mechanism defers first-touch work; it is not sparse storage or a new numerical
representation. On this macOS host, even a complete **read** brings every payload
page into residency. Median matrix-only read control time is 40.068 ms before
versus 67.162 ms after; full write is 14.819 versus 41.848 ms; COW detachment is
57.373 versus 85.115 ms. These are the later operation intervals, excluding
initial creation. Full read/write reaches about 764.6 MiB RSS in either Rust
build, and COW reaches about 1527.6 MiB with two distinct full buffers. Immutable
cloning shares the original pointer; mutation detaches and preserves original
values. No COW implementation was changed.

Reinitialization retains the old covariance owner exactly as before. With one
pulsar reinitialized there are two distinct live 800 MB ranges; after has no
resident covariance pages before touching. `destroy_one` releases the current
owner, leaving the historical owner inspectable until context drop. After logical
drop, OS-only queries of former ranges still find mappings: resident pages remain
resident in the touched controls. This is allocator/OS retention on this host,
not a claim that Rust still owns those freed buffers. No freed pointer is read or
passed to `malloc_size`; released-range queries use only VM APIs. No forced heap
purge or reclamation strategy was introduced.

Observer cost is reported separately: median empty warmed snapshot is 3 us with
zero RSS delta; allocation snapshots take about 2.08 ms and add a median 64 KiB
of observer storage. A largest such snapshot takes 38.25 ms. The per-operation
timers stop before observation. Positive-zero controls after observation and
source inspection establish that the observer does not modify cells. All
full-workflow timings below use uninstrumented CLI executables.

For the complete **instrumented initialization processes**, including snapshots,
destruction and startup, median wall/CPU time is 0.1271/0.08 s before versus
0.0396/0.01 s after for one context, and 0.2378/0.20 s versus 0.0767/0.04 s for two.
Their median process peaks are 928.64 versus 165.70 MiB and 1856.53 versus
330.61 MiB. The raw phases also retain cumulative user/system CPU. These CPU
figures include observer cost and are not asserted to be pure initializer CPU;
the allocation/init intervals in the table exclude the subsequent observer call.

## Matched complete workflows

210 processes completed successfully: 180 observation/control processes and 30
uninstrumented CLI processes. Each workflow has five fresh samples per engine,
with before/after/native order reversed on alternating samples. Serial/default
Rust features, `-npsr 2 -nobs 10000 -noaccel`, AK worker count 1, single-threaded
providers, C locale, original NITS=2, covariance model and output names are retained.
Chebyshev uses `PKS 58000 58000.03 1000 2000 8 4 3600`.

| Workflow / engine | Wall seconds, median [min, max] | CPU seconds, median | Peak RSS MiB, median [min, max] |
| --- | --- | --- | --- |
| Cheby before | 0.459811 [0.449184, 0.462440] | 0.40 | 833.469 [833.438, 833.516] |
| Cheby after | 0.403618 [0.386373, 0.407174] | 0.36 | 70.500 [70.469, 70.547] |
| Cheby native | 0.130855 [0.127121, 0.132137] | 0.07 | 161.875 [161.813, 161.906] |
| Correlated-global before | 0.516534 [0.499611, 0.944594] | 0.45 | 1578.125 [1578.031, 1579.641] |
| Correlated-global after | 0.402634 [0.400192, 0.841236] | 0.37 | 52.406 [52.344, 53.953] |
| Correlated-global native | 0.040283 [0.039320, 0.077356] | 0.02 | 161.297 [161.266, 161.375] |

Median RSS falls by 762.969 MiB (91.54%) and 1525.719 MiB (96.68%), respectively.
Median wall time falls by 12.22% and 22.05%. These are desktop measurements, not
platform-independent promises or a performance gate. CPU counters are rounded to
0.01 s by `/usr/bin/time`. First CLI launches have conspicuous wall-time outliers;
all remain included. One-minute host load spans 3.32–3.88. Central reported no
competing benchmark, but the desktop was not exclusively reserved; a later host
snapshot shows about 1.8 GiB swap in use. We do not assign individual outliers to
an unmeasured cause. The immediate untouched-range result is much more stable
than a universal timing claim.

## Allocation choice and safety

The pinned 1.97.1 standard-library source was inspected and hashed. A compilation
probe confirms `Box::<[f64]>::try_new_zeroed_slice` is still gated by
`allocator_api`. `Box::new_zeroed_slice` is stable but infallible; `vec![0.0; len]`
also loses the recoverable payload failure contract. Reservation followed by
resize preserves that error path but is the measured eager-write cause.

[Bytemuck 1.25.2's `try_zeroed_vec`](https://docs.rs/bytemuck/1.25.2/bytemuck/allocation/fn.try_zeroed_vec.html)
is a maintained safe generic alternative with a fallible zeroed-allocation
contract. Its source uses a zeroed global allocation through a boxed slice. It
would add a dependency and generic zeroability machinery solely for this concrete
`f64` buffer. It was source-assessed, not installed or performance-tested. The
standard allocator is sufficient for the small local boundary.

The chosen [standard `alloc_zeroed`](https://doc.rust-lang.org/1.97.1/std/alloc/fn.alloc_zeroed.html)
plus `Vec::from_raw_parts` preserves `Arc<Vec<f64>>`. Both unsafe blocks have local
obligations: nonzero checked layout, global allocator/deallocator compatibility,
alignment, initialized positive-zero `f64` cells, matching length/capacity,
exclusive initial ownership and transfer exactly once. Existing dimension/header
and row-table overflow classifications are retained. Null payload allocation
returns the same `malloc_blas / cells / allocation_failure` boundary. Existing
infallible Arc bookkeeping and COW cloning are unchanged; this does not promise
recovery from all process OOM or an OS overcommit failure at later first touch.

Zero initialization is an API guarantee. Lazy residency is **not**: a downstream
custom global allocator can initialize eagerly, and another OS may account for
shared zero pages differently. The pinned Unix System implementation forwards
this layout to `calloc`; the optimization never passes libc-owned memory directly
to a Vec using a different allocator. This is a measured Rust implementation
improvement under 0099, not a native TEMPO2 defect correction under 0053.

## Behavior verification and inherited differences

* Release lifecycle controls: nine existing tests pass before, ten after, including
  the new exhaustive 100-million-cell positive-zero/layout test. The after debug
  profile also passes all ten. The isolated clock-fault recovery control passes
  separately in both release builds.
* Rejected 64 KiB payload allocation: both builds return the identical recoverable
  boundary, keep the affected context terminal and allow a fresh context to allocate
  successfully. No enormous allocation or host exhaustion is used.
* Eleven challenged lifecycle cases compare exactly before/after using the existing
  S metadata projection: rectangular/square matrices, zero-dimension boundaries,
  full initialization, two-pulsar wrapper, reinitialization and destruction.
  Historical native S comparison passes ten cases in each build. Its seeded
  reinitialization case still differs at `p.nchromx`: Rust 0 versus historical
  native 11, identically before/after. Raw evidence and the failure are retained.
* Every CLI output filename and byte matches across all ten Rust samples of each
  workload. Supplemental full machine-readable JSON is byte-identical with no
  projection: the same working directory and relative filenames preserve metadata.
  All seven fit diagnostic files and both predictor files also match exactly.
  Complete fit JSON SHA-256 is
  `348a8ae5fb6e3971b0da27e91c4248fbabbc58b8d6c44030cbbf8d75d90df57e`;
  Chebyshev JSON is
  `609794e058f0c5b5c5005df5adf050ba7c33dcab88dca555cfb961e84ae74ec4`.
* Existing native full correlated-global fitting comparison: 4,555 checks pass
  both before and after, with identical gate statistics. No comparator, raw SVD
  array, cutoff or tolerance was changed. This fixture also passes the baseline's
  stricter raw-position checks; AN does not alter or adjudicate decision 0120/0122.
* Existing isolated-PKS predictor gates with the released native-ABI/serialization
  adapter: 4,399 Chebyshev checks pass both builds. The neighboring polyco case has
  one identical failure among 1,003 checks: `model[0].log10rms`, Rust
  `-7.364344127317825`, native `-7.36434222569590790641314015374518931`, exact
  native-width gate. This is retained as an inherited difference, not waived.
* Combined `calceph,lapack` build: all four existing matrix/BLAS/LAPACK tests pass,
  including rectangular products, factorization triangles, QR optional outputs,
  constraints and recovery. The separate real-ephemeris CALCEPH test remains
  ignored; this allocation change does not modify that provider's calculations.
* Formatting of changed Rust/observer files, Python compilation and patch whitespace
  checks pass. No whole migration campaign or unrelated test expansion was run.

Central's integration review identified an inherited-environment issue in the
recipe: its driver supplies a shared `CARGO_TARGET_DIR`, while the original
`bind()` expected a local target and hardcoded a separate Cargo cache. The final
build/verification helpers now force a separate target under each frozen source,
preserve an explicit caller `CARGO_HOME` or Cargo's normal default, and honor a
prepared GMP cache. A narrow control builds two real dependency-free crates under
an inherited shared target setting, checks both outputs and first-binary
preservation, and records the effective environments. This passes under
`path-control-v2/`; no production or scientific/benchmark run was repeated for
the path-only correction. The measured receipts already bind separate targets.
`recipe-identities.json` distinguishes the executed recipes from these final
ones, so the old measurements are not represented as executions of revised code.

The initial local failures are retained: Python's unsupported `extractall(filter=)`
argument before baseline construction; a sandbox-denied macOS time-counter query
(`measure-v1`, rerun with read-only system-query access in `measure-v2`); and three
verification setup attempts correcting Cargo target selection, an absent runtime
alias and an absent diagnostics directory (`verify-v1`–`v3`). None was a production
allocator failure. Final evidence uses `verify-v4`; earlier failed outputs were
not overwritten or relabelled.

## Evidence handoff

All paths below are relative to `validation/allocation_profile/build/` in the
isolated clone. JSON/JSONL, source exports, logs, binaries and caches stay ignored.

| Receipt | SHA-256 |
| --- | --- |
| `before-v1/receipt.json` | `9af72fb0bfdca99effd2655f789c8388aaeebd039ad95902e3a8b0062e4eebbc` |
| `after-v1/receipt.json` | `09ac6caf7d9c76b29fc5153e977b9d94ef4f7c9f2396f09fcf7a720ac645abc7` |
| `measure-v2/receipt.json` | `47692c95ccefdd417e22e652294fa0348b626427694c603e63782c69a067b30c` |
| `verify-v4/receipt.json` | `fd3a9acdf2a69631d9f556f4d26a754c3b2e2b53214d57936a7f82018e2e972a` |
| `debug-checks-v1/receipt.json` | `4ddbfb94691cf080f695e9067afae5a278ccb9aa44c93360ba6873adb1712237` |
| `provider-checks-v1/receipt.json` | `30028ec01fad2e5fabae7051dfe83186a7ec02efc6434b8990e5fc4ed49bd634` |
| `predictor-before-v1/receipt.json` | `1eec9d3c9e582757dba0c98243b69e8c1d2f503effbf5705a999cdcbebc2b3e6` |
| `predictor-after-v1/receipt.json` | `a83361080d6621688acdaa8d9e85670b1deb2a0d1c7c03d5a1b9a37821d086fb` |

`std-apis/receipt.json`, `toolchain.json`, `host-after-measurement.txt`, all raw
time/range records, `predictor-equality.json`, `path-control-v2/receipt.json` and
`recipe-identities.json` provide supporting identities.
The concrete Linux recipe and minimal central wiring are in
`validation/allocation_profile/README.md`. No workflow was dispatched, no remote
was added and nothing was pushed. Acceptance on both selected platforms awaits
actual Linux evidence and central's integration review.
