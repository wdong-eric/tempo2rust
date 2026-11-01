# Isolate saved-evidence comparator work during Linux qualification

Central decision, 1 November 2026. Under the user's independent-work instruction
and 0114, release the already-specified 0113 diagnostic-rendering comparator in
a separate Git worktree while the fourth Linux run continues. This supersedes
0113/0114's scheduling freeze only for the bounded work below. It changes no
scientific, numerical, source-attribution or acceptance contract.

## Why this work can proceed

The run is immutable at `fecbc18af95196a16db2046306099b6345a50976`; a private
successor checkout cannot change that candidate, its comparisons or its artifact.
The three macOS emissions, complete backing messages, native formatting source
and all other case fields are already independently verified by 0112/0113.
Implementing their paired proof and rejection controls therefore does not require
new Linux results. Integration and actual Linux qualification still do. This
uses a concrete diagnosed issue to advance AL without starting optimization or
inventing another feature while waiting.

## Ownership and isolation

Central creates branch `al/diagnostic-rendering-0116` in
`/private/tmp/tempo2-al-rendering-0116-20261101` at the commit containing this
release. AL must set every editing/commit command's working directory to that
worktree. Its existing main checkout is read-only for this assignment. Central
retains the main branch, running job, artifact collector, review and later
integration/publication decisions. AL is not alone: preserve all other writers,
AM's accepted documentation, the accepted source-map refresh and historical data.

AL may change only:

- `validation/release/diagnostic_attribution.py`, reusing its strict adapter;
- `validation/release/corpus_compare.py`, for the bounded W staging integration;
- `validation/release/test_diagnostic_attribution.py`, including saved-evidence
  replay and meaningful rejection controls (one directly necessary companion
  Python test/module in the same directory is allowed if a shared invariant
  warrants it; explain the need rather than adding a second comparison mode);
- new `rust/RELEASE_DIAGNOSTIC_RENDERING_REPORT.md`.

No Rust/native production source, Cargo/build/installer/workflow, existing
historical runner, original cap witness or central decision may change. No
engine execution, native/Rust compilation, full corpus regeneration, network
workflow action or publication is released. Generated receipts and mutations
stay in a fresh ignored `validation/release/build/` directory of the isolated
worktree; record external evidence paths explicitly and never overwrite them.

## Dependency-ordered implementation

1. Implement 0113's paired proof using the existing attribution/logger patterns.
   Preserve `Attribution.stream` unchanged in meaning. Validate both real emit
   sites, independently stored full messages, event/severity/storage association,
   source identity and the prescribed formatting/359-byte cap/C-string/redirected
   byte-processing order. Reproduce each entire observed emission and all bytes
   outside it; do not repair a common prefix, split on marker-like message data,
   use a generic ANSI regex as the source parser, or whitelist case identifiers.
   Unavailable, truncated or ambiguous backing data must remain a failure.
2. Wire the same entry point into the AL-owned W staging adapter. Historical
   files remain untouched; bind the exact staged edits. Exercise that actual
   staged comparison on the saved complete pairs, including schema/state/order/
   counters/outcome/path identity, so an isolated helper pass does not stand in
   for integration. Preserve the original strict failures separately.
3. Run the narrow existing attribution controls and all 0113 rejection cases,
   including a changed Rust-only visible tail, source site, message/filename/
   numeric token, severity/channel/event association, order, counter, membership,
   newline, missing diagnostic, common-prefix truncation, internal label and
   missing/truncated backing evidence. Use original native/Rust records and
   mutated copies; no new scientific producer is required for this stage.
4. Commit only owned authored files with Conventional Commits and return the
   exact branch/commit, report, identities, original versus successor results
   and controls for central review. Stop at this boundary. Fresh complete W
   execution, downstream controls and actual fourth-run Linux replay remain
   pending until central reviews the run and explicitly releases them.

Use the existing 0113 evidence, particularly
`validation/release/build/third-cap-review-v5/receipt.json` in the main checkout
(SHA-256 `d68ec56eb523df4679f610681ce478aa747fb7f3a406fa7a0731fa443a1d006b`),
and its 22 bound inputs. Read the recorded source files and raw event/stream
products at their bound paths. Do not infer missing messages from Rust output
or relabel a saved-data replay as fresh execution. Source labels must remain
truthful to the actual Rust files.

## Continuing acceptance boundary

The existing fifteen-minute monitor and collector continue on run `41294531402`,
job `124812485584`, attempt 1. A timeout never authorizes redispatch. All reviewed
main implementation files remain identical to the frozen candidate throughout
this isolated work. Central must inspect the delivered diff and actual Linux
result before integrating or releasing fresh execution. The original failures,
raw SVD order, precision/tolerance policies, versioned DDH clocks, source weighting
and the AL-first optimization sequence remain unchanged. This release does not
accept AL, M2, Linux qualification or full migration.


