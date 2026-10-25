"""Automatic covariance, source helper closure and generated diagnostic files."""
from pathlib import Path
from decimal import Decimal as D, getcontext
import argparse, hashlib, json, os, subprocess
from fitting_corrections import corrected

getcontext().prec = 70
ROOT = Path(__file__).resolve().parents[2];HERE = Path(__file__).resolve().parent
BUILD = HERE / "build/automatic"
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()


def run(profile):
    BUILD.mkdir(parents=True, exist_ok=True)
    identity = json.loads((HERE / "build/native/identity.json").read_text())
    objects = [];commands = []
    for name in ["choleskyAutomatic.C", "choleskyRoutines.C"]:
        source = subprocess.check_output(["git", "-C", str(ROOT / "tempo2"), "show", identity["pin"] + ":" + name]).decode()
        source = corrected(name, source)
        if name == "choleskyRoutines.C":
            token = "    cholesky_formL(cholesky_L,m,np);"
            source = source.replace(token, token + "\n    extern void ae_factors(double**,double*,int,int);ae_factors(cholesky_L,covarFunc,np,ndays);")
        path = BUILD / name;path.write_text(source)
        obj = BUILD / (name + ".o");objects.append(obj)
        cmd = identity["commands"][0].copy();cmd[cmd.index("-c") + 1] = str(path);cmd[cmd.index("-o") + 1] = str(obj)
        subprocess.run(cmd, check=True, capture_output=True);commands.append(cmd)
    native = BUILD / "native"
    cmd = identity["commands"][-1].copy();cmd[cmd.index(str(HERE / "native_fit.cc"))] = str(HERE / "native_automatic.cc")
    cmd = [arg for arg in cmd if Path(arg).name not in {p.name for p in objects}]
    at = cmd.index(str(ROOT / "validation/reference/build/libtempo2-reference.a"));cmd[at:at] = list(map(str, objects));cmd[cmd.index("-o") + 1] = str(native)
    subprocess.run(cmd, check=True, capture_output=True);commands.append(cmd)
    rust = ROOT / "rust/target" / profile / "examples/fitting_covariance"
    out = BUILD / profile;out.mkdir(exist_ok=True)
    runtime = ROOT / "validation/reference/build/source/T2runtime"
    par = ROOT / "validation/fitting/inputs/f0.par";tim = ROOT / "validation/fitting/inputs/bary.tim"
    paths = {engine: out / engine for engine in ["native", "rust"]}
    for p in paths.values(): p.mkdir(exist_ok=True)
    trace = paths["native"] / "observations.tsv"
    commands += [[str(native), str(trace), str(par), str(tim)], [str(rust), str(runtime), str(par), str(tim), str(paths["rust"])]]
    exits = {}
    for engine, cmd in zip(paths, commands[-2:]):
        proc = subprocess.run(cmd, cwd=paths[engine], env={**os.environ, "TEMPO2": str(runtime)}, capture_output=True, timeout=120)
        exits[engine] = proc.returncode
        (paths[engine] / "stdout").write_bytes(proc.stdout);(paths[engine] / "stderr").write_bytes(proc.stderr)
    failures = []
    if any(exits.values()): failures.append({"kind": "execution", "exits": exits})
    else:
        result = json.loads((paths["rust"] / "stdout").read_text())
        rows = [line.split() for line in trace.read_text().splitlines()]
        maximum = max(int(r[1]) for r in rows if r[0] == "C")
        cov = [D(r[3]) for r in rows if r[0] == "C" and int(r[1]) == maximum]
        controls = [D(r[2]) for r in rows if r[0] == "V"]
        lower = {(int(r[2]), int(r[3])): D(r[4]) for r in rows if r[0] == "L" and int(r[1]) == maximum}
        assert len(result["covariance_function"]) == len(cov) and len(result["controls"]) == len(controls)
        assert len(lower) == sum(len(r) for r in result["lower"])
        pairs = [(f"covariance.{i}", result["covariance_function"][i], v, D("5e-14") * max(map(abs, cov))) for i, v in enumerate(cov)]
        pairs += [(f"control.{i}", result["controls"][i], v, D(0) if i in [6, 7] else D("5e-14") * abs(v)) for i, v in enumerate(controls)]
        pairs += [(f"lower.{i}.{j}", result["lower"][i][j], v, D("5e-14") * max(abs(a) for (a_i, a_j), a in lower.items() if a_j == j)) for (i, j), v in lower.items()]
        for quantity, got, want, gate in pairs:
            got = D(got)
            if want == 0: gate = D(0)
            if not got.is_finite() or not want.is_finite(): equal = ((got.is_nan() and want.is_nan()) or got == want) and got.is_signed() == want.is_signed()
            else: equal = abs(got - want) <= gate and (want != 0 or got != 0 or got.is_signed() == want.is_signed())
            if not equal: failures.append({"identity": quantity, "got": str(got), "reference": str(want), "gate": str(gate)})
        for name in ["comp.dat", "compchol.dat", "cholSpec.dat", "automatic.dcf"]:
            if (paths["native"] / name).read_bytes() != (paths["rust"] / name).read_bytes(): failures.append({"identity": name, "kind": "file_bytes"})
    receipt = {"pin": identity["pin"], "profile": profile, "commands": commands, "inputs": {str(p): sha(p) for p in [par, tim]}, "native_sha256": sha(native), "rust_sha256": sha(rust), "builder_sha256": sha(__file__), "failures": failures}
    (out / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print("automatic covariance failures:", len(failures), failures[:3])


if __name__ == "__main__":
    p = argparse.ArgumentParser();p.add_argument("--profile", default="debug", choices=["debug", "release"])
    run(p.parse_args().profile)
