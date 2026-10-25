# Assignment AC implementation contract

Source: TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
Baseline: AA `ea9797236fae0f0dd29a32e5d3050e33eaf2fefe`, production `99cf714`.
Authority: decisions 0068/0069 and handoff 29. This is an implementation plan,
not an acceptance claim.

## Public boundary

The `bats` module will expose these operations (all fallible operations return
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
