//! Seeded tensor/general/anisotropic/dipole sources on actual canonical TOAs.
mod analysis_support;
use analysis_support::*;
use std::{
    fs::File,
    io::{Seek, SeekFrom},
};
use tempo2_rust::{
    gw_file::{self, Profile},
    gw_sim::*,
    tempo2::Label,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = args()?;
    let (mut input, _runtime, _timing) = load(&a)?;
    let mut seed = value(&a, "--seed", -12345i64)?;
    let count = value(&a, "--count", 4usize)?;
    let amp = value(&a, "--amplitude", 1e-15)?;
    let alpha = value(&a, "--alpha", -2.0 / 3.0)?;
    let dist = ld(value(&a, "--distance", 3.08568e19)?);
    let o = PopulationOptions {
        flo: ld(value(&a, "--flo", 1e-9)?),
        fhi: ld(value(&a, "--fhi", 1e-7)?),
        loglin: value(&a, "--loglin", 1)?,
    };
    let (psr, rows) = input.context.pulsar_observations_mut(0)?;
    let n = psr.nobs.unwrap() as usize;
    let epoch = psr.param.slots()[Label::param_pepoch as usize]
        .cell(0)?
        .value()
        .clone();
    let kp = setup_pulsar(
        psr.param.slots()[Label::param_raj as usize]
            .cell(0)?
            .value(),
        psr.param.slots()[Label::param_decj as usize]
            .cell(0)?
            .value(),
    );
    let times: Vec<_> = rows[..n]
        .iter()
        .map(|r| (r.sat.clone().unwrap() - &epoch) * 86400.0)
        .collect();
    let mut session = GwSession::default();
    let harmonics = vec![
        Harmonic {
            l: 0.0,
            m: 0.0,
            amplitude: 1.0,
        },
        Harmonic {
            l: 1.0,
            m: -1.0,
            amplitude: 0.2,
        },
    ];
    let mut tensor = session.background(count, &mut seed, &o, amp, alpha)?;
    let anisotropic =
        session.anisotropic_background(count, &mut seed, &o, amp, alpha, &harmonics)?;
    let dipole =
        session.dipole_background(count, &mut seed, &o, amp, alpha, [0.01, 0.005, -0.003])?;
    for (name, mut sources) in [
        ("tensor", tensor.clone()),
        ("anisotropic", anisotropic),
        ("dipole", dipole),
    ] {
        for g in &mut sources {
            g.setup();
        }
        for (i, t) in times.iter().enumerate() {
            let mut r = ld(0.0);
            for g in &sources {
                r += g.residual(&kp, t, &dist)?;
            }
            extended(&format!("{name}.residual.{i}"), "seconds", &r);
        }
    }
    let s = GeneralSpectrum {
        tensor_amp: amp,
        st_amp: amp,
        sl_amp: amp,
        vl_amp: amp,
        tensor_alpha: alpha,
        st_alpha: alpha,
        sl_alpha: alpha,
        vl_alpha: alpha,
    };
    let mut general = session.general_background([count; 4], &mut seed, &o, s)?;
    let aniso = session.general_anisotropic_background(
        [count; 4],
        &mut seed,
        &o,
        s,
        &std::array::from_fn(|_| harmonics.clone()),
    )?;
    for (name, mut sources) in [("general", general.clone()), ("general_anisotropic", aniso)] {
        for g in &mut sources {
            g.setup();
        }
        for (i, t) in times.iter().enumerate() {
            let mut r = ld(0.0);
            for g in &sources {
                r += g.residual(&kp, t, &dist)?;
            }
            extended(&format!("{name}.residual.{i}"), "seconds", &r);
        }
    }
    let prefix = required(&a, "--output")?;
    let profile = Profile::LittleEndianI32Binary128;
    for g in &mut tensor {
        g.setup();
    }
    for g in &mut general {
        g.setup();
    }
    let tensor_path = format!("{prefix}.tensor.bin");
    let general_path = format!("{prefix}.general.bin");
    {
        let mut f = File::create(&tensor_path)?;
        gw_file::write_tensor(&mut f, 2, &tensor, profile)?;
        gw_file::write_tensor(&mut f, 9, &tensor, profile)?;
    }
    {
        let mut f = File::create(&general_path)?;
        gw_file::write_general(&mut f, 2, &general, profile)?;
        gw_file::write_general(&mut f, 9, &general, profile)?;
    }
    let mut read = vec![];
    let mut f = File::open(&tensor_path)?;
    emit(
        "file.tensor.count",
        "exact",
        gw_file::read_tensor(&mut f, 9, &mut read, profile)?,
    );
    emit("file.tensor.cursor", "exact", f.stream_position()?);
    for g in &mut read {
        assert!(g.geometry.is_none());
        g.setup();
    }
    f.seek(SeekFrom::Start(0))?;
    gw_file::read_tensor(&mut f, 2, &mut read, profile)?;
    let mut read = vec![];
    let mut f = File::open(&general_path)?;
    emit(
        "file.general.count",
        "exact",
        gw_file::read_general(&mut f, 9, &mut read, profile)?,
    );
    emit("file.general.cursor", "exact", f.stream_position()?);
    emit("seed.final", "exact", seed);
    emit("rng.draws", "exact", session.rng.state().2);
    for w in session.warnings {
        eprintln!("{w}");
    }
    Ok(())
}
