# Assignment N — native TIM I/O reference

Delivered for central review under decision 0028, from release `8f4d31e`.
Only `validation/tim_io/` is owned. No Rust, canonical TEMPO2, accepted validation,
central contract, numerical budget or M2 diagnostic was changed. Central decides
acceptance and the later Rust input-lifecycle contract.

## Evidence and execution

The final suite contains **157 cases, 299 requested actions and 906 ordered
events**, including **116 defined cases, 35 bounded host observations and six
normal source exits**. Initial state and exit-state events are included; six
terminal actions exit before an end event. All four primary raw blobs match
0028 and all **19 routines** are inventoried and entered by measured coverage.
No unsafe-storage case is executed to manufacture a value.

The native profile is GCC 16.2.0 / macOS 26.7 arm64, C locale, `-O2`, GNU C++11,
with the raw binary128 header and real source dependencies. The arithmetic probe
measures **113 significant bits**, independently of its macro. Native float and
double are 32/64 bits; native long double is 64 bits with 53 significant bits.
Compiler macros record char/short/int/long/long long/size_t as 8/16/32/64/64/64
bits and little-endian order. Observation storage is 7,680 bytes. Runtime
capacity is two pulsars and twelve initialized observations each. The lowered
MAX_OBSN=4 exit case retains twelve physically allocated slots. MAX_FLAGS=40,
MAX_FLAG_LEN=32, MAX_FILELEN=500, MAX_TOFFSET=10, MAX_JUMPS=2000 and both T2 noise
list capacities=100 are exported by the native caller.

The A2 config is reused only as three-line build configuration, SHA-256
`470c61b68a61504558e76ef772639e46b565a1e4500115d6e3fb6a71286dd69e`.
The initializer and corrected A2 engine are not linked. TKlongdouble.C,
tempo2Util.C, TKlog.C and observatory.C are complete raw dependency units;
global.C data before extra_delays and CVSdisplayVersion through EOF are the
only verbatim slices. Real observatory records and aliases are explicit runtime
fixture input. Every actual header dependency, preprocessed unit, compiler and
backend digest, GCC runtime path/digest, link dependencies and link map digest
is in `profile.json`. System frameworks/dyld are bounded by the recorded OS;
this does not claim a hermetic alternative-OS runtime.

Actual final O2 executable SHA-256:
`be9b5e1ce4854b6e958d67bc239a509bf3b9497d1e44b91eeaf1dc011658a743`.
Its actual path is `validation/tim_io/build/probe` under this workspace.
`regeneration.json` records the final newly compiled executable and current
seven runner/harness identities, rather than borrowing the frozen profile as
an execution receipt. `coverage.json` separately records the instrumented
executable, SHA-256
`65a19abcd5374b7088760435f492d8c4dc54adffe4b759a98423256f9c381d83`; each of the 13 compiled mutants has its own executed profile in
`verification.json`.

## Reproduction

Run from `/Users/Eric.Dong/Downloads/tempo2rust` with the existing pinned source,
A2 config, GCC 16 and gcov 16 installed:

```sh
python3 validation/tim_io/run.py
python3 validation/tim_io/verify.py
python3 validation/tim_io/coverage.py
python3 validation/tim_io/run.py
```

These commands were run. The final replay is last because the negative CLI
control deliberately removes a stale success receipt and coverage uses its
own executable. `run.py` always builds fresh; `--no-build` is rejected.
`--select CASE_ID` also builds and compares that case, without publishing a
full-suite success. Only explicit `--record` regenerates the numerical baseline,
case catalog, inventory, profile and raw reference archive. Default verification
cannot overwrite them. Build files and transient per-case raw files are ignored;
all reproduction sources and frozen evidence are tracked here.

