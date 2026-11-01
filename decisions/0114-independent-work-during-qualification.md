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
