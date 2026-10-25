# Shared-parameter continuous rounding contract

Central decision, 25 October 2026. AC's new K corpus adapter exposed low-bit differences between the adopted Rug128 arithmetic and the 113-bit native reference. Supply the previously unset Rust comparison contract for the two diagnosed continuous expressions below. Preserve production Rug128, source expression/order/constants, exact discrete behavior and the existing milestone gates. This implements 0065's functional priority without a new reference assignment.

## Scope and evidence

Decision 0026 explicitly states that exact native regeneration did not choose a Rust numerical tolerance. K's exact native self-replay remains unchanged. AC's diagnostic that rounds every Rust value to 113 bits and demands equality is useful for localization, but is not an earlier approved arithmetic gate. Keep its unequal results visible as diagnosed continuous rounding differences; do not relabel them as exact matches. This decision supplies a narrow comparison contract after that diagnostic, rather than claiming it was frozen before the differences were observed.

At pinned TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, `readParfile.C:2047-2048,2159-2161,2180-2182,2193-2195,2245-2247,2254-2256` applies its binary64 threshold and scale constants in extended arithmetic. `tempo2Util.C:443-450` computes linked KIN as `asin((2.0*stig)/(1+stig*stig))*180.0/M_PI`. The current Rust expressions preserve these boundaries and operation order. The threshold is the original binary64 `1e-7`, not a newly parsed extended decimal; the `1e-7` input takes the scaling branch in both engines.

The preserved central reproducer is `decisions/evidence/parameter-rounding/central-review.py`. It reads the unchanged K native values, decodes them at their actual 113-bit precision, evaluates the identical source expressions at 113, 128, 256 and 512 bits with the same binary64 constants, and retains exact integer/exponent outputs. Both 113-bit calculations reproduce native exactly. Error decreases from 113 through 128 and 256 bits against the 512-bit calculation. This checks the cause and numerical convergence; it does not establish integrated production acceptance.

| Expression/input | Exact decoded Rug128 minus native magnitude | Relative magnitude | Rug128 error against 512-bit source expression |
| --- | --- | --- | --- |
| `1e-7 * binary64(1e-12)` | `5.99194e-54` | `5.99194e-35` | `2.69169e-61` |
| KIN with STIG `0.5`, in degrees | `6.29595e-33` | `1.18501e-34` | `1.15751e-37` |

The decoded values differ slightly from subtracting the printed decimal strings directly: those strings are round-trip transport, not exact decimal encodings of each binary value. Compare values decoded at each engine's actual precision.

## Narrow comparison rules

Let `u113 = 2^-113`, the relative round-to-nearest error bound for normal 113-bit values, and `S = max(abs(native), abs(rust))`. Compute the comparison with extended arithmetic after decoding native at 113 bits and Rust at 128 bits. Both signs and finite classes must agree; zeros retain exact sign and value. These allowances do not apply to nonfinite, subnormal or conversion-boundary cases.

1. In K's `threshold_scaling` case, the scaled `val` of PBDOT, XPBDOT, A1DOT/XDOT, EPS1DOT, EPS2DOT and EDOT/ECCDOT, plus the copied scaled `prefit` where the source performs that copy, use `abs(rust-native) <= 4*u113*S`. EDOT's separately retained prefit is not added to this allowance. Parsing and multiplication give two rounding steps per engine. The sum of their standard two-step error bounds, including the smaller 128-bit contribution and conversion to the observed scale, is below this bound. Token routing, threshold selection, aliases, fit flags, uncertainties, retained values and all unrelated fields remain under their existing exact checks.
2. For the linked KIN result only, in K's `link_KIN_before`, `link_KIN_after` and `link_nonrecursive` cases with STIG exactly `0.5`, use `abs(rust-native) <= 8*u113*S`. Here multiplication, squaring and addition preceding the quotient are exact; the quotient is near 0.8. The relative condition number of asin there is below 1.44. Quotient error propagation plus rounding of asin, multiplication by 180 and division by the binary64 pi constant is below this bound, including the 128-bit contribution. The native result is independently reproduced by the correctly rounded 113-bit source replay. Link registration/order, nonrecursive selection and all other outputs remain exact.

These are expression-derived bounds with margin, not bounds fitted to the observed differences. No blanket relative or absolute tolerance is introduced for K, Q, other component corpora or the final timing results. An unexplained discrepancy remains a finding. Source operation-order review remains necessary; a numerical allowance is not permission to rewrite the expression.

AC should keep the named policies in its existing corpus comparator, report exact matches separately from these qualified numerical comparisons, and exercise focused negative cases: a changed scaling branch/factor, a materially changed KIN value, a wrong field/class/sign, and a low-bit change in an unrelated exact field. The central diagnostic rejects a relative `1e-30` perturbation for both quantities; AC must also verify its actual comparison implementation. Rerun affected parameter comparisons, then continue the existing functional acceptance checks. No historical whole-suite replay is required solely for this clarification.

## Boundaries and retained identities

This does not resolve decision 0049's four discrete quotient/rounding differences, decision 0012's exact raw SVD ordering failures, the original DDH 1 ns failure, or any other open numerical issue. Integrated BAT/BBAT/response gates remain at `1e-12` seconds. No native reference, production arithmetic, backend, physical model or previously accepted numerical threshold is changed.

The central run's copied evidence is retained locally under `decisions/evidence/parameter-rounding/build/review-20261025/`; its receipt records the original temporary execution paths and actual compiler/dependency/source/executable hashes. Receipt SHA-256: `036a3065ebeeadc19937b80fb15dd29518a7cc6a3c1e75ee055ffac69d4840a7`. Only the reproducer and this decision enter Git; generated values, binaries and receipts remain ignored under 0039. AC's implementation/report and central capability acceptance remain pending.
