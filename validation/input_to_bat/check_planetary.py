#!/usr/bin/env python3
"""Replay every Z driver history against its accepted caller/exit projection.

The separate check_random.py retains all 15 complete random private histories.
Read accepted files/resources only; never run or edit the Z recording machinery.
"""
import argparse
import gzip
import hashlib
import json
import os
from pathlib import Path
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
Z = ROOT / 'validation/planetary_ephemeris'
BUILD = HERE / 'build/planetary'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--release', action='store_true')
    parser.add_argument('--case', action='append')
    args = parser.parse_args()
    BUILD.mkdir(parents=True, exist_ok=True)
    reference = Z / 'outputs/reference.json.gz'
    observed = Z / 'outputs/observed.json.gz'
    ordinary = json.loads(gzip.decompress(reference.read_bytes()))
    observer = json.loads(gzip.decompress(observed.read_bytes()))
    resources = json.loads((Z / 'build/resources.json').read_text())
    for record in resources.values():
        if 'path' in record:
            assert sha(Path(record['path'])) == record['sha256']
    selected = []
    for native, case in zip(ordinary, observer, strict=True):
        assert native['id'] == case['id']
        if not any(a[0] == 'driver' for a in case['actions']):
            continue
        if args.case and case['id'] not in args.case:
            continue
        assert native['actions'] == case['actions']
        assert native['stdout'] == case['stdout']
        assert native['stderr'] == case['stderr']
        for old, new in zip(native['events'], case['events']):
            for key in ('event', 'phase', 'operation', 'result', 'seeds', 'globals', 'caller'):
                assert old[key] == new[key], (case['id'], key)
        # Calls/private records are independent Z observer evidence; the Rust
        # driver contract here checks the entire public caller, not call mocks.
        for event in case['events']:
            for key in ('private', 'calls', 'open_handles'):
                event.pop(key)
        selected.append(case)
    cases = BUILD / 'cases.json'
    cases.write_text(json.dumps(selected) + '\n')
    env = {**os.environ, 'CARGO_HOME': str(ROOT / 'rust/.cargo-home'),
           'GMP_MPFR_SYS_CACHE': str(ROOT / 'rust/target/gmp-cache'),
           'AC_PLANETARY_CASES': str(cases)}
    command = ['cargo', 'test', '--manifest-path', str(ROOT / 'rust/Cargo.toml'),
               '--locked', '--offline', *(['--release'] if args.release else []),
               '--lib', 'read_ephemeris::corpus_tests::z_driver_histories', '--',
               '--ignored', '--exact', '--nocapture', '--test-threads=1']
    run = subprocess.run(command, env=env)
    profile = 'release' if args.release else 'debug'
    (BUILD / f'receipt-{profile}.json').write_text(json.dumps({
        'reference_sha256': sha(reference), 'observer_sha256': sha(observed),
        'resource_manifest_sha256': sha(Z / 'build/resources.json'),
        'cases_sha256': sha(cases), 'cases': len(selected), 'profile': profile,
        'command': command, 'returncode': run.returncode,
    }, indent=2) + '\n')
    raise SystemExit(run.returncode)


if __name__ == '__main__':
    main()