| Artifact | Purpose |
|---|---|
| SOURCE_MAP.md / inventory.json | Complete definitions, conditionals, preprocessor boundaries, caller candidates, field projection, history/type conventions and withheld cases |
| fixtures.py / cases.json | Input bytes, action order, caller storage contract, classifications, expected exits and output membership, declared before native runs |
| probe.cc / profile.py | Explicit initialized native caller; unchanged raw source build and measured ABI |
| observations.json | Exact complete ordered states/deltas, result classes/bits, output file bytes, normalized diagnostics and process statuses |
| reference.raw.json.gz | Actual baseline launch challenges, executable identity, raw JSONL events and stdout/stderr bytes |
| replay.raw.json.gz / regeneration.json | Actual final replay raw evidence and successful complete comparison receipt |
| verification.json | 24 identity/comparator/launch controls and 13 compiled source mutation observations |
| coverage.json / COVERAGE.md | Separate per-case measured function/line/branch data, full instrumented differences and every unhit-edge disposition |
| delivery.json | Current artifact digests and executed identities for central review |

Whole initialized observation representations use lossless zero-run encoding:
`size` is the byte extent; each `nonzero` entry is an offset and hex byte run;
all omitted bytes are zero. This includes array tails, unused slots and retained
fields. Typed numeric exports supplement raw state. Initial state plus each
ordered before/after delta reconstructs every later state; exit-state records
preserve partial writes on normal source exits. Caller sentinels and zeroed
backing bytes are declared input, never asserted initializer defaults. File and
pointer addresses are not physical results.

Only known temporary source/build roots and sort log CPU-time text are
normalized. Diagnostic order, other content, raw pre-normalization output and
path-truncation effects are retained. File output bytes are compared exactly.
No new float tolerance is used. Exact bit/byte replay is restricted to this
profile; NaN representation bytes and host observations are not portable
scientific targets.

## Findings

1. **State persists across boundaries.** Direct and wrapper TIM reads append;
   wrapper jumps restart per pulsar, but T2 lists and counts persist. INFO crosses
   includes, later files and pulsars. Includes reset local EFAC/TIME controls,
   share jump state and restore parent controls. The nested case publishes five
   rows and advances the direct caller jump from 7 to 10. Both observation jump
   arrays take jumpVal, while fdjumpVal remains 13. This defect is preserved.

2. **EOF and rejection have observable writes.** A normal FORMAT 1 read writes
   SAT, split SAT and other fields into the next unused slot again at EOF,
   without incrementing nobs. Frequency filtering and SKIP retain intermediate
   writes. An I-prefixed line with five successful scans can accept the caller's
   retained telescope string; this is not a source default. The independent
   first-five-byte SAT split can disagree with full exponent parsing. FORMAT/HEAD
   selection uses the last matching token anywhere, including comment text.
   Legacy formats retain fields that FORMAT 1 initializes.

3. **Ordering of adjustments matters.** EQUAD precedes SIGMA, EFAC and ESET;
   TIME uses its binary64 intermediate; error flooring/deletion and frequency
   rejection remain distinct. T2/global controls are stored but not applied
   here. The deletion test rejects exact equality to the binary64 0.001 literal
   promoted to binary128, deletes below it and preserves above it for both
   pulsars. psr[0]'s deletion path is used for each. The deletion stream leak is
   left intact without exhaustion tests.

4. **Writer output is not an assumed inverse.** Source order for phase/F0,
   selected offsets, repeated -to and -addsat is observed. Positive strict
   bounds, equal-bound exclusions, actual alias lookup, any-match flags,
   numeric/nonflag/punctuation tokens and repeated calls are covered. -selectAll
   is omitted case-insensitively. Legacy output truncates the live filename at
   byte 22. A negative quadrature radicand produces `nan`; the subsequent <0
   check does not reject it. Empty output reads initialized obsn[0] headers.
   Existing files are truncated, and a failed fopen returns without publishing
   a file. Read/write/read cases preserve the resulting flag/time/format changes.
   Full writer results remain host/ABI evidence because of ld_* cleanup defects.

5. **Sort state is separate from data append.** compareObs returns zero for
   equal SAT and for direct NaN comparisons. Deleted rows participate in sorting;
   all record bytes move together. sortToAs sets its latch, and a later appended
   earlier SAT remains unsorted until the caller clears that latch. Tie order is
   the measured host qsort permutation, without a stable-sort guarantee. No full
   NaN sort was run.

