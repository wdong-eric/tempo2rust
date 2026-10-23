#!/usr/bin/env python3
"""Build/check Assignment F's fixed-revision source inventory; no live Rust reads."""
import argparse
import copy
import csv
import hashlib
import json
import platform
import re
import subprocess
from collections import Counter, defaultdict
from functools import lru_cache
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE = "fda6ad4a9753ad7dc5d291406f2f310a9161423f"
BASE = "ddc07b2"
POLICY = "1322f8b"
REFERENCE = "7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572"
TARGETS = ("libtempo2_la_SOURCES", "libt2toolkit_la_SOURCES", "libtempo2pred_la_SOURCES")


def git(repo, *args):
    return subprocess.check_output(["git", "-C", str(repo), *args])


@lru_cache(None)
def blob(repo, revision, path):
    return git(repo, "show", f"{revision}:{path}")


def decoded(repo, revision, path):
    return blob(repo, revision, path).decode("utf-8", errors="replace")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def tsv(name):
    with (HERE / name).open() as stream:
        return list(csv.DictReader(stream, delimiter="\t"))


def assignments(text):
    # These pinned source-list definitions use only = and the expansions below.
    text = text.replace("\\\n", " ")
    return dict(re.findall(r"(?m)^(\w+)\s*=\s*([^\n]*)", text))


def expand(value, variables):
    def replace(match):
        key, substitution = (match[1].split(":", 1) + [None])[:2]
        tokens = expand(variables[key], variables)
        if substitution:
            old, new = substitution.split("=", 1)
            tokens = [token[:-len(old)] + new if token.endswith(old) else token for token in tokens]
        return " ".join(tokens)
    expanded = re.sub(r"\$\(([^)]+)\)", replace, value)
    assert "$" not in expanded, expanded
    return expanded.split()


def location(repo, revision, value):
    path, line = value.rsplit(":", 1)
    line = int(line)
    data = blob(repo, revision, path)
    lines = data.decode("utf-8", errors="replace").splitlines()
    assert 1 <= line <= len(lines), value
    return {"path": path, "line": line, "sha256": sha(data), "anchor": lines[line - 1].strip()}


def locations(repo, revision, value):
    return [] if value == "-" else [location(repo, revision, part.strip()) for part in value.split(";")]


def check_make_expansion(variables, names, expected, prefix=""):
    # Independent GNU make expansion catches errors in the small Python expander.
    definitions = "\n".join(f"{key} = {value}" for key, value in variables.items())
    display = "\n".join(f"$(info {name}=$({name}))" for name in names)
    makefile = definitions + "\n" + display + "\n.PHONY: inventory\ninventory:\n\t@:\n"
    result = subprocess.run(["make", "--no-print-directory", "-f", "-", "inventory"],
                            input=makefile, text=True, capture_output=True, check=True, cwd=HERE)
    expanded = {line.split("=", 1)[0]: [prefix + p for p in line.split("=", 1)[1].split()]
                for line in result.stdout.splitlines() if "=" in line}
    assert expanded == {name: expected[name] for name in names}, "GNU make expansion mismatch"


