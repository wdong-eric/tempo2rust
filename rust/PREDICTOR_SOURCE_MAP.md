# AF source and departure map

Raw source pin: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
`validation/predictors/native.py` records raw and corrected hashes, object/archive
identities, build commands and the exact correction/instrumentation diff.
Canonical TEMPO2, A2 and accepted timing/fitting evidence remain read-only.

| Source body/declaration | Rust route / disposition |
| --- | --- |
| `cheby2d.c::parse_ld_cheby` | `PredictorNumber::parse_prefix`, source numeric prefix grammar with native rounding. |
| `Cheby2D_Init/Destroy/Copy` | `Cheby2D::new/from_coefficients`, Clone/Drop. No uninitialized coefficient storage or alias destruction. |
| `Cheby2D_Construct` | `Cheby2D::construct`: source cosine nodes, y-major layout, accumulation order. |
| `Cheby2D_Construct_x_Derivative/Evaluate/Test` | `x_derivative/evaluate/test`: source recurrences and midpoint test grid. |
| compiled `testFunc/testCheby2D` | `predictor_compiled_source_testfunc`: source sine callback and 30×30/90×90 construction/test dimensions. Disabled main is not a production API. |
| `ChebyModel_Init/Copy/Destroy` | Owned model/polynomial initialization, Clone/Drop. |
| `ChebyModel_GetPhase/GetFrequency/Write/Read` | `ChebyModel::phase/frequency/write/read`; ranges, dispersion and continuation rows retained. |
| `ChebyModelSet_GetNearestIndex/GetNearest/GetPhase/GetFrequency` | `nearest_index` and generic `evaluate`; inclusive time coverage, first tie, source -1 no-match plus owned status. |
| `ChebyModelSet_Init/Insert/Keep/Destroy/Read/Write` | Default/Clone/Drop and corresponding set methods; ordered append/retention, no sorting. |
| `cheby2d_int.C::chebyModelFunc` | `cheby_model_func`: source reference row and sample indices, BAT/residual sequencing, native assignment boundaries and retained observations. |
| `ChebyModel_Construct/Test`, `ChebyModelSet_Construct/Test` | `construct_model/test_model/construct_set/test_set`; explicit mutable canonical context and output path. |
| `polyco.C::polyco/atimfake/tzFit/chebpc/pcshft` | Corresponding snake-case functions; sidereal/day/half-hour/truncation rules, 31 samples, trueDM, two formats and literal prefix. |
| `t1polyco.c::T1P_grabString/LongDouble/Int`, `Read/Read_NewFormat` | Fixed-field/prefix parsing and `T1Polyco::read/read_new`; D exponents and source field meanings. |
| `T1Polyco_GetPhase/GetFrequency/Write` | `phase/frequency/write`; observing-frequency dispersion only in phase. |
| `T1PolycoSet_GetNearest/GetPhase/GetFrequency/Init/Destroy/Read/Write` | Owned set methods, generic dispatch and Clone/Drop; nearest-midpoint extrapolation and successful-prefix reading. |
| `tempo2pred.c::Read/FRead/Write/FWrite/Init/Destroy` | Generic owned I/O, Default/Drop; stream positions and errors are explicit. |
| `T2Predictor_Copy/Insert/Keep` | `copy_from/insert/keep`; T1 source exits become recoverable unimplemented errors. |
| `GetPSRName/GetSiteName/GetStartMJD/GetEndMJD/GetStartFreq/GetEndFreq/Kind` | `metadata` and enum; first/last ordered segments, no extrema recomputation. |
| `GetPhase/GetFrequency/GetPlan/GetPlan_Ext` | `evaluate/plan/get_plan/get_plan_ext`; source interval averages, not instantaneous-frequency samples. |
| `tempo2.C:359–503`, predictor `getInputs.C` branches | `predictor::load_par/cli_defaults` and `predictor_cli`: PAR-only preprocessing, source defaults/even adjustment, outputs and no-fit return. Full legacy CLI belongs to its later owner. |
| `tempo2pred.h`, `tempo2pred_int.h`, `Makefile.am` | Rust public types/modules and runnable consumers. Internal header's T1 nearest signature omits the set argument; the native caller declares the actual definition. Exported C library/header/install/consumer work remains AJ. |
| `TKlongdouble.float128.h`, `TKlongdouble.C` | Existing Rug128 timing helpers; native predictor wrapper, quad cosine before native assignment, native sqrt and explicit double conversions. No second timing backend. |

