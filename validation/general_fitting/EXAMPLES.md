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

Verification runners record runtime, source, input and executable identities
under ignored `build/`. Executed outcomes and failed gates are in the report.

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

`fitting_standalone_callbacks RUNTIME PAR TIM` exercises sinc, GWB and standalone
glitch updates. `check_standalone_callbacks.py` supplies a fully declared model.
`check_families.py --profile debug` (or release) runs 58 natural PAR/TIM recipes;
its receipt records failed gates and explicit execution boundaries. The physical
update output separates immediate `after_value/error` from `postfit_value/error`
after noise projection.

`check_regressions.py` runs affected AC/AD/M1/M2 comparisons into AE-owned staging
without modifying accepted runners. `check_order.py`, `check_bootstrap_stack.py`,
`check_gw_boundary.py` and `check_contraction.py` retain focused source witnesses.
A successful script exit does not turn failures listed in its receipt into passes.

`fitting_svd` exposes the complete standalone decomposition, bidiagonalization,
extended-RHS back-substitution and norm/maximum helpers. `check_svd_api.py`
compares their native outputs without sorting singular modes.

`fitting_prepared RUNTIME PAR TIM` assembles the public callback/constraint rows,
applies the same solved coordinates and checks exact canonical postfit equality.
`fitting_recovery RUNTIME PAR TIM DIRECTORY` checks five same-process failure and
recovery histories. `fitting_covariance RUNTIME PAR TIM DIRECTORY recovery` adds
automatic-covariance failure/recovery and fresh-context/file equality.
`check_api_lifecycle.py --profile debug` or `release` runs these examples.

`fitting_callback_grid RUNTIME PAR TIM` evaluates scalar and telescope callbacks
without a solve. `check_callback_grid.py` supplies natural state and retains
FDDI/GLEP/clock-offset exits and GWECC null-callback checks.
`check_constraint_payloads.py` checks parameter and solar constraint scanner
grammar. `check_bootstrap_history.py` checks one/two-pulsar partial-copy history;
`check_bootstrap_capacity.py` checks IBOOT -1/0/12/13, guarding native capacity
before overflow. All accept `--profile debug` or `release`.

The regression runner creates a fresh `build/regressions/run-*` directory on
each invocation because the accepted lifecycle examples require unused scratch
paths. It preserves earlier runs. Comparator mutation controls are stored by
baseline case under `build/comparator-controls/`.
