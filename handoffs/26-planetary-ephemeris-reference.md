# Assignment Z: TEMPO2 planetary ephemeris reference

Implement the native reference released by [decision 0060](../decisions/0060-planetary-ephemeris-reference-stage.md) in `/Users/Eric.Dong/Downloads/tempo2rust`. Read HANDOFF.md, policies 0053/0039/0050, accepted H/I JPL evidence and the full decision before editing.

Own only new `validation/planetary_ephemeris/`. You are not alone: W retains Rust until central acceptance, then Y may take that ownership. X and every other accepted reference are read-only. Central owns design, release and acceptance. Do not modify Rust, accepted runners, original native files or either nested repository; use raw pinned Git blobs in disposable builds.

Proceed in dependency order:

1. Freeze all three primary routines, fields/branches/callers, typed state, source profile and safety classifications before experiments.
2. Build unchanged readEphemeris.C with the real pinned JPL and utility/clock dependencies. Reuse H's authored synthetic generator read-only; keep new inputs and binaries in Z.
3. Build defined initialized caller histories for the full driver and random2/gasdev, including multi-pulsar/resource/call order, all vector tails, parameter flags, offsets and retained helper state. Record the original oracle once.
4. Freshly rebuild and replay the complete corpus twice, audit any observer instrumentation, run strict comparison/identity controls and actual compiled mutation witnesses, then measure coverage with explicit remaining gaps.
5. Return REPORT.md with actual source/build/input/executable identities, verification, original defects and limits. Commit only owned authored paths with Conventional Commits; generated evidence stays ignored.

Follow the source's actual decisions. Small demonstrated corrections follow 0053, with original evidence retained and separately identified corrected experiments; this reference stage does not silently replace the original behavior. Existing precision, tolerances and M2 gates are unchanged. Trust validated invariants in later production design; reference safety preflight and deliberate verifier-corruption tests do not justify redundant kernel checks.

## Dispatch prompt

Implement Assignment Z under decisions/0060-planetary-ephemeris-reference-stage.md and handoffs/26-planetary-ephemeris-reference.md. Own only validation/planetary_ephemeris/. Create the complete unchanged native reference for all three active readEphemeris.C routines with real JPL/utility dependencies, all initialized state and caller histories, fresh strict replay, observer equivalence, mutation witnesses and measured coverage. Preserve accepted H/I/V/X evidence and concurrent W/Y work. Keep generated evidence local and ignored, commit explicit owned source/document paths, and return REPORT.md for central acceptance.
