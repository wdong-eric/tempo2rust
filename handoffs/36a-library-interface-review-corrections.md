# AJ early interface review corrections

Central review, 27 October 2026, under 0084/0095 and the existing 0053 correction policy. **AJ remains the sole Rust writer. These are implementation corrections before final handover, not new scientific choices or milestone acceptance.** Central has relayed the findings to AJ; final committed artifacts and their verification still require review. AK remains unreleased.

The executed subject is a frozen copy of the then-current `validation/library_interfaces/build/debug-prefix/lib/libtempo2pred_rust.dylib`, SHA-256 `97614452cbbf05fb66323b3894b0291bc2e54298e67d49fedfa09496eb6801ea`. This is a provisional installed binary, not a final delivery revision. Current working source snapshots explain the findings but are not asserted to be its compiled source identity. The native comparator uses the unchanged pinned A2 archive `313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5`; its `t1polyco.c` bytes equal the raw `fda6ad4` file. Header/compiler/link commands, executables and outputs are retained.

## 1. Preserve defined C set reads with low declared counts

The current `T1PolycoSet_Read` stops after successfully reading a segment whose `ncoeff` is less than two, on the premise that immediate evaluation would lack initialized C1. That changes a defined reader operation. C callers may inspect the parsed metadata and initialize the required exposed coefficient backing before evaluating. Existing owned Rust file constructors still require 2..32; that invariant is not a universal precondition for this separate C operation.

The five external-consumer cases show:

| Input | Native result | Reviewed adapter result |
| --- | --- | --- |
| One count-0 record | status 0; one segment; EOF reached | status -1; zero segments; EOF not reached |
| One count-1 record | status 0; one segment; EOF reached | status -1; zero segments; EOF not reached |
| One count-2 record | status 0; one segment | Matches |
| Count-2 followed by count-0 | status 0; two segments | status 0; only one segment |
| Count-2 followed by count-1 | status 0; two segments | status 0; only one segment |

The witness reads only assigned header/count fields, then explicitly sets C0/C1 before frequency evaluation. Every resulting native frequency is 100.5; no unwritten coefficient is evaluated. Preserve successful records, count, EOF/cursor behavior and the caller's ability to supply backing. Do not synthesize missing coefficients or weaken the owned Rust file invariant. Add these direct and mixed-prefix cases to the final external consumer. A standalone file client such as getPeriod must separately check the backing its evaluation needs, because that client has no intervening caller write.

AJ has reported implementing this correction during ongoing work; independent final closure remains pending.

## 2. Preserve successful reads from borrowed nonseekable streams

The reviewed generic `T2Predictor_FRead` rejects `ftell < 0` before attempting a parse and returns on failed fallback `fseek`. Raw `tempo2pred.c:51-71` records the position, tries Cheby first and attempts an unchecked rewind only after failure. A complete one-segment Cheby pipe input succeeds natively, returns Cheby with one segment and leaves the `T` of a following `TAIL` line unread. The reviewed adapter returns -1 and leaves the initial `C` unread. The seekable control matches. This is a defined regression.

**Correction to the initial T1 interpretation:** AJ identified, and central confirmed from raw/A2 `cheby2d.c:651-668`, that the initial Cheby trial uses `fgets` and consumes the entire first line. The original `central-t1-pipe-read.py` therefore loses its `TEMPO2:` marker before the unsuccessful rewind. The fallback treats the short body lines as fixed-format input; `t1polyco.c:111-216` has unchecked conversions returning unwritten scalar locals. Its observed status 0/name/count/EOF do not prove a valid parse. The original receipt and exact script are retained only as historical diagnostic evidence; their assertion of a defined success is withdrawn. Do not use or rerun that driver as an acceptance oracle, and do not emulate its undefined conversions.

AJ's replacement histories use a discarded leading line followed by a complete T1 record for the generic pipe reader, a direct T1 pipe read, and a separate failed-Cheby cursor check. Those histories must check all assigned header/count/coefficient values, not just apparent success. Removing only the initial `ftell` rejection is insufficient if a failed fallback seek prevents a defined remaining T1 trial. Preserve source trial order and actual consumption; do not promise rollback on a nonseekable stream. Keep caller ownership of FILE and the separately justified active-union correction. Include seekable controls and the independently required supplied-object partial reads. Final evidence remains pending.

## 3. Make private scalar conversion independent of caller numeric locale

The bridge uses locale-sensitive `snprintf("%La")` to feed a Rust numeric path whose libc prefix scan and numeric conversion do not consistently share that locale, and locale-sensitive `strtold` to consume Rust decimal output. Both directions can change a valid native scalar. The original numerical API uses initialized scalar backing directly and has no file parse in these witnesses.

