"""Replay committed AK execution contracts in an isolated, fresh Rust build."""
from pathlib import Path
import hashlib
import json
import os
import subprocess
import tempfile
import time

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
REV = subprocess.check_output(['git', 'rev-parse', '81de1e6'], cwd=ROOT, text=True).strip()
OUT = Path(tempfile.mkdtemp(prefix='tempo2-ak-committed-review-'))
Path('/tmp/tempo2-ak-committed-review-current').write_text(str(OUT) + '\n')
WS = OUT / 'workspace'
WS.mkdir()

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

archive = OUT / 'source.tar'
with archive.open('wb') as stream:
    subprocess.run(['git', 'archive', REV, 'rust', 'validation/fitting/inputs'],
                   cwd=ROOT, stdout=stream, check=True)
subprocess.run(['/usr/bin/tar', '-xf', archive, '-C', WS], check=True)
files = {}
for line in subprocess.check_output(
    ['git', 'ls-tree', '-r', REV, 'rust', 'validation/fitting/inputs'], cwd=ROOT, text=True
).splitlines():
    meta, name = line.split('\t', 1)
    mode, kind, oid = meta.split()
    assert kind == 'blob', name
    data = (WS / name).read_bytes()
    assert hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest() == oid, name
    files[name] = sha(WS / name)
(WS / 'tempo2').symlink_to(ROOT / 'tempo2', target_is_directory=True)
native_pin = subprocess.check_output(['git', '-C', str(ROOT/'tempo2'), 'rev-parse', 'HEAD'], text=True).strip()
assert native_pin == 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
env = dict(os.environ, PATH='/Users/Eric.Dong/.cargo/bin:/opt/homebrew/bin:' + os.environ.get('PATH', ''),
           CARGO_HOME=str(ROOT/'rust/.cargo-home'), CARGO_NET_OFFLINE='true', CARGO_BUILD_JOBS='2',
           FC='/opt/homebrew/bin/gfortran-16', CC='/usr/bin/clang', PYTHONDONTWRITEBYTECODE='1')
for key in ['CARGO_TARGET_DIR', 'CALCEPH_DIR', 'TEMPO2_ACCELERATE', 'TEMPO2', 'TEMPO2_CLOCK_PATH',
            'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'TEMPO2_AC_TEST']:
    env.pop(key, None)
identity = dict(revision=REV, native_pin=native_pin, source_archive_sha256=sha(archive),
                source_files=files, script_sha256=sha(__file__),
                environment={k: env[k] for k in ['PATH', 'CARGO_HOME', 'CARGO_BUILD_JOBS', 'FC', 'CC']})
(OUT/'identity.json').write_text(json.dumps(identity, indent=2) + '\n')
commands = []

def run(label, command, selected_env=env, timeout=120):
    start = time.monotonic()
    with (OUT/(label+'.stdout')).open('wb') as stdout, (OUT/(label+'.stderr')).open('wb') as stderr:
        result = subprocess.run(command, cwd=WS, env=selected_env, stdout=stdout, stderr=stderr, timeout=timeout)
    record = dict(label=label, command=command, exit=result.returncode, seconds=time.monotonic()-start,
                  stdout_sha256=sha(OUT/(label+'.stdout')), stderr_sha256=sha(OUT/(label+'.stderr')))
    commands.append(record)
    (OUT/'commands.json').write_text(json.dumps(commands, indent=2) + '\n')
    print(json.dumps(record), flush=True)
    assert result.returncode == 0, label

print('Fresh committed AK source: '+str(OUT), flush=True)
run('build', ['cargo', 'test', '--offline', '--locked', '--manifest-path', str(WS/'rust/Cargo.toml'),
              '--lib', '--no-run', '--message-format=json'], timeout=1200)
executables = []
for line in (OUT/'build.stdout').read_text().splitlines():
    event = json.loads(line)
    if event.get('reason') == 'compiler-artifact' and event.get('executable'):
        executables.append(event['executable'])
assert len(executables) == 1, executables
executable = executables[0]
tests = [
    'execution_tests::observation_workers_preserve_full_rows_effects_and_failed_prefix',
    'execution_tests::solar_workers_preserve_resource_history_and_failure_cleanup',
    'execution_tests::concurrent_complete_workflows_own_controls_and_diagnostic_collection',
    'execution_tests::execution_controls_restore_on_unwind',
    'execution_tests::observation_scratch_retains_unwritten_and_float_classes',
    'fitting_tests::terrestrial_fit_failure_recovers_with_the_same_runtime',
]
for number, test in enumerate(tests):
    run(f'test-{number}', [executable, test, '--exact', '--nocapture', '--test-threads=1'],
        dict(env, TEMPO2_AC_TEST=test))
for name, digest in files.items():
    assert sha(WS/name) == digest, name
completion = dict(revision=REV, tested=tests, passed=len(tests), production_files=len(files),
                  executable=executable, executable_sha256=sha(executable),
                  identity_sha256=sha(OUT/'identity.json'), commands_sha256=sha(OUT/'commands.json'),
                  scope='fresh macOS default debug build; six selected execution/recovery contracts; no benchmark or Linux claim')
(OUT/'completion.json').write_text(json.dumps(completion, indent=2) + '\n')
print(json.dumps(completion), flush=True)
