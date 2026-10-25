# Assignment AA: observer-coordinate Rust transfer

Delivered for central review under decisions 0062, 0065 and release 0066.
Central retains acceptance. The public grammar remains the isolated F0–F3
`@`/PKS subset; the complete internal observer driver is a dependency of the
general input-to-BAT track, not a claim that that track is complete.

Release: `4498cf1c4bb9167a57e4cb3b1226cbd73e3ee2d3`. Accepted Y: `21dc956`
(production `11165d1`). Accepted X: `02234cd`, contract `a59be00`, harness
`6d19dd4`. Native source: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`;
independent A2: `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`.

Authored stages are `bd7ce9d` (pre-edit contract/map freeze), `716375f`
(implementation and canonical probe), `e4883a4` (verification), and `99cf714`
(single IAU boundary and focused live-frontend history). The last stage removes
a duplicate TT conversion check and adds a test; it changes no formula or
source branch. All 133 histories were compared again in both profiles on that
final implementation. Earlier regression/control executions retain their actual
716375f source identities; they are not relabeled as 99cf714 executions.

## Frozen source, field and caller map

`../validation/observer_coordinates_port/CONTRACT.md` was committed before
production changes. Its ignored `build/freeze.json` contains all nine raw
routine bodies, every lexical branch and preprocessor site, every pinned
caller/declaration occurrence, complete 349-pulsar/95-observation field types,
the 36-routine SOFA dependency closure, and actual accepted Y consumer sources
and Git blobs. It also pins 40,923 accepted files without freezing evolving
central or independent assignment directories. The completed central whitespace
bundle is a numerical input; its evolving parent directory is not frozen.

| Source counterpart | Rust entry and consumers | Read/write and retained-state contract |
| --- | --- | --- |
| `vectorPulsar.C::vectorPulsar` | `vector_pulsar::vector_pulsar`; canonical example/probe. `form_bats` uses the same `position` body | RAJ/DECJ narrow before trig; PM/PM2 remain extended through source grouping and uncancelled cos(dec). Writes posPulsar, equatorial copy, velocity and acceleration. Any nonzero eclCoord rotates only the position copy. No observation owner required. |
| `get_obsCoord.C::get_obsCoord` | `get_obs_coord::driver` over canonical `PulsarContext` | Pulsar/observation source order. Delay bypass retains everything and uses first-row warning rule. Exact satellite names select polynomial/flags/offset branches. STL_BAT changes only position prefix, velocity and pulsar correctTroposphere. Other names use the real alias database; IMAG mutates shared xyz. |
| `get_obsCoord_IAU2000B` | `get_obs_coord::iau2000b`; full driver and existing `form_bats` | One TT split, EOP lookup and SOFA calculation. UTC clock correction bypass for 0/2 remains independent of stored TT sum. Full driver rotates position/velocity only for eclCoord==1. Zenith stays unscaled. |
| `lmst` | `get_obs_coord::lmst`; legacy driver and direct probe | Checked native truncation/subtraction, source doubles, public extended fortran_mod at Rug128, negative-remainder adjustment, two outputs. |
| `get_precessionMatrix` | `get_obs_coord::get_precession_matrix`; legacy driver | Source angles, supplied delp/dele, OBLQ, precession/nutation product and transpose write order. All nine outputs. |
| `ang` | `get_obs_coord::ang`; callable internal helper | Floor-based packed components and exact selectors, with defined integer range. Distinct from fang. |
| `remove_white` | `get_obs_coord::remove_white`; callable internal helper | Byte/NUL algorithm and full stale tails. No pinned core caller exists; it is not inserted into parsing. Three source shifts corrected under 0053. |
| `tai2ut1.C::tai2ut1` | `tai2ut1::tai2ut1`; canonical driver/example | Exactly method 2; nonzero clockCorr uses ut1red(SAT) then actual correction to TAI, binary64 addition then extended field write. Zero writes zero; other methods retain correctionUT1. |
| `ut1red` | `Ut1::red/calculate`; tai2ut1 and direct probe | Two header reads, fgets size 1000, 57-byte prefix, sequential partial scans, zero sentinel, row-limit prefix, strict interior four-sample interpolation, endpoint/warning behavior and final negative sign. All 1,000 MJD and 10,000 entry cells retained. |

The satellite base flag must equal 1; derivative masks are independent. Missing
epoch defaults to zero except the original X-velocity exit after velocity zeroing
and before position assignment. Flags follow polynomials, duplicate flags
replace in order, failed scans retain the target before STL divides it by c.
Each axis keeps its own count/table/selector and left-inclusive/right-exclusive
first interval. Modes 0/1 interpolate, 2 holds, others add zero; count one has
no interval. No source asymmetry or interpolation policy was changed.

Exactly method 1 chooses IAU; other methods use legacy geometry, supplied UT1
and nutations. Legacy zenith zeroing, ecliptic position before velocity formation,
then velocity rotation remain source-faithful. Coordinate cells 3–5, all six
nutations, satellite zenith, unset axes, flag tails and every non-target typed
field remain observed. Bypassed/unwritten fields are not eagerly read.

## Ownership, frontend integration and unsafe map

`ClockSession::ut1` is distinct from clock routing, EOP, observatories, weather,
IFTE, Fortran-reader and FB state. Its first successful table survives file/root
changes. An unsuccessful load never publishes `read=true`: it keeps its written
prefix and a table-local terminal error. This defines safe continuation after
a native exit; it neither invents a native result nor poisons the other resource
owners. A later interpolation boundary is an error at that call, not a global
clock failure. The source's automatic `itsav=-1` is not an inter-call cache.

The outer owner supplies a single session borrow. Actual clock calls use the
existing clock fault contract; nested work does not relock. The existing scoped
`pulsar_observations_mut` now returns disjoint mutable pulsar and row borrows,
with foreign/released owner and offset checks retained. Signed nonpositive
observation loops need no row owner. No authoritative pulsar clone was added.

IMAG resolves `ObservatoryDatabase::index`, then updates that shared record's
xyz in place. Latitude, longitude and height remain cached. `RuntimeContext`
no longer stores a geometry clone: every frontend calculation fetches the live
canonical record, then calls the same IAU function as the full driver. Resource
provenance comes from already consumed/cached metadata, without a second
stateful lookup for reporting. Fixed-direction vector arithmetic is likewise
shared, while downstream propagation's normalized direction stays distinct.

The isolated real-runtime test first creates and evaluates an existing public
runtime, maps IMAG to PKS, changes xyz through the full canonical driver, and
evaluates the same public runtime again. Its geometry changes and agrees with
a newly constructed runtime; cached geodetic fields remain exact. It passes in
both profiles. Canonical ownership tests also cover signed no-ops without rows,
vector formation without rows, foreign/released owners, failed UT1 prefixes,
unrelated subsequent work, unterminated bytes and invalid integer conversions.

| Necessary unsafe boundary | Necessity, ownership and lifetime |
| --- | --- |
| Existing `get_obs_coord.rs` foreign declarations | Retain the established SOFA Fortran ABI, by-reference f64 scalars and contiguous column-major arrays. |
| Existing SOFA call block | Owned initialized 3/6/9-element inputs/outputs. Cross-product position and velocity slices are disjoint; other input/output buffers are separate; pointers never escape. Automatic Fortran locals remain protected by the unchanged `-frecursive` build. |

Both unsafe blocks are byte-identical to the release's blocks. There is no new
unsafe in Rust coordinate, whitespace or UT1 logic and no second FFI layer or
SOFA implementation. Safe copy implements the source cp semantics; c2t00a
remains declared-but-uncalled source scope. Existing scanner FFI and all build
flags remain unchanged.

## Original, corrected and safe-boundary dispositions

All **133 original histories / 1,734 actions / 3,462 ordinary events** compare
exactly in Rust debug and release. The separately audited X observer adds the
six exit snapshots, yielding **3,468 compared events per profile**. There are
127 normal histories and six defined source-exit histories. No original case
was dropped, given a relaxed tolerance or replaced by a corrected oracle.

Comparisons preserve ordered member/type identity, knownness, source prefixes
and retained tails, logical owners, complete private UT1 state, resource and
warning histories, finite binary64 bits, signed zero and nonfinite classes.
Extended outputs are exported from Rug128 to binary128 only at the observation
boundary; production stays Rug128. Exact declared path-root substitutions are
recorded; raw streams and input/executable challenges are retained.

The six source exits are missing UT1, the UT1 thousandth-row limit, TAI failure
prefix, missing EOP, missing-X-velocity epoch, and IMAG's defined EOP failure
prefix. They produce contextual errors with the same observed writes/diagnostics.
Invalid/unwritten backing, invalid native conversions, missing string terminators
and empty UT1 tables stop before invalid source operations; no successful native
golden is claimed. A failed UT1 load cannot be retried over its partial state.

At `fda6ad4 get_obsCoord.C::remove_white:464,469,474`, strcpy can read and
write overlapping suffixes (e.g. `"  x"`). Rust `copy_within` moves the same
suffix including NUL, preserving source branch order and old tail bytes. The
adjacent comment identifies the defect and correction. All **19 complete
corrected buffers**, each repeated, match the existing central memmove experiment;
its **seven original no-shift neighbors** remain exact. Mixed separators,
newlines, carriage returns, embedded NUL, non-ASCII bytes and guards are covered.
Original overlapping copies were not executed. This is the sole upstream bug
correction in AA; it is not a new parser normalization policy.

The first diagnostic replay matched 115/133 histories and exposed 18 contraction
witnesses in lmst, precession, UT1 interpolation, legacy geometry and satellite
offsets. Actual accepted native objects and freshly emitted assembly establish
the precise GCC16 O2 operations. Local mul_add reproduces them; the final
histories match exactly. Adjacent source-site comments and
`build/arithmetic/receipt.json` preserve the evidence. No global contraction,
reassociation, solver change, sorting, precision switch or tolerance change was
introduced. Angle and derivative coefficients retain source construction,
including the M_PIq decimal literal at the approved backend precision.

## Verification and receipt chronology

All paths below are under `../validation/observer_coordinates_port/` unless
stated otherwise. Generated evidence remains ignored under 0039.

| Execution | Result and exact applicability |
| --- | --- |
| Final `outputs/{debug,release}/direct.json` | Final 99cf714 implementation: all 133 X histories, ordinary and complete private-state comparisons. Each receipt pins compiler, retained executable, source/harness/input identities and every result hash. |
| `build/pre-single-boundary/{debug,release}/controls/receipt.json` | 716375f bodies: 35 controls (3 positive/32 rejecting), 19 supporting checks, 15 retained compiled-native mutation witnesses newly compared with Rust results, 19 corrected buffers / 7 original neighbors. No native mutation corpus was rebuilt. |
| `build/regressions/Y/{debug,release}/receipt.json` | Complete 97 accepted Y histories / 1,368 events / nine source exits per profile against retained corrected evidence. The equal-length AA staging root replaces only Y's exact input-root bytes, including hex-encoded diagnostics. Raw streams remain separate. |
| `build/regressions/W/{debug,release}/receipt.json` | Complete 130 accepted W histories / 2,256 events per profile, including its accepted NMF_wet correction. |
| `build/regressions/T/{debug,release}/receipt.json` | Seven affected lifecycle histories / 29 events: initialized/wrapper rows, null destruction, partial/shared/forward-overlap/backward-overlap copies. Unrelated large matrix histories retain their historical receipts. |
| `build/regressions/R/{debug,release}/receipt.json` | Complete R probe: 95 fields, three source cases, four copy cases, unwritten/boundary cases. Accepted R runner and preservation membership are unchanged. |
| `build/regressions/O/{debug,release}/receipt.json` | Complete O probe: 2,000 slots / 9,616 cells, ten cases, eight boundaries and supplemental copies. |
| `build/public/aa-receipt.json` | Accepted M1/M2 verifier, output routing only: debug/release tests (including JPL, clock/table, lifecycle, Y and W units), M1a/M1b/4 ns gates, actual public CLI/resource recovery, comparator controls and debug/release equivalence. Exactly four historical M2 failures remain. |
| `build/followup/receipt.json` | Final 99cf714: formatting, all-target Clippy, three ownership/boundary tests and isolated real public frontend IMAG history per profile. Retained actual test binaries. |
| `outputs/examples/{debug,release}/` | Fresh executable terrestrial, satellite and combined vector/UT1 examples, with inputs, challenges, raw state and summaries. Initial example receipts execute 716375f; final direct/followup cover the subsequent IAU consolidation. |
| `build/preservation.log` and `build/delivery.json` | Final protected-file, independent-repository, unsafe-block and receipt identity audit. |

Earlier complete X replay/control bundles originally written under
`outputs/debug/` and `outputs/release/` were preserved byte-for-byte under
`build/pre-single-boundary/debug/` and `build/pre-single-boundary/release/`.
Their recorded launch paths resolve with that explicit prefix relocation;
their original compiler/source/executable/input hashes are unchanged. The
initial failing diagnostic bundle is `build/initial-debug/run/`. Its intermediate
Rust binary was superseded before content-addressed retention was introduced;
that bundle is diagnostic history, not a reusable final-execution identity.
The actual pinned native objects, all final successful binaries and final
source/input identities are retained. Successful retained executables live at content-addressed `build/binaries/{profile}/SHA256` paths.
No prior accepted X/Y/W/R/O/T oracle, runner or receipt was overwritten.

These are implementation/source-transfer checks on the declared macOS arm64
host, not convergence tests or validation of the entire physical timing model.

## Executable examples and reproduction

From the repository root:

```sh
python3 validation/observer_coordinates_port/examples.py --example terrestrial
python3 validation/observer_coordinates_port/examples.py --example satellite
python3 validation/observer_coordinates_port/examples.py --example vector-ut1 --profile release
python3 validation/observer_coordinates_port/run.py --profile debug
python3 validation/observer_coordinates_port/run.py --profile release
python3 validation/observer_coordinates_port/controls.py --profile debug
python3 validation/observer_coordinates_port/controls.py --profile release
python3 validation/observer_coordinates_port/followup.py
python3 validation/observer_coordinates_port/freeze.py --check
```

Examples construct canonical state explicitly and execute production routines.
Terrestrial demonstrates the IAU/clock/EOP path, satellite demonstrates TELX
polynomials and retained coordinate prefixes, and vector/UT1 combines source
vector formation, TAI correction and legacy observer geometry. JSON summaries
are observations of these executable calls, not manually written outputs.
`public_checks.py` exercises the existing public frontend and its recovery gates.
`regressions.py --suite Y|W|T|R|O --profile debug|release` replays affected Rust
probes against recorded native evidence, using AA staging only.

## Remaining work and acceptance limits

General PAR/TIM grammar, satellite directives/flags, PM/PM2 and legacy-UT1 public
configuration, multi-pulsar input lifecycle, planetary/propagation integration,
secular motion, full phase/reference orchestration and larger timing/binary
models still need parser/orchestration integration. Internal source routines do
not establish new public CLI capability. Source CVS gaps not executed by X,
native-invalid aliases/casts/backing, external concurrent resource writers,
broader concurrency, other platforms/locales and full SOFA branch coverage
remain outside this evidence. No independent reference-only phase was added.

M2 remains unaccepted with the same four raw SVD-order/singular-index findings.
The four decision-0049 binary113/Rug128 precision-boundary findings remain open.
No precision switch, sorting, tolerance relaxation or waiver is proposed.

## Key receipt SHA-256 identities

| Local evidence | SHA-256 |
| --- | --- |
| `build/freeze.json` | `8c7d116a02394199e9194877b67bd5041b911e950a9ae3692c009c3afd98c1e4` |
| `outputs/debug/direct.json` | `85f5a9887894943f0dcf936acc1a882724b34d94c8d5e002a997c729103c8925` |
| `outputs/release/direct.json` | `fa8a104fe00698225413cfe9d5f0032ab9f92198f7752f5ef7c20042e9e9c9e9` |
| `build/pre-single-boundary/debug/controls/receipt.json` | `52e96617975e517f060d05e0471e456159214668b1c4e58a3db22f93aa9c6892` |
| `build/pre-single-boundary/release/controls/receipt.json` | `549d5fb5440ce2a4ed43e43d4cb731151702424506d7a8292079b682843f7880` |
| `build/followup/receipt.json` | `1468c87cce55a208a81cacd32b827c8b9f7aa7837d3ab273f144a1a6cb5198e3` |
| `build/public/aa-receipt.json` | `159f776e4f5e163557e891f544bf0590d9d7593bfc23b9d45a839df5e537630f` |
| `build/arithmetic/receipt.json` | `474932d1a9c9e0ae122461837ebd9a63951a9f50698672eff0bf2ec462157ea7` |
| `build/delivery.json` | `87c49810d758791f7519ce4da3d05432bc43634036bf9292027b9de42df7dd2e` |
