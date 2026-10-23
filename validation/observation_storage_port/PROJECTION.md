# R source and caller projection — frozen before production edits

Baseline O delivery 8d2be6f / production 8bad0be; current shared HEAD and
all pre-edit Rust module hashes are in baseline.json. Governing contract 0037.
fields.json records all 95 fields in raw tempo2.h order, exact source spelling,
source types, dimensions, Rust cell representation and frontend writes. The
nested checkout expands $Id$; raw Git bytes, not the filtered worktree header,
are the canonical hash 0313b6eaf9f9ae9d7e5cce340379ad4b7068394e5222a01f73949e3a2640de93.

## Native allocation, assignment and scope

initialise.C:68 allocates observation backing with malloc. Lines 87–94 are
commented out. Every scalar, array cell and byte begins logically unwritten.
tempo2Util.C:293 performs ordinary whole-observation assignment. It copies all
logical fields and full arrays, including inactive entries and bytes after NUL.
Rust Clone represents independent whole-owner assignment, not copyPSR itself,
its wrapper counts, pointer layout, padding, destruction or sorting.

M copypsr_0/1/2/3 supplies four initialized histories, all 95 field spans in
all three backing observations. m_projection.json freezes the observation-only
records (including unchanged source, retained destination and bytes); offsets,
pulsar fields and parameter fields are excluded from comparison. M's spans
include nested correction padding; the typed projection will exclude that
padding, while retaining every nested correction value and destination byte.
N inventory.json establishes the same 95-field inventory; its probe.cc and
SOURCE_MAP.md establish explicit caller initialization and history. No N output
is evidence of a Rust source TIM parser. Full source parsing is deferred.

## Frontend and consumers

read_timfile.rs retains strict FORMAT 1, site/@/PKS, signed-value checks and
existing Rug128/binary64 parsing. It writes only sat, freq, toaErr, fname and
telID, including string NULs. Provenance retains original name/site/token text,
line and index separately. The 501-byte fname is a storage constraint measured
in UTF-8 bytes; overflow returns a contextual boundary without truncation.
input.rs profile selection, form_residuals.rs site/SAT reads, form_bats.rs
site/SAT/frequency reads and t2fit.rs sigma read use canonical cells. main.rs
profile selection uses source telID. Historical Observation is a detached
snapshot; serializer derives exactly the existing keys and values. No BAT or
residual output is installed in source input storage.

## L clock caller projection and read/write order

Old ClockObservation stores sat, tel_id, clock_corr, count, 30 corrections and a
variable jump vector. Map these to sat, telID, clockCorr, nclock_correction,
correctionsTT, jump[40] and signed obsNjump. Remove old owned types/constants.

The bounded terrestrial adapter clones its canonical record, explicitly writes
clockCorr=1, nclock_correction=0, obsNjump=0 and zeroes every correction value
and all 32 destination bytes, preserving L's actual seed. These are caller
writes, never new-observation defaults. SAT and telID come from the clone.
The direct L harness seeds sat=0, empty telID, clockCorr=0, count=0, no members,
zero corrections; its obs operation writes requested SAT/site/controls/count
and all 30 values 900+i, fills destination bytes with '~', then snprintfs
sentinel_i into the first 31 bytes. slot uses snprintf capacity 32; members
sets signed count and assigned indices without discarding the fixed tail.

clkcorr.rs source_clock reads telID only without explicit nonempty from;
get_clock_corrections first writes count=0, resolves source clock, and returns
on equal names before reading SAT. fill_corrections reads count for capacity,
loads/checks chain, reads SAT for evaluation, then writes value, destination
through NUL, increments count and total. get_correction has the same early
identity return; get_correction_tt reads signed count and only positive-loop
correction values. Negative counts remain signed, with the source empty loops.

toa2utc reads clockCorr, conditionally calls get_clock_corrections; only inside
positive pulsar nJumps loops does it read obsNjump and active jump entries.
Keep the source jumpSAT[l] versus jumpVal[k] mismatch and extra shifted slot.
Increment count before reverse shift; each slot assigns correction then
strncpy of 31 destination bytes, preserving byte 31. strncpy reads no bytes
past the first NUL and does not require a NUL within its 31-byte limit. SATJMP
snprintf uses capacity 31. Preserve completed scalar/byte writes at a boundary.
The process ClockSession retains its existing terminal failure semantics.

## Evidence and protected membership

protected_hashes.json extends O's immutable manifest with accepted O artifacts,
central release records and other tracked historical evidence. P preprocessing
and Q par_specialized are expressly excluded. O run/native/verify routing is
excluded only for the authorized output/build-root changes; their baseline
committed contents remain the comparison for permitted edits. Both nested
repositories remain clean. No native unwritten read is a golden.

Supplemental caller will initialize every typed cell/byte with distinguishable
values, measure ABI types/extents, assign whole records and inspect full typed
state before/after. Signed integer extremes, negative zero, nonfinite classes
and exactly representable binary128-only finite values are included. Rust-only
partial initialization, string holes, operation-local failure prefixes, fixed
limits, independent ownership and canonical input changes cover safe boundaries.
No new unsafe block, numerical expression, backend or scientific gate is planned.
