# M1b contract and Assignment D release

Central decision, 20 October 2026, incorporating the user's instruction to follow existing TEMPO2 decisions unless they are wrong or prone to errors, and to explain departures in comments beside the changed code. **The contract below is adopted and Assignment D is released for implementation. M1b acceptance remains pending D's evidence.** This supersedes the earlier strict-meteorology proposal and satisfies the release conditions in decision 0003.

## Decisions adopted

| Decision | Adopted choice | Reason |
| --- | --- | --- |
| Fixture and scientific scope | Adopt A2's `terrestrial` and `terrestrial_4ns` as compatibility fixtures for the isolated PKS path, with their exact resource identities and active corrections | The corrected exports are stable and already expose the required intermediate quantities |
| Missing meteorology | Preserve TEMPO2's default `101.325 kPa` pressure and zero zenith wet-delay substitutions, including the source's zero-value behavior; report warnings and effective values | The user directs compatibility with existing decisions; no evidence here establishes that this declared approximation is a software error |
| Retained numerical code | Retain the pinned bundled SOFA routines and low-level JPL reader/interpolator behind narrow owned interfaces; port the TEMPO2 timing pipeline and IF99 reader/state into Rust | Preserves the reference's numerical algorithms while removing timing-core global state and process exits |
| Comparison gates | Adopt the existing proposed terrestrial stage budgets, including `1e-12 s` delay/epoch/residual and `1e-10 turns` phase gates, with independent exact-count checks | Makes the target explicit before implementation; discrepancies require diagnosis rather than automatic relaxation |

Missing weather alone must not become a new fatal error or require a new opt-in mode. The fallback remains a declared model assumption, not a claim of historical weather accuracy. Disabling the troposphere or requiring measured weather would change the adopted behavior.

## Compatibility and documented departures

Follow the pinned TEMPO2 algorithms, constants, numerical boundaries, default choices, resource-selection behavior and supported fallbacks. A preference for a cleaner design or stricter policy alone is not evidence that an existing scientific decision is wrong.

When a departure addresses a demonstrated error or concrete failure risk, put an explanatory comment at the changed behavior. State the original TEMPO2 behavior/source routine, the problem it causes, and the intended difference; reference the regression or central decision where useful. Record the departure and its evidence in the implementation report as well. A report alone does not replace the local comment.

The already adopted checked-overflow, initialized-state, owned-state and recoverable-library-error contracts remain in force. Replacing a process exit with a returned error, or preventing invalid memory access, is authorized within those contracts; explain the changed behavior locally and test it. Other changes to scientific values, conventions, defaults or accepted output require a concrete reproducer and central review before adoption. Continue unaffected implementation while such a choice is unresolved.

## Pinned scope and evidence

The starting implementation is M1a commit `49e122c3648e5cb91bae17233a062a444a10f9e4`, accepted in [decision 0003](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0003-m1a-acceptance.md). The reference remains A2 commit `7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572`, with the source/correction identity in [decision 0002](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0002-m0-acceptance-and-c-release.md).

| Accepted compatibility fixture | SHA-256 |
| --- | --- |
| `terrestrial` | `c19aac714e38521c5dd309dbf50f815f49430171430206d7af969fc790043d83` |
| `terrestrial_4ns` | `5455b5dc4d387ec8bea42a43665dffe0b3d7a39ced47e0f131395d003adb62eb` |

These hashes were rechecked during this planning review. The [A2 report](/Users/Eric.Dong/Downloads/tempo2rust/validation/reference/REPORT.md:144), [terrestrial export](/Users/Eric.Dong/Downloads/tempo2rust/validation/reference/outputs/terrestrial.json) and [schema](/Users/Eric.Dong/Downloads/tempo2rust/validation/reference/schema.json) supply the configuration, resource hashes, operational coverage, stage fields and proposed budgets. This review inspected saved data and source; it did not rerun the reference.

Use `test1.par` with `test2.tim`: seven ordered PKS observations, fixed isolated model, DE405, IF99, TT(TAI), TCB model/BAT/BBAT, IAU2000B, frequency dilation, solar and planetary Shapiro delays, interstellar dispersion, IPM=1, NE_SW=4, SWM=0 and troposphere enabled. SAT remains observatory-clock time. Preserve the first eligible TOA's fractional phase reference, no fitting, no sorting and no mean subtraction. The omitted UNITS/EPHVER=5 convention remains the one already adopted.

Use the pinned observatory, clock, EOP, DE405 and IF99 data identified in the exports. Record and compare each TOA's applied clock chain in order; the chain can differ across epochs. Distinguish used resources from files merely scanned by the reference. Resolve resources from an explicit runtime root and check coverage, including shifted diagnostic epochs. Follow the source's declared fallback and warning behavior where the supported path defines it; do not invent replacement resources or a blanket strict policy. Missing indispensable resources, malformed data, or native errors must produce contextual recoverable failures instead of process exits or invalid access. Document any intentional behavioral difference beside its implementation.

Initial terrestrial support is the declared PKS isolated configuration. Existing M1a F0–F3 arithmetic remains available. Additional observatories, alternative time conventions, proper motion/parallax models, binary timing, fitting, and the bundled example are separate coverage work unless required by the accepted fixtures and reviewed centrally.

## Meteorology semantics

The supplied PKS tables cover only MJD 53000–53001. The fixture epochs are around MJD 50000, 54000 and 56000. [The upstream implementation](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/tropo.C:317) substitutes zero wet delay and `101.325 kPa` pressure when no value is returned.

The adopted Rust contract follows TEMPO2:

