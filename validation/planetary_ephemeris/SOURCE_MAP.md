# Assignment Z source and state map

The authoritative raw source map is `outputs/frozen/source_map.json`, created
before any native execution. The pin is `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
Its complete bodies, exact file hashes, active decisions and type declarations
are retained locally; this file explains their meaning. The frozen prose
contract's gasdev upper line 573 is a typo: the raw definition ends at **570**,
as correctly extracted in the frozen machine inventory.

| Routine / raw span | State and behavior | Executing harness |
| --- | --- | --- |
| readEphemeris, 56–494 | Signed outer/inner loops; per-pulsar real open/close; AU/IFTE conversion; complete JD; ordered planetary queries, all nine differences, mass/frame operations, independent offset tables, exact STL_BAT scans | `probe.cc` driver actions, separately compiled unchanged `readEphemeris.C` |
| random2, 498–548 | Process-static binary128 r[100]/result, int iff/ix1/ix2/ix3; native long seed; 97 shuffle cells; binary64 rm1/rm2 construction | random actions and real gasdev calls; separate pointer observer |
| gasdev, 550–570 | Process-static int iset, binary64 gset; two real random draws per attempt, rejection loop, cache | gaussian actions and mixed shared-seed histories |

`MAX_SHOTS` is unused. `HAVE_CONFIG_H` selects the accepted configuration.
The literal `#if 0` block at 96–152 and commented phase application near 364
are inactive; executable coverage does not credit them.

## Fields, extents and initialization

| Source object | Read / write contract |
| --- | --- |
| pulsar `nobs`, `JPL_EPHEMERIS[MAX_FILELEN]`, `units`, `eclCoord`, `obsn` | Read; signed counts, exact SI==1 and ecliptic==1 predicates, actual owned observation pointer |
| `param[param_dphaseplanet]`, `param[param_dmassplanet]` | Read flags[0..8]; load vectors if either flag==1; mass correction if mass flag!=0; binary128 mass values. Phase values unused. All nine values/flags and ownership exported. |
| observation `sat`, `correctionTT_Teph`, `nclock_correction`, `correctionsTT[30]` | Binary128 SAT/Teph; real getCorrectionTT sums binary64 correction fields. Full clock backing exported. Other clock fields retained. |
| `sun_ssb`, `sun_earth`, `jupiter_earth`, `saturn_earth`, `venus_earth`, `uranus_earth`, `neptune_earth`, `earth_ssb`, `earthMoonBary_ssb`, `earthMoonBary_earth` | Ten complete binary64 six-vectors. Source scales all components; ordinary ecliptic rotations alter 0..2, with Earth also rotating 3..5. |
| `planet_ssb[9][6]`, `planet_ssb_tmr[9][6]`, `planet_ssb_derv[9][6]` | Explicit initialized caller sentinels. Two selected-load loops followed by nine unconditional six-component subtractions. Every inactive cell retained/exported; no native-initializer claim. |
| `nutations[6]` | Real JPL optional output; all six cells exported, including cleared tail 4..5. Absent optional quantity returns -3 and zeros all six. |
| pulsar `nTelDX/Y/Z`, `telD*_t/v[500]` | Independent count/time/value axes; full backing exported, including unselected 496-cell tails. Other e/vel/vel_e arrays remain initialized zero and are recursively checked. |
| observation `telID[100]`, `nFlags`, `flagID/flagVal[40][MAX_FLAG_LEN]` | Complete initialized string storage, including retained suffixes and inactive slots, exported; exact STL_BAT and flag names, sequential sscanf retention semantics. |
| all other fields | All 349 pulsar and 95 observation top-level fields and nested FitInfo/FitOutput/parameter/storePrecision/clock_correction fields are initialized through C++ value initialization. Generated recursive field checks compare every non-target field, including arrays and pointer identities, without struct padding. Parameter values/flags are checked separately by actual allocated extent. The same checks run at observed defined exits. |

The fixture assigns five parameter objects per pulsar (phase, mass and ignored
x/y/z mode parameters), each with nine value/flag cells and `aSize=9`. Other
parameter pointers are initialized null, not allocated or dereferenced. All
observation owner slots are exported, including unused observations and the
second pulsar when npsr==1. Local `nams[2000][6]`/`vals[2000]` are supplied to
real init, bounded by generated/pinned resource counts and never serialized;
unused/uninitialized local backing is not evidence.

## Calls and dependencies

The exact complete repository occurrence list distinguishes actual calls from
function declarations and diagnostic strings. Active core calls are
`global.C:112,129`: vectorPulsar → readEphemeris → get_obsCoord → tt2tb →
readEphemeris, with CALCEPH selected on the other branch. Plugins contain
calls in bary:150, basic:138/141, delays:161/164 and fermi:1728/1731.
The public declaration is `tempo2.h:1017`. random2 has local forward declaration
at line 40 and only active native calls in gasdev:558–559; gasdev is declared
at line 41 and has no other active repository caller. Harness calls expose both
helpers independently. The driver does not call either helper, including noise
requests. Global wrappers, CALCEPH, vector/observer, propagation and reference
phase orchestration are mapped only; their execution is outside this stage.

Each observation executes Sun/SSB then Sun/Earth; selected planet/SSB at JD;
selected planet/SSB at JD+1e-6; unconditional planet differences; Jupiter,
Saturn, Venus, Uranus, Neptune/Earth; Earth/SSB; mass adjustment and final Earth
rotation; EMB/SSB; EMB/Earth; nutation (14/3); offsets; STL_BAT. All JPL calls
request velocity=1. The second date is a small finite difference, not a
normalized derivative or a one-day interval. `earthMoonBary_earth` is EMB minus
Earth despite the source's nearby Moon comment.

Real `clkcorr.C:810` getCorrectionTT, `tempo2Util.C:186` public binary128
fortran_mod, `tempo2Util.C:235` equ2ecl, TKlongdouble/TKlog and raw global
variables/CVS implementation are linked. Clock's table/observatory dependencies
are compiled in full to resolve the real object. No dependency calculation is
stubbed. Ordinary getCorrectionTT is pure over the supplied clock chain; no
clock database load or routing is claimed. Ordinary source overload visibility
is verified from readEphemeris.o's undefined symbols and the utility symbols.

## Observer boundary

`profile.py` records the exact separate observer patch. It inserts pointer
registration after random/static declarations, and redirects only the primary
JPL call sites to forwarding functions with the same argument/return types.
Forwarders call unchanged real JPL once; no outputs or cache fields are altered.
Handle numbers describe owned open/close history and do not expose addresses.
Each Gaussian attempt also records its initialized v1/v2/rsq values immediately
after the native arithmetic, and random-entry counts record actual calls. These
callbacks only copy observations; the complete ordinary projection is re-audited.
Statics are exported only after their pointers have been registered; `null`
means not observed yet (C static zero initialization is separately known).
Every initialized r[0..99] cell and result/ix/iff, plus gasdev iset/gset, is read
without writing or resetting it. Ordinary exported fields and raw diagnostics
must match for every case. Exit observations are additional, separately
identified evidence. Instrumentation never supplies native missing state.

H's generator and all H/I evidence remain read-only. Binary fixture files are
new Z outputs; real DE405 is read through a Z symlink with its actual original
path and hash recorded. Neither native numerical evidence nor exact replay
supplies astronomical validation, random quality evidence or a Rust tolerance.
