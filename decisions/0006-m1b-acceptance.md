# M1b acceptance

Central decision, 20 October 2026. **M1b is accepted for the declared macOS arm64 isolated-pulsar PKS path. Assignment D is complete.** Both blocking findings in [decision 0005](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0005-m1b-review-and-resource-correction.md) are closed by the reviewed correction and passing debug/release recovery checks. M0 and M1a remain accepted. M2 requires a separate fitting contract and is not released by this decision.

The [adopted M1b contract](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0004-m1b-contract-and-d-release.md) remains unchanged. Acceptance preserves TEMPO2's valid scientific expressions, operation order, numerical boundaries, meteorology fallbacks and comparison tolerances.

## Accepted delivery identity

The accepted delivery is the corrected working tree described in [M1B_REPORT.md](/Users/Eric.Dong/Downloads/tempo2rust/rust/M1B_REPORT.md), including its decision 0005 correction section. There is no separate D commit. Project HEAD remains the M1a base `49e122c3648e5cb91bae17233a062a444a10f9e4`; the following artifact hashes identify the accepted D delivery.

**Subsequent Git record, 20 October 2026:** At the user's request, this same delivery was committed in stages as `3f08a8c` (implementation), `31d55bf` (validation) and `87f3933` (design/acceptance documentation). The hashes below remain unchanged. The paragraphs describing the working tree/index retain the state at the original acceptance review. [Decision 0007](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0007-m2-fitting-contract-and-e-release.md) subsequently adopts M2's contract and releases E.

| Artifact | SHA-256 |
| --- | --- |
| `rust/M1B_REPORT.md` | `6ce3be793fef035361075238d6f291568c0a86613b40d3e39715b743514d4b62` |
| `rust/verification/m1b/source_hashes.json` | `66f6ff8dda2a5ee3a8f88da914755e88a8787b16b21e21fbd2b8a9f8def05a2a` |
| Retained `rust/native/jpleph.c` | `d8ae1becbab4585af6b2b76abbeac1599fe17f9a452ac3f4d7da694eb930d88d` |

All 96 source-manifest entries match the live files. Relative to the previously reviewed manifest, the correction changes only `verify_m1b.py`, `src/ifteph.rs`, `src/read_ephemeris.rs`, `src/resource_tests.rs` and `examples/check_terrestrial_resources.rs`. The report, README and verification records explain and identify those changes. Native source is unchanged from the reviewed delivery.

