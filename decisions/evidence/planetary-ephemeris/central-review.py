"""Central Z review: fresh native cases/controls/observer; audit retained coverage."""

from pathlib import Path
import collections, gzip, hashlib, json, shutil, subprocess, sys, tempfile

sys.dont_write_bytecode = True
ROOT = Path("/Users/Eric.Dong/Downloads/tempo2rust")
HERE = ROOT / "validation/planetary_ephemeris"
DELIVERY = "d517acf"
base = Path("/tmp")
# Paths appear in diagnostics and fixed JPL filename buffers. Preserve byte width.
prefix_length = len(str(HERE)) - len(str(base)) - 1 - 8
assert prefix_length >= 2
OUT = Path(tempfile.mkdtemp(prefix="z-" + "z" * (prefix_length - 2), dir=base))
assert len(str(OUT).encode()) == len(str(HERE).encode())
print("OUT", OUT, flush=True)
sys.path.insert(0, str(HERE))
import inventory, profile, run

sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
load = lambda p: json.loads(p.read_text())
accepted = HERE / "outputs"
original_files = sorted(
    p for p in HERE.rglob("*") if p.is_file() and "__pycache__" not in p.parts
)
before = {str(p): sha(p) for p in original_files}
assert (
    subprocess.check_output(["git", "diff", DELIVERY, "--", str(HERE)], cwd=ROOT) == b""
)
inventory.check()
catalog = run.frozen_cases()
baseline = run.read_gzip(accepted / "reference.json.gz")
ids = [c["id"] for c in catalog]
run.compare(ids, [r["id"] for r in baseline], "case membership")
assert len(catalog) == 96 and sum(len(r["events"]) for r in baseline) == 3540
for c, r in zip(catalog, baseline):
    run.validate(c, r)
receipts = {
    n: load(accepted / (n + ".json"))
    for n in ("last_execution", "controls", "independent", "coverage", "observer_audit")
}
for n, r in receipts.items():
    assert r["status"] == "PASS", n
for kind, name in (
    ("ordinary", "last_execution"),
    ("coverage", "coverage"),
    ("observed", "observer_audit"),
):
    profile.actual_identity(HERE / "build" / kind / "probe", receipts[name]["profile"])
original = load(accepted / "manifest.json")
assert receipts["last_execution"]["reference_sha256"] == sha(
    accepted / "reference.json.gz"
)
assert receipts["last_execution"]["manifest_sha256"] == sha(accepted / "manifest.json")
assert receipts["last_execution"]["harness"] == run.harness()
# Preserve and audit the explicit verifier identity transitions, not a value rebase.
reb = load(accepted / "manifest_rebind.json")
assert reb["new_sha256"] == sha(accepted / "manifest.json")
assert reb["previous_sha256"] == sha(accepted / reb["previous_manifest"])
assert reb["unchanged_oracle_sha256"] == sha(accepted / "reference.json.gz")
prev = load(accepted / reb["previous_manifest"])
changed = [n for n, h in original["harness"].items() if prev["harness"][n] != h]
assert changed == reb["changed_harness"] == ["controls.py", "run.py"]
for k in ("cases_sha256", "resources", "frozen"):
    assert prev[k] == original[k]
first_rebind = load(accepted / "observer-extension/manifest_rebind.json")
assert first_rebind["unchanged_oracle_sha256"] == sha(accepted / "reference.json.gz")
# Reconstruct coverage from raw gcov and independently audit every recorded result.
cov = receipts["coverage"]
raw = run.read_gzip(HERE / "build/coverage/readEphemeris.gcov.json.gz")
f = next(v for v in raw["files"] if Path(v["file"]).name == "readEphemeris.C")
strip = lambda x: {k: v for k, v in x.items() if k not in ("classification", "reason")}
assert f["functions"] == cov["routines"] and f["lines"] == [
    strip(v) for v in cov["lines"]
]
edges = [
    dict(line=l["line_number"], index=i, **b)
    for l in f["lines"]
    for i, b in enumerate(l.get("branches", []))
]
assert edges == [strip(v) for v in cov["branches"]]
summary = dict(
    routines=len(f["functions"]),
    routines_entered=sum(v["execution_count"] > 0 for v in f["functions"]),
    lines=len(f["lines"]),
    lines_hit=sum(v["count"] > 0 for v in f["lines"]),
    branches=len(edges),
    branches_hit=sum(v["count"] > 0 for v in edges),
)
assert (
    summary
    == cov["summary"]
    == dict(
        routines=3,
        routines_entered=3,
        lines=207,
        lines_hit=207,
        branches=108,
        branches_hit=106,
    )
)
assert cov["ordinary_exact_cases"] == ids and cov["ordinary_differences"] == []
assert sha(Path(cov["command"][0])) == cov["gcov_sha256"]
by_id = {v["id"]: v for v in baseline}


