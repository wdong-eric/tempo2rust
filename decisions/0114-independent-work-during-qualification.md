# Continue independent work during external qualification

Central decision, 1 November 2026, recording the user's instruction to continue
independent work while Linux tests run and check their status roughly every
fifteen minutes. Waiting for one execution must not suspend unrelated work
needed for the migration. Central remains responsible for design, coordination
and milestone acceptance.

## Current boundary

Run `41294531402`, job `124812485584`, attempt 1 continues on reviewed
`fecbc18af95196a16db2046306099b6345a50976` under 0112. Its candidate, comparisons
and artifact identity stay fixed. Central's local monitor now checks every
900 seconds and invokes the reviewed collector when that same run completes.
A timeout observing the run does not authorize another dispatch. Respond to
user input or a concrete execution issue when needed; fifteen-minute polling
is not a reason to delay independent work or error handling.

AL's implementation freeze still covers `rust/`, `validation/release/` and its
workflow until central reviews the result. Decision 0113 prepares the paired
diagnostic-rendering contract but does not release that implementation yet.
The user-approved AL-first sequence remains: the 0099 covariance allocation
investigation and broader performance optimization follow AL. All numerical,
precision, resource and acceptance policies remain unchanged.

## Independent documentation assignment

The live repository has no root README or user migration guide. The product
entry point and installation/workflow contracts exist in `rust/README.md`,
`rust/TERMINAL_EXAMPLES.md`, `rust/LIBRARY_INTERFACES_API.md` and the release
recipes, but the repository landing page does not lead a TEMPO2 user to them.
Addressing this gap directly serves the 0106 familiarity principle and is
independent of Linux scientific results.

Release Assignment AM under `handoffs/39-user-migration-documentation.md`.
AM owns only new root `README.md`, `docs/MIGRATION.md` and
`docs/MIGRATION_REPORT.md`, with generated evidence in ignored `docs/build/`
or a named temporary directory. It may read existing code and run narrow
isolated command demonstrations. It must preserve AL's files, source snapshot,
installed products and evidence. It must not change scientific behavior,
regenerate qualification, install over an existing prefix or publish anything.
Central and AM are concurrent writers in explicitly separate paths; neither
may stage or revert another writer's changes.

The documentation should lead with the native-style human-facing `-f` workflow,
correct runtime/build setup, truthful Rust diagnostics and a small migration
map to the existing detailed contracts. Distinguish evaluation from fitting,
explicit machine JSON adapters from terminal reports, core Rust APIs and
predictor C compatibility from excluded legacy/plugin ABI guarantees, and
implemented capabilities from accepted numerical/platform qualification.
Reuse the existing documentation and tooling; no new documentation framework,
CLI wrapper, formatter, package or status dashboard is needed.

AM may commit authored owned paths using `docs:` messages and return its report
for central review. Its work is local and does not change public `fecbc18` or the
running workflow. A later publication/package decision must review the actual
changes. Documentation delivery cannot close AL, M2 or any scientific gate.

## Continuing coordination

Choose additional parallel work only when it has a concrete migration need,
a bounded deliverable and no dependency on the unfinished execution. Keep
ownership and exact input identities clear. Central will review AM's delivered
commands/claims and the Linux result independently, then release the next
implementation work from actual evidence. Full migration remains pending.


## Actual AM dispatch

Central created **Assignment AM: User migration documentation**, thread
`01a196a7-fd45-7043-b025-70c612415e5e` in the local `rustify-tempo2` project, from
release commit `a8752fcec2af65e5756e6cfc41d85f0de23e4a3f`. The first observed turn
is in progress and has confirmed the clean release and the evaluation/fitting
contract before writing. Its final report will be read here for central review.
This records actual task startup, not documentation delivery or acceptance.


## Independent AL source-map documentation refresh

