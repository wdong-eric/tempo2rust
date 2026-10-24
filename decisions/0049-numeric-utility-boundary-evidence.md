# Numeric utility boundary evidence

Central decision, 24 October 2026. Record a verified fidelity gap for the eventual full `tempo2Util.C` port. Preserve the fixed Rug128 policy in 0001, the unchanged SVD gate in 0012 and T/U/V's current scopes. This finding does not authorize a production arithmetic change, a new tolerance, an accepted mismatch or a precision switch. Full utility equivalence remains unproven.

## Source expression and controlled experiment

The unchanged pinned `tempo2Util.C:186–217` implements the longdouble remainder as `a - (int)(a/p)*p` and nearest long conversion as `(long)(x > 0 ? x+0.5 : x-0.5)`. The quotient is rounded at the native arithmetic precision before the bounded native integer conversion. Source casts, operation order, sign-dependent half addition and binary64 constants remain part of the contract. A mathematical remainder operation alone does not specify this behavior.

Central freshly compiled the accepted M native harness and measured its 113-bit significand. Sixteen cases cover eight integer-rounding inputs and eight remainders. Every input was first parsed by the native implementation and decoded exactly from its binary128 bytes; the identical exact values were then given to Rug. The disposable Rust probe extracts the existing `fortran_mod` and `fortran_nlong` bodies byte-for-byte from accepted R revision `04918d5`. It also evaluates the actual source remainder expression at 113 and 128 bits, and Rug `%` with two Float operands at both precisions. Native quotient conversions stay within their defined integer domain.

All sixteen source-order calculations at 113 bits match native values and zero signs exactly. At 128 bits, twelve match and four differ:

| Native input and operation | Native 113-bit result | Same source order at 128 bits |
| --- | --- | --- |
| nlong(0.5 - 2^-114) | 1 | 0 |
| nlong(-(0.5 - 2^-114)) | -1 | 0 |
| mod(1, native-rounded 0.1) | +0 | positive remainder close to 0.1 |
| mod(-1, native-rounded 0.1) | +0 | negative remainder close to -0.1 |

The first pair results from the rounded half addition crossing an integer boundary. For the second pair, rounding the division changes the integer quotient. These are discrete differences; a small absolute-error allowance does not establish equivalent integer decisions.

Rug `%` has additional semantic differences independent of the selected precision. For mod(-4,2) and mod(-0,-2), native subtraction returns positive zero while `%` returns negative zero. For mod(1,+infinity) and mod(0,+infinity), the source's finite quotient conversion followed by `0*infinity` produces NaN, while `%` returns 1 and 0 respectively. The rounded 0.1-divisor cases also differ at 113 bits. Only two of the eight `%` cases match at each precision. The accepted partial helper's binary64 divisor adds another discrepancy for the native-rounded 0.3/0.1 case; it matches one of eight cases in this broader probe.

These utility-level inputs are broader than the current strict frontend contract and its unit-period phase use. The experiment does not establish a current end-to-end M1 failure or change prior bounded acceptance. It prevents those narrower successes from being presented as full utility equivalence. Raw source callers also include residual construction, observatory coordinates, time ephemeris, polyco and random-number routines; their defined input domains need explicit review when each is transferred.

## Reproduction and identities

The executed reproducer is preserved without modification as `decisions/evidence/numeric-boundaries/central-review.py`, SHA-256 `8432fbba248b6ae2b951e57add612e13eb11f0d181e2bad83e64d7d62e2d1c6e`. Run it with `python3` from the existing workspace after M's accepted native build/profile and the Rust debug Rug dependency are available. It builds in a new temporary directory and never writes accepted M files or the Cargo target. It requires the pinned workspace path and exactly one existing debug Rug rlib; it is a central diagnostic, not an ordinary verifier or an oracle recorder.

The completed run used `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-numeric-boundaries-fc78rsrs`. Its full bundle is copied locally under `decisions/evidence/numeric-boundaries/build/fc78rsrs/`; the original absolute paths in its provenance remain unchanged. All generated evidence remains ignored under 0039.

- Receipt SHA-256: `1314d2a657a9037970b76ed40d30c8f000767ce6a048d4037739ed5394fdcc0d`.
- Fresh native executable: `a0f5fc89805c180645d93023bb9aa37e2371a97bacf1f9d80df077871f457bbb`.
- Disposable Rust executable: `2c2fa5c8f91d33a7d10471968da64669c03f3472938b70ca2c73e0ac2b0b55bb`.
- Actual generated Rust probe: `94322b5c907ddced65ea9022fdc7640941100e3991849fcd623b6e62fafe4080`.

The receipt records the actual compiler, command, dependency-file hashes, native measured profile and all 56 comparisons. Every stable native profile field matches M except raw macro emission order: the complete sorted macro lines, retaining duplicates, are identical. Both raw digests and this narrow projection are retained. The eighteen protected M source/evidence files and linked Rust dependency bytes remain unchanged across execution. The probe made no production edits and added no acceptance gate.

## Consequence for later implementation

The full utility handoff must preserve the actual source expressions and native conversion domains. It must not describe `%` as universally equivalent to `fortran_mod`, or convert a longdouble divisor through binary64 without source justification. Adjacent source-mapping comments must explain any approved departure.

Rug128 remains mandatory under the existing design. Its demonstrated conversion-boundary differences must stay visible until a separate central precision decision resolves them. Replaying selected operations at 113 bits in this diagnostic is evidence about cause, not permission to add mixed production precision. Do not silently relax exact integer or signed-zero comparisons, broaden a native undefined-conversion case into a golden result, or claim the full utility port accepted while these cases are unresolved. M2's four historical strict-order diagnostics also remain unresolved under 0012. Independent state, reader and resource work can continue.
