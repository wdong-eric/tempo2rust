"""Read-only source, execution, coverage and correction checks for S at 58ddc07."""
import gzip,hashlib,json,subprocess,sys
from pathlib import Path
root=Path('/Users/Eric.Dong/Downloads/tempo2rust');here=root/'validation/pulsar_lifecycle'
sys.path.insert(0,str(here))
import profile as pm
import run,inventory
from fixtures import cases
load=lambda n:json.loads((here/n).read_text())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
run.protected();run.compare(load('inventory.json'),inventory.build(),'current source inventory')
for n,h in pm.runners().items():
 if n.endswith(('.py','.cc')):
  assert hashlib.sha256(subprocess.check_output(['git','-C',str(root),'show','6cc0fd0:validation/pulsar_lifecycle/'+n])).hexdigest()==h,n
profiles={'ordinary':load('profile.json'),'replay':load('regeneration.json')['profile'],'coverage':load('coverage.json')['profile']}
identities={n:run.execution_identity(Path(p['executable_path']),p) for n,p in profiles.items()}
ver=load('verification.json');mutants=[]
for m in ver['mutants']:
 p=m['profile'];run.compare(m['execution'],run.execution_identity(Path(p['executable_path']),p),'mutant.'+m['name'])
 result=json.loads(gzip.decompress((here/'mutations'/(m['name']+'.json.gz')).read_bytes()))
 expected=run.reference(m['case']);a=[e['state'] for e in expected['events'][1:]];b=[e['state'] for e in result['events'][1:]]
 try:run.compare(a,b,'post-profile state')
 except AssertionError as error:mutants.append(dict(name=m['name'],case=m['case'],witness=str(error)))
 else:raise AssertionError('missing post-profile mutation witness: '+m['name'])
cv=load('coverage.json');inv=load('inventory.json');cs=cases()
assert cv['status']=='PASS' and cv['ordinary_exact_cases']==[c['id'] for c in cs]
assert [r['name'] for r in cv['functions']]==[r['name'] for r in inv['functions']]
for expected,row in zip(inv['functions'],cv['functions']):
 run.compare(expected,{k:v for k,v in row.items() if k!='measured'},'coverage source function')
 assert row['measured']['execution_count']>0
summary=dict(functions_entered=len(cv['functions']),lines=len(cv['lines']),lines_hit=sum(r['count']>0 for r in cv['lines']),branches=len(cv['branches']),branches_hit=sum(r['count']>0 for r in cv['branches']))
run.compare(summary,cv['summary'],'coverage totals')
assert summary==dict(functions_entered=12,lines=711,lines_hit=702,branches=256,branches_hit=236)
assert sha(Path('/opt/homebrew/bin/gcov-16'))==cv['gcov_sha256']
assert len(ver['controls'])==25 and len(ver['mutants'])==14 and ver['retained_link_records_checked']==24000 and ver['raw_validated_records']==5329662
corr=load('correction.json');assert corr['status']=='PASS' and corr['cases']==30 and corr['known_link_records']==24000 and corr['added_records']==4 and corr['all_other_projected_values_order_streams_status_unchanged']
protected=load('protected_hashes.json')
receipt=dict(status='PASS',delivery='58ddc07',source_commit='6cc0fd0',runner_sources=len(pm.runners()),profiles=identities,mutants=mutants,coverage=summary,coverage_replayed_centrally=False,coverage_calls={r['name']:r['measured']['execution_count'] for r in cv['functions']},protected_files=len(protected),corrected_link_records=24000,added_covariance_records=4,raw_records=5329662)
Path('/tmp/tempo2-pulsar-lifecycle-acceptance-checks.json').write_text(json.dumps(receipt,indent=2,sort_keys=True)+'\n')
print(json.dumps({k:v for k,v in receipt.items() if k not in ['profiles','mutants']},indent=2))
