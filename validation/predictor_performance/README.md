# Assignment AO predictor performance

This directory observes the real installed predictor C route. It reuses
`validation/parallel_execution/evaluation.c`, the portable release resource
parser, `qualify_predictor.py`, the existing native numerical gates and the
caller-exclusive sanitizer campaign. It does not provide another evaluator.

Production: `0bd8e2cf2ef54e9da6feace122acd8aa96aa3fbe`, based on
`68973d669af0e1180ad8532bb8b364452ca203d7`. Both retain Rug128 timing and the
host's ordinary 53/64-bit predictor arithmetic. See
[`rust/PREDICTOR_PERFORMANCE_REPORT.md`](../../rust/PREDICTOR_PERFORMANCE_REPORT.md).

## Portable replay / pending Linux execution

Run in a disposable checkout containing the selected commits. Set the four
paths below to **already prepared** inputs. `CARGO_HOME` must contain the pinned
offline dependencies; do not replace it with an empty temporary cache. The
predictor must contain the benchmark epochs 58000.010000 through 58000.010006
and 1400 MHz. Use the same file for all engines. Its hash is recorded.

```sh
export CARGO_HOME=/path/to/prepared/cargo-cache
export NATIVE_REPOSITORY=/path/to/pinned/tempo2-git
export PREDICTOR=/path/to/prepared/t2pred.dat
export AO_OUT=/path/to/fresh/predictor-performance/build/run
export CC=clang FC=gfortran NATIVE_CC=gcc NATIVE_CXX=g++

python3 validation/predictor_performance/build.py \
  --revision 68973d669af0e1180ad8532bb8b364452ca203d7 \
  --out "$AO_OUT/before" --cargo-home "$CARGO_HOME"
python3 validation/predictor_performance/build.py \
  --revision 0bd8e2cf2ef54e9da6feace122acd8aa96aa3fbe \
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

`build.py` exports Git blobs, records their hashes and tool versions, and invokes
the existing installer with distinct `before/prefix-build/target` and
`after/prefix-build/target` directories. It ignores `CARGO_TARGET_DIR` and never
reuses another checkout's target. If central cherry-picks onto another baseline,
substitute the reviewed paired commits; preserve the three-file production
diff and verify the rest of the frozen crate is equal. Central owns integration,
GitHub publication and workflow wiring. No workflow change is included here.

The release qualifier needs its existing C/German/French locales. Preserve
its locale failures instead of skipping them. The Linux allocation observer
uses the installed static archive and GNU `--wrap`; macOS uses a separately
loaded dyld interposer. They count external allocator requests, not hidden
libc allocator internals, and their timings are never performance evidence.

After all builds and qualification finish, announce a quiet window to central
and run only the already-built clients:

```sh
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

This retains all 63 shuffled fresh processes, raw resource reports, outputs,
checksums, executable/input hashes and load averages. Iterations 0 denote actual
getPeriod processes; 1 includes load/parse/initialization; 100000 separately
reports the existing timed phase/frequency loop. CPU counters may round short
runs to zero. A checksum is only a timing control: `verify.py` compares complete
30,258-line outputs/state, 38 legacy product/transcript files, native symbols,
source identities and direct mutation/restoration. It reports supplemental
native differences separately and never changes a qualification gate.

For macOS stack sampling (may require permission to inspect the child process):

```sh
python3 validation/predictor_performance/sample.py \
  --executable "$AO_OUT/verify/after-evaluation" --predictor "$PREDICTOR" \
  --iterations 400000 --out "$AO_OUT/sample-after"
```

This runs adjacent plain/sampled processes and records sampling overhead. Its
categories are inclusive and overlap; it avoids counting nested occurrences
twice within one category. Linux can use `perf record -g --call-graph dwarf`
on the same client with a longer iteration count, retaining a matching unprofiled
run and all raw output. Do not treat unavailable perf permissions as scientific
failure or claim Linux performance from macOS results.

## Delivered macOS evidence

Root: `/private/tmp/tempo2-ao-predictor-20261102/validation/predictor_performance/build`.
The initial before install is `before/`, source `before-source/`; the candidate
is `after-v2/prefix/`, source `after-v2/source/`. `qualify-before/` and
`qualify-after/` contain the existing gates. Final complete comparisons are in
`verify-v4/`; the timed clients were prepared in `verify-v3/` from the same frozen
libraries and unchanged evaluation source. `measure-v2/`, `consumers/` and
`concurrency-after/` contain the completed campaigns. Stack samples are
`before.sample.txt` and `after.sample.txt`; `profile-summary.json` is reproduced
by `sample.py::summarize`.

All failed preparation attempts remain in this ignored root. They include the
Python 3.11 archive-filter mismatch, the Darwin feature-macro compile failure,
an executable-only allocator interposer that correctly failed its nonzero-count
assertion, and sandbox denials for sampling/resource counters. Completed runs
use the corrected harness and permitted OS observations. None is a suppressed
production failure. Generated files, exports, installs, traces and binaries stay
ignored; only the authored runners/observer sources and reports are committed.
