# AD general fixed-parameter timing delivery

AD implements the released 0070 contract on AC production `18c9065` / report
`d47ae51`, released by 0075 (`e46465a`). The interface freeze is `8b1bdcb`; production and validation are committed in
`a8e5178` (`feat(timing): implement complete fixed-parameter residual core`).
Raw TEMPO2 pin: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
Central retains milestone acceptance. This delivery does not close the inherited
P, M2, 0049 or original DDH gates.

## Delivered public behavior

`timing --par P --tim T [ordered pairs] --runtime ROOT [--no-mean]
[-- PREPROCESS_ARGS]` uses the canonical AC loader, preprocessing, resource owner,
formBatsAll and full formResiduals. Mean removal defaults to true, matching the
source CLI; no fitting is performed. `timing::{evaluate, form_residuals,
average_residuals, average_dm_residuals}` and `binary::{model, fit_function,
update}` operate on the same canonical `PulsarContext` / parameter / observation
owners. Direct residual and binary calls require prepared state and do not redo
BAT calculation. Binary delay dispatch is enabled by PB[0] or FB[0] set exactly
1, not BINARY text alone. Model names remain source-sensitive.

The version-1 result retains AC inputs/resources/byte diagnostics and adds PET,
binary delay, phase/nphase, exact signed pulse number, residual/residualtn,
applicable averaged fields and value/error/set/fit parameter mutations. Pulse
numbers serialize as decimal strings. Units/frames come from each pulsar. Expanded
TIM input indices are attached before preprocessing and follow complete canonical
copies, source sorting and TZR handling. TZR is a separate complete record;
its averaged fields are inapplicable. Unwritten/inapplicable optional fields are
null, signed nonfinite classes are explicit, and required fields must be written.
JSON is serialized completely before publication.

The old bounded `load`, `residuals`, barycentric/terrestrial evaluation and fit
contracts use a thin working-copy/projection adapter in `form_residuals.rs`.
The previous selected spin kernel is removed. The adapter supplies only the
controls guaranteed by its bounded five-column TIM grammar; it does not create
another scientific engine or parameter store. The general workflow supports
multiple pulsars and source mutations explicitly. General fitting remains later.

## Routine and precision map

| Source at the raw pin | Rust counterpart and extent |
| --- | --- |
| BTmodel.C, BTJmodel.C, BTXmodel.C | `binary/{bt,btj,btx}.rs`: each model and updater |
| DDmodel.C, DDKmodel.C, DDSmodel.C, DDHmodel.C, MSSmodel.C | `binary/{dd,ddk,dds,ddh,mss}.rs`: each model and updater |
| DDGRmodel.C | `binary/ddgr.rs`: model, updater, mass2dd |
| ELL1model.C, ELL1kmodel.C | `binary/{ell1,ell1k}.rs`: each model and updater |
| ELL1Hmodel.C | `binary/ell1h.rs`: model, updater, calcDH |
| T2-PTAmodel.C | `binary/t2_pta.rs`: model, updater, computeU |
| T2model.C | `binary/t2.rs`: model, updater, getParameter, calcGR, getKeplerian, addKeplerianJumps, getPostKeplerian, updateParameters, deriveKeplerian, derivePostKeplerian, KopeikinTerms, computeU, calcDH |
| t2fit_stdFitFuncs.C | `binary/mod.rs`: t2FitFunc_binaryModels and t2UpdateFunc_binaryModels only |
| formResiduals.C:396–2473 | `residual_source.rs`: complete formResiduals, including every source phase-contribution family, reference, tracking, noise and averaging calls |
| formResiduals.C:53–393 | `averaging.rs`: averageDMResiduals and averageResiduals; shared loops with explicit DM differences |
| GWsim.C:351–740 | `gw_ecc.rs`: eccRes, Rs, dadt, dedt, dtdt, Fe only |

This transfers 42 binary bodies, two binary callbacks, three residual/averaging
routines and six eccentric-GW bodies. `residualTracking` has no definition/call
in this source and is not claimed as another body. `eccResWithEnergy` and the
remaining GW simulation/background/file APIs are not included. Rust functions
carry individual raw-source line comments; G's existing inventory supplies model
coordinates. Expanded `$Id$` source copies and raw-pin hashes remain distinguished
by the native build recipe.

