# Second Linux follow-through (decision 0104)

Work starts from central decision `4472d99`. The reviewed Linux run executed
`4accd26`: 54/66 top-level stages passed, twelve failed, none blocked. No early
native corpus executed. Historical failures and native runners remain intact.
This report records successor evidence, not milestone acceptance.

Production repairs are committed as `3e5f344` (Lomb) and `5c8c0aa` (EXP/GAUS).
The latter is the clean source exported and installed for the focused production
checks below. Harness corrections are `cecee60`, `0104447`, `ca60751` and
`ba12c8f`. Central review commits remain separate. No push or Actions dispatch
was performed here; central owns both under 0101/0104.

## Native history staging

The original third HTTPS fetch failed with Git's shallow metadata consistency
error, after two successful fetches. It did not establish a missing commit.
Local Git 2.54 traces show detached automatic maintenance/repack jobs between
the original sequential fetches. That is a plausible competing metadata writer;
the Linux Git 2.56 run did not retain a trace proving which writer changed it.
The three-fetch failure has **not reproduced** on macOS: twelve local sequences,
one HTTPS sequence, and a separately delayed/overlapped repack control completed.
Those observations are retained; they are not Linux qualification.

`native_corpora.py` now disables automatic maintenance on each exact depth-one
history fetch and retains each root under `refs/al-history/<SHA>`. It records
Git trace events and checks each resulting ref. It neither fetches broad history
nor retries a failure. `corpus_compare.py` publishes the failed preparation's
identity and every unavailable comparison when no native stages were reached.

Executed evidence: `validation/release/build/second-followthrough-history-fixed-v1/`
starts with a fresh depth-one checkout and fetches all seven pinned historical
commits through HTTPS. Every tree matches the local historical tree exactly.
The actual failed Linux preparation receipt produces fourteen explicit unavailable
comparisons without changing its bytes. Two focused unittest controls additionally
exercise a failed fetch, separate fresh recovery, retained old receipt, exact
shallow roots, and unavailable prerequisite publication. No native corpus was run
by these checks. The next Linux replay must still execute all thirteen native
corpora and the Rust comparisons, and confirm the staging correction there.

Original investigation directories are `second-followthrough-history-v1`,
`second-followthrough-history-race-v1`, `second-followthrough-history-repeat-v1`,
`second-followthrough-history-overlap-v1`, and `second-followthrough-history-https-v1/v2`
under `validation/release/build/`. The v1 HTTPS attempt failed DNS inside the
sandbox; v2 was the authorized network execution. No failed receipt was replaced.

## Observation corrections

`m2.py` compiles the historical exporter unchanged, then a separately hashed
successor changing only NaN serialization to preserve `signbitq`. Both campaigns
run against the same A2 library and inputs. Recursive neutrality permits only
raw `NaN` becoming `-NaN`; finite values, infinities, signed zero, arrays and
membership must remain identical. Eight native class controls include both NaN
and zero signs. The local `second-followthrough-m2-v1` campaign passes neutrality
for all 31 nonempty JSON products with no changed values on macOS.
`second-followthrough-m2-comparison-v1` retains two SVD-order failures under both
observers. The Linux ten NaN-sign rows and two spin callback rows still require
the successor execution; they are not accepted from this local result.

The AL polyco successor preserves construction, evaluation, modern-file and
header gates. For legacy coefficients it checks the exact `%25.17le` token from
the source binary64 cast, then the measured-width `%Lf` parse of that token.
The 31-significant-digit companion supplies a decimal rounding interval for the
pre-write timing value. Both interval endpoints must convert directly to the
same binary64 value, including binary64 exponent/subnormal handling; an ambiguous
cast fails. Exact scientific-format zero preserves its sign without a fabricated
nonzero interval. The old lossless comparison rows remain in the receipt.
Generated predictor files are now selected as evidence so the next Linux packet
retains the actual tokens; the previous packet did not retain those `.dat` files.

`polyco_write_read.cc` executes the three source writer roles (double, ordinary
long double and binary128), their actual formats and ordinary parser. Its eighteen
native witnesses pass locally. Three observer unittest controls cover neutral
NaN correction, wrong token/readback rejection, ambiguous conversion rejection,
signed zeros, subnormals and overflow. Offline successor checks pass for all
seven retained macOS polyco cases in both native/Rust directions. Full successor
macOS generation/exchange retains seven RMS failures; Linux's old 168 coefficient
failures and ten RMS failures remain bound to the original run.
The qualified full observer campaign is `second-followthrough-predictors-v2`:
14 cases and 37,814 checks, using the retained pre-repair installed executable
(`57dbf859…`). It qualifies the revised observation contract, not a fresh full
generation run of the later event/Lomb production candidate. The v1 campaign was
started before the signed-zero helper correction and is retained as exploratory.

## Cache lifetime design for central review