All 80 pinned runtime resources and both immutable terrestrial goldens retain their accepted hashes. Canonical TEMPO2 remains clean at `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; the independent A2 repository remains clean at `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`. The two fixture identities remain:

| Fixture | SHA-256 |
| --- | --- |
| `terrestrial` | `c19aac714e38521c5dd309dbf50f815f49430171430206d7af969fc790043d83` |
| `terrestrial_4ns` | `5455b5dc4d387ec8bea42a43665dffe0b3d7a39ced47e0f131395d003adb62eb` |

## Closure of the two findings

The central review inspected the complete correction, its callers, adjacent explanations, focused tests and verification changes.

The DE405 wrapper now validates a finite positive integral record count and bounds the final native seek offset against both the native `u32` multiplication and the platform's C `long` limit before native entry. It rejects nonfinite query epochs before the FFI. Valid interpolation and boundary selection remain in the unchanged native reader. Short files still exercise its checked read-error path and the approved stale-cache correction.

The IF99 loader validates its record grid against the complete records in the owned image. Evaluation bounds floating record/subinterval values before conversion and uses checked index and coefficient-offset arithmetic. The original interpolation expressions and final-epoch selection are retained. The new comments identify the demonstrated overflow and the intended returned-error behavior.

Both exact pinned-file reproducers from decision 0005 now return contextual constructor errors in debug and release:

| Mutated resource | Previous outcome | Corrected outcome |
| --- | --- | --- |
| DE405 step at byte 2668 changed from `32.0` to `1e-300` | Native SIGBUS | `JPL: invalid JPL record grid or native seek offset` |
| IF99 step at byte 280 changed from `32.0` to `1e-300` | Integer-overflow panic | `IF99: invalid IF99 record grid for available data` |

For each case the CLI exits 1 with empty stdout. In the same library host process, restoring the copied resource and constructing a new valid context reproduces the complete baseline result. Restored CLI stdout is byte-identical. No panic-catching, clipping, replacement resources or subprocess workaround was introduced into the library.

## Verification and numerical agreement

Central checks rerun for this acceptance:

- All eight resource unit tests pass in debug and all eight pass in release, including both new grid regressions, valid endpoints and the existing repeated-short-read/cache-recovery regression.
- The public resource checker passes in both modes using disposable runtime copies. It directly exercises the two malformed-step library cases and same-process recovery, corresponding CLI failures/restoration, missing/malformed resources, valid/missing/zero weather, invalid geometry, nonfinite propagation, late failure and unchanged inputs.
- Both terrestrial comparisons pass: 490 and 1162 scalar measurements, with the separate exact configuration, resource, clock-destination, identity, flag and pulse-count checks. All 184 comparator mutations pass. Recomputed signed errors and the 4 ns response agree with the saved evidence.
- All ten before/after output hashes in [resource_correction_equivalence.json](/Users/Eric.Dong/Downloads/tempo2rust/rust/verification/m1b/resource_correction_equivalence.json) match the live artifacts. The previously reviewed terrestrial exports, error summaries, complete result, recovery record and three M1a projections are byte-identical.

D's complete verification records also pass all 18 Rust tests, unchanged M1a comparisons, formatting and Clippy. Its [release equivalence record](/Users/Eric.Dong/Downloads/tempo2rust/rust/verification/m1b/release_equivalence.json) records four terrestrial/result/recovery exports identical between debug and release. Central review inspected those records against matching source hashes; it reran the focused checks above rather than overwriting D's evidence with another full verifier run.

The central resource-check harness needed canonical temporary paths because its error-path assertions compare paths literally. An initial invocation through macOS's `/var` alias stopped at that test assertion; reruns used the resolved `/private/var` path. A preliminary debug run exceeded the review harness's 50-second limit; it passed when allowed 180 seconds. These were review-harness conditions, not failures of the corrected returned-error behavior.

| Quantity | Maximum absolute difference | Unchanged gate |
| --- | ---: | ---: |
| BAT | `5.684341885448086137600e-14 s` | `1e-12 s` |
| Residual | `1.13686836941064559950026880243e-13 s` | `1e-12 s` |
| 4 ns response versus the local BAT derivative | `1.416875336045782805627568e-13 s` | `1e-12 s` |

The independent phase, vector, frequency, parameter and exact-count gates also remain satisfied. These measurements establish compatibility for the pinned cases; the atmospheric fallback does not establish historical meteorological accuracy.

## Accepted behavior and scope

The library's `load`, explicit `RuntimeContext` and `evaluate_terrestrial` operations and the runtime-enabled residuals CLI implement the adopted homogeneous PKS FORMAT 1 subset. The context owns resource state; results appear only after successful evaluation. The resource-free M1a operation remains compatible.

The accepted isolated F0–F3 path uses the pinned DE405/IF99/TT(TAI) configuration, TCB model/BAT/BBAT, IAU2000B, frequency dilation, solar and planetary Shapiro, interstellar dispersion, the adopted solar-wind model and troposphere. Input order, first-TOA phase reference, no fitting and no mean subtraction remain as adopted. PKS is accepted case-insensitively; aliases and mixed barycentric/terrestrial files remain outside this profile.

Source meteorology selection and zero/missing/out-of-range fallbacks remain `101.325 kPa` pressure and zero zenith wet delay, with warnings and effective-value provenance. The documented ownership, initialization, recoverable-error and invalid-access departures are accepted, including the demonstrated JPL stale-cache correction and the two record-grid guards. Scientific expressions and valid-data outputs are unchanged.

Linux and other platforms, additional observatories/models, binary timing/DDH, fitting, performance, concurrency stress testing and distribution packaging remain outside verified scope. The original DDH ideal-fixture failure remains open under its unchanged 1 ns gate. This acceptance does not establish exhaustive malformed-file coverage.

## Git state and next milestone

This acceptance changes central planning documents only. The primary repository contains staged delivery/planning files and unstaged corrections; the staged index is preserved. Its review-entry digest, SHA-256 of `git diff --cached --binary`, is `e92860204ff8a3c102cd6c3ceb3074d940c81d7a760d1fe96d671004907acee2`. No implementation source, report, reference artifact, stage or commit is changed by the acceptance update.

Before the next implementation assignment, the central chat must adopt M2's selected parameters and derivatives, weighted-fit conventions and iteration behavior, reference fixtures, design-matrix/update/covariance/residual gates, and invalid-fit behavior. Follow TEMPO2's existing decisions unless concrete evidence justifies a locally documented correction. M1a and M1b remain regression requirements for M2.
