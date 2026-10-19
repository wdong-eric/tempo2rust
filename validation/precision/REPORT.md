# Assignment B — precision study

Prepared 19 October 2026. Owned directory:
`/Users/Eric.Dong/Downloads/tempo2-rust/validation/precision/`.

**Recommendation for central review: use `rug::Float` with a fixed 128-bit
significand for extended scalars, including MJD epochs and phase arithmetic.**
Keep pulse counts as checked integers and retain TEMPO2's split spin-phase
calculation. This is a recommendation, not a shared decision or production
integration. No files outside this assignment's directory were changed.

`qd` 0.8.0 is fast and retains nanosecond epoch differences in ordinary
arithmetic, but is unsuitable as supplied: its decimal parsing trait always
rejects input, it has no decimal formatter or trigonometric API, and its
truncation implementation can select the wrong pulse. The established MPFR
alternative passes the tested contracts without new project-owned mathematics.

## 1. Delivered artifacts and identities

- `Cargo.toml`, `Cargo.lock`, `src/main.rs`: isolated executable comparison.
- `BUDGET.md`: quantities, units, tolerances and experimental conventions.
- `OPERATION_SUPPORT.md`: upstream operation inventory and dependency source audit.
- `run.py`, `README.md`: reproduction, platform probes and command capture.
- `snapshot_reference.py`: read-only consumption of A's available exports.
- `summarize.py`: Decimal-based reduction of full precision results and file hashes.
- `probes/*.rs`, `probes/quadmath/{Cargo.toml,Cargo.lock,src/main.rs}`:
  native qd API and two distinct f128 feasibility checks.
- `results/numerical.tsv`, `parsing.tsv`, `special.tsv`, `timing.tsv`, `summary.md`:
  actual measurements, including failures. Every numerical row contains the
  actual value, expected value, absolute error, unit and tolerance.
- `results/reference_inputs/`: immutable-for-this-run snapshots of four A
  exports, A's precision probe, final report and exporter source. `provenance.json` records
  originating paths, hashes, source identity, compiler and instrumentation identity.
- `results/commands.json`, compiler/build/probe logs, `dependencies.txt`, and
  `artifact_hashes.json`: commands, outcomes, dependency graph and checksums.
- `.gitignore`: excludes local crate caches and build products.

Versions tested:

| Component | Pinned/tested version |
| --- | --- |
| TEMPO2 reference source | `fda6ad4a9753ad7dc5d291406f2f310a9161423f`, canonical checkout clean at final check |
| qd | **0.8.0**, `sarah-quinones/qd`, default std feature; declares 105 significand bits |
| Rug | **1.30.0**, features float, integer, std; defaults disabled |
| gmp-mpfr-sys | **1.7.1**, bundled **GMP 6.3.0 / MPFR 4.2.2** |
| num-traits | **0.2.19** |
| separate f128 crate | **0.2.9**, f128_internal **0.2.2**, f128_input **0.2.1** |
| Stable Rust / Cargo | rustc **1.97.1** (`8bab26f4f`, 2026-07-14), cargo **1.97.1** |
| Installed nightly probe | **1.101.0-nightly**, `d080e7dff`, 2026-09-27; toolchain `nightly-2026-09-28` |
| Host / target | macOS arm64, Darwin **25.6.0**; **aarch64-apple-darwin**, LLVM **22.1.6** stable |
| Optional GCC retry / A reference | Homebrew GCC **16.2.0** |

