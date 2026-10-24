# AB original propagation reference contract

Released at b8d6263 under decision 0063. Native source is
fda6ad4a9753ad7dc5d291406f2f310a9161423f. Only this directory is owned.
W bd290cc supplies fixed Rust context; no evolving central/Y/Z/AA files enter
immutable membership. Decisions 0039 and 0053 govern evidence and corrections.

`inventory.py` freezes all 25 routine bodies (including both ifunc overloads),
lexical decisions, preprocessor sites, raw caller/declaration occurrences, source
hashes and S's complete typed map: 349 pulsar and 95 observation fields plus
parameter, clock correction, precision and fit types. The generated local map
contains exact source lines, field names, declarations, types and extents.
Textual caller occurrences include plugins and are not claims of execution.

## Source and caller contracts

| Primary | Order, precision and state | Native caller safety |
| --- | --- | --- |
| calculate_bclt | Reads first nobs even for nonpositive npsr. Per pulsar real tropo for selector 1; selector 0 writes first pulsar's observation, other selectors retain. Bypass freqSSB is MHz, roemer zero, direction normalized. Otherwise double geometry, binary128 epoch expression narrowed to double, PMRV only flag 1, parallax uses binary128 parameter then narrows. Starts dt_SSB=0; Shapiro then DM, Sun/Jupiter-only convergence sum with extra planet factor; veryFast==1 breaks before increment; deleted==1 or strict tolerance or 100 limit. | Two allocated pulsars and six observations each, including first-pulsar backing outside logical count. Nonfinite floating arithmetic allowed where no invalid integer conversion follows. |
| shapiro_delay | Normalizes direction before bypass; STL_BAT/bypass clears only Sun/Jupiter. STL_FBAT geometry omits Earth and clears five planets. Other planets already multiply planetShapiro; dt_SSB unused. Real precision no-op. | Full vector backing. Zero norms/radii/log boundaries are floating nonfinite observations, not finite golden values. |
| dm_delays | Bypass clears tdis1/2 only. Velocity FBAT requires all three flags exactly 1. Frequency in Hz, dilation predicates preserved. Eight derivative orders, first pulsar series selector; CM accumulator and powl use built-in long double. DMMODEL links via real accessor, first link warning selector, process-static t mask, real ifunc; otherwise strict open DMX and ordered -dm/-dmo. First-pulsar shapelet data but current event selector, BAT fallback only narrowed zero. Annual, chromatic, solar order unchanged. | All parameter and table backing initialized. First failed -dmo scan invalid; failed later scan after success retains initialized local. Nonpositive DMOFF count only where helper access is demonstrably safe. |
| formBats | Bypass writes BAT only. calcShapiro==-1 disables whole sum; all other values include ordered planetary terms. Two real TT sums; stored correction and BAT separately evaluated. bbat uses supplied shklovskii. | Initialized clock steps and delay fields; no replacement by SAT+stored correction. |
| secularMotion | Gates on first pulsar flags, computes current values/linked distance/epoch. L literals constructed as measured built-in long double before binary128 conversion; extended squared time. Zero branch and unconditional bbat. | Valid parameter backing and link target; no new distance convention. |
| refphs_init/clean | Init per-object flag; equality capacity exit; increments, zeroes full observation, supplies TZR fields and exact site rules. Clean gates on first object, full last observation copy to current tzrobs, decrement. | Real spare backing. Zero-count cleanup with active first flag is invalid/unrun. Repeated valid histories retain logical ownership and all typed tails. |
| solarWindModel | eclCoord exactly 0/1; integer date and seconds narrowing, real mcl2/rotation/resources. Wind 400, split <20 degrees, densities 2.5/10, actual ordered file reads. | Nonempty finite pairs, capacity below 500 including partial final scan. Required environment and bounded paths. Invalid eclCoord and empty sheet in this caller unrun. |
| mcl2 | 36-element arrays, source 5-degree stepping, one-sided cep clamp, approximate coefficients, truncating rotation arithmetic, last-cell tail fill, source exit. | Finite integer conversions, initialized direct output cells. Internal locals only observed after assignment. |
| readCurrentSheet | while(!feof), two binary32 scans, increments only for complete pair; partial longitude write survives beyond count; missing file exit. | Direct empty is defined. Capacity preflight includes attempted scan; non-consuming malformed input only isolated finite timeout, never a native exit. |
| findAngle/calcRotN/amod | Haversine exact a==1 branch; half-circle rotation selection and integer rounding; p==0 returns zero before quotient cast. | Bound all nonzero-divisor quotient casts and rotation casts; no generic modulo substitution. |
| elsun2/mjd2date/convertEcliptic | Float seconds cast, years 1..199 or exit; source calendar integer truncations/leap branches; source obliquity and longitude wrapping. | Valid integer operations. Seed direct outputs so defined retained prefixes can be reported. |
| outputResults | Exactly 36 rows, float intermediate conversion and one-step wrapping. | Full initialized arrays, retain raw stdout. |
| ifunc (both)/sinfunc | Endpoint holding, ordered interior scan, basis mask and assertion, sinc exact zero. | Valid array/backing and index ranges; negative basis k is defined all-zero mask; N<=0 value overload only before-first early return. Nonmonotone tables considered by actual read path; unsafe lookahead excluded. |
| shapelet helpers | Unsigned factorial loop; TNothpl always writes cells 0/1 even nonpositive n; scalar recurrence retains one seed at n=0/1; evaluate full coefficient/normalization/exponential order. | >=2 TNothpl output cells, initialized scalar seeds, bounded coefficients. UINT_MAX factorial nontermination excluded; zero width may yield nonfinite arithmetic. |
| recordPrecision | Actual empty active body, all typed state unchanged. | No invented precision record. |

