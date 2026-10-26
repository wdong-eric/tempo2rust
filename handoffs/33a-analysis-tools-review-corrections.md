# Assignment AG: focused review corrections

Central review, 26 October 2026, during AG implementation. AG remains the sole writer under 0089; central owns this brief and the review probe. This is a bounded review of preliminary evidence, not final acceptance or a claim that the implementation has finished. Continue independent AG work. Apply the existing contract and 0090; no new scientific choice or reference-only stage is introduced.

## 1. Enforce the frozen exact min/max comparisons

`validation/analysis_tools/CONTRACT.md` requires min/max to match exactly. The current native/toolkit public projections instead label `stat.min`/`stat.max` as `stat` and `float.min`/`float.max` as `f32`. Comparator `16828a73feadc6701633a9c54fa4064f8e34d33fee2b5ef0f8118641705b8c28` therefore grants a scaled arithmetic budget or two-ULP allowance to these exact quantities.

Central froze the actual preliminary native and Rust toolkit exports and replayed that comparator. The unchanged control passes 7,195 checks. Changing one field at a time by one representable step toward positive infinity produces four false acceptances, each with 7,195 checks and zero failures:

| Field | Precision | Original value | Mutated value accepted |
| --- | --- | --- | --- |
| `stat.min` | binary64 | -3 | -2.99999999999999956 |
| `stat.max` | binary64 | 5.25 | 5.25000000000000089 |
| `float.min` | binary32 | -3 | -2.99999976158142090 |
| `float.max` | binary32 | 5.25 | 5.25000047683715820 |

Correct the typed projection/comparison so those fields retain source-width exact values and signed/nonfinite classes. Do not widen or otherwise replace the frozen contract. Source arithmetic reductions keep their separately declared budgets. Preserve the original exports and observer. Rerun the affected unchanged toolkit comparison and prove that all four one-field mutations are rejected. A changed observer must be identified in the final report and its actual results; the preliminary passing count alone does not close this finding. This review demonstrates an observer defect, not incorrect Rust min/max calculations.

Authored replay: `decisions/evidence/analysis-tools/central-ag-comparator-review.py`. It accepts `--native`, `--rust` and `--comparator` paths and copies all inputs before execution. Defaults name the preliminary debug toolkit exports. If the corrected projection changes those fields' representation, adapt the focused mutation inputs to that representation and retain the original witness; a renamed field or tag must not merely bypass the four contract checks.

## Finalization checks observed during this review

These are completion checks against the existing scope, rather than demonstrated production defects:

- Include `TKcmonot` at pinned `TKspectrum.C:707` in the source/body map. Its Rust implementation and component caller exist, but the frozen map currently jumps from `TKhann` to `TKspline_interpolate`. Keep the complete source-body accounting requirement.
- Complete 0090's diagnostic contract: warnings identify the actual averaging/spline/Hann/first-or-second-difference transforms when a weighted estimator is reached, and public callers expose them. The current generic warning detects differencing, while the API paragraph still lists only resampling. Record the final warning behavior and its relevant checks.
- `components.h` currently emits literal `population.warnings = 6`. Treat that as an expected count, not a native observation. Capture/check actual native warning records/counts and the corresponding Rust diagnostics before claiming warning correspondence. Preserve raw logs and explicit original/corrected provenance.

## Evidence and return

Ignored review evidence: `decisions/evidence/analysis-tools/build/comparator-20261026/`, copied from `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-ag-comparator-review-4198hmd5`. Receipt SHA-256: `dae42f899caf4d5e2ab1b8db859cb2fe4b17f583edd6a46c06d9a50a497c931d`; preservation receipt: `0c779b96207a42b51db740e31fc81f903ecaee9c4a3407eaf298aa92216db39f`; authored probe: `e9d6e16f0adffd2477367021324e04e62a677fad1aa152be2dc8203cdb46280e`. The receipt hashes the frozen contract, comparator, original exports, four single-row mutants and individual comparison outputs.

Central executed only the observer replay; no timing/RNG engine, native build, full suite or Linux execution is claimed. Generated exports, logs and receipts remain ignored. Return the corrections and focused verification in the normal authored AG commits and `rust/ANALYSIS_TOOLS_REPORT.md`. Central will close the demonstrated finding against that evidence before implementation handover. The full functional scope and all inherited numerical gates remain unchanged.
