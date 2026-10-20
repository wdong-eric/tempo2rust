# Rust observatory timing implementation

**Assignment D is complete and M1b is accepted under [decision 0006](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0006-m1b-acceptance.md)** for the declared macOS arm64 PKS path. [Decision 0004](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0004-m1b-contract-and-d-release.md) remains the adopted terrestrial contract; [decision 0003](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0003-m1a-acceptance.md) accepts M1a at `49e122c`. The assignment requirements below are retained as the implementation contract. Shared design changes and milestone acceptance remain with the central chat.

**Correction accepted, 20 October 2026:** Both findings from [decision 0005](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0005-m1b-review-and-resource-correction.md) are closed. Impossible DE405 grids are rejected before native evaluation, and IF99 record/index arithmetic is validated and checked. Contextual failures and recovery pass in debug and release, while the reviewed valid outputs remain byte-identical. Scientific expressions, meteorology fallbacks and tolerances remain unchanged.

## Ownership

Own `/Users/Eric.Dong/Downloads/tempo2rust/rust/` after C has finished, including resource-loading code, timing stages, tests, documentation, and `M1B_REPORT.md`. Other chats share the workspace; preserve their edits. Read the accepted reference artifacts without modifying them. A needed exporter or fixture-contract change returns to its owner and central review.

Preserve the existing staged central documents and the independent Git histories. Do not stage, commit, reset or overwrite another chat's work. The root Git bootstrap is already complete.

## Compatibility rule

The user directs us to follow existing TEMPO2 decisions unless they are wrong or prone to errors. Preserve the pinned algorithms, operation order, constants, defaults, selection rules and supported fallbacks. Any intentional behavioral difference must have a nearby code comment explaining the source behavior, its concrete problem and the intended change, with a focused regression and report entry. Material scientific corrections require central review; ordinary implementation choices inside the adopted contract do not.

In particular, retain TEMPO2's normal meteorology lookup and fallback: standard pressure `101.325 kPa` and zero zenith wet delay when the source selects them, including its zero-value behavior. Preserve source-equivalent warnings and record effective assumptions in provenance. Do not make missing weather fatal or require an opt-in compatibility mode.

## Target behavior

Extend the existing core to the explicitly supported isolated PKS path using `test1.par` with `test2.tim`. Decision 0004 accepts the hashes of A2's `terrestrial` and `terrestrial_4ns` exports and their DE405/IF99/TT(TAI)/TCB/IAU2000B configuration. Include every active correction: frequency dilation, solar/planetary Shapiro, interstellar dispersion, solar-wind dispersion and troposphere. Any deliberately simpler case must be a separately named fixture whose changed assumptions are accepted centrally.

The target is a complete path from observatory TOAs through resource lookup, clock/time transformations, geometric and propagation delays, barycentric arrival times, and fixed-parameter phase/residuals. Preserve the original ordering and iteration where stages depend on one another. Retain selected established astronomical/numerical libraries behind narrow interfaces as specified by the central design.

## Work sequence

1. Verify the accepted fixture/resource identities and map the selected calculation onto [formBatsAll](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/global.C:134), its clock/ephemeris/delay routines, [formBats](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/formBats.C:36), and the existing Rust residual stage. Record required resources and parameter dependencies before editing the pipeline. Copy the needed immutable reference data into a separate terrestrial test contract; preserve the M1a copies and comparator.
2. Introduce the runtime context to own the required observatory definitions, clock resources, ephemerides, and other active data. The accepted barycentric interface does not require this context. Resolve resource provenance and validity ranges explicitly.
3. Retain the pinned SOFA and low-level JPL numerical routines behind narrow owned interfaces. Port IF99 resource state/interpolation and the timing pipeline into Rust, preserving the source's initial/refined time and ephemeris passes. Compare tightly related stages before integrating the next. Preserve binary64 versus extended-precision choices; comment intentional changes to global-state ownership, process-exit handling or other behavior where implemented.
4. Integrate the terrestrial path with the existing library and CLI. Preserve TOA identity and the accepted phase/mean-removal conventions.
5. Exercise missing indispensable resources, malformed data, unknown observatories, unsupported configurations, native range/read errors and failure followed by successful processing. Separately test source-compatible meteorology fallback and warnings for absent/out-of-range/zero data, plus valid table interpolation. Preserve other source-defined fallbacks within the declared support; do not invent a blanket strict-resource policy or substitute a different ephemeris.
6. Enforce decision 0004's stage, phase/residual and exact-count gates, including the 4 ns response against the local BAT slope. Verify the accepted M1a cases still pass and document any newly supported inputs. The bundled example and broader configurations remain separate coverage work. Write the report and document supported use.

## Completion criteria

Deliver stage-level agreement for the declared correction chain and agreement in final phase/residuals for the accepted terrestrial fixtures, with per-quantity units and tolerances. Report both maximum and representative numerical differences while retaining a failing-TOA identifier for any mismatch. Demonstrate that a final result is exposed only after successful completion and that runtime provenance is available.

Core timing logic for the claimed path must execute in Rust, with retained numerical dependencies identified. Binary models and fitting follow in later milestones. Additional observatories or alternate time/ephemeris conventions are separate coverage claims requiring their own fixtures.

Save `M1B_REPORT.md` using the shared format. Include the original-to-Rust stage map, resource and dependency inventory, supported configurations, numerical comparisons, failure/fallback behavior, and any limits affecting the next fitting milestone. List every intentional behavioral departure with its rationale, evidence, adjacent comment location and regression. The central chat reviews the evidence and accepts M1b.
