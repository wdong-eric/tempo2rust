"""Central read-only audit of Y's delivered evidence; does not rerun implementations."""
import hashlib
import importlib.util
import json
import subprocess
import sys
from pathlib import Path

root = Path(sys.argv[1]).resolve()
output = Path(sys.argv[2]).resolve()
here = root / 'validation/time_ephemeris_port'
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
read = lambda p: json.loads(Path(p).read_text())
spec = importlib.util.spec_from_file_location('y_central_delivery', here / 'run.py')
y = importlib.util.module_from_spec(spec)
spec.loader.exec_module(y)
delivery_path = here / 'outputs/delivery.json'
assert sha(delivery_path) == 'b0c19132bf54bb3bea2bfb53a19b22ac1ec73c1b5e0ff0e93dab83ff634a9d5c'
delivery = read(delivery_path)
for name, digest in delivery['artifacts'].items():
    assert sha(root / name) == digest, name
assert sha(here / 'outputs/module_inventory.json') == delivery['module_inventory_sha256']
native = read(here / 'outputs/native/receipt.json')
assert sha(native['profile']['executable_path']) == native['profile']['executable_sha256']
records = {row['id']: row for row in native['records']}
assert len(records) == 97
profiles = {}
for profile in ['debug', 'release']:
    direct_path = here / 'outputs' / profile / 'direct.json'
    direct = read(direct_path)
    assert sha(direct_path) == delivery['profiles'][profile]['direct_sha256']
    assert direct['complete'] and not direct['failures']
    assert [r['id'] for r in direct['cases']] == list(records)
    changed = []
    for name, digest in direct['identity']['sources'].items():
        if sha(root / name) != digest:
            changed.append(name)
            # These source bodies are reviewed separately against the follow-up.
            original = subprocess.check_output(['git', 'show', '5343b1d:' + name], cwd=root)
            assert hashlib.sha256(original).hexdigest() == digest, name
    assert set(changed) <= {'rust/src/read_fortran.rs', 'rust/src/tt2tdb.rs',
                            'rust/src/time_ephemeris_tests.rs',
                            'validation/time_ephemeris_port/CONTRACT.md'}, changed
    nonces = set()
    for row in direct['cases']:
        case = row['id']
        path = Path(direct['run_directory']) / case
        assert sha(path / 'result.json') == row['result_sha256'], case
        raw_path = here / 'outputs/native/equalpaths' / (case + '.json.gz')
        assert sha(raw_path) == records[case]['sha256'], case
        actual = read(path / 'result.json')
        launch, inputs = read(path / 'launch.json'), read(path / 'input.json')
        assert launch['status'] == 0 and launch['identity'] == direct['identity']
        assert sha(path / 'input.json') == launch['input_sha256']
        assert actual['events'][0]['nonce'] == inputs['nonce']
        assert actual['events'][0]['input_sha256'] == launch['input_sha256']
        assert inputs['nonce'] not in nonces
        nonces.add(inputs['nonce'])
        assert actual['events'] == [json.loads(s) for s in (path / 'events.jsonl').read_text().splitlines()]
        assert actual['stdout'] == (path / 'events.jsonl.stdout').read_bytes().hex()
        assert actual['stderr'] == (path / 'events.jsonl.stderr').read_bytes().hex()
        assert len(actual['events']) == row['events'] and actual['status'] == row['status']
        expected = y.ownership_disposition(y.project(y.native_payload(y.unpack(raw_path))))
        y.compare(expected, y.project(actual), case)
    control_path = here / 'outputs' / profile / 'controls.json'
    pointer = read(control_path)
    assert sha(pointer['path']) == pointer['sha256']
    controls = read(pointer['path'])
    assert len(controls['controls']) == pointer['count'] == 50
    assert sum(c.get('rejected') is True for c in controls['controls']) == 42
    assert sum(c.get('passed') is True for c in controls['controls']) == 3
    assert sum(c.get('boundary') == 'Fortran_incomplete_record_int' for c in controls['controls']) == 4
    assert sum(c.get('status') == 2 and '--no-build' in c.get('stderr', '') for c in controls['controls']) == 1
    assert len(controls['independent']) == pointer['independent'] == 36
    profiles[profile] = dict(cases=len(direct['cases']), events=sum(r['events'] for r in direct['cases']),
        native_exits=sum(r['status'] == 1 for r in direct['cases']), unique_launch_nonces=len(nonces),
        original_identity=direct['identity'], followup_changed_sources=changed,
        direct_sha256=sha(direct_path), control_pointer_sha256=sha(control_path))
    assert profiles[profile]['events'] == 1368 and profiles[profile]['native_exits'] == 9
    print('PASS recorded', profile, '97 comparisons / 1368 events / 50 controls / 36 independent checks', flush=True)
