# Essential A2 authored construction

These six files are byte-preserving Git-blob exports from the independent local
A2 repository at `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`. `provenance.tsv`
records each blob and SHA256. The independent repository has no public remote;
these authored inputs make a primary clean checkout sufficient to reconstruct
the pinned reference. No generated oracle, JSON, archive or binary is included.

`../bootstrap.py` replaces the platform-specific construction in A2's build.py.
It preserves the initializer correction and records resource instrumentation
separately. Original A2 remains unchanged. The exporter is an observer, not the
production engine. Full legacy build/run reports retain their original identity.
