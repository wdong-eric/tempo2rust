# Assignment AJ: public Rust APIs and predictor C compatibility

**Prepared only; not released.** Follow the user's interface choice and decision 0084 after AI's reviewed handover, ownership closure and explicit central release under 0074/0082. Consult HANDOFF.md for current Rust ownership; this prepared brief grants no ownership. Work in `/Users/Eric.Dong/Downloads/tempo2rust`; read HANDOFF.md, decisions 0084/0082/0074/0065/0053/0039/0050 and the actual predecessor baseline/reports named at release.

Once released, own `rust/` and new `validation/library_interfaces/` as the sole Rust writer. You are not alone in this repository: preserve other chats' changes, central files, canonical TEMPO2 and accepted evidence. Central retains shared design, numerical policy and milestone acceptance. Preserve original 0008 and its amendments.

Deliver full external Rust access to the transferred core/toolkit APIs, a standalone source-compatible predictor C interface and the installed getPeriod-equivalent client. The user chose Rust APIs for the general core/toolkit; no legacy pulsar-layout C bridge is required. Preserve full scientific function scope and source correspondence. Reuse the canonical engines and owners; a native shim may marshal predictor ABI types but may not duplicate scientific algorithms.

Proceed in dependency order:

1. Freeze the complete public source/API/symbol/type/ownership map, actual Rust interfaces, native ABI profile, error mapping, minimal build targets and external consumer recipes against the released baseline.
2. Complete and qualify external Rust core/toolkit consumers using canonical public types, including complete direct APIs and representative integrated workflows. Expose existing capabilities coherently and retain meaningful ownership invariants.
3. Add the complete predictor C boundary, headers and required thin native type shim. Preserve native long-double and exposed struct semantics, callbacks/streams, borrowed-pointer lifetimes, global out-of-range state and source error/unsupported dispositions. Explain every necessary unsafe boundary locally.
4. Implement and install the source getPeriod workflow with documented small initialization/error fixes. Reuse predictors, parsers and formatters; preserve defined native calculations and text.
5. Verify separately compiled C/C++ clients and external Cargo clients from an isolated install prefix, actual artifact loading, native/Rust values/state/file exchange, ABI/layout and ownership histories, failure recovery, affected debug/release checks, formatting and Clippy.
6. Commit explicit authored owned paths with Conventional Commits and return `rust/LIBRARY_INTERFACES_REPORT.md`, executed recipes, exact identities and the complete source/unsafe/departure and open-gate inventory. Keep generated artifacts ignored.

Full scientific migration, operational/platform qualification and inherited numerical gates remain separate obligations. New consequential questions go to central for the 0082 user checkpoint; continue independent work while a choice is pending.

## Dispatch prompt after release

Implement Assignment AJ under decisions/0084-library-interfaces-and-predictor-c-contract.md, decisions/0074-implementation-handover-and-acceptance.md and handoffs/36-library-interfaces-and-predictor-c.md from the baseline explicitly released by central. Own rust/ and validation/library_interfaces/ as sole Rust writer. Deliver the user's selected full Rust core/toolkit API guarantee, complete standalone predictor C compatibility and getPeriod-equivalent client using existing scientific implementations. Qualify external Rust/C/C++ callers and actual ABI/ownership/file behavior, preserve source correspondence and numerical gates, apply 0053 to demonstrated small bugs, explain necessary unsafe, commit authored files only and return rust/LIBRARY_INTERFACES_REPORT.md for central's 0082 checkpoint.
