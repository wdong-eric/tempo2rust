# AL release qualification recipes

Qualification is authorized by 0101, including centrally reviewed AL fixes and
reruns. Central coordinates publication and workflow dispatch for each reviewed
candidate. Actual Linux execution remains mandatory, and a failing numerical
gate stays failing.

Use Python 3.12+, Git, patch, C/C++ compilers, GFortran, CMake, Rust/Cargo 1.97.1,
rustfmt and Clippy. The baseline is serial Rug128. Network is used only to fetch
pinned native/CALCEPH inputs and Cargo.lock dependencies when no explicit input
cache is supplied. Initial CI has no build cache. Native source is pinned to
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`. The independent A2 authored input
provenance is in `a2/provenance.tsv`; its original repository is never modified.

## One clean execution

From the repository root, use a new output path and an exact committed candidate:

```sh
# macOS arm64; CMake must be installed or passed as an absolute --cmake path.
export FC=gfortran-16 NATIVE_CC=gcc-16 NATIVE_CXX=g++-16 NATIVE_GCOV=gcov-16 CC=clang
export TEMPO2_ACCELERATE=1 VECLIB_MAXIMUM_THREADS=1
python3.13 validation/release/integrated.py \
  --revision REVIEWED_FULL_COMMIT \
  --native-repository "$PWD/tempo2" \
  --out validation/release/build/macos-final

# Linux x86_64, only after approved workflow publication and dispatch.
export FC=gfortran-13 NATIVE_CC=gcc-13 NATIVE_CXX=g++-13 NATIVE_GCOV=gcov-13 CC=clang
export TEMPO2_OPENBLAS=1 OPENBLAS_NUM_THREADS=1
python3 validation/release/integrated.py \
  --revision REVIEWED_FULL_COMMIT \
  --out validation/release/build/linux
```

`--provider-source ARCHIVE --ephemeris DATA --cmake PATH` explicitly supplies
cached dependency inputs; both downloaded or supplied inputs must match the
pinned SHA-256 in `providers.py`. They are rebuilt into a fresh private prefix.
The pipeline exports a Git archive, binds the separately identified fresh runtime,
generates M2 observer inputs from the original authored recipe, installs the full
CLI/source/C interface, compiles outside consumers, and executes default and
selected provider profiles. It never writes to the canonical TEMPO2 installation.

Scripts use fresh destinations. Preserve failed runs and choose another path.
Each command's exit and stdout/stderr remain evidence. The aggregate records
failed and dependency-blocked stages and continues independent stages. A failure
receipt is not converted to a successful scientific assertion. Pure builds,
operational tests, numerical correspondence and central acceptance are distinct.

The aggregate audits both private installs against the selected source export,
including membership, source bytes, licenses and report documents. To run that
check separately:

```sh
python3.13 validation/release/audit.py \
  --candidate validation/release/build/macos-final/candidate/source \
  --prefix validation/release/build/macos-final/install-default \
  --prefix validation/release/build/macos-final/install-combined \
  --out validation/release/build/package-audit
```

The historical P corpus can be replayed on its measured macOS profile explicitly:

```sh
FC=gfortran-16 CARGO_TARGET_DIR="$PWD/validation/release/build/test-target" \
python3.13 validation/release/check_preprocessing.py --release \
  --candidate validation/release/build/macos-final/candidate/source \
  --reference-root validation/preprocessing \
  --out validation/release/build/p-replay
