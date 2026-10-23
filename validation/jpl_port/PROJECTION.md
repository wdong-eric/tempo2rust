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
gated separately. New boundary tests cover direct third derivatives, invalid
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
