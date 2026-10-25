# General input-to-BAT implementation: prepared Assignment AC

Central design decision, 25 October 2026. **Prepare one functional implementation stage for general PAR/TIM input through BAT/BBAT using the native JPL backend. Do not release AC until AA is accepted and its Rust ownership closes.** AA remains the sole Rust writer. Central will pin its accepted revision and explicitly release AC. This preparation is not an acceptance or permission for a second concurrent Rust writer.

This implements 0065's functional priority. The deliverable is a usable library and CLI workflow, with the source reader/preprocessing/planetary/propagation responsibilities connected. It is not another reference-only assignment. Implementation proceeds in coherent internal commits; intermediate components do not replace the final workflow acceptance target. Full residual/binary/fitting, enabled CALCEPH, complete upstream CLI/plugin integration and release qualification remain later obligations of the full core migration.

## Functional boundary and public interface

Add a `bats` command to the existing crate/CLI. It accepts one or more ordered PAR/TIM pairs, an explicit runtime root and optional source preprocessing arguments after `--`:

```text
tempo2-rust bats --par MODEL.par --tim INPUT.tim --runtime T2runtime
tempo2-rust bats --par P0.par --tim P0.tim --par P1.par --tim P1.tim --runtime T2runtime -- -nofit
```

Pair the nth PAR with the nth TIM, retain pair order and reject an unequal count before loading. This command reads the general source grammar, performs source preprocessing and the full native-JPL formBatsAll sequence, then returns all main observations' SAT/BAT/BBAT, correction components and declared units/frames/site identity, source deletion/order state, diagnostics and actual input/resource provenance. Report TZR reference state separately from the main observations when active. Do not produce residuals or fitting results, and do not imply that parsing a binary/noise parameter implements its later residual or fit model.

Expose the same workflow in the library: load ordered source files into one owned canonical PulsarContext with provenance, run preprocessing, and evaluate BATs through mutable canonical state and RuntimeContext. Source-level read/preprocess/formBatsAll/updateBatsAll entry points must be callable without going through command-line parsing. Use existing types and ownership; a small result/provenance wrapper is acceptable, but no second authoritative parameter/observation store or configurable backend framework is needed. Freeze concrete signatures and the small output schema in AC's authored contract before implementing consumers, applying these semantics.

Keep the existing `load`, `residuals` and `fit` public contracts and their fixed JSON schemas. Their bounded input validation remains a frontend concern while shared numerical calculations use the newly complete source implementation. Their existing unsupported residual/model declarations must not silently become accepted because the new BAT reader recognizes those parameters. There must be one authoritative calculation for each scientific routine. Distinct declared frontend capabilities do not justify duplicate timing engines.

For the new command, preserve the source time-system/units choices and resolved defaults, including TCB/TDB/TEMPO1, rather than labelling every BAT as TCB. Preserve native numeric types and Rug128 decisions; report MJD values with round-trip decimal precision. Names, flags and input bytes stay lossless in canonical storage and provenance. Nonfinite numerical classes, partial native prefixes and recoverable Rust errors have explicit output dispositions; no successful JSON is published before the requested calculation and serialization finish.

The default supported ephemeris engine here is native JPL. Transfer the no-HAVE_CALCEPH failure entries and convertUnits helper in readEphemeris_calceph.C, preserve global selection/order and return the corresponding explicit unavailable-backend error when selected. Do not silently substitute JPL or supplied CALCEPH values for a requested CALCEPH computation. The enabled CALCEPH implementation remains a named full-core obligation; this stage cannot close that entire file. Likewise, a BAT command does not close full getInputs.C, tempo2.C, textOutput.C or external plugin binary compatibility.

## Scope and existing evidence

