# Assignment X measured coverage and limits

The separate GCC/gfortran 16.2 `-O2 --coverage -fno-inline` profile entered all
nine primary routines. All 133 instrumented executions agree exactly with the
ordinary profile in recursive types, initialized state, finite bits/nonfinite
classes, signed zero, streams and statuses. There are **zero profile numerical
or diagnostic differences**, and no tolerance/projection was added for coverage.
Counts are compiler line/branch outcomes, not an assertion of every possible path.

| Routine | Calls | Lines hit / measured | Branch outcomes hit / measured |
| --- | ---: | ---: | ---: |
| get_obsCoord_IAU2000B | 93 | 23 / 23 | 2 / 2 |
| get_obsCoord | 126 | 150 / 150 | 131 / 132 |
| remove_white | 7 | 7 / 13 | 20 / 24 |
| lmst | 159 | 15 / 15 | 2 / 2 |
| get_precessionMatrix | 126 | 33 / 33 | 4 / 4 |
| ang | 56 | 8 / 8 | 4 / 4 |
| vectorPulsar | 43 | 29 / 29 | 6 / 6 |
| tai2ut1 | 16 | 12 / 12 | 9 / 10 |
| ut1red | 58 | 50 / 50 | 29 / 30 |
| Total | — | 327 / 333 | 207 / 214 |

The raw gcov JSON files also show entry into all 36 compiled SOFA closure routines.
That establishes entry, not exhaustive SOFA branch validation. Full raw counts
remain in `build/coverage/sofa_*.gcov.json.gz`; the local derived list is
`outputs/sofa_coverage.json`.

Every unhit measured outcome is accounted for below. Branch indices are GCC's
zero-based per-line indices in `outputs/coverage.json`.

| Source location | Unhit membership | Disposition |
| --- | --- | --- |
| get_obsCoord.C:140 | branch 0 | Defined, unverified: driver-level displayCVSversion==1 report. The IAU helper's own reporting branch is exercised. |
| get_obsCoord.C:462 | branch 4 | Native-invalid shifting path: leading whitespace invokes overlapping strcpy. |
| get_obsCoord.C:464–465 | both executable lines | Native-invalid shift and decrement; intentionally unrun. |
| get_obsCoord.C:467 | branches 7 and 8 | Repeated-space/tab shifting paths, conservatively excluded with their overlapping strcpy. The compiler's individual short-circuit outcomes are not portable semantic branch identifiers. |
| get_obsCoord.C:469–470 | both executable lines | Native-invalid shift and decrement; intentionally unrun. |
| get_obsCoord.C:472 | branch 0 | Newline-removal shift excluded. No newline fixture is executed. |
| get_obsCoord.C:474–475 | both executable lines | Native-invalid shift and decrement; intentionally unrun. |
| tai2ut1.C:56 | branch 0 | Defined, unverified: tai2ut1-level displayCVSversion==1 report. |
| tai2ut1.C:193 | branch 1 | Inactive for defined interior inputs: itsav is automatic -1 each call; t>0 makes it>=0. There is no inter-call interpolation-index cache. |

The machine report uses a conservative generic `still-defined-unverified` label
for the last UT1 outcome; the source invariant above refines that label to
inactive. Its measured zero count is unchanged. It is not counted as executed.
No branch is removed from the denominator.

Remaining unrun boundaries are the six authored groups in fixtures.UNRUN and
CONTRACT.md. Coverage cannot establish safe behavior for invalid integer casts,
empty or inconsistent tables, invalid backing/strings, final-sample EOP slopes,
aliasing without a C++/Fortran validity argument, concurrency or other platforms.
No fault injection or full timing propagation is claimed.

The public binary128 lmst overload is checked by the actual object symbol and
separate raw translation-unit construction. There is no numerical sensitivity
claim for swapping its overload at modulus 1, and such a swap is not counted among
the 15 successful behavior mutations. All counted mutations have actual changed
result/state witnesses, not merely altered executable hashes.
