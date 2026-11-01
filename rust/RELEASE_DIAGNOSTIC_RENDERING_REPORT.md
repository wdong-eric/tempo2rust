# Paired terrestrial diagnostic rendering: isolated macOS verification

1 November 2026. **Saved-data proof and fresh macOS W/native/downstream
verification pass. Linux qualification, AL, M2 and full migration acceptance
remain pending.**

Implementation commit: `25bd81fa2bdbce7759de6ddba5bca1784e0be48c` on
`al/diagnostic-rendering-0116`, based on
`f73f1c82d8ba08ff658a2cd337d9c5ae8751bea1`. Work is isolated in
`/private/tmp/tempo2-al-rendering-0116-20261101`; the main checkout is read-only.
[0116](../decisions/0116-isolated-diagnostic-comparator-work.md) releases this
bounded implementation of [0113](../decisions/0113-diagnostic-rendering-verification-contract.md).
The strict-pass clarification was read from main commit
`efbd22c4aba59de0b954129724e285648f78cb92`; central decisions were not merged or
edited in this branch. Main commit `553fb0ed7b0955fdfd69fc03dec01cc1222454e1`
subsequently released fresh macOS W execution in this same isolated worktree.
Central retains integration and acceptance. The fresh run used a clean export of
`13e33f27dc043b3a68ac6f8ff8133c417996a57a`; no implementation correction was needed.

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
failed receipts remain unchanged. This table records the saved-evidence phase;
the subsequent fresh producer, complete comparison and downstream results are
reported separately below.

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

## Fresh macOS execution

The clean export is retained at
`validation/release/build/macos-w-candidate-v1/source`, with export receipt
SHA-256 `e4fe22e782c7e0572452f24b1ab14af4212e9ada6fcac06e388dec9f0329d4e3`.
The accepted preparation at `/private/tmp/t2-local/linux/native-corpora/staging`
was verified as **macOS 26.7 arm64**, despite its historical directory name.
Preflight checked its receipt, ordinary executable, all 334 recorded dependencies,
36 protected preparation sources, exact platform-adaptation patch, A2 receipt,
compiler binaries/versions and every exported candidate file. Preflight v1
incorrectly compared pre-adaptation source hashes with staged files; it failed
before any engine execution and is retained with its reason. V2 checked the
original hashes against original sources and the staged hashes against the
protected manifest, and passed. Its receipt is
`validation/release/build/macos-w-preflight-v2/receipt.json`, SHA-256
`2ab5e192bc8bfb69076663d161b6406ae3c909dc900398f944e15e26b2cdd188`.

The recorded tool selection was GCC/GFortran 16.2.0:
`NATIVE_CXX=/opt/homebrew/bin/g++-16`,
`NATIVE_CC=/opt/homebrew/bin/gcc-16`,
`FC=/opt/homebrew/bin/gfortran-16`, and `CC=/usr/bin/clang`.
`CARGO_HOME=/Users/Eric.Dong/.cargo`, `LC_ALL=C`, `LANG=C` and
`PYTHONDONTWRITEBYTECODE=1` were explicit. Optional provider variables and Rust
flag overrides were absent. No optional Cargo features were selected. Native
binary128, Rust Rug precision, tolerances, resources and case ordering were
unchanged. The isolated launch receipt records the complete command/environment:

```sh
python3.13 validation/release/build/macos-w-candidate-v1/source/validation/release/corpus_compare.py \
  --out /private/tmp/alw16a \
  --candidate /private/tmp/tempo2-al-rendering-0116-20261101/validation/release/build/macos-w-candidate-v1/source \
  --reference-root /private/tmp/t2-local/linux/native-corpora/staging \
  --target /private/tmp/tempo2-al-rendering-0116-20261101/validation/release/build/macos-w-target-v1 \
  --profile release --stages terrestrial_resources
```

Execution ran from this isolated worktree, 11:51:18–11:59:07 UTC. The fresh
workspace `/private/tmp/alw16a/wxxxxxxxxxxxxxxxxxxxxxxxxxxxxx` is exactly 50 path
bytes, matching the retained preparation's pathname length. Compilation used
the new dedicated target sequentially. The Rust build completed with
`cargo test --lib --locked --offline --no-run --message-format=json --release`;
the target's GMP/MPFR dependencies were freshly built. The retained preparation,
previous targets and original evidence were not overwritten.

| Fresh gate | Result |
| --- | --- |
| Native controls | Pass: 29 controls (26 rejections and three positives), plus all 13 separately compiled behavioral mutations detected. |
| Corrected native producer | Pass: separate executable and 130 cases under the existing 0053 correction. |
| W release corpus | Pass: all 130 cases; 127 strict passes and three additional paired rendering proofs. |
| W downstream controls | Pass: 27 rejections and one fresh challenged positive execution. |
| Preservation and receipt audit | Pass: 732 protected files, all 130 case proof hashes, source/compiled dependency identities, raw products, exact staged adaptations and 22 original cap inputs. |

