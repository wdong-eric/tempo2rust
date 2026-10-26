# AG prospective comparison contract

Freeze baseline/source and recipes: rust/ANALYSIS_TOOLS_API.md. These budgets
are fixed before Rust comparisons. Existing gates are unchanged. Native source
uses the pinned A2 binary128 configuration; production uses Rug128. Compiler
contraction observations must be recorded, not silently treated as extra budget.

Exact: shape, source/written/reported counts, row/family/bin/realization membership,
seed writes, MT words/index/draw counts, integers/uniform bits, sorting/associated
arrays, warnings/statuses, structural zero, signed zero, infinity/NaN sign/class.
NaN payload is not a scientific observable. For ordinary nonzero numerical zeros
from cancellation, use the quantity scale; structural zeros are separately tagged.

| Quantity | Units and budget |
| --- | --- |
| f64 toolkit reductions/conversions | 5e-14 times max absolute input in that quantity; variance uses squared input scale; f32 results 2 ulp; min/max/sort/sign/conversions exact |
| Gaussian RNG | 5e-14 absolute; exact accepted/rejected draw history and uniform bits |
| Spectral frequency | cycles/day, 5e-14 relative per bin |
| DFT/LS coefficients and amplitude | seconds, 5e-14 times maximum reference component; real and imaginary independently |
| Spectral power/PSD/error | s^2, yr^3 or dimensionless as actual route; 5e-14 times maximum reference magnitude per independently typed output, retaining AE standalone density gate |
| Spline/smoothing/differences | x days and y seconds, 5e-14 times independent reference/input max; f32 differencing exact |
| Sigma-z fit coefficients/covariance | s/day^k and corresponding squared unit; 5e-14 times independent column/reference scale |
| Sigma-z log tau/bias/error | log10(year), log10(dimensionless sigma-z), log10-error respectively; 5e-14*max(1,abs(reference)) |
| GW angular geometry and harmonics | radians / dimensionless; 5e-14 absolute |
| GW angular frequency | rad/s, 5e-14 relative |
| GW amplitudes/strain | dimensionless, 5e-14 times maximum reference amplitude per polarization family |
| GW residual / timing | 1e-12 s; phase 1e-10 turn, unchanged |
| Eccentric histories | source f64 units independently, 5e-14 relative; flags exact |
| eOut | source kg/s^3 (W/m^2), 2 binary32 ulp, exact class/sign; this source estimate is not validated radiated energy |
| File exchange | exact headers/counts/cursors/prefix bytes for represented values; binary128 nearest-even serialization boundary; class/sign exact, NaN payload canonicalized |

Shared fitting matrix/solver/error gates remain validation/general_fitting/CONTRACT.md.
Original bug witnesses and corrected comparisons are separate. No comparison can
pass by omitting a field/family, sorting modes, accepting stale output, replacing
units or using distribution averages. Rejection controls mutate each such field.

Native layout witness `layout.cc`: int 4, long 8, timing scalar 16/alignment 16,
little endian; tensor prefix 9 scalars/144 bytes and general 17/272. Full structs
544/672 are not wire records. Native binary128 signed zero, normal/subnormal,
infinity and quiet NaN byte witnesses are recorded before codec implementation.
Command: g++-16 -O2 -DHAVE_CONFIG_H -Ivalidation/reference/build/source
validation/analysis_tools/layout.cc -lquadmath -o validation/analysis_tools/build/layout.
Executable SHA256 bd221cea9da840872aef2f0a65407c2697a020632f07a5bb6905a8b9e17db4a9.

Implementation order: toolkit/RNG, full spectrum and sessions/sigma-z, GW and
file codec, public consumers/verification, final report. All inherited 0088/0089
failures remain open, including seven AF raw log10RMS cells.
