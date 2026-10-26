"""Verify AG delivery identities and the bounded committed-source central replay."""
from pathlib import Path
import hashlib
import json
import re
import subprocess

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT = Path(Path('/tmp/tempo2-ag-handover-current').read_text().strip())
PRODUCTION = '87099caf24089a9ca28091bb3072654adca9f040'
VALIDATION = 'cf4dfeb8ceb21e335193797e8c8ef6f3ebea5ba6'
REPORT = '6fcf5aac79ae490d15b049f1ea1fccfe53f3e9da'
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git_blob(revision, name):
    return subprocess.check_output(['git', 'show', revision + ':' + name], cwd=ROOT)


def checked_public(folder, workspace, executable_root):
    data = json.loads((folder / 'receipt.json').read_text())
    assert len(data['records']) == 18
    assert sum(record['checks'] for record in data['records']) == 21520
    assert all(not record['failures'] for record in data['records'])
    assert len(data['negative_controls']) == 12
    assert all(record['failures'] for record in data['negative_controls'])
    assert all(len(record['failures']) == 1 for record in data['negative_controls'][:4])
    assert len(data['commands']) == 43
    assert all(command['status'] == 0 for command in data['commands'])
    for name, digest in data['artifacts'].items():
        assert sha(folder / name) == digest, name
    for name, digest in data['inputs'].items():
        assert sha(workspace / name) == digest, name
    for name, digest in data['rust_executables'].items():
        assert sha(executable_root / name) == digest, name
    assert sha(ROOT / 'validation/analysis_tools/build/native-final/analysis') == data['native']
    assert sha(ROOT / 'validation/analysis_tools/build/native-original/analysis') == data['original_native']
    differences = {
        name: {'recorded': digest, 'current': sha(workspace / name)}
        for name, digest in data['authored'].items()
        if sha(workspace / name) != digest
    }
    return {
        'profile': data['profile'], 'receipt_sha256': sha(folder / 'receipt.json'),
        'source_revision': data['source_revision'], 'checks': 21520, 'failures': 0,
        'rejected_controls': 12, 'artifact_hashes_verified': len(data['artifacts']),
        'input_hashes_verified': len(data['inputs']),
        'executable_hashes_verified': len(data['rust_executables']),
        'recorded_authored_differences': differences,
    }


index_path = ROOT / 'validation/analysis_tools/build/delivery-identity.json'
assert sha(index_path) == 'e0870580ec787f083d62ed3f9fe5022b3a882df14cc97ddcefa3960d949a49c8'
index = json.loads(index_path.read_text())
for group, revision in [('logs_and_regression_receipts', None),
                        ('production_files', PRODUCTION), ('validation_files', VALIDATION)]:
    for name, digest in index[group].items():
        assert sha(ROOT / name) == digest, name
        if revision:
            assert hashlib.sha256(git_blob(revision, name)).hexdigest() == digest, name

reported = []
for profile, name in [('debug', 'debug-v2'), ('release', 'release-v1')]:
    reported.append(checked_public(ROOT / 'validation/analysis_tools/outputs' / name,
                                   ROOT, ROOT / 'rust/target' / profile / 'examples'))
assert not reported[0]['recorded_authored_differences']
assert reported[1]['recorded_authored_differences'] == {
    'validation/analysis_tools/regression.py': {
        'recorded': 'e2215b1fb5f33aadda669f014dbf0f8f4daf9c11cfd8f79025a24ddb186663b9',
        'current': '0cf57c3507fb405cc4737668888e023fbce1dae9ae1aa8074a5ee6d7e0f390de'}}

native = []
for name in ['native-original', 'native-final']:
    folder = ROOT / 'validation/analysis_tools/build' / name
    data = json.loads((folder / 'identity.json').read_text())
    assert data['pin'] == PIN
    for filename, hashes in data['sources'].items():
        assert sha(folder / 'source' / filename) == hashes['executed'], filename
    for filename, digest in data['objects'].items():
        assert sha(Path(filename)) == digest, filename
    for filename, digest in data['harness'].items():
        assert sha(ROOT / 'validation/analysis_tools' / filename) == digest, filename
    assert sha(ROOT / 'validation/analysis_tools/native.py') == data['builder']
    assert sha(ROOT / 'validation/reference/build/libtempo2-reference.a') == data['archive']
    assert sha(folder / 'corrections.patch') == data['patch']
    assert sha(folder / 'analysis') == data['executable']
    native.append({'name': name, 'identity_sha256': sha(folder / 'identity.json'),
                   'objects_verified': len(data['objects']), 'sources_verified': len(data['sources']),
                   'executable_sha256': data['executable']})

