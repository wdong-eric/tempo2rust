# M2 review and bounded correction closure

Central review, 23 October 2026. Assignment E has delivered implementation and evidence, and its requested verifier/correspondence corrections are closed. **M2 is not accepted.** The exact raw singular-mode order requirement remains failing under the user's instruction recorded in decision 0012. Neither receiving this delivery nor releasing another independent stage waives that gate.

## Reviewed delivery

| Commit | Responsibility |
| --- | --- |
| `4ba0f208897fed5c5a46dc424d12e02bbc4b5526` | Independent fitting exporter, fixtures and comparator |
| `e7efeac326f9f198f987194b7a533c3156ec8135` | ZERO/F0–F3 fitting implementation and tests |
| `979b73c` | Initial final verification records and report, including the open order gate |
| `75b9e9390aeab6db1138580568e3a9a247663328` | Exact shared-artifact/cycle coverage checks, mutations and JPL alignment disclosure |
| `90b52bb185e51d4e8827a3625a8d8348abf00012` | Correct selected source/routine correspondence and IFTE comments; executable code unchanged |

The current [report](/Users/Eric.Dong/Downloads/tempo2rust/rust/M2_REPORT.md), [source map](/Users/Eric.Dong/Downloads/tempo2rust/rust/SOURCE_CORRESPONDENCE.md) and [behavior ledger](/Users/Eric.Dong/Downloads/tempo2rust/rust/BEHAVIOR_DIFFERENCES.md) identify the supported local fitting profile, source boundaries, adapters and remaining limitations. E's implementation ownership window is complete; no additional E edits are requested by this decision. A subsequent Rust-writing chat requires its own explicit central release and must preserve this recorded M2 state.

## Findings closed

The verifier previously used `zip` without checking shared-output cardinality. It now validates the exact shared-file set, array type and expected cycle count before comparison, and removes earlier generated shared files before each build. Central reran all 13 empty/missing/extra/type mutation checks; each was rejected, while complete one- and two-cycle controls passed. Final debug and release evidence each contains 29 shared cases and 34 shared solves. This closes the skipped-cycle finding; it is not a general proof of every verifier path.

The JPL call comments and ledger now explicitly distinguish U1's unenforced file-identity assumption from U2/B29's packed-AU alignment gap. These are accurate limitations, not proofs that the native boundary is safe. No native calculation/cache behavior changed. Decision 0011 assigns the reference preparation for replacing this boundary to H.

The source map and module headers now name the actual generated `TKlongdouble.h` and selected numeric implementation, the additional timing responsibilities fused into `form_bats.rs`, the relevant routine names and Rust-only glue. IFTE comments distinguish unchecked header reads from checked data reads that terminate on failure. Central inspected the complete 21-file Rust diff; these changes affect comments only. They do not close the underlying input/resource behavior departures or establish whole-source coverage.

## Verification and unresolved acceptance

Central checked all 94 numerical-run source hashes against `75b9e93`, the preserved numerical manifest hash, and all 98 post-review hashes against the live files. The post-review record identifies its comment-only scope separately from the executed numerical revision. Formatting passed in E's recorded check; numerical execution was not repeated solely for comments.

Central independently replayed every saved shared solve, timing case and callback through the current comparator. Each build has 50 comparison records and 10,666 numerical measurements. The resulting error list exactly matches the four recorded diagnostics: raw retained flags and the retained singular-value position in the dependent-column fixture, once each in debug and release. All remaining comparisons pass. E's source-bound execution evidence records 23 Rust tests, 17 CLI checks and 65 numerical comparator mutations per build, plus unchanged M1 checks. This central replay is an artifact/comparator review, not another Rust execution or a new platform test.

The production Rug128 spectrum has the opposite mode order from binary128 TEMPO2 for that rank-one matrix. Paired spectra, rank and fitted physical coordinates agree within the existing gates; decision 0012 deliberately keeps this insufficient for acceptance. The full verifier returns 1. No production sorting, precision change, tolerance change or reinterpretation of a failed gate is authorized here.

The current fitting profile remains bounded to ZERO plus selected local F0–F3, homogeneous `@`/PKS inputs and the declared MODE/NITS policies. Full fitting, binary fitting, other platforms, finite nonconvergence evidence, concurrency and the baseline fidelity ledger remain open in the overall transfer inventory. Further stages may advance independent source responsibilities, but M2 and full transfer acceptance remain central decisions.
