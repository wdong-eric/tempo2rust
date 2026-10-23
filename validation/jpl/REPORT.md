# Assignment H: JPL reference preparation

Delivered 23 October 2026 under decision 0011 and contract commit `173f0fb`.
Only `validation/jpl/` was written. Canonical `tempo2/`, independent
`validation/reference/`, Rust, fitting, coverage and binary evidence were not
modified. Other owners' concurrent commits were preserved.

The delivery inventories all **12 implemented JPL functions**, the declaration
without an implementation, and the raw Sun pointer macro. It provides **55
ordinary scenarios / 1,443 events**, ten isolated sanitizer/assertion scenarios,
the unchanged F/D prior probe replay, reproducible inputs, full ordered output
observations and executable mutation checks. This is reference preparation,
not a Rust port or milestone acceptance.

## Essential interpretation

The canonical initializer leaves `pvsun_t`, Sun components and some interpolation
bookkeeping unwritten. Numerical experiments explicitly supply controlled values
for those fields **after** the unchanged initializer. A separate natural-init
probe records status only. Neither is an initializer correction.

An additional source issue was found: UBSan diagnoses `jpleph.c:625` forming
`&eph->ipt[15][0]` outside the 15-row array before the pointer is reassigned.
This occurs even with controlled initialization and enlarged caller arrays.
Consequently, **full `jpl_state`/`jpl_pleph` numbers in this delivery are
repeatable host observations, not portable C numerical goldens**. The defined,
initialized `interp` helper calculations are distinguished from these calls.
No numerical result is fabricated after a sanitizer failure or from ordinary
`jpl_pleph`'s failed uninitialized local arrays.

## Artifacts and reproduction

| File | Purpose |
| --- | --- |
| `SOURCE_MAP.md` | Every harness/module counterpart; binary and packed layouts; indices, components, units, caches, output mutation and undefined-state distinctions. |
| `coverage.json` | Complete function matrix, requirement mapping and explicit unexecuted/nonportable branches. Validated against names extracted from pinned source. |
| `probe.c` | Observational harness including unchanged canonical C; explicit controlled-state command, libc fault injection, file operations and byte exports. |
| `run.py` | Pinned-source extraction, declared binary generation, execution/repetition, oracle comparison, round trips, sanitizer isolation and mutation checks. |
| `analyze.py` | Source-contract assertions, file/cache interpretation, paired byte-order checks and exact rational Chebyshev diagnostic. |
| `inputs.json`, `cases.json` | Input hashes/specifications and exact ordered commands with `$INPUTS`/`$REAL` markers. All binary fixtures are deterministically generated. |
| `outputs/reference.json` | Full observations, finite decimal/bit pairs, explicit nonfinite classes, statuses, sentinels, cache hex, I/O counts and actual fread return counts. |
| `outputs/checks.json` | Toolchain, flags, source/harness hashes, case counts, repeatability, artifact/source/coverage mutations. |
| `outputs/analysis.json` | Numerical scales, semantic assertions, failure/lifetime summaries and finite/nonfinite API comparisons. |
| `outputs/isolated.json` | Exact isolated failure contract: return status, failure kind, last completed stage, commands, compiler/flags and pinned source-location expectations. No trace text or post-failure values. |
| `outputs/isolated_diagnostics.json` | Latest sanitizer/assertion summary text, kept as diagnostic evidence outside exact reproducibility comparison. |
| `test_isolated.py`, `diagnostic_fixtures.json` | Regression tests using original unsymbolized summaries and central's symbolized replay; wrong failure kind/stage and source-location mutations. |
| `outputs/prior_replay.json` | Exact earlier probe source, hash, command and fresh canonical stdout; F's files remain read-only. |

From `/Users/Eric.Dong/Downloads/tempo2rust`:

```sh
python3 validation/jpl/run.py
python3 validation/jpl/analyze.py
python3 -m unittest discover -s validation/jpl -p test_isolated.py -v
```

The first command builds and executes, compares against the checked-in reference
and manifests, repeats every ordinary scenario in a fresh process, and runs the
mutation/isolated probes. The second checks semantic assertions and writes the
derived analysis. The third runs the focused isolated-diagnostic regression
tests. To deliberately regenerate the reference from canonical C:

```sh
python3 validation/jpl/run.py --record
python3 validation/jpl/analyze.py
```

