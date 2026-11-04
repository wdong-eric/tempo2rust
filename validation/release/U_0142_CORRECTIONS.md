# U platform corrections and integrated Linux preparation

AL handover, 4 November 2026, under
[0142](../../decisions/0142-u-platform-correction-and-linux-candidate-preparation.md).
Production commit `af6639a` corrects the six released U expressions;
`b59300b` adds their object capture and the accepted coefficient-view/GUI
qualification plumbing. This is a locally checked AL slice for central review,
not the freeze or acceptance of a combined Linux candidate.

## Arithmetic change and controls

The [0137 diagnosis](U_0137_DIAGNOSIS.md) supplies the saved discriminating
operands and counterfactuals. The implementation follows the existing
`read_ephemeris::telescope_offset` measured-platform pattern: macOS arm64 keeps
the current FMAs; Linux x86_64 uses source-ordered binary64 operations. It
changes only the following production expressions, with source/evidence
references beside each one:

| Source site | Linux operation |
| --- | --- |
| `ifteph.C:433` | Rounded multiply, then subtraction for `pc` |
| `ifteph.C:459` | Rounded multiply, two ordered additions, then subtraction for `vc` |
| `ifteph.C:442` | Separate products in the descending position accumulation |
| `tt2tdb.C:299` | Rounded multiply, then subtraction in the FB recurrence |
| `tt2tdb.C:304` | Separate products in the descending FB accumulation |
| `tempo2Util.C:43` | Three rounded products and the left-to-right dot sum |

The dot change is at the shared f64 helper. Propagation, troposphere, position/
standard fitting and constraints retain that call path. GW simulation uses its
own Rug128 `Float` helper and is outside this change. Coefficient ordering,
caches, resets, epoch selection, units, conversions and Rug128 timing are
unchanged. The unisolated record-origin and velocity-accumulation expressions
remain untouched. No tolerance or nonfinite acceptance policy changed.

Four new production-path tests bind the actual IFTE `pc`/`vc` inputs, the
`tt_zero_vectors` reverse sum, FB counts 3/8/16 and the zero-deltaT dot witness.
Each retains both platform targets. The interpolation tests call the production
cache method; the FB test writes the existing authored binary layout and calls
the normal file/cache path. A generic polynomial evaluator or alternate timing
model would change the exact operation contract and was not introduced.

Local macOS verification:

- Debug and release U selections each pass eight tests, including the four new
  witnesses. The external-input probe remains ignored in that unit selection.
- A separate fresh release replay executes all 97 U histories against the
  unchanged, hash-bound corrected native records: zero differences after the
  existing named diagnostic-attribution projection. Complete state, diagnostic,
  error, file and bit/sign comparisons remain active.
- Four focused shared-caller checks pass in both profiles: terrestrial fit
  failure/recovery, fit controls with fixed evaluation, canonical constraint
  callback snapshots and wet mapping at latitude knots.
- Formatting and scoped diff checks pass.

The first history-run attempt used a relative output path and stopped before
the first history completed. Its recipe/log are retained; the successful fresh
attempt uses absolute paths. No native scientific campaign was rerun. These
checks preserve macOS behavior; they do not demonstrate Linux correction.

## Actual U object capture

`time_ephemeris_objects.py` uses the existing command/receipt infrastructure.
The corpus runner calls it immediately after the U comparison, before later
builds can replace its shared Rust test artifact. It requires complete U
membership, binds the Rust report to the native receipt, and checks each actual
executable against its recorded hash before and after disassembly. Numerical
comparison failures do not suppress capture; one executable's identity or
inspection failure does not suppress the other executable's evidence.

The retained `native.s`, `rust.s`, command logs and receipt live under
`corpus/time-ephemeris-objects/`, outside excluded build/source directories.
The local macOS invocation captured the saved corrected native executable and
the freshly executed Rust binary. This tests the capture path, not Linux
instructions. The old Linux artifact still contains only recorded executable
identities; it has not acquired historical machine-code bytes retroactively.

