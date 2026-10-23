# Parameter storage Rust acceptance

Central decision, 24 October 2026. **Accept Assignment O's storage/copy foundation and canonical-store integration at delivery `8d2be6f`. Close O's Rust ownership window.** Production implementation is `8bad0be`; verifier and output routing are `71cd6d8`, strengthened by `1462ca3` and `3e48030`; final verification evidence is `7d24a91`. This accepts the scope of corrected contract 0033. M2 and the complete core transfer remain unaccepted.

## Source review and accepted behavior

Central reviewed the source enum/capacity registry, allocation and ordered labels, owned backing and unwritten fields, `copyParam` field/byte order, safe partial-copy boundaries and the loader/evaluator/fitter integration. There is one authoritative source-indexed parameter store. The old mutable scalar map and separate physical fit-error array are removed; the public scalar map is a fresh compatibility snapshot. Supplied uncertainty and original lines remain frontend provenance rather than a second physical-value authority. The existing strict frontend, supported keywords, JSON membership and numerical expressions remain unchanged.

All 166 enum entries, 2,000 slots and 9,616 backing cells are retained. Flags, link entries/counts and logical extents use signed source integers. Natural prefit/prefit-error and inactive links remain explicitly unwritten. Allocation and source label writes retain complete initialized byte buffers. Both `param_daop` and `param_iperharm` carry `IPERHARM`, including the overwritten first label's suffix bytes, as independently verified in the correction to 0033.

`copy_param` retains the source, destination links, backing and uncopied tails; copies labels through NUL; writes fields in source order; and assigns logical size last. Zero and negative source extents preserve their defined source behavior. Invalid storage and unwritten reads return the named operation-local boundary after preserving completed writes. The destination is then unavailable through its scientific access/copy operations; its diagnostic prefix remains inspectable. Rust ownership excludes overlapping mutable buffers and replaces undefined native lifetime behavior. Whole-owner cloning remains distinct from source copying.

Rug128, finite public Scalar construction, dependencies, C numeric ABI, rounding and fusion policy are unchanged. O introduces no unsafe block. Central verified that the twelve existing stdio/clock-text/SOFA unsafe sites and their files, Cargo manifest/lock and build script are unchanged from accepted L.

## Independent verification

Central built and ran the delivered direct verifier in **debug and release**, with outputs in a fresh temporary directory and a byte-identical supplemental caller copied to a separate temporary native build. The unchanged pinned source and real K dependencies compiled there. Only output/build locations changed; no fixture, expected value, comparator, control, source routine or tolerance changed.

Each profile passes:

- Six K and four M cases, **32 ordered events**, the complete registry and every initialized label/shortlabel byte.
- Supplemental negative-count and shorter-string native copies, plus eight Rust-only boundary cases with full retained prefixes.
- **30 comparison/identity controls**: one accepted positive and 29 rejected mutations/failures.
- **96,906 exact finite-number comparisons**, 664,159 field visits and 591,516 scalar leaf comparisons. These are overlapping traversal counts, not distinct cell counts.
- Four focused Rust tests covering initialization/ownership, boundary termination, signed/nonfinite/zero copying and canonical-store/snapshot/evaluation integration.

All fresh raw Rust and native state data equal the delivered records after removing only fresh launch token/input identity fields. Debug/release test executables match O's recorded hashes: `27757b2f11b06bbf9343b38d2ff2cbcdcd137d5ab2dad9c51e9aba6247e518d0` and `b623d5206fcfa4c5d46bc67d7122f42945b4ef2b4c682af686f436697d8fd425`. The independently located native build has hash `04c425d26102cee7af6305e590a72ea18573744d08a47fdc6f810d885fa5254d`; its paths/build identity differ while its complete observed state agrees.

The [central reproducer](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/parameter-storage/central-review.py), log and compressed complete receipt retain the executed identities and adaptations. [Acceptance checks](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/parameter-storage/acceptance-checks.json) verify **230 final artifacts against both current and committed delivery bytes, 54 executed-source identities, 1,618 protected files and all 36 module hashes**. Both nested repositories are clean at their pinned revisions. Current debug/release CLI hashes match O's regression receipts.

O's full command evidence passes formatting, Clippy, ordinary Rust tests, clock/glob/provenance, JPL and M1a/M1b. Central checked the committed artifacts, source/executable identities, command results and output-root-only verifier changes; it did not independently repeat those full suites. The final full-run receipt records shared HEAD `6b5d373`; executed-source hashes identify the production implementation despite concurrent central documentation commits.

M2 still exits 1 with exactly the four historical debug/release dependent-column raw retention-order and singular-value-index-zero failures. Their complete content matches historical evidence and is retained in the acceptance checks. No sorting, numerical backend change or tolerance relaxation is accepted.

## Ownership and remaining work

This accepts the parameter-only foundation and its integration with the existing bounded frontend and timing/fitting consumers. It does not establish full pulsar initialization/destruction, general PAR/TIM/preprocessing, linked-value arithmetic, `copyPSR`, additional physical models, Linux/other locales, allocation-failure behavior, concurrency stress or performance. Existing scientific and numerical scope limits remain in force.

O's report, projections and `rust/verification/parameter_storage/` become immutable acceptance evidence. `validation/parameter_storage_port/` is the corresponding regression verifier; later changes need an explicit release. Historical reference and verification files remain protected. P and Q retain their separate preprocessing and specialized-PAR reference directories. A subsequent Rust owner requires a new central contract; this decision does not release one or complete the overall goal.
