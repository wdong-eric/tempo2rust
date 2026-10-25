# Telescope-table bounds in the specialized PAR reference

Central decision, 25 October 2026. AC's Rust parser comparison exposed out-of-bounds accesses in seven historical Q fixtures. This decision narrows the applicability of those observations under 0043; it preserves the original reference files and recorded executions. It does not accept AC or change its ownership, numerical gates or implementation scope.

## Source and recorded evidence

At pinned TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, `tempo2.h:76-79` declares `MAX_TEL_CLK_OFFS`, `MAX_TEL_DX`, `MAX_TEL_DY` and `MAX_TEL_DZ` as 500. The clock arrays at lines 731-732 and telescope position/velocity arrays at lines 746-764 therefore hold 500 binary64 values each. Q's recorded initial state independently gives 4,000 bytes for each corresponding array.

`readParfile.C:1563-1637` obtains a one-based suffix and accesses `number-1`. These branches have no upper-bound check. Entry 500 reaches the last valid element, while entries 501 and 1000 exceed the declared arrays. Some branches reject nonpositive suffixes; that does not make their upper indices valid.

The affected case IDs are:

- `samples_TEL_CLK_OFFS`
- `samples_TEL_DX`, `samples_TEL_DY`, `samples_TEL_DZ`
- `samples_TEL_VX`, `samples_TEL_VY`, `samples_TEL_VZ`

Their first four recorded dispatch keywords use entries 1, 3, 1 and 1000. The first invalid dispatch is token 3 (zero-based), not the later malformed-value scan. All seven are historically labelled `host-diagnostic-observation` and returned zero. That label and successful termination do not establish defined array access. The broader statement in Q's report that OOB writes remained unrun does not apply to these seven executions.

Central inspected the actual source, frozen input/event records and initial-state sizes. Relevant SHA-256 identities are:

| Artifact | SHA-256 |
| --- | --- |
| Raw pinned `tempo2.h` | `0313b6eaf9f9ae9d7e5cce340379ad4b7068394e5222a01f73949e3a2640de93` |
| Raw pinned `readParfile.C` | `ecdbec472eff256ab1c7d875a4ea56536c146012599684408b03fe738ac7550e` |
| Q `observations.json.gz` | `beb6122689951013cc26db58093d89d1fac51cca9cddc76faff65e29f48e5420` |
| Q `initial_state.json.gz` | `df3e00e4e3cf1e16f9e8a7747b6766f27ea1ab984cbe26cb91f6bd9a5995b52f` |

## Applicability and AC verification

Preserve Q's original artifacts, historical classifications and acceptance record. This later disposition is authoritative for their use by AC and subsequent stages. In each affected case, the invalid dispatch and all subsequent state, stream and status observations are historical evidence of an undefined execution, not numerical or retained-state oracles for Rust. A defined-looking value or unchanged neighboring field after that access is not an expectation to reproduce. Do not enlarge the Rust arrays to accommodate the faulty fixtures.

The operations before token 3 have a source-defined prefix, but the original whole execution later invokes undefined behavior. Use its earlier snapshots as supporting historical evidence, and establish the prefix independently with a bounded native run that ends before the invalid dispatch. Reuse Q's existing executable/initialization and source; route all new inputs and outputs to AC-owned staging and record their actual identities. For each of the seven families, the same small safe run can include entry 500 to establish the real valid upper boundary. This is a focused boundary check, not a new reference campaign or a replay of Q's full suite.

Rust must retain the valid source prefix and report contextual errors at invalid entries 501 and 1000, with the relevant source correspondence and boundary explanation. Check the actual attempted access rather than declaring an entire family unsupported. Existing source ordering and defined partial writes before a rejected access remain governed by AC's parser contract. Normal in-range table processing remains part of the public input capability.

Report complete case comparisons separately from these seven prefix/boundary dispositions. An expected rejection must not be counted as a full match to its historical native case. Preserve any still-applicable initialized failed-scan, token-consumption and retained-state checks by exercising them on valid indices; the invalid suffix of an old case cannot supply those expectations. The other Q cases retain their existing classifications unless separately demonstrated otherwise.

Policies 0053 and 0065 govern the scoped bounds correction and proportional verification. No scientific expression, precision, tolerance, M2 requirement or accepted-evidence preservation rule is changed. AC continues its functional implementation and returns the evidence in its final report; AD and AE remain prepared until their release conditions are met.
