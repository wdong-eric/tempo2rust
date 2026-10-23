# Assignment O: parameter storage delivery

Delivered 24 October 2026 from central release `73fc537`, under decision 0033.
**The released O checks pass. Central acceptance is pending. M2 remains unaccepted,
and this is not completion of the overall TEMPO2 transfer.**

## Implementation and scope

Production implementation commit: `8bad0be`. Pre-implementation projection:
`4ac2171`. Exact verifier, supplemental native caller and authorized output routing:
`71cd6d8`, strengthened by `1462ca3` (frozen boundary callers and build-time
identity), with the failure-injection control isolated in `3e48030`. First evidence checkpoint: `8d9f14f`; final verification artifacts: `7d24a91`. Shared HEAD advanced through central's P dispatch while O worked;
execution receipts pin source and executable bytes independently of HEAD.

- `src/tempo2.rs`: all 166 source enum entries, signed `param_label`, source
  capacities, 2,000 parameter slots / 9,616 cells, signed flags/counts/logical size,
  independent owned backing, complete label buffers and explicit unwritten fields.
- `src/initialise.rs`: parameter-only allocation and all source-order label writes;
  ownership destruction replaces C allocation lifetime. No full pulsar initializer.
- `src/tempo2_util.rs::copy_param`: label/shortlabel/value/error/fit/set/prefit/
  prefit-error order, logical size last, unchanged destination links and tails,
  through-NUL byte copies with suffix retention, zero and negative counts.
- `src/input.rs`, `read_parfile.rs`, `lib.rs`, `form_residuals.rs`, `form_bats.rs`,
  `std_freq.rs`, `t2fit.rs`: loading, both evaluators and fit value/error callbacks
  use the same canonical cells. The old mutable numeric map and separate mutable
  physical-error array are removed. Supplied uncertainty/source-line provenance
  remain frontend metadata. No synchronization or compatibility-map read occurs
  at evaluation boundaries.
- `src/parameter_storage_tests.rs`: controlled caller probes and boundary,
  ownership, signed/nonfinite, snapshot and integrated evaluation tests.
  `src/fitting_tests.rs` changes only its storage/callback API call sites; its
  accepted fixture values, assertions and numerical gates retain their meaning.

The registry and parameter field/case projection were written before implementation
in `../validation/parameter_storage_port/PROJECTION.md`,
`registry_projection.json`, `case_projection.json` and `protected_hashes.json`.
The source transcription helper `translate.py` is neither a build dependency nor
a runtime dependency. Production does not read reference JSON or a TEMPO2 checkout.
Rug remains 1.30.0, at 128 significant bits, with the same arithmetic boundaries,
finite Scalar construction and source fitting precision conversions.

`parameters()` now returns a fresh owned `BTreeMap<String, Parameter>` instead of
`&BTreeMap<...>`. Binding the snapshot supports callers needing references. This is
the source API change authorized in 0033; JSON fields/values, five historical keys
and absence of undeclared optional keys are unchanged. `Parameter` remains the
scalar compatibility view; `SourceParameter` is the array-valued source object.
`parameter_store()` exposes only read-only inspection and documents the frontend
bridge: prefit fields are unwritten, source err starts at zero, and supplied
uncertainty is not a completed source readValue conversion.

## Source fidelity and safe boundaries

Canonical TEMPO2: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
K evidence: `38d62af` / `ccdf55c`; M: `a01d428` / `e754581`.
No accepted reference/projection data was edited. Source indices are not derived
from PAR spelling or alphabetical order. Blank and unnamed slots remain allocated.
Inactive links and prefit values are `None`, distinct from K's declared 17/19
caller seeds. Whole-owner Clone remains distinct from source copyParam.

One contract prose discrepancy was resolved explicitly by the user in this chat:
**preserve both source labels.** Source `initialise.C:314–317` overwrites D_AOP with
IPERHARM; lines 409–410 also label `param_iperharm` IPERHARM. K registry entries
88/89 agree. Both labels and the overwritten buffer's retained suffix bytes are
preserved. Central independently confirmed the evidence and corrected 0033 at
`82f6e10`; no production behavior or gate change was needed.

Copy boundaries identify cell, field and class: missing source/destination cell,
missing string terminator, insufficient label storage or unwritten numeric read.
Earlier writes remain available for diagnostics; the destination's `cell()` and
subsequent copies return its terminal boundary. Safe disjoint ownership excludes
buffer overlap. No invalid native probe, fabricated prefit default, rollback,
implicit growth, truncation or native pointer/layout comparison is used. Rust
replacement/drop releases all backing allocations, even beyond a reduced logical
extent; C allocation leaks are not emulated.