All three stage commands exited zero. No downstream step was skipped. The three
new paired receipts preserve their strict failures and complete raw/native/Rust
renderings; their emission lengths remain 336/282, 344/240 and 344/256 bytes.

| Fresh paired case | `diagnostic-rendering.json` SHA-256 |
| --- | --- |
| `alias_bounded_tokens` | `35afe37c1cae7acf81789815fd86d40ba2da44b79bc79ec9deba270217f54e75` |
| `observatory_exit_read_alias` | `20240708d8c3da673f266b8bf9767c3bd4ac224397327f76efed923bf979a3e3` |
| `observatory_missing_alias` | `5fa0b9147788434d704865c0c5c48f68d38b8acdea48ab6a3715f3bf183010d6` |

The read-only audit is retained under
`validation/release/build/macos-w-campaign-v1/`: `launch.json`, complete launch
stdout/stderr, `verify.py`, `verification.json` and audit stdout/stderr. The final
verification binds 4,805 input paths and records every case's proof hash and
strict/paired disposition. It rechecks the clean export, native preparation,
corrected producer dependencies, Rust compiler/executable/source identities,
native and Rust raw case products, original cap evidence, and the six main files
listed in the saved v6 preservation receipt. All remain unchanged. W's freeze
explicitly protects the copied verifier; its hash is the same accepted
`cb83f64c6714ed7322a596c202848814a72e00e858a548966353334aebda909e`.
The 33 staged file adaptations are retained as exact before/after hashes and a
patch. The 51 accepted saved-data outcomes were reused; no comparator code changed
and no new defect required repeating those controls.

Fresh evidence identities (all SHA-256):

| Artifact | Identity |
| --- | --- |
| `macos-w-campaign-v1/verification.json` | `3838e3052b5e844a393d2577589c26ebe9d9ace4e6369f0ba788c12595828b96` |
| `macos-w-campaign-v1/verify.py` | `658eaa1af7a4fe5df52e1e903ff8c8135d09187c3c6cb76f3e1dc8d48c25bbaa` |
| `/private/tmp/alw16a/receipt.json` | `b88669f0e8e5acc69732303a5b154d052881c0a33ffb3d9d5d1a0e8a328b8739` |
| `/private/tmp/alw16a/adaptations.patch` | `92e598c540756d6b34903962257a7fd06031d9f967e3c11911a492eed07b12e2` |
| Workspace `al-source-adaptations.json` | `ba486fdb970ad201ec6004fe476fcb3b8425901e05407006e27ea437dfcd20c6` |
| W `build/freeze.json` | `47d892cc119122557a2d2c9b8f05082d8752c135ca19e6e54f33cb2ef4eaa058` |
| Native `outputs/controls.json` | `4206bd6f17e346e3edd2baeb54a6be2145e2cf11045341f9f70c2f4ddaa4cbde` |
| Native `outputs/mutations.json.gz` | `a9064210ef73f823002dbca07c8e2bbb57a3e18f79fbf985a8ffa3f908427b29` |
| Corrected native executable | `7301391b7998db339516761c971ed95a954affb06b1d72744483f594c9ff50a3` |
| Rust release executable | `aaaeae1a0a8ea85ac4ee011cc2edc2200bec3236a3a1272036e0670e3ec5b2c9` |
| `corrected_native/reference.json.gz` | `11b0d67775e1a7abb81d56be2b6c8fdeff4b670b26295b40a88b3b2a9984d083` |
| `corrected_native/receipts.json.gz` | `de81954bda8b838551b731cf482c01f406aef5c9461863919335fc2336fb6b20` |
| `corrected_native/contract.json` | `099d876b8582fa897e2e73a589ebe57f8cd09d73efd175b491ad97ef14dca693` |
| W `release/direct.json` | `129c4bede485162e40a7b5d350f6f506f339278feb35a9885e4dedea03b0a133` |
| W `release/controls.json` | `92a30dd794e78bb8ebe87b0cb569853e5a2d7b85dd24c11678e4294f85e69c2e` |

This stage changes only this report. The implementation remains `25bd81f`;
the complete isolated branch still has only the four originally owned files.
These results establish the bounded macOS comparator contract, not broader
scientific validation or migration acceptance.

## Remaining execution boundary

Fresh macOS W/native/downstream execution is complete. No Linux replay,
integration into main, publication or workflow action was performed. The
additional proof is limited to the reviewed redirected,
single-message source/event shape and requires its independently bound evidence.
Unsupported histories retain strict comparison. Other formatting modes, ambiguous
histories or changed source profiles cannot gain acceptance through this proof
without further review. All original scientific gates and
failures, production precision/resources and deferred covariance work remain.
Central must review this isolated result and the exact fourth Linux result before
releasing integration or further execution. Work stops at this handoff.