## Common-prefix control and strict-pass clarification

AL's initial negative control found that an early return on strict equality can
bypass the required rendering proof: truncating Rust output to the native visible
body produces equal attribution-projected text. Central independently reproduced
this with all three bound 0113 emissions, removing 51, one and seventeen bytes
respectively. The original streams differ; each altered Rust stream compares
equal under the unchanged strict adapter. Other fields remain the original
bound values. This is an offline comparator counterexample, not a new engine
execution or a scientific discrepancy.

The central witness receipt is retained in ignored
`decisions/evidence/release-qualification/build/common-prefix-review-20261101/`;
SHA-256 `c9ae272ea15285034cf86be2f54d0058b91ac4d342b7cf99e3d6133e8037879c`.
It binds the original cap receipt, unchanged adapter and all original/altered
stream hashes. Central also inspected the actual saved campaign at the cap
receipt's bound root `/private/tmp/t2-local/linux/corpus/wxxxxxxxxxxxxxx`:
127 strict passes and the original three differences remain; none of the 127
passing histories is selected by the draft's single-message operation gate.
An earlier inspection pointed at the main workspace's older corpus and stopped
on its native-labelled Rust output; it was not used as current campaign evidence.

Clarify 0113 item 2: for a source-qualified single-message W history, validate
independent message/render consistency before returning a pass, including when
strict text is equal. A differing rendering can gain acceptance only through
the complete paired proof. This preserves the standalone strict adapter and
unsupported-history behavior; it does not create a generic evidence-free
fallback, case whitelist or common-prefix normalization. Keep the strict outcome
and paired source-consistency outcome distinct in the receipt.

The rejection controls must separate stale-identity failures from semantic
failures. Rebind copied raw/result/receipt hashes for stream/message mutations
so that the appropriate contract check is reached, and record the actual
rejection stage/reason. Retain all original attempts. This clarification remains
within isolated 0116 work; production, fresh execution, integration and Linux
acceptance boundaries do not change.


## Central review of the isolated implementation

Central reviews code `25bd81fa2bdbce7759de6ddba5bca1784e0be48c` and report
`13e33f27dc043b3a68ac6f8ff8133c417996a57a` on `al/diagnostic-rendering-0116`.
The exact four-file diff stays within the released ownership; the worktree is
clean. No production source or historical runner changed. Main retains its
frozen implementation, and these commits are not integrated or published.

The implementation follows the clarified 0113 contract. It binds the complete
independent messages, source arguments/sites, raw products and event association;
reconstructs the source byte formatter/cap/parser and whole surrounding stream;
and reuses the actual W staging/positive-control entry point. The strict
`Attribution` class and `compare_pairs` are byte-identical to the release base.
Resource-map ordering is not scientific field ordering; ordered W state/schema
comparisons remain exact. Unqualified profiles/history do not gain a rendering
exception. The report retains the original strict failures and development
attempts rather than replacing them with the successor result.

Central independently ran the committed test module through that staged W
entry point with the bound cap evidence and a new output directory. All thirteen
test methods pass. The 51 recorded outcomes comprise three paired proofs, one
unchanged strict positive, and 47 rejected mutations: eleven integrity, three
attribution, thirteen non-rendering, five event-association, three backing-message
and twelve rendering failures. Each rejection records a nonempty reason at the
asserted stage. The coherently rebound tail/newline/common-prefix/native-site
controls reach semantic checks. The original 22 inputs remain unchanged.
Central also matched the final AL v6 receipt hashes to the report; no engine,
compiler or full scientific corpus was run in this review.

The central replay receipt SHA-256 is
`610fd490e2c23ade8dd81e26997cae62b58734b3dcfa190c0d89e0504e93ce2a`;
its 51-outcome summary is
`1a3c692be46edc4b7e984ccfa0b94c5533f2d56c25f514c2929de47523d5cec1`.
The complete central review output is copied byte-for-byte to ignored
`decisions/evidence/release-qualification/build/isolated-rendering-review-20261101/`:
772 files, 80271445 bytes and 1 explicitly recorded source symlink.
Copy-manifest SHA-256 is `b41846c9db6784930249601e4678d388e0e28efdd940e5cdefda2adfdfc42bf8`. Generated evidence is not committed.

This accepts the isolated implementation and its saved-evidence proof for the
released stage only. It is ready for later integration review; fresh full W,
downstream controls and actual Linux verification remain required. The Linux
job was still executing its matrix at 11:39:26 UTC and the same fifteen-minute
monitor continues. No extra execution, integration, arithmetic repair, publication
or optimization is released by this review. AL, M2 and full migration acceptance
remain pending; retain the isolated branch until the fourth-run review.
