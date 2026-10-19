# Rust observatory timing implementation

Assignment D implements M1b. Read the [shared handoff](/Users/Eric.Dong/Downloads/tempo2-rust/HANDOFF.md), the accepted A and B reports, and C's `M1A_REPORT.md`. Begin after M1a acceptance and a central decision fixing the terrestrial fixture configuration, required corrections, retained numerical dependencies, and tolerances.

## Ownership

Own `/Users/Eric.Dong/Downloads/tempo2-rust/rust/` after C has finished, including resource-loading code, timing stages, tests, documentation, and `M1B_REPORT.md`. Other chats share the workspace; preserve their edits. Read the accepted reference artifacts without modifying them. A needed exporter or fixture-contract change returns to its owner and central review.

## Target behavior

Extend the existing core to one explicitly supported isolated-pulsar observatory path. Start from `test1.par` with `test2.tim`, using the configuration resolved by A. Include every correction active in that fixture. Any deliberately simpler case must be a separately named fixture whose changed assumptions are accepted centrally.

The target is a complete path from observatory TOAs through resource lookup, clock/time transformations, geometric and propagation delays, barycentric arrival times, and fixed-parameter phase/residuals. Preserve the original ordering and iteration where stages depend on one another. Retain selected established astronomical/numerical libraries behind narrow interfaces as specified by the central design.

## Work sequence

1. Map the selected fixture onto [formBatsAll](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/global.C:134), its clock/ephemeris/delay routines, [formBats](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/formBats.C:36), and the existing Rust residual stage. Record required resources and parameter dependencies before editing the pipeline.
2. Introduce the runtime context to own the required observatory definitions, clock resources, ephemerides, and other active data. The accepted barycentric interface does not require this context. Resolve resource provenance and validity ranges explicitly.
3. Port tightly related stages in dependency order and compare each stage against the accepted reference before integrating the next. Preserve existing binary64 versus extended-precision choices unless central evidence authorizes a change.
4. Integrate the terrestrial path with the existing library and CLI. Preserve TOA identity and the accepted phase/mean-removal conventions.
5. Exercise missing resources, unknown observatories, unsupported configurations, out-of-coverage data, and failure followed by successful processing. A resource error must name the relevant input and resource; it must not silently select a different ephemeris or clock realization.
6. Compare the complete fixed-parameter result and verify that the accepted M1a cases still pass. Add the bundled isolated-pulsar example only within the accepted support contract. Write the report and document supported use.

## Completion criteria

Deliver stage-level agreement for the declared correction chain and agreement in final phase/residuals for the accepted terrestrial fixtures, with per-quantity units and tolerances. Report both maximum and representative numerical differences while retaining a failing-TOA identifier for any mismatch. Demonstrate that a final result is exposed only after successful completion and that runtime provenance is available.

Core timing logic for the claimed path must execute in Rust, with retained numerical dependencies identified. Binary models and fitting follow in later milestones. Additional observatories or alternate time/ephemeris conventions are separate coverage claims requiring their own fixtures.

Save `M1B_REPORT.md` using the shared format. Include the original-to-Rust stage map, resource and dependency inventory, supported configurations, numerical comparisons, failure behavior, and any limits affecting the next fitting milestone. The central chat reviews the evidence and accepts M1b.