Generated sources, kernels, executables and disposable mutants stay in ignored
`validation/jpl/build/`. No downloaded dependencies are needed. Runtime tools:
Python 3.11.2, Homebrew GCC 16.2.0 (`gcc-16 -O2 -std=c11`, assertions enabled,
no fast-math), Apple Clang 21.0.0 for ASan/UBSan, macOS 26.7 arm64 LP64.
The prior probe uses its original GCC `-O2` profile and directly links the raw
canonical source. Full commands are in the JSON evidence. A different host is
not implicitly accepted by these bitwise, host-specific comparisons.

Canonical identity:

| Input | SHA-256 |
| --- | --- |
| `jpleph.c` at `fda6ad4a9753ad7dc5d291406f2f310a9161423f` | `d032fc069e9020af37333038d49f078d1ddefeb2ae253f520c17f8881c98960e` |
| `jpleph.h` | `c04f5f1b0fc961868ebd3ca0c4af6d9e5f2fca045689c963865484d581e1809d` |
| `jpl_int.h` | `d595b3aa8727567539cf5f73ef0977ca487237f2f2356538e1f5586e24315f72` |
| Real `DE405.1950.2050`, 9,316,736 bytes | `86786f07aeddc1c5b6ddb24d8c65930803458938c6a2f920e8ea72a86da7479c` |
| Same DE405 truncated by 4,000 bytes | `c91ca9f3b91c1ca2e890cef3eb8e784bc012966b0c4621ecb4c4d5fb14607042` |

The source hash, original/truncated resource hashes and unchanged probe hash
are checked against F's `validation/coverage/native_replay.json`. Its original
stdout `0 -2 0 0 0` is freshly reproduced in H's directory. F's retained-reader
`0 -2 -2 0 0` remains prior evidence, not a fresh Rust execution here.

## Findings for the later port

### Cache-before-read and failures

The original DE405 sequence is JD 2450000.5 → end−8 → end−8 → JD 2450000.5,
with target Earth relative to SSB in pleph and corresponding initialized direct
state requests. Both API layers return **0, −2, 0, 0**. The failed direct-state
call leaves caller sentinels intact, changes the cached record to 1141 and
partially overwrites the coefficient buffer. The repeat performs no additional
seek/read. Since the file is big-endian, the short-read prefix is not swapped;
the remaining buffer contains older host-order coefficients.

The repeated direct-state Mercury x observation is
`−3.1529565638093364e+129 AU`. The repeated pleph Earth vector includes
`4.059193595025865e+299 AU` and a **+Infinity** velocity component. Failed pleph
output is suppressed; its local arrays are unwritten. The subsequent successful
observed outputs are retained with the full-call portability caveat above.
The direct EMB/Moon combination matches the successful pleph observations:
**17 finite components have zero absolute difference, and one +Infinity class
matches exactly**. No `Infinity−Infinity` is treated as a finite zero error.
Returning to the good record restores all observed components exactly.

Synthetic little- and big-endian sequences demonstrate the same early cache
commit, skipped byte swap, repeated success and recovery. Restoring the file
while the bad record remains cached does not force a reload. Switching to a
different record and back does. An injected seek failure similarly gives
`0, −6, 0, 0`, reusing the old buffer under the new cache number. A read failure
before the first successful record uses the allocation's zeroed coefficient
buffer on the repeat. No source cache correction was applied.

### U1: file identity and lifetime

U1 is an **unenforced safety precondition** of validating supplied bytes then
reopening a path. It is not proof of a race exploit or a crash. H's deterministic
demonstration substitutes valid synthetic B after bytes A were prepared and
before native open: the result comes from B. No scheduling race is needed.
The production Rust wrapper was not executed or changed.

After native open, path replacement (`rename`) leaves `fstat(FILE)` and the path
on different inodes: uncached reads still use the original file. Overwriting
the same inode with `pwrite` preserves identity: cached queries retain old
values, uncached/reloaded records see the new data. Close/reopen sees replacement
data in either case. The recorded inode-equality relations avoid unstable inode
numbers while proving the setup.

