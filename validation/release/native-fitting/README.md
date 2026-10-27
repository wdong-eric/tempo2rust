# AE fitted-state observer construction

This patch reconstitutes the twenty compiled source inputs named by AE's native
fit receipt, with its original corrections and observer hooks. The full original
receipt digest, raw Git source digest, historical source digest and normalized
staging digest are recorded per row. Object identities were verified before
extracting the construction. LF normalization affects mixed source line endings,
not tokens; both historical and staged identities are explicit.

The observer's source is the unchanged committed
`validation/general_fitting/native_fit.cc`. The native A2 archive is rebuilt on
the actual target. This is the AE comparison profile, separately identified from
the later CLI/provider corrections. Original evidence is never rewritten.
