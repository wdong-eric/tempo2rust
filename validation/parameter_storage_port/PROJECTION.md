# O parameter storage projection (written before implementation)

Baseline central release: 73fc537. Canonical TEMPO2:
fda6ad4a9753ad7dc5d291406f2f310a9161423f. K: 38d62af (ccdf55c identity
correction); M: a01d428/e754581. Accepted reference files are read-only.

## Source / field contract

`tempo2.h:156,170–209,382–396` maps signed `param_label`, every `label`
enumerator (including LAST/ZERO/JUMP/FDJUMP), and the array-valued `parameter`
into `tempo2.rs`. `registry_projection.json` enumerates every source slot and
cell extent, independently of PAR spellings. All 2,000 slots / 9,616 cells
exist. `param_LAST` does not size the allocation. Source indices and i32 flags,
counts and logical `aSize` are preserved. Backing extent is independent.

`initialise.C:777–846` maps allocation to owned initialized cells: val/err,
fitFlag/paramSet, nLinkTo/nLinkFrom and all 100 label bytes start at zero.
prefit/prefitErr and all five entries of each link array are explicitly unwritten.
`initialise.C:248–767` maps parameter label assignments in source order, including
loops, overwritten labels, trailing spaces and still-blank cells. `destroyMemory`
is replaced by normal ownership destruction of the complete backing allocation,
including tails hidden by a smaller logical extent. Reallocation replaces owned
storage without emulating leaked pointers. Other initializer controls, observations,
covariance, constraints and pulsar destruction are outside this projection.

`tempo2Util.C:340–355` maps copyParam into `tempo2_util.rs::copy_param`:
for each i < signed source aSize, label then shortlabel (through NUL, retaining
later bytes), val, err, fitFlag, paramSet, prefit, prefitErr; destination aSize
last. Links/counts, backing allocation and uncopied cells remain unchanged.
Zero and negative counts perform no cell access. Entire-object Clone is a separate
ownership operation. Pointer/layout bytes are excluded; retention is observed.

At the first missing cell/string byte, unterminated source string, unwritten
numeric read or inadequate destination buffer, return a contextual named boundary.
Earlier completed field writes remain observable only in diagnostics. The affected
destination becomes unavailable for scientific reads or subsequent copy. Safe
exclusive ownership prevents overlapping buffers; no native invalid probe runs.
All numbers are Rug128; explicit controlled nonfinite values and signed zero clone
without going through the public finite Scalar constructor.

## Ordered reference cases

`case_projection.json` contains original ordered actions and projected event keys.
All begin/end events and all selected actions are retained, even when the action
has no parameter effect. K natural state excludes caller-seeded prefit values;
K init harness explicitly seeds 17/19 only after initialization. These are caller
inputs in tests, never defaults. K changes are filtered to parameter fields only.

| Cases | Compared scope | Excluded scope |
| --- | --- | --- |
| K registry | init, all 2,000-slot natural registry, destroy; full registry.json enumeration | initializer pulsar controls and ABI sizes |
| K init_full / init_all | ordered init/destroy, one/two independent stores, parameter changes; declared caller 17/19 seed | fullSetup allocation, observations/covariance, log counters |
| K allocate_0 / allocate_1 | complete natural parameter state for both ignored realloc values | native allocation addresses |
| K copy_parameter | init/seed, prior values/flags, copy and exact parameter change map | unrelated controls |
| M copyparam_0/1/3/4 | full source before/after and destination before/after; four backing cells, all label bytes, all link entries/counts, signed flags and aSize; ownership retained | native pointer values and MPFR/C object layout |
| O supplemental | unchanged pinned copyParam with negative count and shorter strings; full initialized destination suffix | undefined native inputs |
| O Rust boundaries | missing source/destination cells, missing NUL, short destination labels, unwritten prefit/prefitErr, retained prefix and blocked reentry | native invocation of invalid access |

Numbers compare exactly as mathematical binary values, including zero sign and
nonfinite class; M binary128 little-endian bytes decode to values, not Rust object
bytes. K selected values are exact zeros/integers/dyadic fractions. No tolerance.
Missing/reordered case or field membership, enum/extent changes, altered labels,
flags/numbers, lost tails/links, boundary classes and stale executable/runner
identity must reject. A fresh challenge and fresh output are required; delete
old success receipts before any full attempt, and verify identities around launches.

## Integration and completion

The strict frontend writes source val/fitFlag/paramSet. Source err stays zero until
its fit callback assigns it; prefit fields remain unwritten. Supplied uncertainty
and original lines live in a distinct metadata map. Evaluation and fit callbacks
read/write source cells, with no scalar map or boundary synchronization.
The five historical always-present keys remain F0/F1/F2/F3/PEPOCH. Optional keys
appear only when supplied. parameters() returns a fresh owned compatibility map;
changing it cannot change the input. JSON fields, numbers and membership stay fixed.
This bridge is not full readValue or PAR/TIM transfer.

Required gates: direct debug/release exact comparisons and controls; fmt, Clippy,
Rust tests; redirected accepted clock/glob/provenance, JPL and M1a/M1b checks; M2
complete with its nonzero exit and exactly four historical raw-order diagnostics.
Source/executable/compiler/runner/input identities and protected-byte checks accompany
receipts. Central alone accepts this stage; M2/full transfer remain open.

## Contract discrepancy for central review

0033 says param_iperharm remains blank. Pinned initialise.C:422–423 and K registry
entry 89 both assign IPERHARM. Lines 314–317 separately overwrite param_daop (88)
with IPERHARM. Recommendation: preserve BOTH source labels and correct the sentence
in 0033. No accepted reference is changed. The user explicitly approved preserving both source labels in O's chat. This
resolves the implementation choice; the shared decision text remains central-owned.
