# Observer coordinates and astrometry reference: Assignment X

Central decision, 24 October 2026. **Release Assignment X for bounded native evidence covering the nine routines below.** X owns only new `validation/observer_coordinates/`. W remains the sole Rust owner under 0056. Central retains implementation contracts, numerical design and milestone acceptance.

The existing Rust IAU2000B/PKS path does not establish the complete observer driver, satellite paths, legacy TEMPO rotation, UT1 reader or pulsar-vector expressions. This reference supplies those missing contracts while W completes terrestrial resources.

## Pinned source and build boundary

Use raw Git blobs from TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, rather than expanded checkout identifiers:

| Source | SHA-256 | Selected routines |
| --- | --- | --- |
| get_obsCoord.C | `40184b982564c13a4c02865d04045139c078edbd08a9ec40fc25c2265c7bf1d3` | get_obsCoord_IAU2000B, get_obsCoord, remove_white, lmst, get_precessionMatrix, ang |
| vectorPulsar.C | `46091a30f1bab414599acf0db631e57db6ad4d77396d3618a4ddcdd81c7df7df` | vectorPulsar |
| tai2ut1.C | `95b4e823ec95984bdd45de6526238f593ab6a1678aa5312880f2ce50bee4782b` | tai2ut1, ut1red |

Freeze complete routine/branch membership, caller fields, read/write order, static state and direct/transitive dependencies before experiments. Inspect real callers in global.C and the current Rust integration, but use fixed accepted revisions for Rust context while W edits it. Header fields include six observatory_earth cells and six nutation cells, even where these routines use only a prefix.

Compile unchanged primary translation units with the real pinned clock, observatory, EOP, numeric and diagnostic dependencies. Reuse accepted J/M/S/U preparation and observation patterns, without changing their evidence. Seed all required caller fields explicitly; do not imply that synthetic zero/seed state came from initialiseOne. No fake clocks, resource lookup, EOP values or coordinate calculations may replace called source operations.

Compile the actual pinned SOFA Fortran dependency closure, recording its sources, symbols, compiler/runtime identities and flags. The eight directly called entries are iau_cp_, iau_pom00_, iau_rxp_, iau_sxp_, iau_pxp_, iau_c2t00b_, iau_trxpv_ and iau_trxp_. iau_c2t00a_ is declared but not called here. Account for transitive functions without claiming the entire SOFA library is validated. Do not substitute ERFA or W's Rust/native build artifacts. The pinned sofa/Makefile.am hash is `2d0f7ba86115bcb0bd53ba5b6a8ad3d2dca2e716be94097c1937efb353e259d4`.

Preserve original translation-unit visibility: lmst sees only the public binary128 fortran_mod declaration in tempo2.h, despite its binary64 local variables. Including tempo2Util.C before get_obsCoord.C would expose another overload and could change the calculation. Verify actual overload resolution and compiled identity. Record native promotion points, including binary128 parameter arithmetic in vectorPulsar and the satellite polynomials.

Use the measured binary128/C-locale ordinary profile, with actual source/header/configuration/compiler/link/runtime/input/executable identities and no fast-math/reassociation. The existing A2 config may be reused by exact hash; no A2 initializer correction or timing instrumentation may execute. Verbatim global-data/CVS slices are allowed when bounded and hashed.

## Caller state and observation

Fresh processes isolate independent cases; ordered actions establish retained state. Export complete initialized inputs, outputs and retained cells relevant to each action, exact strings/known bytes, diagnostic history, streams and return/exit class. Use the accepted pulsar/observation maps to detect unintended changes in initialized non-target fields. Keep exact nested types, case/action order and logical resource/owner identity; never serialize raw pointer addresses or padding.

Private static arrays need an explicit observation boundary. If read-only snapshot instrumentation is required, preserve an unchanged ordinary executable, record the patch and separate identity, and compare instrumented and ordinary executions before using snapshots as evidence. Do not add reset hooks, repair operations or change cache lifetimes to ease tests. No observer may change file position, state or diagnostic output before the tested action.

