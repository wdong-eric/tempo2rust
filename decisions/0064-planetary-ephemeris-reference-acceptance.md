# Planetary-ephemeris reference acceptance

Central decision, 25 October 2026. **Accept Assignment Z's bounded native reference at `d517acf` under 0060.** Its ownership of `validation/planetary_ephemeris/` closes. This accepts the original native evidence for all three active `readEphemeris.C` routines: readEphemeris, random2 and gasdev. It does not accept a Rust planetary transfer or broaden any milestone, frontend, platform or numerical gate. Y's delivered Rust time-ephemeris work and AB's delivered propagation reference still await central review; AA remains prepared and unreleased.

## Reviewed source and execution contract

Central inspected the complete primary source, caller/state map, explicit initialized backing, separate native builds, observer, comparator, controls and independent checks. The primary compiles unchanged from raw TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f` as its own translation unit with real pinned JPL, clock, numeric, table, observatory and diagnostic dependencies. The accepted A2 config supplies configuration only; no opaque archive, initializer, Rust implementation or substituted numerical helper executes. Symbols establish use of the public binary128 fortran_mod rather than the private binary64 overload.

The ordinary evidence includes two pulsars, twelve observation backing cells, complete selected parameter/offset/flag backing, all target six-vectors and three nine-by-six planet arrays, retained tails and recursively checked non-target fields. Padding and uninitialized storage are excluded explicitly. Ordinary caller prefixes and a separately audited exit observer distinguish completed state from operations that never returned. The frozen machine source map correctly ends gasdev at raw line 570; the older prose endpoint 573 is a documented typo.

The final comparator identity transition changed controls.py and run.py, retained the earlier manifests, and kept the original ordinary oracle unchanged. The earlier observer extension separately records added random-entry and Gaussian-attempt observations. Fresh execution of the final harness against the original oracle establishes agreement independently of these metadata transitions. Original ordinary oracle SHA-256 remains `91e1c049594f007d71ae700cb308419d9b59bcf246f26d5528ee33a508f8374c`; final manifest SHA-256 is `d9cc6d06b5a5b37f5bdb5c288cede9e774ec2a9dc693fa0b5dcab21efe66966e`.

## Central verification

A fresh central native build passed all **96 cases / 3,540 events**, each run twice in fresh processes. Ordered case/action/field membership, recursive types, finite bits, signed zero, nonfinite classes, state, diagnostics and statuses agree exactly. All **31 integrity controls** passed their dispositions: one exact-copy positive control and thirty rejected changes. All **18 freshly compiled source mutations** produced actual numerical/state/diagnostic witnesses. The changed-caller challenge also executes a fresh altered caller; hash changes alone do not count as behavioral detection.

Central separately rebuilt the observer and ran every case twice. Complete observer results match the delivered observer exactly after the declared build-root substitution. Removing only added observer/private/exit records recovers the ordinary outputs exactly. The observer forwards real JPL operations and reads initialized random/Gaussian state without resetting it or substituting calculations.

The supporting independent checks reproduced 793 random returns, 921 complete random-state snapshots, 107 Gaussian returns and thirty rejected pairs; 33,600 polynomial components, 5,611 complete query/date requests, 33,666 transformed components, 1,215 offset components, 417,792 retained cells and 173 completed call histories. Their counts, maxima and declared diagnostic envelopes match delivery. Those analytic envelopes are supporting diagnostics, not Rust acceptance tolerances or full astronomical validation. Exact ordinary comparisons were not relaxed.

Full fresh source/header/compiler/backend/runtime profiles and resource identities agree after the explicit build-root substitution. Path byte widths are equal to retain fixed-buffer and diagnostic-truncation behavior. Complete macro definitions agree as sorted lines, with both raw byte hashes retained. No scientific value or diagnostic wording is normalized. Each actual executable/dependency is checked at launch.

All **3,249 Z files** and **10,003 prior protected members** remain byte-for-byte unchanged. Both independent repositories remain clean at their established pins. Generated evidence remains local and ignored under 0039.

## Coverage and limits

Central reconstructed the retained raw gcov counts and checked all recorded coverage results against the ordinary oracle and raw receipt hashes. Coverage was audited, not rerun centrally. The separate GCC 16.2 O2 coverage profile enters **3/3 routines, 207/207 executable lines and 106/108 branch outcomes**, with zero ordinary-result differences. Both unhit edges remain in the denominator: the random index-error branch is unhit within the released nonnegative-state seed domain, and exact Gaussian rsq==0 is defined but unsampled. Other arithmetic-defined first-use seeds and arbitrary longer sequences remain unverified; no random-quality claim follows.

The actual profile is macOS arm64, GCC/G++ 16.2 O2, configured binary128 and the pinned DE405 plus authored synthetic resources. Existing JPL initializer/cache/alignment/pointer-formation defects mean full native JPL-driven values remain host-qualified observations. Uninitialized planetary failure paths are excluded from numerical goldens; the defined absent-nutation error and four ephemeris-open exits are explicitly covered. Other platforms, real resources, arbitrary malformed layouts, I/O fault injection, concurrent file mutation and process-state races are outside this acceptance. The frozen contract and GAPS.md retain these limits.

No source bug is corrected by this reference acceptance. The source's flag predicates, frame/scale order, unconditional planet differences, scan retention, noise warning and inactive phase behavior remain the original evidence. Later small corrections require 0053's adjacent source-mapped explanation, causal regression and explicit corrected-result disposition; scientific reinterpretation still requires central review.

## Retained central evidence and next work

Reproducer: `decisions/evidence/planetary-ephemeris/central-review.py`, SHA-256 `31330793d05bc75d21e392d690880bca59cb22b6ae7ac2f86e5e69b1d6351174`.

The completed 2,508-file execution bundle is retained at `decisions/evidence/planetary-ephemeris/build/central-rzw4aj4p/`. Its original temporary execution path is a symlink to the retained bundle, preserving recorded absolute identities. Every file was hashed before and after the move; the additional relocation receipt records that check. No duplicate bundle was created.

- Central receipt: `aef102aa14b06ffbd0acf43909d8ecc1b8e9a930d318da43e6997aba90b45b13`.
- Relocation receipt: `dbdc7d1f35d922dd11ef1ac0566ddc538a15e0c6f415fc7e6698ec30a501ccc0`.
- Fresh ordinary executable: `027ccf680693cbda236be99ea0b9844fa06b95b6b338cc0d55e07f07a476b19e`.
- Fresh observer executable: `34f94239517d60b732d7888a6a319134c8336131062df6647a2aab509befd573`.

The subsequent planetary Rust handoff must cover these complete routines and caller/state histories using the canonical pulsar/observation and resource owners. It must distinguish source-defined results, host observations, safe invalid-operation boundaries and deliberate corrections. No new Rust ownership is released here. Central reviews Y next before releasing prepared AA; AB review and the remaining full-core dependency plan continue separately. Rug128, 0049's discrete boundary differences and M2's four historical raw-order failures remain unchanged.
