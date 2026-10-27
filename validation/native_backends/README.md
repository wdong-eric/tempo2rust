# Assignment AI reproducible evidence

Run from the repository root. Generated sources, dependency binaries, data,
executables, stdout/stderr, TSV/JSON and logs stay under ignored `build/`.
Native oracles use the pinned source and the hash-verified AH native-v9 dependency
objects; they never edit `tempo2/`, `validation/reference/` or accepted outputs.
`CONTRACT.md` freezes quantities, gates and profile boundaries.

## Provider preparation

The official CALCEPH release host timed out in this execution. The installed
4.0.5 source came from the public mirror
`https://github.com/achalumeau/calceph`, subsequently checked file-for-file
against immutable commit `718e7945ccd27216b522243c3ff8aab63ce1d99d`.
All 533 archive files match the built tree. This records a mirror provenance,
not a verified upstream release signature. The real TT-TDB/position data came
from IMCCE's published INPOP17a distribution, not CALCEPH's test ephemeris.

The executed local build used CMake 4.4.4 in a task-local Python venv, Apple
clang 21, static PIC, Release, Fortran API disabled. Reproduction:

```sh
mkdir -p validation/native_backends/build
python3 -m venv validation/native_backends/build/tools
validation/native_backends/build/tools/bin/pip install cmake==4.4.4
curl -L --fail https://codeload.github.com/achalumeau/calceph/tar.gz/718e7945ccd27216b522243c3ff8aab63ce1d99d \
  -o validation/native_backends/build/calceph-718e794.tar.gz
curl -L --fail https://ftp.imcce.fr/pub/ephem/planets/inpop17a/inpop17a_TDB_m100_p100_tt.dat \
  -o validation/native_backends/build/inpop17a_TDB_m100_p100_tt.dat
# Verify the hashes below before unpacking/building.
mkdir validation/native_backends/build/calceph-main
tar -xzf validation/native_backends/build/calceph-718e794.tar.gz \
  -C validation/native_backends/build/calceph-main --strip-components=1
validation/native_backends/build/tools/bin/cmake \
  -S validation/native_backends/build/calceph-main \
  -B validation/native_backends/build/calceph-cmake \
  -DCMAKE_INSTALL_PREFIX="$PWD/validation/native_backends/build/provider" \
  -DENABLE_FORTRAN=OFF -DCMAKE_BUILD_TYPE=Release \
  -DBUILD_SHARED_LIBS=OFF -DCMAKE_POSITION_INDEPENDENT_CODE=ON
validation/native_backends/build/tools/bin/cmake --build validation/native_backends/build/calceph-cmake -j4
validation/native_backends/build/tools/bin/cmake --install validation/native_backends/build/calceph-cmake
```

| Input/product | SHA256 measured here |
| --- | --- |
| Immutable CALCEPH tar | `b38f6ccc726bab51c5901bb74c655e13941753ca7c3d0054396a37b793040426` |
| Real INPOP17a data | `32517ec82bfb298d53baeba4065750dec82143fab79357e4487f84fb439bc49c` |
| Installed calceph.h | `17fbd05ae48449e1e0c27d70a153ddae07bde7f7722e0a75162ce71d9ff66417` |
| Installed libcalceph.a | `2228faa63fd734c6bf23046ef9fed15524c00015241f925a64d5678e2cb5f4e5` |

The initial downloaded `main` tar (retained as `calceph-mirror.tar.gz`) has
SHA256 `f1374a67b650aa989370cf61a884ea4933401f8fcd5029fa46e65406c5f976c4`.
`provider_identity.py` documents that initial-to-immutable correspondence and
records compiler/OS/SDK versions, linked and loaded Accelerate images, archive
and resource identities. Its historical audit also expects the retained initial
tar and `mirror-commit.json`; a fresh reproduction can verify the immutable
tar directly and compile `provider_probe.c` using its recorded command.
Build logs and CMakeCache.txt retain the actual configuration. The ABI probe
reports CALCEPH 4.0.5, int=4, double=8, ordinary long double=8, INPOP17A TDB,
JD span 2414105..2488985, and a successful target-16 sample at 2458000+0.125.

## Executed verification recipes

