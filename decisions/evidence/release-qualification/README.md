# Central AL review evidence

30 October 2026. This bounded review verifies the delivered evidence and closes
two demonstrated CI evidence defects. It does not accept AL, Linux qualification,
any inherited numerical gate or the full migration. Decision 0100 records the
checkpoint; decision 0099 keeps allocation profiling after AL.

## Delivered identities and focused controls

`central-delivery-audit.py` is the exact executed recipe. It reads a fresh frozen
depth-one checkout of `c14b1ee6fb703484036345c00453ba27c66c19df` and the original
local AL evidence. Its input pointer `/tmp/tempo2-al-shallow-review-current`
identifies the earlier original-defect reproduction checkout; to repeat it,
create that checkout at the exact revision and write the new temporary path to
that pointer. The full fetch/check commands are retained in the original shallow
summary. Use a new output directory; historical receipts are read-only.

The review verifies the report's receipt hashes, all **9,985 selected evidence
files / 291,065,141 bytes**, committed Rust/release/workflow Git contents, the
unchanged scientific Rust bodies and both reference pins. It reruns the frozen
four comparator controls and the package audit of both existing private prefixes:
**2,463 file checks, zero mismatches**. There is no fresh installation, Linux
execution, benchmark or new scientific campaign in this check.

The aggregate belongs to `ca49531f5eb0b21362f585887f7278c269d4f7be`, not a newly
executed c14 aggregate: 58 stages, 52 passing, six failing, zero blocked. The
failures are `m2-comparison`, `fits`, `boundaries`, `ddh`, `predictors` and `output`.
Later separately executed checks stay separately identified in the owner report.
The 208 current runtime files and historical overlay membership/hashes pass.
The pinned current tree has no `clock/gps2utc.clk`; the historical overlay adds
that one file. Current/historical DDH results therefore remain separate.