The common successor caller flushes stdout after all foreign pthreads join and
before process shutdown. Historical callers and the truncated Linux capture are
unchanged. `second-followthrough-cache-v2` executes the original installed library,
the explicit-local-cleanup caller, and a guard-only library copy: each produces
twelve complete identical workflow blocks and identical storage products locally.
Plain/ASan/TSan and the two ASan interventions exit zero on macOS. This does not
reproduce or resolve Linux's retained 544-byte/16-allocation leak.

The concrete proposed production change is generated by `cache_lifetime.py` as
`library-owned-cache.patch`: a zero-sized `ForeignThreadCache` in thread-local
storage, registered inside the existing `finish()` panic boundary. Its destructor
calls locked Rug 1.30.0's safe `float::free_cache(FreeCache::Local)` on that same
thread. Repeated calls and callback reentry reuse the guard; cleanup occurs once
at foreign pthread exit after synchronous calls have returned. No borrowed C
pointer is retained, no global cache is freed, and no public cleanup API is added.
The existing C clients exercise repeated calls, reentry and recovery.

Production still has no guard. The next Linux run first requires twelve flushed
baseline/explicit-cleanup blocks and all storage products to match; only then
does the runner build and execute the exact one-file guard counterfactual. This
preserves the leaking baseline even if the guard proves effective. Central must
review the concrete lifetime change before publication as production. Sanitizer
instrumentation covers the C callers/shim, not Rust/MPFR/providers.

## Measured source arithmetic

Lomb's nine contraction sites now select the demonstrated source profile:
fused on macOS arm64 and separate multiply/add on Linux x86_64. Every input,
grouping and bin stays present. The diagnostic's frozen `lomb_baseline.rs` is
the exact `4accd26` body; it remains a historical counterfactual after the repair.
On macOS the installed `5c8c0aa` API is exactly unchanged from the retained old
output and passes all 102 quantities against native default/fast. The unfused
counterfactual matches native off and retains its Nyquist difference from
default/fast. The corrected installed Linux API has not yet executed.

`ordinary_long_double.rs` extracts the existing predictor rounding kernel,
including its single-rounding/subnormal behavior, into a neutral arithmetic
module. Predictor-specific formatting, cosine and contraction are not used by
events. Ordinary has 53 significand bits on macOS arm64 and 64 on Linux x86_64;
source timing-wide arithmetic remains binary128 versus Rust Rug128. macOS basic
multiply/divide and exp/log retain native f64 behavior, including class signs.
Linux uses the ordinary-width Rug operations; agreement with actual native
libm exp/log is an explicit next-run gate, not an assumed theorem.

For EXP and GAUS, frequency division is double before promotion; epoch difference
is wide before ordinary assignment; amplitude, time scale and index narrow on
assignment. Each ordinary exponent operation follows the source grouping. EXP
phase multiplication stays wide; GAUS narrows its wide product into ordinary
`val` before wide phase accumulation. All eight callbacks retain their distinct
wide promotions, ordinary intermediate assignments and final double return.
The Gaussian epoch sign and width denominator `6.` are preserved literally.
No frequency convention, timing precision or tolerance changes.

The native expression-type probe compiles in the real A2 header/macro context.
Eleven authored cases produce **209 exact ordinary/callback comparisons**,
including all eight callbacks, defaults, onset neighbors, negative/zero widths,
signed amplitudes and a large-amplitude Gaussian witness. All pass on macOS in
`second-followthrough-events-profile-v2`. The initial v1 probe exposed one NaN
sign mismatch in macOS zero-width division; that receipt remains, and the fixed
source-operation path passes v2. These witnesses do not establish exhaustive
ordinary exponent-range or transcendental correctness.

The installed candidate also passes both original event timing gates and the
Gaussian-only native gates with/without mean removal. Eighteen native executions
retain original/observed/f64-temporary identities; the observed path is neutral.
All six full residual JSON products are exactly equal to the retained pre-repair
Rust executable on identical paths. The event-amplitude fit passes its native
gate and is also exactly unchanged. The Gaussian-only amplitude is a numerical
precision witness, not a proposed astrophysical model.

## Focused verification and open obligations

The source audit covers 93 library units, 148 unchanged Rust files and nine changed
entries (including two additions), all 36 identical SOFA sources and 63 lexical
unsafe occurrences. It is correspondence accounting, not full branch coverage.
The fresh `5c8c0aa` private installation contains 260 files. All-target Clippy
passed; formatting, twelve predictor tests protecting the extracted rounding
kernel, and seventeen Python harness controls passed. The later collector-only
edit passed its two focused controls and Python compilation. A new ignored event
probe requires explicit fixture/output paths; it does not silently expand the
ordinary unit suite.

This is a focused macOS follow-through, not a new full matrix. Required Linux
execution is the complete integrated pipeline at the final reviewed candidate:

1. Fetch all seven exact historical roots in a fresh shallow checkout, then
   actually run all thirteen native corpora and their Rust comparisons. Retain
   every raw, corrected, observer and coverage identity; unavailable rows do not
   substitute for corpus execution.
