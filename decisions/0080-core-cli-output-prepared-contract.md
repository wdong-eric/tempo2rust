# Core command entry point and output APIs: prepared Assignment AH

Central design decision, 25 October 2026; delivered API and accepted interface/platform references reconciled on 26 October. Originally prepared after AG; **AH is released by decision 0091 following AG implementation handover, ownership closure and the 0082 checkpoint.** AG production `87099ca`, validation `cf4dfeb` and report `6fcf5aa` are the reviewed predecessor baseline. AE, AF and AG ownership are closed. Functional handover remains distinct from scientific acceptance.

AH owns `rust/` and new `validation/core_output/` as the sole implementation writer. Reuse the canonical input, timing, fitting, predictor and toolkit APIs delivered by earlier stages. Each complete source routine remains required even where one caller uses only a subset. This stage does not duplicate those scientific engines or add a new parameter/observation store. Policies 0053, 0039, 0050 and 0065 continue to govern small demonstrated source corrections, ignored generated outputs, evidence preservation and proportionate verification. Original 0008 remains intact.

## Usable deliverable

Add TEMPO2-style invocation to the existing `tempo2-rust` binary while retaining the existing explicit JSON subcommands and their contracts. The native-style route accepts the source core command grammar and produces the source reports/files through the same canonical engine. Freeze runnable committed inputs against the released fitting baseline; the following prepared examples must become executed recipes:

```sh
TEMPO2=/path/to/T2runtime tempo2-rust -f MODEL.par INPUT.tim -newpar -residuals
TEMPO2=/path/to/T2runtime tempo2-rust -f P0.par P0.tim -f P1.par P1.tim -dcf COVARIANCE_FILE -outpar fitted.par
TEMPO2=/path/to/T2runtime tempo2-rust -f MODEL.par INPUT.tim -pre -list
TEMPO2=/path/to/T2runtime tempo2-rust INPUT.tim
```

Also integrate source `-polyco`, `-polyco_file` and `-pred` argument forms with AF's implementation and source PAR-only synthesis path. The binary name is explicit; do not install over the user's canonical `tempo2` during development. Packaging and external-consumer qualification remain later work.

Expose safe public equivalents of the complete output/helper routines below, using canonical state and existing diagnostic/formatting infrastructure. Freeze their concrete mutable-state, stream and file-destination signatures against the released baseline. CLI file paths retain source working-directory conventions; library callers need an explicit owned I/O contract, without changing global working directory to implement it. Preserve completed source writes on failure unless a separately documented correction changes that behavior; do not imply a multi-file transaction.

## Delivered integration points

AE production `ddff7f8` / report `ecf41ed` and AF production `7420622` / report `7a94491` are reviewed implementation baselines under 0088/0089, with numerical acceptance still pending. AG production `87099ca`, validation `cf4dfeb` and report `6fcf5aa` are reviewed under 0091; read its analysis API/source map and retain 0090 warnings. Read `rust/GENERAL_FITTING_API.md`, `rust/PREDICTOR_API.md` and their source maps; reuse the canonical library calls rather than calling the JSON executable or translating its output back into internal state.

`general_fitting::fit` already owns a complete NITS loop with prefit and postfit evaluation. It does not provide the source main's interleaved output calls or `-pre` path. AH's main must use the existing BAT/residual evaluation and single `general_fitting::t2_fit` operation at the actual source points in `tempo2.C:512–624`, preserving one fit per outer iteration, prefit-slot updates, `displayParameters`, report mutations and warning state. `-pre` skips the postfit evaluation inside each outer iteration after that iteration's fit; it does not bypass the outer NITS loop. The stateful `textOutput` call occurs before the next outer iteration, so collecting all fits first and printing afterward is not equivalent. Keep the existing complete-fit JSON/library contract intact.

