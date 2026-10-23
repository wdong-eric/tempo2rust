#!/usr/bin/env python3
"""Central O review: fresh direct runs, isolated outputs, unchanged delivery."""
import gzip
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

sys.dont_write_bytecode = True
ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
DELIVERY = '8d2be6f'
HERE = ROOT / 'validation/parameter_storage_port'
FROZEN = ROOT / 'rust/verification/parameter_storage'
sys.path.insert(0, str(HERE))
import run as port
import native

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def git_blob(path):
    return subprocess.check_output(['git', 'show', DELIVERY + ':' + path], cwd=ROOT)

work = Path(tempfile.mkdtemp(prefix='tempo2-parameter-central-', dir='/private/tmp'))
out = work / 'results'
out.mkdir()
summary = {
    'delivery_commit': subprocess.check_output(['git', 'rev-parse', DELIVERY], cwd=ROOT, text=True).strip(),
    'driver_sha256': sha(__file__),
    'work': str(work),
    'adaptations': [
        'Redirect run.OUT and native.OUT to a fresh temporary directory.',
        'Redirect native.HERE to a temporary caller directory containing an exact copy of O probe.cc; unchanged K prepare stages raw source there.',
        'No source, fixture, expected value, comparator, control or tolerance changes.',
    ],
}
print('central work', work, flush=True)
artifacts = port.load(FROZEN / 'artifact_hashes.json')
for rel, expected in artifacts.items():
    path = FROZEN / rel
    assert sha(path) == expected, ('current delivery artifact', rel)
    assert hashlib.sha256(git_blob(str(path.relative_to(ROOT)))).hexdigest() == expected, ('committed artifact', rel)
summary['committed_artifacts_verified'] = len(artifacts)
for path in [FROZEN / 'artifact_hashes.json', ROOT / 'rust/PARAMETER_STORAGE_REPORT.md']:
    assert path.read_bytes() == git_blob(str(path.relative_to(ROOT)))
summary['protected_files_verified'] = len(port.protected())
for folder, revision in [('tempo2', 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'), ('validation/reference', '7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572')]:
    assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT/folder, text=True).strip() == revision
    assert not subprocess.check_output(['git', 'status', '--porcelain'], cwd=ROOT/folder)
summary['nested_repositories'] = 'clean at both pinned revisions'
base_identity = port.identity()
for rel, digest in base_identity['sources'].items():
    assert hashlib.sha256(git_blob(rel)).hexdigest() == digest, ('committed executed source', rel)
summary['committed_executed_sources_verified'] = len(base_identity['sources'])
delivery_checks = port.load(FROZEN / 'delivery_checks.json')
assert delivery_checks['runner_sha256'] == sha(HERE / 'verify.py')
assert delivery_checks['command_receipts_sha256'] == sha(FROZEN / 'commands.json')
assert delivery_checks['regressions_sha256'] == sha(FROZEN / 'regressions/delivery_checks.json')
for command in port.load(FROZEN / 'commands.json'):
    assert command['returncode'] == 0
    assert command['log_sha256'] == sha(FROZEN / command['log'])
assert port.load(FROZEN / 'regressions/m2/open_gate_failures.json') == port.load(ROOT / 'rust/verification/m2/open_gate_failures.json')
assert len(delivery_checks['M2_failures']) == 4
summary['delivered_full_regressions'] = delivery_checks

port.OUT = out
native.OUT = out
native.HERE = work / 'native_caller'
native.HERE.mkdir()
shutil.copyfile(HERE / 'probe.cc', native.HERE / 'probe.cc')
assert sha(native.HERE / 'probe.cc') == sha(HERE / 'probe.cc')
summaries = []
for profile in ['debug', 'release']:
    sys.argv = [str(HERE / 'run.py'), '--profile', profile]
    port.main()
    fresh = port.load(out / (profile + '_direct.json'))
    old = port.load(FROZEN / (profile + '_direct.json'))
    port.validate_identity(old['identity'], fresh['identity'])
    assert fresh['counts'] == old['counts']
    assert fresh['controls'] == old['controls']
    for suffix, omitted in [('_raw.json.gz', ['token', 'input_sha256']), ('_native.json.gz', ['token'])]:
        a = json.loads(gzip.decompress((out / (profile + suffix)).read_bytes()))
        b = json.loads(gzip.decompress((FROZEN / (profile + suffix)).read_bytes()))
        for key in omitted:
            a.pop(key)
            b.pop(key)
        assert a == b, (profile, suffix, 'raw data differ')
    command = ['cargo', 'test', '--locked', '--offline', 'parameter_storage_tests'] + (['--release'] if profile == 'release' else [])
    p = subprocess.run(command, cwd=port.RUST, env=port.ENV, capture_output=True, text=True)
    log = out / (profile + '_focused_tests.log')
    log.write_text(p.stdout + p.stderr)
    assert p.returncode == 0, p.stderr
    assert '4 passed; 0 failed; 1 ignored' in p.stdout, p.stdout
    port.validate_identity(fresh['identity'], port.identity(Path(fresh['executable'])))
    summaries.append({
        'profile': profile, 'counts': fresh['counts'], 'cases': fresh['cases'],
        'events': fresh['events'], 'controls': fresh['controls'],
        'identity': fresh['identity'], 'native_identity': fresh['native'],
        'fresh_receipt_sha256': sha(out / (profile + '_direct.json')),
        'focused_tests': {'command': command, 'returncode': p.returncode, 'log_sha256': sha(log)},
        'raw_delivery_equality': True,
    })
    print(profile, 'fresh direct and four focused tests verified', flush=True)
assert port.identity() == base_identity
assert len(port.protected()) == summary['protected_files_verified']
for rel, expected in artifacts.items():
    assert sha(FROZEN / rel) == expected, ('delivery changed after review', rel)
summary['profiles'] = summaries
summary['status'] = 'PASS: central fresh debug/release direct checks; unchanged pinned full-regression evidence; M2 still unresolved'
summary['artifacts'] = {str(path.relative_to(out)): sha(path) for path in sorted(out.rglob('*')) if path.is_file()}
target = Path('/tmp/tempo2-parameter-central-review.json')
target.write_text(json.dumps(summary, indent=2, sort_keys=True) + '\n')
print('PASS', target, flush=True)
