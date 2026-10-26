# Predictor APIs and runnable consumers

AF implements decision 0076 on the existing canonical timing engine. Numeric
acceptance and platform limits are listed in `PREDICTOR_REPORT.md`.
Commands below run from the repository root after `cargo build --manifest-path
rust/Cargo.toml --release --bins --examples`.

```sh
rust/target/release/tempo2-rust predictor cheby \
  --par validation/predictors/fixtures/binary.par \
  --runtime validation/reference/build/source/T2runtime --site PKS \
  --start-mjd 58000 --end-mjd 58000.1 \
  --freq-start-mhz 1000 --freq-end-mhz 2000 \
  --time-coeffs 8 --freq-coeffs 4 --segment-seconds 3600 \
  --output-dir validation/predictors/build/example-cheby

rust/target/release/tempo2-rust predictor polyco \
  --par validation/predictors/fixtures/tempo1.par \
  --runtime validation/reference/build/source/T2runtime --site PKS \
  --start-mjd 58000 --end-mjd 58000 --span-minutes 60 \
  --coefficients 12 --max-hour-angle 1 --frequency-mhz 1400 \
  --output-dir validation/predictors/build/example-polyco -- -tempo1

rust/target/release/tempo2-rust predictor evaluate \
  --input validation/predictors/build/example-cheby/t2pred.dat \
  --mjd 58000.05 --frequency-mhz 1400

rust/target/release/tempo2-rust predictor plan \
  --input validation/predictors/build/example-cheby/t2pred.dat \
  --start-mjd 58000 --end-mjd 58000.1 --step-seconds 1000 --frequency-mhz 1400
```

Generation reads PAR and preprocesses it without reading TIM or performing a
parameter fit. It writes source-format `t2pred.dat` and `pred.tim`, or
`polyco_new.dat`, `newpolyco.dat` and `polyco.tim`. JSON success follows completed
output writes. Failure can leave partial files; there is no atomic-publication
promise. Existing files at the chosen output location are replaced.
The trailing `--` passes canonical preprocessing options. An explicitly supplied
EPHVER can override `-tempo1` in the source preprocessing routine; the authored
TEMPO1 fixture uses EPHVER 2 and TDB.

CLI missing-TZR defaults are PEPOCH, 1400 MHz and the requested site when
TZRSITE is `NULL`. Odd Cheby dimensions round upward; overlap is 10% of the
requested segment length. Direct set construction retains supplied dimensions
and overlap and uses start MJD/start frequency/empty-site defaults. These caller
contracts are deliberately separate.

## Safe library interfaces

All modules are exported by `tempo2_rust`; predictor types are owned Rust
objects, not C ABI layouts. Native `PredictorNumber` has 53 significand bits on
macOS arm64 and 64 on Linux x86_64. Timing `Scalar`/`Float` retains Rug128.
No runtime precision setting exists.

| API | Contract |
| --- | --- |
| `predictor::load_par(par, runtime, arguments)` | Returns canonical mutable context and consumed inputs; PAR-only lifecycle/preprocessing. |
| `predictor::cli_defaults`, `spin_frequency`, `snapshot` | Explicit caller defaults, F0, and retained SAT/BAT/phase/frequency state. |
| `Cheby2D::{new,from_coefficients,construct,evaluate,x_derivative,test}` | Callback accepts native x/y slices and returns y-major/x-minor values; owned dimensions/coefficients. Test uses the source off-node midpoint grid. |
| `ChebyModel::{phase,frequency,read,write}` | Time in MJD, observing frequency in MHz, phase in turns, derivative in Hz; source dispersion separation/restoration. |
| `ChebyModelSet::{nearest_index,insert,keep,read,write}` | Ordered append; inclusive coverage; nearest center with first tie; keep preserves source order. Clone/Drop implement deep ownership. |
| `cheby2d_int::{cheby_model_func,construct_model,test_model,construct_set,test_set}` | Mutates canonical context and runtime, accepts explicit TIM output path. `ChebyRequest` supplies direct ranges, dimensions and overlap. |
| `polyco::{polyco,atimfake,tz_fit,chebpc,pcshft}` | `PolycoRequest` supplies timing-valued MJD/frequency/hour angle, integer span/order, explicit true-DM mode and literal filename prefix. Source 31-point fit and coefficient transformations. |
| `T1Polyco::{new,with_coefficients,phase,frequency,read,read_new,write}` | Validated 2–32 stored coefficients; generator fits 2–31. Legacy D exponents and TEMPO2 format; write uses the legacy source grammar. Frequency intentionally ignores observing frequency. |
| `T1PolycoSet::{nearest_index,read,write}` | Nearest midpoint and extrapolation outside declared span; completed prefix survives malformed trailing input with `trailing_error`. |
| `T2Predictor::{read,read_stream,read_into,write,write_stream}` | Owned kind dispatch, original remaining input bytes/hash/format. Stream reader requires Read+Seek, restores position on failure and advances by consumed input on success. Failed `read_into` retains the prior object. |
| `T2Predictor::{metadata,evaluate,copy_from,insert,keep}` | None/empty backing errors; Cheby no-match returns phase/frequency -1 plus explicit status. T1 Copy/Insert/Keep return source-unimplemented errors. Clone remains ordinary Rust deep ownership. |
| `T2Predictor::plan`, `tempo2pred::{get_plan,get_plan_ext}` | Exact source ceiling, shortened final interval, phase0 and unwrapped consecutive-phase average frequencies; endpoint membership/status is retained. No timing runtime needed. |

`ChebyModel` metadata/ranges and derivative polynomial remain caller-visible,
matching direct source construction responsibilities. Use the constructors for
coefficient storage and maintain consistent model ranges/derivative state when
editing a model. Direct contexts must be initialized and PAR-preprocessed.

Executable API consumers:

```sh
rust/target/release/examples/predictor_components polynomial
rust/target/release/examples/predictor_subnormal
rust/target/release/examples/predictor_workflows \
  validation/predictors/fixtures/isolated.par \
  validation/reference/build/source/T2runtime \
  validation/predictors/build/direct-example
rust/target/release/examples/predictor_quality \
  validation/predictors/build/example-cheby/t2pred.dat \
  validation/predictors/fixtures/binary.par \
  validation/reference/build/source/T2runtime \
  validation/predictors/build/quality-example
```

The workflow consumer requires a fresh output directory. It covers direct odd
orders, failed output/read followed by recovery, repeated contexts, changed
sites, direct defaults, true-DM modes and source sample/order limits. The quality
consumer returns independent phase and centered one-second frequency diagnostics;
it does not change the requested approximation order.
