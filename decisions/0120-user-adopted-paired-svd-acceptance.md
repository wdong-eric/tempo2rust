# User-adopted paired SVD acceptance

Central decision, 2 November 2026. The user answered **“Adopt paired-mode
acceptance; retain Rug128 (Recommended)”** to the concrete 0119 proposal after
review of the precision-sensitive QR branch. This supersedes 0012's requirement
that corresponding singular modes occupy the same raw array positions. It does
not accept M2 or any broader fitting/migration milestone before verification.

## Adopted contract

Keep Rug128, the source algorithm and operation order, raw U/W/V arrays, retention
flags and all production outputs unchanged. No sorting, precision switch,
alternative solver or artificial tie-break is introduced in production.

For acceptance comparison only, match modes one-to-one by singular value while
carrying each mode's original index and retained/dropped flag together. Reuse
0010's stable descending paired-record comparison and expose the resulting
reference-to-candidate index mapping. Require exact cardinality, effective rank
and matched retention classification. Validate each engine's flags against its
own unrounded spectrum and the unchanged source cutoff, including original zero,
all-zero and nonzero threshold-equality cases. A mismatch cannot be hidden by
sorting values independently of their flags.

Attach every existing numerical gate to the corresponding matched mode. The
M2 paired comparison already treats discarded modes through their own source
cutoff/zero classification and applies its existing relative gate to retained
values; retain that established rule. Where general/robust fitting currently
checks numerical values of discarded modes, retain those checks at the matched
positions. Changing mode position does not authorize a new exemption for tiny
values, different cutoffs, nonfinite classes or basis rotations. Preserve exact
reference/candidate outcome classification before attempting finite-mode pairing.

Physical parameter/observation ordering, design and whitened matrices, increments,
updates, errors, covariance coordinates, rank, DOF, chi-square, error scaling,
modeled effects, residuals, pulse counts and every other existing gate remain
unchanged. A pure mode permutation may pass only if those independent gates also
pass. Do not use mode matching to permute parameter columns or observation rows.

Keep both raw arrays, their original cutoff flags, the comparison permutation and
the original exact-position discrepancy visible in diagnostics. Under this new
contract, a qualified pure permutation is diagnostic information rather than a
failing acceptance gate. Every original pre-0120 failed run remains unchanged and
identified under its former rule; no historical receipt is relabelled as passed.

## Scoped implementation release

AL may implement this comparison-policy change alongside the already released
0119 Q/event work, as a separate Conventional Commit. This is an explicit narrow
exception to the previous historical-comparator freeze: current
`validation/fitting/compare.py`, its focused comparator tests and the directly
dependent release recorder may change. Native producers, fixture inputs,
scientific outputs and historical receipts remain read-only.

Inspect the current general-fitting and robust-inner-solve raw-mode comparisons
as well. Apply the same position-only rule where they compare SVD modes, preserving
their own existing value gates and all physical-coordinate comparisons. Reuse
existing comparison logic and tests; do not introduce a general matching framework
or reinterpret QR pivots, model parameters, noise components or other indexed
objects as interchangeable singular modes. Update only directly affected current
validation/reporting paths and record their exact ownership/identity.

Verification must include positive pure permutations with their mappings and
rejections for changed retained values, missing/extra modes, wrong rank, wrong
retention, flag/value mismatch, threshold-equality errors, nonfinite changes and
physical-parameter/covariance/observation permutations. Retain the complete
existing rejection suite and add only controls needed to protect the changed
contract. Raw production artifacts must remain byte-identical across a
comparison-only change.

Re-evaluate the retained complete M2 macOS and Linux comparisons with the new
versioned comparator and exact input identities, without overwriting old results.
Use existing saved general-fitting records to distinguish pure permutations from
still-failing value/physical checks; do not declare a whole family fixed from one
fixture. New producer execution is required only when production changes or an
unresolved check makes it necessary. The already planned full Linux matrix must
run the adopted comparator and preserve all remaining failures.

Return the concrete comparator/report commit and focused control evidence for
central review. Central retains milestone acceptance and publication/dispatch.
AL's Q/event work continues; the source and mathematical causal explanation from
0119 remains scoped to its measured fixture. No covariance optimization or other
precision/tolerance policy change is released.
