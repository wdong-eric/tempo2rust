# Assignment AC implementation contract

Source: TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
Baseline: AA `ea9797236fae0f0dd29a32e5d3050e33eaf2fefe`, production `99cf714`.
Authority: decisions 0068/0069, 0072–0074 and handoff 29. The interfaces below
are implemented. Implementation handover and milestone acceptance remain central decisions.

## Record chronology and current implementation status

The proposed API and integration plan were first committed as
`rust/INPUT_TO_BAT_CONTRACT.md` in `4114aec`. The contract remains here, as confirmed by central after review. At that review,
initial source PAR/TIM
parsers and preprocessing primitives already existed as uncommitted implementation;
no public BAT evaluator or CLI consumer had been implemented. This records the
actual documentation chronology, not a retrospective claim that every parser edit
followed a frozen contract.

The concrete implemented loader is `bats::load_pairs(&[InputPair], &Path) ->
Result<BatInput, TimingError>`. `BatInput` owns the canonical `PulsarContext`, ordered
pairs, and a vector of `ConsumedInput { path: PathBuf, role: String, sha256: String,
bytes: Vec<u8> }`. `source_par::read_parfiles` and `source_tim::read_timfiles` populate
that context directly. The remaining signatures and version-1 output below are now implemented. The
“Concrete projection before public consumers” section recorded the actual output
shape before connecting the public evaluator and CLI. Execution results and open
gates are recorded in `INPUT_TO_BAT_REPORT.md`.

## Public boundary

The `bats` module exposes these operations (all fallible operations return
`Result<_, TimingError>`):

```rust,ignore
pub struct InputPair { pub par: PathBuf, pub tim: PathBuf }
pub struct BatInput {
    pub context: PulsarContext,
    pub pairs: Vec<InputPair>,
    pub inputs: Vec<ConsumedInput>,
}
pub fn load_pairs(pairs: &[InputPair], runtime: &Path) -> Result<BatInput, TimingError>;
pub fn preprocess(input: &mut BatInput, arguments: &[Vec<u8>], runtime: &Path)
    -> Result<(), TimingError>;
pub fn evaluate(input: &mut BatInput, runtime: &mut RuntimeContext)
    -> Result<BatResult, TimingError>;
pub fn form_bats_all(context: &mut PulsarContext, runtime: &mut RuntimeContext)
    -> Result<(), TimingError>;
pub fn update_bats_all(context: &mut PulsarContext, runtime: &mut RuntimeContext)
    -> Result<(), TimingError>;
```

The source reader, preprocessing stages and helpers remain callable independently
of CLI parsing. Mutable canonical `SourcePulsar`/`SourceObservation` state lives
only in `PulsarContext`; metadata records input bytes and identities, not a second
scientific store. File readers preserve bytes, scanner position and partial writes.
The process session owns mutable source globals; drivers take its existing outer
lock once and use scoped canonical borrows internally.

CLI: `bats --par P0 --tim T0 [--par P1 --tim T1 ...] --runtime ROOT [-- ARGS]`.
The nth PAR pairs with the nth TIM. Unequal counts fail before any file is read.
Output is serialized completely before stdout is written. Error output is not a
successful partial result. Existing load/residuals/fit interfaces and schemas stay
bounded to their declared input and model capabilities.

## Output schema

Version 1 contains `schema_version`, `status`, `engine`, `runtime_root`, `inputs`,
`resources`, `diagnostics`, and ordered `pulsars`. Each pulsar contains byte-lossless
`name`, actual `units`, `time_ephemeris`, `coordinate_frame`, `sorted`, ordered
`observations`, and optional separate `reference_observation`. Each observation
contains its canonical order/index, filename/site/flag bytes, deletion state,
SAT/BAT/BBAT in MJD, and correction components with explicit second/day/Hz units.
Extended values use round-trip decimal strings; nonfinite values have explicit
class representations. Provenance hashes the opened bytes actually consumed,
including INCLUDE, preprocessing, clock, EOP, time and planetary resources.

## Integration map and implementation sequence

1. This contract and authored examples freeze interfaces and comparison intent.
2. PAR/TIM/preprocessing: reuse K/Q/N/P maps in `validation/parameters`,
   `par_specialized`, `tim_io`, `preprocessing`. Extend `read_parfile`,
   `read_timfile`, `input`, source scanners and canonical setters; add source
   preprocessing/units/epoch modules. Preserve registry enum order and links.
3. Planetary/propagation: reuse Z/AB maps in `validation/planetary_ephemeris` and
   `propagation`; extend `read_ephemeris` and replace selected calculation bodies
   with source routines, using accepted JPL/IFTE/FB, clocks, observatories, EOP,
   troposphere and AA vector/observer drivers.
