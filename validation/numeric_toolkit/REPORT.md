# Assignment M: numeric utility reference delivery

Prepared 23 October 2026 under release `c03583442294c059768c9727034a6e7554a96108`, decision 0025 and handoff 13. **The bounded native reference is delivered for central review.** It contains 350 fresh-process cases, 604 ordered actions and 1,207 begin/end events: 305 defined-result cases, 20 formatter ABI observations, 24 conservative host-diagnostic observations and one normal source exit. Known invalid accesses/overflows and the nine withheld J experiments were not executed. This is reference evidence, not a Rust transfer or acceptance of the whole numeric/logging subsystem.

Only `validation/numeric_toolkit/` was changed. Rust, central documents, all other references and both independent repositories were preserved. K's corrected bounded reference is now accepted by decision 0026 at `8f92575`; its accepted scope and M's independent utility evidence are distinguished in [SOURCE_MAP.md](SOURCE_MAP.md). Central retains all representation/API/tolerance and acceptance decisions. Rug128, existing budgets, **M2's four exact-order diagnostics**, and the overall transfer remain unchanged/open.

## Reproduction and artifacts

From `/Users/Eric.Dong/Downloads/tempo2rust`, run sequentially:

```sh
python3 validation/numeric_toolkit/verify.py
python3 validation/numeric_toolkit/coverage.py
```

`verify.py` first rebuilds and exactly compares the complete suite, then performs all integrity/comparator controls and compiled source mutations. It never rewrites goldens. The narrower full-regeneration command is `python3 validation/numeric_toolkit/run.py`. `coverage.py` builds a separate O0/gcov executable; it reports reachability, not O2 numerical validation. The scripts deliberately reuse their owned per-case paths, so do not run the suites concurrently.

Initial creation used `python3 validation/numeric_toolkit/inventory.py` and explicit `python3 validation/numeric_toolkit/run.py --record`. The latter is the only golden-generation option; there is no cached/no-build verification mode. Re-recording establishes new evidence and is not an ordinary check. The pinned local TEMPO2 checkout, existing A2 generated config.h, GCC 16 and gcov 16 are required; no network, new package, Rust build or timing resources are needed.

| Artifact | Contents |
| --- | --- |
| [SOURCE_MAP.md](SOURCE_MAP.md), [inventory.json](inventory.json) | Every selected function/overload, both numeric headers and branches, undefined/inactive/deferred dispositions, full caller-field inventory and J/K scope |
| [cases.json](cases.json), [fixtures.py](fixtures.py) | Fixed initialized inputs, exact ordered actions, function membership, result classification and termination expectations |
| [probe.cc](probe.cc) | Declared caller storage, raw native entry points, lossless exporter, full copy/state projection and fresh launch challenge |
| [profile.json](profile.json) | Frozen native/source/config/compiler/backend/header/runtime/harness identities, build argv, executable and link-map digests |
| [observations.json](observations.json) | Complete source numerical/class/text/state/return/exit records, formatter buffer bytes and ordered diagnostics |
| [regeneration.json](regeneration.json) | Current successful complete replay and the actual executable/current runner identities used |
| [verification.json](verification.json) | Positive, artifact, identity, freshness and compiled behavior-mutation results; independent rotation diagnostics |
| [coverage.json](coverage.json) | Fresh instrumented function/case/line/branch counts and separate executed identity |
| [delivery.json](delivery.json) | Final artifact digests and independent-repository status/pins; excludes its own digest |

## Source and execution identities

The build compiles raw Git units at `fda6ad4a9753ad7dc5d291406f2f310a9161423f` plus the exact global-data prefix. It uses the real TKlog state. No A2 object/archive, corrected initializer or instrumentation provides implementation. A2 contributes only its hashed three-line minimal configuration. Source extraction and actual dependencies are documented in SOURCE_MAP; the global slice is `global.C:1–64`, bytes `[0,2085)`.

| Input | SHA-256 |
| --- | --- |
| TKlongdouble.C | `4e8da8ed6c9fd6d60a671e330f09f975e32caba7aacd0c522e8c9a5a0d75a05c` |
| tempo2Util.C | `1f8f7534c98404958d8839250d5107b2af90ad8d790547405a391c5494264765` |
| TKlongdouble.float128.h | `214911b81f82489996d698626cf8d30ab5bcc20099f6b5c5aa089715b85bc55e` |
| TKlongdouble.ld.h | `b9acbfa5a68e3427f83ed3a8ec9551d9fb202d8b73502be219205ad1269a713d` |
| Generated config.h | `470c61b68a61504558e76ef772639e46b565a1e4500115d6e3fb6a71286dd69e` |
| Verbatim global-data slice | `896d8f9193010749e7889046e45cd6096ee8f460865ffda9d8d86b00daa012ba` |
| GCC 16.2.0 driver | `b1f98e3089a1f165872b09e8628e4c92a315f5bc6e9a61ef85aa0c1c2887f363` |
| cc1plus backend | `9d8bd8a5840699af210fd5f46e743031005a7a5bb8bc00080b99c02fbcd0a62b` |
| O2 reference executable | `3c5732b6b3afd5047876703540b042af354481b6b53d72c59c9073a49eaad5fa` |

