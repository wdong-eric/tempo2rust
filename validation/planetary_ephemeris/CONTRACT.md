# Assignment Z frozen reference contract

Source pin `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; release
`7946aeee6e7e105fc276b311f1c36a4288767d0d`. Z owns only this directory.
No central acceptance or Rust scientific tolerance is implied.

## Source and build

`inventory.py` freezes all three complete raw bodies (readEphemeris:56–494,
random2:498–548, gasdev:550–573), active control-flow sites, preprocessor lines,
all repository occurrences/callers/declarations, full accepted S type map,
read/write fields, cases and ordinary profile **before experiments**.
The original file is compiled as a separate TU with GCC 16 O2, binary128
TKlongdouble and the accepted A2 config. Real JPL C, clock, table, observatory,
utility, longdouble and logging dependencies are compiled from pinned bytes.
Global data/CVS are exact slices; excluded orchestration bodies are not linked.
The actual driver object must reference the public `_fortran_mod` symbol;
the private double overload must remain invisible. No Rust is linked.
All resolved headers, macros, compiler/backend/linker/runtime libraries,
commands, source/object/executable/input bytes and launch identities are saved.

## Caller/state and interpretation

Two value-initialized pulsars own twelve value-initialized observations (six
each). Explicit sentinels initialize every six-vector and all three 9×6 planet
arrays; the native initializer is not credited with these writes. All 349
pulsar and 95 observation fields are initialized typed values; unused pointers
are null. Five parameter backing pairs (phase/mass plus three ignored offset
mode parameters) have nine initialized value/flag cells per pulsar. Their
ownership and every cell are retained. No padding/raw addresses are exported.
Full 500-cell time/value offset backing, 30 clock steps, all 40 flag slots,
known string bytes and vector tails are compared; other typed fields are
checked recursively against their pre-call values, including pointer identity.

The signed loops support negative/zero counts; positive counts stay in actual
storage. A processed pulsar opens/closes its resource even with no observations.
Missing/short files produce the source exit, with completed prefix retained.
Noise is warning-only. CVS/debug histories use real dependencies.
SAT is parsed directly as binary128; getCorrectionTT sums binary64 corrections
in order, divides by SECDAY before its addition to SAT; Teph is binary128.
The final JD and JD+binary64(1e-6) are converted at the JPL binary64 interface.
All dates and next-day differences remain in the resource's valid interval.

Selection requires phase or mass flag exactly one; mass adjustment later uses
any nonzero mass flag. Phase values have no active Earth adjustment. All nine
six-component differences are unconditional, including retained/unselected
arrays, and are not normalized derivatives. Scaling preserves AU conversion,
SI_UNITS-only mixed IFTE_K multiplication and position/velocity order.
Most ecliptic conversions rotate only the position prefix. Earth mass correction
uses the already processed planet vectors before Earth's position and velocity
rotation. These source frame asymmetries remain unchanged.

Offset axes are independent: signed count, first-match left-inclusive/right-
exclusive intervals, zero outside/default, count one, unsorted overlap, retained
backing and position-only addition. Mode parameters have no effect here.
STL_BAT is exact and case sensitive, occurs after ephemeris/offsets, scans flags
in order, retains values on failed scans and zeros velocity. All other spellings
follow the ordinary branch. Defined scanned NaN/infinities have explicit classes.

H's authored `synthetic` generator is imported read-only with bytecode disabled;
new binaries remain in Z. It supplies discontinuous dyadic polynomial inputs,
both byte orders, shifted resource and absent optional quantities. Real DE405
is read at its actual pinned path/hash. Ordinary full JPL calls remain **host
observations**: H documented uninitialized initializer cache fields, misaligned
public getters and out-of-bounds pointer formation. Z does not repair/control
those internals or pretend sanitizer-clean C semantics. Absent nutations give
a defined -3 status and six zero outputs; later offsets/STL processing is
witnessed. Planetary range/read/seek failures continue into uninitialized local
pv and are native-invalid, not successful numeric oracles.

## Helpers and separate observer

random2 has 100 binary128 cells (97 active), binary128 result, int iff/ix states,
and two caller-owned native long seeds shared across helper calls. rm1/rm2 are
constructed in binary64 then stored as binary128; public remainder semantics
and native integer arithmetic are retained. Seeds are bounded to avoid overflow
and int conversion faults. Negative seeds reseed. Zero reseeds only on first
use through iff==0; subsequent zero continues. gasdev caches binary64 gset
under iset, including returning a cached Gaussian without touching a negative
seed. Deterministic histories test sharing/interleaving/rejection, not quality.

A separately identified observer registers pointers to initialized function
statics at entry and only reads them after actions/defined exits. It never
resets/writes state or calls either random routine. Separate forwarding JPL
observers record actual target/centre/date/status/order and open/close handles;
they never substitute calculations, outputs or initialization. Ordinary source
remains byte-identical. Every ordinary projected field and raw diagnostic is
audited across the entire observer corpus; added observation fields are the
only excluded projection. Failed ordinary calls have pre-call records; the
observer also records exit state and completed calls.

## Exact evidence gates

Cases, ordered actions and expected status/membership are authored in fixtures.py
and frozen before execution, with unique case-folded IDs and bounded preflight.
Recording is explicit and one-shot. Default verification rebuilds and executes
every case twice from fresh processes, verifies resources after writing and
actual binaries/dependencies before/after launch, and compares recursive types,
ordered object/list membership, finite bits (including signed zero), nonfinite
classes, initializedness/ownership, statuses and exact diagnostics. Raw streams
and every receipt remain local. Resource paths are stable; only declared build
roots may be normalized and equal path widths are required for observers.

Independent synthetic polynomial/target, units/rotation/prefix, offset and random
transition checks are diagnostics against separately calculated expectations.
They do not define a Rust tolerance. Actual compiled mutations must change
numerical/state/diagnostic fields beyond their source hash. Comparator/identity
controls use nested real records and fresh execution challenges. Coverage runs
all cases and audits any differences from ordinary O2, records actual function,
line and branch counts, and classifies every unhit executable site/edge.
Undefined, host-qualified, fault/unrun and defined-unsampled gaps remain distinct.

Accepted preservation membership excludes evolving central Markdown, all live
Rust and W/Y work. Rust context uses the fixed release Git revision. Both nested
repositories and accepted reference evidence stay unchanged. Generated JSON,
binaries, raw streams and coverage are ignored. Only owned authored sources and
concise reports are committed. Reference preflight/corruption controls do not
justify redundant future production guards.
