# First Linux qualification: scoped follow-through

30 October 2026. Authorities: decisions 0101 and 0102. Central coordinates
publication and the next run. This follow-through changes harnesses and two test
assertions; **production scientific Rust bodies, precision and gates are
unchanged**. It builds on full corpus/worker handoff `df587cd` and central review
`07ce62e`. It does not qualify Linux or accept the release.

## Original evidence and corrections

Original run: [40903715685](https://github.com/wdong-eric/tempo2rust/actions/runs/40903715685),
candidate `b59a84024317653d92cfd757709b2beee3e766a3`. Central verified 6,580 files,
267,082,716 bytes; manifest SHA-256
`c99c608a2f7d79d4df5ba08ea892d7748c2505d1176891951f0fc934be6cbe04`.
The unchanged original packet is at
`decisions/evidence/release-qualification/build/linux-run-40903715685-review-20261030/artifact/linux/`.

| Finding | Source/contract and change | Remaining verification |
| --- | --- | --- |
| CALCEPH `floor` and boundary `nextafterl` link errors | `provider_abi.py` declares Linux libm. `qualify_predictor.py` declares direct libm/ELF libdl dependencies for all native and installed C/C++ client links. Static Rust requirements still come from rustc. | Actual ELF links and provider ABI in the next run. |
| Failed library targets counted as unexecuted | `rust_suite.py` parses successful and failed summaries, retains passing/failing/ignored counts and marks incomplete execution. Re-accounting the original logs gives 94 pass, 2 fail, 36 ignored in each profile; exit 101 stays failed. | Full debug/release targets must run after correcting the assertions. |
| Square-fit test assumes positive NaN | `TKfit.C:461` multiplies errors by `sqrt(chisq/(ndata+nconstraints-nparams))`, runtime 0/0 here. `tkfit.rs` already preserves the resulting sign. The test requires a NaN multiplier and identical signed error transport, and explicitly checks positive/negative NaN serialization. No normalization was added. | New native `platform_semantics.c` witness and source-facing solver checks on Linux. |
| T1 test assumes exact binary64 round-trip | `t1polyco.c:106` writes `%25.17Le`, 18 significant decimal digits. These recover the original binary64 inputs, but reparse to different 64-bit-significand values. The test expects the three explicit written decimal tokens at native predictor precision. D-exponent/count assertions remain. | Native writer/reader exchange and 64-bit ABI on Linux. |

The native ordinary-long-double witness on macOS measures 53 bits, positive
runtime NaN and the expected decimal tokens. The original Linux test failure
retains negative NaN and the parsed coefficients; the new native witness has
**not yet run on Linux**. No universal arithmetic NaN sign is promised and no
source-facing sign comparison is removed.

## Diagnostic interventions; scientific gates remain failed

**MPFR cache.** Linux ASan/LSan reports 544 bytes in sixteen allocations, with GMP
allocation and MPFR cosine stacks repeated across four threads. Bundled MPFR
4.2.2 `doc/mpfr.texi` recommends freeing local caches before thread termination;
Rug 1.30.0 documents `FreeCache::Local` for that lifetime. This supports a
hypothesis, not proved attribution. `c_concurrency.py` adds a separate caller
using MPFR's documented local-cache cleanup at each thread's end. It compares
all twelve id/pass transcripts and stored TSV products with the original caller.
Original sanitizer exits remain in the gate; no suppression or production
cleanup was added. Both variants pass with equal products on macOS. Linux
attribution remains unrun. Only C callers/shim are sanitizer-instrumented,
not Rust/MPFR/providers.

**Lomb.** Linux native default/off/fast outputs agree; saved assembly uses
separate SSE multiply/add instructions at the recurrence sites. Rust explicitly
fuses nine sites selected from the earlier GCC16 arm64 witness. The failed bin
is preserved: 0.068781682269196565 native versus 0.0520084305734319907 Rust.
`lomb_diagnostic.py` extracts the installed function into a separate caller,
proves its unchanged copy equals the installed API exactly, then replaces only
those nine fused sites with products/sums. On macOS, all 102 quantities pass
against native contraction-off; the same Nyquist bin differs against fused
default/fast. This isolates the macOS contraction effect and prepares the Linux
experiment. Production arithmetic is unchanged; the Linux counterfactual is unrun.

**Events.** Only `events` fails among 130 Linux timing comparisons; maximum
residual discrepancy is about 9.946e-10 s against 1e-12 s. Native
`formResiduals.C:675,705` uses ordinary `long double` event temporaries; Rust
`residual_source.rs` uses f64. That boundary alone does not explain the magnitude.
`event_diagnostic.py` records native event temporaries and phase4, checks observer
neutrality against complete original state, and separately tests double native
temporaries. Both mean modes and an events-removed control are retained. All
twelve macOS native executions succeed; observed and counterfactual state equal
the original, as expected on that host. Linux traces/interventions remain
necessary before attribution or correction.

Both diagnostics are included in the next matrix. Their successful execution
status means controls ran; it does not accept counterfactual arithmetic or turn
the separate failed scientific stages into passes.

## Failure identities against macOS

Reproducible comparison:
`validation/release/build/linux-followthrough-ledger-v1/{producer.py,receipt.json,ledger.md}`.
It binds both hosts' receipts, field identities and input hashes.

* The fit aggregate **1,862** comprises **1,857 numerical mismatches and five
  execution failures**. All 72 cases have matching authored input hashes.
  Linux has 15 failing cases; macOS had 17, with 2,478 numerical mismatches and
  the same five execution cases.
* Newly failing Linux fit cases: `chrom-poly`, `event-amplitudes` (570),
  `position-decj`, `position-raj`, `solar-gp-fit-no-subtraction`, public `global`.
  Event-amplitude prefit phase/residual already differ while BBAT matches;
  maximum prefit residual difference is about 6.278e-9 s. This localizes an
  upstream timing difference without proving its cause.
* The same four discrete cases fail on both hosts: `half_below`,
  `negative_half_inside`, `mod_rounded_quotient`, `mod_negative_quotient`.
* DDH retains 542 current-clock and 520 historical-clock fidelity failures;
  historical 1 ns ideal tests pass. The receipt retains quantity maxima and
  both clock profiles; ideal success does not establish fidelity.
* Linux output has seventeen failing groups, thirty differing and three missing
  files. Newly failing groups: `jumps-fd`, `nits`, `parameter-prior`, `reload`,
  `robust`, `unweighted`. Earlier `ell1h-h4` differences disappear. No extra
  files, changed inputs or unexpected statuses are accepted.

Disappearing or fewer failures are not acceptance. Shared and newly failed
field identities remain in the ledger.

## Verification and remaining work

Focused receipts are under `validation/release/build/linux-followthrough-*`:
four C11/C++17 static/shared profiles, provider probe, plain/ASan/TSan and separate
cache diagnostic, event/Lomb controls, original-log accounting, harness controls
and source audit. Source audit reports 153 unchanged Rust files and exactly two
changed test modules, 93 responsibilities, 63 unchanged unsafe lexical sites and
36 identical SOFA sources. The two revised tests pass in debug and release;
all-target Clippy, formatting and eleven harness controls pass.

Central must review the exact candidate before authorized publication/rerun.
The next Linux run still owes previously blocked performance, M2 and predictor
checks; the full early corpora, current Rust history comparisons, worker campaign
and source/branch ledger from `df587cd`; and every unchanged numerical gate.
Cache/numerical diagnoses require actual Linux evidence before production
correction or acceptance. Covariance optimization stays deferred under 0099.

## Bound local evidence

The selected packet contains 1,300 files (13,239,796 bytes), all read back and
hash checked. Eleven original run directories remain in place, including both
source-audit iterations. Recipe snapshots accompany the selected evidence.

| Receipt under `validation/release/build/` | SHA-256 |
| --- | --- |
| `linux-followthrough-rust-v1/receipt.json` | `31972ad4278eaeebab3056b67f67fb7c784c244619044e0323b175b69f5ed098` |
| `linux-followthrough-validation-v1/receipt.json` | `629a986fece3739ff9f65c4df9f1cfac484da4e123042075bdf79fdf99d41cb5` |
| `linux-followthrough-source-audit-v2/receipt.json` | `b2a1b8be03fd2115ac91087799007fe58d6b78c8f0d65ac9aab13d7dab28efa8` |
| `linux-followthrough-ledger-v1/receipt.json` | `cb553b9d981b0a670c856596e5e2be5b2157cebb0dc2d8ea19929275b10ba5ba` |
| `linux-followthrough-events-v1/receipt.json` | `039de0c30163296606cfce68c670e7e4a41439e61ec60c4ef21530d9d9a59d6a` |
| `linux-followthrough-lomb-v1/receipt.json` | `2f6cf6e96988b85dd017c9976e1a88b6e600e3f28122f174f99ec1737fc4825d` |
| `linux-followthrough-cache-v1/receipt.json` | `c745c7010a8428f0fae3bd02c531fab60a1959375c6e5f90a44390d8e2368e21` |
| `linux-followthrough-predictor-v1/receipt.json` | `7bfd94eed8e5e62d5f96a97c728403562d3dfeebccc060b38b2ba17f7223e8ff` |
| `linux-followthrough-evidence-v1/manifest.json` | `378a1bf361dd619060606c742ef6f63c5e148a398f900125145c59c6c29549ce` |
