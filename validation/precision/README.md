# Assignment B: isolated precision study

Read [REPORT.md](REPORT.md) for the recommendation and [BUDGET.md](BUDGET.md)
for gates established before the numerical run. All work is confined to this
folder. There is no production crate or backend selector.

From this directory, with stable Rust, Python 3 and a C toolchain:

```sh
cd /Users/Eric.Dong/Downloads/tempo2-rust/validation/precision
export CARGO_HOME="$PWD/.cargo-home"
export GMP_MPFR_SYS_CACHE="$PWD/target/gmp-cache"
cargo fetch --locked
cargo fetch --locked --manifest-path probes/quadmath/Cargo.toml
python3 run.py
```

The fetch commands require network access. `run.py` then runs offline. Cargo
builds bundled GMP 6.3.0 / MPFR 4.2.2 through gmp-mpfr-sys; the first build can
spend several minutes compiling and checking the native libraries. No Homebrew
installation, global Cargo configuration, production integration, or reference
source modification is performed. Caches and build outputs stay here.

`run.py` runs the experiment, records compiler/platform information, runs
bounded negative API/build probes, runs installed nightly/GCC probes if
available, checks formatting, records dependencies, and writes a numerical
summary. Expected compile failures are recorded with exit status rather than
mistaken for failed execution of the main study. Read `results/commands.json`
and each log for their outcome. The locally tested optional tools are
`nightly-2026-09-28` and Homebrew `gcc-16`; the script does not install them.

For just the numeric experiment with cached dependencies:

```sh
cargo run --release --locked --offline
python3 summarize.py
```

`src/main.rs` writes `results/numerical.tsv`, `parsing.tsv`, `special.tsv`, and
`timing.tsv`. It asserts the Rug128 gates, MPFR256/512 convergence, lossless
round trips, invalid-input handling, exact half ties and selected diagnosed
qd behavior. qd/control numerical failures are data, not an overall successful
backend verdict. The pre-written budget is not relaxed to accommodate failures.

A's four final JSON exports (including the explicitly provisional terrestrial
case), their losslessly derived TSV, and final report are included under `results/reference_inputs/`, with source paths
and SHA-256 hashes in `provenance.json`. The main run uses those snapshots, so
later work by A does not silently change B's comparison. To deliberately refresh
from A's completed exports, run `python3 snapshot_reference.py`, then rerun the
study and review changed results. That script reads A and writes only here.
The original TIM decimal inputs are read from the pinned reference checkout;
`results/artifact_hashes.json` records their identities.

The full `numerical.tsv` records candidate, expected value, absolute error,
units, tolerance and pass/fail without an f64 comparison bottleneck. Values can
have 156 digits because the comparison uses 512-bit MPFR. `summary.md` reduces
them with Python Decimal rather than binary64. `OPERATION_SUPPORT.md` maps
required operations to source routines and inspected dependency implementations.
