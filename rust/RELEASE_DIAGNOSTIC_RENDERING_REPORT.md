# Paired terrestrial diagnostic rendering: isolated saved-evidence handoff

1 November 2026. **Saved-data verification only; fresh W/Linux qualification,
AL, M2 and full migration acceptance remain pending.**

Implementation commit: `25bd81fa2bdbce7759de6ddba5bca1784e0be48c` on
`al/diagnostic-rendering-0116`, based on
`f73f1c82d8ba08ff658a2cd337d9c5ae8751bea1`. Work is isolated in
`/private/tmp/tempo2-al-rendering-0116-20261101`; the main checkout is read-only.
[0116](../decisions/0116-isolated-diagnostic-comparator-work.md) releases this
bounded implementation of [0113](../decisions/0113-diagnostic-rendering-verification-contract.md).
The strict-pass clarification was read from main commit
`efbd22c4aba59de0b954129724e285648f78cb92`; central decisions were not merged or
edited in this branch. Central retains integration, execution and acceptance.

## Result and preserved boundary

The three independently bound macOS pairs retain their original strict stdout
differences and pass the additional source-rendering proof through the **actual
staged W `compare_case` entry point**. Both source renderings are reconstructed
from their own complete stored messages; all non-rendering fields and all stdout
bytes outside the single emission remain exact under W's established projections.
No diagnostic is removed, shortened, repaired or compared only by its prefix.

| Saved case | Original strict comparison | New saved-data proof | Visible bytes, native / Rust |
| --- | --- | --- | --- |
| `alias_bounded_tokens` | Fail | Pass | 336 / 282 |
| `observatory_exit_read_alias` | Fail | Pass | 344 / 240 |
| `observatory_missing_alias` | Fail | Pass | 344 / 256 |

The names above identify evidence, not acceptance conditions in the comparator.
Eligibility comes from the recorded action, event phase and single counter/storage
transition at the two qualified `observatory.C` emit sites. The original three
failed receipts remain unchanged. No new producer, full terrestrial comparison
or downstream control execution is claimed.

## Implementation

[diagnostic_attribution.py](../validation/release/diagnostic_attribution.py)
retains the entire `Attribution` class and `compare_pairs` function byte-for-byte.
String-only checks keep their strict behavior. The added W entry point first
compares the original ordered schema, types, state, counters, actions, status and
stderr. Its proof then binds the independent native oracle/receipt, concrete
inputs, initial resource names/bytes and subsequent effects, Rust challenge and
outcome, raw event/stream products, native compiled-source identities, and the
Rust logger/emitter/caller identities. Resource hash maps compare file membership
and hashes, independent of filesystem enumeration order; the existing ordered
scientific/state comparisons remain unchanged.

The pinned native `TKlog.h`, `TKlog.C`, `observatory.C` and probe hashes establish
the formatter, emission sites, stream routing and action markers. The proof
requires one complete warning/error storage slot and its matching counter
transition. It also checks the source message template against the recorded
call arguments and alias result or filename. Truncated, missing, ambiguous or
unexplained backing data fail. Native and Rust raw filenames must agree in this
qualified scope; only the existing declared fixture-path projection is used.

The formatter applies C-string termination, the 359-byte colored-buffer cap,
then the actual redirected `_TKchklog` byte parser: ESC begins deletion and the
next byte `m` ends it, even for a sequence outside ANSI grammar. A generic ANSI
library would not implement this source behavior; Python's existing byte and
hash facilities suffice. The complete expected stdout is assembled from the
verified event sequence and the independently rendered emission. Emission
bounds therefore do not depend on splitting message data at newlines or markers.
Each proof records raw streams, backing messages, prefixes, expected/observed
render hashes, byte bounds, and formatted tails after the cap.

A source-qualified single-message history undergoes this consistency check even
when its strict text is equal. The control that truncates Rust to the native
visible prefix exposed why an early strict-pass return is insufficient. The
receipt preserves `strict_difference` separately from the rendering outcome;
unsupported histories retain strict comparison. Central independently reproduced
this counterexample for all three pairs and checked that none of the saved 127
strict passes is selected by the draft gate; its witness is
`c9ae272ea15285034cf86be2f54d0058b91ac4d342b7cf99e3d6133e8037879c`.
That is central's saved-data review, not another AL corpus execution.

[corpus_compare.py](../validation/release/corpus_compare.py) uses one bounded
staging function to install this entry point in the copied W runner and its fresh
positive control. The same function prepares the saved-evidence test runner.
Existing native-against-native mutation controls remain strict. The adaptation
hashes bind the exact staged edits, W's preservation manifest includes the copied
verifier, and each case records its diagnostic receipt hash. Historical runners
and production Rust/native files are untouched. No companion module, dependency,
production mode or generic comparison fallback was added.

## Verification and evidence