commands = read(here / 'outputs/checks/commands.json')
for command in commands:
    assert command['status'] == 0 and sha(command['log']) == command['sha256']
mutations = read(here / 'outputs/mutations/receipt.json')['mutations']
assert len(mutations) == 13
for row in mutations:
    assert sha(row['path']) == row['sha256']
    assert sha(row['profile']['executable_path']) == row['profile']['executable_sha256']
    assert row['witness']
arithmetic = read(here / 'build/arithmetic/receipt.json')
for name, digest in arithmetic['files'].items():
    assert sha(here / 'build/arithmetic' / name) == digest
assert len(arithmetic['checks']) == 100
assert all(c['o2'] == c['rust'] for c in arithmetic['checks'])
for name, index in arithmetic['witnesses'].items():
    column = ['recurrence', 'derivative', 'sum', 'record', 'dot'].index(name)
    assert arithmetic['checks'][index]['o2'][column] != arithmetic['checks'][index]['unfused'][column]
pins = {}
for repo, pin in [('tempo2', 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'),
                  ('validation/reference', '7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572')]:
    assert subprocess.check_output(['git', '-C', str(root/repo), 'rev-parse', 'HEAD'], text=True).strip() == pin
    assert not subprocess.check_output(['git', '-C', str(root/repo), 'status', '--short'], text=True)
    pins[repo] = pin
followup_root = here / 'outputs/fb-eof-review/dc1372fcfe9b3d6d'
followup_path = followup_root / 'receipt.json'
assert sha(followup_path) == 'd1abe756f062c66d2cdcc2ea1395445321ac57a8df84b5ab79a1e2e56037deb3'
followup = read(followup_path)
assert followup['status'] == 'PASS'
assert sha(followup_root / 'commands.json') == followup['commands_sha256']
assert sha(followup_root / 'before.json') == followup['before_sha256']
before = read(followup_root / 'before.json')
for name, digest in before['protected'].items():
    assert sha(root / name) == digest, name
for command in read(followup_root / 'commands.json'):
    assert command['returncode'] == 0 and sha(command['log']) == command['sha256']
for profile, data in followup['profiles'].items():
    identity = data['identity']
    for name, digest in identity['sources'].items():
        assert sha(root / name) == digest, name
    assert sha(identity['compiler']) == identity['compiler_sha256']
    assert sha(identity['executable']) == identity['executable_sha256']
    retained = followup['retained_delivered_executables'][profile]
    assert sha(retained['copy']) == retained['sha256'] == profiles[profile]['original_identity']['executable_sha256']
    log = (followup_root / (profile + '-fb-regressions.log')).read_text()
    for test in data['tests']:
        assert 'test time_ephemeris_tests::' + test + ' ... ok' in log
    assert '2 passed; 0 failed;' in log
    assert [r['id'] for r in data['primitive_eof']] == ['fortran_big', 'fortran_little', 'fortran_partial']
    for row in data['primitive_eof']:
        path = followup_root / profile / row['id']
        result = read(path / 'result.json')
        assert sha(path / 'result.json') == row['result_sha256']
        raw_path = here / 'outputs/native/equalpaths' / (row['id'] + '.json.gz')
        assert sha(raw_path) == row['native_record_sha256']
        launch, inputs = read(path / 'launch.json'), read(path / 'input.json')
        assert launch['status'] == 0 and launch['identity'] == identity
        assert sha(path / 'input.json') == launch['input_sha256'] == result['events'][0]['input_sha256']
        assert inputs['nonce'] == result['events'][0]['nonce']
        y.compare(y.project(y.native_payload(y.unpack(raw_path))), y.project(result), row['id'])
print('PASS focused follow-up identities, two FB tests and three EOF histories each profile; 94 protected files', flush=True)
receipt = dict(status='PASS identity audit and recomparison of delivered evidence; no fresh implementation replay',
    reviewer_sha256=sha(__file__), reviewed_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),
    original_delivery_sha256=sha(delivery_path), artifacts=delivery['artifacts'], profiles=profiles,
    original_focused_logs=len(commands), recorded_compiled_mutations=13,
    followup_receipt_sha256=sha(followup_path), followup_protected_files=len(before['protected']),
    followup_profiles={k:dict(tests=v['tests'], primitive_eof=v['primitive_eof'], identity=v['identity']) for k,v in followup['profiles'].items()},
    recorded_arithmetic_inputs=100, separating_expressions=list(arithmetic['witnesses']), pins=pins,
    retained_M2_failures=delivery['M2_failures'],
    preservation='Original delivery reported 23666 unchanged files; this audit does not rehash that whole manifest.')
output.parent.mkdir(parents=True, exist_ok=True)
output.write_text(json.dumps(receipt, indent=2)+'\n')
print('PASS evidence identities, recorded arithmetic and mutations, and original source pins; receipt', output, flush=True)
