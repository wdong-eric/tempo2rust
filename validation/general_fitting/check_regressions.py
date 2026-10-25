"""Affected AC/AD and M1/M2 checks, using unchanged accepted comparison gates.

Only output destinations are redirected into AE staging. Original runners,
fixtures, reference artifacts and membership definitions remain read-only.
"""
from pathlib import Path
import hashlib, importlib.util, json, sys, tempfile

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
AD = ROOT / "validation/timing_models"
OUT = HERE / "build/regressions"
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()


def run():
    global OUT
    OUT.mkdir(parents=True, exist_ok=True)
    OUT = Path(tempfile.mkdtemp(prefix="run-", dir=OUT))
    sys.path.insert(0, str(AD))
    path = AD / "check_compatibility.py"
    spec = importlib.util.spec_from_file_location("ae_accepted_compatibility", path)
    compatibility = importlib.util.module_from_spec(spec);spec.loader.exec_module(compatibility)
    compatibility.OUT = OUT / "compatibility"
    compatibility_exit = compatibility.main()

    path = AD / "check_public.py"
    original = path.read_text()
    token = "work=HERE/f'build/public-{profile}/{name}-mean{mean}'"
    assert original.count(token) == 1
    redirected = original.replace(token, "work=AE_OUT/f'{profile}/{name}-mean{mean}'")
    namespace = {"__name__": "ae_accepted_ad_public", "__file__": str(path), "AE_OUT": OUT / "timing"}
    exec(compile(redirected, str(path), "exec"), namespace)
    selected = ["ordinary", "ecliptic", "events", "glitches", "jumps-fd", "ifunc-linear", "ifunc-sinc", "ifunc-step",
                "waves", "wave-dm-scale", "quad-interpolation", "quadrupole", "gw-single", "gw-memory", "gw-memory-polarizations",
                "gw-cosmic-string", "shapelet", "ell1h-h3", "library-noise", "library-state", "two-cut", "two-shapelet", "two-tzr"]
    records = []
    for profile in ["debug", "release"]:
        sys.argv = [str(path), *(["--release"] if profile == "release" else []), *[v for name in selected for v in ["--case", name]]]
        status = namespace["main"]()
        receipts = sorted((OUT / "timing" / profile).glob("*/comparison.json"))
        observed = {p.parent.name.rsplit("-mean", 1)[0] for p in receipts}
        assert observed == set(selected), ("AD case membership", observed.symmetric_difference(selected))
        records.append({"profile": profile, "exit": status, "cases": selected, "receipts": {str(p): sha(p) for p in receipts}})
    (OUT / "receipt.json").write_text(json.dumps({
        "builder_sha256": sha(__file__), "compatibility_comparator_sha256": sha(AD / "check_compatibility.py"),
        "timing_comparator_sha256": sha(path), "output_redirect": [token, "work=AE_OUT/f'{profile}/{name}-mean{mean}'"],
        "compatibility_exit": compatibility_exit, "records": records,
    }, indent=2) + "\n")
    print("Regression receipt:", OUT / "receipt.json")


if __name__ == "__main__":
    run()
