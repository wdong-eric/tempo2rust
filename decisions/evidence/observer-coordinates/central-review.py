"""Central X replay, mutation and observer verification in owned local output."""
from pathlib import Path
import collections, gzip, hashlib, json, shutil, subprocess, sys, tempfile
sys.dont_write_bytecode = True
ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
HERE = ROOT/'validation/observer_coordinates'
# Source paths enter bounded TKlog buffers. Preserve their byte width so the
# ordinary exact comparator sees the same diagnostic bytes before projection.
base = Path(tempfile.gettempdir())
prefix_length = len(str(HERE))-len(str(base))-1-8
assert prefix_length >= 2
OUT = Path(tempfile.mkdtemp(prefix='x-'+'x'*(prefix_length-2)))
assert len(str(OUT).encode()) == len(str(HERE).encode())
print('OUT', OUT, flush=True)
sys.path.insert(0, str(HERE))
import inventory, profile, run
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
load = lambda p: json.loads(p.read_text())
accepted = HERE/'outputs'
original_files = [p for p in HERE.rglob('*') if p.is_file() and '__pycache__' not in p.parts]
before = {str(p): sha(p) for p in sorted(original_files)}
assert inventory.preservation_status() == {}
catalog = run.frozen_cases()
baseline = run.read_gzip(accepted/'reference.json.gz')
ids = [c['id'] for c in catalog]
run.compare(ids, [r['id'] for r in baseline], 'membership')
assert len(catalog) == 133 and sum(len(r['events']) for r in baseline) == 3462
for c, r in zip(catalog, baseline): run.validate(c, r)
receipts = {n: load(accepted/(n+'.json')) for n in ('last_execution','controls','independent','coverage','observer_audit')}
for n, r in receipts.items():
    assert r['status'] == 'PASS', n
    assert r['preservation_before'] == r['preservation_after'] == {}, n
for kind, name in (('ordinary','last_execution'),('coverage','coverage'),('observer','observer_audit')):
    profile.actual_identity(HERE/'build'/kind/'probe', receipts[name]['profile'])
# Reconstruct measured primary coverage directly from the retained gcov files.
cov = receipts['coverage']; raw = []
for p in (HERE/'build/coverage').glob('*.gcov.json.gz'): raw.extend(run.read_gzip(p)['files'])
routines = []; lines = []; branches = []
for r in load(accepted/'frozen/source_map.json')['routines']:
    matches = [f for f in raw if Path(f['file']).name == r['file']]
    assert len(matches) == 1
    f = matches[0]
    fn = next(v for v in f['functions'] if v['demangled_name'].split('(')[0] == r['name'])
    routines.append(dict(file=r['file'], name=r['name'], measured=fn))
    for item in f['lines']:
        if not r['start'] <= item['line_number'] <= r['end']: continue
        lines.append(dict(file=r['file'], function=r['name'], **item))
        branches.extend(dict(file=r['file'], function=r['name'], line=item['line_number'], index=i, **b) for i,b in enumerate(item.get('branches',[])))
strip = lambda x: {k:v for k,v in x.items() if k not in ('classification','reason')}
assert routines == cov['routines']
assert lines == [strip(x) for x in cov['lines']]
assert branches == [strip(x) for x in cov['branches']]
summary = dict(routines=len(routines), routines_entered=sum(r['measured']['execution_count']>0 for r in routines), lines=len(lines), lines_hit=sum(x['count']>0 for x in lines), branches=len(branches), branches_hit=sum(x['count']>0 for x in branches))
assert summary == cov['summary'] == dict(routines=9,routines_entered=9,lines=333,lines_hit=327,branches=214,branches_hit=207)
assert cov['ordinary_exact_cases'] == ids and cov['ordinary_differences'] == []
by_id = {r['id']:r for r in baseline}
cr = run.read_gzip(accepted/'coverage_receipts.json.gz')
run.compare(ids, [r['id'] for r in cr], 'coverage membership')
for r in cr:
    name = r['id']; folder = accepted/'coverage_raw'
    events = gzip.decompress((folder/(name+'.jsonl.gz')).read_bytes())
    stdout = (folder/(name+'.stdout')).read_bytes(); stderr = (folder/(name+'.stderr')).read_bytes()
    assert r['raw_sha256'] == {k:hashlib.sha256(v).hexdigest() for k,v in [('events',events),('stdout',stdout),('stderr',stderr)]}
    exe = HERE/'build/coverage/probe'; expected = by_id[name]
    run.compare(expected['events'], [json.loads(run.normalize(s,exe)) for s in events.decode().splitlines()], name+'.coverage.events')
    run.compare(expected['stdout'], run.normalize(stdout.decode('latin1'),exe), name+'.coverage.stdout')
    run.compare(expected['stderr'], run.normalize(stderr.decode('latin1'),exe), name+'.coverage.stderr')
