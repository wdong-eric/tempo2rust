# Assignment H source correspondence

Canonical source: `tempo2/{jpleph.c,jpleph.h,jpl_int.h}` at
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`. The build includes exact Git bytes;
it never uses `rust/native/jpleph.c`. Source hashes, host, compiler and flags
are in `outputs/checks.json`. No source patch initializes or repairs anything.

| H artifact | Source responsibility | Boundary |
| --- | --- | --- |
| `probe.c` | All 12 implemented functions in `jpleph.c`; layouts in `jpl_int.h`; API in `jpleph.h` | Includes unchanged source to access static helpers. Macros interpose libc seek/read/calloc for counting and explicitly requested fault injection. Ordinary libc calls remain ordinary calls. |
| `run.py` | Binary encoding documented in `jpl_int.h`; initializer and query sequences in `jpleph.c` | Fixture generation, process isolation, serialization checks and mutation comparisons; no production counterpart. |
| `analyze.py` | `interp`, `jpl_state` cache flow, Earth/Moon combination in `jpl_pleph` | Independent rational polynomial diagnostic and observed-sequence assertions; no replacement reference engine. |
| `test_isolated.py`, `diagnostic_fixtures.json` | No standalone TEMPO2 counterpart; validates H's sanitizer evidence comparator | Original unsymbolized and central symbolized failure summaries must give the same contract; failure kind/stage and available source locations remain gated. |
| `inputs.json`, `cases.json` | Declared encoded fields and ordered caller actions | Reproducible manifests; generated binary inputs live under ignored `build/inputs/`. |
| `outputs/reference.json` | Values, classifications, statuses, raw cache bytes and caller mutations | Host observations explicitly distinguished from defined initialized helper calculations. |
| `coverage.json` | Every implemented function, declaration-only function and macro | Machine-checked named cases and explicit gaps; not a branch-coverage percentage. |
| `outputs/isolated.json`, `outputs/isolated_diagnostics.json` | Invalid/terminating canonical paths | First file contains the exact failure contract; second retains variable trace presentation as diagnostic evidence only. No post-failure numerical values. |

The table in `coverage.json` gives exact source spans for every implementation.
`make_sub_ephem` has a declaration but no implementation in the pinned checkout.
The mathematical Sun state and the raw `jpl_get_pvsun` macro are distinct contracts.

## File and packed memory layouts

File offsets below are bytes, independent of host memory layout. Binary64 and
32-bit fields use the declared file byte order; titles and six-byte names do not.

| File bytes | Field |
| --- | --- |
| 0–251 | Three 84-byte title lines. Only the first is read by the initializer; `atoi(title+26)` selects the version. The 84-byte buffer is not NUL-terminated. |
| 252–2651 | First 400 six-byte names; no per-name terminator. |
| 2652, 2660, 2668 | Start JD, end JD, step days (binary64). |
| 2676 | Constant count (uint32). Raw host value `>65536` triggers byte swapping. |
| 2680, 2688 | AU in km; Earth/Moon mass ratio (binary64). |
| 2696–2839 | IPT rows 0–11 (36 uint32 values). |
| 2840 | On-disk version integer; discarded in favor of title parsing. Synthetic title 430/header 999 demonstrates this. |
| 2844–2855 | IPT row 12 (libration). Initializer rearranges words after reading the packed 204-byte header. |
| 2856 onward | Extra constant names when count >400; then rows 13–14 at `2856+max(ncon-400,0)*6` when title version >=430 and count !=400. |
| Record 1 | Constant values at `recsize+8*index`. |
| Record 2 onward | Two epoch doubles followed by coefficients. Record indices passed through the cache are zero-based; reads skip the two header records. |

The source computes `kernel_size=4+sum(2*ncf*na*dimension(ipt_index))` in
32-bit storage, `recsize=4*kernel_size`, `ncoeff=kernel_size/2`. It does not
derive size from the largest offset or validate arbitrary layouts. The fixtures
use contiguous one-based IPT coefficient offsets starting at 3. Each component
stores `ncf` coefficients, inside each subinterval. Synthetic polynomials are
deliberately discontinuous across records/subintervals so selection errors show.
They are not astronomical ephemerides.

On the executed LP64 host, `jpl_eph_data` is packed to alignment 1, size 648:

| Memory offsets | Type / selection |
| --- | --- |
| 0, 8, 16 | Start/end/step binary64; aligned generic `jpl_get_double` selections. |
| 24 | `ncon`, uint32. Generic getter loads signed int32 then promotes to `long`. |
| 28, 36 | AU, EMRAT binary64. Packed member access is distinct from generic getter's misaligned typed-pointer dereference (U2). |
| 44–223 | IPT[15][3], uint32. All 45 integer selections exercised. |
| 224, 228, 232, 236, 240, 244 | Version, kernel size, record size, coefficient count, swap flag, current cache location (uint32). Public integer selectors stop at 240. |
| 248–319 | `pvsun[9]`; raw macro returns base+248. Nine mathematical components are position, velocity, acceleration. |
| 320, 328 | `pvsun_t` binary64; cache pointer. |
| 336–639 | `interpolation_info`: 18 position polynomials, 18 derivative polynomials, `twot`, two unsigned availability counts. |
| 640–647 | Owned `FILE *`. Coefficient allocation begins at byte 648, aligned on this host. |

No arbitrary-offset getter result or misaligned getter numeric value is claimed.
`jpl_get_pvsun` carries neither length nor initialized-state information.

## Index, output and unit conventions

| Interface | Selection / destination | Units |
| --- | --- | --- |
| `jpl_pleph` 1–9 | Mercury through Pluto, with 3 = Earth | AU, AU/day |
| 10 / 11 / 12 / 13 | Moon / Sun / SSB / EMB | AU, AU/day |
| 14 / 15 / 16 / 17 | Nutation / libration / lunar mantle angular velocity / TT−TDB | Native coefficient units, then per day for derivatives; no AU scaling. Nutation/libration convention is rad, rad/day. Synthetic mantle/TT−TDB fields are unit-tagged test coefficients, not calibrated physical data. |
| `jpl_state list[0..9]` | Mercury, Venus, EMB, Mars…Pluto, geocentric Moon → pv[0..9][6] | AU, AU/day after scaling by 1/AU |
| `list[10..13]` | IPT[11..14] optional quantities, all → shared `nut` destination | 2/3/3/1 components respectively, multiplied by requested quantity count |
| Sun | IPT[10] always demanded as three derivatives when epoch changes → pvsun[9] | AU, AU/day, AU/day² |

`list` is 14 entries despite stale source comments. In direct state, bary=0
subtracts Sun from requested pv rows 0–8; Moon remains geocentric. The loop
visits interval counts 1,2,4,8, then increasing list index; simultaneous optional
requests overwrite the same destination in that order. Libration/mantle velocity
writes six values despite the function's `nut[4]` spelling and outdated pv[10]
documentation. H's ordinary harness supplies six initialized slots and 16 pv
rows. A separate four-slot ASan case demonstrates the overflow.

`jpl_pleph` first clears six outputs, then returns success for target==centre
before index/range validation. Optional targets ignore centre validity. Any
nonzero velocity flag requests two derivatives. Earth/Moon combination and
Earth↔Moon special handling retain exact source order. Position-only output
usually leaves the last three cleared slots; the EMB copy also reads unwritten
local velocity slots, which is an explicit undefined-source limitation.

The public epoch is one binary64 Julian ephemeris date in days, not the stale
two-element epoch described in comments. No time-scale conversion is performed;
these probes pass the ephemeris argument directly and do not equate it to UTC.

## State and failure interpretation

Canonical initialization copies an incompletely written stack structure over
the calloc block. H's `control` command explicitly writes only previously
unwritten Sun/cache-polynomial bookkeeping: Sun sentinels, NaN `pvsun_t`, zero
unused polynomial slots/`twot`, and availability counts 2. It preserves the
source-written seeds and record cache location. This is experimental input,
not evidence that canonical initialization supplies those values. No raw
unwritten structure bytes are exported.

These controls do **not** remove the line-625 pointer UB. Full `jpl_state` and
`jpl_pleph` numeric exports are reproducible host observations, not portable C
goldens. Isolated sanitizer cases export no post-failure numerical result.
The `interp` helper's separately initialized arrays provide the defined kernel
diagnostic. Undefined-output `pleph` failures have status and cache metadata
only, never a fabricated six-vector. `NaN`, `+Infinity`, `-Infinity` are explicit
classes; finite values include 17-digit decimals and exact binary64 bits.

Source statuses: state range −1, read −2, seek −6; pleph additionally absent
quantity −3 and invalid index −5. Ordinary pleph continues after state error.
Initializer error −7/−8 can accompany a non-null handle; close it. The global
init error resets on each init and is not a per-handle error register.
