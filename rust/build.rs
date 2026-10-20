use std::{env, fs, path::Path, process::Command};

fn run(program: &str, args: &[&str]) {
    let status = Command::new(program)
        .args(args)
        .status()
        .unwrap_or_else(|e| panic!("cannot run {program}: {e}; install GCC/GFortran or set CC/FC"));
    assert!(status.success(), "{program} {args:?} failed");
}

fn main() {
    println!("cargo:rerun-if-changed=native");
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=FC");
    let out = env::var("OUT_DIR").unwrap();
    let fc = env::var("FC").unwrap_or_else(|_| "gfortran-16".into());
    let cc = env::var("CC").unwrap_or_else(|_| "gcc-16".into());
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
    let object = Path::new(&out).join("jpleph.o");
    run(
        &cc,
        &[
            "-O2",
            "-fPIC",
            "-c",
            "native/jpleph.c",
            "-o",
            object.to_str().unwrap(),
        ],
    );
    objects.push(object);
    let archive = Path::new(&out).join("libtiming_native.a");
    let mut args = vec!["crs", archive.to_str().unwrap()];
    args.extend(objects.iter().map(|p| p.to_str().unwrap()));
    run("ar", &args);
    println!("cargo:rustc-link-search=native={out}");
    println!("cargo:rustc-link-lib=static=timing_native");
}
