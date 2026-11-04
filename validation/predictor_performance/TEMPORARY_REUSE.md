# AO predictor temporary-reuse qualification (decision 0143)

The three-file production slice consumes the existing selection sum, reads the
seven model scalars into a fixed array, and uses Rug's rounded primitive `f64`
right operands. It retains the same scientific expressions, native precision,
nearest-even operations, first rounding direction, ordinary range handling and
NaN-sign rules. All N/N implementations and the separate Rug128 timing type are
unchanged. Borrowed inputs remain live; owned inputs may supply result storage.

Frozen production pair:

* Before: `a1851c55fcc0b1ac733b064e631bfbbd2e0118d5` (accepted coefficient view).
* After: `e1d34d835377451dcc7822161afce96a87948005` (temporary reuse and Rust witnesses).
* Native: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, with the unchanged
  qualification script's recorded corrections.

Use the unchanged 8-by-4 predictor, SHA-256
`a8b4242d2600cc72198a7f40240b0e946d17d173804e6d10d7f5f31c9f8811ee`.
The macOS evidence root is `build/temporary-reuse-20261104/`. Its `after-v1`
install is new; immutable before artifacts come from
`build/coefficient-view-20261104/{after-v1,after-direct,qualify-after}`.

## Portable replay and exact contracts

Execute the complete [coefficient-view recipe](COEFFICIENT_VIEW.md) in a fresh
`AO_OUT`, replacing only its two production build revisions:

```sh
python3 validation/predictor_performance/build.py \
  --revision a1851c55fcc0b1ac733b064e631bfbbd2e0118d5 \
  --out "$AO_OUT/before" --cargo-home "$CARGO_HOME"
python3 validation/predictor_performance/build.py \
  --revision e1d34d835377451dcc7822161afce96a87948005 \
  --out "$AO_OUT/after" --cargo-home "$CARGO_HOME"
```

Run its direct/native/C comparisons, coefficient witness, installed consumers,
public-borrow consumer, all 18 debug/release predictor tests, caller-exclusive
plain/ASan/TSan controls and explicit cache cleanup. Preserve all 5,880 direct
edge results, 28 finite results, 30,258 complete C lines, 38 legacy products and
8,516 coefficient-witness lines across ten profiles. Keep all thirteen ordered
diagnostics. The frequency path still reads the unused dispersion slot; all
seven reads precede coefficient validation, and the coefficient borrow ends
before output publication. General scalar vectors, callbacks, derivatives, T1
paths and native output conversion retain their existing ownership contracts.

The extended operation matrix compares both mixed-operand ownership forms for
Add/Sub/Mul/Div against unchanged borrowed N/N operations. It covers both signs
of zero and NaN, infinities, finite maxima and normal/subnormal neighbors, with
live public Float borrows. Selection includes ties, overlaps, endpoints, gaps,
the strict 1e6 sentinel and nonfinite metadata.

The first-rounding witness uses `e=2^-22`, an ordinary left input proportional to
`1-e+e^2`, and an exactly representable f64 right input proportional to `1+e`.
Their exact product is proportional to `1+e^3`; the p-bit intermediate rounds to
half the minimum subnormal, with a nonzero direction that must survive the
ordinary-range step. Both positive and negative owned/borrowed cases are exact.
Execute this test on actual Linux x86_64 as well as macOS arm64. The Linux
matrix retains left inputs `1+2^-63`, `2^-16445`, `2^16382` and the Linux maximum;
no f64 narrowing of the left input or cfg-swapped macOS run qualifies Linux.

Inspect the actual optimized library. The macOS native recurrence must retain
direct coefficient loads, source FMA sites and separate doubling/additions.
`t2r_model` must no longer call `values`; all seven scalar reads remain.
Rounded mixed operations must reach the corresponding MPFR `_d` primitives,
with returned direction carried through ordinary range handling. Source and
output checks remain necessary alongside instruction inspection.

## Combined measurements

After correctness and central quiet-window clearance, run the unchanged
seven-sample, 100,000-pair direct and complete-C campaigns linked by the
coefficient-view recipe. Retain all 91 process records, every sample and peak
RSS. Allocation observers run separately at 1 and 1,001 iterations; report
actual per-pair differences and requested bytes. One combined campaign is
sufficient; no measured contribution is assigned to an individual edit.

Native T1 NaN-sign and polyco log10RMS discrepancies remain separate failures.
The sanitizer scope is C clients and shim, not Rust/MPFR/Fortran. This recipe is
portable procedure, not evidence of Linux execution or performance-budget approval.

## Integrated Linux preparation (decision 0145)

`validation/release/integrated.py` fetches the exact after revision above and
reuses the coefficient-view after install, qualifier and direct clients as the
before endpoint. It does not rebuild those baseline artifacts. The existing
full-C/native/legacy and direct comparisons, C coefficient/ordered-diagnostic
witness, concurrency/cleanup and installed consumers remain separate stages.
`consumers.py` includes the public-borrow consumer in its canonical invocation.

Both library profiles run the predictor tests. Each actual Linux log must
contain passing executions of:

- `cheby2d::evaluation::platform::tests::predictor_recurrence_linux_keeps_wide_inputs`
- `predictor_tests::predictor_owned_operations_preserve_rounding_and_classes`
- `predictor_tests::predictor_native_subnormal_single_rounding`
- `predictor_tests::predictor_owned_subnormal_midpoint_keeps_first_rounding_direction`

A zero-test, ignored, failed or missing selection is a failure, not a range or
mixed-input/live-borrow witness. The installed optimized library is disassembled.
Every pair build, check and GUI stage precedes the existing sequential timing
block. Direct and full-C measurements retain 100,000 pairs and seven samples,
with the same 8-by-4 predictor. A coefficient-witness failure blocks the full-C
measurement; direct measurements keep their own verification dependency.
Inherited native discrepancies remain explicit failures without suppressing
independent evidence. No tolerance, precision or performance budget changes.

The integrated recipe and focused routing/retention controls are preparation.
Actual Linux execution, scientific and image review, remote preflight and
publication/dispatch remain central responsibilities.
