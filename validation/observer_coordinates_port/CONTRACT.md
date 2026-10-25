# AA frozen implementation contract

Release 4498cf1c4bb9167a57e4cb3b1226cbd73e3ee2d3; accepted Y 21dc956,
production 11165d1; X 02234cd, mapping a59be00, harness 6d19dd4.
Decision 0066 releases all of 0062. Native pin fda6ad4. Rug128 stays fixed.

The local build/freeze.json records the complete accepted X routine bodies,
lexical branch sites, caller occurrences, 349/95 field types, SOFA closure, actual
accepted Y Rust blobs and live consumers before implementation. X CONTRACT.md
and 0062 supply the per-branch scientific contract; no new reference phase.

| Source routine | Rust destination and live callers | Reads / writes / lifetime |
| --- | --- | --- |
| vectorPulsar | vector_pulsar; canonical driver; form_bats fixed-direction adapter | RAJ/DECJ narrowed before trig; PM/PM2 extended; four 3-vectors; no observation owner needed |
| get_obsCoord | get_obs_coord; canonical driver | ordered pulsars/observations; delay bypass; exact satellite names; independent polynomial flags then repeated flags then axis offsets; STL_BAT writes correctTroposphere; terrestrial aliases/IMAG mutate shared xyz only; retains all tails |
| get_obsCoord_IAU2000B | get_obs_coord; full driver and form_bats | TT split before EOP, UT1 after EOP; shared SOFA closure; independent 3/6/9-element buffers; initialized, nonescaping FFI borrows |
| lmst | get_obs_coord; legacy driver and callable helper | bounded native int/subtraction; source double expressions then public Rug128 fortran_mod; two outputs |
| get_precessionMatrix | get_obs_coord; legacy driver | supplied nutations; source multiplication and transpose order; nine writes |
| ang | get_obs_coord; callable helper | floor-based integer components; exact selectors; no state |
| remove_white | get_obs_coord; callable helper (no pinned core caller) | first NUL, source branch order, whole buffer/tails; three overlapping strcpy sites corrected with copy_within |
| tai2ut1 | tai2ut1; canonical driver | method exactly 2; clockCorr nonzero uses SAT then real TAI correction; otherwise zero; other methods retain |
| ut1red | tai2ut1; tai2ut1 and callable helper | independent ClockSession UT1 cache: read, dates, 1000 MJD, 10000 entries, counts; partial scan writes, zero sentinel, row-limit prefix, strict interior and negative result |

Ownership: existing ClockSession mutex acquired once outside nested work. UT1
errors do not poison unrelated resource operations. Failed loads keep inspectable
prefixes but never publish read=1; continuation beyond native exit is a safe
boundary. Canonical PulsarContext split borrow becomes mutable in both halves;
existing validated ownership checks remain. Signed empty loops need no rows.
ObservatoryDatabase.index supplies authoritative identity; RuntimeContext loses
its geometry snapshot. Geodetic values remain cached after IMAG xyz mutation.

Original comparison membership is all 133 X histories, 1734 actions, 3462 events,
127 normal and six exit-prefix cases. Recursive ordered types, initialized cells,
finite bits, signed zeros, nonfinite classes, parameter/owner/tail retention and
raw diagnostics stay exact. Extended values export to binary128 only for evidence.
The separate X UT1 observer supplies full private cells and exit state. No original
case is silently omitted or rewritten. Precision differences require an explicit
unresolved disposition, never tolerance or backend changes.

Correction membership is the existing central whitespace experiment: seven common
original no-shift buffers and nineteen corrected complete buffers. Overlapping
original operations have no fabricated golden. Invalid casts, backing, strings,
empty UT1 and invalid brackets return contextual boundaries. Historical CVS,
alias/concurrency/platform gaps remain unless new evidence closes them.

Completion: all nine routines, one shared frontend mathematical implementation,
executable terrestrial/satellite/vector/UT1 examples, debug/release X comparisons,
retained-state/type/ownership and corrected-buffer controls, affected regressions,
identity-bearing report. Public grammar stays isolated F0-F3 @/PKS; general
PAR/TIM, planetary/propagation and full BAT orchestration remain later integration.
Four M2 raw SVD-order and four 0049 precision boundaries remain unresolved.
