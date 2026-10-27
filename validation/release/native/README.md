# Reviewed core correction construction

`core-corrections.patch` consolidates the **executed source edits**, including
observer hooks, used by AI's final combined-release native profile. It applies
to raw canonical blobs at `fda6ad4a9753ad7dc5d291406f2f310a9161423f`.
It is a build input, not a new scientific correction or a numerical golden.

Every row in `provenance.tsv` was checked against its owning immutable delivery
receipt and compiled-object digest before extraction. The table records raw
and executed source hashes and the predecessor receipt identity. The reviewed
AC/AD/AE/AF/AH/AI source/cause/fix descriptions remain normative. Empty observer
hooks are supplied verbatim; they are not a production timing implementation.

The portable builder reconstitutes and hash-checks these sources, then applies
and records only its target ABI declaration adaptation. It rebuilds every object
on the actual host and links against the newly built A2 archive. It never reuses
a macOS object on Linux. Original A2 and every predecessor runner/output remain
unchanged. AG analysis and AK instrumented worker profiles have their own
additional construction and are not implied by this core/provider overlay.
