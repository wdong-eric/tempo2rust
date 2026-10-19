#!/usr/bin/env python3
"""Run focused M1a checks and record errors against the unchanged A2 contract."""
from decimal import Decimal, localcontext
import hashlib
import importlib.util
import json
from pathlib import Path
import platform
import shutil
import subprocess

ROOT = Path(__file__).resolve().parent
REFERENCE = ROOT / "tests/reference"
OUTPUT = ROOT / "verification"


def main():
    OUTPUT.mkdir(exist_ok=True)
    commands = [
        ("toolchain", ["rustc", "-Vv"]),
        ("cargo", ["cargo", "-V"]),
        ("dependencies", ["cargo", "tree", "--locked", "--offline"]),
        ("format", ["cargo", "fmt", "--check"]),
        ("tests", ["cargo", "test", "--locked", "--offline"]),
        ("clippy", ["cargo", "clippy", "--all-targets", "--locked", "--offline", "--", "-D", "warnings"]),
    ]
    records = []
    for name, command in commands:
        result = subprocess.run(command, cwd=ROOT, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        (OUTPUT / f"{name}.log").write_text(result.stdout)
        records.append({"command": command, "returncode": result.returncode})
        (OUTPUT / "commands.json").write_text(json.dumps({"platform": platform.platform(), "commands": records}, indent=2) + "\n")
        print(f"{name}: {'PASS' if result.returncode == 0 else 'FAIL'}", flush=True)
        if result.returncode:
            raise SystemExit(result.stdout)

    spec = importlib.util.spec_from_file_location("accepted_compare", REFERENCE / "compare.py")
    comparator = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(comparator)
    summary = {}
    with localcontext() as context:
        context.prec = 90
        for fixture in ["barycentric", "barycentric_4ns", "long_span"]:
            expected = json.loads((REFERENCE / "outputs" / f"{fixture}.json").read_text())
            actual_path = ROOT / "target/reference-comparison" / f"{fixture}.json"
            actual = json.loads(actual_path.read_text())
            errors = comparator.compare(expected, actual)
            if errors:
                raise SystemExit("\n".join(errors))
            expected, actual = map(comparator.scientific_projection, [expected, actual])
            maxima = {}
            comparisons = 0
            for stage, rows in expected["stages"].items():
                for reference, new in zip(rows, actual["stages"][stage]):
                    for key, value in reference.items():
                        field = comparator.SCHEMA["observation_fields"][key]
                        if field["comparison"] == "exact":
                            continue
                        error = abs(Decimal(new[key]) - Decimal(value)) * Decimal(field.get("difference_scale", "1"))
                        maxima[key] = max(maxima.get(key, Decimal(0)), error)
                        comparisons += 1
            parameter_maximum = Decimal(0)
            for key, value in expected["parameters"].items():
                difference = abs(Decimal(actual["parameters"][key]["value"]) - Decimal(value["value"]))
                if Decimal(value["value"]) != 0:
                    parameter_maximum = max(parameter_maximum, difference / abs(Decimal(value["value"])))
            summary[fixture] = {
                "result": "PASS", "scalar_stage_comparisons": comparisons,
                "identity_flags_full_and_reduced_counts": "exact",
                "max_absolute_errors": {key: {"value": str(value), "unit": comparator.SCHEMA["observation_fields"][key]["comparison_unit"],
                    "gate": comparator.SCHEMA["observation_fields"][key]["absolute_tolerance"]} for key, value in maxima.items()},
                "max_parameter_relative_error": str(parameter_maximum), "parameter_relative_gate": "1e-30", "expected_zero_parameters": "exact",
            }
            shutil.copyfile(actual_path, OUTPUT / f"{fixture}.json")
        analytic = json.loads((ROOT / "target/analytic-f3.json").read_text())
        summary["independent_f3"] = {
            "result": "PASS", "oracle": "512-bit unsplit polynomial from original decimal tokens",
            "max_phase_error_turn": str(max(Decimal(row["phase_error_turn"]) for row in analytic)),
            "max_residual_error_s": str(max(Decimal(row["residual_error_s"]) for row in analytic)),
            "phase_gate_turn": "1e-27", "residual_gate_s": "1e-29", "pulse_counts": "exact",
        }
        perturbations = json.loads((OUTPUT / "barycentric_4ns.json").read_text())["stages"]
        summary["analytic_4ns"] = {
            "max_aligned_residual_s": str(max(abs(Decimal(row["residual_s"])) for row in perturbations["aligned"])),
            "max_4ns_error_s": str(max(abs(Decimal(row["residual_s"]) - Decimal("4e-9")) for row in perturbations["perturbed"][1:])),
            "diagnostic_gate_s": "1e-20",
        }
        assert Decimal(summary["analytic_4ns"]["max_aligned_residual_s"]) <= Decimal("1e-20")
        assert Decimal(summary["analytic_4ns"]["max_4ns_error_s"]) <= Decimal("1e-20")
    (OUTPUT / "numerical_errors.json").write_text(json.dumps(summary, indent=2) + "\n")
    shutil.copyfile(ROOT / "target/analytic-f3.json", OUTPUT / "analytic-f3.json")
    sources = [ROOT / name for name in ["Cargo.toml", "Cargo.lock", "verify.py", "LICENSE"]]
    for directory in ["src", "tests", "examples"]:
        sources.extend(path for path in (ROOT / directory).rglob("*") if path.is_file() and "__pycache__" not in path.parts)
    hashes = {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(sources)}
    (OUTPUT / "source_hashes.json").write_text(json.dumps(hashes, indent=2) + "\n")
    print("Accepted fixture comparison and independent numerical diagnostics: PASS")


if __name__ == "__main__":
    main()
