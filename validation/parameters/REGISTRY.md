# Parameter registry

Raw canonical initialization: 166 enum entries (162 physical parameters, `param_LAST`, three virtual values), 2,000 allocated slots, 9,616 parameter cells. `registry.json` records every allocated slot/index and label, including empty labels. `observations.json` retains the independent allocate-only snapshots for both unused argument values.

`longdouble` is GCC `__float128` (113 significand bits); `val`, `err`, `prefit`, `prefitErr` are arrays of that type. `aSize`, fit/set flags and link indices/counts are `int`; labels are allocated character strings. `prefit` and `prefitErr` are **unwritten** by canonical initialization. Inactive link entries and the observations/covariance payload are not read as goldens.

| Enum index | Name | Array size | First label | Last label |
| --- | --- | --- | --- | --- |
| 0 | `param_raj` | 2 | RAJ (rad) |  |
| 1 | `param_decj` | 2 | DECJ (rad) |  |
| 2 | `param_f` | 13 | F0 (s^-1) | F12 (s^-13) |
| 3 | `param_pepoch` | 1 | PEPOCH (MJD) | PEPOCH (MJD) |
| 4 | `param_posepoch` | 1 | POSEPOCH (MJD) | POSEPOCH (MJD) |
| 5 | `param_dmepoch` | 1 | DMEPOCH (MJD) | DMEPOCH (MJD) |
| 6 | `param_dm` | 10 | DM (cm^-3 pc) | DM9 (cm^-3 pc yr^-9) |
| 7 | `param_cm` | 10 | CM (arb) | CM9 (arb yr^-9) |
| 8 | `param_pmra` | 1 | PMRA (mas/yr) | PMRA (mas/yr) |
| 9 | `param_pmdec` | 1 | PMDEC (mas/yr) | PMDEC (mas/yr) |
| 10 | `param_pmra2` | 1 | PMRA2 (mas/yr^2) | PMRA2 (mas/yr^2) |
| 11 | `param_pmdec2` | 1 | PMDEC2 (mas/yr^2) | PMDEC2 (mas/yr^2) |
| 12 | `param_px` | 1 | PX (mas) | PX (mas) |
| 13 | `param_sini` | 1 | SINI | SINI |
| 14 | `param_pb` | 4 | PB (d) | PB_4 (d) |
| 15 | `param_fb` | 20 | FB0 | FB19 |
| 16 | `param_t0` | 4 | T0 (MJD) | T0_4 (mjd) |
| 17 | `param_a1` | 4 | A1 (lt-s) | A1_4 (lt-s) |
| 18 | `param_om` | 4 | OM (deg) | OM_4 (deg) |
| 19 | `param_pmrv` | 1 | PMRV (mas/yr) | PMRV (mas/yr) |
| 20 | `param_ecc` | 4 | ECC | ECC_4 |
| 21 | `param_edot` | 1 | EDOT | EDOT |
| 22 | `param_e2dot` | 1 | E2DOT | E2DOT |
| 23 | `param_xpbdot` | 1 | XPBDOT | XPBDOT |
| 24 | `param_pbdot` | 4 | PBDOT | PBDOT_4 |
| 25 | `param_pb2dot` | 1 | PB2DOT (s^-1) | PB2DOT (s^-1) |
| 26 | `param_a1dot` | 4 | XDOT | XDOT_4 |
| 27 | `param_a2dot` | 4 | X2DOT | X2DOT_4 |
| 28 | `param_omdot` | 1 | OMDOT (deg/yr) | OMDOT (deg/yr) |
| 29 | `param_om2dot` | 1 | OM2DOT(1e-20 rad/yr^2) | OM2DOT(1e-20 rad/yr^2) |
| 30 | `param_orbpx` | 1 | ORBPX (kpc^-1) | ORBPX (kpc^-1) |
| 31 | `param_tasc` | 1 | TASC (MJD) | TASC (MJD) |
| 32 | `param_eps1` | 1 | EPS1 | EPS1 |
| 33 | `param_eps2` | 1 | EPS2 | EPS2 |
| 34 | `param_m2` | 1 | M2 | M2 |
| 35 | `param_gamma` | 1 | GAMMA | GAMMA |
| 36 | `param_mtot` | 1 | MTOT | MTOT |
| 37 | `param_glep` | 40 | GLEP_1 | GLEP_40 |
| 38 | `param_glph` | 40 | GLPH_1 | GLPH_40 |
| 39 | `param_glf0` | 40 | GLF0_1 | GLF0_40 |
| 40 | `param_glf1` | 40 | GLF1_1 | GLF1_40 |
| 41 | `param_glf2` | 40 | GLF2_1 | GLF2_40 |
| 42 | `param_expep` | 40 | EXPEP_1 | EXPEP_40 |
| 43 | `param_expph` | 40 | EXPPH_1 | EXPPH_40 |
| 44 | `param_exptau` | 40 | EXPTAU_1 | EXPTAU_40 |
| 45 | `param_expindex` | 40 | EXPINDEX_1 | EXPINDEX_40 |
| 46 | `param_gausep` | 40 | GAUSEP_1 | GAUSEP_40 |
| 47 | `param_gausamp` | 40 | GAUSAMP_1 | GAUSAMP_40 |
| 48 | `param_gaussig` | 40 | GAUSSIG_1 | GAUSSIG_40 |
| 49 | `param_gausindex` | 40 | GAUSINDEX_1 | GAUSINDEX_40 |
| 50 | `param_glf0d` | 40 | GLF0D_1 | GLF0D_40 |
| 51 | `param_gltd` | 40 | GLTD_1 | GLTD_40 |
| 52 | `param_start` | 1 | START (MJD) | START (MJD) |
| 53 | `param_finish` | 1 | FINISH (MJD) | FINISH (MJD) |
| 54 | `param_track` | 1 | TRACK (MJD) | TRACK (MJD) |
| 55 | `param_bp` | 1 | BP | BP |
| 56 | `param_bpp` | 1 | BPP | BPP |
| 57 | `param_tzrmjd` | 1 | TZRMJD | TZRMJD |
| 58 | `param_tzrfrq` | 1 | TZRFRQ (MHz) | TZRFRQ (MHz) |
| 59 | `param_fddc` | 1 | FDDC | FDDC |
| 60 | `param_fddi` | 1 | FDDI | FDDI |
| 61 | `param_fd` | 9 | FD1 | FD9 |
| 62 | `param_dr` | 1 | DR | DR |
| 63 | `param_dtheta` | 1 | DTHETA | DTHETA |
| 64 | `param_tspan` | 1 | TSPAN (min) | TSPAN (min) |
| 65 | `param_bpjep` | 5 | BPJEP_1 | BPJEP_5 |
| 66 | `param_bpjph` | 5 | BPJPH_1 | BPJPH_5 |
| 67 | `param_bpja1` | 5 | BPJA1_1 | BPJA1_5 |
| 68 | `param_bpjec` | 5 | BPJEC_1 | BPJEC_5 |
| 69 | `param_bpjom` | 5 | BPJOM_1 | BPJOM_5 |
| 70 | `param_bpjpb` | 5 | BPJPB_1 | BPJPB_5 |
| 71 | `param_wave_om` | 1 | WAVE_OM | WAVE_OM |
| 72 | `param_kom` | 1 | KOM | KOM |
| 73 | `param_kin` | 1 | KIN | KIN |
| 74 | `param_shapmax` | 1 | SHAPMAX | SHAPMAX |
| 75 | `param_dth` | 1 | DTH | DTH |
| 76 | `param_a0` | 1 | A0 | A0 |
| 77 | `param_b0` | 1 | B0 | B0 |
| 78 | `param_xomdot` | 1 | XOMDOT | XOMDOT |
| 79 | `param_afac` | 1 | AFAC | AFAC |
| 80 | `param_eps1dot` | 1 | EPS1DOT | EPS1DOT |
| 81 | `param_eps2dot` | 1 | EPS2DOT | EPS2DOT |
| 82 | `param_tres` | 1 | TRES | TRES |
| 83 | `param_trestn` | 1 |  |  |
| 84 | `param_wave_dm` | 1 | WAVE_DM | WAVE_DM |
| 85 | `param_waveepoch_dm` | 1 | WAVEEPOCHD (MJD) | WAVEEPOCHD (MJD) |
| 86 | `param_dshk` | 1 | DSHK (kpc) | DSHK (kpc) |
| 87 | `param_ephver` | 1 | EPHVER | EPHVER |
| 88 | `param_daop` | 1 | IPERHARM | IPERHARM |
| 89 | `param_iperharm` | 1 | IPERHARM | IPERHARM |
| 90 | `param_dmassplanet` | 9 | DMASSPLANET1 (Msun) | DMASSPLANET9 (Msun) |
| 91 | `param_dphaseplanet` | 9 | DPHASEPLANET1  | DPHASEPLANET9  |
| 92 | `param_waveepoch` | 1 | WAVEEPOCH (MJD) | WAVEEPOCH (MJD) |
| 93 | `param_ifunc` | 1 | IFUNC | IFUNC |
| 94 | `param_clk_offs` | 1 | CLK_OFFS | CLK_OFFS |
| 95 | `param_dmx` | 512 | DMX_0001 (cm^-3 pc) | DMX_0512 (cm^-3 pc) |
| 96 | `param_dmxr1` | 512 | DMXR1_0001 (MJD) | DMXR1_0512 (MJD) |
| 97 | `param_dmxr2` | 512 | DMXR2_0001 (MJD) | DMXR2_0512 (MJD) |
| 98 | `param_dmmodel` | 1 | DMMODEL | DMMODEL |
| 99 | `param_gwsingle` | 1 | GW_OMEGA | GW_OMEGA |
| 100 | `param_cgw` | 1 |  |  |
| 101 | `param_quad_om` | 1 | QUAD_OM | QUAD_OM |
| 102 | `param_h3` | 1 | H3 | H3 |
| 103 | `param_h4` | 1 | H4 | H4 |
| 104 | `param_nharm` | 1 | Number of Shapiro delay harmonics | Number of Shapiro delay harmonics |
| 105 | `param_stig` | 1 | STIG | STIG |
| 106 | `param_telx` | 4 | TELX (lt-s) | TELX3 (lt-s) |
| 107 | `param_tely` | 4 | TELY (lt-s) | TELY3 (lt-s) |
| 108 | `param_telz` | 4 | TELZ (lt-s) | TELZ3 (lt-s) |
| 109 | `param_telEpoch` | 1 | TEL EPOCH (MJD) | TEL EPOCH (MJD) |
| 110 | `param_quad_ifunc_p` | 1 | QIFUNC_p | QIFUNC_p |
| 111 | `param_quad_ifunc_c` | 1 | QIFUNC_c | QIFUNC_c |
| 112 | `param_tel_dx` | 500 | TEL_DX |  |
| 113 | `param_tel_dy` | 500 | TEL_DY |  |
| 114 | `param_tel_dz` | 500 | TEL_DZ |  |
| 115 | `param_tel_vx` | 1 | TEL_VX (km/s) | TEL_VX (km/s) |
| 116 | `param_tel_vy` | 1 | TEL_VY (km/s) | TEL_VY (km/s) |
| 117 | `param_tel_vz` | 1 | TEL_VZ (km/s) | TEL_VZ (km/s) |
| 118 | `param_tel_x0` | 1 | TEL_X0 (km) | TEL_X0 (km) |
| 119 | `param_tel_y0` | 1 | TEL_Y0 (km) | TEL_Y0 (km) |
| 120 | `param_tel_z0` | 1 | TEL_Z0 (km) | TEL_Z0 (km) |
| 121 | `param_gwm_amp` | 2 | GWM_AMP | GWM_AMP_2 |
| 122 | `param_gwcs_amp` | 2 | GWCS_AMP1 | GWCS_AMP2 |
| 123 | `param_gwecc` | 2 | GWECC_AMP |  |
| 124 | `param_gwb_amp` | 2 | GWB_AMP | GWB_AMP_2 |
| 125 | `param_dm_sin1yr` | 1 | DM_S1YR (cm^-3 pc) | DM_S1YR (cm^-3 pc) |
| 126 | `param_dm_cos1yr` | 1 | DM_C1YR (cm^-3 pc) | DM_C1YR (cm^-3 pc) |
| 127 | `param_brake` | 1 | BRAKING INDEX | BRAKING INDEX |
| 128 | `param_stateSwitchT` | 40 | SWITCH_1 | SWITCH_40 |
| 129 | `param_df1` | 1 | DF1 | DF1 |
| 130 | `param_red_sin` | 1 |  |  |
| 131 | `param_red_cos` | 1 |  |  |
| 132 | `param_jitter` | 1 |  |  |
| 133 | `param_red_dm_sin` | 1 |  |  |
| 134 | `param_red_dm_cos` | 1 |  |  |
| 135 | `param_red_chrom_sin` | 1 |  |  |
| 136 | `param_red_chrom_cos` | 1 |  |  |
| 137 | `param_swgp_sin` | 1 |  |  |
| 138 | `param_swgp_cos` | 1 |  |  |
| 139 | `param_band_red_sin` | 1 |  |  |
| 140 | `param_band_red_cos` | 1 |  |  |
| 141 | `param_sx` | 512 | SX_0001 (cm^-3 pc) | SX_0512 (cm^-3 pc) |
| 142 | `param_sxr1` | 512 | SXR1_0001 (MJD) | SXR1_0512 (MJD) |
| 143 | `param_sxr2` | 512 | SXR2_0001 (MJD) | SXR2_0512 (MJD) |
| 144 | `param_sxer` | 512 | SXER_0001 (MJD) | SXER_0512 (MJD) |
| 145 | `param_group_red_sin` | 1 |  |  |
| 146 | `param_group_red_cos` | 1 |  |  |
| 147 | `param_ne_sw` | 1 | NE_SW (cm^-3) | NE_SW (cm^-3) |
| 148 | `param_shapevent` | 1 |  |  |
| 149 | `param_orbifunc` | 1 | ORBIFUNC | ORBIFUNC |
| 150 | `param_gltd2` | 40 | GLTD2_1 | GLTD2_40 |
| 151 | `param_gltd3` | 40 | GLTD3_1 | GLTD3_40 |
| 152 | `param_glf0d2` | 40 | GLF0D2_1 | GLF0D2_40 |
| 153 | `param_glf0d3` | 40 | GLF0D3_1 | GLF0D3_40 |
| 154 | `param_ne_sw_sin` | 1 | NE_SW_SIN (cm^-3) | NE_SW_SIN (cm^-3) |
| 155 | `param_ne_sw_ifunc` | 1 | NE_SW_IFUNC | NE_SW_IFUNC |
| 156 | `param_chromx` | 512 | CHROMX_0001 | CHROMX_0512 |
| 157 | `param_chromxr1` | 512 | CHROMXR1_0001 (MJD) | CHROMXR1_0512 (MJD) |
| 158 | `param_chromxr2` | 512 | CHROMXR2_0001 (MJD) | CHROMXR2_0512 (MJD) |
| 159 | `param_nudot_epoch` | 32 | NUDOT_EPOCH_1 (MJD) | NUDOT_EPOCH_32 (MJD) |
| 160 | `param_nudot_amp` | 32 | NUDOT_AMP_1 (Hz^2) | NUDOT_AMP_32 (Hz^2) |
| 161 | `param_nudot_transition_time` | 32 | NUDOT_TIME_1 (days) | NUDOT_TIME_32 (days) |
| 162 | `param_LAST` | 1 |  |  |
| 163 | `param_ZERO` | 1 |  |  |
| 164 | `param_JUMP` | 1 |  |  |
| 165 | `param_FDJUMP` | 1 |  |  |

Slots 166–1999 each have size one and empty labels; see the complete JSON. Natural `val`, `err`, `fitFlag`, `paramSet`, and link counts are zero in this build. Both `param_daop` and `param_iperharm` are labeled `IPERHARM`: the former source label was overwritten. This is preserved.

The initializer leaves `ne_sw_ifuncN` **and `nchromx`** unwritten. Explicit caller markers 117 and 119 survive initialization. They are controlled caller values, not defaults. The later controlled parser experiments use `ne_sw_ifuncN=117`, `nchromx=0`; neither is inferred from malloc.
