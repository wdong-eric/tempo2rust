# Accept the bounded JPL reference evidence

Central review, 23 October 2026. Accept Assignment H at `9917229`, `c642f3a`, `54162de` and correction `1df4d37` as source-reference evidence under decision 0011. This accepts no Rust JPL implementation, portable full-call C numerical result, M2 gate or complete-core milestone.

## Reviewed evidence

The [report](/Users/Eric.Dong/Downloads/tempo2rust/validation/jpl/REPORT.md), source map and coverage matrix account for all 12 implemented functions, the declaration-only `make_sub_ephem` and the raw Sun pointer macro. The ordinary cases contain 55 scenarios and 1,443 ordered events, using the available DE405 and declared synthetic layouts. Both file byte orders, header branches, getters/constants, query selection, interpolation demand, cache failures and file lifetime are represented. Ten additional isolated cases classify invalid/terminating source operations.

The canonical source remains `fda6ad4a9753ad7dc5d291406f2f310a9161423f`. Both independent repositories were clean and pinned when reviewed; A2 remains `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`. H uses unmodified canonical JPL bytes, not the corrected retained C reader or a new initializer patch.

| Artifact at corrected delivery | SHA-256 |
| --- | --- |
| `validation/jpl/outputs/reference.json` | `2efb8fe4cc91489af856bde4baaf15abc2b2ee03265238db24ff1e41eb5a60f5` |
| `validation/jpl/coverage.json` | `575b27bdd58dd5154dbb838b24d3449e8647d87883a91b5277322da8f72db068` |
| `validation/jpl/outputs/checks.json` | `e0445b8a5790e3dc7958c6eb80bb555990b843b06c88d001ea5560d901479ff5` |
| `validation/jpl/outputs/isolated.json` | `c8bb58b13fa69a0e979bab182e48657c7ea93f9972e1a5f6477a216f42182c82` |
| `validation/jpl/REPORT.md` | `7284f3162287fd8deff140af102bc75bf8524091640f203656c3b0366fbd660f` |

## Central verification and correction

Central inspected the harness, exporter, projection/classification logic, source map and required source routines. The first independent default run reproduced the ordinary numerical/reference data exactly but failed comparing sanitizer summaries: ASan sometimes returned source locations and sometimes executable offsets. Status and failure type were unchanged. This was a reproducibility defect in diagnostic comparison, not a numerical discrepancy.

H corrected it in `1df4d37`: exact comparison now gates failure kind, return status, completed stage and the expected source location when reported. UBSan/assertion source-location requirements remain strict. Variable symbolization text is preserved separately in `outputs/isolated_diagnostics.json`. The correction retains both observed trace presentations as regression fixtures and detects wrong kind, stage, location, return status, missing cases and altered assertion condition. Numerical references, inputs and analytic results are unchanged.

Central reran the corrected `run.py` entry point with only generated check/diagnostic summaries redirected to `/tmp/tempo2-jpl-central-final`; all source builds, comparisons and mutations executed normally. All 55 cases repeated in fresh processes and matched exactly. The nine artifact mutations, five compiled disposable source mutations, coverage deletion check and ten isolated classifications passed. Central also ran `python3 -m unittest discover -s validation/jpl -p test_isolated.py -v`: all nine tests passed. `analyze.py` reproduced its existing analysis byte for byte. No tracked H file changed during final review.

The separately initialized interpolation helper has 576 exact-rational polynomial/derivative comparisons with zero observed absolute difference on its dyadic inputs. Maximum synthetic magnitudes are 311.3125, 14811.125/day and 431353.75/day². The DE405 direct/pleph combination has 17 finite components with zero observed difference and one matching +Infinity class. These are bounded diagnostics and host observations, not general scientific error bounds.

The original failure sequence remains `0,-2,0,0`; the earlier corrected-reader sequence `0,-2,-2,0` is separate historical evidence. A central disposable opening-sequence probe additionally matches all 55 cases / 1,443 events with a same-descriptor read/rewind/`fdopen` adapter. Its source and result are recorded with the later implementation contract; no production code was changed here.

## Interpretation and next stage

Canonical initialization leaves some Sun/interpolation fields unwritten. H's explicit controlled-state experiments do not establish native initial values. UBSan also diagnoses an out-of-range IPT pointer formation inside ordinary `jpl_state`. Thus full state/pleph numbers remain reproducible **host observations**, even where arithmetic agrees exactly. Misaligned getter dereferences, failed pleph locals, inadequate caller buffers and partial elements have explicit classifications. No undefined result is promoted to a portable golden.

File lifetime and stdio buffering are part of the observed contract. Path replacement preserves the open inode; in-place changes affect later uncached reads; a cached record remains cached. The neighboring-record short read returns 680 buffered bytes despite 137 bytes remaining on disk. A later Rust port must preserve this behavior through the selected bounded C-stdio boundary and safe Rust state/indexing; decision 0017 specifies that release and its undefined-source dispositions.

Only macOS arm64 with the recorded GCC/Clang profiles is verified. Other hosts/libcs, concurrent mutation, global-error races, arbitrary malformed layouts, other real ephemerides, exhaustive resource exhaustion and full branch coverage remain unaccepted. H's implementation window is closed after this correction. M2 remains unaccepted under decisions 0012/0015, and I requires its own contract and central acceptance.