The first attempt stopped at an overly strict clean-working-tree assertion:
canonical TEMPO2 contains five untracked Finder `.DS_Store` files, with no tracked
change. The revised recipe records that metadata explicitly, preserves it and
checks the pinned Git inputs actually used in construction. That stopped attempt
is retained under `build/initial-metadata-stop-20261030/`; it is not a scientific
failure or a claim that the entire native working tree is clean. Independent A2
remains clean at `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`; native HEAD remains
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`.

## Shallow-checkout defect and correction

A fresh c14 depth-one checkout lacks baseline
`6acaf53d2e13291eb6d0b0988faf7dee7d33a635`. The original audit returned success
while all 155 baseline reads failed and every Rust body was labeled changed.
This is unavailable evidence misclassified as source change; it does not
invalidate the earlier full-history macOS audits.

AL corrected it in `8d9c11afdf56160d909952aa23f98d9478db71a9`. The workflow fetches
only the required baseline snapshot. The audit fails before classification if
the baseline is unavailable, requires existing blob reads to succeed, and
separately identifies actual additions, removals and modifications.

`central-shallow-fixed.py` is the exact independently executed correction check.
It creates a real depth-one checkout of that commit: the missing-baseline case
exits 1 with a failure receipt and no changed-body list. After the workflow's
`git fetch --no-tags --depth=1 origin BASELINE`, HEAD is unchanged, the checkout
remains shallow and the complete audit passes with 155 unchanged bodies and no
changed/added/removed files. The authored regression also passes, including real
added, removed and modified controls. This closes the specific false-pass finding;
no scientific campaign is needed for that infrastructure-only correction.

## Retained artifacts

Byte-preserving selected copies live under ignored `build/` in this directory.
Their original paths and original receipt bytes are retained; each bundle has a
preservation manifest. Temporary checkout trees and their `.git` directories are
omitted because their exact revisions/fetch commands are retained. Generated JSON,
logs and archives are not committed. Only these authored recipes and explanations
are versioned.

| Receipt | SHA-256 |
| --- | --- |
| Original shallow reproduction summary | `bc919c38eae4d80bd094be1e4293f38bcf193e1956611cb05ac704f7ae2c804c` |
| Central delivery audit | `3851a03408395e0f40dff71a55119966e3a19188118a8e5e393897d06dd0538f` |
| Delivery audit commands | `615534c791eb036e276a085e6381fa0899aab6e1758f9393b2f3401e9bfe4097` |
| Corrected shallow summary | `08408cd50ab799c63f066c6b7415fa4f83df43daccfd6eb2d9c8a5bfd83c3ca3` |
| Corrected shallow commands | `c919c7f3924cf08c3993256a5d380c6e30832a8b7221bced0c4c615c9359608e` |
| Executed delivery recipe | `e6c543faae348aef097490ed7a13d72e4c5972db25ac1e866569bd5cc19dbea6` |
| Executed correction recipe | `83e726537e76d6452d8da74b2df538f9a12d5b61deab38cddee9c4fa403c998c` |

AL retains Rust/release/workflow ownership and is continuing portable early native
corpora, the measured worker/performance campaign and source-to-evidence closure.
The selected Linux run still requires a concrete reviewed candidate and authorized
publication/dispatch. The already public c14 revision does not establish either
permission to dispatch a newer run or Linux success. Central has not pushed or
dispatched during this review.

## Compressed evidence retention, 30 October follow-through

The collector at `8d9c11a` accepts ordinary `.json` but inspects only the final
suffix; real `reference.json.gz`, `coverage_receipts.json.gz` and gcov JSON files
therefore fall through its whitelist. `central-collector-original.py` reproduces
this using the actual 3,541,285-byte observer reference under a representative
Linux staging path. The collector exits 0 and retains the control receipt but
omits the compressed reference. This would discard required evidence when the
disposable Linux runner ends; it is not a numerical difference.

AL corrects that predicate in `1c10ae07ebbc31259f6695a95d8bd09a8a23eecc`, admitting
`.json.gz` and `.jsonl.gz` after the existing source/cache/runtime/install/symlink
exclusions. `central-collector-fixed.py` freezes the three relevant source files
from that commit and collects the exact same input. Both files are retained;
the compressed reference's bytes and SHA-256 are unchanged. The authored
inclusion/exclusion control passes for references, coverage receipts, gcov JSON,
JSONL events and the existing AE exception, while rejecting excluded trees and
arbitrary compressed archives/binaries. Central closes this specific finding;
no scientific rerun or Linux acceptance follows.

The original script uses the exact 8d9c11a checkout identified by
`/tmp/tempo2-al-shallow-fixed-current`; the correction script reads the original
input via `/tmp/tempo2-al-collector-review-current`. Each recipe creates a fresh
output and records those inputs. Ignored byte-preserving bundles are
`build/collector-original-20261030/` and `build/collector-fixed-20261030/`, including
the real compressed input, output, logs, source snapshots and manifests.

| Receipt | SHA-256 |
| --- | --- |
| Original collector reproduction | `6f22258b0ffc1ae5e7319be4f3102f0405cbfa1624e4265bf599e12b449e27ec` |
| Corrected independent check | `b6fea7322607adc1c77cff4c7ef2ce8dedcc2ba8b994bff0e951b1e1e9574b4f` |
| Corrected selected manifest | `50e51180a254b56804f6efef19a6dd08283fc47cc462e90ec2ddfa099cf0152d` |
| Preserved real reference | `e810f70fa52c778059413cc50ea24d5df96cfebd7b345aaa1efb4531b19b0a85` |

Historical selections remain unchanged. Future AL selections use the corrected
collector; compressed generated evidence remains ignored by Git.

## Lifecycle localization and first authorized Linux run

Central read the retained `compare-v3` seeded-reinitialization failure without
rerunning either engine. `central-lifecycle-localization.py` checks six events /
469,282 state records under the unchanged lifecycle projection. Exactly sixteen
records differ, covering `nchromx`, `nWhite_dm`, `AverageDMResiduals` and
`ne_sw_ifuncN`, including retained `old2` state: Rust zero versus raw-native seed
11. Every other compared state and diagnostic value is exact. These correspond
to existing documented initializer departures; the raw comparison remains failed.
This single-case localization is not a corrected-native proof or acceptance of
the complete lifecycle corpus. AL is separately constructing that proof.

The summary SHA-256 is
`5be5cb14ffc8e9d397a75a90e5796016e164cbf4c9755260a26cd8e387ea3df9`.
The exact executed recipe and all three inputs are retained under ignored
`build/lifecycle-localization-20261030/`, with original hashes. The raw events
are compressed losslessly; decompression was checked against their original
SHA-256. Historical outputs are unchanged and generated inputs are not committed.

Decision 0101 records the user's explicit authorization for AL publication and
qualification, including later centrally reviewed fixes and reruns in scope.
The frozen first candidate is b59a840, with its review/dispatch packet under
`build/first-linux-b59a840-20261030/`. Central observed it already public, made
no push and dispatched run 40903715685 once. The first Linux execution has since
completed; 0102 records its failed matrix and the correction brief. AL's local
follow-through remains ongoing. Neither this packet nor a workflow status
constitutes scientific or milestone acceptance.

## First Linux artifact review

Decision 0102 records completed run 40903715685 at b59a840: 60 stages, 45 passing,
12 failing and three blocked. `central-linux-run-audit.py` verifies the downloaded
GitHub archive digest and all 6,580 selected files / 267,082,716 bytes, then
summarizes the original stage receipts without rerunning or changing their gates.
The package audit passes 2,474 checks and the source audit records 155 unchanged
Rust bodies. Linux, AL and inherited scientific gates remain unaccepted.

The packet is ignored `build/linux-run-40903715685-review-20261030/`. It preserves
API identities, the exact archive, extracted evidence and the job log. The first
job-log retrieval was rejected by gh's terminal-escape filter; that failed attempt
is retained. The successful retry enabled raw escape sequences only while
redirecting bytes to `job.raw.log`; no job was repeated. Archive retrieval and all
hash checks succeeded independently.

Replay (use a fresh `--out` filename):

```sh
python3 decisions/evidence/release-qualification/central-linux-run-audit.py \
  decisions/evidence/release-qualification/build/linux-run-40903715685-review-20261030 \
  --out /tmp/tempo2-first-linux-review-new.json