Source `longdouble` becomes fixed Rug128, not native binary128's 113 significand
bits. Source double and native C++ long double retain binary64 boundaries on this
Darwin host; float locals retain binary32. Expressions preserve source grouping,
literal promotion, narrowing and helper variants. Native DD returns extended
precision; its fit wrapper narrows to binary64. Averaging retains float timestep
and `powf` argument/result narrowing. Eccentric integration retains binary64 RK4
locals, source constants, 1e9 factor, integration count/cap and prior-orbit state.
No local FMA, alternative numerical dependency or precision mode was added.
Accepted dependency FMA sites and numeric utility boundaries are unchanged.

New scientific modules contain no unsafe Rust. Three extra binary64 formatting
choices reuse `clock_text::format_number`'s existing typed, capacity-bounded
`snprintf` call; `%Lg` six-significant-digit diagnostics use safe Rug formatting.
No Rust binary model calls native TEMPO2. Retained SOFA/libc resource dependencies
remain as documented in `SOURCE_CORRESPONDENCE.md`.

## Source lifecycle and boundaries

The full routine retains split integer/fractional phase through F12, state/braking,
glitch phase/frequency/three recoveries, nudot transitions, events, jump/FD terms,
shapelets, WAVE/WAV_DM, scalar/quadrupole interpolation, periodic/transient GW,
clock offsets, radd/padd, IPERHARM, TRACK/pn/pnadd and phase-jump ordering. SAT,
BAT, BBAT, frequency units, epoch inequalities and source-written geometry caches
remain source-specific. Defined oddities, including BT EDOT, ELL1k EPSDOT and
ELL1H NHARM's mode overwrite, remain unchanged.

Reference selection follows eligible source order and the SAT/BAT START/FINISH
fit selectors. Residual evaluation does not sort to conceal a tracking error.
Tracking and eccentric coalescence locals retain their outer-pulsar-loop lifetime.
TZR is reinstated by whole-record copy, included in phase construction, stored
back, removed, and then used to shift main residuals at the source point. Source
mean removal does not imply the same change to phase, counts or residualtn.

Noise subtraction requires actually supplied TNRedSignal, TNDMSignal,
TNChromSignal and SWGPSignal when selected; CM uses the existing interpolation.
There is no production zero-noise default or fitted-noise inference. The natural
eccentric path consumes a previously produced residual; the public library example
runs a real base timing evaluation before applying its eccentric controls. A fresh
request without that producer returns an unwritten-residual error.

Averages retain discovery order, case-sensitive discovery versus case-insensitive
matching, last-match ECORR/SECORR, inverse-variance weights and source output order.
Ordinary averaging writes exact source-format `avg.dat` in the working directory;
DM averages are seconds at 1400 MHz. All arrays, including AverageFreq, are owned.
A missing ordinary flag retains old averaged fields if present; an actual unwritten
read errors. DM's missing-flag warning and early return preserve all prior fields.
File errors are recoverable. Completed source writes survive errors; no rollback
or arbitrary same-context retry is promised. Fresh-context/runtime recovery and
specified repeat-call paths are exercised separately.

Invalid signed counts/conversions/overflow and consumed unwritten storage return
contextual errors instead of native undefined behavior or Rust wrapping. Failed
`%lld` conversion leaves its previous local unchanged; overflow errors. IFUNC and
ORBIFUNC step selectors retain the source's first matching `k < N-1 && SAT >= T[k]`
rule. If no assignment occurs, Rust errors rather than guessing extrapolation.
The unknown delay model retains its diagnostic and becomes an error. The binary
fit wrapper's unknown model has no defined native return; Rust uses that explicit
error path instead of inventing a coefficient. Unknown update dispatch remains
source no-op.

## Demonstrated corrections (0053)

Original G records/classifications remain immutable. `native_timing.py` builds
separate corrected source in AD staging using the verified AC recipe. Exact patch,
source/object/executable hashes and compiler commands are recorded there.

