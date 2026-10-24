# Assignment Z: complete active planetary ephemeris reference

24 October 2026. **Delivered for central review, not centrally accepted.** This
stage supplies the unchanged native reference for all three active routines in
`readEphemeris.C`: readEphemeris, random2 and gasdev. It changes no Rust code,
scientific convention, native calculation, accepted evidence or numerical gate.
Y retains the Rust ownership window. M2 remains unchanged and unaccepted.

The release is `7946aeee6e7e105fc276b311f1c36a4288767d0d` under decision 0060;
the native pin is `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. All authored changes
are inside `validation/planetary_ephemeris/`. Both independent repositories and
the original 10,003-file accepted-evidence preservation membership remain
unchanged. Evolving central documents, live Rust and W/Y work were excluded
from the initial immutable membership; Rust context uses the release Git blobs.
W's later acceptance does not retroactively alter that frozen membership.
The subsequently released `validation/propagation/` work is also outside it.

## Delivered contract and implementation

`CONTRACT.md`, `fixtures.py` and `inventory.py` froze the complete raw routine
bodies, active branches/preprocessor membership, all repository caller/declaration
occurrences, complete accepted S type map, field/backing/ownership rules,
ordinary build profile and **96 authored cases before native experiments**.
`outputs/frozen/` contains that original freeze. `SOURCE_MAP.md` explains the
full state/caller/dependency correspondence; `GAPS.md` retains classifications.
The frozen prose's gasdev upper line 573 is a typo; its machine-extracted body
correctly ends at 570, as noted in the source map.

The ordinary build compiles raw readEphemeris.C as its own translation unit.
It uses real pinned JPL C, clkcorr.C/getCorrectionTT, tempo2Util/equ2ecl/public
fortran_mod, TKlongdouble, TKlog, observatory/table dependencies, and exact
native global-data/CVS slices. No calculation or JPL call is stubbed; no Rust
is linked. Both the primary undefined-symbol list and utility/link map verify
public binary128 fortran_mod resolution. The private binary64 utility overload
is not visible to the primary translation unit.

H's authored synthetic generator is executed read-only to produce new Z files:
little/big endian, shifted polynomials and absent optional quantities. H's
recording runner is never invoked. The actual pinned DE405 path/hash is recorded
and read through a Z-local symlink. Written resource bytes and every binary,
object, resolved header, compiler/backend/linker/runtime and harness identity are
verified and retained with actual launch receipts. Resource paths are stable;
there is no diagnostic/numeric path-substitution normalization.

`probe.cc` supplies two initialized pulsars and twelve owned observations,
explicit planet/vector sentinels, complete parameter backing, all clock/flag
slots and 500-cell offset arrays. It exports all target cells/tails and checks
every non-target typed field recursively, including pointers and allocated
parameter backing. Padding, raw addresses and unwritten native locals are not
exported. Defined exits have separately observed retained state and completed
call histories. These are controlled caller values, not claims about native
initializer coverage.

The separate observer records actual JPL open/query/close arguments, ordering,
statuses and outputs. Pointer registration exposes every initialized random2
static cell and gasdev cache. It also records actual random-entry counts and
Gaussian v1/v2/rsq attempts without resetting/writing native state. Complete
ordinary projections and raw diagnostics agree for every case; observer-only
fields and exit records are explicitly additional evidence.

## Actual verification

Executed on macOS 26.7 build 25G229, arm64 LP64; Homebrew GCC/G++ 16.2.0 and
Python 3.11.2. Ordinary C++ flags are `-O2 -std=gnu++11 -DHAVE_CONFIG_H` with
the accepted A2 config and binary128 header; JPL C uses `-O2 -std=c11`.
libquadmath, libstdc++ and libgcc identities and actual Apple linker/assembler
paths are in each profile. No fast-math or numerical tolerance was added to
native replay. Coverage adds `--coverage -fno-inline` and uses gcov 16.

| Check | Result |
| --- | --- |
| Final default ordinary verification | 96 cases / 3,540 events, rebuilt twice and run in fresh processes for each build; exact oracle and repeated-build agreement |
| Final observer equivalence | All 96 cases project exactly onto ordinary output; all 96 full observer executions repeat exactly |
| Integrity controls | 31 checks: exact-copy positive control and 30 rejected corruptions, including nested bool/float-for-int, field/action/case order/membership, signed zero, nonfinite class, retained tails, ownership, actual executable/dependency/resource identity and a fresh changed-caller execution |
| Compiled source mutations | All 18 produce numerical/state/diagnostic witnesses beyond a changed hash |
| Measured primary coverage | 3/3 routines, 207/207 executable lines, 106/108 branch edges |
| Coverage/ordinary audit | All 96 instrumented cases agree exactly; zero profile differences |
| Preservation | Original 10,003 protected files and both clean nested repositories pass before/after checks |

Measured primary call counts are readEphemeris **177**, random2 **961** and
gasdev **107**. The two unhit edges are raw line 543's random index-error path
within the frozen nonnegative congruential-state seed domain, and line 561's
exactly zero Gaussian radius. No executable line is unhit. Every measured edge
and its disposition is present in `outputs/coverage.json`; the domain limit on
index-error reachability is explicit in `GAPS.md`.

Independent checks passed for **793 exact random returns**, **921 complete
random-state snapshots**, **107 Gaussian returns**, **30 directly observed
rejected Gaussian pairs**, **33,600 polynomial components**, **5,611 complete
JD conversions/query requests**, **33,666 driver transformation components**,
**1,215 offset components**, **417,792 retained vector cells**, and **173
completed driver call-order histories**. The random table model uses exact
rational arithmetic with explicit binary128 rounding and binary64 rm1/rm2.
Synthetic checks use expanded/differentiated Chebyshev polynomials; rotation
uses independently calculated high-precision trigonometric coefficients.

The independent diagnostic maxima are 4.547473508864641e-13 for raw synthetic
JPL components, 5.551115123125783e-13 for transformed driver components, and
3.885780586188048e-16 for Gaussian arithmetic. The first category combines AU,
AU/day and optional native coefficient/angle units; it is not a physical norm.
The driver category includes position seconds and light-seconds/second.
`outputs/independent.json` states the separate diagnostic envelopes. **These
are bounded algebraic checks, not Rust tolerances or astronomical validation.**
Ordinary/observer/coverage replay uses exact finite representations, including
signed zero, and exact nonfinite classes, statuses and diagnostics throughout.

Mutation witnesses cover exact-one selection, Sun velocity tails, Teph date
terms, the 1e-6-day step, unconditional retained-array differences, SI scaling,
non-one mass flags, mass sign/frame use, Earth's velocity rotation, ecliptic
predicates, right-edge offsets, STL_BAT velocity/failed-scan retention,
zero-seed continuation, random shuffling, cached negative-seed Gaussian behavior
and negative noise warnings. Each compiled patch, binary/profile, case, actual
field difference and raw stream remains local in the mutation evidence.

## Source behavior and limits

The complete original driver behavior includes signed no-op counts; per-pulsar
open/close even without observations; multi-resource/repeated histories;
warning-only noise; source-precision SAT/clock/Teph arithmetic and JPL binary64
conversion; exact target/centre/query order; all nine unconditional six-vector
differences; exact-one loading versus nonzero mass correction; phase values'
inactive Earth adjustment; asymmetric position/velocity frame rotations;
independent first-match offset tables; and ordered exact STL_BAT scans.

random2 initializes on `*idum < 0 || iff == 0`. Any tested first-use seed
initializes; after initialization, zero/positive seeds continue and negative
seeds reinitialize. A negative seed on a cached gasdev return remains untouched.
Central's later 0060 wording clarification is consistent with this original
source contract; no behavior was changed to follow the earlier shorthand.

**Full native JPL calculations remain host-qualified observations.** H already
identified unwritten initializer bookkeeping, misaligned public getters and
out-of-bounds pointer formation. Z leaves those dependencies unchanged and
makes no sanitizer-clean or portable-C numeric claim. Absent nutations has a
defined -3 return with six cleared outputs, followed by witnessed offset/STL
processing. Earlier planetary range/read/seek errors can consume uninitialized
local pv values; their complete-driver numeric paths are excluded rather than
presented as safe successful oracles. Missing/short-open failures are executed
with exact source exit diagnostics and initialized prefix evidence.

No upstream bug was corrected. Other ABI/compiler profiles, additional real
resources, artificial static corruption, overflowing/invalid native inputs,
concurrent file mutation and allocation/I/O fault injection remain unrun.
CALCEPH, whole timing orchestration, propagation, secular motion and reference
phase lifecycle are mapped dependencies/callers, not executed stage coverage.
No Rust integration, M1/M2 regression or whole-core acceptance is claimed.

## Evidence integrity during harness completion

The original ordinary oracle was recorded once and remains byte-identical.
An observer extension added direct Gaussian-attempt/entry records; the verifier
then gained explicit shape assertions for the already frozen field contract.
A fresh-input negative control was corrected to skip the first driver
computation: skipping a redundant second identical call legitimately left its
outputs unchanged. These were harness changes, not source/case/result changes.

`outputs/original-recording/` and `outputs/observer-extension/` preserve the
previous manifests/observer evidence. `outputs/manifest_rebind.json` records
the final harness identity transition and unchanged oracle. Final ordinary,
observer, independent, control and coverage checks use the delivered harness.
No reference value, case, frozen source/profile contract or tolerance was
rebaselined. Generated outputs remain ignored under decision 0039.

## Reproduction and acceptance boundary

From `/Users/Eric.Dong/Downloads/tempo2rust`:

```sh
python3 validation/planetary_ephemeris/run.py
python3 validation/planetary_ephemeris/observer_audit.py
python3 validation/planetary_ephemeris/independent.py
python3 validation/planetary_ephemeris/controls.py
python3 validation/planetary_ephemeris/coverage.py
```

Run sequentially: executions deliberately reuse the fixed Z case/resource
paths. `run.py` always builds and executes, has no no-build shortcut, and never
replaces its oracle. For a fresh evidence checkout, README.md documents the
explicit one-shot `inventory.py` / `run.py --record` sequence and its accepted
A2/S/X/H input prerequisites. This is not a bootstrap for older reference stages.

Authored delivery files are `.gitignore`, CONTRACT.md, SOURCE_MAP.md, GAPS.md,
README.md, this report, inventory.py, fixtures.py, profile.py, probe.cc, run.py,
observer_audit.py, independent.py, controls.py and coverage.py. Only these
explicit owned paths are committed. Generated JSON/JSONL/compressed evidence,
raw streams, binary fixtures, native copies and executables stay local.

Central retains acceptance and any later Rust release/scientific disposition.
This delivery closes Z's requested reference implementation and measurement
work within the declared corpus/profile; it does not declare central acceptance.

## Exact local identities

All paths below are relative to this owned directory unless stated otherwise.
Full compiler/backend/header/runtime/object commands and identities are in each
profile; these hashes identify the actual completed delivery evidence.

| Artifact | SHA-256 |
| --- | --- |
| `outputs/reference.json.gz` | `91e1c049594f007d71ae700cb308419d9b59bcf246f26d5528ee33a508f8374c` |
| `outputs/manifest.json` | `d9cc6d06b5a5b37f5bdb5c288cede9e774ec2a9dc693fa0b5dcab21efe66966e` |
| `outputs/frozen/source_map.json` | `8d82882c994ee3aed698d02d38140e316fd400c7b76120cf79b4f1e13094ef80` |
| `outputs/frozen/cases.json` | `d6332043b9a79ca57411d81199e0999ed8ee16a4bc6cd195bffb23ffda011b33` |
| `outputs/frozen/protected.json` | `9321a78bcf7de138e0168dd9df7c2b402a4315275683649b4ae9a6bcf356351c` |
| `outputs/last_execution.json` | `4530074669d5132aaaa8783e2b7c3815667acbb7e2cb49e4eea46817bad858cc` |
| `outputs/execution_receipts.json.gz` | `9a694d985f29c4a3b8df69fe1f22359c88b6a41f1d01af8feda7a2f0b21cb444` |
| `outputs/observed.json.gz` | `76d88c41746be3a86734700658b484f9f1e464853e30930b09a8f2050479acd7` |
| `outputs/observer_audit.json` | `51d011023db98f93e95881709fc7c070aa89f6e9f503e34d3aa288fe3d122bf1` |
| `outputs/observer_receipts.json.gz` | `ddeafa25ab6d2b3ffe5e2a8ea875cd39a9baac5774299e793267fbbee2e39bdb` |
| `outputs/independent.json` | `8c62a22598bcffb6ecd38ef4f13a9a6d853f786fb37ea3b0447a3be67ffb9400` |
| `outputs/controls.json` | `5a96b60abcf1887043486e5691706b8c6684d7dbc6d375fc977b71df066107da` |
| `outputs/mutations.json.gz` | `82a7c7b0a8a9714d0b4e7410b21ca503c158519f601d64c0cdc5818d9460d2d3` |
| `outputs/coverage.json` | `19bc908ecda42aa3cb68884476995ec41ab3b1a38d609ca82468674b14a93145` |
| `outputs/coverage_receipts.json.gz` | `a0535fee2714bf367acb5b5148fd299b32473c13b0b7e9818cffd6bdf4dc37b2` |
| `outputs/manifest_rebind.json` | `e3100841d853c6d286d7f62bf9c3ea7812152eaef6aa9a9b6562a927c4458b7a` |
| `build/ordinary/probe` | `df4291e207362ff0740435c684c05616d18533322213bb360db81cb08cf1370b` |
| `build/observed/probe` | `94728ca82e32912013c4a821715f79137c47fc851b1576fa8f2c94a3142cd4a9` |
| `build/coverage/probe` | `85074cf1e410a9c46356d2880e9c76207139fcefb0577cd1b2e7e45b57e01d8c` |
| `accepted H authored validation/jpl/run.py` | `890d9f8909e4533546506c72a1d6d8e945584e10c526d8305a85d2778a75896b` |
| `native readEphemeris.C` | `822678396155955e8f55f307b373d1666b4def988d0b21e0762bf6fd5e53ce3a` |

Native JPL source SHA-256 is `d032fc069e9020af37333038d49f078d1ddefeb2ae253f520c17f8881c98960e`;
tempo2.h is `0313b6eaf9f9ae9d7e5cce340379ad4b7068394e5222a01f73949e3a2640de93`;
A2 config is `470c61b68a61504558e76ef772639e46b565a1e4500115d6e3fb6a71286dd69e`.
The real resource is `/Users/Eric.Dong/Downloads/tempo2rust/tempo2/T2runtime/ephemeris/DE405.1950.2050`,
9,316,736 bytes, SHA-256 `86786f07aeddc1c5b6ddb24d8c65930803458938c6a2f920e8ea72a86da7479c`.
