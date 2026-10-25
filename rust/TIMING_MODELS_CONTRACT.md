# AD interface and implementation contract

Released by 0075 (`e46465a`) on AC production `18c9065`, report `d47ae51`.
AC handover is approved and its ownership closed; numerical acceptance is pending.
TEMPO2 source pin: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.

## Canonical public interfaces

Reuse `bats::{InputPair, BatInput, load_pairs, preprocess}` and `RuntimeContext`.
No additional parameter/state store or numerical backend is introduced.

```rust,ignore
pub fn timing::evaluate(input: &mut BatInput, runtime: &mut RuntimeContext,
                        remove_mean: bool) -> Result<TimingResult, TimingError>;
pub fn timing::form_residuals(context: &mut PulsarContext,
                             remove_mean: bool) -> Result<(), TimingError>;
pub fn binary::model(context: &mut PulsarContext, pulsar: usize,
                     observation: usize, selector: i32, subindex: usize)
                     -> Result<Float, TimingError>;
pub fn binary::update(context: &mut PulsarContext, pulsar: usize,
                      label: Label, subindex: usize, value: f64, error: f64)
                      -> Result<(), TimingError>;
pub fn binary::fit_function(context: &mut PulsarContext, pulsar: usize,
                            observation: usize, label: Label, subindex: usize)
                            -> Result<f64, TimingError>;
```

`selector=-1` requests delay. Source-specific selectors (including DDGR -2)
retain their original meanings. DD returns extended precision; the fit wrapper
narrows to f64. All callers borrow the same canonical parameter and observation
owners. Direct routines do not redo BAT preparation. Errors preserve completed
source writes and are recoverable with a fresh input and reusable runtime.

CLI: `timing --par P --tim T [--par P --tim T ...] --runtime ROOT
[--no-mean] [-- PREPROCESS_ARGS]`. Default removeMean=1. No fitting occurs.

## Frozen output projection

Version 1 retains AC's byte-lossless inputs/resources/diagnostics and ordered
pulsar metadata. Each observation contains AC's BAT projection plus PET (MJD),
binary delay (s), phase/nphase (turn), exact signed pulse number, residual and
residualtn (s), and applicable source averaged fields with explicit units.
TZR is a separate complete record. Optional unwritten/inapplicable fields are
null; nonfinite results use AC's signed class representation. Main required
fields must be written. Publish JSON only after calculation and serialization.
Source parameter mutations are projected by source label and subindex with
values/errors/set flags. Mean removal never implies shifting phase or counts.

## Implementation order and completion

1. Freeze this API, source/projection maps and executable public input recipes.
2. Transfer 14 binary modules, 42 bodies and two fit wrapper callbacks; preserve
   each source's operation order, precision, derivative and update coordinates.
3. Replace the selected residual kernel with full formResiduals, two averaging
   routines and GWsim's six-body eccentric closure; retain canonical ownership.
4. Integrate public timing and compatible bounded adapters over shared routines.
5. Run G component comparisons and independently prepared native/public examples
   in debug/release; affected bounded/AC checks, failure/recovery, fmt and Clippy.
6. Deliver source/unsafe/departure maps, execution identities and unresolved gates.

Every item in 0070 remains required, including phase/state/glitch/GW/noise,
tracking/TZR/averaging, multiple pulsars, frames, sites and time choices. Public
input recipes are in `validation/timing_models/EXAMPLES.md`; they are executable
acceptance targets, not claims that unimplemented paths currently pass.

## Comparison policy and inherited ledger

Freeze native inputs/state/projections before inspecting Rust values. Reuse G's
original records and classifications unchanged. Delay/residual/MJD-in-seconds
budget is 1e-12 s; phase 1e-10 turns; columns 5e-14 times each reference column's
maximum absolute value; fixed extended update values/errors 1e-30 relative with
separate increment/unit/response checks. Exact structural/selector/count/nonfinite
membership gates remain. No precision switch, new blanket tolerance or compensation.

Inherited failing gates: P SIMUL 32, units 71, ordered SAT offsets 2, linked
preprocessing 4, epoch transforms 45; M2 four exact raw-order cases; four 0049
utility boundaries; DDH original 1 ns test. Original AB's 277 differences remain
separate from 0075-approved corrected two-pulsar behavior. Localized policy issues
may remain under 0074 but must never be reported as passed.

Owned paths: `rust/`, `validation/timing_models/`. Preserve central files,
original 0008 bytes, accepted evidence, canonical TEMPO2 and independent reference.
Generated outputs stay under ignored `build/`. Corrected native experiments get
separate pinned patches and evidence under 0053; all unsafe must be justified.
