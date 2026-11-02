# AN allocation measurement and Linux handoff

See `rust/ALLOCATION_PROFILE_REPORT.md` for measured macOS results, immutable
production revision `68973d669af0e1180ad8532bb8b364452ca203d7`, and inherited gate
differences. This directory contains bounded observers and runners, not a new
benchmark framework. Original AK and release scripts remain unchanged.

`build.py` exports an exact Git revision, builds its crate offline in release
mode, compiles the unchanged AK serial CLI wrapper, and binds standalone Rust/C
residency and failure observers. Both build and verification force
`CARGO_TARGET_DIR` to that export's `source/rust/target`, overriding any inherited
shared integrated target. They preserve the caller's `CARGO_HOME`, including
Cargo's normal default when it is unset. `measure.py` runs fresh interleaved processes,
separates untouched ranges from read/zero/write/COW controls, and executes the two
AK CLI workloads. `verify.py` reuses the existing lifecycle and fitting gates,
checks bounded allocation rejection, and compares complete same-engine science
and files. `predictor_gate.py` selects the existing isolated-PKS case with the
already released ABI/serialization adapter; it does not change its gates.

Every output directory must be new. All generated evidence belongs under `build/`
or another ignored/export directory. Retain failed runs. The measurement has no
speed/RSS acceptance threshold. Do not interpret observer intervals as full CLI
timings. After free, `mincore` queries numerical VM ranges only; it never reads
freed memory or invokes allocator introspection on freed pointers.

## Linux recipe for central review (not executed by AN)

Use the selected Ubuntu 24.04 x86_64 release environment, Rust 1.97.1, the same
GCC/GFortran 13 and Clang toolchain as the integrated run, and its native A2/core
products. No workflow dispatch or publication is needed to prepare these commands.
Supply a disposable checkout with the AN commits and both source revisions
available locally; the baseline revision must not be lost to a shallow checkout.
Do not use the live macOS A2 binaries on Linux.

The two paired source exports below differ only by AN's allocator/unit-test work
and its observer scaffolding. If central instead measures an integrated AL+AN
pair, prepare both exact revisions with identical AL changes and only the
allocator change between them; record those new identities. An old release vs a
new scientific candidate is not an allocation-only comparison.

From that isolated checkout, set the actual existing Linux release paths:

```sh
export AN_RELEASE=/tmp/t2/linux
export AN_A2="$AN_RELEASE/a2"
export AN_S_EXPECTED="$AN_RELEASE/native-corpora/staging/validation/pulsar_lifecycle/expected"
export AN_BEFORE=dc435f53a64e4dd5c25cae6319841a28bd981fcb
export AN_AFTER=68973d669af0e1180ad8532bb8b364452ca203d7
export AN_OUT="$PWD/validation/allocation_profile/build/linux-an"
export RUSTUP_TOOLCHAIN=1.97.1
export FC=gfortran-13
export NATIVE_CXX=g++-13
export CC=clang
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export GMP_MPFR_SYS_CACHE="$PWD/validation/allocation_profile/build/gmp-cache"
export CARGO_BUILD_JOBS=4
export LC_ALL=C
export OPENBLAS_NUM_THREADS=1
export VECLIB_MAXIMUM_THREADS=1

# Reuse the release cache, or populate this selected cache before offline builds.
cargo fetch --locked --manifest-path rust/Cargo.toml
mkdir -p "$AN_OUT"
uname -a > "$AN_OUT/uname.txt"
cat /etc/os-release > "$AN_OUT/os-release.txt"
getconf PAGESIZE > "$AN_OUT/page-size.txt"
ldd --version > "$AN_OUT/libc-version.txt"
rustc -Vv > "$AN_OUT/rust-version.txt"

python3 validation/allocation_profile/build.py \
  --revision "$AN_BEFORE" --out "$AN_OUT/before" --a2 "$AN_A2"
python3 validation/allocation_profile/build.py \
  --revision "$AN_AFTER" --out "$AN_OUT/after" --a2 "$AN_A2"

# Finish builds first; run fresh processes serially without other heavy jobs.
python3 validation/allocation_profile/measure.py \
  --before "$AN_OUT/before" --after "$AN_OUT/after" \
  --native-cli "$AN_RELEASE/core-disabled/tempo2" --a2 "$AN_A2" \
  --out "$AN_OUT/measure" --samples 5
```

