# AF predictor implementation handover

AF delivers the PAR-only Chebyshev/polyco generator, both predictor grammars,
phase/frequency evaluation, ordered model lifecycle and frequency plans through
the canonical Rust timing engine. Implementation handover is complete; central
acceptance remains separate under 0074. Seven exact raw `log10rms` comparisons
remain open, localized below. No inherited gate or precision policy changed.

Production/API/source map: **7420622**. Authored validation: **543760f**.
Interface/comparison freeze: **5aee9bf**. Release baseline: `a002e6f` / dispatch
`480ef2d`, AE `ddff7f82cfe05bc152e55ddbdb33f9bfc4d06bc9` and report `ecf41ed`.
Raw TEMPO2 pin: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; A2 pin:
`7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`. Canonical and independent reference
checkouts are clean. Accepted reference/general-fitting artifacts were not edited.

## Delivered behavior

`PREDICTOR_API.md` contains runnable CLI and direct consumers.
`PREDICTOR_SOURCE_MAP.md` maps all assigned routines, source/header differences,
branch semantics, corrections and unsafe boundaries. Generation initializes and
preprocesses PAR-only canonical state, synthesizes source observations and calls
formBatsAll then uncentered formResiduals. It never reads a fabricated TIM or runs
the general timing-parameter fit. The source predictor's own polynomial fitting
remains part of generation.

All four CLI routes are available: `predictor cheby`, `polyco`, `evaluate` and
`plan`. File results preserve original bytes/hash/format; generation preserves
input/resources, warnings, requested/effective dimensions and retained SAT/BAT/
phase/frequency observations. Main and direct-constructor defaults remain
separate. Cheby uses inclusive nearest-center selection/first tie and explicit
-1 no-match status; T1 uses nearest midpoint and extrapolation. Plans use source
ceiling, shortened final intervals and unwrapped phase differences.

T1 Copy/Insert/Keep remain explicit source-unimplemented errors. Empty backing,
malformed required input and I/O failure are recoverable errors. Successful T1
prefixes survive malformed trailing input with a diagnostic. Failed owned reads
retain prior backing; stream positions are preserved. Outputs may remain partial
after failure, and existing requested output paths are replaced as documented.

No new dependency, unsafe Rust or native timing wrapper was added. Timing stays
Rug128; the native predictor type follows the compile target, 53 bits on this
macOS arm64 host and 64 bits on Linux x86_64. No runtime precision switch exists.
The native oracle uses GCC16.2.0, native binary64 long double and binary128113
for its timing typedef. Measured contraction sites are explicit. Linux execution,
compiler/ABI qualification and standalone C packaging remain AJ/AL work.

## Executed evidence

All paths below are beneath `validation/predictors/build/` and remain ignored.
The original failures and intermediate snapshots are retained; later runs use
fresh directories. `native.py` verifies the accepted AD object and A2 archive
hashes before constructing its isolated corrected oracle. The public runner
copies executable snapshots into each fresh run directory.

| Check | Debug | Release | Evidence |
| --- | --- | --- | --- |
| Complete library suite, including inherited timing/fitting/lifecycle tests | 80 passed, 34 ignored | 80 passed, 34 ignored | `library-{debug,release}-delivery.log` |
| Focused predictor tests | 12 passed | 12 passed | `predictor-{debug,release}-corrected.log` |
| Existing behavioral integration | 7 passed | 7 passed | `behavior-{debug,release}.log` |
| Accepted-reference projection/provenance | 1 passed | Not repeated | `reference-contract-debug.log` |
| Constant/linear/polynomial identical-input construction, derivatives, evaluations and error summaries | 396 checks passed | 396 checks passed | `auxiliary-delivery/component-{profile}-{case}/` |
| Direct API histories and malformed CLI/output recovery | Passed | Passed | `auxiliary-delivery/`: direct, failures and subnormal records |
| Public generation/exchange/plans, seven PAR/site/convention cases × two forms | 37,296 checks; **7 raw diagnostic failures** | Same | `public-{debug,release}-corrected/` |
| Separate plan metadata/endpoints, serialization cells and source-summary replay | 3,906 checks passed | 3,906 checks passed | `audit-{debug,release}-final/` |
| Integrated constructed/plan JSON corruption controls, unit/class controls | 15 rejected | Same observer, not duplicated | `rejections-final/` |
| Independent approximation and timing fidelity | 24 file/engine cases, 4,224 fidelity checks passed | Same | `quality-{debug,release}-corrected/` |
| Formatting / all-target Clippy with `-D warnings` | Passed | Profile-independent check | `fmt-final.log`, `clippy-final.log` |

