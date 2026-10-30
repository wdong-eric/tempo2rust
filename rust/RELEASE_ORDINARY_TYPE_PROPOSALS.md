# Remaining ordinary-type sites: proposal for central review

Decision 0104 authorizes the demonstrated EXP/GAUS and Lomb corrections. The
sites below remain proposals, with no production edits. None is closed by the
event result. Native timing remains binary128 and Rust timing remains Rug128.

## Spin callbacks and L-suffixed scale constants

The second Linux M2 receipt measures two differences: `columns[1]` (F0),
absolute difference `1.164153218269348144533750e-10`, and `columns[4]` (F3),
`1.323488980084844279796776e-23`, in the source callback coordinates. The
fixture uses double `x=-1234.56789` days and wide
`F0=123.45678901234567890123456789`. These are measured discrepancies, not yet
a type-only causal experiment. The other twelve M2 failures have separate causes
or observation contracts; correcting NaN signs does not accept these two values.

Source `t2fit_stdFitFuncs.C:25-95` assigns `t2`, `arg3`, `arg4`, `f0`, `f1`,
and `bindex` to ordinary long double. L-suffixed literals make the F0–F4
expressions ordinary arithmetic; `v` is double and narrows on assignment.
For F5 and above, the reciprocal-factor prefix is ordinary, but `powl` is
`powq`, so multiplication/division after that boundary are timing-wide. The
fallback also promotes `x/(longdouble)k` to wide. Brake corrections mix these
roles, including compound assignment back into double `v`. The source's zero
`bindex` and `84000.L` factor must be retained, not silently repaired.

Both `std_freq.rs` (bounded F0–F3) and `t2fit_stdFitFuncs.rs` (general dispatch)
currently use f64 ordinary locals. The bounded caller even narrows F0 before
entering `std_freq`. Recommended correction, subject to central approval: feed
the canonical parameter store into one source-ordered arithmetic path shared by
both callers, round each ordinary operation, and preserve each double assignment
and wide promotion. Do not change the fit-coordinate scaling or comparison gate.
Audit `update_spin` in the same component: source scale is wide **after** parsing
`1.0e23L` at ordinary precision; the present f64 literal loses that distinction
on Linux. `pow(24.0*3600.0,k+1)` stays double.

Evidence required: an observer-neutral native temporary/return trace for the
two M2 rows; an ordinary-to-double temporary counterfactual; F0–F13, fallback and
brake-set/unset neighbors including signed x; direct checks that bounded/general
callbacks agree on canonical inputs; and update-scale observations at k=9/10.
Then rerun M2 and affected spin fits on both hosts. Preserve raw SVD-order rows.

## Chromatic accumulation

`dm_delays.C:139-152` initializes ordinary `cmval` from wide CM0. Every
polynomial contribution is deliberately cast to **double**, then promoted into
the ordinary accumulation. `:286` is a different boundary: CHROMX is wide, so
compound addition promotes `cmval`, adds in wide arithmetic, then narrows back
to ordinary. `:291` multiplies ordinary cmval by wide `powl`, adds to the double
delay in wide arithmetic, and assigns back to double. The existing f64 `cm`
in `propagation.rs` collapses both accumulation boundaries on Linux.

Recommended bounded repair after evidence: preserve each explicit double term,
restore only the ordinary accumulator and its distinct CHROMX assignment path,
then keep the current wide power and final double delay. Trace CM0, every term,
CHROMX membership/update and final delay on the measured `chrom-poly` failure;
include ordinary/Taylor series and overlapping CHROMX neighbors. Prove observer
neutrality and type-only causality before attributing that failure to this site.

## Cosmic-string time offset

`formResiduals.C:1456,1538` assigns ordinary `dt` from the wide expression
`(sat-gwcs_epoch)*86400.0`; width and several geometry/extra values stay double.
`fabs(dt)`, powers with a double exponent, and `dt+0.5*width` therefore require
actual overload/type probes, including the macro/header context. The current
`residual_source.rs` narrows dt to f64. Do not mechanically widen the other
geometry or width variables.

Recommended evidence: source expression-type assertions and neutral dt/interior
term traces across both half-width boundaries, the epoch and the two exterior
branches. A dt-only counterfactual must test the retained cosmic-string failure
identities. Preserve source comparisons, double `extra1/extra2` assignments and
wide phase accumulation. Implement only after central reviews that attribution.

## Secular-motion constants

`secularMotion.C:40-41` initializes wide constants from the ordinary literals
`3.08568025e19L` and `1.536281850e-16L`. `propagation.rs` currently promotes their
f64 approximations. Proposed correction: parse each exact decimal at the
qualified host ordinary precision, then promote to Rug128; all later timing
arithmetic and the first-pulsar enable policy remain unchanged.

Measure both compiler literal values and promoted wide values on each host,
then compare Shklovskii delay and BBAT for a nonzero DSHK/proper-motion fixture
with the individual constant substitutions. This is a source-type obligation;
no current failed gate has yet been causally assigned to it. CALCEPH's existing
ordinary-JD conversion already uses the predictor rounding path and is not a
missing implementation.