def lexical_index(text):
    """Navigation only: not a C/C++ parser, reachability proof or full call graph."""
    functions = []
    # Mask comments and quoted text while retaining line numbers for signatures.
    masked = re.sub(r'''//[^\n]*|/\*[\s\S]*?\*/|"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*' ''',
                    lambda m: re.sub(r"[^\n]", " ", m[0]), text, flags=re.VERBOSE)
    lines = masked.splitlines()
    for index, line in enumerate(lines):
        prefix, paren, rest = line.partition("(")
        words = prefix.strip().split()
        if not paren or not words or not re.fullmatch(r"[\w\s*:<>&]+", prefix):
            continue
        name = words[-1].lstrip("*")
        if name in {"if", "while", "switch", "for", "catch"} or words[0] in {"return", "else"}:
            continue
        if len(words) == 1 and (index == 0 or lines[index - 1].strip() not in {"void", "double", "int", "longdouble"}):
            continue
        signature = "(" + rest + "\n" + "\n".join(lines[index + 1:index + 20])
        if re.match(r"\([^;{}]*\)\s*\{", signature):
            functions.append({"name": name, "line": index + 1})
    for match in re.finditer(r"(?im)^\s*(?:DOUBLE PRECISION FUNCTION|SUBROUTINE)\s+(\w+)", text):
        functions.append({"name": match[1], "line": text.count("\n", 0, match.start()) + 1})
    parameters = defaultdict(list)
    includes = []
    conditions = []
    keywords = []
    for number, line in enumerate(text.splitlines(), 1):
        for name in sorted(set(re.findall(r"\bparam_\w+", line))):
            parameters[name].append(number)
        if re.match(r"\s*#\s*include", line):
            includes.append({"line": number, "text": line.strip()})
        if re.match(r"\s*#\s*(?:if|elif|else|endif)", line):
            conditions.append({"line": number, "text": line.strip()})
        if any(x in line for x in ("strcmp(", "strcasecmp(", "strncasecmp(", "strncmp(")):
            keywords.append({"line": number, "text": line.strip()})
    return {"function_definition_candidates": functions, "parameter_occurrences": dict(parameters),
            "include_directives": includes, "preprocessor_branches": conditions,
            "string_dispatch_lines": keywords}