The seven public cases are isolated/binary barycentric, isolated/binary PKS,
TEMPO1/FB90 (EPHVER 2, TDB), ecliptic/TDB, and missing-TZR/odd-CLI dimensions.
TZRSITE `@` differs from generated PKS in the terrestrial fixtures. Both legacy
and TEMPO2-marked polycos and Cheby files pass both reader directions. Constructed
state, parsed state, decimal-format loss and evaluation are checked separately.
Every plan interval's count, endpoints, selected segments and status is checked
against a native trace; the actual list length is independent of declared count.

The native lifecycle witness and Rust tests cover copy/drop/append/keep, ties,
endpoints/gaps and failed stream reads. Rust ownership permits clone-before-self
operations without executing undefined alias destruction. Direct consumers cover
repeated runtime/context use, failed output/read then recovery, changed sites,
trueDM false/true, literal filename prefixes, direct odd dimensions/defaults,
source sample/count limits and recovery after their rejection. Reader tests cover
legacy D exponents, source continuation rows, missing dimensions, derivative
extent, invalid T1 counts, trailing prefixes and overwidth Doppler neighbors.

The final observer adds two source-summary comparisons per Cheby case to the
public runner after the recorded public runs. The 14 additional checks were
executed by `audit_records.py` against those immutable records and are included
in its 3,906 count. A fresh public run therefore has 37,310 checks with the same
seven unresolved diagnostic failures; this is a checked observer extension,
not an unexecuted assertion that a different campaign passed.

## Approximation is separate from implementation fidelity

Independent points use five off-node times per segment, three frequencies and
centered one-second time pairs. Late points enter Cheby overlaps, exercising
selection independently of the construction segment. For each engine and each
file owner, the full canonical timing model supplies unwrapped phase and a
centered frequency diagnostic. Requested orders/spans are unchanged.

Across both profiles and all 24 independent cases, maximum phase/F0 error is
**2.94006316e-7 s (0.294007 microseconds)**, below the predeclared 1-microsecond
target. Maximum centered-frequency discrepancy is **2.20025981e-7 Hz**. Frequency
is reported as a finite-difference approximation diagnostic; no separate
frequency-approximation acceptance threshold was invented. Source RMS/MAV and
all per-point values are retained. These approximation results do not waive a
Rust/native comparison failure or establish universal predictor accuracy.

## Open AF diagnostic comparisons and localization

The remaining failures are exact native rounding-cell comparisons of the
constructed polyco `log10rms`, before its three-decimal serialization. Segment
indices are zero-based. Both profiles have the same seven failures:

| Case / segment | Rust | Native (rounded here for readability) |
| --- | --- | --- |
| binary-bary / 0 | -7.846849513037533 | -7.8468495130375775 |
| binary-bary / 1 | -8.164600452940519 | -8.164600452940384 |
| isolated-pks / 0 | -7.364344127317825 | -7.364342225695908 |
| binary-pks / 0 | -7.297627367497283 | -7.297625968316819 |
| binary-pks / 1 | -7.2172706760784875 | -7.217270682388441 |
| ecliptic-tdb / 1 | -7.2208133026607415 | -7.2208154550504 |
| defaults-odd / 0 | -7.364344127317825 | -7.364342225695908 |

