#!/usr/bin/env python3
"""Review the installed 0106/0107 presentation change; no science gate waiver."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT = Path('/tmp/tempo2-central-presentation-review-v2-20261031')
CANDIDATE = '801bc94139cca90d6072234b09f5a1bcec5e4b05'
BUILD = ROOT / 'validation/release/build'
SOURCE = BUILD / 'presentation-candidate-v1/source'
PREFIX = BUILD / 'presentation-install-v1'
OUT.mkdir()
shutil.copyfile(__file__, OUT / 'executed-recipe.py')


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def read(path):
    return json.loads(path.read_text())


export = read(SOURCE.parent / 'receipt.json')
assert export['candidate'] == CANDIDATE
archive = OUT / 'candidate.tar'
with archive.open('xb') as stream:
    subprocess.run(['git', '-C', str(ROOT), 'archive', '--format=tar', CANDIDATE],
                   stdout=stream, check=True)
assert sha(archive) == export['archive_sha256']
with tarfile.open(archive) as bundle:
    git_files = {member.name: hashlib.sha256(bundle.extractfile(member).read()).hexdigest()
                 for member in bundle if member.isfile()}
assert git_files == export['source']
for name, digest in git_files.items():
    assert sha(SOURCE / name) == digest, name

install = read(BUILD / 'presentation-install-v1-build/install.json')
assert {str(p.relative_to(PREFIX)) for p in PREFIX.rglob('*') if p.is_file()} == set(install['files'])
for name, digest in install['files'].items():
    assert sha(PREFIX / name) == digest, name
    if name.startswith('share/tempo2-rust/rust/'):
        assert sha(PREFIX / name) == sha(SOURCE / name.removeprefix('share/tempo2-rust/')), name

original = BUILD / 'presentation-installed-v1'
retained = OUT / 'installed-evidence'
shutil.copytree(original, retained)
evidence_files = {}
for path in sorted(retained.rglob('*')):
    if path.is_file():
        name = str(path.relative_to(retained))
        assert sha(path) == sha(original / name), name
        evidence_files[name] = {'sha256': sha(path), 'bytes': path.stat().st_size}
receipt = read(retained / 'receipt.json')
assert receipt['status'] == 'pass' and all(receipt['checks'].values())
assert receipt['current'] == sha(PREFIX / 'bin/tempo2-rust')
assert receipt['baseline'] == sha(BUILD / 'second-followthrough-install-v1/bin/tempo2-rust')
for name, digest in receipt['inputs'].items():
    assert sha(Path(name)) == digest, name
for name, digest in receipt['diagnostic_attribution']['sources'].items():
    assert sha(SOURCE / name) == digest, name

# Freeze the delivered comparison implementation before executing it. Its full
# hashes are retained and checked against the later implementation commit.
harness = OUT / 'harness'
release = harness / 'validation/release'
release.mkdir(parents=True)
(harness / 'rust').symlink_to(SOURCE / 'rust', target_is_directory=True)
runner_hashes = {}
for name in ['common.py', 'diagnostic_attribution.py', 'attribute_generated_output.py',
             'test_diagnostic_attribution.py', 'presentation.py']:
    source = ROOT / 'validation/release' / name
    shutil.copyfile(source, release / name)
    runner_hashes[name] = sha(release / name)
assert runner_hashes['presentation.py'] == receipt['runner']
assert runner_hashes['diagnostic_attribution.py'] == receipt['projection']
sys.path.insert(0, str(release))
from diagnostic_attribution import Attribution
from presentation import normalized
mapping = Attribution(SOURCE)
# The receipt serializes tuple-valued source sets as JSON arrays.
assert json.loads(json.dumps(mapping.manifest())) == receipt['diagnostic_attribution']
checks = {}
for case in ['report', 'versions', 'debug']:
    before = normalized((retained / (case+'-before.stdout')).read_bytes(), 'native', mapping)
    after = normalized((retained / (case+'-after.stdout')).read_bytes(), 'rust', mapping)
    checks[case] = before == after
    assert before == (retained / (case+'-before.projected.stdout')).read_bytes()
    assert after == (retained / (case+'-after.projected.stdout')).read_bytes()
for case in ['evaluation', 'fit', 'bats']:
    checks[case] = mapping.structured(read(retained / (case+'-before.json')), 'native') == mapping.structured(read(retained / (case+'-after.json')), 'rust')
    checks[case+'-stderr'] = normalized((retained / (case+'-before.stderr')).read_bytes(), 'native', mapping) == normalized((retained / (case+'-after.stderr')).read_bytes(), 'rust', mapping)
assert all(checks.values()), checks

environment = dict(os.environ, PYTHONDONTWRITEBYTECODE='1')
command = [sys.executable, '-B', '-m', 'unittest', 'discover', '-s', str(release),
           '-p', 'test_diagnostic_attribution.py', '-v']
test = subprocess.run(command, env=environment, capture_output=True)
(OUT / 'controls.stdout').write_bytes(test.stdout)
(OUT / 'controls.stderr').write_bytes(test.stderr)
assert test.returncode == 0

# One fresh installed-product run establishes the user-facing example. The
# retained before/after suite above supplies the broader presentation checks.
work = OUT / 'fresh-demo'
work.mkdir()
command = [str(PREFIX / 'bin/tempo2-rust'), '-npsr', '1', '-nobs', '100', '-noaccel',
           '-f', str(SOURCE / 'validation/general_fitting/inputs/robust-H.par'),
           str(SOURCE / 'validation/general_fitting/inputs/multi-frequency.tim')]
environment.update(TEMPO2=receipt['runtime'], LC_ALL='C', VECLIB_MAXIMUM_THREADS='1',
                   OPENBLAS_NUM_THREADS='1', OMP_NUM_THREADS='1')
demo = subprocess.run(command, cwd=work, env=environment, capture_output=True, timeout=60)
(OUT / 'fresh-demo.stdout').write_bytes(demo.stdout)
(OUT / 'fresh-demo.stderr').write_bytes(demo.stderr)
assert demo.returncode == 0 and not demo.stderr
assert normalized(demo.stdout, 'rust', mapping) == normalized((retained / 'report-after.stdout').read_bytes(), 'rust', mapping)
report = {
    'scope': 'macOS installed presentation review only; no independent scientific equivalence or Linux acceptance',
    'candidate': CANDIDATE, 'archive_sha256': sha(archive),
    'verified_git_files': len(git_files), 'verified_installed_files': len(install['files']),
    'verified_evidence_files': evidence_files, 'replayed_checks': checks,
    'negative_controls': {'exit': test.returncode, 'tests': 4, 'stderr_sha256': sha(OUT/'controls.stderr')},
    'fresh_demo': {'argv': command, 'exit': demo.returncode,
                   'stdout_sha256': sha(OUT/'fresh-demo.stdout'), 'stderr_sha256': sha(OUT/'fresh-demo.stderr')},
    'runner_hashes': runner_hashes, 'recipe_sha256': sha(OUT/'executed-recipe.py'),
    'installed_receipt_sha256': sha(retained/'receipt.json'),
    'export_receipt_sha256': sha(SOURCE.parent/'receipt.json'),
    'install_receipt_sha256': sha(BUILD/'presentation-install-v1-build/install.json'),
}
(OUT/'review.json').write_text(json.dumps(report, indent=2)+'\n')
print(json.dumps({k: report[k] for k in ['candidate', 'verified_git_files', 'verified_installed_files', 'replayed_checks', 'negative_controls']}))
print('review_sha256', sha(OUT/'review.json'))
