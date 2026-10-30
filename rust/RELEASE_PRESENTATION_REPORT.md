# AL terminal presentation and Rust attribution

31 October 2026. Decisions [0106](../decisions/0106-migration-familiarity-and-terminal-output.md)
and [0107](../decisions/0107-truthful-rust-diagnostic-attribution.md), released at
`8b0fbc8839fc37c340d6fed001b64a8bc722bf02`, authorize this presentation-only work.
Production is **`801bc94139cca90d6072234b09f5a1bcec5e4b05`**; the corrected release
checker is **`0b83606cad069bbfa23ca83bcf642c121a2e244f`**. Subsequent report commits
preserve those production bodies. This is not AL scientific or Linux acceptance.
Central retains publication, workflow dispatch and acceptance.

## User-facing behavior

The main executable, README and installed `TERMINAL_EXAMPLES.md` now lead with
`tempo2-rust -f model.par arrivals.tim`, runtime setup and the native text report.
`--help` reuses `core_cli::help()` and the retained source option catalogue.
The obsolete CVS-version description is corrected and the plugin execution
boundary is explicit, including native `-h` and `-H`. Their primary example is
runnable without a plugin. Source help files themselves remain unchanged.
Native `-h`, `-H` and `-v` still return 1; explicit `--help` returns 0.

JSON stays an explicit machine interface. Native `-pre` still fits and skips
its second evaluation; `timing` evaluates unless `--fit` is requested. The
bounded residual and ZERO/F0–F3 fit interfaces retain their scopes. There is no
TTY-based format switch, additional calculation path or change to PAR controls.

## Source and departure map

The source-bound index contains **193 log/timing emit sites and 41 version sites**.
Each comparison receipt includes the exact Rust source hashes, compiler line
numbers and named native counterparts. Native source remains pinned to
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`.

| Responsibility | Implementation and preserved contract |
| --- | --- |
| Rust diagnostic entry | `clock_diagnostics::rust_log` and `rust_timing` use `Location::caller()` and the Rust basename. Native file/line arguments move into adjacent `TEMPO2` comments. Existing rendering, severity, routing, counters and storage are shared. |
| Thin wrappers | `core_cli::main_log` and `cholesky::source_log` capture the tracked caller before entering a closure. `clkcorr::log_loaded` propagates its caller. Native line-only arguments and tuple fields are removed; message construction is unchanged. |
| Deferred observation events | Log events retain their captured file bytes and line; version events retain the captured location. Replay uses the shared recorder without recapturing its own location or applying another source-directory prefix. |
| Explicit external metadata | Public `log`/`timing` still accept the actual emitter's metadata. Their source-directory prefix preserves bytes, including non-UTF8. Rust captures do not inherit that prefix. Genuine provider/native attribution and filenames inside bodies remain data. |
| Version rows | One Rust package/source-pin/Rug128 identity precedes actual Rust file/line plus an explicitly named TEMPO2 counterpart. The original counterpart pair remains the once-only key, preserving suppression across multiple Rust owners of one native routine. No native CVS/header version is presented as the Rust build. |
| Generated report source | `attribute_generated_output.py` is the explicit post-generation successor. The historical generator runs only in a disposable export. Its 14 log sites convert to compiler captures plus comments; rustfmt then reproduces committed `core_output_source.rs` byte for byte. The source audit rejects restoration of literal native-labelled Rust log/timing calls. |
| Owned comparison adapters | `check_output.py` and `check_timing.py` retain their native line relocation and scientific gates, then validate Rust emit metadata before one named projection. Unknown Rust sites, wrong counterparts and internal comparison labels in raw input fail. Message bytes, numerical data, field/file membership and order remain binding. |

When several native branches share one Rust emitter, the index explicitly names
that native line set; it does not infer a line from body values. `Diagnostics`
summary lines identify `clock_diagnostics.rs`, where those messages are built.
Two EOP branches remain distinct to retain their distinct emission sites, with a
narrow explained Clippy allowance. No arithmetic, units, reference frames,
precision, tolerances or cache ownership changed.

## Actual installed before/after example

A clean Git export of production `801bc94` contains 2,008 files. Its private
install contains 262 files; package audit passed 2,270 checks with no mismatches.
The installed executable SHA-256 is
`e80a89f305c47e4f09f580b1b7ddb95c8ff6df9e07abac56adaa5d0d9aea698b`.
The retained pre-change installed executable is
`11743d8d141af0b8b2648607dc2a085d035e83a47d9b476396f233e47d7eac0b`.
Runtime is the separately identified A2 build under
`validation/release/build/integrated-v1/a2/runtime`.

From the repository root, the demonstrated command is:

```sh
export TEMPO2="$PWD/validation/release/build/integrated-v1/a2/runtime"
validation/release/build/presentation-install-v1/bin/tempo2-rust \
  -npsr 1 -nobs 100 -noaccel \
  -f validation/release/build/presentation-candidate-v1/source/validation/general_fitting/inputs/robust-H.par \
     validation/release/build/presentation-candidate-v1/source/validation/general_fitting/inputs/multi-frequency.tim