regression_indexes = []
for name in ['regression-debug-rest', 'regression-release']:
    folder = ROOT / 'validation/analysis_tools/build' / name
    data = json.loads((folder / 'index.json').read_text())
    assert not data['failures']
    for filename, digest in data['receipts'].items():
        assert sha(folder / filename) == digest, filename
    regression_indexes.append({'name': name, 'sha256': sha(folder / 'index.json'),
                               'receipts_verified': len(data['receipts'])})

build = json.loads((OUT / 'build-identity.json').read_text())
completion = json.loads((OUT / 'completion.json').read_text())
replay = json.loads((OUT / 'public-replay-completion.json').read_text())
assert build['revision'] == VALIDATION and build['production'] == PRODUCTION
assert sha(OUT / 'source.tar') == build['archive_sha256']
assert completion['build_exit'] == completion['behavior_exit'] == 0
assert completion['public_exit'] == 1  # Original Git-metadata publication failure remains recorded.
assert 'not a git repository' in (OUT / 'public.log').read_text()
assert replay['exit_code'] == 0 and replay['revision'] == VALIDATION
assert '3 passed; 0 failed' in (OUT / 'behavior.log').read_text()
fresh = checked_public(OUT / 'public-release-replay', OUT / 'workspace',
                       OUT / 'workspace/rust/target/release/examples')
assert fresh['source_revision'] == VALIDATION and not fresh['recorded_authored_differences']
for name, digest in completion['executables'].items():
    assert sha(OUT / 'workspace/rust/target/release/examples' / name) == digest

# This bounded textual inventory supplements the reviewed source/body map; it is
# not a C++ parser or proof of complete branch coverage.
source_map = (ROOT / 'rust/ANALYSIS_TOOLS_SOURCE_MAP.md').read_text()
inventory = []
pattern = r'(?m)^(?:(?:static|inline)\s+)*(?:void|float|double|longdouble|int|unsigned\s+long|unsigned\s+int|long|char)\s+([A-Za-z_]\w*)\s*\([^;{}]*\)\s*\{'
for name in ['T2toolkit.C', 'TKspectrum.C', 't2spectrum.C', 'GWsim.C']:
    source = subprocess.check_output(['git', '-C', str(ROOT / 'tempo2'), 'show', PIN + ':' + name], text=True)
    names = re.findall(pattern, source)
    missing = [name for name in names if '`' + name + '`' not in source_map]
    assert not missing
    inventory.append({'source': name, 'body_candidates': names, 'unmapped_candidates': missing})
assert sha(ROOT / 'decisions/0008-preservation-policy-and-stage-plan.md') == '2c5f81b82c43105fcd48b62139f1d78f2dc7c4089055ea162e9feef82ff4e12c'
receipt = {
    'scope': 'Central AG implementation handover; macOS arm64. Fresh committed-source release build, three behavioral tests and complete 18-case public replay. Broader owner receipts checked without rerunning unchanged campaigns. Not full migration or astrophysical validation.',
    'production_revision': PRODUCTION, 'validation_revision': VALIDATION, 'report_revision': REPORT,
    'delivery_index_sha256': sha(index_path),
    'delivery_entries_verified': {name: len(index[name]) for name in ['logs_and_regression_receipts', 'production_files', 'validation_files']},
    'reported_public': reported, 'native': native, 'regression_indexes': regression_indexes,
    'recorded_source_difference_disposition': 'The sole release census mismatch is regression.py, which run.py neither executes nor imports. The final wrapper and its final regression receipts are independently hash-verified. The old wrapper delta was not reconstructed exactly; the final committed public replay has no authored-source mismatch.',
    'fresh_public': fresh, 'fresh_behavioral_tests': 3,
    'initial_public_attempt': 'Exit 1 at receipt Git revision lookup in a source export without Git metadata. Raw log and products retained. Replay supplies a disposable shared bare repository at the exact export revision; committed observer unchanged.',
    'build_identity': build, 'replay_identity': replay, 'executables': completion['executables'],
    'body_inventory_scope': 'Regex candidates only, supplementary to source and header inspection.',
    'body_inventory': inventory,
    'checks_not_rerun': ['entire library suite centrally', 'full inherited numerical campaigns', 'actual Linux', 'installed C ABI', 'concurrency/performance'],
}
(OUT / 'central-review-receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
print(json.dumps({'receipt_sha256': sha(OUT / 'central-review-receipt.json'), 'fresh': fresh,
                  'delivery_entries_verified': receipt['delivery_entries_verified']}, indent=2))