Use raw source at `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. Reuse accepted maps and corpora; do not re-inventory or regenerate them as a separate prerequisite. Freeze only AC's integration mapping and changes from those maps against accepted AA.

| Component | Required transfer and starting evidence |
| --- | --- |
| PAR readers | Complete readParfile.C dispatch and simple/full/global reader semantics, checkAllSet, scalar/coordinate/value helpers and source-written metadata. K `38d62af` under 0026 and Q `bb7e629` under 0043 cover the shared and specialized branches. Reuse O/T's canonical registry, storage, initialization and subtraction helpers. |
| TIM I/O and ordering | Complete readTimfile.C, sortToAs.C, readJBO_bat.C and read_fortran2.h scope under N `68de89a` / 0032: nineteen declared entries, including writer/helper and legacy paths. Reuse the existing owned stream boundary, with independent source state where required. |
| Preprocessing, units, epochs | All fifteen routines in preProcess.C, preProcessSimple.C, units.C and updateEpoch.C under P `2038c41` / 0040. Later report `b05538e` changes evidence policy only. Include direct helpers, repeated/multi-pulsar state, source selections, transformations and active branches. |
| Planetary ephemerides | All three readEphemeris.C routines under Z `d517acf` / 0064, including random2/gasdev state despite their inactive driver use. Reuse accepted JPL and time-resource implementations. |
| Propagation and phase lifecycle | All 25 routines under AB `cb2c41f` / 0067: geometric/Shapiro/DM/CM/solar-wind delays, BAT/secular assembly, phase-reference initialization/cleanup, interpolation/shapelets and precision no-op. |
| Orchestration | global.C's extra_delays, clock_corrections, ephemeris_routines, formBatsAll and updateBatsAll, reusing the existing CVS/logging implementation and canonical process state. Include source dependency helpers needed by these paths; use accepted M evidence where applicable. |
| Public integration | Existing input/runtime/library/CLI glue, actual resource-root and file identities, a `bats` workflow and executable examples. Reuse accepted AA/W/Y and remove obsolete selected calculation bodies when replaced. |

Raw primary hashes are:

| Source | SHA-256 |
| --- | --- |
| `initialise.C` | `5da876556455e5fda635b30917ae9e745ef880b93ef513b47b6e4823458868c1` |
| `readParfile.C` | `ecdbec472eff256ab1c7d875a4ea56536c146012599684408b03fe738ac7550e` |
| `readTimfile.C` | `01463be0d78e75078cdbba7ae7fcd578e5d7db08fbec476d4f289488803a1ea2` |
| `sortToAs.C` | `59dea2a1e05373edb049725379c90be41939618a261b6601e5026acaac82ec49` |
| `readJBO_bat.C` | `0c7046c8f5db8e19829dfe70bab90c6fcc44ae16c5476c6fb321eb17b393b956` |
| `read_fortran2.h` | `c42706c8c960dfc4b9693e5f3f33b03549f40ee1e40371c33f0628c49e722fca` |
| `preProcess.C` | `51195accf0ae4415cc0e4927f62d6109c3f0567693b59485130ec3cade7b52f3` |
| `preProcessSimple.C` | `662366f42a3033daa68daa47b28b0ac67a565af2c1985ff3c1d90b3c8a100344` |
| `units.C` | `eacea32922f1206fcad02b3e2e7ae6274a006bca5463ae498fcc042b2c757cc3` |
| `updateEpoch.C` | `22724e91713e7cc103a878ab6e2c4e8b125652b4266ff7efee884b6a91fd1889` |
| `readEphemeris.C` | `822678396155955e8f55f307b373d1666b4def988d0b21e0762bf6fd5e53ce3a` |
| `calculate_bclt.C` | `899c58042bc053efeae5b25eacc46de9b4ca1bb0943c995a846e0affb8d597e7` |
| `shapiro_delay.C` | `9ebbfb8af3a5309bf7ad852f1cf5e11408f12190a6902e6706142e53619238d2` |
| `dm_delays.C` | `43e563cbdc570be9d836e0fe26576ad8512261494e5267c28bee99c59a84c7a8` |
| `formBats.C` | `c948373de8fe17ef490ffa360bb7ed5667eb2af47020a490399ae94c50198718` |
| `secularMotion.C` | `02aa8d5c49fcbed2ba52b0e0e3bcec75e1ee0b0524f6370413fa02eb578b6ef2` |
| `refphs.C` | `b8bba6f9fa29da61a2d0ff17cbc8d9fe8b2c7c89e80e5dd19f7e7a9249431100` |
| `sw_delay.C` | `f71de099d59cc6e1adb71d34bf72ed4fe1649248c84a143f2420d21b211647c2` |
| `ifunc.C` | `47bb12351b45abdd5db3b5a30b735837e61363ba58febd7349a4f47b2569b65d` |
| `shapelet.C` | `bc3d57bd355e0f0ac369b22e301f5044dd462ce8a9745779bc5452e6204ffd3b` |
| `storePrecision.C` | `039e08795529ca8eb0c8244c1cefd2030dc6ce1bd60abb4fd701c20b25401aa8` |
| `global.C` | `e61fc1a83a9adf0b6d905d2c5a58cd764a1fd0261c276fccb33daadb6788a2a7` |
| `readEphemeris_calceph.C` | `387ce174dd7108ac335bab4794d1efcc78ad592323a33a11c4a5c6b9bde05b1d` |

Every listed source responsibility remains accounted for, including helper APIs that are not invoked by the public BAT command. Preserve declared inactive blocks as inactive; do not enable the binary epoch update guarded by `&& 0 == 1`, readEphemeris's commented phase/noise application, or unrelated model code. The source inventory remains the full-migration checklist; this stage grouping removes no later obligation.

## Canonical state, source order and integration

Use accepted AA's actual interfaces at release, not a guessed copy of its in-progress code. Extend existing scoped borrows and the established process owner for source-static parser INFO, ephemeris random caches, DMMODEL warning masks, mutable ecliptic obliquity and other actual globals. Keep their distinct lifetimes. Reuse the outer-lock/borrow pattern and avoid recursive locking. A cache read must refer to the same resource identity used in the calculation; provenance must not reopen a changed file and describe unused bytes.

PAR/TIM parsing must retain source scanning/dispatch order, aliases, indexed families, sequential assignments, partial writes, link semantics, include-relative paths, file-format selection, flags, deleted rows, noise/error/time directives and repeated/multi-pulsar histories. Reuse existing byte scanners, numeric construction and canonical setters where their semantics match; the old strict five-token parser is not the general reader. Source-unwritten values stay unwritten until a real source write or an explicitly justified bug correction. Fixture sentinels are never production defaults. Preserve the source sorting comparator and actual equal-key behavior; do not infer a stable-sort contract from the old inventory prose.

Preprocessing must use the native sequencing of links, fit/set changes, noise/error weighting, simple stages, selections, jumps, units and epochs. Track ECLIPTIC_OBLIQUITY as mutable source state; TEMPO1 changes it, and later frame conversions must read the same value. Preserve literal widths, enum/index order and disabled branches. An unsupported external plugin request must fail explicitly at that boundary; silently omitting a requested processing operation is not valid BAT output. Portable plugin binary compatibility is outside this stage, but native direct helper and built-in selection behavior remain in scope.

The complete source orchestration is mandatory:

1. refphs_init before clock or ephemeris work.
2. clock_corrections: toa2utc, tai2ut1 and the source tt2tb/CALCEPH selection order.
3. ephemeris_routines: vectorPulsar, ephemeris read, get_obsCoord, refined time conversion, then ephemeris reread. Preserve the first-pulsar backend selector until a demonstrated correction has an explicit disposition.
4. extra_delays/calculate_bclt, then formBats, then secularMotion.
5. refphs_clean after the completed calculation. Do not insert cleanup earlier or fabricate source mutation after a failure; Rust still releases owned external resources safely.

updateBatsAll retains its narrower vector/delay/BAT/secular sequence and must not silently invoke the whole resource pipeline. General RuntimeContext use must select actual PAR/runtime resources and canonical observatories; it must not retain a hard-coded PKS/DE405/IF99 assumption for the new workflow. The existing narrow adapters still enforce their declared input profile.

Follow Z/AB's detailed contracts for selective planet reads and unconditional retained-tail subtraction; exact flag tests; six-vector prefix versus velocity rotations; perturbation/mass/offset order; MHz versus Hz boundaries; Shapiro normalization/bypass/planet factors; DM/CM derivative, DMMODEL/DMX/shapelet/solar paths; source iteration limit and veryFast behavior; independent TT sums and source BAT grouping; secular gates; and full reference-observation copies. Do not simplify expressions, normalize vectors early, erase source-retained state or replace the model with a conventional alternative.

## Bug corrections needed by a usable workflow

Apply current 0053, not older reference briefs' blanket deferral language. A demonstrated local bug may be fixed during this stage without another permission round, with its original file/routine/location, cause and fix explained beside the changed code, focused regression and explicit original/corrected expectations. Preserve every original oracle.

The ordinary no-`-setdm` preprocessing path needs the known local initialization correction: raw preProcess.C:48 declares ndm uninitialized and line 480 passes it by value. Its later short circuit does not make the argument read defined. Initialize the no-table count to zero, consistent with direct preProcessSimple's explicit zero argument and the defined comment-only setdm neighbor. Prove the corrected default path with a disposable native patch and a focused default/empty-table/nonempty-table regression. Do not publish the undefined original no-setdm call as a numerical golden, require users to supply a dummy DM file, or replace the full wrapper with preProcessSimple to evade its other responsibilities.

Carry the already demonstrated A2 initialization correction into the canonical Rust initializer: initialiseOne leaves ne_sw_ifuncN unwritten, while dm_delays.C:320 reads it on the ordinary spherical-solar-wind path. Initialize that count to zero, with the original initializer/consumer location and A2 cause/fix evidence beside the write. Retain the original S/T uninitialized-state evidence and explicitly record this one changed initialized field; do not zero unrelated unwritten state. Add a normal reader/preprocessing-to-DM regression so the public default path does not depend on harness-provided seeds.

Also trace ordinary post-reader/preprocessing initializedness through readEphemeris.C:252-257, which subtracts inactive planetary arrays unconditionally. Z's explicitly seeded reference backing is not a normal initializer guarantee. Resolve any real unwritten-input dependency with a narrow documented correction or explicit knownness disposition under 0053; preserve defined seeded-neighbor calculations and avoid blanket zero-filling. A harness-only successful path is insufficient for public acceptance.

Reuse accepted corrections in W/Y/AA, including wet mapping, IFTE/FB/CALCEPH supplied-state handling and safe suffix copying. Where W's corrected wet expression changes a composed AB case, retain the original result and generate the narrowly corrected native expectation within AC. Suspicious cross-pulsar accesses require actual bug evidence; mutation sensitivity alone is not proof of an intended correction. Broad physical, precision or algorithm changes remain central decisions.

At real malformed-data, conversion, allocation/owner and source-unwritten boundaries, return contextual errors rather than undefined access or invented values. Preserve defined neighboring source behavior. Trust validated canonical domain invariants; do not scatter defensive checks justified only by fabricated internal states. Do not make an unrelated known M2/DDH failure a prerequisite for implementing this independent capability.

## Delivery sequence and verification

Work through these internal implementation commits, keeping the same final functional objective:

1. Freeze the concrete API/output contract, accepted-AA interface mapping and existing reference applicability; define small authored end-to-end input files before comparisons.
2. Complete the general PAR/TIM readers and preprocessing on canonical state, including the documented default-path correction and source unit/epoch handling.
3. Complete the planetary and propagation source routines using the accepted references and current owners.
4. Connect the global orchestration, library entry points, runtime/provenance and `bats` CLI; route existing scientific consumers through shared implementations.
5. Verify the component contracts and integrated workflows, correct failures caused by the changes, and deliver one functional report for central acceptance.

Reuse all applicable defined K/Q/N/P/Z/AB cases in Rust debug/release with explicit source-safe/corrected dispositions and complete relevant state membership. Previously accepted initialization/resource/AA cases need rerunning when the changed integration affects them; unrelated old native mutation builds do not automatically need repeating. Preserve native-invalid and host-qualified classifications instead of inventing portable goldens. Additional native experiments must answer a concrete new integration or correction question and stay inside AC's owned harness. Do not make a complete new reference campaign the first delivery.

The integrated engineering gates use the existing M1b absolute **1e-12 seconds** budget for BAT/BBAT and timing-response differences, converting MJD differences to seconds with extended arithmetic. Preserve all existing direct typed/structural/numerical contracts, numerical-class checks, source selector/order/lifetime behavior and the explicit M2 exact-order gate; no blanket new tolerance or backend switch is introduced. Freeze component comparison projections before seeing Rust outputs. Localize and report any new unexplained numerical discrepancy for central disposition while continuing independent implementation. A good final BAT alone does not justify wrong units, frames, input selection or compensating delay errors.

Deliver executable examples that run from authored PAR/TIM files through the public library/CLI, without harness-only seeded physical state: ordinary terrestrial data at more than PKS, aliases and mixed barycentric/terrestrial rows; ordered multiple pulsars; proper motion/parallax/ecliptic and unit/time selectors; INCLUDE/flags/time/error/selection behavior; DM/CM/DMX/DMMODEL and solar variants; and reference-phase/secular paths. Ensure all required fixture/runtime metadata is explicit. Retain the existing A2 isolated and terrestrial workflows as integration regressions. Include repeated evaluation and missing/bad-resource failure followed by valid processing, no stale successful output, debug/release agreement, formatting and Clippy. Fits/residuals outside their old subset remain explicitly unsupported until their own stages.

Report which scientific combinations actually ran and which optional/backend/platform/plugin paths remain open. A skipped difficult branch must stay on the source inventory; no routine-count or parser-acceptance claim can substitute for a working input-to-BAT example. Do not claim CALCEPH-enabled, full residual/binary/fitting, performance, concurrency or distribution validation from this milestone.

## Ownership and release

After explicit release, AC will own `rust/` and new `validation/input_to_bat/` as sole Rust writer. It may read every accepted reference, but may not edit their runners, immutable membership, or local accepted artifacts. Use owned output routing and exact original/staged identities. Central owns decisions, this handoff and milestone acceptance. Do not freeze evolving central or unrelated assignment files.

Deliver `rust/INPUT_TO_BAT_REPORT.md`, updated source correspondence/unsafe/behavior maps, concrete API/CLI examples, actual verification identities and limitations. Commit explicit authored sources, fixtures/generators, contracts and concise reports with Conventional Commits. All reproducible JSON/JSONL, compressed traces, binary fixtures and large validation products remain local and ignored under 0039. No implementation chat is created by this preparation; AA must finish and be accepted first.