def build():
    source_repo = ROOT / "tempo2"
    reference_repo = ROOT / "validation/reference"
    make = decoded(source_repo, SOURCE, "Makefile.am")
    variables = assignments(make)
    expanded = {name: expand(variables[name], variables) for name in TARGETS}
    check_make_expansion({key: variables[key] for key in (*TARGETS, "FITFUNCS", "headers", "BUILT_SOURCES")}, TARGETS, expanded)
    sofa_variables = assignments(decoded(source_repo, SOURCE, "sofa/Makefile.am"))
    expanded["libsofa_la_SOURCES"] = ["sofa/" + p for p in expand(sofa_variables["libsofa_la_SOURCES"], sofa_variables)]
    check_make_expansion({"libsofa_la_SOURCES": sofa_variables["libsofa_la_SOURCES"]}, ("libsofa_la_SOURCES",), expanded, "sofa/")
    memberships = defaultdict(list)
    for library, paths in expanded.items():
        assert len(paths) == len(set(paths)), f"duplicate expanded sources in {library}"
        for path in paths:
            memberships[path].append(library)
    source_names = {p for p in memberships if p.endswith((".C", ".c"))}
    mappings = tsv("source_map.tsv")
    assert len({r["source"] for r in mappings}) == len(mappings), "duplicate curated source keys"
    assert {r["source"] for r in mappings} == source_names, ("source map != expanded lists", source_names ^ {r["source"] for r in mappings})
    by_source = {r["source"]: r for r in mappings}
    batches = json.loads((HERE / "batches.json").read_text())
    batch_ids = {b["id"] for b in batches}
    assert len(batch_ids) == len(batches)
    for batch in batches:
        assert set(batch["depends_on"]) <= batch_ids
    generated = {
        "enum_str.C": ["extract_enums.sh", "tempo2.h"],
        "enum_str.h": ["extract_enums.sh", "tempo2.h"],
        "TKcholesky.h": ["cholesky.h"],
        "TKlongdouble.h": ["TKlongdouble.float128.h", "TKlongdouble.ld.h"],
        "t2help.h": ["convertxt.c", "help.txt", "help_extra.txt"],
    }
    rust_files = git(ROOT, "ls-tree", "-r", "--name-only", BASE, "rust").decode().splitlines()
    records, headers = [], []
    for path in sorted(memberships):
        is_source = path.endswith((".C", ".c", ".for"))
        record = {"source": path, "libraries": memberships[path], "generated": path in generated}
        if path in generated:
            record["generation_inputs"] = [{"path": p, "sha256": sha(blob(source_repo, SOURCE, p))} for p in generated[path]]
            record["sha256"] = None
            record["source_evidence"] = [location(source_repo, SOURCE, "Makefile.am:185" if path.startswith("enum_str") else "Makefile.am:21")]
        else:
            data = blob(source_repo, SOURCE, path)
            record["sha256"] = sha(data)
            text = data.decode("utf-8", errors="replace")
            record["line_count"] = len(text.splitlines())
            record["lexical_features"] = lexical_index(text)
            entries = record["lexical_features"]["function_definition_candidates"]
            record["source_evidence"] = [location(source_repo, SOURCE, f"{path}:{entry['line']}") for entry in entries[:4]] or [location(source_repo, SOURCE, f"{path}:1")]
        if is_source:
            if path.startswith("sofa/"):
                native = "rust/native/" + path
                retained = native in rust_files
                record.update(batch="astrometry-time", status="retained_native" if retained else "native_not_retained",
                              responsibility_and_remaining_work="Bundled SOFA dependency; " + ("byte-identical routine in selected IAU2000B closure. Native retention needs central final disposition; fixture use is not routine-wide validation." if retained else "outside retained closure; needed by additional source branches or full public SOFA disposition, not silently excluded."),
                              rust_locations=locations(ROOT, BASE, native + ":1") if retained else [])
                if retained:
                    assert record["sha256"] == sha(blob(ROOT, BASE, native)), native
            else:
                row = by_source[path]
                record.update({k: v for k, v in row.items() if k not in {"source", "rust_locations"}})
                record["rust_locations"] = locations(ROOT, BASE, row["rust_locations"])
            assert record["batch"] in batch_ids
            record["completion_disposition"] = "open: implementation/native disposition and central fidelity evidence required"
            record["proof_required"] = next(b["proof"] for b in batches if b["id"] == record["batch"])
            records.append(record)
        else:
            record["disposition"] = "shared source declarations/configuration; no standalone Rust C-ABI compatibility claimed"
            headers.append(record)
    sofa_paths = {Path(r["source"]).stem.upper(): r["source"] for r in records if r["source"].startswith("sofa/")}
    sofa_graph = {}
    for stem, path in sofa_paths.items():
        code = "\n".join(line for line in decoded(source_repo, SOURCE, path).splitlines() if not line.startswith(("*", "C", "c", "!")))
        names = set(re.findall(r"\biau_(\w+)\b", code, re.I))
        sofa_graph[path] = sorted(sofa_paths[name.upper()] for name in names if name.upper() != stem)
        next(r for r in records if r["source"] == path)["native_dependencies"] = sofa_graph[path]
    pending = [sofa_paths[name] for name in ("C2T00B", "POM00", "RXP", "SXP", "PXP", "TRXPV", "TRXP")]
    closure = set()
    while pending:
        path = pending.pop()
        if path not in closure:
            closure.add(path)
            pending.extend(sofa_graph[path])
    assert closure == {r["source"] for r in records if r["status"] == "retained_native"}, "retained SOFA closure mismatch"
    modules = tsv("module_map.tsv")
    prod = {p for p in rust_files if p == "rust/build.rs" or (p.startswith("rust/src/") and p.endswith(".rs") and p != "rust/src/resource_tests.rs")}
    assert {r["path"] for r in modules} == prod, "production module coverage mismatch"
    assert len(modules) == len(prod)
    for row in modules:
        row["sha256"] = sha(blob(ROOT, BASE, row["path"]))
        row["header_first_five_lines"] = decoded(ROOT, BASE, row["path"]).splitlines()[:5]
        row["source_locations"] = locations(source_repo, SOURCE, row.pop("counterparts"))
    unsafe = tsv("unsafe_sites.tsv")
    actual_unsafe = {(p, n) for p in prod for n, line in enumerate(decoded(ROOT, BASE, p).splitlines(), 1) if re.search(r"\bunsafe\b", line) and not line.lstrip().startswith("//")}
    assert {(r["path"], int(r["line"])) for r in unsafe} == actual_unsafe
    assert len(unsafe) == len(actual_unsafe)
    for row in unsafe:
        row["line"] = int(row["line"])
        row["site"] = location(ROOT, BASE, f"{row['path']}:{row['line']}")
        row["native_locations"] = locations(source_repo, SOURCE, row.pop("native_counterpart"))
    ledger = tsv("behavior_ledger.tsv")
    assert len({row["id"] for row in ledger}) == len(ledger)
    for row in ledger:
        row["classification"] = row["classification"].split(";")
        row["source_locations"] = locations(source_repo, SOURCE, row["source_locations"])
        row["rust_locations"] = locations(ROOT, BASE, row["rust_locations"])
    evidence = []
    native_manifest = json.loads(blob(ROOT, BASE, "rust/native/upstream_manifest.json"))["files"]
    for path, digest in native_manifest.items():
        assert sha(blob(source_repo, SOURCE, path)) == digest, path
        if path != "jpleph.c":
            assert sha(blob(ROOT, BASE, "rust/native/" + path)) == digest, path
    accepted_sources = json.loads(blob(ROOT, BASE, "rust/verification/m1b/source_hashes.json"))
    for path, digest in accepted_sources.items():
        assert sha(blob(ROOT, BASE, "rust/" + path)) == digest, path
    equivalence = json.loads(blob(ROOT, BASE, "rust/verification/m1b/resource_correction_equivalence.json"))
    equivalent_paths = []
    for row in equivalence["outputs"]:
        path = row["path"].replace("target/reference-comparison/", "verification/")
        assert row["before_sha256"] == row["after_sha256"] == sha(blob(ROOT, BASE, "rust/" + path))
        equivalent_paths.append({"recorded_path": row["path"], "verified_baseline_path": "rust/" + path})
    evidence_paths = {
        "primary": (ROOT, BASE, ["rust/M1A_REPORT.md", "rust/M1B_REPORT.md", "rust/Cargo.lock", "rust/build.rs", "rust/native/departures.patch", "rust/native/upstream_manifest.json", "rust/verification/m1b/source_hashes.json", "rust/verification/m1b/native_source_probe.json", "rust/verification/m1b/resource_checks.json", "rust/verification/m1b/resource_checks_release.json", "rust/verification/m1b/resource_correction_equivalence.json", "rust/verification/m1b/release_equivalence.json", "rust/verification/numerical_errors.json", "rust/verification/m1b/numerical_errors.json"]),
        "reference": (reference_repo, REFERENCE, ["REPORT.md", "initializer-correction.patch", "initialization_probe.cc", "checks/results.json", "checks/ddh.json", "checks/initialization-empty.log", "checks/initialization-samples.log", "build_manifest.json"]),
        "uncorrected_reference": (reference_repo, "5289797", ["initialization_probe.cc", "checks/initialization-probe.log"]),
        "policy": (ROOT, POLICY, ["HANDOFF.md", "handoffs/06-core-fidelity-inventory.md", "decisions/0006-m1b-acceptance.md", "decisions/0007-m2-fitting-contract-and-e-release.md", "decisions/0008-preservation-policy-and-stage-plan.md", "decisions/evidence/0005-m1b-review.json"]),
    }
    for kind, (repo, revision, paths) in evidence_paths.items():
        for path in paths:
            evidence.append({"repository": kind, "revision": git(repo, "rev-parse", revision).decode().strip(), "path": path, "sha256": sha(blob(repo, revision, path))})
    other_sources = {
        "tempo2.C": "core executable orchestration, separately audited below; not a library translation unit",
        "getPeriod.C": "installed predictor client tool (Makefile.am:99); its libtempo2pred dependency remains core",
        "convertxt.c": "noninstalled build helper generating help data (Makefile.am:19,197)",
        "age.C": "root source absent from pinned library/program source lists; recorded as unbuilt, not silently counted as transferred",
    }
    root_units = {p for p in git(source_repo, "ls-tree", "--name-only", SOURCE).decode().splitlines() if p.endswith((".C", ".c"))}
    assert root_units - source_names == set(other_sources), "unclassified root translation unit"
    return {"schema_version": 1, "source_revision": SOURCE, "rust_revision": git(ROOT, "rev-parse", BASE).decode().strip(),
            "policy_revision": git(ROOT, "rev-parse", POLICY).decode().strip(), "reference_revision": REFERENCE,
            "scope": "expanded three root libraries including generated enum_str.C, all bundled SOFA sources, plus core executable orchestration; plugins/UI/tools separately classified",
            "index_limits": "lexical_features are source navigation, including inactive preprocessor branches and some commented text; not AST, exhaustive function enumeration, dynamic call graph or feature acceptance. File membership is exhaustive and independently checked against pinned lists.",
            "build_definitions": [location(source_repo, SOURCE, "Makefile.am:37"), location(source_repo, SOURCE, "Makefile.am:40"), location(source_repo, SOURCE, "Makefile.am:42"), location(source_repo, SOURCE, "Makefile.am:44"), location(source_repo, SOURCE, "Makefile.am:78"), location(source_repo, SOURCE, "sofa/Makefile.am:6")],
            "expanded_build_lists": expanded, "sources": records, "headers": headers,
            "core_executable": {"source": "tempo2.C", "sha256": sha(blob(source_repo, SOURCE, "tempo2.C")), "lexical_features": lexical_index(decoded(source_repo, SOURCE, "tempo2.C")), "rust_locations": locations(ROOT, BASE, "rust/src/main.rs:1"), "status": "partial adapter; full orchestration and prediction/fit commands absent"},
            "other_root_sources": [{"source": path, "sha256": sha(blob(source_repo, SOURCE, path)), "disposition": note} for path, note in other_sources.items()],
            "conditional_dependencies": [
                {"name": name, "source_locations": locations(source_repo, SOURCE, refs), "baseline_disposition": note}
                for name, refs, note in [
                    ("SOFA", "Makefile.am:48;sofa/Makefile.am:6", "Always linked by core; Rust retains verified 36-file closure out of 121 bundled files."),
                    ("OpenMP", "Makefile.am:52;configure.ac:142", "calculate_bclt per-object flags; disabled in A2 and not a Rust parallel timing implementation."),
                    ("LAPACK/BLAS", "Makefile.am:55;Makefile.am:60;T2accel.C:17", "Optional accelerated paths present in source, disabled in reference and absent in baseline Rust."),
                    ("CALCEPH", "Makefile.am:65;readEphemeris_calceph.C:41", "Both enabled code and disabled failure stubs remain indexed; absent in baseline Rust."),
                    ("float128/native/QD", "Makefile.am:164;Makefile.am:91;configure.ac:65", "Reference uses float128 and quadmath; Rust Rug128 is a different fixed representation. QD/native alternatives unverified."),
                    ("libm/Fortran/C++ runtime", "configure.ac:57;Makefile.am:88", "Native build/link dependencies; actual recorded compiler/runtime profile in A2 and Rust manifests."),
                    ("GSL", "configure.ac:195;Makefile.am:48", "Configure detection exists but root core LIBADD has no GSL link entry; do not infer core GSL dependency from plugin support."),
                    ("plugins/UI/tests/tools", "Makefile.am:4;Makefile.am:16;Makefile.am:87;Makefile.am:118;Makefile.am:128;Makefile.am:141", "Separate targets/subdirectories; core entrypoint and embedded fit/output wrappers still require disposition. No plugin/GUI binary-compatibility claim.")]],
            "historical_artifact_checks": {"m1b_source_manifest_entries": len(accepted_sources), "native_manifest_entries": len(native_manifest), "unchanged_correction_outputs": len(equivalence["outputs"]), "equivalence_paths": equivalent_paths},
            "production_modules": modules, "unsafe_sites": unsafe, "behavior_ledger": ledger, "batches": batches, "evidence": evidence}