After AM closure under 0115, central releases a second independent,
documentation-only item to the existing AL owner. `rust/RELEASE_SOURCE_MAP.md`
still describes the predictor cache guard as present only in a private
diagnostic copy. Current `predictor_ffi.rs` contains the accepted production
guard from `fd185ec`; 0112 binds it to the frozen candidate. The map's older
body-change counts and execution rows also need clearly dated historical scope
when read beside the current candidate. This is a concrete source/status
documentation gap, not a newly diagnosed scientific or memory defect.

AL may edit only `rust/RELEASE_SOURCE_MAP.md` and the corresponding documentary
update in `rust/RELEASE_THIRD_LINUX_FOLLOWUP.md`. Reconcile the map with exact
`fecbc18` production sources, accepted 0107-0111 changes, preserved predecessor
records and current known execution limits. Distinguish previous private-cache
evidence from the promoted guard and its scoped macOS evidence; actual current
Linux qualification remains pending. Keep routine presence, source correspondence,
lexical unsafe/comment indexes, executed branches and scientific acceptance
separate. Do not turn a count or a source-map refresh into a safety or coverage
proof. Preserve the original failures and historical identities.

Use existing source/audit recipes and saved evidence. Read-only Git/source
inspection, a fresh ignored documentation/source-identity receipt, link checks
and exact diff checks are authorized. No source/test/manifest/installer/script/
workflow changes, engine execution, compilation, numerical comparison, new
precision policy, optimization, publication or workflow dispatch is released.
The production implementation freeze and running candidate remain unchanged.

Return a scoped `docs:` commit and the updated report for central review, then
stop. AM's accepted `rust/README.md` is preserved. Central monitors the same
Linux job on the fifteen-minute schedule and retains every acceptance decision.


## Central acceptance of the independent source-map refresh

Central accepts documentation commit `062a567b943c18855ab05742d51f504da8278cdd`
on 1 November. Its exact two-file change corrects the production cache status,
dates historical source counts and execution evidence, and retains current
Linux/scientific qualification limits. The real AL follow-up turn completed in
thread `01a17c51-359e-7041-b04b-efebd2b2ec6e`; this bounded documentation item is
closed. AL retains its main assignment and implementation remains frozen.

Central inspected the final diff and source/report claims, checked the exact
parent and path membership, independently rehashed all 332 declared frozen
files, and verified that those files have no Git difference from `fecbc18`.
The direct baseline comparison confirms 158 Rust files: 107 unchanged, 48
modified, three added and none removed. Both historical audit receipts and
counts remain preserved. All 32 relative links resolve with exact tracked
path case; the accepted AM README hash is unchanged. Across the frozen Rust,
release and workflow trees, only the two reviewed documents and AM README differ.
No engine was run and no scientific evidence was regenerated for this review.

The AL documentary receipt is
`df3ad68181bb9ab8cccd9950520d1b9ce07ad94476b49bb7dd4be90b1d998529`;
its source-audit receipt is
`49857535a7772c2d921f2a3c9fe28adcde9b12f39ae3009ee172cac895f82c7e`.
The central receipt and reviewed diff are retained in ignored
`decisions/evidence/release-qualification/build/source-map-review-20261101/`;
central receipt SHA-256 is
`6274ac5b95064fc128755ce73d83b0692cd8d16a9a7d44c09d0fa9e326fb19bf`.
Generated receipts remain outside version control.

This accepts the documentation correction only. The same fourth Linux job was
verified in progress at 11:09:24 UTC, executing its native/installed matrix.
The fifteen-minute monitor remains active. AL, M2 and full migration acceptance,
0113 implementation and the post-AL profiling release remain pending.


## Isolated implementation follow-through

After AM and the source-map refresh are accepted, 0116 releases the previously
specified 0113 comparator and saved-evidence controls in a separate Git worktree.
This supersedes the blanket scheduling freeze only for that bounded successor;
the main implementation and active candidate remain unchanged. Fresh engine
execution, actual Linux replay, integration and publication still await central
review. The user-requested independent work now advances a diagnosed AL issue
without starting the deferred covariance/performance pass.
