# Assignment AG: toolkit, spectrum and GW APIs

**Prepared only; not released.** Follow decision 0078 after AF's implementation handover, ownership closure and an explicit central release under 0074. AD currently owns Rust; AE and AF remain prepared. Read HANDOFF.md, decisions 0078/0074/0065/0053/0039/0050, the released AD/AE/AF reports and the actual baseline named by central. Preserve original 0008 and its later amendments.

Work in `/Users/Eric.Dong/Downloads/tempo2rust`. Once released, own `rust/` and new `validation/analysis_tools/` as sole Rust writer. You are not alone in the shared repository: preserve other chats' changes, central files, canonical TEMPO2 and accepted reference evidence. Central retains design and acceptance.

Deliver every defined source toolkit/statistics/RNG, spectrum/sigma-z and GW simulation API in 0078, together with runnable public toolkit, real-timing spectrum and seeded-GW/file-exchange examples. Reuse AE's fitting/toolkit/spectrum dependencies and AD's eccentric closure. Complete the remaining standalone responsibilities without a duplicate solver, RNG, FFT, parameter store or timing engine. Exported native ABI packaging and later core integration remain explicit obligations, not implicitly completed by Rust APIs.

Proceed in coherent stages:

1. Freeze concrete public API/state ownership, executable input recipes, complete source/body/dispatch map, numeric/units map, file profile and quantity-specific comparison contract against the released baseline.
2. Complete reusable toolkit and RNG operations, preserving source draw order, state and numeric boundaries. Integrate all existing consumers through the canonical routines.
3. Complete spectrum/sigma-z drivers, helpers and callback/state lifecycle on the canonical timing/fitting engine. Resolve demonstrated row/backing/diagnostic defects under 0053; refer any broader resampling or scientific decision to central while proceeding independently.
4. Complete remaining GW geometry/population/polarization/residual/energy and native file operations. Preserve source constants, sampling conventions and type boundaries; correct demonstrated local initialization/assignment/I/O bugs with adjacent comments and separate original/corrected evidence.
5. Deliver public examples and complete component, pipeline, RNG-history, file-exchange and recovery checks in debug/release. Run affected regressions, formatting and Clippy. Preserve existing numerical gates and inherited unresolved findings.
6. Commit explicit authored owned paths with Conventional Commits and return `rust/ANALYSIS_TOOLS_REPORT.md`, runnable examples, exact identities, maps and unresolved scope for central review. Generated outputs stay ignored.

Do not replace the implementation assignment with another reference-only campaign. Use focused native callers for concrete uncovered contracts and defects. A header declaration without a source body is not permission to invent a routine; a passing subset is not completion of a whole source file.

## Dispatch prompt after release

Implement Assignment AG under decisions/0078-toolkit-spectrum-gw-prepared-contract.md, decisions/0074-implementation-handover-and-acceptance.md and handoffs/33-toolkit-spectrum-gw.md from the baseline explicitly released by central. Own rust/ and validation/analysis_tools/ as sole Rust writer. Deliver complete source-defined toolkit/RNG, spectrum/sigma-z and GW simulation APIs plus usable real-timing examples and native background-file exchange. Reuse existing canonical dependencies, preserve source notation/types/state and all scope obligations, apply 0053 to demonstrated small bugs, explain necessary unsafe, keep original/corrected evidence distinct, commit authored paths only and return rust/ANALYSIS_TOOLS_REPORT.md for central review.