6. **JBO observations are compiler-bounded.** Complete type 0/1/2/3 and ignored
   records exercise header/site interpretation, trimming, double byte swaps,
   computed corr/diff, source date arithmetic, replacement count and final feof
   reuse. A final type 0 is logged twice; an ignored final type prevents a second
   unsafe data interpretation. Source corr/diff do not alter stored BAT/SAT.
   The selected data record stores binary64-derived BAT
   `59945.5000014288962120190262794494629` in binary128. Repeated JBO calls replace
   the count while retaining unwritten metadata. The separate O0 build disagrees
   with O2 on individual word diagnostics at readJBO_bat.C:108, including
   1230101 versus 0 for word zero. Exported state and all other evidence agree.
   Full differing observations are retained in coverage.json; this is consistent
   with the raw-record/helper aliasing limitation, not a tolerance discrepancy.

7. **Primitive I/O uses actual source representations.** Actual host detection,
   both explicitly supplied swap modes, first-space filename truncation,
   character order, native record-int reads, repeated opens, file position,
   signed zero, infinities, NaNs and fully assigned fgetc-EOF bytes are retained.
   read_record_int2 deliberately does not swap. Partial fread into its unwritten
   local and closed/null streams remain unrun. Selected date tests preserve the
   +1900 encoding and divisible-by-four rule, including bounded non-Gregorian
   cases; no calendar repair is made.

The TKlog color-removal before-buffer pointer formation is retained as a host
limit. Four MODE-warning case classifications were corrected during dependency
review after the initial draft replay and before final freeze. Plain and
suppressed warning paths are separately exercised. Central's later decision
0030 accepts M at a01d428/e754581 with these limits; that accepted evidence is
read-only corroboration, and all N invocations are fresh. K remains accepted
under 0026 and read-only. Draft harness corrections included a missing C++ header
and moving execution identity checking ahead of output cleanup; their failed
build/control attempts were not reported as native successes.

## Integrity and sensitivity

The 14 identity/CLI controls include a real fresh-build positive comparison,
changed compiler/backend/source/runtime/header/runner identities, actual source,
probe, runner and alias-file byte changes, a same-path forwarding executable,
removed --no-build, and normal CLI failure clearing a prior success receipt.
Ten comparison/launch controls reject missing/reordered cases, missing events
or fields, altered classes/status/diagnostics, missing output files, stale nonce
receipts and absent launch files. Actual executable and runner bytes are checked
before and after every invocation; full input/runtime identity is checked at
build acceptance and before successful receipt publication.

Thirteen disposable compiled source mutations cover format selection, retained
INFO, error order, recursive jumps, the fdjump bug, writer inversion and filename
mutation, sort latch, a valid descending comparator, JBO byte swapping, date year,
record-int result and character order. Every mutant fails the normal identity
gate and independently yields a state/result/file/status difference. Source-file
names alone cannot satisfy the mutation comparator. Canonical sources are never
edited and no invalid comparator sort is introduced by the mutation.

## Limits and completion boundary

Measured primary coverage is **577/580 executable lines and 569/723 compiler
edges**. All 154 unhit edges have explicit dispositions. The O0 build matches
154 complete cases; three JBO diagnostic-only differences retain their full
evidence. Current identities of all 15 native binaries (O2, O0 and 13 mutants)
were rechecked, and both raw archives were cross-checked against every ordered
event and normalized stdout/stderr byte.

This is exhaustive routine membership and a broad bounded case/branch corpus,
not exhaustive inputs or portable C++ behavior. See COVERAGE.md for measured
counts and every unhit compiler edge. All source-invalid storage, include cycles,
failed-realpath consumption, unchecked capacities/string overflows, invalid site
or float-to-int conversion, natural unwritten fields and integer overflow remain
source-inspected/unrun. Allocation failure, TTY output, alternative OS/ABI/header,
parallel library use, sanitizers and resource exhaustion were not tested.

General preprocessing, selection/noise-model processing, specialized PAR,
production initialization/storage policy, Rust APIs and later safe dispositions
remain for central's later stages. Existing Rug128 representation, numerical
budgets and all four M2 exact raw-order diagnostics remain unchanged. Both
independent repositories retain their pins and clean states.

Harness and pre-execution projection commit: `266a1d3`
(`test(tim-io): add pinned native reference and integrity harness`).
Evidence commit: `e11455d`
(`test(tim-io): freeze exact replay mutation and coverage evidence`). Each commit stages only explicit
owned paths. The report is delivered in a subsequent documentation commit.
