# Retained numerical sources

All upstream files came from TEMPO2 commit
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
`upstream_manifest.json` records their original SHA-256 hashes.

- `sofa/`: 36 bundled Fortran routines, the transitive closure of
  `iau_c2t00b`, `iau_pom00`, `iau_rxp`, `iau_sxp`, `iau_pxp`, `iau_trxpv`
  and `iau_trxp`. Sources and license text are unmodified. This is the bundled
  historical implementation, not a substitution with a current SOFA or ERFA.
  `sofa/copyr.lis`, `sofa/intro.lis` and each routine retain the original notices.
  The bundled license restricts commercial use; release packaging is not claimed.
- `jpleph.c`, `jpleph.h`, `jpl_int.h`: low-level JPL reader/interpolator,
  Project Pluto copyright 2011, GPL-2.0-or-later, with the source's 2014 layout
  notes. Header files are unmodified. `departures.patch` contains the two
  memory/recovery changes in the C implementation, with explanatory comments:
  initialize otherwise unread header slots; invalidate a failed read's cache
  and mark a new record cached only after successful I/O.

`build.rs` invokes GCC/GFortran (`gcc-16`/`gfortran-16` by default; `CC`/`FC`
overrides) with `-O2`, builds a static archive and links it into the Rust crate.
SOFA uses `-fno-second-underscore -frecursive`; the latter gives independent
automatic locals to concurrent calls. No additional native timing core, IF99
reader, C++ wrapper, or old TEMPO2 executable is linked. These numerical routines
do not require a GFortran runtime link on the tested macOS arm64 toolchain.

Rust owns the JPL handle and checks all native statuses. DE405 structural
preflight protects the native fixed-size polynomial buffers and title parsing.
The native global initialization error slot is protected by a Rust mutex.
SOFA's Fortran arrays remain opaque column-major arrays at the small FFI boundary.
No C `long double` or binary128 values cross this interface.

`resource_tests::native_read_failure_recovers_without_stale_cached_record`
exercises range errors, repeated short reads, recovery to an earlier valid
record, independent contexts and invalid headers. IF99 and all astronomical
timing orchestration live in Rust; their binary64/extended boundaries follow
the pinned source.
