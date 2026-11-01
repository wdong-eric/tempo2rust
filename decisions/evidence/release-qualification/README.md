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


## First correction reviewed for the third Linux candidate

Central reviewed `cecee60899611f52ba41975b082d4951e0c66195` on 31 October.
Two history/publication tests passed from an isolated archive of that commit.
The offline evidence review verified seven exact shallow Git trees, absence of
automatic maintenance children in their corrected fetch traces, the unchanged
actual Linux failed-preparation receipt, and fourteen explicit unavailable
comparisons. It binds 102 retained files. No native corpus or new Linux run was
executed by these checks. The historical metadata race remains a plausible
explanation, not a proved cause. Linux must still confirm staging and execute
the complete native/Rust corpora.

The exact executed offline recipe is `central-history-review.py`. Its original
unit-test and review packet is
`/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-central-history-cecee60-xuyonfrm/`;
a byte-identical copy is retained in ignored
`build/history-cecee60-review-20261031/`. `copied-artifacts.json` maps original
and saved paths without relabelling the executed commands. The central evidence
review SHA-256 is
`1ab565e15df00dc7bb6f06d21340ae064c5b77434f32f34fd7e7a83de9488120`.
The recipe refuses to overwrite that original review receipt.

This component is suitable for the next reviewed candidate. AL is still working
on the other 0104 corrections; no whole candidate, scientific milestone or
platform qualification is accepted here.


## Observation and cache controls reviewed

Central reviewed observation commit `0104447` and cache-diagnostic commit
`ca60751` on 31 October. The independently frozen observer files all match the
committed source. Three focused tests and eighteen macOS native writer/parser
rows pass, including signed-zero preservation and wrong-sign rejection.

`central-observer-cache-review.py` is the exact executed offline review. It
verifies 960 retained files, all 31 nonempty raw/signed M2 JSON products, and
replays all 21 saved macOS predictor serialization checks. The complete M2
comparison still reports two SVD-order failures; the fourteen predictor cases
still report seven raw RMS failures. These gates are not waived. The first
review attempt incorrectly included historical native log files in the JSON
membership check; its script and failure are preserved. The corrected review
verifies every recorded file hash and separately compares exact JSON membership.

The cache review verifies the original library, explicit cleanup and guard-only
copy, twelve complete equal workflow blocks, all equal stored TSV products,
and the single changed source file. The proposed guard registers within the
existing C adapter panic boundary and frees only the calling thread's MPFR cache
at thread exit. This design is suitable for the Linux counterfactual in 0104;
production remains unchanged. macOS does not reproduce Linux's retained
544-byte/16-allocation leak, so no Linux leak closure is claimed. The next run
must first obtain complete flushed baseline/explicit-cleanup evidence, then
execute the one-file guard variant. Production promotion remains a central
review decision after that evidence.

Original successful review: `/private/tmp/tempo2-central-observer-cache-review-20261031-v2/`.
The failed attempt, successful review and focused native/unit controls are copied
byte-for-byte to ignored `build/observers-cache-review-20261031/`; its
`copied-artifacts.json` preserves original/saved addresses. Successful receipt
SHA-256: `5868a7f65e918dd21d39360ed7b209e749220c3679e971f588e740166c7b5c36`.
The recipe takes `--root`, `--focused` and a fresh `--out` directory.
The prior Linux artifact lacks the generated predictor `.dat` files; its full
source-token check cannot be replayed from that packet. The new collector retains
the named generated products for the next Linux review. Original failures remain
intact, and no scientific milestone or full candidate is accepted here.


## Third candidate reviewed, published and dispatched

Decision 0105 approves `a35104a` for Linux qualification under the existing 0101
authorization. Central verified all 8,037 selected evidence files / 66,411,856
bytes, all 157 production Rust source identities and the exact Git patches, then
passed seventeen harness controls from a frozen candidate. The workflow remains
unchanged. Saved-product review replays 209 event type/callback observations and
all nine Lomb profile/variant comparisons; all six macOS event/control/Gaussian
products and the amplitude-fit product retain their pre-repair scientific values.

