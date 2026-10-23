# Measured coverage and limits

Separate native O0/gcov execution, with O2 caller/exporter. Counts include compiler exception/cleanup edges. The numerical reference uses O2 throughout. See coverage.json for the actual build/runner/runtime identity and per-case counters.

| Unit | Functions | Lines | Compiler edges |
|---|---:|---:|---:|
| preProcess.C | 6/6 | 514/521 | 710/826 |
| preProcessSimple.C | 4/4 | 244/244 | 303/328 |
| units.C | 3/3 | 62/62 | 14/14 |
| updateEpoch.C | 2/2 | 93/93 | 78/84 |

Instrumented comparisons: 302 exact, 0 different. Any differences are retained in full in coverage.json.

Every unhit edge is recorded individually in coverage_classification.json, with source text, location, index and disposition. coverage_cfg.json.gz preserves fresh compiler CFG dumps supporting the untagged strcasecmp cleanup edges; these are distinct from source comparison outcomes. Source conditional-to-line attachment is not an assertion that one source condition equals one compiler edge.

- compiler exception control/unrun: 122 edges.
- defined but unexercised: 2 edges.
- defined prefix not exercised; unsafe continuation withheld: 1 edges.
- source-inspected/unrun: 17 edges.
- structurally unreachable: 5 edges.

The seven unhit primary executable lines are plugin failure diagnostics withheld under the documented dlerror/null boundary. The source-literal-disabled binary epoch block has no executable coverage requirement and remains in the static inventory.

Defined but unexercised edges remain unfinished coverage; accepted M logger evidence does not turn them into executed P branches. These bounded witnesses are not exhaustive input/combinatorial coverage, another-platform evidence, sanitizer results or scientific accuracy validation.
