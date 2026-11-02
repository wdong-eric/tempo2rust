# Paired SVD acceptance delivery (0120)

2 November 2026. The user-approved position-only policy is implemented in the
current M2, general-fit and standalone/robust-inner comparators. Production
Rug128, solver arithmetic, raw arrays, fixture inputs and old receipts are
unchanged. Central retains milestone acceptance.

M2 reuses its existing stable descending mode pairs, with each singular value
and retained flag carried together. Former exact-position failures and their
raw spectra/flags remain in `mode_permutations` diagnostics. General-fit and
standalone/robust comparisons retain their **numerical checks on discarded
values**, including their original relative gates and exact structural zeros.
All paths retain cardinality, rank, matched flags and source-cutoff checks;
physical columns, observations, increments, covariance and updates are not
permuted. Nonfinite spectra do not enter finite-mode sorting. A robust native
trace without exported cutoff flags cannot establish a nonfinite cutoff/rank
contract and is explicitly rejected instead of guessed.

The M2 release recorder now shares the same comparison function with read-only
saved-output replay and labels the adopted policy. Direct standalone and legacy
receipts also expose mappings and original position diagnostics. The generic
QR/pivot and SVD API U/V checks are unchanged; no basis-rotation allowance or
matching framework was added.

## Saved evidence replay

The ignored `validation/release/build/paired-0120/` packet contains the replay
recipe, pre-0120 comparator snapshots, before/after reports and exact input
hashes. Every original input/output/receipt and archive hash was checked again
after replay. No producer or production executable ran for these comparisons.
The collector omits authored PAR files; replay reconstructs their exact embedded
Rust input bytes and verifies both their recorded input hash and the original
candidate export manifest. Two failed preparation attempts are retained.

| Complete M2 record set | Pre-0120 failures | 0120 failures | Raw-position diagnostics retained |
| --- | ---: | ---: | ---: |
| macOS retained 0117 campaign, 50 records | 2 | 0 | 2 |
| Linux run 41312212554, 50 records | 2 | 0 | 2 |
| Earlier Linux run 41294531402, 50 records | 4 | 2 | 2 |

The earlier Linux run still fails two spin-callback values; the comparator does
not relabel those historical engine outputs. The later focused run contains the
separately reviewed spin correction. The original failed receipts stay failed
under their original contract.

145 saved general-fit records were inspected: 72 macOS, 72 from the complete
fourth Linux run and one focused Linux spin record. Only **macOS public/red**
changes from failure to pass solely through pairing. After pairing, macOS has
56 passing, eleven still failing and five unusable/empty saved candidate records;
the fourth Linux set has 57 passing, ten failing and five unusable records. The
focused Linux spin record passes. These are retained snapshot results, not new
family qualification. The detailed receipt identifies the surviving scientific,
physical, class and value failures; discarded-value failures are not exempted.

## Focused verification

- All 70 M2 controls pass, including two positive permutations; changed values,
  cardinality, rank, flags, value/flag mismatch, cutoff equality, nonfinite classes,
  physical covariance/parameter/observation ordering remain rejection controls.
- General-fit controls: spin has four positive permutations and 70 rejections;
  robust-H has five positives and 90 rejections. Added controls include discarded
  values, zero/equality classification, paired flags and nonfinite sign matching
  before any finite-mode sort. An initial robust control edited the first of
  repeated native SVD records instead of the final exported value; it was
  corrected to update every same-key record in that synthetic control only.
- Standalone and robust-inner saved solver controls: sixteen positives and forty
  rejections, including all-zero spectra, threshold equality, discarded-value
  changes, missing/extra modes and unchanged physical-coordinate checks.
- Release harness suite: 35 passed, six data-dependent tests skipped. No skipped
  test is reported as passed. The Q-specific controls are separately documented.

The current comparison source hashes, original artifacts and all mappings are
bound in `validation/release/build/paired-0120/receipt.json`. General controls are
under `/private/tmp/al120-general-controls-v2/spin/` and
`/private/tmp/al120-general-controls-v3/robust-H/`; standalone controls and their
recipe are retained in the ignored packet. The 70-control M2 result is
`/private/tmp/al120-m2-controls.json`.

No full W, producer campaign or full Linux matrix was rerun for this policy-only
change. Unrelated physical failures, the internal event-sign investigation and
complete integrated qualification remain open.
