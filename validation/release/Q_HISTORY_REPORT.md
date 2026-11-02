# Q bound-history correction (0119)

The comparison workspace now reuses `native_corpora.stage_history` to retain Q's
exact `38d62af9…` and `ad6fbf692…` shallow roots from the bound native preparation.
The historical `git show` membership/configuration lookups run unchanged. No
canonical source, historical runner, fixture, protection manifest or output is
rewritten. The comparison stage also executes Q's existing native verification;
its preservation check uses the same explicitly bound adaptation ledger as the
staged runner. Unlisted source drift remains a rejection.

Before: Linux run `41312212554` failed `par-native-matched` at
`ad6fbf6:decisions/evidence/par-specialized/released-branches.json` because the
comparison copy lacked `.git`. The original stderr is retained in the new
ignored `build/q-history-0119/` evidence packet.

After: fresh macOS preparation and the **actual staged execution** pass:

- 107 native authored cases, with the seven telescope inputs bounded to the
  already approved capacity 500.
- 16 identity/launch controls, 11 comparison controls and 14 compiled semantic
  mutations from the unchanged native verifier, including failure cleanup.
- 121 Rust comparison cases, including the existing capacity-boundary cases.
- Eight focused history/preservation tests, including missing-history failure,
  shallow-origin recovery, preservation of the failed receipt and rejection of
  an unbound staging change. No source mismatches were reported.

The recipe was `native_corpora.py --stages par_specialized`, followed by
`corpus_compare.py --stages par_specialized`, using the clean `b52d595` export
and the accepted A2 library. Exact commands, source/executable bindings, history
refs and trace logs are in `/private/tmp/al119q-native/receipt.json` and
`/private/tmp/a119q/receipt.json`. The new aggregate receipt and their hashes are
in `validation/release/build/q-history-0119/receipt.json`.

No Rust production code changed in this correction. Linux verification is still
due through central's next execution; unchanged W was not rerun. This report
makes no broader parser or release-qualification acceptance claim.
