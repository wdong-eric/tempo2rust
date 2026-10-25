"""Central AA evidence audit and selected fresh Rust checks; accepted inputs stay read-only."""
import copy
import hashlib
import importlib.util
import json
import os
import secrets
import subprocess
import sys
from pathlib import Path

root = Path(sys.argv[1]).resolve()
out = Path(sys.argv[2]).resolve()
out.mkdir(parents=True, exist_ok=True)
here = root / 'validation/observer_coordinates_port'
x = root / 'validation/observer_coordinates'
rust = root / 'rust'
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
read = lambda p: json.loads(Path(p).read_text())
def save(p, value):
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps(value, indent=2) + '\n')
spec = importlib.util.spec_from_file_location('central_aa', here / 'run.py')
aa = importlib.util.module_from_spec(spec)
spec.loader.exec_module(aa)
delivery_path = here / 'build/delivery.json'
assert sha(delivery_path) == '87c49810d758791f7519ce4da3d05432bc43634036bf9292027b9de42df7dd2e'
delivery = read(delivery_path)
for name, digest in delivery['artifacts'].items():
    assert sha(root / name) == digest, name
catalog = aa.fixtures.cases()
expected = aa.unpack(x / 'outputs/observer_snapshots.json.gz')
ordinary = aa.unpack(x / 'outputs/reference.json.gz')
assert len(catalog) == len(expected) == len(ordinary) == 133
aa.exact_compare(catalog, read(x / 'outputs/frozen/cases.json'))
expected_by = {c['id']: e for c, e in zip(catalog, expected)}
profiles = {}
for profile in ['debug', 'release']:
    base = here / 'outputs' / profile
    direct = read(base / 'direct.json')
    assert direct['status'] == 'PASS' and direct['scope'] == 'all'
    assert direct['identity'] == aa.identity(Path(direct['identity']['executable']))
    assert [r['id'] for r in direct['cases']] == [c['id'] for c in catalog]
    for name, digest in direct['oracle'].items():
        assert sha(x / 'outputs' / name) == digest
    nonces = set()
    for row, e, o in zip(direct['cases'], expected, ordinary):
        p = base / row['id']
        actual = read(p / 'result.json')
        assert sha(p / 'result.json') == row['result_sha256']
        aa.exact_compare(e, actual, row['id'] + '.private')
        projection = copy.deepcopy(actual)
        projection['events'] = [v for v in projection['events'] if v['phase'] != 'exit']
        for v in projection['events']:
            if 'private' in v:
                v['private'] = None
        aa.exact_compare(o, projection, row['id'] + '.ordinary')
        launch, inputs = read(p / 'launch.json'), read(p / 'input.json')
        assert launch['status'] == 0
        assert launch['input_sha256'] == sha(p / 'input.json')
        assert launch['executable_sha256'] == direct['identity']['executable_sha256']
        challenge = read(p / 'events.jsonl.launch')
        assert challenge == dict(nonce=inputs['nonce'], input_sha256=launch['input_sha256'])
        assert inputs['nonce'] not in nonces
        nonces.add(inputs['nonce'])
        def norm(s):
            return s.replace(str(x/'build/ordinary/source'), '{source}').replace(str(here/'work/x'), '{cases}').replace(str(x), '{observer}')
        assert actual['events'] == [json.loads(norm(v)) for v in (p/'events.jsonl').read_text().splitlines()]
        assert actual['stdout'] == norm((p/'events.jsonl.stdout').read_bytes().decode('latin1'))
        assert actual['stderr'] == norm((p/'events.jsonl.stderr').read_bytes().decode('latin1'))
    previous = here / 'build/pre-single-boundary' / profile
    controls = read(previous / 'controls/receipt.json')
    assert controls['status'] == 'PASS'
    assert len(controls['controls']) == 35
    assert sum(r['status'] == 'REJECTED' for r in controls['controls']) == 32
    assert len(controls['supporting_checks']) == 19
    assert len(controls['retained_mutation_witnesses']) == 15
    assert controls['corrected_buffers'] == 19 and controls['original_buffers'] == 7
    assert controls['direct_sha256'] == sha(previous / 'direct.json')
    assert controls['mutation_source_sha256'] == sha(x / 'outputs/mutations.json.gz')
    assert controls['control_source_sha256'] == sha(x / 'controls.py')
    assert all(r['differences'] for r in controls['retained_mutation_witnesses'])
    for name, digest in controls['identity']['sources'].items():
        if sha(root / name) != digest:
            blob = subprocess.check_output(['git', 'show', 'e4883a4:'+name], cwd=root)
            assert hashlib.sha256(blob).hexdigest() == digest, name
    assert sha(controls['identity']['executable']) == controls['identity']['executable_sha256']
    profiles[profile] = dict(direct_sha256=sha(base/'direct.json'), identity=direct['identity'],
        cases=len(direct['cases']), events=sum(r['events'] for r in direct['cases']),
        native_exits=sum(e['status'] == 1 for e in expected), unique_nonces=len(nonces),
        historical_controls_sha256=sha(previous/'controls/receipt.json'))
    assert profiles[profile]['events'] == 3468 and profiles[profile]['native_exits'] == 6
    print('PASS recorded', profile, '133 complete comparisons, 3468 events; controls retain original identity', flush=True)
