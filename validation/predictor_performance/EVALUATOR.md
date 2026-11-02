# AO continuation: canonical evaluator ownership

Handoff 42 compares the accepted transport production `0bd8e2c` with evaluator
production `8716d1d`. The latter descends from delivered `f077292`; the intervening
commits add tests, recipes and documentation. This is a separate experiment from
the original [transport recipe](README.md). Its artifacts and report stay intact.

Use a disposable checkout with both commits and an already prepared Cargo cache,
the pinned native TEMPO2 checkout, and the same 8-by-4 `t2pred.dat` input. The
measured file SHA-256 is
`a8b4242d2600cc72198a7f40240b0e946d17d173804e6d10d7f5f31c9f8811ee`.
It covers epochs 58000.010000 through 58000.010006 and 1400 MHz.

## Build and compare, including the pending Linux replay

```sh
export CARGO_HOME=/path/to/prepared/cargo-cache
export NATIVE_REPOSITORY=/path/to/pinned/tempo2-git
export PREDICTOR=/path/to/prepared/t2pred.dat
export AO_OUT=/path/to/fresh/predictor-performance/build/evaluator-run
export CC=clang FC=gfortran NATIVE_CC=gcc NATIVE_CXX=g++

python3 validation/predictor_performance/build.py \
  --revision 0bd8e2cf2ef54e9da6feace122acd8aa96aa3fbe \
  --out "$AO_OUT/before" --cargo-home "$CARGO_HOME"
python3 validation/predictor_performance/build.py \
  --revision 8716d1dc026641f3a51c2ba88850c5599293cbdd \
  --out "$AO_OUT/after" --cargo-home "$CARGO_HOME"

python3 validation/release/qualify_predictor.py \
  --prefix "$AO_OUT/before/prefix" --out "$AO_OUT/qualify-before" \
  --native-repository "$NATIVE_REPOSITORY"
python3 validation/release/qualify_predictor.py \
  --prefix "$AO_OUT/after/prefix" --out "$AO_OUT/qualify-after" \
  --native-repository "$NATIVE_REPOSITORY"
python3 validation/predictor_performance/verify.py \
  --before "$AO_OUT/before/prefix" --after "$AO_OUT/after/prefix" \
  --before-qualification "$AO_OUT/qualify-before" \
  --after-qualification "$AO_OUT/qualify-after" \
  --predictor "$PREDICTOR" --out "$AO_OUT/verify"

python3 validation/predictor_performance/evaluator_build.py \
  --prefix "$AO_OUT/before/prefix" --out "$AO_OUT/before-direct" \
  --cargo-home "$CARGO_HOME"
python3 validation/predictor_performance/evaluator_build.py \
  --prefix "$AO_OUT/after/prefix" --out "$AO_OUT/after-direct" \
  --cargo-home "$CARGO_HOME"
python3 validation/predictor_performance/evaluator_verify.py \
  --before "$AO_OUT/before-direct" --after "$AO_OUT/after-direct" \
  --predictor "$PREDICTOR" --out "$AO_OUT/direct-verify"

python3 validation/release/c_concurrency.py \
  --prefix "$AO_OUT/after/prefix" --out "$AO_OUT/concurrency"
python3 validation/predictor_performance/consumers.py \
  --before "$AO_OUT/before/prefix" --after "$AO_OUT/after/prefix" \
  --out "$AO_OUT/consumers" --cargo-home "$CARGO_HOME"
cargo test --offline --locked --manifest-path "$AO_OUT/after/source/rust/Cargo.toml" \
  --target-dir "$AO_OUT/predictor-tests" --lib predictor -- --test-threads=1
cargo test --offline --locked --release --manifest-path "$AO_OUT/after/source/rust/Cargo.toml" \
  --target-dir "$AO_OUT/predictor-tests" --lib predictor -- --test-threads=1
```

Preserve the existing qualifier's locale requirements and all native failures;
do not replace a failed gate with before/after equality. Central may substitute
reviewed commits when integrating, but must keep the frozen original pairs
identifiable. Build exports and installed consumers use distinct targets and
the supplied offline cache. No new dependency enters the production crate.

The direct client calls the public canonical polynomial and model evaluators.
Its `leaf` scope evaluates both 8-by-4 polynomials at seven dimensionless x
values and y=0.25; the second polynomial returns dphase/dx, not frequency in Hz.
Its `model` scope uses seven preconstructed epochs and 1400 MHz. The full C
scope additionally includes native storage reconstruction and result transport.
These are separate workloads, so their times must not be subtracted to claim
an exact FFI overhead. Every direct run prints all 28 finite results; a separate
edge client prints 5,880 class/sign/range results for five polynomial shapes.

Allocation observations subtract whole-process counts at 1 from those at 1,001
iterations. Both raw results are retained. The three-byte argv-length difference
is visible as 0.003 requested bytes/iteration for direct clients and is not
silently corrected. `from` and `owned` scopes isolate scalar creation and one
owned multiply (including its input clone); they are allocation observations,
not performance claims. macOS interposes external dyld references. Linux links
a separate counter client with GNU `--wrap`; its interception scope differs.
Hidden libc allocator internals are not counted. All timed clients are plain.

## Quiet timing window

Finish all compilation and checks, announce the window to central, then run
these sequentially without another benchmark or compiler on the host:

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

Retain every sample, including startup outliers, resource records and process
snapshots. The 28 direct and 63 full-route processes are shuffled within their
own campaigns. Loaded evaluation time excludes parsing/setup; wall/CPU/RSS are
whole-process observations. These scripts are Linux-ready recipes, not evidence
of actual 64-bit-profile execution. AL owns that qualification and central owns
integration, publishing and workflow dispatch.

The actual macOS continuation root is
`build/evaluator-20261102/`. It reuses the immutable accepted baseline at
`build/after-v2/prefix` and `build/qualify-after`; the new installed candidate
is `after-v1/prefix`. The final direct baseline is `before-direct-v2`, which adds
the complete edge witness to the first baseline build. Initial allocation
attribution and the test failure that exposed division's NaN-sign rule remain
in the evidence root. See the [evaluator report](../../rust/PREDICTOR_EVALUATOR_PERFORMANCE_REPORT.md)
for measured results, evidence identities and limits.
