#!/usr/bin/env python3
"""Replay D's existing deterministic JPL cache probe with both pinned sources."""
import json
import platform
import shutil
import subprocess
from pathlib import Path

from inventory import BASE, HERE, ROOT, SOURCE, blob, sha


def main():
    cc = shutil.which("gcc-16")
    if cc is None:
        raise SystemExit("gcc-16 required to replay the recorded compiler profile")
    build = HERE / "build/native-replay"
    build.mkdir(parents=True, exist_ok=True)
    previous = json.loads(blob(ROOT, BASE, "rust/verification/m1b/native_source_probe.json"))
    probe = previous["probe_source"].encode()
    (build / "probe.c").write_bytes(probe)
    ephemeris = blob(ROOT / "tempo2", SOURCE, "T2runtime/ephemeris/DE405.1950.2050")
    (build / "truncated.de405").write_bytes(ephemeris[:-4000])
    results = {}
    for label, repo, revision, prefix in (("original", ROOT / "tempo2", SOURCE, ""),
                                           ("retained", ROOT, BASE, "rust/native/")):
        directory = build / label
        directory.mkdir(exist_ok=True)
        for path in ("jpleph.c", "jpleph.h", "jpl_int.h"):
            (directory / path).write_bytes(blob(repo, revision, prefix + path))
        command = [cc, "-O2", "-I", str(directory), str(build / "probe.c"), str(directory / "jpleph.c"), "-o", str(directory / "probe")]
        compile_result = subprocess.run(command, text=True, capture_output=True, check=True)
        execution = subprocess.run([str(directory / "probe"), str(build / "truncated.de405")], text=True, capture_output=True, timeout=30, check=True)
        observed = execution.stdout.strip()
        expected = previous[label]["good_first_failure_repeated_failure_recovery_delta"]
        assert observed == expected, (label, observed, expected)
        assert sha((directory / "jpleph.c").read_bytes()) == previous[label]["jpleph_sha256"]
        results[label] = {"command": command, "run_command": execution.args, "stdout": observed,
                          "compile_stderr": compile_result.stderr, "stderr": execution.stderr,
                          "jpleph_sha256": sha((directory / "jpleph.c").read_bytes())}
    layout_source = '''#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include "jpl_int.h"
int main(void) {
    printf("{\\"au_offset\\":%zu,\\"double_alignment\\":%zu,\\"struct_size\\":%zu}\\n",
           offsetof(struct jpl_eph_data, au), _Alignof(double), sizeof(struct jpl_eph_data));
    return 0;
}
'''
    (build / "layout.c").write_text(layout_source)
    layout_command = [cc, "-std=c11", "-I", str(build / "retained"), str(build / "layout.c"), "-o", str(build / "layout")]
    subprocess.run(layout_command, capture_output=True, text=True, check=True)
    layout = json.loads(subprocess.check_output([str(build / "layout")], text=True))
    assert layout["au_offset"] == 28
    layout["au_offset_aligned_for_double"] = layout["au_offset"] % layout["double_alignment"] == 0
    layout["command"] = layout_command
    layout["probe_source"] = layout_source
    layout["scope"] = "layout only; no unaligned getter execution or sanitizer result"
    result = {"status": "PASS", "scope": "native kernel cache reproducer; not full Rust timing validation",
              "platform": platform.platform(), "compiler": subprocess.check_output([cc, "--version"], text=True).splitlines()[0],
              "probe_source_sha256": sha(probe), "original_resource_sha256": sha(ephemeris),
              "truncated_resource_sha256": sha(ephemeris[:-4000]), "truncated_bytes": 4000,
              "source_revision": SOURCE, "rust_revision": BASE, "results": results, "native_layout": layout}
    (HERE / "native_replay.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
