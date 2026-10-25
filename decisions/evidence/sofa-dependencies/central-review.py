#!/usr/bin/env python3
"""Read-only pinned-core SOFA closure review; writes only a new evidence directory."""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys

sys.dont_write_bytecode = True
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('workspace', type=Path)
parser.add_argument('output', type=Path)
parser.add_argument('--native-archive', type=Path, required=True)
args = parser.parse_args()
root = args.workspace.resolve()
out = args.output.resolve()
out.mkdir(parents=True, exist_ok=False)
spec = importlib.util.spec_from_file_location('coverage_inventory', root / 'validation/coverage/inventory.py')
inv = importlib.util.module_from_spec(spec)
spec.loader.exec_module(inv)
repo = root / 'tempo2'
pin = inv.SOURCE
sources = {}

def source(name):
    data = inv.blob(repo, pin, name)
    sources[name] = inv.sha(data)
    return data.decode()

def cpp_code(text):
    # These pinned call sites have no raw-string literals. Preserve line numbers.
    pattern = r'//[^\n]*|/\*.*?\*/|"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\''
    return re.sub(pattern, lambda match: '\n' * match[0].count('\n'), text, flags=re.S)

def closure(roots, graph):
    found = set()
    pending = list(roots)
    while pending:
        name = pending.pop()
        if name not in found:
            found.add(name)
            pending.extend(graph[name])
    return found

variables = inv.assignments(source('Makefile.am'))
expanded = {name: inv.expand(variables[name], variables) for name in inv.TARGETS}
inv.check_make_expansion(variables, inv.TARGETS, expanded)
root_units = {path for paths in expanded.values() for path in paths if path.endswith(('.C', '.c'))}
assert len(root_units) == 93
sofa_vars = inv.assignments(source('sofa/Makefile.am'))
sofa_files = inv.expand(sofa_vars['libsofa_la_SOURCES'], sofa_vars)
assert len(sofa_files) == len(set(sofa_files)) == 121
paths = {Path(name).stem.upper(): 'sofa/' + name for name in sofa_files}
graph = {}
static_declarations = {}
for name, path in paths.items():
    text = source(path)
    code = '\n'.join(line for line in text.splitlines() if line and line[0] not in '*Cc!')
    names = {item.upper() for item in re.findall(r'\biau_(\w+)\b', code, re.I)}
    assert name in names
    assert names <= paths.keys(), (path, names - paths.keys())
    graph[name] = sorted(names - {name})
    static_declarations[path] = [line.strip() for line in code.splitlines()
                                 if re.match(r'\s*(SAVE|COMMON|DATA|EQUIVALENCE)\b', line, re.I)]

all_root_paths = inv.git(repo, 'ls-tree', '--name-only', pin).decode().splitlines()
scanned = sorted(path for path in all_root_paths if path.endswith(('.C', '.c', '.h')))
calls = {}
declarations = {}
for path in scanned:
    text = source(path)
    decl_pattern = r'extern\s+"C"\s+long\s+iau_([a-z0-9]+)_\s*\([^;]*\);'
    declarations[path] = sorted({name.upper() for name in re.findall(decl_pattern, text, re.I)})
    without_declarations = re.sub(decl_pattern, lambda match: '\n' * match[0].count('\n'), text, flags=re.I)
    calls[path] = sorted({name.upper() for name in re.findall(r'\biau_([a-z0-9]+)_\s*\(', cpp_code(without_declarations), re.I)})
calls = {path: names for path, names in calls.items() if names}
declarations = {path: names for path, names in declarations.items() if names}
assert set(calls) == {'get_obsCoord.C'}, calls
source_roots = set(calls['get_obsCoord.C'])
assert source_roots == {'CP', 'POM00', 'RXP', 'SXP', 'PXP', 'C2T00B', 'TRXPV', 'TRXP'}
assert set(declarations['get_obsCoord.C']) - source_roots == {'C2T00A'}

# enum_str.C is the sole generated library translation unit. Reuse 0080 evidence.
enum_dir = root / 'decisions/evidence/core-output/build/preparation-enums-20261025'
enum_receipt = json.loads((enum_dir / 'receipt.json').read_text())
assert inv.sha((enum_dir / 'enum_str.C').read_bytes()) == enum_receipt['generated']['enum_str.C']
assert inv.sha(inv.blob(repo, pin, 'tempo2.h')) == enum_receipt['source_header_sha256']
assert inv.sha(inv.blob(repo, pin, 'extract_enums.sh')) == enum_receipt['generator_sha256']
assert not re.search(r'\biau_\w+\s*\(', cpp_code((enum_dir / 'enum_str.C').read_text()), re.I)
assert root_units - set(scanned) == {'enum_str.C'}

