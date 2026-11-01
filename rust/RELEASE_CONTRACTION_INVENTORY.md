# Coordinated contraction profile review (0111)

Updated 2 November 2026 under 0117. The measured five-unit correction below is
released in addition to clock interpolation (0110) and LMST (0111). Unmeasured
sites remain unchanged. Implementation is not corrected Linux acceptance.

The saved Linux profile is GCC 13.3.0, x86_64, `-O2 -std=gnu++11`. The accepted
macOS profile is GCC16, arm64, with the same optimization/language flags.
`OBSERVER_COORDINATES_REPORT.md` records AA's eighteen original whole-history
contraction witnesses and object/assembly evidence; `TERRESTRIAL_RESOURCES_REPORT.md`
records W's four bounded scalar witnesses. Those measurements are host-specific.

## 0117 implementation inventory

The fourth Linux run binds GCC13 `-O2 -std=gnu++11` for these five units.
Central's 144-member review receipt is
`43019b5c9972dbc8c25cc1c3f5918ffdd2fc102ac29d1b53e8f440a25e0c8cdd`.
Actual observer/terrestrial objects contain no FMA instruction or symbol found
by that bounded review. Original and contraction-off supplementary assembly
agree apart from debug-string ordering and the producer flag string; all 22
bounded terrestrial scalar outputs agree. The exact review recipe and original
case-sensitive archive are recorded in decision 0117. No assembly normalization
is applied to scientific products.

Every expression below has binary64 operands/results, including the listed
integer-to-double conversions. Linux follows the source grouping shown;
macOS keeps the existing measured GCC16 contractions. No libm call, preceding
wide SAT calculation, integer boundary, table selection, or correction policy
changes. First direct output fields are named separately from the saved first
failure: an earlier catalog difference can mask later driver fields.

| Source counterpart | Separate source contract | First direct output / affected gate |
| --- | --- | --- |
| `observatory.C::fang:373-375` | Cast integer degree/minute components to double; retain `f-ia*1e4`, then `f-ia*1e4-ib*1e2` | Parsed angle; W catalog and observer catalog state |
| `observatory.C::ITRF_to_GRS80:396,403,405-415` | `x*x+y*y`, `u*u+esq*esq*q`, `(u+v)+w*w`, `D*D+z*z`; preserve longitude subtraction/addition and all divisions/sqrt/atan2 | Geodetic height/latitude/longitude; saved observer catalog coordinate[4] differs by one ULP; W geometry |
| `observatory.C::GRS80_to_ITRF:423,426` | `1-esq*sin(lat)*sin(lat)` and `N*(1-esq)+height` | Cartesian z, then roundtrip geometry; W/observer catalogs |
| `get_obsCoord.C::get_precessionMatrix:568-571` | `t*(a+t*(b+t*c))/seconds_per_rad` for each angle | Precession angles feeding c010 matrix[0][0], whose saved bits differ by one ULP |
| `get_obsCoord.C:587-598` | Left-to-right products and source signed sum/difference in four matrix cells | Four precession cells; c010 and legacy driver histories |
| `get_obsCoord.C:629` | `(nut0*prc0+nut1*prc1)+nut2*prc2` | All final matrix cells; c010 and legacy driver histories |
| `get_obsCoord.C:255-260,282-287,310-315` | Slope unchanged; `vl-lo*m`, then narrowed `sat*m+intercept`; mode 2 unchanged | Satellite position axis; c114-c123, with saved c114 cancellation witness |
| `get_obsCoord.C:388` | `(x*x+y*y)+z*z` before sqrt | Legacy radius feeding observer position/velocity; c041/c127/c131 macOS witnesses |
| `get_obsCoord.C:399` | `84381.448+t*(-46.815+t*(-0.00059+t*0.001813))` | Legacy obliquity feeding phase/position |
| `get_obsCoord.C:410,424` | `(tsid+n0*cos_oblq)-alng`; `(eeq0*prn0+eeq1*prn1)+eeq2*prn2` | Legacy phase and position axes; full observer histories |
| `tai2ut1.C::ut1red:203-205,210` | Convert table integers first; retain `f2=(1/6)*sum`, two coefficient assignments, then `t*(y1+t*t*y2)+s*(y1+s*s*y2)` | UT1 correction; saved c017/c023 differ by one ULP |
| `eop.C:197-199` | Three `previous+f*difference` writes; leap subtraction and dut1dot unchanged | xp, yp, dut1; W EOP state/actions |
| `tropo.C:116-127` | Seasonal subtraction; separately interpolate average/amplitude, then subtract their product | Hydrostatic coefficients/mapping; W scalar and driver cases |
| `tropo.C:156` | Assign height correction before adding basic mapping | Hydrostatic mapping; accepted macOS bounded ULP witness |
| `tropo.C:204-206` | `f*upper+(1-f)*lower` for all three coefficients | Wet mapping; retain 0053's corrected upper c coefficient and controls |
| `tropo.C:434-435,450` | `(1-.00266*cos(lat))-2.8e-7*height`; `zhd*mh+wet*mw` | Zenith hydrostatic and total tropospheric delay; W driver and independent scalar probe |