The current replay's exact executable/link-map hashes, command and all runner hashes are in `regeneration.json`; each mutant and the coverage executable records its own identities. The build records 291 included source/header dependencies, raw and source-path-normalized preprocessed hashes, macro profile, dynamic link dependencies and hashes of libquadmath, libstdc++ and libgcc_s. Build products/maps/logs/preprocessed text stay under ignored `build/`. Executable and link-map byte equality across unrelated build paths is not assumed. libSystem's shared-cache image, loader, kernel and system linker are not separately frozen; their host/OS/link boundary is stated rather than claiming a hermetic environment.

The measured host is macOS 26.7 arm64, Homebrew GCC 16.2.0, C locale. `float` is 4 bytes/24 significand bits; `double` and native `long double` are 8 bytes/53 bits; selected `longdouble` is 16 bytes/**113 measured significand bits**. int is 32 bits, long 64 bits. All exact numerical comparisons apply to this profile. The native numeric model is not Rug128.

## Inventory, measured coverage and observations

The inventory verifies **6 TKlongdouble routines + 18 tempo2Util routines, including both remainder overloads + 11 selected inline wrappers** from the raw source. Every one executes. Both numeric headers' macro/conditional inventories are complete; inactive IEEE754/DDREAL/sun/C branches are explicitly retained as transfer scope, not mislabeled deleted. All 349 pulsar fields and 95 observation fields are generated directly from the pinned struct declarations and included in the copy projection.

| Instrumented unit | Functions entered | Executable lines reached | Compiler branches reached |
| --- | ---: | ---: | ---: |
| TKlongdouble.C | 6/6 | 97/97 | 50/56 |
| tempo2Util.C | 18/18 | 209/209 | 112/116 |
| Selected TKlongdouble.h | 11/11 | 22/22 | 0/0 |
| TKlog.C dependency | 2/2 | 40/40 | 25/30 |

These are O0/gcov counters, not proof of exhaustive valid inputs or portable behavior. Unexecuted compiler branches include unsupported formatter endings/modifiers, exception edges, impossible scan counts above three, warningNum above 50, non-TTY versus TTY behavior, and the storage-full branch at the specific `Too many warnings!` call site. Storage overflow-of-count beyond 30 is exercised at the ordinary error call site; the latter specific call site is an explicit reachable-but-untested branch. Full line coverage does not waive any source-inspected undefined operation. `coverage.json` preserves every unexecuted branch index and exact per-case measured membership.

The complete results are machine-readable. Findings that matter for a later port include:

- `parse_longdouble("1D3")` and `"1d-3"` yield 1, since the selected source uses strtoflt128 without an end pointer. Empty input yields +0 here. Minimum binary128 subnormal `0x1p-16494` is retained; half of it rounds to zero with ERANGE=34; a value just above half rounds to the minimum subnormal with ERANGE still set. Overflow is Infinity, not a fabricated finite value.
- Direct display is lossy `%Qg`: `1.0000000000000000000000000000000002` survives parsing as the next binary128 value above 1 but displays `1`; `1.23456789` displays `1.23457`. The lossless exporter remains distinct.
- Every selected formatter entry point returns zero in the fixed formats. Buffer bytes, file text and stdout match exactly. Uninitialized oargs cleanup and caller/callee va_end handling make these **host/ABI observations**, even though regeneration is stable. No snprintf-style length/capacity semantics are inferred.
- Empty sexagesimal input reaches sscanf EOF and returns zero; nonmatching `abc` returns one turn. HMS mutates colons in its caller input; DMS copies first, removes the first minus wherever it appears, and wraps at most once (`720:0:0` gives one turn). Negative-zero DMS output has plus sign. Adjacent inputs expose the coarse source carries: roughly 59.99995 seconds for HMS versus 59.9995 seconds for DMS, despite seven/five printed decimal places. Exact expressions and adjacent inputs are preserved.
- Both remainder overloads use an **int quotient**, with their original arithmetic types. Half-tie rounding follows the rounded `x +/- 0.5` expression. For example, binary128 immediately below +0.5 still gives nlong=1 because the addition itself rounds to 1. Boundary validity is checked on that expression, not inferred from the original input.
- Vectors preserve binary64 evaluation order and aliasing-visible sequential writes. Rotations retain binary128 trig/intermediates and binary64 writes, source LD_PI and the binary64 obliquity global. Controlled obliquity changes are fixture inputs. Independent binary64 norm/round-trip diagnostics use scale=max(1,max(abs(input component))). The maximum norm difference/scale is 2.3684757858670006e-16; the maximum round-trip component error/scale is 2.970494814274842e-17. These diagnostics are retained in verification.json and set no port tolerance.
- `copyPSR` leaves parameter arrays and most pulsar state intact, copies source observation count, copies jump slot zero even for count zero, and uses the **destination's prior nfdJumps**. It retains fdjumpIdx, jumpSAT/jumpScaled and parameter links. `copyParam` copies source aSize cells and then aSize, retaining link arrays/counts and the capacity tail. The full field projection, source-unchanged checks and sparse initialized bytes make these observable rather than inferred from a few scalars. Changed-byte masks do not claim stores of identical values.
- All five link formulas, unlinked/default/unknown-link cases, a valid SINI→KIN→STIG nonrecursive dependency, repeated reads with unchanged metadata/value arrays, alternate requested array cells and duplicate-link exit are preserved. No recursive evaluation or synthesized recovery value is introduced.
- `displayMsg` has a 50-key case-insensitive process history, distinct from TKlog's 30 stored warning/error strings. DUP1 is emitted once. noWarnings=2 bypasses warning/minor history; negative settings print directly. Quiet does not suppress warnings/errors or key recording, and debug does not affect displayMsg. `logerr_check` resets counters but retains stored strings and key history. Actual channel/order, repeated calls, storage and 360-byte truncation are retained.

The TKlog color-strip loop forms a before-buffer pointer before immediately restoring it; these calls do not dereference it, but the formation is not portable C++ arithmetic. Full diagnostic fixtures are therefore conservatively classified as host observations. This additional source finding was not repaired and does not use a known overread/overflow experiment.

## Integrity and sensitivity

A normal full attempt removes its previous success receipt, compiles fresh raw source and compares stable identities before execution. Every case's directory is recreated, stdout/stderr are new files, and the native process must echo a fresh random challenge into a newly created event stream. Executable bytes and current runner/harness hashes are checked before and after each process and before publishing success. The verifier rejects missing output, stale challenge, missing/extra/reordered cases/actions/events/field arrays/components, changed dictionary field sets/types and all changed scalar/text/state results. NaN payloads, pointer addresses and untouched padding are not invented scientific values.

Only the actual staged source-directory prefix is normalized in ordinary diagnostic text. Source line numbers, text, ordering, counters, truncation and return/termination remain exact. Recorded stdout positions refer to the raw pre-normalization file byte offsets; do not use them to slice the shorter normalized text. Mutant sensitivity additionally removes those offsets and source file/line labels so a source path alone cannot cause rejection.

Final checks include a complete exact-replay positive control, an independently built positive vector record, 21 artifact/comparator controls, 11 identity controls, and two freshness controls (**35 controls in the receipt**, plus the independent positive build). Identity controls use the same normal gate with wrong source/compiler/backend/runtime/header/preprocessed/config/macro/runner/harness identities and a same-path replacement executable. Freshness controls bind the actual control-executable digest deliberately, then test the separate fresh-event gate: a no-output program cannot reuse a previous valid case, and a stale challenge is rejected. These controls do not expose a no-build bypass in the normal CLI.

**22 compiled disposable source mutants** must produce a value/state/return/stream difference, independently of source/executable identity changes. They cover parse precision, display precision, format return/value, HMS conversion, DMS wrapping, both carry thresholds, remainder quotient, both half-tie rounders, dot-product association, overlap write order, rotation sign, parameter copy extent/retained links, pulsar FD/observation copy extent, link semantics/nonrecursion, duplicate suppression and TKlog storage capacity. All mutation sources remain under ignored owned build paths. Canonical source and golden records are unchanged by verification.

## Limits and later decisions

No known buffer overflow/overread, invalid float-to-integer conversion, uninitialized numeric value, overlapping strcpy, undersized destination or withheld J experiment is executed. `inventory.json` records the specific source locations and undefined/inactive/deferred dispositions. No sanitizer, signal, timeout or portable result is claimed for those unrun operations. Fixed-format formatter observations and colored diagnostics have the host-only limitations above. TTY behavior, I/O/allocation failures, arbitrary formats/strings, other platforms/ABIs/locales and alternative numeric-header execution remain unverified.

This suite establishes exact bounded source regeneration and verifier sensitivity. It is not an independent physical-validation oracle or an exhaustive mathematical accuracy study. Norm/round-trip diagnostics are implementation diagnostics; all source numerical budgets and downstream scientific gates remain central's responsibility.

Later central decisions must choose Rust text/format APIs and safe treatment of invalid/ABI-only paths, source-global diagnostic lifetime/routing, checked integer-conversion boundaries, copy ownership/capacity semantics and mixed-precision/public text compatibility. M makes none of those choices. Rug128 remains selected for the existing Rust work; M2 and the overall transfer remain open.

## Commit stages

- `be4b4c9` — `test: add pinned numeric utility reference harness` (harness, fixed fixtures, strict runner, controls, inventory and source/caller contract).
- `5734dc9` — `test: freeze numeric utility reference and mutation evidence` (complete O2 observations, executed profiles/receipts, controls/mutations and fresh O0 coverage).
- `e754581` — `fix: strengthen numeric vector and link caller evidence` (final disjoint/alias/overlap callers, valid nonrecursive dependency, repeated-read/state checks, 22 mutants, and freshly regenerated full evidence).
- The final documentation commit ID is recorded in the delivery message; the receipt excludes its own digest and avoids a self-referential commit hash.
