#!/usr/bin/env python3
"""Compile the direct evaluator against one frozen installed source tree."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess

HERE = Path(__file__).resolve().parent

if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['prefix', 'out', 'cargo-home']:
        p.add_argument('--'+name, type=Path, required=True)
    a = p.parse_args()
    out = a.out.resolve(); (out/'src').mkdir(parents=True, exist_ok=False)
    installed = a.prefix.resolve()/'share/tempo2-rust/rust'
    shutil.copy2(HERE/'evaluator.rs', out/'src/main.rs')
    (out/'src/bin').mkdir()
    shutil.copy2(HERE/'evaluator_edges.rs', out/'src/bin/edges.rs')
    (out/'Cargo.toml').write_text('[package]\nname="ao-evaluator"\nversion="0.1.0"\nedition="2024"\n'
        '[dependencies]\n'+f'tempo2-rust={{path={json.dumps(str(installed))}}}\n'
        'rug={version="=1.30.0", default-features=false, features=["float", "integer", "std"]}\n')
    env = dict(os.environ, CARGO_HOME=str(a.cargo_home.resolve()))
    env.pop('CARGO_TARGET_DIR', None)
    records = []
    def run(command, label):
        command = list(map(str, command)); r = subprocess.run(command, capture_output=True, env=env)
        (out/(label+'.stdout')).write_bytes(r.stdout); (out/(label+'.stderr')).write_bytes(r.stderr)
        records.append(dict(command=command, status=r.returncode)); assert r.returncode == 0, r.stderr
    cargo = ['cargo', 'build', '--offline', '--release', '--manifest-path', out/'Cargo.toml', '--target-dir', out/'target']
    run(cargo, 'build')
    shutil.copy2(out/'target/release/ao-evaluator', out/'plain')
    shutil.copy2(out/'target/release/edges', out/'edges')
    cc = os.environ.get('CC', 'clang')
    if platform.system() == 'Darwin':
        run([cc, '-std=c11', '-O2', '-dynamiclib', HERE/'evaluator_allocations.c', '-o', out/'counter.dylib'], 'counter-build')
    else:
        run([cc, '-std=c11', '-O2', '-c', HERE/'evaluator_allocations.c', '-o', out/'counter.o'], 'counter-build')
        run(['cargo', 'rustc', '--offline', '--release', '--manifest-path', out/'Cargo.toml', '--target-dir', out/'target',
             '--bin', 'ao-evaluator', '--', '-C', 'link-arg='+str(out/'counter.o'),
             '-C', 'link-arg=-Wl,--wrap=malloc,--wrap=calloc,--wrap=realloc,--wrap=free'], 'counter-link')
        shutil.copy2(out/'target/release/ao-evaluator', out/'counted')
    sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
    (out/'receipt.json').write_text(json.dumps(dict(commands=records,
        installed=str(installed), cargo_home=env['CARGO_HOME'],
        inputs={str(path):sha(path) for path in [HERE/'evaluator.rs', HERE/'evaluator_edges.rs', HERE/'evaluator_allocations.c', HERE/'allocations.c', Path(__file__)]},
        files={str(path.relative_to(out)):sha(path) for path in out.rglob('*') if path.is_file() and 'target' not in path.relative_to(out).parts}), indent=2)+'\n')