| Raw source / cause | Narrow correction and focused evidence |
| --- | --- |
| DDKmodel.C:267, derivative path reads uninitialized dtdpx with no return use | Omit that dead calculation, without assigning a parallax. Corrected G baseline/default/derivative neighbors |
| T2model.C:getParameter:634–651 admits k==aSize | Change `>` to `>=`, preserving absent-companion zero. Natural two-companion fixture and corrected G |
| T2model.C companion predicates directly index scalar paramSet[com] | Check declared availability before consuming an optional companion flag. The natural two-companion case exposed this separately after getParameter was fixed; absence matches getParameter semantics, with no invented backing cells |
| ELL1H/T2 orthometric intermediates use inputs absent in H3/H4 modes | Evaluate STIG fs/lgf/lsc only in mode 1 or explicit STIG derivative; ELL1H brace/dlogbr/Cm2/Csi only in consuming selectors. Undefined STIG and H3-only SINI/M2 requests remain withheld; NHARM selection unchanged |
| ELL1model.C step ORBIFUNC uses ifuncN | Use orbifuncN. Corrected step neighbors pass; no-assignment range is an explicit error |
| formResiduals.C:2164–2183 selects psr[0] reference data | Use current p for selector values and observation. Distinct two-pulsar cut windows agree with standalone second pulsar; first and standalone neighbors unchanged |
| formResiduals.C:796–804 shapelet coefficients use psr[0] | Use current p; distinct shapelet/F0 two-pulsar witness and unchanged single-pulsar neighbors |
| formResiduals.C:2463,2468 averages psr[0] repeatedly | Pass current pulsar. Distinct width/F0 pair produces the second pulsar's standalone avg.dat; ordinary and DM routines share this ownership correction |
| formResiduals.C:2410,2450 loops through i<=nobs after TZR removal | Restrict to active i<nobs. Original active noise mean changes with spare-row sentinels; corrected mean is invariant, both with and without a removed TZR |
| formResiduals.C:2388–2392 unused newmean always reads TNRedSignal | Remove unused accumulation. No signal is invented; selected noise subtraction still requires its real producer |
| initialise.C:236 omits AverageDMResiduals | Initialize opt-in flag to zero. Original initialization retains sentinel 763; corrected initialization writes zero |
| initialise.C:120 / readParfile.C:1126 omit initial nWhite_dm | Initialize empty wave-DM count to zero. Original initialization retains sentinel 41; corrected initialization writes zero; real WAV_DM fixture exercises count growth |

The Rust changes have adjacent source/cause comments. Binary corrections are
encoded in `corrections.py`; original/corrected initializer and spare-row/TZR
witnesses are in `check_corrections.py`; defined cross-pulsar witnesses are in
`check_cross_pulsar.py`. The latter removes only the unrelated dead newmean read
from its original variant and never reads the original second pulsar's unwritten
averaged fields. These are explicitly qualified original-source experiments,
not replacements for accepted original artifacts.

## Executed comparisons and gates

Host: `aarch64-apple-darwin`, rustc 1.97.1 (`8bab26f4f`, LLVM 22.1.6), Homebrew
GCC 16.2.0. Native uses `-O2 -std=gnu++11 -DHAVE_CONFIG_H` and the accepted
binary128 configuration/archive. No enabled-CALCEPH path is claimed.

| Check | Executed outcome |
| --- | --- |
| Original G, debug and release | 4,684 checks each: 4,570 delay/callback/update records, 108 integrated residual calls and six source-exit cases; 432 undefined records withheld; zero failures |
| Separate corrected G corpus, eight cases, both profiles | 506 checks each; zero failures. Excluded invalid selectors acquire no corrected golden |
| Independent public native/Rust workflows, both profiles | 65 cases × two mean settings = 130 comparisons each, zero failures; all fourteen families and the authored matrix in EXAMPLES.md |
| Normal Cargo tests, both profiles | 57 library + seven behavior + one reference-contract tests pass; ignored runtime/corpus tests are accounted for separately |
| Averaged-projection comparator controls | Two stored baselines pass; all eleven omitted/extra-field and unit/null mutation controls are rejected. All 260 stored public outputs pass a separately identified recomparison |
| Explicit timing lifecycle tests, both profiles | Six pass: repeat/recovery, active noise/TZR membership, empty/nonpositive count and step boundary, missing eccentric producer, retained DM fields, extended diagnostics |
| CLI failures and following valid work, both profiles | Four pairs each: unknown model, missing stored noise, missing averaged field, unwritable avg.dat; failing stdout is empty |
| M1a/M1b original comparators, both profiles | Three barycentric and two terrestrial cases pass, including 4 ns response |
| Resource and AC lifecycle examples, both profiles | Pass: affected resource failure/recovery, mutable source lifecycle and fresh valid work |
| Affected M2, both profiles | 15 end-to-end, 34 shared-cycle checks and callback comparison per profile. Exactly four inherited raw-order differences remain; verifier exits 1 and does not relabel them passes |
| Formatting and all-target Clippy | `cargo fmt --check` and `cargo clippy ... -- -D warnings` pass |