Lossless binary64/binary128 representations, signed zero and established nonfinite classes remain distinct. Prove unusual fixtures are within native storage, scan and conversion bounds. Undefined accesses, overlapping strcpy, invalid float-to-int conversions and uninitialized reads stay unrun, with source-inspected prefixes distinguished from executed results. Host-qualified diagnostics do not establish portable behavior.

## Required experiments

### Pulsar vectors and direct mathematical helpers

For vectorPulsar, cover zero/multiple pulsars, signed loop-count no-ops with valid backing, equatorial/ecliptic and other nonzero eclCoord values, cardinal/general directions, proper motion and second derivatives, and repeated calls with distinct retained fields. Preserve the source's mas/year-to-century factors, binary64 trigonometric arguments, binary128 parameter expressions, and position-only equatorial copy conversion. Do not algebraically cancel the cos(dec) divisions. Extreme/nonfinite cases require path-specific validity checks.

For lmst, cover fractional and negative dates within integer bounds, longitude wrapping, zero/sign boundaries, both output cells and return value. Preserve truncation to int, native remainder overload and write order. For get_precessionMatrix, cover epoch/time direction, distinct nutation terms, matrix orientation and all nine cells. For ang, distinguish all four documented selector behaviors and other defined integer selectors, packed components and negative inputs within conversion bounds. Independent simple identities are diagnostics, not replacement goldens.

For remove_white, cover defined no-shift paths, empty input and retained writable tails. Its shifting branches call strcpy on overlapping ranges: classify these as native-invalid, do not execute them to obtain an ordinary golden. A later safe Rust correction is permitted under 0053 with its own explicit expectation.

### UT1 file and driver

Generate bounded ut1.dat fixtures from the actual reader, independently checking bytes. Cover skipped headers, the character-57 cutoff, partial successful rows, zero termination, row/sample counts, the five-day interpolation, both strict range boundaries and endpoint extrapolation. Include a bounded pinned real-resource check. Freeze static read/mjd1/mjd2/mjdVal/entry/count/count2 state, including written but logically unused cells. itsav is local and starts at -1 for every call; it is not an inter-call cache.

Exercise first successful load, repeated queries and root/file changes after caching. Record the actual signed UT1-TAI result and CLK10 warning history. Cover the defined missing-file and row-limit exits with their real prefixes. Empty/truncated inputs, integer overflow and invalid interpolation/table indexing are classified before execution; a process that happens to return is not proof of validity.

For tai2ut1, distinguish exactly T2C_TEMPO from other method values, clockCorr zero/nonzero, real correction-to-TAI lookup, overrides, warning controls, multiple pulsars and retained fields. The source uses SAT as the ut1red argument. Method bypass retains correctionUT1; clockCorr zero in the TEMPO method writes zero. A fresh valid process after an exit is not in-process recovery.

### IAU2000B helper and full observer driver

Exercise the direct IAU helper with distinct initialized position/zenith vectors, TT/UTC fractions near date-split boundaries, nonzero polar motion and UT1 rate from real EOP fixtures. Record all position, velocity and zenith outputs. Preserve integer Julian-day splitting, the exact Earth-rotation factor, zero s-prime, transpose/matrix layout and light-second/dimensionless scaling. Cover EOP cache and clock/resource histories through actual dependencies. Reuse U's invalid-boundary classifications; do not invent a final-row EOP slope. Aliased buffer cases need a validity argument for the complete C++/Fortran call chain.

For get_obsCoord, cover zero/multiple observations and pulsars, delayCorr bypass and its first-observation warning, method/clock/ecliptic switch distinctions and retained array tails. The IAU branch uses clockCorr neither zero nor two for UTC correction, and real getCorrectionTT independently. The legacy branch consumes supplied nutations and correctionUT1, clears zenith, preserves original position/velocity expressions and ecliptic conversion order. get_obsCoord falls through to legacy for methods other than IAU2000B, whereas tai2ut1 runs only for T2C_TEMPO.