```

Archive SHA-256: `df69c6ef346ae5ddef5562c137d6f89c8947ae5ce2e4922468f12c8a87a0c20c`.
Selected manifest: `c99c608a2f7d79d4df5ba08ea892d7748c2505d1176891951f0fc934be6cbe04`.
Central review: `c866eef82c916ba11dbb67519a5c4271c7d9cb5c9d6747682aceb1717e613baf`.
The raw failures and blocked stages are retained for AL's corrections and the
next centrally reviewed, already authorized qualification run.


## Expanded AL candidate and second Linux run

Decision 0103 reviews df587cd / 4accd26 and dispatches run 41056090173 under 0101.
The ignored packet is `build/second-linux-4accd26-review-20261030/`: frozen source
identity/diff/workflow, independent focused checks, archive verification and the
exact dispatch response. The public branch already matched the candidate;
central made no push. The original b59 execution remains unchanged.

`central-followthrough-review.py` is the exact executed archive verifier. It uses
`/tmp/tempo2-al-next-review-current` to locate the review output; repeating the
recipe requires pointing that file to a fresh review directory while retaining
the original packet. It verifies 118,917 archive members and 1,300 focused files,
then resolves all nineteen report receipt hashes. Nine receipt paths now resolve
through preserved archives, not their empty/absent working files. The first
path-only failure is retained with the successful archive-based check.

The fresh frozen-source controls pass eleven harness tests, the two changed Rust
tests and a native ordinary-long-double/serialization witness on macOS. Their
receipt is `focused-checks/receipt.json`; commands and source hashes are included.
The archive-review SHA is
`4a7f0769f851385e8c57fd8297991e27d71903dff6881c5ba158adfaa2520821`;
the focused receipt is
`cfa5a3aa52801b91416dcce49383040a19d36e59572d7247d1649705586776cb`.
The expanded Linux result, numerical acceptance and full migration remain pending.


The subsequent evidence-location documentation correction is `e484f28`.
Central verified its nine corpus archive/member addresses and eight focused
archive receipt members. `documentation-review.json` in the second-run packet
records those exact locations and hashes; its SHA-256 is
`af98c3f7b027f5062c90eedb40b2c3b39f16af9dbd63305ec69e7ce5c70cc32e`.
This closes the location finding without repeating scientific checks or changing
the running 4accd26 Linux candidate.


## Event source-type diagnostic

[event-type-boundaries.md](event-type-boundaries.md) records the exact local
compile-time probe and the inherited barycentric fixture frequency scale. It
identifies the mixed ordinary-long-double/binary128 expression boundaries and
explains why tiny rounding changes can reach nanoseconds in this fixture. It
is not a causal finding, an approved precision change or a Linux qualification
result; the existing second Linux run supplies the pending diagnostic evidence.


## Second Linux artifact and correction review

Decision 0104 records the actual 4accd26 result: 54/66 top-level stages pass,
twelve fail, none are blocked. The early native corpus did not execute; the
history fetch failed and its comparator then encountered a missing stages field.
The passed closure stage is evidence accounting, not corpus completion.

`central-second-linux-review.py` is the exact offline review. Run it with the
ignored packet `build/linux-run-41056090173-review-20261030/` and a fresh `--out`
filename. It checks the GitHub archive identity, every one of the 11,242 selected
files / 617,149,255 bytes, and the result/diagnostic identities. The central review
SHA-256 is `f782f1de5cbe9ac4234fc27096bd442e24071fbe438ea6b1dabf1af01568a378`.
The complete successful job log is `job.complete.raw.log`; earlier command/live
log retrieval failures remain distinct and did not restart the qualification.

The Lomb and event counterfactuals now isolate the Linux source-profile
mismatches. The cache intervention still lacks a complete original stdout
control. M2 and predictor generation ran for the first time on Linux, exposing
both genuine open values and two source-observation defects detailed in 0104.
The original artifact and every failing gate remain immutable. AL has further
implementation/verification work; this review is not milestone acceptance.
