# Assignment N: source and caller-state contract

Written before native experiments. Authority: decision 0028, raw TEMPO2 revision
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`. This directory owns only reference
experiments. `inventory.json` enumerates all 19 definitions, active lexical
conditions, preprocessor boundaries, field accesses and raw caller candidates.
Comment-only callers are marked lexical candidates, not execution evidence.

| File | Complete routine membership |
|---|---|
| readTimfile.C | readTimfile, readTim, writeTim, removeCR2 |
| sortToAs.C | sortToAs, compareObs |
| readJBO_bat.C | swap8(double*), readJBO_bat, date2mjd, swap4, swap8(char*), makechars |
| read_fortran2.h | open_file2, close_file2, read_character2, read_int2, read_float2, read_double2, read_record_int2 |

All four files are compiled unchanged, including the header through readJBO_bat.C.
Raw TKlongdouble.C, tempo2Util.C, TKlog.C and observatory.C supply real numeric,
diagnostic and alias behavior. global.C data and CVSdisplayVersion are verbatim
slices; no initializer or timing engine is linked. The A2 three-line config and
binary128 header selection are build inputs, not A2 corrected algorithms.

## Storage projection

Every process starts with explicitly caller-initialized pulsars, observation
storage and F0 storage. These are experimental inputs, not source defaults.
All bytes of observations are initialized before use. The harness records the
whole observation representation (lossless zero-run encoding, including tails
and padding), plus typed numeric fields and full string-array bytes. qsort's
whole-record movement is therefore observable, including unrelated retained
fields. Every allocated slot is projected, including the current unused slot.
Snapshots are initial state plus ordered before/after deltas; they reconstruct
complete state after every action. Normal exit also exports partial writes via
an atexit callback. Counts never authorize reading uninitialized memory.

Pulsar projection: name, nobs, sorted, fixedFormat, jboFormat, deleteFileName,
noWarnings, fitMode, nPhaseJump, phaseJump/ID/Dir, nT2efac/equad, complete
T2efac/equad flag/value arrays, T2globalEfac, nToffset and complete tOffset,
tOffset_f1/f2/t1/t2/Site/Flags arrays, F0, jump/fdjump caller counters. Read-only
controls are included before and after. Runtime and compile-time capacities,
all numeric widths, binary128 significand, logger flags/counts/buffers and
primitive stream position/swapByte2 are recorded. FILE pointer addresses and
process addresses are not numeric contracts. Static INFO, diagnostic suppression
and observatory alias cache are tested through later calls; private locals are
projected by their effects, not rewritten for instrumentation.

Canonical initialise.C inspection: nobs=0 (64), fixedFormat=0 (140), delete path
NONE (144), nPhaseJump=0 (154), noWarnings=argument (156), fitMode=0 (157),
nToffset=0 (164), jboFormat=0 (169), noise counts=0/global=1 (187-189), sorted=0
(240). Its observation allocation leaves most fields unwritten; this stage does
not call it or claim arbitrary observation sentinels as its results. Explicit
nonzero retained fields distinguish reads/writes from absence of assignment.

## History and type boundaries

readTimfile resets local jump and fdjump per pulsar, but does not reset nobs or
noise lists. psr[0] supplies JBO and deletion controls for every pulsar. Direct
readTim and INCLUDE append; includes reset local scalar controls, retain shared
jump state and static INFO, then restore parent controls on return. INFO also
survives repeated files and pulsars. sortToAs sets a latch; append does not clear
it. compareObs compares SAT only; ties have no stability promise.

FORMAT/HEAD selection scans tokens across the entire file, case sensitively;
the last matching token wins. fixedFormat skips fgets chunks and forces legacy.
HEAD seeks a whole case-insensitive TOAS line. FORMAT 1 records split the first
five SAT bytes into sat_day and the remainder prefixed with 0 into sat_sec,
independently parsing the full SAT. Frequency/error/phase controls are binary64.
-addsat scans binary64 then divides in binary128; writer parses the same string
as binary128. TIME accumulates and divides in binary64 before extended addition.
PHASE uses binary64 1/SECDAY then extended addition. Both jump arrays use jumpVal.

Corrections: @ prefix or case-insensitive bat -> 0/0; exact STL/STL_FBAT -> 0/1;
all others -> 1/1. Error order is quadrature, optional sigma, efac, optional eset,
then time, floor and error deletion; frequency/skip reject count increments but
retain partial writes, INFO insertion and origErr. T2/global values are stored,
not applied here. Flags use a character hyphen scan and literal spaces.

Writer compares exact tempo2, reverses phase/F0, selected offsets, -to and
-addsat sequentially, uses strict offset bounds and any-match flag selection,
and omits case-insensitive -selectAll. Original error is divided by efac then
quadrature is subtracted. Negative radicands give NaN; the subsequent <0 test
is false. Legacy writes truncate fname at byte 22 in caller storage. No identity
round-trip assumption is made. Empty nobs still reads initialized obsn[0].

JBO uses 212-byte records, swaps a 4-byte type and 25 eight-byte words (except
text word 13 for data). Its final feof iteration reuses and re-swaps the prior
type; fixtures end with a complete ignored record or type 0, so final reuse is
bounded. Local nobs replaces the published count. BAT is computed in binary64
then converted to binary128; corr/diff affect diagnostics only. date2mjd adds
1900 to encoded year and uses divisible-by-four rather than Gregorian rules.

Fortran readers retain global FILE/swap state. open truncates mutable filename
at the first literal space and detects actual host byte order. Controlled 0/1
swap inputs are not executions on another host. read_record_int2 uses native
fread without swap. Character reads reverse order only at swap=1. Union reads
and EOF-to-char conversions are bounded host observations, not portable goldens.

## Pre-execution classifications

- Defined results: bounded complete text records, initialized partial destinations,
  bounded helper/date arithmetic, comparator calls including direct NaN, unique
  finite sorting and explicit initialized caller storage.
- Normal source exits: missing TIM/include/primitive file; final valid flag write
  at MAX_FLAGS; bounded nobs capacity exit after a valid row. Export partial state.
- Host/ABI observations: all full writer paths through existing ld_* variadic
  cleanup defects; JBO raw_record type-punning/alignment, swap8 overloads; primitive
  union representations and EOF conversions; exact qsort permutations for ties.
- Source-inspected/unrun: empty auto-detected file (firstWord uninitialized), empty
  removeCR2, short SAT buffer tails, I-prefixed incomplete records with naturally unwritten telescope strings (the
  separately seeded missing-site case is defined retained caller state), legacy short/blank first records, first -dme,
  invalid numeric flag scans, malformed TIME (dtime unwritten), PHASE without
  previous observation, unchecked array/string/recursive include overflow, include
  realpath failure, null JBO/Fortran streams, incomplete first JBO record, invalid
  site/date conversions, partial record-int fread, signed count overflow, full NaN
  sort and unchecked allocation failure. No such experiment is run for a number.
- Inactive: commented legacy JBO loop, #if 0 dependency bodies, alternate PATH_MAX
  allocation branch on this host. Deferred: general preprocessing, specialized PAR,
  all Rust implementation and safe-boundary decisions, other ABI/backend platforms.

Deletion streams are not closed by source; finite fixtures preserve this defect.
The threshold is extended fabs against the binary64 literal 0.001 converted to
extended precision. Boundary fixtures use its exact decimal expansion.
EOF may repeat partial writes in the unused slot without incrementing nobs.
No new tolerance or scientific correction is introduced.

Colored diagnostic dependency boundary: TKlog.C's non-TTY strip loop forms
`buffer-1` on an initial ESC before restoring the pointer. Although no such
pointer is dereferenced here, affected MODE-warning cases are host observations.
This was identified while reviewing dependency evidence after the initial draft
replay and corrected before final evidence freeze. Direct negative-noWarnings
and suppression=2 cases avoid the colored path. M's source finding was checked
against the raw TKlog.C body; During final verification, central accepted M at a01d428/e754581 by decision
0030. Its accepted formatter/diagnostic limits now provide corroborating
read-only dependency evidence; N's native observations are independently run.
