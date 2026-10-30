# AL release qualification handover

Latest scoped follow-through: [first Linux findings, harness/test corrections,
controlled diagnostics and remaining gates](RELEASE_LINUX_FOLLOWUP.md). The first
Linux run is failed; its original evidence remains unchanged. The fuller corpus
and worker handoff below still requires execution on the next Linux candidate.

Updated 30 October 2026. **Incomplete qualification; no migration acceptance.** The
clean macOS aggregate at `ca49531f5eb0b21362f585887f7278c269d4f7be`
completed all 58 stages: 52 passed, six retained scientific failures, none blocked.
Under 0101, the user has authorized AL qualification, including centrally
reviewed fixes and reruns. Central dispatched the initial Linux run
[40903715685](https://github.com/wdong-eric/tempo2rust/actions/runs/40903715685)
at candidate `b59a84024317653d92cfd757709b2beee3e766a3`. Central reports that the
matrix completed with failure while setup, evidence collection and upload passed;
its independent artifact review is in progress. That initial run excludes the
fuller corpus/worker follow-through below. A completed Linux qualification is not
yet established.
Central retains publication/dispatch coordination, numerical policy and acceptance.

The production build/install change is `d10c855` on central release
`6acaf53d2e13291eb6d0b0988faf7dee7d33a635`. Scientific Rust bodies remain unchanged.
New release recipes, examples and current documentation are in the subsequent
AL handover commit(s). The initial installed/source campaigns deliberately use a
clean archive of `d10c855`; they do not claim to contain later report documents.
The final aggregate-run section below distinguishes any later execution.

## Scope and actual identities

* Native source: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; independent A2
  construction provenance: `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`.
* macOS 26.7 build 25G229, Darwin 25.6.0 arm64; GCC/G++/GFortran 16.2,
  clang 21; rustc 1.97.1 (`8bab26f4f68e0e26f0bb7960be334d5b520ea452`),
  Cargo 1.97.1. Exact command/version/load outputs are retained in receipts.
* Rust Cargo.lock SHA-256:
  `5847bf7fbf5be0042ad8e714b1d43c86820dda9ee0188ed081013c4ef5d1a90f`.
  Rug 1.30.0, production 128 bits; default serial execution unchanged.
* Fresh A2 measures native timing binary128 with 113-bit significand. Predictor
  ordinary long double measures 53 bits, size/alignment 8/8 on this host.
  The Linux x87 64-bit branch remains a required actual measurement.
* CALCEPH 4.0.5 mirror revision
  `718e7945ccd27216b522243c3ff8aab63ce1d99d`; source archive SHA-256
  `b38f6ccc726bab51c5901bb74c655e13941753ca7c3d0054396a37b793040426`.
  Fresh static PIC build. INPOP17a data SHA-256
  `32517ec82bfb298d53baeba4065750dec82143fab79357e4487f84fb439bc49c`.
* Actual macOS Accelerate LP64 provider, measured integer width four bytes,
  Fortran default/C integers 32/32 with round-trip 42; matrix/provider checks
  pass. The actual loaded BLAS path is retained. SOFA retains 36 identical
  upstream routines and notices; Fortran is PIC with recursive locals.
* Historical DDH overlay adds only `gps2utc.clk` from
  `0759584ebf8315bbbe6a6d0b41c08ebbcfe527f2`, SHA-256
  `30727fdfaaec0f1775543804210d057ffe62076594bb8884d8f8ccafafb676be`.
  It is a separately selected runtime, not a new production default.

## Initial requirement-to-evidence ledger

All paths below are relative to ignored `validation/release/build/`. This table
records the initial component campaigns; the final committed aggregate and its
additional package/evidence checks are recorded below.

| Requirement | Actual macOS result | Evidence | Linux |
| --- | --- | --- | --- |
| Fresh pinned reference/resources | Pass; no prior objects reused | `macos-a2-v2`, `macos-provider-v1` | Unrun |
| Private install, core CLI, getPeriod, Rust source, static/shared C | Default/combined pass | `install-default-v1-build/install.json`, `install-combined-v1-build/install.json` | Unrun |
| C11/C++17 × static/shared | All four combinations pass in each install; all 64 symbols/layouts/storage/locale/pipe/recovery and rejection controls | `predictor-default-v1`, `predictor-combined-v1` | Unrun |
| Outside installed Rust consumers | 30 default and 66 combined client invocations pass, including concurrent complete workflows; 33 compiled source files each | `external-default-v1`, `external-combined-v1` | Unrun |
| Full applicable default suite | Debug/release each 111 tests pass across targets; library 96 pass/36 ignored. Nine explicitly enabled runtime tests additionally pass per profile. fmt and all-target Clippy pass | `suite-v2` | Unrun |
| Disabled, BLAS-only, LAPACK, CALCEPH-only, combined, QR-default | Frozen AI comparisons all pass; combined 59 cases/41,961 checks. Independent qr-default without LAPACK build/test also passes its declared disabled-provider behavior | `profiles-v1`, `paired-combined-v1` | Unrun |
| Lomb/QR contraction | All default quantities pass; every Lomb bin retained. Native Lomb default differs from off and equals fast. 32 QR cases and variant assembly retained | `contraction-v1` | Unrun |
| All 14 binaries/fixed timing/lifecycle | 130 comparisons pass | `timing-v1` | Unrun |
| General public/family fits | 72 cases execute; 2,478 numerical failures and five execution boundaries retained | `fits-v1` and its exported-candidate receipts | Unrun |
| M2 shared/public fitting | Two dependent-column raw-order diagnostics in each debug/release profile; four total remain failures | `m2-comparison-v2`, `m2-comparison-debug-v1` | Unrun |
| P corpus | 154 inherited differences reproduced, including 32 SIMUL; immutable measured macOS corpus explicitly reused | `preprocessing-v2` | Native reconstruction pending |
| 0049 utility boundaries | All 16 identical native-parsed inputs executed; four discrete mismatches reproduced through current public Rug128 functions | `boundaries-v1` | Unrun |
| Predictor generation/exchange | 14 cases/37,310 checks; seven raw polyco RMS failures retained | `predictor-science-v2` | Unrun |
| Core CLI/output | Same twelve failing groups; zero unexpected statuses/input changes | `output-v4` | Unrun |
| DDH fidelity and ideal | 401/433 fidelity failures; separate historical strict 1 ns pre/post checks pass for both engines; original current-clock ideal failure retained | `ddh-v1` | Unrun |
| C caller exclusivity | 12 complete workflows each in plain, ASan/UBSan and TSan pass | `c-concurrency-v2` | Unrun |
| Affected serial performance | Three samples each of installed predictor and matched CLI workload measured; no threshold | `performance-v2` | Unrun; full AK worker campaign pending |
| Current source/unsafe/departure closure | 93 responsibility rows, 155 unchanged Rust body files, all 36 SOFA files verified; 63 unsafe lexical sites indexed | `source-audit-v3`, RELEASE_SOURCE_MAP | Full branch/corpus closure pending |

Ignored tests are not reported as passed. Many are driver protocols requiring
named corpus/action files; the nine explicitly enabled tests cover timing,
constraint snapshot, JPL stream lifetime, observer aliases and terminal-clock
recovery. Early driver corpora not replayed here remain inherited evidence.
Broader observational validation and exhaustive scientific branch execution are
not established by this matrix.

The first default external runner printed 31 and combined 67 client counts due
to counting lock generation as a client. Actual command inventories contain
30/66; the successor counts client executions explicitly and records its lockfile.

## DDH and numerical policy

Current-clock maximum absolute residuals are about 4.599 ns pre-fit and
6.510 ns post-fit in both implementations, so the original 1 ns ideal assertion
fails. Historical-clock maxima are approximately 0.466 ns pre-fit and 0.417 ns
post-fit; the strict gate passes independently for native and Rust. Agreement of
these residuals does not close the full 401/433 fidelity mismatches in other fit
quantities, or the two shared-solve raw-order differences.

No tolerances, mode order, warning conventions, precision, clock selection or
scientific expressions were changed. The C ABI comparator now decodes native
hex exactly. The AF successor decodes exact fields at measured native precision,
keeps explicitly binary64 fields binary64, and computes metadata with native
rounded operations. Its EPS remains 2^-52 and all quantity/serialization bounds
remain frozen. Rejection tests distinguish x87 neighbors that Python float would
collapse. Linux execution, not these controls, must establish Linux fidelity.

## Complete unresolved acceptance ledger

M0/M1a/M1b retain their scoped acceptance; M2 is unaccepted. None of these items
is waived by an operational pass:

* P: 154 records including 32 SIMUL. M2: four raw SVD-order diagnostics across the
  two profiles. 0049: four discrete boundaries and three unconstrained
  interpolation cases. All were replayed: the three unconstrained family cases
  retain their native-success/Rust-failure execution boundary. Constrained
  interpolation passes; no boundary is counted as a numerical pass.
* AE: red six raw modes; chromatic-log sixteen errors and one multiplier;
  ELL1H nineteen whitened-data differences; single-pulsar quadrupole/GW-single/
  memory/cosmic-string 770/750/448/468 failures.
* DDH: 401/433 current/historical fidelity and two shared-solve raw-order
  differences. Two first-pass SWGP unwritten-state boundaries remain. AF: seven
  raw polyco log10RMS failures. Historical ideal passes are a separate gate.
* AH: `pre-list-bary-unwritten`, `pre-list-tempo1`, `pre-list-terrestrial`,
  `dm-grid`, `red-noise`, `ell1h-h4`, `waves`, `wave-dm-scale`, `global`,
  `cheby-odd-defaults`, `cheby-ecliptic-tdb`, `polyco-prefix`. Absent DDH derived
  inputs and all-deleted span boundaries remain.
* Actual Linux execution, native-width/provider/contraction qualification,
  early-corpus reconstruction, complete body/branch evidence and broader
  scientific/resource coverage remain unfinished. The selected `ca49531`
  package documents were checked; any eventual release must bind its own final
  document revision too.

AK's closed diagnostic finding remains closed only within its complete-workflow
contract. Outer workflow composition, shared process histories and C caller
exclusivity remain required. No independent-context or blanket thread-safety
claim is made. The C sanitizer campaign instruments C callers/shim, not the
entire Rust/MPFR/Fortran/provider stack.

## Performance and failed preparation attempts

The final aggregate's installed predictor fixture measures 20,000 calls per
sample, with equal aggregate checksums; median native/Rust intervals are
0.000186/0.193290 s. This is a different predictor fixture from AK and is not an
improvement measurement. Matched serial global-fit CLI medians are
0.03138/0.47651 s. All three samples and CPU/RSS records remain intact.

Earlier component measurements overlapped compilation; the final aggregate
placed these intervals after its backend builds. Neither run reserved this
desktop host. These are bounded regression observations, not controlled capacity
or scaling claims. No optimization, precision concession or speed gate was
introduced.

Retain AK's corrected original identities: `performance-v2/receipt.json`
`c9169bfd16d6847bb92a88ca44960a63845dfca3b574dea09fc263a569c51418` and
`evaluation-v1/receipt.json`
`5a9691b10c2ae71842ceace0e3e570647962a14cbd22ffcd3657513a8f1214c1`.
Those measurements are not replaced by AL's limited serial rerun.

Failed preparation attempts remain intact: unsupported Python3.11 extraction;
raw/mixed-newline construction binding errors; missing clean-export M2/observer
inputs; zero-test filters rejected by the suite runner; copied executable mode;
source-directory mapping; and sandbox denial of macOS time counters. Successor
runs correct the harness/provisioning cause without changing numerical gates.
The final performance run used the authorized timing counters. These were AL
harness issues, not new scientific-engine corrections or passed checks.

## Workflow and final aggregate run

The recipe is [validation/release/README.md](../validation/release/README.md).
The workflow pins checkout `11bd71901bbe5b1630ceea73d27597364c9af683` and
upload-artifact `ea165f8d65b6e75b540449e92b4886f43607fa02`, uses ubuntu-24.04
x64, read-only permissions, explicit toolchain/provider selection, no cache and
30-day selected evidence retention even on failure. Actual apt/image identities
are recorded at execution; no immutable apt snapshot is claimed.

`integrated-v1` executed the committed recipe and clean candidate
`ca49531f5eb0b21362f585887f7278c269d4f7be`, rebuilding A2, CALCEPH, all six native
profiles, default/combined installs and external consumers. Public source/data
inputs were supplied from hash-verified local copies; no prior project objects
or installed products were reused. Cargo's dependency cache was reused. The
aggregate exited **1**, with 52 passing stages, the six failure stages below,
and no dependency-blocked stages:

| Failing stage | Actual result |
| --- | --- |
| M2 release | Two dependent-column raw singular-order failures; separate debug replay retains the other two |
| General fitting | 72 cases; 2,478 numerical failures plus three interpolation and two SWGP execution boundaries |
| Utility boundaries | The same four discrete mismatches across 16 inputs |
| DDH | 401 current-clock and 433 historical-clock fidelity failures; zero historical 1 ns ideal failures |
| Predictors | Seven raw log10RMS failures across 14 cases / 37,310 checks |
| Core output | The same twelve groups; 17 different files, three missing, zero extra, zero unexpected statuses or changed inputs |

Both default debug/release suites again pass 111 tests plus nine explicitly
enabled runtime tests per profile; ignored cases remain ignored. Formatting and
all-target Clippy pass. Both installs pass C11/C++17 static/shared clients and
plain/ASan/UBSan/TSan caller campaigns. Outside Cargo consumers compile 35 source
files per install and execute 30 default / 66 combined invocations. The six
backend profiles, Lomb/QR witnesses and 130 timing comparisons pass. The final
predictor receipt binds the frozen exact-width comparator actually executed.

After the aggregate, `final-checks-v1` verifies 2,463 source/package file checks
with zero mismatches: each prefix contains 256 files, including 250 installed
source/document files. This binds the report and source map present at `ca49531`;
this completion note is a later document revision. Four independent qr-default
without-LAPACK tests also pass. These two checks are now wired into the successor
aggregate recipe; that expanded orchestration was not rerun in full.

The corrected collector preserves 9,985 selected evidence files, including all
502 AE fitting evidence files formerly excluded with the temporary source tree.
Every selected file was rehashed against its manifest. The collector and audit
are identified in `final-checks-v1`; generated evidence remains ignored. No
scientific gate or production body changed during this final bookkeeping.

The local candidate/workflow diff is ready for central review. Linux execution
still requires the explicit publication/dispatch approval in 0097/0098. Early
Linux corpus reconstruction, full AK native worker/performance execution and
exhaustive body/branch evidence remain unfinished. A workflow file or green
infrastructure build is insufficient for AL completion.

## Selected evidence identities

SHA-256 of each named receipt, under `validation/release/build/`:

| Receipt | SHA-256 |
| --- | --- |
| `integrated-v1/receipt.json` | `cf103977c56cb6fc9d1f8cca4be2d823b14ef78a5a4e218bdee2ccaf54d43298` |
| `integrated-v1/rust-suite/receipt.json` | `cf45570fd56f09f33a057a22b422a09b93531675abab83cdd165b369d3f3a1ec` |
| `integrated-v1/provider-abi/receipt.json` | `4c8e89def1fe947ec5b10ec57fff4ff8be10f7b9213d206af3dff6db91c39ecb` |
| `integrated-v1/source-audit/receipt.json` | `5ca64d44c783e00908c40136ba2f99dafb35a67f8d0d255aa13704c195748784` |
| `integrated-v1/predictors/receipt.json` | `14faac136bef68cc68b39694c03afc5cba0406f9cfee97bfffd928436affa0f3` |
| `integrated-v1/output/receipt.json` | `72569d8df1594c4ca9c68af954a2d9258c5fd4736acaf3536bc6e93b34f89d5d` |
| `integrated-v1/performance/receipt.json` | `b5e0657a97e4d0ee9ec774ef5db3cd7cf1b3daee85c56480a3357e5135e8f371` |
| `final-checks-v1/receipt.json` | `c15b56a9541eba43c2a36f3869ea6b68ccb0335f00bdebfbe4a231ad6a74a4f6` |
| `final-checks-v1/package-audit/receipt.json` | `479f142d44bcd6c297e61ecbe3b9ffc3884d13ad3a910baba5004b2d172a5eae` |
| `selected-evidence-v1/manifest.json` | `c86b797d5dafc13be00518911db36f0ccafe085aeebdd93cb538e0e5510d6807` |
| `macos-a2-v2/receipt.json` | `896ae8eddb2ed20cf19437604ab5ef95aa77aab76b521d89edfe4c6486ff4f77` |
| `macos-provider-v1/receipt.json` | `90ca2e9e10b450a5850e251d1c4eb4e30e726016cf64ade03a6fdf9f6d0de71f` |
| `suite-v2/receipt.json` | `5cbcc8bed799e4764f8594725dfb957d407c01c26d0ba226a20b0c05e904eac9` |
| `source-audit-v3/receipt.json` | `74da37cfda06a95d4c4054b466f727cb2f063f93173a92af94d3c03ef433499a` |
| `predictor-default-v1/receipt.json` | `ea44cf1efd1f9b71868d0f099e494910d145e733d33a853a357662a2d1abed01` |
| `predictor-combined-v1/receipt.json` | `64689b3dee3f59c6a49471f2bb7d01e72797e89ca3b40e648d73d5ce224d8775` |
| `external-default-v1/receipt.json` | `17b372710f04db6444a37a49b36349e46e121f9d705a0c0b212189553806146a` |
| `external-combined-v1/receipt.json` | `0fc3bca60f40e71333721b46e00f264a51b0c6cbc25cb998d042286f9e8c1517` |
| `profiles-v1/receipt.json` | `21c094373097f6d05f44704462ed38260e48dfeb21e538b2db39346b2d66a8f8` |
| `paired-combined-v1/receipt.json` | `7d9413682a54ef5d561de6d4981f0eacf5928e09d742eb3a30b6191d0a042e0d` |
| `contraction-v1/receipt.json` | `3ce38fa3f9f502b8d8efc6234aefb4a10483fe2f58aed7915f644236b13d4932` |
| `timing-v1/receipt.json` | `d9856b11df9804a5d799529b1b92e6cd2129e318ce765552d20aa69df0baf366` |
| `fits-v1/receipt.json` | `0111b74704b163e6d9853c6fc8b935266834e478cc6252a9a74647fe287f4aad` |
| `m2-comparison-v2/receipt.json` | `3153dfd91ba09db9d30e68572eabaeedc6726fbf62f03d029d82dc1b1c494204` |
| `m2-comparison-debug-v1/receipt.json` | `bcbb0abbbf6eaf7e5ba60267bfbb70281b282b774e9517459371db1180958b0f` |
| `boundaries-v1/receipt.json` | `c44d4da64ee743e144414fea06db6a0d7c7be09dc0ea8e222ea22e9d5809348f` |
| `predictor-science-v2/receipt.json` | `cfc75093d3b377c642801ef5336c1b2b588ce57cf5012dae6aad391d019df2a0` |
| `output-v4/receipt.json` | `4278ced8cfa3d7da6978118297eb94bf41d99d4c38dacd893270ec525a62ed30` |
| `ddh-v1/receipt.json` | `1149eb65196f83dcae599cfa3650f5bc36a365a634670d2f75e9bfa07c576ce3` |
| `c-concurrency-v2/receipt.json` | `0ce7801bbdcd6d3af4ef90a0b1acaa13878b0853675628596b3b3018d02d7f41` |
| `performance-v2/receipt.json` | `4a5e6e972ff3d087ca6d8d18f8e96350b86bcbb5f40249a8dd05edfaf3f8ad5c` |
| `preprocessing-v2/receipt-release.json` | `c15f8a6de3c24f31f3dca6cc079b875ef510ab8acf54fd041d0c06b3376b25d1` |

## 30 October review correction and continued qualification

Central found a false pass in a depth-one checkout: baseline `6acaf53` was absent,
all 155 `git show` calls failed, and the old audit labeled them changed while
returning success. Its original reproducer remains unchanged at
`/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-al-shallow-review-58vbmv7z/`;
summary SHA-256 `bc919c38eae4d80bd094be1e4293f38bcf193e1956611cb05ac704f7ae2c804c`.
The earlier full-history macOS audits are not invalidated by this caller failure.

The workflow now fetches the single required baseline with `--no-tags --depth=1`.
The audit fails before classification when that baseline is unavailable; existing
blob reads must succeed, and genuine additions/removals/modifications remain
separate from unavailable evidence. A focused shallow regression checks failure,
narrow-fetch recovery, unchanged candidate HEAD, and real changed/added/removed
files. The full audit in a fresh shallow checkout of central `5dacc3c` plus the
explicit successor script exits 1 without the baseline, then exits 0 with
155 unchanged bodies after the narrow fetch. All five harness controls pass.
Receipt: `shallow-followup-v1/receipt.json`, SHA-256 `bee54f8ef2e708d2f642ea1cbe26f039edd97330438f0f1ea85a2194f601590d`.
No scientific campaign was rerun solely for this infrastructure correction.

Decision 0099's evidenced implementation-departure policy applies to this work;
the covariance allocation experiment stays deferred. AL remains unaccepted.
Central reports the prior `c14b1ee` candidate is now public; this chat has not
pushed or dispatched a workflow, and publication alone does not authorize a run.
The follow-through plan is recorded in `validation/release/PLAN.md`.
## 30 October follow-through: compressed evidence retention

Central's collector review found that `Path.suffix` is `.gz` for compressed JSON,
so the earlier collector omitted native reference/private-state/gcov records.
The successor admits `.json.gz` and `.jsonl.gz` after the existing source, cache,
runtime, installation and symlink exclusions. It does not admit arbitrary gzip
archives. A realistic path-based control covers reference records, compressed
JSONL events, coverage receipts, gcov JSON, the existing AE evidence exception,
and excluded source/cache/runtime/binary/archive/symlink inputs; it passes.

The original central reproducer remains at
`/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-al-collector-review-r2gxgw2l/`.
Recollection into `validation/release/build/collector-review-followup-v1` retains
both its receipt and the actual 3,541,285-byte observer reference, with unchanged
SHA256 `e810f70fa52c778059413cc50ea24d5df96cfebd7b345aaa1efb4531b19b0a85`.
The new manifest SHA256 is
`50e51180a254b56804f6efef19a6dd08283fc47cc462e90ec2ddfa099cf0152d`.
This is artifact-retention verification only; no scientific comparison or Linux
execution was rerun or accepted for this correction. The historical selected
artifact directory remains unchanged; subsequent corpus collections use the
corrected selector.

## 30 October full native reconstruction and worker follow-through

`native_corpora.py` now reconstructs all thirteen early native corpora from
authored catalogs and raw pin `fda6ad4`, in a new working snapshot on the actual
host. The A2 archive/configuration remain separately hash-bound. Required
historical source witnesses are fetched as individual shallow commits. Each
run retains its executed producer, build adaptation patch, compiler identities,
ordinary results and separate coverage/observer receipts. Canonical source and
historical runners are unchanged.

| Corpus | Catalog size | Selected fresh native evidence |
| --- | ---: | --- |
| H / JPL | 55 | `corpora-followup-v3` |
| J / clock chain | 151 catalog, 142 permitted native executions | `corpora-followup-v10` |
| K / parameter input | 306 | `corpora-followup-v9` |
| M / numeric utilities | 350 | `corpora-followup-v9` |
| N / TIM input | 157 | `corpora-followup-v9` |
| P / preprocessing | 302 | `corpora-followup-v9` |
| Q / specialized PAR | 107 | `corpora-followup-v7` |
| S / pulsar lifecycle | 30 | `corpora-followup-v9` |
| V / time ephemeris | 97 | `corpora-followup-v9` |
| U / terrestrial resources | 130 | `corpora-followup-v5` |
| X / observer coordinates | 133 | `corpora-followup-v5` |
| Z / planetary ephemeris | 96 | `corpora-followup-v5` |
| AB / propagation | 311 ordinary + 13 supplemental | `corpora-followup-v8` |

These selected native stages pass their authored recording/validation and
applicable private-state observer and coverage runs. Catalog membership is not
a claim that withheld undefined continuations executed. H retains its declared
case/function map, rather than a newly measured gcov map. Q's seven telescope
fixtures obey 0072's native capacity 500; original 1000-row fixtures remain
unchanged. The Rust 501/1000 boundary cases remain required. Only Q's gcov
process allowance changed from 90 to 300 seconds for its complete scalar history.

Fresh P comparison in `p-fresh-followup-v1` reproduces all 154 inherited
differences, including 32 SIMUL. `p-classification-followup-v1` runs the authored
CFG classifier against the fresh v9 coverage: 122 compiler/exception unrun
controls, two defined-but-unexercised entries, one defined prefix with unsafe
continuation withheld, seventeen source-inspected unrun entries and five
structurally unreachable entries. Classification does not convert unrun paths
into numerical passes.

`workers-followup-v1` passes the complete AK eleven-case campaign: 99 processes
at requested counts 1/2/4 with actual native-worker measurements, complete Rust
histories and unchanged comparison quantities. It records 108 timing samples
from six workloads, two engines, three worker counts and three shuffled repeats.
All twelve instrumented/uninstrumented native product controls pass.
`worker-parser-followup-v1` confirms that the portable CPU/RSS parser reproduces
all 108 stored macOS measurements exactly. Linux RSS is explicitly converted
from KiB to bytes; synthetic parser controls are not Linux execution evidence.
No performance threshold, optimization or controlled desktop-load claim is added.

## 30 October current Rust corpus comparisons

All following comparisons use a clean export of `8d9c11a`; its scientific Rust
bodies are identical to the released baseline. Each selected receipt binds its
own fresh native producer and exact compiler/input/executable identities. The
native coverage selections above may be later reconstructions of the same
catalog; they are separate evidence, not substituted oracle identities.

| Route | Actual release-profile result | Comparator receipt directory |
| --- | --- | --- |
| JPL | Pass at the existing 55-scenario projection, including the declared title boundary/unreachable events | `compare-v16` |
| Clocks | 142 exact native comparisons and eight Rust boundaries pass; the native cycle remains unrun | `compare-v3` |
| K parameters | 304 cases pass; two ambient-environment cases retain their explicit-root API disposition | `compare-v10` |
| O parameter storage | 10 histories/32 events, 2,000 slots/9,616 cells, six current boundaries and 30 controls pass; AE's initialized/unwritten copy test also passes | `compare-v22` |
| R observation storage | Three histories, all 95 fields, four M cases, fifteen boundaries and 39 controls pass | `compare-v17` |
| N TIM | All 157 histories pass, including five separate JBO diagnostic corrections | `compare-v6` |
| Q specialized PAR | 121 checks pass: 100 ordinary histories, seven bounded native histories and fourteen 501/1000 Rust rejections | `compare-v14` |
| S/T lifecycle | Raw: eight failing histories retained. Separate approved native corrections: all 30 Rust histories and controls pass | `compare-v18` |
| V/Y time ephemeris | All 97 histories pass; fresh native 30 controls, 36 independent polynomial checks and thirteen compiled mutations pass; Rust controls pass | `compare-v20` |
| U/W terrestrial resources | All 130 histories, existing corrected-coefficient proof and native/Rust controls pass | `compare-v11` |
| X/AA observer coordinates | Raw: eighteen failing histories retained. Separate approved UT1 producer: all 133 histories pass | `compare-v12` |
| Z planetary/random | Complete driver and random gates pass, covering the 96 unique native cases | `compare-v5` |
| AB propagation | All 324 raw histories execute: 277 field/event differences remain in three approved correction cases; the other 321 pass. Three separately reproduced corrected histories and helper checks pass | `compare-v21` |
| P preprocessing | 296 cases execute; all 154 inherited differences remain. Six plugin-ABI cases retain their explicit dispositions | `p-fresh-followup-v1` |

The lifecycle proof first isolated exactly sixteen records in the four approved
initializer fields (`compare-v10/lifecycle-corrected-native`). The final witness
also applies AE's existing reserved jump/FDJUMP slot-zero correction, using the
authored `fitting_corrections.py` transformation. Across all thirty histories,
exactly 88 records differ: sixteen initializer records and 72 reserved-slot copy
records. Every other complete native state/diagnostic record is exact, including
all nonreserved array/string cells. The raw eight failures remain separately
visible; they are not new production changes or erased comparisons.

The observer witness reuses AH's exact `observer_corrected` transformation from
`native_build.py` under 0094. Only 143 correctionUT1 field values change across
the 133 native histories; every other caller/private state, diagnostic and
outcome is exact. The corrected private observer independently agrees with its
corrected ordinary execution before the exact Rust comparison. The eighteen
raw failures remain retained. O's two former unwritten-prefit error expectations
were superseded by AE's documented ownership copy; the successor retains the
six remaining boundaries, retargets the same prefix mutation controls, and runs
AE's existing initialized/unwritten-state test.

Preparation failures remain evidence: obsolete Cargo artifact selection,
missing type/catalog/control prerequisites, historical ownership snapshots,
source/caller pathname truncation, the tracked JPL build-log overwrite and
relocated native profile identities. The successor recreates caller files in
matched paths before truncation, emits every staging adaptation and checks that
candidate source bytes remain unchanged. At execution, native launch scratch was
left in its producer directory rather than copied into every follow-up; this is
not a retention guarantee for working directories. See the archive locations below.
No historical runner, numerical tolerance, production precision or scientific
body was changed.

## Follow-through closure, controls and remaining work

`closure-followup-v1` joins all 26 requirement families and 93 library units to
91 explicitly selected execution records and 34 native source/coverage artifacts.
It retains measured function/edge objects, authored correspondence rows with
hashes/line numbers, static dispositions and exact comparator receipt identities.
The separate fresh P classification receipt is included. Functions with only an
authored case map, unhit branches and bodies without fresh counters remain
identified as such. This is successful evidence accounting, not exhaustive
branch execution or acceptance of the inherited numerical failures.

`followup-checks-v1` passes nine harness controls, Python compilation and the
diff check. `source-audit-followup-v2` again verifies all 155 unchanged scientific
Rust body files, the 93-source inventory and all 36 retained SOFA files. The
collector now also retains plain failed JSONL and exact producer snapshots.
One local collection attempt named a nonexistent comparison directory and
produced an empty selection; that attempt is explicitly labeled in the new
archive index, and the collector now rejects missing input directories. No
empty selection is evidence of an executed comparison.

The current `integrated.py` defaults to the full native/corpus and AK campaigns,
then writes the evidence ledger. Its components were exercised in the isolated
follow-ups listed here; this is not a claim that the expanded aggregate was
executed once end-to-end. The earlier 58-stage aggregate and every retained
scientific failure remain unchanged. The next reviewed full Linux candidate
must execute the new corpus/worker paths and measure its actual compiler, ABI,
provider, contraction and process behavior. Initial run 40903715685 at b59a840
does not contain these additions and cannot substitute for that execution.
Central acceptance and remaining source/branch/numerical dispositions stay open.

## Follow-through retained identities

`followup-evidence-v1` contains 44 nonempty run archives plus the explicitly
labeled empty selection attempt. All 118,917 selected file entries were read
back from their archives and verified against their source hashes. The archives
total 4,119,169,211 bytes; the index binds each archive and its per-file manifest.
Some original successful/failed working directories are now empty. The nine
report-bound receipts below are available from the preserved archives at the
exact locations listed after the hash table; their receipt and archive identities
are unchanged. `selected-evidence-v1` is a separate historical selection.
The package excludes generated binaries, runtime datasets and Cargo caches.

| Receipt under `validation/release/build/` | SHA-256 |
| --- | --- |
| `followup-checks-v1/receipt.json` | `04218a16a37e47fe487f36166768108a971d2a2a8184dd228670a642f3cc1822` |
| `source-audit-followup-v2/receipt.json` | `60b993b55e61e5a6569672f70a0a4e1b7109226a0c9616a076d12bebfe5d08d3` |
| `closure-followup-v1/receipt.json` | `58fa964cb9896dc10c78974a46e9c3bdccfa731f4796aecaaf673ef5dcb16675` |
| `workers-followup-v1/receipt.json` | `d1edde23a44d4453416729039c37c73a4166ce5e41325deed6452b69501955ba` |
| `p-fresh-followup-v1/receipt-release.json` | `45698765395812869836b839bded159b507242fedad594238c2b5c3a17120282` |
| `compare-v18/lifecycle-corrected-native/receipt.json` | `4ef92a96905ccacf9326b557e42629b1d7fb19ae45547b3a74f4db0873da3cef` |
| `compare-v12/observer-corrected-native/receipt.json` | `f89def63795394f0ec8d79eaa803b8617b4c3798fd6da7ff476e2be0aec33447` |
| `compare-v21/receipt.json` | `a274555570b8411a601c4d712a94342fc7f6af3a6df69a5d79c4f13db5c90828` |
| `compare-v22/receipt.json` | `2b2060c0241f142a41c37f4b5b5532507549954b746ec3b76ae05bd67e1a3ce0` |
| `followup-evidence-v1/manifest.json` | `0f9694fb0973ee4abd0fed9b30d585e6c3154c199d160c3d41bbfdeb1330658e` |

The receipt paths above identify the original runs. For retrieval, archives are
under `validation/release/build/followup-evidence-v1/`; member paths are relative
to each archive root, with **no run-directory prefix**. The unchanged
`manifest.json` binds each archive's SHA-256 and its per-file manifest under
`manifests/`. Central's independent verification is recorded in
`decisions/evidence/release-qualification/build/second-linux-4accd26-review-20261030/evidence-review.json`
as `verified_at: "verified selected archive"` for these nine receipts.

| Original receipt path under `validation/release/build/` | Archive in `followup-evidence-v1/` | Exact member |
| --- | --- | --- |
| `followup-checks-v1/receipt.json` | `followup-checks-v1.tar.gz` | `receipt.json` |
| `source-audit-followup-v2/receipt.json` | `source-audit-followup-v2.tar.gz` | `receipt.json` |
| `closure-followup-v1/receipt.json` | `closure-followup-v1.tar.gz` | `receipt.json` |
| `workers-followup-v1/receipt.json` | `workers-followup-v1.tar.gz` | `receipt.json` |
| `p-fresh-followup-v1/receipt-release.json` | `p-fresh-followup-v1.tar.gz` | `receipt-release.json` |
| `compare-v18/lifecycle-corrected-native/receipt.json` | `compare-v18.tar.gz` | `lifecycle-corrected-native/receipt.json` |
| `compare-v12/observer-corrected-native/receipt.json` | `compare-v12.tar.gz` | `observer-corrected-native/receipt.json` |
| `compare-v21/receipt.json` | `compare-v21.tar.gz` | `receipt.json` |
| `compare-v22/receipt.json` | `compare-v22.tar.gz` | `receipt.json` |

For example, from the repository root, read the lifecycle receipt without
recreating or overwriting its original working directory:

```sh
tar -xOf validation/release/build/followup-evidence-v1/compare-v18.tar.gz \
  lifecycle-corrected-native/receipt.json
```

Verify the archive against the index and the extracted member against the receipt
hash table. Location verification is not a new scientific execution. Selected
archives exclude binaries, runtime datasets and Cargo caches; they are evidence
packages, not complete copies of the original working directories.
