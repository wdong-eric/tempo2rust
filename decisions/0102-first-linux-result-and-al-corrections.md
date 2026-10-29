# First Linux qualification result and AL correction brief

Central review, 30 October 2026. The authorized first Linux run completed, but
**Linux and AL remain unaccepted**. Decision 0101 continues to authorize centrally
reviewed fixes, publication and reruns within AL scope. AL retains implementation
ownership; central retains design, numerical policy and milestone acceptance.
Covariance profiling remains after AL under 0099.

## Frozen execution and evidence

[Run 40903715685](https://github.com/wdong-eric/tempo2rust/actions/runs/40903715685)
executed candidate `b59a84024317653d92cfd757709b2beee3e766a3`. The Linux job took
29 minutes 45 seconds. Checkout, provisioning, evidence collection and upload
passed; the matrix exited 1. Actual recorded host: Ubuntu 24.04.5 LTS, x86_64,
runner image `20261026.354.1`, Rust 1.97.1 / LLVM 22.1.6, GCC/GFortran 13.3.0 and
Clang 18.1.3. Provider and predictor ABI qualification are incomplete despite
successful package installation and backend comparisons.

Central downloaded artifact `tempo2-linux-40903715685-1` (ID 12912911728),
40,601,296 bytes, verified its GitHub SHA-256 and every selected manifest entry:
**6,580 files / 267,082,716 bytes**, no missing, extra or mismatched files.
The retained packet is
`decisions/evidence/release-qualification/build/linux-run-40903715685-review-20261030/`.
It contains API identities, original archive, extracted evidence, job log and the
central review. Generated JSON/logs/archives remain ignored under 0039.

| Evidence | SHA-256 |
| --- | --- |
| Downloaded archive / GitHub digest | `df69c6ef346ae5ddef5562c137d6f89c8947ae5ce2e4922468f12c8a87a0c20c` |
| Selected manifest | `c99c608a2f7d79d4df5ba08ea892d7748c2505d1176891951f0fc934be6cbe04` |
| Central review | `c866eef82c916ba11dbb67519a5c4271c7d9cb5c9d6747682aceb1717e613baf` |

`decisions/evidence/release-qualification/central-linux-run-audit.py` is the
executed read-only verification recipe. Repeating it requires a fresh output
filename. It verifies identities and summarizes original gates; it does not
rerun either scientific engine or convert failed checks to passes.

## Matrix result

The original receipt records **60 stages: 45 pass, 12 fail, three blocked**.
Six paired backend profiles pass, as do both installations, external Rust
consumers, source audit and package audit. The latter checks 2,474 installed
files with zero mismatches; all 155 audited Rust bodies match the baseline.
These passes retain their individual scope.

| Failed stage | Observed result and required disposition |
| --- | --- |
| `provider-abi` | Static CALCEPH link cannot resolve `floor`; ELF command omits direct `libm` dependency. Correct the harness link, then measure the actual ABI. |
| `c-default`, `c-combined` | Native boundary caller cannot resolve `nextafterl`; ELF command omits `libm`. Correct direct dependencies and complete all installed C11/C++17 linkage, storage, exchange and layout checks. |
| `rust-suite` | Both library profiles execute 94 passing, two failing and 36 ignored tests before Cargo stops. Failures are `source_failure_boundaries_and_nonfinite_transport` (`-NaN` versus `NaN` strings) and `predictor_t1_legacy_d_exponents_and_reader_bounds` (decimal parsing versus f64-derived expected constants). Diagnose against native platform semantics and existing contracts before changing expectations. Ten separate runtime-enabled invocations pass, totaling nine tests per profile. The current summary incorrectly reports the failed aggregate as unexecuted/zero passing tests; preserve failure while accounting for partial execution and unrun targets. |
| `c-concurrency-default`, `c-concurrency-combined` | Plain and TSan executions complete all twelve caller-exclusive workflows. ASan/LSan reports 544 bytes in sixteen allocations with GMP/MPFR stacks. Distinguish dependency thread-cache lifetime from lost application objects using focused evidence; no suppression or unproved leak-fix claim. Instrumentation covers C caller/shim, not the complete Rust/dependency graph. |
| `contraction` | One of 102 Lomb comparisons fails: `0.power.7`, native 0.068781682269196565 versus Rust 0.0520084305734319907, difference 0.0167732516957645743. The recorded native default/off/fast outputs agree; all 32 QR cases pass. Preserve and localize this scientific discrepancy. |
| `timing` | 128/130 comparisons pass. Only `events`, both modes, fails; largest observed residual difference is approximately 9.946e-10 s against the unchanged 1e-12 s gate. Preserve and localize the Linux-specific discrepancy. |
| `fits` | 1,862 reported numerical failures; compare their identities to the existing ledger, not just the total. |
| `boundaries` | Four known discrete precision-boundary failures remain. |
| `ddh` | 542 current-clock plus 520 historical-clock fidelity failures; all historical 1 ns ideal checks pass. Keep current/historical resources and the paired requirements under 0087 distinct. |
| `output` | Seventeen failing groups, thirty differing files and three missing files; no extra files, unexpected statuses, changed inputs or artifact mismatches. Compare with the earlier twelve-group macOS ledger and retain new Linux differences. |

`performance` and `predictors` were blocked by `c-default`; `m2-comparison` was
blocked by `rust-suite`. They were not executed and must run after prerequisites
are corrected. This b59 matrix also explicitly lacks the portable early corpora,
full measured-worker campaign and complete source/branch closure. AL's fuller
local follow-through remains required; the first run does not reduce that scope.

## Assigned follow-through

Central relayed the evidence paths and correction brief to AL's existing chat
`01a17c51-359e-7041-b04b-efebd2b2ec6e`. Finish the separately identified corpus/worker
handoff, then commit Linux corrections and diagnoses in scoped stages. Preserve
this first failed evidence and all predecessor evidence. Fix direct ELF linking
and truthful result accounting first. Resolve platform-specific test assumptions
only after native/contract diagnosis. Investigate the leak report, Lomb and event
model discrepancies without suppressing checks or silently changing science.

Return the exact candidate, source/diagnosis references, focused verification and
updated unresolved ledger for central review. Central coordinates publication and
rerun under 0101; no repeat permission request is required for that approved
scope. Every blocked capability and the fuller campaign must remain accounted for.
No precision, scientific assumption, public output convention or tolerance change
is made by this decision. Any such proposed change still requires an explicit
central decision, and full numerical/release acceptance remains pending.
