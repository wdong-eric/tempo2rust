#!/usr/bin/env python3
"""Qualify call-local C coefficients against an immutable owned baseline."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess

HERE = Path(__file__).resolve().parent

if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['before', 'after', 'out']:
        p.add_argument('--'+name, type=Path, required=True)
    a = p.parse_args()
    out = a.out.resolve(); out.mkdir(parents=True, exist_ok=False)
    darwin = platform.system() == 'Darwin'
    env = dict(os.environ, LC_ALL='C', UBSAN_OPTIONS='halt_on_error=1:print_stacktrace=1')
    for name in ['DYLD_INSERT_LIBRARIES', 'LD_PRELOAD']:
        env.pop(name, None)
    records, inputs, outputs = [], {}, {}
    sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
    def run(label, command):
        command = list(map(str, command))
        r = subprocess.run(command, capture_output=True, env=env)
        (out/(label+'.stdout')).write_bytes(r.stdout); (out/(label+'.stderr')).write_bytes(r.stderr)
        records.append(dict(label=label, command=command, status=r.returncode))
        assert r.returncode == 0, (label, r.stderr)
        return r
    errors = ['invalid native polynomial dimensions']*3 + ['null native scalar backing',
        'invalid native polynomial dimensions', 'null native scalar backing',
        'null native scalar backing', 'null native scalar backing', 'null native output backing',
        'null native scalar backing', 'invalid native polynomial dimensions',
        'null native scalar backing', 'null native output backing']
    expected_errors = ''.join('tempo2pred-rust: '+error+'\n' for error in errors).encode()
    source = HERE/'coefficient_view.c'
    for side, prefix in [('before', a.before.resolve()), ('after', a.after.resolve())]:
        install = prefix.parent/(prefix.name+'-build')/'install.json'
        libs = json.loads(install.read_text())['native_static_libs']
        archive = prefix/'lib/libtempo2pred_rust.a'
        shared = prefix/('lib/libtempo2pred_rust'+('.dylib' if darwin else '.so'))
        shim = prefix/'share/tempo2-rust/rust/native/predictor.c'
        for path in [install, archive, shared, shim, prefix/'include/tempo2pred.h', prefix/'include/tempo2pred_int.h']:
            inputs[str(path)] = sha(path)
        for language, compiler, standard in [('c', 'clang', 'c11'), ('cpp', 'clang++', 'c++17')]:
            for linkage in ['shared', 'static']:
                label = f'{side}-{language}-{linkage}'
                obj, exe = out/(label+'.o'), out/label
                flags = ['-x', 'c' if language == 'c' else 'c++', '-std='+standard,
                         '-O2', '-I'+str(prefix/'include')]
                run(label+'-compile', [compiler, *flags, '-c', source, '-o', obj])
                link = (['-L'+str(prefix/'lib'), '-ltempo2pred_rust', '-Wl,-rpath,'+str(prefix/'lib')]
                        if linkage == 'shared' else [archive, *libs])
                run(label+'-link', [compiler, obj, *link, '-lm', '-o', exe])
                r = run(label, [exe]); assert r.stderr == expected_errors, label
                outputs[label] = r.stdout
        label = side+'-asan-ubsan'
        flags = ['-std=c11', '-O1', '-g', '-fno-omit-frame-pointer', '-fsanitize=address,undefined',
                 '-fno-builtin-expl', '-fno-builtin-logl', '-I'+str(prefix/'include')]
        obj, exe = out/(label+'-shim.o'), out/label
        run(label+'-shim', ['clang', *flags, '-c', shim, '-o', obj])
        run(label+'-link', ['clang', *flags, source, obj, archive, *libs, '-lm', '-o', exe])
        r = run(label, [exe]); assert r.stderr == expected_errors, label
        outputs[label] = r.stdout
    reference = outputs['before-c-shared']
    assert all(value == reference for value in outputs.values()), 'coefficient/state transcript changed'
    for path in [source, Path(__file__)]: inputs[str(path.resolve())] = sha(path)
    receipt = dict(exact_profiles=list(outputs), transcript_lines=len(reference.splitlines()),
        diagnostics=errors, records=records, inputs=inputs,
        sanitizer_scope='C witness and native ABI shim instrumented; Rust/MPFR/Fortran are not instrumented.',
        files={path.name:sha(path) for path in out.iterdir() if path.is_file()})
    (out/'receipt.json').write_text(json.dumps(receipt, indent=2)+'\n')
    print(f'{len(reference.splitlines())} exact lines across {len(outputs)} profiles; diagnostics and ownership assertions passed')
