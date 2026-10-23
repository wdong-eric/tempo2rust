# Assignment P — preprocessing reference

Native evidence under decision 0034, starting from central release e68806e.
Only `validation/preprocessing/` is owned. Central retains acceptance and the
later Rust lifecycle contract. The source, Rug128, accepted reference/projection
data and numerical gates remain unchanged, including the four open M2 failures.
Q's active `validation/par_specialized/` and R's active Rust/storage-port work
are outside immutable evidence membership. O/central changes made during this
assignment were preserved; R now owns Rust under decision 0037.

The reference contains **302 cases, 1,429 caller actions and 3,451 state events**:
288 defined results, 11 normal source exits and 3 host observations. All 15
primary routines have bounded executed witnesses. The 14 identity controls
(including a positive fresh-build control), 9 comparison/launch controls and
16 compiled mutations passed. The final replay, coverage and preservation
receipts below establish the delivered state; central acceptance remains open.

Under the 24 October 2026 tracking instruction, generated validation payloads
remain local. Source, fixture declarations, scripts and concise reports belong
in Git; generated JSON, compressed observations/raw dumps, profiles, coverage
and receipts must not be staged again. Central manages their untracking without
deleting local files or rewriting history. The measurements below describe the
completed execution snapshot, not a new validation run after this policy change.

## Reproduction

From `/Users/Eric.Dong/Downloads/tempo2rust`, with the pinned TEMPO2 checkout,
existing three-line reference config, Python 3, Homebrew GCC 16 and gcov 16:

For a fresh checkout without local baseline outputs, first generate them with
`python3 validation/preprocessing/run.py --record`. Preserve existing local
evidence when it is available, then run the verification sequence below.

```sh
python3 validation/preprocessing/run.py
python3 validation/preprocessing/verify.py
python3 validation/preprocessing/coverage.py
python3 validation/preprocessing/classify_coverage.py
python3 validation/preprocessing/run.py
python3 validation/preprocessing/inspect.py
python3 validation/preprocessing/seal.py
```

Run these sequentially: the native suites share owned build/case paths. The
last ordinary replay restores the full success receipt after negative controls.
`run.py` always builds fresh; `--no-build` is absent. `--select CASE_ID` builds
and checks only that case and does not publish full success. Only explicit
`--record` regenerates the baseline. Default verification cannot rewrite its
scientific observations, case catalog, inventory, initial state or raw archive.
`verify.py` emits the full uncompressed local verification receipt; `seal.py`
archives its exact bytes as `verification.json.gz` and verifies current executed
binaries, post-initial mutation differences, receipt membership and protected
inputs. Its manifest hashes every delivered file except itself. Generated output
is local evidence; running these commands does not authorize committing it.
The existing delivery manifest records the original snapshot at commit 2038c41;
subsequent report-only edits are outside that historical seal. An explicit future
`seal.py` run creates an updated local manifest.

`inspect.py --case CASE_ID` reconstructs that case's complete final state and
ordered deltas for examination. Its decimal binary128 decoder is a readable
view of exact stored bits, not a replacement numeric oracle.

## Source and caller contract

The four primary raw Git blobs at fda6ad4a9753ad7dc5d291406f2f310a9161423f match
0034 exactly. All 15 routines are inventoried. Actual dependencies are complete
raw initialise/readParfile/TKlongdouble/tempo2Util/TKmatrix/TKlog/clkcorr/
tabulatedfunction/observatory units. Only global.C's data prefix and
CVSdisplayVersion suffix are sliced verbatim; profile.json records exact byte
bounds and generated digest. The accepted A2 config contributes three build
defines, SHA-256 470c61b68a61504558e76ef772639e46b565a1e4500115d6e3fb6a71286dd69e;
no corrected A2 implementation is linked and no core dependency is stubbed.

The measured arithmetic is GCC binary128 with a 113-bit significand, under C
locale. Float/double/native long double are 32/64/64 bits, with native long double
having 53 significand bits. Source IFTE_MJD0/KM1 double-literal representations
and extended IFTE_K, source START_FINISH_DELTA and actual compiled field layouts
are exported. The native reference is distinct from Rug128.

The recorded host is macOS 26.7 arm64, with Homebrew GCC 16.2.0. The final native
executable SHA-256 is
`5fb97cc15d5f7cf82f3e0626a36e9a34bcd7d34a494a8164cfbec1b312b425f7`.
Profiles retain the exact compiler/backend/runtime/header hashes and commands;
the final delivery seal checks the normal, instrumented and 16 mutated
executables against their execution receipts.

Real initialiseOne supplies parameter allocation/labels/defaults. The caller
explicitly initializes all prefit/prefitErr cells and observation backing; these
are not claimed source defaults. Default capacity is two pulsars with twelve
initialized rows each. One explicit FJUMP capacity case supplies fifty rows and
2000 fully assigned short flag values, reaching the last safe capacity exit.
The projection includes every selected control, full parameter arrays/labels/
links, full initialized observation records, retained jump/FD/phase arrays and
unused tails, obliquity/global-fit state and logger controls/storage.

