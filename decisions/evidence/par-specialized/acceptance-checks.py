"""Read-only identity, source membership, and coverage checks for Q delivery bb7e629."""
import gzip,hashlib,json,re,subprocess,sys
from pathlib import Path
root=Path('/Users/Eric.Dong/Downloads/tempo2rust');here=root/'validation/par_specialized'
sys.path.insert(0,str(here))
import profile as pm
import run,inventory,protected
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
load=lambda n:json.loads((here/n).read_text())
delivery=load('delivery.json')
for name,h in delivery['artifacts'].items():assert sha(here/name)==h,name
for n,h in pm.runners().items():
 assert hashlib.sha256(pm.project_blob('validation/par_specialized/'+n,'b61896f')).hexdigest()==h,n
profiles={'ordinary':load('profile.json'),'replay':load('regeneration.json')['executed_profile'],'verifier':load('verification.json')['profile'],'coverage':load('coverage.json')['profile']}
identities={}
for name,p in profiles.items():identities[name]=run.execution_identity(Path(p['executable_path']),p,True)
verify=load('verification.json');mutants=[]
for m in verify['compiled_mutations']:
 path=here/m['artifact'];assert sha(path)==m['sha256']
 data=json.loads(gzip.decompress(path.read_bytes()));p=data['profile']
 run.execution_identity(Path(p['executable_path']),p,True)
 assert p['executable_sha256']==m['executed_binary']
 mutants.append(m['name'])
run.compare(load('protected.json'),protected.snapshot(),'protected')
inv=load('inventory.json');run.compare(inv,inventory.inventory(),'pinned inventory')
cv=load('coverage.json');ledger=load('branch_coverage.json');entry=load('entry_coverage.json')
assert len(cv['observations'])==107 and all(x['comparison']=='exact' for x in cv['observations'])
case_ids=[c['id'] for c in load('cases.json')]
assert [x['case'] for x in cv['observations']]==case_ids
assert ledger['case_membership']==case_ids
assert [x['id'] for x in ledger['selected']]==[x['id'] for x in inv['selected']]==[x['id'] for x in entry['branches']]
selected_lines={};selected_gaps=[]
source=pm.blob('readParfile.C').decode().splitlines()
for spec,record,first in zip(inv['selected'],ledger['selected'],entry['branches']):
 lines={n:x for n,x in cv['files']['readParfile.C']['lines'].items() if spec['line']<=int(n)<spec['end_line']}
 assert record['lines_hit']==sum(x['count']>0 for x in lines.values()) and record['lines_total']==len(lines)
 assert record['unhit_lines']==[int(n) for n,x in lines.items() if not x['count']]
 selected_lines.update(lines);selected_gaps.extend(record['unhit_lines'])
 assert spec['line']<=first['first_scan_line']<spec['end_line']
 assert re.search(r'\b(?:fscanf|sscanf|fgets|readValue)\s*\(',source[first['first_scan_line']-1]),first
 measured=cv['files']['readParfile.C']['lines'][str(first['first_scan_line'])]
 assert first['count']==measured['count']>0 and first['cases']==measured['cases']
 for d in record['nested']:
  if str(d['line']) in cv['files']['readParfile.C']['lines']:assert d['measured']==cv['files']['readParfile.C']['lines'][str(d['line'])]
branches=[b for l in selected_lines.values() for b in l['branches']]
counts=dict(entries=len(inv['selected']),nested=sum(len(x['nested']) for x in inv['selected']),fields=len(inv['projected_fields']),helpers=len(inv['helper_names']),lines_hit=sum(x['count']>0 for x in selected_lines.values()),lines_total=len(selected_lines),branches_hit=sum(x['count']>0 for x in branches),branches_total=len(branches),unhit_lines=sorted(set(selected_gaps)))
assert counts==dict(entries=117,nested=207,fields=308,helpers=10,lines_hit=623,lines_total=633,branches_hit=812,branches_total=1184,unhit_lines=[1182,1183,1186,1187,1202,1203,1219,1220,1236,1237]),counts
cp=cv['profile'];commands=cp['compile_commands']
for command in commands[:-1]:
 assert '-c' in command and '--coverage' in command
 unit=Path(command[command.index('-c')+1])
 assert ('-O2' if unit.name=='probe.cc' else '-O0') in command
assert len(commands)==len(pm.UNITS)+2
p=load('protected.json')
receipt=dict(status='PASS',delivery='bb7e629',source_commit='b61896f',artifacts=len(delivery['artifacts']),runner_sources=len(pm.runners()),profiles=identities,mutants=mutants,coverage=counts,coverage_replayed_centrally=False,protected_project=len(p['project']),protected_nested={n:len(v['files']) for n,v in p['nested'].items()},protected_commits={n:v['commit'] for n,v in p['nested'].items()})
Path('/tmp/tempo2-par-specialized-acceptance-checks.json').write_text(json.dumps(receipt,indent=2,sort_keys=True)+'\n')
print(json.dumps({k:v for k,v in receipt.items() if k not in ['profiles','mutants']},indent=2))
