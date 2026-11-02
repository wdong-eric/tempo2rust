# Internal event NaN probe checkpoint (0119/0121)

2 November 2026. A bounded source/production probe is ready for actual Linux
execution. No production arithmetic, precision, return transport or acceptance
gate changed. The thirteen Linux callback/Gaussian sign failures remain open.
Under 0121 this diagnostic is delivered alongside the completed Q/comparator
checkpoint instead of delaying their review.

`event_profile.py --nan-trace` retains the existing profile and appends runtime
ordinary/binary128 multiplication and division controls, assignment controls,
and source-expression prefixes for only `zero_amplitude`,
`negative_zero_amplitude`, `zero_width` and `zero_width_onset`. It uses the same
native A2 library and actual Rust callback path. Independent prefix expressions
must also reproduce each engine's actual callback class/sign. The native
arithmetic operands are supplied at runtime and observed across no-IPA helpers;
no host sign is inferred from a precision or architecture label.

The intended source localization distinguishes:

- Gaussian amplitude `val/amp`: a newly generated `0/0` result.
- Exponential width/epoch callbacks: wide products followed by division by zero.
- Gaussian epoch/width callbacks at zero width: ordinary zero-times-infinity.
- Gaussian onset at zero width: first invalid division, subsequent NaN division,
  exp, wide multiplication, assignment and callback propagation. An intermediate
  may already have the right sign and lose it in a later operation.

These are the operations the Linux probe will observe, not a claim that their
first differing Linux step has already been measured.

## Actual macOS result

The new receipt contains 588 observations. All **249 original profile rows are
unchanged**, including the forty direct library/transport controls. All sixteen
assignment controls and all 67 event-prefix observations match. There are 60
new arithmetic-control differences among the 256 multiplication/division rows:
56 involve negative operand-NaN propagation; four are wide invalid divisions
with opposite operand signs. They are retained failures, not normalized away.
This shows why a blanket NaN-sign rule would be unjustified.

All 44 within-engine comparisons between traced returns and actual callbacks
pass. All 76 actual ordinary-to-binary64 return observations remain bound.
The source snippets do not replace production calculations. Formatting and
all-target Clippy with denied warnings pass. No unchanged W campaign ran.

The first native probe failed after reusing C++ `main` under another name: its
implicit return applied only while it was named `main`. Adding explicit
`return 0` preserves the original profile and makes that reuse defined. The
failed binary/receipt and original source are retained; this was a probe-harness
failure, not a demonstrated native event failure.

Evidence is in `validation/release/build/event-nan-0119/receipt.json` and
`/private/tmp/al119-event-nan-v3/receipt.json`, including source and executable
hashes, source-library identities, neutral original outputs and the retained
thirteen Linux failures. The probe intentionally returns failure while its new
exact-sign controls disagree.

For central's actual Linux execution, use the existing configured native
compiler environment and fresh source-bound A2/export/target:

```sh
python3 validation/release/event_profile.py \
  --out /path/to/new/event-nan-profile \
  --a2 /path/to/fresh/a2 \
  --candidate /path/to/export/source \
  --target /path/to/target --nan-trace
```

Publication, dispatch and acceptance remain central-owned. A source-local sign
correction follows that measured Linux result; no general NaN policy or finite
operation rewrite is proposed by this checkpoint.
