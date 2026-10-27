"""Replay AJ's supplied consumers against central's fresh committed install."""
from pathlib import Path
import hashlib
import json
import os
import shutil
import subprocess
import time

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT = Path(Path('/tmp/tempo2-aj-committed-review-current').read_text().strip())
WS = OUT / 'workspace'
PREFIX = OUT / 'prefix-release'

def sha(p):
    h = hashlib.sha256()
    with Path(p).open('rb') as f:
        for block in iter(lambda: f.read(1048576), b''):
            h.update(block)
    return h.hexdigest()

source = ROOT / 'validation/library_interfaces'
dest = WS / 'validation/library_interfaces'
dest.mkdir()
observers = {}
for p in sorted(source.iterdir()):
    if p.is_file() and p.suffix in {'.py', '.c', '.rs', '.md'}:
        digest = sha(p)
        shutil.copy2(p, dest / p.name)
        assert sha(p) == sha(dest/p.name) == digest, p
        observers[str(p.relative_to(ROOT))] = digest
# These read-only dependencies are used by the supplied qualification recipes.
(WS / 'tempo2').symlink_to(ROOT / 'tempo2', target_is_directory=True)
reference = WS / 'validation/reference/build'
reference.parent.mkdir(parents=True, exist_ok=True)
reference.symlink_to(ROOT / 'validation/reference/build', target_is_directory=True)
env = dict(os.environ, PATH='/Users/Eric.Dong/.cargo/bin:/opt/homebrew/bin:' + os.environ.get('PATH', ''),
           CARGO_HOME=str(ROOT/'rust/.cargo-home'), CARGO_NET_OFFLINE='true', CARGO_BUILD_JOBS='2',
           FC='/opt/homebrew/bin/gfortran-16', CC='/usr/bin/clang', PYTHONDONTWRITEBYTECODE='1')
for key in ['CARGO_TARGET_DIR', 'CALCEPH_DIR', 'TEMPO2_ACCELERATE', 'TEMPO2', 'TEMPO2_CLOCK_PATH', 'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS']:
    env.pop(key, None)
records = []
identity = dict(production_revision=json.loads((OUT/'identity.json').read_text())['revision'],
                observer_sources=observers, script_sha256=sha(__file__),
                install_completion_sha256=sha(OUT/'install-completion.json'))
(OUT/'replay-identity.json').write_text(json.dumps(identity, indent=2)+'\n')

def run(label, command):
    command = list(map(str, command))
    print('Running ' + label, flush=True)
    log = OUT / (label+'.log')
    start = time.monotonic()
    with log.open('wb') as f:
        r = subprocess.run(command, cwd=WS, env=env, stdout=f, stderr=subprocess.STDOUT, timeout=1200)
    records.append(dict(label=label, command=command, exit=r.returncode,
                        seconds=time.monotonic()-start, log_sha256=sha(log)))
    (OUT/'replay-commands.json').write_text(json.dumps(records, indent=2)+'\n')
    assert r.returncode == 0, records[-1]
    print('Completed ' + label, flush=True)

run('external-c-cpp', ['python3','-B',dest/'qualify.py','--prefix',PREFIX,'--out',OUT/'c-qualification'])
run('external-rust', ['python3','-B',dest/'qualify_rust.py','--prefix',PREFIX,'--out',OUT/'rust-qualification','--release'])
for path,digest in observers.items():
    assert sha(WS/path) == digest, path
for path,digest in json.loads((OUT/'install-completion.json').read_text())['installed_files'].items():
    assert sha(PREFIX/path) == digest, path
c = json.loads((OUT/'c-qualification/receipt.json').read_text())
rust = json.loads((OUT/'rust-qualification/receipt.json').read_text())
completion = dict(identity_sha256=sha(OUT/'replay-identity.json'),commands=records,
                  c_receipt_sha256=sha(OUT/'c-qualification/receipt.json'),
                  rust_receipt_sha256=sha(OUT/'rust-qualification/receipt.json'),
                  c_profiles=c['profiles'],rejected_controls=c['rejected_controls'],
                  rust_client_invocations=len(rust['commands'])-2,
                  scope='Fresh committed default-release install and separately compiled supplied external C11/C++ static/shared and Cargo consumers. No Linux, concurrency, performance or inherited numerical-gate acceptance.')
(OUT/'replay-completion.json').write_text(json.dumps(completion,indent=2)+'\n')
print(json.dumps({k:completion[k] for k in ['c_profiles','rust_client_invocations','scope']},indent=2),flush=True)
