# Assignment AG: toolkit, spectrum and GW APIs

**Released by decision 0089 on AF production `7420622`, validation `543760f` and report `7a94491`.** Central completed the 0074 handover review and 0082 checkpoint. AF ownership is closed; AG is the next sole Rust writer. Numerical/scientific acceptance remains pending. Read HANDOFF.md, decisions 0089/0088/0087/0086/0084/0082/0078/0074/0065/0053/0039/0050, the released AD/AE/AF reports and the actual central release commit. Preserve original 0008 and its later amendments.

Work in `/Users/Eric.Dong/Downloads/tempo2rust`. Own `rust/` and new `validation/analysis_tools/` as sole Rust writer. You are not alone in the shared repository: preserve other chats' changes, central files, canonical TEMPO2 and accepted reference evidence. Central retains design and acceptance.

Deliver every defined source toolkit/statistics/RNG, spectrum/sigma-z and GW simulation API in 0078, together with runnable public toolkit, real-timing spectrum and seeded-GW/file-exchange examples. Reuse AE's fitting/toolkit/spectrum dependencies and AD's eccentric closure. Complete the remaining standalone responsibilities without a duplicate solver, RNG, FFT, parameter store or timing engine. The accepted interface guarantee in 0084 is full Rust core/toolkit APIs plus standalone predictor C compatibility. AJ qualifies those installed external consumers; AG retains the full scientific function scope through safe Rust APIs. Actual macOS arm64 and Linux x86_64 release qualification remains required under 0086.

Proceed in coherent stages:

1. Freeze concrete public API/state ownership, executable input recipes, complete source/body/dispatch map, numeric/units map, file profile and quantity-specific comparison contract against the released baseline.
2. Complete reusable toolkit and RNG operations, preserving source draw order, state and numeric boundaries. Integrate all existing consumers through the canonical routines.
3. Complete spectrum/sigma-z drivers, helpers and callback/state lifecycle on the canonical timing/fitting engine. Resolve demonstrated row/backing/diagnostic defects under 0053; refer any broader resampling or scientific decision to central while proceeding independently.
4. Complete remaining GW geometry/population/polarization/residual/energy and native file operations. Preserve source constants, sampling conventions and type boundaries; correct demonstrated local initialization/assignment/I/O bugs with adjacent comments and separate original/corrected evidence.
5. Deliver public examples and complete component, pipeline, RNG-history, file-exchange and recovery checks in debug/release. Run affected regressions, formatting and Clippy. Preserve existing numerical gates and inherited unresolved findings.
6. Commit explicit authored owned paths with Conventional Commits and return `rust/ANALYSIS_TOOLS_REPORT.md`, runnable examples, exact identities, maps and unresolved scope for central review. Generated outputs stay ignored.

Inherit the complete 0088/0089 numerical ledger, including AF's seven exact raw polyco log10RMS failures. Shared-phase fits match, but the original failures remain unaccepted. Central owns their disposition; AG implements any later authorized correction. Reuse the existing native predictor number type only at matching source native-long-double boundaries; preserve f32/f64 and Rug128 timing boundaries.

Do not replace the implementation assignment with another reference-only campaign. Use focused native callers for concrete uncovered contracts and defects. A header declaration without a source body is not permission to invent a routine; a passing subset is not completion of a whole source file.

## Released dispatch prompt

Implement Assignment AG under decisions/0078-toolkit-spectrum-gw-prepared-contract.md, decisions/0074-implementation-handover-and-acceptance.md and handoffs/33-toolkit-spectrum-gw.md from central release 0089 on AF production 7420622 / validation 543760f / report 7a94491. Own rust/ and validation/analysis_tools/ as sole Rust writer. Deliver complete source-defined toolkit/RNG, spectrum/sigma-z and GW simulation APIs plus usable real-timing examples and native background-file exchange. Reuse existing canonical dependencies, preserve source notation/types/state and all scope obligations, apply 0053 to demonstrated small bugs, explain necessary unsafe, keep original/corrected evidence distinct, commit authored paths only and return rust/ANALYSIS_TOOLS_REPORT.md for central review.