For `-pred`, `-polyco` and `-polyco_file`, reuse AF's canonical PAR-only preparation (`predictor::load_par`, or the already initialized/preprocessed context at the matching core stage), `predictor::cli_defaults`, `ChebyRequest` / `cheby2d_int::{construct_set,test_set}`, `PolycoRequest` / `polyco::polyco` and `T2Predictor` file operations. Do not read/preprocess the same request twice. Preserve the native argument grammar rather than inheriting the JSON adapter's option grammar. Keep source CLI defaults separate from direct-library defaults, including Cheby odd-order adjustment, seconds-to-days type boundary/overlap, polyco true-DM choice and literal file-prefix behavior. Preserve source file-write/test order and the resulting context mutations. The existing `predictor_cli.rs` is a useful integration example, not an alternate scientific engine or an internal JSON protocol.

AG's adopted weighted-spectrum warnings in 0090 are part of its public diagnostics contract. Any AH adapter that exposes those operations must retain the warnings and defined source weighting. This does not add a new spectral command to the pinned core grammar. Preserve the full inherited numerical ledger, including AF's seven raw polyco log10RMS failures; output formatting must not mask them or turn the displayed rounded values into engine acceptance.

## Source responsibilities

| Source at the pin | Required responsibility |
| --- | --- |
| `tempo2.C` | All active core entry-point orchestration: early options, aliases, runtime/environment and process settings, input/preprocessing, native predictor forms, source iteration/fit/output sequence and final diagnostics/status. Declarations without bodies and the disabled test main are not extra implementation bodies. |
| `getInputs.C` | `getInputs`, `setPlugPath`, `printplugs`: source argument grouping/defaults, file pairing/discovery, input controls, help and path/list behavior. Reuse AC preprocessing and actual resource controls. |
| `textOutput.C` | All six bodies: `textOutput`, `calcRMS`, `m2`, `printGlitch`, static `dglep_eval_phase` and `dglep`. Include every active report, derived quantity, parameter-family/coordinate/error branch and file write, not only the main parameter table. |
| `displayParameters.C` | Complete `displayParameters` and its position-dependent observations, clocks, geometry and delay output. |
| `storePrecision.C` | `recordPrecision` is an actual no-op at this pin: its proposed storage body is commented out. Preserve that contract; do not invent a precision-monitoring feature. |
| `TKlog.C` / `.h` | Complete warning/error storage and summary/reset behavior, logging macros' controls, timing and terminal/nonterminal presentation. Extend existing owned diagnostics rather than introducing another process-global logger. |
| `TKlongdouble.C` and selected/generated header | Complete defined parsing/printing and formatter responsibilities used by the core and its public equivalents, with source type/promotion and return conventions. Reuse source scanners, fixed Rug128 and safe formatting. Account explicitly for alternative native/QD configurations without claiming they were executed. |
| `extract_enums.sh`, `tempo2.h`, generated enum tables | Complete label/constraint/series string tables, ordering, marker entries and empty terminal sentinels. Reuse canonical enums from O/AE; no duplicate enum namespace. |
| `help.txt`, `help_extra.txt`, `convertxt.c`, `Makefile.am` | Account for the generated embedded help data and build-helper responsibility. Rust may embed the pinned authored text directly with an equivalent terminating-string contract; a redundant installed converter is unnecessary. Include source license/provenance and truthful feature/version information. |

Plugin programs and their binary compatibility remain outside this initial migration scope. The core path discovery, selector handling and wrapper dispositions still require explicit coverage. Recognize external plugin selections and report an unavailable integration when the installed Rust build cannot provide it; never silently substitute the built-in fitter or output handler. Do not claim that a discovery listing proves a loadable plugin ABI. Existing bounded source-disabled routes remain distinct from an enabled backend.

