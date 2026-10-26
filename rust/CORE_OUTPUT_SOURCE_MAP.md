# AH source, state, type and branch correspondence

Pin `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; release `732fd0e`.
This is implementation correspondence, not a claim of exhaustive dynamic branch
coverage. The generated, ignored `validation/core_output/build/source-inventory.json`
lists all six `textOutput.C` bodies and all 375 decision/loop/case sites with
original line numbers and predicates. `source_inventory.py` reproduces that
inventory without rewriting Rust. Unsupported AST operations fail translation;
Cargo compiles the committed Rust and never invokes the development translator.

## Routine and state map

| Pinned source | Rust / responsibility | Dynamic evidence and remaining branch scope |
| --- | --- | --- |
| `tempo2.C:61-625` active main | `core_cli::run`, executable dispatcher in `main.rs`; explicit environment, source early flags and aliases, one input/preprocess pass, actual fit/output loop | Standalone TIM; repeated `-f`; normal/pre/NITS; writetim; robust/global/noise; both predictor forms; plugin selectors and failure statuses. Allocation is owned/lazy rather than native fixed storage. Full native debug-allocation transcripts are not qualified. |
| `getInputs.C::getInputs` | `core_cli::get_inputs`; all active source options; final positional TIM rule; source case rules; parse unknown tokens for existing preprocessing | Pairing/missing operands, outpar, whitespace/metacharacter directory pairing, byte paths, alias cycles/long valid expansion. Native source's lone-argument TIM grammar is retained; unsafe short paths fail. |
| `setPlugPath`, `printplugs` | Public `core_cli` functions; override path order, empty path elements, runtime `/plugins/`, first-underscore name and duplicate marker rules | Unit path ordering and real help/plugin controls. Filesystem iteration order is retained. Discovery does not implement the plugin ABI. Alternate compiled `TEMPO2_CONFIGURE_PLUG` profiles are not claimed. |
| `textOutput.C:60-1849`, `textOutput` | `core_output::text_output` -> full `core_output_source`; mutable canonical `PulsarContext`; source output/file/mutation order | Identical-state snapshot, real workflows and file comparisons below. Every active AST branch transfers; uncommon derived/family branches without a dynamic fixture remain source-inspected, not numerically accepted. |
| `textOutput.C:1851-1928`, `calcRMS` | `core_output::calc_rms`; current pulsar observations, START/FINISH, source SAT/BAT selectors, deleted rows, turn weights and microsecond RMS | Distinct two-pulsar cutoffs and deleted rows; returns sum of weights; writes rmsPre/rmsPost/rmstn and TRES/TRESTN. Source zero/NaN behavior remains; all-deleted full reports have no invented time span. |
| `textOutput.C:1945`, `m2` | Public `core_output::m2`; original 10,000-step Newton iteration and -1 failure sentinel | Three dyadic helper cases include zero inclination. No physical-domain coercion. Exhaustion and every nonfinite branch are implemented but not all independently exercised. |
| `textOutput.C:1967`, `printGlitch`; `dglep_eval_phase`; `dglep` | `print_glitch`, `dglep`; phase normalization and current bracketed/damped solve, exponential decay and finite checks | Positive/zero/negative phase helpers, actual GLITCH report and PAR, invalid public index errors. Nonfinite/bisection/exhaustion alternatives remain in source order; not all are dynamically witnessed. |
| `displayParameters.C::displayParameters` | Public `core_output::display_parameters` (private implementation module) | All positions/headers and row branches retained. Actual position 13 is exercised for modern and legacy terrestrial sites; includes clock, ephemeris, site and delay rows. First-file labels explicitly remain `[0]` for all pulsars. Barycentric UT1 absence is an explicit boundary. |
| `storePrecision.C::recordPrecision` | Public `record_precision` no-op | No body exists at the pin. No monitor, precision store or invented output. |
| `TKlog.C/.h` | Existing `Diagnostics`, publicly reexported; log/check/check_log/timing controls | First 30 entries, counters beyond 30, warning-before-error summary/reset, 359-byte truncation before ANSI removal, real stdout PTY and redirected stdout/stderr. CPU ticks preserve source f32 division. Whole-engine debug transcript parity is unqualified. |
| `TKlongdouble.C`, selected header | Public typed `source_format::{format,ld_printf,ld_fprintf,ld_sprintf,ld_vsprintf,print_longdouble,parse_longdouble}` | 70 extended format/value rows plus two mixed integer/string/binary64 rows; signed zero, infinities, NaN, e/f/g, sign/width/precision/padding and zero-success return. Reuses source scanner, never strict `Scalar` for source parsing. |
| `extract_enums.sh`, `tempo2.h`, enum strings | `LABEL_STR` from canonical `PARAMETER_NAMES`; exact 68 constraints and 3 series, each with empty sentinel | Unit checks include marker entries and canonical enum order; no new enum namespace. |
| `help.txt`, `help_extra.txt`, `convertxt.c`, `Makefile.am` | Pinned byte copies under `rust/data`, embedded with `include_str!`; safe strings replace generated C array terminators | Copies hash-match pin. Converter's only runtime responsibility is embedded text/NUL; no redundant installed converter. Truthful Rust identity and unsupported-plugin message are intentional presentation changes. |

## Report branches and files

The ranges below partition the active report. The machine-readable inventory
lists individual predicates, including each family-specific alternative.

| Source region | Implemented branch families | Representative executed evidence |
| --- | --- | --- |
| 60-178 | residual dump; RMS; closest nondeleted midpoint TOA / first tie; TZRMJD/TZRFRQ/TZRSITE mutation; fit/robust/global counts | Snapshot, robust, unweighted, global, NITS and pre variants |
| 179-340 | parameter/index ordering, parameter-set filtering, START/FINISH S/B/N flags, equatorial/ecliptic and SINI/KIN linkage, errors and coordinate post strings, series choices, jumps and FDJUMPs | Snapshot, jump/FD files, parameter prior, T2 and ELL1H reports; ecliptic predictor route. Linked coordinate/error alternatives are source-inspected where no direct output fixture is listed. |
| 341-711 | WAVE/WAVDM and scale variants; GW/quadrupole/interpolation; DM/CM weights; clock/IFUNC/telescope output; shapelets/noise and plus/cross files | Fitted WAVE and WAVE_SCALE=1, DM grid, clock offsets, IFUNC, quadrupole, GW memory; unfitted WAVE errors remain unwritten boundaries. Telescope XYZ, fitted shapelet, scale=2 and all GW alternatives are not individually exercised by AH. |
| 712-1074 | spin period/derivatives, age, field and energy; binary mass functions/solutions, DDH, inclination and parallax, proper-motion corrections, ELL1 conversions | Snapshot binary DD/spin derived fields, helper masses, ELL1H and T2. Remaining astrophysical derived subbranches retain source formulas/types but have no new independent numerical acceptance. |
| 1075-1160 | glitch helper/report, Jupiter-mass special ephemerides, GW geometry, time span, usage selectors and unit warnings | Glitches, snapshot, GW memory, TCB/TDB/tempo1 controls. Jupiter-mass alternatives are source-inspected, unexecuted here. |
| 1161-1240 | global and quadrupole covariance indexing/normalization/labels | Two-pulsar global, identical matrix snapshot, quadrupole interpolation |
| 1241-1849 | PAR filename/prompt and suffixes; every parameter/uncertainty/unit/binary/time/noise/jump/wave/grid/constraint/selector record | Snapshot reload and prompt, real fitted PAR reload, two-pulsar filenames, noise/DM/jump/wave/interpolation cases. Source DMOFF first-knot break and literal WAVDM_DM spelling remain; no silent repair of uncommon reload semantics. |

Files remain source-named and created at the source point. Fixed names can be
overwritten by later pulsars. `residuals.dat` contains every active observation
row (including deleted rows): BAT-PEPOCH in days, residual and TOA uncertainty in
seconds, each converted to binary64 at the pinned cast. `<PSR>.dm` uses MJD,
DM coefficient/error in source DM units and dimensionless weights; `<PSR>.cm`
uses MJD, CM delay/error in seconds and weights. `clockOffset.dat`, `ifunc.dat`,
`telescopeXYZ.dat`, `aplus_t2.dat`, `across_t2.dat` retain their literal headers,
units and row formats. Global/quadrupole covariance files retain source row
identities and normalized matrix entries. New PAR defaults to `new.par`; multiple
pulsars append `_1`, `_2`, etc.; an explicitly empty name reads one supplied token.
Optional fit diagnostic files are produced by AE, predictor files by AF.

## Types, ownership and departures

* Timing parameters, epochs and source longdouble expressions remain fixed
  Rug128. Native reference timing uses binary128 with 113 significant bits;
  that difference is provenance, never an allowance. Ordinary predictor long
  double retains AF's target profile (53 bits on this macOS arm64 host).
* Binary64 casts, RMS accumulation, helper iteration variables, ordinary printf
  values and covariance entries remain f64; source float variables and CPU-time
  division remain f32 before promotion. Source constants/grouping and explicit
  integer conversions are retained. No new tolerance, clipping or rounding mode.
* Canonical parameter/observation/matrix/constraint owners are reused. No ABI
  layout casts, alternative engine, internal JSON subprocess, or report-specific
  scientific state object. Missing source producers remain `None` and return
  errors instead of guessed data. A plain public glitch index is checked before
  parallel array reads; canonical object invariants are otherwise trusted.
* Owned argument/file/stream buffers remove C fixed-buffer overflow and alias
  recursion hazards. `-allParTim` enumerates real suffix-matched pairs, preserving
  path bytes; this corrects source shell splitting and first-substring rewriting.
  Tests retain original bounded failures and valid neighbors.
* File open behavior follows source branches: some failures print a message and
  continue; attempted writes to absent handles or failed writers return errors.
  No transaction/rollback promise: completed files and mutations survive. The
  repeated-context test verifies recovery after a real stdout write failure.
* OutputIo.directory controls report destinations and top-level input pairs.
  Existing preprocessing/covariance include resolvers retain their established
  path contracts; callers needing their relative paths should use the process
  working directory or absolute arguments. The library never calls chdir/setenv.
* Native help/version/no-args statuses remain 1; explicit Rust `--help` retains
  its old JSON-adapter help contract. Plugin selection is explicit unavailable,
  not substitution. `-pre` fits once per outer NITS, skips the second evaluation;
  writetim precedes fitting. `-model`, old/newfit and dcm preserve inactive source
  consumers. Default native acceleration selector is 1; JSON defaults stay 0.

## Demonstrated corrections and boundaries

| Cause / pinned site | Narrow correction / evidence |
| --- | --- |
| calcRMS 1898-1899 uses pulsar zero cutoffs with current flags | Current-pulsar ownership; original second-pulsar weight zero vs corrected equal weight; first pulsar/other helpers unchanged |
| Unweighted CHI2R reads unassigned local at 1441 | Initialize from fitChisq for each pulsar; defined poison witness -12345 vs actual 2, plus real unweighted workflow |
| START/FINISH, default EPHVER, `-last`, TZR/TRES activation leaves prefit metadata unreadable | Snapshot defined previous value/error on first activation; preserve supplied PAR state. Original poison survives; corrected metadata is prior zero. No new fitted/scientific value is invented. |
| Nonglobal globalNfit lacks producer, quadrupole/covariance consumers read it | Reset count at prefit, global branch overwrites; guard global covariance with nGlobal; native poison and real quadrupole files |
| Existing Rust weights ran for empty constraint matrices | Match source nonempty-matrix condition; unconstrained DM/CM output weights now match native exactly |
| Modern get_obsCoord omits correctionUT1 despite using its components | Retain applied site-to-UTC seconds plus existing EOP UT1-UTC; no new lookup/model/fallback. 48 before/after Rust rows and native geometry/BAT/residual fields unchanged; all new values match independent native composition for clock modes 0/1/2. Barycentric/satellite paths remain separate. |
| TKlongdouble unmatched/double va_end; empty conversion skips terminator | Typed Rust arguments/owned buffers; narrow native cleanup and empty-conversion scan fix. Raw undefined original formatter bytes are not goldens. |
| Missing/empty/cyclic aliases, fixed buffers, shell names | Bounded errors/dynamic tokens and suffix enumeration. Native original status/prefix retained under resource/time limits. |

The translator's early covariance ArrayRef misclassification and omitted source
log formatting were AH implementation defects corrected before delivery, not
upstream bugs. Historical failed outputs are retained. Unfitted WAVE error arrays,
barycentric `-list` UT1, all-deleted report span and absent DDH derived inputs
remain source-unwritten boundaries. Scientific/numerical failures are separately
listed in CORE_OUTPUT_REPORT.md; no acceptance is implied by this map.

## Unsafe and external boundaries

New unsafe calls are limited to `clock_diagnostics::cpu_clock` and
`clock_text::format_double`. `clock()` has no arguments/pointers and uses the
host clock_t typedef. `format_double` is private and called only after the typed
parser accepts one floating conversion without stars; two snprintf calls first
size then fill an owned NUL-terminated buffer with exactly one f64 argument.
No pointer escapes. Existing scanner libc and SOFA FFI remain unchanged. All
new CLI/output/formatter implementation modules otherwise contain no unsafe.
No new numerical/library dependency. POSIX-only paths, clock ABI, target-specific
predictor precision and SOFA retain the existing platform qualification boundary.