The exact executed recipes are `central-third-candidate-review.py` and
`central-event-products-review.py`. Their original `/tmp` outputs and publication
records are preserved byte-for-byte in ignored
`build/third-linux-a35104a-review-20261031/`; `copied-artifacts.json` records
original/saved paths and hashes. Candidate review SHA-256 is
`d6f70ed3c734ad47836fe0dd1483cbe1f9eeed96eca979bec61f62cc456ab8a0`;
event/Lomb review SHA-256 is
`0d5ee761b933a54a5433645c1c343a9c08534fdf1e78599988fad81690babefb`.
These exact recipes refuse to overwrite their original output directories.

Central fast-forwarded public main from 4accd26 to the exact reviewed a35104a at
14:10:15 UTC on 30 October and dispatched run 41069548581 once at 14:10:42 UTC.
The exact-head checkout and provisioning passed; the matrix remained active at
14:16 UTC. The recorded run is the one to observe; no duplicate dispatch is
needed when a status/log request times out. The full native corpora, Linux
arithmetic, revised observations and complete cache controls await actual
results. The guard remains diagnostic-only. AL/M2/full migration acceptance
remain pending, and allocation optimization remains deferred.

## Truthful Rust terminal presentation review, 31 October

`central-presentation-review.py` is the exact executed review for production
commit `801bc94139cca90d6072234b09f5a1bcec5e4b05`. It verifies the Git archive
against the delivered clean source export, all 262 installed files, the retained
before/after evidence and its comparison implementation. It independently
replays nine report/JSON comparisons, reruns four source-attribution corruption
controls and executes the installed robust-H report once in a fresh directory.
All those checks pass. This is a macOS presentation review; scientific equality
to native TEMPO2 and Linux acceptance are separate obligations.

The original temporary review and its tuple-versus-JSON-array assertion stop
are preserved alongside the corrected successful execution under ignored
`build/presentation-801bc94-review-20261031/`. Its copy manifest binds 158 files
and 36,214,932 bytes; generated archives, JSON and logs remain untracked. The
verified receipt is
`0459603a9fdbbe88a7db1bd3e707361534b76b055a90fdb1b618020f32969ff4`,
the recipe is
`940e5e87a21265369c98ec6c5f13f069722ca35ccc3f3ef53dd40f97c606ed12`,
and the preservation manifest is
`7ecf01fe2a55ff4497e5d5bc12f8c2bf3de6b4e5dfb2554b6fac6a68f464c13e`.
The recipe intentionally retains its exact original temporary destination and
refuses to overwrite it. Decision 0107 records the review boundary and remaining
final AL harness/report binding.

## Final presentation delivery accepted, 1 November

`central-presentation-final-review.py` independently verifies the delivered
b093586 archive and source binding, replays nine genuine report/JSON comparisons,
rejects all three original internal-tag substitutions and freshly passes five
attribution control families. All 1,802 selected files / 25,682,677 bytes and
1,803 archive members match. The review receipt is
`2763906694e9c3c8eda72796b6a0f0aa1b5b4c8bcb8eb60a9d2f19839f7d9e67`.
The exact recipe is
`6ce142d9f20e5df2099afa903fe029358036a6de0b4958a953703e5a1f7a8129`.

Ignored `build/presentation-final-review-20261101/` retains the review artifacts,
frozen candidate archive and AL evidence archive, with original-path hashes.
The disposable extracted Git tree is omitted; its exact archive is retained.
Decision 0108 accepts this focused macOS presentation work, preserving the
separate clean-export missing-fixture failure/preparation/rerun and the unresolved
integrated scientific and Linux qualification obligations.


## Third Linux result recovered and reviewed, 1 November

Run 41069548581 completed on exact a35104a with 54 passing / thirteen failing
matrix stages. No duplicate run was dispatched. `central-third-linux-collect.py`
is the original collector, whose artifact download timed out at 595,591,168
bytes. `central-third-linux-recover.py` resumed the same artifact with curl;
its exclusive extraction then stopped at the first case-distinct filename pair
on macOS. Both original failures and the partial extraction are retained.

