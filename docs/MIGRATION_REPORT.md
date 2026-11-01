# Assignment AM: migration documentation delivery

1 November 2026. This delivery implements [handoff 39](../handoffs/39-user-migration-documentation.md)
under [decision 0114](../decisions/0114-independent-work-during-qualification.md).
It owns only [README.md](../README.md), [MIGRATION.md](MIGRATION.md) and this report.
No production code, release recipe, workflow, installation or runtime was changed.
Central retains design, publication and milestone acceptance.

## Delivered guidance and source checks

The landing page leads with the installed native-style `-f` report. The guide
covers prerequisites, offline private installation after dependency fetch,
source-tree commands, separate runtime selection, evaluation versus fitting,
file side effects, actual Rust diagnostics and the Rust/predictor C interfaces.
It uses portable paths and links to existing contracts instead of duplicating
API inventories or a scientific-failure ledger.

| Claim | Inspected authority |
| --- | --- |
| Locked dependencies, default features, native compiler selection | [`Cargo.toml`](../rust/Cargo.toml), `Cargo.lock`, [`build.rs`](../rust/build.rs); pinned `gmp-mpfr-sys` 1.7.1 README in the existing Cargo cache for build tools/path restrictions |
| Fresh-prefix guard, offline build, installed membership and receipt | [`install.py`](../rust/install.py), especially prefix handling, Cargo commands and source-copy list; existing completed install receipt independently hash-checked below |
| Native-first CLI, help statuses, explicit JSON dispatch | [`main.rs`](../rust/src/main.rs), [`core_cli.rs`](../rust/src/core_cli.rs), [`TERMINAL_EXAMPLES.md`](../rust/TERMINAL_EXAMPLES.md) |
| Native evaluation/fit/recompute order; `-pre` still fits | `core_cli::run`'s NITS/iteration loops and [`CORE_OUTPUT_API.md`](../rust/CORE_OUTPUT_API.md) |
| General JSON evaluation/mean/fit controls; bounded residual/fit scope | `main::run_bats`, [`timing.rs`](../rust/src/timing.rs), [`general_fitting.rs`](../rust/src/general_fitting.rs), [`README.md`](../rust/README.md) |
| Output names, suffixes, working-directory and partial-write behavior | [`core_output.rs`](../rust/src/core_output.rs), [`core_output_source.rs`](../rust/src/core_output_source.rs), `core_cli.rs`, [`polyco.rs`](../rust/src/polyco.rs), predictor API |
| Runtime/consumed-file provenance and native clock override | [`runtime.rs`](../rust/src/runtime.rs), [`bats.rs`](../rust/src/bats.rs), `core_cli.rs`, [`INPUT_TO_BAT_CONTRACT.md`](../rust/INPUT_TO_BAT_CONTRACT.md) |
| Truthful emitter metadata | [`clock_diagnostics.rs`](../rust/src/clock_diagnostics.rs), actual installed emitter lines, [0107](../decisions/0107-truthful-rust-diagnostic-attribution.md), [0108](../decisions/0108-terminal-presentation-acceptance.md) |
| Rust ownership and standalone predictor C scope | [`LIBRARY_INTERFACES_API.md`](../rust/LIBRARY_INTERFACES_API.md), [`PREDICTOR_API.md`](../rust/PREDICTOR_API.md), [`get_period.c`](../rust/native/get_period.c), [0084](../decisions/0084-library-interfaces-and-predictor-c-contract.md) |
| Feature/provider selection and different native/JSON acceleration defaults | [`NATIVE_BACKENDS_API.md`](../rust/NATIVE_BACKENDS_API.md), [`t2accel.rs`](../rust/src/t2accel.rs), `main.rs`, `install.py` |
| Process-state/concurrency limits and measured costs | [`PARALLEL_EXECUTION_API.md`](../rust/PARALLEL_EXECUTION_API.md), [`PARALLEL_EXECUTION_REPORT.md`](../rust/PARALLEL_EXECUTION_REPORT.md), [0099](../decisions/0099-source-fidelity-and-deferred-covariance-profiling.md) |
| Implemented capability versus accepted migration | [0074](../decisions/0074-implementation-handover-and-acceptance.md), [0086](../decisions/0086-release-platforms-and-integrated-qualification.md), [qualification report](../rust/RELEASE_QUALIFICATION_REPORT.md), [0112](../decisions/0112-fourth-linux-candidate-review-and-execution.md) |

