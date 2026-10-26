# Toolkit, spectrum and GW APIs: prepared Assignment AG

Central design decision, 25 October 2026; interface and sequencing references reconciled on 26 October with accepted decisions 0084/0086/0088. **Prepare AG for the remaining source-defined statistical, random, spectral and GW simulation APIs, sequenced after AF. AG is not released.** Under 0074/0082, an explicit release must name its reviewed predecessor implementation baseline, acceptance status and ownership transfer. AF currently owns Rust under 0088; AE ownership is closed. Functional implementation continues while localized numerical-policy questions remain tracked under 0065/0074.

After release AG owns `rust/` and new `validation/analysis_tools/` as the sole Rust writer. Reuse the canonical state/runtime, timing/fitting APIs and all applicable code delivered by AD/AE/AF. AE supplied the spectrum/toolkit bodies needed by its fitting dependency closure; AG completes the remaining standalone responsibilities using those canonical implementations. AD's six-routine eccentric-GW closure is reused. A source file is not complete merely because one consumer's dependency subset is present.

Policies 0053, 0039 and 0050 remain binding: small demonstrated bugs may be corrected with adjacent pinned source/cause/fix comments and focused original/corrected evidence; generated outputs stay ignored; accepted evidence/runner membership stays unchanged. Preserve original 0008, source notation/counterparts and justified minimal unsafe. Central retains numerical design and milestone acceptance.

## Usable deliverables

Deliver public safe Rust equivalents of every defined in-scope source routine and authored runnable examples for three workflows:

```text
cargo run --manifest-path rust/Cargo.toml --example toolkit -- INPUT_AND_SEED_ARGUMENTS
cargo run --manifest-path rust/Cargo.toml --example spectrum -- --par MODEL.par --tim INPUT.tim --runtime T2runtime -- EXPLICIT_ANALYSIS_ARGUMENTS
cargo run --manifest-path rust/Cargo.toml --example gw_simulation -- --par MODEL.par --tim INPUT.tim --runtime T2runtime -- EXPLICIT_GW_ARGUMENTS
```

These are prepared entry points. Freeze their concrete argument maps and committed input recipes against the released baseline before implementation; they must become executable acceptance examples, not remain placeholders in the report. The toolkit example covers statistics, associated-array sorting and deterministic RNG histories. The spectrum example obtains actual canonical timing residuals and exercises source spectral estimation, covariance-aware routes and sigma-z. The GW example constructs seeded source populations, prepares their geometry, computes per-TOA residuals and exchanges the source background-file format. It must cover tensor and general polarization families and anisotropic/dipole cases, not merely call a scalar helper on hand-filled buffers.

Examples remain thin callers of public APIs. Use existing input/preprocessing/BAT/residual/fitting routines. A spectrum operation does not silently fit a timing model: an example requiring postfit data explicitly calls the released fit cycle and identifies its output. Every example fixes SAT/BAT/BBAT choice, epoch, units, row identity, selection and supplied/effective controls before comparison. For a source-grounded GW caller, the inspected `GWbkgrd` plugin uses `(SAT-PEPOCH_of_pulsar_zero)*SECDAY` as the shared time origin and prepares GW geometry before summing source residuals. Reproduce the chosen caller contract explicitly; the generic core residual API still accepts its caller's time argument. This uses plugin call-site context without requiring a GUI/plugin port or silently injecting changes into TOAs.

Results expose the source's scalar returns, written output extent and reported count separately where those differ, original array mutations, exact random-seed/state transitions, source warnings and defined nonfinite classes. Name spectral amplitude, power, normalized power, PSD, real/imaginary components and error estimates according to their actual source units. Do not label every spectral output as power or every return value as a success code. Unsupported/removed source declarations are identified explicitly rather than supplied with an invented algorithm.

## Source responsibilities

