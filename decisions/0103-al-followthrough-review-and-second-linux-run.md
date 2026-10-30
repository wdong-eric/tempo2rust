# AL follow-through review and second Linux run

Central review, 30 October 2026. The user resumed the active goal. Review of
`df587cd` and `4accd26e2d51b3dc805d9a1ae0a2be7195e171ea` supports the next full AL
qualification run under the existing 0101 authorization. **AL, Linux and full
migration acceptance remain pending.** The source policy in 0099 supersedes the
older goal text's unconditional preservation of bugs; precision, scientific
assumptions, output conventions and tolerances remain unchanged.

## Reviewed candidate and focused verification

Relative to first-run b59a840, the candidate changes 35 authored files. Its only
changes under Rust source/native/include directories are the two test modules
`fitting_tests.rs` and `predictor_tests.rs`. Production scientific bodies are
unchanged. No generated validation JSON is added. The largest changed file is
HANDOFF.md, 105,757 bytes. The frozen source export and review diff are retained
under `decisions/evidence/release-qualification/build/second-linux-4accd26-review-20261030/`.

The ELF link changes explicitly supply libm for direct mathematical calls and
libdl for ELF clients. Static Rust link requirements still come from rustc.
Failed libtest summaries now count passing and failing executions while retaining
the nonzero exit and the fact that later targets were not completed.

The square-fit assertion follows `TKfit.C:461`'s runtime sqrt(0/0) result, checks
its NaN classification and preserves its sign through error transport. Separate
positive/negative NaN serialization checks remain exact. The T1 assertion follows
`t1polyco.c:106`'s eighteen significant decimal digits, comparing the actual
written tokens at native predictor precision. It preserves D-exponent and invalid
count checks. Neither revision changes production formatting or relaxes a
scientific comparator.

Central exported the exact Git candidate into a fresh temporary source tree.
Eleven harness controls and both changed Rust tests passed there. A separately
compiled C witness measured ordinary long double at 53 bits on this macOS host,
observed positive runtime NaN and confirmed the three written decimal tokens.
This independent check used debug Rust tests; AL's separately bound receipts
include both profiles, Clippy, formatting, provider and C interface checks.
Actual Linux results remain required.

## Expanded qualification and preserved diagnostics

The matrix now reconstructs all thirteen early native corpora and executes
current Rust comparisons, the complete AK eleven-case worker campaign, 108
matched timing measurements, twelve instrumented/uninstrumented product controls
and the requirement/source evidence ledger. The ledger retains 26 requirement
families and 93 source responsibilities with unhit branches and missing evidence
explicit. Successful ledger construction does not establish branch or numerical
acceptance. The native stage's coverage failure cannot hide independent ordinary
corpus comparisons.

The selected local evidence includes separate corrected-native lifecycle and
observer proofs using already documented corrections. Raw histories remain
available. The bounded Q telescope inputs follow 0072, while the 501/1000 Rust
rejections remain checked. No preceding runner or accepted output was rewritten.

Three diagnostic interventions preserve their original gates: caller-local MPFR
cache cleanup, a separate Lomb copy with nine explicit fused sites replaced by
products/sums, and observed/native-double event temporaries. Their results measure
attribution and observer neutrality; they do not change production code or accept
the counterfactual arithmetic. Original leak, contraction and timing failures
remain in the next matrix. The full previously blocked performance, M2 and
predictor checks also remain required.

## Independent evidence verification

Central verified all **118,917 selected file entries** across 45 archive selections
(44 nonempty and the explicitly labeled empty attempt), whose compressed archives
total **4,119,169,211 bytes**. It checked each archive digest, each internal
manifest and every member's length/hash. It also checked all **1,300 files /
13,239,796 bytes** in the focused Linux-follow-through selections and all nineteen
report-bound receipt identities. These checks validate retained evidence, not a
new execution of the complete scientific corpus.

Nine cited receipts are currently available through their verified archives,
while their original working paths are absent or empty. The central review
records that location explicitly. The initial path-only audit stopped on a
missing receipt and is preserved; it was then extended to resolve the exact
verified archive member. This changes evidence lookup, not the receipt bytes.
The report's statement that all original run files remain in place should be
read against this current archive-based preservation observation.

The archive verifier is `decisions/evidence/release-qualification/central-followthrough-review.py`;
the exact executed copy, focused command receipt and stopped preparation/audit
attempts remain in the ignored review packet. An initial export attempt used a
Python lacking tarfile's data filter; the successful export used Python 3.13.
Neither stopped preparation executed scientific code.

| Evidence | SHA-256 |
| --- | --- |
| Frozen Git source archive | `5f52ff316f85345190bc71a94969dd5288c2fa3d15b5421e0a51aec485542764` |
| Publication/review diff | `a89c0aed97d35930c22ec3989d2b64a60c0ab00d73a8635f070d3d2232c875cd` |
| Reviewed workflow | `31a185d92e8e62bd7cd93cb0e9dc1d72fccbfd833cb5af36cc64a3b32e786d2d` |
| Independent archive/receipt review | `4a7f0769f851385e8c57fd8297991e27d71903dff6881c5ba158adfaa2520821` |
| Independent focused-check receipt | `cfa5a3aa52801b91416dcce49383040a19d36e59572d7247d1649705586776cb` |

AL's cross-host ledger also clarifies the first Linux fit aggregate: 1,862 means
**1,857 numerical mismatches plus five execution failures**, not 1,862 numerical
mismatches. Decision 0102's summary is corrected accordingly; raw evidence is
unchanged. New Linux fit/output identities remain open, even when aggregate
counts are lower than macOS.

## Public identity and actual dispatch

At action-time inspection, public main already pointed to exact reviewed
4accd26. Central made no push and does not attribute that prior update. Only the
first b59 run existed. Central revalidated both facts and dispatched once with
the full 4accd26 SHA, retaining the command/response record.

GitHub created [run 41056090173](https://github.com/wdong-eric/tempo2rust/actions/runs/41056090173)
at 2026-10-30 12:32:19 UTC. Job `124063879789` passed checkout and tooling
provisioning and entered the matrix at the initial live inspection. The workflow retains the selected Ubuntu x86_64 host,
explicit tools, read-only permissions and thirty-day evidence retention. Its
limit is now 180 minutes, and native gcov is explicitly matched to GCC 13.
This is a live qualification run, not an acceptance result. Reuse its existing
handle when observation times out; never dispatch a duplicate for a polling timeout.

AL retains implementation ownership. Central will inspect actual Linux ABI,
scientific, cache and expanded-corpus evidence before issuing further corrections
or accepting a milestone. No new user decision is required for this already
approved rerun. Optimization remains deferred until after AL; full source,
interface, numerical and platform scope remains intact.


## Evidence-location correction reviewed

AL corrected the report and retrieval guidance in documentation-only commit
`e484f28eef8f3e29fc653b4df7a34c69e0a2d2ea`. Central checked all nine corpus
archive/member addresses against the bound receipt hashes and the previously
verified archive index, and verified all eight focused receipt members in their
separate archive. Receipt and archive identities are unchanged. This closes the
stale working-directory retention statement; no scientific execution was repeated.

The bounded review is `documentation-review.json` in the ignored second-run
packet, SHA-256 `af98c3f7b027f5062c90eedb40b2c3b39f16af9dbd63305ec69e7ce5c70cc32e`.
Run 41056090173 continues at the exact reviewed 4accd26 candidate; this later
location-only documentation commit does not replace its source identity or
establish Linux/AL acceptance.
