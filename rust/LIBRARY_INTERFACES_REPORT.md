# Assignment AJ delivery: external Rust and predictor C interfaces

27 October 2026. Implementation is delivered for central's 0074/0082 review;
this report does not accept AJ, any numerical gate, or release AK.

Production: `3e41497d5a86e627b0ce994d4882e4b2e77d3bf7`.
Baseline: AI `20e368fe6e7d3e05565ae7f0132f4498dfa9069e`, central AJ release
`841b9442563ea34784424f5725c0ea63c104a435`; correction brief `ce5bd36`.
Validation: `31c729079da44afe7155e42707edeac20d66bd2b`.
The report commit follows; neither changes installed production code.
Canonical TEMPO2 remains `fda6ad4a9753ad7dc5d291406f2f310a9161423f` and the
independent reference remains `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`;
both nested repositories are clean.

## Delivered capability

External Cargo users can access the transferred full core/toolkit through the
canonical Rust owners, sessions, callbacks and public operations. AJ exposes
previously private resource/coordinate/time-ephemeris/stream operations and
preserves the existing integrated timing/fitting/output/predictor/analysis
routes. All 93 library translation units have public responsibility routes in
[LIBRARY_INTERFACES_SOURCE_MAP.md](LIBRARY_INTERFACES_SOURCE_MAP.md); detailed
routine/branch maps from AC through AI remain normative. Private algorithm
helpers are reached through their public operations, not duplicated.

The private installation contains the locked buildable Rust source, native
sources/data/notices, both predictor headers, `libtempo2pred_rust.a`,
`libtempo2pred_rust.dylib` and `getPeriod-rust`. The complete standalone source
predictor surface has **64 matched function/data symbols**, including all twenty
generic functions. The public header is byte-identical to the pin. C owns actual
native structs, fixed coefficient backing, allocations and FILE streams; a thin
C layer preserves source scanning/formatting and delegates scientific arithmetic
to shared AF Rust leaves. No canonical executable or runtime tree is replaced.

The [API contract](LIBRARY_INTERFACES_API.md) and source map enumerate source
symbols/types, public Rust routes, borrowed pointer/callback/stream lifetimes,
error/unsupported dispositions, necessary unsafe sites and every deliberate
departure. getPeriod retains the source binary64 atof MJD, fixed 1400 MHz,
frequency/reciprocal conversions and text format, with initialized ownership,
argument/file errors and its own missing-C1 check.

## Executed profile and results

Measured macOS 26.7 (25G229), arm64, SDK 27.0; Rust/Cargo 1.97.1 (LLVM22.1.6),
GCC/G++16.2.0 and Apple clang21.0.0. Ordinary C long double has 53 significant
bits and size/alignment 8/8. Timing remains Rug128. The enabled external Cargo
profile uses AI's measured CALCEPH4.0.5/INPOP17A and Accelerate legacy LP64;
actual Accelerate loading and static CALCEPH/data identities are recorded.
Linux implementation/provider execution remains AL work.

| Verification | Final result |
| --- | --- |
| C11/C++17 × static/shared × debug/release | All eight installed profiles pass; each compares 44 ABI-layout rows, 68 value/state/diagnostic rows, 121 storage trace rows, 48 low-count rows and 26 pipe/locale/class rows. Each storage run also matches 270 phase diagnostic lines. Assertions cover ownership, callback identity, borrowed pointers, nearest flags, copies/inserts/keep, empty extents, failure/recovery and all unsupported exits. |
| Native file exchange and installed client | Both directions and both predictor formats read/write identical same-input values and bytes; six source getPeriod filename/MJD controls per build match exactly, plus argument/missing/malformed/low-count failure and recovery controls. |
| Comparator controls | Twelve deliberate missing/order/metadata/sign/zero/NaN/infinity/magnitude/count/phase/frequency corruptions rejected per campaign; unchanged AF numeric gates. |
| External Cargo | 31 sources compile to 30 actual executables in disabled debug, disabled release and combined release; 29/29/65 invocations pass. This includes complete QR output-mask combinations, real CALCEPH unit/frame calls, toolkit/RNG, spectrum/GW, source resources, two-pulsar timing/fitting/output and recovery. Unexecuted example entry points are compilation coverage only. |
| External combined executable versus native AI oracle | **59 paired cases / 41,961 comparisons; zero failures**, using the existing unchanged AI comparator and independently built native probe. Real timing/fitting/provider workflows therefore exercise the installed source through a separately built client. |
| Serial library suites | Debug and release each **91 passed, zero failed, 36 ignored**. Ignored historical/resource tests are not passes. |
| Formatting and lint | Cargo fmt --check and offline locked all-target Clippy with -D warnings pass. Full staged whitespace checking reports retained upstream whitespace in the byte-preserved public header and source-derived C/header text; with only end-of-line/EOF whitespace disabled it passes. |
| Native memory boundary | Four installed C clients pass ASan/UBSan. Instrumentation covers C shim/client code, not Rust/MPFR/provider internals. |
| Final integrity audit | **2444 unique files checked, zero hash or installed-source membership mismatches**. Every installed source file equals its Git blob at the production commit; observer sources, output receipts, libraries, executed binaries and provider evidence retain their identities. |