`central-third-linux-verify-archive.py` verified all 30,316 files / 6,605,117,390
bytes by exact ZIP member names, preserving twelve case-distinct groups. The
774,024,809-byte archive hash is
`cdb2fca1cf14f4bd5c9c025c4aa2f9185bdf5c289a60e1d4a4456bff4831cf0a`.
Its verification receipt is
`bef1530dd2ea464e3c67d7749c339ecd57022058f556ecc5a021da03c54e24b6`.
The packet is ignored `build/linux-run-41069548581-review-20261030/`; use
`recovery-20261101/artifact.zip`, not its incomplete extracted directory.
The three collection scripts preserve their exact original paths and refuse to
overwrite them. Replaying them is unnecessary for the completed review.

`central-third-linux-review.py` binds 157 frozen source files and independently
rechecks 180 selected artifact members. Both cache profiles retain twelve equal
complete workflow transcripts and twelve equal stored TSV files across original,
explicit cleanup and library-owned cleanup variants. The original 544-byte /
sixteen-allocation leak remains recorded; both interventions succeed. All fifty
M2 records replay exactly. A diagnostic-only recognized-set extension for -NaN
removes eleven observation/status errors, leaves four numerical/order failures,
and passes seven sign-sensitive controls. No historical checker is changed.
The review also localizes the TIM failure to a 152-byte fixture path copied into
its 100-byte caller field before readTimfile runs.

The selected review receipt is
`46640be739a8fde1b4a22ac6c7d71449a4a30a5fccaecd40575e31d17e32ce19`,
and its executed recipe is
`5892ce432841b1fdff832cecd37b062014c2d128769b87814a45c5bf959d3e72`.
Original outputs are `/tmp/tempo2-central-third-linux-review-20261101/`, copied
byte-for-byte into the packet's `central-review-20261101/`. The selected review
accepts --root, --packet, --frozen and a fresh --out directory. Decision 0109
releases bounded fixes and the measured cache promotion; full scientific and
Linux acceptance remain pending. Generated JSON, logs, binaries and archives
remain ignored and are not committed.


## Clock interpolation platform witness, 1 November

`central-clock-contraction-review.py` verifies the third-run native fixture,
five finite query results, compiler/source identities and retained Rust failure.
Separate binary64 operations agree with all five Linux results. Exact rational
single-rounding and `math.fma` agree with each other and with 0027's accepted
macOS result; they differ from Linux at the duplicate knot's lower neighbor.
Decision 0110 releases the corresponding one-site platform correction without
changing scientific gates. No new Linux executable or corrected Rust code was
run by this review.

The receipt is `1df02b2601fc8574959a16e469446411d68f0cdccb24b87e6eea298ba483e89f`;
the executed recipe is `a43349ca41787b4accaefac3577a935fc18c801c59337094e181fa281f6af91b`.
Original `/tmp/tempo2-central-clock-contraction-review-20261101/` outputs are
copied byte-for-byte into ignored `build/clock-contraction-review-20261101/`.
The recipe accepts --root, --archive and a fresh --out. Original 0027 evidence,
historical runners and generated JSON remain unchanged/uncommitted.


## Sidereal-time contraction witness and source navigation, 1 November

`central-observer-contraction-review.py` binds the pinned LMST source map and
frozen Rust body to 45 saved Linux queries / ninety scalar outputs. Native
ordinary and observed results agree. Source separate arithmetic matches every
native output; the explicit fused tree matches every recorded Rust output.
Ten sidereal-time values differ. Each fused witness is cross-checked with exact
rational single-rounding. This is a saved-product diagnosis, not corrected Rust
execution or a new Linux experiment. Decision 0111 releases the bounded LMST
correction and a coordinated diagnostic review of remaining contraction sites.
The lexical inventory includes tests/comments and already qualified helpers;
its 130 lines / 23 files do not count demonstrated defects or branch coverage.

