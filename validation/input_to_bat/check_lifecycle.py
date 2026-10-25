#!/usr/bin/env python3
"""Public library histories and CLI failure-output checks, all owned staging."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--release', action='store_true')
    args = parser.parse_args()
    profile = 'release' if args.release else 'debug'
    work = HERE / 'build' / ('lifecycle-' + profile)
    work.mkdir(parents=True, exist_ok=True)
    runtime = ROOT / 'validation/reference/build/source/T2runtime'
    env = {**os.environ, 'CARGO_HOME': str(ROOT / 'rust/.cargo-home'),
           'GMP_MPFR_SYS_CACHE': str(ROOT / 'rust/target/gmp-cache')}
    cargo = ['cargo', '--manifest-path', str(ROOT / 'rust/Cargo.toml'), '--locked', '--offline',
             *(['--release'] if args.release else [])]
    command = [cargo[0], 'run', *cargo[1:], '--example', 'check_input_to_bat_lifecycle', '--', str(runtime), str(work)]
    subprocess.run(command, env=env, check=True)
    subprocess.run([cargo[0], 'build', *cargo[1:], '--bin', 'tempo2-rust'], env=env, check=True)
    executable = ROOT / 'rust/target' / profile / 'tempo2-rust'
    tim = HERE / 'fixtures/ordinary.tim'
    outcomes = []
    for name in ['missing-ephemeris', 'corrupt-ephemeris', 'calceph']:
        command = [str(executable), 'bats', '--par', str(work / (name + '.par')), '--tim', str(tim), '--runtime', str(runtime)]
        result = subprocess.run(command, env=env, capture_output=True, timeout=60)
        (work / (name + '.stdout')).write_bytes(result.stdout)
        (work / (name + '.stderr')).write_bytes(result.stderr)
        assert result.returncode != 0 and result.stdout == b'' and result.stderr, name
        outcomes.append({'name': name, 'returncode': result.returncode, 'stdout_bytes': len(result.stdout),
                         'stderr_sha256': hashlib.sha256(result.stderr).hexdigest()})
    (work / 'cli-failures.json').write_text(json.dumps({'profile': profile,
        'executable_sha256': hashlib.sha256(executable.read_bytes()).hexdigest(), 'cases': outcomes}, indent=2) + '\n')
    print('PASS: library histories and three CLI failures without partial success output')


if __name__ == '__main__':
    main()
