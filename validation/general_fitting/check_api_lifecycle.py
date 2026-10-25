"""Public preparation wiring and same-context error recovery; not a native oracle."""
from pathlib import Path
import argparse
import json
import subprocess
from run_public import ROOT, HERE, RUNTIME, FIT_INPUTS, sha


def run(profile):
    out = HERE / "build/api-lifecycle" / profile
    out.mkdir(parents=True, exist_ok=True)
    executables = ROOT / "rust/target" / profile / "examples"
    records = []
    for name, par in [
        ("all", FIT_INPUTS / "all.par"),
        ("prior", HERE / "inputs/parameter-prior.par"),
        ("red", HERE / "inputs/red-noise.par"),
        ("recovery", HERE / "inputs/bootstrap.par"),
        ("automatic", FIT_INPUTS / "f0.par"),
    ]:
        directory = out / name
        directory.mkdir(exist_ok=True)
        recovery = name == "recovery"
        exe = executables / ("fitting_covariance" if name == "automatic" else "fitting_recovery" if recovery else "fitting_prepared")
        tim = FIT_INPUTS / ("pks.tim" if recovery else "bary.tim")
        cmd = [str(exe), str(RUNTIME), str(par), str(tim)]
        if recovery:
            cmd.append(str(directory))
        if name == "automatic": cmd.extend([str(directory), "recovery"])
        proc = subprocess.run(cmd, cwd=directory, capture_output=True, timeout=120)
        (directory / "stdout").write_bytes(proc.stdout)
        (directory / "stderr").write_bytes(proc.stderr)
        assert proc.returncode == 0, (name, proc.stderr.decode())
        result = json.loads(proc.stdout)
        if name == "automatic": assert result["failure_recovery_checked"]
        if recovery:
            assert {r["case"] for r in result} == {
                "missing-covariance", "diagnostic-write", "bootstrap-write",
                "bootstrap-capacity", "spectral-cutoff",
            }
        records.append({"case": name, "command": cmd, "exit": proc.returncode,
                        "inputs": {str(p): sha(p) for p in [par, tim]},
                        "executable_sha256": sha(exe), "stdout_sha256": sha(directory / "stdout"),
                        "result": result})
        print(name, "passed", flush=True)
    (out / "receipt.json").write_text(json.dumps({
        "profile": profile, "builder_sha256": sha(__file__), "records": records,
        "contract": "Exact same-engine preparation and postfit state; explicit failures followed by valid calls",
    }, indent=2) + "\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--profile", choices=["debug", "release"], default="debug")
    run(parser.parse_args().profile)
