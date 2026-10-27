# AJ source, public API and boundary map

Pin `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; reviewed AI baseline
`20e368f`; released by 0095. This is a public responsibility map, not exhaustive
branch coverage or numerical acceptance. The older coverage inventory describes
`ddc07b2`; its "missing" labels are not the current implementation status.

## All 93 library translation units

Every root library unit in `validation/coverage/SOURCE_INVENTORY.md` occurs below.
Private implementation modules are reached through the named public canonical
route; exporting another scientific implementation or C pulsar layout is not
necessary. Routine/branch details and outstanding scientific comparisons remain
in the indicated existing maps and API documents.

| Pinned units | Public Rust route / responsibility |
| --- | --- |
| `BTJmodel.C`, `BTXmodel.C`, `BTmodel.C`, `DDGRmodel.C`, `DDHmodel.C`, `DDKmodel.C`, `DDSmodel.C`, `DDmodel.C`, `ELL1Hmodel.C`, `ELL1kmodel.C`, `ELL1model.C`, `MSSmodel.C`, `T2-PTAmodel.C`, `T2model.C` | `binary::{model,fit_function,update}` dispatches every model family from canonical `SourcePulsar`; AD binary/timing API and source modules retain individual helpers. |
| `GWsim.C` | `gw_sim`, `gw_file`; complete AG generation, polarization, response, eccentric evolution and record APIs; ANALYSIS_TOOLS_SOURCE_MAP. |
| `T2accel.C` | `t2accel`; five AI provider helpers and scoped `with_acceleration`; NATIVE_BACKENDS_API. |
| `T2toolkit.C`, `t2toolkit_global.C` | `toolkit` statistics, associated sorting, MT/Gaussian state, capacity; ANALYSIS_TOOLS_SOURCE_MAP. |
| `TKfit.C`, `TKsvd.C`, `TKmatrix.C`, `TKrobust.C` | `tkfit`, `tksvd`, `tk_matrix`, `tkrobust`; GENERAL_FITTING_API/SOURCE_MAP and later AI provider semantics. |
| `TKlog.C`, `TKlongdouble.C` | reexported `Diagnostics`, typed `source_format`, `Scalar`; CORE_OUTPUT_SOURCE_MAP. Source timing arithmetic remains Rug128. |
| `TKspectrum.C`, `t2spectrum.C` | `spectrum`, `sigma_z`; all AG estimators, transforms, smoothing, interpolation and error/warning policy under 0090. |
| `bootstrap.C` | `bootstrap` owned RNG and resampling lifecycle. |
| `calculate_bclt.C`, `dm_delays.C`, `shapiro_delay.C`, `formBats.C`, `secularMotion.C` | direct `propagation::{calculate_bclt,dm_delays,shapiro_delay,form_bats,secular_motion}`; integrated `global`/`timing`; AB/AD semantics retained. |
| `cheby2d.c`, `t1polyco.c`, `tempo2pred.c` | `cheby2d`, `t1polyco`, `tempo2pred`; standalone native C headers/shim use these same AF arithmetic leaves; full symbol list in LIBRARY_INTERFACES_API. |
| `cheby2d_int.C`, `polyco.C` | `cheby2d_int`, `polyco`, `predictor` timing-dependent construction/tests/workflow; PREDICTOR_SOURCE_MAP. |
| `cholesky.C`, `choleskyAutomatic.C`, `choleskyRoutines.C` | `cholesky`, `cholesky_automatic`, `cholesky_routines`, `cholesky_spectrum`; AE full public APIs. |
| `constraints.C`, `constraints_covar.C`, `constraints_nestlike.C`, `constraints_param.C` | `constraints` functions and owned `ConstraintSpecial`; GENERAL_FITTING_SOURCE_MAP enumerates callbacks. |
| `doFit.C`, `t2fit.C` | `general_fitting`, `legacy_fitting`, `fit_callbacks`; preparation, standalone derivative/updates, local/global solves. |
| `t2fit_dmmodel.C`, `t2fit_dmother.C`, `t2fit_fitwaves.C`, `t2fit_glitch.C`, `t2fit_gw.C`, `t2fit_ifunc.C`, `t2fit_position.C`, `t2fit_solarwind.C`, `t2fit_stdFitFuncs.C` | `fit_callbacks::{derivative,update}` and named standalone functions; private family modules are implementations, not missing public responsibilities. |
| `t2fit_nestlike.C` | public `t2fit_nestlike`, `general_fitting::postfit`; source basis/noise operations. |
| `displayParameters.C`, `textOutput.C`, `storePrecision.C` | `core_output` display/report/helper APIs; `propagation::record_precision` source no-op. |
| `enum_str.C` | `tempo2::LABEL_STR`, canonical parameter/constraint/series names and sentinels; generated enum namespace has no separate mutable owner. |
| `getInputs.C`, `preProcess.C`, `preProcessSimple.C` | `preprocess`, `selection`, `bats::{load_pairs,preprocess}`; `core_cli` source CLI dispatch. |
| `readParfile.C`, `readTimfile.C` | `source_par`, `source_tim`, `source_text`; canonical context/bytes/partial source state. |
| `formResiduals.C`, `refphs.C`, `global.C` | `timing`, `refphs`, `global::{clock_corrections,ephemeris_routines,extra_delays,form_bats_all,update_bats_all}`. |
| `readEphemeris.C`, `readEphemeris_calceph.C` | `read_ephemeris`, `calceph`; AI owned descriptor and source first-pulsar/current-pulsar conventions. |
| `readJBO_bat.C` | `read_jbo_bat` reader, swaps, date and character helpers. |
| `ifunc.C`, `shapelet.C`, `sw_delay.C` | `ifunc`, `shapelet`, `solar_wind`; existing direct canonical operations. |
| `sortToAs.C`, `units.C`, `updateEpoch.C` | `sort_toas`, `units`, `update_epoch`; source permutation/unit/epoch semantics. |
| `clkcorr.C`, `tabulatedfunction.C`, `observatory.C`, `eop.C`, `tai2ut1.C`, `get_obsCoord.C`, `ifteph.C`, `jpleph.c`, `toa2utc.C`, `tropo.C`, `tt2tdb.C`, `vectorPulsar.C`, `initialise.C`, `tempo2Util.C` | AJ makes their existing owners and direct operations public; exact routes below. |
| `tai2tt.C`, `utc2tai.C` | Entire legacy bodies are `#if 0` at the pin, explicitly replaced by `clkcorr.C`; active generic clock chains supply these responsibilities. No duplicate dead routine. |

