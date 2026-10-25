# AD runnable fixed-parameter timing examples

Run from the repository root. The existing pinned reference/runtime and offline
Cargo dependencies are required; no fixture generator supplies production values.

```sh
export CARGO_HOME="$PWD/rust/.cargo-home"
export GMP_MPFR_SYS_CACHE="$PWD/rust/target/gmp-cache"
cargo build --manifest-path rust/Cargo.toml --locked --offline --all-targets
cargo build --manifest-path rust/Cargo.toml --locked --offline --release --all-targets

for model in bt btj btx ell1 dd ddk dds mss ddgr t2 t2-pta ddh ell1h ell1k; do
  rust/target/debug/tempo2-rust timing \
    --par validation/binary/fixtures/${model}_baseline.par \
    --tim validation/binary/fixtures/barycentric.tim \
    --runtime validation/reference/build/source/T2runtime
done
rust/target/debug/tempo2-rust timing \
  --par validation/binary/fixtures/pks_dd.par \
  --tim validation/binary/fixtures/pks.tim \
  --runtime validation/reference/build/source/T2runtime --no-mean
```

The default subtracts the residual mean, not phase or pulse counts. Both settings
are tested. `timing` evaluates the supplied fixed model and performs no fit.

## Canonical library lifetimes

`rust/examples/timing_lifecycle.rs` uses public APIs: load, preprocess, canonical
state control, and timing evaluation. It records control files in provenance.
The modes below respectively supply source DF1/switch arrays, measured/stored
noise signals, and eccentric-GW controls after a real prior residual calculation.
They do not supply fabricated Earth/site vectors or bypass BAT preparation.

```sh
rust/target/debug/examples/timing_lifecycle state \
  validation/reference/build/source/T2runtime \
  validation/timing_models/fixtures/library/base.par \
  validation/timing_models/fixtures/library/base.tim \
  validation/timing_models/fixtures/library/switches.txt
rust/target/debug/examples/timing_lifecycle eccentric \
  validation/reference/build/source/T2runtime \
  validation/timing_models/fixtures/library/base.par \
  validation/timing_models/fixtures/library/base.tim \
  validation/timing_models/fixtures/library/eccentric-both.par --no-mean
```

Noise and ordinary averaging write `avg.dat` in the working directory. The
comparison runner executes those cases in separate owned `build/` directories.
A caller invoking them directly should choose its intended output directory.
Noise subtraction consumes supplied signals; it does not estimate a noise model.

## Authored public matrix

`check_public.py` independently executes native and Rust pipelines in fresh
processes. Every listed case runs with removeMean 0 and 1, in both profiles.
Cases and finite gates are explicit in the script; changes require source evidence.

| Fixture group | Source paths exercised |
| --- | --- |
| Fourteen G baselines, `pks_dd`, `t2-companions` | Model delays, real terrestrial geometry, natural multiple-companion state |
| `ordinary`, `ecliptic`, `tempo1` | AC site/clock/time/frame choices |
| `high-spin`, `braking`, library `state` | F2–F12, braking index, DF1 and state-switch epochs |
| `glitches`, `nudot`, `events` | Glitch phase/frequency terms, all three recoveries, nudot transitions, exponential/Gaussian events |
| `jumps-fd`, `flags-phase`, `iperharm` | Jump/FD/FDJUMP, radd/padd, phase jumps, fractional harmonic adjustment |
| `waves`, `wave-dm-scale`, `clock-offsets` | WAVE/WAV_DM, source frequency scaling, clock offsets |
| `ifunc-linear`, `ifunc-step`, `ifunc-sinc`, `quad-interpolation`, `shapelet` | Scalar/quadrupole interpolation and red shapelets |
| `quadrupole`, `gw-single`, `gw-continuous` | Periodic GW phase contributions and source distance choices |
| `gw-memory`, `gw-memory-polarizations`, `gw-burst`, `gw-cosmic-string` | Transient GW branches |
| Library eccentric earth, both, pulsar, coalescence | Six-routine closure, prior residual producer, integration and coalescence/cap path; diagnostic parameters, not an astrophysical validation |
| `positive-tracking`, `pulse-tracking`, `gradient-tracking` | Positive TRACK, -2 supplied pn/pnadd, -1 gradient tracking |
| `cut-sat`, `cut-bat`, `deleted-controls` | Source reference selectors, deletion/selection, original row identity |
| `tzr`, `tzr-averaging` | Complete separate TZR record and active-only averaged-field membership |
| `averaging`, library `noise` | Source avg.dat bytes, ECORR/SECORR, all four stored-noise signals, DM averages at 1400 MHz |
| `two-cut`, `two-shapelet`, `two-averaging`, `two-tzr` | Distinct current-pulsar state and retained outer-loop lifetimes |
| `ell1-step`, `ell1h-h3`, `ell1h-h4`, `t2-h4` | Separately corrected source branches; original G invalid values remain withheld |

