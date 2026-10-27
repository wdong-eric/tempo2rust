# AK observation execution qualification

Own only this directory and `rust/`. Canonical TEMPO2, independent reference,
accepted AC–AJ runners and their evidence remain read-only. Every runner takes
a **fresh** output directory under ignored `build/`; failed and provisional
directories are retained. See `rust/PARALLEL_EXECUTION_REPORT.md` for the final
receipt selection, source identities, limitations and numerical ledger.

## Executed recipes

`build_tools.py build/tools` bootstraps the private GNU tools used by
`makefile_witness.py build/makefile-v3`. The latter archives the exact canonical
pin into two disposable directories and executes bootstrap/configure/make/nm.
Its corrected Makefile comments identify source, cause and fix. Neither the
canonical Makefile nor a host installation is changed.

`native.py build/native-v4` hash-verifies and reuses AI/AH native dependencies,
rebuilds pinned calculate_bclt/DMMODEL/SWM source, and links serial, corrected
OpenMP and original-racy profiles. Actual row visits and SWM events are recorded
per observation. The original-racy profile is diagnostic evidence only.
`native_warning.py build/native-warning-v1 build/native-v4` preserves the source
race and its corrected comparison; generated DMMODEL offsets are declared in
the recipe. Do not use racy output as a scientific golden.

```sh
python3 validation/parallel_execution/qualify.py \
  validation/parallel_execution/build/qualify-release-v2 \
  --native validation/parallel_execution/build/native-v3 \
  --rust rust/target/release/examples/parallel_execution
python3 validation/parallel_execution/external.py \
  validation/parallel_execution/build/external-v4
python3 validation/parallel_execution/consumers.py \
  validation/parallel_execution/build/consumers-v2 \
  --client validation/parallel_execution/build/external-v4/client
```

The eleven AC-based cases retain the existing comparator and gates. Repeated
large data repeat validated TIM epochs/frequencies; only row names differ.
SWM uses AC's explicitly synthetic current sheets in a separate runtime overlay.
Rust complete result/diagnostic/provenance comparisons across worker counts are
exact. Native scientific TSV and tagged SWM events are exact across counts;
native messages use an explicit scheduler-independent multiset projection.

External packages compile the existing AJ/AF/AI clients against the canonical
library and use explicit complete workflow scopes. `consumers.py` records all
products. Only the generated caller-directory identity is normalized in fit
provenance/predictor files; scientific values and diagnostic vectors are kept.
`concurrent.rs` joins four genuine caller threads and bounds channel completion.
It exercises repeated correlated fits, predictor construction, failure/recovery
and owned diagnostic collection. Full histories and scientific projections are
separate retained artifacts.

`providers.py OUT --client CLIENT` builds a separate combined CALCEPH/LAPACK
consumer, reuses AI's unmodified native oracle/comparator, compares all worker
counts with the same real provider, then exercises concurrent complete callers.
`--reuse PREVIOUS_OUT` continues after a runner-only failure using that retained
build and paired receipt; it does not retry or relax a scientific failure.
The final continuation `providers-v2` uses `providers-v1`: the latter's paired
campaign passed, then its added direct command lacked TEMPO2. The continuation
supplies the same explicit runtime root and runs only the remaining commands.

`c_concurrency.py OUT` compiles the existing C clients plus the boundary shim in
plain, ASan/UBSan and TSan modes. Four actual callers obey process-wide C
exclusivity. The generated copy of AJ's single-process client resets only its
test-local callback counter between repeated `main` calls; all assertions remain.
Rust/MPFR/Fortran/provider code is not sanitizer-instrumented.

## Measurements and calibration

`native_timed.py build/native-v4 build/native-timed-v1` builds uninstrumented
native timing binaries. Its calculate_bclt objects contain no `ak_*` calls;
parallel objects reference GOMP_parallel. `timed_control.py` checks twelve full
product sets against the instrumented run. Actual native/Rust worker visits
come from the separately retained calibration; timing records state requested
counts explicitly.

```sh
python3 validation/parallel_execution/performance.py \
  validation/parallel_execution/build/performance-v2 \
  --native validation/parallel_execution/build/native-timed-v1 \
  --cli validation/parallel_execution/build/external-v4/client/target/release/cli \
  --rust validation/parallel_execution/build/source-snapshots/parallel_execution-final \
  --inputs validation/parallel_execution/build/qualify-release-v2/inputs
```

This measures three shuffled process samples for each of six workloads, both
engines, and requested counts 1/2/4. Raw BAT runs have five loaded evaluations.
Compare the matched load/evaluation intervals; native TSV and Rust null-list
publication differ, so raw total-process wall/CPU/RSS are separately labeled
workloads. CLI fitting/Cheby/polyco runs request the same input/model/output
work and retain exact per-engine files across counts/samples. Publication and
startup are included in those process measurements. No cold OS page cache is
claimed: “cold” means a fresh process/source session. Performance-v1 includes
native observer overhead and is provisional, superseded for timing claims.

`evaluation.py OUT CHEBY_FILE` measures identical serial C phase/frequency calls
through the native library and Rust ABI bridge. It separates the timed loop
from parsing/publication, preserves a checksum and records process resources.
No worker parallelism is claimed for this caller-exclusive C interface.

macOS `/usr/bin/time -l` requires permission to read its kernel resource counters
in this environment. The first sandbox-only attempt failed at that wrapper;
successful campaigns use the permitted resource query. This is a measurement
tool failure, not an engine exit. Worker/provider/thread settings, exact
commands and UTC starts are recorded. No precision downgrade or speed threshold
is introduced.

## Preservation and audit

The qualification example originally had only bats/timing/fit modes. Its exact
executable was reproduced byte-for-byte after adding raw-bats and preserved at
`build/source-snapshots/parallel_execution-qualification` with the expected
SHA-256 and source snapshot. The current driver is separately frozen there.
Historical receipts continue to name their original build path; the relocation
record documents the immutable matching copy. Final performance copies its
executables and recipe before running. Do not overwrite old evidence to make
historical source revisions appear current.
