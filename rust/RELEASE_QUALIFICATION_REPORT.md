# AL release qualification handover

27 October 2026. **Incomplete qualification; no migration acceptance.** The
macOS component campaigns below are executed. Linux is unrun. The manual
workflow and portable recipes are prepared for the review required by 0097;
no push, workflow dispatch, canonical installation replacement or release
publication has occurred. Central retains numerical policy and acceptance.

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

## Requirement-to-evidence ledger

All paths below are relative to ignored `validation/release/build/`.

| Requirement | Actual macOS result | Evidence | Linux |
| --- | --- | --- | --- |
| Fresh pinned reference/resources | Pass; no prior objects reused | `macos-a2-v2`, `macos-provider-v1` | Unrun |
| Private install, core CLI, getPeriod, Rust source, static/shared C | Default/combined pass | `install-default-v1-build/install.json`, `install-combined-v1-build/install.json` | Unrun |
| C11/C++17 × static/shared | All four combinations pass in each install; all64 symbols/layouts/storage/locale/pipe/recovery and rejection controls | `predictor-default-v1`, `predictor-combined-v1` | Unrun |
| Outside installed Rust consumers | 30 default and 66 combined client invocations pass, including concurrent complete workflows; 33 compiled source files each | `external-default-v1`, `external-combined-v1` | Unrun |
| Full applicable default suite | Debug/release each 111 tests pass across targets; library 96 pass/36 ignored. Nine explicitly enabled runtime tests additionally pass per profile. fmt and all-target Clippy pass | `suite-v2` | Unrun |
| Disabled, BLAS-only, LAPACK, CALCEPH-only, combined, QR-default | Frozen AI comparisons all pass; combined 59 cases/41,961 checks. Independent qr-default without LAPACK build/test also passes its declared disabled-provider behavior | `profiles-v1`, `paired-combined-v1` | Unrun |
| Lomb/QR contraction | All default quantities pass; every Lomb bin retained. Native Lomb default differs from off and equals fast. 32 QR cases and variant assembly retained | `contraction-v1` | Unrun |
| All14 binaries/fixed timing/lifecycle | 130 comparisons pass | `timing-v1` | Unrun |
| General public/family fits | 72 cases execute; 2,483 inherited failures retained | `fits-v1` and its exported-candidate receipts | Unrun |
| M2 shared/public fitting | Two dependent-column raw-order diagnostics in each debug/release profile; four total remain failures | `m2-comparison-v2`, `m2-comparison-debug-v1` | Unrun |
| P corpus | 154 inherited differences reproduced, including 32 SIMUL; immutable measured macOS corpus explicitly reused | `preprocessing-v2` | Native reconstruction pending |
| 0049 utility boundaries | All16 identical native-parsed inputs executed; four discrete mismatches reproduced through current public Rug128 functions | `boundaries-v1` | Unrun |
| Predictor generation/exchange | 14 cases/37,310 checks; seven raw polyco RMS failures retained | `predictor-science-v2` | Unrun |
| Core CLI/output | Same twelve failing groups; zero unexpected statuses/input changes | `output-v4` | Unrun |
| DDH fidelity and ideal | 401/433 fidelity failures; separate historical strict1ns pre/post checks pass for both engines; original current-clock ideal failure retained | `ddh-v1` | Unrun |
| C caller exclusivity | 12 complete workflows each in plain, ASan/UBSan and TSan pass | `c-concurrency-v2` | Unrun |
| Affected serial performance | Three samples each of installed predictor and matched CLI workload measured; no threshold | `performance-v2` | Unrun; full AK worker campaign pending |
| Current source/unsafe/departure closure | 93 responsibility rows, 155 unchanged Rust body files, all36 SOFA files verified; 63 unsafe lexical sites indexed | `source-audit-v3`, RELEASE_SOURCE_MAP | Full branch/corpus closure pending |

Ignored tests are not reported as passed. Many are driver protocols requiring
named corpus/action files; the nine explicitly enabled tests cover timing,
constraint snapshot, JPL stream lifetime, observer aliases and terminal-clock
recovery. Early driver corpora not replayed here remain inherited evidence.
Broader observational validation and exhaustive scientific branch execution are
not established by this matrix.

The first default external runner printed 31 and combined67 client counts due
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
rounded operations. Its EPS remains2^-52 and all quantity/serialization bounds
remain frozen. Rejection tests distinguish x87 neighbors that Python float would
collapse. Linux execution, not these controls, must establish Linux fidelity.

## Complete unresolved acceptance ledger

M0/M1a/M1b retain their scoped acceptance; M2 is unaccepted. None of these items
is waived by an operational pass:

* P:154 records including32 SIMUL. M2:four raw SVD-order diagnostics across the
  two profiles. 0049:four discrete boundaries and three unconstrained
  interpolation cases. The four boundaries were replayed; the three interpolation
  cases remain inherited and unreplayed in this AL campaign.
* AE:red six raw modes; chromatic-log sixteen errors and one multiplier;
  ELL1H nineteen whitened-data differences; single-pulsar quadrupole/GW-single/
  memory/cosmic-string770/750/448/468 failures.
* DDH:401/433 current/historical fidelity and two shared-solve raw-order
  differences. Two first-pass SWGP unwritten-state boundaries remain. AF:seven
  raw polyco log10RMS failures. Historical ideal passes are a separate gate.
* AH:`pre-list-bary-unwritten`, `pre-list-tempo1`, `pre-list-terrestrial`,
  `dm-grid`, `red-noise`, `ell1h-h4`, `waves`, `wave-dm-scale`, `global`,
  `cheby-odd-defaults`, `cheby-ecliptic-tdb`, `polyco-prefix`. Absent DDH derived
  inputs and all-deleted span boundaries remain.
* Actual Linux execution, native-width/provider/contraction qualification,
  early-corpus reconstruction, complete body/branch evidence and broader
  scientific/resource coverage remain unfinished. Final package documents
  must be checked from the eventual selected release commit.

AK's closed diagnostic finding remains closed only within its complete-workflow
contract. Outer workflow composition, shared process histories and C caller
exclusivity remain required. No independent-context or blanket thread-safety
claim is made. The C sanitizer campaign instruments C callers/shim, not the
entire Rust/MPFR/Fortran/provider stack.

## Performance and failed preparation attempts

The new installed predictor fixture measures 20,000 calls per sample, with equal
aggregate checksums; median native/Rust intervals are0.000185/0.195191 s. This
is a different predictor fixture from AK and is not an improvement measurement.
The matched serial global-fit CLI medians are0.0301/0.5543 s on this desktop
host. Compilations and other checks overlapped some measurements; these are
bounded regression observations, not controlled capacity or scaling claims.
No optimization, precision concession or speed pass gate was introduced.

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

The reviewed recipe is [validation/release/README.md](../validation/release/README.md).
The workflow pins checkout `11bd71901bbe5b1630ceea73d27597364c9af683` and
upload-artifact `ea165f8d65b6e75b540449e92b4886f43607fa02`, uses ubuntu-24.04
x64, read-only permissions, explicit toolchain/provider selection, no cache and
30-day selected evidence retention even on failure. Actual apt/image identities
are recorded at execution; no immutable apt snapshot is claimed.

The aggregate recipe has not yet been executed as one final committed run at
this report revision. Its constituent macOS campaigns above are executed; Linux
and outstanding corpus/worker obligations remain visible in its pending ledger.
A workflow file or green infrastructure build is insufficient for AL completion.

## Selected evidence identities

SHA-256 of each named receipt, under `validation/release/build/`:

| Receipt | SHA-256 |
| --- | --- |
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
