# DDH ideal-fixture failure localized to clock-resource version

Central review, 26 October 2026. **The observed pre-fit failure of the original DDH ideal fixture is explained by its GPS-clock resource version.** With the same A2 executable, original PAR/TIM inputs and current runtime, all 284 recorded residuals regenerate exactly and 120 exceed the original 1 ns threshold. Adding only the GPS-to-UTC table from the Git revision that last updated the fixture makes every residual satisfy that same threshold. The unchanged full upstream `checkIdeal` test then passes both before and after fitting.

**Acceptance disposition is pending the user's answer.** Central has proposed retaining current production clocks and the original failed observation, then requiring current-clock Rust/native agreement and the unchanged 1 ns ideal check with explicitly versioned historical clocks. This record accepts the bounded causal evidence; it does not yet adopt that replacement acceptance rule, accept Rust DDH behavior or release another writer. AE remains active; independent functional work continues under 0065/0074.

## Controlled experiment

The original fixture is `tests/test_data/test3.par` / `test3.tim`, pulsar J1017-7156, DDH/DE414/TT(TAI)/IF99. The actual input contains 284 TOAs, despite NTOA metadata saying 294. No input value, model parameter, fitted flag, threshold, source algorithm, executable or compiler profile was changed for the pre-fit comparison. The exporter executes the original simple-preprocessing/BAT/residual path with `removeMean=0` and no fitting.

| Case | Changed resource | Pre-fit observations at or above 1 ns | Maximum absolute pre-fit residual |
| --- | --- | ---: | ---: |
| Current pinned runtime | None; exact A2 replay | 120 / 284 | 4.598955163859953392 ns |
| Disposable historical-clock overlay | Add one historical `clock/gps2utc.clk` | 0 / 284 | 0.465895042333431499 ns |

The overlay retains every current runtime entry and adds the historical file; the original runtime has no `gps2utc.clk`. Its other entries are read-only symlinks to the existing resources. The baseline opened 82 distinct read resources and the overlay 83. All 82 original resource identities were checked before and after; canonical TEMPO2 and the independent A2 repository remain clean. The original DDH JSON and all accepted evidence remain unchanged.

Parsed observations, parsed parameter values/flags and all pulse counts are identical between the two runs. The first TOA remains the phase reference. The source selects the historical direct `UTC(GPS) -> UTC` edge instead of the current intermediate `UTC(GPST)` or `UTC(GPS_USNO)` route. The first TOA's total clock correction is unchanged. Across the full dataset, the residual change follows the change in total clock correction relative to that reference within `2.16062095490488895e-13 s` (0.2161 ps). This is a measured response, not a newly assumed unit derivative or tolerance.

For the first failed row, index 35, the clock change is `-1.171116537079797126e-9 s` and the residual change is `-1.171144751031100840e-9 s`. At the original worst row, index 38, they are `-4.531628405857190955e-9 s` and `-4.531418131468589754e-9 s`. The intervened resource accounts for the nanosecond discrepancy without a DDH formula change, a fit, a time-precision change or removal of active physics.

## Source history and full-test control

The fixture's last update is `0759584ebf8315bbbe6a6d0b41c08ebbcfe527f2` (11 November 2020), titled "Update tests for new shapiro code." The historical table comes from that exact revision. `f4deb54466cec48682d940ea2aabadbda3925e06` removed the legacy table in December 2023 during GPS-clock revision. `6650a302cf01e36d4f96d34097d5637048ed8d94` added the separate GPST/GPS-USNO routes in April 2024; their source coverage boundary includes MJD 55559. This establishes the source/resource chronology. The original fixture-generation environment was not fully recorded, so do not claim an independently recovered generation recipe.

A separate control executes G's already rebuilt but source-unchanged `tests/testEndToEnd.C::testEndToEnd.checkIdeal`, with its original `TEST_DELTA=1e-9l` and verified original compile-time PAR/TIM files. With current clocks it returns 1 at the same pre-fit `1.13759e-9 s` assertion. With the one-file historical overlay it returns 0; the test reaches `t2Fit` (20 parameters, 284 data rows) and both pre-fit and post-fit assertions pass. No source test, tolerance or input was edited. The full-test executable's identity matches G's recorded evidence.

This localizes this fixture failure to a calibration-version mismatch; it does not show that the current GPS tables are scientifically wrong, validate the DDH physical model generally, establish Linux behavior or accept the Rust port. Keep current production/resource selection behavior. The old blanket description "cause unknown" is superseded for this particular observed failure, while its final acceptance treatment remains pending.

## Proposed acceptance rule, not yet adopted

Central recommends the following paired requirements, subject to the user's pending answer:

1. Preserve the original current-clock failure and its exact identities as a historical/native-behavior record. Verify Rust against the current-clock native path using the existing quantity-specific port gates, exact input/order/pulse/state checks and matching preprocessing/reference conventions.
2. Keep `abs(residual) < 1e-9 s` before and after fitting for an explicitly versioned ideal fixture: unchanged original PAR/TIM inputs, the single historical GPS table identified below, and otherwise the pinned runtime. Verify the full native and Rust workflows; this review supplies native evidence only.
3. Label resource versions and the two purposes explicitly. Do not make the old table a production default, overwrite accepted original outputs, regenerate TOAs to hide a discrepancy, relax 1 ns, or report the current-clock original assertion as passed.

If adopted, these requirements supersede the unexplained-DDH acceptance placeholder inherited by 0079/0086 while preserving an actual ideal-accuracy check and current-source fidelity. They would still need the required Rust/integrated verification and central acceptance. If the user keeps the original gate pending, it remains a failing gate with the cause now documented. The P, M2 and 0049 ledgers are unrelated and unchanged.

## Reproduction and evidence

Authored probes are `decisions/evidence/ddh-clock/central-clock-review.py` and `original-test-review.py`, copied byte-for-byte from the executed probes. Run the first and then the second in the current workspace with the existing verified A2/G builds available. They create fresh temporary evidence and never write accepted reference outputs. The first writes `/tmp/tempo2-ddh-clock-current` for the second to locate its disposable overlay. Both use fresh native processes. No Rust code or binary was modified or executed in this review.

Preserved ignored evidence: `decisions/evidence/ddh-clock/build/review-20261026/`, including complete exports, original evidence copies, selected clock/phase deltas, actual resource identities, command/environment settings, logs, executables, source history and dependency provenance. Four recorded non-system dynamic dependency hashes were checked against A2. Generated JSON, native table copies and binaries remain untracked under 0039.

| Evidence | SHA-256 |
| --- | --- |
| Original A2 DDH JSON, unchanged | `d1fdd35a4cdacc386cd91bfce2b55e09994c914fc2c2ea3c7194ed5e223cca97` |
| A2 build manifest, matched to original DDH identity | `a5daa986c6c054c7fa314271b5fa918cedc3f7412707a9cc7670ec3306d92dc8` |
| Unchanged A2 exporter | `04f2df5b63b73990fb5640285ce8fa245a3f2377e0711824a4bbd59c31260c24` |
| Historical GPS table | `30727fdfaaec0f1775543804210d057ffe62076594bb8884d8f8ccafafb676be` |
| Controlled-export receipt | `80c351a1dc0f12a3ee0eb827542b8806e4732948efee91487f2629983df3fcd0` |
| Full upstream-test receipt | `74964828658da5324ee411313ca80868f66a5fde5b09e3e160fb25fd1485854f` |
| Source/dependency provenance | `9fc3e1ce9a2c894e4839ac16a4622dd0324bc6bbd42b352e21d62b0e2f66292c` |
