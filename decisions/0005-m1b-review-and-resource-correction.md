# M1b review and ephemeris resource correction

Central review, 20 October 2026. **M1b remains pending. D's numerical evidence passes, but two reproduced malformed-resource failures violate the adopted recoverable-error contract.** D retains implementation ownership for the bounded correction below. M0 and M1a remain accepted; M2 is not released.

Subsequent resolution: [decision 0006](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0006-m1b-acceptance.md) accepts the corrected M1b delivery and closes both findings after debug/release failure and recovery checks. The review, failed reproducers and pending status below record the earlier delivery and remain historical evidence.

This review does not change scientific expressions, meteorology policy, numerical representation, fixture identities or tolerances. It applies [decision 0004](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0004-m1b-contract-and-d-release.md), which requires malformed resources and native errors to return contextual recoverable failures instead of terminating the host or permitting invalid access.

## Reviewed delivery

The delivery is the working tree reported in [M1B_REPORT.md](/Users/Eric.Dong/Downloads/tempo2rust/rust/M1B_REPORT.md). D is not committed separately; project HEAD is still the accepted M1a base `49e122c3648e5cb91bae17233a062a444a10f9e4`. The following hashes identify the reviewed submission:

| Artifact | SHA-256 |
| --- | --- |
| `rust/M1B_REPORT.md` | `e5ec95c44ac15abb43e072f89ec2122f850f500afc2b56fd3951c641edb1489e` |
| `rust/verification/m1b/source_hashes.json` | `75529426e3c634ef346fbc68dcdb6cb152d01f2113a9ddf3dec0d13ee9034097` |
| `rust/src/ifteph.rs` | `c5a28e3dbf194b40ecab6c4308b0f57ea54bb76b5df25e3672cb8b91beb263d2` |
| `rust/src/read_ephemeris.rs` | `4efe66a33f0a1f83ea22a7df04abff8cf9740dd248b72e2c7d7cf86dfbb4dd1d` |
| Retained `rust/native/jpleph.c` | `d8ae1becbab4585af6b2b76abbeac1599fe17f9a452ac3f4d7da694eb930d88d` |

All 96 entries in the source manifest match the current files. All 80 pinned runtime resources and both immutable terrestrial fixtures match their manifest hashes. All 41 native-source/license manifest entries match canonical TEMPO2; vendored files other than the documented JPL patch are unchanged. Canonical TEMPO2 is clean at `fda6ad4a9753ad7dc5d291406f2f310a9161423f`; the independent A2 repository is clean at `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`.

## Blocking findings

### 1. P1: reject impossible DE405 record grids before native evaluation

[The DE405 preflight](/Users/Eric.Dong/Downloads/tempo2rust/rust/src/read_ephemeris.rs:97) accepts any finite positive record step without bounding the resulting record index. In a copy of the pinned DE405 file, changing only its big-endian step at byte offset 2668 from `32.0` to `1e-300` passes these checks. Evaluating the ordinary PKS fixture then terminates with **SIGBUS**: Python subprocess return code `-10`, empty stdout and empty stderr on the reviewed macOS arm64 host.

The native reader computes `(et - ephem_start) / ephem_step` and converts it to `uint32_t` at [jpleph.c:572](/Users/Eric.Dong/Downloads/tempo2rust/rust/native/jpleph.c:572). The malformed header makes this value unrepresentable. The Rust wrapper's status check after the native call cannot recover from a process signal.

Validate the record grid and the native index/offset limits before allowing unsafe native evaluation. Reject impossible metadata with a contextual `TimingError`. Preserve interpolation and record-boundary behavior for valid resources. This correction belongs at the resource/native boundary; wrapping the CLI in a subprocess or catching panics downstream would not satisfy the library contract.

### 2. P2: bound IF99 record arithmetic before integer conversion

[IF99 record selection](/Users/Eric.Dong/Downloads/tempo2rust/rust/src/ifteph.rs:128) converts an unchecked floating record number to `usize` and adds two before checking the slice. Changing only the copied IF99 file's big-endian step at byte offset 280 from `32.0` to `1e-300` passes initialization. The ordinary PKS fixture then panics at line 129 with `attempt to add with overflow`; CLI exit status is **101**, with empty stdout.

The later `data.get(begin..coefficient_end)` cannot protect arithmetic that has already overflowed. Validate the record grid and bound record indices and offsets before conversion, addition or multiplication, returning a contextual `TimingError`. Do not rely on debug overflow checks, release wrapping, clipping, or a caller catching the panic.

These cases modify only disposable resource copies. They do not indicate a discrepancy in the pinned valid fixture calculations. Nevertheless, handling malformed resources is an explicit M1b acceptance requirement. The existing truncated-header checks do not exercise these otherwise valid headers with impossible record steps.

## Reproducer

The central review rebuilt and ran the existing offline Rust tests before these probes. To reproduce against a subsequent delivery, build its normal CLI first:

```sh
cd /Users/Eric.Dong/Downloads/tempo2rust/rust
CARGO_HOME="$PWD/.cargo-home" GMP_MPFR_SYS_CACHE="$PWD/target/gmp-cache" \
  cargo build --locked --offline --bin tempo2-rust
```

Run the following Python code. It verifies the resource hashes, copies only the 80 declared resources to a temporary directory, changes one copied header at a time, and restores each before a valid follow-up calculation. It does not modify canonical resources or Rust source.

