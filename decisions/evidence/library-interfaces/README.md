# Central AJ interface review evidence

Decision 0096 records the completed handover. Generated receipts, traces,
archives, binaries and install trees remain ignored under build/. Never
force-add them or overwrite earlier campaigns.

The original low-count, Cheby-pipe and two locale scripts reproduce the
provisional findings described in handoffs/36a. Their original frozen artifact
and receipts are in build/review-20261027. `central-t1-pipe-read.py` is retained
only as the exact historical diagnostic: its defined-success interpretation
was wrong because the source consumes its marker and then uses unchecked
fixed-field conversions. Do not rerun or use it as an acceptance oracle.
The final supplied boundary.c tests the defined replacement histories.

The three later scripts are the exact executed central session recipes:

1. `central-install.py` archives production 3e41497, verifies committed bytes,
   builds a fresh default-release private prefix and checks required source files.
2. `central-interface-replay.py` freezes the supplied qualification sources,
   compiles C11/C++ static/shared clients and a separate Cargo package, and runs
   the affected interface/workflow checks. Its thirteen copied files were later
   verified byte-identical to validation commit 31c7290. It borrows canonical Git
   source and accepted reference inputs for reads only.
3. `central-delivery-audit.py` independently hashes the delivered campaign and
   binds source trees, frozen observers, recorded results and counts to their
   commits. It normalizes the recorded short production revision through Git.

They use the recorded workspace and temporary pointer paths and require fresh
output directories and the existing offline Cargo cache. The install script
creates `/tmp/tempo2-aj-committed-review-current`; subsequent scripts consume it.
These describe the executed session, not a portable distribution installer.
Run the project installer and qualification scripts documented in the delivered
validation/library_interfaces/README.md for a new campaign, keeping old outputs.

The fresh review directory is
`/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-aj-committed-review-a4ra8qic`.
Selected byte-preserving copies are in build/handover-20261027. Its preservation
receipt enumerates 795 files; Cargo caches and most external Rust executables
remain at the original location. Original receipt paths were not rewritten.
The archive predates the delivery report; all production and exercised observer
bytes are separately bound to final commits. See 0096 for hashes, executed
scope, the full unresolved ledger and AK's explicit release.
