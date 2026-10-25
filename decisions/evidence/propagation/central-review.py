"""Central AB evidence audit plus selected fresh ordinary executions under 0065."""
import collections
import gzip
import hashlib
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(sys.argv[1]).resolve()
OUT = Path(sys.argv[2]).resolve()
HERE = ROOT/'validation/propagation'
sys.path.insert(0, str(HERE))
import inventory
import profile as native
import run
import supplemental
import observer_audit
import independent
import controls

sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
read = lambda p: json.loads(Path(p).read_text())
OUT.mkdir(parents=True, exist_ok=True)
source_before = {p.name:sha(p) for p in [*HERE.glob('*.py'), HERE/'probe.cc', HERE/'CONTRACT.md']}
verified = read(HERE/'outputs/verification.json')
assert sha(HERE/'outputs/verification.json') == 'df4ffad35228f2f1ac3bbd1ae21c361a05b5a99b78a79a5fb62abbd25d143fe7'
assert verified['status']=='PASS' and verified['sources']==source_before
for name, digest in verified['oracles'].items():
    assert sha(HERE/'outputs'/name)==digest
phase = {}
for name, row in verified['receipts'].items():
    path=HERE/'outputs'/row['path']
    assert sha(path)==row['sha256'], name
    phase[name]=read(path)
    assert phase[name]['status']=='PASS', name
source_map=read(HERE/'outputs/frozen/source_map.json')
assert len(source_map['routines'])==25
assert len(source_map['types']['pulsar'])==349 and len(source_map['types']['observation'])==95
for name, digest in source_map['hashes'].items():
    assert hashlib.sha256(inventory.blob(name)).hexdigest()==digest, name
profiles={}
for name, directory in [('ordinary','ordinary'),('supplemental','supplemental'),('observer','observer'),('coverage','coverage')]:
    pf=phase[name]['profile']; exe=HERE/'build'/directory/'probe'
    native.actual_identity(exe,pf)
    if name in ('ordinary','supplemental','coverage'):
        for file in inventory.PRIMARY:
            assert (exe.parent/'source'/file).read_bytes()==inventory.blob(file), (name,file)
    profiles[name]=dict(executable=str(exe),executable_sha256=sha(exe),dependencies=len(pf['dependencies']))
print('PASS all delivered source, phase, original-source and four build identities',flush=True)

catalog=run.frozen_cases()+supplemental.frozen_cases()
results=run.read_gzip(HERE/'outputs/reference.json.gz')+run.read_gzip(HERE/'outputs/supplemental/reference.json.gz')
assert len(catalog)==len(results)==324
run.compare([c['id'] for c in catalog],[r['id'] for r in results])
by={r['id']:r for r in results}; cases={c['id']:c for c in catalog}
for case,result in zip(catalog,results):
    run.validate(case,result)
    for event in result['events']:
        for _,record in event['state']['changes']:
            independent.check_record(record)
assert sum(len(r['events']) for r in results[:311])==3917
assert sum(len(r['events']) for r in results[311:])==255
outcomes=collections.Counter(str(r['status']) for r in results)
assert outcomes=={'0':314,'1':7,'-6':2,'timeout':1}, outcomes
assert len(phase['independent']['checks'])==167
for row in phase['independent']['checks']:
    assert row.get('exact') is True or 0<=row['absolute_difference']<=row['supporting_bound']
assert len(phase['controls']['controls'])==32
assert collections.Counter(r['status'] for r in phase['controls']['controls'])=={'PASS':3,'REJECTED':29}
observed=run.read_gzip(HERE/'outputs/observer_snapshots.json.gz')
assert sha(HERE/'outputs/observer_snapshots.json.gz')==phase['observer']['snapshot_sha256']
assert len(observed)==324
for original,extra in zip(results,observed):
    run.compare(original,observer_audit.project(extra),original['id']+'.recorded_observer_projection')
loops=[x for e in next(r for r in observed if r['id']=='iteration_limit')['events'] if e.get('private') for x in e['private']['events'] if x['kind']=='iteration']
assert len(loops)==100 and loops[0]['loop']==0 and loops[-1]['loop']==99
assert loops[0]['old']=='0000000000000000'
del observed
print('PASS 324 typed oracle cases and complete recorded observer projections; 100-iteration state',flush=True)

coverage=phase['coverage']
assert coverage['summary']==dict(routines=25,routines_entered=25,lines=585,lines_hit=581,branches=400,branches_hit=388)
assert coverage['ordinary_exact_cases']==[c['id'] for c in catalog] and not coverage['ordinary_differences']
raw_files=[]
for path in (HERE/'build/coverage').glob('*.gcov.json.gz'):
    raw_files.extend(json.loads(gzip.decompress(path.read_bytes()))['files'])
for row in coverage['routines']:
    file=next(f for f in raw_files if Path(f['file']).name==row['file'])
    assert row['measured'] in file['functions']
