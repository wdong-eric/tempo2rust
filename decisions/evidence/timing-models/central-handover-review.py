"""Central AD handover review using the delivered identities and existing binaries.

This reviews implementation handover under 0074, not full numerical acceptance.
Writes fresh outputs only to a new temporary directory; never rebuilds or mutates
AD's original evidence. Reproduction requires the recorded AD checkout/binaries.
"""
from pathlib import Path
from decimal import Decimal
from copy import deepcopy
import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile

sys.dont_write_bytecode = True
ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
HERE = ROOT/'validation/timing_models'
BUILD = HERE/'build'
PRODUCTION = 'a8e5178778f7eeb3a0fc65f8934a64a03f94208d'
DELIVERY = '0f44180a021655074c387e2c03012d0c4253276e'
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
OUT = Path(tempfile.mkdtemp(prefix='tempo2-ad-handover-'))
verified = {}

def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024*1024), b''):
            h.update(block)
    return h.hexdigest()

def verify(path, digest):
    path = Path(path)
    key = str(path)
    if key not in verified:
        verified[key] = sha(path)
    assert verified[key] == digest, (key, digest, verified[key])

manifest_path = BUILD/'handover.json'
manifest_sha = sha(manifest_path)
m = json.loads(manifest_path.read_text())
assert m['head'] == DELIVERY and m['source_pin'] == PIN
head = subprocess.check_output(['git','rev-parse','HEAD'], cwd=ROOT, text=True).strip()
assert head == DELIVERY
assert not subprocess.check_output(['git','status','--porcelain'], cwd=ROOT)
for repo, pin in [(ROOT/'tempo2', PIN), (ROOT/'validation/reference', '7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572')]:
    assert subprocess.check_output(['git','rev-parse','HEAD'], cwd=repo, text=True).strip() == pin
    assert not subprocess.check_output(['git','status','--porcelain'], cwd=repo)
for group in ['source_sha256','validation_source_sha256','fixtures_sha256','original_G_sha256']:
    for name, digest in m[group].items():
        verify(ROOT/name, digest)
        if group != 'original_G_sha256':
            raw = subprocess.check_output(['git','show',DELIVERY+':'+name], cwd=ROOT)
            assert hashlib.sha256(raw).hexdigest() == digest, ('committed identity', name)
for group in ['logs_sha256','identities_sha256']:
    for name, digest in m[group].items():
        verify(BUILD/name, digest)
verify(ROOT/'decisions/0008-preservation-policy-and-stage-plan.md', m['original_0008_sha256'])
assert m['original_0008_sha256'] == '2c5f81b82c43105fcd48b62139f1d78f2dc7c4089055ea162e9feef82ff4e12c'
components = json.loads((BUILD/'components.json').read_text())
assert components['source_sha256'] == m['source_sha256']
assert len(components['commands']) == 8
for command in components['commands']:
    assert command['returncode'] == 0
    for name, digest in command['executable_sha256'].items():
        verify(Path(name), digest)
ni = json.loads((BUILD/'native/identity.json').read_text())
assert ni['pin'] == PIN
for name, key in [('native_timing.py','builder_sha256'), ('native_timing.cc','harness_sha256'), ('corrections.py','binary_corrections_sha256')]:
    verify(HERE/name, ni[key])
verify(BUILD/'native/timing', ni['executable_sha256'])
for group in ['objects','sources']:
    for name, digest in ni[group].items():
        verify(Path(name), digest)

sys.path.insert(0, str(HERE))
import check_public as comparator
comparator_sha = sha(HERE/'check_public.py')
public = []
maxima = {}
case_receipts = {}
for profile in ['debug','release']:
    assert len(m['public'][profile]) == 130
    for name, summary in m['public'][profile].items():
        work = BUILD/f'public-{profile}'/name
        path = work/'comparison.json'
        verify(path, summary['comparison_sha256'])
        r = json.loads(path.read_text())
        assert r['native_returncode'] == r['rust_returncode'] == 0 and not r['errors']
        assert r['rust_source_sha256'] == m['source_sha256']
        for filename, key in [('native.tsv','native_output_sha256'), ('rust.json','rust_output_sha256'), ('native-identity.json','native_identity_sha256')]:
            verify(work/filename, r[key])
        verify(Path(r['rust_command'][0]), r['rust_executable_sha256'])
        verify(Path(r['native_command'][0]), r['native_executable_sha256'])
        for filename, digest in r['input_sha256'].items():
            verify(Path(filename), digest)
        result = json.loads((work/'rust.json').read_text())
        for item in result['inputs'] + result['resources']:
            verify(Path(os.fsdecode(bytes(item['path']['bytes']))), item['sha256'])
        errors, measurements = comparator.compare((work/'native.tsv').read_text(), result)
        assert not errors, (profile, name, errors)
        updated_path = BUILD/f'recomparison-{profile}'/(name+'.json')
        verify(updated_path, m['corrected_comparator_recomparison_sha256'][profile+'/'+name])
        updated = json.loads(updated_path.read_text())
        assert updated['original_comparison_sha256'] == sha(path)
        assert updated['comparator_sha256'] == comparator_sha
        assert not updated['pipelines_rerun'] and not updated['errors']
        assert updated['measurements'] == measurements
        for item in measurements:
            field = item['field']
            if field.endswith('phase_turn') and not field.endswith('nphase_turn'):
                key = 'phase_turn'
            elif field.endswith(('residual_s','residualtn_s','binary_delay_s','pet_mjd')):
                key = 'AD_time_seconds'
            else:
                key = item.get('unit','exact_count')
            maxima[key] = max(maxima.get(key, Decimal(0)), abs(Decimal(item['difference'])))
        public.append(dict(profile=profile, case=name, measurements=len(measurements)))
        case_receipts[profile,name] = r
