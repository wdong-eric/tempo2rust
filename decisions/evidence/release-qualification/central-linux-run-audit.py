#!/usr/bin/env python3
"""Verify the retained first AL Linux artifact and summarize its original gates."""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("packet", type=Path)
parser.add_argument("--out", type=Path, required=True)
args = parser.parse_args()
packet = args.packet.resolve()
evidence = packet / "artifact"


def sha(path):
    with path.open("rb") as source:
        return hashlib.file_digest(source, "sha256").hexdigest()


run = json.loads((packet / "run.json").read_text())
assert run["id"] == 40903715685
assert run["head_sha"] == "b59a84024317653d92cfd757709b2beee3e766a3"
assert run["status"] == "completed" and run["conclusion"] == "failure"
artifacts = json.loads((packet / "artifacts.json").read_text())["artifacts"]
assert len(artifacts) == 1
artifact = artifacts[0]
assert artifact["id"] == 12912911728
assert artifact["workflow_run"]["head_sha"] == run["head_sha"]
assert artifact["digest"] == "sha256:" + sha(packet / "artifact.zip")
assert artifact["size_in_bytes"] == (packet / "artifact.zip").stat().st_size
manifest = json.loads((evidence / "manifest.json").read_text())
files = manifest["files"]
for name, entry in files.items():
    path = evidence / name
    assert path.is_file() and not path.is_symlink(), name
    assert path.stat().st_size == entry["bytes"], name
    assert sha(path) == entry["sha256"], name
assert {str(p.relative_to(evidence)) for p in evidence.rglob("*") if p.is_file()} == set(files) | {"manifest.json"}

matrix = json.loads((evidence / "linux/receipt.json").read_text())
assert matrix["candidate"] == run["head_sha"]
stages = matrix["stages"]
failed = [name for name, row in stages.items() if row["exit"] not in (0, None)]
blocked = [name for name, row in stages.items() if row["exit"] is None]
assert failed == matrix["failed"] and blocked == matrix["blocked"]
counts = dict(Counter("blocked" if r["exit"] is None else "pass" if r["exit"] == 0 else "fail" for r in stages.values()))
receipts = {name: json.loads((evidence / "linux" / name / "receipt.json").read_text())
            for name in ["rust-suite", "contraction", "fits", "boundaries", "ddh", "timing", "output", "package-audit", "source-audit"]}
report = {
    "scope": "Retained-artifact identity review; no new engine execution or milestone acceptance",
    "run": {key: run[key] for key in ["id", "head_sha", "status", "conclusion", "html_url", "created_at", "updated_at"]},
    "archive_sha256": sha(packet / "artifact.zip"),
    "manifest_sha256": sha(evidence / "manifest.json"),
    "verified_files": len(files), "verified_bytes": sum(row["bytes"] for row in files.values()),
    "stage_counts": counts, "failed": failed, "blocked": blocked, "pending": matrix["pending"],
    "timing": {key: receipts["timing"][key] for key in ["cases", "failures"]},
    "contraction": {"lomb": receipts["contraction"]["lomb"], "qr_cases": len(receipts["contraction"]["qr"]),
                    "qr_failed_cases": sum(bool(row["comparison"]["failures"]) for row in receipts["contraction"]["qr"])},
    "fit_failures": receipts["fits"]["failures"],
    "boundary_failures": receipts["boundaries"]["failures"],
    "ddh": {"fidelity_by_clock": {row["clock"]: row["fidelity_failures"] for row in receipts["ddh"]["records"]},
            "historical_ideal_failures": receipts["ddh"]["historical_ideal_failures"]},
    "output": receipts["output"]["summary"],
    "package_checks": receipts["package-audit"]["checked_files"],
    "package_mismatches": receipts["package-audit"]["mismatches"],
    "unchanged_rust_bodies": len(receipts["source-audit"]["unchanged_rust_bodies"]),
    "source_changes": {key: receipts["source-audit"][key] for key in ["changed_rust_bodies", "added_rust_bodies", "removed_rust_bodies"]},
    "receipt_sha256": {name: sha(evidence / "linux" / name / "receipt.json") for name in receipts},
    "recipe_sha256": sha(Path(__file__)),
}
with args.out.open("x") as target:
    target.write(json.dumps(report, indent=2) + "\n")
print(json.dumps({key: report[key] for key in ["verified_files", "verified_bytes", "stage_counts", "package_checks", "unchanged_rust_bodies"]}))
print("Review SHA-256:", sha(args.out))