for row in coverage['lines']:
    file=next(f for f in raw_files if Path(f['file']).name==row['file'])
    line=next(x for x in file['lines'] if x['line_number']==row['line_number'])
    assert all(row[k]==v for k,v in line.items())
for row in coverage['branches']:
    file=next(f for f in raw_files if Path(f['file']).name==row['file'])
    line=next(x for x in file['lines'] if x['line_number']==row['line'])
    assert all(row[k]==v for k,v in line['branches'][row['index']].items())
mutant_count=0
for name in ['mutations','branch_mutations']:
    path=HERE/'outputs'/(name+'.json.gz')
    gate=phase['controls' if name=='mutations' else 'branch_controls']
    assert sha(path)==gate['mutation_sha256']
    mutants=run.read_gzip(path)
    assert len(mutants)==gate['mutations']
    for row in mutants:
        pf=row['profile'];exe=Path(pf['commands'][-1][pf['commands'][-1].index('-o')+1])
        assert sha(exe)==pf['executable_sha256']
        file,old,new=pf['mutation']
        original=inventory.blob(file).decode();assert original.count(old)==1
        assert (exe.parent/'source'/file).read_text()==original.replace(old,new)
        expected=row.get('supporting_original')
        expected=expected['original'] if expected else by[row['case']]
        witness=controls.differences(expected,row['result'])
        run.compare(row['witnesses'],witness,row['name']+'.recorded_witness')
        assert witness and row['receipt']['launch_before']['executable_sha256']==sha(exe)
        assert row['receipt']['launch_after']==row['receipt']['launch_before']
        mutant_count+=1
assert mutant_count==48
print('PASS raw coverage counts and 48 recorded compiled behavioral witnesses',flush=True)

# Only the case work directory is routed; accepted builds, oracles and runners are read-only.
run.BUILD=OUT/'work'
selected=['counts_-1','bclt_tropo_0','bclt_tropo_1','bclt_STL_FBAT_1',
 'iteration_limit','iteration_fast_1_deleted_0','shapiro_A_2','shapiro_bypass',
 'derivative_cross_selector','dmmodel_1','shape_cross_pulsar','solar_model_0_grid',
 'solar_model_1_polar','bats_-1_1','secular_flags_1_0','phase_cross_0_1',
 'phase_cross_1_0','composed','sheet_prefix','sheet_nonconsuming','ifunc_one',
 'hermite_0','shapelet_12_0','precision_noop','supp_mcl_conjunction_exit',
 'supp_mcl_year_exit','supp_ifunc_nan_assert']
selected.append(next(c['id'] for c in catalog if c['id'].startswith('flags_') and ['flag','0','-dmo','bad'] in c['actions']))
rows=[]
for name in selected:
    case=cases[name]
    engine='supplemental' if name.startswith('supp_') else 'ordinary'
    exe=Path(profiles[engine]['executable'])
    actual,receipt=run.execute(case,exe,phase[engine]['profile'],OUT/'raw')
    run.validate(case,actual)
    run.compare(by[name],actual,name+'.central_fresh')
    result_path=OUT/'results'/(name+'.json.gz')
    run.write_gzip(result_path,actual)
    rows.append(dict(id=name,events=len(actual['events']),status=actual['status'],result_sha256=sha(result_path),receipt=receipt))
    print('PASS fresh',name,flush=True)
assert len({r['receipt']['nonce'] for r in rows})==len(rows)
assert {p.name:sha(p) for p in [*HERE.glob('*.py'), HERE/'probe.cc', HERE/'CONTRACT.md']}==source_before
for name,digest in verified['oracles'].items():assert sha(HERE/'outputs'/name)==digest
for repo,pin in [('tempo2',inventory.PIN),('validation/reference','7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572')]:
    assert subprocess.check_output(['git','-C',str(ROOT/repo),'rev-parse','HEAD'],text=True).strip()==pin
    assert not subprocess.check_output(['git','-C',str(ROOT/repo),'status','--short'],text=True)
receipt=dict(status='PASS',scope='Source/evidence identity audit and selected fresh ordinary execution, not a full native replay',
 reviewer_sha256=sha(__file__),verification_sha256=sha(HERE/'outputs/verification.json'),source_hashes=source_before,
 source_map_sha256=sha(HERE/'outputs/frozen/source_map.json'),oracles=verified['oracles'],phase_receipts=verified['receipts'],
 profiles=profiles,recorded_cases=324,recorded_observer_projections=324,recorded_compiled_mutations=mutant_count,
 recorded_supporting_checks=167,coverage=coverage['summary'],outcomes=dict(outcomes),fresh_cases=rows,
 reviewed_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
 preservation='Original delivery checked 8306 accepted files; central preserves AB files and does not repeat the whole accepted-file hash campaign.')
(OUT/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print('PASS central propagation review;',len(rows),'selected fresh cases; receipt',OUT/'receipt.json',flush=True)