The standalone executable `getPeriod.C` maps to installed `getPeriod-rust`.
`tempo2.C` maps to existing `core_cli` plus the existing Rust executable; AJ
installs the source and predictor client, not a new core binary distribution.
`read_fortran.h` / `read_fortran2.h` map to the public reader/stream owners below.
SOFA retains exactly the settled 36 native routines under 0083; direct native
SOFA exports are not a promise of an additional 121-routine toolkit. Its sources
and notices are installed. Plugins remain outside the selected C ABI guarantee.

## Newly accessible direct owners and functions

| Public route | Source operation / lifetime |
| --- | --- |
| `clkcorr::ClockSession::{run,add_indexed_clock_file,ensure_initialized,ensure_function_loaded,build_directed_path,cache_sequence,get_sequence,define_sequence,fill_corrections,accumulate_correction,get_clock_corrections,get_correction}` | Source process/session clock cache, graph and correction operations. `ClockCorrectionFunction`, `ClockSequence`, `PathBuildResult`, `TableQuery` are their returned/owned records. `get_correction_tt` sums the already applied corrections. Resource exporters and `steps` retain provenance. |
| `clkcorr::{with_process,with_resources}` | Borrow the synchronized canonical owner once; closures call borrowed-session operations. Do not recursively acquire these functions or an integrated driver that acquires them. Independent explicit `ClockSession::default().run` owns its own state. |
| `tabulated_function::TabulatedFunction::{load,start,end,get_value}` | File-owned table, source interpolation/extrapolation and terminal failed-state behavior. The cfg(test) byte fixture constructor stays private to tests. |
| `observatory::{Observatory,ObservatoryDatabase,AliasList}` and methods; `fang`, `itrf_to_grs80`, `grs80_to_itrf` | Source catalog/alias initialization and lookup, source coordinate conversions. Strings keep byte semantics. |
| `eop::{Eop,load_eop}`; `Eop::{initialize,get_eop}` | First-loaded EOP state, source interpolation and resource identity. |
| `tai2ut1::{Ut1,tai2ut1}`; `Ut1::red` | Source UT1 parser/cache and multi-pulsar application. |
| `get_obs_coord::{SiteGeometry,iau2000b,remove_white,lmst,ang,get_precession_matrix,driver,zenith_trs}` | Source terrestrial/SOFA geometry and direct helpers; no guessed UT1 or rewritten time frame. |
| `ifteph::IfteEphemeris::{init,close_file,get_values,get_delta_t_delta_t_dot,delta_t,delta_t_dot,get_ve_ve_dot,get_ve,get_ve_dot}`; `IFTswap*`, source constants | Owned IFTE descriptor/buffers; source two-part dates and output shapes. Static interpolation cache stays internal: a zeroed cache is not a public initialized interpolation session. |
| `jpleph::{Ephemeris,Interpolation,Boundary,jpl_init_error_code,swap_32_bit_val,swap_64_bit_val}` | `Ephemeris::{init,state,pleph,get_double,get_long,get_constant,pvsun}`, Drop; `Interpolation::{new,default,interp}` preserves source initialized polynomial-cache state. |
| `read_fortran::FortranReader::{open_file,close_file,read_char,read_character,read_int,read_float,read_double,read_record_int,require_complete}` | Owned source buffered Fortran reader; existing `read_fortran2` process wrappers remain public. |
| `stdio::Stream::{from_file,getc,status,seek,read}` | Adopts an owned `File` once, borrows exclusively, closes in Drop; raw FILE never escapes. Existing bounds check protects fread's real destination extent. |
| `toa2utc::{ClockPulsarView,toa2utc}` | Borrow full or single-observation source state without replacing the source clock convention. |
| `tropo::{MeteorologyFunction,Meteorology,MeteorologicalValue,initialize_meteorology_table,nmf_hydrostatic,nmf_wet,TropospherePulsar,compute_observation,compute_tropospheric_delays}` | Owned resource tables, pressure/wet delays, source Niell functions, borrowed per-observation/multi-pulsar operations. |
| `tt2tdb::{FbCache,init_ifte,if_delta_t,fb_delta_t,compute_observation,tt2tb}` | Existing IF/FB source time ephemerides and session ownership. CALCEPH remains AI's separate selected provider route. |
| `vector_pulsar::{position,rotate,vector_pulsar}` | Source unit vector/rotation and canonical pulsar mutation. |
| `initialise::{allocate_memory,initialise_parameters,label_parameters}` | Canonical parameter slots/labels; `PulsarContext::initialise` remains full owner initialization. |
| `tempo2_util::{dot_product,hms_turn,dms_turn,turn_deg,fortran_mod,fortran_nlong}` | Existing source conversions/rounding expressions; existing `copy_param`, `get_parameter_value` remain public. 0049 unresolved boundaries are unchanged. |

