# Localize event sign failures and retain actual machine code

Central review, 4 November 2026, continuing the bounded event diagnosis in
[0119](0119-focused-linux-review-and-svd-contract-proposal.md) and
[0122](0122-m2-acceptance-and-al-checkpoint-review.md). No arithmetic, precision,
NaN policy or acceptance gate changes here.

## Saved Linux result

Central replayed all 588 comparisons from run `41392963741`, rebound five
archive members against its collection manifest and eleven source files against
the executed Git revision `9991e804`. The original 107 failures reproduce:
thirteen original event rows, sixteen expression-prefix rows and 78 arithmetic
controls. Every failure is negative native NaN versus positive Rust NaN.
All sixteen assignment controls pass. All 76 actual ordinary-to-binary64
return observations preserve value, class and sign.

The prefix expressions reproduce their actual callback results, but are separate
computations; their output does not reveal the instructions inside a callback.
They localize the following distinct boundaries:

| Input history | First differing prefix or return |
| --- | --- |
| Zero or negative-zero amplitude | Gaussian `val/amp`, an ordinary zero/zero division |
| Zero width, Gaussian epoch/width | Ordinary multiplication of zero by an infinite factor |
| Zero width, exponential width/epoch | Wide zero/zero division; the width callback's second division follows it |
| Zero width at Gaussian onset | The first `-0/+0` division already matches as negative NaN; the next division by width loses that sign |

The 78 arithmetic-control failures split into 54 propagated operand-NaN
differences and 24 invalid-operation differences. The native ordinary and wide
operations differ when both operands are NaNs: for the observed canonical
opposite-sign pair, ordinary operations choose positive NaN, whereas wide
multiplication and division select different operand signs. A single universal
NaN-sign rule would not reproduce these controls.

The pinned MPFR 4.2.2 `mul.c` and `div.c` explain the Rust mechanism. Operand-NaN
branches set the destination's NaN classification without selecting a source
operand's sign. Division selects the quotient sign before its zero/zero and
infinity/infinity branches, whereas invalid multiplication leaves the destination
sign. A newly created Rug result and a reused result therefore need not match
the native operation's NaN behavior. This source explanation does not establish
the native compiler's operand order or authorize a general arithmetic rewrite.

Retain mature Rug/MPFR finite arithmetic and the existing ordinary rounding
boundaries. A later correction must address demonstrated generation and
propagation at their source sites, preserving the first rounding direction,
Linux ordinary range and Rug128 timing. The passing native transcendental and
return-transport controls provide no reason to replace those paths.

## Evidence preparation and verification

`event_profile.py` now binds executed binaries and checks hashes around inspection,
verifies the
actual `t2fit_glitch.C.o` against the A2 archive member and source/compile receipt,
and saves disassemblies of the native event executable, executed Rust test
executable, native callback object and Rust adapter object. A scoped archive
contains exactly those four files. The existing scientific comparisons still
run and report their original failures. AL owns the matching narrowly scoped
collector rule alongside its GUI collection changes.

Two focused tests pass, covering strict sign/neighbor comparisons and rejection
of changed archives, sources, objects and executables before/during inspection.
A fresh macOS run completes all 588 comparisons and retains exactly the existing
60 arithmetic-control failures. Its native table, Rust table and 76-return table
are byte-identical to the previous macOS probe. All four archived binaries
rebind to their recorded identities. This is evidence-retention verification,
not event-science acceptance or Linux execution of the new capture.

Evidence remains outside Git under
`decisions/evidence/release-qualification/build/event-code-20261104/`.
Fresh macOS receipt SHA-256:
`c9ec2a9e96a41cfe2dbf8eeb34165dff5b18ad279bc0a7e93281a61e026dc643`.
Machine-code archive SHA-256:
`80bfc4b656285ed202cc105ceca1780baf561fa6199ef36c46c459ecd0c5ec56`.
Central saved-data/retention review SHA-256:
`c801676b49e7475788817b95ac4c6a82c051aa86946afbf316588bec310423e6`.

The next combined Linux run must supply this machine-code evidence before
source-local operand/sign changes are accepted. The original Linux sign
failures, macOS diagnostic failures and all unrelated scientific gates remain
open. This preparation adds no separate execution campaign or publication
authorization.
