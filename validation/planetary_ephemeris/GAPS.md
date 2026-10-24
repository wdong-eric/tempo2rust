# Assignment Z boundaries and remaining gaps

The ordinary oracle is a bounded macOS arm64 GCC O2 binary128 observation.
Exact replay is an implementation/evidence check; it is neither astronomical
validation nor a portable-C correctness claim. No Rust tolerance is established.

| Classification | Boundary |
| --- | --- |
| Host-qualified native JPL | H identified uninitialized initializer cache/bookkeeping, misaligned public AU/EMRAT getters, state IPT out-of-bounds pointer formation and a temporary pleph local-pointer formation. Z uses unchanged real code. No native initializer repair, cache reset or safe replacement is silently inserted. Successful repeated outputs are explicitly host observations. |
| Native-invalid, unexecuted | Out-of-range, short-read and seek failures in ordinary planetary requests can flow through uninitialized local pv arrays inside pleph. A retained initialized caller buffer does not make that local read defined. Z excludes these paths from complete-driver numerical goldens. |
| Defined optional error | Absent nutations returns -3 with all six outputs zero. The driver ignores this status and continues into offset and STL_BAT processing. This does not establish safe behavior for the earlier planetary failure paths. |
| Defined exits | Missing and short ephemeris resources return null and cause the original exit(1), including zero-observation pulsars and a second pulsar after a completed first. Separate observer exports initialized state and completed call order at exit. Ordinary builds retain pre-call and completed-prefix evidence only. |
| Unreachable in bounded defined seed domain | random2's index error. Native states remain nonnegative and bounded, hence floor(97*ix3/243000) is 0..96. Artificial writes to static state are excluded. |
| Defined-unsampled | Exact gasdev rsq==0, arbitrary longer random sequences, all possible flags/seeds/decimal boundary values and unmeasured input combinations. Actual rejection and acceptance histories, cache reuse, reseeding and shared seed objects are measured. No statistical-quality claim. |
| Native-invalid caller inputs | Overflowing long seed operations, out-of-range int conversions, oversized loop/array/string counts, malformed JPL layout, insufficient constant buffers, invalid pointers and unterminated strings. Preflight protects the finite authored harness corpus; these are not proposed duplicate checks inside future validated kernels. |
| Fault/unrun | Memory exhaustion, actual seek/read fault injection, concurrency, file mutation during an open handle and races in process-global state. H's accepted kernel fault evidence remains separate. |
| Other host/profile | Other architectures, operating systems, compilers, binary128 implementations, system libm versions and real resources beyond pinned DE405. No sanitizer-clean claim is made. Coverage instrumentation has its own profile and full ordinary-difference audit. |
| Outside stage | CALCEPH, global orchestration, observer/pulsar-vector routines, propagation/dispersion, secular motion and reference-phase lifecycle. Their actual interfaces/callers are mapped; no execution or transfer acceptance is claimed. M2 remains unchanged. |

All primary executable lines and branch edges measured by gcov, including every
unhit site and its individual disposition, are retained in `outputs/coverage.json`.
The report records actual totals; this text does not convert coverage into proof
of every scientifically meaningful path.

No source bug was corrected in Z. Warning-only noise, phase values' inactive
Earth correction, non-one mass flags acting on retained planet arrays, and
asymmetric coordinate rotations are original behavior, not silently repaired
contracts. The unit and frame conventions require a separate central decision
before any scientific reinterpretation. Policy 0053 remains available for
subsequently demonstrated, bounded bugs with explicit changed-result evidence.

The selected first-use seed interval is a corpus boundary, not a claim that every
larger positive seed overflows. The index-error reachability statement is limited
to the frozen nonnegative congruential-state domain; other arithmetic-defined
first-use seeds remain unsampled. No static-state corruption is used.
