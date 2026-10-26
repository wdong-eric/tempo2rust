# AG analysis tools replay

This directory owns new evidence only. The pinned scientific source is
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`; accepted AE/AD observer objects and
reference archive are reused with checked SHA-256 identities. `native.py` writes
isolated source copies, explicit correction diffs, commands, object/executable
identities and compiler logs. It never edits canonical or accepted source.

From the repository root, with the released local reference dependencies present:

```sh
export CARGO_HOME="$PWD/rust/.cargo-home"
cargo build --manifest-path rust/Cargo.toml --locked --offline --examples
cargo build --manifest-path rust/Cargo.toml --locked --offline --release --examples
python3 validation/analysis_tools/native.py validation/analysis_tools/build/replay-native
python3 validation/analysis_tools/native.py validation/analysis_tools/build/replay-original --original
python3 validation/analysis_tools/run.py --profile debug --native validation/analysis_tools/build/replay-native/analysis --original validation/analysis_tools/build/replay-original/analysis --output validation/analysis_tools/outputs/replay-debug
python3 validation/analysis_tools/run.py --profile release --native validation/analysis_tools/build/replay-native/analysis --original validation/analysis_tools/build/replay-original/analysis --output validation/analysis_tools/outputs/replay-release
python3 validation/analysis_tools/regression.py --profile debug --output validation/analysis_tools/build/replay-regression-debug
python3 validation/analysis_tools/regression.py --profile release --output validation/analysis_tools/build/replay-regression-release
cargo test --manifest-path rust/Cargo.toml --locked --offline --lib --test analysis_tools
cargo test --manifest-path rust/Cargo.toml --locked --offline --release --lib --test analysis_tools
cargo fmt --manifest-path rust/Cargo.toml --check
cargo clippy --manifest-path rust/Cargo.toml --locked --offline --all-targets -- -D warnings
```

Destinations must be new. Native compilation is the observed GCC 16.2.0 arm64
`-O2` profile with A2 binary128; driver-only calculations use `-ffp-contract=off`.
This recipe does not qualify Linux or installed consumers. `layout.cc` is the
preimplementation native wire witness. `weighted_bug.cc` is the earlier independent
two-column normal-equation witness; `run.py` also retains original/corrected outputs.

`run.py` fails on process errors, scientific comparison failures, missing fields,
unit/class/sign mismatches, exchange byte/cursor errors, diagnostic file differences,
or accepted corruption controls. Native anisotropy warning counts are measured
from captured stdout. Exact f32/f64 min/max projections compare source-width bits,
not decimal-text identity or arithmetic tolerance. Twelve corruption controls cover
all four extrema, RNG state/draw count, associated sort, frequency/count/unit,
polarization and omitted membership.

`regression.py` imports affected AE comparators with fresh AG output roots. Its
local legacy-link stubs supply three observer-only callbacks missing from that old
probe's link against the later AE objects. Scientific code and accepted files stay
unchanged. It exits nonzero on collected numerical failures.

TSV projections are `key<TAB>unit<TAB>value`; the comparator rejects duplicate keys
and unknown units. The public examples emit warnings on stderr. See
`rust/ANALYSIS_TOOLS_API.md` for the actual timing/epoch/unit recipes and
`rust/ANALYSIS_TOOLS_REPORT.md` for measured results and the inherited open ledger.
Generated binaries, source copies, JSON, logs and wire files are intentionally ignored.
