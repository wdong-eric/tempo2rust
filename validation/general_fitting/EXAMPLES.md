# General fitting examples

Run from the repository root. Runtime is `validation/reference/build/tempo2/T2runtime`
when using the existing A2 build; verify the actual runtime path on the host.

```sh
cargo run --manifest-path rust/Cargo.toml -- timing --fit \
  --par validation/fitting/inputs/all.par --tim validation/fitting/inputs/bary.tim \
  --runtime tempo2/T2runtime
cargo run --manifest-path rust/Cargo.toml -- timing --fit \
  --par validation/fitting/inputs/f0.par --tim validation/fitting/inputs/bary.tim \
  --par validation/fitting/inputs/f1.par --tim validation/fitting/inputs/bary.tim \
  --runtime tempo2/T2runtime --norescale
```

The final verification runner will record the actual runtime, source, input and
executable hashes under ignored `build/`. Examples are authored before comparing
Rust; executed outcomes and added family fixtures are recorded in the report.
