# Pre-run contracts and error budgets

Defined before the numerical experiment, 19 October 2026. These are local study
gates, not approved whole-engine tolerances. Each comparison computes an absolute
difference in MPFR, without first reducing the candidate to binary64.

| Quantity | Study gate | Reason |
| --- | --- | --- |
| Epoch parse and decimal serialization, converted to seconds | 1e-12 s | 1 ps allocation, 1000 times below the 1 ns delay case |
| Nearby-epoch difference and recovered +/-1 ns, +/-4 ns delay | 1e-12 s | Resolve a nanosecond with margin, including day crossings |
| Reduced and absolute spin phase | 1e-9 turns | At the synthetic 716 Hz frequency, about 1.4 ps |
| Fractional spin phase | 1e-9 turns | Compare the signed, rounded residual phase, not a wrapped distance |
| Spin residual | 1e-12 s | Independent allocation; do not hide a pulse slip by wrapping |
| Pulse number, integer rounding, truncation | Exact | Discrete scientific outputs |
| Scalar arithmetic and transcendental values | 1e-28 * max(1, abs(reference)) | Probe extended accuracy, including dimensionless orbital arguments; not an engine tolerance |
| Signed remainder near unit boundaries | 1e-28 absolute | Detect incorrect quotient truncation |
| MPFR 256 versus 512-bit reference | 1e-60 * max(1, abs(reference)) | Must be negligible relative to all candidate gates |
| Serialization then parse | Exact value for Rug and exact limbs for qd diagnostic transport | Do not allow the comparison path to erase low bits |

Reference inputs are exact decimal strings rounded directly at 256 and 512 bits.
The proposed Rug candidate uses **128 significand bits**, distinct from IEEE
binary128 (113 significand bits). qd advertises 105 significand bits and is a
pair of binary64 values. qd has no native decimal parser/formatter: its arithmetic
tests are explicitly MPFR-assisted. The adapter splits the 512-bit decimal into a
nearest binary64 high limb and a separately rounded residual low limb. Comparison
reconstructs both limbs exactly at 512 bits. This is not a production solution.

Synthetic spin cases use F0=716.12345678901234567890123456789 Hz,
F1=-1.234567890123456789e-15 Hz/s, F2=2.34567890123456789e-27 Hz/s^2,
F3=-3.4567890123456789e-39 Hz/s^3, PEPOCH=55000.1234567890123456789012345 MJD,
and spans up to +/-20000 days. They test the source's F0 split and F1-F3 term
order, with no binary delay, other phase terms, fit, or mean removal. The
PEPOCH anchor is the reference observation, hence its reference phase is zero. All
synthetic times share one abstract uniform barycentric time coordinate; no
TCB/TDB conversion, ephemeris or clock realization is applied. Original TIM
epochs are also tested as decimal numbers, not as recomputed arrival times.

Boundary probes independently test +/-half ties and offsets of 1e-20 turns,
including phases of order 1e11 turns, and values just below signed integer
boundaries. TEMPO2's fortran_nlong uses sign-dependent +/-0.5 then truncation
(ties away from zero), not ties to even. fortran_mod uses truncation toward zero
and a signed remainder. Its C int conversion range is an upstream limitation;
the synthetic reduced spin phases remain inside it. No tolerance is relaxed
after a failure. Invalid/non-finite input and invalid math domains have explicit
recorded outcomes; they are not silently coerced into valid timing values.

Additional diagnostics use the same gates: source `units.C` affine epoch
transforms use 1e-12 s, and boundary fractional phase/residual use 1e-9 turns /
1e-12 s. The affine test deliberately promotes the upstream binary64
`IFTE_MJD0` and `IFTE_KM1` values exactly, rather than reinterpreting their printed
decimals. An independent unsplit spin polynomial at 512 bits checks the
source-order transcription at 1e-140 * max(1, abs(reference)); this checks an
algebraic identity, not a production tolerance. The PEPOCH anchor is separately
asserted to have zero phase, so its reference-phase subtraction is zero.