On the next actual Linux run, inspect the six changed sites and the unisolated
`ifteph.C:250/471` sites in this new disassembly. Verify all 97 histories and
their later snapshots before closing any of the original 300 differing leaves.
New demonstrated discrepancies require central review.

## Predictor pair and GUI preparation

The coefficient-view pair is exactly `e24622d5210fdfc8abe10f9f58a21b7e4a2a1aca`
to `a1851c55fcc0b1ac733b064e631bfbbd2e0118d5`. Its before install, qualifier and
direct clients reuse the recurrence after endpoint. The new after source is
explicitly fetched, with per-fetch failure evidence retained. The existing C
coefficient witness, direct/full-C/legacy comparisons, concurrency and canonical
installed consumers execute separately. The obsolete manual borrow-copy loop
was removed from [COEFFICIENT_VIEW.md](../predictor_performance/COEFFICIENT_VIEW.md).

Both predictor library profiles run, and Linux gates require the named wide
recurrence, ordinary-owner and native-subnormal tests to execute successfully.
The installed library disassembly is retained. Direct and full-C measurements
keep the 100,000-iteration, seven-sample workloads and separate native failures.
A failed C coefficient witness blocks its full-C measurement; direct public-
owner measurements retain their own verification dependency. No new pair
measurement was performed here. AO's later `e1d34d8` endpoint is not wired by
this slice; its disposition belongs to central.

GUI integration uses root's existing source and tests, including Linux native
preparation `67a5748`. No `gui/` file was edited. For the fresh candidate it:

1. Fetches locked GUI dependencies separately, then uses standard offline
   `cargo install` for the release viewer and a separate GUI target directory.
2. Records `ldd` output for the installed binary; runs the existing library
   checks, control renderer and full CLI/session/save/reload parity script.
3. Supplies the explicit A2 runtime and fresh output paths, then executes the
   prepared native test under Xvfb/Mesa with a 60-second process timeout.
4. Requires actual named control/native tests, complete byte equality between
   native timing JSON and the matching CLI result, all seven PNG products,
   OpenGL context identity and saved PAR/TIM/JSON files. It binds the installed,
   example and test executables and the linked-library identities.

The five build packages follow the pinned eframe README; X11/Mesa/Vulkan runtime
packages support the declared native and test renderers. GUI provisioning is
limited to the full workflow scope. Graphics/build stages all precede the
sequential predictor measurement block. GUI failures remain independent of
unrelated scientific stages. The library inventory describes linked libraries;
it does not claim an exhaustive inventory of dynamically opened graphics
drivers. PNG signatures/dimensions are checked automatically; central must
inspect actual Linux images. No Linux GUI compilation, native execution,
Wayland qualification or deferred macOS desktop launch is claimed here.

## Retention, checks and delivery

The collector retains GUI PNG/PAR/TIM products under the platform's GUI output,
alongside existing JSON/context/failure logs. Source, target and install trees
remain excluded. Per central's [0144](../../decisions/0144-event-nan-localization-and-object-evidence.md),
only the exact `event-source-profile/machine-code.tar.gz` path under a platform
root is admitted; unrelated and nested archives remain excluded. Central owns
the event capture implementation and its unchanged-arithmetic evidence.

All 43 final focused Python controls pass. They cover U capture identity and
ordering after failed comparisons, coefficient/recurrence baseline reuse and
failure routing, actual range-log gates, GUI failure isolation and strict
evidence checks, collector inclusions/exclusions, history staging, and central's
two event-profile controls. Compiler/render/process boundaries are mocked in
the plumbing controls; the local U tests and 97-history replay above are real
Rust executions. The original Linux result remains 86 passing / 11 failing.

Evidence is under `decisions/evidence/release-qualification/build/u-0142-20261104/`.
`delivery.json` binds the committed sources, reports, local logs, replay inputs/
outputs and prior immutable deliveries. Central retains combined-candidate
freeze, AO coordination, Linux dispatch, image review, performance budgets and
acceptance. No push or dispatch occurred.