| Fully initialized T1 frequency input | Native under C/German/French | Reviewed adapter under German/French |
| --- | --- | --- |
| F0=100.125, C1=30, count=2 | 100.625 in every locale | NaN |
| MJD/reference MJD/F0/C1 all 1, count=2 | `0x1.0444444444444p+0` (about 1.0166666666666666) in every locale | `0x1p+0` (1), silently truncated |

The second case uses exact power-of-two input scalars so hexadecimal input formatting has no fractional separator, isolating the output conversion failure. Calls use actual available `de_DE.UTF-8` and `fr_FR.UTF-8`; resetting to C recovers. Result formatting is deliberately performed under C in the witness, so the comparison observes scalar changes rather than different presentation conventions.

Fix both private conversion directions with a lossless, locale-independent boundary. Preserve the caller's process/thread locale and the source text reader/writer conventions; changing global locale is not an acceptable conversion shortcut. Retain the native-long-double width and signed/nonfinite handling. Verify nontrivial input, finite fractional output, locale preservation and subsequent recovery. This is a translation defect; no estimator, precision or numerical tolerance change is authorized.

## 4. Include authored notices in the installed source tree

At inspection, `rust/install.py` SHA-256 `d3b1aa1440998eadba2d3a0be5590c05f80d23b34a61b9516b9bdd18514eda29` copies `*.md` and selected package files but omits `rust/LICENSE` and `rust/MT19937_LICENSE.txt`. Both were absent from `debug-prefix/share/tempo2-rust/rust`, despite the installed README's LICENSE link and the stated source/notice deliverable. Include those required authored files and verify their installed bytes. Source hashes are `8ceb4b9ee5adedde47b31e975c1d90c73ad27b6b165a1dcd80c7c545eb65b903` and `0ea2610f1e15d7709686620987d3be8f6cc985c8302850ffa8ee51b232fccbf8`, respectively. Broader final packaging remains AL's obligation; this does not authorize publication or replacement of canonical installations.

## Evidence and closure

Authored original probe recipes are under `decisions/evidence/library-interfaces/`. The first script creates a new temporary workspace and freezes the provisional binary; the four subsequent scripts use its recorded pointer and separate output names. They are historical review recipes, not the final AJ verifier. Reuse their drivers against an explicitly identified final artifact in a new directory rather than overwriting these results. The T1 pipe recipe is a historical invalid oracle as explained above; its native run and asserted scope must not be treated as defined source behavior. Each script's preserved hash identifies the exact executed form, not endorsement of every original interpretation.

Five additional executions enabled `DYLD_PRINT_LIBRARIES=1`: every program loaded the frozen library at its explicit private rpath and reproduced its original stdout exactly. This ties the observations to an actually loaded artifact; it does not establish that every original native path was defined. This does not assert the mutable checkout was that artifact's source.

Original directory: `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-aj-low-count-review-tj37og1s`. Seventy-eight selected files are copied byte for byte into ignored `decisions/evidence/library-interfaces/build/review-20261027/`; original paths in receipts remain intact. Generated evidence and binaries are not committed.

| Receipt | SHA-256 |
| --- | --- |
| Low-count reads | `2ad6122e55b311f2bf04882e985ede45cd00b4990c18342b86c0ca7b8a06a0d8` |
| Cheby pipe | `60ca8779443dc568e87a800da6a84299a805d8f48080bf04df446e7b0cc9c455` |
| Original T1 pipe diagnostic; defined-success claim withdrawn | `ecd1357e2a03dbc2d463cfd05a4803d1980cf82b85a34722cafadf318a2ffced` |
| Locale input/conversion | `73d069de9081550e3d0b5ee3bf83aad4d5c753ca57dd238fd68d3b14e8c5940a` |
| Locale fractional output | `d02db2e8ff385cdb679d38acb8b9d76d5c36d29829b13cc85a9d75a7ce63a490` |
| Actual loaded artifacts | `65dc298f6979c25e499596b6011ba1348d944c47b2bbb0fc8ad8af4933a48869` |
| Central preservation | `9d9efb942abe5e40407c370af5944d7540424cb5560b04a201ea42421f3d2636` |

Close these findings with the corrected committed source, final installed artifacts, exact reader/state/stream/locale checks, source/bug explanations and affected regressions. Keep the original witnesses, the explicit T1 interpretation correction and the complete 0095 numerical ledger. A corrected preliminary run is not central acceptance of AJ or release of AK.
