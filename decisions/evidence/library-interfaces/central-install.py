"""Build AJ's committed default release into a fresh private prefix."""
from pathlib import Path
import hashlib
import json
import os
import subprocess
import tempfile
import time

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
REV = subprocess.check_output(['git', 'rev-parse', '3e41497'], cwd=ROOT, text=True).strip()
OUT = Path(tempfile.mkdtemp(prefix='tempo2-aj-committed-review-'))
Path('/tmp/tempo2-aj-committed-review-current').write_text(str(OUT) + '\n')
WS = OUT / 'workspace'
WS.mkdir()

def sha(p):
    h = hashlib.sha256()
    with Path(p).open('rb') as f:
        for block in iter(lambda: f.read(1048576), b''):
            h.update(block)
    return h.hexdigest()

archive = OUT / 'source.tar'
with archive.open('wb') as f:
    subprocess.run(['git', 'archive', REV], cwd=ROOT, stdout=f, check=True)
subprocess.run(['/usr/bin/tar', '-xf', archive, '-C', WS], check=True)
files = {}
for line in subprocess.check_output(['git', 'ls-tree', '-r', REV, 'rust'], cwd=ROOT, text=True).splitlines():
    meta, name = line.split('\t', 1)
    mode, kind, oid = meta.split()
    if kind != 'blob':
        continue
    data = (WS / name).read_bytes()
    assert hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest() == oid, name
    files[name] = sha(WS / name)

env = dict(os.environ, PATH='/Users/Eric.Dong/.cargo/bin:/opt/homebrew/bin:' + os.environ.get('PATH', ''),
           CARGO_HOME=str(ROOT / 'rust/.cargo-home'), CARGO_NET_OFFLINE='true', CARGO_BUILD_JOBS='2',
           FC='/opt/homebrew/bin/gfortran-16', CC='/usr/bin/clang', PYTHONDONTWRITEBYTECODE='1')
for key in ['CARGO_TARGET_DIR', 'CALCEPH_DIR', 'TEMPO2_ACCELERATE', 'TEMPO2', 'TEMPO2_CLOCK_PATH', 'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS']:
    env.pop(key, None)
prefix = OUT / 'prefix-release'
cmd = ['python3', '-B', str(WS / 'rust/install.py'), '--prefix', str(prefix), '--profile', 'release']
identity = dict(revision=REV, source_archive_sha256=sha(archive), production_files=files,
                script_sha256=sha(__file__), command=cmd,
                environment={k: env[k] for k in ['PATH', 'CARGO_HOME', 'CARGO_BUILD_JOBS', 'FC', 'CC']})
(OUT / 'identity.json').write_text(json.dumps(identity, indent=2) + '\n')
print('Building committed AJ default release in ' + str(OUT), flush=True)
start = time.monotonic()
log = OUT / 'install.log'
with log.open('wb') as f:
    result = subprocess.run(cmd, cwd=WS, env=env, stdout=f, stderr=subprocess.STDOUT, timeout=1200)
completion = dict(revision=REV, command=cmd, exit=result.returncode, seconds=time.monotonic()-start,
                  log_sha256=sha(log), identity_sha256=sha(OUT/'identity.json'))
(OUT / 'install-completion.json').write_text(json.dumps(completion, indent=2) + '\n')
assert result.returncode == 0, completion
for name, digest in files.items():
    assert sha(WS / name) == digest, name
installed = prefix / 'share/tempo2-rust/rust'
for name in ['LICENSE', 'MT19937_LICENSE.txt', 'Cargo.toml', 'Cargo.lock', 'build.rs', 'install.py', 'jpl_source_manifest.json']:
    assert (installed/name).read_bytes() == (WS/'rust'/name).read_bytes(), name
completion['installed_files'] = {str(p.relative_to(prefix)):sha(p) for p in sorted(prefix.rglob('*')) if p.is_file()}
(OUT / 'install-completion.json').write_text(json.dumps(completion, indent=2) + '\n')
print(json.dumps({k:completion[k] for k in ['revision', 'exit', 'seconds']}) + '\nPrivate prefix: ' + str(prefix), flush=True)