The selected installed interface guarantee is full Rust core/toolkit APIs plus standalone predictor C compatibility under 0084; AJ qualifies its external consumers and C headers/ABI. Enabled CALCEPH and optional accelerated backends, the remaining SOFA dependency disposition, clean installation, broader workflows, actual macOS arm64 and Linux x86_64 qualification under 0086, performance and concurrency remain explicit full-migration obligations. AH completion cannot remove those obligations or resolve inherited numerical failures by omission.

## Source conventions that affect the result

- The native route initializes controls, reads PAR/TIM and preprocesses in source order, then follows `psr[0].nits`. At subsequent NITS iterations it copies postfit values/errors into prefit slots. Each ordinary iteration performs prefit BAT/residual evaluation, fitting, then postfit BAT/residual evaluation and output. Reuse AE's canonical fit lifecycle; avoid fitting twice when adding the wrapper.
- **`-pre` still performs the fit at iteration zero.** It prints at that point and skips the postfit recomputation. Do not reinterpret it as `timing` without fitting. Likewise the source `-nofit` preprocessing control still permits the source mean/offset behavior; do not derive meaning from the flag's name alone.
- `-writetim` writes `out.tim` immediately after preprocessing in the inspected main, despite help text saying after fitting. Follow actual code and explain the distinction. The source `-model` flag is stored but its legacy fit call is commented out in this main; trace actual consumers before activating a new behavior. Preserve source-defined no-ops and unsupported routes explicitly.
- `textOutput` is stateful. It calls `calcRMS`, updates RMS/TRES/TRESTN fields and chooses a nondeleted TOA nearest the START/FINISH midpoint to rewrite TZRMJD/TZRFRQ/TZRSITE. Preserve selection/ties, writes and repeat-call effects. `calcRMS` returns the sum of weights while writing RMS values; do not change its return into an RMS scalar. Its SAT/BAT selectors, source delta, weighting, casts and units remain exact source choices.
- Reports/PAR output preserve parameter order, labels, index families, fit flags, linked SINI/KIN representation, equatorial/ecliptic coordinates and uncertainty conversions, binary/time/unit selectors, jumps/FD jumps, waves/interpolation/constraints/noise and other active source records. Derived astrophysical/glitch values use the pinned formulas/constants, actual f32/f64/Rug boundaries and solver fallbacks. In particular retain the current bracketed/damped `dglep` algorithm and the `m2` iteration/sentinel behavior; do not substitute a remembered older implementation.
- Preserve all source output files and their membership, order and naming: `residuals.dat`, pulsar `.dm`/`.cm` files, `clockOffset.dat`, `ifunc.dat`, `telescopeXYZ.dat`, `aplus_t2.dat`, `across_t2.dat`, covariance outputs and fitted PAR files, plus the main's optional command history and existing fit/predictor files. Identify actual units and selected rows for each. Fixed filenames can be overwritten by later pulsars; multi-pulsar PAR filenames have their own suffix rule. Do not silently invent names or change record membership.
- Explicit output requests can write files even when the main newpar flag is false; exercise the actual source branches. The direct `textOutput` empty-filename prompt is a defined interface: expose a supplied input stream/response or an explicit adapter outcome instead of inventing a filename or unexpectedly blocking a noninteractive library caller.
- Source `parse_longdouble` uses its selected native conversion, including prefix/nonconversion and nonfinite behavior. This is distinct from the bounded Rust `Scalar` validator. Reuse the canonical source scanner for the source API and retain strict bounded adapters. Formatting is an explicit boundary; do not route extended values through binary64 or change production precision to mimic file text. Preserve width/precision/sign/exponent, signed classes and the source formatter's successful return value where it is defined.
- Logging retains quiet/debug/tcheck distinctions, first-30 stored messages and counters beyond that limit, truncation-before-ANSI-removal order, warning/error summary order and reset behavior. `_TKchklog` tests the configured LOG_OUTFILE terminal state, which need not equal its destination argument. Qualify terminal and redirected streams explicitly. CPU-time/version/path fields need controlled or narrowly identified comparison treatment; scientific numbers may not be masked as incidental output.
- Preserve source statuses as well as text: the inspected help/version and no-argument routes exit with status 1. Package/version identity must truthfully identify the Rust implementation and native pin; document that presentation difference rather than claiming to be the original binary.
- Preserve actual argument case rules, ordering, repeated controls, alias expansion, runtime/clock-path precedence and directory pairing semantics. Use byte-preserving paths and canonical scanners. Owned safe directory/argument handling can replace unsafe C storage or shell plumbing while retaining the defined contract; explain any changed filename/error behavior with a concrete 0053 witness.

