# AB source and dependency map

Raw source `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; release `b8d6263`. The local frozen
`outputs/frozen/source_map.json` contains full bodies, field/type/extent metadata,
lexical decisions, all preprocessor sites, and raw repository caller occurrences.
The table below is a concise index, not a substitute for compiler coverage.

| File | Routine | Raw lines | Lexical decision sites |
| --- | --- | --- | ---: |
| calculate_bclt.C | calculate_bclt | 41–182 | 23 |
| shapiro_delay.C | shapiro_delay | 38–138 | 15 |
| dm_delays.C | dm_delays | 41–349 | 52 |
| formBats.C | formBats | 36–88 | 7 |
| secularMotion.C | secularMotion | 36–67 | 5 |
| refphs.C | refphs_init | 6–45 | 8 |
| refphs.C | refphs_clean | 47–57 | 3 |
| sw_delay.C | solarWindModel | 62–177 | 8 |
| sw_delay.C | findAngle | 182–197 | 2 |
| sw_delay.C | readCurrentSheet | 199–215 | 3 |
| sw_delay.C | outputResults | 217–231 | 3 |
| sw_delay.C | mcl2 | 260–486 | 19 |
| sw_delay.C | amod | 488–494 | 1 |
| sw_delay.C | elsun2 | 498–541 | 1 |
| sw_delay.C | calcRotN | 544–560 | 3 |
| sw_delay.C | mjd2date | 562–606 | 29 |
| sw_delay.C | convertEcliptic | 609–627 | 2 |
| ifunc.C | ifunc | 5–21 | 3 |
| ifunc.C | ifunc | 33–56 | 5 |
| ifunc.C | sinfunc | 23–31 | 2 |
| shapelet.C | iter_factorial | 8–14 | 1 |
| shapelet.C | TNothpl | 21–39 | 1 |
| shapelet.C | iterative_TNothpl | 45–65 | 3 |
| shapelet.C | evaluateShapelet | 68–80 | 1 |
| storePrecision.C | recordPrecision | 39–46 | 0 |

## Typed state and logical owners

The caller exports all 349 pulsar fields and all 95 observation fields recursively,
including nested clock/fit/precision records. It has two owners with six allocated
observations each. All 2,000 parameter slots retain all 13 typed members; identical
adjacent parameter records and null pointer arrays use lossless run-length encoding.
The 26 consumed parameter slots per pulsar have sixteen initialized value/flag cells.
Other pointers are explicit nulls; no absent backing is dereferenced. Every observation
field and fixed array is typed-seeded before consumed values/counts are supplied.
The field map is the accepted S map cross-checked with M/R, not a new ABI assumption.

Initial states retain all ordered field records; each subsequent state stores changed
records and the same total membership count. Expanding changes reconstructs the full
typed state at every action. Byte runs encode primitive units only; no struct padding,
process addresses or uninitialized automatic locals enter the evidence.

## Actual dependency closure

Primary files compile separately with public raw headers. The link explicitly includes
`clkcorr.C`, `tropo.C`, `observatory.C`, `tabulatedfunction.C`, `tempo2Util.C`,
`TKlog.C`, `TKlongdouble.C`, and verbatim `global.C` data/CVS slices. No archive is
linked. The build records every included header, backend, assembler, linker and runtime
file identity available outside the macOS shared cache. Coverage records executed
dependency functions. Clock corrections and geometry are supplied caller state;
the real TT sum, parameter links, troposphere and solar resources are still executed.

Accepted A2 contributes only its three-line config. Original U troposphere arithmetic
is used. W bd290cc is fixed Rust context only. M qualifies native formatter/logging
diagnostics; R/S supply field, count, knownness and ownership conventions.

## Source callers and lifecycle

`global.C` supplies the broader ordering boundary; the authored composed case performs
`refphs_init → supplied state → calculate_bclt → formBats → secularMotion → refphs_clean`,
with a second arithmetic pass before cleanup. It does not execute global wrappers.
`calculate_bclt` invokes real troposphere, TT, Shapiro and dispersion; `dm_delays` invokes
real linked values, ifunc, shapelets and solarWindModel. Solar-wind helpers are also
called directly. The frozen caller map distinguishes textual occurrences from runtime
coverage; compiler line/edge results and unhit dispositions live in local coverage data.

`field_map.py` produces `outputs/field_map.json`, cross-referencing active member
uses and parameter indices with complete accepted types/extents. It filters
lexical member tokens through actual declarations; the original frozen raw
source remains available for caller/index interpretation and whole-record writes.

## Frozen execution corpora

`fixtures.py` authors 311 primary cases. `supplemental.py` authors 13 additional
coverage-directed cases and records them in a separate immutable original-source
oracle. `combined.py` checks exact membership before presenting both corpora to
the observer, structural checks and coverage runner. The original 311-case oracle
and its source/harness manifest remain unchanged.

`outputs/frozen/source_map.json` is the pre-execution raw inventory;
`outputs/field_map.json` adds active field/type/parameter cross-references and the
measured native precision trace. `GAPS.md` is the human-readable safety and
coverage disposition. All generated maps and execution data stay local under
the directory's ignore rules.
