# M2 central source probe

This diagnostic supports [decision 0007](/Users/Eric.Dong/Downloads/tempo2rust/decisions/0007-m2-fitting-contract-and-e-release.md). It calls the accepted A2 static library rather than implementing a solver. It exercises `TKleastSquares` and the spin derivative/update callbacks. It is not an end-to-end timing fit or evidence that Rust M2 has passed.

`source_probe.cc` contains the complete deterministic inputs. `result.json` records the outputs. `manifest.json` records the source/library/build-manifest/result hashes and the host/compiler identity. The static-library hash is checked against A2's accepted `build_manifest.json`; the independent reference repository is unchanged.

The reference uses a 113-bit `longdouble` abstraction but this host's native C++ `long double` has 53 significand bits. Spin columns explicitly use native `long double`; `TKsign_d` is also binary64. The probe records these type facts and actual source callback results.

The weighted six-row/two-column case has chi-square approximately `22.89105527654461`. Rescaling multiplies its errors by `sqrt(chi_square / 4)` and does not change covariance or the fitted increments. Identical columns yield a finite source pseudoinverse. Separate diagonal matrices place one singular value at `1e-28` or `1e-26`, on opposite sides of the source cutoff. An additional diagonal case uses the same binary64 `1e-27` value as the tolerance, demonstrating retention at equality. These are solver diagnostics, not artificial timing observations.

The two-row/two-column exact fit has zero degrees of freedom. Rescaling produces nonfinite errors; without rescaling the solution, covariance and errors are finite. The diagnostic JSON encodes these nonfinite error values as `null` beside explicit finiteness flags; this is evidence transport only. The original decision 0007 proposed an error for this case. Decision 0008 supersedes that proposal: the port preserves the source's nonfinite outcome and explicitly represents its class; this saved probe remains historical evidence with its original null-plus-flag transport.

The spin probe deliberately supplies large increments to make sequential F0 use visible. It does not recompute timing residuals or claim those updates represent a useful pulsar fit.

## Reproduction

Run from `/Users/Eric.Dong/Downloads/tempo2rust`, with the accepted A2 build available. All generated executables/results go into a fresh temporary directory; do not replace the saved evidence while merely checking it.

```sh
probe_dir="$(mktemp -d /tmp/tempo2-m2-probe.XXXXXX)"
/opt/homebrew/bin/g++-16 -O2 -std=gnu++11 -DHAVE_CONFIG_H \
  -Ivalidation/reference/build/source -Ivalidation/reference \
  decisions/evidence/m2/source_probe.cc \
  validation/reference/build/libtempo2-reference.a \
  -lquadmath -lgfortran -o "$probe_dir/source_probe"
TEMPO2="$PWD/tempo2/T2runtime" \
  "$probe_dir/source_probe" "$probe_dir/result.json" >"$probe_dir/run.log" 2>&1
cmp decisions/evidence/m2/result.json "$probe_dir/result.json"
```

The existing static library contains its original compilation paths, so its diagnostic logs still mention the old `tempo2-rust` directory spelling. Those strings do not indicate that the old directory was used for this run.

The central review additionally checked that the rescaled and unscaled covariance/solutions match exactly; errors follow the source multiplier; the dependent-column solution and cutoff cases have the expected shape; and the square-fit pair distinguishes nonfinite rescaled errors from finite unscaled errors. No new Rust tests are justified by these planning-only files; Assignment E must implement and verify the corresponding Rust contracts.