Source stdio buffering also matters. After loading a 6,600-byte synthetic record,
the adjacent-record truncation case retains **680 readable bytes** in stdio even
though only **137 bytes remain on disk** in that record. The actual call is
`fread(size=8, count=825)` and returns **85 complete items**, with statuses
`0, -2, 0` and identical cache bytes on retry. The delivered cache prefix equals
those observed 680 bytes, followed by the old buffer suffix.
Distant-record cases avoid that read-ahead: both 136-byte and 137-byte cuts return
17 complete doubles; the latter additionally concerns a partially read element
whose C value is indeterminate. Raw bytes are observational evidence, not a
portable value for that element. libc/OS buffer behavior is explicitly scoped
to this host.

An immutable file snapshot would remove observed mutation/error paths. Central
independently reproduced the adjacent-record result from the canonical source
and H's probe, then directed the later port to **retain a minimal bounded C-stdio
I/O wrapper while moving JPL headers, state and interpolation into Rust**. A
Rust `File`/`BufReader` substitution must not silently change the host stdio
behavior. Validation must still cover the actual consumed state and the original
cache transitions. The wrapper interface and its justified unsafe invariants
belong to the later central release; H implements no production wrapper.

### U2 and other undefined boundaries

U2 is now supported by both layout and isolated UBSan evidence: AU offset 28
and EMRAT offset 36 are not aligned to eight bytes. The generic getter's cast
and dereference is invalid even though compiler-supported packed member access
can load the field. Both probes terminate under instrumentation; no unaligned
numeric getter value is exported. Safe byte decoding/field access can preserve
field values and selector meaning without copying the invalid typed access.

The ten isolated cases additionally establish the line-625 IPT pointer issue,
NaN/tiny-step record-conversion errors, a large coefficient-offset invalid
access, the `ncf < 18` assertion, insufficient `nut[4]` storage for six components,
and unterminated numeric title parsing. These are failure classifications, not
successful numerical solutions. The transient `pv[14]` formation inside pleph's
13-row local array, position-only EMB copying unwritten velocities and failed
pleph locals are separately source-inspected limitations. No MemorySanitizer
claim is made.

### Layout, selection and interpolation observations

Both file byte orders, pre-430/extended headers, count below/above 400 and the
special 400-name scan are exercised. Invalid extended offsets are zeroed by the
source. Title version wins over the disk integer. The source's EMRAT comparison
accepts NaN; zero AU is not rejected and produces nonfinite arithmetic. These
are observations, not proposed new valid scientific input domains.

The suite preserves target-equals-centre success before validity/range checks,
optional targets' centre bypass, the Earth/Moon special pair, SSB/EMB mapping,
nonzero velocity semantics, and direct-state list/barycentric behavior. Exact
record boundaries select the previous record with t=1; subinterval endpoints,
adjacent binary64 dates, repeated epochs, forward/backward order and increased
derivative demand are recorded. Only interval counts 1,2,4,8 are processed.
All optional outputs share one buffer; loop ordering can leave a mixture from
several requests. Six-component calls require six caller slots.

The generic getters and constant-name/value routines retain their actual
selection and partial-read behavior. All initializer status codes are observed,
including **non-null handles with −7/−8** for value/name read failures. Fault
injection covers otherwise impractical seek/allocation failures; real missing,
short, truncated and malformed files cover ordinary I/O paths. Successful init
resets the global error. Closing a handle does not reset it.

## Actual verification

### Central-review reproducibility correction

Central reproduced every ordinary numerical event but found that ASan summary
formatting differed with symbolization: for example, the same coefficient-offset
BUS failure appeared either as an executable offset or as `jpleph.c:373 in
interp`. Exact comparison of that presentation was an H checker defect, not a
TEMPO2 result discrepancy.

The corrected gate derives the failure kind from the observed diagnostic and
compares status, kind, completed stage, commands and build profile exactly.
Reported source locations must match the pinned source: AU/EMRAT 77, IPT pointer
625, record conversion 582, Chebyshev assertion 326, coefficient read 373,
six-component write 403 and title parse 726. UBSan/assertion locations are
required; ASan source locations are checked when present because unsymbolized
summaries legitimately omit them. Unknown diagnostics fail classification.
Symbolized/unsymbolized summary text is saved separately in
`outputs/isolated_diagnostics.json` and is not an exact oracle.

