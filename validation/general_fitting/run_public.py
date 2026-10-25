"""Run independent AE public workflows and preserve identities and failed gates."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import subprocess
from compare import compare

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
INPUTS = HERE / "inputs"
RUNTIME = ROOT / "validation/reference/build/source/T2runtime"
FIT_INPUTS = ROOT / "validation/fitting/inputs"
CASES = [
    ("spin", [FIT_INPUTS / "all.par"], {}),
    ("independent", [FIT_INPUTS / "f0.par", FIT_INPUTS / "f1.par"], {"norescale": True}),
    ("global", [INPUTS / "global0.par", INPUTS / "global1.par"], {}),
    ("prior", [INPUTS / "parameter-prior.par"], {}),
    ("red", [INPUTS / "red-noise.par"], {}),
    ("covariance", [FIT_INPUTS / "all.par"], {"covariance": str(INPUTS / "covariance.model")}),
    *(("robust-" + kind, [INPUTS / ("robust-" + kind + ".par")], {}) for kind in "HBW"),
    *((name, [INPUTS / (name + ".par")], {}) for name in ("dm-noise", "chromatic-noise", "band-noise", "dm-log-noise", "chromatic-log-noise")),
]


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def run(profile="debug", names=()):
    build = HERE / "build/public" / profile
    build.mkdir(parents=True, exist_ok=True)
    rust = ROOT / "rust/target" / profile / "tempo2-rust"
    native = HERE / "build/native/fit"
    records = []
    for name, pars, options in CASES:
        if names and name not in names:
            continue
        dest = build / name
        dest.mkdir(exist_ok=True)
        tims = [FIT_INPUTS / "bary.tim"] * len(pars)
        env = {**os.environ, "TEMPO2": str(RUNTIME)}
        for key in ("AE_COVARIANCE", "AE_NORESCALE"):
            env.pop(key, None)
        cmd_r = [str(rust), "timing", "--fit", "--runtime", str(RUNTIME)]
        cmd_n = [str(native), str(dest / "native.tsv"), str(len(pars))]
        for par, tim in zip(pars, tims):
            cmd_r.extend(["--par", str(par), "--tim", str(tim)])
            cmd_n.extend([str(par), str(tim)])
        if options.get("norescale"):
            cmd_r.append("--norescale")
            env["AE_NORESCALE"] = "1"
        if options.get("covariance"):
            cmd_r.extend(["--covariance", options["covariance"]])
            env["AE_COVARIANCE"] = options["covariance"]
        record = {"name": name, "profile": profile, "options": options, "commands": [cmd_n, cmd_r], "inputs": {str(p): sha(p) for p in pars + tims}, "native_sha256": sha(native), "rust_sha256": sha(rust)}
        for engine, cmd in (("native", cmd_n), ("rust", cmd_r)):
            proc = subprocess.run(cmd, cwd=ROOT, env=env, capture_output=True, timeout=120)
            record[engine + "_exit"] = proc.returncode
            (dest / (engine + ".stderr")).write_bytes(proc.stderr)
            (dest / ("rust.json" if engine == "rust" else "native.stdout")).write_bytes(proc.stdout)
        if record["native_exit"] == record["rust_exit"] == 0:
            comparison = compare(dest / "native.tsv", dest / "rust.json", dest / "comparison.json")
            record["failures"] = len(comparison["failures"])
            record["comparison_sha256"] = sha(dest / "comparison.json")
        else:
            record["failures"] = "execution"
        (dest / "identity.json").write_text(json.dumps(record, indent=2) + "\n")
        records.append(record)
        print(name, record["failures"], flush=True)
    (build / ("receipt-" + "-".join(names) + ".json" if names else "receipt.json")).write_text(json.dumps(records, indent=2) + "\n")
    return records


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--profile", choices=("debug", "release"), default="debug")
    parser.add_argument("cases", nargs="*")
    args = parser.parse_args()
    run(args.profile, args.cases)