`predictor_refit` replaces only the canonical observation phases with retained
native phases, then calls the same public `atimfake`/`tz_fit` implementation.
All **10** replayed log10RMS values match native binary64 rounding cells exactly.
`rms-localization-delivery/receipt.json` records this controlled result. The
original phase discrepancies are 2.742e-14 turns for binary-bary and at most
7.018e-12 turns for the terrestrial cases; the existing 1e-10-turn gate passes.
This localizes the diagnostic mismatch to timing-phase inputs for these cases,
not the polyco fit on shared inputs. It does not establish a universal precision
explanation or authorize changing the diagnostic gate. The raw failures remain
unaccepted; public comparison commands deliberately return exit status 1.

## Original/corrected distinctions

The source-map correction table and `native-corrected/corrections.patch` describe
initialization, writer ABI, parser/storage bounds, newline/date/I/O and
barycentric delay fixes. The original invalid variadic writer and uninitialized
plan storage are classified from source/type evidence; undefined output is not
a scientific golden value. Initial terrestrial unwritten-vector and barycentric
trueDM failures are retained in the early public runs.

The final legacy reader correction is independently visible: the source writer's
`%6.3f` is a minimum width. Its barycentric `-9999.990` overflows six columns;
the old reader returned -9999 and then misread log10RMS as 990. Parsing the
writer's decimal extent recovers -9999.99 and the actual RMS, without changing
the source Doppler/frequency convention or file bytes. `audit-*-delivery/`
retains the original eight failures across isolated/binary barycentric records;
`audit-*-final/` passes after the correction. Native source identities differ only
in `t1polyco.c` between `native-delivery-v2` and `native-corrected`.

Central's bounded 32a reviews independently closed the two native subnormal
witnesses, constructed-model omissions and plan membership omissions, and
verified the native reader correction. Their original and corrected evidence
remains under `decisions/evidence/predictor-review/build/`; AF's final focused
regressions and exchanges are separate evidence, not a claim of whole-stage
central acceptance. Both subnormal operations now return native bit 1; normal,
minimum subnormal/normal and negative-zero neighbors remain exact. Decimal/hex
parsing and the native-width exponent/subnormal logic have focused Rust coverage.

## Reproduction and identities

Run from repository root with a fresh output directory for each command:

```sh
cargo build --manifest-path rust/Cargo.toml --release --bins --examples
python3 validation/predictors/native.py validation/predictors/build/review-native
python3 validation/predictors/run_public.py \
  --native validation/predictors/build/review-native/predictor \
  --rust rust/target/release/tempo2-rust --out validation/predictors/build/review-public
# The public command retains seven raw log10RMS failures and returns 1.
python3 validation/predictors/audit_records.py \
  --public validation/predictors/build/review-public --out validation/predictors/build/review-audit
python3 validation/predictors/check_rejections.py \
  --public validation/predictors/build/review-public --out validation/predictors/build/review-rejections
python3 validation/predictors/run_quality.py \
  --native validation/predictors/build/review-native/predictor \
  --rust rust/target/release/examples/predictor_quality \
  --public validation/predictors/build/review-public --out validation/predictors/build/review-quality
python3 validation/predictors/diagnose_rms.py \
  --rust rust/target/release/examples/predictor_refit \
  --public validation/predictors/build/review-public --out validation/predictors/build/review-rms
```

`check_components.py --case {constant,linear,polynomial}` and
`check_failures.py` accept explicit executable/public/output paths. Direct
consumers are documented in the API guide. Native source dependencies are the
existing verified AD/A2 build; generated artifacts stay under AF staging.

Key SHA-256 identities:

