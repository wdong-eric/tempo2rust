#!/usr/bin/env python3
"""Compare complete frozen results and prepare clients before any timing window.

Run validation/release/qualify_predictor.py for each prefix first. This adds
strict before/after equality and mutable-storage observations to its existing
native gates, then observes allocations in a separate instrumented client.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT/'validation/release'))
from predictor_compare import native_number
from compare import phase_bound

if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['before', 'after', 'before-qualification', 'after-qualification', 'predictor', 'out']:
        p.add_argument('--'+name, type=Path, required=True)
    a = p.parse_args()
    out = a.out.resolve(); out.mkdir(parents=True, exist_ok=False)
    sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
    before, after = a.before_qualification.resolve(), a.after_qualification.resolve()
    receipts = [json.loads((path/'receipt.json').read_text()) for path in [before, after]]
    assert receipts[0]['symbols'] == receipts[1]['symbols'] and len(receipts[0]['symbols']) == 64
    assert receipts[0]['source'] == receipts[1]['source']
    matched = []
    for tag in ['c-shared', 'c-static', 'cpp-shared', 'cpp-static']:
        for suffix in ['.stdout', '-low-count.stdout', '-boundary.stdout', '-storage.stdout', '-storage.tsv', '-layout.stdout']:
            name = tag+suffix
            assert (before/name).read_bytes() == (after/name).read_bytes(), name
            matched.append(name)
    for file in before.glob('*-run/*.dat'):
        relative = file.relative_to(before)
        assert file.read_bytes() == (after/relative).read_bytes(), relative
        matched.append(str(relative))
    env = dict(os.environ, LC_ALL='C'); env.pop('TEMPO2', None); env.pop('TEMPO2_CLOCK_PATH', None)
    cc = os.environ.get('CC', 'clang'); darwin = platform.system() == 'Darwin'
    suffix = '.dylib' if darwin else '.so'
    commands = []
    def run(command, label, extra_env=None):
        command = list(map(str, command))
        r = subprocess.run(command, capture_output=True, env=dict(env, **(extra_env or {})))
        commands.append(dict(label=label, command=command, status=r.returncode))
        (out/(label+'.stdout')).write_bytes(r.stdout); (out/(label+'.stderr')).write_bytes(r.stderr)
        assert r.returncode == 0, (label, r.stderr)
        return r
    libraries = {}
    allocations = {}
    observer = out/'libao_counter.dylib'
    if darwin:
        run([cc, '-std=c11', '-O2', '-DAO_COUNTER_LIBRARY', '-dynamiclib', HERE/'allocations.c', '-o', observer], 'counter-build')
    for engine, prefix in [('before', a.before.resolve()), ('after', a.after.resolve()), ('native', before/'native')]:
        libdir = prefix if engine == 'native' else prefix/'lib'
        libname = 'native' if engine == 'native' else 'tempo2pred_rust'
        library = libdir/('lib'+libname+suffix); libraries[engine] = library
        link = ['-L'+str(libdir), '-l'+libname, '-Wl,-rpath,'+str(libdir), '-lm']
        if not darwin: link += ['-ldl']
        flags = [cc, '-std=c11', '-D_POSIX_C_SOURCE=200809L', '-O2', '-I'+str(a.after.resolve()/'include')]
        run([*flags, HERE/'outputs.c', *link, '-o', out/(engine+'-outputs')], engine+'-outputs-build')
        result = run([out/(engine+'-outputs'), a.predictor.resolve()], engine+'-outputs')
        loaded = Path(result.stderr.decode().split('LOADED ', 1)[1].splitlines()[0]).resolve()
        assert loaded == library.resolve(), loaded
        run([*flags, ROOT/'validation/parallel_execution/evaluation.c', *link, '-o', out/(engine+'-evaluation')], engine+'-evaluation-build')
        run(['otool', '-L', library] if darwin else ['ldd', library], engine+'-dependencies')
        if engine == 'native': continue
        # Linux wraps the actual installed archive; Darwin interposes shared
        # external calls. This observer is excluded from all wall/CPU claims.
        if darwin:
            flags += ['-DAO_COUNTER_CLIENT']
            link += [observer]
        else:
            install = json.loads((prefix.parent/(prefix.name+'-build')/'install.json').read_text())
            link = [prefix/'lib/libtempo2pred_rust.a', *install['native_static_libs'],
                    '-Wl,--wrap=malloc,--wrap=calloc,--wrap=realloc,--wrap=free']
        run([*flags, HERE/'allocations.c', *link, '-o', out/(engine+'-allocations')], engine+'-allocations-build')
        for count in [1, 1001]:
            result = run([out/(engine+'-allocations'), a.predictor.resolve(), count], f'{engine}-allocations-{count}',
                         {'DYLD_INSERT_LIBRARIES': str(observer)} if darwin else {})
            fields = dict((key, int(value)) for key, value in re.findall(r'(\w+)=(\d+)', result.stderr.decode()))
            assert fields['malloc'] > 0, 'allocator observer did not intercept calls'
            allocations[f'{engine}-{count}'] = fields
    original = (out/'before-outputs.stdout').read_bytes()
    changed = (out/'after-outputs.stdout').read_bytes()
    assert original == changed, 'complete scientific/state transcript changed'
    rows = {tuple(line.split()[:2]): line.split()[2] for line in original.decode().splitlines()
            if line.startswith(('PHASE ', 'FREQUENCY '))}
    for key in ['PHASE', 'FREQUENCY']:
        assert rows[key, '64'] != rows[key, '1'], 'direct mutation was hidden'
        assert rows[key, '128'] == rows[key, '2'], 'coefficient restoration was hidden'
    native = (out/'native-outputs.stdout').read_bytes()
    # Supplemental diagnostic, not a replacement/relaxation of the unchanged
    # qualifier's native gates. Report every inherited difference if present.
    native_lines, original_lines = native.splitlines(), original.splitlines()
    assert len(native_lines) == len(original_lines)
    differences = [{'line': i+1, 'native': n.decode(), 'rust': r.decode()}
                   for i, (n, r) in enumerate(zip(native_lines, original_lines)) if n != r]
    finite_passes, inherited = [], []
    for difference in differences:
        n, r = difference['native'].split(), difference['rust'].split()
        # These two finite differences are already below even AF's absolute
        # phase floor; no scale-dependent allowance or new tolerance is needed.
        if (n[0] in ['PHASE', 'DERIVATIVE'] and n[:2] == r[:2] and n[3:] == r[3:]
                and abs(native_number(n[2])-native_number(r[2])) <= phase_bound(0)):
            finite_passes.append(difference['line'])
        else:
            inherited.append(difference)
    receipt = dict(exact_before_after=True, transcript_lines=len(original_lines),
        native_differences=differences, exact_legacy_files=matched, allocations=allocations,
        native_finite_AF_floor_passes=finite_passes, inherited_native_failures=inherited,
        commands=commands, libraries={str(path): sha(path) for path in libraries.values()},
        qualification_receipts={str(path/'receipt.json'): sha(path/'receipt.json') for path in [before, after]},
        inputs={str(path): sha(path) for path in [a.predictor.resolve(), HERE/'outputs.c', HERE/'allocations.c', Path(__file__), ROOT/'validation/parallel_execution/evaluation.c']},
        files={path.name: sha(path) for path in out.iterdir() if path.is_file()})
    (out/'receipt.json').write_text(json.dumps(receipt, indent=2)+'\n')
    print(f'{len(original_lines)} exact before/after lines; {len(differences)} supplemental native differences')
    print(json.dumps(allocations, indent=2))
