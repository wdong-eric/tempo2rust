# Predictor transport review and next CPU stage

Central checkpoint, 2 November 2026. Accept AO's measured **macOS arm64**
representation improvement for integration. Production `0bd8e2cf2ef54e9da6feace122acd8aa96aa3fbe`,
validation `b618519dd38a6c0b689d8993a445a9b0db885c26` and report
`f0772922559b33ce71f9ae1b383f80a0adeb0ecc` are integrated with their original
history through `a44bda2defc2616c3e05c34ff0e2ecc7401918e7`.
Actual Linux x86_64 qualification remains due. This does not accept all predictor
science, AL, general fitting or full migration.

## Evidence and implementation

The measured installed C phase/frequency loop previously spent about 65% of
sampled stacks in scalar reads. The new private transport uses exact unsigned
significand/exponent/class copies and safe Rug value access. It removes text
formatting/parsing and locale creation from this boundary. Public native scalar
storage is reread on every call; no pointer cache or alternative evaluator is
introduced. The canonical scientific expressions, operation/rounding order,
native 53/64-bit profiles and Rug128 timing precision are unchanged. Existing
event-libm helpers and public text/locale behavior remain unchanged.

Central reviewed all three production files, including exact finite/subnormal
reconstruction, class/sign handling, checked public null boundaries and synchronous
pointer borrows. Existing qualified profile assertions bound the significand to
u64. Source-counterpart and safety comments sit beside the changed operations.
The pinned Rug facility assessment avoids an extra MPFR ABI/build boundary.

Central independently verified all 3188 delivered files / 274591921 bytes and
1200 frozen source bindings. Exactly three crate files differ. It replayed eight
C/C++ static/shared native value/storage profiles and 24 corruption controls,
compared all 30258 per-call/state lines and 38 legacy files, checked same-pointer
coefficient mutation/restoration, and recomputed all 63 resource records and
the sampled profiles. The original sanitizer/cache and two installed Rust
consumer outputs are bound and checked. A fresh release execution of the
975-case scalar transport regression passes. No unrelated full timing campaign
was repeated.

Seven interleaved fresh-process samples for 100000 phase plus 100000 frequency
calls give loaded medians of **4.668811 s before, 1.732665 s after**, or **2.6946x**
speedup / **62.9%** less time. Native is 0.004530 s. Short getPeriod startup has
no established improvement. External allocation requests per pair decrease from
985 to 639; requested bytes decrease from 145663 to 14864. These are measured
macOS workload results, not Linux or universal performance guarantees. Allocation
counts, sampled fractions, loaded time and whole-process RSS remain distinct.

The complete original supplemental native comparison remains available: 1429
finite differences fall within the report's conservative AF floor; one negative-NaN
T1 frequency sign mismatch is unchanged before/after. The unchanged installed
qualifier passes. The new witness does not waive a class/sign gate or close
inherited polyco log10RMS failures. AL owns their shared compatibility ledger.

Review receipt SHA-256:
`1e7ceed949b4a52624860c17373889e0aadeb26be6f9ca5d4b06b0b87d41cb81`.
The executed central recipe is `/private/tmp/tempo2-central-ao-review-20261102/review.py`;
its initial filename-only preparation failure is retained. Central evidence is
copied to ignored `decisions/evidence/release-qualification/build/ao-central-20261102/`.
All delivered artifacts are independently copied and hash-checked under ignored
`validation/predictor_performance/build/macos-reviewed-20261102/`, preserving
original paths and identities in receipts. Generated artifacts remain out of Git.

## Integration and next work

AL owns one compatibility/qualification stream and prepares the full Linux
candidate with both AN and AO. Use the exact original frozen pairs separately
from the integrated candidate. Keep prepared caches, isolated targets, complete
outputs, native failures and actual platform evidence. The existing AL publication
approval remains scoped to a centrally reviewed candidate; this decision itself
records no push or workflow dispatch.

The review of `d46c658` identified a lifecycle acceptance-routing correction:
AN's raw original S mismatch must remain visible, but already approved initializer
and reserved-slot corrections use the complete separately corrected-native
contract. Reuse `validation/release/lifecycle_correction.py` and the existing
0068/0079 authority/proof; never erase fields or grant a new numerical exemption.
Missing/failed corrected proof blocks its dependent comparison, and unrelated
fitting/predictor failures remain failures. AL is correcting this before dispatch.

The remaining CPU gap is substantial: native is about 382x faster in this
workload, and canonical Cheby evaluation now appears in about 66% of sampled
stacks. The fraction is inclusive, not an exclusive cost allocation. The next
measured target is allocation and ownership reuse in the canonical numeric
operations/evaluator. Release [handoff 42](../handoffs/42-predictor-evaluator-allocation.md)
to the existing AO chat in its isolated clone. It must establish the actual
allocation cause, preserve arithmetic and precision exactly, and demonstrate
before/after benefit; no second evaluator, pointer cache or native scientific
kernel is authorized. AL's frozen Linux candidate proceeds independently.

This checkpoint keeps the project on the user's requested major-progress track:
functional breadth exists, while scientific fidelity, platform qualification and
performance still prevent full migration. No new scientific-policy choice is
needed for this bounded optimization. M2 remains accepted only within 0122's scope.
