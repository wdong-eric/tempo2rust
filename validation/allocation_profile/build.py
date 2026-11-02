#!/usr/bin/env python3
"""Freeze a revision, build offline, and bind separate AN observers to that build."""
import argparse
import io
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import sys
import tarfile

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / 'validation/release'))
from common import sha


def cargo_environment(source):
    """Keep each frozen build isolated while reusing the caller's prepared cache."""
    # CARGO_HOME remains supplied by the caller, or unset for Cargo's own default.
    return dict(os.environ,
        GMP_MPFR_SYS_CACHE=os.environ.get('GMP_MPFR_SYS_CACHE', str(HERE / 'build/gmp-cache')),
        CARGO_TARGET_DIR=str(source / 'rust/target'), CARGO_BUILD_JOBS='4')


def command(out, name, args, env=None):
    args = list(map(str, args))
    result = subprocess.run(args, cwd=ROOT, env=env, capture_output=True)
    (out / (name + '.stdout')).write_bytes(result.stdout)
    (out / (name + '.stderr')).write_bytes(result.stderr)
    record = {'argv': args, 'exit': result.returncode}
    if env is not None:
        record['cargo_environment'] = {k: env[k] for k in
            ['CARGO_HOME', 'CARGO_TARGET_DIR', 'GMP_MPFR_SYS_CACHE', 'CARGO_BUILD_JOBS'] if k in env}
    (out / (name + '.json')).write_text(json.dumps(record, indent=2) + '\n')
    assert result.returncode == 0, (name, result.stderr.decode()[-3000:])
    return record


def bind(out, a2):
    """Bind only already-frozen inputs; also used for the initial manual build."""
    frozen = json.loads((out / 'freeze.json').read_text())
    source = out / 'source'
    assert all(sha(source / p) == h for p, h in frozen['sources'].items())
    target = source / 'rust/target/release'
    bins = out / 'bin'
    bins.mkdir()
    sources = {str(p.relative_to(ROOT)): sha(p) for p in HERE.glob('*') if p.is_file()}
    cc = os.environ.get('CC', 'clang')
    cxx = os.environ.get('NATIVE_CXX', 'g++-16' if platform.system() == 'Darwin' else 'g++')
    commands = [command(out, 'observer-c', [cc, '-O2', '-std=c11', '-Wall', '-Wextra', '-Werror',
                            '-c', HERE / 'observe.c', '-o', out / 'observe.o'])]
    common = ['rustc', '--edition=2024', '-C', 'opt-level=3', '-L', 'dependency=' + str(target / 'deps'),
              '--extern', 'tempo2_rust=' + str(target / 'libtempo2_rust.rlib')]
    for name, extra in [('observe', []), ('failure', ['--cfg', 'fail_allocator'])]:
        commands.append(command(out, name + '-build', [*common, *extra, HERE / 'observe.rs',
                          '-C', 'link-arg=' + str(out / 'observe.o'), '-o', bins / name]))
    wrapper = out / 'ak-cli.rs'
    wrapper.write_bytes(subprocess.check_output(['git', 'show', frozen['revision'] + ':validation/parallel_execution/cli.rs'], cwd=ROOT))
    commands.append(command(out, 'ak-cli-build', [*common, '--crate-name', 'an_ak_cli', wrapper, '-o', bins / 'cli']))
    shutil.copy2(target / 'tempo2-rust', bins / 'tempo2-rust')
    commands.append(command(out, 'native-build', [cxx, '-O2', '-std=c++11', '-I' + str(a2 / 'source'),
                            HERE / 'observe_native.cc', out / 'observe.o', a2 / 'libtempo2-reference.a',
                            '-lquadmath', '-lgfortran', '-o', bins / 'observe-native']))
    for name, argv in [('rustc-version', ['rustc', '-Vv']), ('cargo-version', ['cargo', '-V']),
                       ('cc-version', [cc, '--version']), ('cxx-version', [cxx, '--version']),
                       ('linker-version', ['ld', '-v'] if platform.system() == 'Darwin' else ['ld', '--version'])]:
        commands.append(command(out, name, argv))
    for p in bins.iterdir():
        command(out, p.name + '-libraries', ['otool', '-L', p] if platform.system() == 'Darwin' else ['ldd', p])
        p.chmod(0o555)
    for p in frozen['sources']:
        (source / p).chmod(0o444)
    receipt = {'revision': frozen['revision'], 'freeze_sha256': sha(out / 'freeze.json'),
               'commands': commands, 'observer_sources': sources, 'ak_wrapper_sha256': sha(wrapper),
               'binaries': {p.name: sha(p) for p in bins.iterdir()}, 'host': platform.platform(),
               'machine': platform.machine(), 'page_size': os.sysconf('SC_PAGE_SIZE'),
               'features': [], 'profile': 'release', 'workers': 1,
               'allocator': 'Rust default System / platform libc; no replacement in timing or residency binaries',
               'a2': {str(a2 / p): sha(a2 / p) for p in ['receipt.json', 'libtempo2-reference.a', 'source/TKmatrix.C', 'source/initialise.C', 'source/tempo2.h']}}
    (out / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
    return receipt


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--revision', required=True)
    p.add_argument('--out', type=Path, required=True)
    p.add_argument('--a2', type=Path, required=True)
    a = p.parse_args()
    out = a.out.resolve(); out.mkdir(parents=True, exist_ok=False)
    revision = subprocess.check_output(['git', 'rev-parse', a.revision], cwd=ROOT, text=True).strip()
    raw = subprocess.check_output(['git', 'archive', revision, 'rust'], cwd=ROOT)
    source = out / 'source'; source.mkdir()
    # Trusted local Git archive, not an externally supplied tar file.
    tarfile.open(fileobj=io.BytesIO(raw)).extractall(source)
    import hashlib
    (out / 'freeze.json').write_text(json.dumps({'revision': revision,
        'export_sha256': hashlib.sha256(raw).hexdigest(),
        'sources': {str(f.relative_to(source)): sha(f) for f in source.rglob('*') if f.is_file()}}, indent=2) + '\n')
    env = cargo_environment(source)
    command(out, 'cargo-build', ['cargo', 'build', '--offline', '--locked', '--release',
                               '--manifest-path', source / 'rust/Cargo.toml'], env)
    bind(out, a.a2.resolve())