The stable contract was migrated from **54162de's original observations**, not
re-recorded from central's latest trace. `diagnostic_fixtures.json` preserves both
presentations with input provenance/hashes. Nine focused tests cover their
equivalence plus wrong failure kind, wrong completed stage, wrong source file or
line, missing required locations, unknown diagnostics, changed return status,
dropped cases and changed assertion conditions. The ordinary numerical reference,
its classifications, inputs, canonical sources and all existing numerical gates
remain unchanged. The default full runner and analysis were rerun without
`--record`; the original numerical reference remains byte-identical.

### Verification results

- Unmodified source hashes and resource hashes checked; both independent source
  repositories clean at final inspection. No unrelated failures were encountered
  or repaired. Other owners' work was not tested by H.
- All 55 ordinary scenarios repeated in fresh processes with exact structured
  equality across 1,443 events. Default regeneration check passed against the
  recorded reference/manifests. Every finite decimal round-trips to its bits;
  nonfinite classes compare exactly, without NaN payload comparisons.
- Ordinary paired little/big-endian query outputs agree exactly. Short-read
  mixed-byte results deliberately differ and are preserved.
- The initialized helper has **576** component comparisons against explicit
  exact-rational Chebyshev polynomials and analytic derivatives: maximum absolute
  difference **0** for position, velocity and acceleration on these dyadic inputs.
  Maximum observed magnitudes are **311.3125**, **14811.125/day** and
  **431353.75/day²** in synthetic coefficient units. Zero-step helper cases
  separately export NaN/infinity classes. This is a bounded implementation
  diagnostic, not scientific validation of an ephemeris or a general error bound.
- Nine artifact mutations detected: indices, units, class, missing/reordered
  steps, component count, cache location/bytes and status. Five compiled disposable
  source mutants detected: cache update timing, AU scaling, optional dimension,
  target/centre sign and finite-to-NaN arithmetic. Removing a function's coverage
  row is also detected. Canonical source remains unchanged.
- Ten isolated instrumented processes reach their declared invalid/terminating
  classifications. Their expected failures are verification evidence, not
  sanitizer-clean claims. GCC and Clang profiles are recorded separately.
- Nine focused diagnostic-comparator tests pass, including acceptance of both
  recorded trace presentations and rejection of wrong failure kind/stage.

No Rust tolerance is adopted. Exact equality here checks repeatability and
artifact integrity on the declared host. It is not a proposed cross-platform
floating-point acceptance budget.

## Remaining limitations and central decisions

The complete gap list is in `coverage.json`. Other hardware/operating systems,
other real JPL resources, concurrent mutation/global-error races, arbitrary
malformed layouts and pointer misuse, actual memory/disk exhaustion and leak
instrumentation remain untested. Synthetic optional quantities establish
encoding/algorithm behavior, not real DE430/TT−TDB scientific validation.

Central review is needed for:

1. Undefined-source dispositions: unwritten initializer fields, U2, the newly
   diagnosed IPT pointer formation, pleph's local arrays and caller buffer
   requirements. Recommend safe ownership/indexing and explicit initialization
   with each observable adapter difference recorded; no invented portable value
   for undefined accesses.
2. The bounded C-stdio wrapper contract, file/reader ownership, validation of
   consumed state and unsafe invariants. Central has selected retention of the
   host stdio boundary; an immutable snapshot or Rust File/BufReader replacement
   is not the released direction.
3. The Rust interface set and its failure/result representation, including
   partial-init handles, exact status ordering, stale cache results and defined
   nonfinite classes. Deterministic cache-before-read behavior remains required.
4. Numerical gates, accepted resource/layout domain, crate ownership after E,
   and removal of obsolete native-reader machinery in a later released stage.

H's source evidence and reproducible exporter are delivered for review. No JPL
port, M2, binary-model milestone or overall transfer acceptance is asserted.

## Commits

`9917229` (`test(jpl): add canonical reader probes and declared fixtures`)
contains the probes/manifests/correspondence. `c642f3a`
(`test(jpl): record source observations and reference report`) contains the
observations, actual checks and initial report. The subsequent documentation
commit records central's independent confirmation and C-stdio direction; its
identity is returned in this chat. That update changes no executable probe or
numerical observation: the function/case coverage check was rerun and its
documentation hash refreshed. A subsequent bounded correction fixes isolated
diagnostic comparison and adds the focused regression evidence described above;
its commit identity is returned with the updated report in this chat. Only
explicit `validation/jpl/` paths are staged. No changes to another owner's history
are included.
