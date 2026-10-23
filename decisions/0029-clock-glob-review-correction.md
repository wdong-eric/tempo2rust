# Clock delivery review: native glob fidelity correction

Central review, 23 October 2026. **Do not accept L's delivery `189fed3` yet. Return the glob behavior finding for a bounded correction.** The delivered direct clock gates and JPL/M1/M2 outcomes remain evidence; this additional check exposes a defined source behavior outside those frozen cases. Decisions 0023 and 0027, all budgets and M2's four unresolved diagnostics remain unchanged.

## Independently reproduced finding

Central compiled the verbatim `clkcorr.C::globClockFiles` function from the pinned Git blob with GCC 16.2.0, `-O2 -std=gnu++11`, and compared it with the unchanged delivered `clock_glob.rs` and `clock_text.rs` modules in temporary storage. The raw function calls host `glob(...,0,...)` and then `std::sort` on full strings. Only bounded ordinary temporary directories/files were used. No canonical or reference source was edited, and no withheld native experiment was executed.

**Eight of eighteen patterns differ.** [Raw results and identities](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/clocks/glob-review.json) and the [pinned reproducer](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/clocks/glob-review.py) contain the complete paths and positive cases. The reproducer defaults to the reviewed delivery; an explicit Git revision argument permits comparison with a corrected implementation without rewriting historical evidence.

| Input directory pattern suffix | Native result and Rust difference |
| --- | --- |
| `*` | Native full-string order places `a b/a.clk` before `a/a.clk`; Rust `PathBuf::sort` places `a/a.clk` first because it compares path components. |
| `[^x]` | The pinned host matches the literal `^` and `x` directories. Rust interprets `^` as negation and admits a different set. |
| `[[:alpha:]]`, `[[=a=]]`, `[[.a.]]` | The pinned host yields no matches for these fixtures; Rust adds character-class/equivalence/collating matches. Generic glob expectations cannot replace this source's actual host behavior. |
| `.*` | The native result includes the explicit `./a.clk` entry; Rust's read_dir traversal omits `.` (and does not enumerate `..`). |
| `a/`, `a//` | Native strings preserve the extra separators before `a.clk`; Rust collapses them. Those bytes feed source diagnostics, truncation and stored paths. |

File membership and insertion order can alter which clock edges exist and which equal-cost route wins. These are scientific-input differences, not merely alternate path presentation. Path spelling also matters to the explicit diagnostic projection. The same helper is used by the selected observatory lookup. A broad statement that glob conformance is incomplete does not satisfy this already released source behavior.

## Required correction and verification

L retains ownership for this correction. Correct the safe Rust glob path so it preserves the pinned source's actual membership, full-string byte order and returned path spelling, including the cases above. Keep the existing source fallback/state/error behavior. Do not repair TEMPO2, substitute a stable route tie rule, relax comparisons or rewrite J. Preserve safe Rust where possible; if a new native boundary or dependency is proposed, return its concrete need and ownership/invariants for central review before adding it. This brief does not grant a new unsafe boundary.

Add supplemental reference/port cases in L's owned directory, retaining J's frozen evidence. Include all eighteen helper comparisons and at least one integrated clock-index/route case with equal-cost files under wildcard directories where full-string ordering differs from PathBuf ordering. The native side may use the accepted J executable with safe new temporary inputs or an exact raw source harness; record actual executable, harness and source identities. Also cover selected observatory file ordering where this helper is used. Preserve intermediate path bytes so equality does not disappear through normalization.

Use the normal comparator and deliberate membership/order/path mutations to show these checks reject the original behavior. Rerun complete debug/release direct and supplemental gates, existing Rust/format/Clippy checks, redirected JPL and M1/M2 verification against the final source. Require exactly the same four open M2 diagnostics and unchanged protected artifacts. Update source correspondence, behavior/projection notes and `rust/CLOCK_REPORT.md`, commit coherent owned stages, and return the corrected delivery for central review. No other Rust stage is released while this correction remains open.
