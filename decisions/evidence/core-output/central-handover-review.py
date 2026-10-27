"""Verify the corrected AH delivery and the bounded fresh central release replay."""
from pathlib import Path
import hashlib
import json
import subprocess

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT = Path(Path('/tmp/tempo2-ah-committed-review-current').read_text().strip())
BASE = ROOT / 'validation/core_output/build'
NATIVE = BASE / 'native-v9'
PRODUCTION = '0dbcf0842394eb9cfaf93cb31bfac428e2ef7d67'
VALIDATION = '6ef441a1a46a964770352bebdc3395ef12045c10'
CORRECTION = '50f2bf33baaa3a0bfea0666c062ab5b59ba6b63b'
REPORT = '9dec65cfe7696a150f131077494e487871b22fb7'
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
verified = {}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(path, digest):
    assert path.is_file() and sha(path) == digest, str(path)
    verified[str(path)] = digest


def git(*args, cwd=ROOT):
    return subprocess.check_output(['git', *args], cwd=cwd)


def blob_sha(revision, name):
    return hashlib.sha256(git('show', revision + ':' + name)).hexdigest()


def matrix(folder, root, executable):
    d = json.loads((folder / 'receipt.json').read_text())
    assert d['summary'] == {'unexpected_status': 0, 'changed_inputs': 0,
        'artifact_mismatches': 0, 'different_files': 17, 'missing_files': 3,
        'extra_files': 0, 'failed_cases': 12, 'exit_status': 1}, d['summary']
    assert len(d['comparison']) == 37 and len(d['runs']) == 79
    verify(executable, d['rust'])
    verify(NATIVE / 'tempo2', d['native'])
    verify(NATIVE / 'identity.json', d['native_identity'])
    verify(root / 'validation/core_output/CONTRACT.md', d['contract_sha256'])
    verify(root / 'validation/core_output/check.py', d['checker_sha256'])
    assert d['runtime_before'] == d['runtime_after']
    assert d['rust_sources_before'] == d['rust_sources_after']
    for name, digest in d['runtime_before'].items():
        verify(root / 'validation/reference/build/source/T2runtime' / name, digest)
    for name, digest in d['rust_sources_before'].items():
        verify(root / name, digest)
        assert blob_sha(PRODUCTION, name) == digest, name
    count = 0
    for record in d['runs']:
        assert record['status'] == record['expected']
        assert record.get('inputs_unchanged', True)
        for name, digest in record.get('inputs', {}).items():
            verify(Path(name), digest)
        if record['kind'] == 'identity':
            assert 'directory' not in record
            continue
        directory = Path(record['directory'])
        assert directory.is_dir() and directory.resolve().is_relative_to(folder.resolve())
        current = {str(p.relative_to(directory)): sha(p) for p in directory.rglob('*') if p.is_file()}
        assert current == record['files'], record['case']
        assert record['artifact_check'] == {'missing': [], 'extra': [], 'changed': []}
        for name, digest in current.items():
            verify(directory / name, digest)
        count += len(current)
    assert count == 267
    help_dir, extra_dir = folder / 'controls/help', folder / 'controls/help-extra'
    assert not help_dir.samefile(extra_dir)
    assert (help_dir / 'stdout').read_bytes() != (extra_dir / 'stdout').read_bytes()
    return {'receipt_sha256': sha(folder / 'receipt.json'), 'summary': d['summary'],
            'comparison': d['comparison'], 'run_file_hashes_verified': count,
            'runtime_files': len(d['runtime_before']), 'rust_sources': len(d['rust_sources_before'])}


old_path = BASE / 'delivery-identity.json'
new_path = BASE / 'delivery-identity-help-corrected.json'
verify(old_path, '7c63ad15e0ab1a0fcc3b7a48991f3c28c273f4b2b2a036f6e3db3366bd7dacc7')
verify(new_path, '8ab38dd6a78ecdad4b33d2b33ebc4785670067c5131231af1b08aa5944bda498')
old, new = [json.loads(p.read_text()) for p in [old_path, new_path]]
for group in ['artifacts', 'executables']:
    for name, digest in old[group].items():
        verify(ROOT / name, digest)
for name, digest in old['authored_files'].items():
    revision = PRODUCTION if name.startswith('rust/') else VALIDATION
    assert blob_sha(revision, name) == digest, name
    if name not in new['authored_files']:
        verify(ROOT / name, digest)
for name, digest in new['authored_files'].items():
    verify(ROOT / name, digest)
    assert blob_sha(CORRECTION, name) == digest
for name, digest in new['artifacts'].items():
    verify(ROOT / name, digest)
assert new['production_commit'] == PRODUCTION
assert new['validation_correction_commit'] == CORRECTION
verify(ROOT / 'rust/CORE_OUTPUT_REPORT.md', blob_sha(REPORT, 'rust/CORE_OUTPUT_REPORT.md'))
for name, state in new['protected_checkouts'].items():
    assert git('rev-parse', 'HEAD', cwd=ROOT / name).decode().strip() == state['head']
    assert git('status', '--short', cwd=ROOT / name).decode().strip() == state['status'] == ''
verify(ROOT / 'decisions/0008-preservation-policy-and-stage-plan.md',
       '2c5f81b82c43105fcd48b62139f1d78f2dc7c4089055ea162e9feef82ff4e12c')
for name in ['help.txt', 'help_extra.txt']:
    assert (ROOT / 'rust/data' / name).read_bytes() == git('show', PIN + ':' + name, cwd=ROOT / 'tempo2')
preservation = json.loads((ROOT / new['preservation']['before_index']).read_text())
assert len(preservation['historical_artifacts']) == 622
for name, digest in preservation['historical_artifacts'].items():
    verify(ROOT / name, digest)

