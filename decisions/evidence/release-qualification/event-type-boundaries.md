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
