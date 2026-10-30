# Second Linux result and source-profile corrections

Central decision, 31 October 2026 (the run and review timestamps below are UTC on
30 October). **AL, M2 and full migration acceptance remain pending.** Continue AL
under the existing 0101 publication/fix/rerun authorization and 0099 source
policy. AL remains the sole implementation writer. The user-selected AL-first
sequence is unchanged; covariance profiling remains deferred.

## Actual execution and independently verified evidence

[Run 41056090173](https://github.com/wdong-eric/tempo2rust/actions/runs/41056090173)
executed candidate `4accd26e2d51b3dc805d9a1ae0a2be7195e171ea`. Linux job
`124063879789` ran from 12:32:24 to 13:08:47 UTC. Setup, collection and upload
passed; the matrix failed. All 66 top-level stages ran: **54 passed, twelve failed,
none blocked**. This does not mean every nested corpus executed.

Central checked the GitHub artifact identity/digest and every selected member:
**11,242 files / 617,149,255 bytes**. The exact authored offline review is
`decisions/evidence/release-qualification/central-second-linux-review.py`; the
ignored packet is `decisions/evidence/release-qualification/build/linux-run-41056090173-review-20261030/`.

| Evidence | Identity |
| --- | --- |
| Artifact | `12979305748`, 86,697,688 compressed bytes |
| Archive SHA-256 | `8b2c0beeadf1779d0ac99572be1767cf14f98b9ab7a6aeb6850327f046e9e018` |
| Selected manifest SHA-256 | `8fd1e3403a06d1b4fbc25adddb9e287fb0b8bca3bc74ebb44b4ecff009c9568a` |
| Central review SHA-256 | `f782f1de5cbe9ac4234fc27096bd442e24071fbe438ea6b1dabf1af01568a378` |

The background collector's optional job-log call used the gh flag at the wrong
command level. Its failure is retained. The separately corrected `gh api
--allow-escape-sequences` retrieval succeeded into `job.complete.raw.log`, whose
hash is bound by the central review. A live-log request returned 404 before job
completion. Neither observation failure restarted qualification.

The provider ABI and both installed predictor C profiles now pass: 64 symbols,
four C11/C++ static/shared combinations and twelve rejection controls per
profile. The actual ordinary type has 64 significand bits and 16-byte size;
Fortran/BLAS integers are 32-bit, OpenBLAS is 0.3.26 LP64 and CALCEPH is 4.0.5.
These interface checks do not close the separate concurrency or generation gates.

Both Rust profiles pass 111 ordinary tests plus nine explicitly enabled runtime
tests; 36 ignored tests remain identified per profile. Formatting and Clippy
pass. The complete worker campaign passes 99 processes, 33 comparisons and
484,314 quantity checks; 108 timing samples and twelve native product controls
are retained. Package audit passes 2,494 checks. Source audit records 153
unchanged Rust files and the two reviewed test-module changes, with no added or
removed files. Production scientific bodies are unchanged.

## Demonstrated platform arithmetic corrections

**Lomb:** the independent copied implementation equals the installed API. On
Linux, replacing its nine explicit fused operations by separate products/sums
passes all 102 quantities against each native default/off/fast profile. Maximum
dimensionless difference is 4e-17. The original contraction gate remains failed.
Together with the earlier macOS native/counterfactual evidence, this supports
restoring the measured source execution profile: retain the existing macOS
arm64 fused sites and use the native Linux x86_64 separate operations. AL may
implement this scoped correction with adjacent source/compiler/profile comments
and focused controls. Preserve every bin, expression grouping and existing gate;
unsupported compiler/target profiles are not silently qualified.

**EXP/GAUS:** the observer is neutral in both mean modes. The original Linux
maximum residual difference remains 9.94614202194096e-10 s. Changing only native
ordinary-long-double event temporaries to double makes the comparison pass with
maximum residual difference 6.0076138228887974e-27 s. The events-removed controls
remain exact. This establishes the temporary-type block as causal for this
fixture; the double-native counterfactual is not the new oracle.

Central approves restoring the source's host ordinary type in EXP/GAUS residuals
and their connected fitting callbacks: 53 significand bits on macOS arm64 and 64
on Linux x86_64, with source rounding/conversion/operation boundaries preserved.
**The existing timing mapping remains native binary128 versus Rust Rug128.**
Frequency conventions, scientific formulas, thresholds and output conventions
are unchanged. In particular, retain the 0063 barycentric frequency behavior
that amplifies this fixture, and retain the source Gaussian derivative signs and
6. denominator. Do not substitute newly derived derivatives.

The detailed contract and connected obligations are in
`decisions/evidence/release-qualification/event-type-boundaries.md`, including
AL's completed read-only proposal. The earlier hold on implementation pending
causal evidence is satisfied by this run. AL may implement the source-preserving
repair while obtaining the remaining focused evidence; qualification still
requires the actual Linux log/expl/assignment-return type probe, exponent/results
and Gaussian-val observations, all eight callbacks and a Gaussian-only witness
that exposes ordinary-width narrowing. The existing fixture alone does not
adequately test its Gaussian branch. Keep the original event gates and
amplitude-fit case, exact macOS before/after event products and relevant predictor
rounding controls. A shared private rounding kernel may reuse the existing
host-width/subnormal/exponent handling; keep predictor-specific formatting,
quad_cos and contraction semantics out of residual expressions. Measure Linux
transcendental correspondence; equal declared precision alone is insufficient.

Other ordinary-type assumptions in chromatic accumulation, cosmic-string dt,
spin callbacks/L literals and secular constants remain explicit obligations.
Two direct M2 spin callback differences are now measured. Return a per-site
source/evidence proposal for their correction; do not mechanically widen every
f64 or imply the event repair completes the ordinary-type review. CALCEPH's
existing ordinary-JD conversion is already implemented.

## Validation and execution corrections

1. **Native history staging and failure publication.** No early native corpus
   executed. The third historical fetch failed with `fatal: shallow file has
   changed since we read it` for `7946aeee...`; the first two fetches succeeded.
   This is not evidence that the commit is absent. Diagnose/reproduce the actual
   Git staging failure and correct it without a broad history fetch or silently
   discarding the failed attempt. `corpus_compare.py` then raised `KeyError:
   'stages'` on the failed preparation receipt. Preserve that receipt and publish
   explicit unavailable prerequisites for every affected comparison instead of
   crashing. Exercise the real shallow/history construction and failure/recovery
   path before the next full Linux replay. The closure stage's successful exit
   includes zero native-corpus records and does not close this requirement.

2. **M2 NaN observation.** The now-executed M2 comparison reports fourteen
   failures across fifty records: two retained SVD-order differences, ten NaN
   sign differences and two spin callback values. The historical exporter at
   `validation/fitting/export_reference.cc:20` writes every NaN as positive NaN;
   the actual Linux platform witness reports a negative runtime sqrt(0/0) NaN.
   Correct an AL-owned successor observer to preserve the sign actually present,
   retaining the historical exporter and raw result. Prove observer neutrality
   for other values and preserve exact signed-class controls. This authorizes an
   observation correction, not NaN canonicalization or acceptance of the other
   four differences.

3. **Polyco serialization invariant.** Predictor generation/exchange executes
   fourteen cases / 37,310 checks and reports 178 failures: 168 coefficient
   round-trip comparisons and ten raw log10RMS comparisons. The coefficient check
   requires equality to the pre-write binary value. Source writing uses eighteen
   significant decimal digits; the actual Linux native witness demonstrates
   that reading those digits into 64-significand-bit long double need not recover
   the pre-write value. Replace this false lossless invariant in an AL-owned
   successor with the exact source write/read invariant, respecting each writer's
   argument type, decimal format and parser precision. Validate tokens and
   decoded values, including a native-written witness and rejection controls;
   keep pre-serialization construction/evaluation gates. This is an explicit
   central correction of the observation contract, not a new numerical tolerance.
   Preserve the old 168 failures and all ten raw RMS failures separately until
   the corrected evidence establishes its precise result.

4. **Cache diagnostic completeness and ownership.** Both original ASan callers
   retain the 544-byte/16-allocation leak. Explicit local-cache cleanup exits zero
   and matches stored TSV products, but the original captured stdout has twelve
   BEGIN markers and only eleven END markers (143,360 bytes, ending mid-line).
   The cleanup variant has twelve complete blocks; its common completed blocks
   match the original. Thus the intended full-output control is not satisfied.
   Investigate/preserve the incomplete capture and flush the common caller's
   evidence before shutdown in both diagnostic variants, without suppressing
   LeakSanitizer or changing library calculations. Require twelve complete,
   matching workflows and all storage products before claiming full attribution.
   Prepare a library-owned, thread-local cache-lifetime correction using the
   documented safe Rug mechanism when that proof succeeds; do not require legacy
   C clients to issue MPFR cleanup calls or free another thread's/global caches.
   Return the concrete ownership/lifetime design to central for review before
   publication. Sanitizer coverage remains C callers/shim, not the whole stack.

## AL follow-through and acceptance boundary

Execute these corrections in dependency order in the existing AL chat. Own
`rust/`, `validation/release/` and the named workflow; preserve central documents,
all predecessor runners/evidence, canonical native source and independent A2.
Use staged Conventional Commits and keep generated JSON, archives and binaries
ignored. Reuse existing functions and the smallest shared domain primitives.
Explain each source counterpart and necessary departure beside the changed code.

First make validation observations and native staging dependable, then implement
and verify the demonstrated Lomb/event source-profile repairs. Preserve old
production/native/counterfactual identities when adapting diagnostics to a
corrected candidate. Prepare the actual candidate, focused local checks, precise
remaining ledger and updated report for central review. Central coordinates
publication/dispatch under 0101; no further user permission is needed for a
reviewed in-scope correction/rerun. A complete actual Linux native-corpus replay
remains required. Repeat affected checks and interactions; do not rerun science
for documentation-only changes.

The other raw gates remain open: 1,862 aggregate fit failures, four discrete
utility boundaries, current/historical DDH fidelity, predictor RMS and seventeen
output groups, as well as the previous full numerical/source/scientific coverage
ledger. Historical-clock ideal checks still pass separately. Neither diagnostics
nor corrected observation assumptions accept those gates. Central retains design,
scientific policy and milestone acceptance, and will perform the 0082 checkpoint
before releasing a successor stage. No optimization workstream is opened here.