The prospective gates remain 1e-12 s for time/delay/residual components (MJD
converted in extended arithmetic), 1e-10 turns for phase, exact counts/membership,
5e-14 times each native derivative-column maximum, and 1e-30 relative for stored
extended updates/errors with exact structural zero. Native DD and wrapper calls
are evaluated separately. Updates additionally check binary64 supplied coordinate
bits, actual increment and post-update delay. Before/after subtraction accounts
for recorded binary128 endpoint rounding with
`max(|before|,|after|) * (2^-112 + 1e-35) + |native_increment| * 1e-30`;
this follows endpoint precision, not the observed Rust error. All original G and
legacy comparator budgets remain unchanged.

Public comparisons use separate native/Rust file-reader/preprocessing/BAT/residual
processes, both mean settings, exact row/selector/mutation membership and diagnostic
bytes, and byte-identical avg.dat. Only native diagnostic path/line locations are
mapped back to the raw pin using recorded source-line maps. G component checks
instead install G's recorded prepared state, including its real geometry, to
isolate callbacks from the public preparation pipeline. These two evidence paths
are intentionally distinct.

Central handover review found that the initial public comparator iterated only
supplied averaged keys and did not check units: omitting a residual, changing
its unit to ms or dropping the whole averaged object escaped that comparator.
Production `timing.rs` already supplied the correct six-field projection. The
comparator now requires exactly `bat`, `residual`, `uncertainty`, `dm_bat`,
`dm_residual`, `dm_uncertainty`, each with exactly `value` and its declared `unit`,
including null/inapplicable entries and TZR. `check_comparator.py` rejects all
eleven mutations against two unaltered passing baselines. `recompare_public.py`
checks all 260 stored native/Rust outputs under the corrected comparator without
rebuilding or rerunning either pipeline. Original comparison receipts and output
bytes remain intact; separate `build/recomparison-{debug,release}/` receipts hash
them and identify the new comparator/rechecker. Original diagnostic-byte and
avg.dat comparisons remain recorded by the original pipeline executions.

An early direct G preparation diagnostic failed at
`observation_unwritten_earth_ssb[3]` when it called BAT calculation after load but
omitted preprocessing. Public `load_pairs -> preprocess -> form_bats_all` succeeds
on that same PKS fixture. This was an invalid probe call path, not an AC production
failure or a reason to seed zero geometry. Earlier incomplete fixture attempts
and logs are not final numerical evidence; named final receipts below supersede
them.

## Reproduction, identities and remaining obligations

Run commands in `../validation/timing_models/EXAMPLES.md`. The local ignored
`build/handover.json` indexes the final component logs, complete public comparisons,
compatibility results, correction witnesses, compiler/executable/source/input hashes
and protected-source status. `check_components.py` records each actual test
executable. Each public comparison carries its native identity snapshot, Rust
executable/source hashes, actual commands, consumed-file hashes and measurements.
No large generated JSON/JSONL, native objects or binary outputs are committed.

Current corrected native executable SHA-256:
`d0a77af120df5888aeac9d02aed9be096bdd53fda823f9001bf6715118b73ed3`.
Correction patch SHA-256:
`44ddd74fc9666c5f31c702dc38c2e1baac10a63e5b0dba795a093e1fb371124e`.
Original 0008 remains SHA-256
`2c5f81b82c43105fcd48b62139f1d78f2dc7c4089055ea162e9feef82ff4e12c`.
Original accepted reference outputs and canonical TEMPO2 are unmodified.

The inherited ledger remains: P SIMUL 32, units 71, ordered SAT offsets two,
links four and epochs 45; M2 four raw-order differences; four 0049 utility
boundaries; original DDH 1 ns gate (first 1.137585 ns, maximum 4.598955 ns).
Original AB's 277 differences remain distinct from the 0075-approved corrected
behavior. Decision 0077 reproduces P's 154 records at 128 bits and removes them
in a disposable 113-bit diagnosis, localizing the cause without changing the
production precision or any gate. AD makes no speculative P arithmetic rewrite
and does not repeat its unchanged full corpus.

These checks establish engineering correspondence on the stated inputs. They do
not independently validate every physical approximation, all branch combinations,
scientific inference, a general fit, predictor, full upstream CLI/textOutput,
remaining GWsim APIs, external selection plugins, CALCEPH execution, cross-platform
precision, concurrent execution or performance. Central review and unresolved
numerical-policy decisions remain necessary for overall migration acceptance.