4. Orchestration: source `refphs_init`, clocks, two ephemeris reads surrounding
   observer/refined-time work, delays, BAT, secular motion, `refphs_clean`.
   `updateBatsAll` retains its narrower sequence. Connect the library and CLI;
   bounded legacy frontends become adapters to shared scientific calculations.
5. Verify component and authored public workflows in debug/release; report actual
   coverage, identities and remaining limits in `INPUT_TO_BAT_REPORT.md`.

## Frozen verification policies

Reuse the defined K/Q/N/P/Z/AB typed/structural/numerical projections and original
oracles. Corrected expectations are separate. Integrated BAT/BBAT differences
are compared in extended arithmetic after conversion from MJD to seconds, with
the existing absolute 1e-12 second budget. No new blanket tolerance applies to
components. M2's exact raw mode-order gate and four 0049 findings remain open.

Authored inputs under `validation/input_to_bat/fixtures` cover ordinary multiple
sites/aliases and mixed barycentric rows, ordered multiple pulsars, ecliptic/PM/PX,
time/units selectors, INCLUDE/flags/error/time/selection, DM/CM/DMX/DMMODEL/solar,
and reference/secular paths. Each final example must run through files and public
APIs without seeded physical fields. Also check repeat evaluation, error recovery,
old A2 integrations, formatting and Clippy.

Required corrections are localized and source-commented: no-setdm `ndm=0`, A2
`ne_sw_ifuncN=0`, and a disposition for genuinely unwritten inactive planetary
arrays. Ecliptic rotations share the preprocessing-mutated obliquity. CALCEPH
requests fail at the unavailable source boundary. Enabled CALCEPH, full residual,
binary/fitting, plugin ABI and release/platform qualification remain later work.

Generated evidence stays ignored under owned `validation/input_to_bat/build/`;
accepted runners, evidence and independent repositories are read-only. Commits
stage explicit authored owned paths only.

## Concrete projection before public consumers

The implemented `load_pairs` and `preprocess` signatures above are retained.
`form_bats_all` and `update_bats_all` are defined in `global` and re-exported by
`bats`. `RuntimeContext::from_root` constructs the general runtime; source drivers
open PAR-selected resources during evaluation. `RuntimeContext::new` remains the
existing bounded frontend constructor. Its evaluator delegates to the shared
canonical input-to-BAT driver.

`BatResult` projects canonical state using these concrete records:

- `schema_version: 1`, `status: "complete"`, `engine`, `runtime_root`, `inputs`,
  `resources`, `diagnostics`, and `pulsars`.
- Lossless strings are `{bytes: [u8...], text: string|null}`; `text` is present only
  for valid UTF-8. Inputs contain lossless paths, role, SHA-256 and consumed bytes.
- Numerical values are round-trip decimal strings, or `{class: "nan"|"-nan"|
  "+infinity"|"-infinity"}`. A null optional component denotes source-unwritten
  retained state; it is never a fabricated zero correction.
- Pulsars contain name, integer source unit/time/frame selectors with descriptive
  labels, the source sorted latch, main observations, and separate reference state.
- Each observation includes canonical index, lossless filename/site/flags,
  deleted/clock/delay selectors, SAT/BAT/BBAT MJD, observing frequency with MHz
  units, source freqSSB with its actual branch-dependent Hz/MHz unit, uncertainty
  in microseconds, clock steps, and named corrections with seconds/day units.

Main MJD outputs must be written; absent mandatory output is an error. A complete
result means the calculation finished, not that source-native nonfinite outputs
are finite or astronomically validated. Diagnostics contain captured source bytes
and source channel, drained after the completed evaluation. No CLI writes success
JSON before projection and serialization have both succeeded.


## Retained process state and error recovery

The canonical context owns pulsars, observations and constraint payloads. Source
process globals, diagnostic suppression/version state, and resource tables live
in the existing locked process session. Drivers serialize access to that session.
`RuntimeContext` provenance is the union of resources consumed through that runtime,
including earlier successful calls; use a new runtime for a standalone inventory.
Input provenance records each file consumption in source order.

An error preserves writes already performed, matching the source prefix. A failed
`form_bats_all` may therefore leave an appended reference row in that caller's
context. Recovery examples create a fresh input context and reuse the same runtime;
they do not claim rollback or that retrying a partially mutated context is idempotent.
`update_bats_all` intentionally reuses clocks and ephemerides and does not create or
clean a reference row. It requires previously prepared canonical observations.

The `status: "complete"` output means the requested calculation and projection
completed. It does not declare milestone acceptance or compatibility of every
numerical boundary. See the report's explicit unresolved-check ledger.