def audit_raw(rows, folder, expected):
    run.compare(ids, [v["id"] for v in rows], "receipt membership")
    for r in rows:
        n = r["id"]
        events = gzip.decompress((folder / (n + ".jsonl.gz")).read_bytes())
        out = (folder / (n + ".stdout")).read_bytes()
        err = (folder / (n + ".stderr")).read_bytes()
        assert r["raw_sha256"] == {
            k: hashlib.sha256(v).hexdigest()
            for k, v in [("events", events), ("stdout", out), ("stderr", err)]
        }
        run.compare(
            expected[n]["events"],
            [json.loads(v) for v in events.decode().splitlines()],
            n + ".events",
        )
        run.compare(expected[n]["stdout"], out.decode("latin1"), n + ".stdout")
        run.compare(expected[n]["stderr"], err.decode("latin1"), n + ".stderr")
        c = next(c for c in catalog if c["id"] == n)
        assert r["case_sha256"] == hashlib.sha256(run.packed(c)).hexdigest()


audit_raw(
    run.read_gzip(accepted / "coverage_receipts.json.gz"),
    accepted / "coverage_raw",
    by_id,
)
ordinary_receipts = run.read_gzip(accepted / "execution_receipts.json.gz")
audit_raw([r["first"] for r in ordinary_receipts], accepted / "check_raw", by_id)
audit_raw([r["replay"] for r in ordinary_receipts], accepted / "replay_raw", by_id)
old_observed = run.read_gzip(accepted / "observed.json.gz")
assert (
    sha(accepted / "observed.json.gz")
    == receipts["observer_audit"]["observed_sha256"]
    == receipts["independent"]["observed_sha256"]
)
obs_by_id = {v["id"]: v for v in old_observed}
observer_receipts = run.read_gzip(accepted / "observer_receipts.json.gz")
audit_raw([r["first"] for r in observer_receipts], accepted / "observer_raw", obs_by_id)
audit_raw(
    [r["replay"] for r in observer_receipts],
    accepted / "observer_replay_raw",
    obs_by_id,
)
print(
    "PASS delivered identities, rebind history, complete ordinary/observer raw records and coverage",
    summary,
    flush=True,
)
# Only the declared build root changes. All numerical/state/diagnostic bytes stay exact.
old_build = HERE / "build"
new_build = OUT / "build"


def rebind(x, old=old_build, new=new_build):
    if isinstance(x, str):
        return x.replace(str(old), str(new))
    if isinstance(x, list):
        return [rebind(v, old, new) for v in x]
    if isinstance(x, dict):
        return {rebind(k, old, new): rebind(v, old, new) for k, v in x.items()}
    return x


expected = rebind(baseline)
expected_observed = rebind(old_observed)
run.write_gzip(OUT / "reference.json.gz", expected)
shutil.copytree(accepted / "frozen", OUT / "frozen")
profile.BUILD = new_build
run.BUILD = new_build
run.OUTPUT = OUT
resource_manifest = profile.resources()
run.compare(rebind(original["resources"]), resource_manifest, "fresh resources")
exe, fresh = profile.compile_probe()
old_macros = (old_build / "ordinary/macros.txt").read_bytes()
new_macros = (new_build / "ordinary/macros.txt").read_bytes()
assert hashlib.sha256(old_macros).hexdigest() == original["native"]["macros_sha256"]
assert sorted(old_macros.splitlines()) == sorted(new_macros.splitlines())
a = rebind(profile.stable(original["native"]))
b = profile.stable(fresh)
a.pop("macros_sha256")
b.pop("macros_sha256")
assert a == b, [k for k in set(a) | set(b) if a.get(k) != b.get(k)]
manifest = rebind(original)
manifest["native"] = profile.stable(fresh)
inventory.save(OUT / "manifest.json", manifest)
print(
    "PASS fresh profile, equal path widths, complete macro-line and exact resource identities",
    flush=True,
)
central_receipts = []
for c, wanted in zip(catalog, expected):
    actual, r = run.execute(c, exe, fresh, OUT / "ordinary_raw", resource_manifest)
    run.validate(c, actual)
    run.compare(wanted, actual, c["id"])
    again, rr = run.execute(c, exe, fresh, OUT / "replay_raw", resource_manifest)
    run.validate(c, again)
    run.compare(actual, again, c["id"] + ".replay")
    central_receipts.append(dict(first=r, replay=rr))
    if len(central_receipts) % 16 == 0:
        print("PASS fresh central ordinary/replay", len(central_receipts), flush=True)
