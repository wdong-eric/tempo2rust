# AC implementation handover and AD release

Central decision, 25 October 2026. **Approve AC's implementation handover at production `18c9065` and report `d47ae5135d64fe61e52dee56e0eb2eaa68355cdc`, close AC's ownership, and release Assignment AD under 0070/0074 and handoff 30. AC's numerical milestone acceptance remains pending.** This is an implementation release with a reviewed functional baseline, not a declaration that the full input-to-BAT equivalence gate has passed.

## Reviewed baseline and evidence

AC delivers the general PAR/TIM → preprocessing → native-JPL → propagation → BAT/BBAT library and `bats` CLI on canonical `PulsarContext`. Existing bounded timing/fit frontends use shared calculations and retain their declared schemas and validation. Read `rust/INPUT_TO_BAT_REPORT.md` at `d47ae51` and the contract at `rust/INPUT_TO_BAT_CONTRACT.md` for API, provenance, failure/recovery and source mappings.

Central's handover review verified 569 distinct recorded file identities, including the 496 authored files in the delivery manifest; checked that the main checkout and the independent TEMPO2/reference repositories were clean; and confirmed the original 0008 identity. It recomputed all 36 stored public comparisons (18 cases in each profile), checking native output/input identities and unchanged comparator measurements. Across the stored second-valued component/BAT comparisons, the largest absolute difference was `2.5011104298755526543e-15 s`, below the existing `1e-12 s` gate. Other dimensions retained their own gates and units.

Four fresh calls using the final existing release binary passed against their identified native results: `ordinary`, `mixed_mean_tzr`, `mixed_tzr_mean` and `mixed_tropo_disabled`. No rebuild or full historical rerun was performed. Earlier finite-workflow executions retain their actual executable identities; the report separately identifies the final negative-NaN parser/formatter correction and its focused native/debug/release evidence. These chronology distinctions are retained, not relabelled as one execution snapshot.

The reviewed delivered checks include K 304, Q 121, N 157, Z's 82 driver plus 15 random histories, both bounded terrestrial contracts, library/CLI recovery, standard tests, formatting and Clippy. Counts are scoped checks, not a migration-completion percentage. Six external-plugin P cases and the unavailable enabled-CALCEPH path retain explicit later-stage dispositions. P's 296 executed histories still fail at 154 differing field/event records. Original AB still records 277 differences in three histories; their separately corrected counterparts pass in both profiles.

The central reproducer is `decisions/evidence/input-to-bat/central-handover-review.py`, with copied original-run results under ignored `build/handover-20261025/`. It expects the recorded baseline checkout and existing identified binaries/resources. Its receipt retains the actual temporary execution paths and has SHA-256 `918b8de5ddb59c7ff22e8a1d8e9318f828c4247faa87e2948d8bb78fae1e6583`. It is a handover review, not a full independent replay of every component corpus or a physical validation of arbitrary timing models.

## Source-correction disposition

Confirm the two per-pulsar corrections under 0053: disabled troposphere must clear the currently processed pulsar's observation (`calculate_bclt.C`), and reference cleanup must test the same pulsar selected by reference initialization (`refphs.C`). The original expressions incorrectly target pulsar zero. Adjacent Rust comments identify source, cause and correction. Central inspected the narrowly changed native source/patch construction, verified the corrected proof and linked-object identities, and checked the fresh mixed-pulsar public behavior.

The preserved original histories reproduce the accepted AB oracle before the separate two-unit corrected native build is used. `bclt_tropo_0`, `phase_cross_0_1` and `phase_cross_1_0` then pass against corrected expectations. The original 9/149/119 record differences are acknowledged consequences of these approved corrections, not unexplained arithmetic differences. Keep the original failed comparisons and separate corrected evidence; do not make the original oracle appear changed or passing. This closes the central correction-review question for AC-AB-ORIGINAL without accepting all AC numerical behavior.

## Open obligations transferred to the current Rust owner

AC's complete difference records and `validation/input_to_bat/build/open-checks.json` remain available. The report's five P groups remain unresolved:

- AC-P-SIMUL: 32 deletion/whole-record records at the diagnosed 113/128-bit strict 60-second branch boundary.
- AC-P-UNITS: 71 unit-conversion value/error records.
- AC-P-STAGE2: 2 ordered SAT-offset records.
- AC-P-LINK: 4 linked-value/units preprocessing records.
- AC-P-EPOCH: 45 epoch-transformation records.

The 122 continuous records are localized, not proved equivalent by a blanket rounding argument. Decision 0073 remains limited to its named K expressions. No P tolerance, threshold adjustment or production113 mode is approved. M2's exact raw SVD order, the four 0049 utility boundaries and the original DDH 1 ns check also remain open. Central owns numerical disposition; after this handover, AD is the writer for any explicitly assigned correction. Preserve failure reproductions and avoid repeatedly running unchanged failures as a prerequisite to independent model implementation.

## AD scope and ownership

AD is now the sole Rust writer and owns `rust/` plus new `validation/timing_models/`. Start from the reviewed AC source and report, with this central release committed on top. Deliver the full general fixed-parameter `timing` workflow in 0070: complete residual/phase/tracking/averaging behavior, all fourteen binary families and their derivative/update wrappers, and the six-routine eccentric-GW dependency. Reuse existing G and AC/native reference evidence; do not replace this implementation stage with another reference-only assignment.

Freeze concrete public interfaces and the output projection against the delivered AC API, then implement the source responsibilities in coherent commits. Keep source notation/counterpart comments, one scientific implementation per routine, justified minimal unsafe, explicit original/corrected distinctions and the generated-output policy. AC's open numerical gates do not authorize silently changing its backend, adding a second pipeline, hiding failures or narrowing AD's scope. Report new dependencies or failures while continuing independent work. Return `rust/TIMING_MODELS_REPORT.md` for central review under 0074.

AC is closed and must make no further shared-source edits unless centrally reassigned. AE remains prepared and unreleased; it follows AD handover/ownership closure and a separate central release. Full migration remains incomplete.
