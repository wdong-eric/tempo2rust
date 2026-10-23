# Assignment I comparison projection (specified before implementation)

Decision 0017, canonical fda6ad4a9753ad7dc5d291406f2f310a9161423f,
H delivery 1df4d37. `projection.json` enumerates every scenario and ordered event
(including startup), its complete top-level reference field set and comparison
fields. The comparator must first reject extra/missing cases, events, fields,
array components and numeric-object fields. No zip truncation is allowed.

Finite values compare exact binary64 bits, including signed zero. Decimal text
must round-trip to those bits but need not use C's spelling. Nonfinite classes
compare exactly; NaN payloads have no gate. All statuses, names, selectors,
header values, caller buffers (including untouched sentinels), raw cache bytes,
Sun/interpolation caches, inode relationships and fread/seek event counts compare
exactly. Units follow H SOURCE_MAP: AU/AU-day/AU-day-squared for planets/Sun and
native optional quantity units; no epoch conversion. Command order and its exact
indices/flags are checked against H cases.json, never inferred from outputs.

Startup C struct size/alignment/offsets are reference ABI facts, retained in the
projection as classifications, not emitted as a fake Rust layout. `io_calls[2]`
is the source allocation checkpoint/fault event, not Rust's allocator count.
The test-only observer counts that checkpoint once per initialized handle.
`io_calls[0:2]` and the whole last-fread tuple retain actual stream call semantics.

Unwritten natural-init fields have no reference value and are never exported by
that scenario. Rust initializes recurrence storage and represents Sun absence;
`control` explicitly supplies H's experimental sentinels, NaN epoch, and counts.
Failed pleph events retain status and cache but have no numerical `out` field.
Position-only EMB copies only defined requested components; its public cleared
suffix remains zero. All state/pleph values remain host observations (source
invalid pointer formation), even after exact agreement. Positive partial-element
bytes remain raw host observations, not portable C double values.

The ten H isolated cases have separate Rust dispositions: packed AU/EMRAT return
decoded fields; state/pleph one-past pointer cases select valid rows directly;
NaN and tiny-step record conversions fail at `record_conversion`; ncf=18 at
`chebyshev_count`; oversized coefficient offset at `coefficient_storage`;
four-slot nut at `output_storage`; overflowing/unterminated title at
`title_conversion`. No sanitizer trace or process exit is a numerical oracle.
Each isolated sequence is replayed; last safe Rust event and disposition are
gated separately. New boundary tests cover direct acceleration demands, invalid
selectors/counts, same-open-file provenance, fdopen ownership failure, independent
handles, drop/reopen, and recovery. No race scheduling is needed.

## Production responsibility map

| Canonical function / macro | Rust representation |
| --- | --- |
| jpl_init_ephemeris | jpleph::Ephemeris::init, decoded header, owned byte cache |
| jpl_init_error_code | sequential process-global AtomicI32, initially -9 |
| jpl_close_ephemeris | Ephemeris drop -> stdio::Stream drop, exactly once |
| jpl_get_double | Ephemeris::get_double, declared decoded field selection |
| jpl_get_long | Ephemeris::get_long, u32 bits interpreted as signed i32 |
| jpl_get_constant | Ephemeris::get_constant, caller name and raw value bytes |
| swap_32_bit_val | swap_32_bit_val over a four-byte array |
| swap_64_bit_val | swap_64_bit_val over complete eight-byte byte chunks |
| dimension | dimension, including source's i+1 state convention |
| interp | Interpolation::interp, recurrence/demand storage |
| jpl_state | Ephemeris::state, early cache commit, bounded actual destination |
| jpl_pleph | Ephemeris::pleph, requested-component arithmetic |
| jpl_get_pvsun | Ephemeris::pvsun bounded optional nine-component view |
| make_sub_ephem | excluded: declaration only at pin |

stdio owns only fdopen/fread/fseek/fclose. A Rust File is opened once, optionally
hashed and rewound by runtime provenance, then consumed. No alternate production
reader, descriptor clone, escaped FILE pointer, or Send/Sync implementation.
Private cfg(test) fault/counter state models H seek/read/allocation checkpoints.
read_ephemeris retains timing call order and extended precision scaling.


## Decision 0019 amendment: title_zero

Central decision 0019 requires a title_conversion boundary when whitespace, sign
or numeric scanning reaches byte 84. H is unchanged. The title_zero projection
retains all five original events, their event numbers, complete reference field
lists and canonical JSON hashes: startup compares defined fields; open requires
boundary, global status zero, one preceding seek/two reads, no allocation, and
last fread (204,1,1); control/state/close are explicitly unreachable. Cleanup
occurs through Rust ownership, not execution of the later source close command.
Only this named scenario has that classification; unexpected exclusions fail.
Counts distinguish reference accounting, executed events, exact events, boundary
and unreachable events. Supplementary I canonical probes preserve all six
C-locale whitespace bytes, signed versions and in-bounds no-digit zero. ASan
classifies all-whitespace/final-sign scans; it supplies no numerical oracle.

Supplemental optional-demand scenarios cover all four optional indices with
INT_MIN, -1, 0, 1, 2, 3, 4 and INT_MAX. Only the observation harness's optional
buffer is enlarged to 12 initialized slots; canonical algorithms remain exact
Git bytes. All fields/events compare with the same startup-ABI projection. These
remain controlled host observations subject to source pointer-formation UB.

Twelve additional Rust-only header controls cover both byte orders: normal and
unwritten-comparison cases for pre-430 and ncon400 headers, plus valid extended
headers with small and large constant counts. In either no-extension route,
first-comparison mismatch still clears both optional rows. If the computed end
of row 12 equals UINT32_MAX, reject before the second comparison consumes unread
row-13/14 words. Four boundary events require global status zero, one seek/two
reads, no allocation, and last fread (204,1,1). The unread native comparison is
never executed and supplies no numerical oracle. H's projection is unchanged.
