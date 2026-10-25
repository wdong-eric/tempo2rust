"""Direct standalone fitting examples, with independently timed native inputs."""
from pathlib import Path
from decimal import Decimal, getcontext
import argparse
import hashlib
import json
import os
import subprocess

getcontext().prec = 70
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
BUILD = HERE / "build/components"
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()


def run(profile):
    BUILD.mkdir(exist_ok=True, parents=True)
    identity = json.loads((HERE / "build/native/identity.json").read_text())
    cmd = identity["commands"][-1].copy()
    cmd[cmd.index(str(HERE / "native_fit.cc"))] = str(HERE / "native_components.cc")
    exe = BUILD / "native"
    cmd[cmd.index("-o") + 1] = str(exe)
    proc = subprocess.run(cmd, capture_output=True)
    (BUILD / "compile.log").write_bytes(proc.stdout + proc.stderr)
    assert proc.returncode == 0, proc.stderr.decode()
    runtime = ROOT / "validation/reference/build/source/T2runtime"
    par = HERE / "inputs/dm-grid.par"
    tim = HERE / "inputs/multi-frequency.tim"
    rust = ROOT / "rust/target" / profile / "examples/fitting_components"
    records = []
    for name, mode, extra in [("weights", "grid-weights", []), ("automatic", "grid-auto", ["50", "100", "53900", "54160", "0"]), ("fixed-cm", "grid-auto", ["50", "100", "53900", "54160", "1"])]:
        out = BUILD / profile / name
        out.mkdir(parents=True, exist_ok=True)
        commands = [[str(exe), str(out / "native.tsv"), mode, str(par), str(tim), *extra], [str(rust), mode, str(runtime), str(par), str(tim), *extra]]
        record = {"name": name, "profile": profile, "compile": cmd, "commands": commands, "inputs": {str(p): sha(p) for p in [par, tim]}, "native_sha256": sha(exe), "rust_sha256": sha(rust), "native_fit_identity_sha256": sha(HERE / "build/native/identity.json")}
        for engine, cmd in zip(["native", "rust"], commands):
            proc = subprocess.run(cmd, env={**os.environ, "TEMPO2": str(runtime)}, cwd=ROOT, capture_output=True, timeout=120)
            record[engine + "_exit"] = proc.returncode
            (out / ("rust.json" if engine == "rust" else "native.stdout")).write_bytes(proc.stdout)
            (out / (engine + ".stderr")).write_bytes(proc.stderr)
        failures = []
        if record["native_exit"] == record["rust_exit"] == 0:
            source = [line.split() for line in (out / "native.tsv").read_text().splitlines()]
            rows = json.loads((out / "rust.json").read_text())["rows"]
            if len(source) != len(rows): failures.append({"kind": "row_count", "source": len(source), "rust": len(rows)})
            for i, (s, r) in enumerate(zip(source, rows)):
                if (s[0], int(s[1])) != (r["family"], r["index"]): failures.append({"row": i, "kind": "identity"})
                for j, key in enumerate(["epoch", "value", "weight"], 2):
                    # Epochs and initialized grid values are binary64 contracts.
                    a, b = Decimal.from_float(float(s[j])), Decimal.from_float(float(r[key]))
                    gate = Decimal("1e-10") * abs(a) if key == "weight" else Decimal(0)
                    if not a.is_finite() or not b.is_finite():
                        equal = a == b or (a.is_nan() and b.is_nan())
                    else: equal = abs(a - b) <= gate
                    if not equal: failures.append({"row": i, "key": key, "source": str(a), "rust": str(b), "gate": str(gate)})
        else: failures.append({"kind": "execution"})
        record["failures"] = failures
        (out / "comparison.json").write_text(json.dumps(record, indent=2) + "\n")
        records.append(record)
        print(name, len(failures), flush=True)
    (BUILD / profile / "receipt.json").write_text(json.dumps(records, indent=2) + "\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--profile", default="debug", choices=["debug", "release"])
    run(parser.parse_args().profile)
