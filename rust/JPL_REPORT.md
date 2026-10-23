# Assignment I: Rust JPL transfer

23 October 2026. **The required JPL and M1 checks pass; M2 has exactly its four pre-existing diagnostics.** The implementation transfers all 12 implemented JPL functions and the Sun-view macro to safe Rust, with the approved owned libc stdio boundary. Central acceptance of this bounded assignment is still required. M2 and the overall TEMPO2 transfer remain unaccepted.

## Scope, revisions and evidence identity

The governing contracts are decisions 0017 and 0019. The canonical TEMPO2 pin is `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; H's accepted reference revision is `1df4d37`; the previous Rust ownership baseline is E's `90b52bb`. The executed implementation snapshot is `3e3d48f32d1e7cdaf814886422ec70ecd55e23ec`. The final delivery adds this report and generated evidence; it does not change executable inputs.

| Owned commit | Change |
| --- | --- |
| `df791fe` | Complete H-to-Rust comparison projection, before implementation |
| `59a856a` | Owned stdio, header/constant handling, interpolation, state and pleph |
| `0b0b1d5` | Same-open-file provenance, timing integration and retained JPL C removal |
| `6f48f05` | Bounded title conversion, decision 0019 projection and fresh-output enforcement |
| `d91dfe8` | Preserve optional derivative flags using actual storage bounds |
| `3e3d48f` | Clarify acceleration-demand and caller-buffer terminology |

Only `rust/` and `validation/jpl_port/` were changed by I. Concurrent assignments, independent repositories, copied fixtures, H's evidence and historical M1/M2 evidence remain outside I's edits. The final verifier checks the immutable files before and after execution.

| Canonical input | SHA-256 |
| --- | --- |
| `jpleph.c` | `d032fc069e9020af37333038d49f078d1ddefeb2ae253f520c17f8881c98960e` |
| `jpleph.h` | `c04f5f1b0fc961868ebd3ca0c4af6d9e5f2fca045689c963865484d581e1809d` |
| `jpl_int.h` | `d595b3aa8727567539cf5f73ef0977ca487237f2f2356538e1f5586e24315f72` |
| H `outputs/reference.json` | `2efb8fe4cc91489af856bde4baaf15abc2b2ee03265238db24ff1e41eb5a60f5` |
| Reviewed title-scan source evidence | `4b65ff721fbdfa75764acba1c93090d45ccaa57c6fbc2d9f25f6233c519db4b3` |

[Executed source and harness hashes](/Users/Eric.Dong/Downloads/tempo2rust/rust/verification/jpl/source_hashes.json) identify every included Rust source, test, example, native source, build/dependency manifest, I harness/projection and H input. The per-profile manifests and comparisons also identify the executed test binaries. [Immutable hashes](/Users/Eric.Dong/Downloads/tempo2rust/rust/verification/jpl/immutable_hashes.json) cover the protected evidence. [JPL attribution](/Users/Eric.Dong/Downloads/tempo2rust/rust/jpl_source_manifest.json) retains the upstream license, authors and source identity.

## Production correspondence

| Canonical responsibility | Rust counterpart |
| --- | --- |
| `jpl_init_ephemeris` | `jpleph::Ephemeris::init`: decoded headers, constants, initialized owned cache |
| `jpl_init_error_code` | Sequential process-global `AtomicI32`, startup value -9 |
| `jpl_close_ephemeris` | `Ephemeris` destruction and exactly one `stdio::Stream::drop` |
| `jpl_get_double` | Declared selectors over decoded binary64 fields |
| `jpl_get_long` | Declared selectors with source u32-to-signed interpretation |
| `jpl_get_constant` | Caller name buffer and partial raw value bytes |
| `swap_32_bit_val` | Four-byte array reversal |
| `swap_64_bit_val` | Complete eight-byte chunk reversal |
| `dimension` | Same source selector, including the state's `i+1` convention |
| `interp` | `Interpolation::interp`, source recurrence and reverse accumulation |
| `jpl_state` | `Ephemeris::state`, source cache/I/O order and bounded destinations |
| `jpl_pleph` | `Ephemeris::pleph`, selection and requested-component arithmetic |
| `jpl_get_pvsun` macro | Optional bounded nine-component view with explicit absence |

`make_sub_ephem` is declaration-only at the pin and is excluded. The complete per-function scenario expansion is in [debug coverage](/Users/Eric.Dong/Downloads/tempo2rust/rust/verification/jpl/debug_coverage.json) and its release counterpart. [The projection](/Users/Eric.Dong/Downloads/tempo2rust/validation/jpl_port/projection.json) enumerates all 55 cases, every ordered event and every reference field; [its explanation](/Users/Eric.Dong/Downloads/tempo2rust/validation/jpl_port/PROJECTION.md) defines the representation and classifications.

`read_ephemeris.rs` retains the selected TEMPO2 call order, binary64 JD and Rug128 scaling. Planet/Sun quantities retain AU, AU/day and AU/day²; optional quantities retain their native units. No epoch conversion or replacement numerical algorithm was introduced. The source's interval loop, operation grouping, previous-record boundary selection, Sun/demand caches, target-equals-centre shortcut and Earth/Moon arithmetic remain in source order.

The old JPL C sources, headers, corrected-cache patch, build/link rules, FFI and superseded preflight/cache tests were removed. The native archive is rebuilt without stale members. SOFA's 36 pinned routines and notices remain. [Native inventory](/Users/Eric.Dong/Downloads/tempo2rust/rust/verification/jpl/native_inventory.json) records archive members and confirms that the newly executed debug/release main binaries contain no retained C JPL entry points.

## Exact host comparison and approved title rejection

The acceptance profile is macOS 26.7 arm64, Rust 1.97.1 (`aarch64-apple-darwin`), GCC/GFortran 16.2.0 and Apple Clang 21.0.0 for the supplemental sanitizer probes. Commands record the actual executables. Finite values require identical binary64 bits, including signed zero; nonfinite classes match exactly with NaN payload excluded. Statuses, ordered events, discrete fields, caller storage, cache bytes and I/O observations are exact. No epsilon or permutation was added.

Per-profile results are identical:

| Quantity | Debug | Release |
| --- | ---: | ---: |
| Original H scenarios accounted for | 55 | 55 |
| Original H reference events | 1,443 | 1,443 |
| Executed Rust events | 1,440 | 1,440 |
| Events compared exactly under the projection | 1,439 | 1,439 |
| Approved boundary events | 1 | 1 |
| Explicitly unreachable reference events | 3 | 3 |
| Finite numeric comparisons | 43,310 | 43,310 |
| Nonfinite class comparisons | 96 | 96 |
| Discrete comparisons | 17,098 | 17,098 |
| Maximum finite difference | 0 | 0 |
| Independent polynomial/derivative components | 576 | 576 |

The 576 exact dyadic polynomial checks have zero position, velocity and acceleration difference. Debug/release H and isolated raw JSON agree completely. [Debug comparisons](/Users/Eric.Dong/Downloads/tempo2rust/rust/verification/jpl/debug_comparison.json), [release comparisons](/Users/Eric.Dong/Downloads/tempo2rust/rust/verification/jpl/release_comparison.json), raw JSON and analytic results are retained alongside the logs.

H's `title_zero` is malformed: its 84-byte title contains only whitespace from byte 26 to the end, so C's `atoi` can read outside that title. Following the user's explicit “reject and document it” instruction and decision 0019, Rust returns `title_conversion`. The report does **not** treat H's successful host observation as a defined version-zero result.

The projection retains all five original events and their reference identities, complete field lists and event hashes. Startup compares normally. Open must produce the boundary after one seek and two reads, before allocation, with global initialization status zero and last fread `(204, 1, 1)`. The later control, state and close commands remain accounted for as unreachable. Rust drops the stream during the failed initialization; this cleanup is distinct from executing the reference's later close command.

The all-whitespace input hash is `e7c4123943651a0c42d7e98dafb3ce4aebbb4059d89cd7763144c6220bf205e7`. Fresh all-whitespace/final-plus/final-minus canonical probes reproduce ASan stack-buffer-overflow and Rust rejection in both profiles. The original reviewed [source evidence](/Users/Eric.Dong/Downloads/tempo2rust/rust/verification/jpl/title_scan_source_evidence.json) is unchanged; per-profile title-boundary artifacts retain compiler, source, executable and input identities and full diagnostic text. Sanitizer output supplies a boundary diagnosis, not post-failure numerical values.

Two supplemental families compare unchanged pinned C algorithms with Rust:

| Family, per profile | Scenarios | Exact events | Finite | Nonfinite | Discrete |
| --- | ---: | ---: | ---: | ---: | ---: |
| C-locale whitespace, signed versions and terminated no-digit conversion | 8 | 24 | 40 | 0 | 600 |
| Optional indices 10–13 × INT_MIN, -1, 0, 1, 2, 3, 4, INT_MAX | 32 | 160 | 6,592 | 32 | 3,200 |

The title controls include vertical tab, which Rust's `is_ascii_whitespace` omits. All six C-locale whitespace bytes are explicit. Only scans that reach the title boundary or overflow C int reject; in-bounds no-digit conversion remains zero.

Optional demand flags preserve the source unsigned assignment and signed interpolation flag. Flags greater than one request velocity; exactly three additionally requests acceleration; negative signed flags request position. Optional rows skip AU scaling, so bounded flag four behaves like two. Planetary/Sun scaling uses source-width wrapping multiplication and checks the actual destination. The supplemental C observation harness enlarges only its optional output buffer to 12 initialized slots and renames the command; the included JPL source is unchanged. [Demand comparison evidence](/Users/Eric.Dong/Downloads/tempo2rust/rust/verification/jpl/debug_demand_comparison.json) records those exact harness changes and hashes. Insufficient four-slot optional and six-slot planetary callers explicitly reject in direct Rust tests.

## Boundaries, ownership and preservation

All ten H isolated cases have separately gated dispositions in [isolated_projection.json](/Users/Eric.Dong/Downloads/tempo2rust/validation/jpl_port/isolated_projection.json):

| H case | Rust result |
| --- | --- |
| Packed AU getter; packed EMRAT getter | Correct decoded field without unaligned typed access |
| State one-past pointer; pleph one-past pointer | Select the actual valid row first; status zero |
| NaN epoch; tiny-step record conversion | `record_conversion` |
| Excessive Chebyshev count | `chebyshev_count` |
| Oversized coefficient offset | `coefficient_storage` |
| Four-slot libration destination | `output_storage` |
| Unterminated/overflowing title | `title_conversion` |

These are operation-local boundaries, not invented canonical error codes or successful scientific vectors. Natural Sun absence and initialized recurrence storage are explicit owned-state adapters. The test-only control command supplies H's experimental state separately. Failed ordinary pleph preserves status and prior mutations without exposing undefined private output; position-only EMB reads only requested components. Unvisited private body rows produce `pleph_unwritten_row`. Low-level defined nonfinite arithmetic remains visible; the existing high-level timing-vector rejection remains an adapter.

U1 is removed: runtime hashes and rewinds one opened `File`, then transfers that descriptor to stdio. The ownership test hashes a file, replaces its pathname, proves initialization consumed the old object's AU, and proves reopen sees the new AU. H scenarios additionally cover in-place writes, cached reads, reopen, short reads and recovery. The digest describes the bytes observed during a pass; it does not promise a coherent snapshot during concurrent writes.

U2/B29 is resolved through decoded AU/EMRAT selection. B03's canonical cache-before-read bug is deliberately preserved: the record number changes before seek/read, partial and old bytes remain, failure skips swapping, and same-record retry skips I/O. The obsolete corrected-cache expectation was removed. Live initializer handles with status -7/-8, positive partial constant reads, ignored constant seeks and sequential global status semantics remain covered.

The stdio ownership test exercises complete-item counts, a partial ninth byte after an eight-byte item, retained buffer suffix, EOF, rejected size products, seek/re-read and descriptor closure on both successful transfer and failed `fdopen`. Title-boundary tests close the stream and perform a subsequent valid initialization/query in the same process for whitespace, final signs, digits and overflow. Separate tests cover independent handles, drop/reopen, malformed consumed headers/coefficient fields, direct nine-slot acceleration, invalid selectors and valid calls after errors. The two descriptor-sensitive tests run alone in fresh processes; their full invocations and outputs are in the per-profile ownership JSON.

## Unsafe inventory

There are seven production unsafe sites: five necessary stdio sites and the two retained SOFA sites. JPL parsing, arithmetic, indexing and cache logic contain no unsafe code.

| Site | Necessity and enforced invariants |
| --- | --- |
| `stdio.rs` foreign declarations | Opaque libc FILE operations preserve observed buffering; pointer does not escape the module |
| `stdio.rs` fdopen | File owns its live fd until success; static NUL mode; failure closes File; success relinquishes ownership once |
| `stdio.rs` fread | Checked size product fits initialized exclusive byte storage; live exclusive stream; no retained caller buffer |
| `stdio.rs` fseek | Source-width offset/origin and source order; exclusive stream position/cache access |
| `stdio.rs` fclose | One non-cloneable owner and one Drop after all calls end |
| `get_obs_coord.rs` declarations | Retained SOFA by-reference scalar/column-major-array ABI |
| `get_obs_coord.rs` call block | Initialized aligned storage, exclusive outputs, disjoint slices and no retained pointers |

Each site has adjacent necessity and invariants in the source. There is no unsafe Send/Sync implementation or public raw-handle API. The test-only observer duplicates a descriptor for metadata only, never reads/seeks through it, and is absent from production. The process-global initialization code uses atomic storage; this preserves sequential behavior without claiming equivalence to C data races. [SOURCE_CORRESPONDENCE.md](/Users/Eric.Dong/Downloads/tempo2rust/rust/SOURCE_CORRESPONDENCE.md) and [BEHAVIOR_DIFFERENCES.md](/Users/Eric.Dong/Downloads/tempo2rust/rust/BEHAVIOR_DIFFERENCES.md) contain the complete active ledger.

## Verification commands and regressions

From `/Users/Eric.Dong/Downloads/tempo2rust`:

```sh
python3 validation/jpl_port/verify.py
```

The driver sets the existing local `rust/.cargo-home` and `rust/target/gmp-cache`, uses Cargo `--locked --offline`, and records every top-level command and return code in [commands.json](/Users/Eric.Dong/Downloads/tempo2rust/rust/verification/jpl/commands.json). It runs `run.py --profile debug`, `run.py --profile release`, Python comparator tests, the existing M1a verifier and the existing M2 verifier (which invokes M1b), redirecting outputs to `rust/verification/jpl/`. Existing numerical budgets are unchanged. The final driver returned zero after confirming the exact expected M2 exit/failure set and unchanged source/harness and immutable-evidence hashes; [delivery_checks.json](/Users/Eric.Dong/Downloads/tempo2rust/rust/verification/jpl/delivery_checks.json) records these outcomes.

The seven Python test methods include 24 output mutations, five classification mutations, one changed isolated disposition, three real subprocess execution controls and a complete positive control. They reject missing/extra coverage and components, wrong fields/order/indices/status/units/classes, altered cache timing/bytes, invented failure output, title omission or fabricated success, missing/reordered unreachable events and unapproved classifications. Execution controls reject stale output with `/usr/bin/true`, zero selected tests even when an output is written, and a claimed test pass without a fresh output. The runner removes the previous output before invocation and requires the named probe and exactly one passing test.

Debug and release Rust suites each pass 16 library tests, seven behavior tests and one reference-contract test. Three intentionally ignored library entry points are executed explicitly by the JPL harness: the per-scenario probe and the two isolated ownership tests. Formatting, Clippy with warnings denied, resource recovery, CLI/library agreement, and the existing fitting/terrestrial comparator mutations pass.

| Regression | Result and unchanged numerical contract |
| --- | --- |
| M1a | 177 scalar-stage comparisons; largest listed residual discrepancy approximately `3.76201e-25 s`, below `1e-12 s` |
| Independent F3 polynomial | Maximum phase error `2.03183e-32 turn` below `1e-27`; residual error `1.64578e-34 s` below `1e-29` |
| M1a independent 4 ns | Response error `2.490095854e-30 s`, below `1e-20 s` |
| M1b | 1,652 terrestrial scalar comparisons; maximum residual discrepancy approximately `1.13687e-13 s`; timing/vector gates remain `1e-12 s` |
| M1b 4 ns response | Maximum response error `1.4168753360457828e-13 s`, below `1e-12 s` |
| M2 | 50 comparison records / 10,666 measurements per profile; exactly four pre-existing diagnostics, no additional failures |

Detailed maxima, units and gates remain in the new M1a, M1b and M2 `numerical_errors.json` artifacts. **M2 returns exit code 1 and remains unaccepted.** Its exact failure list equals the protected historical list: each profile has the dependent-column retained flags `[True, False] != [False, True]` and raw singular-value index zero difference approximately `3.464101615`, above the approximately `3.464101615e-10` gate. No raw ordering, precision or tolerance was changed. [The exact four diagnostics](/Users/Eric.Dong/Downloads/tempo2rust/rust/verification/jpl/m2/open_gate_failures.json) are retained; the outer verifier succeeds only if this list is unchanged and every other required check passes.

## Limits and acceptance

These results establish the specified host comparison and bounded Rust ownership checks. H full-call values remain controlled host observations because the canonical C forms invalid intermediate pointers; partial-element numerical representations are likewise not portable C guarantees. The report does not turn sanitizer failures into numerical oracles.

Linux, Windows, 32-bit hosts, big-endian hardware, alternate libc implementations, exhaustive malformed inputs, real operating-system allocation exhaustion and concurrent mutation/race stress were not validated. Synthetic fixtures cover both file byte orders and DE405 supplies real-data coverage. The atomic global removes a race rather than reproducing it. The source's intentionally preserved cache bug remains a known limitation until the full transfer's later correction phase.

Full binary models/fitting, SOFA transfer, parser/fallback reconciliation, packaging and overall acceptance remain separate work. Central must review this report and evidence before accepting Assignment I; this delivery does not accept M2 or the complete transfer.
