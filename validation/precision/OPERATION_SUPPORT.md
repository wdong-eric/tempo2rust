# Operation inventory and source audit

Source: TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. Paths below are
relative to `../../tempo2/`. This is an operation inventory, not a claim that the
experiment implements these whole routines.

| Scope | Source locations | Extended work | Existing binary64 work / convention |
| --- | --- | --- | --- |
| PAR/TIM input | `TKlongdouble.C:44-80`, `readParfile.C:2826-2849,2889`, `readTimfile.C:262-265` | Direct decimal -> `longdouble`; parameter values and `sat` | PAR translates D/d exponents to e. RA/DEC sexagesimal conversion goes through double routines. TIM also keeps split fields; its full `sat` is parsed directly. |
| M1a spin | `formResiduals.C:515-543` | `(bbat-pepoch)*86400+torb`; separate integer and fractional F0/day terms; multiply F1-F12 powers in source order | Integer F0/day casts and binary64 literals such as 86400 and 0.5 (exactly representable). The experiment exercises F0-F3; higher derivatives are inventoried only. |
| Reference phase and pulse counting | `formResiduals.C:2194-2208,2330,2350-2355`; `tempo2Util.C:186-219` | Signed remainder, sign-dependent +/-0.5 then truncation, subtraction and divide by F0 | `fortran_mod` casts quotient to C int; `fortran_nlong` casts to C long. Track/jumps and mean subtraction are separate conventions, not interchangeable rounding modes. |
| M1b epoch assembly | `formBats.C:67-83` | Add seconds/day corrections to large MJD; subtract Shklovskii correction | Individual Roemer, Shapiro, dispersion and troposphere values are largely double. Preserve the original grouping and each intentional cast. |
| M1b time scales | `tt2tdb.C:68-70,127-150,210-275`, `units.C:48-65`, `ifteph.h:56-64` | Form `mjd_tt`, `mjd_teph`, extended scale `IFTE_K`, affine epoch transform | Time-ephemeris interpolation returns double; drift explicitly casts `(mjd_tt-IFTE_MJD0)` to double. `IFTE_MJD0`, `IFTE_KM1`, `IFTE_TEPH0` originate as unsuffixed double literals. Parsing the printed constants directly at higher precision would change source semantics. |
| BT / ordinary ELL1 | `BTmodel.C:37-55`, `ELL1model.C:61-67,154-175` | Subtract epochs before narrowing; ELL1 optional orbital interpolation uses `sinl` | Main orbital variables, Newton solves and returned delays are double in these routines. Do not indiscriminately promote all calculations. |
| DD and related binary work | `DDmodel.C:38-50,118-165`; `TKlongdouble.float128.h:18-82` | Extended orbital phase, `sin`, `cos`, `sqrt`, `atan2`, `log`, powers, comparisons; Newton solve | Some constants (M_PI, SUNMASS) are already double. Header overloads route extended operands to quadmath in A's build. DD's negative orbit adjustment and 1e-12 Newton stopping rule are source conventions, not repaired here. |
| T2 / further models | `T2model.C:57-84,91-118,254-255`; `formResiduals.C` extra phase terms | Mixed extended epochs and double orbit parameters; optional extended trig, exp/log and powers | Coverage requires selected model-specific fixtures; scalar tests do not validate these models. |

## Package APIs actually inspected