| Artifact | SHA-256 |
| --- | --- |
| Native corrected executable | `3bf82ca766e45599c23a575868d6977d97c2f25c46428c02c877ccca0276a27f` |
| Debug public executable snapshot | `8a7316a09f977852d08395877a3c3eadd99df279a14cf52ae78d3e02440eabc4` |
| Release public executable snapshot | `1942f4351873830fbb36d042488c84d37dddbcf2d5a9e5b3dac761f9fbda6723` |
| Native identity record | `12f8b4cf7896ba36dccfa9bdae670db5d8541ad8356fb3cadb929e68c83de668` |
| Debug public receipt | `645ac840bf3d3873aa6a38b4b3744bd46774fe009ab37ee21c31ccde1093d6ef` |
| Release public receipt | `c59bd089c4a6fe3a78b767156571b3a8bb7c943ca76b4d2885b63eefba1609f6` |
| Final replay audit (both profiles) | `bcea259d7c3791794f85e291d049e0700b446e3a0a9d950170c46b4a178919c3` |
| Rejection controls | `fd2c7dad7c0f1cfeb100b1bdc16988e0f112f5bc01290b7e64f338144c7f668b` |
| Shared-phase localization | `7ef6eea634e22fe138e7eb2a8734d21a299519d1df3f50c689074a31a3bb7e8b` |
| Debug / release quality receipts | `86d4d3e345c760d0baeca1634b39a0aa33040e808fbd9cdac33b8175e527c454` / `20dd0c0c0b53a911c5f8ef68ca3520c7a7279d79183647dc585d11537c991bf4` |

`build/delivery-index.json` indexes these and the test logs; its SHA-256 is
`324e4ab1b8874a8098bd5a3be36b2a640ae9820783dfb52c37bc92da6571efb4`.
Native build metadata retains every object/source/patch/archive hash and command.
Public receipts retain executable hashes, source snapshots by hash and each file
product. The source-map line correction to polyco.C:419 is documentary only.
A rebuild from committed production reproduces the tested release executable
byte for byte. The rebuilt debug file has SHA-256
`15f0e30e5f0ba48b55c0cfed886aa1f61c127d15db5ff7d9abb3856c4181a45a`;
every named Mach-O section is byte-identical to the tested debug snapshot.
`build/committed-debug-sections.json` records that comparison (SHA-256
`cbbc8a7df5cb867e3da8c945bcf8d6ce19fcaadcfd1390f9d31650971f99dcc4`).

## Unchanged inherited ledger and limits

| Inherited check | Still-open disposition |
| --- | --- |
| P preprocessing | 154 field/event differences, including 32 SIMUL; 0077 precision boundary, unchanged exact discrete semantics. |
| M2 | Four raw SVD-order failures; user's 0012 exact-order requirement remains. |
| 0049 | Four quotient/rounding boundaries and three unconstrained interpolation inputs reaching the existing boundary near 2.5e12 s; no clipping or precision mode. |
| AE public red | Six raw-mode failures over two cycles, rank nine with all modes retained; physical checks separate. |
| Chromatic-log noise | 16 reconstructed-error failures plus one multiplier, singular-value ratio about 1.05e26; no universal precision attribution. |
| ELL1H family fits | 19 whitened-data differences across A1/EPS1/EPS2/TASC/H3/PB. |
| Single-pulsar GW | Quadrupole / GW single / memory / cosmic-string: 770 / 750 / 448 / 468 failures; supported global cases remain separate. |
| DDH / 0087 | Historical ideal 1-ns pass; current/historical fidelity retains 401/433 failures; shared solves retain two raw-order differences. Production clocks and original failure preserved. |
| SWGP | Two TNsubtractSWGP=1 first passes read unwritten SWGPSignal before prefit initialization; explicit boundary, not an invented zero. |

Exact inherited identities remain in AE's immutable evidence index
`971f96f9ca9c8aafb519756235343d8bb369ec24a8842edf4f174797cfe6a7c9` and 0088.
AF did not replay unchanged large campaigns as a prerequisite or accept their
failures. The 34 ignored corpus tests are not passes. Linux execution, enabled
acceleration, standalone predictor C ABI/packaging, wider platform/concurrency
qualification and the remaining toolkit/core-output stages are not delivered by
AF. Full migration and scientific acceptance remain incomplete.
