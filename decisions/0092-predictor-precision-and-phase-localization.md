# Predictor precision and phase-stage diagnosis

Central evidence decision, 27 October 2026. **Changing only the shared timing precision from 128 to 113 bits does not remove AF's seven raw polyco log10RMS failures.** All fourteen constructed log10RMS values are identical between those two diagnostic variants. The largest terrestrial phase differences coincide with a one-binary64-ulp Rømer-delay difference; the barycentric BT case has one-ulp orbital-delay differences. This is bounded localization, not a complete arithmetic-cause finding or acceptance. Retain production Rug128 and the existing numerical gates. AH continues functional implementation under 0091.

## Controlled experiment

The experiment archives AF production `742062270f0cd29af57c0a43299f23865d05d834` and observer `543760f288dd7dffbcefb2d76efbf50892c84de1` into disposable storage. Both variants compile that same source with the same direct-rustc release settings, existing frozen dependency archives and retained SOFA archive. The only variant edit is `longdouble::PRECISION_BITS`: 128 to 113. The ordinary `PredictorNumber` profile and existing literal-128 constructors remain unchanged. Thus the second variant is a controlled shared-constant experiment, not a complete IEEE binary128 emulation or proof about every extended operation.

Seven existing public polyco workflows are rerun: isolated-bary, binary-bary, isolated-PKS, binary-PKS, TEMPO1-PKS, ecliptic-TDB and defaults-odd. The fresh native executable is a byte-identical snapshot of AF's corrected oracle, SHA-256 `3bf82ca766e45599c23a575868d6977d97c2f25c46428c02c877ccca0276a27f`. Every native result TSV reproduces the original AF observation bytes. The 128-bit Rust control reproduces every original observation, model, timing coefficient and result warning exactly. This checks the control beyond its failure count.

The experiment reuses the frozen `constructed` comparison and the public workflow's SAT/BAT/phase/frequency gates. It exercises generation and pre-serialization state only; it does not rerun Cheby generation, file exchange, plans or the larger approximation campaign.

| Shared timing precision | Workflows | Comparisons | Failed comparisons | Raw log10RMS values matching native cells |
| --- | ---: | ---: | ---: | ---: |
| 128 bits, control | 7 | 2,247 | 7 | 7 of 14 |
| 113 bits, diagnostic | 7 | 2,247 | 7 | 7 of 14 |

Both variants retain exactly the same seven failures and fourteen log10RMS values. All other comparisons pass, including the existing `1e-10`-turn phase gate and `1e-12`-second SAT/BAT gates. This differs from the P result in 0077; the P precision explanation must not be generalized to these predictor failures.

## Observation-only stage trace

A separate native observer adds field dumps after the existing polyco calculation. It links the same verified native objects and reference archive. A small external Rust consumer calls the same public PAR loader, defaults and polyco implementation from each already-compiled variant, then reads canonical state. Both observers reproduce their original SAT, BAT, phase and log10RMS results. The new observer avoids terrestrial fields on barycentric/reference rows whose producers are bypassed.

The trace covers binary-bary, isolated-PKS, binary-PKS and ecliptic-TDB at both precisions. It retains raw extended values; its binary64-cell census is a localization diagnostic, not a new comparison projection or permission to narrow extended values in an acceptance test.

- Isolated-PKS and binary-PKS row 15 have a Rømer-delay difference of approximately `-5.6843418860808015e-14 s`, one binary64 ulp of the stored value. Their largest BAT difference is approximately `-5.6843418867e-14 s`, and their phase difference is approximately `-7.01770597615e-12 turn`. Subtracting the BAT contribution times the native predictor's stored spin frequency leaves about `4e-24 turn` at that row. Ecliptic-TDB row 34 has the opposite-sign one-ulp Rømer difference and corresponding largest BAT/phase difference.
- The terrestrial Earth-vector trace already contains a few one-ulp component differences; the observed pulsar position and observatory vector components match binary64 cells. This localizes the inherited discrepancy upstream of polyco fitting, but does not establish whether a particular ephemeris interpolation, conversion, contraction or other arithmetic step is its cause. No such cause is asserted without a controlled intervention.
- In binary-bary, twelve observation rows have one-ulp BT orbital-delay differences of `2.2204460492503131e-16 s` in magnitude. At 128 bits its BAT values match native exactly, while its maximum phase difference is about `2.741291396623e-14 turn`. Four orbital-delay rows also differ in binary-PKS. These counts and the listed one-ulp differences persist at 113 bits.

