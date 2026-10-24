# Assignment X frozen native observation contract

Release: `64cab7fc47f72d054a5d3af61441f20f069d9962`, decision 0057. Source:
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`. This directory alone is writable
assignment scope. W owns Rust and terrestrial_resources_port; central owns
decisions and acceptance. Neither is preservation membership. Existing accepted
validation evidence and both independent repositories are immutable.

The pre-experiment `inventory.py` freezes all nine routine bodies, every lexical
decision and preprocessor site, all raw repository caller/declaration occurrences,
the complete S type map (349 pulsar and 95 observation fields), and the recursive
SOFA closure. The generated map is local evidence, not a committed source copy.
The supplied raw hashes are checked. Rust context is read at release 64cab7f.

## Call and state map

| Routine | Reads, order and writes | Retained state and safety |
| --- | --- | --- |
| vectorPulsar | Per pulsar RAJ/DECJ narrow to double before trig; position and its equatorial copy; any nonzero eclCoord converts only the copy; binary128 PMRA/PMDEC then second derivative expressions narrow on assignment | Full four 3-vectors; parameter backing, other fields retained. No algebraic cancellation of cos(dec). Signed nonpositive counts are no-ops with valid backing. Finite trig inputs, bounded backing; nonfinite parameter coefficients may be tested separately from nonfinite trig arguments. |
| lmst | MJD truncation to int, subtraction of 51545, double polynomial; public binary128 fortran_mod then narrow to double; negative remainder adds one; writes tsid then derivative; returns zero | Separate primary TU, no inclusion of tempo2Util.C. Values keep both integer conversion and subtraction in int range. Two distinct output cells. |
| get_precessionMatrix | Double epoch/precession angles, supplied delp/dele, fixed OBLQ; constructs prc and nut; writes prn[j][i] in nested i,j order; returns zero | All nine initialized cells, no hidden state. Nonaliasing input values. |
| ang | floor(f/1e4) to int, packed minutes to int, seconds remainder; selectors >2 multiply 15, 1/3 multiply 2pi | Four documented and other integer selectors. Both float-to-int conversions bounded. |
| remove_white | strlen loop, tests leading/repeated spaces/tabs and newline; otherwise advances time flag | Entire writable buffer seeded and exported. Only empty, single internal separators, trailing single separator, CR and nonspace paths run. Every shifting strcpy is conservatively native-invalid due to overlap and remains unrun. |
| ut1red | First call reads two headers then 57-character row prefix into mjdVal[count] and entry[count2..count2+5]; increments rows and nonzero sample count; zero terminates; count>999 exits; sets limits then read=1 | Static read/mjd1/mjd2/mjdVal[1000]/entry[10000]/count/count2 are zero initialized. Export all cells, including scanf writes outside logical count. itsav is automatic -1 every call. Interior accesses it..it+3 must be in initialized backing; row-limit attempt stops after row 1000 before another write. Missing file is a defined exit. Empty/truncated tables that reach count-1 or invalid brackets are unrun. |
| tai2ut1 | Exactly method 2: nonzero clockCorr calls ut1red(SAT) then real getCorrection to TAI; zero clockCorr writes zero. Other methods retain UT1 | Multiple pulsars/observations and repeated calls; override and warning history; six nutations and coordinate cells preserved. Defined exits expose only initialized prefix. |
| get_obsCoord_IAU2000B | TT/UT1 integer JD split; real EOP interpolation/cache; eight direct SOFA entries; zero s-prime; Earth angular velocity includes dut1dot; transpose rotation; position/velocity scaled by c, zenith unscaled | All position/velocity/zenith outputs and separate inputs exported. TT/UT1 integer bounds; EOP first/interior/below-first safe, final row excluded. No buffer alias is asserted safe across the C++/Fortran chain. |
| get_obsCoord | Pulsar then observation order; delay bypass; exact STL/STL_FBAT polynomials, flags, offsets; STL_BAT zero prefix/velocity and pulsar-wide troposphere disable; other IDs look up shared observatory; exact IMAG updates xyz only; method 1 uses IAU, all other methods legacy; ecliptic conversion only value 1 | Coordinates[3..5], nutations[0..5], satellite zenith, unset satellite axes and flag tails retained. Legacy uses supplied UT1/nutations; clears zenith; calculates velocity after position ecliptic rotation. IAU clockCorr 0/2 bypass UTC correction, independently sums stored TT corrections. |

global.C orders clock/TAI-to-UT1 before barycentric vector and observer formation;
nutations are supplied by the ephemeris path. This harness explicitly seeds them
and does not claim a complete BAT/ephemeris run or initialiseOne defaults.

Satellite polynomial base flags must equal 1. Derivative masks through cubic are
independent; missing epoch defaults to zero except X velocity, which exits after
clearing velocity but before assigning position. Flags follow polynomial writes,
duplicates overwrite in order, failed scans retain then STL divides that retained
value by c. Offsets follow flags; count one does no interpolation; times strictly
increase, intervals are left inclusive/right exclusive, modes 0/1 interpolate,
2 holds and other modes add zero. Each axis has independent tables/mode/count.

## Dependencies and observation boundary

All primary TUs compile separately and unchanged in the ordinary build. Real raw
observatory.C, eop.C, clkcorr.C, tabulatedfunction.C, TKlog.C, TKlongdouble.C,
tempo2Util.C and verbatim global data/CVS slices supply dependencies. SOFA is
compiled from the recursive pinned Fortran closure with its original layout and
symbol spelling. No A2 archive/initializer code, fake dependency or Rust object
runs; only the exact accepted minimal config is reused.

The observer build may add pointer-registration calls for UT1 function statics
only. EOP cache histories are observed through real repeated calls and resource changes. They do not reset/read/mutate source state, seek files or
print before an action. A caller-side snapshot reads live initialized storage
after each action and at defined exit. It is separately identified and audited
against the unchanged ordinary executable, projecting only private observer
state and the exit snapshot. No numerical/diagnostic projection is permitted.
Observatory/clock namespace state is visible through unchanged dependency
inclusions in the caller, never before the primary TU overload declaration.

Caller structs use explicit aggregate zero initialization plus distinguishable
relevant fields and writable tails, with real parameter backing; no source
initializer is called. Generated typed field comparisons from S/R check all
non-target initialized fields, excluding padding. Pointer slots are compared
locally and represented by logical owner tags, never serialized addresses.
Finite binary64 bits and binary128 bytes, signed zeros and nonfinite classes are
distinct. Streams remain raw alongside exact declared-root projection. The
harness uses unbuffered stdout/stderr and begin/end markers; this is an explicit
observation profile, not a claim about the full CLI's buffering.

## Verification and disposition

Freeze source maps before experiments, then freeze authored cases/harness before
recording the ordinary oracle. Normal verification rebuilds and runs every case
twice in fresh processes, checks actual input/source/tool/runtime/executable
identities, recursively compares ordered membership/types/state/streams/status,
and never overwrites the oracle. Filenames are unique under casefold and bytes
are checked after all resources are written. Controls include bool/int/float
type substitutions at nested leaves and real launch identity failures.

Compiled mutations must change a native result, retained state or defined exit
prefix; changed hashes alone do not count. Separate gcov coverage measures every
primary routine and classifies unhit outcomes. Profile differences retain exact
field paths and values; no tolerance is introduced. Simple mathematical checks
are supporting diagnostics only. Original defects remain original reference;
0053 permits separately named corrections, with source bug/correction/regression
identification. No corrected experiment or production Rust edit is implicit here.

Unrun boundaries include overlapping remove_white shifts, invalid integer casts,
nonterminated/oversized strings, invalid counts/backing, empty UT1 interpolation,
EOP final-row slope, stale resource pointers, unproved aliasing, allocation/read
fault injection, other platforms/locales and concurrency. TKlog host diagnostic
limits from M/U carry forward. All generated outputs stay local under 0039.
