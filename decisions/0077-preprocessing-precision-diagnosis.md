# Controlled preprocessing precision diagnosis

Central evidence decision, 25 October 2026. **The 154 differences in AC's released P preprocessing comparison are reproduced at 128 bits and all disappear in a controlled 113-bit diagnostic of the same pinned logic.** This localizes these recorded cases to the numerical-precision difference. It does not accept P/AC, change production precision, approve a new tolerance or close a discrete compatibility gate. Continue AD's functional implementation under 0065/0074/0075.

## Controlled result

The diagnostic archives AC production `18c906594595e18c92a59a83842d56bc2119d5de` into disposable storage. Both variants use that source, the same accepted P inputs, the same compiled dependency archives and the same pinned comparator. The only variant changes are `PRECISION_BITS` and seven test-fixture constructors made to use that precision. Their seeds are exact integers/dyadic fractions, so the constructor change changes storage precision rather than input values. No production or accepted reference file is edited. This exercises preprocessing; it does not change or test the whole timing engine at 113 bits.

| Diagnostic variant | Executed P histories | Return code | Field/event difference records | Diagnostic differences |
| --- | ---: | ---: | ---: | ---: |
| Production precision, 128 bits | 296 | 101 | 154 | 0 |
| Reference precision, 113 bits | 296 | 0 | 0 | 0 |

The 128-bit control matches AC's complete recorded difference list after normalizing only disposable work-directory prefixes, not merely its count. The original six external-plugin cases retain their explicit excluded disposition. NaN payload transport follows the already disclosed typed projection in 0074; no new field omission or comparison allowance is introduced. The 154 records remain the same five groups: SIMUL 32; units 71; ordered SAT offsets 2; linked preprocessing 4; epoch transformations 45. They are field/event records, not 154 distinct failed input files.

The compiled test uses the existing Rug/serde/serde_json/sha2 archives and retained SOFA archive, with its actual direct-rustc command and executable hashes in the receipt. No Cargo/shared-target rebuild occurs. The original AC handover receipt, AC difference list, native P observations and initial-state files are checked unchanged before/after execution, as are the selected dependency archives. This is a fresh diagnostic in one declared host/test configuration, not a fresh production debug/release or platform acceptance run.

## Cancellation witness

One continuous discrepancy comes from `units100`, `param_t0[0].val`, whose initial value is zero. `units.C::xform_mjd` evaluates `(value - IFTE_MJD0) * f + IFTE_MJD0`; `ifteph.h` supplies binary64 `IFTE_MJD0=43144.0003725` and `IFTE_KM1=1.55051979176e-8`, promoted at the source's extended arithmetic boundary. The result is close to -0.000669 days after subtracting large intermediate terms. A relative-to-result allowance alone ignores this cancellation.

A separate four-precision expression probe verifies the initial T0 label/value, reproduces the native 113-bit and production 128-bit recorded values exactly, and compares against the exact rational result for those binary64 constants. At 256 and 512 bits the expression equals that rational value exactly. Absolute error, converted from days to seconds, is:

| Precision | Absolute error against the exact source-constant expression |
| --- | ---: |
| 113 bits | approximately 2.3907985884e-25 s |
| 128 bits | approximately 4.1600086799e-30 s |
| 256 and 512 bits | zero |

The native/Rust relative difference is approximately 4.13655763e-27. This witness establishes a rounding/cancellation mechanism and greater accuracy of the 128-bit result for this expression. It is not independent physical validation of a real pulsar epoch, and does not prove every other continuous field converges in the same way. The full 113-bit replay supplies the broader causal localization; no full 256/512-bit corpus replay is claimed.

## Central disposition and next action

Retain production Rug128 and the source expressions. These results do not justify assigning a speculative arithmetic rewrite to AD or rerunning unchanged P failures before unrelated residual/binary work. They do justify treating the released P differences as a central numerical-policy item, with this concrete reproduction rather than an unexplained translation-failure label.

P's 122 continuous records still need a scoped comparison disposition that accounts for source operations, input/intermediate scales, units, cancellation and exact structural state. The existing K bounds in 0073 are not extended to P. Neither a blanket relative bound nor a tolerance selected solely from the largest observed error is approved.

SIMUL's 32 records change a discrete deletion decision at the strict 60-second boundary described in 0074. A continuous rounding allowance cannot accept them. Their numerical-policy disposition remains open alongside the four 0049 utility boundaries. M2's exact raw-order gate, the original DDH 1 ns requirement and all existing milestone gates remain unchanged. The 113-bit experiment is diagnostic evidence, not authorization for a production precision switch or a second numerical engine.

AC remains numerically unaccepted. Under 0074, AD continues on its reviewed functional baseline and inherits the same explicit unresolved ledger. Any later production correction belongs to the current Rust owner after central disposition. Full migration still requires resolution of these acceptance obligations.

## Evidence and reproduction

The exact executed main review source is `decisions/evidence/preprocessing-precision/central-review.py`, SHA-256 `5ac7cb138c85dfc3a3fe788a9a9d3524e05091765810bfe6da4789a0d529b293`. It regenerates the pinned disposable variants and preserves the original data. Its original output directory is `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-preprocess-precision-tblgbazu`. Minimal original-run receipts, compile/run logs and complete difference records are copied byte-for-byte to ignored `decisions/evidence/preprocessing-precision/build/review-20261025/`; `copied-artifacts.json` records original/saved paths and hashes. Actual original executable paths remain in the receipt rather than being relabelled as copied executions.

The main receipt has SHA-256 `d47655371e4b90dcedc9da022edf1242731e69c269f553891674e7d9abfa7873`. The parameterized `epoch-cancellation-review.py` was freshly executed against the copied 128-bit differences into the separate `build/review-20261025/cancellation/` directory. Its receipt has SHA-256 `92f0f7080feb56066e84c502cd1fd3914196f41e6d902e9f0271c7db6a4cdbe6` and records its source, compiler, executable, initial state and input identities. Generated artifacts and binaries are not committed.

To reproduce from the workspace:

```sh
python3 decisions/evidence/preprocessing-precision/central-review.py
# Use the newly printed output directory and a fresh destination below.
python3 decisions/evidence/preprocessing-precision/epoch-cancellation-review.py PREPROCESSING_RUN --output NEW_CANCELLATION_DIRECTORY
```

The original 0008 document and canonical TEMPO2/reference source identities remain unchanged. This decision records diagnosis and workflow disposition; it supplies no milestone acceptance.