rust_path = root / 'rust/src/get_obs_coord.rs'
rust_text = rust_path.read_text()
rust_roots = {name.upper() for name in re.findall(r'\bfn iau_([a-z0-9]+)_\(', rust_text)}
assert rust_roots == source_roots - {'CP'}
retained = closure(rust_roots, graph)
assert retained == closure(source_roots, graph)
assert len(retained) == 36
manifest_path = root / 'rust/native/upstream_manifest.json'
manifest = json.loads(manifest_path.read_text())
assert manifest['source_commit'] == pin
manifest_routines = {Path(path).stem.upper() for path in manifest['files'] if path.endswith('.for')}
assert manifest_routines == retained
for path, digest in manifest['files'].items():
    assert inv.sha(inv.blob(repo, pin, path)) == digest, path
    assert inv.sha((root / 'rust/native' / path).read_bytes()) == digest, path
actual_routines = {path.stem.upper() for path in (root / 'rust/native/sofa').glob('*.for')}
assert actual_routines == retained

# Observe existing binaries, using immutable local copies; no native/Rust rebuild.
archives = {}
for label, archive in [('rust-native', args.native_archive),
                       ('reference', root / 'validation/reference/build/libtempo2-reference.a')]:
    data = archive.read_bytes()
    copied = out / (label + '.a')
    copied.write_bytes(data)
    listing = subprocess.check_output(['ar', 't', str(copied)], text=True)
    defined = subprocess.check_output(['nm', '-gU', str(copied)], text=True)
    undefined = subprocess.check_output(['nm', '-u', str(copied)], text=True)
    (out / (label + '-members.txt')).write_text(listing)
    (out / (label + '-defined.txt')).write_text(defined)
    (out / (label + '-undefined.txt')).write_text(undefined)
    assert inv.sha(archive.read_bytes()) == inv.sha(data), 'archive changed while observed'
    owners = {}
    current = None
    for line in undefined.splitlines():
        if line.endswith(':'):
            current = line[:-1]
        match = re.search(r'\b_iau_([a-z0-9]+)_\b', line)
        if match:
            assert current
            owners.setdefault(current, set()).add(match[1].upper())
    definitions = {name.upper() for name in re.findall(r'\bT _iau_([a-z0-9]+)_\b', defined)}
    if label == 'rust-native':
        assert definitions == retained
        # Darwin ar lists its symbol index alongside the object members.
        index_members = {'__.SYMDEF', '__.SYMDEF SORTED'}
        object_members = [member for member in listing.splitlines() if member not in index_members]
        assert all(member.endswith('.o') for member in object_members)
        assert {Path(member).stem.upper() for member in object_members} == retained
        assert {name for names in owners.values() for name in names} <= retained
    else:
        root_owners = {owner: names for owner, names in owners.items() if owner.endswith(('.C.o', '.c.o'))}
        assert root_owners == {'get_obsCoord.C.o': source_roots}, root_owners
    archives[label] = {'original_path': str(archive), 'sha256': inv.sha(data),
                       'defined_sofa': sorted(definitions),
                       'undefined_sofa_by_member': {owner: sorted(names) for owner, names in sorted(owners.items())}}

outside = set(paths) - retained
rows = ['source\tdisposition\tdirect_dependencies\tsha256']
for name in sorted(paths):
    disposition = 'retained_core_dependency' if name in retained else 'not_reached_by_pinned_core'
    rows.append('\t'.join([paths[name], disposition, ','.join(paths[dep] for dep in graph[name]), sources[paths[name]]]))
(out / 'sofa-dispositions.tsv').write_text('\n'.join(rows) + '\n')
inputs = {name: inv.sha((root / name).read_bytes()) for name in [
    'validation/coverage/inventory.py', 'rust/src/get_obs_coord.rs', 'rust/build.rs',
    'rust/native/upstream_manifest.json', 'decisions/0008-preservation-policy-and-stage-plan.md']}
assert inputs['decisions/0008-preservation-policy-and-stage-plan.md'] == '2c5f81b82c43105fcd48b62139f1d78f2dc7c4089055ea162e9feef82ff4e12c'
receipt = {'source_commit': pin, 'root_head': inv.git(root, 'rev-parse', 'HEAD').decode().strip(),
           'reviewer_sha256': inv.sha(Path(__file__).read_bytes()), 'command': sys.argv,
           'root_library_units': sorted(root_units), 'root_source_header_scan': scanned,
           'source_calls': calls, 'source_declarations': declarations,
           'rust_roots': sorted(rust_roots), 'retained': sorted(paths[name] for name in retained),
           'outside_core_closure': sorted(paths[name] for name in outside),
           'static_declarations': {name: rows for name, rows in static_declarations.items() if rows and name in {paths[item] for item in retained}},
           'source_sha256': sources, 'workspace_input_sha256': inputs,
           'generated_enum_sha256': enum_receipt['generated']['enum_str.C'], 'archives': archives,
           'dispositions_sha256': inv.sha((out / 'sofa-dispositions.tsv').read_bytes())}
(out / 'receipt.json').write_text(json.dumps(receipt, indent=2, sort_keys=True) + '\n')
print(json.dumps({'output': str(out), 'root_library_units': len(root_units), 'authored_root_scan': len(scanned),
                  'sofa_units': len(paths), 'retained': len(retained), 'outside_core_closure': len(outside),
                  'source_callers': list(calls), 'receipt_sha256': inv.sha((out / 'receipt.json').read_bytes()),
                  'dispositions_sha256': receipt['dispositions_sha256']}, indent=2))