The exact executed commands and dependencies are in
[validation/library_interfaces/README.md](../validation/library_interfaces/README.md).
The final native ABI campaigns are `final-abi-debug-v4` and
`final-abi-release-v3`, using `profiles-final-v1/{debug,release}-prefix`.
Final Rust campaigns are `final-rust-debug`, `final-rust-release`,
`final-rust-combined`; their installed source has no workspace crate dependency.
The install snapshots predate this delivery report and contain the production
API/source maps, not this subsequently authored report. Static nm inspects the
actual predictor.o member because Apple's LLVM21 nm cannot decode LLVM22 Rust
bitcode members; actual standalone static clients link and run successfully.

## Central corrections and preserved history

The low-count set reader now preserves successful declared counts 0/1, complete
mixed prefixes, EOF/cursor and caller-supplied coefficient backing. The standalone
getPeriod consumer separately refuses to evaluate files missing C1. Owned Rust
file-constructor minima remain 2..32; C evaluations and new-format partial writes
retain their operation-specific source requirements.

Generic FRead attempts the source trials without preemptive ftell/fseek success
requirements. Cheby pipes retain the trailing TAIL byte. Defined T1 evidence
includes direct T1 pipe reading and generic reading after its discarded trial
header; full assigned headers and both coefficients are checked. Central's
original bare-T1-pipe apparent success is explicitly withdrawn in committed
36a: Cheby fgets consumes TEMPO2:, failed rewind leaves the short body, and the
unchecked source fixed-width parser reads unwritten locals. The final probe
records the exact remaining `R` cursor without invoking those undefined
conversions. No accidental success is synthesized.

Private scalar conversion uses explicit C-locale formatting/parsing and a
complete numeric-token Rust parser. The latter bypasses the source prefix
scanner's libc locale dependency. Both fractional input/output directions now
match natively under C/German/French, preserving both process and thread locale,
and signed-zero/subnormal/infinite/NaN neighbors. Public source text formatters
and scanners keep their source locale conventions. Both required license files
and the JPL source manifest are installed and byte-verified.

Original central evidence, including the withdrawn T1 interpretation, remains
under `decisions/evidence/library-interfaces/build/review-20261027` and its
original temporary paths; the report does not relabel any provisional artifact.
Early owned campaigns retain compile/test mistakes, overstrict exploratory
all-exact comparisons later replaced by the existing AF gates, provisional
low-count/pipe/locale defects, the whole-archive nm incompatibility, C++ feature
macro conflict and external-example output-directory precondition errors.
Provisional directories named final are historical when not in the final list
above. All corrected exercised sources/binaries are bound by the final receipts.

## Evidence identities

Paths below are under `validation/library_interfaces/build/`.