run.write_gzip(OUT / "execution_receipts.json.gz", central_receipts)
import controls, observer_audit, independent

controls.main()
control = load(OUT / "controls.json")
assert control["status"] == "PASS" and len(control["controls"]) == 31
assert collections.Counter(v["status"] for v in control["controls"]) == {
    "PASS": 1,
    "REJECTED": 30,
}
mutants = run.read_gzip(OUT / "mutations.json.gz")
assert len(mutants) == 18
rebased_by_id = {r["id"]: r for r in expected}
for i, m in enumerate(mutants):
    assert m["name"] == controls.MUTATIONS[i][0]
    profile.actual_identity(new_build / f"mutant{i:02}" / "probe", m["profile"])
    assert (
        controls.differences(rebased_by_id[m["case"]], m["result"]) == m["differences"]
    )
observer_audit.main()
obs = load(OUT / "observer_audit.json")
assert (
    obs["status"] == "PASS"
    and obs["cases"]
    == obs["ordinary_projection_exact"]
    == obs["observer_repeat_exact"]
    == 96
)
run.compare(
    expected_observed,
    run.read_gzip(OUT / "observed.json.gz"),
    "delivered complete observer",
)
independent.main()
ind = load(OUT / "independent.json")
for key in ("status", "counts", "max_absolute_errors", "limits"):
    run.compare(receipts["independent"][key], ind[key], "independent." + key)
inventory.check()
profile.actual_identity(exe, fresh)
assert {str(p): sha(p) for p in original_files} == before
assert set(before) == {
    str(p) for p in HERE.rglob("*") if p.is_file() and "__pycache__" not in p.parts
}
result = dict(
    status="PASS",
    scope="Fresh central ordinary and full observer replay, controls, compiled mutations and independent checks; delivered raw coverage audited, not rerun",
    delivery_revision=DELIVERY,
    central_revision=subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
    ).strip(),
    output=str(OUT),
    script_sha256=sha(Path(__file__)),
    cases=96,
    events=3540,
    controls=31,
    compiled_mutations=18,
    observer_cases=96,
    coverage=summary,
    independent=ind,
    original_reference_sha256=sha(accepted / "reference.json.gz"),
    fresh_profile=fresh,
    macro_identity=dict(
        delivered=hashlib.sha256(old_macros).hexdigest(),
        fresh=hashlib.sha256(new_macros).hexdigest(),
        complete_sorted_lines=hashlib.sha256(
            b"\n".join(sorted(new_macros.splitlines()))
        ).hexdigest(),
    ),
    build_root_projection=dict(
        original=str(old_build), fresh=str(new_build), equal_byte_width=True
    ),
    protected_z_files=before,
    protected_prior_count=len(load(accepted / "frozen/protected.json")),
    accepted_receipt_hashes={n: sha(accepted / (n + ".json")) for n in receipts},
    fresh_receipt_hashes={
        n: sha(OUT / n)
        for n in (
            "execution_receipts.json.gz",
            "controls.json",
            "mutations.json.gz",
            "observed.json.gz",
            "observer_receipts.json.gz",
            "observer_audit.json",
            "independent.json",
        )
    },
)
inventory.save(OUT / "central-review.json", result)
print(
    "PASS central Z: 96 cases, 31 controls, 18 mutations and complete observer/independent agreement",
    flush=True,
)
print(
    "RECEIPT", OUT / "central-review.json", sha(OUT / "central-review.json"), flush=True
)