## Executed checks

Host: macOS 26.7 arm64, `aarch64-apple-darwin`, C locale. Rust 1.97.1
(`8bab26f4f`, LLVM 22.1.6); supplemental native build: Homebrew GCC 16.2.0.
Actual compiler/backend/runtime/header hashes are in both direct receipts.

The full command, run from the project root, was:

```sh
python3 validation/parameter_storage_port/verify.py
```

It runs `cargo fmt --check`, `cargo clippy --locked --offline --all-targets -- -D warnings`,
debug/release `cargo test --locked --offline`, and the exact direct runner in each
profile. Cargo runs use `rust/.cargo-home` and `rust/target/gmp-cache`, as recorded
in the scripts. Component commands are:

```sh
python3 validation/parameter_storage_port/run.py --profile debug
python3 validation/parameter_storage_port/run.py --profile release
TEMPO2_CLOCK_VERIFICATION_ROOT="$PWD/rust/verification/parameter_storage/regressions" python3 validation/clocks_port/verify.py
```

Only `validation/clocks_port/run.py` and `regressions.py` received the expressly
authorized output-root plumbing. With the variable absent their old paths remain.
No expected result, case, projection or comparison rule was changed. JPL build
cache files also route beneath O when that variable is present. The imported L
receipt retains its original descriptive status string; it is a regression run,
not a new L or O acceptance decision.

| Check | Actual result |
| --- | --- |
| Formatting / Clippy | Pass, including all targets and warnings as errors |
| Debug and release ordinary Rust tests | Each: 23 library + 7 behavior + 1 reference test pass; six explicit probes ignored by the ordinary test invocation and handled by their runners |
| K selected parameter cases | All six cases / 24 ordered events exact, including both allocation arguments, registry, init/full/all, prior mutation, copy and destruction |
| M copyparam_0/1/3/4 | All four cases / eight ordered events exact; complete source and destination backing, labels, flags, values, links and ownership retention |
| Full registry / initializer | All 166 actual enum discriminants, 2,000 slots, 9,616 cells; all initialized 100-byte label and shortlabel buffers match unchanged native routines |
| O supplemental native copies | Negative aSize -7 and shorter labels match exactly, including full retained destination suffixes and all backing cells |
| Rust copy boundaries | Eight cases compare full defined prefixes and block reentry/scientific reads; no native undefined accesses run |
| Comparator/identity controls | Each profile: one positive control and 29 rejecting controls, including field/case/order/extent/value/flag/label/tail/link/boundary mutations, compiler/runner/source/executable identity, no-op/stale launch and deletion of a previous success receipt on failure |
| Direct comparison counters per profile | 664,159 object-field visits; 96,906 exact finite-number comparisons; 591,516 scalar leaf comparisons (overlapping traversal counters, not distinct cells) |
| Clock and table regressions | Each profile: 142 exact J comparisons; eight Rust boundaries; cycle remains unrun. 34,818 finite / 196 nonfinite / 68,256 discrete comparisons; finite difference zero |
| Glob and process provenance | Each profile: 18 helper patterns, three integrated cases; nine supplemental controls and all eight original mismatches rejected; process provenance passes |
| JPL | Each profile: 1,440 executed events, 1,439 exact events plus one accepted boundary; 43,310 finite / 96 nonfinite / 17,098 discrete comparisons; zero finite difference. Three source events remain unreachable under the accepted boundary |
| M1a | All unchanged comparisons, CLI checks and independent F3 diagnostics pass |
| M1b | All unchanged comparisons, resource/recovery/provenance and CLI checks pass through the complete M2 driver |
| M2 | Completes, exit 1, exactly the four historical dependent-column raw-order failures retained; no new failure |
| Protected inputs | All 1,618 recorded files byte-identical; both nested repositories clean at their pins |

Build-time source/compiler hashes are checked again after compilation, then around
each launch. Boundary expectations begin with the frozen M caller states and
apply only the declared Rust-only invalid-storage changes; they do not trust
the observed before/source states. Two controls alter both before and after
caller values/links together and must still reject.

The direct debug/release records are identical after removing only their fresh
launch token/input identity. Exact assignment/copy comparisons have zero numerical
difference; no floating tolerance is applied. Native binary128 bytes are decoded
to exact values, not compared to MPFR object bytes. Signed zero and explicit
nonfinite cloning are separately tested in Rust.

