# Familiar TEMPO2 workflows and human-readable terminal results

Central decision, 31 October 2026, recording the user's direct instruction.
This amends the working principles and active migration goal. AL remains the
sole implementation owner; central owns design and acceptance. The running
Linux candidate `a35104a` and run `41069548581` remain immutable.

## Working principle and goal amendment

**Reduce the effort and cognitive load of moving from TEMPO2 to the Rust
implementation. Preserve familiar commands, terminology, parameter conventions,
units, result layouts and workflow behavior wherever practical. Improve memory safety,
reliability, usability and measured efficiency without unnecessary relearning.**
Apply this principle across the project, including help, examples, diagnostics,
installed interfaces and everyday result presentation. Keep the Rust identity
clear. Source fidelity and evidenced improvements still follow 0053/0099;
changes to scientific meaning, precision or numerical gates need their own
central decision.

Human-facing terminal workflows should present results in TEMPO2's familiar
text style. Structured JSON remains useful as an explicitly identified machine
interface for scripts and validation. Selecting a display format must not change
whether fitting occurs, model/state updates, mean removal, preprocessing,
iteration order, numerical precision, scientific values or source-defined files.
Do not switch formats implicitly based on whether stdout is a terminal or pipe.

## Existing capability and the small first correction

At reviewed candidate `a35104a`, `rust/src/main.rs` routes native root options
such as `-f model.par arrivals.tim` through `core_cli::run`, which already calls
`core_output::text_output`. These implement the pinned `tempo2.C`, `getInputs.C`
and `textOutput.C` entry points. The older `bats`, `timing`, `residuals`, `fit`
and `predictor` adapters are explicit JSON routes under 0080/AH.

Discoverability is misleading: `--help` currently lists only those JSON routes,
while native `-h` uses the native help. `rust/README.md` also introduces the
program as a JSON CLI without leading with the delivered native text interface.
Correct that primary presentation first; reuse the existing native route and
formatter instead of adding another report engine. This preserves the explicit
machine-interface contracts already used by validation and scripts.

The user clarified that the observed program was an unspecified executable under
`target/*`; the old invocation is unavailable. This does not establish that the
native root route itself emits JSON. Resolve the discoverability/demonstration
problem without guessing which binary was used. When presenting runnable product
capabilities, lead with the main native-style CLI and a human-readable result;
clearly label diagnostic/example executables and machine interfaces. This applies
to implementation documentation and central-chat demonstrations alike.

A native-root fit command is not an interchangeable rendering of every JSON
command: `timing` without `--fit` only evaluates, bounded `fit` has a narrower
contract, and native `-pre` still fits. Do not rewrite an invocation onto another
pipeline merely to obtain text. If a particular human-facing adapter needs its
own text presentation, retain its calculation semantics and use the existing
source formatter and established units. Do not invent fitted quantities or
synthesize missing state to call the stateful report. Keep machine output
explicit and update owned consumers together for any changed interface.

## AL follow-through after the current run review

1. Make native-style usage the first help/README/installed-example path, including
   the runtime setup and a real `-f` example. Label structured adapters as machine
   interfaces and explain their different calculation scopes. Preserve established
   native short-help/version exits; the existing Rust `--help` success status can
   remain. Reuse native help rather than maintaining a second option catalogue.
2. Use the clarified entry-point context above; recovering the old invocation is
   not a prerequisite. Use the existing source-style route where its semantics
   match; otherwise implement the smallest presentation change on that same
   calculation path. Keep source names, units,
   order and displayed precision, using existing output routines. Document the
   counterpart and intentional presentation difference beside changed code.
3. Check the affected terminal/help path and structured output contract with
   representative controls. Preserve JSON schemas and scientific products where
   the machine interface is retained; verify any migrated consumer explicitly.
   Existing identical-state output comparisons and raw failing groups remain
   applicable. Do not normalize away numerical differences to make text match.
4. Return a scoped Conventional Commit, concrete before/after command examples,
   verification and any remaining interface gap. Generated output stays ignored.
   A presentation-only change needs focused checks, not an unrelated scientific
   campaign. Central reviews it with the next candidate before publication.

This instruction does not close AH's remaining output comparisons or accept
AL/M2/full migration. Keep the AL-first sequence and deferred covariance
optimization. The current Linux result is reviewed before releasing edits to
its frozen implementation; no running job is restarted for this amendment.


## Executed native-style application example

Central ran the actual installed `second-followthrough-install-v1/bin/tempo2-rust`
from tested production `5c8c0aa`; its main source is byte-identical to the reviewed
`a35104a` candidate. The command used the existing `robust-H.par` and
`multi-frequency.tim` inputs, the recorded A2 runtime, and native options
`-npsr 1 -nobs 100 -noaccel -f PAR TIM`. It exited zero with 3,780 bytes of text
stdout and empty stderr. The output includes the PSR result heading, pre/post-fit
RMS and chi-square, the native parameter/uncertainty/fit-flag table, derived
quantities and timing conventions. It emits no JSON.

This is an application demonstration, not a fresh native/Rust scientific
comparison. The receipt records the exact executable, input/runtime identities,
argv, environment and original temporary working directory. It and the complete
stdout/stderr are preserved byte-for-byte in ignored
`decisions/evidence/release-qualification/build/native-cli-demo-20261031/`;
`copied-artifacts.json` retains original/saved addresses. Receipt SHA-256:
`4b93f968c45e0c761c89d1d9f8b069e74293ab300a86f7b47644934dfa853e33`.
Use this existing working path for the primary terminal example; the established
output-comparison failures remain open.
