//! Rust-only Cargo build glue for retained SOFA, replacing selected
//! Makefile.am and sofa/Makefile.am rules; no standalone timing routine counterpart.
use std::{env, fs, path::Path, process::Command};

fn run(program: &str, args: &[&str]) {
    let status = Command::new(program)
        .args(args)
        .status()
        .unwrap_or_else(|e| panic!("cannot run {program}: {e}; install GFortran or set FC"));
    assert!(status.success(), "{program} {args:?} failed");
}

fn main() {
    println!("cargo:rerun-if-changed=native");
    println!("cargo:rerun-if-env-changed=FC");
    let out = env::var("OUT_DIR").unwrap();
    let fc = env::var("FC").unwrap_or_else(|_| "gfortran-16".into());
    let mut sources: Vec<_> = fs::read_dir("native/sofa")
        .unwrap()
        .map(|p| p.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "for"))
        .collect();
    sources.sort();
    let mut objects = Vec::new();
    println!("cargo:rerun-if-changed=include");
    let predictor_object = Path::new(&out).join("predictor.o");
    let cc = env::var("CC").unwrap_or_else(|_| "clang".into());
    run(
        &cc,
        &[
            "-std=c11",
            "-O2",
            "-fPIC",
            "-Iinclude",
            "-c",
            "native/predictor.c",
            "-o",
            predictor_object.to_str().unwrap(),
        ],
    );
    objects.push(predictor_object);
    for key in ["CC", "CALCEPH_DIR", "TEMPO2_ACCELERATE"] {
        println!("cargo:rerun-if-env-changed={key}");
    }
    let blas = env::var_os("CARGO_FEATURE_BLAS").is_some();
    let calceph = env::var_os("CARGO_FEATURE_CALCEPH").is_some();
    if blas || calceph {
        let cc = env::var("CC").unwrap_or_else(|_| "clang".into());
        let object = Path::new(&out).join("backends.o");
        let mut args = vec![
            "-std=c11".to_owned(),
            "-O2".into(),
            "-fPIC".into(),
            "-c".into(),
            "native/backends.c".into(),
            "-o".into(),
            object.to_str().unwrap().into(),
        ];
        if blas {
            assert!(
                env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos"
                    && env::var("TEMPO2_ACCELERATE").as_deref() == Ok("1"),
                "blas/lapack currently require the explicitly selected, measured macOS LP64 provider: TEMPO2_ACCELERATE=1; other provider ABIs need qualification"
            );
            args.push("-DT2_BLAS".into());
            if env::var_os("CARGO_FEATURE_LAPACK").is_some() {
                args.push("-DT2_LAPACK".into());
            }
            println!("cargo:rustc-link-lib=framework=Accelerate");
        }
        if calceph {
            let prefix = env::var("CALCEPH_DIR").expect(
                "calceph requires CALCEPH_DIR containing include/calceph.h and lib/libcalceph.a",
            );
            args.push("-DT2_CALCEPH".into());
            args.push(format!("-I{prefix}/include"));
            println!("cargo:rerun-if-changed={prefix}/include/calceph.h");
            println!("cargo:rerun-if-changed={prefix}/lib/libcalceph.a");
            println!("cargo:rustc-link-search=native={prefix}/lib");
            println!("cargo:rustc-link-lib=static=calceph");
        }
        run(&cc, &args.iter().map(String::as_str).collect::<Vec<_>>());
        objects.push(object);
    }
    for source in sources {
        let object = Path::new(&out)
            .join(source.file_name().unwrap())
            .with_extension("o");
        // Legacy Fortran builds may keep mutable locals in static storage.
        // Automatic locals prevent cross-context races without changing the
        // arithmetic; numerical fixture gates cover the compiled closure.
        run(
            &fc,
            &[
                "-O2",
                "-fno-second-underscore",
                "-frecursive",
                "-c",
                source.to_str().unwrap(),
                "-o",
                object.to_str().unwrap(),
            ],
        );
        objects.push(object);
    }
    let archive = Path::new(&out).join("libtiming_native.a");
    // Recreate the archive so an earlier build cannot retain a removed JPL object.
    if archive.exists() {
        fs::remove_file(&archive).unwrap();
    }
    let mut args = vec!["crs", archive.to_str().unwrap()];
    args.extend(objects.iter().map(|p| p.to_str().unwrap()));
    run("ar", &args);
    println!("cargo:rustc-link-search=native={out}");
    println!("cargo:rustc-link-lib=static=timing_native");
}
