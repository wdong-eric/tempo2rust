# User migration guide review and bounded README clarification

Central decision, 1 November 2026. Accept the repository onboarding delivery
from Assignment AM: `81af30989aa39748a65d13a015397d8ab997ca9e` adds the root
README and migration guide; `8f293a2667c20d80cd6b1b608f6a5b79734ba576` records its
verification. This accepts the scoped documentation delivery under 0114, not
AL, M2, platform qualification or full migration. A small documentation-only
follow-up below remains assigned to AM.

## Reviewed delivery

The new entry page and guide lead with the native-style human report, preserve
explicit evaluation/fitting distinctions, explain runtime/input choices and
working-directory outputs, and link the existing Rust/predictor C contracts.
They keep truthful Rust diagnostics and describe implemented capabilities and
pending qualification separately. The user-facing paths are portable and
repository-relative. Central checked the installer and CLI source for the
introduced command/default/runtime claims.

Central verified the three committed document hashes and exact changed-file
scope, all 68 links against case-sensitive tracked paths and heading fragments,
and eight shell blocks with `sh -n`. All 21 recorded commands have their expected
exit status, with complete stdout/stderr and generated-file membership/hashes
bound; fourteen are main-CLI invocations. The unchanged output checker was
replayed on a private evidence copy: all fifty documentation assertions and
24 actual installed Rust emit sites agree. The initial nine-command harness
attempt is retained separately; its case-colliding directory labels are not
an application failure.

These demonstrations executed the provenance-checked default macOS installation
from `a8ba6f15d2933b206f85ce70c99003b10f3f653d`, binary SHA-256
`df523407e7db73ba3dabe27377252627fafe7ae466d9922fabe95a6865c62e5f`.
They do not represent a new build of the running Linux candidate. Central reused
the previously accepted full installation audit and rechecked the named receipts,
executable, inputs and consumed resources. No new engine execution, build,
installation, scientific comparison or Markdown renderer was part of this
central review. The initial delivery leaves `rust/`, release scripts and workflow
identical to `fecbc18`.

The exact review recipe is
`decisions/evidence/user-migration/central-review.py`, SHA-256
`d88f929ac5ab4beec294f704e497a1f250f7c9a4cfeaad35654796c4ab1a444a`.
Its receipt is
`81b145c3f184cca61b5064c4aa444e8f5f0b261018930225656c8f974e2c7772`.
Original `/tmp/tempo2-central-am-review-20261101/` evidence is retained byte-for-byte
under ignored `decisions/evidence/user-migration/build/am-review-20261101/`.
Only the authored recipe and central decision are committed.

## Released documentation-only follow-up

AM identified two stale statements in the older `rust/README.md`. Central
confirmed them against `main::run_bats`, `calceph.rs`'s feature branches and the
existing backend contract:

1. The general input-to-BAT section categorically says CALCEPH requests are
   unavailable. State the actual distinction: CALCEPH requires its explicitly
   enabled feature/provider/data; external selection plugins remain unavailable.
2. The build section's unqualified runtime-free barycentric statement describes
   the bounded adapter. Identify that scope and state that general `bats` and
   `timing` require `--runtime`, while the native route requires `TEMPO2`.

AM temporarily owns **only these clarifications in `rust/README.md`**, plus the
follow-up record in `docs/MIGRATION_REPORT.md`. This is an explicit partition
from AL's broader path ownership. All production code, tests, manifests,
installer logic, release scripts and workflow remain frozen. The current remote
candidate/run are immutable. No code, numerical, format, resource or acceptance
policy change is released.

Keep the original AM evidence and its historically true source-identity claims
intact; record the subsequent documentation-only exception explicitly. Check
new wording against source, relative links, Markdown/shell syntax as applicable
and the exact two-file diff. Reuse the accepted operational demonstrations;
no fresh scientific suite or new installation is needed for this wording.
Commit with `docs:` and return the exact commit for central review. AM ownership
closes after the bounded follow-up is reviewed.

The new root/guide pages are repository onboarding; the current installer does
not copy them. Changing package membership is separate AL work and is not
released here. The drafted covariance investigation stays unreleased, and
central's Linux monitor continues at roughly fifteen-minute intervals.
