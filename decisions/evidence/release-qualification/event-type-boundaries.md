# Event expression types and fixture scale

Central source review, 30 October 2026, while Linux run 41056090173 remains
in progress. This is a diagnostic contract and scale estimate. It does not
establish the cause of the Linux timing difference or approve a correction.
Production Rust, native source, gates and conventions are unchanged.

## Source types

`event-type-boundaries.cc` is the exact executed compile-time probe for native
`formResiduals.C:675–728` at `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
It includes the pinned `TKlongdouble.float128.h`, whose `powl` macro expands to
`powq`. The probe checks these C++ expression types:

- `freqSSB / 1.4e9` is calculated as double, then assigned to ordinary long double.
- `bbat - epoch` is timing binary128, then narrowed to ordinary long double.
- Event amplitude, width and index are assigned to ordinary long double.
- The exponent argument and `exp` result use ordinary long double.
- `powl` and the products involving its result use binary128.
- The Gaussian product is narrowed into ordinary-long-double `val` before
  multiplication by binary128 F0 and accumulation into binary128 `phase4`.

The local GCC 16 probe compiles and runs successfully: ordinary long double has
53 significand bits on macOS arm64. The retained first Linux A2 precision witness
reports 64 ordinary-long-double bits and 113 timing bits. The expression probe
itself has not yet been compiled on Linux. Do not replace the mixed expression
with uniformly 64-bit or uniformly 128-bit arithmetic based on this local check.

The ignored packet `build/event-type-boundaries-20261030/` contains the exact
header, source, binary, commands, compiler identity and outputs. Header SHA-256:
`214911b81f82489996d698626cf8d30ab5bcc20099f6b5c5aa089715b85bc55e`.
Probe SHA-256: `0bf67c7e678740fdee69ef8bdbeb11ff17e5752d33b5f4de69974c19693f5572`.
Receipt SHA-256: `004547184cea2d02508b595fc2c6fc6054e5521b5ae4a811c51382b8203d8d00`.

To replay, export the pinned header into a fresh directory, compile the authored
probe with that directory on the include path using the target native compiler,
`-std=c++17 -O0`, and execute it. This checks types and the ordinary type width;
it performs no timing-model calculation.

## Why small rounding differences can matter in this fixture

The existing source contract in decision 0063 preserves
`calculate_bclt.C:80–82`: when `delayCorr == 0`, `freqSSB` receives `freq` without
MHz-to-Hz conversion. `validation/timing_models/fixtures/events.tim` uses this
barycentric branch. The first Linux native export and Rust output both retain
2100, 800 and 1600 for its three post-epoch observations; the Rust output explicitly
labels those values MHz. `formResiduals.C:684` nevertheless divides them by 1.4e9.
The native exporter writes the actual stored field, without rescaling it.

For the fixture's EXPPH = 3e-5, EXPTAU = 2 and EXPINDEX = -2, evaluate
`3e-5 * (freqSSB / 1.4e9)^(-2) * exp(-dt / 2)` using the recorded BBAT and the
actual binary64 frequency quotient. A 90-digit Decimal calculation gives these
scale estimates, before F0 multiplication and residual wrapping:

| Observation index | dt (day) | Exponential contribution (s) | Shift from narrowing 3e-5 to f64 alone (s) |
| --- | --- | --- | --- |
| 2 | 0.25 | 11,766,625.3678 | +2.98097931713e-10 |
| 3 | 1.5 | 43,398,677.0331 | +1.09947036285e-9 |
| 4 | 4 | 3,108,482.28684 | +7.87508832405e-11 |

For the last column substitute `Decimal.from_float(3e-5) - Decimal('3e-5')`
for the amplitude in the same expression. These are isolated scale estimates,
not measured native/Rust differences or a reproduction of all rounding sites.
They explain why considering only the nominal 30 microsecond amplitude would
underestimate sensitivity to ordinary-long-double boundaries. Preserve the
existing frequency convention and raw failure while testing the source-type
hypothesis. Changing that convention is a separate scientific decision.

The running Linux event diagnostic retains the original native output, verifies
observer neutrality, and separately replaces only the ordinary-long-double event
temporaries with double. Its actual result is needed before causal attribution
or an implementation handoff. Any eventual correction must preserve every source
conversion boundary, operation order, original evidence and existing gates.


## Connected source obligations and conditional AL proposal

AL completed a read-only design review in chat
`01a17c51-359e-7041-b04b-efebd2b2ec6e`, turn
`01a18ce1-3d87-72d2-b209-202610427719`, at 2026-10-30 13:00:33 UTC.
Central checked the connected source expressions. No implementation or numerical
policy is authorized by this proposal; the actual Linux intervention remains
its first dependency.

A coherent event correction must cover both residuals and
`t2fit_glitch.C::t2FitFunc_expdip/t2FitFunc_gausdip`. Let D denote binary64, L the
measured ordinary long double, and W the existing native-binary128/Rust-Rug128
timing representation. The W mapping remains unchanged. Each L operation must
round at its own source boundary; evaluating the whole expression in W and
narrowing only the final value does not reproduce it.

The residual contract is given above. In the callbacks, with P the W power and E
the L exponential, the additional boundaries are:

| Callback | Source expression and conversion before returning D |
| --- | --- |
| EXPPH | P * E in W, then assign L val, then convert to D |
| EXPTAU | dm * P * E * dt / tau / tau, left to right in W, then L val, then D |
| EXPINDEX | dm * log(freq) first in L; multiply by P and E in W, then L val, then D |
| EXPEP | dm * P * E / tau in W, then L val, then D |
| GAUSAMP | Common Gaussian product in W, narrowed into L val; val / amp in L, then D |
| GAUSEP | val * ((-dt / sig) / sig) in L, then D |
| GAUSSIG | val * ((((dt * dt) / 6.) / sig) / sig / sig) in L, then D |
| GAUSINDEX | val * log(freq) in L, then D |

Preserve source exp/expl selection, the narrowed-dt activation test, zero-return
branches, predicates, loop order, signs, the Gaussian 6. denominator, and existing
parameter updates. The purpose is source-type consistency, not rederiving or
correcting the source derivative formulas.

AL proposes sharing only host-width selection and the existing ordinary-type
rounding kernel from `predictor_number.rs`, retaining PredictorNumber's public
API. That kernel preserves rounding direction through subnormal conversion and
the host exponent limits. Predictor parsing, formatting, quad_cos and contraction
helpers do not belong in event expressions. Keep the existing macOS f64 exp/ln
path; Linux ordinary-width transcendental results require direct native evidence.
This is a proposed reuse boundary, not a decided module refactor.

Before implementation authorization, inspect the actual Linux event intervention
and observer neutrality. Then bound any missing evidence with the Linux type
probe extended for log/expl and callback assignment/return conversions, exponent
arguments/results and Gaussian val, and direct native outputs for all eight
callbacks. The existing GAUSINDEX=2 fixture suppresses its Gaussian term under
the preserved frequency convention, so add a separate Gaussian-only witness that
can distinguish ordinary-width narrowing. It must complement the original
fixture, not replace it. A candidate must retain the original event gates, the
event-amplitude fit case, exact macOS before/after event outputs, and predictor
rounding controls if the shared kernel changes. Do not infer numerical agreement
from matching declared precision alone.

A bounded source scan of the 93 responsibility files found related sites that
remain outside the focused event proposal:

| Source boundary | Current Rust counterpart / disposition |
| --- | --- |
| dm_delays.C:139 ordinary cmval accumulation | propagation.rs:216 explicitly retains the earlier macOS f64 profile |
| formResiduals.C:1456 ordinary cosmic-string dt | residual_source.rs:3000 uses f64 |
| t2fit_stdFitFuncs.C:29–32 and spin L-suffixed expressions | t2fit_stdFitFuncs.rs and the shared std_freq.rs currently use f64 |
| secularMotion.C:40–41 L-suffixed constants before timing promotion | propagation.rs:809–810 constructs them from f64 |
| readEphemeris_calceph.C:215 ordinary JD assignment | Already uses PredictorNumber::from_float in calceph.rs:220; not a missing conversion |

These are source-contract obligations, not established causes of particular
Linux failures. The scan does not claim to exhaust literal/typedef/compiler
semantics or full function coverage. Keep the broader ordinary-type review
visible in AL's remaining ledger without silently expanding an event correction
or changing any caller before its own source/evidence review.
