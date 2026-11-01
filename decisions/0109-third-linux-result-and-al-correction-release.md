# Third Linux result and bounded AL correction release

Central decision, 1 November 2026. **AL, M2 and full migration remain pending.**
The focused presentation delivery is accepted by 0108. AL resumes implementation
under this brief as the sole owner of `rust/`, `validation/release/` and the named
qualification workflow. Central retains policy, publication review and milestone
acceptance. AL-first sequencing and the deferred 0099 covariance investigation
remain unchanged.

## Reviewed result and evidence

[Run 41069548581](https://github.com/wdong-eric/tempo2rust/actions/runs/41069548581)
completed with failure at 16:02:33 UTC on 30 October, on frozen
`a35104a611abb5c1d5c48dc4d1866f239ccd3ab9`. It predates the presentation change.
Of 67 matrix stages, 54 passed and 13 failed; no top-level stage was blocked.
The corpus stage separately records four unavailable native prerequisites.

The failed stages are native-corpora, corpora-comparison, both C-concurrency
profiles, M2 comparison, fits, boundaries, DDH, timing, event diagnostic, event
source profile, predictors and output. The new Linux Lomb/contraction checks
pass. Provider ABI, the installed C interfaces, Rust suite, worker campaign,
package audit and source audit report passing executions. The measured ordinary
long double is 16 bytes / 64 significand bits; Fortran default integer is 32 bits,
OpenBLAS is 0.3.26 LP64 and CALCEPH is 4.0.5. These results qualify their recorded
checks, not the whole platform. The package audit checked 2,524 files without a
mismatch. Source presence/navigation is not branch coverage.

The original artifact download timed out after 595,591,168 bytes. A standard
curl Range continuation recovered the same artifact; no new workflow was run.
Exclusive extraction then stopped on Linux case-distinct `DM.txt`/`dm.txt` names
on the macOS filesystem. The partial extraction is retained and is not the
complete evidence tree. Standard Python `zipfile` subsequently verified all
30,316 selected files / 6,605,117,390 bytes inside the original archive, retaining
all twelve case-distinct path groups. Read evidence by exact ZIP member name.

- Artifact 12991483041: 774,024,809 bytes, SHA-256 `cdb2fca1cf14f4bd5c9c025c4aa2f9185bdf5c289a60e1d4a4456bff4831cf0a`.
- Selected manifest: `6484c6b3238d807478740c4a8f20ae3770c0a99ae0b7481bdb72307ca140dd15`.
- Complete archive verification receipt: `bef1530dd2ea464e3c67d7749c339ecd57022058f556ecc5a021da03c54e24b6`.
- Central selected-product review: `46640be739a8fde1b4a22ac6c7d71449a4a30a5fccaecd40575e31d17e32ce19`.
- Executed selected-product recipe: `5892ce432841b1fdff832cecd37b062014c2d128769b87814a45c5bf959d3e72`.

The central recipe independently binds 157 Rust sources to the frozen candidate,
rechecks 180 selected artifact members, compares the cache-control outputs and
replays all fifty M2 records. The exact collection, recovery, archive verification
and selected-product recipes are committed in
`decisions/evidence/release-qualification/`; generated evidence remains ignored.
The packet is `build/linux-run-41069548581-review-20261030/` there, with complete
ZIP and verification under `recovery-20261101/` and central review under
`central-review-20261101/`.

## 1. Restore portable execution and honest comparison

Implement these in AL-owned successors, preserving historical runners, original
failures and raw evidence. No generic stderr scrubbing or scientific gate change.

| Finding | Released correction and required check |
| --- | --- |
| JPL `max_cheby_assertion` is rejected as an unclassified sanitizer result; Linux emits the expected `jpleph.c:326` `ncf < MAX_CHEBY` assertion. | Recognize the measured GNU assertion form with source/predicate/termination checks. Reject unrelated crashes and sanitizer defects. |
| TIM `deletion_boundary` aborts before `readTimfile`. Its first probe action copies a 152-byte path into native `deleteFileName[100]`; the last event is `begin action 0 set`, followed by fortified-buffer termination. | Apply the existing short-path staging pattern to the native producer too, preserving the input bytes and scientific boundary. Verify the complete path fits including NUL before the caller copy. Do not truncate the filename, enlarge the native field, disable fortification or treat this fixture-caller overflow as a demonstrated TEMPO2 algorithm bug. Retain the original failure and confirm corrected Linux execution. |
| Lifecycle coverage/corrected-native assertion comparisons differ in executable labels `probe`, `coverage`, `fixed`. | Compare the explicitly bound executable identity separately; retain source assertion text, status and all state. |
| Planetary rebuild and time controls compare ASLR addresses in `ldd` output. | Retain raw loader output; compare bound library identities, paths and hashes with only measured load addresses treated as runtime metadata. |
| Propagation requires macOS's ordinary long-double size/significand. | Use the measured host ABI profile, retain binary128's 113-bit check and reject inconsistent type evidence. |
| PAR and terrestrial preservation checks see the declared staged adaptations to `parameters/catalog.py` and `time_ephemeris/verify.py` as unexplained drift. | Bind exact original/adapted hashes and patches in the successor preservation contract. Unexpected source changes must still fail; do not overwrite historical protected manifests to conceal drift. |
| Event source-profile C++ declares a global `gamma`, conflicting with glibc's function of that name. | Rename or scope that probe variable without changing types, operations or observations. Actually execute the Linux profile afterward. |
| Output `direct-api-recovery` times out while Cargo cold-compiles, before its 75-second execution allowance can test the API. | Prepare the exact test binary with a separately recorded build budget, then retain the bounded execution and recovery assertions. Preserve the original timeout. |

Central reproduced all fifteen reported M2 failures. Extending only the
comparator's in-memory recognized set to include `-NaN` removes ten spelling
errors and the derived completion-status error; four original numerical/order
failures remain. Seven positive/negative controls retain NaN-sign distinctions.
Implement an AL-owned successor with those semantics, keeping the historical
checker and both raw/signed observations. `NaN` and `-NaN` must remain distinct;
finite substitutions, infinity and unsupported spellings must fail. Do not sort
SVD modes or relax any value gate.

Apply 0107's validated attribution adapter when replaying historical native-label
corpora against current Rust. Preserve message bodies, ordering, storage, file
membership, scientific values and the measured full-line cap. Never accept an
internal comparison tag as raw Rust attribution.

## 2. Promote the measured foreign-thread cache lifecycle correction

The guarded private Rust library passes the actual Linux C-client intervention
in both default and combined configurations. Each original, explicit caller
cleanup and library-owned variant captures twelve complete workflows and twelve
TSV storage products. Central compared their complete keyed transcripts and
storage bytes: they agree. Original ASan/LSan executions report 544 leaked bytes
in sixteen allocations and exit 1; both cleanup interventions exit 0. Only
`src/predictor_ffi.rs` changes in the private library. Its measured guard-source
hash is `9cef7c936bb5e92bd5d991f125190ee24e4e7afccd821521cbee2a9683e8d91e`;
patch hash is `5081b6113f6e276918507946f63b7d9f61578e53e2b82deda99ab1882add8fc9`.

This meets 0099's evidence requirement for the scoped Rust ownership improvement.
Promote that design: register a thread-local owner inside the existing numeric C
adapter's panic boundary and use Rug's existing safe
`rug::float::free_cache(FreeCache::Local)` at calling-thread exit. Rug 1.30.0's
local `src/float/mod.rs:370-407` documents this use and delegates to MPFR's local
cache release. Reuse it; no custom allocator, new public cleanup API, global cache
freeing or additional unsafe code is justified.

Place the explanation beside the guard: identify the predictor C counterparts
(`tempo2pred.c`, `cheby2d.c`, `t1polyco.c`), the Rust/Rug cache lifetime introduced
by this implementation, the measured foreign-pthread exit leak and the local
cleanup. This is a Rust integration correction, not a claimed native TEMPO2 bug.
Keep the original control source and evidence as an intentional historical
artifact; adapt the current campaign so promotion does not inject a second guard
or require a known-bad production result. Verify actual installed plain,
ASan/UBSan and TSan C workflows on both selected hosts and both configurations,
with exact output/storage agreement and the established caller exclusivity.
The sanitizer scope remains C clients and C shim; Rust, MPFR and providers are
not all instrumented. This is not a blanket concurrent-API safety claim.

## 3. Pursue the next source arithmetic correction

The remaining event error is 2.4902768649276456321128455906671e-12 s, above the
unchanged 1e-12 s gate; its phase error is 3.07441585495698007242e-10 turns.
Native observer neutrality passes. The f64-temporary counterfactual is much
worse, and Gaussian-only/without-events controls pass, but these observations do
not establish the remaining cause. Obtain the actual Linux source-type profile
and use a bounded first-divergence trace. Shared nominal precision alone does not
establish identical `exp`/`log` results or authorize substituting a library call.

Release the spin item in `rust/RELEASE_ORDINARY_TYPE_PROPOSALS.md` first. Measure
native and counterfactual F0–F13/fallback/braking callbacks, signed x and update
indices 9/10. If those traces confirm the proposed source ordinary assignments
and literal promotions, restore exactly those boundaries in the bounded and
general Rust paths, with counterpart comments and before/after evidence. This
explicitly permits host ordinary precision 53/64 at the documented source sites;
it does not change native binary128 / Rust Rug128 policy. Preserve double output
assignments, source operation order, `84000L`, zero-braking behavior and the
ordinary `1e23L` promotion; do not opportunistically alter formulas. Demonstrate
which of the two retained M2 callback errors is explained and retain any others.
Recheck affected macOS results and actual Linux execution before acceptance.

Chromatic, cosmic-string and secular proposals remain subsequent bounded work.
The new early-corpus differences also need diagnosis: the clock duplicate-table
case differs by one binary64 ULP; preprocessing reports 160 differences plus
three diagnostic differences; observer-coordinate runs pass 95 of 133 cases in
both original/corrected comparisons. Lifecycle original-state differences remain
separate from the assertion-label issue. Do not label these scientific differences
as platform metadata or assume all are explained by ordinary type width.

The retained Linux ledger includes 1,404 fit failures, four discrete-boundary
failures, 1,062 current/historical DDH fidelity failures (historical ideal checks
pass), and ten raw predictor log10RMS failures across 37,814 checks. Output's
other discrepancies remain open after isolating its cold-build timeout. These
counts are recorded observations, not permission to relax thresholds. The exact
SVD-order gate, production clocks, versioned DDH rule and all earlier unresolved
scientific decisions remain binding.

## Delivery and execution boundary

Work in dependency order: portable observation/preparation and focused controls;
measured cache promotion; event/spin source evidence and authorized corrections;
then the affected installed/macOS checks and an updated complete ledger. Reuse
the existing stage scripts and native-corpus selectors for focused diagnostics.
The integrated runner has no stage selector; a new generic orchestration layer
is unnecessary. Any proposed focused Linux workflow must remain explicitly
partial, preserve candidate/source/resource identity and retain failed evidence;
it cannot stand in for the final full matrix.

Return scoped Conventional Commits, `rust/RELEASE_QUALIFICATION_REPORT.md` and
small authored recipes/reports with ignored raw products. No reproducible JSON,
binaries or archives in Git. Preserve central and other writers' work. The 0101
user authorization covers later centrally reviewed fixes/reruns, but this brief
does not itself publish or dispatch a candidate. Bring the concrete candidate
and workflow diff to central; no independent push or dispatch. Scientific-policy
changes still require a separate central decision.