Two older README statements needed scope clarification in the new guide:
the general input-to-BAT section says CALCEPH requests are unavailable, but current
`build.rs`/`calceph.rs` and the native-backend contract implement explicitly enabled
CALCEPH. Also, runtime-free barycentric evaluation describes the bounded adapter;
`main::run_bats` requires `--runtime` for general `bats`/`timing`, even for the
guide's `@` fixture. The guide states the actual feature/route distinction.
No behavioral correction was necessary or made; the frozen older files remain
unchanged.

## Exact source and executable identities

- Assignment release, initially clean: `a8752fcec2af65e5756e6cfc41d85f0de23e4a3f`.
- Frozen qualification candidate: `fecbc18af95196a16db2046306099b6345a50976`,
  Linux run `41294531402`, job `124812485584`, attempt 1. AM did not query,
  dispatch or change that run and makes no assertion about its result.
- Executed installation source: **`a8ba6f15d2933b206f85ce70c99003b10f3f653d`**,
  exported under `validation/release/build/third-cache-candidate-v1/source`.
- Executed binary: `validation/release/build/third-cache-default-v1/bin/tempo2-rust`,
  SHA-256 **`df523407e7db73ba3dabe27377252627fafe7ae466d9922fabe95a6865c62e5f`**.
  Release profile, default features, no optional provider.