Public does not mean arbitrary cache mutation is valid. Existing source state
and `None`/initialized-field rules remain intact. No extra guessed defaults were
added to scientific objects to make external examples pass.

## C representation, unsafe and correction inventory

`tempo2pred.h` is byte-identical to the pinned public header. The standalone
internal header retains native structs, long double, integer widths and callback
signatures, fixes actual declaration/definition mismatches, and declares all
standalone definitions. Core-dependent construction functions are Rust APIs.
The verifier checks all public ABI symbols and native type size/alignment/offsets.

`native/predictor.c` is the retained C allocation/scanning/formatting/dispatch
layer. It marshals pointers to `predictor_ffi.rs`. No pointer is interpreted as
a Rust `Float`, struct or Vec. Values cross through exact `%La` and 25-digit
native decimal text. Conversion uses a private C locale; the complete Rust token
parser bypasses the locale-sensitive public source-prefix scanner. On macOS
snprintf_l/strtold_l leave both caller locales intact. Linux's formatting scope
uses thread-local uselocale and restores it; execution remains AL work.

The Rust bridge's unsafe sites have local necessity, lifetime and extent comments:
foreign scalar access (C performs typed reads/writes), NUL text access, paired
native malloc/free, synchronous callback invocation, native phase dispatch and
writing advertised integer/array outputs. Private exported functions catch
Rust unwinding; errors produce diagnostics/status or the initialized NaN scalar.
C pointer validity/alignment/extents and callback initialization remain caller
preconditions. C++ exceptions/longjmp must not cross Rust frames. No allocator
exhaustion or full Rust sanitizer guarantee is claimed.

