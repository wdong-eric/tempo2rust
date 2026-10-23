#!/usr/bin/env python3
"""Probe defined table/parser departures using the hash-verified A2 library."""
import json
import re
import shutil
import subprocess
from pathlib import Path

from inventory import HERE, REFERENCE, ROOT, SOURCE, blob, sha


def main():
    reference = ROOT / "validation/reference"
    source = reference / "build/source"
    library = reference / "build/libtempo2-reference.a"
    manifest = json.loads(blob(reference, REFERENCE, "build_manifest.json"))
    assert sha(library.read_bytes()) == manifest["static_library_sha256"]
    build = HERE / "build/table-probe"
    build.mkdir(parents=True, exist_ok=True)
    # Compile the raw table source: A2's copy contains resource_fopen tracing.
    table_source = blob(ROOT / "tempo2", SOURCE, "tabulatedfunction.C")
    (build / "tabulatedfunction.C").write_bytes(table_source)
    source_hashes = {}
    for path in ("tabulatedfunction.h", "TKlongdouble.C", "TKlongdouble.float128.h", "tempo2.h"):
        actual = (source / path).read_bytes()
        raw = blob(ROOT / "tempo2", SOURCE, path)
        assert actual == raw or re.sub(rb"\$Id:[^$]*\$", b"$Id$", actual) == raw, path
        source_hashes[path] = {"pinned_blob_sha256": sha(raw), "a2_build_sha256": sha(actual),
                               "git_ident_expansion_only": actual != raw}
        if path == "tempo2.h":
            (build / path).write_bytes(raw)
    source_hashes["tabulatedfunction.C"] = sha(table_source)
    assert (source / "TKlongdouble.h").read_bytes() == blob(ROOT / "tempo2", SOURCE, "TKlongdouble.float128.h")
    fixtures = {
        "skipped_row": "# A B\n1 0\nbad row\n2 0\n",
        "all_duplicate": "# A B\n1 0\n1 1\n",
        "duplicate_end": "# A B\n1 0\n2 1\n2 2\n",
        "singleton_proxy": "# A B\n1 0\n",
        "decreasing_proxy": "# A B\n2 0\n1 0\n",
    }
    for name, text in fixtures.items():
        (build / (name + ".clk")).write_text(text)
    cc = shutil.which("g++-16")
    if cc is None:
        raise SystemExit("g++-16 required for the pinned reference profile")
    command = [cc, "-O2", "-std=gnu++11", "-DHAVE_CONFIG_H", "-I" + str(build), "-I" + str(source),
               str(HERE / "table_probe.cc"), str(build / "tabulatedfunction.C"), str(library), "-lquadmath", "-lgfortran", "-o", str(build / "probe")]
    compiled = subprocess.run(command, text=True, capture_output=True, check=True)
    executed = subprocess.run([str(build / "probe"), str(build), str(build / "result.json")],
                              text=True, capture_output=True, timeout=30, check=True)
    values = json.loads((build / "result.json").read_text())
    assert [(row["samples"], row["value"]) for row in values["tables"]] == [(2, "0"), (2, "NaN"), (3, "NaN"), (1, "0"), (2, "0")]
    assert values["parser"] == {"numeric_prefix": 1, "hex": 1, "nan_class": "NaN"}
    result = {"status": "PASS", "scope": "raw canonical table source compiled ahead of hash-verified A2 support library; unchanged TKlongdouble parser object; no Rust execution or whole timing validation",
              "source_revision": SOURCE, "reference_revision": REFERENCE,
              "library_sha256": sha(library.read_bytes()), "source_hashes": source_hashes,
              "probe_sha256": sha((HERE / "table_probe.cc").read_bytes()), "fixtures": fixtures,
              "compiler": subprocess.check_output([cc, "--version"], text=True).splitlines()[0],
              "command": command, "run_command": executed.args, "compile_stderr": compiled.stderr,
              "stdout": executed.stdout, "stderr": executed.stderr, "results": values}
    (HERE / "table_replay.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps(values, indent=2))


if __name__ == "__main__":
    main()