```sh
python3 -m unittest discover -s validation/native_backends -p test_check.py -v
python3 validation/native_backends/provider_identity.py
python3 validation/native_backends/native_build.py \
  validation/native_backends/build/native-combined-published --profile combined
CALCEPH_DIR="$PWD/validation/native_backends/build/provider" TEMPO2_ACCELERATE=1 \
  cargo build --offline --manifest-path rust/Cargo.toml \
  --features calceph,lapack --example native_backends --bins
python3 validation/native_backends/check.py \
  validation/native_backends/build/paired-combined-debug \
  --native validation/native_backends/build/native-combined-published/probe \
  --rust rust/target/debug/examples/native_backends --profile combined
python3 validation/native_backends/cli_check.py \
  validation/native_backends/build/cli-combined-debug \
  --native validation/native_backends/build/native-combined-published/tempo2 \
  --rust rust/target/debug/tempo2-rust \
  --par validation/native_backends/build/paired-combined-debug/inputs/calceph.par \
  --tim validation/native_backends/build/paired-combined-debug/inputs/pks.tim --lapack
python3 validation/native_backends/qualify_profiles.py \
  validation/native_backends/build/profiles-final-v3
python3 validation/native_backends/final_checks.py validation/native_backends/build/final-checks-v3
python3 validation/native_backends/boundary_check.py
python3 validation/native_backends/audit_delivery.py
```

The final executed profile command also passed `validation/native_backends/build/profiles-final-v2`
as a third argument to reuse its hash-verified unchanged native executables and
objects after the Rust diagnostic-order correction. Omitting that argument, as
above, rebuilds them from the same recorded recipe. Every runtime comparison is
fresh in the final directory.

Each output directory must be fresh. `qualify_profiles.py` builds disabled,
BLAS-only, LAPACK, CALCEPH-only, combined debug/release, disabled release, and combined
QR_DEFAULT profiles. It runs the focused tests and public direct consumer for
each, then compares the relevant native executable built with identical features.
Every profile checks the literal no-argument feature banner and exit status.
The QR_DEFAULT profile runs the ordered CLI and JSON-default checks.
Five focused mutation controls test the comparator itself: exact finite
sign (even inside tolerance), scalar/vector family separation, matrix-column
separation, membership/type/zero/nonfinite classes, and exact row order.
Native and Rust observers serialize the same ordered quantity families; the
ordering control does not sort away pulsar, observation, body or coordinate order.

`check.py` snapshots executable bytes per run and hashes production Rust sources,
the comparator/contract, inputs and every produced file, then rechecks them before
publishing its receipt. Native identities include actual config, original and
corrected source hashes, object/archive hashes, patches and complete compiler
commands. Check receipts retain native/Rust exits and every failed quantity.
The 21 public pipelines cover modes 0/1/2 with equatorial TCB, TDB, ecliptic,
nonzero parameter prior, Huber robust, two-pulsar global (12/10 observations), and
correlated covariance fits. Direct CALCEPH uses two distinct pulsars (2/3 TOAs),
both frames/units, selected/mass planets, defined inactive arrays, repeated passes.
Direct QR covers all eight output masks, constraints on/off, rescaling on/off.

Real-provider failure/recovery and descriptor count checks are explicit:

```sh
CALCEPH_DIR="$PWD/validation/native_backends/build/provider" TEMPO2_ACCELERATE=1 \
AI_EPHEMERIS="$PWD/validation/native_backends/build/inpop17a_TDB_m100_p100_tt.dat" \
AI_POSITION_ONLY="$PWD/validation/reference/build/source/T2runtime/ephemeris/DE405.1950.2050" \
  cargo test --offline --manifest-path rust/Cargo.toml --features calceph,lapack \
  --test native_backends -- --include-ignored --test-threads=1
```

Repeat with `--release` for release evidence. Serial library tests supplement
these focused checks; ignored historical corpora are not silently counted as
passes. The original CALCEPH ignored-status witness is executed as:

```sh
validation/native_backends/build/native-combined-published/probe failed-position \
  validation/native_backends/build/calceph-original-failed-position.tsv \
  validation/native_backends/build/inpop17a_TDB_m100_p100_tt.dat
```

Its supplied buffers are defined; the selected provider reports out-of-range
errors and source still publishes a zero position. The Rust test reports the
failure and then recovers with a valid call. The source uninitialized time-buffer
failure is not used as a golden. Earlier developmental runs, including their
comparator identities and test failures, remain in `build/`; the report names
the final evidence explicitly.
