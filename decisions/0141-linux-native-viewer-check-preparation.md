# Prepare a Linux native viewer check

Central preparation, 4 November 2026, continuing
[0140](0140-gui-installation-and-portable-checks.md). Root owns the GUI sources;
AL will own reviewed integration into the Linux qualification runner.

The existing offscreen WGPU test exercises controls but cannot establish that
the production OpenGL window opens and closes on Linux. Use eframe's existing
native event loop and screenshot callback for that boundary. A Linux-only
ignored test wraps the existing `Viewer`, loads the same 16-TOA fixture, checks
its complete state is available, captures a native frame and closes the window.
It records the OpenGL context identity and full timing JSON for comparison with
the separately executed CLI. No production GUI flag or alternate model path is
introduced.

The test uses winit's documented X11 `with_any_thread` hook because Rust tests
run on worker threads. It explicitly selects X11 and the production Glow
renderer, and uses a deadline plus an external process timeout in the recipe.
The screenshot callback reads pixels through eframe; it does not automate a
user desktop or substitute for the deferred macOS desktop observation.

Implementation order: add the native test module and the two test-only direct
dependencies already present transitively; preserve every locked package
version/checksum; document the Xvfb/Mesa recipe, separate output directory,
full-output comparison and visual inspection. The selected APIs were checked
in the pinned eframe 0.37.0, egui 0.37.0 and winit 0.30.13 sources.

Formatting and a macOS all-target check pass; all GUI and engine dependency
versions/checksums remain unchanged. Those checks do not compile or execute the
Linux-only body. Actual Linux compilation, native rendering, timing-output
comparison and image review remain required before acceptance. The earlier
macOS installation/render receipts still describe their original frozen source;
this preparation does not relabel them as execution of the new test.

The next Linux candidate should run the existing library/control/parity checks,
install the release viewer with Cargo and inventory its actual dynamic
dependencies, then execute this X11 native check. Retain failures independently
of the numerical matrix, and retain its PNG/JSON/context identity outside build
caches. This entry is preparation, not authorization for an unreviewed push,
workflow dispatch, public release or a Wayland qualification claim.
