//! External source-direct resource calls, failure/recovery and owned state.
use std::{fs::File, path::Path};
use tempo2_rust::{
    Diagnostics, clkcorr, eop, get_obs_coord, ifteph, initialise, jpleph, observatory,
    read_fortran, stdio::Stream, tabulated_function::TabulatedFunction, tai2ut1, tempo2_util,
    tropo, tt2tdb, vector_pulsar,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut diagnostics = Diagnostics::default();
    let path = Path::new("table");
    std::fs::write(path, b"broken\n")?;
    let mut table = TabulatedFunction::default();
    assert!(table.load(path, &mut diagnostics).is_err());
    std::fs::write(path, b"# simple\n0 2\n1 4\n2 6\n")?;
    let mut table = TabulatedFunction::default();
    table.load(path, &mut diagnostics)?;
    assert_eq!(table.start()?, 0.);
    assert_eq!(table.end()?, 2.);
    assert_eq!(table.get_value(0.5, &mut diagnostics)?, 3.);
    let mut params = initialise::initialise_parameters();
    let mut series = None;
    initialise::label_parameters(&mut params, &mut series);
    assert!(!params.slots().is_empty());
    assert!(!initialise::allocate_memory(0).slots().is_empty());
    let mut o = observatory::Observatory {
        latitude: 0.,
        longitude: 0.,
        height: 0.,
        ..Default::default()
    };
    observatory::grs80_to_itrf(&mut o);
    let xyz = o.xyz;
    observatory::itrf_to_grs80(&mut o);
    assert_eq!(xyz[1], 0.);
    assert!(o.height.abs() < 1e-6);
    assert_eq!(observatory::fang(0, 0.)?, 0.);
    let p = vector_pulsar::position(0., 0.);
    assert_eq!(p, [1., 0., 0.]);
    let mut v = [1., 2., 3.];
    vector_pulsar::rotate(&mut v, true, 0.);
    assert_eq!(v, [1., 2., 3.]);
    assert_eq!(tempo2_util::dot_product([1., 2., 3.], [2., 3., 4.]), 20.);
    assert_eq!(tempo2_util::hms_turn(12., 0., 0.), 0.5);
    assert_eq!(tempo2_util::turn_deg(0.5), 180.);
    assert_eq!(tempo2_util::dms_turn(90., 0., 0., 1.), 0.25);
    assert!(
        get_obs_coord::lmst(58000., 0.)?
            .iter()
            .all(|v| v.is_finite())
    );
    let mut spaces = b"  1 2\0".to_vec();
    get_obs_coord::remove_white(&mut spaces)?;
    assert_eq!(&spaces[..4], b"1 2\0");
    assert!(
        get_obs_coord::get_precession_matrix(58000., 0., 0.)
            .iter()
            .flatten()
            .all(|v| v.is_finite())
    );
    assert!((tropo::nmf_wet(0.5, std::f64::consts::FRAC_PI_2)? - 1.).abs() < 1e-14);
    assert!(
        (tropo::nmf_hydrostatic(58000., 0.5, 100., std::f64::consts::FRAC_PI_2)? - 1.).abs()
            < 1e-14
    );
    let mut word = [1, 2, 3, 4];
    ifteph::IFTswap4(&mut word);
    assert_eq!(word, [4, 3, 2, 1]);
    ifteph::IFTswap4(&mut word);
    assert_eq!(word, [1, 2, 3, 4]);
    let mut integers = [1, 2];
    ifteph::IFTswapInts(&mut integers, 2)?;
    ifteph::IFTswapInts(&mut integers, 2)?;
    assert_eq!(integers, [1, 2]);
    let mut bytes = [1, 2, 3, 4, 5, 6, 7, 8];
    ifteph::IFTswap8(&mut bytes);
    ifteph::IFTswap8N(&mut bytes, 1)?;
    assert_eq!(bytes, [1, 2, 3, 4, 5, 6, 7, 8]);
    let mut ds = [1., -0.];
    ifteph::IFTswapDoubles(&mut ds, 2)?;
    ifteph::IFTswapDoubles(&mut ds, 2)?;
    assert_eq!(ds[0], 1.);
    assert!(ds[1].is_sign_negative());
    let mut pv = [0.; 2];
    let mut interpolation = jpleph::Interpolation::new();
    let raw: Vec<_> = [1f64, 2.].iter().flat_map(|v| v.to_ne_bytes()).collect();
    interpolation.interp(&raw, [0.5, 1.], 2, 1, 1, 2, &mut pv)?;
    assert_eq!(pv, [1., 4.]);
    assert!(jpleph::Ephemeris::init(File::open("missing-ephemeris"), None, None)?.is_none());
    assert_eq!(jpleph::jpl_init_error_code(), -1);
    let mut ifte = ifteph::IfteEphemeris::default();
    assert!(
        ifte.init(Path::new("missing-ifte"), &mut diagnostics)
            .is_err()
    );
    ifte.close_file();
    let mut reader = read_fortran::FortranReader::default();
    let mut filename = b"missing-fortran\0".to_vec();
    assert_ne!(reader.open_file(&mut filename, &mut diagnostics)?, 0);
    std::fs::write("bytes.bin", [10, 20, 30, 40])?;
    let mut stream = Stream::from_file(File::open("bytes.bin")?)?;
    assert_eq!(stream.getc(), 10);
    assert_eq!(stream.status().0, 1);
    let mut b = [0; 2];
    assert_eq!(stream.read(&mut b, 1, 2)?, 2);
    assert_eq!(b, [20, 30]);
    assert_eq!(stream.seek(0, 0), 0);
    drop(stream);
    let mut session = clkcorr::ClockSession::default();
    let value = session.run(|_| Ok(12))?;
    assert_eq!(value, 12);
    assert_eq!(clkcorr::with_process(|_| Ok(17))?, 17);
    assert_eq!(clkcorr::with_resources(Path::new("."), |_| Ok(18))?, 18);
    let _: eop::Eop = Default::default();
    let _: tai2ut1::Ut1 = Default::default();
    let _: tt2tdb::FbCache = Default::default();
    let _: tropo::Meteorology = Default::default();
    let _: tropo::MeteorologyFunction = Default::default();
    let _: observatory::ObservatoryDatabase = Default::default();
    println!("resource owners, direct kernels, failure recovery and stream lifetime passed");
    Ok(())
}
