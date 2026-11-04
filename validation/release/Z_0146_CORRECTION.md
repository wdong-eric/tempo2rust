# Z Gaussian radius correction — 4 November 2026

Decision [0146](../../decisions/0146-linux-wiring-review-and-gaussian-radius-correction.md)
releases one operation-order correction in `RandomState::gaussian`, following
the saved Linux diagnostic. The existing polar Gaussian generator is required
for compatibility with native seed, shuffle-table, rejection and cache
sequences; replacing it with another generator would change that contract.

## Source and evidence

The only production change is `rsq` in
[`read_ephemeris.rs`](../../rust/src/read_ephemeris.rs). Linux x86_64 now uses
`v1 * v1 + v2 * v2`, matching the separate binary64 operations in the actual
ordinary GCC 13.3.0 `readEphemeris.C::gasdev` object. Other targets retain the
existing fused expression, including the qualified macOS arm64 profile.
Affine transforms, random draws, extended table arithmetic, seeds, rejection,
cache, `ln`, `sqrt` and final products are unchanged. No comparator, tolerance
or numerical policy changed.

Central's diagnostic is retained at
`decisions/evidence/release-qualification/build/z-gaussian-20261104/review.json`,
SHA-256 `f9e41dc9e82b42dba75e08302b5f026a051423d5bce3e7e5470f8ad76d8f2c2a`.
It binds the native pin `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, original
Linux observer and ordinary object. Separate operations reproduce all 84
saved radii; fusion changes seven radii without changing recorded rejection
decisions. Four constructed pairs and their cached calls account for all
sixteen saved result/cache discrepancies. These are accepted saved-data
findings, not execution of corrected Linux Rust.

The new ordinary unit test replays the four affected seed/call histories:
`(-12345, 7)`, `(0, 19)`, `(1, 11)` and `(1, 19)`. Each expected result
distinguishes the two radius roundings. Linux expectations are bound to the
diagnostic, and macOS expectations to the original native observer. The next
cached call must return the exact cached bits, clear the availability flag,
and leave the seed, indices, initialization flag, extended result and complete
shuffle table unchanged. No production observer or numeric abstraction was
added for the test.

## Verification

Local verification is recorded in the fresh evidence directory
`decisions/evidence/release-qualification/build/z-0146-20261104/`.
The retained `verify.py` uses the existing `Run` command recorder and original
`check_random.py`; only the latter's output directory is redirected to preserve
older results. The source, diagnostic, original observer, command logs and
actual test executable identities are bound in `checks/receipt.json`.

Both macOS profiles pass two focused tests: the new four-history/cache
regression and the existing `c076` telescope-offset witness. The external
history test is ignored in this ordinary selection, then explicitly executed
by the existing checker in each profile. All fifteen complete original
macOS random histories pass in both debug and release: 934 events, including
107 Gaussian calls, and zero differences per profile. The original observer
SHA-256 is `76d88c41746be3a86734700658b484f9f1e464853e30930b09a8f2050479acd7`,
verified against its saved audit before and after execution.

Four dispatch controls execute the actual corpus dispatch loop with only its
process boundary replaced: debug/release, each with a passing or failing
planetary driver. All four retain the complete independent random-history
command. These controls establish wiring, not Linux scientific execution.
The verification receipt SHA-256 is
`75d280d17d3e03dc7d41089b40544356a2b8a1159719095ebb24b878a65cabc6`.
`delivery.json` binds the committed sources, retained executable copies,
verification recipe, logs and results.

## Integrated Linux obligation

The existing Ubuntu x86_64 workflow still runs `integrated.py`, whose
`corpora-comparison` stage invokes `corpus_compare.py` with the fresh native
corpora. Its default release planetary stage calls the unchanged
`check_random.py` independently of the preceding driver result. This selects
all fifteen histories and explicitly executes `z_random_histories` with
`--ignored --exact`; every result, seed, RNG state/table projection and cached
value remains exact. The standard debug/release Rust suites also include the
new non-ignored unit test. No wiring edit was needed.

Corrected Linux execution and closure of the original sixteen discrepancies
remain pending. This work does not establish general cross-platform libm
equivalence or close another scientific, GUI or performance gate. GUI/event
and AO predictor sources were not edited. No push or workflow dispatch occurred.