Cover exact STL/STL_FBAT/STL_BAT spelling and case variants; independent TELX/Y/Z polynomial masks through cubic order; optional epoch and the source's x-velocity-only missing-epoch exit; flag order, duplicates and defined scan failure retention; STL flag-only metre conversion versus STL_FBAT units. Test per-axis offset tables with counts zero/one/multiple, interval boundaries, mode 0/1/2/other, and no matching interval, after proving denominator/index validity. Keep parameter values, flags and offsets in their actual source order.

STL_BAT zeros the position prefix and velocity, and changes the pulsar-wide correctTroposphere flag; test subsequent observations/pulsars. Other satellite branches retain zenith and unassigned coordinate components. IMAG changes the shared observatory xyz through flags without recomputing its stored geodetic fields; observe later calls and aliases through the actual resource owner. Preserve exact case comparisons and partial effects before defined failures. These observations are evidence of original behavior, not automatic endorsements of its physical meaning.

## Verification, delivery and policy

Freeze ordinary case membership and expectations before verification. Give cases and resource files unique names under the host filesystem's case rules, and verify their bytes after all fixtures are written; differently cased satellite IDs must not overwrite one another's fixtures. Fresh replay must check source/toolchain/runner/input/executable identities, exact ordered events/state/types/streams/statuses, and actual execution. Retain raw diagnostics; path projection cannot hide truncation or dropped messages. Clear stale success receipts and never overwrite the ordinary oracle during verification.

Include meaningful positive and rejecting controls for membership, nested scalar types, retained state, output values, diagnostics/status and identity. Compile bounded behavior mutations with actual witnesses, such as the lmst overload, UT1 cache reload/sign/boundaries, satellite unit/epoch handling, IMAG retention, ecliptic scope or matrix transpose. Hash differences alone are insufficient. Measure all nine primary routines and branch dispositions in a separate coverage profile. Compare its output with ordinary execution and report numerical/compiler differences, native-invalid branches, inactive paths and still-defined unverified cases explicitly.

Deliver `validation/observer_coordinates/REPORT.md`, complete source/caller/state/branch maps, reproducible commands, actual identities, independent checks, replay/controls/mutations/coverage and remaining limits. Commit only sources, generators, authored inputs/contracts and concise reports with explicit-path Conventional Commits. Under 0039, generated JSON/JSONL/compressed evidence, generated binary fixtures and other large artifacts remain local and ignored; do not force-add them.

0053 governs small demonstrated bugs: retain original evidence, explain the exact pinned source defect and correction in any separate corrected experiment, and provide a focused regression and explicit changed expectation. X supplies original native evidence and may record a bounded separately identified corrected experiment; it does not alter the canonical source or accepted oracle. Later Rust fixes need adjacent source-mapped comments and do not require renewed user permission for routine corrections. Scientific conventions, precision, algorithms and tolerances remain central decisions.

Protect both independent repositories and accepted evidence. W's evolving `rust/` and `validation/terrestrial_resources_port/`, X's own directory and central documents are outside X's immutable preservation membership. Do not edit W's files or accepted reference runners, or freeze W's dirty working state as an oracle. Preserve source-defined process boundaries.

This release does not accept a Rust geometry implementation, full SOFA, new precision/tolerances, JPL/CALCEPH integration, secular motion, full BAT/propagation, concurrency/platform behavior or M2. Return concrete contract questions with evidence while completing independent work. Central acceptance follows review of the delivered reference.

## Dispatch

Created Assignment X in the local rustify-tempo2 project from release commit `64cab7f`, chat `01a16c44-a1ca-7e43-a83a-1cdce98d44e8`. An immediate status snapshot confirms its reference turn is active. W received the separate ownership boundary and remains the sole Rust owner. Both chats inherit 0053 and 0039; central retains acceptance. This records dispatch, not completed or accepted evidence.
