# Assignment X: observer-coordinate native reference

Prepared 24 October 2026 from release `64cab7fc47f72d054a5d3af61441f20f069d9962`
under decision 0057. This report records bounded original native evidence. It
changes no production Rust, numerical convention, tolerance or milestone status.
Central review and acceptance remain separate.

## Scope and original-source identity

Only `validation/observer_coordinates/` is owned. The raw TEMPO2 pin is
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`. The ordinary build compiles unchanged
get_obsCoord.C, vectorPulsar.C and tai2ut1.C as separate translation units. The
three SHA-256 values are checked against 0057 before the source map is used.
[CONTRACT.md](CONTRACT.md) is the pre-experiment routine/caller/state/safety map;
local `outputs/frozen/source_map.json` contains every routine body, 100 lexical
decision sites, raw caller/declaration occurrences, preprocessor branches, the
complete accepted S/R field inventory and recursively identified SOFA closure.

Real pinned observatory, EOP, clock, table, numeric and diagnostic dependencies
run. The eight direct SOFA symbols resolve from **36 actual pinned Fortran
routines**: anp, anpm, bi00, bp00, bpn2xy, c2i00b, c2ibpn, c2ixy, c2ixys, c2t00b,
c2tceo, cp, cr, era00, ir, numat, nut00b, obl80, pn00, pn00b, pnm00b, pom00,
pr00, pxp, rx, rxp, rxpv, rxr, ry, rz, s00, sxp, tr, trxp, trxpv and zr.
`iau_c2t00a_` is declared but not called or linked. This is not whole-SOFA validation.

The accepted A2 minimal config is reused at SHA-256
`470c61b68a61504558e76ef772639e46b565a1e4500115d6e3fb6a71286dd69e`.
The archive is checked as context, never linked; no initializer correction runs.
Global-data and CVS reporting slices are copied verbatim from global.C:1–64 and
176–200. Parameter/observation state is explicit caller initialization, not an
inferred initialiseOne default. Rust context was read at the release revision.

The primary object references `_fortran_mod`, the public C-linkage binary128
function; the build asserts that the binary64 C++ overload symbol is absent.
`tempo2Util.C` is compiled separately. Neither its overload nor helper inclusion
can change visibility inside the unchanged primary translation unit.

## Reproduction and profile

Run sequentially from `/Users/Eric.Dong/Downloads/tempo2rust`:

```sh
python3 validation/observer_coordinates/run.py
python3 validation/observer_coordinates/independent.py
python3 validation/observer_coordinates/controls.py
python3 validation/observer_coordinates/observer_audit.py
python3 validation/observer_coordinates/coverage.py
python3 -m py_compile validation/observer_coordinates/*.py
git diff --check -- validation/observer_coordinates
```

On a checkout without local evidence, first run `inventory.py`, then
`run.py --record`. Freeze and recording refuse to overwrite their existing
baselines. Normal verification never rewrites the ordinary oracle. Do not run
these commands concurrently: their owned case directories are reused. The
snapshot must remain the original snapshot for this delivery's preservation
claim; freezing a later workspace is a different baseline.

Prerequisites: local pinned TEMPO2, accepted S inventory and A2 minimal config,
Python 3.11, Homebrew GCC/gfortran/gcov 16 and macOS command-line tools. No network,
Rust build, package install or independent repository mutation is needed.

The measured host is macOS **26.7, build 25G229, arm64**, C locale. GCC and gfortran
are **16.2.0**. Native double has 53 significand bits; selected longdouble is
16-byte binary128 with 113 bits. C++ flags are `-O2 -std=gnu++11 -DHAVE_CONFIG_H`;
Fortran flags are `-O2 -fno-second-underscore -fPIC`. No fast-math or reassociation
flag is added. Ordinary linkage uses libquadmath/libgfortran and the normal
C++/system runtimes. Coverage adds `--coverage -fno-inline` to both languages.

The profile records **375 hashed inputs**, full compiler argv, compiler/backends,
assembler/linker, Python, dependency headers, sources, config, generated typed
field checker, macro digest, SOFA symbols, actual executable, link map and loaded
library identities. Every launch verifies inputs and executable before and after.
macOS linker UUID/map byte equality across rebuilds is not assumed: each actual
binary is recorded and checked against its own profile. libSystem is qualified
by the measured OS/dyld ABI; its shared-cache image is not separately hashed.

| Runtime | SHA-256 |
| --- | --- |
| libgfortran.5.dylib | `09c11f298a73589b0fa79d93bad7fda25c29055999b5c331507e02f456bf1775` |
| libquadmath.0.dylib | `6f885df6ad8277edf614f0f5f7fbc6f8564a95b64679b88fb860382c9d1c0e9a` |
| libstdc++.6.dylib | `6b6012c574ec8d680aaadeaed32a488449b7b75fafcd7bc45348bafdc1d8400c` |
| libgcc_s.1.1.dylib | `5b741a6f933be1fea16db7361121d178bb88b81fc8c567846d1d5593bd0cc146` |

## Cases and observation boundary

The frozen catalogue has **133 cases and 3,462 ordinary ordered events**. Every
case has a unique numeric filename prefix; telescope spelling is data. Resource
filenames are checked under case folding and their complete bytes checked after
all writes, immediately before execution. The UT1 generator independently checks
57-column prefixes, scanner integer bounds, row/sample counts and interpolation
backing before native execution, including the complete pinned real ut1.dat.

| Area | Bounded evidence |
| --- | --- |
| Direct helpers | Defined remove_white no-shift strings and all writable tails; packed-angle selectors -3/0/1/2/3/4/5, signs and components; fractional/negative LMST dates, wrapping and both output cells; forward/backward precession, distinct nutations and all nine matrix cells. |
| Pulsar vectors | Zero/signed counts, two pulsars, cardinal/general/near-pole directions, repeated calls, eclCoord 0/1/2/-1, PM and second derivatives with binary128 parameters, finite-angle/nonfinite-PM classes. |
| UT1 | Header skip, character-57 cutoff, partial successful rows, single-MJD termination, zero plus later written unused entries, both strict boundaries, interpolation and endpoint extrapolation, sign, warning history, first load and changed file/root after cache; missing-file and 1000-row exits; real-resource smoke. |
| TAI driver | Methods 1/2/0/3; clockCorr 0/1/2/-1; real LOCAL→UTC→TAI chains, override, fallback/warnings, two pulsars, repeated calls and a partial failure after an earlier zero write. |
| IAU helper/driver | Distinct position/zenith, TT/UTC split-boundary fractions, nonzero EOP polar motion/rate, below-first proxy and interior rows, real EOP first-use retention, missing file, independent stored TT steps and clockCorr UTC switches. |
| Legacy/full driver | Multiple pulsars/observations, signed/zero counts, delay bypass/first-observation warning, methods other than IAU falling through, ecliptic 0/1/2, supplied nutations/UT1, near-pole velocity branch and combined TAI/observer history. |
| Satellites | Exact and differently cased STL/STL_FBAT/STL_BAT; all 16 polynomial masks per axis, non-one flags, epoch optionality and X-velocity-only missing-epoch exit; flag order/duplicates/failed scans; metre versus FBAT units; per-axis offset modes 0/1/2/other, counts 0/1/multiple and interval boundaries/no match. |
| Shared state | STL_BAT troposphere effect scoped to its pulsar and visible before subsequent observations/pulsars; retained satellite zenith/unassigned axes, all coordinate/nutation tails; exact IMAG xyz updates, alias/case lookup and unchanged stored geodetic coordinates; shared state before an unknown-observatory exit. |

Every source action exports initialized relevant caller inputs/results and checks
all other initialized fields using the complete typed S/R inventory, including
parameter backing. Padding and pointer addresses are excluded; pointers use
logical owner/index identities. Binary64 finite bits and binary128 bytes retain
signed zero; nonfinite classes remain distinct. Known string tails and private
UT1 cells outside logical counts are preserved. No snapshot performs a resource
lookup, advances a file or clears a cache.

The observer build adds one recorded pointer-registration call inside ut1red,
then reads its live statics after actions and at exit. The ordinary executable
has no such patch or exit callback. The audit strips only the added private
snapshot/exit event and requires exact ordinary result, caller state, diagnostic
stream and status agreement. Observer executions are themselves freshly replayed.
EOP cache retention is observed through real calls/file/root changes; no private
EOP reset hook or fabricated final-row derivative exists.

Diagnostics retain raw stdout/stderr files. Projection replaces only declared
source/case/assignment prefixes; it never scrubs numbers, line endings or warning
text. Harness streams are unbuffered with begin/end markers, an explicit profile
that does not assert whole-CLI buffering equivalence.

## Verification results and local evidence

All five verification commands exited **0** with `status: PASS`; preservation
before/after differences are empty. The Python syntax and owned whitespace checks
also pass. These are native implementation/reference checks, not scientific
validation or a Rust regression run.

- **127 normal completions and six defined exits**, with **1,734 actions**;
  1,728 complete and six stop at the recorded failure prefix. Recording and normal
  verification each execute every case twice in fresh processes.
- **35 verifier controls:** three positive controls and 32 rejected corruptions,
  including ordered/duplicate membership, nested bool/int/float/null/string
  types, owner/state/tail/value changes, diagnostics/status and actual launch,
  source dependency, input and resource identities.
- **15 compiled mutations**, each with actual changed native state/results:
  UT1 sign/boundary/reload, SAT argument, vector ecliptic scope/PM scale,
  precession transpose, satellite units/epoch/offset mode, IMAG shared retention,
  STL_BAT troposphere, IAU rate/matrix direction and clockCorr==2 handling.
- **19 supporting analytic/structural checks**, including cardinal direction,
  J2000 identity, packed-angle values, SOFA rotation norms, UT1 sign/cache,
  EOP retention, IMAG geodetic retention and pulsar-local STL_BAT side effects.
  Their small numerical diagnostic bounds are not comparison tolerances.
  Maximum observed norm differences were 3.469446951953614e-18 light-seconds
  for position and 4.440892098500626e-16 m for the seeded zenith vectors;
  the cardinal, epoch-identity and packed-angle checks had zero difference.
- **133 exact observer-equivalence cases**, independently replayed, plus seven
  prefix/retention assertions. Complete UT1 arrays include all 1,000 MJD and
  10,000 entry cells, not only the logical table.
- **9/9 routines, 327/333 executable lines and 207/214 compiler branch outcomes**
  in the separate coverage profile; all **133 cases agree exactly**, so measured
  profile numerical and diagnostic differences are **zero**. All 36 linked
  SOFA routines were entered. [GAPS.md](GAPS.md) accounts for every unhit outcome,
  including two defined reporting gaps and the local itsav invariant.

| Actual executable | SHA-256 |
| --- | --- |
| Ordinary (recording and fresh check) | `fc14717e6dc57929944f1cea5e1f31f658a3688acef82f5c86028048001709b6` |
| Private-state observer | `e3f0bbcb54bb81296a01a839ca2b18c852ff6e27dd607ab4e8c884d6ec6ecc2c` |
| Coverage | `63949c48148d844b56f5c383063410d055d06116f722512a367c58aeb4b90c1d` |

Each mutant's actual binary/source/profile/input identities and field witnesses
are in `outputs/mutations.json.gz`; no changed hash alone is a behavioral success.
The ordinary raw transcript and oracle remain separate from all mutated runs.
Pre-oracle pilots are retained in `build/pilot*` and `outputs/pilot*_raw`.
A pilot synthetic IMAG row was rejected by the source's geodetic-format heuristic;
only the authored fixture was corrected, before freezing/recording the oracle.
This was not a TEMPO2 source correction.

All following paths are local and ignored, relative to this directory:

| Evidence | SHA-256 |
| --- | --- |
| outputs/frozen/source_map.json | `9d6f8f3b82f4da5e230c7d6f74337a25fdf95f6b9b6df382a437f6c484cf1989` |
| outputs/frozen/protected.json | `f546ae37f2ccd32e6ed989fb93a2658b1b19475fd8dbe8ed4bc2a0ff872f00ae` |
| outputs/frozen/cases.json | `05e65905bc9cb2d418713288e58184410ba8532f186cfd360bfe8b91cdcf4ded` |
| outputs/reference.json.gz | `e810f70fa52c778059413cc50ea24d5df96cfebd7b345aaa1efb4531b19b0a85` |
| outputs/manifest.json | `b1934cb291627fe6332f57348bacdcc8abcba59df366fff2e8c8dab52091cb6c` |
| outputs/last_execution.json | `7c7b6d671b4cc99701220d1867b53599118ccfe54b90827e42d0c200468f4836` |
| outputs/execution_receipts.json.gz | `8324fbee40c83342ddf8d32b560afba90525fd7cd4b743b6a3a89dfa295ea044` |
| outputs/controls.json | `c2acfb7cbf38fc5803512475ed7581525c6f9d88c5a7d7906d2a93946d453e67` |
| outputs/independent.json | `7b711ee54b50941ba2acb70cc24ab8b6217fe0116a38ddd938c8e32b5296c64b` |
| outputs/mutations.json.gz | `23926ce3d83e7114e3b2b51ba76c478dd5b8f6941c67caa571deee80d1774c8a` |
| outputs/observer_audit.json | `569ebdbf02529e885f5bdf69207f0434fb69d9f61c285be2dfb0473f8a0cdb3a` |
| outputs/observer_snapshots.json.gz | `1d83defe91aaa0cf1487933b908a543a3a27daee4da6a9141b699a91c27b6ea2` |
| outputs/coverage.json | `23b8f3f3024edee66972742e264c3c1ec1927260f9679ad5dc53c027f84d8c07` |

Raw stdout/stderr/event streams are in `outputs/record_raw`, `check_raw`,
`replay_raw`, `observer_raw`, `observer_replay_raw`, `mutations` and `coverage_raw`.
Execution receipts retain their hashes and before/after resource identities.
Build inputs, dependency files, link maps and symbols remain under `build/`.
`outputs/commands.json` records actual command statuses and elapsed times.

The 7,129-member preservation baseline excludes W's evolving Rust/resource-port
paths, this directory and central documents. Both independent repository pins
and clean statuses are checked before/after each verification command. Generated
JSON, compressed snapshots, raw streams, binaries, profiles and coverage remain
local under `.gitignore`, as required by 0039.

The source-contract commit is `a59be00`; the harness/verification commit is
`6d19dd4`. The final report commit ID is returned in the delivery message.
Only explicit owned paths are staged.

## Limits and source-bug policy

No scientific correction or separately corrected native experiment is included.
Compiled changes are deliberately wrong test mutations, not accepted alternatives.
Original source evidence is retained under 0053. Overlapping strcpy shifts in
remove_white (get_obsCoord.C:462–475) are a demonstrated native-invalid operation;
a future safe implementation needs its own corrected expectation and regression.
IMAG stale geodetic fields, failed-scan rescaling and the X-only epoch guard are
reported original behaviors, not automatic claims about intended physical meaning.

Unrun: shifting remove_white branches; invalid float-to-int conversions and
integer overflow; empty/invalid UT1 backing; final-row EOP slope and malformed
brackets; oversized/nonterminated strings or bad counts; unproved cross-language
buffer aliases; allocation/read fault injection; concurrency and other platforms
or locales. TKlog host portability limits from M/U remain. No full SOFA,
JPL/CALCEPH, ephemeris/nutation generation, secular motion, complete BAT timing,
production Rust transfer or scientific validation is claimed. M2 and the fixed
numerical gates are unchanged.
