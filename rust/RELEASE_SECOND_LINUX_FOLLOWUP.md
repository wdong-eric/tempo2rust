# Second Linux follow-through (decision 0104)

Work starts from central decision `4472d99`. The reviewed Linux run executed
`4accd26`: 54/66 top-level stages passed, twelve failed, none blocked. No early
native corpus executed. Historical failures and native runners remain intact.
This report records successor evidence, not milestone acceptance.

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
