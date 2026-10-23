# Assignment L source correspondence (implementation plan)

All counterparts refer to fda6ad4a9753ad7dc5d291406f2f310a9161423f.
This maps all 32 measured J entries before implementation; the report records
executed verification. Selected dependency coverage is not whole-file coverage.

| Source entries | Rust counterpart / responsibility |
| --- | --- |
| tabulatedfunction.C:40,116,168,173 | `tabulated_function.rs`: load, get_value, start, end; source chunks, samples and endpoint/bracket order |
| clkcorr.C:88,101,110 constructors | `ClockCorrectionFunction`, `ClockSequence`, `PathBuildResult`: explicit source defaults; absent unloaded table fields |
| :117,127,135,140; normalize lambda | trim, normalize_clock_name, clock_name_equals, basename_from_path; source byte rules and C-locale ASCII case |
| :148,168,211,240 | parse_clock_header_line, add_indexed_clock_file, glob_clock_files, ensure_initialized; typed fixed-format libc conversions and sorted index |
| :296,307,313,331,340 | ensure_function_loaded, function_valid_at_mjd, sequence_mjd_range, sequence_matches, find_function_index_from_token |
| :358 and getNode lambda | build_directed_path and insertion-order node registration; binary32 distances, directed adjacency, retry and reverse hints |
| :530,548 | cache_sequence, get_sequence; first match and append-only range cache |
| :606,646 | fill_corrections, accumulate_correction; shifted binary64 queries, capacity asymmetry and retained writes |
| :681,728,810,819 | define_sequence, get_clock_corrections, get_correction_tt, get_correction; independent fallback ladders |
| toa2utc.C:60 | `toa2utc.rs`: p/observation/k/l order, bypass and SAT insertion including extra slot |
| observatory.C:72,152,199,256,295 | `observatory.rs`: read definitions/aliases, independent lazy initialization, alias accumulation/warning and actual lookup |
| observatory.C:419,449; fang | reuse current ITRF_to_GRS80 expression; source geodetic diagnostic conversion, with operation-local integer bounds |
| tempo2Util.C:362; TKlog.C/h | `clock_diagnostics.rs`: shared displayMsg keys/DUP1, source formatting/channels/counters/storage; library sink adapter |
| global.C:176 CVSdisplayVersion | selected diagnostic dedup/version presentation only |
| libc sscanf fixed source formats | `clock_text.rs`: private typed conversion only, initialized buffers and bounded source string widths; no C clock algorithm |
| global.C formBatsAll; runtime.rs | outer synchronized process session, stored TT chain and distinct UTC accumulator, cache-faithful resource provenance |
| tropo.C selected table consumers | same table implementation; existing weather formula/fallback remains scoped |

No reset switch, alternate strict parser, new cycle detection, finite-weight
guard or compensated sum is introduced. Literal-disabled legacy clock bodies
stay inactive. BAT input, general PAR/TIM, full geometry/weather/EOP/IFTE state,
other models and full toolkit/logging remain outside this release.
