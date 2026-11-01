# Fourth Linux corpus follow-up evidence

Central read-only planning note, 2 November 2026 (local). This continues the
independent review while AL implements decision 0117. It does not expand AL's
current production-edit scope, add an allowance, or accept a scientific gate.
The frozen evidence is run 41294531402 at `fecbc18`; no engine or compiler ran
for this note. The exact archive and its integrity review are recorded in 0117.

## Preprocessing is not one numerical discrepancy

`linux/corpus/preprocessing/differences.json` contains 160 recorded field
comparisons. Grouping their existing case tags gives:

| Case family | Recorded differing fields |
| --- | ---: |
| Unit transforms | 71 |
| Epoch transforms | 45 |
| SIMUL selection | 32 |
| Phase division by zero | 6 |
| Linked-parameter wrapper history | 4 |
| Stage-2 ordering case | 2 |

These are a work map, not demonstrated common causes or counts of distinct
physical errors. Arrays contain multiple scalar observations; repeated history
and whole-record comparisons also appear. The transport comparison is empty.

The 32 SIMUL rows comprise sixteen typed `deleted` comparisons and sixteen
whole-record consequences. The typed fields are native `0` versus Rust `1`
across eight cases, each observed twice in its history. Keep these discrete
selection changes separate from continuous-value and nonfinite comparisons.
The existing `p_precision_diagnosis` in `preprocessing_corpus_tests.rs` already
records the relevant source threshold investigation as unresolved; this review
does not rerun it or authorize rounding production to another precision.

The six `phase_divzero` rows cover three SAT values plus their whole records.
The remaining field groups require source/expression and class/sign analysis
before any conclusion about rounding bounds. Do not apply 0073's limited K
allowances to these P fields. Decisions 0012/0049 and the fixed Rug128 policy
remain unchanged; neither a small difference nor a case-family label establishes
acceptable scientific equivalence.

## Planetary interpolation lead

The planetary ledger has three rows, all repeating
`c076_absent_nutation` observation 0 `earth_ssb[1]`. The values are native
`401437be1a8e5718` versus Rust `401437be1a8e54bc`, a difference of 604 binary64
ULPs at that magnitude. These rows occur at event 12 end and event 15 begin/end.
The fixture name alone does not establish that missing nutation caused the error.

The case applies a Y-offset interpolation, with the first segment
`(50000,3)` to `(50020,7)` and SAT exactly `50010.25`. Pinned
`readEphemeris.C:432-447` uses separate source expressions for the intercept
and evaluation. Frozen `rust/src/read_ephemeris.rs:284-288` explicitly fuses
those operations to reproduce the earlier measured macOS GCC16 profile.

A binary64 scalar counterfactual using the exact case operands gives:

| Value | Hexadecimal binary64 |
| --- | --- |
| Slope | `0x1.999999999999ap-3` |
| Intercept in both calculations | `-0x1.3868000000000p+13` |
| Separate evaluation | `0x1.4333333333800p+2` |
| Fused evaluation | `0x1.43333333335a4p+2` |
| Separate minus fused | `0x1.2e00000000000p-41` |

The last value equals the observed native-minus-Rust vector shift exactly.
This numerically explains the shift at the scalar level. It is not an executed
Rust correction or a new full native/Rust comparison. The actual Linux
`readEphemeris` object/profile and neutral before/after engine evidence are still
required before correcting or accepting this source group. Do not infer the
cause of its separate random-number failures or other propagation differences
from this witness. Retain the accepted macOS behavior and source-site comments.

## Reproduction and next use

`central-fourth-corpus-triage.py FRESH_OUTPUT_DIRECTORY` reads only the verified
ZIP and pinned Git blobs. It binds the case/difference members, fixed probe SAT,
source interpolation expressions and current Rust contractions. It uses ordinary
Python binary64 operations and `math.fma` for this bounded scalar counterfactual;
no production setting, tolerance or backend changes. The executed receipt is
`8c211055ea591e0ea6c7baefaaad2021482c3807e2b380c34be454178453a664`.
The exact recipe and receipt are retained under ignored
`build/fourth-corpus-triage-20261102/`; generated JSON remains out of Git.

Use this classification when defining the next source-profile batch after the
current 0117 work. Preserve all failures. The four discrete utility mismatches,
M2 raw SVD ordering, preprocessing selection and continuous/nonfinite failures
remain distinct obligations. This note makes no new numerical-policy decision.