The original cap receipt was read at
`/Users/Eric.Dong/Downloads/tempo2rust/validation/release/build/third-cap-review-v5/receipt.json`:
SHA-256 `d68ec56eb523df4679f610681ce478aa747fb7f3a406fa7a0731fa443a1d006b`.
All 22 bound inputs were checked before and after the controls. The saved producer
root is `/private/tmp/t2-local/linux/corpus/wxxxxxxxxxxxxxx`. Generated copies,
mutations and receipts are confined to this worktree's ignored
`validation/release/build/rendering-0116-v1/` through `rendering-0116-v6/`.

The final v6 command was:

```sh
AL_RENDERING_EVIDENCE=/Users/Eric.Dong/Downloads/tempo2rust/validation/release/build/third-cap-review-v5/receipt.json \
AL_RENDERING_OUTPUT=/private/tmp/tempo2-al-rendering-0116-20261101/validation/release/build/rendering-0116-v6/controls \
PYTHONDONTWRITEBYTECODE=1 python3.13 -m unittest discover \
  -s validation/release -p test_diagnostic_attribution.py -v
```

Run from the isolated worktree; the output directory must be fresh. All **13 test
methods pass**, including the five existing attribution families and two byte
parser/complete-backing boundary families. The saved-data families record:

| Outcome / rejection stage | Count | What the controls establish |
| --- | --- | --- |
| Complete paired proof | 3 | The three original strict failures pass the additional proof through the staged runner. |
| Unchanged strict positive | 1 | A saved ordinary W case retains strict handling; no proof fallback is needed. |
| Evidence integrity rejection | 11 | Missing raw/input/receipt data, stale raw/input/source/executable identities, changed file membership, outcome identity and unsupported logger flags fail. |
| Attribution rejection | 3 | Fictional Rust site, native-labelled Rust output and internal comparison tags fail. |
| Non-rendering rejection | 13 | Schema, case/classification, action/event order, missing event, counter/type, numeric state, backing message, outcome, stderr and channel changes fail. |
| Event/storage association rejection | 5 | Missing/ambiguous storage, ambiguous counters and wrong event/operation association fail. |
| Backing-message rejection | 3 | Truncated, NUL-bearing and mutually equal but source-inconsistent messages fail. |
| Exact rendering rejection | 12 | Hidden Rust tail, numeric/message token, severity, valid-but-wrong Rust/native sites, newline, missing emission, common-prefix truncation, outside bytes, marker-like additions and filename changes fail. |

Thus 51 saved outcomes comprise four positives, **36 semantic rejections** and
**eleven integrity rejections**. Stream and matched-message mutations update the
copied dependent raw/result/receipt hashes consistently. The tests assert the
intended rejection stage and a concrete reason; stale checksums cannot stand in
for semantic protection. Missing/permuted full-catalogue controls retain all 130
original case identifiers. They are membership checks, not a 130-case rerun.

V1 retained three failed positive proofs caused by an incorrect assumption that
W's source receipt included nested Rust modules, one wrong parser-test expectation
(`unterminated` itself contains `m`), and the strict-pass counterexample. V2 retained
three failed positive proofs caused by treating resource-map enumeration order
as semantic. These attempts do not qualify the negative controls. V3–V5 are
retained passing intermediate checks; v6 adds the coherently rebound native-site
control and is the final implementation evidence.

Final v6 identities:

- `controls/summary.json`: `0413213c18a0c2f79a227d313b182f561f8c429ba56932a693a19cf23d595eea`.
- `verification.json`: `6ff5f9aaf3b49b0680effd1e011b92b8a4eff4999ac941e5c044aaac2184f0d6`.
- `diagnostic_attribution.py`: `cb83f64c6714ed7322a596c202848814a72e00e858a548966353334aebda909e`.
- `corpus_compare.py`: `d9e51667a2dd532b8d6e72b9c3da3cf2019f0c326e75241f7b66439ece39d342`.
- `test_diagnostic_attribution.py`: `7cf1804d4f86d1d4262f56cbad151b7b8d96f2983422a0c36f2b91d34a88d700`.

The final identity check verifies the strict adapter, the unchanged main copies
of all three owned Python files, AM's README, the accepted source-map/report,
and the 22 original evidence inputs. The diff is confined to the three owned
Python files and this report; exact diff checks pass. No generated evidence is
committed.

## Remaining execution boundary

No engine execution, native/Rust compilation, fresh full W/downstream controls,
actual fourth-run Linux replay, integration into main, publication or workflow
action was performed. The additional proof is limited to the reviewed redirected,
single-message source/event shape and requires its independently bound evidence.
Unsupported histories retain strict comparison. Other formatting modes, ambiguous
histories or changed source profiles cannot gain acceptance through this proof
without further review. All original scientific gates and
failures, production precision/resources and deferred covariance work remain.
Central must review this isolated commit and the exact fourth Linux result before
releasing integration or further execution. Work stops at this handoff.