## Build and observation boundary

Ordinary O2 serial profile compiles each original primary as a separate TU.
Real raw clkcorr, tropo, observatory, tabulatedfunction, tempo2Util, TKlog,
TKlongdouble and verbatim global data/CVS slices supply dependencies. Accepted
A2 minimal config hash is 470c61b68a61504558e76ef772639e46b565a1e4500115d6e3fb6a71286dd69e.
No archive, source initializer, stub or Rust implementation runs. Original U
wet mapping remains unchanged; W's correction is not silently incorporated.
Compiler/backend/linker/headers/runtime/macros and actual executable identities
are recorded and checked at launch. OpenMP source sites remain inventoried but
are not compiled or executed.

Caller pulsars are aggregate initialized and every observation field is typed-seeded
with distinguishable owner values before consumed fields/counts are set. They have separately allocated parameter
values/flags, observation backing and distinguishable typed retained tails.
Every S/R field is serialized by type, arrays by element type and run length;
pointers use registered logical owners; padding is never serialized. Floating
values retain finite bytes/signed zero; nonfinite classes are explicit. Full
field membership is retained, with incremental changed records between states.
Unbuffered diagnostic streams and ordered action markers are a declared harness
profile. Raw streams are saved; only exact harness source/resource roots may be
projected. Controlled geometry, TT and ephemeris values are supplied, not computed
by global wrappers. Composed calls do not establish formBatsAll or full physics.

The separate observer may report initialized iteration locals, static warning
mask, real TT calls and resource read order; it may not change math, random state,
files, diagnostics or source control flow. Its ordinary projection must match
all ordinary cases. Independent arithmetic checks support the oracle without
changing tolerance. Source mutations must change actual state/numbers/diagnostics,
not merely identity. Coverage retains every unhit compiler line/edge and profile
difference. Verifier corruption cases do not justify future duplicate kernel
guards when domain objects and dependencies already guarantee the invariant.

Cases are authored and frozen before oracle recording. Recording never overwrites
an existing oracle. Normal verification freshly builds and executes every case
twice, checks actual launch/input/resource identities and nonce, recursive types,
ordered membership, all states, statuses and streams exactly. Generated evidence,
fixtures, builds and archives stay local/ignored. No port, tolerance, precision,
physical convention or milestone acceptance is delegated.

Native logging inherits M's conservative host qualification for TKlog color stripping; solarWindModel's ld_printf inherits the accepted formatter ABI qualification. Defined-numerics labels never assert portable diagnostic/formatter behavior. No invalid backing or uninitialized automatic value is exported.
