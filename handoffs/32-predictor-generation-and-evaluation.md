# Assignment AF: predictor generation and evaluation

**Prepared only; not released.** Follow decision 0076 after AE's implementation handover, ownership closure and an explicit central release under 0074. AE currently owns Rust under 0079; AD ownership is closed. A successor handover does not imply that inherited numerical gates are accepted.

Work in `/Users/Eric.Dong/Downloads/tempo2rust`. Read HANDOFF.md, decisions 0076/0074/0065/0053/0039/0050, the released AD/AE reports and the baseline named by central. Preserve original 0008 and apply its later amendments. Once released, own `rust/` and new `validation/predictors/` as sole Rust writer. You are not alone in the shared repository: preserve other chats' changes, central documents, canonical TEMPO2 and accepted reference artifacts.

Deliver the complete source-defined predictor functionality: PAR-only polyco/Chebyshev generation using synthesized TOAs and canonical BAT/uncentered residuals, both file formats and cross-reader exchange, phase/frequency evaluation, segment selection, frequency plans and all defined direct lifecycle/construction/test APIs. Use 0076's CLI routes and source responsibility map. Reuse existing parser, runtime, writer and numeric routines; no fabricated TIM input, automatic fit cycle, second scientific engine or independent reference-only stage. C ABI packaging and the remaining core inventory retain explicit later obligations.

Proceed in coherent stages:

1. Freeze concrete canonical API, direct versus CLI caller conventions, authored public examples, source/type map and quantity-by-quantity comparison contract against the released baseline.
2. Transfer polynomial/model/set and generic predictor evaluation, parsing/writing, ownership and plans. Preserve actual source numeric types, exact selection/status semantics and source-unimplemented T1 operations. Establish component evidence and safe failure/recovery.
3. Connect Chebyshev and polyco construction to canonical PAR/preprocessing/runtime/timing state. Preserve source artificial-TOA, TZR, sampling, overlap, coefficient and output conventions. Diagnose the named source initialization/format/bounds issues and correct demonstrated small bugs under 0053 with adjacent comments and separate original/corrected evidence.
4. Deliver public generation/evaluation/plan routes and all scoped file products. Compare native/Rust components and independent full workflows, both cross-reader directions, repeated histories and approximation quality separately. Keep precision/format loss distinguishable from timing-engine or implementation errors.
5. Run affected debug/release checks, regressions, formatting and Clippy; commit explicit authored owned paths with Conventional Commits. Return `rust/PREDICTOR_REPORT.md`, runnable examples, actual identities, maps and the complete unresolved ledger for central review. Keep generated outputs ignored.

Rug128 timing values, native-long-double boundaries and existing numerical gates remain as specified in 0076 and earlier decisions. Do not widen gates, alter source algorithms, silently change precision or turn a source approximation envelope into acceptance of a Rust discrepancy. Central owns broader decisions; continue independent implementation while a localized decision is pending.

## Dispatch prompt after release

Implement Assignment AF under decisions/0076-predictor-generation-and-evaluation-prepared-contract.md, decisions/0074-implementation-handover-and-acceptance.md and handoffs/32-predictor-generation-and-evaluation.md from the baseline explicitly released by central. Own rust/ and validation/predictors/ as sole Rust writer. Deliver complete source-defined polyco/Chebyshev generation, native file exchange, evaluation, lifecycle and frequency plans on the canonical timing engine. Preserve source naming, numeric/caller boundaries and all scope obligations; apply 0053 to demonstrated small bugs, explain necessary unsafe, preserve original/corrected evidence, commit authored paths only and return rust/PREDICTOR_REPORT.md for central review.
