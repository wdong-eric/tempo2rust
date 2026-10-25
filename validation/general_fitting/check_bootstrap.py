"""Seed bits, partial-copy bootstrap history and writeTim on natural fit inputs."""
from pathlib import Path
from decimal import Decimal as D, getcontext
import argparse, hashlib, json, os, subprocess
from fitting_corrections import corrected

getcontext().prec = 70
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
BUILD = HERE / "build/bootstrap"
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()


def run(profile):
    BUILD.mkdir(parents=True, exist_ok=True)
    identity = json.loads((HERE / "build/native/identity.json").read_text())
    source = subprocess.check_output(["git", "-C", str(ROOT / "tempo2"), "show", identity["pin"] + ":bootstrap.C"]).decode()
    source = corrected("bootstrap.C", source)
    declaration = "void ae_bootstrap_begin(int);void ae_bootstrap_sample(int,int);void ae_bootstrap_fit(pulsar*);\n"
    source = source.replace("int bootstrap(pulsar", declaration + "int bootstrap(pulsar")
    source = source.replace("        sum   = 0.0;", "        ae_bootstrap_begin(iter);\n        sum   = 0.0;")
    source = source.replace("            for (j=0;j<nFit;j++)\n                xmean", "            ae_bootstrap_sample(i,ii);\n            for (j=0;j<nFit;j++)\n                xmean")
    source = source.replace("        t2Fit(&psr[p],1,NULL);", "        t2Fit(&psr[p],1,NULL);\n        ae_bootstrap_fit(psr+p);")
    src = BUILD / "observed.C";src.write_text(source)
    cmd = identity["commands"][0].copy();cmd[cmd.index("-c") + 1] = str(src);cmd[cmd.index("-o") + 1] = str(BUILD / "observed.o")
    subprocess.run(cmd, check=True, capture_output=True);commands = [cmd]
    util = subprocess.check_output(["git", "-C", str(ROOT / "tempo2"), "show", identity["pin"] + ":tempo2Util.C"]).decode()
    util_source = BUILD / "tempo2Util.C";util_source.write_text(corrected("tempo2Util.C", util))
    cmd = identity["commands"][0].copy();cmd[cmd.index("-c") + 1] = str(util_source);cmd[cmd.index("-o") + 1] = str(BUILD / "tempo2Util.o")
    subprocess.run(cmd, check=True, capture_output=True);commands.append(cmd)
    cmd = identity["commands"][-1].copy();cmd[cmd.index(str(HERE / "native_fit.cc"))] = str(HERE / "native_bootstrap.cc")
    cmd.insert(cmd.index(str(ROOT / "validation/reference/build/libtempo2-reference.a")), str(BUILD / "observed.o"))
    cmd.insert(cmd.index(str(ROOT / "validation/reference/build/libtempo2-reference.a")), str(BUILD / "tempo2Util.o"))
    native = BUILD / "native";cmd[cmd.index("-o") + 1] = str(native)
    subprocess.run(cmd, check=True, capture_output=True);commands.append(cmd)
    rust = ROOT / "rust/target" / profile / "examples/fitting_bootstrap"
    out = BUILD / profile;out.mkdir(exist_ok=True)
    records = []
    for seed in [-999, -1, 1]:
        a = subprocess.check_output([str(native), "rng", str(seed), "1000"], timeout=30).decode().splitlines()
        b = json.loads(subprocess.check_output([str(rust), "rng", str(seed), "1000"], timeout=30))
        assert a[:-1] == b["values"] and int(a[-1].split()[1]) == b["seed"]
        records.append({"seed": seed, "values": b["values"], "final_seed": b["seed"]})
    (out / "rng.json").write_text(json.dumps(records, indent=2) + "\n")
    runtime = ROOT / "validation/reference/build/source/T2runtime"
    par = HERE / "inputs/bootstrap.par";tim = ROOT / "validation/fitting/inputs/bary.tim"
    paths = {engine: out / engine for engine in ["native", "rust"]}
    for p in paths.values(): p.mkdir(exist_ok=True)
    native_output = paths["native"] / "history.tsv"
    commands += [[str(native), str(native_output), str(par), str(tim)], [str(rust), str(runtime), str(par), str(tim), str(paths["rust"])]]
    exits = {}
    for engine, cmd in zip(paths, commands[-2:]):
        proc = subprocess.run(cmd, cwd=paths[engine], env={**os.environ, "TEMPO2": str(runtime)}, capture_output=True, timeout=120)
        exits[engine] = proc.returncode
        (paths[engine] / "stdout").write_bytes(proc.stdout);(paths[engine] / "stderr").write_bytes(proc.stderr)
    failures = []
    if any(exits.values()): failures.append({"kind": "execution", "exits": exits})
    else:
        result = json.loads((paths["rust"] / "stdout").read_text())
        native_rows = [line.split() for line in native_output.read_text().splitlines()]
        samples = [r for r in native_rows if r[0] == "S"]
        expected = [(it, i, j) for it, v in enumerate(result["iterations"]) for i, j in enumerate(v["sampled_indices"])]
        if [tuple(map(int, r[1:])) for r in samples] != expected: failures.append({"kind": "sample_membership"})
        final = [r for r in native_rows if r[0] == "F"]
        columns = [list(map(int, r[2:4])) for r in final]
        if columns != result["columns"]: failures.append({"kind": "parameter_membership"})
        # This authored case fits only F0. Its value/error are converted to the
        # same maximum-baseline seconds coordinate as public physical-spin gates.
        assert columns == [[2, 0]]
        scale = D("160") * 86400 / D("10.12345678902")
        for row in native_rows:
            if row[0] not in ("P", "F"): continue
            it = int(row[1]);values = result["values"] if row[0] == "F" else result["iterations"][it]["fitted_values"]
            delta = (D(values[0]) - D(row[4])) * scale
            if abs(delta) > D("1e-11"): failures.append({"kind": "physical_value", "iteration": it, "difference_s": str(delta)})
            if row[0] == "F":
                delta = (D(result["errors"][0]) - D(row[5])) * scale
                if abs(delta) > D("1e-11") + D("1e-6") * abs(D(row[5]) * scale): failures.append({"kind": "physical_error", "difference_s": str(delta)})
        if (paths["native"] / "testout.tim").read_bytes() != (paths["rust"] / "testout.tim").read_bytes(): failures.append({"kind": "writeTim_bytes"})
    receipt = {"pin": identity["pin"], "profile": profile, "commands": commands, "inputs": {str(p): sha(p) for p in [par, tim]}, "native_sha256": sha(native), "rust_sha256": sha(rust), "builder_sha256": sha(__file__), "observed_source_sha256": sha(src), "rng_sha256": sha(out / "rng.json"), "failures": failures}
    (out / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print("3000 RNG values match; bootstrap failures:", failures)


if __name__ == "__main__":
    p = argparse.ArgumentParser();p.add_argument("--profile", default="debug", choices=["debug", "release"])
    run(p.parse_args().profile)
