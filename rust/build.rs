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
