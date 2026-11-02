# Discrete-boundary disposition for central review

2 November 2026. Read-only analysis of frozen candidate `9991e804d0e230248e5fb20974b7e63384d4efeb`
and native source `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
This brief proposes a disposition; it changes no arithmetic, precision, gate,
threshold or oracle and makes no claim about the running Linux qualification.

**Recommendation:** retain the canonical Rug128 expressions and ask central to
explicitly choose Rug128 discrete-decision semantics for these boundaries. The
small source-expression correction required by 0049 is already implemented under
0058. The remaining differences are numerical-contract choices, not demonstrated
translation defects. Choosing Rug128 would be an explicit departure from native
binary128 decisions, with separately retained raw failures and exact checks of
the declared contract; it must not be described as native equivalence or silently
installed as an allowance. If exact native decisions are mandatory, central must
instead authorize a precision exception and its limited domain. Neither choice
is authorized by this brief. Until that decision, the existing gates remain open.

## 0049: same exact operands, different discrete arithmetic

Pinned `tempo2Util.C:186–190` computes `a - (int)(a/p)*p`; lines 211–220 compute
`(long)(x > 0 ? x+0.5 : x-0.5)`. Division, half addition, multiplication and
subtraction round at the source arithmetic precision before/after the stated
integer casts. The recorded native significand is 113 bits; production Rug is
128 bits. The witnesses stay within defined native integer-conversion domains.

The preserved [0049 experiment](../../decisions/0049-numeric-utility-boundary-evidence.md)
parses inputs natively, decodes their binary128 bytes exactly, and gives those
same values to Rug. All 16 source-order cases match at 113 bits; four differ at
128 bits. The following operands are lossless, not rounded display decimals:

| Operation | Exact input | Native 113-bit outcome | Source order at Rug128 |
| --- | --- | --- | --- |
| `nlong(x)` | `x = 1/2 - 2^-114`, hexadecimal token `0x1.ffffffffffffffffffffffffffffp-2` | integer `1` | integer `0` |
| `nlong(-x)` | negative of that same value | integer `-1` | integer `0` |
| `mod(1,p)` | `p = 4153837486827862102824397063376077 × 2^-115`, the native parsing of `0.1` | `+0` | `4153837486827862102824397063376075 × 2^-115` |
| `mod(-1,p)` | same positive `p` | `+0` | negative of the preceding nonzero remainder |

For the first pair, `x+1/2 = 1-2^-114` is exactly halfway between the two
neighboring 113-bit values and rounds to `1`; at 128 bits it is exact and
truncates to `0`. The negative case is symmetric. For the remainder pair,
`p = 1/10 + 1/(5×2^115)`: the exact positive quotient is below 10, but its
113-bit rounded value is 10. Rug128 selects quotient 9. The native product
`10*p` rounds from `1+2^-114` to 1, giving positive zero after subtraction.
Rounding only the quotient to 113 bits while retaining the product at 128 bits
would instead return `-2^-114` for the positive witness: that partial patch is
not source-faithful.

Current [tempo2_util.rs](../../rust/src/tempo2_util.rs) lines 45–58 already retain
the expression, a Rug divisor and a checked native-i32 quotient. The former `%`
semantics and binary64-divisor narrowing were corrected under
[0058](../../decisions/0058-time-ephemeris-rust-contract.md). Existing
`time_ephemeris_tests::source_remainder_and_unresolved_precision_boundaries`
checks the source signed-zero/nonfinite behavior, rejects undefined quotient
conversions, and explicitly preserves the four outstanding outcomes. Reverting
to `%`, changing truncation to a generic nearest-integer operator, or widening a
tolerance would lose source semantics rather than resolve the conflict.

The scientific exposure is caller-specific:

| Pinned native caller → current Rust caller | Meaning and demonstrated limit |
| --- | --- |
| `formResiduals.C:2194,2205,2291,2355` → `residual_source.rs:4790,4806,5088,5332` | Unit-period reference phase, nearest pulse count, tracking count and published `pulseN`. A one-count difference changes the residual by one turn before later processing; it is not a small continuous residual perturbation. No end-to-end pulse slip is established by these utility witnesses. |
| `polyco.C:196,388,402` → `polyco.rs:73,283,311` | Sidereal wrap, seconds within a day and orbital phase. Divisors here are 1 or 86400, not the decimal `p` above. |
| `get_obsCoord.C:525`, `tt2tdb.C:288` → `get_obs_coord.rs:204`, `tt2tdb.rs:125` | Sidereal rotation and time-ephemeris fractional coordinate; both use divisor 1. The public longdouble overload remains relevant even when the caller has binary64 locals (0057/0059). |
| `readEphemeris.C:524–541`, `bootstrap.C:292–309`, `readJBO_bat.C:196–199` | Integer-state random recurrences and calendar decomposition use integer divisors. Current `read_ephemeris.rs:356` uses the canonical helper; these are not evidence that the decimal-divisor witness occurs in those workflows. |

For identical representable operands and divisor 1 within the checked quotient
domain, quotient truncation is not the decimal-divisor mechanism above. Upstream
phase generation can still differ by precision. Do not turn the four utility
witnesses into an unsupported claim that all phase wrapping or random streams
fail, or claim that caller coverage establishes full helper equivalence.

## 0077: SIMUL changes the retained observation set

The public route is `preProcess -select FILE` → `useSelectFile` →
`processSimultaneous`. Native `preProcess.C:871,930` tests
`fabs(SAT_i-SAT_j)*SECDAY < tdiff`; `tdiff` is a parsed binary64 value with default
60. The Rust route is `preprocess.rs:857` → `selection.rs:249` →
`selection.rs:99–161`, preserving the strict comparison, observation order,
flag matches and deletion writes.

The lossless input tokens are `58000` and
`58000.0006944444444444444444444444444444`, in days. The recorded values are:

| Representation | Exact second SAT | Separation relative to 60 seconds | `<60` |
| --- | --- | --- | --- |
| Native binary128 | `4595233480846888881998005781671299 × 2^-96`; bytes LE `065bb0055bb0055bb0055b0020c50e40` | `+1.696366490587263786…e-25 s` | false |
| Rug128 | `301153221400781709770621306907610241069 × 2^-112` | `-8.504906634414033527…e-30 s` | true |
| Exact decimal tokens | the decimal above, interpreted rationally | exactly `-3.84e-30 s` | true |

Read-only rational decoding reproduces the recorded native bits and both
reported separations. After subtraction and multiplication by 86400, the exact
dyadic numerators need only 95 and 111 bits respectively. Thus **neither arithmetic
step rounds in this witness**: the opposing branches originate in the stored SAT
values. Evaluating the *same native SAT bits* in Rug128 would remain above 60.
Rounding the final Rug128 separation to 113 bits cannot fix the mismatch because
that separation already fits exactly in 113 bits. The stored 256/512-bit
diagnosis approaches the exact decimal result; it does not establish a more
accurate physical arrival time.

The complete witness is retained in `validation/preprocessing/initial.json.gz`,
`observations.json.gz` and `fixtures.py:77–82`, with the extracted actions in
`validation/input_to_bat/build/preprocessing/simul-native.json`. It sets SATs
`58000`, the boundary token, `58000.0003`, `58001`, sets the first two TOA errors
to 2, then runs the selection twice. Relevant cases are:

| Cases | Selector | Order |
| --- | --- | --- |
| `simul_123,124` / `127,128` | `PROCESS SIMUL -be A B toaerr`, default / explicit 60 | forward / reversed |
| `simul_139,140` / `143,144` | `PROCESS SIMUL -be A B 2`, default / explicit 60 | forward / reversed |

Each case contributes two scalar deletion differences and two duplicate
whole-record manifestations across the two calls: **32 records in eight
histories**, not 32 independent observations. Native retains the affected row
(`deleted=0`); Rust deletes it (`1`): row 0 forward, row 3 after reversal.
This changes the data entering timing/fitting and potentially its weight and
constraints. The tiny time discrepancy does not justify accepting the deletion
change through a continuous tolerance. The authored exact-threshold control
uses SATs `58000`/`58000.5` and threshold `43200`, all exactly representable;
its strict boundary remains distinct from the almost-60-second decimal fixture.

## Concrete choice and smallest follow-through

1. **Recommended: explicit Rug128 decision contract.** Keep the present source
   order and canonical storage. For these witnesses Rug128 also follows the
   exact-input decision shown above; this is mathematical evidence for the
   stated inputs, not a universal physical-accuracy claim. Central would need
   to approve the departure from native decisions explicitly, retain the raw
   native failures, and require exact target decisions and full observation
   state under that declared contract. No epsilon, `<` to `<=` change, field
   omission, blanket acceptance of SIMUL differences, or tolerance fitted to
   observed errors is justified. No production edit is needed for these known
   cases. The tradeoff is loss of bitwise native branch compatibility.
2. **If native branch compatibility wins:** the narrow source-faithful utility
   implementation must reproduce 113-bit operand/intermediate rounding and the
   original integer-conversion domains, including the remainder product and
   subtraction. SIMUL requires native-compatible SAT values *before* subtraction,
   not rounding the seconds afterward. Projecting the two raw fixture SATs to
   113 bits would reproduce this witness, but projecting arbitrary Rug128 SATs
   after prior clock/offset/unit operations does not reconstruct their native
   history and cannot establish general equivalence. This option requires an
   explicit exception to 0001/0049/0077 and a defined scope; it is not an ordinary
   local bug fix. Existing Rug precision/rounding facilities and the preserved
   diagnostic demonstrate the needed primitive; no new arithmetic package or
   parallel timing engine is warranted.

After central chooses, the current Rust owner should make only the approved
contract change and verify the four identical-input utility cases plus defined
zero/infinity/domain neighbors; the eight SIMUL histories plus existing exact
threshold, neighboring threshold, reverse-order and repeated-call controls;
and complete resulting state. A caller-level pulse-count witness is needed
before claiming an actual timing-path correction. Preserve all original
evidence. The 122 continuous P records, DDH and other scientific gates remain
separate. Another unchanged full 113-bit campaign would not decide this policy.

## Evidence checked for this brief

These are existing executed results, not new producer runs. The receipt hashes
below were verified; all eleven copied 0077 receipt/log/difference bindings were
checked, and the 32 SIMUL entries were read directly. Additional work was source
inspection and exact Python `fractions.Fraction` arithmetic on the saved inputs.

| Evidence | SHA-256 |
| --- | --- |
| `decisions/evidence/numeric-boundaries/build/fc78rsrs/review.json` — 16 cases, 56 comparisons | `1314d2a657a9037970b76ed40d30c8f000767ce6a048d4037739ed5394fdcc0d` |
| `decisions/evidence/preprocessing-precision/build/review-20261025/receipt.json` — 296 histories, 154 differences at 128 / zero at 113 | `d47655371e4b90dcedc9da022edf1242731e69c269f553891674e7d9abfa7873` |
| `validation/input_to_bat/build/preprocessing/simul-precision-diagnosis.json` | `7187511076f94cdb238316b42b064f79ea0f901b6f1117f028696917a0f0f0d3` |
| `validation/preprocessing/observations.json.gz` | `1e44d2446fa37ce22cbe0de36fbd1c2879b0005861c94819181b7e97a13999f2` |
| `validation/preprocessing/initial.json.gz` | `94f933a3744bff139cf141eed8241d75f78089161a1e8d43eb298874887fddc8` |

Authority: [0049](../../decisions/0049-numeric-utility-boundary-evidence.md),
[0058](../../decisions/0058-time-ephemeris-rust-contract.md),
[0074](../../decisions/0074-implementation-handover-and-acceptance.md) and
[0077](../../decisions/0077-preprocessing-precision-diagnosis.md).
