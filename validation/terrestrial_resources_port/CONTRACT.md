# W frozen transfer contract

Frozen before production edits, 24 October 2026. Governing decision 0056;
raw pin fda6ad4a9753ad7dc5d291406f2f310a9161423f. U's immutable
CONTRACT.md, GAPS.md, outputs/frozen/source_map.json and cases.json are incorporated
by exact identity in build/freeze.json. The complete routine bodies, every branch,
all native caller occurrences and 349/95-field type map remain at those original
paths and are checked, not reauthored or rebaselined. W owns rust/ and this harness.

## Correspondence, fields, lifetime and callers

| Source routine | Canonical Rust target | State / branches / callers |
| --- | --- | --- |
| readObservatoryFile | ObservatoryDatabase::read_file | Append six f64 cells; fixed name[32], code[16], clock[16] Option bytes, retaining local tails across rows, reset per file. Numeric-prefix scan, comments/chunks, geodetic-before-count error, 5/6 fields, default clock truncation; direct/init callers. |
| readAliases | ObservatoryDatabase::read_aliases | code[100] Option bytes retained per call; accumulated alias list, warned flag, file order; init/direct callers. |
| initObservatories | ObservatoryDatabase::initialize | Explicit call always appends main before glob extras, then aliases, publishing initialized only on success; root/alias selectors. |
| lookup_observatory_alias | ObservatoryDatabase::lookup_alias | Lazy guard only here; first case-insensitive match, warning per list, initialized output storage and retained tails, in-place identity supported. Used by get and future preprocessing/TIM consumers. |
| getObservatory | ObservatoryDatabase::get/index | Alias before code/name first match; index/derived snapshot survives vector relocation. Clock source lookup, runtime, tropo, geometry consumers. Unknown site explicit source_exit. |
| fang | fang | Both checked integer conversions, all mode conditions, binary64 expression order. Geodetic-error path and direct caller. |
| ITRF_to_GRS80 | itrf_to_grs80 | Writes longitude/latitude/height only, GRS80 constants, y>=0 includes -0; read_file/geometry. |
| GRS80_to_ITRF | grs80_to_itrf | Writes xyz only; geodetic-error/geometry. |
| load_EOP | load_eop | Explicit caller-owned Vec<[f64;4]>, append without sort/finite filtering. Per-call format; fixed widths and numeric-prefix scans, failed-scan exit prefixes. |
| get_EOP | Eop::get_eop | Process-private first-use samples, separate initialized flag, no RuntimeContext-owned cache. Initialized output slice + four checked indices; proxy writes then derivative/xp/yp/dut1. Strict leaps, 86400 divisor. get_obs_coord consumes query result. |
| MeteorologyFunction_load | MeteorologyFunction::load | Canonical TabulatedFunction and site[256], fixed filename/header[256] knownness/tails; direct objects distinct from shared tables. |
| MeteorologyFunction_getValue | MeteorologyFunction::get_value | Existing table binary search, diagnostics and endpoint behavior. |
| MeteorologyFunction_getStartMJD | MeteorologyFunction::start | Existing table first sample boundary. |
| MeteorologyFunction_getEndMJD | MeteorologyFunction::end | Existing table last sample boundary. |
| initialize_meteorology_table | initialize_meteorology_table | Clear one vector, reusable local record, sorted existing glob helper, warning-dependent diagnostics; per-kind explicit call. |
| initialize_meteorology_tables | Meteorology::initialize | zwd before sap; publish shared flag after success; explicit reinit replaces vectors. |
| getMeteorologicalValue | Meteorology::get_value | Lazy initialize, first matching case-insensitive site and inclusive range, +0 on miss; return selection/provenance from same lookup. |
| getZenithWetDelay | Meteorology::wet | Zero sentinel emits TROP1, stays zero; shared diagnostic history. |
| getSurfaceAtmosphericPressure | Meteorology::pressure | Zero sentinel emits TROP2, 101.325 kPa; shared diagnostic history. |
| NMF_hydrostatic | nmf_hydrostatic | Conversion-safe latitude, clamp/interpolation, hemisphere/season, source height formula and binary64 constants. |
| NMF_wet | nmf_wet | Same latitude branches, original mixed b/c interpolation retained pending demonstrated correction evidence. |
| compute_tropospheric_delays | compute_tropospheric_delays / borrowed single-observation body | T SourcePulsar, R SourceObservation; signed loops/backing; per-pulsar warned; reset before tests; zero zenith, STL_FBAT and COE order; SAT weather and clock-shifted UTC season; no other field mutation. Real clock/get/table helpers; form_bats consumes this body. |

All old RuntimeContext EOP/weather fields and parsing disappear when callers are
integrated. Observatory snapshot is derived from the one canonical record.
ClockSession remains the single mutex owner with separate observatory/EOP/weather
flags. Resource operations and lifecycle/diagnostic access do not inherit terminal
clock calculation faults; clock operations keep their existing terminal boundary.
Outer operations lock once; nested helpers borrow state. Provenance records actual
loaded bytes, retained paths and selected tables, without a second lookup.
Source exits are library errors with inspectable prefixes; invalid storage is a
contextual boundary, not a native golden. Direct failed table objects retain the
existing table-local fault. EOP query boundaries remain recoverable; at/after the
last row there is no selected replacement slope. Source-invalid glob NOMATCH with
nonzero warnings is a safe contextual error. No concurrency behavior is claimed.