The W test observer repeats its native probe's two driver expressions with the
same host distinction. It still compares independently emitted state and scalar
fields. The changed production sites use explicit source expressions, avoiding a
global multiply-add policy. Full W and observer comparisons, native controls,
mutation controls and downstream regressions remain required on both hosts.

## Original 0111 diagnosis inventory (before the fourth Linux run)

| Group / counterpart | Types and source grouping to retain | Existing evidence and next bounded comparison |
| --- | --- | --- |
| Clock `tabulatedfunction.C:163-164` | Binary64 subtractions/division, then final product/addition | 0110 binds the Linux separate tree and macOS final FMADD. `993ec81` branches only this site. Real five-query API and full macOS comparison pass; corrected Linux execution pending. |
| LMST `get_obsCoord.C:496-531` | Binary64 Horner terms, grouped three-product sum; final wide modulo unchanged | 0111 binds all 45 queries / 90 scalars. `75692fd` restores the Linux grouping. The real API passes locally; complete observer campaign is tracked separately. |
| Precession `get_obsCoord.C:568-629` | Binary64 polynomial angles, trigonometric results, ordered matrix products/sums | Saved Linux `c010` first differs at matrix[0][0], `3feffd90f58b964c` versus `3feffd90f58b964b`. AA's macOS c010 evidence supports the existing contractions. Compare intermediate angles/products on Linux before changing any site; matching nominal types does not establish matching libm results. |
| UT1 `tai2ut1.C`, interpolation body | Binary64 coefficient construction and grouped cubic interpolation | Saved Linux c017/c023 differ by one binary64 ULP. Preserve table selection, warning/cache state and cubic grouping. Compare original compiled coefficients, interpolation terms and final result with explicit separate/fused witnesses. |
| Observatory `observatory.C:373-375,396-426` | Binary64 angle component subtraction and both geodetic conversion directions; integer casts unchanged | Many saved observer histories first differ at catalog coordinate[4], `bfe93d8244e4ce21` versus `bfe93d8244e4ce20`. Fresh macOS source/off witness reproduces these latitude bits in the opposite host branches. This is a strong localization lead, not proof for every downstream history or every conversion expression. |
| Satellite offsets `get_obsCoord.C:254-259,280-285,307-312` | Binary64 slope/intercept; explicit narrowing of SAT, then evaluation | Saved c114/c115/c118/c119/c122/c123 differ in position; c114 native `408f980000000000`, Rust `408f980000000006`. Observe intercept and evaluation independently, retaining mode 2 and interval boundaries. |
| Legacy site geometry `get_obsCoord.C:388,399,410,424` | Binary64 norm, obliquity polynomial, sidereal/nutation addition and matrix-vector products; earlier SAT arithmetic remains wide | AA c041/c127/c131 witnesses support macOS contractions. Saved Linux histories can fail earlier at catalog geometry; isolate that earlier difference before attributing their later positions here. |
| EOP `eop.C:197-199` | Three binary64 final interpolation writes; preserve preceding differences and leap adjustment | W's accepted assembly identifies three FMADD sites. Fresh grouped campaign retains original/off assembly; source-specific Linux scalar localization remains pending. |
| Niell/driver `tropo.C:116-127,156,206,434-435,450` | Binary64 coefficient interpolation, height correction, zenith denominator and final delay | Fresh macOS bounded experiment repeats the hydrostatic ULP difference. Preserve 0053's separate wet-coefficient correction. Linux terrestrial corpus availability and original/separate/fused scalar evidence must precede any arithmetic edit. |

The original Linux observer receipt has 95/133 passing histories. Its first
differences are retained, including those masked by an earlier failing field.
They were read by exact case-sensitive ZIP member name and checked against the
third-run manifest. Neither the above grouping nor the two accepted corrections
closes all 38 failed histories.

## Reusable campaign and measured limit

`validation/release/contraction_profile.py` reuses W's existing source-extracted
bounded experiment, changing only its compiler/paths and removing its old
macOS-only requirement that a contraction-off run must differ. It binds the
actual ordinary profile and objects, records actual-object disassembly, and
emits supplementary source assembly with the original flags and with
`-ffp-contract=off`. U includes primary sources in `probe.cc`; the receipt names
that actual translation unit instead of claiming nonexistent separate objects.
It does not enable fast math or edit production Rust.

Fresh macOS evidence is `validation/release/build/third-contraction-profile-v2/`.
The original/off bounded experiment repeats exactly four differences: latitude,
height, roundtrip height and one hydrostatic sample. The campaign is included as
one diagnostic stage in the existing integrated runner. Its next Linux result
will supply compiler/object evidence and the same bounded scalar products.
Explicit fused/separate comparisons for the remaining observer intermediates
are still required; this assembly packet alone does not close them.

Central's frozen lexical inventory has 130 matching lines across 23 files. It
includes comments/tests, predictor ordinary arithmetic, already qualified
Lomb/QR paths, and other sites in `cholesky`, `ifteph`, propagation, ephemerides,
shapelets, solar wind, likelihoods and utility routines. Keep these unverified
sites, explicit native fused operations and already platform-aware helpers
unchanged. Reuse the existing Rug/native arithmetic and compiler tools; no new
numerical library or global contraction helper is warranted by this inventory.
