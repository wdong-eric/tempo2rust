# Assignment AC: general input-to-BAT implementation

**Released by decision 0069 from accepted AA `ea9797236fae0f0dd29a32e5d3050e33eaf2fefe` (production `99cf714`).** AA ownership is closed. AC is the sole Rust writer for this functional implementation assignment.

Work in `/Users/Eric.Dong/Downloads/tempo2rust`. Read HANDOFF.md, [contract 0068](../decisions/0068-general-input-to-bat-prepared-contract.md), priorities 0065, current bug policy 0053, generated-output policy 0039 and preservation rule 0050. The direct M2 exact-order requirement in 0012 remains unchanged. Use the accepted AA revision and actual interfaces named above.

Own only `rust/` and new `validation/input_to_bat/`. You are not alone in the shared repository: preserve other chats' changes, accepted evidence/runners and both independent source repositories. Central retains design and milestone acceptance. Use source-mapped modules and adjacent explanations for each necessary unsafe block and demonstrated source correction.

Deliver a general `bats` library/CLI workflow: ordered PAR/TIM pairs, source preprocessing, native-JPL planetary/observer/propagation calculations, full global.C formBatsAll/refphs sequencing, and SAT/BAT/BBAT/correction/provenance output. Keep the existing residuals/fit contracts bounded. Preserve source units/frames and input selection; reading a binary/noise parameter does not establish its residual or fit implementation. Enabled CALCEPH and complete upstream CLI/plugin compatibility remain explicit later core obligations; the current no-CALCEPH error path must work.

Proceed through coherent internal stages:

1. Reuse K/Q/N/P/Z/AB maps and freeze only the concrete public API/output and integration mapping against accepted AA. Define authored end-to-end inputs.
2. Implement the complete scoped PAR/TIM and preprocessing responsibilities on canonical state. Correct the documented no-setdm ndm initialization with source comments and focused corrected-native/default/neighbor evidence.
3. Transfer all scoped planetary, propagation, DM/solar, interpolation/shapelet and phase/secular routines, reusing existing resource/math owners and accepted corrections.
4. Connect global orchestration and public examples. Remove replaced subset calculation bodies and retain thin bounded legacy frontend validation.
5. Run affected component comparisons and full new workflow checks in debug/release; preserve original evidence, known numerical gates and exact identities. Avoid redundant historical/native replay for documentation changes.
6. Commit explicit owned authored paths and return `rust/INPUT_TO_BAT_REPORT.md` for central acceptance, with runnable public examples, measured comparisons, source/unsafe/behavior maps and remaining limits.

All reproducible generated JSON/JSONL/compressed evidence, binary fixtures and large outputs stay ignored. Small demonstrated bugs may be fixed under 0053; unexplained scientific/numerical-policy differences require central disposition while independent work continues. No private fixture adapter or prepared canonical state alone satisfies the final input-to-BAT deliverable.

## Dispatch prompt

Implement Assignment AC under decisions/0068-general-input-to-bat-prepared-contract.md and handoffs/29-general-input-to-bat.md from the accepted AA revision named in central's release. Own rust/ and validation/input_to_bat/ as sole Rust writer. Deliver the complete scoped source readers/preprocessing, native-JPL planetary/propagation and global orchestration as a usable general bats library/CLI workflow. Reuse all accepted references and canonical state/resources, preserve source notation and conventions, apply only demonstrated source-mapped bug corrections, and keep original/corrected evidence distinct. Implement in coherent stages through public end-to-end examples and affected verification; do not stop at internal routines or another reference campaign. Commit authored owned paths with Conventional Commits, leave reproducible output untracked, and return rust/INPUT_TO_BAT_REPORT.md for central acceptance.