```

This command consumes the immutable historical P data read-only. The integrated
successor instead calls `native_corpora.py` to reconstruct H/J/K/M/N/P/Q/S/V/U/X/Z/AB
from their authored complete catalogs and the raw pin on the executing host,
then calls `corpus_compare.py` with the exported candidate. Both accept an explicit
`--stages` list for focused follow-up; default execution retains the full list.
Staging copies have source hashes and emitted adaptation patches. Historical
runners and outputs are not rewritten. Required old source witnesses are fetched
as individual shallow snapshots; the full repository history is not fetched.

Q's seven telescope append fixtures use the defined native capacity 500 under
0072. The original 1000-row fixtures are preserved but are not run as native
goldens. Rust's explicit 501/1000 boundary checks remain. The separate gcov run
has a 300-second per-case allowance for Q's full scalar history; no scientific
threshold changes. Instrumented differences and unhit branches remain distinct
from ordinary numerical comparison. K and N execute both engines against the
same concrete caller paths before native log truncation.

Raw lifecycle and observer expectations remain binding, failed evidence when
they encounter later accepted changes. `lifecycle_correction.py` separately
builds the four documented initializer writes (0068, AC, 0079), and
AE's reserved jump/FDJUMP slot-zero copy correction, while
`observer_correction.py` reuses AH's exact UT1 producer transformation (0094).
These proofs require every other state/diagnostic field to remain exact before
the Rust comparator consumes their separately identified corrected records.
W/Y likewise rebuild their existing corrected-native witnesses and controls.
O retains six copy boundaries and AE's initialized/unwritten copy test; its two
old prefit-read boundaries were superseded by AE's documented ownership copy.

`workers.py` retains AK's eleven cases, actual measured native workers at 1/2/4,
complete Rust histories, six workloads, three shuffled samples and the separate
instrumented/uninstrumented product control. `/usr/bin/time` measurements use
host-specific parsers with RSS converted explicitly to bytes. There is no new
performance threshold or optimization. The Linux recipe requires an actual
Linux run; local macOS measurements do not qualify its compiler or ABI.

`requirements.tsv` and `evidence_closure.py` join the 26 requirement families,
all 93 library units, authored routine/branch maps, fresh measured function and
edge records, exact comparator receipts and explicit missing evidence. The
ledger's successful generation does not close unhit defined branches, numerical
failures, platform execution or central acceptance. The integrated pending
ledger continues to report those obligations.

## Numeric and construction boundaries

* A2 initializer correction, instrumentation, reviewed core corrections and the
  AE observer are separately identified. `native/provenance.tsv` records original
  bytes, predecessor source/receipt identities and staged LF bytes. The LF
  normalization is explicit because some historical compiled inputs mixed CRLF
  and LF. Linux's CBLAS/LAPACK declaration adaptation is a separate emitted patch.
* `provider_abi.py` measures integer sizes, Fortran round-trip, actual provider
  path/config/threads, matrix operations and CALCEPH. Requested thread limits are
  not inferred worker measurements.
* `qualify_predictor.py` measures native ordinary long double, layout, calling
  conventions and all 64 C symbols. `predictor_compare.py` parses `%La` exactly,
  including x87 neighbors and subnormals, without a binary64 conversion.
* `check_predictor_science.py` keeps AF cases and numeric gates, but uses the
  measured significand for exact fields; explicit C double fields stay binary64.
  Metadata follows native rounded division and addition/subtraction. EPS remains
  exactly `2**-52`; serialization allowances are unchanged. Controls reject an
  x87 neighbor that a Python float would collapse.
* `contraction.py` preserves every Lomb input/bin and compares default source
  compilation. Off/fast builds and assembly record contraction effects. Its 32
  QR cases compare unchanged AI quantities; variant differences are measurements.
* `ddh.py` executes current-clock fidelity and the separate versioned historical
  1 ns ideal check, including the original native check's raw exits. Neither
  clock selection nor the fidelity gate changes.
* `boundaries.py` feeds identical native-parsed values to current public Rug128
  utility functions and preserves 0049's four discrete failures. No 113-bit
  production workaround is introduced.
* `check_output.py` only maps the recorded compilation source directory to its
  byte-identical observer copy before AH's existing pinned-line normalization.
  Every numerical, text, file-membership and exit gate remains unchanged.

## Workflow review and evidence retention

The manual workflow selects `ubuntu-24.04` x64 and checks a full reviewed SHA.
Checkout remains depth one; a separate `git fetch --no-tags --depth=1` obtains
only the required `source_audit.BASELINE` snapshot. Other shallow callers must
fetch that same baseline before auditing. Missing baseline evidence is a failing
audit, not a list of changed Rust files.
It records actual image/OS/package/compiler/Rust/provider identities; the label
is not an immutable image. GCC/GFortran 13, OpenBLAS pthread LP64 and LAPACK
headers are explicitly provisioned. Rust 1.97.1 is selected. Dependencies are
resolved packages with versions recorded, rather than a claim of pinned apt bytes.

Reviewed action references are checkout v4.2.2
`11bd71901bbe5b1630ceea73d27597364c9af683` and upload-artifact v4.6.2
`ea165f8d65b6e75b540449e92b4886f43607fa02`. Their official action definitions
were checked at those revisions. Checkout persists no credentials; permissions
are `contents: read`. Selected evidence uploads on failure, with **30-day**
retention, identified by run ID and attempt. The collector retains hashes/logs/
comparisons/patches/assembly, executed producer snapshots and JSON/JSONL (plain
or compressed), including the fixed-output AE results under the isolated
candidate export. A missing input directory is an error. It excludes runtime datasets, libraries, binaries,
source trees and Cargo caches. Evidence retention is not release publication.
Required acceptance evidence must be preserved separately before expiration.

Review the local publication diff with:

```sh
git diff 6acaf53d2e13291eb6d0b0988faf7dee7d33a635 HEAD -- \
  rust validation/release .github/workflows/tempo2-rust.yml
```

0101 authorizes central to publish reviewed AL diffs and dispatch qualification.
A manual workflow must exist on the remote default branch; central selects
the workflow in Actions and supplies the reviewed full SHA. Nothing here
sends that request, changes the remote or dispatches a run.

Official references: [checkout action at the pin](https://github.com/actions/checkout/blob/11bd71901bbe5b1630ceea73d27597364c9af683/action.yml),
[artifact action at the pin](https://github.com/actions/upload-artifact/blob/ea165f8d65b6e75b540449e92b4886f43607fa02/action.yml),
[manual dispatch](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/manually-run-a-workflow),
[OpenBLAS C ABI](https://github.com/OpenMathLib/OpenBLAS/blob/v0.3.26/cblas.h),
[LAPACK character-length macros](https://github.com/Reference-LAPACK/lapack/blob/v3.12.0/LAPACKE/include/lapack.h).
