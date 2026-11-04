# Minimal timing viewer and frontend choice

Central implementation decision, 4 November 2026, under the approved minimal
GUI sequence in [0129](0129-qualified-release-continuation.md).

The first question is residual versus site arrival time, with reported TOA
uncertainty and original input identity available for inspection. Implement the
first slice in the separate `gui/` Cargo package. All science continues through
`bats::load_pairs`, `bats::preprocess` and `timing::evaluate`; mean removal matches
the default `tempo2-rust timing` command. Screen conversion to binary64 does not
replace the original Number values. Missing, nonfinite or unplottable data must
be reported, not silently filtered. This slice loads and inspects only.

## Reuse and platform choice

Use the mature Rust [eframe](https://docs.rs/eframe/0.37.0/eframe/) app framework
and [egui_plot](https://docs.rs/egui_plot/0.38.0/egui_plot/) plotting library,
pinned to mutually compatible 0.37.0/0.38.0 versions. Their documented macOS and
Linux support fits the two targets. Use one native OpenGL renderer and retain
the existing engine dependency versions in the GUI lockfile. Rust 1.95 is the
frontend minimum. Actual installed support still requires target qualification.

This reuses plotting, pointer interaction, accessibility and application event
handling without introducing a web server, JavaScript engine bridge or another
scientific representation. The established PGPLOT C/Fortran option was checked;
embedding it would add a graphics adapter and event/widget integration for this
workflow. Eframe's native Rust API is the smaller complete choice here. No
replacement for source numerical algorithms is implied by the frontend choice.

Use the official [egui_kittest](https://docs.rs/egui_kittest/0.37.0/egui_kittest/)
harness for repeatable interaction and rendering checks. Its WGPU test renderer
is a development dependency; the native app uses Glow. A successful harness
render does not prove native window launch. An attempted desktop automation
selection did not complete, so that launch check remains open.

## Ordered implementation and completion criteria

1. `gui/src/lib.rs`: call the canonical workflow and project display-only
   coordinates, preserving full engine output and input identity.
2. `gui/src/main.rs`: asynchronous load, residual/error-bar plot, zoom/pan/reset,
   TOA selection and exact detail/diagnostic inspection. A failed new load must
   show the engine error and remove the previous snapshot.
3. `gui/verify.py` and the example exporter: compare complete CLI/GUI output for
   the authored barycentric and terrestrial fixtures in fresh processes; bind
   inputs/executables and confirm every core lockfile dependency is preserved.
4. Run focused format/lint/tests and the rendered interaction check; inspect the
   normal/error images. Document the tested boundary and remaining launch/Linux
   checks in `gui/README.md`. Retain generated evidence outside Git.

Selection/exclusion, refitting and saving follow as a separate slice using the
general Rust timing/fitting state and existing writers. Establish the state,
identity and persistence contracts before implementing that slice; do not use
the older restricted M2 fitting interface or recreate PAR/TIM scientific
serialization in the frontend. The GUI does not relax any open scientific or
release-performance gate.

## First-slice result

The four steps above pass on macOS arm64: complete CLI/GUI JSON agrees for all
23 fixture TOAs; original input identities and all engine lockfile versions are
preserved. Formatting, all-target Clippy with warnings denied and the focused
display test pass. The explicitly run rendered test passes load, list/point
selection, zoom/reset, unchanged engine output and failed-load clearing. Central
inspected both normal and error PNGs. Native OpenGL window launch and Linux GUI
qualification remain unverified.

Evidence is retained outside Git under `gui/build/first-viewer-20261104/`.
The `viewer-review.json` SHA-256 is
`2fc71e21ea369f348e727ceef6643410773cc8e9c2a569a5c66de3c37f3ba0d4`.
This is implementation/wiring verification, not a new scientific validation.
