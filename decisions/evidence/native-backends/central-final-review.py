"""Central isolated replay of committed AI code; all output remains temporary."""
from pathlib import Path
import hashlib
import json
import os
import shutil
import subprocess
import sys
import time

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT = Path(Path('/tmp/tempo2-ai-committed-review-current').read_text().strip())
WS = OUT / 'workspace'
REV = 'e96b5898827cfd15f0890431922460860d05d349'
PROVIDER = ROOT / 'validation/native_backends/build/provider'
NATIVE = ROOT / 'validation/native_backends/build/profiles-final-v2/combined-release/native'

def sha(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as f:
        for block in iter(lambda: f.read(1048576), b''):
            h.update(block)
    return h.hexdigest()

# Keep the earlier successful build and source archive as their actual revision.
prior = OUT / 'pre-correction-bin'
prior.mkdir()
shutil.copy2(WS / 'rust/target/release/tempo2-rust', prior / 'tempo2-rust')
assert sha(prior / 'tempo2-rust') == json.loads((OUT / 'build-completion.json').read_text())['binary_sha256']
archive = OUT / 'final-source.tar'
with archive.open('wb') as f:
    subprocess.run(['git', 'archive', REV], cwd=ROOT, stdout=f, check=True)
subprocess.run(['/usr/bin/tar', '-xf', str(archive), '-C', str(WS)], check=True)

files = {}
for line in subprocess.check_output(['git', 'ls-tree', '-r', REV, 'rust', 'validation/native_backends'], cwd=ROOT, text=True).splitlines():
    meta, name = line.split('\t', 1)
    mode, kind, oid = meta.split()
    if kind != 'blob':
        continue
    data = (WS / name).read_bytes()
    assert hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest() == oid, name
    files[name] = hashlib.sha256(data).hexdigest()
before = json.loads((OUT / 'identity.json').read_text())['production_files']
changed = [name for name, digest in before.items() if files[name] != digest]
assert changed == ['rust/src/tkfit.rs'], changed

# The accepted runtime is borrowed for reads. Every new product goes to OUT/WS.
reference = WS / 'validation/reference/build'
reference.parent.mkdir(parents=True, exist_ok=True)
reference.symlink_to(ROOT / 'validation/reference/build', target_is_directory=True)
native_identity = json.loads((NATIVE / 'identity.json').read_text())
for name, digest in native_identity['executables'].items():
    assert sha(NATIVE / name) == digest, name
for name, digest in native_identity['objects'].items():
    assert sha(name) == digest, name

env = dict(os.environ, PATH='/Users/Eric.Dong/.cargo/bin:/opt/homebrew/bin:' + os.environ.get('PATH', ''),
           CARGO_HOME=str(ROOT / 'rust/.cargo-home'), CARGO_TARGET_DIR=str(WS / 'rust/target'),
           CARGO_NET_OFFLINE='true', CARGO_BUILD_JOBS='2', FC='/opt/homebrew/bin/gfortran-16',
           CC='/usr/bin/clang', CALCEPH_DIR=str(PROVIDER), TEMPO2_ACCELERATE='1',
           AI_EPHEMERIS=str(ROOT / 'validation/native_backends/build/inpop17a_TDB_m100_p100_tt.dat'),
           AI_POSITION_ONLY=str(ROOT / 'validation/reference/build/source/T2runtime/ephemeris/DE405.1950.2050'),
           PYTHONDONTWRITEBYTECODE='1')
records = []

def run(label, command, timeout=1200):
    command = list(map(str, command))
    print('Running', label, flush=True)
    start = time.monotonic()
    log = OUT / (label + '.log')
    with log.open('wb') as f:
        result = subprocess.run(command, cwd=WS, env=env, stdout=f, stderr=subprocess.STDOUT, timeout=timeout)
    records.append(dict(label=label, command=command, exit=result.returncode,
                        seconds=time.monotonic()-start, log_sha256=sha(log)))
    (OUT / 'final-commands.json').write_text(json.dumps(records, indent=2) + '\n')
    assert result.returncode == 0, records[-1]
    print('Completed', label, flush=True)

base = ['cargo', 'test', '--offline', '--manifest-path', WS / 'rust/Cargo.toml', '--release', '--features', 'calceph,lapack']
run('final-focused-tests', [*base, '--test', 'native_backends', '--', '--include-ignored', '--test-threads=1'])
run('final-build-consumers', ['cargo', 'build', '--offline', '--manifest-path', WS / 'rust/Cargo.toml', '--release', '--features', 'calceph,lapack', '--example', 'native_backends', '--bin', 'tempo2-rust'])
run('final-comparator-controls', ['python3', '-B', '-m', 'unittest', 'discover', '-s', WS / 'validation/native_backends', '-p', 'test_check.py', '-v'])
run('final-paired', ['python3', '-B', WS / 'validation/native_backends/check.py', OUT / 'paired-release', '--native', NATIVE / 'probe', '--rust', WS / 'rust/target/release/examples/native_backends', '--profile', 'combined', '--ephemeris', env['AI_EPHEMERIS']])
run('final-cli', ['python3', '-B', WS / 'validation/native_backends/cli_check.py', OUT / 'cli-release', '--native', NATIVE / 'tempo2', '--rust', WS / 'rust/target/release/tempo2-rust', '--par', OUT / 'paired-release/inputs/calceph.par', '--tim', OUT / 'paired-release/inputs/pks.tim', '--lapack'])
sys.path.insert(0, str(WS / 'validation/native_backends'))
from cli_check import banner
banner(OUT / 'banner-release', NATIVE / 'tempo2', WS / 'rust/target/release/tempo2-rust', True, True)

paired = json.loads((OUT / 'paired-release/receipt.json').read_text())
assert paired['failures'] == 0 and paired['changed_artifacts'] == []
for name, digest in files.items():
    assert sha(WS / name) == digest, name
for name, digest in paired['files'].items():
    assert sha(name) == digest, name
result = dict(revision=REV, archive_sha256=sha(archive), changed_production_files=changed,
              committed_files=files, commands=records, native_identity_sha256=sha(NATIVE/'identity.json'),
              paired_receipt_sha256=sha(OUT/'paired-release/receipt.json'),
              paired_cases=len(paired['records']), paired_comparisons=sum(r['comparison']['checks'] for r in paired['records']),
              paired_failures=paired['failures'], paired_files_verified=len(paired['files']),
              cli_receipt_sha256=sha(OUT/'cli-release/receipt.json'),
              banner_receipt_sha256=sha(OUT/'banner-release/receipt.json'),
              rust_cli_sha256=sha(WS/'rust/target/release/tempo2-rust'),
              scope='Fresh combined-release build and selected complete backend/CLI comparisons plus real-provider recovery. Other build profiles and full library suites remain separately evidenced by the implementation report; no Linux or migration acceptance.')
(OUT/'final-review-completion.json').write_text(json.dumps(result, indent=2)+'\n')
print(json.dumps({k: result[k] for k in ['revision','paired_cases','paired_comparisons','paired_failures','paired_files_verified','scope']}, indent=2), flush=True)