```

The campaign records absolute arguments and runs in fresh working directories.
Selected original lines, with repeated SVD lines omitted here only:

```text
[t2fit.C:850] ss/fs 0 0
[t2fit.C:879] START=53910.125 FINISH = 54150.1250000883665
[TKrobust.C:25] Doing robust fit!
[TKfit.C:345] SVD nparams=2 nconst=0 ndata=16
[tempo2.C:566] Complete fit
RMS pre-fit residual = 16.875 (us), RMS post-fit residual = 3.673 (us)
```

The corresponding current lines are:

```text
[general_fitting.rs:210] ss/fs 0 0
[general_fitting.rs:250] START=53910.125 FINISH = 54150.1250000883665
[tkrobust.rs:124] Doing robust fit!
[tkfit.rs:365] SVD nparams=2 nconst=0 ndata=16
[core_cli.rs:819] Complete fit
RMS pre-fit residual = 16.875 (us), RMS post-fit residual = 3.673 (us)
```

All repetitions, the complete parameter report and both empty stderr streams are
retained. The complete report is 3,780 bytes before and 3,825 bytes after; it
compares exactly after validated attribution and the existing CPU-time allowance.
The version header and first row now read:

```text
[TEMPO2-RUST VERSION:] tempo2-rust 0.1.0 (TEMPO2 source fda6ad4a9753ad7dc5d291406f2f310a9161423f; Rug128)
[TEMPO2-RUST VERSION:] [core_cli.rs:505] [TEMPO2 counterpart: tempo2.C::main()]
```

## Verification and retained failures

Evidence paths below are relative to `validation/release/build/`.

| Evidence | Result and scope |
| --- | --- |
| `presentation-installed-v2/receipt.json` | 20 installed commands; all 18 checks pass. Normal/version/debug reports agree, every debug file matches, help scopes/statuses are correct, and `bats`, `timing`, `timing --fit` JSON products and stderr agree after validated diagnostic metadata. This is a before/after regression check, not an independent scientific reference. |
| `presentation-output-v1/` and `presentation-final-checks-v2/projection-receipt.json` | Fresh native and clean-export Rust `format`, `helpers`, `log`, `snapshot` executions; complete membership and bytes agree. The snapshot includes report, PAR, residuals, covariance and mutation state. Retained raw products were re-compared successfully after the checker correction. |
| `presentation-timing-v2/receipt.json` | Six independent native/Rust comparisons pass: ordinary, events and tempo1, each with mean 0/1. Existing numerical thresholds and diagnostic channel/order checks are unchanged. Lifecycle scenarios were not selected. |
| `presentation-v1/unit.stdout` | Working-tree suite: 99 passed, 37 ignored. Later source changes before the production commit were the explained EOP lint allowance and test initialization style. |
| `presentation-v1/unit-final.stdout` | Clean `801bc94` export: 98 passed, one failed, 37 ignored. The failure at `fitting_tests.rs:31` was missing generated `validation/fitting/cases.json`, before any numerical assertion. The original failed run remains. |
| `presentation-m2-v2/` and `presentation-v1/m2-unit-final.stdout` | Existing M2 recipe prepared native inputs in that disposable export using verified A2 and `NATIVE_CXX=g++-16`; the affected fitting test then passed. The first preparation attempt used Apple's `g++` alias and failed on missing `quadmath.h`; `presentation-m2-v1/` retains it. No generated JSON was committed, test weakened or production modified. The other 98 passing tests were not repeated. |
| `presentation-v1/diagnostics-final.stdout`, `harness-final.stderr`, `clippy-fixed.stderr` | Three focused diagnostic tests, all 22 release Python tests, rustfmt and all-target Clippy with `-D warnings` pass. Initial Clippy failures and the earlier 21-test run remain separately recorded. Worker ordering and concurrent workflow contracts also passed in the library suites. |
| `presentation-source-audit-v1/` and `presentation-source-audit-v2/` | 93 library units; 157 Rust files, 117 unchanged and 40 changed against the release-audit baseline; 63 lexical unsafe mentions; all 36 SOFA sources identical. The second audit binds the updated source-map document. These counts include earlier AL changes and are navigation/identity evidence, not exhaustive branch coverage. |

The 359-byte **full-line** truncation still occurs before ANSI removal. The
installed-library diagnostic client records 176 rows across two terminal modes,
44 body lengths and two prefix roles. For this client's actual
`diagnostic_controls.rs` label versus its clearly synthetic old `TKfit.C:345`
prefix, a 400-byte body retains 299 versus 313 body bytes: **14 more bytes are
truncated**. The largest body retaining a newline is 298 versus 312 bytes in
both modes. Warning count and the original `min(body_length, 359)` backing
storage remain exact. This is prefix-specific diagnostic evidence, not a claim
that every production line loses 14 bytes. All raw rows and bytes are in
`presentation-boundaries-v2/`; its first client compile failure is retained in
`presentation-boundaries-v1/`. The comparator does not restore a truncated tail.

Central independently verified the production install and presentation products
in `/tmp/tempo2-central-presentation-review-v2-20261031/review.json`, SHA-256
`0459603a9fdbbe88a7db1bd3e707361534b76b055a90fdb1b618020f32969ff4`.
It subsequently demonstrated three checker false acceptances: an internal
`[TEMPO2:...]` label substituted for a Rust emit site, and internal source-version
tags substituted for the Rust identity or version call site. Those failed
negative controls are retained in
`/tmp/tempo2-central-attribution-prefix-control-20261031/`, separately from the
correct production output. Checker `0b83606` rejects all three at the raw-input
boundary, including ANSI-prefixed ordinary tags; literal tags inside message
bodies stay data. The original four control families still pass, and the added
fifth family protects this regression. All three exact central mutations are
rejected by `presentation-final-checks-v2/recheck_projection.py` while the
unaltered before/after streams still agree.

Final projection SHA-256:
`a06a9ed8d47c9b8f8a347e4904fc1db6fe3697fdced4a7cfa99480da9f387071`.
The current installed campaign receipt is
`d9d779901d16fde1d22292a8b68c8806c08d5eef894f3efca265f14166cbda40`;
the exact raw-product recheck receipt is
`2a81fdf0e5d8bcfff2995efe974c94ab011cda756ddf8496de39f0652019f1d5`.
The final review packet records the documentation candidate, all selected raw
artifact hashes and archive readback in `presentation-review-v1/REVIEW.md`.

## Remaining qualification boundary

The 37 ignored library cases are not reported as a passing corpus campaign.
Only the explicitly named output probe was separately run here. Historical
exact-native-label expectations remain immutable and require their own 0107
metadata successor when next replayed; this delivery does not claim full
integrated-corpus qualification. The generated source guard and owned adapters
prevent silent restoration of the old labels within their scoped paths.

Central's separate Linux run uses frozen `a35104a611abb5c1d5c48dc4d1866f239ccd3ab9`;
this work neither substitutes its candidate nor asserts its outcome. Further
ordinary-type proposals and the cache ownership change remain outside this
presentation task. The existing scientific gates, failures and acceptance
requirements in the qualification reports remain binding.
