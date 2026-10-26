# AG source/body/dispatch/type map

Raw pin fda6ad4a9753ad7dc5d291406f2f310a9161423f. This authored map freezes every defined routine before implementation; evidence disposition is completed in the report.

| Source location / routine | Canonical Rust responsibility | Types / dispatch |
| --- | --- | --- |
| T2toolkit.C:45 `TKconvertFloat1` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:52 `TKconvertFloat2` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:66 `TKfindMin_f` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:79 `TKfindMin_d` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:93 `TKsign_d` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:101 `TKfindMedian_f` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:119 `TKfindMedian_d` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:137 `TKfindRMS_f` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:149 `TKfindRMS_d` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:161 `TKfindRMSweight_d` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:179 `TKfindMax_f` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:192 `TKretMax_d` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:198 `TKretMin_d` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:204 `TKretMax_f` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:210 `TKretMin_f` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:217 `TKretMin_i` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:223 `TKfindMax_d` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:237 `TKmean_d` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:247 `TKmean_f` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:257 `TKvariance_d` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:275 `TKrange_d` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:280 `TKrange_f` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:291 `TKsort_f` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:312 `TKsort_d` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:333 `TKsort_2f` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:357 `TKsort_3d` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:388 `TKzeromean_d` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:404 `TKranDev` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:422 `TKgaussDev` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:443 `TKsetSeed` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:503 `init_genrand` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:520 `genrand_int32` | `toolkit` | source f32/f64; seed i64; MT u32 |
| T2toolkit.C:556 `genrand_real1` | `toolkit` | source f32/f64; seed i64; MT u32 |
| TKspectrum.C:85 `TKspectrum` | `spectrum` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:428 `TKfirstDifference` | `spectrum` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:446 `TK_fitSinusoids` | `spectrum` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:465 `TK_weightLS` | `cholesky_spectrum` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:522 `TK_dft` | `spectrum` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:559 `TKaveragePts` | `spectrum` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:611 `TKsortit` | `spectrum` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:650 `TKboxcar` | `spectrum` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:677 `TKhann` | `spectrum` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:848 `TKspline_interpolate` | `spectrum` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:881 `TKlomb_d` | `cholesky_spectrum` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:963 `TK_fft` | `cholesky::fft` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:1041 `TKcalcSigmaz` | `sigma_z` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:1218 `fit4` | `sigma_z` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:1345 `mat20` | `sigma_z` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:1439 `getprtj` | `sigma_z` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:1449 `getweights` | `sigma_z` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:1457 `readin` | `sigma_z` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:1597 `indexx8` | `sigma_z` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:1644 `TKinterpolateSplineSmoothFixedXPts` | `spectrum` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:1666 `calcSpectraErr` | `cholesky_spectrum` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:1685 `calcSpectraErr_complex` | `cholesky_spectrum` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| TKspectrum.C:1739 `calcSpectra` | `cholesky_spectrum` | f64; TKfirstDifference f32 scratch; TK_weightLS Rug128 sums |
| t2spectrum.C:12 `calcSpectra_ri` | `spectrum::SpectralSession` | f64; I ordinary/IFUNC, T unit binfactor, other ordinary |
| t2spectrum.C:22 `calcSpectra_ri_T` | `spectrum::SpectralSession` | f64; I ordinary/IFUNC, T unit binfactor, other ordinary |
| t2spectrum.C:71 `fitMeanSineFunc` | `spectrum::SpectralSession` | f64; I ordinary/IFUNC, T unit binfactor, other ordinary |
| t2spectrum.C:84 `fitMeanSineFunc_IFUNC` | `spectrum::SpectralSession` | f64; I ordinary/IFUNC, T unit binfactor, other ordinary |
| t2spectrum.C:150 `fitCosSineFunc` | `spectrum::SpectralSession` | f64; I ordinary/IFUNC, T unit binfactor, other ordinary |
| GWsim.C:62 `setupGW` | `gw_sim` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:159 `matrixMult` | `gw_sim` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:176 `dotProduct` | `gw_sim` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:188 `GWbackground` | `gw_sim` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:225 `calculateResidualGW` | `gw_sim` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:288 `setupPulsar_GWsim` | `gw_sim` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:300 `GWbackground_read` | `gw_file` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:334 `GWbackground_write` | `gw_file` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:351 `eccRes` | `gw_ecc` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:512 `eccResWithEnergy` | `gw_ecc` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:682 `Rs` | `gw_ecc` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:695 `dadt` | `gw_ecc` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:707 `dedt` | `gw_ecc` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:719 `dtdt` | `gw_ecc` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:732 `Fe` | `gw_ecc` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:744 `psrangle` | `gw_sim` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:768 `sphharm` | `gw_sim` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:817 `Findphi` | `gw_sim` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:844 `setupgeneralGW` | `gw_sim` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:955 `GWgeneralbackground` | `gw_sim` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:1023 `calculateResidualgeneralGW` | `gw_sim` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:1084 `GWgeneralanisotropicbackground` | `gw_sim` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:1226 `GWanisotropicbackground` | `gw_sim` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:1330 `GWdipolebackground` | `gw_sim` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:1394 `GWgeneralbackground_read` | `gw_file` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |
| GWsim.C:1425 `GWgeneralbackground_write` | `gw_file` | Rug128 source fields/geometry; f64 population angles/harmonics/eccentric; eOut f32 |

`t2toolkit_global.C::MAX_OBSN` reads canonical process `source.max_observations`.
`TK_fitSine` is explicitly removed at TKspectrum.C:425; `sineFunc` has declarations only, no body. No invented implementation.
`gwsim_Ngrid` is owned by GwSession; source default 1000 and doubling retained. Sigma-z global zero initialization and static fit history live in SigmaZ. GLOBAL_OMEGA lives in SpectralSession.

Source driver dispatch: spectrum estimator 1 DFT, 2 Lomb, 3 FFT, 4 weighted LS, 5 simultaneous sinusoids; output-kind 1 PSD, 2 amplitude, 3 power, Lomb 4 normalized power, other source raw values. Diagnostic selectors 1–6 return table with source exit status 1 at their actual stage. calcWhite=1 returns RMS before estimation. Counts remain separate.

GW file prefix order: theta_g, phi_g, omega_g, phi_polar_g, phase_g, aplus_g, aplus_im_g, across_g, across_im_g; general appends ast_g, ast_im_g, asl_g, asl_im_g, avx_g, avx_im_g, avy_g, avy_im_g. Geometry and phi_bin/theta_bin/inc_bin/dist_bin are not serialized.

Native ordinary long-double boundaries use AF PredictorNumber only where the source actually uses native long double. GW timing fields and weighted-LS sums use Rug128. Constants remain routine-specific, including tensor c=299792458 and general c=299792456.200. No unsafe boundary is planned.
