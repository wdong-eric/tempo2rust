# M0 acceptance and Assignment C release

Central decision, 19 October 2026. **M0 is accepted for the declared macOS arm64 reference and initial barycentric path. Assignment C is released for implementation.** The numerical representation, scientific conventions, tolerances, library/CLI interface, and Git layout in [decision 0001](/Users/Eric.Dong/Downloads/tempo2-rust/decisions/0001-reference-and-m1a-contract.md) remain in force. M1a and M1b are not accepted.

## Accepted reference identity

Accept A2 at independent reference-repository revision `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`. The canonical TEMPO2 checkout remains clean and unchanged. The reference is the pinned upstream source plus the approved initializer correction and unchanged instrumentation, built with the adopted GCC 16.2.0 binary128 configuration.

| Component | SHA-256 or revision |
| --- | --- |
| Canonical TEMPO2 base commit | `fda6ad4a9753ad7dc5d291406f2f310a9161423f` |
| Initializer correction | `9d86ce0571e33aa28d0167137f37ee0afe524579a7b5e16dc81fee0d033c7b1c` |
| Instrumentation | `32cc90af950a01d49f9faacf80f1edeb4d393ecbec477aa0bd19076a0a3479ed` |
| Build manifest | `a5daa986c6c054c7fa314271b5fa918cedc3f7412707a9cc7670ec3306d92dc8` |
| Artifact manifest | `4f5ef112f977ec4a30b84bfb2e891748a5003198a0aa388ec8b45af74ff4c9ed` |
| A2 report | `ce9cc98f444ae770e1acbff167a713ff973f849e6f65e77cfec50ca146095880` |
| Schema 2 | `39de24bd60d7d33ea88a776771e0932fca1182913c3973b95a6f3708e9a46043` |
| Comparator | `67fc9cbcdc6e5185ca06789b1b2b638c51746b774864b5044e1715213d476679` |

The [A2 report](/Users/Eric.Dong/Downloads/tempo2-rust/validation/reference/REPORT.md) and reference manifests intentionally retain their submission-time statement that M0 awaits central review. This decision records the subsequent acceptance; do not edit the accepted reference snapshot merely to replace that historical status.

## Acceptance evidence

The central review verified all 26 manifest-listed artifact hashes, five built executable hashes, four linked dependency hashes, and 87 distinct input/resource file hashes across the six exports. The disposable `initialise.C` matches the pinned source plus the approved initialization line and recorded instrumentation. Both upstream test sources match the pin, and the instrumentation patch matches A at `5289797` byte for byte.

The regression source and recorded results establish that `ne_sw_ifuncN` becomes zero before parsing, stays zero with absent samples, and becomes two with the supplied samples. Their indices, values, uncertainties, and active delay contribution survive preprocessing and evaluation. The recorded terrestrial poison checks address the corrected collection, without claiming a general audit of legacy memory safety.

The corrected build's recorded checks pass the 113-bit precision probes, all four residual/BAT tests, regeneration, publication failures, and the analytic barycentric diagnostics. `gracefulBadFit` remains excluded because its final assertion is tautological. The central review independently compared every exported parameter, stage, and original observation with A's committed outputs: all six retain exactly the same scientific values. Scientific configuration also matches after removing the separately identified engine metadata.

The central review exercised the comparator in memory: all three accepted fixtures compare successfully; a reduced independent-engine projection passes; half-budget and double-budget perturbations give the expected outcomes for seven scalar quantities; altered full/reduced pulse counts fail; terrestrial and smoke-only exports remain rejected after availability relabeling. These are checks of the comparison contract, not a Rust timing implementation.

The central review did not rebuild or rerun TEMPO2 calculations. It inspected the regression/test implementation and recorded execution evidence, checked current artifact identity, and reran the read-only comparisons above. This is sufficient for this gate. The [B report](/Users/Eric.Dong/Downloads/tempo2-rust/validation/precision/REPORT.md) still has the previously reviewed hash `fb6d91a8f25d9482195c70eabebdcbb8c4d4243bd270128f0e70a2f89ebbee5e`. Exact preservation of the barycentric values means its backend decision does not need another experiment for A2.