Canonical leaf extraction shares phase-only/frequency-only T1 evaluation,
Cheby model scalar evaluation, nearest selection, span bounds, sample plans and
testFunc with owned AF operations. Declared counts 0/1 still read the initialized
fixed cells actually required by each source operation; neither the unused cells
nor owned file-constructor minima are imposed on the C evaluation/read APIs.

| Pinned defect or interface issue | Scoped disposition and executable neighbor |
| --- | --- |
| Generic FRead tries the wrong active union; GetPlan_Ext has uninitialized owner | Initialized temporary owner and source trial order; previous owner survives failure; initialized plan owner is destroyed after use. consumer.c failure/recovery and plan cases. |
| Nonseekable streams need no initial successful ftell; failed fseek must not short-circuit trials | Attempt source reads/rewinds, retain libc cursor. boundary.c checks Cheby pipe+TAIL, direct T1 pipe, generic T1 pipe after its discarded trial header, and exact failed-trial cursor. A bare T1 marker is consumed by the source Cheby trial; its leftover short lines invoke undefined source fixed-field conversions. That accidental native success is not a defined numeric golden. |
| Native scalar text inherits comma decimal separator | Private explicit C-locale conversion in both directions and complete-token parsing; no source file-format change. Two independent fractional input/output cases under C/German/French with locale preservation/recovery. |
| Raw C reader low counts were initially overrestricted by the adapter | Final reader accepts 0..32, preserves complete prefix and unwritten omitted backing. low_count.c initializes required cells only after metadata inspection. getPeriod separately rejects absent C1. |
| Cheby writer passes ordinary long double to binary128 varargs | Type-correct native fprintf, original precision/order; same correction as AF oracle. Native patch retained. |
| T1 non-multiple-of-three coefficient newline; expanded Doppler; blank isolated binary fields | Existing AF corrections retained, native generated/parsed file exchange. |
| Unbounded scans and array counts, negative/overflow dimensions, incomplete Cheby rows | Bounded string/capacity scans and explicit errors; source successful partial assignments/cursor remain. Malformed input/recovery controls; no speculative valid-input normalization. |
| Cheby failed set read leaves uninitialized future owners; realloc loses original pointer | Initialize only owner pointers/counts before reading; checked allocation and partial-owner cleanup. Preserve already parsed fields. |
| Cheby self copy/insert destroys or reallocates its source | Self-copy retains owner; self-insert snapshots source before append. Zero-extent native shape remains unchanged. |
| Empty T1/metadata dereference, short/blank fixed-field uninitialized outputs | Explicit NULL/NaN/error or documented empty string/AF binary metadata; invalid source bytes are not goldens. |
| T1 Copy/Insert/Keep unsupported exits and file-open write exit | Preserved exactly, tested in separate processes. |

No changes to backend arithmetic, numerical tolerances, solver metadata,
statistical conventions, warning policy, resource calibration or milestone gates.
