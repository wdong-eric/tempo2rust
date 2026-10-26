# QR optional-output allocation and RHS correction

Central decision, 27 October 2026. **The prepared AI implementation must allocate its QR working matrix and copy a valid supplied RHS independently of whether fitted parameters are requested.** A controlled native-provider experiment now demonstrates the source defect and the narrow correction. Apply 0053 with adjacent pinned-source/cause/fix comments and focused regressions. AI remains unreleased; this changes neither AH ownership nor milestone acceptance.

## Source contract and demonstrated bug

Pinned `TKfit.C::TKrobustDefConstrainedLeastSquares` sets `computeParam = (outP != NULL && data != NULL)` at line 194. Its QR branch at lines 260–270 allocates `augmented_DM` and copies the supplied whitened data only inside `if (computeParam)`, then fills the matrix unconditionally. Thus a real caller providing data and requesting only covariance, errors or the returned statistic dereferences a null matrix. Independently, the low-level `T2accel.C::accel_lsq_qr` accepts an absent parameter-output pointer, still consumes the RHS, and returns its residual chi-square.

The correction in disposable source allocates the matrix and copies the real supplied whitened RHS before processing independent output requests. It changes no solver expression, source matrix layout, requested-output membership, constraint target, rescaling or parameter-producing calculation. Allocating only the matrix is insufficient because the solver also consumes the RHS. An absent RHS remains a separate input boundary; this decision does not authorize invented zero observations.

## Controlled native-provider evidence

The experiment compiles raw pinned `TKfit.C` and `T2accel.C` plus the narrow corrected TKfit variant, linking the immutable A2 archive and the host's existing Accelerate framework. No Rust timing/fitting implementation is used. The recorded SDK header and a compiled type probe establish the local legacy LP64 interface's four-byte integer and eight-byte double; the prototype has pointer character arguments without added hidden lengths. Loaded-image and linkage records identify actual `libLAPACK.dylib`/`libBLAS.dylib` paths and Accelerate framework version 4.0.0 on macOS 26.7 arm64. This is a bounded test provider, not a production dependency selection or a general ABI guarantee for other providers.

The same exactly representable, dimensionless input is used throughout: rows `[1,-1]`, `[1,0]`, `[1,1]` with RHS `[1,2,4]`. A second case adds constraint row `[0,1]` with target 3. Each is run with rescaling disabled/enabled and four output requests: all outputs, covariance only, errors only and no output buffers (returned chi-square only). Every case supplies real input data and whitened data.

| Source variant | Full-output controls | Requests without parameter output | Total calls |
| --- | ---: | ---: | ---: |
| Original pinned TKfit | 4 complete | 12 terminate with SIGSEGV | 16 |
| Corrected allocation/RHS | 4 complete | 12 complete | 16 |

For every successful corrected request, each returned/requested value matches the corresponding original full-output control's round-trip decimal output exactly, including zero signs. Full-output controls are unchanged by the correction. The harness also verifies that caller data and design matrices remain unchanged.

Observed unscaled/rescaled chi-square is `0.16666666666666671` without the constraint and `1.666666666666667` with it. The latter includes the augmented constraint residual, as the source specifies. Rescaled covariance uses augmented rows minus parameters: denominators one and two, respectively. These dimensionless toy results illustrate the source definition; they are not new numerical goldens or allowances for real timing data. No SVD singular spectrum/rank output is introduced for QR.

## Implementation disposition

AI must reuse the canonical shared solver and matrix owners, preserve the source QR statistic when parameter output is absent, and keep SVD's separate output-dependent behavior. Add native/Rust coverage for these legitimate optional-output requests, both covariance/error paths and nonzero constraint targets, with the actual selected provider. Keep source and corrected native evidence separate; the central probe can be reused as the original/corrected defect witness rather than rediscovering it. Document `TKfit.C:194,260–270`, the absent matrix/RHS problem and its correction beside the changed Rust code under 0053.

The experiment does not qualify the complete backend: actual Rust calls, other dimensions and conditioning, provider errors/recovery, failed output-file creation, workspace-query boundaries, all five accelerated helpers, CALCEPH, disabled/profile dispatch and actual Linux execution remain AI/AL work. The BLAS-only declaration guard in 0081 retains its independent syntax witness and still needs selected-provider execution. No library was installed or production source changed centrally.

This is a demonstrated small implementation defect within the user's existing authorization, not a new scientific convention requiring another approval. There is no ownership transfer, additional AH task, solver-default change or milestone acceptance. The full 0091 ledger, predictor diagnosis 0092, adopted DDH clock rule and selected interface/platform guarantees remain intact.

## Evidence and reproduction

Authored reproducer: `decisions/evidence/native-backends/central-qr-outputs-review.py`, SHA-256 `8a4194f5923460863dce4e3eb9009733638630290663364c409a229dc8f61ef8`. It creates a fresh disposable directory, uses the pinned Git source and immutable reference archive, records actual failures, and refuses any unexpected result. The 42 recorded commands consist of provider profiling/build/linkage commands and 32 runtime cases; twelve original SIGSEGV outcomes are expected observations, not passing original requests.

Original directory: `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-native-qr-outputs-lqu0vq2b`. Receipt SHA-256: `037078155e1f27888b1767da8e74f47dd981ebdc13921a82a87dfbd5d8930c15`. Correction patch SHA-256: `2c13bbca6770d3ea1ba54427c85428908b8537240fdd2d398afe5084b66a1da0`. All seven protected reference/configuration/provider-header or stub/decision files are checked unchanged before and after execution. The receipt records source/configuration/compiler/executable identities and exact commands.

All original artifacts are copied byte-for-byte into ignored `decisions/evidence/native-backends/build/qr-optional-outputs-20261027/`. Preservation receipt SHA-256: `0fd644b1fb79eed8688d98f7d96441ff159ceb09386307575e1b3f4ca71f98cf`. Original execution paths remain recorded; saved copies are not relabelled as executions. Generated JSON, copied native sources, objects and binaries remain outside Git.

```sh
python3 decisions/evidence/native-backends/central-qr-outputs-review.py
```

This recipe requires the recorded local reference archive, GCC toolchain and macOS Accelerate framework. A different provider or platform needs its own identified execution.
