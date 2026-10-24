# Planetary ephemeris native reference (Assignment Z)

Run from the project root with Python 3.11 and the pinned installed GCC 16 tools:

```sh
python3 validation/planetary_ephemeris/run.py
python3 validation/planetary_ephemeris/observer_audit.py
python3 validation/planetary_ephemeris/independent.py
python3 validation/planetary_ephemeris/controls.py
python3 validation/planetary_ephemeris/coverage.py
```

The ordinary verifier builds the raw source twice and runs every case in fresh
processes for each build. It never replaces its oracle. Observer evidence is
separate and fully audited against ordinary outputs. Independent diagnostics,
real compiled mutations and measured coverage have their own receipts.
`REPORT.md` supplies the actual results and identities; `CONTRACT.md`,
`SOURCE_MAP.md` and `GAPS.md` define interpretation and limits.

For a new evidence checkout, first restore the recorded accepted dependencies
and preservation inputs specified by the report. Then explicitly establish a
new local freeze/oracle (existing freezes/oracles are never overwritten):

```sh
python3 validation/planetary_ephemeris/inventory.py
python3 validation/planetary_ephemeris/run.py --record
```

This is not a bootstrap for earlier accepted assignments. The A2 binary128
config, accepted S type map, X preservation manifest and H authored generator
must already exist at their recorded identities. Newly generated resources,
source copies, objects, executables, streams and JSON live only under ignored
`build/` and `outputs/`. The real DE405 file is read at its original path via a
local symlink. Do not invoke H's recording runner, edit accepted evidence, or
rebaseline a failing preservation gate. Central owns acceptance and Rust scope.
