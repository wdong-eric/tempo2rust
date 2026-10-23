# Retained SOFA sources

All files come from TEMPO2 `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
`upstream_manifest.json` records the unchanged hashes of 36 Fortran routines and
their notices: the transitive closure of `iau_c2t00b`, `iau_pom00`, `iau_rxp`,
`iau_sxp`, `iau_pxp`, `iau_trxpv` and `iau_trxp`. This is the bundled historical
SOFA implementation. `sofa/copyr.lis`, `sofa/intro.lis` and routine notices remain.
The bundled license restricts commercial use; release packaging is separate work.

`build.rs` uses `gfortran-16` (`FC` override), `-O2`,
`-fno-second-underscore -frecursive`, and a freshly created static archive.
Automatic locals prevent shared mutable Fortran locals. No GFortran runtime
link is needed by this closure on the tested macOS arm64 toolchain.

Assignment I removes the retained JPL C/header files, departures patch, JPL
compiler invocation and FFI. `../jpl_source_manifest.json` preserves their
canonical pin/hashes and attribution; `../src/jpleph.rs` contains the translation.
`../src/stdio.rs` retains only owned libc fdopen/fread/fseek/fclose semantics.
No JPL C algorithm, packed structure, IF99 C reader, C++ wrapper, or TEMPO2
executable is linked. SOFA's two unsafe sites remain in `get_obs_coord.rs`.
Historical M1 verification retains its original source records unchanged.