- Native source pin: `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.

The completed installer manifest's **263 files** were checked for exact membership
and hashes. All **257 packaged source/document files** also matched the candidate
export and their bytes at the exact Git source commit. This establishes the
executed artifact's provenance; it does not relabel it as a build of `fecbc18`.
The later production-source change is the LMST platform branch in
`get_obs_coord.rs`; the remaining source changes add test modules/tests, alongside
report updates. The selected CLI, input, timing and fitting bodies are unchanged.
The new branch preserves the prior macOS expression, and the example uses `@`
observations. None of this establishes Linux qualification of the older artifact.

Central concurrently committed documentation after the assignment release.
The successful demonstration identity records checkout HEAD
`0d032c9a7ac907e0c3bbf667e162490a8e510709`; later central handoff work was preserved.
The tracked `rust/`, `validation/release/` and qualification workflow contents
remain identical to frozen `fecbc18`, independently of documentation HEAD.

Reused, existing receipts (all paths relative to `validation/release/build/`):

| Receipt | SHA-256 |
| --- | --- |
| `third-cache-candidate-v1/receipt.json` | `05c9fd852edcc575e5378c02c264b1399c3cd060838659642b86d9ea90bf2e83` |
| `third-cache-default-v1-build/install.json` | `e6bc616cb061a876c9cf124f25d8b17f95f9add94d9a7f92f4afafc0a431faa4` |
| `integrated-v1/a2/receipt.json` | `9a02b11d502fefb7a422da9a6d90e3a235756316c041f8f91f4cca543d68e7ea` |
| `third-output-v2/receipt.json` (prior explicit installed-CLI binding) | `8c62fe480ee6bb9075c2b9ba8908040167a633c0652365055ad521bec9d33001` |

The prior library and CLI build logs end with successful release builds; their
hashes are `d1cff433f1ed80bc99bec81d2a3ddc05a6b5dfcb6d5c12f721c96fe78e152968`
and `d4240e64e5bdc50597f09b245f8182b9c53447090a91d55c7ea03f065d773591`.
Build/install evidence was reused rather than regenerated.

## Narrow executions and observed results

All new evidence is ignored under `docs/build/am-migration-20261101-v2/`.
`verify_examples.py` records exact absolute argv, per-command fresh working
directories, exit codes, complete raw stdout/stderr, product membership and hashes
in `commands.json`. No products were written into the existing installation,
qualification evidence or runtime. `check_examples.py` checks the saved outputs.

Execution host: macOS **26.7 (25G229), arm64**, Python **3.11.2**. Read-only
tool queries reported rustc **1.97.1** (`8bab26f4f68e0e26f0bb7960be334d5b520ea452`),
Cargo **1.97.1**, Apple clang **21.0.0**, GFortran **16.2.0**. Those are current
tool observations, not a fresh compilation. `otool -L` identifies libSystem and
libiconv as the main executable's dynamic dependencies. Each process inherited
only `PATH`, `HOME`, `TMPDIR`, plus explicit `LC_ALL=C` and the named `TEMPO2`;
other runtime/plugin/loader overrides were not inherited.

Inputs, unchanged before/after execution:

| Input | SHA-256 |
| --- | --- |
| `validation/general_fitting/inputs/robust-H.par` | `02caccd9154c46b62dec9951c695045cc092b661aa76bc3beb6c4c85ef98da60` |
| `validation/general_fitting/inputs/multi-frequency.tim` | `7a4c6c072a38133490baf228a6efee32a98bd386c7d8a52545e83141fa59245e` |

Runtime: `validation/release/build/integrated-v1/a2/runtime`. All **208 files**
matched the original A2 manifest before and after execution. No historical-clock
overlay was used. JSON timing records consuming `ephemeris/DE405.1950.2050`
(`86786f07aeddc1c5b6ddb24d8c65930803458938c6a2f920e8ea72a86da7479c`) and
`ephemeris/TIMEEPH_short.te405`
(`a455fe86dd66c173ebb5a58a6fe12f65b3e9c21c2000be235b669f84d5561565`).
These are fixture resources, not a recommended substitute for users' runtime data.

In the table, `PAR`, `TIM` and `RUNTIME` denote the exact paths above; the binary
is the identified private installation. All stdout was captured through a pipe.

| Command/selection | Exit | Observed result |
| --- | --- | --- |
| `--help` | 0 | Native workflow precedes JSON routes; evaluation/fit and plugin boundaries explicit. |
| `-h`, `-H`, `-v`, `-version`, no arguments | 1 each | Expected native help/identity behavior; empty stderr with runtime set. |
| `-f PAR TIM` | 0 | Human report, 3,825 stdout bytes, empty stderr. |
| `-npsr 1 -nobs 100 -noaccel -f PAR TIM` | 0 | Robust-H human report, 3,825 stdout bytes, empty stderr. |
| Previous command plus `-newpar -residuals -writetim -displayVersion` | 0 | 5,611 stdout bytes; exactly `new.par`, `residuals.dat`, `out.tim` produced; empty stderr. |
| `timing --par PAR --tim TIM --runtime RUNTIME` | 0 | Complete JSON, sixteen observations, `remove_mean=true`; no fit cycles or parameter mutations in this fixture. |
| Previous evaluation plus `--no-mean` | 0 | `remove_mean=false`; phase/nphase/pulse-number fields unchanged for all sixteen observations. |
| `timing --fit --par PAR --tim TIM --runtime RUNTIME` | 0 | `tempo2-general-fit-v1`, `fit_performed=true`, one cycle, ZERO and F0 fit coordinates, acceleration mode 0. |
| Previous fit plus `--no-mean` | 1 | Incompatible-control error on stderr (75 bytes), no stdout. |
| `bats --par PAR --tim TIM --runtime RUNTIME` | 0 | Complete JSON with sixteen observations. |
| `python3 rust/install.py --help` | 0 | Installer options/source-described fresh-prefix contract; installation not invoked. |

There are **21 recorded commands**, including six host/tool/dependency queries,
the installer help and fourteen main-CLI invocations. All returned their expected
statuses. Fifty output-contract assertions pass, including **24 unique actual
Rust emit sites** checked against the installed source, text-report sections,
mean/fit distinctions, exact requested file membership, string-valued extended
quantities and runtime hashes. These assertions check documentation behavior,
not independent numerical correctness.

Selected lines actually observed in `native-robust/stdout`:

```text
[general_fitting.rs:210] ss/fs 0 0
[tkrobust.rs:124] Doing robust fit!
[tkfit.rs:365] SVD nparams=2 nconst=0 ndata=16
[core_cli.rs:819] Complete fit
RMS pre-fit residual = 16.875 (us), RMS post-fit residual = 3.673 (us)
```

Only selected lines are quoted; complete output and all repeated SVD messages
remain in the raw evidence. No native/Rust scientific comparison was run.

The initial local recorder in `docs/build/am-migration-20261101/` stopped after
`-h`: directory labels `native-h` and `native-H` collide on this host's
case-insensitive filesystem. This was a documentation-check harness error,
before the `-H` process launched. Its script and nine completed command records
remain intact. The successor used `native-long-help` in a fresh evidence root;
it made no application change or altered expectation.

Key new receipt hashes:

| Artifact under `docs/build/am-migration-20261101-v2/` | SHA-256 |
| --- | --- |
| `identity.json` | `e0297e9225e7015171ad031483415dd99468c1cb9b817caf7b71630dc2fc70e8` |
| `commands.json` | `b0b57ae3ede6b82e67ec13ee6e636627e8196597cebef1eb3d89336637c9eea6` |
| `checks.json` | `3e01220d4efd57465e7a66f40e482e5684da0efef47734ea3ffccc4f77b28d25` |
| `verify_examples.py` | `cac19a2add1ce87aa72caf5ae8f9d8a052a8de221e7b0fe2902b219ff0bde3ce` |
| `check_examples.py` | `1cebd210030dbace7cbb9acfd044db77397ce9e42084a819a9bf23b1561cb921` |

## Unexecuted steps and remaining boundaries

- Dependency fetch, a new Cargo build and a new install were source-checked and
  supported by the unchanged installer/build receipts, not rerun. A fresh checkout
  needs its own toolchain, dependency cache and supplied runtime. Placeholder
  user paths were instantiated with the named fixture/install for execution.
- Predictor generation/evaluation/plans, C/Rust external consumer compilation,
  optional backend builds, multiple pulsars, `-pre`, `-outpar`, model-specific
  auxiliary files, bounded `residuals`/`fit`, terrestrial and real-data workflows
  were source-checked or linked to their existing contracts, not newly executed.
- Linux execution, the full test suite, benchmarks, sanitizer campaigns and the
  qualification matrix were not run by AM. Existing failures and acceptance gates
  remain owned by central/AL; this delivery accepts neither AL nor M2 nor migration.
- The current installer copies `rust/*.md`, not the new root README or `docs/`
  guide. This delivery provides repository onboarding. Shipping these new pages
  inside an installed package would require a later packaging change outside AM.
- Older contract documents retain historical examples/statuses. The new guide
  resolves the concrete route/feature ambiguities above and points to current
  qualification/central records; no broad documentation rewrite was attempted.

## Documentation verification and commits

All **68 relative links** resolve, including heading fragments, and all **eight
shell blocks** pass `sh -n`. All three documents pass whitespace/portable-link
checks. `check_docs.py` and `docs-checks.json` retain the recipe, document hashes
and individual links under the successful evidence root. No Markdown renderer
was run. Git's scoped `diff --check` also passes.

User-facing documentation commit:
**`81af30989aa39748a65d13a015397d8ab997ca9e`**, `docs: add TEMPO2 user migration guide`
(`README.md`, `docs/MIGRATION.md`), on preserved central HEAD
`1fad8ce2d2588496845bdd6928353906f2aed5e1`. This report follows in a separate
`docs:` commit; its exact identity is returned in the final handoff.

Only authored owned paths were staged/committed. Frozen implementation/workflow
files match `fecbc18`; executable, inputs and runtime match their pre-run hashes.
Generated evidence remains ignored. No push, workflow dispatch, broad suite or
optimization was performed. This bounded assignment stops for central review.