Use raw pinned TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f` and the existing reference build. Complete routine/body/dispatch mapping inside implementation; this preparation inspected interfaces, drivers, RNG, global-state paths, major GW generation/residual/I/O branches and selected fitting consumers, not every arithmetic expression.

| Source | Required transfer |
| --- | --- |
| `T2toolkit.C/.h` | Every defined float/double conversion, min/max/median/mean/variance/RMS/range/sign helper, associated-array sort, zero-mean operation, seed helper and MT19937/uniform/Gaussian routine. Reuse existing selected helpers in SVD/fitting; expose one implementation of each source responsibility. |
| `t2toolkit_global.C` | Standalone toolkit MAX_OBSN definition and its relationship to the canonical process configuration. Do not create a second inconsistent capacity setting. |
| `TKspectrum.C/.h` | Full TKspectrum driver and every defined DFT/Lomb/FFT/weighted-fit, averaging/differencing/smoothing/spline, covariance-spectrum and sigma-z routine, including their helpers and state. Header-only removed declarations such as TK_fitSine/sineFunc need an evidenced source disposition, not a guessed implementation. |
| `t2spectrum.C` | calcSpectra_ri/_T, ordinary/IFUNC sine/cosine callbacks and GLOBAL_OMEGA lifecycle. Retain the current delegated calcSpectra_ri route; do not revive its commented older call. |
| `GWsim.C/.h` | Complete remaining source/type/geometry/matrix/background/polarization/harmonic/dipole/residual/read/write responsibilities and eccResWithEnergy. Reuse AD's eccRes/Rs/dadt/dedt/dtdt/Fe bodies. Preserve the energy variant's distinct writes and numeric boundaries; merge code only where source equivalence is established. |
| Existing TKfit/TKmatrix/Cholesky and core state | Consume AE's canonical implementations and fill any required dependency gap under the same source contract. No alternate solver, FFT package, RNG, parameter store or generic inference framework is introduced. |

Safe Rust public functionality and source file exchange are this stage's deliverables. Decision 0084 resolves the earlier broad C-interface placeholder: the selected guarantee is full Rust core/toolkit APIs plus standalone predictor C compatibility. AJ qualifies installed external Rust consumers and the predictor C library; general core/toolkit C/C++ ABI compatibility is outside that selected guarantee. Every source-defined scientific function remains required through its mapped Rust interface. Remaining core text/PAR output, full command entry points, optional backends and platform/install/performance/concurrency qualification remain required, including actual macOS arm64 and Linux x86_64 execution under 0086.

## Numeric, state and scientific conventions

Retain actual source f32/f64, timing-longdouble/Rug128, native-long-double and integer boundaries. TKfirstDifference copies into float scratch and subtracts there; TKfindRMSweight_d returns float; TK_weightLS has extended accumulators with different cast positions. Keep those distinctions. Preserve source sum/order, exact classes and integer/count/branch semantics. Fixed-width MT state must implement the source's explicit masks and unsigned arithmetic; it is not permission to use wrapping for unrelated signed/count operations.

Use the established owned process/session state pattern for source globals, callback frequency, sigma-z arrays/history and RNG state. Preserve actual initialization and repeat/interleaving behavior; fresh versus reused state must be deliberate and observable. No unchecked mutable static or new general state framework is needed. A direct API may expose an explicit owner where appropriate, but one canonical state path must serve all consumers. A later parallel implementation must not silently replace the serial source random stream with independent streams.

Important source contracts are:

- MT19937's default uninitialized seed is 5489; a negative TKranDev seed initializes the generator and is then replaced by 1. Uniform output divides by 2^32-1 and includes endpoints. TKgaussDev repeats the polar rejection loop and returns one component without caching the other. TKsetSeed uses negative wall-clock seconds; qualify it with a controlled clock witness instead of asserting live-clock reproducibility. Exact seeded integers, seed writes, draw order and rejection history are acceptance observables.
- Toolkit bubble sorts exchange only on strict greater-than. Equal keys, signed zeros, NaNs and associated arrays retain the source-defined behavior. Do not substitute a total-order sort or numerically improved reduction without evidence. Empty and singleton inputs have routine-specific outcomes: defined NaN/zero is distinct from invalid backing.
- TKspectrum preserves its ordered sort/average/mean/spline/smooth/white-noise/difference/estimator/postdarkening operations. Its first-difference precision, fixed spline grid, FFT direction/scaling, frequency grid and every specOut normalization matter. Diagnostic output selectors can terminate early after producing an intermediate table; represent those outcomes explicitly and preserve their source status at the adapter boundary.
- Source spectral frequencies are commonly cycles/day, with some PSD paths normalized to yr^3 using 365.25 and 86400. Trace each quantity rather than applying one global normalization. The returned TKspectrum count and written extent need explicit mapping. Retain source mean versus no-mean design columns, bin factors, tolerance/rescaling and real/imaginary/error conventions in covariance-aware routes.
- Sigma-z uses canonical observations, source SAT/residual/error conversions, one-based arrays, weighting choices, polynomial fits, bias/error expressions, tau iteration and retained state. Preserve actual deletion/sorting/membership choices rather than borrowing a generic filtered timing view. Recover from a failed analysis without silently reusing partially invalid state.
- GW angular quantities and sky vectors follow the source coordinates and rotation order. Despite an imprecise header comment, omega_g participates in phase as time*omega and is constructed with 2*pi: document angular frequency in rad/s and distinguish it from flo/fhi in Hz. Distances and time arguments retain the residual routine's metres/seconds convention. Keep dist==0's Earth-term-only branch and real/imaginary polarization terms.
- Preserve the different speed constants in calculateResidualGW and calculateResidualgeneralGW unless a separately justified correction is made; do not silently harmonize scientific constants. Likewise preserve the source eccentric evolution, coalescence/history, 1e9 residual factor and eOut cast locations. Establish eOut's dimensional interpretation from the adopted source constants before presenting it as a physical energy quantity.
- General populations use the source four-family order/counts, log/linear frequency sampling, spectrum/amplitude conventions and exact RNG consumption. Preserve the anisotropic grid/doubling, harmonic convention, CDF selection and source warning/shift for negative grid probabilities. Keep Findphi's source bisection and tolerance. Do not replace these choices with another sampler or silently reject a source-defined renormalization.

## Background file exchange

The source writes native realization-id/count integers followed by the first nine (tensor) or seventeen (general) timing-longdouble fields, not a complete source struct. Derived h/h_im/kg and later binary metadata are not serialized. Preserve the defined prefix, ordering, cursor/realization semantics and actual caller's separate setup step after reading.

Support the identified reference wire profile explicitly: native little-endian 32-bit integer headers and binary128 scalar fields. Freeze sizes/offsets/endian/float class behavior in a native layout witness before implementation. Encode/decode those fields by value with a safe owned codec; never write Rug object memory. Rounding to the file's binary128 format is an explicit I/O boundary, not a second production timing precision. Imported untagged files require known profile/provenance; do not guess another native layout. Further profiles require explicit qualification. Existing reference files remain immutable, and generated background files remain untracked.

Demonstrate native-to-Rust and Rust-to-native exchange, multiple/nonconsecutive realization ids, cursor position, exact record counts, truncated/missing records and subsequent valid processing. Preserve defined successful-prefix behavior unless a separately documented defect warrants correction. A read must not silently initialize nonserialized scientific fields and claim they came from the file.

## Focused defect questions inside implementation

These are source-inspection findings/questions, not approved numerical replacements. The active owner implementing a required dependency should address them under 0053; do not postpone an AE-needed correction merely because the final standalone coverage belongs to AG. Preserve original and narrowly corrected evidence and add adjacent explanations.

| Source | Concrete check |
| --- | --- |
| TKspectrum sort/weighted/resampling paths | x/y are sorted without e, and later weighted routes can consume transformed rows with original errors. Establish a real row/uncertainty mismatch with unsorted distinct errors. Correct a proven association bug locally. Error propagation through resampling can require a broader scientific decision; do not invent it or hide an unsupported combination. |
| TKspectrum fixed grid, counts and helpers | The 4096-point spline route writes arrays sized from n; output copies nSpec values but reports nSpec-1. Check actual backing and caller count conventions, with valid neighbors. Bounds/allocation correction is distinct from changing grid/normalization; an unusual count alone is not proof of a defect. |
| TK_weightLS | Compare the displayed two-coefficient elimination with its weighted design columns and an independently solved, well-conditioned synthetic sinusoid. Establish the intended equations before changing factors or cast positions; no wholesale estimator replacement. |
| calcSpectraErr_complex / TKfit diagnostics | The solver receives global sigma-z data instead of resy as unwhitened input. Its actual whitening solve uses white_data, while writeResiduals consumes the unwhitened pointer. Test the diagnostics-enabled path and call history; do not incorrectly claim every ordinary fitted spectrum is affected. |
| GWgeneralbackground / GWgeneralanisotropicbackground | The linear-frequency vector branch uses across_g==0 inside a chained assignment. Prove the incorrect residual polarization/state with initialized distinguishable fields and a valid neighbor, then correct the assignment rather than zeroing an entire object indiscriminately. |
| calculateResidualGW / calculateResidualgeneralGW | The exact 1+cosMu==0 branch initializes the real factor but leaves its imaginary partner unwritten. Use a bounded valid geometry and a defined corrected-state witness; do not manufacture a golden result from arbitrary native memory or add a heuristic near-alignment threshold. |
| GW readers, sigma-z and shared histories | Check read counts, negative/excess sizes, seed conversion limits, one-based storage bounds and real initialization/cache histories. Preserve defined nonfinite outcomes and source diagnostics; correct demonstrated unwritten/allocation/I/O defects without broad fabricated preconditions. |

## Verification and delivery

Freeze the public input examples, projection/units, native source identities and new quantity-specific budgets before Rust comparisons. Reuse existing E/AD/AE evidence and native infrastructure. No independent reference-only prerequisite is created. Additional native experiments belong to these concrete routines or source-defect questions.

Required evidence includes exact RNG histories across twist/reseed/uniform/Gaussian/interleaved callers; all defined toolkit operations and associated-array mutations; all spectrum estimator/output and sigma-z weighting/state families; shared fit/whitening callback behavior; all GW population/polarization and file routes; the energy variant and inherited eccentric neighbors. Test every defined body/branch family with source-grounded inputs, not only the three demonstration workflows. Keep source-removed declarations and genuinely invalid native paths explicitly classified.

Also require the public workflows on actual loaded timing state, paired native/Rust array inputs, independent sinusoid/zero-amplitude/geometry diagnostics where applicable, cross-reader files, failure/recovery and changed-context histories. Test comparator rejection of altered units, bins/counts, RNG draws, polarization membership and state mutations. A matching distribution mean or a passing scalar helper does not establish an exact RNG or full timing workflow.

Retain the existing 1e-12 s timing/GW-residual and 1e-10 turn phase gates where those observables occur, the existing fitting column/solver gates and all discrete/class requirements. New spectral, sigma-z, coefficient/amplitude and eOut quantities need explicit source-type/unit/scale/structural-zero budgets before comparison; physical approximation/model validation is distinct from implementation equivalence. A proposed change to an existing gate, numerical architecture or scientific assumption returns to central. Source algorithm differences are not fixed by a blanket tolerance.

Run affected debug/release checks and relevant inherited regressions, formatting and Clippy. Report original versus corrected comparisons, actual run/source/build identities and unrun platforms/backends. Localized pending numerical decisions may coexist with a reviewed functional handover under 0074, but cannot be relabelled as accepted. Keep M2, 0049, DDH, P and any new gaps explicit.

Deliver Conventional Commits of authored owned files, `rust/ANALYSIS_TOOLS_REPORT.md`, the runnable examples, complete source/type/unsafe/departure maps, native exchange and state evidence, comparison maxima/failures and remaining inventory. Preserve original license/provenance notices, including MT19937. Generated JSON, binary outputs and traces remain ignored. Full migration and the library guarantees selected in 0084 remain unaccepted until their separate obligations are proved.

## Preparation evidence

This is a prepared contract based on source inspection. It introduces no Rust implementation, new numerical experiment, scientific correction or acceptance claim. The GWbkgrd plugin was read only for an example call-site convention; its implementation is not included in AG. Raw source identities are:

| Source | SHA-256 |
| --- | --- |
| `T2toolkit.C` | `70a6f7c0f24e5f55e244a02edbe49f7cbcc56068dc364e8e7cb7efe2c9cdd6c6` |
| `T2toolkit.h` | `d10ac747095d34c52334af2198888f8b19fa24942be00625eab9dbf12066f2f7` |
| `t2toolkit_global.C` | `6de90d090755ff7633918ec573c221a77453904fe9e4842996da74381234f8f0` |
| `TKspectrum.C` | `d7980bdac984091ed9b444ea60b301f8243cc04116a8ed1342174babc49dcb07` |
| `TKspectrum.h` | `f4d93e5352fc9e01fe86c3c67e3f6c3bab7db1317cf02b68ef1bd213e3a1a160` |
| `t2spectrum.C` | `ddb4a7c1cf8f9fdad7be2155a3a56f464b7121f8a3f0afd67d9bbac14e6fe937` |
| `GWsim.C` | `412d74045d9b4d8196c554e38d519a019c021eae02b3b6b1a418f3d21984296c` |
| `GWsim.h` | `e53d5868392d1f0d5dfdc4bd2a2dd2b73bc30b82abec41f7ddf9f7f4e499cc54` |
| `TKfit.C` | `18c4124563edceeea3f19694aec1256288fa1e744f69bea97c2070c2c59f4abc` |
| `plugin/GWbkgrd_plug.C` | `6a640800bca84eb5f20f4e292325cfa65ea9254abc274046a33a2d8acd5d0791` |