The registry source was inspected, not inferred from a trait name or cached
older docs. See [qd 0.8.0 docs](https://docs.rs/qd/0.8.0/qd/) and
[Rug 1.30.0 docs](https://docs.rs/rug/1.30.0/rug/). Cargo.lock pins transitive
versions and registry checksums.

## 2. Reproduction and platform evidence

```sh
cd /Users/Eric.Dong/Downloads/tempo2-rust/validation/precision
export CARGO_HOME="$PWD/.cargo-home"
export GMP_MPFR_SYS_CACHE="$PWD/target/gmp-cache"
cargo fetch --locked
cargo fetch --locked --manifest-path probes/quadmath/Cargo.toml
python3 run.py
```

After fetching, `run.py` uses `--locked --offline`. It performs real execution
on this host; no toolchains or Homebrew packages are installed. The first
sandboxed fetch could not resolve crates.io; an approved network retry
succeeded. This was an environment failure, not a crate failure.

The GMP/MPFR source build succeeded with the default Apple C toolchain. Its
first build included native library checks and took several minutes; no reliable
cold-build benchmark was captured. The final executable is about 1.0 MiB with
debug level 1; `otool -L` lists only System and iconv, consistent with bundled
static GMP/MPFR linkage. This size includes the experiment and both main
candidates and is not a production-size prediction. Distribution must account
for Rug/GMP/MPFR licensing and native build dependencies.

| Candidate | Compile evidence | Runtime evidence | Assessment |
| --- | --- | --- | --- |
| qd 0.8.0 | Stable native build succeeds. FromStr, Display, sin/cos/atan2/floor/round probe fails as expected | Arithmetic tested using clearly labeled MPFR-assisted input/output; boundary defects reproduced | Reject as supplied |
| Rug128 | Stable native build succeeds with bundled GMP/MPFR | All **516** candidate comparison rows pass, plus parser/round-trip/integer assertions | Recommend for central review |
| built-in f128, stable | Rejected as unstable | None | Not a stable production option |
| built-in f128, installed nightly | Arithmetic builds; decimal FromStr/Display fail; math probe fails at link with missing **`_atan2f128`** | Literal-based 1 ns arithmetic passes the study's 1 ps gate | Experimental and incomplete on this tested platform |
| separate f128 crate | Default compiler fails: `quadmath.h` missing. Explicit GCC 16 + library path succeeds | Decimal 36-digit round trip and +/-1/4 ns recovery pass; parser accepts malformed prefixes; default display loses digits | Feasible with GCC, but less attractive than Rug |
| Linux x86_64 | No execution environment available; only arm64 target installed, no Docker/Podman found | **Not tested** | Portability remains open; no cross-compile claim |

The built-in status agrees with [official f128 documentation](https://doc.rust-lang.org/std/primitive.f128.html).
The [separate crate's current README](https://github.com/jkarns275/f128) discourages
use because of incomplete support and performance/linking issues. The study
bounded that exploration at native build, parsing, formatting and epoch
arithmetic; it did not repair either alternative.

A first built-in arithmetic smoke probe used an unnecessarily tight 1e-25 s
assertion and failed. Its recorded bits correspond to a recovered
`1.00000000000000024241955688680834421e-9` s, an error of about 2.42e-25 s.
The final feasibility probe uses the **already documented 1e-12 s timing gate**.
This is finite binary128 rounding, not a demonstrated built-in arithmetic defect.
The missing decimal API and math link failure remain decisive.

## 3. Numerical results and interpretation

The final run records **1,460 numerical comparisons**: 516 Rug128, 441 assisted
qd, 324 MPFR256/512 convergence checks, 80 binary64 controls, 92 MPFR113 replays
of A, and 7 independent spin formula identities. There are **92 failed gates**:
76 binary64 controls and 16 qd boundary-related comparisons. Missing native qd
I/O is a separate capability failure, not hidden among arithmetic passes.

Budgets were defined in `BUDGET.md`: 1 ps for epoch/delay/residual accuracy;
1e-9 turns for spin phase; exact discrete counts; 1e-28 * max(1, |reference|)
for the scalar math survey. They are local study allocations, **not approved
TEMPO2 acceptance tolerances**. The original 4 ns tests also use distinct units:
`subtractBATs` compares seconds, while `offsetSATs` subtracts MJD quantities.
A single number must not be applied to both without unit conversion.

| Quantity | Assisted qd maximum absolute error | Rug128 maximum absolute error | Unit / outcome |
| --- | ---: | ---: | --- |
| Parse 16 supplied/synthetic epoch strings | 1.67504e-23 | 7.06788e-30 | s |
| Recover +/-1 ns and +/-4 ns, including midnight crossings | 1.12081e-23 | 7.53109e-30 | s |
| Subtract nearby independently parsed epochs | 6.66694e-24 | 1.17410e-29 | s |
| Source-constant affine epoch scale/inverse | 2.11787e-23 | 2.32978e-29 | s |
| Source-order F0-F3 absolute phase | 1.88191e-20 | 3.01665e-27 | turns |
| Source-order F0-F3 fractional phase | 2.03941e-20 | 2.93461e-27 | turns |
| Source-order F0-F3 residual | 2.84784e-23 | 4.09791e-30 | s |
| Ordinary synthetic spin pulse counts | exact | exact | 7 spans, up to +/-20000 days |
| Boundary residual error | **1.39641e-3** | 1.20862e-31 | s; qd selects wrong pulse |
| Decimal serialization -> parse | exact with MPFR assistance | exact natively | 16 values each; low limbs retained |

All signed delay rows include recovered seconds, not just errors. For example,
the separate quadmath crate recovered +4 ns as
`3.99999999999999987915691216970665703e-9` s and +1 ns as
`1.00000000000000024241955688680834421e-9` s on this host. In contrast,
the f64 controls recover **zero** for all 64 signed nanosecond additions at
these large MJDs. Their maximum epoch parse error is 2.08244e-7 s.
A's native `long double` probe reports 53 significand bits on this host;
its actual reference engine therefore uses GCC `__float128`, with 113 bits.

The 324 reference-convergence checks pass 1e-60 * max(1, |expected|); their
largest ratio to that budget is about 2.38e-5. Large absolute errors in exp(100)
are naturally scaled to its approximately 2.69e43 output; inspect the normalized
ratio in `summary.md`, not only the unscaled number. The seven independent
512-bit unsplit spin polynomials agree with the source-split expression within
the 1e-140 scaled identity gate. These checks establish negligible arithmetic
reference error for this matrix, not a proof over all real inputs.

The qd failures are concrete:

- `trunc(0.99999999999999999999)` returns **1**, and its negative counterpart
  returns **-1**. Truncating limbs separately misses the low limb's effect.
- The resulting `% 1` has the wrong signed quotient, with nearly one turn error.
- Source `fortran_nlong` applied to `0.49999999999999999999` returns **1**
  rather than **0**; negative and +/-1e11-turn boundary cases also fail.
  The downstream fractional phase shifts by one turn, or about **1.396 ms**
  at the synthetic 716.123... Hz frequency.
- Inherent `Quad::le` reports `(1,1e-30) <= (1,0)` as true. The derived Rust
  `<=` operator reports false on that same normalized pair.
- `exp(709)` returns infinity and `exp(-709)` returns zero, while MPFR gives
  finite nonzero values. These are recorded range-edge diagnostics outside the
  main timing-domain math grid, not additional accepted scientific behavior.

The valid-domain arithmetic, sqrt/ln/log10/exp matrices pass for assisted qd.
Rug additionally passes sin/cos/tan, asin near +/-1, atan2 across the branch cut,
and real powers. No local trig, decimal conversion or qd repair was implemented.

### Comparison with A's pinned reference

The snapshots contain A's final `barycentric`, `barycentric_4ns`,
`long_span`, and **provisional** `terrestrial` JSON exports, plus derived TSV
without any numeric conversion. A's exports record TCB, TT(TAI), DE405, no fit,
no mean removal, binary model NONE, PEPOCH=50000, `refphs=0`, original row
ordering and flags. F0=1 Hz for the original fixtures. The added long-span
fixture has F0=123.456789123456789123456789 Hz, F1/F2 terms, and four epochs
across about 40 years; its exported parameters are used directly in the replay. In the replay, the reference offset is the signed fractional
phase of the first observation, matching the inspected `formResiduals` path.
The TIM epochs themselves were also independently tested from their exact
original decimal strings in the main arithmetic matrix.

For **23 observation/stage records** (16 barycentric candidate records and
7 provisional terrestrial records), the replay consumes A's exported bbat
and evaluates source-order phase, rounded phase, pulse count and residual:

| Backend | Largest residual difference from A | Largest phase/count-field difference | Discrete counts |
| --- | ---: | ---: | --- |
| Rug128 | 7.97058e-27 s | 5.39880e-24 turns | exact |
| Assisted qd | 1.69031e-23 s | 6.35308e-22 turns | exact |
| MPFR113 source-precision replay | 4.33055e-37 s | 3.63491e-26 turns | exact |

All 276 comparisons pass. The perturbed A residuals are
`4.0000000000000000318298963225603978e-9` and
`3.99999999999999994458819109235826022e-9` s. Rug128 agreement with these
exports is far inside 1 ps. The remaining differences are consistent with
113-bit reference arithmetic and A's 36-digit decimal export rounding; the
MPFR113 replay greatly reduces them. This is separate from the exact-decimal
arithmetic oracle and from qd's independently reproduced defects.

This replay **does not calculate observatory corrections**: bbat is input.
A's final report became available before completion and is included in the
snapshots. It identifies uninitialized `pulsar::ne_sw_ifuncN` in the pinned source;
terrestrial exports remain provisional despite repeatability. The replay is
only a conditional arithmetic comparison given their published bbat, never
acceptance of those values as scientific goldens. A separately reports the
`subtractSATs` signal failure (-11), end-to-end test failures, and a 1.13759 ns
pre-fit discrepancy after a named diagnostic initialization correction. These
are existing upstream findings, not failures introduced or fixed by B. Build,
export and availability identity are retained in `reference_inputs/provenance.json`.
No fitting or tolerance relaxation was used to remove discrepancies.

## 4. Scalar cost and packaging tradeoff

Release build, one process, 100 warm-up calls, seven trials of 10,000 items each,
16 cyclic epoch strings/values. Actual min/median/max are in `timing.tsv`.
No CPU pinning or control of concurrent desktop work was attempted.

| Work per item | qd median | Rug128 median |
| --- | ---: | ---: |
| F0-F3 source phase, absolute/fractional phase, residual, pulse count; already parsed | 187.742 ns | 1074.208 ns |
| Add precomputed 1 ns/day, subtract original epoch, multiply by 86400 | 3.221 ns | 40.867 ns |
| Parse one finite decimal string | 452.592 ns, **via MPFR512 + limb split** | 307.883 ns |
| Serialize one value as round-trip decimal | 658.342 ns, **via MPFR512 limb sum** | 368.808 ns |

The qd arithmetic path is faster but fails correctness contracts. Its I/O
figures measure assistance, not nonexistent native APIs. Rug's phase sample
allocates/clones Float values for clarity; it is not an optimized MPFR kernel.
A future implementation can use existing in-place Rug operations after
profiling. These timings do not predict whole-engine speed, parsing throughput
for a complete PAR/TIM file, or the cost of ephemerides, fitting or I/O.

## 5. Recommended representation and minimum interface

Select one fixed policy: **Rug Float at 128 significand bits, round-to-nearest
arithmetic**, with precision established at construction. This is not IEEE
binary128 (113 bits), C long double, or a foreign ABI value. Do not cast memory
between those types. Exchanging round-trip decimal strings with a separate
reference process is sufficient here.

The initial core needs only:

1. A finite decimal scalar constructor with full token validation; translate
   PAR D/d exponents explicitly, reject malformed/non-finite inputs, preserve
   original tokens and context in errors. Rug's native grammar is broader than
   the proposed PAR/TIM numeric subset.
2. Fixed-precision arithmetic, comparison, abs and signed truncating remainder;
   source-compatible +/-0.5 then truncation for pulse rounding. Check machine
   integer range at pulse-count conversion. Do not substitute ties-to-even or
   MPFR's nearest-quotient `remainder()` for `%`/fmod.
3. MJD values held directly in that representation, while retaining the source's
   integer/fractional F0/day calculation. The measured accuracy does not require
   a second production epoch representation or custom double-double framework.
4. Round-trip decimal serialization and an extended-precision absolute-difference
   comparison with explicit units. A printed low-precision number or f64 cast
   must not become the comparison input.
5. Explicit boundaries to already-binary64 upstream routines/constants. Preserve
   those source choices until central review authorizes a scientific change.
   For selected later models, use Rug's existing sqrt, trig, log/exp and power APIs.

A small domain type enforcing construction precision may be useful when C's
interface is designed; this study introduces no production wrapper, traits,
backend switches, local math library, or new parser for entire timing files.
Rug is the simplest established option that meets this matrix. Choosing qd
would require fixing numerical defects plus maintaining decimal I/O and model
math that Rug already provides.

## 6. Supported scope, limits and milestone status

Implemented: exact decimal epoch cases from both supplied TIM files; e/E and
explicit D/d normalization; strict malformed/non-finite outcomes; signed 1/4 ns
delays and midnight crossings; nearby epoch subtraction; affine transforms with
source binary64 constants; F0-F3 spin over about +/-55 years; pulse ties and
boundaries; representative scalar math; lossless transport; scalar timings;
and a bounded phase replay of A's available exports.

Not implemented or validated: a full PAR/TIM parser, F4-F12 numerical sweeps,
tracking/glitches/jumps/noise models, any full binary model or Kepler solve,
observatory correction calculation, a scientific accuracy guarantee for all
parameter domains, Linux execution, release packaging, whole-engine performance,
or production integration. Unsupported operations remain explicit in the table;
there is no f64 fallback. Scalar checks are numerical evidence, not M1a/M1b
scientific validation.

**Assignment B's executable study and required report are delivered.** Central
review must select the backend/precision, confirm source rounding/constants and
input conventions, and set shared fixture tolerances after reading A and B.
M0 acceptance remains open; this chat does not mark that decision complete.
