# U first-divergence diagnosis and recurrence preparation

AL report, 4 November 2026, under
[0137](../../decisions/0137-q-z-local-acceptance-and-u-diagnosis.md).
U production code is unchanged. The original Linux run `41392963741`, candidate
`9991e804d0e230248e5fb20974b7e63384d4efeb`, still has 30 failing and 67 passing U
histories. Its complete matrix remains 86 passing / 11 failing / zero blocked.

## Result and scope

Every one of the 30 first differing saved U snapshots has an exact paired
arithmetic reconstruction: source-ordered binary64 operations reproduce the
native value, and the current fused operations reproduce the Rust value.
The counterfactual TT calculations retain Rust's 128-bit timing arithmetic.
No tolerance, precision, scientific convention or nonfinite policy needs to be
changed to explain these first differences.

This establishes the numerical cause at the observed boundaries. It is not a
patched-engine replay or closure of all 300 differing leaves in those histories.
The saved begin/end events do not expose instruction order inside a multi-
observation `tt` call. The report identifies the first differing event, then
reconstructs the relevant primitive or the first observation's TT calculation;
it does not treat serialized field order as an instruction trace.

| First observed boundary | Histories | Discriminating reconstruction |
| --- | ---: | --- |
| IFTE `pc` recurrence | 4 | `ifte_big`, `ifte_little`, `tt_multi_calc_1`, `real_tt`: separate product/subtraction versus one FMA reproduces both cache-cell bit patterns from identical operands |
| IFTE `vc` recurrence | 6 | `tt_e2_u{0,1,2}`, `tt_delta_retention`, `tt_negative_clock`, `tt_repeat_fb`: the first product/add must be separate for the native cell; subsequent additions/subtraction retain source order |
| FB recurrence | 2 | `fb_count_3`, `fb_count_8`: only separate recurrence matches the first native output; either accumulation choice happens to give that same output at these inputs |
| FB recurrence and accumulation | 1 | `fb_count_16`: both must be separate to match the first native output |
| IFTE reverse accumulation feeding TT correction | 11 | `tt_e1_u{0,1,2}`, `tt_dilation_{-1,0,2}`, `tt_multi_calc_{0,2}`, `tt_mixed_calceph_backing`, `tt_cvs`, `tt_zero_vectors`: separate accumulation is necessary and sufficient among the tested operation choices for both first-observation TT correction fields |
| Source dot-product order feeding TT correction | 6 | `tt_e0_u{0,1,2}`, `tt_e-7_u{0,1,2}`: `deltaT` starts at zero; separate dot products and ordered additions reproduce both correction fields |

For each of the 17 TT-correction histories, all eight combinations of fused or
separate polynomial recurrence, accumulation and dot product were evaluated
with Rug128-equivalent timing rounding. For the three FB histories, all four
recurrence/accumulation combinations were evaluated. This separates the causes
instead of attributing every final correction difference to the dot product.
For example, ten of the eleven IFTE-accumulation histories also have a different
dot product, but that difference does not change their first rounded correction
once accumulation is source-ordered. `tt_zero_vectors` independently isolates
the accumulation difference with zero position/velocity vectors.

## Source operations and numerical method

Let `R` mean one binary64 rounding. The implicated source sites are:

- `ifteph.C:433`: `R(R(twot * pc[i-1]) - pc[i-2])`; the Rust expression currently
  rounds the complete product/subtraction once with `mul_add`.
- `ifteph.C:459`: multiply, add `pc[i-1]`, add it again, subtract `vc[i-2]`, in
  that order. Rust currently fuses the first multiply/add.
- `ifteph.C:442` and `tt2tdb.C:304`: descending coefficient accumulation,
  `s = R(s + R(p * coefficient))`, versus a fused product/addition.
- `tt2tdb.C:299`: the corresponding FB polynomial recurrence.
- `tempo2Util.C:43`: `R(R(R(a0*b0) + R(a1*b1)) + R(a2*b2))`. Rust's
  `dot_product` instead uses two nested FMAs, with `a1*b1` rounded first.

Chebyshev recurrence and rounded-operation analysis are established methods.
A generic polynomial evaluator would change operation order and would not be
an appropriate oracle for this exact source contract. The diagnostic reuses
the repository's rational `round_binary` implementation, the authored fixture
constructor and existing U comparator/diagnostic attribution. It reconstructs
only the implicated finite calculations, with the original binary64/binary128
encodings retained in the evidence. It is not another production ephemeris
implementation.

The IFTE cache replays require the two engines' relevant input operands to be
identical. The FB and TT replays independently decode hash-matched binary file
contents and start from the saved input state. TT uses the existing clock sum,
epoch split, constants, unit branches and conversion order. Native timing
operations are rounded at 113 bits; the Rust reconstruction uses 128 bits and
the existing final binary128 projection. IFTE's conversion by 86400 and FB's
`IFTE_TEPH0` subtraction retain their source meaning. No unit reinterpretation
is proposed for the authored `units=0` branch.

The record-origin expression at `ifteph.C:250` has identical fused and separate
results at the TT witness operands; the diagnostic asserts this explicitly.
These first differences therefore do not establish a separate defect at that
site. They also do not isolate the velocity accumulation at `ifteph.C:471`.

## Evidence and missing machine-code observation

Evidence root:
`decisions/evidence/release-qualification/build/u-0137-20261104/`.

