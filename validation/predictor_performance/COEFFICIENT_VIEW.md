# AO call-local coefficient qualification (decision 0136)

The macOS arm64 C evaluation/model path uses a fresh immutable binary64 slice
inside its synchronous evaluator closure. The closure returns an owned N result
before output publication. The existing checked shape/null gates precede the
view; caller-owned backing must remain aligned, initialized, live, contiguous
and unmodified during the call. No native pointer is retained. Darwin NaN input
sign/quieting agrees with the unchanged C-parts conversion.

Owned Rust coefficients and the native view feed one source Clenshaw recurrence.
Model normalization, dispersion and frequency expressions stay shared. Linux
x86_64 retains owned Rug64 reconstruction, wide range and separate operations.
Callbacks and derivatives retain their owned snapshots. Public Float owners,
borrows, T1 and Rug128 timing are unchanged.

Frozen production pair:

* Before: `e24622d5210fdfc8abe10f9f58a21b7e4a2a1aca` (accepted recurrence).
* After: `a1851c55fcc0b1ac733b064e631bfbbd2e0118d5` (call-local coefficients).
* Native: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, with the existing
  qualifier's recorded corrections.

Use the unchanged 8-by-4 input, SHA-256
`a8b4242d2600cc72198a7f40240b0e946d17d173804e6d10d7f5f31c9f8811ee`.
The actual macOS root is `build/coefficient-view-20261104/`, candidate
`after-v1`. It reuses `build/recurrence-20261104/after-v1`, `after-direct` and
`qualify-after` as immutable before artifacts. Do not rebuild or relabel them.
Use new output directories for a replay; all previous manifests stay immutable.

## Portable correctness recipe

This recipe requires execution on each qualified host; it is not evidence of
Linux execution. Use `clang`, `gfortran`, `gcc` and `g++` on Linux, or the
qualified macOS tools (`clang`, `gfortran-16`, `gcc-16`, `g++-16` here).

```sh
export CARGO_HOME=/path/to/prepared/offline-cargo-cache
export NATIVE_REPOSITORY=/path/to/pinned/tempo2-git
export PREDICTOR=/path/to/t2pred.dat
export AO_OUT=/path/to/fresh/coefficient-view-run
export CC=clang FC=gfortran NATIVE_CC=gcc NATIVE_CXX=g++

python3 validation/predictor_performance/build.py \
  --revision e24622d5210fdfc8abe10f9f58a21b7e4a2a1aca \
  --out "$AO_OUT/before" --cargo-home "$CARGO_HOME"
python3 validation/predictor_performance/build.py \
  --revision a1851c55fcc0b1ac733b064e631bfbbd2e0118d5 \
  --out "$AO_OUT/after" --cargo-home "$CARGO_HOME"

for side in before after; do
  python3 validation/release/qualify_predictor.py \
    --prefix "$AO_OUT/$side/prefix" --out "$AO_OUT/qualify-$side" \
    --native-repository "$NATIVE_REPOSITORY"
  python3 validation/predictor_performance/evaluator_build.py \
    --prefix "$AO_OUT/$side/prefix" --out "$AO_OUT/$side-direct" \
    --cargo-home "$CARGO_HOME"
done
python3 validation/predictor_performance/coefficient_verify.py \
  --before "$AO_OUT/before/prefix" --after "$AO_OUT/after/prefix" \
  --out "$AO_OUT/coefficient-verify"
python3 validation/predictor_performance/verify.py \
  --before "$AO_OUT/before/prefix" --after "$AO_OUT/after/prefix" \
  --before-qualification "$AO_OUT/qualify-before" \
  --after-qualification "$AO_OUT/qualify-after" \
  --predictor "$PREDICTOR" --out "$AO_OUT/verify"
python3 validation/predictor_performance/evaluator_verify.py \
  --before "$AO_OUT/before-direct" --after "$AO_OUT/after-direct" \
  --predictor "$PREDICTOR" --out "$AO_OUT/direct-verify"
python3 validation/release/c_concurrency.py \
  --prefix "$AO_OUT/after/prefix" --out "$AO_OUT/concurrency"
python3 validation/predictor_performance/consumers.py \
  --before "$AO_OUT/before/prefix" --after "$AO_OUT/after/prefix" \
  --out "$AO_OUT/consumers" --cargo-home "$CARGO_HOME"

for side in before after; do
  cp validation/predictor_performance/recurrence_consumer.rs \
    "$AO_OUT/consumers/$side/src/bin/recurrence_consumer.rs"
  cargo run --offline --release \
    --manifest-path "$AO_OUT/consumers/$side/Cargo.toml" \
    --target-dir "$AO_OUT/consumers/$side/target" --bin recurrence_consumer \
    > "$AO_OUT/consumers/$side/recurrence_consumer.stdout" \
    2> "$AO_OUT/consumers/$side/recurrence_consumer.stderr"
done
cmp "$AO_OUT/consumers/before/recurrence_consumer.stdout" \
    "$AO_OUT/consumers/after/recurrence_consumer.stdout"
cargo test --offline --locked --manifest-path "$AO_OUT/after/source/rust/Cargo.toml" \
  --target-dir "$AO_OUT/predictor-tests" --lib predictor -- --test-threads=1
cargo test --offline --locked --release \
  --manifest-path "$AO_OUT/after/source/rust/Cargo.toml" \
  --target-dir "$AO_OUT/predictor-tests" --lib predictor -- --test-threads=1
```

