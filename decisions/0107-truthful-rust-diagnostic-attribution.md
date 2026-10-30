# Truthful Rust diagnostic source attribution

Central decision, 31 October 2026, implementing the user's correction to the
terminal demonstration in 0106. **Rust-produced diagnostics must identify the
Rust code that emitted them.** Familiar TEMPO2 report conventions do not justify
presenting a C/C++ counterpart as the executing source. This is an explicit
user-authorized presentation-policy amendment to 0080/AH and 0106; scientific
algorithms, precision, output quantities and numerical gates are unchanged.

## Observed cause and scope

The preserved 0106 example prints `[t2fit.C:850]`, `[TKfit.C:345]`,
`[TKrobust.C:25]`, `[tempo2.C:566]` and `[textOutput.C:306]`, although those
messages were emitted by Rust. Inspection at `f69a89e` finds native file names
and line numbers supplied directly to `Diagnostics::log` by
`general_fitting.rs`, `tkfit.rs`, `tkrobust.rs`, `core_output_source.rs` and the
`core_cli.rs::main_log` wrapper. This pattern extends to other timing, parser,
clock and lifecycle modules; it is not confined to the displayed fitting lines.
`Diagnostics::version` and its callers also identify native source files.

The shared renderer is `clock_diagnostics.rs`. Observation-worker transcripts
retain log intents and replay them later, so a logger-only caller-location change
could incorrectly identify the replay/helper location. `core_output_source.rs`
is an authored/generated transfer whose historical producer embeds native
locations. Preserve that producer and accepted evidence; update any maintained
successor generation path so it cannot silently restore misleading labels.

## Approved contract

- Rust-owned log/timing/version attribution names the actual Rust module/file
  and, where a line is displayed, a real emitting call site. Use compiler-derived
  location information or explicit captured call-site metadata. Do not merely
  replace `.C` by `.rs` or keep native line numbers: `t2fit.C` maps here to
  `general_fitting.rs`, not a fictional `t2fit.rs`.
- Preserve the emission location through helper wrappers, deferred/worker replay
  and diagnostics draining. An internal logger/replay line is not the originating
  location. Prefer concise truthful file labels over checkout-specific paths.
- Keep TEMPO2 counterparts, pinned source locations and the rationale for this
  presentation departure in nearby comments and source maps. Preserve actual
  native/provider attribution for messages genuinely emitted by retained native
  code. Do not rewrite user filenames or native references appearing as message
  data. Version/identity displays must clearly distinguish the Rust build from
  its TEMPO2 source pin; do not present the native header/version as Rust's source.
- Retain established report layout, terms, units, message intent/severity/order,
  stdout/stderr routing, suppression/once histories, counters and storage rules.
  Keep public caller-supplied diagnostic metadata APIs truthful and documented;
  no second compatibility identity mode or runtime switch is needed.
- Check long-message boundaries explicitly: the current logger limits the full
  rendered line to 359 bytes, so changing the prefix can affect its visible tail.
  Retain that evidence and report any extra truncation rather than silently
  claiming body equality. A broader truncation-policy change is not bundled into
  this source-attribution correction.

## AL implementation and verification handoff

After central reviews frozen Linux run `41069548581`, AL owns this correction
alongside the 0106 native-first help/documentation work. AL may edit its existing
`rust/` and `validation/release/` scope; historical accepted runners and reference
exports remain immutable under 0050. No independent push or dispatch.

1. Inventory Rust-owned log/timing/version emission sites and their wrappers;
   reuse the existing diagnostics owner and fix location capture at the source.
   Preserve the native correspondence beside changed code. Cover the same report
   example plus warnings, debug/timing/version messages and worker replay.
2. Extend AL-owned comparison adapters for the intentional metadata change.
   First validate that Rust labels/locations correspond to the candidate's real
   source, then compare ordered message bodies and the unchanged scientific
   records. Retain raw native/Rust text and the earlier C-labelled Rust output.
   Do not globally strip prefixes or discard diagnostic lines to make tests pass.
   Keep named source mappings separate from unrelated numerical differences.
3. Verify actual caller attribution (including wrappers and replay), rejection of
   wrong-file/wrong-location metadata, and rejection of missing/reordered messages
   or changed numeric bodies. Exercise existing quiet/debug/version behavior,
   warning/error counters, long-line boundaries and affected machine diagnostics.
   Keep JSON schemas and structured numerical fields intact. Run appropriate
   compile/tests, formatting and Clippy; report any old exact-text test that now
   requires the explicitly changed metadata contract.
4. Return a scoped Conventional Commit, before/after runnable terminal example,
   source map, raw evidence and focused results for central review. Do not claim
   that attribution changes resolve inherited scientific/output failures.

The current Linux candidate is unchanged. Central reviews its full result before
releasing edits; AL-first sequencing, deferred optimization and all milestone
acceptance obligations remain in force.