follow = read(here/'build/followup/receipt.json')
assert follow['status'] == 'PASS'
assert follow['sources'] == {n: sha(root/n) for n in follow['sources']}
for row in follow['commands']:
    assert row['status'] == 0 and sha(here/'build/followup'/(row['name']+'.log')) == row['log_sha256']
for profile, row in follow['profiles'].items():
    assert sha(row['executable']) == row['sha256'] == profiles[profile]['identity']['executable_sha256']
arithmetic = read(here/'build/arithmetic/receipt.json')
assert len(arithmetic['witnesses']) == 18
assert sha(here/'build/initial-debug/run/direct.json') == arithmetic['initial_receipt_sha256']
for name, row in arithmetic['objects'].items():
    assert sha(row['source']) == row['source_sha256']
    assert sha(row['accepted_object']) == row['accepted_object_sha256']
    assert sha(here/'build/arithmetic'/(name+'.s')) == row['assembly_sha256']
    assert sha(here/'build/arithmetic'/(name+'.object.s')) == row['object_disassembly_sha256']
public = read(here/'build/public/aa-receipt.json')
assert public['failures'] == delivery['M2_failures'] == read(rust/'verification/m2/open_gate_failures.json')
assert len(public['failures']) == 4
assert sha(here/'build/public/commands.json') == public['commands_sha256']
assert sha(here/'build/public/numerical_errors.json') == public['results_sha256']

# Fresh processes use only central-owned files; equal-length input roots preserve
# the accepted byte/string projection without altering X or AA work directories.
input_parent = out / ('i' * (len(str(x/'build/cases')) - len(str(out)) - 1))
assert len(str(input_parent)) == len(str(x/'build/cases'))
env = dict(os.environ, LC_ALL='C', LANG='C', RUST_TEST_THREADS='1')
selected = [c for c in catalog if c['id'].startswith(('c038_', 'c041_', 'c114_', 'c127_', 'c128_', 'c131_'))]
assert len(selected) == 6
white_path = root/'decisions/evidence/observer-whitespace/build/central-il7tunxj/receipt.json'
assert sha(white_path) == 'cb9ff2427e37a7b1df76fd377b41efc396f69889f5ae77b9e694c351c5ed9239'
white = read(white_path)
selected += [dict(id='corrected_white', classification='0053-overlap-safe-correction', files={},
    actions=[['white_hex', r['input_hex']] for r in white['records']['corrected']])]
