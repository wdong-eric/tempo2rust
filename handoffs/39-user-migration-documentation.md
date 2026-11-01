# Assignment AM: user migration documentation

Released by decision 0114 while AL's Linux run executes. Work in
`/Users/Eric.Dong/Downloads/tempo2rust`. Read HANDOFF.md and decisions
0065, 0074, 0084, 0086, 0099, 0106, 0107, 0108, 0112 and 0114 as needed for the
contracts below. The reviewed production snapshot is
`fecbc18af95196a16db2046306099b6345a50976`; central documentation commits may
follow it. Inspect actual current files before making claims.

## Ownership and outcome

Own only new root `README.md`, `docs/MIGRATION.md` and
`docs/MIGRATION_REPORT.md`. Generated command output and receipts belong in
ignored `docs/build/` or a named temporary directory, never in Git. You are not
alone in the repository: central is editing decisions/handoffs, and AL owns
`rust/`, `validation/release/` and its workflow. Preserve other writers' edits
and the frozen Linux candidate; stage and commit only your explicit paths.

Deliver a usable repository entry page and a concise guide that helps an
existing TEMPO2 user build/install the Rust product and choose the intended
workflow without changing scientific semantics. Central retains design,
publication, scientific policy and acceptance. Your assignment does not depend
on the running Linux result and must not claim that it has passed.

Proceed in dependency order:

1. Inspect existing installation, CLI and API contracts and their actual source
   entry points. Reuse `rust/README.md`, `rust/TERMINAL_EXAMPLES.md`,
   `rust/LIBRARY_INTERFACES_API.md`, `rust/PREDICTOR_API.md`,
   `rust/CORE_OUTPUT_API.md`, `rust/NATIVE_BACKENDS_API.md`, `rust/install.py`,
   the Cargo manifests and `validation/release/README.md`. Resolve a concrete
   inconsistency from source; report any required behavioral change to central.
   No new documentation framework or implementation infrastructure is needed.
2. Write a short root README: what this project delivers, its current
   qualification status, a clear route to prerequisites/build/private-prefix
   installation, a familiar `tempo2-rust -f model.par arrivals.tim` example,
   runtime data setup, and links to the guide and existing detailed contracts.
   Use repository-relative links and portable paths, not Eric's local paths or
   a retained validation build assumed to exist in a fresh checkout.
3. Write the migration guide around user tasks. Explain source-tree versus
   installed commands, runtime and input provenance, files written to the
   working directory, selecting one concrete workflow and interpreting the
   report. Keep names, conventions and units familiar; Rust-owned diagnostics
   name actual Rust files/lines. Provide a small native-to-Rust command/interface
   map where semantics really agree. Separate fixed-parameter evaluation from
   fitting; native `-pre` still fits. Explain the explicitly named JSON routes
   and narrower residual/fit adapters. Link the canonical predictor/C/Rust
   contracts rather than reproducing their entire API lists.
4. Verify all relative links and the commands/claims actually introduced.
   Execute focused help and representative human-report/evaluation examples
   against a named current build or an existing installed artifact whose source
   identity is established. Use fresh working directories and record the real
   environment, inputs/runtime, exit codes and output. Identify commands that
   were only source-checked or require user-supplied data. Do not fabricate
   numerical examples, copy untested output as executed evidence or treat a
   demonstration as scientific validation. Reuse valid existing receipts for
   expensive unchanged build/install paths; do not rerun AL's matrix or historical
   scientific suites for this documentation task. If a small build is necessary,
   use a private target/prefix and preserve existing artifacts.
5. Commit the authored owned paths with a scoped `docs:` message and return
   `docs/MIGRATION_REPORT.md`: source and executable identities, the claim/command
   checks, observed results, unexecuted steps, any concrete remaining gap and
   the exact commit(s). Stop for central review when this bounded delivery is
   complete. Do not push or dispatch workflows.

## Required distinctions

- The first qualified release targets macOS arm64 and Linux x86_64; delivered
  code and scoped evidence do not establish full release qualification. AL,
  M2 and inherited numerical acceptance remain pending. Link the existing
  qualification report/central status; avoid a new duplicated failure ledger
  or an automatic badge that implies scientific acceptance.
- The external target is full Rust core/toolkit APIs plus the standalone
  predictor C API. It does not promise legacy `pulsar` C/C++ structure ABI or
  execution of excluded TEMPO2 plugins/GUI programs. The native-style help
  catalogue alone is not proof that an external plugin works.
- Runtime resources are supplied separately. Preserve the chosen clocks,
  ephemeris, units and inputs; do not suggest silently substituting resources
  to obtain a closer result. Describe required user choices explicitly.
- Preserve the default scientific precision and existing numerical policies.
  Do not advertise an unmeasured speed/memory benefit or blanket concurrency
  safety. Link the measured limits and established execution contract where
  relevant to a user's choice.
- A report-format change must not turn evaluation into fitting or alter mean
  removal, state updates, iteration count or scientific outputs. Existing source
  contracts determine the commands, not a wish to make examples look alike.
- Keep product guidance concise and task-focused. Detailed review receipts and
  internal assignment history belong in the delivery report or linked existing
  development documents, not in the user's main workflow.

## Dispatch prompt

Implement Assignment AM under decisions/0114-independent-work-during-qualification.md
and handoffs/39-user-migration-documentation.md. Own only new README.md,
docs/MIGRATION.md and docs/MIGRATION_REPORT.md. Create a source-checked, narrowly
executed onboarding and migration guide using the existing native-style CLI,
installer and API contracts. Preserve AL's frozen source/workflow and every
scientific gate. Work independently of the running Linux result, commit only
authored owned paths, and return the report and exact commits for central review.


## Initial delivery accepted; narrow follow-up under 0115

Central accepts `81af309` / report `8f293a2` as repository onboarding, with the
review and exact evidence in decision 0115. That decision extends AM ownership
to two stale statements in `rust/README.md` only: CALCEPH feature availability
and the bounded scope of runtime-free barycentric evaluation. Correct those
statements from source and append the follow-up to `docs/MIGRATION_REPORT.md`.
Preserve the initial evidence and make its later documentation-only exception
explicit. Recheck wording, links/syntax and exact two-file scope; do not rerun
scientific qualification. All other AL files remain frozen. Return the scoped
`docs:` commit for central review, then close this ownership window.


## Ownership closed after central review

Decision 0115 accepts initial `81af309` / report `8f293a2` and the scoped
`f792b79` README follow-up. Central verified the concrete documents and retained
evidence; AM ownership is now closed. Preserve all authored files and ignored
evidence. Further work needs a new bounded release; no publication, numerical
acceptance, AL release or optimization is implied.
