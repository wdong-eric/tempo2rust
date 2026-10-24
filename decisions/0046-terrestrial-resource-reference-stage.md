# Terrestrial resource reference and Assignment U

Central decision, 24 October 2026. **Release Assignment U to establish the complete bounded native reference for observatory lookup/geometry, Earth-orientation tables and meteorology/tropospheric delays.** U owns only new `validation/terrestrial_resources/`. T remains the sole Rust owner under 0045. Central retains later Rust contracts, numerical policies and milestone acceptance.

This advances the remaining resource and propagation dependencies alongside T. J/L already establish selected observatory lookup, aliases, clock dependencies and table behavior; they do not establish full observatory geometry or EOP/meteorology lifetimes. Their accepted evidence remains read-only. F's inventory is a historical planning baseline, not a current implementation claim.

## Source and complete primary membership

Use unchanged raw TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`:

| Primary source | Raw SHA-256 | Active routines |
| --- | --- | --- |
| observatory.C | `fabfaabc78b75652590c5a81c1c2bb05f806550f99dbd7dd3f1ab43d056bf2c3` | readObservatoryFile, readAliases, initObservatories, lookup_observatory_alias, getObservatory, fang, ITRF_to_GRS80, GRS80_to_ITRF |
| eop.C | `ff90736cb57c96ce06411e4aa5dbb692b5c5ec0f4a9069fd9403eb7bb9d469a7` | load_EOP, get_EOP |
| tropo.C | `871d06bb10e647df0d563905db20b560cd659fcb4b372fed0c790937835a7fe6` | NMF_hydrostatic, NMF_wet, MeteorologyFunction_load, MeteorologyFunction_getValue, MeteorologyFunction_getStartMJD, MeteorologyFunction_getEndMJD, initialize_meteorology_table, initialize_meteorology_tables, getMeteorologicalValue, getZenithWetDelay, getSurfaceAtmosphericPressure, compute_tropospheric_delays |

These are **22 primary routines**. Inventory every field, routine, runtime branch and preprocessor disposition in these files, including inactive convertGeocentric/test blocks and host glob macros. Record the observatory record, EOPSample, MeteorologyFunction and actual table/alias/cache state. No other linked routine becomes accepted merely by serving as a dependency.

Compile real pinned dependencies, including actual tabulatedfunction/clock routing, diagnostics, numeric helpers and required globals. Reuse J's measured binary128/C-locale profile and source-slice/build patterns where applicable, with new actual identities. Include unchanged primary files directly in a caller translation unit if necessary for read-only access to file-scope state; do not rewrite core functions, supply fake lookup/interpolation/clock results or add reset hooks. Verbatim global data/CVS slices must be bounded and hashed. No A2 initializer correction or A2 resource instrumentation may execute.

Pin every actual source/header/configuration, compiler/flags, numeric/runtime library, caller/runner, generated input, real resource and executable. A2's configuration and independently checked archive identity can establish profile context; they do not make patched objects the ordinary oracle. Validate raw Git blobs rather than expanded working-tree Id strings. Use explicit synthetic resource roots and a bounded real-resource smoke history, without altering canonical resources.

## Caller state, outputs and native safety

Freeze a source/caller/state/branch map and all case classifications before experiments. Each case starts a fresh process unless it explicitly declares an ordered shared-process history. Record complete initialized inputs, action order, output/state membership, changed/retained fields, streams and return/exit classes. Synthetic zeroed or seeded pulsars/observations are caller inputs, not source initialization. R/S's complete field maps are available for the projection; U does not need to invoke full initialization or allocate its covariance just to call tropospheric routines.

Distinguish initialized prefixes/cells from genuinely unwritten suffixes. Do not serialize object padding, unknown scanner leftovers or invalid pointers. Preserve known retained bytes rather than labeling them unknown. Addressable static vectors may be exported read-only; function-local cache behavior can be established through controlled histories without inventing accessors. Record logical row/alias order and owned pointer identity where meaningful. Do not dereference a saved observatory pointer after vector growth; reacquire it.

Defined IEEE nonfinite results, signed zero and failed scans with defined control flow are observations, not reasons to clamp or repair source. Before each unusual case establish that all native reads, conversions, string sizes, loop bounds and lifetimes remain defined. Undefined accesses/conversions/overflows, uninitialized reads and source-unbounded cycles stay unrun with source evidence. Host-qualified logger/formatter limitations from J/M remain qualified. Separate process exits from library recovery: a later valid fresh process is not recovery within an exited source process.

Snapshots and supplemental queries must not change warning/cache histories before the action being measured. Use independent replay histories for helper observations when their calls mutate state. Source-inspected local expressions and measured outputs are distinct evidence. Any extra instrumentation must be separately identified and shown to agree with the unchanged ordinary execution; it cannot silently replace it.

## Observatory lookup and geometry histories

Exercise direct file loading, explicit initialization and lazy lookup. Retain main observatories.dat before the other glob-ordered .dat files, duplicate/first-match ordering, case-insensitive code/name/alias matching, unchanged input versus in-place alias output, and missing-file/unknown-site exits. Cover TEMPO2_ENVIRON and TEMPO2_ALIAS selection, direct repeated reads/init calls and root/file changes after lazy initialization. Initialization appends observatories/aliases; do not insert a reset.

Preserve readAliases' per-call temporary alias vector accumulating across lines, global append order, comments and once-per-list warnings. Include distinguishable aliases, repeated names/codes, code-only and comment-bearing lines, and initialized output-buffer tails. Use J/L evidence as corroboration; new full-resource claims require their own declared cases and actual execution.

For readObservatoryFile, cover five/six-field forms, default clock-name snprintf truncation, ignored nonnumeric/comment lines, source chunk and scan-prefix behavior, valid coordinate rows followed by defined malformed rows, and the geodetic-detection/diagnostic order. The test of z occurs before the nread count rejection: an initial incomplete row may read unwritten z and must not be run as a normal parser failure. Bound every name/code/clock token to its actual source array. Do not interpret overflowing sscanf or unterminated buffers as defined behavior.

Exercise all fang mode branches, signs, component/carry boundaries and only representable integer conversions. Cover both geometry directions, northern/southern and longitude branches, equator/poles and meaningful height ranges. Preserve source constants and expression order, including defined nonfinite outcomes where preconditions allow them. Round trips and independent geometric diagnostics are supporting checks, not replacement oracles or a Rust tolerance. Literal-disabled legacy conversion remains inventoried and unexecuted.

## Earth-orientation histories

Exercise load_EOP's default fixed format, old-format marker and both accepted modern markers, in-file format transitions, year bounds, ignored lines, actual numeric-prefix scans, arcsecond-to-radian conversion, row order and append-to-existing-vector behavior. Supply sufficient initialized line storage for fixed-offset reads; do not execute short-line paths that inspect unwritten bytes. Use defined malformed fields to observe exact failure streams. Load-only cases may retain duplicate/unordered/nonfinite samples without claiming they can safely be interpolated.

get_EOP lazily retains the first successfully loaded table. Exercise filename/environment/file-content changes after that first use, below-first proxy behavior with a valid following bracket, first/interior/exact interior times, nonuniform intervals and positive/negative/no leap corrections. Preserve strict +/-0.5-second threshold comparisons and source dut1dot division by 86400 independently of sample spacing. Use declared initialized output buffers, including defined aliased-output histories where they expose assignment order.

**Do not call the known invalid upper-endpoint path:** at or beyond the final sample, the proxy is assigned and the subsequent scan still reaches size before reading samp[isamp]. Empty/singleton or other malformed brackets also need explicit bounds analysis. Record the source-inspected prefix/invalid access; do not invent a successful upper-proxy golden or lower a boundary to make it pass. Distinguish load_EOP's caller-owned vectors from get_EOP's private first-use cache.

## Meteorology, mapping functions and delay histories

Exercise each MeteorologyFunction wrapper with real TabulatedFunction behavior. Cover header/site parsing, initialized filename/header/site tails, source row acceptance and table endpoints through valid backing. Source table buffers are smaller than some read limits: use bounded inputs and retain J's unsafe-case classifications. Missing/bad/empty files and no-sample failures must have their actual status and streams.

For initialization, cover glob order, per-kind clear/load order, both zwd/sap kinds, explicit reinitialization, lazy first-use state and subsequent root/file changes. Analyze GLOB_NOMATCH together with dispWarnings and the actual host glob result before executing it: source has a defined early return for warnings==0, while other branches may reach a null path vector. Do not force an unsafe case to produce a golden. Glob allocation/read failures and inactive host branches remain explicit coverage gaps unless a separately justified defined experiment reaches them.

Preserve first matching site-and-inclusive-range table, case-insensitive site matching, overlapping tables, absent/out-of-range return zero, and literal measured zero being treated as missing. getZenithWetDelay retains zero and emits TROP1; getSurfaceAtmosphericPressure substitutes 101.325 and emits TROP2. Test warning suppression/history across both helpers and repeated callers. Do not replace these choices with a scientific missing-data policy.

Exercise both Niell helpers across every latitude interval/clamp, north/south seasonal sign, dates, elevations and heights. **Retain NMF_wet's source mixed-coefficient expression `frac*bs[ilat2] + (1-frac)*cs[ilat1]` for c**, along with the hydrostatic epoch/year and height expressions. Choose defined inputs around branch boundaries; invalid float-to-int latitude conversions remain unrun. Record finite values exactly and IEEE classes separately.

compute_tropospheric_delays requires multi-pulsar/multi-observation and repeat-call cases, signed counts, delayCorr/correctTroposphere flags, unconditional delay reset, source zero-zenith warning order, STL_FBAT spelling, COE name bypass, alias lookup and actual clock conversion to UTC. The local warned flag changes later observations' branch behavior and resets per pulsar/per call; test that history. Preserve use of SAT for meteorology, SAT plus clock correction for the hydrostatic season, the source elevation expression and pressure/height/latitude formulas. Exercise table-backed and fallback pressure/wet values. Other observation/pulsar fields remain unchanged unless a real dependency mutates them; capture that mutation instead of assuming purity.

Standalone helper cases and integrated clock/lookup cases must agree on their declared inputs. Do not import a corrected atmospheric expression, skip negative/nonfinite defined results, replace the source seasonal epoch or fix warning-dependent behavior.

## Verification and delivery

Normal verification freshly builds, checks all actual identities before/after launch, challenges case/input identity, removes stale success receipts and never overwrites its ordinary oracle. Compare exact ordered case/action/field membership, initialized values/bytes, finite bit patterns, signed zero/nonfinite classes, state histories and streams/statuses. Only a narrow declared path/address diagnostic projection is permitted; preserve raw streams so path truncation or differing error text cannot be hidden.

Include a positive control and meaningful omission/permutation/value/knownness/state/stream/status/identity controls. Build defined disposable mutations with observable witnesses for source alias accumulation, first-use cache retention, EOP leap threshold/derivative spacing, table precedence/zero fallback, wet mixed coefficient and warning-dependent delay order. A changed executable/profile hash alone is not a behavioral witness. Preserve executed mutant source and executable identities.

Measure coverage separately for all 22 primary routines and their source branches, including selected dependency reachability. Compare instrumented observations to the ordinary profile; expose any compiler/arithmetic differences and seek a bounded projection decision rather than normalizing them away. Classify every unhit path as inactive, native-invalid, host-dependent or still-defined/unverified. Line reachability does not prove full scientific input coverage.

Deliver `validation/terrestrial_resources/REPORT.md`, complete source/branch/caller/state maps, reproducible commands, actual execution identities, raw/local comparison evidence, controls/mutations/coverage, remaining limits and explicit-path Conventional Commits. Keep all generated JSON/JSONL/compressed state, profiles, raw archives and large outputs local and ignored under 0039. Track sources, generators, authored inputs and concise reports only; never force-add outputs or substitute another large generated format.

Protect all accepted reference/evidence files and both independent repositories. T's active `rust/` and `validation/pulsar_lifecycle_port/`, plus central documents, are concurrent work outside U's immutable membership. Read accepted Git revisions when fixed Rust context is useful; do not freeze T's evolving files as immutable oracle inputs.

No Rust edits, new numerical representation/tolerance, source bug fixes or milestone acceptance are delegated. IFTE/TT-to-TDB, general observatory coordinate transforms, ephemeris/astrometry, geometric/Shapiro/BAT assembly, other propagation models, arbitrary I/O faults, other platforms and concurrency remain subsequent work. Report a concrete out-of-contract finding to central while continuing independent cases.

## Dispatch

Created **Assignment U: TEMPO2 terrestrial resource reference**, chat `01a16bd2-a746-7871-9dfb-52a76ae73c64`, in the local `rustify-tempo2` project from release commit `43001af`. An immediate snapshot confirms an active reference turn. T received U's separate ownership boundary and remains sole Rust owner. Both chats retain policy 0039 and central design/acceptance authority. This records dispatch, not accepted evidence.
