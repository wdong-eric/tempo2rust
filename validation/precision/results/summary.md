# Numerical summary

The normalized ratio is abs(actual-expected) / tolerance. A ratio <= 1 passes; discrete outputs use exact equality.

| Group | Backend | Count | Max absolute error | Unit | Max ratio to budget | Failed |
| --- | --- | ---: | ---: | --- | ---: | ---: |
| A_phase_replay | mpfr113_reference_replay | 23 | 4.33055E-37 | s | 4.33055E-25 | 0 |
| A_phase_replay | mpfr113_reference_replay | 69 | 3.63491E-26 | turn | 3.63491E-17 | 0 |
| A_phase_replay | qd_assisted | 23 | 1.69031E-23 | s | 1.69031E-11 | 0 |
| A_phase_replay | qd_assisted | 69 | 6.35308E-22 | turn | 6.35308E-13 | 0 |
| A_phase_replay | rug128 | 23 | 7.97058E-27 | s | 7.97058E-15 | 0 |
| A_phase_replay | rug128 | 69 | 5.39880E-24 | turn | 5.39880E-15 | 0 |
| affine_epoch | qd_assisted | 32 | 2.11787E-23 | s | 2.11787E-11 | 0 |
| affine_epoch | rug128 | 32 | 2.32978E-29 | s | 2.32978E-17 | 0 |
| arithmetic | qd_assisted | 20 | 2.68920E-24 | dimensionless | 9.17243E-2 | 0 |
| arithmetic | rug128 | 20 | 3.43306E-30 | dimensionless | 7.00283E-9 | 0 |
| boundary | qd_assisted | 51 | 1.00000E+0 | turn | 1.00000E+28 | 8 |
| boundary | rug128 | 85 | 8.65520E-29 | turn | 8.65520E-1 | 0 |
| boundary_fraction | qd_assisted | 8 | 1.00000E+0 | turn | 1.00000E+9 | 4 |
| boundary_fraction | rug128 | 8 | 8.65520E-29 | turn | 8.65520E-20 | 0 |
| boundary_residual | qd_assisted | 8 | 1.39641E-3 | s | 1.39641E+9 | 4 |
| boundary_residual | rug128 | 8 | 1.20862E-31 | s | 1.20862E-19 | 0 |
| decimal_roundtrip | qd_assisted | 16 | 0.00000E+5 | day | exact | 0 |
| decimal_roundtrip | rug128 | 16 | 0.00000E+5 | day | exact | 0 |
| delay | f64_control | 64 | 4.00000E-9 | s | 4.00000E+3 | 64 |
| delay | qd_assisted | 64 | 1.12081E-23 | s | 1.12081E-11 | 0 |
| delay | rug128 | 64 | 7.53109E-30 | s | 7.53109E-18 | 0 |
| epoch_difference | qd_assisted | 3 | 6.66694E-24 | s | 6.66694E-12 | 0 |
| epoch_difference | rug128 | 3 | 1.17410E-29 | s | 1.17410E-17 | 0 |
| epoch_parse | f64_control | 16 | 2.08244E-7 | s | 2.08244E+5 | 12 |
| epoch_parse | qd_assisted | 16 | 1.67504E-23 | s | 1.67504E-11 | 0 |
| epoch_parse | rug128 | 16 | 7.06788E-30 | s | 7.06788E-18 | 0 |
| math | qd_assisted | 32 | 9.26015E+11 | dimensionless | 3.44485E-4 | 0 |
| math | rug128 | 32 | 1.77748E+4 | dimensionless | 1.91241E-11 | 0 |
| orbital_math | rug128 | 31 | 4.69436E-39 | dimensionless | 4.69436E-11 | 0 |
| orbital_math | rug128 | 10 | 7.61076E-30 | rad | 4.84516E-2 | 0 |
| pulse_number | qd_assisted | 7 | 0.00000E+5 | turn | exact | 0 |
| pulse_number | rug128 | 7 | 0.00000E+5 | turn | exact | 0 |
| reference_convergence | mpfr256 | 48 | 3.20179E-73 | day | 6.27064E-18 | 0 |
| reference_convergence | mpfr256 | 79 | 1.43473E-34 | dimensionless | 1.20034E-13 | 0 |
| reference_convergence | mpfr256 | 10 | 2.59021E-68 | rad | 1.64898E-8 | 0 |
| reference_convergence | mpfr256 | 74 | 3.41445E-68 | s | 3.41445E-8 | 0 |
| reference_convergence | mpfr256 | 113 | 2.60225E-65 | turn | 2.37830E-5 | 0 |
| spin_absolute | qd_assisted | 7 | 1.88191E-20 | turn | 1.88191E-11 | 0 |
| spin_absolute | rug128 | 7 | 3.01665E-27 | turn | 3.01665E-18 | 0 |
| spin_formula_identity | mpfr512 | 7 | 1.64011E-142 | turn | 1.32542E-14 | 0 |
| spin_fractional | qd_assisted | 7 | 2.03941E-20 | turn | 2.03941E-11 | 0 |
| spin_fractional | rug128 | 7 | 2.93461E-27 | turn | 2.93461E-18 | 0 |
| spin_reduced | qd_assisted | 7 | 2.03941E-20 | turn | 2.03941E-11 | 0 |
| spin_reduced | rug128 | 7 | 2.93461E-27 | turn | 2.93461E-18 | 0 |
| spin_residual | qd_assisted | 7 | 2.84784E-23 | s | 2.84784E-11 | 0 |
| spin_residual | rug128 | 7 | 4.09791E-30 | s | 4.09791E-18 | 0 |
| transport_delay | qd_assisted | 64 | 1.12081E-23 | s | 1.12081E-11 | 0 |
| transport_delay | rug128 | 64 | 7.53109E-30 | s | 7.53109E-18 | 0 |

