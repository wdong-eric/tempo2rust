# TEMPO2-Rust timing viewer

A first interactive viewer for one PAR/TIM pair: residuals with TOA uncertainty
bars, zoom/pan/reset, point or list selection, and exact TOA values. Loading runs
the existing Rust load/preprocess/timing workflow with residual mean removal.
It does not perform a fit or write input files. Selection/exclusion, refitting
and saving are the next stage of decision 0129.

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

Native Linux windowing/graphics dependencies and installed GUI launch on both
platforms still need release qualification. This package is not yet part of the
engine's private-prefix installer.

## Focused verification

```sh
cargo fmt --manifest-path gui/Cargo.toml --check
cargo clippy --manifest-path gui/Cargo.toml --locked --all-targets -- -D warnings
cargo test --manifest-path gui/Cargo.toml --locked
cargo build --manifest-path gui/Cargo.toml --locked --examples
cargo build --manifest-path rust/Cargo.toml --locked --bin tempo2-rust
python3 gui/verify.py \
  --cli rust/target/debug/tempo2-rust \
  --viewer-export gui/target/debug/examples/export_timing \
  --runtime validation/reference/build/source/T2runtime \
  --out gui/build/parity
```

Use fresh output directories; adjust executable paths if `CARGO_TARGET_DIR` is
set. The parity check compares complete JSON bytes in fresh CLI/viewer processes
for 16 barycentric and 7 terrestrial TOAs, including original input identities.
It also verifies preserved engine dependency versions/checksums.

With the pinned reference runtime and a graphics adapter available:

```sh
cargo test --manifest-path gui/Cargo.toml --locked --bin tempo2-gui \
  load_inspect_and_render -- --ignored --nocapture
```

The `egui_kittest` check loads through the visible button, selects through the
list and plot, zooms and resets, checks that interaction preserves complete
engine output, and verifies a failed new load clears the previous result. Its
test-only WGPU renderer writes `gui/build/first-viewer-20261104/viewer.png` and
`input-error.png` for visual inspection. This verifies widget behavior and
rendering; it does not verify the native OpenGL window or operating-system
integration. Generated evidence and build products stay outside Git.
