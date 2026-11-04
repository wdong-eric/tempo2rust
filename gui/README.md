# TEMPO2-Rust timing viewer

An interactive timing workflow for one PAR/TIM pair: residuals with TOA
uncertainty bars, zoom/pan/reset, exact TOA inspection, exclusion/reinclusion,
refitting and saving. Loading runs the existing Rust load/preprocess/timing
workflow with residual mean removal; fitting runs only when **Refit** is clicked.

From the repository root, with the engine's [native build prerequisites](../rust/README.md)
installed:

```sh
cargo run --manifest-path gui/Cargo.toml --locked
```

Enter the PAR and TIM paths and a complete TEMPO2 resource directory. `TEMPO2`
initializes the resource field when set. Click **Load and evaluate**. The engine
runs on a worker thread; errors and diagnostics remain visible in the viewer.
Scroll or pinch to zoom, drag to pan, and double-click or **Reset view** to reset.
Click a point or use **Inspect TOA** to read its exact values and original input
identity. Active TOAs are blue circles; deleted TOAs are orange crosses.

**Exclude TOA** / **Include TOA** updates the selected original TOA and
reevaluates the current model. **Refit** uses the loaded files' fit flags, mode
and NITS with the general Rust fitter's default options, including error
rescaling. This first interface has no covariance selector or parameter editor.
The **Fit result** panel exposes the complete engine report. A failed edit/fit
keeps the last valid model and result; the engine's owner-aware snapshot supplies
that isolation at the cost of a temporary model copy.

After a successful fit, enter a **New save folder** and click **Save session**.
The directory must not exist. The existing engine writers create `session.par`
and `session.tim`; `session.json` retains exact timing/fit output and provenance.
Engine stdout/stderr and any additional writer products stay in the same folder.
Saving uses a snapshot because the PAR writer derives reporting/phase-reference
fields such as TZRMJD. It leaves the live fitted model unchanged. Native TIM
format preserves excluded TOAs as `C` comments, which the reader skips on reload.
The PAR/TIM exports use native text precision; they are not lossless engine-state
serialization. If a save fails, the error identifies the retained partial folder.
Another selection edit requires a new fit before saving. NITS zero returns no
fit cycles and does not enable saving.

The plot answers how the evaluated residual and its reported uncertainty vary
with site arrival time. The screen uses binary64 MJD and microsecond coordinates;
the original engine values remain unchanged and are shown in the detail panel.
Unplottable/nonfinite values are reported instead of silently dropping rows.
This is an exploratory view, not a publication figure or new scientific
qualification of the timing engine.

## Frontend and dependencies

This separate Cargo package uses pinned `eframe 0.37.0` and `egui_plot 0.38.0`.
The native app uses eframe's OpenGL (`glow`) renderer and enables its macOS and
Linux X11/Wayland support. The GUI lockfile preserves every engine dependency's
version/checksum. Building requires Rust 1.95 or newer as declared by eframe;
the engine still has its documented GMP/MPFR, SOFA Fortran and optional backend
dependencies. The viewer does not add a timing model or a C scientific kernel.

For Linux, the [pinned eframe build instructions](https://github.com/emilk/egui/blob/0.37.0/crates/eframe/README.md)
list `libxcb-render0-dev`, `libxcb-shape0-dev`, `libxcb-xfixes0-dev`,
`libxkbcommon-dev` and `libssl-dev` on Debian/Ubuntu. A working X11/Wayland session
and OpenGL driver are needed for the native window. The test-only WGPU renderer
also needs an available graphics adapter. Actual Linux build, runtime library
inventory and window/render checks remain release qualification work.

## Private installation

Use [Cargo's standard installer](https://doc.rust-lang.org/cargo/commands/cargo-install.html)
from this checkout; no GUI packaging wrapper is required:

```sh
cargo install --path gui --root /absolute/path/to/fresh/gui-prefix \
  --locked --bin tempo2-gui
TEMPO2=/absolute/path/to/T2runtime /absolute/path/to/fresh/gui-prefix/bin/tempo2-gui
```

This builds the release profile and installs `bin/tempo2-gui` plus Cargo's
tracking metadata. Use a fresh private prefix for qualification and retain the
checkout revision and lockfile. Add `--offline` after preparing the dependency
cache. The GUI links the same engine source through its existing path dependency;
it does not load a separate predictor shared library. Supply the TEMPO2 resource
directory separately, as for the core installer. This is a command-line
installation, not a signed macOS application bundle or Linux desktop package.
Native launch on both platforms remains pending.

## Focused verification

```sh
cargo fmt --manifest-path gui/Cargo.toml --check
cargo clippy --manifest-path gui/Cargo.toml --locked --all-targets -- -D warnings
cargo test --manifest-path gui/Cargo.toml --locked
cargo test --manifest-path gui/Cargo.toml --locked --lib -- --include-ignored
cargo build --manifest-path gui/Cargo.toml --locked --examples
cargo build --manifest-path rust/Cargo.toml --locked --bin tempo2-rust
python3 gui/verify.py \
  --cli rust/target/debug/tempo2-rust \
  --viewer-export gui/target/debug/examples/export_timing \
  --session-workflow gui/target/debug/examples/session_workflow \
  --runtime validation/reference/build/source/T2runtime \
  --out gui/build/parity
```

Use fresh output directories; adjust executable paths if `CARGO_TARGET_DIR` is
set. The parity check compares complete JSON bytes in fresh CLI/viewer processes
for 16 barycentric and 7 terrestrial TOAs, including original input identities.
It compares complete edit/repeated-fit histories and every saved file with
direct engine calls in separate processes, verifies fit-row exclusion and
reinclusion, and parses both saved PAR/TIM pairs. It also verifies preserved
engine dependency versions/checksums. The ignored library test uses the pinned
runtime to check that a real no-fit-data failure preserves model/result state.

With the pinned reference runtime and a graphics adapter available:

```sh
export TEMPO2=/absolute/path/to/pinned/T2runtime
export TEMPO2_GUI_TEST_OUTPUT=/absolute/path/to/fresh/render-evidence
cargo test --manifest-path gui/Cargo.toml --locked --bin tempo2-gui \
  load_inspect_and_render -- --ignored --nocapture
```

The `egui_kittest` check exercises loading, list/point selection, zoom/reset,
exclusion, refit, save, existing-folder rejection and reinclusion. It checks
input identity and the engine result at each relevant boundary, and verifies a
failed new load clears the previous result. Its test-only WGPU renderer writes
normal, excluded, fitted, saved and error PNGs to `TEMPO2_GUI_TEST_OUTPUT` for
visual inspection. The directory must be fresh; partial outputs stay available
after a failure. Without this variable, it uses `gui/build/rendered-PID/`.
Both runtime-dependent tests use `TEMPO2` when set, otherwise the local pinned
reference path shown in the parity command. This verifies widget
behavior and rendering; it does not verify the native OpenGL window or
operating-system integration. Native desktop verification is deferred while the
Mac is locked; Linux GUI build/launch is also pending. Generated evidence and
build products stay outside Git.
