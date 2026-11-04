# Interactive session: exclude, refit and save

Central implementation decision, 4 November 2026, continuing the GUI sequence
in [0129](0129-qualified-release-continuation.md) after [0135](0135-minimal-timing-viewer.md).
Root owns `gui/` and this decision. AL/AO work remains separate.

## Scientific and state contract

Retain the existing canonical `BatInput`, `RuntimeContext` and full timing/fit
results for one loaded PAR/TIM pair. Use original `input_index` to identify
edited TOAs across sorting. A point selection alone changes no scientific data.
Exclusion/reinclusion sets the source `deleted` field to 1/0, matching
`plugin/plk_plug.C::deletePoint`, then reevaluates through the Rust engine.
Generated rows without an original input identity remain inspectable but cannot
be edited through this control. No TOAs are silently removed from the display.

Refitting calls `general_fitting::fit`, the same general engine as
`tempo2-rust timing --fit`, on the current model and deletion state. Preserve its
mean removal, PAR/TIM fit flags, mode and NITS, and default `FitOptions` (including
error rescaling). Show these semantics in the interface and expose the full fit
result for inspection. This slice adds no covariance selector, fit backend or
parameter editor. NITS zero is reported as no returned fit cycles; it must not
be presented as a successful fitted model ready for saving.

Reuse the existing owner-aware `PulsarContext::clone`: it remaps internal handles
and retains owned observations/matrices/payloads. Exclusion and fitting operate
on that snapshot, committing model/results only after success. This costs one
temporary scientific-state copy per operation but prevents a partially updated
failed fit from replacing the last valid state. Do not introduce a second
scientific representation, general transaction framework or model reparser.

Saving is available after a successful nonempty fit and before another edit.
Create a new destination directory so existing files cannot be overwritten.
Use `core_output::text_output` and `source_tim::write_tim` (FORMAT 1) on an
owner-aware snapshot. The existing PAR writer derives reporting fields including
TZRMJD/TZRFRQ/TRES; saving must not mutate the live fitted model. Retain engine
stdout/stderr, all writer products and complete timing/fit JSON. Check that both
requested PAR/TIM products were created before reporting success; the source
PAR writer can report an open failure through diagnostics without an error return.
Retain partial outputs and report their location if saving fails.

Native writer formatting and `C` comments for excluded TOAs remain unchanged.
The saved PAR/TIM files are native-format exports, not lossless serialization of
all engine state; exact results/provenance remain in JSON. Do not claim exact
roundtrip residual identity across the source writer's derived phase reference
and text rounding. This is established writer behavior, not a changed scientific
convention or relaxation of release comparisons.

## Dependency-ordered implementation and verification

1. Extend `gui/src/lib.rs` from a one-shot projection into the minimal retained
   session using the existing load/evaluate/fit/writer APIs. Retain display-only
   conversion separately. Verify state ownership, original identity, failed
   operation preservation and save isolation through representative fixtures.
2. Extend the existing worker/channel path in `gui/src/main.rs` to run exclusion,
   refit and save without blocking the UI. Preserve selection by input identity,
   show current fit/edit state and errors, and keep previous valid state on
   operation failure. Avoid adding another execution mode or queue framework.
3. Extend authored verification examples/recipes to compare full results and
   saved products against a direct canonical-engine history in fresh processes,
   covering unsorted barycentric and terrestrial data, deletion, repeated fitting,
   saved-input parsing and existing-destination rejection. This checks frontend
   wiring/ownership, not new numerical qualification against TEMPO2.
4. Exercise the visible controls with `egui_kittest`, inspect rendered normal,
   excluded, fitted, saved and error states, and run focused lint/build/tests.
   Update the README and retain evidence outside Git. Native desktop verification
   remains pending until the user makes the locked desktop available; the user
   explicitly requested continuing other work for now.

## Local result

The slice is implemented. Fresh-process GUI/direct-engine histories agree byte
for byte for the 16-TOA unsorted barycentric and 7-TOA terrestrial fixtures,
including three fits, exclusion/reinclusion, all five saved products, and saved
PAR/TIM parsing with 15/6 active TOAs. The full initial timing result still agrees
with the CLI. Existing-directory rejection leaves live timing unchanged.

Both library tests pass, including the real empty-fit-data failure that changes
START/FINISH on the temporary engine state but leaves the session's model and
result intact. All-target Clippy with warnings denied, formatting and builds
pass. The rendered control test passes load, selection, zoom/reset, exclusion,
refit, save, existing-directory error, reinclusion and failed reload. All six
normal/edit/fit/save/error images were visually inspected.

Evidence is retained outside Git under `gui/build/session-v1/` and
`gui/build/session-viewer-20261104/`, with associated logs in `gui/build/`.
The session review receipt SHA-256 is
`664ad49bb37ff82fa60490126dfe0b596761a6ed71bf1c3c16ac770e58a23653`.
These are frontend wiring and ownership checks. Native macOS launch remains
deferred at the user's request, Linux GUI remains unexecuted, and the existing
scientific release failures remain open.