```python
import hashlib
import json
from pathlib import Path
import shutil
import struct
import subprocess
import tempfile

root = Path("/Users/Eric.Dong/Downloads/tempo2rust")
crate = root / "rust"
manifest = json.loads((crate / "tests/terrestrial/manifest.json").read_text())
with tempfile.TemporaryDirectory(prefix="tempo2-m1b-review-") as temporary:
    runtime = Path(temporary) / "runtime"
    for relative, identity in manifest["resources"].items():
        source = root / "tempo2/T2runtime" / relative
        assert hashlib.sha256(source.read_bytes()).hexdigest() == identity["sha256"]
        destination = runtime / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, destination)
    command = [
        str(crate / "target/debug/tempo2-rust"), "residuals",
        "--par", str(crate / "tests/terrestrial/fixtures/test1.par"),
        "--tim", str(crate / "tests/terrestrial/fixtures/test2.tim"),
        "--runtime", str(runtime),
    ]
    baseline = subprocess.run(command, capture_output=True, timeout=15)
    assert baseline.returncode == 0
    for relative, offset in [
        ("ephemeris/DE405.1950.2050", 2668),
        ("ephemeris/TIMEEPH_short.te405", 280),
    ]:
        path = runtime / relative
        original = path.read_bytes()
        assert struct.unpack_from(">d", original, offset)[0] == 32.0
        changed = bytearray(original)
        struct.pack_into(">d", changed, offset, 1e-300)
        try:
            path.write_bytes(changed)
            result = subprocess.run(command, capture_output=True, timeout=15)
            print(json.dumps({
                "resource": relative,
                "returncode": result.returncode,
                "stdout_bytes": len(result.stdout),
                "stderr": result.stderr.decode(errors="replace"),
            }))
        finally:
            path.write_bytes(original)
        restored = subprocess.run(command, capture_output=True, timeout=15)
        assert restored.returncode == 0
        assert restored.stdout == baseline.stdout
```

The observed failures and review identities are recorded in [the evidence summary](/Users/Eric.Dong/Downloads/tempo2rust/decisions/evidence/0005-m1b-review.json). This exact reproducer was executed during review: both failures reproduced, and both restored resources gave byte-identical valid output in separate CLI processes. That observation alone is not a same-process library recovery test.

## Passing evidence and reviewed departures

The central review inspected the changed timing path, its callers, resource/native boundaries, comparison adapter, tests, naming map and local departure comments. Review scope is sequential use of the declared isolated PKS path and explicit failure/recovery behavior.

Checks rerun during central review:

- `cargo test --locked --offline`: all 16 Rust tests pass, including fresh comparisons of all three M1a fixtures.
- Fresh terrestrial adapter exports, generated into a temporary directory: both pass and are byte-identical to D's delivered exports. There are 490 and 1162 scalar comparisons, respectively, with separate exact identity, flags, counts, clock destinations and resource checks.
- All 184 terrestrial comparator mutations pass. Recomputing every saved signed difference, per-field maximum and 4 ns response reproduces the submitted evidence.
- All ten output hashes in the naming-equivalence record still match, including the freshly generated M1a projections.
- The original-versus-retained JPL short-read probe reproduces `0, -2, 0, 0` versus `0, -2, -2, 0` for valid, failed, repeated-failed and recovered calls. Recovered position difference is zero in both builds. This confirms the stale-cache correction's justification.

| Quantity | Maximum absolute difference | Gate |
| --- | ---: | ---: |
| BAT | `5.684341885448086137600e-14 s` | `1e-12 s` |
| Residual | `1.13686836941064559950026880243e-13 s` | `1e-12 s` |
| 4 ns response versus the local BAT derivative | `1.416875336045782805627568e-13 s` | `1e-12 s` |

The source-faithful meteorology policy, fixed scientific expressions and naming pass are retained. Owned/initialized state, recoverable errors, and guards against invalid access remain authorized by decision 0004. The demonstrated JPL cache correction and initialization correction are accepted as justified departures with local comments and regressions. This does not establish that the current resource validation is complete; the two findings above must be corrected.

Formatting, Clippy and D's full public resource/CLI recovery runs were inspected as recorded evidence tied to matching source hashes; they were not all rerun centrally. The new failing probes expose a gap in those tests. The full verifier was not run centrally because it overwrites D's evidence files.

## Bounded return to Assignment D

1. Correct the DE405 resource/native boundary and IF99 record arithmetic at their sources. Explain the demonstrated failure and intended returned-error behavior beside each change. Keep valid TEMPO2 arithmetic and endpoint conventions unchanged.
2. Add focused regressions for both malformed-step cases. Exercise the public library and CLI, contextual errors, empty CLI stdout, and later valid processing in the same host process using a new valid context where construction rejected the malformed resource. Verify relevant overflow behavior in debug and release builds. Retain the existing short-read/cache recovery checks.
3. Regenerate the required M1b evidence once after correction, preserving immutable reference artifacts and all existing gates. The pinned valid outputs should remain unchanged; investigate and report any difference. Keep all M1a regressions passing.
4. Update `M1B_REPORT.md`, departure references and source hashes. Return the corrected delivery for central acceptance.

This is a resource-validation correction within the existing adopted contract, not a request for a new scientific policy, dependency or broad refactor. D owns the implementation. This central review changes only planning documents and does not dispatch a new chat or message another chat.

Linux, broader observatories/models, binaries/DDH, fitting, performance, concurrency stress testing and distribution packaging remain outside verified scope. The original DDH 1 ns failure remains open. M2 still requires its own central contract after M1b acceptance.