| Receipt | SHA-256 |
| --- | --- |
| `final-abi-debug-v4/receipt.json` | `1064938cc69eb57ba5ccca270374ba6a40d22d269076a89e09b6c12e2fbf685c` |
| `final-abi-release-v3/receipt.json` | `42bc03874da73cdf04c6428a61f59c43d2e0c172a3a203b5d7ddbfb87a238749` |
| `final-rust-debug/receipt.json` | `05d80d3641cd8b7c6e8cc1b265a008e3e9fe3081dbf1913c88ba1980fdd12432` |
| `final-rust-release/receipt.json` | `09a3bdbc662babffa2d20b5d3a7a66cdc7cc2c9df8212095b5247272558a24c5` |
| `final-rust-combined/receipt.json` | `51f43c4a3255e8c114e21bc411eba5a13916b61e9e4987c1e1199787de0a44b2` |
| `final-installed-backend-pairs/receipt.json` | `614d9d7f6a980d10eb093007870f42d4b5c0423f8bf7d0b6971ef33c1df9aaae` |
| `final-sanitizers-v3/receipt.json` | `b3f97800b91b8374eb85ffb13d86d864c5478dce98af3fe46fc43330afc6f162` |
| `final-checks/receipt.json` | `9babb68468acb59c5ce419a5b592bc0e8978417ab242c33e708354a6ae7955cb` |
| `final-audit/audit.json` | `c121d391099aecee9cbdba32cf3c8aa252fdbf2016fa51e63f096e3c7d36cf35` |

| Installed product | Debug SHA-256 | Release SHA-256 |
| --- | --- | --- |
| `lib/libtempo2pred_rust.a` | `4e6f2723e8468ec32e38288522aa0e1cdc227e52eaf8669780cbbe877874b296` | `2ec6dd5ad6279ac5f7e13819d48a3350932df80c040c30d183424fc2a0550908` |
| `lib/libtempo2pred_rust.dylib` | `2147c2d4e82f47c10d05a6462e7643bf379e2bc9d511f87ccea62e87715dc524` | `42a364ba34da0392a5db7f37c0f7639c285b4b2dcfc65b2920d304146c30d4ff` |
| `bin/getPeriod-rust` | `bb71b6bbdcba37de787ea64ccbdceb659c2fde3739212aae9f60d3d566575e18` | `4c202ceedd1ec0e9d9134543f479452fbae46a648974a882507007719e73f325` |

## Complete inherited acceptance ledger and remaining work

M0, M1a and M1b retain their scoped acceptances; M2 remains unaccepted. Carrying
0095 forward without changing any numerical gate:

- P's 154 records including 32 SIMUL; M2's four raw SVD-order failures; 0049's four discrete boundaries and three unconstrained interpolation cases.
- AE red's six raw modes; chromatic-log's sixteen errors and one multiplier; ELL1H's nineteen whitened-data differences; single-pulsar quadrupole/GW-single/memory/cosmic-string 770/750/448/468 failures.
- DDH's 401/433 current/historical fidelity failures and two shared-solve raw-order differences; two first-pass SWGP unwritten-state boundaries; AF's seven raw polyco log10RMS failures. The versioned historical 1 ns ideal passes remain separate from fidelity acceptance under 0087.
- AH's twelve groups: `pre-list-bary-unwritten` (unproduced UT1/missing PAR); `pre-list-tempo1` and `pre-list-terrestrial` (position/velocity signed zeros); `dm-grid` (ill-conditioned RMS/PAR digits); `red-noise` (F2 digit); `ell1h-h4` (offset/RMS); `waves` and `wave-dm-scale` (unfitted errors/missing PAR); `global` (F2 digits); `cheby-odd-defaults`, `cheby-ecliptic-tdb` and `polyco-prefix` (coefficient digits). AH's absent DDH derived inputs and all-deleted span boundaries remain.
- The recorded parallel failure in three warning vectors remains AK's obligation; the other 6,716 scalars matching in that observation and serial success do not prove concurrency safety.

AJ's installed interfaces now have the bounded macOS evidence above; central
review/acceptance remains pending. AK owns subsequent concurrency/performance
work only after release. AL still requires actual Linux x86_64 provider/ABI
implementation and execution, both-platform integrated installed qualification,
broader real scientific/resource coverage and final distribution. No Linux,
allocator-exhaustion, arbitrary invalid-pointer, every locale/nonfinite branch,
full sanitizer or concurrency qualification is claimed. Production Rug128, raw
mode rules, 0090 weighted-spectrum warnings and all existing quantity gates
remain unchanged. The source-map correspondence and successful interface calls
do not close inherited scientific failures or establish full migration acceptance.
