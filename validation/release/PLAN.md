# AL frozen release qualification plan

27 October 2026. Preparation and local execution are authorized by 0098;
publication, push and workflow dispatch are not. Central owns scientific policy
and acceptance. This plan is not a qualification result.

## Candidate and inputs

| Component | Frozen starting identity / selection |
| --- | --- |
| Central release | `6acaf53d2e13291eb6d0b0988faf7dee7d33a635` |
| AK production / qualification / report | `81de1e6bb7d6f6cf1d17c075f402c301b87c9ce9` / `3926ae14375752a6ef9526e8141a2bad864097af` / `eaa30c593b1871cf93edfb4cf23e384a039f47ab` |
| Canonical source | `wdong-eric/tempo2`, `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; clean at inspection |
| Independent A2 | `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`; clean, no public remote; essential authored construction must be supplied here |
| Rust arithmetic | Rug 1.30.0, production 128 bits; committed Cargo.lock; no precision switch |
| Native timing | GCC `__float128` / quadmath, measured 113 bits; A2 initializer correction and resource tracing distinguished |
| Predictor | Host ordinary long double; macOS arm64 53-bit and Linux x86_64 64-bit branches require actual ABI/conversion measurement |
| CALCEPH | 4.0.5 mirror commit `718e7945ccd27216b522243c3ff8aab63ce1d99d`; source SHA256 `b38f6ccc726bab51c5901bb74c655e13941753ca7c3d0054396a37b793040426` |
| CALCEPH data | INPOP17a TT-TDB/positions SHA256 `32517ec82bfb298d53baeba4065750dec82143fab79357e4487f84fb439bc49c` |
| Runtime | Separate pinned `T2runtime`, per-file identities; historical DDH overlay only adds gps2utc.clk from `0759584ebf8315bbbe6a6d0b41c08ebbcfe527f2`, SHA256 `30727fdfaaec0f1775543804210d057ffe62076594bb8884d8f8ccafafb676be` |
| Retained dependency | The 36 source-identical SOFA routines and notices under 0083; PIC and recursive locals |
| Interfaces | Core CLI `tempo2-rust`; full public Rust source; predictor C11/C++17 static/shared 64-symbol contract; getPeriod-rust |

Actual local host: macOS 26.7 (25G229), Darwin 25.6.0 arm64; rustc 1.97.1
(`8bab26f4f68e0e26f0bb7960be334d5b520ea452`), Cargo 1.97.1. Compiler/provider
versions, lockfile, headers, link commands and loaded products are recorded in
each execution receipt, not inferred from this initial table.

Linux selection: actual `ubuntu-24.04` x64 GitHub runner, GCC/GFortran 13,
explicit OpenBLAS pthread LP64 and LAPACK headers. Record resolved package
versions, image identity, Fortran integer/character ABI and actual loaded paths.
Linux is **unrun** until the reviewed workflow is authorized and executed.

## Dependency-ordered work and observable evidence

1. **Owned release construction:** preserve A2 authored inputs with Git/blob/hash
   provenance; export pinned source and runtime into fresh output directories;
   build native timing/predictor tools without Homebrew assumptions. Pin CALCEPH
   source/data and historical clock bytes; record all transformations separately.
2. **Production build and install:** implement explicit Linux provider selection
   in `rust/build.rs` / `rust/native/backends.c`, preserve matrix/scientific
   semantics; PIC Fortran; install core CLI alongside existing products. Exercise
   clean exported source, fresh private prefixes, package membership/notices and
   outside-checkout clients. No canonical installation changes.
3. **Platform-sensitive qualification:** actual type/layout/argument/return
   probes; lossless predictor comparison, unchanged quantity gates and rejection
   controls; matched Lomb uniform/Nyquist/irregular and QR contraction witnesses.
   Never infer Linux fidelity through Python float or macOS measurements.
4. **Integrated profiles:** default debug/release; BLAS-only, LAPACK,
   CALCEPH-only, combined and independent qr-default selections. Reuse existing
   authored checks in isolated staging. Cover external Rust, C11/C++ static/shared,
   resources/missing-resource recovery, serial/parallel and complete-workflow
   caller ownership. Preserve every failure and record all ignored/unrun checks.
5. **Scientific matrix:** input/BAT, fourteen binaries/phase, general local/global/
   robust/noise fits, toolkit/GW/spectrum, predictors and output. Recheck P, M2,
   0049, AE, AF and AH ledgers without changing gates; paired current-clock DDH
   fidelity and historical pre/post-fit strict 1 ns checks. Broader observational
   validation remains distinct from generated fixture correspondence.
6. **Current closure:** reconcile the unchanged historical 93-TU inventory and
   stage body/branch maps with current functions/public consumers, retained
   dependencies, unsafe invariants and 0053 departures. Presence, execution and
   central acceptance are separate columns; missing evidence remains visible.
7. **Delivery:** exact action revisions, small manual workflow calling repository
   scripts, selected failure artifacts with explicit retention; fmt, Clippy,
   narrow harness controls and source/package audit. Commit only explicit authored
   owned paths. Report per-requirement pass/failure/unrun, exact identities and
   the full unresolved ledger in `rust/RELEASE_QUALIFICATION_REPORT.md`.

The completion criterion is evidence for every requirement on both actual
platforms, with central's numerical and publication decisions still explicit.
A green build, cross-compile, inherited receipt or workflow file alone does not
meet it. AK performance evidence is reused only for its recorded source/profile;
affected provider/linkage/platform measurements must be repeated without adding
a performance threshold or optimization workstream.

## 30 October follow-through

1. Fix the shallow workflow's missing baseline at its fetch boundary; distinguish
   unavailable evidence from actual modified/added/removed Rust bodies. Reproduce
   the negative case and the narrow-fetch recovery in new local checkouts.
2. Reconstruct P and required earlier native corpora in isolated staging from
   their authored fixtures and pinned source, with explicit portable build
   adaptations and fresh identities. Run unchanged native validations and Rust
   gates; keep original and corrected source boundaries separate.
3. Rebuild AK measured-worker and uninstrumented timing profiles against AL's
   fresh dependencies. Preserve all eleven worker cases, six performance
   workloads, worker counts, samples and instrumented/uninstrumented controls.
4. Bind source responsibilities to actual functions, branches/families, public
   consumers and executed evidence. Classify justified reused proof separately
   from concrete missing cases; lexical indexes cannot close coverage.
5. Integrate the successors, run affected local checks, preserve all failures and
   update the report with exact commands/identities and remaining Linux gaps.

Decision 0099 allows evidenced Rust implementation improvements within assigned
scope while preserving scientific meaning and required behavior. The covariance
allocation lead remains deferred until after AL; it is not implemented here.
No push or workflow dispatch is authorized.