assert len(public) == 260
assert set(m['public']['debug']) == set(m['public']['release'])
for name in m['public']['debug']:
    debug = json.loads((BUILD/'public-debug'/name/'rust.json').read_text())
    release = json.loads((BUILD/'public-release'/name/'rust.json').read_text())
    assert debug['pulsars'] == release['pulsars'], name
# Independently repeat the three originally observed false-pass controls in memory.
work = BUILD/'public-debug/averaging-mean0'
original = json.loads((work/'rust.json').read_text())
reference = (work/'native.tsv').read_text()
controls = []
for name, alter in [
    ('missing_residual', lambda row: row['averaged'].pop('residual')),
    ('wrong_unit', lambda row: row['averaged']['residual'].__setitem__('unit','ms')),
    ('missing_all_averages', lambda row: row.__setitem__('averaged',{})),
]:
    altered = deepcopy(original)
    alter(altered['pulsars'][0]['observations'][0])
    errors, _ = comparator.compare(reference, altered)
    assert errors, name
    controls.append(dict(name=name, errors=errors))

fresh = []
for name in ['pks_dd-mean0','t2-companions-mean1','two-cut-mean1','library-noise-mean0']:
    stored = BUILD/'public-release'/name
    r = case_receipts['release',name]
    work = OUT/name
    work.mkdir()
    command = r['rust_command']
    run = subprocess.run(command, cwd=work, capture_output=True, timeout=60,
                         env={**os.environ,'LC_ALL':'C'})
    (work/'rust.json').write_bytes(run.stdout)
    (work/'rust.stderr').write_bytes(run.stderr)
    assert run.returncode == 0, (name, run.returncode, run.stderr)
    result = json.loads(run.stdout)
    errors, measurements = comparator.compare((stored/'native.tsv').read_text(), result)
    assert not errors, (name, errors)
    prior = json.loads((stored/'rust.json').read_text())
    assert result['diagnostics'] == prior['diagnostics'], name
    prior_avg = stored/'native.avg.dat'
    avg = work/'avg.dat'
    assert avg.exists() == prior_avg.exists(), name
    if avg.exists():
        assert avg.read_bytes() == prior_avg.read_bytes(), name
    verify(Path(command[0]), r['rust_executable_sha256'])
    assert sha(Path(command[0])) == r['rust_executable_sha256']
    fresh.append(dict(case=name, command=command, returncode=run.returncode,
                      executable_sha256=r['rust_executable_sha256'],
                      output_sha256=sha(work/'rust.json'),
                      stderr_sha256=sha(work/'rust.stderr'),
                      average_sha256=sha(avg) if avg.exists() else None,
                      measurements=len(measurements)))
    print(name, 'PASS', flush=True)
assert len(m['m2_unresolved']) == 4
assert sha(manifest_path) == manifest_sha
assert sha(HERE/'check_public.py') == comparator_sha
for path, digest in m['source_sha256'].items():
    assert sha(ROOT/path) == digest
receipt = dict(scope='AD implementation handover review under 0074; not full numerical acceptance',
               production=PRODUCTION, delivery=DELIVERY, head=head,
               manifest_sha256=manifest_sha, review_source_sha256=sha(Path(__file__)),
               verified_identities=verified, public=public,
               maxima={key:str(value) for key,value in maxima.items()},
               comparator_controls=controls, fresh=fresh,
               inherited_open_gates=m['other_inherited_gates'], m2_unresolved=m['m2_unresolved'])
(OUT/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(dict(output=str(OUT), receipt_sha256=sha(OUT/'receipt.json'),
                      verified_identity_count=len(verified), public_comparisons=len(public),
                      fresh_calls=len(fresh), maxima=receipt['maxima']),indent=2))
