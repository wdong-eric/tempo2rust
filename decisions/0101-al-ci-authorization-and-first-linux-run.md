# AL CI authorization and first Linux qualification

Central decision, 30 October 2026. The user selected **“Approve AL qualification
(Recommended)”** in response to the concrete b59a840 publication/dispatch request.
That request explicitly included **later centrally reviewed AL fixes and reruns
within the same scope**. This supersedes 0097's preparation-only limit for these
actions; it does not accept AL, a numerical gate, a platform or the full migration.

Central may publish reviewed AL candidates/fixes to the existing public
`wdong-eric/tempo2rust` repository and dispatch/review their qualification runs.
Routine follow-through within that authorization does not require another
permission question. Central retains candidate review and publication/dispatch
coordination; AL remains the sole Rust/release/workflow implementation owner.
Changes to scientific policy, precision, conventions or acceptance tolerances
still require an explicit central decision. Decisions 0099 and the user's
AL-first sequence remain binding; covariance optimization is deferred.

## Exact initial candidate and review

The first candidate is `b59a84024317653d92cfd757709b2beee3e766a3`, including the two
CI corrections independently reviewed under 0100. Its delta from public
`c14b1ee6fb703484036345c00453ba27c66c19df` contains sixteen authored files. The
whole Rust tree is identical except for the qualification report; no production
implementation changes are present in this delta. No generated validation JSON
is included, and the largest changed file is 103,924 bytes.

The frozen review packet is
`decisions/evidence/release-qualification/build/first-linux-b59a840-20261030/`.
It contains the exact workflow, publication diff, identities, prior central
receipt bindings and the user-facing review. Workflow SHA-256:
`ed64df9aa3c2354f324e41ce7b996b2d66359c021aa4b63fe7c205b56451e734`;
publication diff SHA-256:
`9a7d1bcc4292e9680e6802649888e64ed57c315354de24751a4d3ecd892a8d68`.
The currently uncommitted AL follow-through was excluded from this candidate.

## Publication observation and actual dispatch

An earlier read-only check found public main at c14b1ee and no qualification runs.
At action-time revalidation it already pointed to the exact approved b59a840.
Central's preflight stopped before its push, then confirmed the desired remote
identity and recorded that **central made no push**. No attribution is made about
how the remote advanced. The candidate was already public, so no duplicate update
was needed.

Central checked for an existing matching run and then dispatched the manual
workflow once at 2026-10-29 15:39:02 UTC (30 October locally), supplying the full
candidate SHA. GitHub returned run
[40903715685](https://github.com/wdong-eric/tempo2rust/actions/runs/40903715685),
created at 15:39:04 UTC with head b59a840 and event `workflow_dispatch`.
The `linux-x86_64` job is `123552666701`. Its first live inspection confirmed
successful checkout and recorded-tool provisioning, followed by the executing
native/installed matrix. These are initial observations, not completed Linux
qualification. Full dispatch output and identity records remain in the ignored
review packet.

The exact workflow selects Ubuntu 24.04 x86_64, has a 120-minute job limit and
read-only repository permissions, and retains selected evidence for 30 days.
Record actual runner/image/compiler/provider/ABI results when the job executes;
the selection label is not proof of those results. Preserve failed artifacts and
verify downloaded manifest hashes. Generated JSON, logs and binaries stay out of
Git under 0039.

## Scope and continuing AL work

Starting this committed matrix establishes actual Linux feedback while AL's
larger portable early-corpus, measured-worker and source/branch follow-through
continues locally. It does not substitute a smaller goal for that work. The
initial matrix still declares those missing pieces; all inherited scientific
failure groups and numerical gates remain visible. A nonzero run does not by
itself identify which Linux requirements have passed, failed or not executed;
review the individual evidence.

The actual full two-platform/interface/source qualification and numerical
acceptance remain pending. Review and publish later AL fixes/candidates within
the user's authorization, repeat affected checks, and retain the first run's
identity/results. No precision change, weakened comparator, allocation experiment
or production cutover is authorized by this initial dispatch.
