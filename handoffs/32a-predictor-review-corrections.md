# AF early central review corrections

Central review, 26 October 2026, during active AF implementation after interface freeze `5aee9bf`. These are two demonstrated implementation/verification defects inside the existing 0076/0088 contract. AF remains the sole Rust writer; central has sent the reproductions to its chat. Correct them before delivery and continue independent predictor work. No new scientific convention, precision mode, tolerance or user decision is required by this brief.

## 1. Preserve native subnormal rounding

Reviewed `rust/src/predictor_number.rs` SHA-256: `6da7a1c4f2b5250253753d1b70b59d6d50abb8f9fe7299558e912eba558e7c50`. Central compiled an isolated snapshot of the actual public Rust type and a native GCC16 C witness on macOS arm64. Both use the measured ordinary-long-double/binary64 profile.

| Operation | Native binary64 result bits | Actual PredictorNumber result bits |
| --- | --- | --- |
| Convert the 128-bit value `2^-1075 + 2^-1180` | `0000000000000001` | `0000000000000000` |
| Multiply binary64 inputs with bits `0010000000000001` and `3c9fffffffffffff` | `0000000000000001` | `0000000000000000` |

The expected result is the smallest positive subnormal; the wrapper returns zero. The first rounding to a full p-bit MPFR significand loses information needed by the later subnormal rounding. Conversion neighbors 1.5, minimum subnormal, minimum normal and negative zero agree exactly. This is a focused arithmetic/representation finding, not evidence of an ordinary pulsar fixture's timing error.

Correct the cause at the native arithmetic/conversion boundary. Direct binary64 conversion/arithmetic can preserve the measured 53-bit profile; alternatively retain the rounding direction through MPFR range/subnormal operations. Inspect parsing and the 64-bit branch for the same two-stage rounding pattern. Preserve the source operation boundary, exponent range, signed classes and deliberate FMA sites; no change to Rug128 timing or source algorithms is authorized. A macOS witness does not qualify Linux/x87 behavior.

Use the two exact failing inputs and defined neighbors as regressions. The executed probe sources are `decisions/evidence/predictor-review/native-subnormal.c` and `rust-subnormal.rs`. The ignored numeric evidence includes the standalone Cargo consumer and full source snapshot; its Cargo manifest links to the adjacent snapshot and pins Rug 1.30.0. The native command is recorded in `source-identity.json`; the Rust consumer was run with `cargo run --offline --release`, the existing Cargo cache and an isolated target directory.

## 2. Reject incomplete or corrupted constructed models

Reviewed `validation/predictors/run_public.py` SHA-256: `8d834f514ead3bcb31f4ead114b2a09be4690e633320bd6c818c95999e7afc11`. Central replayed preserved passing `public-debug-v2/isolated-bary` outputs through the actual comparator, changing only the generated-model JSON. No engine was re-executed for this comparison-only experiment. Original file products and cross-reader outputs stayed unchanged to distinguish the constructed model from its serialization.

The baseline passes. Each of these four independent mutations also falsely passes with zero failures:

- Empty the first Cheby segment's `cheby.coeff` array.
- Change that segment's `cheby.nx` to 999.
- Change that segment's `freq_start` to 1000000.
- Replace the first polyco segment's `coeff` array with `["1e100"]`.

An additional empty-final-polyco-coefficient control already raises an exception; it is not a false-pass finding. Its rejected outcome is retained in the review receipt. The first exploratory review stopped at that exception; the final reviewer records it explicitly and preserves the earlier output directory.

Derive shape and membership from independent native records, check every segment/coefficient and its required metadata, and validate dimensions/ranges/units/kind/status/classes before numerical tolerances. Generated coefficients must be checked separately from file exchange and approximation quality. Apply the frozen per-quantity rules without replacing them with one coarse tolerance. Retain rejection controls for all four proven omissions, plus required phase/frequency/plan/class controls as the complete harness is finished.

The authored comparison-only reviewer is `decisions/evidence/predictor-review/central-comparator-review.py`. When rechecking a revised observer/schema, first regenerate a compatible passing baseline in fresh AF staging; an old trace missing newly required fields is not a valid negative-control baseline. Do not overwrite the original evidence.

## Evidence and return

Preserved ignored evidence: `decisions/evidence/predictor-review/build/early-20261026/`. Central receipt SHA-256: `4a030972892eb05e0f4c8776ec09f488e295acda8181457f50b898150a8b6751`. Numeric original directory: `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-af-type-review-f77a3wfe`. Comparator original directory: `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-af-comparator-review-6yfzbbki`. Exact reviewed source snapshots, probe sources/executables, original records, mutations, native/Rust outputs and source identities are retained. Generated JSON, binaries and snapshots remain ignored under 0039.

Return corrected source identities, the focused before/after results, valid neighbors and comparator rejection evidence in AF's report. Central will close these findings from actual evidence. Their discovery does not release another writer, accept a predictor, or alter the inherited numerical ledger.
