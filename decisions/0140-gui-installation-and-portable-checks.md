# GUI installation and portable verification paths

Central implementation record, 4 November 2026, continuing
[0138](0138-interactive-session-edit-fit-save.md). Root owns `gui/` and this
record. This slice changes test configuration and installation documentation;
the interactive session and scientific engine behavior remain unchanged.

Use the established [Cargo installer](https://doc.rust-lang.org/cargo/commands/cargo-install.html)
with `--path gui --root PREFIX --locked --bin tempo2-gui`. It builds a release
binary and tracks the private installation without a custom packaging layer.
The existing path dependency compiles the same Rust engine source. Scientific
resources remain an explicit external directory. This is a command-line
installation, not a signed macOS bundle or Linux desktop package.

The existing runtime-dependent tests now honor `TEMPO2`, with their original
pinned local runtime as the documented default. The rendered check accepts
`TEMPO2_GUI_TEST_OUTPUT` and requires a fresh output directory; its default is
`gui/build/rendered-PID/`. This permits the same controls and evidence retention
on a separately staged Linux candidate without hard-coded campaign paths.
The test keeps partial output after failure. Missing-input checks now use that
fresh directory rather than a potentially reused global test filename.

The dependency-ordered checks were: inspect Cargo/eframe's documented install
and build contracts; update the two test entry points; install the locked
release binary into a fresh private prefix; run formatting, all-target Clippy,
the two library tests and the existing rendered workflow; inspect its images
and the installed executable's dynamic dependencies.

## Local result and remaining work

Cargo installed an arm64 Mach-O release binary, identical to its build output.
Its dynamic dependency inventory contains only macOS system libraries and
frameworks. This does not remove the engine's documented static/native build
dependencies. Formatting, Clippy with warnings denied and both library tests
pass. The initial sandboxed WGPU run failed with `No adapter found`; the same
offscreen check passed when granted graphics-device access outside the sandbox.
All six load/exclusion/fit/save/error images were inspected. Both failed and
successful render logs are retained.

Evidence is outside Git at `gui/build/portable-20261104/`. Receipt SHA-256:
`92cfc59e08c4695410bb804324ea3d11139a0ee1a9108d814ed962ca3dc7b4ce`.
Installed binary SHA-256:
`662e6ac6772fe2ee015c18488653c41f5ab22cd791a9b73608fbd7f7c77cc8e4`.
The receipt binds source, lockfiles, installation metadata, logs, saved products
and rendered images. The code/recipe commit binds the authored changes.

The README records the [pinned eframe Linux prerequisites](https://github.com/emilk/egui/blob/0.37.0/crates/eframe/README.md).
Linux compilation, runtime dependencies, rendering and native launch still need
execution on Linux. Native macOS window verification remains deferred until the
user unlocks the desktop. An offscreen WGPU render is not a native OpenGL-window
observation, and this record does not claim either platform's full GUI release
qualification. Existing scientific and performance gates remain open.
