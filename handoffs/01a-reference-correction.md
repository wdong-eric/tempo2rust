# Reference correction and regeneration

**Completed at `7795d53` and accepted for M0 in [decision 0002](/Users/Eric.Dong/Downloads/tempo2-rust/decisions/0002-m0-acceptance-and-c-release.md).** The assignment and dispatch below are retained as historical requirements. C is now released; do not redispatch A2 merely because its original return report says central review is pending.

Assignment A2 continued the reference work under the [central design](/Users/Eric.Dong/Downloads/tempo2-rust/decisions/0001-reference-and-m1a-contract.md) and [shared handoff](/Users/Eric.Dong/Downloads/tempo2-rust/HANDOFF.md). Its approved scientific correction was to initialize the missing count before parsing.

## Ownership and reference identity

Own only `/Users/Eric.Dong/Downloads/tempo2-rust/validation/reference/`, continuing from assignment commit `5289797`. Other chats share the workspace; preserve their edits. The canonical TEMPO2 repository and central documents remain unchanged by this assignment.

Create a separate recorded source-correction patch that adds `ne_sw_ifuncN = 0` in `initialiseOne` alongside the related interpolation counts. Apply it to the disposable build source before instrumentation. Preserve the base source commit and include both patch identities in the manifest. Make the corrected build the canonical current reference; the earlier uncorrected evidence remains recoverable from commit `5289797`. Keep any retained uncorrected diagnostic reproduction explicitly identified as such.

Do not zero the whole structure, change the solar-wind density or IPM/SWM flags, reset the count after parsing, or modify delay formulas. Record the intentional behavioral change as removal of undefined initial state when no interpolation samples have been supplied.

## Required work

1. Add the source patch and focused regressions. Demonstrate that initialization overwrites a nonzero storage pattern with a zero count, absent `_NE_SW` entries keep it zero, and explicit valid `_NE_SW` entries populate the expected indices/count/values. Preserve the samples through the path that previously used the count. Do not substitute a harness-only assignment for the initializer fix.
2. Rebuild with the adopted GCC binary128 configuration. Run the precision checks and four residual/BAT tests. Record which checks exercise the corrected source and which describe the original defect. Update checks that formerly required the defect to exist.
3. Regenerate all six fixture exports with new build/correction provenance. Compare the three barycentric cases with the previous committed scientific values and explain any difference. Demonstrate stable terrestrial results with the count initialized independently of allocator layout. Preserve the meteorology warnings and declared defaults; record the actual limits of this check.
4. Update the proposed fixture schema and comparator for the central contract. Label observatory SAT, barycentric SAT, BAT, and BBAT correctly. Preserve engine-specific provenance without forcing identical compiler/allocation/instrumentation fields across implementations. Keep exact scientific conventions and per-TOA identity checks; document the scientific projection needed by C. Use the adopted barycentric gates; terrestrial gates remain explicit proposals for central review.
5. Retain the DDH `checkIdeal` failure at its original 1 ns threshold. Record the failing observation, full value, model/resources, and useful stage quantities. Perform bounded localization sufficient to distinguish what is known from conjecture; do not expand into a DDH rewrite or alter the test threshold. Full causal resolution is a later coverage task unless evidence shows the selected isolated fixtures are affected.
6. Run the relevant regeneration, comparison, serialization, and failure-publication checks once against the corrected build. Reject an altered residual/count and unsupported/failed/provisional data appropriately. Keep uncorrected or smoke-only exports from being silently promoted by a status change.

## Return evidence

Update the report and manifests in your repository and return the new revision and report path. Include the exact correction, test results, old/new scientific comparisons, new artifact hashes, terrestrial availability recommendation, and outstanding DDH or resource-policy limits. The report must distinguish a source defect fixed by initialization from unresolved numerical behavior and from implementation tests.

Terrestrial exports become candidates for central adoption only when the corrected evidence supports them. Do not mark M0 or M1b accepted. The central chat will check the return and decide whether to release C; no messages to other chats are implied.

## Dispatch prompt

```text
Continue Assignment A as A2 in /Users/Eric.Dong/Downloads/tempo2-rust/validation/reference. Read /Users/Eric.Dong/Downloads/tempo2-rust/HANDOFF.md, /Users/Eric.Dong/Downloads/tempo2-rust/decisions/0001-reference-and-m1a-contract.md, and /Users/Eric.Dong/Downloads/tempo2-rust/handoffs/01a-reference-correction.md. Implement the approved initializer correction only in the disposable reference source with a recorded patch and regressions, regenerate the evidence, and update the schema and report as specified. Preserve the canonical TEMPO2 checkout and the original failing DDH threshold. Return your report and revision for central review; M0 remains pending.
```
