# AH core CLI/output validation

The frozen comparison rules are in CONTRACT.md. The current source/routine,
type, state, unsafe and branch map is `../../rust/CORE_OUTPUT_SOURCE_MAP.md`.
Generated executables, receipts, logs, copied native sources and raw outputs stay
under ignored `build/`; no accepted predecessor evidence is overwritten.

## Build and run

Run from this directory. The existing reference, AD and AF build artifacts are
required; `native_build.py` verifies their object/archive identities. It uses the
accepted AF `build/native-corrected` dependency, whose executable identity is
`3bf82ca766e45599c23a575868d6977d97c2f25c46428c02c877ccca0276a27f`, including the
minimum-width Doppler reader correction. GCC16 paths and the configured runtime
are explicit. This is the recorded macOS profile, not a portable installer.

```sh
cargo build --manifest-path ../../rust/Cargo.toml
cargo build --manifest-path ../../rust/Cargo.toml --release
python3 native_build.py build/native-replay
python3 test_check.py
python3 check.py --native build/native-replay --out build/debug-replay
python3 check.py --native build/native-replay --out build/release-replay --release
python3 witness_bugs.py --native build/native-replay --out build/bugs-replay
```

The two `check.py` commands currently **exit 1** because their receipts retain
real numerical/display mismatches and source-unwritten boundaries. A zero exit
is not manufactured by excluding known failures. Exit failures, timeouts,
launch failures, changed inputs, missing/extra files and byte mismatches all
contribute to failure. The receipt publishes each category/count and source,
input, executable, native-builder, contract and runtime identities. No scientific
numbers are normalized. Only log-prefix source locations and the final CPU-time
line in stdout/stderr are normalized; data files are always exact.

`test_check.py` exercises actual checker subprocess exits for omitted/reordered
records, units, flags, digits and missing files, plus actual unexpected process
exit/timeout rejection, valid recovery and a guard that metadata normalization
cannot erase data-file differences. It does not test a second comparator.

`check.py` also executes the ignored Rust direct-API evidence test itself. That
test checks a repeated report, a failing stdout followed by successful output
on the same canonical context, the empty-filename prompt and actual PAR reload.
The native logger runs with stdout attached to a real PTY as well as redirected;
only the PTY's newline translation is disabled. The original helper and poison
probes diagnose concrete source bugs; undefined original bytes are never goldens.

The normal native executable uses the source entry point. `tempo2-bounded` is a
separate test driver setting only default MAX_PSR=2 and MAX_OBSN=100, so argc=2
standalone TIM/help grammar can be exercised without adding command arguments
or allocating the native 40-by-20000 observation capacity. Its distinct identity
is recorded. Original argument-error witnesses have a two-second timeout, a
1-MiB output-file limit and disabled core dumps; complete bounded prefixes and
termination statuses remain in receipts.

## UT1 correction evidence

`build/ut1-before` was recorded before the producer edit; `build/ut1-after`
was recorded after it. The current ignored test can reproduce the latter:

```sh
AH_UT1_DIR="$PWD/build/ut1-after-replay" cargo test --manifest-path ../../rust/Cargo.toml core_output_tests::write_ut1_evidence -- --ignored --test-threads=1
python3 check_ut1.py --native build/native-replay --before build/ut1-before --after build/ut1-after-replay --out build/ut1-replay
```

The original Rust state is retained evidence, not regenerated from corrected
code. `check_ut1.py` hashes both snapshots and independently executes original
and corrected native producers with defined poisoned UT1 storage. It checks the
complete site-clock-plus-EOP value and unchanged BAT/residual/site geometry for
clock modes 0/1/2. Rust snapshots compare every other canonical observation field
exactly. Native original/corrected source and patch hashes are in native identity.

## Development-only source transfer

Cargo needs no Python parser. To regenerate the committed full report transfer
or inspect every active source decision, install the pinned public parser into
a disposable directory (or use an already installed pycparser 2.23):

```sh
python3 -m pip install --target build/python -r requirements-dev.txt
PYTHONPATH=build/python python3 translate.py
cargo fmt --manifest-path ../../rust/Cargo.toml
PYTHONPATH=build/python python3 source_inventory.py build/source-inventory.json
```

The executed inventory used the existing pycparser 2.23 under AD's ignored
`validation/timing_models/build/tooling`; no dependency was installed for AH.
The parser preserves explicit C expression types/grouping in emitted Rust.
The inventory accounts for six bodies and 375 branch/loop/case sites. It is a
static correspondence ledger, not dynamic coverage or independent physics.

Latest authoritative run names and exact identities are indexed in
`../../rust/CORE_OUTPUT_REPORT.md`. Earlier `debug-v*`, `release-v*`, native-v1
through native-v8 and snapshot attempts remain historical evidence. In
particular v1-v6 native builds used the superseded AF dependency selection and
are not the delivery oracle.
