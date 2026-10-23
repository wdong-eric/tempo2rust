# Assignment L comparison contract (written before implementation)

Release 82fc09a and decision 0023 govern this projection. Frozen J inputs,
outputs and source profile are read-only. `projection.json` enumerates all 151
case identities, exact ordered actions and every observed leaf field. No native
undefined-access or cycle fixture is executed. Eight separate Rust-only cases
exercise the operation-local boundaries; the predecessor cycle stays unrun.

Each independent experiment starts a fresh Rust process. Its ignored library
test entry drives the production modules, snapshots initialized state, and emits
begin/end events plus source-style stdout/stderr. The same process retains state
between actions and between context/root changes. There is no production reset
or per-context alternative. Probe input and output live only under L ownership
or temporary storage; frozen files are never rewritten.

Comparison first checks complete case/action/event/field membership and order.
Completed actions compare result, full clock index and sequence cache, warning
and error counters, and actual table-query order. Tables compare all sample
bits, names, newline-preserving headers, range and load state. Clock steps and
all 30 slots compare binary64 bits, destination bytes, and changed-byte masks
constructed from the eight little-endian value bytes followed by 32 destination
bytes. This is an explicit storage projection, not Rust ABI equality. Finite
binary32 weights compare their bits as well as their exactly promoted double.
Nonfinite classes compare exactly; NaN payloads have no gate. Decimal strings
are display metadata; finite bits, including signed zero, are authoritative.

The source binary128 SAT echo is input/profile metadata. Rust retains Rug's
128-bit significand and independently narrows it for the exact binary64 SAT
comparison. Native ABI sizes and native input precision are recorded separately;
clock capacity and arithmetic precisions remain exact obligations. No observed
table/clock number receives a tolerance.

Source exit cases compare every preceding end and the terminating begin, exact
diagnostic prefix and terminal-source classification. The Rust library returns
a contextual terminal error and latches the affected owner/session; the probe
stops the script and records exit status 1. It must not execute later actions.
Safe boundaries are distinct from normal source exits and normal false/zero
outcomes; their defined mutations are checked separately without native runs.

Diagnostics preserve channels, action order, message bytes, ANSI-before-strip
360-byte formatting, counters, the 30-entry stored logs, warning-key suppression
and DUP1. Source locations map explicitly to the named C counterpart and line.
Case/build/runtime paths are substituted only at the presentation boundary.
For any truncated diagnostic, compare with the original source path lengths
(same-length temporary case roots and source-location mapping), then substitute
paths. Never drop a diagnostic because it contains a path. Public library/CLI
presentation may capture these events as warnings/stderr while JSON stays whole.

Freshness gates remove prior per-case output, record command/input/executable
hashes and require the exact expected final event prefix. Manifests pin source,
frozen inputs, Rust sources, comparator, executable and toolchain identities.
Positive controls and deliberate wrong identities must exercise the same gate.
Artifact mutations cover parsing, weights/ties, route direction, fallbacks,
shifted epochs, cache lifetime, SAT index/extra slot, warnings, terminal/boundary
classes and missing/extra/reordered/stale data. Debug and release projections
must agree exactly. New outputs go in `rust/verification/clocks/`.

After direct gates, rerun all I JPL host/isolated/supplemental/ownership gates
with output redirected; preserve their comparator and expected data. Rerun M1a
and M1b unchanged numerical/identity budgets. Run M2 fully and retain its nonzero
exit and exactly four existing dependent-column raw-order diagnostics. Only
active tests contradicted by 0023 may change, with rationale in CLOCK_REPORT.