print('PASS delivered coverage and actual build identities', summary, flush=True)
original = load(accepted/'manifest.json')
macro_bytes = (HERE/'build/ordinary/macros.txt').read_bytes()
assert hashlib.sha256(macro_bytes).hexdigest() == original['native']['macros_sha256']
for name in ('reference.json.gz','manifest.json'): shutil.copy2(accepted/name, OUT/name)
shutil.copytree(accepted/'frozen', OUT/'frozen')
profile.BUILD = OUT/'build'; run.BUILD = profile.BUILD; run.OUTPUT = OUT
exe, fresh = profile.compile_probe()
def rebind(x, build):
    if isinstance(x,str): return x.replace(str(build),'{build}')
    if isinstance(x,list): return [rebind(v,build) for v in x]
    if isinstance(x,dict): return {rebind(k,build):rebind(v,build) for k,v in x.items()}
    return x
old = profile.stable(original['native']); new = profile.stable(fresh)
new_macros = (exe.parent/'macros.txt').read_bytes()
assert sorted(macro_bytes.splitlines()) == sorted(new_macros.splitlines())
macro_evidence = dict(accepted_sha256=hashlib.sha256(macro_bytes).hexdigest(), fresh_sha256=hashlib.sha256(new_macros).hexdigest(), complete_sorted_lines_sha256=hashlib.sha256(b'\n'.join(sorted(macro_bytes.splitlines()))).hexdigest())
old.pop('macros_sha256'); new.pop('macros_sha256')
a = rebind(old,HERE/'build'); b = rebind(new,profile.BUILD)
assert a == b, [(k,a.get(k),b.get(k)) for k in set(a)|set(b) if a.get(k)!=b.get(k)]
print('PASS fresh native profile after explicit build-root rebind and complete macro-line check',flush=True)
ordinary_receipts = []
for case, expected in zip(catalog, baseline):
    actual, receipt = run.execute(case,exe,fresh,OUT/'ordinary_raw'); run.validate(case,actual); run.compare(expected,actual,case['id'])
    replay, again = run.execute(case,exe,fresh,OUT/'replay_raw'); run.validate(case,replay); run.compare(actual,replay,case['id']+'.fresh')
    ordinary_receipts.append(dict(first=receipt,replay=again))
    if len(ordinary_receipts)%20 == 0: print('PASS fresh replay',len(ordinary_receipts),'cases',flush=True)
run.write_gzip(OUT/'execution_receipts.json.gz',ordinary_receipts)
import controls, independent, observer_audit
controls.main()
control = load(OUT/'controls.json'); assert control['status'] == 'PASS'
assert len(control['controls']) == 35 and collections.Counter(x['status'] for x in control['controls']) == {'PASS':3,'REJECTED':32}
mutants = run.read_gzip(OUT/'mutations.json.gz'); assert len(mutants) == 15
for i,m in enumerate(mutants):
    assert m['name'] == controls.MUTATIONS[i][0]
    profile.actual_identity(profile.BUILD/('mutant%02d'%i)/'probe',m['profile'])
    assert controls.differences(by_id[m['case']]['events'],m['result']['events']) == m['witnesses']
independent.main()
observer_audit.main()
observer = load(OUT/'observer_audit.json')
assert observer['status'] == 'PASS' and observer['matches'] == observer['fresh_replays'] == 133 and observer['prefix_and_retention_assertions'] == 7
fresh_observed = run.read_gzip(OUT/'observer_snapshots.json.gz')
run.compare(run.read_gzip(accepted/'observer_snapshots.json.gz'),fresh_observed,'delivered complete observer snapshots')
assert inventory.preservation_status() == {}
assert {str(p):sha(p) for p in sorted(original_files)} == before
assert set(before) == {str(p) for p in HERE.rglob('*') if p.is_file() and '__pycache__' not in p.parts}
profile.actual_identity(exe,fresh)
result = dict(status='PASS', scope='Fresh central ordinary replay, controls, compiled mutations and observer replay; delivered coverage reconstructed and raw-checked, not rerun', output=str(OUT), cases=len(catalog), events=sum(len(x['events']) for x in baseline), controls=len(control['controls']), compiled_mutations=len(mutants), independent_checks=len(load(OUT/'independent.json')['checks']), observer_matches=observer['matches'], observer_prefix_assertions=observer['prefix_and_retention_assertions'], coverage=summary, fresh_profile=fresh, macro_projection=macro_evidence, protected_x_files=before, protected_prior_files=len(load(accepted/'frozen/protected.json')), delivery_revision='02234cd', central_source_revision=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(), script_sha256=sha(Path(__file__)), accepted_receipt_hashes={n:sha(accepted/(n+'.json')) for n in receipts}, fresh_receipt_hashes={n:sha(OUT/n) for n in ('execution_receipts.json.gz','controls.json','mutations.json.gz','independent.json','observer_audit.json','observer_snapshots.json.gz','observer_receipts.json.gz')})
path = OUT/'central-review.json'; path.write_text(json.dumps(result,indent=2)+'\n')
print('PASS central X',len(catalog),'cases, 35 controls, 15 mutations, 133 complete observer snapshots',flush=True)
print('RECEIPT',path,sha(path),flush=True)
