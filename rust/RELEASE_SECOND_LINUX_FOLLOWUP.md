# Second Linux follow-through (decision 0104)

Work starts from central decision `4472d99`. The reviewed Linux run executed
`4accd26`: 54/66 top-level stages passed, twelve failed, none blocked. No early
native corpus executed. Historical failures and native runners remain intact.
This report records successor evidence, not milestone acceptance.

## Native history staging

The original third HTTPS fetch failed with Git's shallow metadata consistency
error, after two successful fetches. It did not establish a missing commit.
Local Git 2.54 traces show detached automatic maintenance/repack jobs between
the original sequential fetches. That is a plausible competing metadata writer;
the Linux Git 2.56 run did not retain a trace proving which writer changed it.
The three-fetch failure has **not reproduced** on macOS: twelve local sequences,
one HTTPS sequence, and a separately delayed/overlapped repack control completed.
Those observations are retained; they are not Linux qualification.

`native_corpora.py` now disables automatic maintenance on each exact depth-one
history fetch and retains each root under `refs/al-history/<SHA>`. It records
Git trace events and checks each resulting ref. It neither fetches broad history
nor retries a failure. `corpus_compare.py` publishes the failed preparation's
identity and every unavailable comparison when no native stages were reached.

Executed evidence: `validation/release/build/second-followthrough-history-fixed-v1/`
starts with a fresh depth-one checkout and fetches all seven pinned historical
commits through HTTPS. Every tree matches the local historical tree exactly.
The actual failed Linux preparation receipt produces fourteen explicit unavailable
comparisons without changing its bytes. Two focused unittest controls additionally
exercise a failed fetch, separate fresh recovery, retained old receipt, exact
shallow roots, and unavailable prerequisite publication. No native corpus was run
by these checks. The next Linux replay must still execute all thirteen native
corpora and the Rust comparisons, and confirm the staging correction there.

Original investigation directories are `second-followthrough-history-v1`,
`second-followthrough-history-race-v1`, `second-followthrough-history-repeat-v1`,
`second-followthrough-history-overlap-v1`, and `second-followthrough-history-https-v1/v2`
under `validation/release/build/`. The v1 HTTPS attempt failed DNS inside the
sandbox; v2 was the authorized network execution. No failed receipt was replaced.
