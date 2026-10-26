# Frozen AH comparison contract

Scope is decision 0080, source fda6ad4, production baseline 732fd0e.
Raw output is retained under build. Identical representable state requires exact
ordered bytes, filenames, record membership, units, fit flags, numeric classes,
signed zero and report mutations; deterministic digits are never stripped.
Only explicitly recorded source/build identity, CPU time and command-history date
may differ as named execution metadata. Source locations remain mapped.

Real-file timing gates remain 1e-12 s and 1e-10 turns; fitting/covariance gates
remain AE's quantity contract. Display comparison checks rounding cells at the
literal conversion precision, separately from internal numerical comparison.
PAR reload is compared to its printed decimal values and source coordinate/error
conversions, not to unrounded engine values. Helpers with identical binary inputs
require exact f64 outcomes/classes. The native binary128 (113 significant bits)
versus Rug128 distinction is provenance, not a numerical allowance or pass rule.
Timing, phase and AE budgets apply only to their matching named quantities.
New raw extended helper/state quantities have no precision-based allowance: before
comparison, declare units, native type, scale, exact or bounded rule and zero/class
handling. Otherwise record them as ungated diagnostics awaiting central disposition.
Mass outputs are solar masses; m2's source iteration tolerance is 1e-10 relative
step (not a comparison allowance). dglep returns days, with source phase stopping
1e-6 turns and step stopping 1e-12 relative seconds. Source sentinel/structural
zeros and convergence branch decisions are exact. Derived f64 display quantities
require the literal source rounding cell; failures stay failures.

The identical-state probe uses dyadic extended inputs (including signed zero),
so both native binary128 and Rug128 receive exactly the same numbers. Its RMS
weights are inverse squared turns, RMS outputs are microseconds, mass outputs
are solar masses and glitch offsets are days. Every returned binary64 value,
class, zero sign and mutation is exact; there is no absolute/relative tolerance
or floor at zero. Formatting uses exactly representable input numbers, native
binary128 for L conversions and binary64 for ordinary conversions. Its bytes
and successful return values must be exact, including nan/inf and padding.

Controls must reject missing/reordered records, altered units/flags/digits and
missing files. Failure then valid processing and repeated stateful calls protect
owned I/O and mutation semantics. All inherited numerical failures in 0091 remain
open; no AH comparison accepts them. Native comparison is implementation evidence,
not independent scientific validation or Linux qualification.

AH modern-terrestrial UT1 diagnostic correction (27 October, before comparison):
`correctionUT1` is seconds from the site TOA to UT1. Retain the binary64 clock
correction actually applied when constructing UTC, add the selected EOP's
binary64 UT1-UTC once in binary64, then promote the result to timing storage.
For clockCorr 0/2 the source applies no site-to-UTC correction, so that addend
is exactly zero. Compare the complete sum exactly, with no tolerance or zero
floor. Original sentinel -12345 is instrumentation proving the missing write,
not a source value. Before/after BAT, residuals, geometry and all other retained
Rust observation fields must be byte-identical after removing only this named
newly initialized diagnostic field. Barycentric/satellite paths do not gain a
value without this producer. The existing EOP/clock source and timing gates stay
unchanged; this is no new UTC/UT1 model or fallback.
