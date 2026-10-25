"""Source FFT contraction witness, with exact binary64 inputs in both engines."""
from pathlib import Path
import argparse, hashlib, json, subprocess
ROOT = Path(__file__).resolve().parents[2];HERE = Path(__file__).resolve().parent
BUILD = HERE / "build/fft";BUILD.mkdir(parents=True, exist_ok=True)
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()


def run(profile):
    archive = ROOT / "validation/reference/build/libtempo2-reference.a"
    native = BUILD / "native"
    cmd = ["/opt/homebrew/bin/g++-16", "-O2", "-std=gnu++11", "-DHAVE_CONFIG_H", "-I" + str(archive.parent / "source"), str(HERE / "fft_probe.cc"), str(archive), "-lquadmath", "-lgfortran", "-o", str(native)]
    subprocess.run(cmd, capture_output=True, check=True)
    obj = BUILD / "TKspectrum.C.o";obj.write_bytes(subprocess.check_output(["ar", "p", str(archive), "TKspectrum.C.o"]))
    assembly = BUILD / "assembly.txt";assembly.write_bytes(subprocess.check_output(["xcrun", "llvm-objdump", "--disassemble", "--demangle", str(obj)]))
    rust = ROOT / "rust/target" / profile / "examples/fitting_covariance"
    records = []
    for n in [2, 8, 128, 1024]:
        for direction in [-1, 1]:
            lines = subprocess.check_output([str(native), str(direction), str(n)]).decode().splitlines()
            got = json.loads(subprocess.check_output([str(rust), "fft", str(direction), str(n)]))
            mismatches = []
            for i, line in enumerate(lines):
                index, re, im = line.split();assert int(index) == i
                for name, want in [("real", re), ("imaginary", im)]:
                    if float(got[name][i]).hex() != float(want).hex(): mismatches.append([i, name, got[name][i], want])
            records.append({"n": n, "direction": direction, "mismatches": mismatches})
    receipt = {"compile": cmd, "archive_sha256": sha(archive), "assembly_sha256": sha(assembly), "native_sha256": sha(native), "rust_sha256": sha(rust), "profile": profile, "records": records}
    (BUILD / (profile + ".json")).write_text(json.dumps(receipt, indent=2) + "\n")
    print("FFT binary64 mismatches", sum(len(r["mismatches"]) for r in records))


if __name__ == "__main__":
    p = argparse.ArgumentParser();p.add_argument("--profile", choices=["debug", "release"], default="debug")
    run(p.parse_args().profile)
