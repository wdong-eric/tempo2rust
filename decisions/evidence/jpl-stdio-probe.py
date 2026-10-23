#!/usr/bin/env python3
"""Central design experiment; no production JPL implementation.

Compare canonical fopen with opening once, reading for provenance, rewinding,
and transferring the descriptor to fdopen. Reuse H's declared ordinary cases
in private temporary storage; never rewrite H's source or evidence.
"""
import hashlib
import importlib.util
import json
from pathlib import Path
import platform
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
PIN = "fda6ad4a9753ad7dc5d291406f2f310a9161423f"
HERE = ROOT / "validation/jpl"

# Inserted into a disposable copy of H's observational harness, before its
# unchanged canonical jpleph.c include. The only alternate operation is opening
# and reading the same descriptor before C stdio takes ownership.
ADAPTER = r'''
static FILE *central_owned_open(const char *path, const char *mode) {
    if (strcmp(mode, "rb")) return fopen(path, mode);
    int fd = open(path, O_RDONLY);
    if (fd < 0) return NULL;
    unsigned char buffer[8192];
    ssize_t n;
    while ((n = read(fd, buffer, sizeof buffer)) > 0) {}
    if (n < 0 || lseek(fd, 0, SEEK_SET) < 0) { close(fd); return NULL; }
    FILE *stream = fdopen(fd, mode);
    if (!stream) close(fd);
    return stream;
}
#define fopen central_owned_open
'''


def main():
    spec = importlib.util.spec_from_file_location("h_reference", HERE / "run.py")
    reference = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(reference)
    oracle = json.loads((HERE / "outputs/reference.json").read_text())
    cc = shutil.which("gcc-16")
    assert cc
    harness = (HERE / "probe.c").read_bytes()
    assert hashlib.sha256(harness).hexdigest() == "a67a1e3c2a294f44a1c0c6156974445fd9d061786947a9462fa005c7dfdd9e6a"
    marker = b'#include "jpleph.c"'
    assert harness.count(marker) == 1
    alternate = harness.replace(marker, ADAPTER.encode() + marker + b"\n#undef fopen")
    with tempfile.TemporaryDirectory(prefix="tempo2-central-jpl-stdio-") as scratch:
        directory = Path(scratch)
        reference.INPUTS = directory / "inputs"
        reference.INPUTS.mkdir()
        sources = {}
        for name in ("jpleph.c", "jpleph.h", "jpl_int.h"):
            blob = subprocess.check_output(["git", "-C", str(ROOT / "tempo2"), "show", f"{PIN}:{name}"])
            (directory / name).write_bytes(blob)
            sources[name] = hashlib.sha256(blob).hexdigest()
        manifest = reference.inputs()
        cases = reference.scenarios(manifest)
        observations = {}
        for name, code in (("canonical", harness), ("owned_fdopen", alternate)):
            source = directory / f"{name}.c"
            source.write_bytes(code)
            exe = directory / name
            subprocess.run([cc, "-O2", "-std=c11", "-I", str(directory), str(source), "-lm", "-o", str(exe)], check=True, capture_output=True)
            observations[name] = {key: reference.execute(exe, case) for key, case in cases.items()}
            reference.compare(observations[name], oracle)
        reference.compare(observations["canonical"], observations["owned_fdopen"])
        partial = observations["canonical"]["partial_cache_le"]
        states = [event for event in partial if event["op"] == "state"]
        item = manifest["extended_le.bin"]
        result = {
            "purpose": "Central host-scoped I/O architecture experiment; full state values remain H host observations subject to the recorded C undefined behavior.",
            "source_commit": PIN,
            "source_sha256": sources,
            "harness_sha256": hashlib.sha256(harness).hexdigest(),
            "adapter_harness_sha256": hashlib.sha256(alternate).hexdigest(),
            "experiment_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
            "h_reference_sha256": hashlib.sha256((HERE / "outputs/reference.json").read_bytes()).hexdigest(),
            "h_runner_sha256": hashlib.sha256((HERE / "run.py").read_bytes()).hexdigest(),
            "h_cases_sha256": hashlib.sha256((HERE / "cases.json").read_bytes()).hexdigest(),
            "h_inputs_sha256": hashlib.sha256((HERE / "inputs.json").read_bytes()).hexdigest(),
            "compiler": subprocess.check_output([cc, "--version"], text=True).splitlines()[0],
            "flags": ["-O2", "-std=c11", "-lm"],
            "platform": platform.platform(),
            "cases": len(cases),
            "events": sum(map(len, observations["canonical"].values())),
            "canonical_vs_h_reference": "exact structured equality",
            "owned_fdopen_vs_h_reference": "exact structured equality",
            "owned_fdopen_vs_canonical": "exact structured equality",
            "adjacent_short_read": {
                "input_sha256": item["sha256"],
                "record_bytes": item["specification"]["recsize"],
                "bytes_left_on_disk_in_record": 137,
                "size_requested_returned": states[1]["last_fread_size_requested_returned"],
                "first_three_statuses": [event["status"] for event in states[:3]],
                "cache_identical_on_retry": states[1]["cache_bytes"] == states[2]["cache_bytes"],
            },
            "limits": "No Rust wrapper, concurrent mutation, alternate host/libc or allocation-equivalence claim; the preparatory read models a same-descriptor provenance pass but does not hash inside the C adapter.",
        }
        print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