SOURCE_MAP.md, PROJECTION.md, TYPES.json, inventory.json and CALLERS.txt document
routine/condition/type/caller mappings and invalid/inactive classifications.
Input/action classifications preceded execution; coverage-driven additions were
defined before their runs. Exact ordered actions and file bytes are in cases.json.
No implicit reset, sort, ideal round-trip or source-defect repair is applied.

## Evidence and integrity

All generated artifacts listed below are local under `validation/preprocessing/`.
The original snapshot has these SHA-256 identities; fresh raw launches contain
new nonces, so raw archives and receipts may change while exact scientific
comparison still passes.

| Local evidence | Original SHA-256 |
|---|---|
| observations.json.gz | `1e44d2446fa37ce22cbe0de36fbd1c2879b0005861c94819181b7e97a13999f2` |
| reference.raw.json.gz | `e6084c26f3c095398c26e1e54aaa13cc743b4ccc8a5cb4d60ed069847eeed157` |
| verification.json.gz | `c53b697217d1c24e04f768b5aebe51ef5aab7879b5d5af1d9e997a535f13a3da` |
| coverage.json | `023828de10c1db04592c7187358c1a0044272f26a194ceadf1025e6bf39898e3` |
| regeneration.json | `2c644869c92312c829d5ba9970a18516ded3e645b98d39c2c0e0be76c71f42da` |

| Artifact | Purpose |
|---|---|
| fixtures.py / cases.json | Complete bounded caller actions, input bytes, expected status and classifications |
| profile.json | Current native/compiler/backend/Python/header/runtime/config/callback/runner identities and build commands |
| initial.json.gz / observations.json.gz | Full common initial state, its per-case digest, ordered complete before/after deltas, exact numeric representations, normalized streams and status |
| reference.raw.json.gz | Actual baseline launch challenges, raw events/streams/status and the profile used for that execution |
| replay.raw.json.gz / regeneration.json | Actual final fresh replay and complete ordered-comparison receipt |
| verification.json.gz | Lossless compressed positive/negative normal-gate controls and full compiled mutation results |
| coverage.json / coverage_classification.json / coverage_cfg.json.gz | Measured per-case function/line/edge counts, every unhit edge's disposition and source-line CFG evidence |
| inspection.json | Lossless raw-event reconstruction, every before-image/delta chain checked, selected readable findings |
| delivery.json | Final current artifact and executed binary identities, preservation checks |

Sparse byte encoding stores an extent and nonzero byte runs; omitted bytes are
zero. Parameter arrays are concatenated in source index order with aSize providing
boundaries. Raw archives deduplicate only identical initial-event text: restoring
that exact line reconstructs each original JSONL stream and its receipt hash.
NaN/sign/bit patterns, array membership and order are preserved. Comparisons use
no new numerical tolerance. Only the declared source/build/directory prefixes in
diagnostic text are normalized; original streams remain in the raw archive.

Normal verification clears stale success before a full attempt, recreates each
case directory, checks the current executable and runner bytes around every
launch, requires a fresh nonce, validates exact action/event/status membership,
and checks full source/compiler/runtime/header/input identities before publishing
success. The frozen profile is comparison data, never an execution receipt for
an arbitrary same-path executable. Callback libraries are hashed separately.

The compiler/link/runtime profile is host-bounded, not a hermetic operating-system
image. GCC runtime libraries, driver/backend, actual included headers and link
map are identified; libSystem/dyld/kernel remain bounded by the recorded macOS
host. Unexecuted alternate backends/platforms/locales are not inferred from this
host's successful replay.

The separate O0 instrumented native build matched all 302 O2 reference records
exactly. Measured primary coverage is:

| Unit | Functions | Executable lines | Compiler edges |
|---|---:|---:|---:|
| preProcess.C | 6/6 | 514/521 | 710/826 |
| preProcessSimple.C | 4/4 | 244/244 | 303/328 |
| units.C | 3/3 | 62/62 | 14/14 |
| updateEpoch.C | 2/2 | 93/93 | 78/84 |

The seven unexecuted lines belong to withheld plugin failure diagnostics.
Every unhit edge is individually classified: 122 compiler exception/cleanup
edges, 17 source-inspected/unrun edges, 5 structurally impossible edges, 2
defined gaps and 1 defined prefix with an unsafe continuation. The defined
gaps are the DM-series warning site's full logger-storage outcome and the
SIMUL-only forward search's deleted second-match guard. A no-DM prefix ending
before stage2 is also unexercised. These are explicit unfinished coverage.

## Preserved observations

1. The full wrapper retains its exact order: enum-order nonrecursive linked
   reads; name/jump/white-file/DM-series controls; nofit then fit; simulation then
   setting; stage1/epoch/forbidden fits; DM loading; T2 then TN then TOBS; flag
   modification; stage2; select/plugin; stage3; final global PAR. IPERHARM writes
   both param_daop and param_iperharm because source labels are duplicated.
   Unknown DAOP does not undo those writes. FD nofit leaves index zero intact.
   The SINI→KIN→STIG chain changes across repeated wrapper calls in source order.

