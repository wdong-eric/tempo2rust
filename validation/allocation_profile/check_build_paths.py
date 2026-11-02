#!/usr/bin/env python3
"""Bounded real Cargo control for inherited cache/target settings; no TEMPO2 run."""
import argparse
import json
import os
from pathlib import Path
from unittest.mock import patch

from build import cargo_environment, command, sha


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('out', type=Path)
    a = p.parse_args()
    out = a.out.resolve(); out.mkdir(parents=True, exist_ok=False)
    cache = out / 'prepared-cache'
    shared = out / 'inherited-test-target'
    records = []
    with patch.dict(os.environ, {'CARGO_HOME': str(cache), 'CARGO_TARGET_DIR': str(shared)}):
        for name in ['before', 'after']:
            source = out / name / 'source'; crate = source / 'rust'
            (crate / 'src').mkdir(parents=True)
            (crate / 'Cargo.toml').write_text('[package]\nname="an-path-control"\nversion="0.1.0"\nedition="2024"\n')
            (crate / 'Cargo.lock').write_text('version = 4\n\n[[package]]\nname = "an-path-control"\nversion = "0.1.0"\n')
            (crate / 'src/main.rs').write_text('fn main() { println!("' + name + '"); }\n')
            env = cargo_environment(source)
            assert env['CARGO_HOME'] == str(cache)
            assert env['CARGO_TARGET_DIR'] == str(crate / 'target')
            record = command(out, name + '-build', ['cargo', 'build', '--offline', '--locked', '--release',
                             '--manifest-path', crate / 'Cargo.toml'], env)
            binary = crate / 'target/release/an-path-control'
            command(out, name + '-execute', [binary])
            assert (out / (name + '-execute.stdout')).read_text() == name + '\n'
            records.append({'name': name, 'command': record, 'binary': str(binary), 'sha256': sha(binary)})
        assert os.environ['CARGO_TARGET_DIR'] == str(shared)
    assert not shared.exists()
    assert all(sha(Path(r['binary'])) == r['sha256'] for r in records)
    with patch.dict(os.environ, {}, clear=True):
        assert 'CARGO_HOME' not in cargo_environment(out)
    (out / 'receipt.json').write_text(json.dumps({'status': 'pass', 'records': records,
        'meaning': 'Two real offline/locked builds preserve separate binaries despite an inherited shared target; prepared cache honored',
        'sources': {str(p): sha(p) for p in [Path(__file__), Path(__file__).with_name('build.py'),
                                          Path(__file__).with_name('verify.py')]}}, indent=2) + '\n')
