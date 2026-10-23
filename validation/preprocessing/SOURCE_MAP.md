# Assignment P source and caller contract (declared before native experiments)

This directory owns reference evidence only, at central release e68806e. Raw source
is fda6ad4a9753ad7dc5d291406f2f310a9161423f. Policy 0008 requires preservation
of deterministic defects; 0012 keeps exact SVD order. Accepted K (0026), M (0030),
N (0032) remain read-only corroboration. No Rust or accepted artifact is an output.

`inventory.py` inventories every definition, active control expression, literal
inactive branch, preprocessor line, field, parameter and direct call in the four
raw primary files. Its 15 routine contracts are the semantic classification map.
`cases.json` is generated from fixtures before execution; native coverage later
adds executed witnesses and individually classifies all unhit compiler edges.
Inventory is not a claim of execution coverage.

## Caller ownership and projection

Each process owns two distinct pulsars. calloc initializes their struct backing;
unchanged initialiseOne(...,0,0) supplies real parameter allocations, labels and
source defaults. The experiment then explicitly supplies all prefit/prefitErr
cells (17/19), observation backing and observation sentinels. The initializer does
not supply these values. MAX_PSR=2, MAX_OBSN=12; all 12 rows are initialized,
including unused rows. Counter and string changes are explicit action inputs.
Source-native calloc label backing includes the entire 100-byte capacity.

Snapshot includes every primary-read/written pulsar field and retained jump,
FD, phase and noise partners; every observation byte in all 12 slots plus typed
selected numbers; all 2000 parameter allocations, numeric arrays, flags, labels,
shortlabels, link counts and full link arrays; global obliquity/global-fit,
plugin search state, logger counters/storage and execution controls. Pointers
are excluded and backing ownership is fixed. Complete byte extents, zero-run
encoding and ordered before/after deltas preserve tails, unused initialized
slots, signed zero and nonfinite representations. Numeric interpretations are
host-profile data, never guessed values for unwritten upstream fields.

The initial state is identical between cases and can be referenced by its digest
in comparison records; the full lossless initial state remains in raw archives.
Every setup and native action has an ordered begin/end boundary, process-exit
state preserves a defined prefix on ordinary exit(1), and raw stdout/stderr/status
are retained. Calls within one case use the same live objects and source globals.
No sorting, resetting, or caller reseeding occurs unless an explicit action says so.

## Action and semantic coverage obligations

| Routine | Defined input and critical branches |
|---|---|
| preProcess | Assigned argument arrays; explicit nonempty -setdm for stage2; enum-order links, all option families and override order, two pulsars and repeated calls; final global PAR scalar input |
| useSelectFile | Nonempty complete bounded lines; exact PASS/REJECT/PROCESS, insensitive LOGIC, MJD/FREQ/TOAERR endpoints, flag lists, EOF replay and token mutation |
| logicFlag | Assigned 5 tokens; EXIST/PASS/REJECT; valid numeric threshold with >/< /=; nonnumeric flag and unknown modes retained |
| processFlag | Assigned 6 tokens; all 7 mode strings, exact ID/value, independent tests, deleted rows retained |
| processSimultaneous | Six/seven tokens, malformed early-return; only/toaerr/1/2/unknown, threshold/ties, reversed and repeated histories |
| readWhiteNoiseModelFile | Real complete tokens: JITTER (last tobs), START_TIME_JITTER, TOAERR_ZERO; raw order and defined nonfinite arithmetic |
| preProcessSimple | Explicit source zero stage2 arguments; repeat history |
| preProcessSimple1 | Both wave scans, sentinel/zero span/nWhite0; EPHVER overrides; final stored SAT for last; emulation and units100 |
| preProcessSimple2 | Weighted zero-uncertainty exit; four reset vectors; phase/F0, offset aliases/strict bounds/any flag, filter/pass, ordinary and FD selectors, repeated -to, float DM indexing/trim/capacity |
| preProcessSimple3 | Encounter-order FJUMP discovery, value-only application, deleted rows, retained scales; phase ID assignment, double narrowing, unresolved -1 |
| scale_param | Index0/index>0, all four fields, flags retained, nonfinite arithmetic |
| xform_mjd | Index0/index>0, prefit[0], promoted binary64 epoch constant, errors scaled |
| transform_units | Both directions/equal/other integer codes; all transformed and retained fields, source exponent types, exact @ gate, sequential BPJ |
| updateEpoch_str | All keywords/numeric prefix; psr0 controls for p1, flags1/2, exact limits/binary128 delta macro, deleted/empty/sentinel |
| updateEpoch | F0–F3, selected prefits/errors, proper motions and second derivatives in source sequence, DM0 only, FITWAVES diagnostics, inactive binary |

