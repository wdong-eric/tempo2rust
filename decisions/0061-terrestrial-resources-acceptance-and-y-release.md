# Terrestrial resources acceptance and Assignment Y release

Central decision, 24 October 2026. **Accept W's complete released 22-routine terrestrial-resource transfer at delivery `bd290cc163ad706d8044d5deb2a637c9ed7803e0`, with final production change `eb2a589`, implementation `7df6234` and verifier `a5cc8c2`. Close W's Rust and validation ownership. Release Assignment Y as the next sole Rust owner under prepared contract 0058.** Z retains only its independent planetary-ephemeris reference directory. X and all other accepted references remain read-only.

## Accepted implementation and correction

Central reviewed the eight observatory, two EOP and twelve meteorology/Niell/troposphere routines, their source counterpart map, canonical process ownership, retained source state, failure boundaries and actual frontend integration. Observatory/EOP/weather state shares the established resource owner and diagnostic registry while keeping separate source lifetimes. The former RuntimeContext EOP/weather implementations are removed. The full source troposphere driver and established frontend use the same scientific calculation body. Resource provenance identifies loaded/cached bytes rather than reopening files to describe unused replacements. No new unsafe block, dependency, general arithmetic mode or public input expansion is accepted by this transfer.

Accept the localized NMF_wet correction under 0053. Raw `tropo.C::NMF_wet:206` reads the upper c coefficient from bs; the adjacent interpolation formulas and discontinuity at c-table knots demonstrate the error. Rust interpolates cs at both ends, with an adjacent pinned-source bug/fix comment. The independent continuity regression fails with the original expression and passes with the correction. The original U oracle remains unchanged. A separate one-line native patch with original dependencies changes exactly **31 wet-result bit fields in ten of the 130 cases**; central recomputed that full delta and checked the patch and actual executable/dependency identities. Other fields, streams and statuses are exact. U's integrated delays are unchanged within this corpus; this does not assert unchanged delay values outside those fixtures.

Explicit mul_add sites reproduce demonstrated contractions in the accepted GCC O2 expressions. Their assembly/witness rationale and adjacent comments remain in W's arithmetic contract. There is no global fast-math option, tolerance relaxation or precision change. TEMPO2 meteorology fallbacks, units, source expressions and established native-invalid boundaries retain their documented dispositions.

## Central execution and final-code evidence

Central ran the Cargo build checks and freshly executed all **130 cases / 1,128 actions / 2,256 events / 24 source exits in each debug and release profile**, plus all **28 controls per profile**, against the explicit corrected U contract. Exact ordered membership, scalar types, initialized storage, retained tails, finite representations/nonfinite classes, resource/cache/alias state, diagnostics and defined exit prefixes pass. Actual final executable hashes agree with W's final scoped receipt. Every new central output uses an owned separate directory; original W/U evidence stays unchanged.

The final production delta after the full regression run is exactly one removed unreachable site-length check. TabulatedFunction::read_bytes rejects headers longer than 255 bytes and requires '#'; after successful load, a site token from header[1..] is at most 254 bytes. Central verified that path and the exact source delta. The replacement comment states the dependency guarantee. This follows the user's updated standing rule: trust validated invariants instead of adding duplicate internal guards. EOP's permissive loader and revocable observation owners still require their real query/liveness boundaries.

The full pre-removal regression run is explicitly identified by recorded HEAD `7946aee` and source hashes. Central checked its receipts/log hashes, preserved original debug/release executables, unchanged dependent algorithm/example inputs, and the sole subsequent production delta. That run passed ordinary debug/release tests, T/O/R, clock/glob/provenance, JPL, M1a/M1b and public resource/CLI recovery checks. The complete M2 command finished with **exactly its four historical raw mode-order/index diagnostics**. It remains unaccepted.

The final code separately passed W's formatting/Clippy, existing loader/meteorology tests, all U cases and controls, followed by central's complete U replay above. The unrelated full chains were not rerun or relabeled as executions of the later code. W's final_review.py and appended contract are inspected verification/documentation changes, not additional production deltas. The old NMF mutation receipt still names its actual earlier source; its mathematical function is byte-identical after the unrelated guard removal.

