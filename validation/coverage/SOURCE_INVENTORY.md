# Pinned source inventory

All `S:path:line` references mean raw canonical Git blobs at `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; all `R:path:line` mean primary baseline `ddc07b20fafc21fe92f2ecc1dd24c59db9df4142`. Use `git -C tempo2 show COMMIT:path | nl -ba` or `git show COMMIT:path | nl -ba`. Live Rust files may already differ because E is active. Generated from the checked JSON/curated TSV; no milestone acceptance is implied.

Status describes baseline implementation, not acceptance. Every row remains open for final disposition. The JSON also includes full expanded lists, hashes, generated-source inputs, direct include directives, parameter and string-dispatch occurrences, conditional branches and lexical function candidates. These indexes are navigation aids, not an AST or exhaustive branch-validation claim.

| Root source | Library membership | Baseline | Evidence / counterpart | Responsibility and remaining transfer |
| --- | --- | --- | --- | --- |
| `BTJmodel.C` | libtempo2_la_SOURCES | missing | `S:BTJmodel.C:40`, `S:BTJmodel.C:167`; — | BTJ orbital jumps: delay, derivatives and updateBTJ; all branches absent. Batch `binary-keplerian`. |
| `BTXmodel.C` | libtempo2_la_SOURCES | missing | `S:BTXmodel.C:37`, `S:BTXmodel.C:176`; — | BTX orbital-frequency expansion, delay, derivatives and updates; all branches absent. Batch `binary-keplerian`. |
| `BTmodel.C` | libtempo2_la_SOURCES | missing | `S:BTmodel.C:37`, `S:BTmodel.C:144`; — | BT delay, derivatives and updateBT; all branches absent. Batch `binary-keplerian`. |
| `DDGRmodel.C` | libtempo2_la_SOURCES | missing | `S:DDGRmodel.C:48`, `S:DDGRmodel.C:298`; — | DDGR, mass2dd and updates; mass-dependent relativistic constraints absent. Batch `binary-relativistic`. |
| `DDHmodel.C` | libtempo2_la_SOURCES | missing | `S:DDHmodel.C:42`, `S:DDHmodel.C:252`; — | DDH orthometric Shapiro delay, derivatives and updates; historical ideal-DDH failure remains unexplained. Batch `binary-relativistic`. |
| `DDKmodel.C` | libtempo2_la_SOURCES | missing | `S:DDKmodel.C:42`, `S:DDKmodel.C:310`; — | DDK Kopeikin terms, delay, derivatives and updates absent. Batch `binary-relativistic`. |
| `DDSmodel.C` | libtempo2_la_SOURCES | missing | `S:DDSmodel.C:39`, `S:DDSmodel.C:201`; — | DDS Shapiro-shape model, derivatives and updates absent. Batch `binary-relativistic`. |
| `DDmodel.C` | libtempo2_la_SOURCES | missing | `S:DDmodel.C:38`, `S:DDmodel.C:203`; — | DD delay, derivatives, updates and source SINI clamping/warnings absent; preserve these semantics when porting. Batch `binary-relativistic`. |
| `ELL1Hmodel.C` | libtempo2_la_SOURCES | missing | `S:ELL1Hmodel.C:34`, `S:ELL1Hmodel.C:306`; — | ELL1H and calcDH harmonic Shapiro terms and updates absent. Batch `binary-relativistic`. |
| `ELL1kmodel.C` | libtempo2_la_SOURCES | missing | `S:ELL1kmodel.C:59`, `S:ELL1kmodel.C:189`; — | ELL1k low-eccentricity evolution and updates absent. Batch `binary-keplerian`. |
| `ELL1model.C` | libtempo2_la_SOURCES | missing | `S:ELL1model.C:61`, `S:ELL1model.C:276`; — | ELL1 low-eccentricity delay, derivative and update paths absent. Batch `binary-keplerian`. |
| `GWsim.C` | libtempo2_la_SOURCES | missing | `S:GWsim.C:62`, `S:GWsim.C:159`; — | Library GW source/background generation, anisotropy, general polarizations, eccentric-source evolution, residuals and file I/O absent; library membership keeps this in inventory despite plugin callers. Batch `gw-spectrum`. |
| `MSSmodel.C` | libtempo2_la_SOURCES | missing | `S:MSSmodel.C:49`, `S:MSSmodel.C:275`; — | MSS spinning companion delay, derivatives and updates absent. Batch `binary-relativistic`. |
| `T2-PTAmodel.C` | libtempo2_la_SOURCES | missing | `S:T2-PTAmodel.C:40`, `S:T2-PTAmodel.C:283`; — | T2_PTAmodel delay/computeU/update path absent. Batch `binary-relativistic`. |
| `T2accel.C` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES | missing | `S:T2accel.C:48`, `S:T2accel.C:66`; — | Optional accelerated Cholesky, upper inverse, QR least squares and BLAS matrix paths absent; reference disables acceleration, which does not remove these source branches from scope. Batch `fit-noise`. |
| `T2model.C` | libtempo2_la_SOURCES | missing | `S:T2model.C:85`, `S:T2model.C:598`; — | Unified multi-companion T2, Keplerian jumps, GR, Kopeikin and Shapiro branches and updates absent. Batch `binary-relativistic`. |
| `T2toolkit.C` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES | missing | `S:T2toolkit.C:45`, `S:T2toolkit.C:52`; — | Statistics, sorting, random generators and TKsign_d absent as source toolkit APIs; generic Rust math does not establish this library transfer. Batch `toolkit`. |
| `TKfit.C` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES | missing | `S:TKfit.C:43`, `S:TKfit.C:65`; — | Least squares, constrained/robust wrappers, polynomial removal and covariance/error calculation absent at baseline; E targets only selected nonaccelerated SVD solve. Batch `fit-basic`. |
| `TKlog.C` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES | missing | `S:TKlog.C:18`, `S:TKlog.C:50`; — | Library logging levels, timing/debug checks and legacy diagnostics absent; Rust contextual errors are a different adapter. Batch `output-integration`. |
| `TKlongdouble.C` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES | partial | `S:TKlongdouble.C:40`, `S:TKlongdouble.C:58`; `R:rust/src/longdouble.rs:1`, `R:rust/src/tempo2_util.rs:6` | Decimal input and output supplied by finite Rug128 Scalar; legacy printf/scanf formatting, power overloads, longdouble ABI and native precision alternatives are not ported. Batch `numeric-toolkit`. |
| `TKmatrix.C` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES | missing | `S:TKmatrix.C:12`, `S:TKmatrix.C:26`; — | Matrix multiplication, forward substitution and source allocation/layout interfaces absent. Batch `fit-noise`. |
| `TKrobust.C` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES | missing | `S:TKrobust.C:21`, `S:TKrobust.C:148`; — | Huber, bisquare and Welsch reweighting/iteration absent. Batch `fit-noise`. |
| `TKspectrum.C` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES | missing | `S:TKspectrum.C:85`, `S:TKspectrum.C:428`; — | Spectrum estimators, FFT/DFT/Lomb, smoothing/splines, sigmaz and spectral errors absent. Batch `gw-spectrum`. |
| `TKsvd.C` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES | missing | `S:TKsvd.C:40`, `S:TKsvd.C:172`; — | Source extended SVD, bidiagonalization, back substitution, cutoff and convergence behavior absent at baseline; E is concurrent and unreviewed here. Batch `fit-basic`. |
| `bootstrap.C` | libtempo2_la_SOURCES | missing | `S:bootstrap.C:48`, `S:bootstrap.C:266`; — | Bootstrap resampling, RNG and parameter uncertainty lifecycle absent. Batch `fit-noise`. |
| `calculate_bclt.C` | libtempo2_la_SOURCES | partial | `S:calculate_bclt.C:41`; `R:rust/src/form_bats.rs:83`, `R:rust/src/form_bats.rs:179` | Static equatorial direction, Roemer, iterative six-body Shapiro/DM calculation with source Sun+Jupiter convergence sum; no PM/parallax/radial motion/acceleration, alternate ephemeris perturbations or general multi-pulsar branch coverage. Batch `propagation`. |
| `cheby2d.c` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | missing | `S:cheby2d.c:77`, `S:cheby2d.c:84`; — | Cheby2D and ChebyModel/Set evaluation, frequency derivative, selection, ownership and serialization absent; compile-time test main is not a production predictor entry. Batch `prediction`. |
| `cheby2d_int.C` | libtempo2_la_SOURCES | missing | `S:cheby2d_int.C:50`, `S:cheby2d_int.C:150`; — | Construct/test predictors by sampling timing engine over time/frequency and fitting Chebyshev coefficients absent. Batch `prediction`. |
| `cholesky.C` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES | missing | `S:cholesky.C:44`, `S:cholesky.C:179`; — | Diagonal weights, covariance-model parsing, kernels, Cholesky whitening and inverse absent; E targets only independent uncertainties. Batch `fit-noise`. |
| `choleskyAutomatic.C` | libtempo2_la_SOURCES | missing | `S:choleskyAutomatic.C:18`; — | Automatic covariance-function estimation absent; core library even when called by plugins. Batch `fit-noise`. |
| `choleskyRoutines.C` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES | missing | `S:choleskyRoutines.C:18`, `S:choleskyRoutines.C:209`; — | Covariance/spectrum estimation, smoothing and Cholesky decomposition absent; despite xtra_dist comment this file is explicitly in both core library source lists. Batch `fit-noise`. |
| `clkcorr.C` | libtempo2_la_SOURCES | partial | `S:clkcorr.C:89`, `S:clkcorr.C:117`; `R:rust/src/clkcorr.rs:1` | Directed route search, badness, coverage/ties, sequence caching and sequential interpolation for PKS UTC/TT(TAI); explicit user-defined chains, full clock alternatives/fallback ladder and global repeat-state semantics remain; indispensable-chain failure differs. Batch `resources`. |
| `constraints.C` | libtempo2_la_SOURCES | missing | `S:constraints.C:10`, `S:constraints.C:123`; — | Constraint naming/automatic selection, weights, DM/CM/IFUNC/quad/telescope functions and dispatch absent. Batch `fit-noise`. |
| `constraints_covar.C` | libtempo2_la_SOURCES | missing | `S:constraints_covar.C:7`, `S:constraints_covar.C:22`; — | IFUNC and DM-model covariance constraints absent. Batch `fit-noise`. |
| `constraints_nestlike.C` | libtempo2_la_SOURCES | missing | `S:constraints_nestlike.C:7`, `S:constraints_nestlike.C:23`; — | Red/DM/chromatic/SWGP/jitter/band/group priors and constraints absent; compiled core despite nestlike naming. Batch `fit-noise`. |
| `constraints_param.C` | libtempo2_la_SOURCES | missing | `S:constraints_param.C:13`; — | Generic parameter constraint callback absent. Batch `fit-noise`. |
| `displayParameters.C` | libtempo2_la_SOURCES | missing | `S:displayParameters.C:44`; — | Full supported parameter/help listing absent. Batch `output-integration`. |
| `dm_delays.C` | libtempo2_la_SOURCES | partial | `S:dm_delays.C:41`; `R:rust/src/form_bats.rs:212` | Constant DM, frequency dilation and default spherical IPM=1 NE_SW=4 SWM=0 expression only; DM derivatives, CM/chromatic/FD/FDDC, DMX, DMMODEL, NE_SW interpolation/sinusoid and SWM=1 paths absent. Spherical default lives here, not in sw_delay.C. Batch `dispersion`. |
| `doFit.C` | libtempo2_la_SOURCES | missing | `S:doFit.C:41`, `S:doFit.C:45`; — | Legacy doFitAll/getParamDeriv wrappers and plugin fit dispatch absent; plugin ABI is separate from core wrapper disposition. Batch `fit-extended`. |
| `enum_str.C` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | missing | `S:Makefile.am:185`; — | Generated enum names from extract_enums.sh and tempo2.h; no matching complete parameter/constraint enum or string registry. Batch `output-integration`. |
| `eop.C` | libtempo2_la_SOURCES | partial | `S:eop.C:55`, `S:eop.C:138`; `R:rust/src/eop.rs:1` | Leap-aware IAU2000 EOP parsing/interpolation and lower proxy; source global first-load state replaced, upper endpoint rejected, alternate file formats/whole API not verified. Batch `resources`. |
| `formBats.C` | libtempo2_la_SOURCES | partial | `S:formBats.C:36`; `R:rust/src/form_bats.rs:261`, `R:rust/src/form_residuals.rs:38` | TCB BAT sum for PKS and BAT=SAT for @; general flags/clock/time/model branches and full observation mutation API absent. Batch `propagation`. |
| `formResiduals.C` | libtempo2_la_SOURCES | partial | `S:formResiduals.C:46`, `S:formResiduals.C:225`; `R:rust/src/form_residuals.rs:29` | Isolated F0-F3 split phase, first-input fractional reference, signed pulse rounding and uncentered residuals; F4+, braking, binary, glitches, waves, IFUNC, GW, noise, tracking, pulse flags, jumps, selections, TZR and averaging/mean-removal branches absent at baseline. Batch `residual-models`. |
| `getInputs.C` | libtempo2_la_SOURCES | partial | `S:getInputs.C:52`, `S:getInputs.C:249`; `R:rust/src/main.rs:1` | Minimal residuals --par/--tim/--runtime CLI only; upstream input options, resource configuration, multi-pulsar lists, switches and plugin path discovery absent. Batch `input-lifecycle`. |
| `get_obsCoord.C` | libtempo2_la_SOURCES | partial | `S:get_obsCoord.C:55`, `S:get_obsCoord.C:128`; `R:rust/src/get_obs_coord.rs:1` | IAU2000B with native SOFA and fixed PKS Cartesian geometry; TEMPO1/precession, special site/barycentric/spacecraft coordinates, telescope interpolation/offsets and other configuration paths absent. Batch `astrometry-time`. |
| `global.C` | libtempo2_la_SOURCES | partial | `S:global.C:65`, `S:global.C:75`; `R:rust/src/form_bats.rs:43` | PKS rough/refined time-ephemeris-site orchestration plus residual evaluation; multi-pulsar, refphs lifecycle, updateBatsAll optimization, CALCEPH and process-global controls absent; repeat-call initialization differs. Batch `input-lifecycle`. |
| `ifteph.C` | libtempo2_la_SOURCES | partial | `S:ifteph.C:70`, `S:ifteph.C:78`; `R:rust/src/ifteph.rs:1` | IF99 DE405 two-component time/velocity image and Chebyshev derivatives in safe Rust; fixed dimensions/file constraints, close/reinit/global state and malformed/range outcomes differ; full external IFTE API not exposed. Batch `resources`. |
| `ifunc.C` | libtempo2_la_SOURCES | missing | `S:ifunc.C:5`, `S:ifunc.C:23`; — | Piecewise linear/sinc scalar and strided interpolation utilities absent. Batch `residual-models`. |
| `initialise.C` | libtempo2_la_SOURCES | partial | `S:initialise.C:40`, `S:initialise.C:56`; `R:rust/src/read_parfile.rs:115`, `R:rust/src/lib.rs:77`, `R:rust/src/runtime.rs:201` | Owned minimal fields/defaults replace pulsar allocation/initialization; full source parameter defaults, global allocations and reinit/destruction semantics absent; A2 initializer is a separate corrected reference. Batch `input-lifecycle`. |
| `jpleph.c` | libtempo2_la_SOURCES | retained_native_modified | `S:jpleph.c:75`, `S:jpleph.c:80`; `R:rust/native/jpleph.c:135`, `R:rust/src/read_ephemeris.rs:11` | Native low-level reader/state/interpolation retained with header-zeroing and cache corrections; Rust exposes DE405 selected targets, not whole native API. Native retention is not a Rust rewrite or full-core acceptance. Batch `resources`. |
| `observatory.C` | libtempo2_la_SOURCES | partial | `S:observatory.C:68`, `S:observatory.C:150`; `R:rust/src/runtime.rs:90` | PKS Cartesian table lookup and ITRF_to_GRS80 only; aliases, geodetic/legacy table conversion, arbitrary observatories and global caching absent or changed. Batch `resources`. |
| `polyco.C` | libtempo2_la_SOURCES | missing | `S:polyco.C:90`, `S:polyco.C:165`; — | TEMPO1 polynomial predictor construction, artificial TOAs, tzFit/chebpc/pcshft absent. Batch `prediction`. |
| `preProcess.C` | libtempo2_la_SOURCES | missing | `S:preProcess.C:41`, `S:preProcess.C:561`; — | Command-driven model edits, epoch changes, selection, global settings, noise models and preprocessing orchestration absent. Batch `input-lifecycle`. |
| `preProcessSimple.C` | libtempo2_la_SOURCES | partial | `S:preProcessSimple.C:33`, `S:preProcessSimple.C:44`; `R:rust/src/read_parfile.rs:189` | Only the adopted omitted-UNITS/EPHVER=5 resolution is represented; three-pass wave defaults, unit transformations, phase offsets, selections, jumps, DM modifications and flag processing absent. Batch `input-lifecycle`. |
| `readEphemeris.C` | libtempo2_la_SOURCES | partial | `S:readEphemeris.C:56`, `S:readEphemeris.C:498`; `R:rust/src/read_ephemeris.rs:47` | DE405 source-order Sun/planet/Earth/EMB/nutation reads and SI/TCB conversion; alternate DE files/units, ecliptic rotation, perturbations/mass and planet-series branches absent; ignored native errors now stop evaluation. Batch `astrometry-time`. |
| `readEphemeris_calceph.C` | libtempo2_la_SOURCES | missing | `S:readEphemeris_calceph.C:66`, `S:readEphemeris_calceph.C:211`; — | CALCEPH ephemeris/time conversion branch and no-CALCEPH failure stubs absent; conditional source remains listed even when disabled. Batch `astrometry-time`. |
| `readJBO_bat.C` | libtempo2_la_SOURCES | missing | `S:readJBO_bat.C:42`, `S:readJBO_bat.C:44`; — | Legacy JBO clock/binary record parsing and endian/date helpers absent. Batch `resources`. |
| `readParfile.C` | libtempo2_la_SOURCES | partial | `S:readParfile.C:47`, `S:readParfile.C:69`; `R:rust/src/read_parfile.rs:1` | 13 numeric keys and five text keys; F0-F3/epochs/DM/RAJ/DECJ and inactive TZR metadata, fit flags recorded only; all remaining parameter branches, indexed families, defaults/checkAllSet/global parsing and duplicate assignment semantics need transfer. Batch `input-lifecycle`. |
| `readTimfile.C` | libtempo2_la_SOURCES | partial | `S:readTimfile.C:56`, `S:readTimfile.C:119`; `R:rust/src/read_timfile.rs:1`, `R:rust/src/input.rs:23` | Single FORMAT 1, exactly five tokens, homogeneous @ or PKS, raw input identity; no other formats/sites/aliases, flags, INCLUDE, selection, clock/time/phase/error/weight directives, pulse numbering or writeTim. Batch `input-lifecycle`. |
| `refphs.C` | libtempo2_la_SOURCES | missing | `S:refphs.C:6`, `S:refphs.C:47`; — | Synthetic TZR reference observation insertion/removal absent; retaining TZRMJD/TZRFRQ/TZRSITE tokens is not activating REFPHS_TZR. Batch `residual-models`. |
| `secularMotion.C` | libtempo2_la_SOURCES | partial | `S:secularMotion.C:36`; `R:rust/src/form_residuals.rs:42` | Only zero-delay isolated BBAT=BAT identity; DSHK/Shklovskii secular correction absent. Batch `astrometry-time`. |
| `shapelet.C` | libtempo2_la_SOURCES | missing | `S:shapelet.C:8`, `S:shapelet.C:21`; — | Shapelet/Hermite recurrence and transient basis evaluation absent. Batch `residual-models`. |
| `shapiro_delay.C` | libtempo2_la_SOURCES | partial | `S:shapiro_delay.C:38`; `R:rust/src/form_bats.rs:194` | Sun and five planetary logarithms embedded in PKS calculation; switches, ecliptic/position evolution and general public routine branches absent. Batch `propagation`. |
| `sortToAs.C` | libtempo2_la_SOURCES | missing | `S:sortToAs.C:12`, `S:sortToAs.C:23`; — | Source stable/deleted observation ordering behavior absent; current API intentionally retains input order. Batch `input-lifecycle`. |
| `storePrecision.C` | libtempo2_la_SOURCES | missing | `S:storePrecision.C:39`; — | Legacy recordPrecision scientific/display precision diagnostics absent; JSON strings do not implement that API. Batch `output-integration`. |
| `sw_delay.C` | libtempo2_la_SOURCES | missing | `S:sw_delay.C:62`, `S:sw_delay.C:182`; — | SWM=1 solar current-sheet/rotation model and supporting astronomy/file handling absent; current default spherical formula is in dm_delays.C. Batch `dispersion`. |
| `t1polyco.c` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | missing | `S:t1polyco.c:35`, `S:t1polyco.c:62`; — | T1Polyco/Set read/write, nearest-segment selection and phase/frequency evaluation absent. Batch `prediction`. |
| `t2fit.C` | libtempo2_la_SOURCES | missing | `S:t2fit.C:60`, `S:t2fit.C:542`; — | Modern fit orchestration, column enumeration, selection, global/local assembly, updates, pre/postfit and legacy wrappers absent at baseline; E covers only ZERO/F0-F3 independent-noise profile. Batch `fit-basic`. |
| `t2fit_dmmodel.C` | libtempo2_la_SOURCES | missing | `S:t2fit_dmmodel.C:9`, `S:t2fit_dmmodel.C:14`; — | DMMODEL DM and common-mode derivative/update callbacks absent. Batch `fit-extended`. |
| `t2fit_dmother.C` | libtempo2_la_SOURCES | missing | `S:t2fit_dmother.C:12`, `S:t2fit_dmother.C:24`; — | DMX, chromatic exponent, FD/FDDC and DM sinusoid callbacks absent. Batch `fit-extended`. |
| `t2fit_fitwaves.C` | libtempo2_la_SOURCES | missing | `S:t2fit_fitwaves.C:5`, `S:t2fit_fitwaves.C:32`; — | FITWAVES basis and update callbacks absent. Batch `fit-extended`. |
| `t2fit_glitch.C` | libtempo2_la_SOURCES | missing | `S:t2fit_glitch.C:11`, `S:t2fit_glitch.C:100`; — | Glitch, exponential dip and Gaussian dip derivative/update callbacks absent. Batch `fit-extended`. |
| `t2fit_gw.C` | libtempo2_la_SOURCES | missing | `S:t2fit_gw.C:5`, `S:t2fit_gw.C:165`; — | GW memory/burst/cosmic-string/single source/quadrupole derivative/update callbacks absent. Batch `fit-extended`. |
| `t2fit_ifunc.C` | libtempo2_la_SOURCES | missing | `S:t2fit_ifunc.C:8`, `S:t2fit_ifunc.C:29`; — | IFUNC/SIFUNC derivatives and updates absent. Batch `fit-extended`. |
| `t2fit_nestlike.C` | libtempo2_la_SOURCES | missing | `S:t2fit_nestlike.C:23`, `S:t2fit_nestlike.C:37`; — | Red/DM/chromatic/SWGP/band/group/jitter/shapelet noise basis, polynomial subtraction and updates absent. Batch `fit-noise`. |
| `t2fit_position.C` | libtempo2_la_SOURCES | missing | `S:t2fit_position.C:6`, `S:t2fit_position.C:110`; — | Astrometry position/proper-motion/parallax derivative/update callbacks absent. Batch `fit-extended`. |
| `t2fit_solarwind.C` | libtempo2_la_SOURCES | missing | `S:t2fit_solarwind.C:6`, `S:t2fit_solarwind.C:11`; — | NE_SW, sinusoid, interpolation derivative/update and constraint callbacks absent. Batch `fit-extended`. |
| `t2fit_stdFitFuncs.C` | libtempo2_la_SOURCES | missing | `S:t2fit_stdFitFuncs.C:14`, `S:t2fit_stdFitFuncs.C:17`; — | ZERO/spin/binary/planet/DM/CM/telescope/jump derivatives and physical updates absent at baseline; E supplies only ZERO/F0-F3 subset. Batch `fit-basic`. |
| `t2spectrum.C` | libtempo2_la_SOURCES | missing | `S:t2spectrum.C:12`, `S:t2spectrum.C:22`; — | Timing spectral estimators using fits/IFUNC and cosine/sine callbacks absent. Batch `gw-spectrum`. |
| `t2toolkit_global.C` | libt2toolkit_la_SOURCES | missing | `S:t2toolkit_global.C:1`; — | Toolkit global flags and spectral controls for standalone linkage absent. Batch `toolkit`. |
| `tabulatedfunction.C` | libtempo2_la_SOURCES | partial | `S:tabulatedfunction.C:40`, `S:tabulatedfunction.C:116`; `R:rust/src/tabulated_function.rs:1` | Linear table interpolation, inclusive range and endpoint proxies retained; malformed/decreasing rows and degenerate intervals now errors, names/header storage and API differ. Batch `resources`. |
| `tai2tt.C` | libtempo2_la_SOURCES | missing | `S:tai2tt.C:53`; — | Legacy dedicated TAI-to-TT table path absent; current modern clkcorr chain can include that conversion but does not transfer this routine. Batch `resources`. |
| `tai2ut1.C` | libtempo2_la_SOURCES | missing | `S:tai2ut1.C:51`, `S:tai2ut1.C:120`; — | Legacy UT1 table/ut1red path absent; modern IAU2000B obtains UT1 through EOP. Source T2C_TEMPO-only branch needs explicit disposition. Batch `resources`. |
| `tempo2Util.C` | libtempo2_la_SOURCES | partial | `S:tempo2Util.C:40`, `S:tempo2Util.C:47`; `R:rust/src/tempo2_util.rs:1` | Binary64 RA/DEC conversion, dotproduct and signed mod/nlong subset; formatters, vector operations, ecliptic transforms, copy helpers, warnings and linked parameter lookup absent; numeric range guards differ. Batch `numeric-toolkit`. |
| `tempo2pred.c` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | missing | `S:tempo2pred.c:39`, `S:tempo2pred.c:51`; — | Public T2Predictor object, T1/Chebyshev dispatch, read/write/copy/insert/keep, metadata, phase/frequency and plan APIs absent. Batch `prediction`. |
| `textOutput.C` | libtempo2_la_SOURCES | missing | `S:textOutput.C:60`, `S:textOutput.C:1851`; — | Full fit/results/PAR output, RMS and derived astrophysical/glitch values absent; minimal Rust JSON is an adapter with different output contract. Batch `output-integration`. |
| `toa2utc.C` | libtempo2_la_SOURCES | partial | `S:toa2utc.C:60`, `S:toa2utc.C:104`; `R:rust/src/clkcorr.rs:215`, `R:rust/src/form_bats.rs:99` | Modern getClockCorrections path represented for PKS; legacy NIST/JBO conversion, special site and clock override branches absent. Batch `resources`. |
| `tropo.C` | libtempo2_la_SOURCES | partial | `S:tropo.C:49`, `S:tropo.C:160`; `R:rust/src/tropo.rs:1` | Niell hydrostatic/wet and source missing/zero/out-of-range weather defaults retained for PKS; general sites/switches/global weather state absent or changed. Batch `propagation`. |
| `tt2tdb.C` | libtempo2_la_SOURCES | partial | `S:tt2tdb.C:65`, `S:tt2tdb.C:197`; `R:rust/src/tt2tdb.rs:1` | IF99 SI/TCB correction with velocity/time derivatives; retains second obsTerm scaling expression. FB90, TDB/TEMPO1, alternate unit/ephemeris choices and general switches absent. Batch `astrometry-time`. |
| `units.C` | libtempo2_la_SOURCES | missing | `S:units.C:40`, `S:units.C:48`; — | TCB/TDB parameter, epoch, error and dimension-dependent conversion absent; current parser accepts TCB only. Batch `astrometry-time`. |
| `updateEpoch.C` | libtempo2_la_SOURCES | missing | `S:updateEpoch.C:4`, `S:updateEpoch.C:68`; — | Epoch selection/re-expansion of spin, DM, position and binary models absent. Batch `input-lifecycle`. |
| `utc2tai.C` | libtempo2_la_SOURCES | missing | `S:utc2tai.C:59`; — | Legacy UTC-to-TAI clock routine absent; modern directed chains are separately partial in clkcorr.C. Batch `resources`. |
| `vectorPulsar.C` | libtempo2_la_SOURCES | partial | `S:vectorPulsar.C:57`; `R:rust/src/form_bats.rs:83` | Static RAJ/DECJ direction and normalization only; proper motion, ecliptic conventions, distance/parallax and related vector state absent. Batch `astrometry-time`. |

## Bundled SOFA: complete list

36 retained routines are the checked transitive closure of C2T00B, POM00, RXP, SXP, PXP, TRXPV and TRXP. All 121 routines stay visible; the remaining 85 need future native/core dependency disposition, not necessarily a Rust reimplementation. Retained files are byte-identical raw blobs.

| Source | Baseline | Direct native dependencies |
| --- | --- | --- |
| `sofa/a2af.for` | native_not_retained | sofa/d2tf.for |
| `sofa/a2tf.for` | native_not_retained | sofa/d2tf.for |
| `sofa/anp.for` | retained_native | none |
| `sofa/anpm.for` | retained_native | none |
| `sofa/bi00.for` | retained_native | none |
| `sofa/bp00.for` | retained_native | sofa/bi00.for, sofa/ir.for, sofa/pr00.for, sofa/rx.for, sofa/rxr.for, sofa/ry.for, sofa/rz.for |
| `sofa/bpn2xy.for` | retained_native | none |
| `sofa/c2i00a.for` | native_not_retained | sofa/c2ibpn.for, sofa/pnm00a.for |
| `sofa/c2i00b.for` | retained_native | sofa/c2ibpn.for, sofa/pnm00b.for |
| `sofa/c2ibpn.for` | retained_native | sofa/bpn2xy.for, sofa/c2ixy.for |
| `sofa/c2ixy.for` | retained_native | sofa/c2ixys.for, sofa/s00.for |
| `sofa/c2ixys.for` | retained_native | sofa/ir.for, sofa/ry.for, sofa/rz.for |
| `sofa/c2s.for` | native_not_retained | none |
| `sofa/c2t00a.for` | native_not_retained | sofa/c2i00a.for, sofa/c2tceo.for, sofa/era00.for, sofa/pom00.for, sofa/sp00.for |
| `sofa/c2t00b.for` | retained_native | sofa/c2i00b.for, sofa/c2tceo.for, sofa/era00.for, sofa/pom00.for |
| `sofa/c2tceo.for` | retained_native | sofa/cr.for, sofa/rxr.for, sofa/rz.for |
| `sofa/c2teqx.for` | native_not_retained | sofa/cr.for, sofa/rxr.for, sofa/rz.for |
| `sofa/c2tpe.for` | native_not_retained | sofa/c2teqx.for, sofa/ee00.for, sofa/gmst00.for, sofa/pn00.for, sofa/pom00.for, sofa/sp00.for |
| `sofa/c2txy.for` | native_not_retained | sofa/c2ixy.for, sofa/c2tceo.for, sofa/era00.for, sofa/pom00.for, sofa/sp00.for |
| `sofa/cal2jd.for` | native_not_retained | none |
| `sofa/cp.for` | retained_native | none |
| `sofa/cpv.for` | native_not_retained | sofa/cp.for |
| `sofa/cr.for` | retained_native | sofa/cp.for |
| `sofa/d2tf.for` | native_not_retained | none |
| `sofa/dat.for` | native_not_retained | sofa/cal2jd.for |
| `sofa/dtdb.for` | native_not_retained | none |
| `sofa/ee00.for` | native_not_retained | sofa/eect00.for |
| `sofa/ee00a.for` | native_not_retained | sofa/ee00.for, sofa/nut00a.for, sofa/obl80.for, sofa/pr00.for |
| `sofa/ee00b.for` | native_not_retained | sofa/ee00.for, sofa/nut00b.for, sofa/obl80.for, sofa/pr00.for |
| `sofa/eect00.for` | native_not_retained | sofa/anpm.for |
| `sofa/epb.for` | native_not_retained | none |
| `sofa/epb2jd.for` | native_not_retained | none |
| `sofa/epj.for` | native_not_retained | none |
| `sofa/epj2jd.for` | native_not_retained | none |
| `sofa/epv00.for` | native_not_retained | none |
| `sofa/eqeq94.for` | native_not_retained | sofa/anpm.for, sofa/nut80.for, sofa/obl80.for |
| `sofa/era00.for` | retained_native | sofa/anp.for |
| `sofa/fk52h.for` | native_not_retained | sofa/fk5hip.for, sofa/ppp.for, sofa/pvstar.for, sofa/pxp.for, sofa/rxp.for, sofa/starpv.for |
| `sofa/fk5hip.for` | native_not_retained | sofa/rv2m.for |
| `sofa/fk5hz.for` | native_not_retained | sofa/anp.for, sofa/c2s.for, sofa/fk5hip.for, sofa/rv2m.for, sofa/rxp.for, sofa/s2c.for, sofa/sxp.for, sofa/trxp.for |
| `sofa/gmst00.for` | native_not_retained | sofa/anp.for, sofa/era00.for |
| `sofa/gmst82.for` | native_not_retained | sofa/anp.for |
| `sofa/gst00a.for` | native_not_retained | sofa/anp.for, sofa/ee00a.for, sofa/gmst00.for |
| `sofa/gst00b.for` | native_not_retained | sofa/anp.for, sofa/ee00b.for, sofa/gmst00.for |
| `sofa/gst94.for` | native_not_retained | sofa/anp.for, sofa/eqeq94.for, sofa/gmst82.for |
| `sofa/h2fk5.for` | native_not_retained | sofa/fk5hip.for, sofa/pmp.for, sofa/pvstar.for, sofa/pxp.for, sofa/rxp.for, sofa/starpv.for, sofa/trxp.for |
| `sofa/hfk5z.for` | native_not_retained | sofa/anp.for, sofa/fk5hip.for, sofa/pv2s.for, sofa/pxp.for, sofa/rv2m.for, sofa/rxp.for, sofa/rxr.for, sofa/s2c.for, sofa/sxp.for, sofa/trxp.for |
| `sofa/ir.for` | retained_native | sofa/zr.for |
| `sofa/jd2cal.for` | native_not_retained | none |
| `sofa/jdcalf.for` | native_not_retained | sofa/jd2cal.for |
| `sofa/num00a.for` | native_not_retained | sofa/pn00a.for |
| `sofa/num00b.for` | native_not_retained | sofa/pn00b.for |
| `sofa/numat.for` | retained_native | sofa/ir.for, sofa/rx.for, sofa/rz.for |
| `sofa/nut00a.for` | native_not_retained | none |
| `sofa/nut00b.for` | retained_native | none |
| `sofa/nut80.for` | native_not_retained | sofa/anpm.for |
| `sofa/nutm80.for` | native_not_retained | sofa/numat.for, sofa/nut80.for, sofa/obl80.for |
| `sofa/obl80.for` | retained_native | none |
| `sofa/p2pv.for` | native_not_retained | sofa/cp.for, sofa/zp.for |
| `sofa/p2s.for` | native_not_retained | sofa/c2s.for, sofa/pm.for |
| `sofa/pap.for` | native_not_retained | sofa/pdp.for, sofa/pm.for, sofa/pmp.for, sofa/pn.for, sofa/pxp.for |
| `sofa/pas.for` | native_not_retained | none |
| `sofa/pdp.for` | native_not_retained | none |
| `sofa/plan94.for` | native_not_retained | sofa/anpm.for |
| `sofa/pm.for` | native_not_retained | none |
| `sofa/pmat00.for` | native_not_retained | sofa/bp00.for |
| `sofa/pmat76.for` | native_not_retained | sofa/cr.for, sofa/ir.for, sofa/prec76.for, sofa/ry.for, sofa/rz.for |
| `sofa/pmp.for` | native_not_retained | none |
| `sofa/pn.for` | native_not_retained | sofa/pm.for, sofa/sxp.for, sofa/zp.for |
| `sofa/pn00.for` | retained_native | sofa/bp00.for, sofa/numat.for, sofa/obl80.for, sofa/pr00.for, sofa/rxr.for |
| `sofa/pn00a.for` | native_not_retained | sofa/nut00a.for, sofa/pn00.for |
| `sofa/pn00b.for` | retained_native | sofa/nut00b.for, sofa/pn00.for |
| `sofa/pnm00a.for` | native_not_retained | sofa/pn00a.for |
| `sofa/pnm00b.for` | retained_native | sofa/pn00b.for |
| `sofa/pnm80.for` | native_not_retained | sofa/nutm80.for, sofa/pmat76.for, sofa/rxr.for |
| `sofa/pom00.for` | retained_native | sofa/ir.for, sofa/rx.for, sofa/ry.for, sofa/rz.for |
| `sofa/ppp.for` | native_not_retained | none |
| `sofa/ppsp.for` | native_not_retained | none |
| `sofa/pr00.for` | retained_native | none |
| `sofa/prec76.for` | native_not_retained | none |
| `sofa/pv2p.for` | native_not_retained | sofa/cp.for |
| `sofa/pv2s.for` | native_not_retained | none |
| `sofa/pvdpv.for` | native_not_retained | sofa/pdp.for |
| `sofa/pvm.for` | native_not_retained | sofa/pm.for |
| `sofa/pvmpv.for` | native_not_retained | sofa/pmp.for |
| `sofa/pvppv.for` | native_not_retained | sofa/ppp.for |
| `sofa/pvstar.for` | native_not_retained | sofa/anp.for, sofa/pdp.for, sofa/pm.for, sofa/pmp.for, sofa/pn.for, sofa/ppp.for, sofa/pv2s.for, sofa/sxp.for |
| `sofa/pvu.for` | native_not_retained | sofa/cp.for, sofa/ppsp.for |
| `sofa/pvup.for` | native_not_retained | none |
| `sofa/pvxpv.for` | native_not_retained | sofa/cpv.for, sofa/ppp.for, sofa/pxp.for |
| `sofa/pxp.for` | retained_native | none |
| `sofa/rm2v.for` | native_not_retained | none |
| `sofa/rv2m.for` | native_not_retained | none |
| `sofa/rx.for` | retained_native | sofa/cr.for, sofa/ir.for, sofa/rxr.for |
| `sofa/rxp.for` | retained_native | sofa/cp.for |
| `sofa/rxpv.for` | retained_native | sofa/rxp.for |
| `sofa/rxr.for` | retained_native | sofa/cr.for |
| `sofa/ry.for` | retained_native | sofa/cr.for, sofa/ir.for, sofa/rxr.for |
| `sofa/rz.for` | retained_native | sofa/cr.for, sofa/ir.for, sofa/rxr.for |
| `sofa/s00.for` | retained_native | sofa/anpm.for |
| `sofa/s00a.for` | native_not_retained | sofa/bpn2xy.for, sofa/pnm00a.for, sofa/s00.for |
| `sofa/s00b.for` | native_not_retained | sofa/bpn2xy.for, sofa/pnm00b.for, sofa/s00.for |
| `sofa/s2c.for` | native_not_retained | none |
| `sofa/s2p.for` | native_not_retained | sofa/s2c.for, sofa/sxp.for |
| `sofa/s2pv.for` | native_not_retained | none |
| `sofa/s2xpv.for` | native_not_retained | sofa/sxp.for |
| `sofa/sepp.for` | native_not_retained | sofa/pdp.for, sofa/pm.for, sofa/pxp.for |
| `sofa/seps.for` | native_not_retained | sofa/s2c.for, sofa/sepp.for |
| `sofa/sp00.for` | native_not_retained | none |
| `sofa/starpm.for` | native_not_retained | sofa/pdp.for, sofa/pm.for, sofa/pvstar.for, sofa/pvu.for, sofa/starpv.for |
| `sofa/starpv.for` | native_not_retained | sofa/pdp.for, sofa/pm.for, sofa/pmp.for, sofa/pn.for, sofa/ppp.for, sofa/s2pv.for, sofa/sxp.for, sofa/zp.for |
| `sofa/sxp.for` | retained_native | none |
| `sofa/sxpv.for` | native_not_retained | sofa/s2xpv.for |
| `sofa/tr.for` | retained_native | sofa/cr.for |
| `sofa/trxp.for` | retained_native | sofa/rxp.for, sofa/tr.for |
| `sofa/trxpv.for` | retained_native | sofa/rxpv.for, sofa/tr.for |
| `sofa/xys00a.for` | native_not_retained | sofa/bpn2xy.for, sofa/pnm00a.for, sofa/s00.for |
| `sofa/xys00b.for` | native_not_retained | sofa/bpn2xy.for, sofa/pnm00b.for, sofa/s00.for |
| `sofa/zp.for` | native_not_retained | none |
| `sofa/zpv.for` | native_not_retained | sofa/zp.for |
| `sofa/zr.for` | retained_native | none |

## Associated headers and generated artifacts

`Makefile.am:40` shares headers broadly; association below is build membership rather than an assertion that every translation unit includes every header. Direct includes are indexed separately.

| Header | Libraries | Generated from |
| --- | --- | --- |
| `GWsim.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `T2accel.h` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `T2toolkit.h` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `TKcholesky.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | cholesky.h |
| `TKfit.h` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `TKlog.h` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `TKlongdouble.float128.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `TKlongdouble.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | TKlongdouble.float128.h, TKlongdouble.ld.h |
| `TKlongdouble.ld.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `TKmatrix.h` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `TKrobust.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `TKspectrum.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `TKsvd.h` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `cholesky.h` | libtempo2_la_SOURCES, libt2toolkit_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `choleskyRoutines.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `constraints.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `constraints_covar.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `constraints_nestlike.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `constraints_param.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `enum_str.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | extract_enums.sh, tempo2.h |
| `ifteph.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `ifunc.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `jpl_int.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `jpleph.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `read_fortran.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `read_fortran2.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `shapelet.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `t2fit.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `t2fit_dmmodel.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `t2fit_dmother.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `t2fit_fitwaves.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `t2fit_glitch.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `t2fit_gw.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `t2fit_ifunc.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `t2fit_nestlike.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `t2fit_position.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `t2fit_solarwind.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `t2fit_stdFitFuncs.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `t2help.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | convertxt.c, help.txt, help_extra.txt |
| `tabulatedfunction.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `tempo2.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `tempo2Util.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `tempo2pred.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |
| `tempo2pred_int.h` | libtempo2_la_SOURCES, libtempo2pred_la_SOURCES | pinned source blob |

## Other root sources and conditional dependencies

- `tempo2.C`: core executable orchestration, separately audited below; not a library translation unit.
- `getPeriod.C`: installed predictor client tool (Makefile.am:99); its libtempo2pred dependency remains core.
- `convertxt.c`: noninstalled build helper generating help data (Makefile.am:19,197).
- `age.C`: root source absent from pinned library/program source lists; recorded as unbuilt, not silently counted as transferred.

- **SOFA**: Always linked by core; Rust retains verified 36-file closure out of 121 bundled files. `S:Makefile.am:48`, `S:sofa/Makefile.am:6`.
- **OpenMP**: calculate_bclt per-object flags; disabled in A2 and not a Rust parallel timing implementation. `S:Makefile.am:52`, `S:configure.ac:142`.
- **LAPACK/BLAS**: Optional accelerated paths present in source, disabled in reference and absent in baseline Rust. `S:Makefile.am:55`, `S:Makefile.am:60`, `S:T2accel.C:17`.
- **CALCEPH**: Both enabled code and disabled failure stubs remain indexed; absent in baseline Rust. `S:Makefile.am:65`, `S:readEphemeris_calceph.C:41`.
- **float128/native/QD**: Reference uses float128 and quadmath; Rust Rug128 is a different fixed representation. QD/native alternatives unverified. `S:Makefile.am:164`, `S:Makefile.am:91`, `S:configure.ac:65`.
- **libm/Fortran/C++ runtime**: Native build/link dependencies; actual recorded compiler/runtime profile in A2 and Rust manifests. `S:configure.ac:57`, `S:Makefile.am:88`.
- **GSL**: Configure detection exists but root core LIBADD has no GSL link entry; do not infer core GSL dependency from plugin support. `S:configure.ac:195`, `S:Makefile.am:48`.
- **plugins/UI/tests/tools**: Separate targets/subdirectories; core entrypoint and embedded fit/output wrappers still require disposition. No plugin/GUI binary-compatibility claim. `S:Makefile.am:4`, `S:Makefile.am:16`, `S:Makefile.am:87`, `S:Makefile.am:118`, `S:Makefile.am:128`, `S:Makefile.am:141`.