The relevant source boundaries are pinned `calculate_bclt.C:152` (binary64 expression assigned to timing `roemer`), `formBats.C:68-83` (arrival-time assembly) and `BTmodel.C:117-119` (binary64 orbital delay), with Rust counterparts in `propagation.rs` and `binary/bt.rs`. Observing their outputs does not prove which internal operation introduced a difference. AF's previous shared-phase replay, in which all ten replayed native RMS cells match, remains the stronger controlled evidence that the polyco fit agrees on those shared phase inputs.

## Disposition and scope

Keep all seven raw diagnostic comparisons open, including binary-bary segments 0/1, isolated-PKS segment 0, binary-PKS segments 0/1, ecliptic-TDB segment 1 and defaults-odd segment 0. Do not substitute serialized three-decimal RMS values, widen a gate, claim native-exact timing, change production precision or introduce a second production engine. A later correction needs an actual source/type/arithmetic witness under the existing policy; a later numerical-acceptance proposal needs a quantity-specific justification and central disposition.

There is no new implementation blocker or consequential choice required before AH continues. Its production work, prepared AI/AJ/AK/AL stages, the complete 0091 ledger, paired DDH rule and selected Rust/predictor-C and macOS/Linux guarantees remain unchanged. This decision accepts no milestone and changes no ownership. Canonical TEMPO2 and the independent reference retain their pinned tracked contents; the five previously present TEMPO2 `.DS_Store` files are preserved.

## Evidence and reproduction

Authored generators:

- `decisions/evidence/predictor-precision/central-predictor-precision.py`, SHA-256 `291d42adb0bf4c499b7c52e7d5ac3d6c38cd8345b99326b87c91f8eeef509a45`.
- `decisions/evidence/predictor-precision/central-predictor-phase-stages.py`, SHA-256 `f1e11507636fb42659ba78153a6f254d5892a9db3cc61549789d27474815bdb4`.

Original output: `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-predictor-precision-ghr8iddj`. The precision receipt has SHA-256 `14a31e0eed90744afe0841eb0f425917fd3929448e9bd542f6f0e5981e019dc7`; the stage receipt has SHA-256 `925cedbb1f6e468259455ca576abf9f92229fc2d577e82978a023b04c3de1714`. The first verifies 549 frozen source files and 233 protected input/resource/dependency/evidence files; only the declared scalar constant differs in the diagnostic snapshot. The original source archives, exact commands, environment, executable identities, observations and patches are recorded. Across the two scripts, all 51 compile/run commands completed successfully; comparator failure records remain seven per precision as shown above. Host/compiler identity is recorded, with macOS arm64 and rustc 1.97.1. This is not Linux or production debug/release qualification.

Receipts, logs, generated model files, observers and complete differences are copied byte-for-byte into ignored `decisions/evidence/predictor-precision/build/review-20261027/`. Preservation receipt SHA-256: `fc138b6bc19ae23cbc001feb8ab26d1da830b77f5c9904f6ef91631360be7b14`. Source snapshots, archives and compiled binaries remain at their recorded original paths and are separately indexed by hash; they are not relabelled as executions from the saved directory. No generated evidence or binary is committed.

Reproduce with a frozen AF release dependency target containing the locked Rug/serde/serde_json/sha2 and SOFA archives:

```sh
python3 decisions/evidence/predictor-precision/central-predictor-precision.py \
  --root /Users/Eric.Dong/Downloads/tempo2rust \
  --dependency-target /var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-af-handover-a_o0ppu7/target
python3 decisions/evidence/predictor-precision/central-predictor-phase-stages.py NEW_PRECISION_OUTPUT \
  --root /Users/Eric.Dong/Downloads/tempo2rust
```

Use a fresh output directory; the scripts refuse to overwrite generated runs. The current working Rust tree is not the input to either diagnostic.