## Focused source questions

These are bounded implementation questions, not approval of a new scientific convention. The owner should resolve demonstrated small defects under 0053, with adjacent pin/file/routine/location/cause/fix comments, original/corrected evidence and valid neighbors.

1. `calcRMS` tests current-pulsar START/FINISH flags but reads `psr->param` values for those cutoffs. Use distinct two-pulsar selection ranges to determine the effect and correct current-pulsar ownership if demonstrated. Inspect the analogous `displayParameters` first-file labels without treating every pulsar-zero access as erroneous.
2. Argument/alias paths use unchecked following arguments, fixed buffers and potentially missing environment/tokens; `-allParTim` uses shell text. Demonstrate missing/malformed/cyclic/large input behavior, preserve valid ordering, and return bounded recoverable errors instead of importing undefined reads or shell-dependent filename corruption. Do not add restrictions to already validated internal state without a real caller need.
3. `TKlongdouble.C::ld_vsprintf` ends an uninitialized `oargs`, ends passed varargs also ended by wrappers, has fixed buffer assumptions and treats selected integer lengths through actual C vararg types. Separate undefined variadic calls from defined formatting. Use a narrowly corrected native formatter or direct typed quadmath witness where needed; do not promote undefined original bytes into a golden output or add broad unsafe Rust to reproduce them.
4. Output loops and covariance/auxiliary-array branches may consume absent or mismatched state; I/O failure handling and fixed filenames can preserve only a successful prefix. Establish each actual producer and valid extent. Preserve defined NaN/zero/sentinel outcomes, and never synthesize missing fitted/noise/geometry data solely to make a report print.

## Verification and handover

Start from existing AC/AD/AE/AF/AG examples and source evidence. Implement the native-style route and output routines with focused direct/native probes as needed; no new reference-only prerequisite is introduced. Freeze the public examples, state/output projection and new derived-quantity budgets before comparing Rust results. Keep all inherited gates and correction identities.

Require both identical-state output/helper comparisons and complete CLI executions from real files: isolated and binary fits; multiple pulsars/global covariance; pre-only and repeated NITS; supported site/frame/time choices; constrained/robust/noise reports; parameter/uncertainty/link families; glitches/derived quantities; source file writes and fitted-PAR reload; predictor options; argument/help/runtime/path controls; direct repeated output and failure followed by valid work. Full source/body/branch disposition accompanies the representative public matrix. Use existing pipeline tests for unchanged scientific kernels rather than rerunning every historical suite merely for formatting edits.

Check complete ordered file/record/field membership, names, flags, units, classes and mutation state. Compare deterministic output text/bytes exactly on identical representable inputs. For end-to-end numerical text, retain the raw bytes and use an explicit per-field format/quantity contract: declared display quantization is separate from internal timing/solver accuracy. PAR round-trip checks must account for the source's printed precision, and must not silently widen engine gates. Do not delete all numbers or diagnostic lines with blanket normalizers. Prove rejection of omitted records, reordered identities, changed units/flags/digits and missing files. Only named nondeterministic execution metadata may be normalized, with its raw evidence retained.

Use existing 1e-12 s timing, 1e-10 turn phase and fitting/covariance budgets where applicable, with exact discrete/class checks. Additional mass/glitch/display quantities need explicit units, source types, reference scales and structural-zero rules before comparison. Different physics, precision, tolerance policy or broad output semantics remain central decisions. Matching TEMPO2 output is implementation correspondence, not independent astrophysical validation.

