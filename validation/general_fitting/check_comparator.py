"""Reject omitted/extra membership, cycle substitution, types and signed zeros."""
from pathlib import Path
from copy import deepcopy
import argparse, hashlib, json
from compare import compare

HERE = Path(__file__).resolve().parent
BUILD = HERE / "build/comparator-controls"


def check(native, candidate):
    BUILD.mkdir(parents=True, exist_ok=True)
    baseline = json.loads(candidate.read_text())
    assert not compare(native, candidate, BUILD / "baseline.json")["failures"]
    mutants = {}
    x = deepcopy(baseline)
    for c in x["cycles"]:
        c["systems"] = []
        c["prefit"]["pulsars"] = []
        c["postfit"]["pulsars"] = []
    mutants["empty_workflow"] = x
    x = deepcopy(baseline);x["cycles"] = [deepcopy(x["cycles"][0])] * len(x["cycles"])
    mutants["duplicated_first_cycle"] = x
    x = deepcopy(baseline)
    for c in x["cycles"]: c["systems"][0]["noise"] = []
    mutants["missing_noise"] = x
    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["noise"][0]["red"] = "-0"
    mutants["noise_zero_sign"] = x
    for name, path in [
        ("cycle", ["cycles"]), ("system", ["cycles", 0, "systems"]),
        ("pulsar", ["cycles", 0, "prefit", "pulsars"]),
        ("observation", ["cycles", 0, "postfit", "pulsars", 0, "observations"]),
        ("noise", ["cycles", 0, "systems", 0, "noise"]),
        ("column", ["cycles", 0, "systems", 0, "columns"]),
        ("matrix_row", ["cycles", 0, "systems", 0, "matrix"]),
        ("covariance_column", ["cycles", 0, "systems", 0, "solver", "covariance", 0]),
        ("update", ["cycles", 0, "systems", 0, "updates"]),
        ("whitening", ["cycles", 0, "systems", 0, "whitening"]),
        ("unscaled_error", ["cycles", 0, "systems", 0, "solver", "errors_unscaled"]),
        ("linearized_residual", ["cycles", 0, "systems", 0, "solver", "linearized_s"]),
    ]:
        for mutation in ("missing", "extra"):
            x = deepcopy(baseline);a = x
            for key in path: a = a[key]
            if mutation == "missing": a.pop()
            else: a.append(deepcopy(a[-1]))
            mutants[mutation + "_" + name] = x
    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["noise"][0]["red"] = 0
    mutants["numeric_type"] = x
    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["noise"][0]["red"] = "NaN"
    mutants["numeric_class"] = x
    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["columns"][1]["parameter"] = "renamed"
    mutants["renamed_column"] = x
    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["updates"][1]["column"]["parameter"] = "renamed"
    mutants["renamed_physical_update"] = x
    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["updates"][1]["after_value"] = None
    mutants["missing_physical_value"] = x
    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["updates"][1]["increment"] = "1e100"
    mutants["changed_update_increment"] = x
    # New observers distinguish unwritten ZERO storage from a defined zero and
    # check its physical result in addition to the duplicate fitted increment.
    if "defined_value.0" in native.read_text():
        for field, replacement in [("before_value", "0"), ("after_value", "1e100"), ("after_error", None)]:
            x = deepcopy(baseline);x["cycles"][0]["systems"][0]["updates"][0][field] = replacement
            mutants["zero_physical_" + field] = x
    if "\tpostfit_update\t" in native.read_text():
        for j in (0, 1):
            for field in ("postfit_value", "postfit_error"):
                for replacement in (None, "1e100"):
                    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["updates"][j][field] = replacement
                    mutants[f"postfit_{j}_{field}_{replacement}"] = x
    records = []
    for name, x in mutants.items():
        path = BUILD / (name + ".json");path.write_text(json.dumps(x) + "\n")
        result = compare(native, path, BUILD / (name + ".comparison.json"))
        assert result["failures"], name + " falsely passed"
        records.append({"name": name, "failures": len(result["failures"])})
    # Comparison-only class controls: change both independent reference and
    # candidate to a defined class, then require its opposite sign to fail.
    for native_value, same, opposite in [
        ("NaN", {"class": "nan"}, {"class": "-nan"}),
        ("-NaN", "-NaN", "NaN"),
        ("Infinity", {"class": "+infinity"}, {"class": "-infinity"}),
        ("-Infinity", "-Infinity", "Infinity"),
        ("-0", "-0", "0"),
    ]:
        name = "class_sign_" + native_value
        ref = BUILD / (name + ".tsv")
        lines = native.read_text().splitlines()
        idx = next(i for i, line in enumerate(lines) if line.startswith("1\tnoise\t0\t0\tred\t"))
        lines[idx] = "1\tnoise\t0\t0\tred\t" + native_value
        ref.write_text("\n".join(lines) + "\n")
        path = BUILD / (name + ".json")
        x = deepcopy(baseline);x["cycles"][0]["systems"][0]["noise"][0]["red"] = same
        path.write_text(json.dumps(x) + "\n")
        assert not compare(ref, path, BUILD / (name + ".same.json"))["failures"]
        x["cycles"][0]["systems"][0]["noise"][0]["red"] = opposite
        path.write_text(json.dumps(x) + "\n")
        result = compare(ref, path, BUILD / (name + ".opposite.json"))
        assert len(result["failures"]) == 1, name
        records.append({"name": name, "failures": len(result["failures"])})
    sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
    (BUILD / "receipt.json").write_text(json.dumps({"native": str(native), "candidate": str(candidate), "native_sha256": sha(native), "candidate_sha256": sha(candidate), "comparator_sha256": sha(HERE / "compare.py"), "controls": records}, indent=2) + "\n")
    print("baseline passes;", len(records), "mutants rejected")


if __name__ == "__main__":
    p = argparse.ArgumentParser()
    p.add_argument("--native", type=Path, default=HERE / "build/public/release/spin/native.tsv")
    p.add_argument("--candidate", type=Path, default=HERE / "build/public/release/spin/rust.json")
    a = p.parse_args();check(a.native, a.candidate)
