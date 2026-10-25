# General fitting examples

Run from the repository root. Runtime is `validation/reference/build/source/T2runtime`
when using the existing A2 build; verify the actual runtime path on the host.

```sh
cargo run --manifest-path rust/Cargo.toml -- timing --fit \
  --par validation/fitting/inputs/all.par --tim validation/fitting/inputs/bary.tim \
  --runtime validation/reference/build/source/T2runtime
cargo run --manifest-path rust/Cargo.toml -- timing --fit \
  --par validation/fitting/inputs/f0.par --tim validation/fitting/inputs/bary.tim \
  --par validation/fitting/inputs/f1.par --tim validation/fitting/inputs/bary.tim \
  --runtime validation/reference/build/source/T2runtime --norescale
```

The final verification runner will record the actual runtime, source, input and
executable hashes under ignored `build/`. Examples are authored before comparing
Rust; executed outcomes and added family fixtures are recorded in the report.

The direct examples `fitting_legacy`, `fitting_covariance_models`, `fitting_spectrum`
and `fitting_kernels` expose standalone source responsibilities. Build them with
`cargo build --manifest-path rust/Cargo.toml --locked --offline --examples`.
Run their native comparisons with `check_legacy.py`, `check_covariance_models.py`,
`check_spectral_session.py` and `check_kernels.py` in this directory; each accepts
`--profile debug` or `--profile release` and requires the corresponding built example.

`fitting_ideal RUNTIME PAR TIM` is a direct-library example of the upstream
checkIdeal lifecycle (simple preprocessing, no mean removal, one fit). It leaves
the general CLI's mean-removal contract intact. `check_ddh.py` constructs the
single-file historical GPS overlay and reports both 0087 checks separately;
`check_pipeline_solvers.py` compares the resulting native and Rust solver inputs
through both standalone solvers. These scripts preserve failed gates in their
receipts; an exit of zero alone is not a numerical acceptance result.