This selects the existing `/usr/bin/time -v` parser on Linux, `mincore`,
`malloc_usable_size`, `/proc/self/statm`, `/proc/self/maps` and `getrusage`.
The Linux branches have not been compiled or executed by AN. Linux can use
shared zero pages on reads, unlike this observed macOS result; assess both
residency and current RSS rather than assuming identical page behavior. Record
the libc version and inherited allocator environment (including any `LD_PRELOAD`,
glibc tunables or malloc settings); a custom allocator invalidates the default
allocator interpretation, not zero-value correctness.

For the focused equivalence and existing gates, expose the canonical native
header at `tempo2/tempo2.h` if the disposable checkout lacks it. Use the release's
Linux-native fitting observer and its native-corpora S expected files (or an
explicitly identified accepted S bundle). `verify.py` creates
its own read-only runtime aliases next to the crate exports; it never writes A2.

```sh
# Only in an export without the usual canonical-native directory:
mkdir -p tempo2
ln -s "$AN_A2/source/tempo2.h" tempo2/tempo2.h

python3 validation/allocation_profile/verify.py \
  --before "$AN_OUT/before" --after "$AN_OUT/after" --a2 "$AN_A2" \
  --lifecycle-expected "$AN_S_EXPECTED" \
  --native-fit "$AN_RELEASE/native-fitting/fit" --out "$AN_OUT/verify"

python3 validation/allocation_profile/predictor_gate.py \
  --out "$AN_OUT/predictor-after" --a2 "$AN_A2" \
  --native "$AN_RELEASE/science/predictor" \
  --rust "$AN_OUT/after/bin/tempo2-rust" \
  --abi-receipt "$AN_RELEASE/c-default/receipt.json"
```

Repeat the predictor command with `before` and a fresh output directory if any
native gate fails, retaining both outcomes. The two macOS inherited differences
are documented in the report; Linux must record its actual result. Do not relabel
native failures as passes merely because same-engine equality holds.

The same existing provider controls can run against the frozen after source:

```sh
export CALCEPH_DIR="$AN_RELEASE/providers/prefix"
export TEMPO2_OPENBLAS=1
export CARGO_TARGET_DIR="$AN_OUT/provider-target"
cargo test --manifest-path "$AN_OUT/after/source/rust/Cargo.toml" \
  --locked --offline --release --features calceph,lapack \
  --test native_backends -- --test-threads=1

export CARGO_TARGET_DIR="$AN_OUT/debug-target"
cargo test --manifest-path "$AN_OUT/after/source/rust/Cargo.toml" \
  --locked --offline --lib pulsar_lifecycle_tests:: -- --test-threads=1
```

`check_build_paths.py NEW_OUTPUT_DIRECTORY` is a small real-Cargo control: it
builds two dependency-free fixtures with an inherited shared target and a prepared
cache, proves the targets stay distinct and the first binary is unchanged, and
records the effective environments. It requires no benchmark or science replay.

The CALCEPH real-ephemeris test is intentionally ignored by that existing test
selection; the four enabled matrix consumers are the affected provider controls.
These commands are the macOS supplement commands with Linux provider/compiler
selection. Keep stdout/stderr and the exact command/environment in the release
receipt when central runs them.

## Minimal integration wiring

No Cargo feature, build glue, installed interface or production mode needs wiring.
Central can cherry-pick the authored AN commits, review the two local unsafe
boundaries, and let the existing source audit enumerate the changed matrix body.
For Linux evidence, make the AN before revision available to `git archive`, pass
the existing A2/core/native-fitting/predictor/ABI paths above, and retain the new
AN evidence directory with the integrated run's artifacts. The accepted S corpus
is needed only for the historical native lifecycle gate; it is not generated or
modified by AN. Use the ordinary integrated scientific matrix for the final
combined candidate and the unchanged user-adopted paired SVD policy.

For an integrated Linux run with the usual layout, central can invoke the build
and measurement block after the default A2/core stages with `AN_RELEASE=/tmp/t2/linux`,
using the same exported `CARGO_HOME` for its existing fetch and these offline
builds. An inherited `CARGO_TARGET_DIR=/tmp/t2/linux/test-target` is safe: the
helper overrides it separately for each frozen export. The required existing
paths are `a2/{source,runtime,libtempo2-reference.a,receipt.json}`,
`core-disabled/tempo2`, `native-fitting/fit`, `science/predictor`,
`c-default/receipt.json`, `providers/prefix`, plus
`native-corpora/staging/validation/pulsar_lifecycle/expected` (or the identified
accepted S bundle) for the historical native S gate. Build both revisions before timed processes;
record the new AN directory in the existing artifact-retention step.

No existing workflow file or evidence collector was edited here. Any addition of
the optional bounded AN step/retention path to central's qualification orchestration
belongs to central. A two-platform improvement claim requires the actual Linux
receipts, not this portable-looking recipe.
