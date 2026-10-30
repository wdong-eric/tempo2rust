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

AL may begin this presentation correction alongside the 0106 native-first
help/documentation work while central reviews frozen Linux run `41069548581`.
The running job uses an immutable Git candidate; new local commits cannot change
that checkout. AL may edit its existing
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

The current Linux candidate is unchanged. Arithmetic corrections and cache-guard
promotion still await its full result; AL-first sequencing, deferred optimization
and all milestone acceptance obligations remain in force.

## Independent presentation preparation released

Central rechecked GitHub run 41069548581 at exact head a35104a and verified the
preserved candidate archive SHA-256
`e04b10cf1e9171b449175cd85cbd617e14b016d4e75efe3a0596ba85c3a6e1bb`.
Before this release the local implementation paths still match that candidate.
The snapshot is retained in ignored
`decisions/evidence/release-qualification/build/third-linux-a35104a-review-20261031/candidate-review/`.
Central will use that frozen source for the run's comparisons, not a later working
tree. Collection is independently bound to the same run and full SHA.

The earlier whole-working-tree freeze is therefore unnecessary for the user's
newly requested presentation work. This amendment releases **only 0106/0107**
to the existing AL owner now, without waiting for the Linux matrix to finish.
Do not change scientific expressions, ordinary-type sites, precision, tolerance
gates or production cache ownership in this preparation. Central documentation
and historical evidence remain outside AL ownership. Keep changes in scoped
Conventional Commits and return a reviewable report/examples/test evidence.
No push or dispatch is released. The current job continues unchanged, and its
result still determines the numerical/cache correction brief and acceptance.

## Central installed-product review, 31 October

AL implemented the presentation correction in
`801bc94139cca90d6072234b09f5a1bcec5e4b05`. Central reviewed the shared logger,
origin capture through wrappers and observation replay, version identity, the
emitter conversions and the native-first help. The earlier unsupported `-gr plk`
primary example is corrected to the supported `-f` route, with the retained
plugin catalogue explicitly marked as unavailable for execution.

The independent executed recipe is
`decisions/evidence/release-qualification/central-presentation-review.py`.
Its ignored packet is
`decisions/evidence/release-qualification/build/presentation-801bc94-review-20261031/`.
The review binds all **2,008 Git-export files and 262 installed files**, replays
nine retained report/version/debug and machine-interface comparisons, and freshly
passes four mutation/control tests. A fresh installed robust-H execution exits
zero with no stderr and matches the retained corrected report after the named
CPU-time metadata projection. It prints actual sites including
`[general_fitting.rs:210]`, `[tkfit.rs:365]`, `[tkrobust.rs:124]` and
`[core_cli.rs:819]`; the scientific report fields remain identical in these
before/after checks. This establishes presentation behavior on the tested macOS
installation, not independent scientific equivalence or Linux qualification.

The verified review receipt has SHA-256
`0459603a9fdbbe88a7db1bd3e707361534b76b055a90fdb1b618020f32969ff4`;
the exact executed recipe has SHA-256
`940e5e87a21265369c98ec6c5f13f069722ca35ccc3f3ef53dd40f97c606ed12`.
The first central attempt stopped on a Python tuple-versus-JSON-array comparison
in the reviewer. Canonical JSON comparison proves the metadata equal; the
original attempt and corrected execution are both retained. It was not a
production or scientific failure.

AL is finishing its scoped harness/documentation delivery and additional
identical-state output evidence. Central will bind that final commit before
closing the complete presentation delivery. The implemented correction and fresh
installed example pass the review above. The existing 359-byte rendered-line
limit remains: longer truthful prefixes can shorten the visible tail of long
messages, and the controls retain this effect rather than repairing or discarding
it in comparison. No numeric tolerance, arithmetic or production cache change
is accepted by this review. Linux run `41069548581` still uses frozen `a35104a`;
there is no new publication or dispatch.
