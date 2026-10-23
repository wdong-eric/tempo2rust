# Assignment L: clock and table transfer

23 October 2026. This is the corrected delivery under decision 0029. Central
withheld acceptance of `189fed3` after finding eight defined glob mismatches.
The correction restores native membership, full-string order and path spelling.
Both build profiles match all 142 executed J cases exactly;
eight separate Rust boundary checks pass and the predecessor cycle stays unrun.
JPL, M1a and M1b regressions pass. **M2 remains unaccepted:** its verifier returned
1 with exactly the four historical dependent-column raw-order diagnostics.
This report does not accept the overall transfer.

The governing contracts are decisions 0023, 0027 and 0029. Decision 0027 records the
user's explicit authorization of the single final interpolation contraction.
The TEMPO2 source pin remains `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.

## Commits and executed identity

| Stage | Commit |
| --- | --- |
| Projection and 32-entry source map, before implementation | `eaf8b29` |
| Source tables, clocks, dependencies, integration and active test replacements | `a2a0911` |
| Fresh-process comparison, mutations and redirected regression runner | `07d27fa0d6a3f09b7f51d8b021b89ba4f57868f1` |
| Original delivery evidence and artifact digests | `4cd1601` |
| Correct pinned glob membership, full-string order and raw path bytes | `e149369` |
| Supplemental native comparisons and correction verification driver | `c5aed3f` |
| Corrected full-run evidence and artifact digests | `aa0dd19` |

The corrected Rust and verification code is committed at `c5aed3f`; both JPL
stage manifests record that execution HEAD. The delivery receipt records the
shared workspace's final HEAD `539299a`: central committed numeric-reference
acceptance documents during the run. All 46 executed source hashes stayed
unchanged. `07d27fa` remains the original run's identity.
New evidence is stored in `verification/clocks/glob_correction/` and
the original L evidence remains unchanged. Exact executed file hashes are in
[`source_hashes.json`](verification/clocks/glob_correction/source_hashes.json).
The direct and supplemental runners embed those hashes in each profile result. Existing
JPL/M1/M2 runners retain their broader manifests in their redirected directories.

| Executable | SHA-256 |
| --- | --- |
| Accepted J executable, used for three new safe integrated cases per profile | `ad01bde230053e437536c204ead2a7235e52c7e08b612910d2ab939024ee900f` |
| Executed debug Rust test probe | `60a1eee8a12e429ea47fab6956730faeee4064c7615ab6369361bee93ac2f8f9` |
| Executed release Rust test probe | `9e6a2ff0b604d1e7a906192ac4ded086ed8410e10beb0949a20abff845672a0b` |
| Final debug CLI | `29c3e35e897cab763d4ef0c04f9534e01b82334a28746c8d31f93a145156957b` |
| Final release CLI | `142c157f6ec199f7d150fcc07f4a1772967abbc71f1ed1fb61908e0adb4f31d1` |

Host: macOS 26.7 build 25G229, arm64, C locale; rustc 1.97.1
(`8bab26f4f`, LLVM 22.1.6). Rug 1.30.0 keeps the approved 128-bit significand.
`host_profile.json`, M1a's toolchain/dependency logs, and J's immutable manifest
record the actual identities. libc supplies fixed-format numeric text conversion
and rendering. Mach-O library versions are load-command metadata; no digest of
the system shared-cache libc image is claimed.

## Source correspondence and implementation

[`../validation/clocks_port/SOURCE_MAP.md`](../validation/clocks_port/SOURCE_MAP.md)
maps all 32 measured J entries: four table functions, four public clock
operations, 18 helpers, three constructors, two lambdas and active `toa2utc`.
[`SOURCE_CORRESPONDENCE.md`](SOURCE_CORRESPONDENCE.md) records the live module map
and retained dependencies. Related helper bodies can remain inline where they
have one caller; a separate function is not required for correspondence.

| Component | Transferred responsibility |
| --- | --- |
| `tabulated_function.rs` | Source 1023-byte chunks, first-byte header, C-string termination, permissive rows, duplicate/decreasing/nonfinite samples, endpoints, bracket search and exact host interpolation |
| `clock_text.rs` | Byte whitespace/token/copy rules; private typed `%lf %lf`, `%lf%n`, `%63s %63s %f` scans and enum-selected bounded diagnostic formatting |
| `clock_glob.rs` | Pinned Darwin flags-zero byte grammar, raw wildcard traversal with dot entries, preserved separators and full-string byte ordering |
| `observatory.rs` | Actual observatory definitions and aliases, independent lazy initialization, alias accumulation/once-per-list warning defect, selected existing geometry calculation |
| `clock_diagnostics.rs` | Shared displayMsg key history and DUP1, used TKlog channels/counters/30-message storage, source truncation, selected CVS version deduplication |
| `clkcorr.rs` | Header index, lazy samples, distinct header/sample provenance, binary32 directed route algorithm, strict ties, invalid-edge retries, reverse hints, explicit chains, first matching cache and all four operations |
| `toa2utc.rs` | Active p/observation/k/l order, bypass 0/2, `jumpSAT[l]` with `jumpVal[k]`, repeated memberships, increment and extra shifted slot, exact destination widths |
| `runtime.rs`, `form_bats.rs`, `tropo.rs` | Actual lookup and process cache integration, stored TT chain/ordered sum, distinct UTC accumulator with actual-query trace, weather consumers using the same table parser |

The clock/observatory databases, loaded samples, sequences and warning history
live in one synchronized process session. An outer lock prevents nested-lock
recursion. A new context, root, extra directory or changed file does not reset
them. Normalized identity returns before clock indexing. Observatory and clock
initialization remain independent. No production reset mode, cycle repair,
weight validation, compensated sum or stricter alternative parser remains.

The TT integration calls active `toa2utc`, uses the resulting stored corrections
and sums in source order. UTC calls `getCorrection` once; its auxiliary trace
comes from those actual evaluations. Public PAR/TIM SAT-jump support was not
added. The full four-stage stored-chain fallback and the accumulator's distinct
UTC-only fallback remain separate. Exhausted lookup still succeeds with zero.

## Authorized interpolation contraction

J's existing GCC `-O2` ARM64 executable has separate subtractions/division followed
by `fmadd` at `tabulatedfunction.C:163–164`. The final Rust expression is
`((x - x0) / (x1 - x0)).mul_add(y1 - y0, y0)`. No other fusion was introduced.

For `table_internal_duplicate`, query `1.9999999999999998`, source event **3**,
end phase, event-array index **7**, J produces `4033ffffffffffff`
(`19.999999999999996`). Separate Rust multiply/add produced `4034000000000000`
(`20`). The user approved the source-matched contraction; decision 0027 records
that one-site exception to 0023. Read-only disassembly and the reproducer are in
[`interpolation_contract.json`](verification/clocks/interpolation_contract.json).
No native clock experiment was executed to obtain that evidence.

## Decision 0029 glob correction

The first delivery used PathBuf component ordering and joining, omitted `.` and
`..` from traversal, treated `^` as bracket negation, and interpreted nested
POSIX class/equivalence/collating syntax. Central's pinned native reproducer
showed eight mismatches among eighteen ordinary patterns. These change scientific
input membership and equal-cost selection; they were not acceptable presentation
differences.

The helper now accepts a complete source pattern and retains path bytes through
traversal and sorting. Only `!` negates a set; the pinned host treats those nested
POSIX spellings as ordinary bracket bytes. Traversal explicitly supplies dot
entries omitted by Rust's `read_dir`, while retaining the native leading-dot
rule. Clock and observatory callers concatenate the source pattern bytes.
Observatory fixed paths and its default-file exclusion also use byte spelling.
The correction uses no new unsafe site or native production dependency.

[`supplemental.py`](../validation/clocks_port/supplemental.py) compiles the
verbatim pinned `globClockFiles` function into an isolated reference helper.
Its function SHA-256 is
`0fbb59b932cf2ce934917d89e4307e1aedc353ecc108c75a439a1b85110b1543`.
Each profile records the compiler, source slice, complete harness, executable,
input and Rust execution identities. The accepted J executable supplies the
three integrated references on new safe temporary inputs; frozen J fixtures and
outputs are not regenerated.

| New check, in each profile | Result |
| --- | --- |
| All 18 original review patterns | Exact native agreement; 35 returned path byte strings |
| Wildcard clock-directory tie | Full-string ordering loads `a b/clock/edge.clk` first and selects 20 s; component ordering would select 10 s |
| Observatory file order | Native first matching duplicate code selects the SPACE clock and 20 s; alternate ordering would select SHORT and 10 s |
| Repeated root separators | Exact index paths, observatory read paths and debug diagnostics, retaining the extra separators |
| Three integrated cases | 36 complete events, 516 finite records and 1,029 discrete records; finite difference 0 |
| Deliberate membership/order/path/selected-value mutations | Nine rejected by the normal comparator |
| Original delivered mismatch records | All eight rejected; the ten original positive controls still pass |

Helper paths compare as full hex-encoded byte strings. Temporary base substitution
occurs only after exact comparison; interior separators, dot components and
spaces remain visible. Integrated cases use the existing complete comparator,
including routes, cache/index order, query values, slot bytes and diagnostics.
Debug/release helper results and all integrated projections agree exactly.
See [`debug_glob_supplemental.json`](verification/clocks/glob_correction/debug_glob_supplemental.json)
and its release counterpart for every returned path and event.

## Direct comparison and controls

The preimplementation [`projection.json`](../validation/clocks_port/projection.json)
enumerates every case/action and observed leaf field. Its accompanying prose
classifies native ABI sizes and binary128 SAT display as metadata. The actual
narrowed binary64 SAT is exact; production precision was not reduced.

| Per profile | Result |
| --- | --- |
| J cases with executed native evidence | 142 exact, including 20 source exits |
| Ordered source events / completed actions | 1,964 / 972 |
| Finite number records | 34,818; bit equality including signed zero |
| Nonfinite records | 196; exact class, NaN payload excluded |
| Discrete fields | 68,256, including finite binary32 weight bits |
| Maximum finite difference | 0 |
| Source-invalid cases tested only in safe Rust | 8 pass |
| Predecessor cycle | 1 unrun; no repaired algorithm |

Debug and release agree on every projected state field, byte, query, message,
terminal prefix and Rust-only diagnostic-storage/boundary snapshot. Unique run
tokens and input-path hashes are invocation identities and intentionally differ.
The probe retains initialized caller sentinels; it does not invent natural C
uninitialized values. All 30 destination arrays and value/destination change
masks are compared wherever J observes them.

Each case runs in a fresh process. Before execution, old per-case output is
removed. The exact test must report one passing invocation and emit its case ID,
unique run token and consumed input SHA-256; executable digests must agree before
and after. The output digest, command and actual process status are saved.
The expected case/action set is checked against the prewritten projection;
missing, extra or reordered cases/events/fields/components reject. Frozen J
fixtures, exporter, raw pinned Git blobs, accepted binary and all real resource
hashes are checked. No J reference is regenerated.

Eight comparator-control tests pass, with positive controls and artifact
mutations covering permissive parsing, f32 precision/strict ties, direction,
fallback, shifted epochs, loaded-cache lifetime, SAT index/extra slot/change
mask, warning suppression/history, exit/boundary classification and retained
prefix, missing/extra/reordered data, finite bits/signed zero/nonfinite class,
stale output and wrong case/source/input/executable/comparator identity. These
are mutations of observed artifacts and identity records, not compiled Rust
algorithm mutation testing. Code and logs state that distinction explicitly.

Additional Rust checks cover raw non-UTF8/embedded-NUL table bytes, typed numeric
prefixes/hex conversion, owner invalidation, source name trimming, glob
components, warning suppression and counters/limited message storage. A fresh
process provenance test in both profiles establishes identity-before-index,
unchanged indexed header identity after a pre-load replacement, a different
loaded-sample digest, survival after removal, and root-A provenance under root B.
The loaded correction remains 7 seconds rather than root B's 99 seconds.
J's lifetime cases separately cover changed roots/extras, before/after-load
copy/replacement/removal, new unindexed files and warning history.

## Explicit adapters and safe boundaries

| Source operation | Rust disposition and tested prefix |
| --- | --- |
| Empty value/start/end | `table_sample_storage` before access |
| Singleton exact or NaN reaching invalid bracket | Earlier endpoint branches preserved; `table_bracket_storage` |
| Header exceeding 256-byte destination | `table_header_storage`; first 256 initialized input bytes retained before the first invalid write, including the would-be overflowing NUL case |
| SAT insertion from count 29 | Count becomes 30, then `clock_correction_storage` before slot 30; all prior slots unchanged |
| Count 28 insertion | Complete source mutation including extra slot 29 and untouched byte 31 |
| Other selected fixed copies, invalid indices, unwritten fields and integer conversions | Named local boundary before invalid access; no guessed numeric value |
| Source `exit(1)` | Distinct `source_exit` and contextual cause, complete defined diagnostic/state prefix; no later script action; same terminal state on reentry |
| Source races and owner lifetime | Safe synchronized ownership; terminal affected session/table cannot retry or reset in the same process |

The eight Rust-only boundary cases have no native numerical golden. All nine
withheld native invalid-access/cycle cases remain unrun.

Diagnostics use raw source-channel byte events. Source file/line text maps to
the counterpart. Temporary case roots have exactly J's original path length,
so TKlog's truncation occurs before path substitution at the same byte.
Only stdout/stderr receive J's Python universal-newline projection; JSON string
and byte fields retain their original contents. The public CLI publishes whole
JSON only after success and drains pending diagnostics to stderr on failure.
Successful public warning strings use the existing UTF-8 presentation, while
the raw sink remains available. Non-UTF8 destination text cannot be represented
by the existing `ClockStep` string and returns `clock_json_text`; this is a
public presentation restriction, not internal Unicode name normalization.

Public resource lists report the bytes used by the cache. Index and lazy sample
identities remain distinct internally; a loaded table contributes its own saved
hash, without reopening a changed path. Cached root-A resources are absolute
when the current context is root B. Unchanged PKS fixtures retain the required
full-file identities. Index hashing reads bytes without parsing sample tables;
this is provenance observation, not eager table loading.

## Integration and unchanged regression gates

The full final driver is [`../validation/clocks_port/verify.py`](../validation/clocks_port/verify.py).
Its corrected subprocess statuses and logs are in `verification/clocks/glob_correction/commands.json`.
Only `validation/jpl_port/run.py` received output/build-directory plumbing;
I's comparators, projections, expected data and supplemental gates are unchanged.

| Gate | Final result |
| --- | --- |
| Rust debug/release suites | 19 library tests, 7 behavior tests and 1 reference-contract test pass per profile; all 5 ignored harness/ownership entries run explicitly by L/I drivers |
| Formatting / Clippy all targets with warnings denied | Pass |
| JPL host per profile | 55 scenarios, 1,443 reference events: 1,439 exact, 1 boundary, 3 unreachable; 43,310 finite and 96 nonfinite records, maximum finite difference 0 |
| JPL isolated / analytic / ownership | All 10 isolated dispositions, 576 exact rational analytic components, descriptor and title-boundary ownership controls pass |
| JPL supplemental | Existing title/legacy-header controls pass; 24 title events and 160 demand events pass, including 6,592 demand finite records and 32 nonfinite records |
| JPL mutation controls / profile comparison | Pass; raw host, isolated and legacy-header projections agree across profiles |
| M1a | 177 scalar-stage comparisons pass; max residual error `3.76200923958827030e-25 s`, max phase error `4.977176e-23 turn`; original parameter/analytic budgets unchanged |
| M1b, via final M2 run | 490 terrestrial and 1,162 perturbed scalar comparisons pass; exact identity/flags/counts/clock destinations/resources pass; barycentric contracts pass |
| M2 | All 29 shared cases / 34 cycles present; 10,666 measurements per profile; exactly four historical diagnostics, exit 1 |

M1b's maximum residual difference is `1.1368683694106456e-13 s` against the
unchanged `1e-12 s` budget; its maximum phase difference is
`1.1368683694106456e-13 turn` against `1e-10 turn`. The maximum TT clock decimal
transport difference is `4.668e-35 s`. The largest 4 ns BAT-response diagnostic
error is `1.4168753360457828e-13 s` against `1e-12 s`. Full signed maxima remain
in [`m2/m1b/numerical_errors.json`](verification/clocks/glob_correction/m2/m1b/numerical_errors.json).
These integrated budgets were not substituted for the exact direct clock gates.

[`m2/open_gate_failures.json`](verification/clocks/glob_correction/m2/open_gate_failures.json)
equals the historical file exactly: raw retention order and first raw singular
value for `dependent_columns`, once in debug and once in release. No extra
failure, tolerance, reordered scientific output or M2 acceptance was introduced.

The corrected runner checked 824 protected reference/fixture/historical files
before and after, including all original L evidence. Their hashes are unchanged.
J and both independent repositories were
untouched; Git status was clean for `tempo2` and `validation/reference`, at
`fda6ad4…` and `7795d53…` respectively. Concurrent parameter/numeric-toolkit/TIM work
was neither staged nor changed by L. Native symbol checks find no retained C
clock or JPL entry points in either freshly executed CLI.

## Active test changes and remaining limits

`resource_tests.rs` replaces strict malformed/decreasing/duplicate-table
rejection with source row/endpoints/IEEE behavior. The obsolete per-context
clock-route test is replaced by J's exact fresh-process route, cache and
fallback cases. `check_terrestrial_resources.rs` now copies clock inputs
explicitly because construction no longer eagerly initializes the clock index;
its assertions test loaded-cache survival and actual originating provenance.
Invalid observatory geometry is tested in a fresh CLI process. Unrelated
JPL/IFTE/EOP recovery requirements are retained and pass.

`compare_fitting.rs` launches each independent fixture in a fresh process to
match the reference's experiment history. Within-fixture NITS and recovery
history remain intact. No production reset was added. Historical reports and
F's baseline ledger were not rewritten.

Coverage remains bounded. The predecessor-cycle case is intentionally unrun;
there is no termination guarantee for that source defect. Full observatory
geometry, general PAR/TIM and BAT input, disabled legacy clock paths, complete
meteorology/IFTE/EOP lifetimes, other timing models and general logging/toolkit
behavior remain outside L. Existing higher-level finite/geometry/timing guards
remain documented adapters. Non-C locales, Linux, concurrent mutation of files
and concurrent-call stress were not validated.

The production unsafe inventory contains **12 sites**: `clock_text.rs:7` declares
the private ABI and lines 35, 43, 57 and 92 call its four typed wrappers; the
existing `stdio.rs` declaration/fdopen/fseek/fread/fclose sites contribute five,
and `get_obs_coord.rs` retains two SOFA sites. Adjacent comments specify owned
NUL-terminated inputs, initialized aligned destination types, fixed formats,
bounded 64-byte header strings and 512-byte formatting output, completed-scan
reads and call-local pointer lifetimes. No pointer escapes those wrappers.
Clock/table arithmetic, graph traversal, storage and synchronization use safe
Rust, with no manual Send/Sync implementation. The retained SOFA and JPL stdio
responsibilities are unchanged; no new numerical package was introduced.

Filesystem fault equivalence is also incomplete: safe whole-file reads do not
model C stdio's partial-read/error state, and the glob implementation is not an
exhaustive libc/glob conformance result. For example, a read failure after a
successful open is currently reported as the load error rather than reproducing
all possible `fgets`/`ferror` prefixes. The exact evidence covers the declared
regular-file scenarios. Public path/diagnostic strings retain the existing
lossy UTF-8 presentation where required by the old JSON schema; internal clock
names, comparisons and fixed slots remain bytes. These limits are not claims
that all remaining scientific or host behavior has been transferred.

Saved subprocess logs preserve their raw final newlines. Git's whitespace check
therefore flags extra blank EOF lines in eight log files; source/document checks
pass. Those logs were not rewritten after their evidence digests were recorded.

Reproduce the complete bounded delivery from the repository root with:

```sh
python3 validation/clocks_port/verify.py
```

It writes only L evidence/build storage and ordinary Cargo targets, uses frozen
J clock outputs without running native withheld cases, and explicitly checks
M2's expected nonzero status and unchanged four-item failure set.