Run affected debug/release workflow/component checks, native file exchange and input/output recovery, formatting and Clippy. Record actual source/input/build/execution identities and unrun configurations. Deliver authored Conventional Commits and `rust/CORE_OUTPUT_REPORT.md`, concrete command recipes, complete source/type/unsafe/departure maps, original/corrected comparison results and remaining inventory. Generated JSON/text dumps/binaries stay ignored. Under 0074, a reviewed functional handover may proceed with localized open numerical decisions, but AH or full migration cannot be claimed accepted without the applicable gates.

## Preparation evidence

Central inspected core main/getInputs orchestration, output mutation/helpers/file paths, logging/precision interfaces, current Rust counterparts and generated-source inputs. This is preparation, not a claim to have independently reviewed every derived expression or executed the new Rust capability. A bounded run of the pinned `extract_enums.sh` against the pinned header produced 166 label, 68 constraint and three series entries, each followed by the source empty sentinel; these counts include the source marker entries where present. This checks generated table membership only.

The enum witness is preserved under `decisions/evidence/core-output/build/preparation-enums-20261025/`; its receipt SHA-256 is `3d6e58a96a361fd5c23424ef713fb060f58710a4f126354ca290e6dbfaadd050`. It came from `/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-output-enums-vw4e7lse` and is ignored. Raw source identities for the contract are:

| Source | SHA-256 |
| --- | --- |
| `tempo2.C` | `aa8df00d26c914dd0c56aaf7ebe15ddd415cc8c18c839b92b259c9b3da0a33ad` |
| `getInputs.C` | `c59797f69cc09696869fa712d1537c5ea9fa1be08da08936ce86861aecc05d28` |
| `textOutput.C` | `52b7859d0015154cf5de0a096b0e271fb06a6e1d6e54ce052b413d143e52d607` |
| `displayParameters.C` | `c237b06d4deeab01792dc4ea53186c4ed9279ce9860afef97c5b3eb64a3a0466` |
| `storePrecision.C` | `039e08795529ca8eb0c8244c1cefd2030dc6ce1bd60abb4fd701c20b25401aa8` |
| `TKlog.C` | `696904f67674dcfa51a485b0943d10bd39358daeb14649fe4c5d728d667f1d90` |
| `TKlog.h` | `977dc204af66a0e61fe3b6288c630e3a3343ebc9dc08d5e1f2e535b5ea518e3c` |
| `TKlongdouble.C` | `4e8da8ed6c9fd6d60a671e330f09f975e32caba7aacd0c522e8c9a5a0d75a05c` |
| `TKlongdouble.float128.h` | `214911b81f82489996d698626cf8d30ab5bcc20099f6b5c5aa089715b85bc55e` |
| `TKlongdouble.ld.h` | `b9acbfa5a68e3427f83ed3a8ec9551d9fb202d8b73502be219205ad1269a713d` |
| `extract_enums.sh` | `17db2aa391dedf85e68c2e016c21d2d9be7ecb887b21e197c2319eb47fad0e5f` |
| `Makefile.am` | `172ec480482f0261f0a845e9aface382c1a441f4e80d53132574bf8ff912bd7a` |
| `tempo2.h` | `0313b6eaf9f9ae9d7e5cce340379ad4b7068394e5222a01f73949e3a2640de93` |
| `help.txt` | `8201d9edf9571d8bb20812611bdff86eea314f77f10d34138916435cf2efd57d` |
| `help_extra.txt` | `b51c24f0b5a544cea470384566343ced1470a2a00da93a521f2b3dee0f1ae27e` |
| `convertxt.c` | `03ce58b61ba0276f783e7b0d9cb4f2f0d3bd8d4538cd989511a56383c3c7b71f` |