2. Execute both M2 observers against the same inputs and retain raw comparisons.
   The two spin-value discrepancies and raw SVD-order failures stay open.
3. Execute source polyco writer/parser witnesses and full generation/exchange;
   retain actual coefficient files, all old lossless rows and the ten prior Linux
   RMS failures. New source-token checks must pass independently.
4. Capture all twelve baseline/explicit-cleanup/guard workflows and storage
   products. Prove library-owned cleanup removes the measured Linux leak without
   product changes before central considers production promotion.
5. Verify installed Lomb against native default/off/fast, and the actual event
   expression types, ordinary exp/log, eight callbacks, Gaussian-only witness,
   original timing gates and affected fits. Passing macOS does not settle Linux.
6. Rerun the full existing fit/output/DDH/utility/provider/interface/worker gates
   and coverage ledger. Keep inherited failures visible. Neither the event repair
   nor Linux ordinary-width differences explain away macOS failures.

The [four remaining ordinary-type proposals](RELEASE_ORDINARY_TYPE_PROPOSALS.md)
are ordered spin callbacks/scales, chromatic accumulation, cosmic-string offset
and secular constants. Their per-site causal evidence precedes implementation;
none was included in production. AL and M2 acceptance remain central decisions.

## Bound evidence for central replay

The final selected package is
`validation/release/build/second-linux-followthrough-evidence-v2/`, with archive
`validation/release/build/second-linux-followthrough-evidence-v2.tar.gz`.
It contains 8,037 files / 66,411,856 bytes (including the manifest). Every selected
file and all 8,037 archive members were read back and hash-verified. The compressed
archive is 7,556,529 bytes. The package binds all 157 current Rust source files
byte-for-byte to the tested `5c8c0aa` export. It includes the original M2 JSON,
named predictor tokens, actual fit products, generated observer/caller sources,
recipe snapshots and exploratory failures; no binaries, runtime data or caches.

| Object | SHA-256 |
| --- | --- |
| Index | `278c7b369c71db0597f9b435c2cb4d313b6ba25fb1691552dc52df5e18e3f425` |
| Per-file manifest | `1014ce35eb4142da3f626dbbecd5b81d00392c5df7fa5fc24c6749a63941d9fd` |
| Archive | `e175e6656b4cd53a06eba5b39ab22024a6185c7c256bb8b2e35c34fdb8186d2f` |

The package index identifies every source directory and selected subdirectory.
Key receipts below are relative to `validation/release/build/`; their complete
bytes and hashes are retained by the package manifest.

| Receipt | What it establishes |
| --- | --- |
| `second-followthrough-history-fixed-v1/receipt.json` | Seven exact shallow roots, original failed Linux preparation preserved and fourteen unavailable rows published; no corpus execution |
| `second-followthrough-m2-v1/receipt.json` | Original/signed observer neutrality for 31 nonempty products and eight native class controls |
| `second-followthrough-m2-comparison-v1/receipt.json` | Two macOS SVD-order failures retained under both observers |
| `second-followthrough-predictors-v2/receipt.json` | Full revised observer campaign, seven inherited RMS failures retained; pre-repair executable |
| `second-followthrough-cache-v2/receipt.json` | Twelve complete matching workflow/storage products across baseline, cleanup and diagnostic guard on macOS |
| `second-followthrough-events-profile-v2/receipt.json` | 209 exact native/Rust type, ordinary and callback observations |
| `second-followthrough-events-v1/receipt.json` | Six original/control timing products, exact macOS before/after identity and observer neutrality |
| `second-followthrough-event-fit-v1/receipt.json` | Native event-amplitude fit gate passes and exact before/after JSON identity |
| `second-followthrough-lomb-v1/receipt.json` | Installed/current/historical/unfused identities and all native compilation-profile comparisons |
| `second-followthrough-source-audit-v1/receipt.json` | Source, responsibility, SOFA and unsafe accounting at `5c8c0aa` |
| `second-followthrough-final-checks-v1/receipt.json` | Seventeen harness controls, formatting and twelve predictor tests |
| `second-followthrough-collector-v1/receipt.json` | Final selected-input/exclusion controls and Python compilation |

The package's recipe snapshot is `ba12c8f`; its only cache-runner changes after
the executed guard experiment are a 900-second cold Cargo-build allowance (caller
execution remains 120 seconds) and evidence selection. The executed earlier
cache recipe is retained in Git at `ca60751`. No result is presented as a rerun
of that later deadline edit. The v1 package is superseded by v2, which additionally
retains the actual NaN observer/control sources and historical exporter/generator.

The final review directory is `validation/release/build/publication-review-v4/`.
Its receipt binds the full candidate SHA, scoped patch from central `4472d99`,
full delta from Linux `4accd26`, final reports, unchanged workflow and this evidence
archive. Those generated files remain ignored under decision 0039. Central's
independent observer/cache review is bound by
`5868a7f65e918dd21d39360ed7b209e749220c3679e971f588e740166c7b5c36`;
its source reviews and this handoff do not constitute Linux or AL acceptance.