The first fitting recovery failure is retained and localized to the initial prefit warning batch: process-lifetime weather loading and TROP/DUP suppression differ on a later call. The owned test exempts only that first warning batch; all numerical/provenance and later-stage fields remain exact, and direct U warning histories remain exact gates. Generator-formatting failure evidence and the subsequent full rerun are also preserved. The original R runner and its preservation membership remain unchanged under 0050; owned staging records its routing and the unchanged unsafe blocks.

## Evidence and limits

Final W report: `rust/TERRESTRIAL_RESOURCES_REPORT.md`, SHA-256 `96001fa4a81c96c258adae462ac2f5c5dc32a7223e6521ffd78c459023d61ec6`.

- Preserved full regression receipt: `c4a1b54094e73e2f6b78d5baca4f2965ec0e8ec93082a8a2335f68f105b3638b`.
- W final scoped receipt: `16830551f1c915aa827bdbb05ac520da14a5c8d5ff3ffddb831321f6c0b77718`.
- Central complete replay/audit: `a1d9cbd877761e00d6eb5dfca66b64b54459938a25c2b9fa85ec458b1bc5483b`.
- Central committed-report/relocation audit: `ee0ba98645ab1b40400d6077ba44b646df84819e02449f9be0f793de30100ed5`.
- Final debug/release executables: `589e0540e5bb7f1c6b83c73db700bdff01b6d162953629a54a8b87aeed70d1c2` / `c754828c3e046f803fdbf82827fbe92c989c9511eb918877ee869292f7f3b49b`.

The executed central reproducer is `decisions/evidence/terrestrial-resources-port/central-review.py`, SHA-256 `13b7e1a56b2390f931bd7da94287c221724792fda51171aff23c9792348c3e41`. Its 3,942-file successful local bundle is retained under `decisions/evidence/terrestrial-resources-port/build/central-o4ynb4e9/`, with original temporary paths preserved by symlink. Every moved file matched its hash. The initial central metadata-membership assumption and failure are retained separately inside that bundle. Generated outputs are ignored; only the reproducer and central documents are committed.

Central checked 2,135 W files and W's frozen preservation gate: all 12,285 accepted-evidence members remain exact; 81 evolving central Markdown files are explicitly exempt, with all 12,366 historical entries retained. Both independent repositories remain pinned and clean. The prior 22-routine U coverage limits, native-invalid/unhit branches, other platforms/locales, broader models, concurrency, performance and full timing-core completion remain open. This accepts the released source transfer and its explicit correction, not independent physical validation of the atmospheric approximation or general frontend support.

## Y release and regression applicability

Y now owns `rust/` and new `validation/time_ephemeris_port/` exclusively, starting from W's accepted `bd290cc` tree and the current central release commit. Follow the complete 29-routine scope, three demonstrated source corrections, Rug128 remainder dependency, stream ownership/necessary-Send proof and integration/verification requirements in 0058 and handoff 25. No implementation scope is reduced by W acceptance.

Z owns only `validation/planetary_ephemeris/`; do not freeze its evolving work or central Markdown as immutable evidence. Preserve X, W and all accepted original outputs/runners. Freeze Rust/source context at an explicit revision. Run W and other accepted regression callers in Y-owned staging when routing is needed, recording original/staged hashes and diffs. Preserve comparison membership and source-correction contracts. Do not overwrite W's verification roots or reinterpret its pre-guard full receipts as newer runs.

Small demonstrated bugs retain 0053 permission, with source-mapped comments, focused regressions and explicit corrected expectations. Trust validated domain invariants; add checks for actual source/file/ownership boundaries rather than fabricated internal violations. No mixed precision, new physics/frame convention, hidden tolerance, SVD sorting or M2 waiver is released. Central retains subsequent design and acceptance; the full core objective remains active.