def check_inventory(actual, expected):
    assert len({s["source"] for s in actual["sources"]}) == len(actual["sources"]), "duplicate inventory sources"
    assert actual == expected, "inventory differs from fixed source/build lists or curated audit inputs; inspect before --write"


def render_reports(data):
    legend = ("All `S:path:line` references mean raw canonical Git blobs at `" + SOURCE + "`; "
              "all `R:path:line` mean primary baseline `" + data["rust_revision"] + "`. "
              "Use `git -C tempo2 show COMMIT:path | nl -ba` or `git show COMMIT:path | nl -ba`. "
              "Live Rust files may already differ because E is active. Generated from the checked JSON/curated TSV; no milestone acceptance is implied.\n\n")
    def refs(values, prefix):
        return ", ".join(f"`{prefix}:{v['path']}:{v['line']}`" for v in values) or "—"
    def cell(value):
        return value.replace("|", "\\|").replace("\n", " ")
    sources = ["# Pinned source inventory\n\n", legend,
               "Status describes baseline implementation, not acceptance. Every row remains open for final disposition. "
               "The JSON also includes full expanded lists, hashes, generated-source inputs, direct include directives, "
               "parameter and string-dispatch occurrences, conditional branches and lexical function candidates. "
               "These indexes are navigation aids, not an AST or exhaustive branch-validation claim.\n\n",
               "| Root source | Library membership | Baseline | Evidence / counterpart | Responsibility and remaining transfer |\n| --- | --- | --- | --- | --- |\n"]
    for row in data["sources"]:
        if not row["source"].startswith("sofa/"):
            evidence = refs(row["source_evidence"][:2], "S") + "; " + refs(row["rust_locations"], "R")
            sources.append(f"| `{row['source']}` | {', '.join(row['libraries'])} | {row['status']} | {evidence} | {cell(row['responsibility_and_remaining_work'])} Batch `{row['batch']}`. |\n")
    sources.extend(["\n## Bundled SOFA: complete list\n\n", "36 retained routines are the checked transitive closure of C2T00B, POM00, RXP, SXP, PXP, TRXPV and TRXP. "
                    "All 121 routines stay visible; the remaining 85 need future native/core dependency disposition, not necessarily a Rust reimplementation. "
                    "Retained files are byte-identical raw blobs.\n\n",
                    "| Source | Baseline | Direct native dependencies |\n| --- | --- | --- |\n"])
    for row in data["sources"]:
        if row["source"].startswith("sofa/"):
            sources.append(f"| `{row['source']}` | {row['status']} | {', '.join(row['native_dependencies']) or 'none'} |\n")
    sources.extend(["\n## Associated headers and generated artifacts\n\n",
                    "`Makefile.am:40` shares headers broadly; association below is build membership rather than an assertion that every translation unit includes every header. Direct includes are indexed separately.\n\n",
                    "| Header | Libraries | Generated from |\n| --- | --- | --- |\n"])
    for row in data["headers"]:
        sources.append(f"| `{row['source']}` | {', '.join(row['libraries'])} | {', '.join(v['path'] for v in row.get('generation_inputs', [])) or 'pinned source blob'} |\n")
    sources.extend(["\n## Other root sources and conditional dependencies\n\n"])
    for row in data["other_root_sources"]:
        sources.append(f"- `{row['source']}`: {row['disposition']}.\n")
    sources.append("\n")
    for row in data["conditional_dependencies"]:
        sources.append(f"- **{row['name']}**: {row['baseline_disposition']} {refs(row['source_locations'], 'S')}.\n")
    correspondence = ["# Production correspondence and unsafe audit\n\n", legend,
                      "The 19-file production Rust set includes build.rs, lib.rs and the CLI. resource_tests.rs is cfg(test); examples/tests are evidence, not production modules. "
                      "There are eight unsafe sites: two extern declarations and six blocks, no unsafe fn/impl. Dependencies' internal Rust unsafe is outside this crate-site enumeration; retained C/Fortran obligations are described below.\n\n",
                      "| File | Header assessment | Actual source counterparts | Finding |\n| --- | --- | --- | --- |\n"]
    for row in data["production_modules"]:
        correspondence.append(f"| `{row['path']}` | {row['status']} | {refs(row['source_locations'], 'S')} | {cell(row['finding'])} |\n")
    correspondence.extend(["\n## U1: JPL file identity is an unenforced safety precondition\n\n",
        "`R:rust/src/runtime.rs:215` reads/hashes bytes and passes them to preflight; `R:rust/src/read_ephemeris.rs:152` then calls native initialization which independently opens the path (`S:jpleph.c:704`). "
        "Neither the Rust borrow nor the mutex locks the resource file. The validated image need not equal the subsequently opened/native-read bytes if another actor changes it. "
        "Therefore the current structural-validation safety argument assumes stable file contents/identity. This is a source-inspected gap, not an executed race or proof that ordinary pinned fixtures fail. "
        "The later owner needs a concrete stable-input/native-boundary contract or implementation plus evidence; documenting a SAFETY label alone cannot enforce this invariant. F changes no implementation.\n\n",
        "`NonNull<c_void>` has no unsafe Send/Sync implementation in the baseline, so JplEphemeris and RuntimeContext cannot be transferred/shared through safe Rust threads. "
        "Separate contexts can be constructed in separate threads; the mutex covers native init/global error access. &mut self serializes calls per handle. "
        "SOFA's -frecursive and immutable DATA tables support independent calls, but there is no concurrency stress result.\n\n"])
    correspondence.extend(["## U2: packed native AU getter lacks portable double alignment\n\n",
        "`S:jpl_int.h:30` packs jpl_eph_data; its au member is at offset 28. `S:jpleph.c:75` casts base+field to double* and dereferences it; "
        "`R:rust/src/read_ephemeris.rs:160` requests field 28. F's layout-only probe reports double alignment 8 and struct size 648 on the actual host. "
        "A calloc-aligned base therefore does not align base+28 for this typed dereference. The wrapper cannot establish a portable C alignment contract merely from NonNull/owned allocation. "
        "This retained-source risk is separate from U1 and from ordinary successful fixture execution; F ran no sanitizer or deliberately unaligned getter probe and makes no crash claim. "
        "Record a central native-boundary disposition under preservation policy rather than silently repairing source. Evidence: native_replay.json native_layout and ledger B29.\n\n"])
    for number, row in enumerate(data["unsafe_sites"], 1):
        correspondence.extend([f"## Unsafe site {number}: R:{row['path']}:{row['line']} ({row['kind']})\n\n",
            f"Native: {refs(row['native_locations'], 'S')}.\n\n",
            "Why unsafe / safe replacement: " + row["safe_replacement"] + "\n\n",
            "Actual contract: " + row["actual_invariants"] + "\n\n",
            "Adjacent explanation: " + row["comment_assessment"] + "\n\n"])
    correspondence.append("## Other misleading source descriptions\n\n`R:rust/src/ifteph.rs:133` says upper dates reach unchecked reads. "
                          "The source's header reads are unchecked (`S:ifteph.C:150`), but its coefficient read is checked and exits on a short read (`S:ifteph.C:266`). "
                          "The malformed-grid conversion and upper-coverage adapter need distinct explanations. Native files preserve original headers; their provenance/corrections are recorded in native/upstream_manifest.json, departures.patch and README.md, rather than per-file Rust counterpart headers.\n")
    behavior = ["# Behavior-difference ledger\n\n", legend,
                "Evidence levels are stated per entry. Fresh F execution covers the native JPL cache and defined table/parser cases only; "
                "baseline Rust behavior is inspected against its code and historical tests. Existing acceptance reports are bounded historical evidence. "
                "No old test, reference artifact, algorithm or acceptance threshold is changed.\n\n"]
    labels = (("legacy_behavior", "Canonical behavior"), ("rust_or_reference_behavior", "Baseline/reference difference"),
              ("reproducer_and_evidence", "Reproducer and evidence"), ("tests_encoding_departure", "Existing test expectation"),
              ("confidence_and_deferred_work", "Confidence and next evidence"))
    for row in data["behavior_ledger"]:
        behavior.extend([f"## {row['id']}: {', '.join(row['classification'])}\n\n",
                         refs(row["source_locations"], "S") + "; " + refs(row["rust_locations"], "R") + ".\n\n"])
        for key, label in labels:
            behavior.append(f"**{label}.** {row[key]}\n\n")
    plan = ["# Candidate dependency batches\n\n", legend,
            "These are planning candidates, not approved exclusions or new assignments. Every root source has one batch; SOFA dependencies are attached to astrometry-time. "
            "E owns the current fitting window. Existing-departure reconciliation follows central review after that window. "
            "Each batch requires its proof below plus each source row's remaining features. Partial files stay open after a bounded batch passes.\n\n",
            "| Batch | Prerequisites | Root source count | Scope and required proof |\n| --- | --- | ---: | --- |\n"]
    for row in data["batches"]:
        count = sum(s["batch"] == row["id"] and not s["source"].startswith("sofa/") for s in data["sources"])
        plan.append(f"| `{row['id']}` | {', '.join(row['depends_on']) or 'none'} | {count} | {cell(row['scope'])}. {cell(row['proof'])} |\n")
    return {name: "".join(parts).rstrip() + "\n" for name, parts in (
        ("SOURCE_INVENTORY.md", sources), ("CORRESPONDENCE.md", correspondence),
        ("BEHAVIOR_DIFFERENCES.md", behavior), ("STAGE_PLAN.md", plan))}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--write", action="store_true", help="regenerate JSON from pinned blobs and reviewed TSV/batches")
    args = parser.parse_args()
    expected = build()
    output = HERE / "inventory.json"
    if args.write:
        output.write_text(json.dumps(expected, indent=2, sort_keys=True) + "\n")
    for name, report in render_reports(expected).items():
        if args.write:
            (HERE / name).write_text(report)
        assert (HERE / name).read_text() == report, f"stale readable audit: {name}"
    check_inventory(json.loads(output.read_text()), expected)
    # Real contract checks: omitted/duplicated build row and dangling mapping must fail.
    mutations = []
    for case in ("omitted_source", "duplicate_source", "dangling_rust_mapping"):
        altered = copy.deepcopy(expected)
        if case == "omitted_source":
            altered["sources"].pop()
        elif case == "duplicate_source":
            altered["sources"].append(altered["sources"][0])
        else:
            next(r for r in altered["sources"] if r["rust_locations"])["rust_locations"][0]["path"] = "rust/src/nonexistent.rs"
        try:
            check_inventory(altered, expected)
        except AssertionError:
            mutations.append(case)
        else:
            raise AssertionError(f"mutation accepted: {case}")
    sources = expected["sources"]
    result = {"status": "PASS", "kind": "static inventory consistency, not timing validation",
              "source_revision": SOURCE, "rust_revision": expected["rust_revision"],
              "counts": {"root_library_translation_units": sum(not s["source"].startswith("sofa/") for s in sources),
                         "sofa_translation_units": sum(s["source"].startswith("sofa/") for s in sources),
                         "unique_headers": len(expected["headers"]), "production_rust_files": len(expected["production_modules"]),
                         "unsafe_sites": len(expected["unsafe_sites"]), "statuses": dict(Counter(s["status"] for s in sources))},
              "independent_expansion": "GNU make matched all four library source lists including generated FITFUNCS headers",
              "historical_artifact_checks": expected["historical_artifact_checks"],
              "negative_checks": mutations, "python": platform.python_version(), "platform": platform.platform(),
              "inventory_sha256": sha(output.read_bytes()), "command": "python3 validation/coverage/inventory.py"}
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