for profile in profiles:
    exe = Path(profiles[profile]['identity']['executable'])
    rows = []
    for case in selected:
        work = input_parent / case['id']
        work.mkdir(parents=True, exist_ok=True)
        for name, data in case['files'].items():
            p = work / name
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text(data)
        destination = out/profile/case['id']
        inp = destination/'input.json'
        save(inp, dict(nonce=secrets.token_hex(16), actions=[[v.replace('{case}',str(work)) for v in a] for a in case['actions']]))
        event = destination/'events.jsonl'
        cmd = [str(exe),'observer_coordinates_tests::observer_probe','--exact','--ignored','--nocapture']
        result = subprocess.run(cmd, cwd=rust, env=dict(env,TEMPO2_AA_INPUT=str(inp),TEMPO2_AA_OUTPUT=str(event)), capture_output=True, text=True, timeout=120)
        save(destination/'launch.json',dict(command=cmd,status=result.returncode,stdout=result.stdout,stderr=result.stderr,
            executable_sha256=sha(exe),input_sha256=sha(inp)))
        assert result.returncode == 0, result.stdout+result.stderr
        assert read(str(event)+'.launch') == dict(nonce=read(inp)['nonce'],input_sha256=sha(inp))
        def normalize(s):
            return s.replace(str(x/'build/ordinary/source'),'{source}').replace(str(input_parent),'{cases}').replace(str(x),'{observer}')
        actual = dict(id=case['id'], classification=case.get('classification','defined-host-profile'),actions=case['actions'],
            status={'success':0,'source_exit':1,'boundary':2}[Path(str(event)+'.outcome').read_text()],
            events=[json.loads(normalize(v)) for v in event.read_text().splitlines()],
            stdout=normalize(Path(str(event)+'.stdout').read_bytes().decode('latin1')),
            stderr=normalize(Path(str(event)+'.stderr').read_bytes().decode('latin1')))
        if case['id'] == 'corrected_white':
            buffers = [e['result']['buffer'] for e in actual['events'] if e['phase']=='end']
            assert buffers == [r['buffer_after_hex'] for r in white['records']['corrected']]
            assert buffers[:7] == [r['buffer_after_hex'] for r in white['records']['original']]
        else:
            aa.exact_compare(expected_by[case['id']],actual,case['id'])
        save(destination/'result.json',actual)
        rows.append(dict(id=case['id'],status=actual['status'],events=len(actual['events']),result_sha256=sha(destination/'result.json')))
    tests = []
    for name, args in [('boundaries',['observer_coordinates_tests::','--test-threads=1']),
        ('live-frontend',['observer_coordinates_tests::existing_frontend_observes_imag_alias_mutation','--exact','--ignored','--nocapture'])]:
        cmd = [str(exe), *args]
        result = subprocess.run(cmd,cwd=rust,env=env,capture_output=True,text=True,timeout=120)
        log = out/profile/(name+'.log')
        log.write_text(result.stdout+result.stderr)
        assert result.returncode == 0, result.stdout+result.stderr
        assert ('3 passed;' if name=='boundaries' else '1 passed;') in result.stdout
        tests.append(dict(command=cmd,status=result.returncode,log=str(log),log_sha256=sha(log)))
    profiles[profile]['fresh'] = rows
    profiles[profile]['tests'] = tests
    assert profiles[profile]['identity'] == aa.identity(exe)
    print('PASS fresh',profile,'six histories, 19 corrected buffers, ownership/boundary tests and live frontend IMAG',flush=True)
for repo,pin in delivery['repositories'].items():
    assert subprocess.check_output(['git','-C',str(root/repo),'rev-parse','HEAD'],text=True).strip()==pin
    assert not subprocess.check_output(['git','-C',str(root/repo),'status','--short'],text=True)
assert sha(root/'decisions/0008-preservation-policy-and-stage-plan.md')=='2c5f81b82c43105fcd48b62139f1d78f2dc7c4089055ea162e9feef82ff4e12c'
receipt = dict(status='PASS',delivery_sha256=sha(delivery_path),reviewed_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),
    script_sha256=sha(__file__),profiles=profiles,artifacts_verified=delivery['artifacts'],source_repositories=delivery['repositories'],
    scope='Recorded AA comparisons rechecked; selected fresh Rust processes; no native rebuild, no historical full rerun, no complete 40923-file preservation rehash')
save(out/'receipt.json',receipt)
print('PASS CENTRAL AA',sha(out/'receipt.json'),flush=True)