native = json.loads((NATIVE / 'identity.json').read_text())
assert native['pin'] == PIN
for name, values in native['sources'].items():
    verify(NATIVE / 'source' / name, values['compiled'])
for name, digest in native['objects'].items():
    verify(Path(name), digest)
for name, digest in native['executables'].items():
    verify(NATIVE / name, digest)
verify(ROOT / 'validation/reference/build/libtempo2-reference.a', native['archive'])
verify(ROOT / 'validation/predictors/build/native-corrected/identity.json', native['af_identity'])
verify(ROOT / 'validation/core_output/native_build.py', native['builder'])
verify(NATIVE / 'corrections.patch', native['patch'])
reported = [matrix(BASE / (profile + '-help-corrected'), ROOT,
                   ROOT / 'rust/target' / profile / 'tempo2-rust') for profile in ['debug', 'release']]
assert reported[0]['comparison'] == reported[1]['comparison']
for profile in ['debug', 'release']:
    assert reported[0]['comparison'] == json.loads((BASE / (profile + '-final/receipt.json')).read_text())['comparison']

bugs = json.loads((BASE / 'bugs-v2/receipt.json').read_text())
assert len(bugs['checks']) == 17 and all(bugs['checks'].values())
bug_files = 0
for record in bugs['runs']:
    directory = BASE / 'bugs-v2' / record['case']
    if record['case'] in ['helpers', 'activation', 'snapshot-poison']:
        directory /= record['kind']
    for name, digest in record['files'].items():
        verify(directory / name, digest)
        bug_files += 1
ut1 = json.loads((BASE / 'ut1-check-v1/receipt.json').read_text())
assert len(ut1['checks']) == 21 and all(ut1['checks'].values())
for when in ['before', 'after']:
    for name, digest in ut1[when].items():
        verify(BASE / ('ut1-' + when) / name, digest)
for record in ut1['runs']:
    clock, kind = record['case'].split('-', 1)
    for name, digest in record['files'].items():
        verify(BASE / 'ut1-check-v1' / clock / kind / name, digest)

build = json.loads((OUT / 'build-completion.json').read_text())
replay = json.loads((OUT / 'replay-completion.json').read_text())
assert build['production'] == PRODUCTION and build['validation'] == VALIDATION
assert replay['validation_revision'] == CORRECTION
assert sha(OUT / 'source.tar') == build['source_archive_sha256']
assert sha(OUT / 'corrected-validation.tar') == replay['overlay_archive_sha256']
for c in build['commands'] + replay['commands']:
    assert c['exit'] == c.get('expected', 0)
    verify(OUT / (c['name'] + '.log'), c['log_sha256'])
assert '5 passed; 0 failed; 2 ignored' in (OUT / 'core-tests.log').read_text()
assert '7 passed; 0 failed' in (OUT / 'behavior.log').read_text()
assert 'Ran 4 tests' in (OUT / 'checker-controls.log').read_text()
fresh = matrix(OUT / 'matrix-release', OUT / 'workspace',
               OUT / 'workspace/rust/target/release/tempo2-rust')
assert fresh['comparison'] == reported[0]['comparison']
assert fresh['receipt_sha256'] == replay['matrix_receipt_sha256']
assert sha(OUT / 'ut1-check/receipt.json') == build['ut1_receipt_sha256']
fresh_ut1 = json.loads((OUT / 'ut1-check/receipt.json').read_text())
assert len(fresh_ut1['checks']) == 21 and all(fresh_ut1['checks'].values())
warning = json.loads(Path('/tmp/central-ah-warning-review.json').read_text())
verify(Path(warning['source']), warning['source_sha256'])
assert warning['equal_scalar_values'] == 6716 and len(warning['differences']) == 3
for name, digest in verified.items():
    assert sha(Path(name)) == digest, name
receipt = {
    'scope': 'Central AH implementation handover, not numerical or full-migration acceptance.',
    'production': PRODUCTION, 'validation': VALIDATION, 'correction': CORRECTION, 'report': REPORT,
    'old_delivery_index_sha256': sha(old_path), 'corrected_delivery_index_sha256': sha(new_path),
    'native_sources_verified': len(native['sources']), 'native_objects_verified': len(native['objects']),
    'old_entries_verified': {key: len(old[key]) for key in ['artifacts', 'authored_files', 'executables']},
    'new_entries_verified': {key: len(new[key]) for key in ['artifacts', 'authored_files']},
    'historical_artifacts_preserved': 622, 'bug_witness_checks': 17, 'bug_files_verified': bug_files,
    'owner_ut1_checks': 21, 'reported': reported, 'fresh': fresh,
    'fresh_core_tests': 5, 'fresh_behavior_tests': 7, 'fresh_control_tests': 4, 'fresh_ut1_checks': 21,
    'build_receipt_sha256': sha(OUT / 'build-completion.json'),
    'replay_receipt_sha256': sha(OUT / 'replay-completion.json'),
    'warning_diagnosis_sha256': sha(Path('/tmp/central-ah-warning-review.json')),
    'warning_diagnosis': warning,
    'protected_checkouts': new['protected_checkouts'],
    'metadata_disposition': 'Five formerly untracked canonical .DS_Store files are absent. Both pins and tracked scientific sources are unchanged; no cause is attributed.',
    'not_rerun': ['entire owner library suite and Clippy', 'unchanged inherited numerical corpus',
                  'actual Linux', 'installed external ABI', 'concurrency/performance'],
    'verified_files': verified,
}
(OUT / 'central-review-receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
print(json.dumps({'receipt_sha256': sha(OUT / 'central-review-receipt.json'),
                  'verified_files': len(verified), 'fresh_summary': fresh['summary'],
                  'bug_files_verified': bug_files}, indent=2))