## Demonstrated corrections and defined neighbors

Adjacent comments identify the original cause in production code. These are
0053 corrections, not scientific tolerance changes.

| Original defect / evidence | Correction / regression |
| --- | --- |
| `tempo2pred.c:350`: GetPlan_Ext reads into uninitialized model storage; generic trial reader assumes Cheby union backing | Initialized native plan witness; fresh Rust enum parsing. Failed read leaves the prior Rust object intact. Stream failure/recovery and native lifecycle histories; no invalid native storage executed as a golden value. |
| `cheby2d.c:430–443`: ordinary native long double passed to binary128 variadic `ld_fprintf` | Typed Rust formatting; corrected native uses `fprintf`. Internal coefficient traces are independent of the original invalid writer. Same 34/25 significant digits and ordering. |
| `cheby2d.c:517`, derivative `nx-2` access | Positive matching dimensions, derivative nx≥2, complete rows and continuation grammar. Tests cover missing/negative dimensions, nx=1, extra/incomplete rows and valid neighbors. Constant nx=1 polynomial storage/evaluation remains valid. |
| `t1polyco.c` coefficient arrays / blank binary fields | 2–32 stored coefficients; blank source isolated-binary metadata uses the new format's explicit zero pair. Original bytes preserve whether it was absent. |
| `t1polyco.c:111`: missing final newline for non-multiple-of-three count | Finish the coefficient line; two consecutive four-coefficient records survive both set reading and trailing malformed input. |
| `polyco.C:419` minimum-width Doppler vs `t1polyco.c:193–194` fixed offset | Parse the writer's signed decimal and three fractional digits before log10rms. Barycentric `-9999.990` and valid-width neighbors retain both fields. Original reader diagnostic corruption is preserved separately. The source barycentric frequency convention and Doppler value are unchanged. |
| `cheby2d_int.C:73–119`, `polyco.C` synthesized rows bypass preprocessing | Initialize only the missing clock scratch/rough observer vectors and the empty synthetic jump/uncertainty fields. `preProcessSimple.C:155–162` supplies the vector invariant. Repeated defined vectors survive. Initial terrestrial failure was `observation_unwritten_earth_ssb[3]`; corrected native initialization is isolated in AF staging. |
| `polyco.C::tzFit` reads tdis1/tdis2 after barycentric propagation skipped dm_delays | Supply the zero-delay state explicitly defined by `dm_delays.C:52–56` for delayCorr=0. Changed-site/trueDM histories cover stale-state recovery. |
| `polyco.C:262`, one-based phase[800], c[31] never assigned | At most 25 sets (31×25+1 rows), fit order 2–31; error before writing arrays. Invalid requests followed by valid recovery. Stored external T1 order 32 is distinct. |
| `polyco.C` date capacity and unchecked fopen/prefix arrays | Owned strings/paths and contextual I/O errors; source date convention through existing MJD calendar helper. Native date buffer is 100 bytes for its strftime bound. No atomic output claim. |
| AF's initial native wrapper double rounding (central 32a) | Carry MPFR rounding direction through exponent/subnormal handling for conversion, arithmetic and parsing. Both exact bit-1 witnesses and normal/minimum/negative-zero neighbors; Linux execution remains unqualified. |
| AF comparator omitted shapes and plan membership (central 32a) | Native-derived constructed and reread shapes, every coefficient/metadata field, every plan endpoint/index/status, actual vector length and integrated JSON mutation controls. No gate widened. |

## Numeric and unsafe boundaries

Native arithmetic rounds after each source operation, with explicit contraction
only at measured GCC16 arm64 sites in Cheby construction/Clenshaw/derivative/test,
nearest-center calculation, polyco fractional reconstruction and T1 phase
accumulation/Doppler/final subtraction. Disassemblies are retained in AF build
staging. The source timing typedef remains binary128113 in the oracle and Rug128
in production. Native 64-bit Linux values do not pass through f64 except at
source-explicit double boundaries. Linux compiler contraction and ABI are not
qualified by this macOS run.

No new unsafe Rust, FFI calls or dependencies were added. Existing canonical
SOFA/libc/resource behavior is reused. The small observation helper extraction
moves the existing missing-clock initialization from TIM reading into a shared
method so synthetic observations can use the same established initialization.
