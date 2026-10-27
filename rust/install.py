#!/usr/bin/env python3
"""Private-prefix core CLI, Rust source, predictor ABI and client install (0084/0086).

No runtime tables or canonical TEMPO2 product are installed. This is local build
packaging, not scientific code. Receipts/build products belong under build/.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess

p = argparse.ArgumentParser(description=__doc__)
p.add_argument('--prefix', type=Path, required=True)
p.add_argument('--profile', choices=['debug', 'release'], default='release')
p.add_argument('--features', default='')
a = p.parse_args()
root = Path(__file__).resolve().parent
prefix = a.prefix.resolve()
if prefix.exists():
    raise SystemExit('Use a fresh private prefix; existing products are not overwritten')
work = prefix.parent / (prefix.name + '-build')
work.mkdir(parents=True, exist_ok=True)
commands = []
def run(args, **kw):
    commands.append(list(map(str, args)))
    subprocess.run(args, check=True, **kw)

command = ['cargo', 'rustc', '--offline', '--locked', '--manifest-path', str(root/'Cargo.toml'),
           '--lib', '--target-dir', str(work/'target')]
if a.profile == 'release': command += ['--release']
if a.features: command += ['--features', a.features]
command += ['--', '--print', 'native-static-libs']
with (work/'build.log').open('w') as log: run(command, stdout=log, stderr=subprocess.STDOUT)
for path in ['lib', 'include', 'bin', 'share/tempo2-rust']: (prefix/path).mkdir(parents=True)
archive = work/'target'/a.profile/'libtempo2_rust.a'
shutil.copy2(archive, prefix/'lib/libtempo2pred_rust.a')
cc = os.environ.get('CC', 'clang')
# rustc's static native requirements are recorded rather than guessed silently.
log = (work/'build.log').read_text()
requirements = [line.split('native-static-libs: ',1)[1] for line in log.splitlines() if 'native-static-libs: ' in line]
if not requirements: raise SystemExit('rustc did not report native static libraries')
import shlex
libs = shlex.split(requirements[-1])
# rustc reports native library names, but not their custom search directory.
# Preserve the explicitly selected static CALCEPH provider for the final link.
if 'calceph' in a.features.split(','):
    libs.insert(0, '-L'+str(Path(os.environ['CALCEPH_DIR']).resolve()/'lib'))
if platform.system() == 'Darwin':
    shared = prefix/'lib/libtempo2pred_rust.dylib'
    flags = ['-dynamiclib', '-Wl,-u,_T2Predictor_Init', '-Wl,-install_name,@rpath/libtempo2pred_rust.dylib']
else:
    shared = prefix/'lib/libtempo2pred_rust.so'
    flags = ['-shared', '-Wl,-u,T2Predictor_Init', '-Wl,-soname,libtempo2pred_rust.so']
run([cc, *flags, str(prefix/'lib/libtempo2pred_rust.a'), *libs, '-o', str(shared)])
for header in (root/'include').glob('*.h'): shutil.copy2(header, prefix/'include'/header.name)
run([cc, '-std=c11', '-Wall', '-Wextra', '-Werror', '-I'+str(prefix/'include'), str(root/'native/get_period.c'),
     '-L'+str(prefix/'lib'), '-ltempo2pred_rust', '-Wl,-rpath,'+str(prefix/'lib'), '-o', str(prefix/'bin/getPeriod-rust')])
command = ['cargo', 'build', '--offline', '--locked', '--manifest-path', str(root/'Cargo.toml'),
           '--bin', 'tempo2-rust', '--target-dir', str(work/'target')]
if a.profile == 'release': command += ['--release']
if a.features: command += ['--features', a.features]
with (work/'core-build.log').open('w') as log: run(command, stdout=log, stderr=subprocess.STDOUT)
shutil.copy2(work/'target'/a.profile/'tempo2-rust', prefix/'bin/tempo2-rust')
# Preserve native sources, build scripts, lockfile and attribution. Exclude build
# caches/evidence; external Cargo builds consume this self-contained source tree.
installed = prefix/'share/tempo2-rust/rust'
installed.mkdir()
for name in ['src','native','include','data']:
    shutil.copytree(root/name, installed/name)
for name in ['Cargo.toml','Cargo.lock','build.rs','README.md','install.py','LICENSE','MT19937_LICENSE.txt','jpl_source_manifest.json']:
    shutil.copy2(root/name, installed/name)
for path in root.glob('*.md'): shutil.copy2(path, installed/path.name)
receipt = {'profile': a.profile, 'features': a.features, 'platform': platform.platform(),
           'commands': commands, 'native_static_libs': libs, 'prefix': str(prefix),
           'files': {str(p.relative_to(prefix)): hashlib.sha256(p.read_bytes()).hexdigest()
                     for p in sorted(prefix.rglob('*')) if p.is_file()}}
(work/'install.json').write_text(json.dumps(receipt, indent=2)+'\n')
print(prefix)
