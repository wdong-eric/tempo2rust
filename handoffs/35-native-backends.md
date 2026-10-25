# Assignment AI: enabled ephemeris and numerical backends

**Prepared only; not released.** Follow decision 0081 after AH implementation handover, ownership closure and explicit central release under 0074. AE currently owns Rust under 0079; AF, AG and AH remain prepared. Work in `/Users/Eric.Dong/Downloads/tempo2rust`. Read HANDOFF.md, decisions 0081/0074/0065/0053/0039/0050, predecessor reports and the exact released baseline.

Once released, own `rust/` and new `validation/native_backends/` as the sole Rust writer. You are not alone in this repository: preserve other chats' changes, central files, canonical TEMPO2 and accepted evidence. Central retains shared design, numerical policy and milestone acceptance. Preserve original 0008 and its later amendments.

Deliver real CALCEPH ephemeris/time processing and enabled BLAS/LAPACK matrix, Cholesky and QR routes through the canonical public timing/fitting/native-style CLI. Complete the scoped direct routines, including unused source `accel_uinv`, and retain disabled profiles. Reuse prior implementations; keep native bindings small and explain every necessary unsafe boundary beside it.

Proceed in dependency order:

1. Freeze the minimal build/provider/ABI configuration, actual canonical APIs, runnable inputs and new comparison quantities against the released predecessor. Record source configuration separately from runtime selection. Use matching providers for native and Rust evidence.
2. Complete owned CALCEPH calls and integrate source parser/global/time/ephemeris paths. Preserve source units, type boundaries and pass order; exercise real data containing TT-TDB as well as position data and provider failure/recovery.
3. Complete the five accelerated routines and connect shared matrix, whitening and fitting dispatch. Preserve layouts, mutations, optional outputs and QR-specific augmented-row chi-square/rescaling. Keep disabled fallback distinct from enabled failure.
4. Demonstrate and correct any small source defects under 0053 using original/corrected witnesses and adjacent explanations. Keep inherited numerical-policy findings open and continue independent work.
5. Run affected debug/release direct and public workflows, profile/ABI/selector checks, failures followed by valid work, disabled regressions, formatting and Clippy. Reuse existing evidence rather than replaying unchanged historical suites.
6. Commit explicit authored owned paths with Conventional Commits and return `rust/NATIVE_BACKENDS_REPORT.md`, executed recipes, source/type/unsafe/departure maps, identities and the open ledger. Keep generated evidence and dependency binaries ignored.

This stage does not accept the full migration. OpenMP/concurrency/performance, remaining SOFA dependency disposition, C interfaces, installation/platform qualification and inherited numerical gates retain their explicit owners in the central plan.

## Dispatch prompt after release

Implement Assignment AI under decisions/0081-native-backends-prepared-contract.md, decisions/0074-implementation-handover-and-acceptance.md and handoffs/35-native-backends.md from the baseline explicitly released by central. Own rust/ and validation/native_backends/ as sole Rust writer. Deliver complete enabled CALCEPH and BLAS/LAPACK source routines and integration through the canonical timing/fitting/CLI workflows, with matching native provider evidence and disabled-profile behavior. Preserve TEMPO2 conventions, apply 0053 to proven small bugs, explain source correspondence and necessary unsafe, retain inherited gates, commit authored paths only and return rust/NATIVE_BACKENDS_REPORT.md for central review.
