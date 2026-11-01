# Coordinated contraction profile review (0111)

1 November 2026. Only the clock interpolation (0110) and LMST (0111) production
corrections are released. All other sites below remain unchanged. This inventory
groups source obligations; it does not classify lexical matches as defects.

The saved Linux profile is GCC 13.3.0, x86_64, `-O2 -std=gnu++11`. The accepted
macOS profile is GCC16, arm64, with the same optimization/language flags.
`OBSERVER_COORDINATES_REPORT.md` records AA's eighteen original whole-history
contraction witnesses and object/assembly evidence; `TERRESTRIAL_RESOURCES_REPORT.md`
records W's four bounded scalar witnesses. Those measurements are host-specific.

## Prioritized source groups

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