- Use the source's table selection, interpolation and coverage conventions when applicable data exist.
- Where the source returns no applicable value, including missing weather files or out-of-range epochs, use `101.325 kPa` surface pressure and zero zenith wet delay without requiring caller opt-in.
- Preserve the source's zero-value behavior: a zero pressure lookup selects standard pressure, and zero wet delay remains zero with the source-equivalent warning semantics. Do not silently reinterpret this rule as part of the port.
- Record effective values and the reason for each substitution, and expose source-equivalent warnings without corrupting JSON stdout. Exact legacy warning prose or duplicate suppression is not a numerical compatibility requirement.
- Use one normal tropospheric calculation path. No strict-weather mode or constant-atmosphere switch is required for M1b.

The two goldens validate compatibility under the source's atmospheric assumptions. They do not validate historical weather. Focused synthetic resource tests must separately exercise valid table lookup, zero values, missing/out-of-range weather with successful fallback, and fatal resource errors followed by valid processing. If a concrete case demonstrates that a source decision is wrong or error-prone, return that evidence centrally and comment any approved correction at the changed code.

## Dependency and interface boundary

The pinned [SOFA calls](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/get_obsCoord.C:42) and [JPL API](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/jpleph.h:52) are the retained native numerical dependencies, in addition to the already accepted Rug/GMP/MPFR stack. Copy and identify only the necessary source/dependency closure, preserve licenses and source hashes, and wrap it with owned lifetime/error handling. The normal build must not depend on the independent TEMPO2 checkout or a live reference executable.

Implement clock/resource handling, IF99 loading/interpolation, time transformations, geometry, delay assembly, BAT/BBAT and residual orchestration in Rust. In particular, the current [IF99 reader](/Users/Eric.Dong/Downloads/tempo2rust/tempo2/ifteph.C:66) has global state and process-exit failure paths; linking it unchanged would violate the library recovery contract. Preserve its interpolation arithmetic and constants when porting. Comment the replacement of shared state and process exits where the new ownership and error handling are implemented. Any broader native dependency requires a central design decision.

Keep the accepted barycentric operation usable without resources. Add one explicit runtime context and terrestrial evaluation operation; keep source-corresponding internal names. Extend the existing residuals CLI with an explicit runtime-resource input; meteorology follows the selected runtime and source defaults. Exact Rust names and option spelling can be routine implementation choices, provided the existing M1a API/output remains compatible and new provenance is truthful.

## Comparison contract to implement in D's test tree

Keep A2 artifacts and the copied M1a contract unchanged. Add a separately declared terrestrial projection and comparator in D's owned test tree, pinned to the two hashes above. The existing `barycentric-v1` comparator intentionally refuses terrestrial data; do not bypass it by relabeling candidate exports.

Compare exact configuration, parameter presence/fit flags, original TOA identity/order/tokens, sites, correction flags, applied clock destinations, and full/reduced pulse counts. Project parameters and known inactive zero terms consistently with M1a. Compare the declared fields at `parsed`, `bats`, `residuals`, and the `offset_4ns`/`slope_minus`/`slope_plus` diagnostic stages, using only quantities whose lifecycle defines them. Original observation identity stays unchanged when the adapter evaluates shifted epochs.

| Quantity | Adopted absolute gate |
| --- | ---: |
| SAT/BAT/BBAT and combined BAT correction, differences converted to seconds | `1e-12 s` |
| Clock steps/total, TT–TB, TT–Teph, Roemer, each stored Shapiro term, troposphere, interstellar/solar dispersion, other declared delays and residual | `1e-12 s` |
| Each Earth/SSB and observatory/Earth vector component | `1e-12 light-second` |
| Full phase | `1e-10 turns` |
| SSB observing frequency | `1e-5 Hz` |
| Input observing frequency | `1e-9 MHz` |
| Input uncertainty | `1e-12 microsecond` |
| Parsed extended parameter | `1e-30` relative; expected zero exact |

These are project compatibility budgets for the pinned cases, distinct from upstream regression thresholds. Preserve original upstream assertions wherever those tests are run; do not silently replace their units or thresholds. Explain any test adaptation locally. Delay, phase, residual and exact-count gates remain independent. Report signed and maximum differences with failing TOA/stage identifiers. If a gate fails, localize the cause before requesting any budget change.

For the 4 ns diagnostic, compare the BAT response to `4e-9 s` times the local BAT derivative estimated from the ±1 ms evaluations, with a separate `1e-12 s` gate. Do not assume a unit derivative. A2's recorded maximum diagnostic discrepancy is approximately `1.41688e-13 s`; this does not establish the future Rust port's error.

The terrestrial comparator must reject altered configuration/resources, missing stages, count changes and perturbations beyond the declared gates. Required resource hashes/clock selection are part of scientific identity; engine names, local paths, warning text, unused file scans and allocation diagnostics are not cross-engine equality requirements. Rust must still report its own warnings and provenance accurately.

## Release and required return

Assignment D may now begin in a separate implementation chat under the [updated handoff](/Users/Eric.Dong/Downloads/tempo2rust/handoffs/04-observatory-core.md). D owns the Rust crate and its new tests/report; the central chat retains design changes and milestone acceptance. Preserve the existing staged planning changes and both independent reference repositories.

D implements resource/context handling, clock and IF99 transformations, ephemeris/site geometry, propagation delays and final timing in dependency order, comparing stages as they become available. Preserve the source's initial and refined time/ephemeris passes.

D returns `M1B_REPORT.md`, runtime/resource provenance, stage errors, retained dependency identity, failure/recovery checks, unchanged passing M1a regressions and a list of intentional departures with evidence, local comment locations and tests.

The DDH discrepancy, Linux execution, packaging and performance remain outside this release. This decision authorizes implementation; it does not accept M1b or claim untested scientific coverage.