`final-replay/receipt.json` binds 393 exact members of the immutable archive.
It checks all 97 native/Rust input-action pairs, input hashes, fixture/resource
bytes, launch identities and per-history result hashes. The original comparator
and frozen diagnostic-attribution rules reproduce exactly the original 30
failed history IDs and 300 differing leaves; the other 67 histories still match.
Source reconstruction applies only the previously adopted native correction
patch and matches all 17 compiled source identities. Earlier diagnostic adapter
attempts and their logs are retained separately.

Recorded execution identities include:

| Identity | SHA-256 / value |
| --- | --- |
| Native compiler | GCC 13.3.0, Ubuntu `13.3.0-6ubuntu2~24.04.1`; `1353e9bdd29a7295c7226bf6c63abccce056d8cac31f112e5cdbecc3f28c2769` |
| Native compiler backend | `840b332fb62ec6f694ac77d91fe69ef7f80b0d69512ed89374af0ee7a506255d` |
| Corrected native executable | `081f6604517e878d3a6e195138c9b766ce3adc8c75d2bc0cc108a68784666247` |
| Executed Rust test executable | `e61913ce73a5b70e20c4ae061a78c9d5217fcdf61f57dd07d9b1208a0f1b430a` |
| Adopted native patch | `80a49ae241d56d6ae6062fc65f370cfb89770f6fe8f31e83585286219e53d322` |

The recorded compiler flags are `-O2 -std=gnu++11 -DHAVE_CONFIG_H` plus the bound
include path. The actual retained macro file verifies 53-bit `double`, 64-bit
ordinary `long double` in 16-byte storage, and 113-bit `_Float128`. The timing
header uses the latter; the U witnesses above concern binary64 operations.

The executable, preprocessed-source and link-map hashes are recorded in the
bound receipts. Their executable/object bytes and U disassembly were not kept
by the original collector. Thus this review rebinds their recorded identities,
not absent object bytes. The existing contraction artifact has no U object
listing. Numerical agreement at the witnesses is demonstrated; a claim about
every actual Linux instruction site would exceed the retained evidence.

Before closing a corrected Linux result, capture `objdump -Cd` of the actual
`time_ephemeris_port` corrected native executable and the executed Rust test
binary, checking each file against its fresh receipt's executable hash first.
Save the disassemblies and a hash receipt under the run output, outside excluded
source/cache directories. The existing collector retains `.s` and `.json` files.
If the order of multiple calls inside one `tt` event is needed, the narrow
additional trace is the IFTE call's normalized time, `np/nv`, recurrence and
accumulation operands, followed by FB `tc/pc/ctatv`, dot-product partial sums and
the two TT corrections. This can distinguish unobserved sites without another
broad diagnostic campaign.

To repeat the bounded saved-data diagnosis with fresh output:

```sh
python3 decisions/evidence/release-qualification/build/u-0137-20261104/replay.py \
  --out /tmp/u-0137-replay-new
```

It uses the retained archive, frozen source export and hash-checked fixture
resources. It neither compiles nor runs TEMPO2 and does not alter its inputs.

## Concrete correction proposal, not implemented

Use the existing measured-platform pattern for the six demonstrated expression
sites: preserve macOS arm64's current fused operations and use source-ordered
separate binary64 operations on Linux x86_64. Retain all coefficient order,
cache/reset logic, epoch selection, conversions, units and Rug128 timing values.
Do not infer a change at the record origin or velocity accumulation solely
from these first witnesses; inspect their actual Linux instructions/operands.

The focused controls should include the four direct `pc` and six `vc` witnesses,
FB counts 3/8/16, the isolated IFTE accumulation in `tt_zero_vectors`, and the
zero-`deltaT` dot witnesses across the existing unit branches. Keep the full
97-history U comparator, its warm-cache/reinitialization/subinterval neighbors
and all original raw comparisons. MacOS controls must preserve the fused bit
patterns. Actual Linux replay must check all later state, output, diagnostic
and nonfinite records; matching the first points is not sufficient acceptance.

The f64 `tempo2_util::dot_product` is shared with propagation, troposphere,
position/standard fitting and constraints. GW simulation defines its own
Rug128 `Float` dot product in `gw_sim.rs`; it is outside this helper's caller
scope. Correct the f64 source helper if released; do not compensate inside TT
conversion. Its other callers require their existing focused gates. No closure
of AB, general fitting, DDH or Z Gaussian failures is inferred from this U
diagnosis.

## Recurrence workflow preparation

Commit `2c13094` adds the accepted `8716d1d` → `e24622d` recurrence pair to the
existing integrated workflow. It reuses the owner-reuse pair's installed after
endpoint, native qualifier and direct clients as the new before endpoint.
The new frozen after source is explicitly fetched and built. Full-C/native,
direct, concurrency, installed-consumer and debug/release library checks remain
separate, as do direct versus full-C measurements and inherited native failures.

The existing installed-consumer runner now includes the public `&Float` borrow
witness once in its canonical path; all earlier frozen endpoints expose the
same API. Both stdout and stderr are compared and bound. The obsolete manual
copy/build sequence was removed from [RECURRENCE.md](../predictor_performance/RECURRENCE.md).
Linux-specific gates require the named wide-significand/exponent test's passing
line in both build profiles; empty, ignored and failed test selections fail.
The installed library disassembly is retained. All pair compilation precedes
the sequential predictor measurement block, without changing sample counts,
native failure policies or performance budgets.

Twenty-five focused tests pass: recurrence and existing pair routing, missing
source/workload behavior, independent failure continuation, borrow-output and
diagnostic rejection, actual wide-range log gates, and artifact inclusion/
exclusion. These are validation-plumbing checks, not fresh Linux scientific or
performance execution. The coefficient-view pair is a separate future addition.
No push, workflow dispatch or new native campaign was performed.
