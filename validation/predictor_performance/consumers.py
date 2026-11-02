#!/usr/bin/env python3
"""Reuse AJ's two affected, runtime-independent installed Rust consumers."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[2]

if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['before', 'after', 'out', 'cargo-home']:
        p.add_argument('--'+name, type=Path, required=True)
    a = p.parse_args()
    out = a.out.resolve(); out.mkdir(parents=True, exist_ok=False)
    env = dict(os.environ, CARGO_HOME=str(a.cargo_home.resolve()))
    env.pop('CARGO_TARGET_DIR', None)
    records = []
    sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
    for engine, prefix in [('before', a.before.resolve()), ('after', a.after.resolve())]:
        work = out/engine; (work/'src/bin').mkdir(parents=True)
        installed = prefix/'share/tempo2-rust/rust'
        dependencies = (installed/'Cargo.toml').read_text().split('[dependencies]\n')[1].split('[lib]')[0]
        manifest = ('[package]\nname = "ao-predictor-consumers"\nversion = "0.1.0"\nedition = "2024"\n'
                    '[dependencies]\n'+dependencies+f'tempo2-rust = {{path = {json.dumps(str(installed))}}}\n')
        (work/'Cargo.toml').write_text(manifest)
        for name in ['predictor_components', 'predictor_subnormal']:
            shutil.copy2(ROOT/f'rust/examples/{name}.rs', work/f'src/bin/{name}.rs')
        build = ['cargo', 'build', '--offline', '--release', '--manifest-path', str(work/'Cargo.toml'), '--target-dir', str(work/'target')]
        r = subprocess.run(build, capture_output=True, env=env)
        (work/'build.stdout').write_bytes(r.stdout); (work/'build.stderr').write_bytes(r.stderr)
        assert r.returncode == 0, r.stderr
        for name, arguments in [('predictor_components', ['polynomial']), ('predictor_subnormal', [])]:
            exe = work/'target/release'/name
            r = subprocess.run([exe, *arguments], capture_output=True, env=env)
            (work/(name+'.stdout')).write_bytes(r.stdout); (work/(name+'.stderr')).write_bytes(r.stderr)
            assert r.returncode == 0, r.stderr
            if engine == 'after':
                assert r.stdout == (out/'before'/(name+'.stdout')).read_bytes()
                assert r.stderr == (out/'before'/(name+'.stderr')).read_bytes()
            records.append(dict(engine=engine, consumer=name, executable_sha256=sha(exe), stdout_sha256=sha(work/(name+'.stdout'))))
    (out/'receipt.json').write_text(json.dumps(dict(exact=True, records=records,
        inputs={str(ROOT/f'rust/examples/{name}.rs'): sha(ROOT/f'rust/examples/{name}.rs')
                for name in ['predictor_components', 'predictor_subnormal']}), indent=2)+'\n')
    print('Both installed Rust consumers agree exactly before/after')
