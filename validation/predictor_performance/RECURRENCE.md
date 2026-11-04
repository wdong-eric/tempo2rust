# AO macOS recurrence qualification (decision 0131)

This slice keeps one Clenshaw recurrence and specializes only its private macOS
arm64 temporaries. The public Float owner and `as_float(&self) -> &Float` remain
unchanged. Linux x86_64 keeps borrowed inputs, Rug64, its wider exponent range
and separate multiply/add operations. This recipe is portable; a prepared Linux
recipe is not evidence of Linux execution. AL owns that platform qualification.

Frozen production pair:

* Before: `8716d1dc026641f3a51c2ba88850c5599293cbdd` (owner reuse).
* After: `e24622d5210fdfc8abe10f9f58a21b7e4a2a1aca` (private recurrence adapter).
* Native: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, with the existing
  qualifier's recorded corrections.

Use the unchanged 8-by-4 predictor, SHA-256
`a8b4242d2600cc72198a7f40240b0e946d17d173804e6d10d7f5f31c9f8811ee`.
Keep previous transport/owner-reuse evidence immutable. Supply a new output
directory, a prepared offline Cargo cache and the pinned native repository.
On macOS use the installed GCC/GFortran names (here `gcc-16`, `g++-16`,
`gfortran-16`); on Linux use the qualified host toolchain.

## Correctness before timing

```sh
export CARGO_HOME=/path/to/prepared/cargo-cache
export NATIVE_REPOSITORY=/path/to/pinned/tempo2-git
export PREDICTOR=/path/to/t2pred.dat
export AO_OUT=/path/to/fresh/recurrence-run
export CC=clang FC=gfortran NATIVE_CC=gcc NATIVE_CXX=g++

python3 validation/predictor_performance/build.py \
  --revision 8716d1dc026641f3a51c2ba88850c5599293cbdd \
  --out "$AO_OUT/before" --cargo-home "$CARGO_HOME"
python3 validation/predictor_performance/build.py \
  --revision e24622d5210fdfc8abe10f9f58a21b7e4a2a1aca \
  --out "$AO_OUT/after" --cargo-home "$CARGO_HOME"

for side in before after; do
  python3 validation/release/qualify_predictor.py \
    --prefix "$AO_OUT/$side/prefix" --out "$AO_OUT/qualify-$side" \
    --native-repository "$NATIVE_REPOSITORY"
  python3 validation/predictor_performance/evaluator_build.py \
    --prefix "$AO_OUT/$side/prefix" --out "$AO_OUT/$side-direct" \
    --cargo-home "$CARGO_HOME"
done
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

# consumers.py now builds and compares recurrence_consumer alongside the
# existing component/subnormal clients, retaining both output streams.

cargo test --offline --locked --manifest-path "$AO_OUT/after/source/rust/Cargo.toml" \
  --target-dir "$AO_OUT/predictor-tests" --lib predictor -- --test-threads=1
cargo test --offline --locked --release \
  --manifest-path "$AO_OUT/after/source/rust/Cargo.toml" \
  --target-dir "$AO_OUT/predictor-tests" --lib predictor -- --test-threads=1
```

All 5,880 direct edge results, 28 finite results, 30,258 complete C transcript
lines and 38 legacy products must match the frozen baseline. Preserve native
failures separately; equality cannot waive the inherited T1 NaN-sign or polyco
log10RMS failures. Keep mutation/restoration, callbacks, installed consumers,
plain/ASan/UBSan/TSan caller checks and cleanup controls. The new external
consumer retains input/coefficient/result `&Float` borrows across evaluations.

The macOS primitive tests compare against unchanged N operations, including
NaN signs and subnormal rounding. The Linux-only adapter test includes a value
requiring 64 significand bits, `2^-16445` and `2^16382`; all three must survive
without binary64 conversion. Existing source-separate and subnormal tests also
remain required. Run on actual Linux x86_64; do not substitute a changed cfg on
macOS. Timing precision stays Rug128 on both platforms.

Decision 0137 wires this sequence into `validation/release/integrated.py`.
The `recurrence-*` stages reuse the owner-reuse pair's after install, native
qualification and direct clients as their before endpoint. Both pair revisions
are fetched explicitly by the Linux workflow. Separate Linux range gates
require the named test's passing output in debug and release, so an empty test
selection cannot count as evidence. The installed-library disassembly, borrow
consumer outputs, native failures and raw measurements survive collection.
All pair compilation precedes the existing sequential measurement block.
This wiring is preparation; actual Linux execution remains pending.

Inspect the actual optimized installed library (`otool -tvV` on macOS,
`objdump -Cd` on Linux). On macOS, source FMA sites must remain fused and the
preceding doubling and subsequent coefficient addition separately rounded.
Do not enable fast-math, alternate rounding modes or subnormal flushing.
Qualification uses the default floating-point environment already required by
the existing Rust FMA path; the adapter does not change control registers.

## Coordinated uninstrumented measurements

Finish compilation/correctness checks and obtain central's quiet-window
clearance before starting. Save process snapshots at the window boundaries.
Run sequentially, retaining every sample and startup outlier:

```sh
python3 validation/predictor_performance/evaluator_measure.py \
  --before "$AO_OUT/before-direct/plain" --after "$AO_OUT/after-direct/plain" \
  --predictor "$PREDICTOR" --out "$AO_OUT/direct-measure" \
  --iterations 100000 --samples 7
python3 validation/predictor_performance/measure.py \
  --before "$AO_OUT/verify/before-evaluation" \
  --after "$AO_OUT/verify/after-evaluation" \
  --native "$AO_OUT/verify/native-evaluation" \
  --before-period "$AO_OUT/before/prefix/bin/getPeriod-rust" \
  --after-period "$AO_OUT/after/prefix/bin/getPeriod-rust" \
  --native-period "$AO_OUT/qualify-before/native-getPeriod" \
  --predictor "$PREDICTOR" --out "$AO_OUT/measure" \
  --iterations 100000 --samples 7
```

The unchanged runners retain 28 direct and 63 full-C/native/startup processes,
whole-process CPU/RSS, loaded-loop times and complete outputs. Allocation
observers run separately at 1 and 1,001 iterations. Their external-call counts
and requested bytes are not RSS; keep the direct argv-length byte delta visible.
Do not subtract different direct/C workloads to claim an exact transport cost.
See [EVALUATOR.md](EVALUATOR.md) for their established scope and definitions.

The actual macOS root is `build/recurrence-20261104/`, candidate `after-v1`.
It reuses the immutable owner-reuse baseline `build/evaluator-20261102/after-v1`,
its `qualify-after`, and `after-direct` rather than relabeling/rebuilding them.
Source, executable, input and earlier evidence identities must be rebound in
the new delivery manifest. Central owns review/integration and release budgets;
this slice does not publish or dispatch a workflow.
