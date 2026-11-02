#!/usr/bin/env python3
"""Freeze a committed predictor build; use the caller's prepared Cargo cache."""
import argparse
import hashlib
import io
import json
import os
from pathlib import Path
import platform
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[2]

if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--revision', required=True)
    p.add_argument('--out', type=Path, required=True)
    p.add_argument('--cargo-home', type=Path, required=True)
    a = p.parse_args()
    out = a.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    source = out / 'source'
    source.mkdir()
    commit = subprocess.check_output(['git', '-C', ROOT, 'rev-parse', a.revision], text=True).strip()
    archive = subprocess.check_output(['git', '-C', ROOT, 'archive', commit, 'rust'])
    with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
        tar.extractall(source)  # Trusted local Git archive of the selected rust/ tree.
    # install.py explicitly selects prefix-build/target. Before and after use
    # distinct out directories; no caller project target is reused or overwritten.
    env = dict(os.environ, CARGO_HOME=str(a.cargo_home.resolve()))
    env.pop('CARGO_TARGET_DIR', None)
    command = ['python3', str(source/'rust/install.py'), '--prefix', str(out/'prefix'), '--profile', 'release']
    with (out/'install.log').open('w') as log:
        subprocess.run(command, check=True, env=env, stdout=log, stderr=subprocess.STDOUT)
    sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
    receipt = {
        'commit': commit, 'platform': platform.platform(), 'command': command,
        'cargo_home': env['CARGO_HOME'], 'target': str(out/'prefix-build/target'),
        'tools': {tool: subprocess.check_output([tool, '--version'], text=True)
                  for tool in ['rustc', 'cargo', env.get('CC', 'clang'), env.get('FC', 'gfortran-16')]},
        'source': {str(path.relative_to(source)): sha(path) for path in sorted(source.rglob('*')) if path.is_file()},
        'install_receipt_sha256': sha(out/'prefix-build/install.json'),
        'builder_sha256': sha(Path(__file__)),
    }
    (out/'receipt.json').write_text(json.dumps(receipt, indent=2)+'\n')
    print(out/'prefix')
