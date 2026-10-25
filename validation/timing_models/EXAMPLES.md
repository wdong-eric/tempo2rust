# AD public executable acceptance examples

These commands are frozen targets before timing consumers are implemented.
Run from the repository root with the AC runtime and each built profile:

```sh
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

Accepted G files are read-only inputs. Reuse both mean settings, every named
model/derivative/update case and original undefined-source classifications.
Success of these 15 commands alone is insufficient for AD: the full 0070 public
matrix also requires higher spin/state switches/braking/glitches, phase/FD events,
waves/interpolation/shapelets, GW including eccentric closure, tracking/pn,
deleted/unsorted/cut rows, TZR, averaging/stored noise, multiple pulsars,
ecliptic/time variants, repeated calls and failure then valid work. Add authored
fixtures here as their source branches are transferred, before Rust comparison.

Native/G outputs and accepted AC artifacts must never be overwritten. All new
executables, logs, JSON and corrected-native patches/results go in `build/`.
