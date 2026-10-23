# JPL title boundary and comparison projection

Central decision, 23 October 2026. **Apply decision 0017's title-conversion boundary to H's `title_zero` scenario.** This resolves I's concrete projection question. It does not accept the port, change any numerical tolerance, or authorize changes to H's reference. M2 remains unaccepted.

## Evidence and source meaning

At canonical pin `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, `jpleph.c:702` declares `char title[84]`; initialization reads those 84 bytes and then calls `atoi(title + 26)` at line 726. H's `run.py` constructs `title_zero` by filling bytes 26 through 83 with spaces. No stopping character remains inside that region. The source can scan outside the object before deciding that the version is zero. H's successful unsanitized output is an observation of that execution, not a defined version-zero result.

Central inspected the source, fixture construction and I's fresh instrumented evidence in `rust/verification/jpl/title_scan_source_evidence.json` (reviewed SHA-256 `4b65ff721fbdfa75764acba1c93090d45ccaa57c6fbc2d9f25f6233c519db4b3`). All-whitespace, final-plus and final-minus variants were reported as AddressSanitizer stack-buffer-overflow in initialization. The all-whitespace input SHA-256 is `e7c4123943651a0c42d7e98dafb3ce4aebbb4059d89cd7763144c6220bf205e7`. These diagnostics confirm the source boundary; addresses and complete sanitizer traces are evidence, not exact comparison fields. I must retain the reproducible source/compiler/input identities in its final evidence.

Separately, central's defined-input probe found that vertical tab (`0x0b`) is accepted by C-locale `atoi` but omitted by Rust's `is_ascii_whitespace`. With prefix `0b3433302f`, C produced version 430, record size 6600 and 825 coefficients; the earlier Rust code produced version 0, record size 6008 and 751 coefficients. I must preserve the complete C-locale whitespace set (space, tab, newline, carriage return, vertical tab, form feed) and keep ordinary, sign and in-bounds no-digit controls exact. This is a port correction, not an upstream behavior change.

## Required disposition and complete accounting

1. Keep `title_zero` in the projection and keep H's raw reference byte-for-byte unchanged (reference SHA-256 `2efb8fe4cc91489af856bde4baaf15abc2b2ee03265238db24ff1e41eb5a60f5`). Its five reference events are `startup`, `open`, `control`, `state`, `close`.
2. Compare the defined startup fields as before. At `open`, require the operation-local `title_conversion` boundary after the source's preceding reads/seek. Preserve and test defined prior state and stream cleanup. Do not substitute a version, canonical error status or initialized handle.
3. Account explicitly for reference `control`, `state` and `close` as unreachable after that boundary. Retain their identities and original fields in the projection, with the reason for non-comparison. Reject missing, extra, reordered or incorrectly classified reference events. Rust stream cleanup is an ownership obligation; it is not execution of the reference's later `close` command.
4. Apply the same bounded scan rule when whitespace, sign or digits reach the end of the title without a stopping byte, as well as the already approved integer-overflow case. Keep terminated no-digit conversion at zero. Reject only the actual invalid conversion boundary; do not impose a broad header restriction.
5. Report exact numerical/state comparisons and boundary/unreachable classifications separately. All 55 scenarios and 1,443 original events remain accounted for, but the report must not claim all 1,443 events matched executed Rust values. Compute and publish the actual comparison counts from the final projection.
6. Add comparator controls that reject deletion of `title_zero`, a substituted successful version-zero result, a wrong boundary, omitted/reordered unreachable events and an unapproved classification elsewhere. Add direct boundary tests, cleanup and a subsequent valid initialization in the same process. Keep the eight defined title controls and all other declared H comparisons at the existing exact gate.

I owns the implementation, projection and supplemental evidence under decision 0017. Re-run final verification from an unchanged source/harness snapshot, with historical M1/M2 and H evidence protected. Central reviews the resulting report before acceptance.
