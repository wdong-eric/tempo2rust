# Parameter reference review and identity correction

Central review, 23 October 2026. **K's numerical reference replay passes, but acceptance remains pending a bounded verifier correction.** This implements decision 0020's evidence-identity requirements; it changes no scientific convention, numerical representation or reference value. L retains the separate Rust ownership window.

Central rebuilt committed K delivery `6d135816a4f57aa8019b011882a7e0d58b51bad6` from raw pinned sources in temporary storage. All **306 cases / 1,545 events**, including 19 normal source exits, reproduced exactly. Stable source/config/compiler/runtime/probe identities matched, allowing only the temporary build path in the compiler command. All 24 delivered artifact hashes, 14 reused binary-fixture hashes and six supporting-evidence hashes matched. Coverage and observations list the same 306 case identities. The inventory/controlled-state distinctions and selected initializer/parser source were also inspected. Central did not execute the isolated FB20 diagnostic or other undefined cases.

The reproducible defect is in `validation/parameters/run.py`'s `--no-build` branch: it reads the saved `profile.json` as the executed profile, then compares that saved object with itself. It never verifies the executable that it actually launches. In a temporary copy, central replaced `build/probe` with a harmless shell wrapper forwarding every argument to the freshly rebuilt native probe. The numerical records still matched, but the runner returned zero and wrote a full 306-case success receipt with the old executable hash.

| Identity | SHA-256 |
| --- | --- |
| Actually launched forwarding executable | `1b39795f5d3009a3529cd0bdfe05400a65f04403dfa81850b974523a261b9b1a` |
| Native executable invoked by that wrapper | `00e59f267c51659ff115997b933e4e5864f34edce5b15f45507b46d53db56400` |
| Executable falsely attributed by the receipt | `ddbe600183cca59ccbddbe99f08229572eb16b77c56b4fe160341a3974df6fa2` |

[Central replay and defect evidence](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/parameters/central-review.json) records the native build profile and both outcomes. [The reproducer](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/parameters/central-review.py) copies the historical committed K files into temporary storage and writes only temporary build/results. Its erroneous success receipt is diagnostic evidence of the defect, not accepted provenance. The real K artifacts and both independent repositories were untouched.

## Correction brief

K remains owner only of `validation/parameters/` for this correction. Remove the unused `--no-build` shortcut and retain one canonical fresh-build verification path. No other checked K script consumes that option. Record the actual executable and current runner/harness identities for the check; verify that the executable used matches the freshly produced build identity. A frozen reference profile must not stand in for a new execution identity.

Add a focused negative control showing that the old shortcut cannot publish a fresh successful receipt, plus positive and deliberately wrong stable-identity controls through the same check used by normal verification. Keep every source, case, observation, numerical value, status and comparison budget unchanged. Rerun the complete ordinary regeneration and existing comparator/source-mutation verification. Reuse measured coverage if its native inputs are unchanged; if they change, regenerate the affected evidence rather than relabeling it.

Return the corrected report, executed identities, verification receipt and coherent commit IDs. Explain any metadata-only artifact changes. Central will review the correction before accepting K or releasing a Rust parameter/PAR assignment. This is a reference verification finding; it does not alter M2's four unresolved exact-order diagnostics.
