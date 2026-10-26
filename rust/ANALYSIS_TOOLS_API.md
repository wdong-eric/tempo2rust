# AG public contract freeze

Frozen before Rust comparisons at release 1310dba (AF 7420622 / 543760f /
7a94491), raw TEMPO2 fda6ad4a9753ad7dc5d291406f2f310a9161423f.
All routines are safe Rust; installed packaging remains AJ. No new numerical
precision, dependency, parameter store, native timing wrapper or unsafe is planned.

## Owners and public inputs

* `toolkit`: source-named snake-case f32/f64 slice operations. Array lengths are
  explicit backing; source undefined empty reads return errors. Defined empty
  reductions retain NaN/zero. `ToolkitRng` owns 624 u32 words and index, exposes
  init_genrand, genrand_int32, genrand_real1, ran_dev, gauss_dev, snapshot and draw
  count. Negative i64 seed resets then writes 1; i64::MIN is an explicit signed
  negation boundary. `set_seed_at(i64)` supplies the controlled-clock witness;
  `set_seed()` reads wall-clock seconds. No Gaussian caching. Existing bootstrap
  ran1 remains its different source algorithm. `max_observations()` reads the
  canonical process setting, not another toolkit capacity.
* `spectrum`: `Options` maps all TKspectrum integer/scalar controls. `analyze`
  mutates x/y/errors, returns scalar, written arrays, reported count, diagnostic
  table/status and warnings separately. Helpers retain source names and counts.
  `cholesky_spectrum` remains the canonical Lomb/weighted/covariance implementation;
  its existing periodogram becomes a thin adapter to the full driver.
  `SpectralSession` owns GLOBAL_OMEGA and direct ordinary/IFUNC callbacks;
  `calc_spectra_ri` retains delegation to the two-column complex route. Explicit
  diagnostic options use AE's solver. `SigmaZ` owns the translated one-based
  arrays/history and provides readin/getprtj/getweights/fit4/calculate;
  standalone indexx8/mat20 remain public. Input is canonical context/pulsar,
  all active observations including deleted rows as in source, SAT/residual/
  toaErr conversions and source sorting, not a generic selected fitting view.
* `gw_sim`: `TensorSource`, `GeneralSource`, `Geometry`, `GeneralSpectrum`,
  `PopulationOptions`, `Harmonic`, `SkyDistribution`, `GwSession` (RNG and
  gwsim_Ngrid=1000) own source state. Generation writes only source-defined
  primary fields; setup computes geometry separately. Tensor/general residual
  methods accept Rug128 time seconds and distance metres. Generic matrix/dot,
  pulsar geometry, psrangle/sphharm/find_phi are public. Eccentric public owner
  wraps AD's canonical implementation plus its energy variant, preserving history.
* `gw_file`: caller-owned Read+Seek/Write streams, explicit
  `LittleEndianI32Binary128` profile, tensor/general source prefixes only. Read
  updates caller source rows only after a complete scalar/record; error reports
  completed prefix and stream cursor remains observable. Missing realizations,
  invalid counts and truncation error. Nonserialized fields retain prior state.
  New imports return primary records with geometry/metadata absent.

The user selected source positional uncertainties plus an explicit warning for
weighted averaging/spline/Hann routes on 2026-10-26. No error propagation is
invented. The direct x/y/error association bug is corrected locally under 0053.
If the source would read beyond actual error backing (e.g. 4096 spline rows from
fewer supplied errors), return an explicit backing error; do not fabricate errors.

## Executable recipes

From repository root (both debug and `--release`):

```
cargo run --manifest-path rust/Cargo.toml --example toolkit -- --seed -12345 --draws 1300
cargo run --manifest-path rust/Cargo.toml --example spectrum -- --par validation/general_fitting/inputs/robust-H.par --tim validation/general_fitting/inputs/multi-frequency.tim --runtime validation/reference/T2runtime --estimator 2 --output-kind 1 --prewhite 0 --weights -1 --mintau 0.01
cargo run --manifest-path rust/Cargo.toml --example gw_simulation -- --par validation/general_fitting/inputs/robust-H.par --tim validation/general_fitting/inputs/multi-frequency.tim --runtime validation/reference/T2runtime --seed -12345 --count 4 --flo 1e-9 --fhi 1e-7 --amplitude 1e-15 --alpha -0.6666666666666666 --distance 3.08568e19 --output validation/analysis_tools/build/background
```

The final evidence records resolved runtime and fixture identities. Spectrum
uses explicit prefit canonical timing with removeMean=true, x=(SAT-PEPOCH0)
days, y=residual seconds, e=toaErr*1e-6 seconds; no silent timing fit. Covariance
example uses a diagonal measurement covariance and source two/three-column
routes. Row identities/deletion and all effective controls are exported.
GW uses (SAT-PEPOCH0)*86400 seconds, RAJ/DECJ radians, setups before sums, and
never mutates timing TOAs. It executes tensor, all four general families,
anisotropic and dipole routes, both frequency samplings, and writes/reads native
backgrounds with multiple nonconsecutive realization IDs.
