# Pulsar lifecycle export correction

Central review, 24 October 2026. **Assignment S at `16737d2` remains unaccepted.** Return two harness/export corrections to S under its existing `validation/pulsar_lifecycle/` ownership. The unchanged native source remains authoritative. No Rust stage or numerical-policy change is released.

## Retained link arrays are hidden after allocation

Raw `initialise.C:777` defines `allocateMemory`. It resets each parameter's link counts and replaces its dynamic buffers, but never writes the embedded `linkTo[5]` or `linkFrom[5]` arrays. After a caller has seeded these arrays, subsequent allocation and initialization retain their initialized values. Newly allocated prefit buffers are a different lifetime and remain unwritten.

S's `probe.cc` uses one `full` flag in `params` for both embedded links and dynamic prefit backing. The `reallocated` snapshot and generated `retainedAfterInit` exporter pass false, so they label the already seeded link arrays unwritten. This occurs in all four repeated-allocation histories and both reinitialization transitions. Exporting saved old structures does not capture the current structure's retained embedded arrays: the saved structure has its own copied arrays.

Central freshly built the ordinary source and a disposable mutation that inserts zeroing of both embedded link arrays after the `nLinkTo = 0` assignment. **Both builds exactly match the delivered projections for `allocate_1` and `seeded_reinitialise`.** Thus the gate accepts a defined, observable behavior change. The mutation uses initialized bounded arrays and does not execute an undefined read. It changes only disposable source.

Correct the projection to track embedded-link knownness independently of fresh prefit-buffer knownness. Preserve the source's reset counts, retained embedded arrays, overwritten dynamic owners and unwritten fresh prefit backing. Natural first allocation must still mark its unseeded link arrays unwritten without reading them. Export all current and saved live owner state at each existing boundary.

## Initial extra covariance backing is omitted

`seedOwner` allocates and initializes `ToAextraCovar` as a 1x2 matrix. The first `caller_seeded` event in `seeded_reinitialise` includes its pointer identity but omits its rows, columns, row offsets and cells. The later `reinitialised` event exports these through the saved old owner. Include this initialized backing in the initial snapshot too, with the existing symbolic ownership identity. Do not dereference an unwritten or null pointer to manufacture a snapshot.

## Correction and verification

S should correct only its harness, generated exporter/field-lifetime documentation, controls and report. Retain the old local bundle or its auditable hashes before deliberate regeneration from unchanged pinned native source. Identify exactly which projected fields/events change; pre-existing initialized values and scientific conventions must remain unchanged. Do not overwrite another stage's evidence or refresh protection membership to conceal drift.

Add sensitivity witnesses for clearing retained linkTo and linkFrom separately, and for replacing an initialized retained link record with an unwritten marker. Require actual post-action state differences, independently of changed source/profile hashes. Check the first extra-covariance snapshot against its declared dimensions, owner and cell contents. Then rerun S's complete fresh replay, integrity controls, defined compiled mutations, coverage/projection comparison and raw/delta verification. Update counts, identities, command results and limitations in REPORT.md and return the correction commit here. Central will complete the remaining review after this correction; the two diagnostic cases do not establish full acceptance.

No upstream behavior should be repaired. The native counts, replacement/leak behavior, source copy order and undefined/host classifications remain those in decision 0038. Policy 0039 applies: generated JSON, raw bundles, inventories, profiles and execution receipts remain local and ignored. Commit source, generators, concise documentation and reports only.

## Central evidence and dispatch

The tracked reproducer and log are `decisions/evidence/pulsar-lifecycle/retained-links-review.py` and `.log`. They diagnose the pre-correction delivery and its original local bundle. The complete receipt is local and ignored at `decisions/evidence/pulsar-lifecycle/retained-links-review.json`, SHA-256 `a392b203162d397a7e04c5bc086f675108957f37c929934c9e56c5dd75132205`.

The fresh ordinary executable SHA-256 is `cc3fd183a4b23ff2b5baf3cd3cfc70c4416275d5cb60fa3759db15a0b384704c`; the reset-link mutant is `81ab97b7534685d3817b5458497bea8e7b37a77572bb6ba4e8e7fa4f8fa7aa8b`. Actual executable, runner, included-source/header, compiler/backend and runtime identities were checked before and after each execution. All 132 S source/evidence files outside build/cache directories retained their original hashes. Protected evidence checks pass and both independent repositories remain clean.

Send this correction to the existing **Assignment S: TEMPO2 pulsar lifecycle reference** chat, `01a16912-7fd8-7392-9c1e-79436775676e`. Central retains design and acceptance; Q's independent reference review may proceed while S corrects its own exporter. No production Rust owner is currently released.