Pinned registry source lives beneath `.cargo-home/registry/src/*/`; Cargo.lock
records package checksums. `qd-0.8.0/Cargo.toml` identifies
[sarah-quinones/qd](https://github.com/sarah-quinones/qd), MIT licensed. This is
neither barandis/qd nor C++ QD.

| Needed operation | qd 0.8.0 | Rug 1.30.0 / MPFR | Executable evidence |
| --- | --- | --- | --- |
| Direct decimal parsing | `Num::from_str_radix` always returns `Err`; no FromStr | `Float::parse` -> `Float::with_val(bits, ...)` directly | `parsing.tsv`; `qd-missing.log` |
| PAR e/E/D/d exponent input | No native parser | e/E native; D/d explicitly normalized in the finite decimal boundary | Four long MJD exponent forms, `1D-3` assertion |
| Malformed/non-finite input | All strings rejected by native trait, including valid ones | Native parser accepts non-finite, whitespace and underscores; study boundary rejects them and checks finite result | `parsing.tsv`; failure followed by successful parse assertion |
| Decimal serialization | No Display / decimal formatter; Debug shows limbs | `to_string_radix(10,None)` emits enough digits to recover exact value | Native qd compile failure; 16 Rug exact round trips. qd decimal/limb transport is explicitly assisted |
| +, -, *, /, unary minus | Available; operators + and - use estimate versions; accurate variants also exist | Available, rounded to destination precision | Arithmetic, epoch and spin rows |
| Finite/NaN, ordering | Available, but inherent `le`/`ge` mishandle equal high limbs; derived <= works on tested normalized finite inputs | Available | `special.tsv`; comparison assertions |
| Abs, sqrt | Available | Available | sqrt matrix and domain diagnostics; abs used in the MPFR comparison path |
| Truncation | Available but wrong for a high limb exactly integer with opposite-sign fractional low limb | `trunc`, toward zero | Boundary matrix: qd returns 1 for 0.99999999999999999999 |
| Nearest pulse | No native round; source +/-0.5 + trunc inherits trunc bug | Source expression tested; Rug `round` uses ties away from zero; `round_even` is a different operation | Exact half ties and +/-1e-20 offsets near zero and +/-1e11 turns |
| Floor / ceil | Missing | Available | qd floor compile error; Rug boundary matrix |
| Signed remainder | `%` computes `self-(self/rhs).trunc()*rhs`; inherits trunc bug | `%` maps to MPFR fmod (toward-zero quotient). **Do not substitute `remainder()`**, which uses nearest-integer quotient | Signed integer-boundary matrix |
| exp, ln, log10 | Present; measured domain passes. exp clamps at +/-709 and has no full MPFR-like exponent range | Present | Math matrix, `special.tsv` range edges |
| sin / cos / tan / asin / atan2 | Missing | Present | qd compile errors; Rug 41 orbital math cases (including powers) |
| Integer powers / real powers | Repeated products work; no general pow interface | Repeated products and `Pow` available | Spin F1-F3, selected real powers |
| C long double / __float128 ABI | Two f64 limbs; **not ABI compatible** | MPFR allocation/struct; **not ABI compatible** | No reinterpret casts or calls across these representations |

Specific source checks:

- `qd-0.8.0/src/lib.rs:58-80`: public pair, normalized-limb assumption,
  `MANTISSA_DIGITS=105`, `DIGITS=31`.
- Lines 181-215: comparison methods, including the equal-high-limb issue in `le`.
- Lines 238-337: sqrt, exp, ln, log2/log10; exp's +/-709 early exits.
- Lines 339-344: component-wise truncation; 409-434: operators and remainder;
  492-503: unconditional parser error. The study does not patch any of these.
- `rug-1.30.0/src/float/big.rs:803,1758,11281,11428,12730-12765`:
  parse, round-trip formatting, away-from-zero round, truncation, MPFR parsing
  with end-pointer check. `float/arith.rs:130,714` routes `%` to fmod.
  `ext/xmpfr.rs:424-448,482-500` routes math/rounding/remainder to MPFR.
- `f128_internal-0.2.2/src/f128_t.rs:230-233` and `src/f128.c:15-16`:
  parser validates only interior NUL, calls `strtoflt128` without end-pointer
  validation. `f128_impl.rs:24-40`: default format `%Qg` has too few digits;
  explicit precision uses `%.NQg`. The feasibility probe uses 36 digits.

Missing qd parsing, formatting, rounding fixes and trigonometry would create
project-owned numeric maintenance. Rug already supplies those mathematical
operations; selecting it requires native dependency packaging and a small
finite-input / precision policy, not new mathematical implementations.