# Observed qd boundary failures

| Group / case | Actual | Expected | Absolute error | Unit |
| --- | ---: | ---: | ---: | --- |
| boundary: nlong(0.49999999999999999999) | 1.00000000000000000000000 | 0 | 1.00000E+0 | turn |
| boundary: nlong(-0.49999999999999999999) | -1.00000000000000000000000 | -0 | 1.00000E+0 | turn |
| boundary: trunc(0.99999999999999999999) | 1.00000000000000000000000 | 0 | 1.00000E+0 | turn |
| boundary: remainder(0.99999999999999999999) | -9.99999999999999945153271E-21 | 0.999999999999999999990000 | 1.00000E+0 | turn |
| boundary: trunc(-0.99999999999999999999) | -1.00000000000000000000000 | -0 | 1.00000E+0 | turn |
| boundary: remainder(-0.99999999999999999999) | 9.99999999999999945153271E-21 | -0.999999999999999999990000 | 1.00000E+0 | turn |
| boundary: nlong(100000000000.49999999999999999999) | 100000000001.000000000000 | 100000000000.000000000000 | 1.00000E+0 | turn |
| boundary: nlong(-100000000000.49999999999999999999) | -100000000001.000000000000 | -100000000000.000000000000 | 1.00000E+0 | turn |
| boundary_fraction: 0.49999999999999999999 | -0.500000000000000000010000 | 0.499999999999999999990000 | 1.00000E+0 | turn |
| boundary_residual: 0.49999999999999999999 | -0.000698203634107899843600421 | 0.000698203634107899843572492 | 1.39641E-3 | s |
| boundary_fraction: -0.49999999999999999999 | 0.500000000000000000010000 | -0.499999999999999999990000 | 1.00000E+0 | turn |
| boundary_residual: -0.49999999999999999999 | 0.000698203634107899843600421 | -0.000698203634107899843572492 | 1.39641E-3 | s |
| boundary_fraction: 100000000000.49999999999999999999 | -0.500000000000000000010000 | 0.499999999999999999990000 | 1.00000E+0 | turn |
| boundary_residual: 100000000000.49999999999999999999 | -0.000698203634107899843600421 | 0.000698203634107899843572492 | 1.39641E-3 | s |
| boundary_fraction: -100000000000.49999999999999999999 | 0.500000000000000000010000 | -0.499999999999999999990000 | 1.00000E+0 | turn |
| boundary_residual: -100000000000.49999999999999999999 | 0.000698203634107899843600421 | -0.000698203634107899843572492 | 1.39641E-3 | s |