The new witness compares all output/class/sign lines and exact diagnostics
across C11/C++17 shared/static clients, plus a C ASan/UBSan client and ABI shim,
for both installed libraries. It exercises every position in constant, single-
and multi-axis shapes, both selected model polynomials, unchanged backing,
mutation/restoration, pointer replacement, full replacement at a repeated
address, callback snapshots and in-place derivatives. Native Rust/MPFR/Fortran
are not sanitizer-instrumented; the existing caller-exclusive plain/ASan/TSan
and explicit cleanup controls retain that same scope.

The Rust representation test initializes native allocated cells with both signs
of binary64 boundary values, infinities, quiet/signaling NaNs and reproducible
bit patterns; the unchanged C-parts-to-N path is its oracle. The Linux C witness
includes `1+2^-63`, `2^-16445` and `2^16382`; the existing Linux adapter and
ordinary-range tests must also execute on actual Linux. Do not substitute a
changed cfg or binary64 approximation on macOS.

Require the retained 5,880 direct edge results, 28 finite results, 30,258 C
output/state lines and 38 legacy products to match exactly. Preserve all native
discrepancies separately; these comparisons do not waive the T1 NaN-sign or
polyco log10RMS failures.

Inspect the optimized installed library with `otool -tvV` (macOS) or `objdump -Cd`
(Linux). The macOS C recurrence must load coefficient doubles directly, retain
source FMA sites and separately rounded doubling/coefficient additions, and
perform no coefficient Vec/N reconstruction. The seven model scalar owners,
input/output conversions and public owned path remain. No fast-math, alternate
rounding mode or subnormal flushing is introduced.

## Coordinated measurements

After correctness and central quiet-window clearance, run the unchanged
interleaved `evaluator_measure.py` and `measure.py` commands in
[RECURRENCE.md](RECURRENCE.md#coordinated-uninstrumented-measurements) with the
before/after directories above: 100,000 pairs, seven samples, 28 direct and
63 complete-C/native/startup processes. Remove allocation observers, retain
all raw samples and startup outliers, and save process snapshots before/after.
`verify.py` and `evaluator_verify.py` already observe allocations separately at
1 and 1,001 iterations; requested bytes are not RSS. Report direct and full-C
workloads independently, including every executable/input identity and peak RSS.
Neither the earlier profile percentages nor source owner counts predict speedup.