## Dependencies and unsafe boundaries

Build links complete raw initialise/readParfile/TKlongdouble/tempo2Util/TKmatrix/
TKlog/clkcorr/tabulatedfunction/observatory units. No core dependency is stubbed.
The only slices are global.C data before extra_delays and CVSdisplayVersion to
EOF, copied byte-for-byte; profile records bounds and hashes. Configuration is
three accepted A2 build defines only, not A2 code. Real observatory alias files
are declared runtime inputs, with their process-lifetime warning/cache effects.
Global PAR examples avoid the reader's uninitialized elong/elat scratch and
specialized guard branches. Header inline math and sexagesimal helpers are real.

TKlog's color removal forms a pointer before its buffer; executed colored/full
logger paths are conservatively host observations per M. Formatter cleanup
limitations are inherited if reached, not repaired. Full plugin dispatch uses
real dlopen/dlsym: fixture library is external callback input with a documented
signature and mutation, not a core stub or general compatibility assertion.
A zero-length plugin path list and the all-missing repeated-dlerror path are unrun.

`inventory.json:unrun` gives source evidence for forbidden/invalid storage,
uninitialized locals, invalid conversions and capacity paths. The binary epoch
block stays literally inactive. Defined coverage gaps will be reported as gaps,
not reclassified as undefined simply to close coverage.

## Fixture interpretation and additional boundaries

Caller SAT/BAT fields are MJD numbers supplied independently; BAT is seeded as
SAT+0.25 day solely to distinguish selection fields. No ephemeris or clock is
evaluated and this offset is not a physical barycentric correction. Source
initial units=SI; fixture changes and EPHVER/tempo1 transitions remain explicit.
Frequency is MHz, uncertainty is microseconds, tOffset/-to are seconds and
phaseOffset is cycles divided by F0 (Hz). The experiments do not fit or remove a
residual mean, nor define a reference phase beyond that explicit caller input.
DM table inputs are parsed to binary32; their values are exported via the source
%g flag text. Native binary128 values are not Rug128 approximations.

The one `fjump_capacity` action owns a new, zeroed 50-row allocation, with 40
assigned flags per row and 2000 distinct short values. Its explicit physical
capacity is exported. Stage3 writes the final valid local val[1999], increments
nf to 2000, then takes the source exit before creating any jumps. No observation
flag or jump array is overrun. Other cases retain the default 12-row capacity.

`CALLERS.txt` is the complete pinned Git grep evidence (including declarations,
comments and plugin call sites); `inventory.json` identifies the 15 definitions
and their real direct dependencies. A lexical call site does not assert that
all external callers provide initialized storage. The executable cases in this
directory provide their own fully declared caller contract.

Missing plugin entry is separately withheld: failed dlsym is followed by dlclose
before dlerror is passed to %s. Its error-string lifetime/null behavior is not
promoted into a portable exit golden. Version mismatch is exercised with a
valid assigned version string, and success with and without a version symbol
is exercised. The callback's signature is the source char*(int,char**,pulsar*,int*).
It preserves npsr, increments a static call count, adds that count to every
pulsar's nits, and adds argc to observation zero's uncertainty when present.
The ordered path-search case first tries a missing directory, then the valid
library. These libraries require no installation and remain owned fixture input.