These are representative executions of complete transferred routines, not every
possible parameter combination or proof of the physical models.

## Reproduction

```sh
# Rebuild only disposable AD native staging from the pin and verified AC recipe.
python3 validation/timing_models/native_timing.py > validation/timing_models/build/native-build-final.log 2>&1
python3 validation/timing_models/corrected_binary.py > validation/timing_models/build/corrected-binary-final.log 2>&1
python3 validation/timing_models/check_corrections.py > validation/timing_models/build/corrections-final.log 2>&1
python3 validation/timing_models/check_cross_pulsar.py > validation/timing_models/build/cross-pulsar.log 2>&1
python3 validation/timing_models/check_public.py > validation/timing_models/build/public-debug-final.log 2>&1
python3 validation/timing_models/check_public.py --release > validation/timing_models/build/public-release-final.log 2>&1
python3 validation/timing_models/check_comparator.py > validation/timing_models/build/comparator-controls.log 2>&1
python3 validation/timing_models/recompare_public.py > validation/timing_models/build/recomparison.log 2>&1
python3 validation/timing_models/check_cli_recovery.py > validation/timing_models/build/cli-recovery.log 2>&1

# Both profiles: normal tests, original G, corrected G, explicit lifecycle tests.
# Original G invalid records remain withheld; corrected evidence stays separate.
python3 validation/timing_models/check_components.py > validation/timing_models/build/components.log 2>&1
python3 validation/timing_models/check_compatibility.py > validation/timing_models/build/compatibility.log 2>&1
# The preceding command intentionally exits 1 while M2's four original gates fail.
cargo fmt --manifest-path rust/Cargo.toml --check > validation/timing_models/build/fmt-final.log 2>&1
cargo clippy --manifest-path rust/Cargo.toml --locked --offline --all-targets -- -D warnings > validation/timing_models/build/clippy-final.log 2>&1
python3 validation/timing_models/write_receipt.py
```

The receipt writer indexes the named final execution logs and checks source,
executable and output identities. It is an evidence index, not a test runner.
The compatibility script runs original M1a/M1b comparators, affected M2 callbacks
and fit workflows, resource recovery and the AC lifecycle example. It does not
rerun the unchanged full P corpus; 0077 assigns its numerical disposition centrally.

All new native patches, executables, compiler commands, hashes, original/corrected
proofs and comparisons stay in ignored `build/`. Original G/A2/AC outputs and
canonical TEMPO2 files are read-only. The native row observer uses unique fixture
filenames to recover original input indices; production row identity has no such
restriction. Native diagnostic file paths/line numbers are mapped to the raw pin;
all other diagnostic bytes are compared exactly.

`check_comparator.py` verifies the fixed six-key averaged schema, exact unit
strings and null membership with eleven mutation controls. `recompare_public.py`
reads existing output bytes and writes separately identified recomparison receipts;
it runs neither native nor Rust pipelines and preserves the original receipts.
This supports comparator-only corrections without an unnecessary numerical replay.
