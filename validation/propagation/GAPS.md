# AB scope and native-safety limits

The reference is a bounded macOS arm64, GCC 16, O2, serial observation of the
original source. It is not a full timing-model validation, a Rust comparison,
or milestone acceptance. The machine-readable coverage report retains every
primary compiler line and branch outcome, including all zero counts.

## Deliberately unexecuted operations

- A first failed `-dmo` scan would consume uninitialized `dmval2`. A failed scan
  after a successful assignment in the same call is separately exercised.
- `solarWindModel` with eclCoord outside exactly 0/1 leaves automatic coordinates
  uninitialized. An empty current sheet is safe for the direct reader but not
  for that caller's first-cell access. Sheet overflow and out-of-range integer
  conversions are excluded; the direct 499-pair case retains a spare scan cell.
- Cleanup with an active first REFPHS flag and no current observation is excluded.
  Above-capacity counts can evade the source equality test; no invalid owner
  access is used as an oracle. Capacity exits, zero-count initialization, repeated
  valid cleanup and mixed flags are exercised separately.
- Invalid ifunc backing/lookahead and nonpositive basis counts are excluded.
  Nonpositive value-overload counts are executed only on the before-first early
  return. A safe NaN query reaches the final assertion in the supplement; its abort
  does not flush compiler coverage counters. No monotonicity guard is inferred for a port.
- Nonfinite/out-of-range integer conversions in amod, rotation, calendar and mcl2
  are excluded. Factorial UINT_MAX wraps its unsigned loop and has no return;
  it is not run. The non-consuming current-sheet loop is a separate three-second
  killed child, explicitly recorded as a timeout rather than a native exit.

## Bounded execution and observation

Two pulsars have six allocated observation cells each; 26 selected parameter
slots per pulsar have sixteen initialized values and flags. Other pointer backing
is explicitly absent. These are controlled caller objects, not source-initializer
outputs or natural defaults. Every observation field and tail is initialized and
compared, including full phase-reference copies. Upstream vectors, clock steps,
coordinate frames and ephemeris values are supplied; global.C wrappers, full
formBatsAll, residuals, binary models, fitting and CALCEPH are not executed.

The observer records complete initialized observations at each iteration, TT and
Shapiro/DM call order, warning-mask lifetime, successful sheet-read order, and
caller state at normal source exits. Native diagnostics identify failed sheet
opens; complete static diagnostic buffers expose count-saturation retention. It does not read uninitialized automatic locals. The supplement captures all initialized mcl2 caller arrays/scalars at both
the loop-limit exit and the propagated year-range exit. Mixed
capacity failure after an earlier pulsar's successful phase initialization is
also not a distinct authored case.

TKlog diagnostics inherit M's conservative host qualification for its color-strip
pointer arithmetic. solarWindModel's ld_printf inherits M's formatter/varargs ABI
qualification. Exact replay of those streams is a host observation, not a portable
language guarantee. macOS shared-cache libSystem, the loader and kernel are
identified by the host boundary rather than separately hashed cache images.

Allocation/I/O fault injection, permissions and short reads beyond the authored
resources, alternate floating environments, TTY logging, concurrent calls,
OpenMP, other compilers/platforms and broad physical calibration remain unverified.
Native-invalid/corruption fixtures do not justify duplicate internal guards when
validated domain objects and dependencies already guarantee a future port's inputs.

## Original and corrected behavior

The original TEMPO2 source and oracle are unchanged. Native mutations are
sensitivity controls, including mutations of the known first-pulsar accesses;
they are not accepted corrected expectations. No scientific correction or W wet
mapping correction is incorporated into this reference. Future corrections require
the source-location/bug/fix comments, focused explicit corrected expectations and
central disposition required by 0053.

The PMRV sensitivity case deliberately increases a finite coefficient because the
nominal short-span contribution rounds away. It establishes verifier sensitivity,
not a realistic pulsar or a new precision/tolerance policy. Independent numerical
bounds are supporting diagnostics only; the ordinary replay gate remains exact.

## Measured compiler gaps

Across the 311 primary and 13 supplemental cases, the measured O2/gcov profile
enters 25/25 routines and covers 581/585 compiler lines and 388/400 compiler
branch outcomes. All 324 projected results match the ordinary O2 oracle exactly.
These are compiler outcomes, not a claim that every scientific input domain or
every source predicate combination is covered. The complete zero-count entries,
their indices and source decision mapping remain in `outputs/coverage.json`.

| Source location | Zero-count outcomes | Disposition |
| --- | --- | --- |
| `refphs.C:12` | Branch index 1 | Storage saturation specifically at the phase-capacity diagnostic was not exercised. General warning/error saturation was exercised separately. |
| `sw_delay.C:119` | Branch index 1 | Invalid eclCoord would leave automatic coordinates uninitialized; deliberately unrun. |
| `sw_delay.C:444,445` | Branch index 0 at each line | Finite mcl2 rotation-adjustment boundaries remain unhit. |
| `sw_delay.C:574` | Branch index 3 | Calendar leap-decision compiler outcome remains unhit; no impossibility claim. |
| `sw_delay.C:587,603` | Branch index 1 at each line | Invalid-month paths outside the tested date domain remain unverified. |
| `sw_delay.C:588,604` | One line at each location | Invalid-month diagnostics remain unexecuted. |
| `ifunc.C:8,42,43,53` | Indices 0, 1, 1, and 0/1 respectively | Terminal assertion cases ran, but abort does not flush gcov counters. No individual zero-count edge is claimed as covered. |
| `ifunc.C:53,54` | One line at each location | The safe NaN-query assertion ran and aborted; terminal counters are unavailable. |

This table accounts for all four zero-count lines and all twelve zero-count branch
outcomes. A defined gap is a remaining test obligation, not evidence that an
alternative branch is safe, unreachable or scientifically acceptable.
