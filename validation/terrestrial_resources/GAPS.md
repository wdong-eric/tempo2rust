# Reviewed coverage dispositions

The separate GCC `-O2 --coverage -fno-inline` run reproduces all 130 ordinary
profile records exactly. It enters all 22 routines and measures 336/341 executable
lines and 217/252 compiler branch outcomes. These are reachability measurements,
not physical validation or exhaustive input-domain coverage. The complete raw
counts, source decisions, dependency entries and function counts are in
`outputs/coverage.json`. Its automatic fallback category is conservative;
the reviewed attribution below refines that fallback without changing counts.

| File | Entered routines | Hit/executable lines | Hit/compiler branch outcomes |
| --- | ---: | ---: | ---: |
| observatory.C | 8/8 | 137/138 | 99/114 |
| eop.C | 2/2 | 60/62 | 50/56 |
| tropo.C | 12/12 | 139/141 | 68/82 |

Every unhit **non-throw** compiler outcome is listed below. Indices are zero-based
within that source line's gcov branch list. Eighteen additional `throw: true`
outcomes are host-dependent compiler unwind/exception paths at file I/O, logging,
vector append and glob calls; allocation/I/O exceptions were not injected.

| File:line / index | Classification | Reason |
| --- | --- | --- |
| observatory.C:163 / 1 | still-defined-unverified | TK error-log storage-capacity guard; the missing-alias exit is exercised, prior error-count exhaustion is not. |
| observatory.C:233 / 1 | still-defined-unverified | quietFlag suppression with TEMPO2_ALIAS selected is not combined in this catalogue. |
| observatory.C:267 / 3 | inactive | The preceding search leaves ilist in range only when ialias is in range; the second condition's false outcome cannot follow that search without external mutation. |
| observatory.C:272 / 1 | still-defined-unverified | TK warning-log storage-capacity guard; once-per-list warnings are exercised, 30 prior stored warnings are not. |
| observatory.C:296 / 0 | inactive | Successful lookup_observatory_alias has already initialized the observatories. |
| eop.C:130 / 1 | still-defined-unverified | Debug suppression for the second modern-format marker; this marker is loaded with debug enabled. |
| eop.C:143 / 3 | host-dependent | Compiler-managed local-static initialization guard; contention/reentrancy is outside the serial caller profile. |
| eop.C:159 / 0 | native-invalid | At or beyond final sample eventually reads samp[size]. No upper-proxy golden exists. |
| eop.C:161 / 0,1 | still-defined-unverified | Optimized gcov attribution for this guard remains unresolved. Below-first and ordinary interior histories are measured, but no separate basic-block attribution is claimed for these two counters. |
| eop.C:179 / 1 | native-invalid | Exhausting the scan reaches the same invalid bracket; supported queries stop before size. |
| tropo.C:274 / 0 | host-dependent | GLOB_NOSPACE was not induced. |
| tropo.C:275 / 0 | host-dependent | Error-output continuation requires GLOB_NOSPACE. |
| tropo.C:277 / 0 | host-dependent | GLOB_ABORTED was not induced. |
| tropo.C:278 / 0 | host-dependent | Error-output continuation requires GLOB_ABORTED. |
| tropo.C:282 / 1 | native-invalid | On measured NOMATCH, nonzero warnings continue into null gl_pathv iteration. Only warnings==0 executes. |
| tropo.C:408 / 0 | still-defined-unverified | debugFlag-on printing at the first zero-zenith warning is not combined in this catalogue. Warning-dependent calculation order itself has a compiled witness. |

The reviewed 35 unhit outcomes therefore comprise **23 host-dependent, seven
still-defined-unverified, two inactive and three native-invalid** outcomes.
Five unhit executable lines are observatory.C:297 (inactive), eop.C:160
(native-invalid), eop.C:161 (compiler attribution unresolved), and tropo.C:275/278
(host failures). Source `#if 0` blocks are absent from executable-line counts:
observatory test conversions, convertGeocentric, the tropo mapping test, and the
table linear-search alternative remain inactive. HAVE_CONFIG_H, GLOB_ABORTED and
GLOB_NOMATCH dispositions are recorded in the actual macro/profile evidence.

Domain limits remain broader than these branch counters: no uninitialized
scanner reads, overflowed strings, invalid numeric-to-integer conversions,
EOP malformed private brackets, table empty/singleton in-range access, stale
observatory-pointer reads, arbitrary filesystem failures, other locale/platform
or concurrent resource mutation is executed. Host-qualified TKlog diagnostics
retain the existing J/M limitation. `outputs/unrun.json` and CONTRACT.md identify
these source boundaries; none is silently treated as a successful observation.