Current consumers frozen in build/freeze.json include runtime.rs, clkcorr.rs,
form_bats.rs, get_obs_coord.rs, resource_tests.rs, clock_tests.rs and the two old
terrestrial examples. Native callers outside current Rust scope (full preprocessing,
TIM, polyco and plugins) are mapped in U, not claimed implemented. Literal-disabled
observatory conversions/test loops, tropo tests and table linear search stay disabled.

## Comparison projection frozen before implementation

All 130 U cases, 1128 actions, 2256 events, including 24 exits, run in debug/release.
U's value-initialized two pulsars/twelve observations are fixture inputs only;
no production initializer defaults change. Every non-target field must retain its
initialized input; fixed bytes and known prefixes/tails remain exact. EOP private
cache is additionally checked through changed-root/file histories.

Finite f64 bits including signed zero, nonfinite classes, nested scalar types,
ordered events/field/case/action membership, diagnostic content/truncation and all
resource state compare exactly. Binary128 SAT serialization is an explicit fixture
representation projection from Rug128; the parsed value stays Rug128 in production.
Only profile sizeof/longdouble ABI fields may differ; capacities, values and
knownness may not. Library outcome source_exit maps to native status 1, success
maps to 0; actual test-process status is separately recorded. Host source/build/
case roots are replaced by the exact U declared placeholders; full raw bytes are
retained. Fixed filename bytes are decoded/projected/re-encoded only at that exact
root (their basename normally contains none). No warning or numeric scrubbing.
Source path length affecting TKlog truncation must match U's source prefix length.

No correction expectations are selected initially. A proved 0053 correction must
have an adjacent pinned-source comment, independent/native-patch witness, separate
explicit case/field expectations, original witness and affected integration gate.
Any expression-specific mul_add requires compiled-source evidence and a bounded
reproducer; blanket contraction/tolerance changes are prohibited. Rug128 and the
four historical M2 diagnostics remain untouched.

## Verification sequence

Freeze identities and maps; observatory storage and geometry; EOP/table state;
meteorology/driver/frontend; complete U comparison and adversarial controls;
format/Clippy/unit and T/O/R, clock/glob/provenance, JPL, M1a/M1b, complete M2.
All older harness output routing occurs in W-owned disposable staging with exact
original hashes and recorded path-only diffs. Generated JSON, binaries and raw
execution evidence stay ignored. Explicit-path Conventional Commits contain only
sources/generators/authored contracts/reports. Final report distinguishes source
agreement, implementation checks, corrections, native-invalid and untested gaps.

## Central ownership clarification (after freeze)

Central's dispatch of 0057 explicitly excludes evolving central documents and
X's new validation/observer_coordinates/ from W's immutable preservation gate.
The original build/freeze.json remains unchanged; its central-document entries
are historical identities. check() still checks every accepted reference/evidence
member and both nested pins. The initial HANDOFF drift failure is retained in
pilot logs. This is an ownership clarification, not a source/oracle rebaseline.

## Selected correction and downstream checker applicability

CORRECTIONS.md selects the demonstrated NMF_wet upper-c coefficient correction
under 0053. The pre-edit table above remains historical. Ordinary run.py compares
against a separately identified native one-line patch, never a rewritten U oracle.

stage_regressions.py records original/staged hashes and every diff. T's direct
and supplemental checks run with ROOT/OUT routing only. R's whole-file SOFA hash
assertion predates W's safe EOP argument adaptation: the staged verifier uses the
current get_obs_coord.rs hash after checking both original unsafe blocks byte for
byte against a8ef85c. The five existing clock_text.rs unsafe blocks are likewise
unchanged. This updates checker applicability, with no numerical/field projection
or protected caller modification. R's remaining checks and the O/clock/JPL/M1/M2
chain execute their original logic. Current frontend resource tests now require
process-lifetime EOP/weather retention and warning history; fresh subprocesses
exercise changed weather files. Old per-context reset expectations contradicted
the accepted U lifetime and are replaced only in the owned Rust example.


The first downstream run exposed an old fitting recovery assertion requiring
identical warning output. Exact recursive comparison showed only
cycles[0].prefit.warnings changed (five initial load/TROP/DUP messages versus an
empty later batch); every other field matched. The owned fitting_tests.rs now
excludes only that first warning batch, retaining exact comparison everywhere
else. U's warning histories remain exact gates. The original failure and its
complete field-level diff remain in W's failed_attempt_1/; no fitting algorithm
or numerical expectation changed.

## Final guard review under the updated standing rules

After the complete regression run finished, review of the actual loader contract
proved MeteorologyFunction::load's site.len() >= 256 guard redundant:
TabulatedFunction::read_bytes rejects headers longer than 255 bytes and requires
'#', so a token from header[1..] is at most 254 bytes. Remove only that duplicate
guard and document the dependency invariant. EOP bracket checks still belong at
the query boundary because load intentionally accepts unordered/duplicate rows;
T scoped owner checks protect real release/view operations. Canonical source
storage may be partly initialized; it is not a fully validated TimingInput.

The complete pre-removal regression receipts remain unchanged, with U traces and
executables copied under before_guard_review/. final_review.py proves that this
is the only subsequent production-code delta, verifies unchanged dependent
algorithms/examples, and runs focused existing loader/meteorology tests,
formatting/Clippy and all 130 U cases/controls in both profiles. final_scope.json
binds the final sources/binaries without relabeling the older full regression
receipts. The unrelated full chains are not rerun solely for a source stamp.
