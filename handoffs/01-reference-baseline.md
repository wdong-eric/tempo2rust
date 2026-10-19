# TEMPO2 reference baseline

Assignment A was delivered at reference-repository commit `5289797` and reviewed centrally. This is the historical initial brief. The current follow-up is [A2](/Users/Eric.Dong/Downloads/tempo2-rust/handoffs/01a-reference-correction.md). Read the [shared handoff](/Users/Eric.Dong/Downloads/tempo2-rust/HANDOFF.md) for current decisions.

## Ownership

Write only within `/Users/Eric.Dong/Downloads/tempo2-rust/validation/reference/`. Own the reference build instructions, exporter, fixture manifests, comparison utilities needed by the reference work, and `REPORT.md`. Other chats share this workspace; preserve their edits. Read the canonical TEMPO2 checkout without changing it. If bootstrap or configure writes into the source, use a disposable source copy inside your owned directory and record its origin and any patches.

## Sources to inspect

- [Build definition](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/Makefile.am:42) and [configuration](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/configure.ac:66).
- [Precision abstractions](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/TKlongdouble.ld.h:13), [float128 abstraction](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/TKlongdouble.float128.h:9), and [formatting helper](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/TKlongdouble.C:39).
- [Barycentric residual tests](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/tests/testTempo2.C:360), [observatory residual tests](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/tests/testTempo2.C:427), and [end-to-end tests](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/tests/testEndToEnd.C:7).
- [Calculation sequence](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/global.C:134) and [residual implementation](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/formResiduals.C:396).

## Work sequence

1. Confirm the pinned commit and working-tree state. Identify available compilers, numerical dependencies, and existing installations. Do not use an installed executable as the reference solely because it is on PATH.
2. Build the reference separately from the canonical checkout. Record compiler versions, architecture, flags, precision backend, linked libraries, and effective significand precision. Use a configuration with demonstrated precision for the claimed timing checks. A binary64-only build can expose a limitation but cannot certify nanosecond agreement.
3. Run the narrow original tests relevant to the selected fixtures. Record pass, failure, or disabled status. Keep upstream failures visible. Build-only adjustments must be reproducible and separate from scientific changes.
4. Build a small exporter around existing TEMPO2 entry points. Run it in a separate process with an explicit runtime data location. Reuse existing interfaces; add the minimum instrumentation needed to localize numerical differences.
5. Export the fixtures below, record resolved configuration and resource provenance, and define a proposed comparison schema and tolerances. Verify regeneration and precision-preserving serialization with meaningful assertions.
6. Write `REPORT.md` and recommend the reference configuration and fixture contract for central adoption.

## Initial fixtures

Start with `/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/tests/test_data/test1.par` paired with `test1.tim` in the same directory. Then use the same parameter file with `test2.tim` for the terrestrial path. The existing test driver defines the invocation and preprocessing sequence; inspect it instead of assuming all tests call the normal CLI pipeline identically.

Retain the original input files. If explicit configuration or a smaller fixture is needed, create a named derivative in your owned directory and document every change and its reason. Add a small long-span spin case and the existing 4 ns perturbation case where needed for precision evaluation. Use the bundled `T2runtime/example_data/example1.par` and `.tim` as a later smoke example if the minimal fixtures are established.

For each run, record code identity, executable identity, compiler and library configuration, input hashes, all resource files actually used and their hashes, time scale, clock realization, ephemeris, relevant model settings, phase reference, selections, mean-removal setting, and TOA identities. Record applicable resource coverage and errors; do not download or update runtime data silently.

Export parsed model values and flags, original TOA values, and initialized calculation results at useful stages. Candidate quantities include clock corrections, time conversions, individual delays, BAT and BBAT, phase, pulse reference/count, and the fixed-parameter residual. Define each field's meaning, units, initialization point, and availability. In particular, verify the lifecycle of any field named prefit before treating it as an actual no-fit residual.

Use enough decimal digits for the selected backend. Encode precision-sensitive values as decimal text or another lossless, documented representation. Comparisons must not first coerce those values to binary64. Identity and pulse-count comparisons need their own exact semantics. The output format should be simple and consumable by the later Rust tests.

## Existing checks requiring care

- The default `print_longdouble` helper uses `%Lg` or `%Qg` without an explicit precision. Do not use that output unexamined as a golden numerical serialization.
- `subtractBATs` tests a 4 ns residual response in seconds. Preserve that physical contract with an explicitly justified tolerance.
- In [offsetSATs](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/tests/testTempo2.C:486), the difference is expressed in days while `TEST_DELTA` is the shared value `1e-9`. The expected offset is already inside the absolute value; the original brief misstated that detail. The threshold is nevertheless 86.4 microseconds, not 1 ns. Express the new diagnostic in seconds and account for the terrestrial transformation's response.
- [gracefulBadFit](/Users/Eric.Dong/Downloads/tempo2-rust/tempo2/tests/testEndToEnd.C:171) ends with a tautological assertion. It is not evidence of a correct error status or diagnostic.
- Some macOS compiler/OpenMP combinations disable the upstream tests. A disabled test is not a pass.

## Completion criteria

Deliver executable build/export instructions, a manifest, reproducible machine-readable outputs for the declared available fixtures, explicit comparison checks, and the report. Localize reference failures by stage. If sufficient reference precision or a required build environment is unavailable, complete the source/configuration diagnosis and independently useful artifacts, then report the specific missing capability. Do not replace failed reference values with guessed results or claim M0 is complete.

Use the shared return-report format. Include the proposed fixture schema, per-quantity units and tolerances, baseline limitations, and decisions the central chat must make. Do not port the Rust engine in this assignment.
