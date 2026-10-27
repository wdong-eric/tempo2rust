# AJ external library qualification

Production `3e41497d5a86e627b0ce994d4882e4b2e77d3bf7`, on reviewed AI and
central AJ release; central correction brief `ce5bd36` is included. See
`rust/LIBRARY_INTERFACES_API.md`, `rust/LIBRARY_INTERFACES_SOURCE_MAP.md` and
`rust/LIBRARY_INTERFACES_REPORT.md`. All paths below are relative to the project
root. Generated evidence is ignored under `build/`. Scripts require fresh output
directories; preserve the delivered directories and use new names for a replay.

## Executed install and consumer recipes

```sh
python3 rust/install.py --prefix validation/library_interfaces/build/profiles-final-v1/debug-prefix --profile debug
python3 rust/install.py --prefix validation/library_interfaces/build/profiles-final-v1/release-prefix --profile release
python3 validation/library_interfaces/qualify.py --prefix validation/library_interfaces/build/profiles-final-v1/debug-prefix --out validation/library_interfaces/build/final-abi-debug-v4
python3 validation/library_interfaces/qualify.py --prefix validation/library_interfaces/build/profiles-final-v1/release-prefix --out validation/library_interfaces/build/final-abi-release-v3
python3 validation/library_interfaces/qualify_rust.py --prefix validation/library_interfaces/build/profiles-final-v1/debug-prefix --out validation/library_interfaces/build/final-rust-debug
python3 validation/library_interfaces/qualify_rust.py --prefix validation/library_interfaces/build/profiles-final-v1/release-prefix --out validation/library_interfaces/build/final-rust-release --release
TEMPO2_ACCELERATE=1 CALCEPH_DIR="$PWD/validation/native_backends/build/provider" \
  python3 validation/library_interfaces/qualify_rust.py \
  --prefix validation/library_interfaces/build/profiles-final-v1/debug-prefix \
  --out validation/library_interfaces/build/final-rust-combined --release --features calceph,lapack
python3 validation/native_backends/check.py validation/library_interfaces/build/final-installed-backend-pairs \
  --native validation/native_backends/build/native-combined-published/probe \
  --rust validation/library_interfaces/build/final-rust-combined/client/target/release/native_backends --profile combined
python3 validation/library_interfaces/sanitize.py --prefix validation/library_interfaces/build/profiles-final-v1/debug-prefix --out validation/library_interfaces/build/final-sanitizers-v3
python3 validation/library_interfaces/verify.py --out validation/library_interfaces/build/final-checks
python3 validation/library_interfaces/audit_delivery.py --out validation/library_interfaces/build/final-audit --production 3e41497d5a86e627b0ce994d4882e4b2e77d3bf7
```

`audit_delivery.py` audits this named frozen campaign; it is not a generic
parameterized campaign runner. Its final source/artifact checks are read-only.
The final audit also executes one external kernel invocation with
DYLD_PRINT_LIBRARIES to record actual Accelerate loading. The real CALCEPH
provider/data and independent native backend are AI's hash-bound dependencies;
see `validation/native_backends/README.md` for their build/provenance recipe.

`qualify.py` builds its own standalone oracle from the raw pinned Git blobs,
records the narrow native varargs/prototype/plan-initialization corrections,
and links no native timing library. It compiles C11/C++17 clients separately
against each installed static/shared library. `dladdr`, nm and otool bind actual
artifacts, symbols and dependencies. Runtime environment paths are removed.
`comparators.py` retains AF's quantity gates and rejects twelve deliberate
trace corruptions; exact ABI/state, class/sign/zero, membership and cursor
checks are not relaxed. On Apple LLVM21, nm cannot decode Rust LLVM22 bitcode
members, so the real predictor.o member is extracted and inspected; separately
linked executable clients prove the full static archive resolves and runs.

`qualify_rust.py` creates a separate Cargo package whose only tempo2-rust path
is the installed source tree. It copies 28 existing example entry points, their
support module and two AJ entry points: 31 source files, 30 binaries. Five old
fixture-bound examples are excluded explicitly in the script. Thirty binaries
compile in all three profiles; selected clients execute 29/29/65 times. Two
default-profile native_backends invocations exercise their disabled branches;
the combined profile runs the actual helpers, 32 QR output combinations and
four real CALCEPH unit/frame cases. Receipt `binaries` additionally hashes three
empty Cargo lock files; they are not counted as executables. Read-only source
hashes, external build commands and actual binary identities are recorded.

The source-direct resource consumer checks file/table/stream ownership and
recovery, swaps/interpolation, geometry/Niell kernels, parameter initialization
and nonrecursive session access. The core consumer loads two pulsars, repeats
timing, fits, produces output and recovers after failed input. Existing public
examples cover toolkit, spectrum, GW, fitting, covariance and predictor routes.
Compilation is an API/accessibility check; execution is bounded behavior
coverage, not every scientific branch or a new acceptance of inherited gates.

`storage.c` is the driver from central's 0084 prepared witness, linked to the
installed interface; it contains no copied scientific evaluator. It preserves
90 evaluation rows, five frequency-only neighbors, nine partial supplied-object
reader cases and their diagnostics/cursors/recovery. `low_count.c` checks twelve
direct/generic prefix/count combinations and initializes required backing only
after reading. `boundary.c` checks full assigned T1 headers/coefficients, defined
pipe histories, failed-trial cursor, process/thread locales, fractional input
and output, signed zero/subnormal/infinity/NaN. The withdrawn bare-T1-pipe
"success" remains central diagnostic evidence, not a valid golden.

ASan/UBSan instrument the native C boundary and the four C consumers. Rust,
MPFR and other dependencies are not instrumented. Fatal unsupported/write
behaviors are checked in separate processes; callback arrays remain synchronous
borrowed storage. Allocation exhaustion, arbitrary foreign-pointer violations,
Linux and concurrency are not claimed.

No scientific kernel, precision, tolerance or accepted historical evidence is
replaced. Earlier provisional failures remain separately named in build/ and
are described in the report. The final delivery is the named receipts above,
not every directory containing "final" in its name.
