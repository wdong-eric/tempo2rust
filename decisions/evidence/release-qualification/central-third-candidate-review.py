"""Bind AL's third Linux candidate, retained evidence and fresh harness checks."""
from pathlib import Path, PurePosixPath
import hashlib
import io
import json
import os
import subprocess
import sys
import tarfile

root = Path('/Users/Eric.Dong/Downloads/tempo2rust')
candidate = 'a35104a611abb5c1d5c48dc4d1866f239ccd3ab9'
baseline = '4accd26e2d51b3dc805d9a1ae0a2be7195e171ea'
out = Path('/tmp/tempo2-central-third-candidate-a35104a-review')
out.mkdir(exist_ok=False)
packet = root / 'validation/release/build/publication-review-v4'
evidence = root / 'validation/release/build/second-linux-followthrough-evidence-v2'

def sha(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()

def git(*args):
    return subprocess.check_output(['git', *args], cwd=root)

review = json.loads((packet / 'receipt.json').read_text())
assert review['candidate'] == candidate
for name, expected in review['files'].items(): assert sha(packet / name) == expected, name
for name, args in [('candidate.patch', ['diff', '--binary', review['central_base'], candidate, '--', 'rust', 'validation/release', '.github/workflows/tempo2-rust.yml']),
                   ('full-linux-baseline.patch', ['diff', '--binary', baseline, candidate])]:
    assert git(*args) == (packet / name).read_bytes(), name
for original, saved in [('rust/RELEASE_SECOND_LINUX_FOLLOWUP.md', 'FOLLOWUP.md'),
    ('rust/RELEASE_ORDINARY_TYPE_PROPOSALS.md', 'PROPOSALS.md'), ('rust/RELEASE_SOURCE_MAP.md', 'SOURCE_MAP.md'),
    ('rust/RELEASE_QUALIFICATION_REPORT.md', 'REPORT.md'), ('validation/release/README.md', 'RECIPES.md'),
    ('.github/workflows/tempo2-rust.yml', 'tempo2-rust.yml')]:
    assert git('show', candidate + ':' + original) == (packet / saved).read_bytes(), original
assert git('show', candidate + ':.github/workflows/tempo2-rust.yml') == git('show', baseline + ':.github/workflows/tempo2-rust.yml')
git('diff', '--check', baseline, candidate)
assert subprocess.run(['git', 'merge-base', '--is-ancestor', baseline, candidate], cwd=root).returncode == 0

bound = json.loads((packet / 'evidence-receipt.json').read_text())
assert sha(evidence / 'index.json') == bound['index_sha256']
assert sha(evidence / 'manifest.json') == bound['manifest_sha256']
archive = Path(bound['archive'])
assert sha(archive) == bound['archive_sha256'] == review['archive_sha256']
assert archive.stat().st_size == bound['archive_bytes']
manifest = json.loads((evidence / 'manifest.json').read_text())['files']
seen = set(); total = 0
with tarfile.open(archive) as tf:
    for member in tf:
        assert member.isdir() or member.isfile(), member.name
        if member.isdir(): continue
        name = str(PurePosixPath(member.name).relative_to(evidence.name))
        assert name not in seen, name
        seen.add(name)
        expected = {'sha256': bound['manifest_sha256'], 'bytes': (evidence / 'manifest.json').stat().st_size} if name == 'manifest.json' else manifest[name]
        stream = tf.extractfile(member); assert stream is not None
        digest = hashlib.sha256(stream.read()).hexdigest()
        assert digest == expected['sha256'] == sha(evidence / name), name
        assert member.size == expected['bytes'] == (evidence / name).stat().st_size, name
        total += member.size
assert seen == set(manifest) | {'manifest.json'}
assert len(seen) == bound['verified_archive_files'] == 8037 and total == bound['selected_bytes']

source_archive = git('archive', candidate)
(out / 'candidate.tar').write_bytes(source_archive)
with tarfile.open(fileobj=io.BytesIO(source_archive)) as tf:
    tf.extractall(out / 'source', filter='data')
index = json.loads((evidence / 'index.json').read_text())
assert len(index['production_source_identity']) == 157
for name, expected in index['production_source_identity'].items():
    assert sha(out / 'source/rust/src' / name) == expected, name
changed = git('diff', '--name-only', index['production_commit'], candidate, '--', 'rust').decode().splitlines()
assert all(Path(name).suffix == '.md' for name in changed), changed
added_or_changed = git('diff', '--name-only', '--diff-filter=ACMRT', baseline, candidate).decode().splitlines()
sizes = {name: len(git('show', candidate + ':' + name)) for name in added_or_changed}
assert max(sizes.values()) < 50 * 1024 * 1024
assert not any(name.endswith(('.json', '.jsonl', '.json.gz', '.jsonl.gz', '.tar.gz')) for name in sizes)

command = [sys.executable, '-m', 'unittest', 'discover', '-s', 'validation/release', '-p', 'test_*.py']
result = subprocess.run(command, cwd=out / 'source', env={**os.environ, 'PYTHONDONTWRITEBYTECODE': '1'}, capture_output=True, text=True)
(out / 'harness.stdout').write_text(result.stdout); (out / 'harness.stderr').write_text(result.stderr)
receipt = {'candidate': candidate, 'baseline': baseline, 'review_source': sha(__file__),
    'publication_receipt': sha(packet / 'receipt.json'), 'archive': str(archive), 'archive_sha256': bound['archive_sha256'],
    'archive_files_verified': len(seen), 'archive_bytes_verified': total,
    'production_files_verified': len(index['production_source_identity']), 'unchanged_production_inputs_after_test': True,
    'workflow_unchanged': True, 'changed_file_sizes': sizes, 'source_archive': sha(out / 'candidate.tar'),
    'harness': {'command': command, 'exit': result.returncode, 'stdout_sha256': sha(out / 'harness.stdout'), 'stderr_sha256': sha(out / 'harness.stderr')},
    'scope': 'Central publication review and fresh frozen harness tests; no Linux qualification acceptance'}
(out / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
print(json.dumps({'out': str(out), 'receipt_sha256': sha(out / 'receipt.json'), 'archive_files': len(seen),
                  'production_files': 157, 'changed_files': len(sizes), 'harness_exit': result.returncode, 'harness_tail': result.stderr[-1400:]}))
raise SystemExit(result.returncode)