M1a maximum fixed residual difference over its three reference fixtures is
`3.76200923958827030e-25 s` against `1e-12 s`; maximum phase difference is
`4.977176e-23 turn` against `1e-10 turn`. M1b's two fixtures each retain maximum
residual difference `1.13686836941064559950026880243e-13 s` against `1e-12 s`,
and maximum phase difference `1.13686836941064559947971e-13 turn` against
`1e-10 turn`. Detailed stages, units and original budgets are in the redirected
`numerical_errors.json` files. These are implementation agreement checks, not a
new scientific validation or changed model convention.

M2's exact failures in each profile are raw retention order `[true,false]` versus
`[false,true]`, and raw singular-value index zero difference approximately
`3.4641016151377546` exceeding `3.4641016151377546e-10`. The complete original
strings are preserved in `verification/parameter_storage/regressions/m2/open_gate_failures.json`
and match historical evidence exactly. No sorting, solver change or relaxed gate
was introduced.

The strengthened verifier initially exposed a test-harness issue: its injected
build-failure mock also intercepted the new compiler-identity queries. The failure
left no direct/full success receipt. The mock was restricted to that control
and all controls and the full suite were rerun. The failed attempt is retained
in `verification/parameter_storage/attempts/build_failure_control_mock.log`; it
was not a Rust/native numerical regression.

## Actual execution identities

| Artifact | SHA-256 |
| --- | --- |
| Debug Rust test executable | `27757b2f11b06bbf9343b38d2ff2cbcdcd137d5ab2dad9c51e9aba6247e518d0` |
| Release Rust test executable | `b623d5206fcfa4c5d46bc67d7122f42945b4ef2b4c682af686f436697d8fd425` |
| Native supplemental executable (both builds) | `4584fe90efb9b57b5224717b99fb65779fe1f6d2805da0ea140b62398bdf96a1` |
| Debug CLI | `6d931c909548f5d35b58c2d6202bde1cc35fa8c429fc8dd77a36bb3e9162ef70` |
| Release CLI | `568a5e19a2d3a7bb8d652d1a94fc9244d1568e4f521bedf8ef3117c465305354` |
| O direct runner run.py | `e32527c28fa9f17966a0990f11801605b0702fbddfb2dacef4abcd72bbdc0f98` |
| O full runner verify.py | `060493db549b327fea60131c692a0a42b3a4d39043aa26f6545cf8e60e035fed` |
| O controls.py | `437645eb19afbe22b4fdb5249139c95831de722ffa0665c39c93f9eb60a877e5` |
| O native.py | `285a8e09002541da2b252945d56c699bf02dbd8da544fb545ac6f59ebda8e737` |
| O native caller probe.cc | `71f79bcf081cf34a94a3a79a7d6f8810eb0792a361ca51d12b81a97aeef49b61` |

`verification/parameter_storage/{debug,release}_direct.json` records actual paths,
compiler/source/input/executable identities, fresh challenge, output hashes and
native headers/libraries. `*_raw.json.gz` and `*_native.json.gz` preserve complete
executed outputs. `commands.json`, `delivery_checks.json`, regression receipts and
`artifact_hashes.json` tie the runnable artifacts together. Failed attempts remove
the preceding success receipt; the controlled failure test confirms that behavior.
The final executed HEAD is recorded in the final `delivery_checks.json`; subsequent
O commits contain verification artifacts and documentation, not different executed
implementation. Final source identities are rechecked after the regressions.

## Inventory, ownership and remaining work

`SOURCE_CORRESPONDENCE.md` is updated; `module_inventory.json` records all 36 current
Rust source files and their hashes. The twelve existing production unsafe sites
remain five stdio, five typed clock-text and two SOFA sites. **O adds no unsafe**,
no dependency, numerical ABI, rounding mode or arithmetic fusion.

All commits stage explicit O-owned paths. Concurrent P's `validation/preprocessing/` and Q's `validation/par_specialized/`
are outside O's frozen membership and were not staged or modified. Historical K/M/N/J,
other validation inputs and old verification remain protected. The optional clock
verifier changes are limited to the authorized path routing above.

This delivers the released storage/copy foundation and canonical-store integration
for central review. General PAR/TIM/preprocessing, linked-value arithmetic, full
copyPSR/pulsar initialization, broader models, arbitrary caller allocation failures,
other platforms/locales, sanitizers/leak instrumentation and concurrency stress
were not run or implemented. Their scientific scope is unchanged. Native undefined
access cases remain unrun; no portability claim is inferred from the declared-host
native observations. M2 and overall transfer completion remain open.
