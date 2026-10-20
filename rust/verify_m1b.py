#!/usr/bin/env python3
"""Reproduce M1b agreement/recovery evidence without modifying A2 or M1a artifacts."""
import argparse
from decimal import Decimal
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent
OUTPUT = ROOT / "verification/m1b"
CONTRACT = ROOT / "tests/terrestrial"


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--runtime", required=True, type=Path)
    args = parser.parse_args()
    runtime = args.runtime.resolve()
    OUTPUT.mkdir(parents=True, exist_ok=True)
    env = os.environ.copy()
    env.setdefault("CARGO_HOME", str(ROOT / ".cargo-home"))
    manifest = json.loads((CONTRACT / "manifest.json").read_text())
    for path, expected in manifest["resources"].items():
        actual = hashlib.sha256((runtime / path).read_bytes()).hexdigest()
        assert actual == expected["sha256"], f"runtime identity mismatch: {path}"
    commands = []

    def run(name, command, **kwargs):
        result = subprocess.run(command, cwd=ROOT, env=env, text=True,
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, **kwargs)
        (OUTPUT / (name + ".log")).write_text(result.stdout)
        commands.append({"command": [str(s) for s in command], "returncode": result.returncode})
        (OUTPUT / "commands.json").write_text(json.dumps(commands, indent=2) + "\n")
        print(f"{name}: {'PASS' if result.returncode == 0 else 'FAIL'}", flush=True)
        if result.returncode:
            raise RuntimeError(result.stdout)
        return result.stdout

    for name, command in [
        ("rustc", ["rustc", "-Vv"]), ("cargo", ["cargo", "-V"]),
        ("cc", [env.get("CC", "gcc-16"), "--version"]), ("fc", [env.get("FC", "gfortran-16"), "--version"]),
        ("dependencies", ["cargo", "tree", "--locked", "--offline"]),
        ("format", ["cargo", "fmt", "--check"]),
        ("tests", ["cargo", "test", "--locked", "--offline"]),
        ("clippy", ["cargo", "clippy", "--all-targets", "--locked", "--offline", "--", "-D", "warnings"]),
        ("release_resource_tests", ["cargo", "test", "--release", "--locked", "--offline",
                                    "--lib", "resource_tests"]),
        ("release_build", ["cargo", "build", "--release", "--locked", "--offline", "--bins", "--examples"]),
    ]:
        run(name, command)

    run("terrestrial", ["cargo", "run", "--locked", "--offline", "--example", "compare_terrestrial", "--",
                        runtime, "target/terrestrial-comparison"])
    run("release_terrestrial", ["cargo", "run", "--release", "--locked", "--offline", "--example",
                                "compare_terrestrial", "--", runtime, "target/terrestrial-release-comparison"])
    release_equivalence = {}
    for name in ("terrestrial.json", "terrestrial_4ns.json", "cli-equivalent.json", "recovery.json"):
        debug = (ROOT / "target/terrestrial-comparison" / name).read_bytes()
        release = (ROOT / "target/terrestrial-release-comparison" / name).read_bytes()
        assert debug == release, f"debug/release output differs: {name}"
        release_equivalence[name] = {"byte_identical": True, "sha256": hashlib.sha256(debug).hexdigest()}
    (OUTPUT / "release_equivalence.json").write_text(json.dumps(release_equivalence, indent=2) + "\n")
    for profile in ("debug", "release"):
        with tempfile.TemporaryDirectory(prefix=f"m1b-{profile}-", dir=ROOT / "target") as scratch:
            directory = Path(scratch) / "checks"
            suffix = "" if profile == "debug" else "_release"
            run("resource_recovery" + suffix, [ROOT / "target" / profile / "examples/check_terrestrial_resources",
                                              runtime, directory, ROOT / "target" / profile / "tempo2-rust"])
            shutil.copyfile(directory / "checks.json", OUTPUT / ("resource_checks" + suffix + ".json"))

    comparator = module("terrestrial_compare", CONTRACT / "compare.py")
    summary = {}
    for fixture in ("terrestrial", "terrestrial_4ns"):
        actual_path = ROOT / "target/terrestrial-comparison" / (fixture + ".json")
        actual = json.loads(actual_path.read_text())
        failures, measurements = comparator.compare(comparator.golden(fixture), actual)
        assert not failures, "\n".join(failures)
        maxima = {}
        for row in measurements:
            key = row["field"]
            if key not in maxima or abs(Decimal(row["signed_difference"])) > abs(Decimal(maxima[key]["signed_difference"])):
                maxima[key] = row
        summary[fixture] = {"status": "PASS", "scalar_comparisons": len(measurements),
                            "exact_identity_flags_counts_clock_destinations_resources": "PASS", "maxima": maxima}
        if fixture == "terrestrial_4ns":
            failures, diagnostic = comparator.response_diagnostic(actual)
            assert not failures, failures
            summary[fixture]["bat_response_4ns"] = diagnostic
        shutil.copyfile(actual_path, OUTPUT / actual_path.name)
        (OUTPUT / (fixture + "_signed_errors.json")).write_text(json.dumps(measurements, indent=2) + "\n")
        run(fixture + "_contract_mutations", ["python3", CONTRACT / "test_contract.py", actual_path])

    # Original comparator and its copied goldens remain byte-for-byte unchanged.
    barycentric = module("barycentric_compare", ROOT / "tests/reference/compare.py")
    for fixture in ("barycentric", "barycentric_4ns", "long_span"):
        expected = json.loads((ROOT / "tests/reference/outputs" / (fixture + ".json")).read_text())
        actual = json.loads((ROOT / "target/reference-comparison" / (fixture + ".json")).read_text())
        assert not barycentric.compare(expected, actual)
        summary[fixture] = {"unchanged_M1a_comparator": "PASS"}
    for name in ("cli-equivalent.json", "recovery.json"):
        shutil.copyfile(ROOT / "target/terrestrial-comparison" / name, OUTPUT / name)

    # Public CLI error contract: no complete-looking JSON escapes a failed call.
    binary = ROOT / "target/debug/tempo2-rust"
    par = CONTRACT / "fixtures/test1.par"
    tim = CONTRACT / "fixtures/test2.tim"
    with tempfile.TemporaryDirectory(prefix="m1b-cli-", dir=ROOT / "target") as scratch:
        scratch = Path(scratch)
        cases = [
            ("missing-runtime", par, tim, []),
            ("invalid-runtime", par, tim, ["--runtime", scratch / "absent"]),
        ]
        for name, content in [
            ("unknown-observatory", "FORMAT 1\nbad 1400 50000 10 unknown\n"),
            ("mixed-sites", "FORMAT 1\na 1400 50000 10 @\nb 1400 50001 10 pks\n"),
            ("outside-coverage", "FORMAT 1\na 1400 50000 10 pks\nb 1400 80000 10 pks\n"),
        ]:
            path = scratch / (name + ".tim")
            path.write_text(content)
            cases.append((name, par, path, ["--runtime", runtime]))
        for name, old, new in [
            ("unsupported-ephemeris", "DE405", "DE414"), ("unsupported-clock", "TT(TAI)", "TT(BIPM)"),
            ("unsupported-model", "PSRJ", "BINARY DDH\nPSRJ"),
        ]:
            path = scratch / (name + ".par")
            path.write_text(par.read_text().replace(old, new))
            cases.append((name, path, tim, ["--runtime", runtime]))
        outcomes = []
        for name, p, t, options in cases:
            result = subprocess.run([binary, "residuals", "--par", p, "--tim", t, *options],
                                    cwd=ROOT, env=env, capture_output=True, text=True)
            assert result.returncode != 0 and not result.stdout, name
            outcomes.append({"case": name, "stdout_empty": True, "returncode": result.returncode, "stderr": result.stderr})
        good = subprocess.run([binary, "residuals", "--par", par, "--tim", tim, "--runtime", runtime],
                              cwd=ROOT, env=env, capture_output=True, text=True)
        assert good.returncode == 0
        doc = json.loads(good.stdout)
        assert doc["status"] == "completed" and len(doc["observations"]) == 7
        assert "TROP1" in good.stderr and "TROP2" in good.stderr
        outcomes.append({"case": "valid-after-errors", "complete_JSON": True, "weather_warnings_stderr": True})
        (OUTPUT / "cli_checks.json").write_text(json.dumps(outcomes, indent=2) + "\n")
    (OUTPUT / "numerical_errors.json").write_text(json.dumps(summary, indent=2) + "\n")
    sources = [ROOT / "Cargo.toml", ROOT / "Cargo.lock", ROOT / "build.rs", ROOT / "verify_m1b.py"]
    sources += sorted((ROOT / "src").glob("*.rs")) + sorted((ROOT / "native").rglob("*"))
    sources += sorted((ROOT / "examples").glob("*.rs"))
    sources += [p for p in sorted((ROOT / "tests").rglob("*")) if "__pycache__" not in p.parts]
    (OUTPUT / "source_hashes.json").write_text(json.dumps({
        str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sources if p.is_file()
    }, indent=2) + "\n")
    print("M1b stage gates, 4 ns response, resource/CLI recovery and M1a regressions: PASS", flush=True)


if __name__ == "__main__":
    main()
