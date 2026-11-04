# Q/Z corrections released by decision 0133

This is a successor to the immutable evidence for Linux run `41392963741`,
candidate `9991e804d0e230248e5fb20974b7e63384d4efeb`. That run remains
86 passing / 11 failing / zero blocked stages. The changes below implement
[0133](../../decisions/0133-linux-baseline-and-q-z-correction-release.md);
local macOS checks do not establish Linux execution or release acceptance.

Scoped commits: `35d0f29` clarifies the adopted zero `nlong` targets versus the
actual native +1/−1 witnesses; `6f151b4` implements Q ledger composition;
`e37ebbb` implements Z's platform arithmetic. No predictor or GUI source is
changed by these commits.

## Q: inherit and compose observed preparation edits

The preparation receipt now includes `source_adaptations`, using the existing
comparison ledger's `original` and `adapted` identities plus observed
`intermediates`. Both producers use the same composition operation. An edit
must start at the preceding recorded hash. Recopying an authored prerequisite
is also recorded, so a subsequent platform edit cannot conceal a broken chain.
Comparison staging checks every inherited original against the preparation
receipt's source identity and every inherited endpoint against the actual
copied bytes before applying its own edits.

A historical freeze may occur at any observed point in that chain. Its
expected current identity is derived from that exact recorded point; unrelated
files retain their original expected hashes. The historical manifests and
validators in the repository are unchanged. This is ordinary hash-bound
provenance composition, reusing the existing ledger and preservation helper;
no new manifest regeneration or broad hash substitution is introduced.

Old native receipts without `source_adaptations` are not silently upgraded or
treated as having no changes. The successor Linux run needs fresh native
preparation. Retain the old receipt and failed Q stderr as baseline evidence.

Focused controls use the actual lifecycle coverage source. The only Linux
gcov-path substitution reproduces `0503677b…` → `31f74085…` exactly. The controls
exercise inheritance, a subsequent staged edit, freezes before and after an
edit, repeated prerequisite copies, unrelated corruption, a false origin and
a broken endpoint. The existing staged Q expressions remain under test.

## Z: restore the measured platform operations

`readEphemeris.C:420–422`, `440–442`, `460–462` computes the same binary64 slope,
intercept and offset on both platforms. The macOS arm64 GCC16 object contracts
the latter expressions; the Linux x86_64 GCC13 object does not. The small
`telescope_offset` function keeps that arithmetic adjacent to the source and
evidence explanation, and exposes the actual production calculation to an
exact-bit regression test. It follows the existing `get_obs_coord.rs` platform
pattern. Range selection, endpoint inclusion, Rug128 conversions and subsequent
vector additions are unchanged.

For the retained c076 operands, macOS's offset is `40143333333335a4`, while
Linux's source-ordered offset is `4014333333333800`. The saved native object and
28 intermediate observations establish the Linux target. The three original
c076 vector-state differences remain in the original artifact; this patch
does not rewrite their expected results or introduce a tolerance.

## Local evidence

Evidence root:
`decisions/evidence/release-qualification/build/qz-0133-20261104/`.

- Ten Python metadata/portability controls passed.
- The existing failed-preparation publication control passed: a failed native
  receipt still publishes all unavailable comparisons without requiring a
  successful preparation ledger.
- The macOS debug and release c076 exact-bit production-helper tests passed.
- The existing release Z driver runner passed all 82 histories with an empty
  difference list, including c076, neighboring offset intervals and endpoint
  cases. Only its Cargo cache and output destination were adapted in the saved
  `run_z.py`; source/resource/case identities and the raw log are retained.
  Its binary completed compilation before the AO merge and is associated with
  the `35d0f29` source snapshot plus the recorded Q/Z overlays.
- Actual Q→S preparation passed both native records and both coverage runs.
  This uses a real macOS gcov16 executable through a distinct symlink path to
  exercise a path change after Q's freeze: Q retains `0503677b…`, and S's
  recorded path-only adaptation produces `46ae57f1…`. These runs contain all
  107 Q and 30 S native histories.
- Actual comparison staging passed native matching, the original Q controls
  including 14 compiled mutations, and all 121 release Rust Q cases with zero
  state/diagnostic differences and no source mismatches. Its candidate snapshot
  is exactly `e37ebbbd93132a3697da6600bad4a5d4bff02c4c`, including the integrated
  AO recurrence source. Nine preparation ledger entries become 41 comparison
  entries; the Q runner retains its original/preparation/comparison chain.
- Additional controls use the actual staged Q comparator. The positive
  snapshot passes; corruption of the adapted coverage source and untouched
  lifecycle inventory each fails, as does a broken composition endpoint.
  Restored files pass again. Historical repository validators remain unchanged.

`local-review.json`, `preservation-controls.json`, the original command receipts,
raw logs and the test executables are retained. `native-and-candidate.tar.gz`
and `comparison.tar.gz` preserve the complete local staging trees and candidate;
their reference-library symlinks still point to the previously bound A2 build.
`delivery.json` binds these artifacts and the updated reports. The previous
Linux delivery receipt is preserved byte-for-byte rather than rewritten with
the changed report hashes.

The original 16 Linux Gaussian discrepancies remain unresolved. No Gaussian,
U, AB, general-fit, DDH or nonfinite policy is changed or accepted here.

## Required combined Linux follow-through

Central owns the candidate freeze, remote-state check and dispatch. Use a
fresh output root and retain both the original run and all successor failures.

1. Run the existing integrated native preparation with coverage, including Q
   before S. Bind the new `source_adaptations` ledger and its source/current
   identities; require Q native matching, Q controls and all comparison cases.
   A standalone Q run without the later S adaptation is insufficient evidence.
2. Run the c076 helper test on actual Linux x86_64 in debug and release, and
   the existing `check_planetary.py --release` driver replay. Require the Linux
   exact offset above and removal of the three c076 vector differences. Keep
   `check_random.py` and all remaining Z failures visible under their original
   gates. Bind the native contraction profile and inspect the actual optimized
   Rust object; a changed macOS cfg is not a Linux qualification.
3. Include the portable correctness sequence in
   [RECURRENCE.md](../predictor_performance/RECURRENCE.md) for the integrated AO
   recurrence change: exact pair `8716d1d…` → `e24622d…`, all direct/full-C
   contracts, concurrency and installed consumers, and predictor library tests
   in debug and release. In particular, execute
   `predictor_recurrence_linux_keeps_wide_inputs` on Linux and run the installed
   `recurrence_consumer` on both endpoints, comparing its complete output. Keep
   the 64-bit significand, `2^-16445`, `2^16382` and public `&Float` borrow
   contracts; retain inherited native failures separately. The existing
   evaluator pair ending at `8716d1d` alone does not test the recurrence change.
4. Keep owner-reuse and recurrence pairs separate from the complete candidate,
   which also includes 0127, Q/Z and central's GUI qualification slice. Any
   measurements follow the existing recipe after compilation and correctness,
   within central's coordinated window. No new performance budget or science
   acceptance is inferred from these local checks.