## Fixtures and comparison projection accepted for C

Adopt [schema 2](/Users/Eric.Dong/Downloads/tempo2-rust/validation/reference/schema.json), its `barycentric-v1` projection, and the associated [comparator](/Users/Eric.Dong/Downloads/tempo2-rust/validation/reference/compare.py) for these fixtures only:

| Fixture | Required stages | Export SHA-256 |
| --- | --- | --- |
| `barycentric` | parsed, bats, residuals | `a9bac02d3ebeb4a65935f208b6e1ca8a58362d604d66a1fb55b9ea2f788a675e` |
| `barycentric_4ns` | parsed, direct_initial, aligned, perturbed | `44a2dfbbb294e932fc66657f2ca8b47f866d17bb26b337c11e6ebe8786e4d9af` |
| `long_span` | parsed, bats, residuals | `d62cc4cfd2a702a255f0c0d3b392c4cf9f93a2ba10feb84964feabca14069b44` |

The three fixtures use `test1.par`/`test1.tim` and `long_span.par`/`long_span.tim` as recorded in the manifest. Preserve original order and decimal tokens. `@` SAT, BAT, and BBAT are TCB for this accepted path. Observatory-clock SAT is separately described and is outside C's support. Keep the first eligible TOA's fractional phase reference, no fit, and no mean subtraction.

The projection compares scientific configuration, parameter presence/fit flags, parsed values, ordered input identity, and defined stage outputs. Engine provenance remains truthful and separate. C need not copy allocation diagnostics, legacy labels, unavailable fields, or unused resource reads. The comparator is a fixture adapter, not validation of arbitrary PAR/TIM input; C must implement and test its own declared input contract.

The adopted gates remain exact identity/flags/pulse counts, at most `1e-12 s` epoch and residual differences, at most `1e-10 turns` full-phase difference, and the parsed-value gates in decision 0001. Enforce the phase, residual, and exact-count checks independently. Implement the aligned and perturbed scenarios through the test adapter while preserving original input identity; do not add scenario modes to the public CLI.

Copy the needed input files, three goldens, schema/comparator, and a compact provenance record into C's test tree. Record the source revision and hashes. Keep them unchanged there unless a central decision revises the contract. Tests must run without the independent reference repository or a live TEMPO2 process. A2's terrestrial outputs and bundled example are not M1a goldens.

## Open issues outside this acceptance

- **DDH/DE414 ideal fixture:** preserve the original `abs(residual) < 1e-9 s` assertion as failing. Of 284 TOAs, 120 fail; the first is index 35 at `1.13758502732523367544302697283684274e-9 s`, and the largest absolute residual is index 38 at `4.59895516385995339247551543745087149e-9 s`. The central review recomputed these summaries from all saved residuals. Decimal replay supports downstream spin arithmetic given exported BBAT/binary delay; it does not independently validate those inputs or locate the cause. This remains broader coverage work. Reopen the relevant acceptance if later evidence implicates a shared component.
- **Terrestrial path:** initialized, stable exports are compatibility candidates. Stage budgets and treatment of absent meteorology still require a separate M1b decision. The declared 101.325 kPa and zero wet-delay defaults are not validated historical weather. M0 acceptance does not approve silent production fallback or release D.
- **Platform and build coverage:** execution evidence covers macOS arm64 only. Linux, the full upstream suite, and an upstream autotools build remain untested. The library's same-process error recovery is a C requirement; the reference demonstrated process isolation instead.

## Release and next return

Proceed with [Assignment C](/Users/Eric.Dong/Downloads/tempo2-rust/handoffs/03-barycentric-core.md): one Rust library and CLI, Rug `Float` at 128-bit significand, checked pulse counts, source-corresponding phase arithmetic, strict supported-input handling, and the accepted comparisons. The root Git bootstrap is now permitted under decision 0001. Preserve the independent reference repositories and other chats' changes.

C returns `rust/M1A_REPORT.md`, supported-parameter/directive tables, source correspondence, reproducible commands, and measured errors. The central chat accepts M1a before D begins. This release authorizes the implementation assignment; it does not create or message a chat.