The final receipt is `bc23955f805339f7233a0343339243deec0528bb2d6fb4921a5fb5351a26ea78`;
the recipe is `9ec87550c21d284daaf278c12ba2ff015705ee1a5fed332486f94e867f1b5491`.
Ignored `build/observer-contraction-review-20261101/` retains both original
`/tmp/tempo2-central-observer-contraction-review-20261101/` and final `-v2/`
outputs byte-for-byte. The refinement binds the Git blob/frozen function map
instead of merely recording the working source's expanded CVS-identifier hash;
the numerical witness is unchanged. The recipe takes --root, --frozen, --archive
and a fresh --out. Generated products remain uncommitted.


## First 0109 adapter components reviewed, 1 November

Central reviewed `993ec81` (0110's clock-site platform branch) and `92862b0`
(portable preparation, signed-NaN successor and historical diagnostic adapters).
The clock change preserves all preceding arithmetic and the macOS fused result;
actual corrected Linux execution remains pending. The Rust diagnostic bridge
lives under `#[cfg(test)]`; it introduces no production Python dependency.
The wrappers retain the immutable historical comparators, bind named staging
changes, separate cold test compilation from bounded API execution, and keep
finite/scientific comparisons in their original owners.

`central-0109-adapter-review.py` freezes exact 92862b0 via Git archive and runs
all release-harness unit tests from that export. All 28 pass, including the five
new native-metadata control families and signed-NaN class controls. This does
not replace actual native preparation, installed Rust workflows or Linux
execution, and does not accept the whole AL candidate.

The archive SHA-256 is `f6d8e3dc230b77fefe6939b046bb552bb14c21f0f19fca6a7973db2302d9a306`.
The exact executed recipe is `50c32db1f130a149e809c807046dbee0b553c3011d8ff85980a2b2399295785e`.
The review receipt is `3b3e57a36c693100b5a24ef536e8950f1491c780feb4ed5f73c0b29a9b46d9ea`.
Original `/tmp/tempo2-central-0109-adapter-review-20261101/` files are retained
byte-for-byte in ignored `build/0109-adapter-review-20261101/`; the disposable
extracted source tree is omitted while its exact archive remains. Generated
JSON, logs and archives are not committed.


## Installed cache promotion reviewed, 1 November

Central independently reviewed both macOS installed C campaigns from exact
`a8ba6f15d2933b206f85ce70c99003b10f3f653d`, including `fd185ec`'s production
cache owner. `central-installed-cache-review.py` reconstructs the measured
private guard from the immutable third-run recipe. Its entire code matches the
production source after removing only whole-line comments and blank lines.
The production source hash is `e13d8c48ec14e71993ffaef3a6da85c6ec3ee25cda918e03f43cb5c186d2ef72`;
the retained Linux private-source hash remains `9cef7c936bb5e92bd5d991f125190ee24e4e7afccd821521cbee2a9683e8d91e`.
The nearby explanation correctly identifies a Rust/Rug integration lifetime,
its native predictor counterparts, the observed leak and local-only cleanup.

The review verifies all 2,030 exported source hashes against the exact Git
archive. For each default and combined installation it checks all 263 installed
files (257 source files bound to the candidate), all 136 campaign files and all
23 successful compile/link/execute records. An independent sequential parser
requires exactly twelve complete keyed workflows and twelve named storage
files. Their complete bytes agree across plain, ASan/UBSan, TSan and explicit
caller cleanup, and between default and combined builds. No guard is injected
into these campaigns. The sanitizers instrument C clients and the C shim;
Rust, MPFR and providers are not all instrumented.

The original premature default check (missing the unfinished installation's
receipt) and first combined build failure (missing explicit
`TEMPO2_ACCELERATE=1` provider selection) are retained and hashed. Successful
fresh checks/installations do not replace those records. This accepts the
bounded macOS installed-cache evidence, not AL or full Linux qualification;
actual corrected Linux installed campaigns remain required by 0109.

The executed recipe hash is `b93368865437eacfa347dc97551dcbfed6a37df4c1a9607d71ef3c4cea7bd8a3`;
the review receipt is `0893ab3a49f9450ab4bb9ee2d80dbf4172d34944bd18bb73cc11952fcfdcfcb8`.
Original `/tmp/tempo2-central-installed-cache-review-20261101/` files are copied
byte-for-byte into ignored `build/installed-cache-review-20261101/`. The recipe
accepts --root and a fresh --out. Generated receipts remain uncommitted.

Central also reviewed `75692fd`'s LMST source diff: the Linux branch restores
0111's native grouping while the accepted macOS fused tree, constants, casts
and final `fortran_mod` path are preserved. Its four real-API regression cases
include negative dates and longitude wrapping. AL's local macOS API execution
passes; the complete observer corpora and actual corrected Linux execution
remain separate requirements. This source review does not claim those broader
checks have passed.


## Source-profile preparation and clean harness review, 1 November

Central reviewed `dc8f6d11a9c70ab5fdcef9dda967c4badcaf67a7`'s diagnostic-only
spin/event/contraction additions and installed-output binding. The spin probe
retains original/native-observed equality, the separate f64 counterfactual,
signed callbacks and update-scale observations. The event trace preserves the
original accumulation statements and checks actual native/Rust observer
neutrality separately from the original residual/phase gates. Comparing wide
trace values at native 113 bits is diagnostic; it changes neither production
Rug128 nor scientific acceptance. The compiler/object campaign is preparation
for source-specific Linux contraction evidence, not a blanket fusion policy.
The output adapter now explicitly invokes the installed CLI and records its
hash, while preserving the historical workflow/checker and original failures.

Review found a real pre-bootstrap issue in the first new unit control: it read
`tempo2/formResiduals.C` before Actions had prepared native sources. A full Git
export plus the then-current Python sources reproduced two passing controls
and one FileNotFoundError. AL fixed the early control to use the exact two
statements as a unit fixture; the live diagnostic still checks them against
its prepared native source and tests observer neutrality. No native-source
absence is hidden or treated as a passed execution.

`central-frozen-harness-review.py` now exports exact dc8f6d1 with no native
checkout and runs all release-harness controls: all 31 pass. This closes the
reported bootstrap dependency within these controls, not AL/scientific/Linux
acceptance. The recipe accepts --root, --revision and a fresh --out, so future
changes can be checked against a named immutable candidate without editing it.

The source archive is `152fd4448841999f2c2e89be0c5716c7020d68947a78d5b433f08e622d362e29`;
the executed recipe is `53b8b4601f445e679abb946d31dc2f48a4af422f59a503e2bca63ee7d529bf3a`;
the receipt is `511d77bf3a91bbca71efc4d38435c3cca9c8ca44b965340a2386e30c4e1e6227`.
Original `/tmp/tempo2-central-frozen-harness-review-20261101/` files are retained
under ignored `build/frozen-harness-review-20261101/`, omitting the extracted
tree but retaining its exact archive. The original failing control, receipts
and logs are retained there under `pre-bootstrap-failure/`. An earlier central
scratch probe lacked committed Python dependencies; its import failure is
separately retained as `incomplete-central-probe/` and is not a project defect.
Generated JSON, logs and archives remain uncommitted.


## Fourth-run packet review, 1 November

Central independently verified AL's `validation/release/build/third-review-v1/`:
all 37 selected groups have exact file membership and every recorded hash/size
matches both the copied and original file. This covers 47,887 selected files /
7,139,104,046 bytes, including failed attempts and prefixed predictor products.
The original Linux receipt matches its already verified archive member and
preserves all 67 stages (54 passed, thirteen failed). No corrected Linux or
scientific acceptance is inferred from this packet review.

The ledger hash is `0ccefc241a9c321dbe960535411282268068c9d82970de049c49f8776e8c97d6`.
Its exact executed producer is `fe371f2e40372d650f86323226461dc4e163b0f68e74f7b469f4e79eafa3f92b`,
committed in `b95879c`. The ledger records collection-time Git HEAD `91309fd`;
that field alone is not a claim that the collection helper had no pending edit.
The helper's final evidence-group selection is bound by its exact source hash.
The production code candidate was separately exported cleanly at `91309fd`.

`central-fourth-evidence-review.py` takes --root, --packet and a fresh --out.
The review receipt is `be71d33fac43f38ab9249ba59c3aaa0ed0fcc38277e971560236f616c5564d30`;
the executed recipe is `f7edec314923d5075875fd2594914bdcaf36b360e1e520297f8fcb3c8fd6193c`.
Original `/tmp/tempo2-central-fourth-evidence-review-20261101/` files are copied
byte-for-byte into ignored `build/fourth-evidence-review-20261101/`. The committed
recipe corrects one explanatory comment; its parsed executable statements are
identical to the retained executed copy. Generated receipts remain uncommitted.
This packet review precedes central review of the final publication candidate.


## Fourth-run collection preparation, 1 November

`central-fourth-linux-collect.py` is bound to run `41294531402`, job
`124812485584`, attempt 1 and reviewed candidate `fecbc18af95196a16db2046306099b6345a50976`.
It reuses the third-run Range download and direct ZIP verification patterns.
Run it with Python 3.13 after the job completes. A still-live run returns a
waiting status without creating a collection directory or dispatching work.

The collector retains the terminal run/job/artifact identities, exact archive,
manifest and all file hashes/sizes under ignored
`build/linux-run-41294531402-review-20261101/`. Case-distinct Linux names remain
separate archive members; nothing is extracted onto the macOS filesystem.
It records the observed stage inventory against the planned 69 stages. Archive
identity and membership verification do not establish scientific or milestone
acceptance. Syntax checking and an actual invocation against the live run pass:
the invocation reports `in_progress`/`waiting` and creates no collection directory.
Completed-run collection and verification remain pending. No generated evidence
is committed.


## Fourth Linux result review, 1 November

The original collector completed successfully at 12:39:47 UTC. Run 41294531402
at fecbc18 finished all 69 stages: 58 passed, eleven failed, none blocked. The
exact 1088755847-byte ZIP has SHA-256
`f6bb6e05a5e40ba094efc4bb08f37ad8b435dfbb826c210470f71dc7ac48760e`.
All 50079 selected files / 8760551506 bytes and 207 case-distinct path groups
are verified. Keep the archive authoritative; do not extract it wholesale on macOS.

`central-fourth-linux-review.py FRESH_OUTPUT_DIRECTORY` reads that verified packet
without running producers. It reconstructs both installed cache campaigns, binds
source/install identities, reviews the actual contraction products and records
the complete failed-stage ledger. Final receipt SHA-256:
`43019b5c9972dbc8c25cc1c3f5918ffdd2fc102ac29d1b53e8f440a25e0c8cdd`.
The exact executed recipe, v3 receipt and original v1/v2 analysis attempts are
retained under ignored `build/fourth-linux-result-review-20261101/`. V2 exposed a
limitation in the review's debug-string parser, corrected in v3; no engine or
scientific fixture changed. Decision 0117 gives the scoped acceptance, remaining
failures and next AL release. Generated JSON/archive evidence is not committed.


## Independent early-corpus triage, 2 November local

While AL implements 0117, [the saved-data triage](fourth-corpus-triage.md)
classifies the 160 P records and tests the three repeated Z vector differences
against the source interpolation operands. `central-fourth-corpus-triage.py`
reproduces this bounded read-only analysis; it does not execute either engine or
release further production edits. Receipt SHA-256:
`8c211055ea591e0ea6c7baefaaad2021482c3807e2b380c34be454178453a664`.
Keep the typed deletion changes separate from continuous and nonfinite records.
The exact interpolation delta is a source-profile follow-up, not full acceptance.
Generated evidence is ignored under `build/fourth-corpus-triage-20261102/`.