2. Stage1 scans undeleted SAT values with its -1 sentinel and uses nWhite for
   both wave families. Zero-span/zero-count arithmetic remains finite/nonfinite
   as the source computes it. EPHVER overrides requested tempo1. Global obliquity
   is reset per stage1 call; later pulsars can change it. `-last` uses the final
   stored row, including a deleted row. units==100 is an active transformation
   trigger and transform_units itself does not update the units control.

3. Stage2 transforms deleted rows, resets only the four specified vectors and
   appends membership counts. Phase/F0 precedes strict-bound alias-aware offsets,
   then filters/pass, ordinary/FD jumps, repeated -to and float DM lookup. TEL
   matches unequal aliases. FJUMP discovery uses undeleted matching IDs, then
   application matches values under any ID, including deleted rows; ordinary
   jump scales and other unwritten fields retain caller values. Phase linking
   assigns -1 regardless of the prior ID, narrows to double and starts at row one.

4. Selection is sequential mutation. Two A-only simultaneous observations can
   survive `only` because found flags carry between the two directional checks.
   A three-row overlapping chain retains source loop-order dependence. Wrapper
   selection processes all pulsars from inside its per-pulsar loop: a later
   pulsar can be deleted before its SAT transformation would put it in range.
   File EOF replay and PASS's in-place tokenization are retained.

5. White-file tokens, T2 and TN controls remain distinct. T2 EQUAD precedes EFAC
   for each flag, while TN applies its globals before per-flag EF/EQ/SQ. Duplicate
   matching flags and repeated wrapper calls compound errors; origErr is retained.
   TNSQ's last -tobs and UNKNOWN/zero/missing behavior preserve source nonfinite
   arithmetic. TNSECORR cases explicitly assign -tobs for every active row.
   Modified flags use strict double-narrowed SAT bounds, including values that
   collapse to the endpoints on narrowing. DM data remain binary32, use the first
   timestamp plus array index, can repeat the last data row at EOF, delete index
   zero, and preserve the final valid flag write before the capacity exit.

6. Unit helpers transform all four numeric fields without consulting paramSet.
   IFTE constants are promoted as written. xform_mjd uses prefit[0] for any index;
   sequential BPJ processing therefore observes the updated element zero. Equal
   from/to codes still execute this behavior, so they need not preserve distinct
   BPJ prefit cells. Only exact TZRSITE `@` transforms TZR values. Higher/unlisted
   fields and metadata remain retained; no ideal inverse is imposed.

7. Epoch selection uses pulsar-zero START/FINISH controls while scanning p's
   observations: the distinct p=1 fixtures select 58011 for SAT controls and
   58010 for BAT controls. Empty selections preserve sentinel -1 and bounded
   integer truncation. dt is narrowed to double; F3 contributions, selective
   prefit writes, RA/DEC/second-derivative order and DM0-only update are preserved.
   FITWAVES emits diagnostics without moving its epochs. Binary parameters remain
   untouched because the binary update block is literally disabled.

8. The real core plugin loader performs path search, version check and entry
   lookup. The declared fixture callback increments its own static count, adds
   that count to every pulsar's nits and argc to observation-zero uncertainty.
   Two pulsars over two wrapper calls yield four callback invocations and nits=11
   for each. Success with/without version symbols, version mismatch and a missing
   first search path followed by success are distinct records. This is external
   callback input evidence, not general plugin ABI compatibility.

## Limits and acceptance boundary

The no-setdm wrapper continuation reads uninitialized ndm at the call site and
remains unrun. The comment-only file initializes ndm=0; it is explicit input and
not evidence for that no-argument path. Empty/missing unchecked DM files,
TNSECORR without assigned -tobs, malformed scans consuming unwritten locals,
invalid numeric-to-integer conversions, unchecked capacities, dangling offset
flags, empty-last access and global-reader uninitialized scratch paths remain
source-inspected/unrun. No source patch is used to make them run.

Zero plugin search paths leave module unwritten. Exhausted search consumes its
error before the final printf; missing-entry handling closes the module before
retrieving the error string. Those null/lifetime-sensitive diagnostics remain
withheld. TKlog color-stripping calls retain M's conservative host classification;
the alias-warning and DM-series-warning cases are not portable defined results.
The failed-first-path loader text is separately an OS diagnostic observation.

COVERAGE.md distinguishes measured execution, compiler-only edges, structural
impossibilities, unsafe withheld paths and defined-but-unexercised branches.
Defined gaps remain unfinished coverage, not accepted exclusions. Resource
exhaustion, allocation/I/O failure injection beyond the declared exits, sanitizers,
TTY, concurrency, arbitrary callback contracts, broad specialized PAR parsing,
complete timing/fitting and alternate hosts remain unverified. P does not choose
Rust adapters, public APIs, precision, tolerances or milestone acceptance.

The delivery seal verifies the 18 listed accepted K/M/N artifact hashes and clean
TEMPO2/reference nested repositories at their recorded pins. Mutable central
coordination documents and active Q/R files are excluded from that comparison.
Rust tests and the existing M2 gates were not rerun for this native-only stage.
