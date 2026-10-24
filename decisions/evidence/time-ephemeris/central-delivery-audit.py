"""Audit V's actual delivered identities, complete coverage observations and O0/O2 differences."""
from pathlib import Path
from collections import Counter
import gzip,json,math,struct,subprocess,sys
sys.dont_write_bytecode=True
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust');HERE=ROOT/'validation/time_ephemeris';O=HERE/'outputs'
OUT=Path(Path('/tmp/tempo2-v-central-review.log').read_text().splitlines()[0][4:])
sys.path.insert(0,str(HERE));import profile,run,verify,coverage,preservation
sha=lambda p:profile.sha(p.read_bytes())
load=lambda p:json.loads(p.read_text())
ordinary=load(O/'last_execution.json');oracle=load(run.REF/'oracle.json');cov=load(O/'coverage.json');mutated=load(O/'verification-mutations-final.json')
cases=run.catalog();ids=[c['id'] for c in cases];assert len(ids)==len(set(n.casefold() for n in ids))==97
run.compare(oracle['inputs'],dict(cases=run.membership(cases),inventory=run.inventory()))
assert Counter(c['classification']=='defined' for c in cases)=={True:75,False:22}
assert ordinary['complete'] and ordinary['mode']=='verify' and ordinary['oracle_sha256']==sha(run.REF/'oracle.json')
assert [c['id'] for c in ordinary['cases']]==ids
assert len({str(Path(c['path'])).casefold() for c in ordinary['cases']})==97
profiles=[]
def check_profile(p):
 profile.check_live(Path(p['executable_path']),p)
 src=Path(p['command'][4][2:]);mutation=p['mutation']
 for n in profile.FILES:
  expected=profile.blob(n)
  if mutation and n==mutation[0]:
   old,new=mutation[1:];s=expected.decode();assert s.count(old)==1;expected=s.replace(old,new).encode()
  assert (src/n).read_bytes()==expected,(src,n)
 profiles.append(p['executable_sha256'])
check_profile(ordinary['profile']);run.compare(oracle['profile'],ordinary['profile']['stable'])
check_profile(mutated['profile']);assert len(mutated['controls'])==30 and len(mutated['independent_polynomials'])==36
run.compare(verify.independent(),mutated['independent_polynomials'])
for case,r in zip(cases,ordinary['cases']):
 p=Path(r['path']);assert sha(p)==r['sha256'];raw=run.unpack(p)
 assert raw['id']==case['id'] and raw['status']==case['status']==r['status']
 assert len(raw['events'].splitlines())==r['events']
 assert profile.sha(raw['input_text'].encode())==raw['input_sha256']
 assert raw['resource_sha256']=={n:profile.sha(b) for n,b in case['files'].items()}
 run.compare(run.projected(run.unpack(run.REF/(case['id']+'.json.gz'))),run.projected(raw))
assert sum(r['events'] for r in ordinary['cases'])==1368 and sum(r['status']==1 for r in ordinary['cases'])==9
mutation_witnesses=[]
assert len(mutated['mutations'])==len(verify.MUTATIONS)==13
for spec,m in zip(verify.MUTATIONS,mutated['mutations']):
 name,file,old,new,case=spec;assert (m['name'],m['case'])==(name,case)
 assert m['profile']['mutation']==[file,old,new];check_profile(m['profile'])
 p=Path(m['path']);assert sha(p)==m['sha256']
 expected=run.projected(run.unpack(run.REF/(case+'.json.gz')));actual=run.projected(run.unpack(p))
 assert expected['events'][0]==actual['events'][0]
 try:run.compare(expected,actual)
 except AssertionError as error:witness=str(error)
 else:raise AssertionError('no actual mutation effect '+name)
 assert witness==m['witness'] and ('.state.' in witness or '.result' in witness),(name,witness)
 mutation_witnesses.append(dict(name=name,case=case,witness=witness))
# Validate every retained O0 observation and its exact relation to the O2 oracle.
check_profile(cov['profile']);assert cov['profile']['stable']['coverage']
assert [r['case'] for r in cov['cases']]==ids and [r['case'] for r in cov['differences']]==ids
numeric_classes=Counter();binary64_deltas=[];numeric_leaves=0
# Parent types prevent a generic .hex suffix from concealing a byte/string difference.
def numeric_only(a,b,path=(),pa=None,pb=None):
 assert type(a)==type(b),('coverage type',path)
 if isinstance(a,dict):
  assert list(a)==list(b),('coverage membership/order',path)
  return sum(numeric_only(a[k],b[k],path+(k,),a,b) for k in a)
 if isinstance(a,list):
  assert len(a)==len(b),('coverage length',path)
  return sum(numeric_only(x,y,path+(i,),a,b) for i,(x,y) in enumerate(zip(a,b)))
 if a==b:return 0
 assert path[-1] in ['bits','hex','value'] and isinstance(pa,dict) and isinstance(pb,dict),path
 assert pa.get('type')==pb.get('type') and pa['type'] in ['binary32','binary64','binary128'],path
 numeric_classes[(pa['type'],pa.get('class','binary128 decimal/bytes'))]+=1
 if pa['type']=='binary64' and path[-1]=='bits':
  x,y=[struct.unpack('<d',bytes.fromhex(v['bits']))[0] for v in [pa,pb]]
  assert math.isfinite(x) and math.isfinite(y),(path,x,y)
  binary64_deltas.append(dict(path=repr(path),absolute_difference=abs(x-y),ulps=abs(x-y)/max(math.ulp(x),math.ulp(y))))
 return 1
calls=Counter()
for r,d in zip(cov['cases'],cov['differences']):
 p=Path(r['path']);assert sha(p)==r['sha256'];actual=run.projected(run.unpack(p))
 expected=run.projected(run.unpack(run.REF/(r['case']+'.json.gz')))
 dif=coverage.differences(expected,actual);run.compare(d['differences'],dif)
 assert len(dif)==r['differences'];numeric_leaves+=numeric_only(expected,actual)
 calls.update(r['calls'])
assert numeric_leaves==576 and sum(bool(d['differences']) for d in cov['differences'])==30
raw_paths=list((HERE/'build/gcov').glob('*.gcov.json.gz'));assert len(raw_paths)==1
raw=run.unpack(raw_paths[0]);selected=[f for f in raw['files'] if Path(f['file']).name in coverage.ROUTINES]
run.compare(cov['raw_selected'],selected)
summary=[];gaps=[]
for f in selected:
 name=Path(f['file']).name
 for routine,a,b in coverage.ROUTINES[name]:
  fns=[v for v in f['functions'] if a<=v['start_line']<=b];assert len(fns)==1
  fn=fns[0];assert fn['execution_count']>0 and calls[name+':'+fn['name']]==fn['execution_count']
  lines=[v for v in f['lines'] if a<=v['line_number']<=b];branches=[br for v in lines for br in v.get('branches',[])]
  summary.append(dict(file=name,routine=routine,calls=fn['execution_count'],lines=len(lines),hit_lines=sum(v['count']>0 for v in lines),edges=len(branches),hit_edges=sum(v['count']>0 for v in branches)))
  for v in lines:
   if v['count']==0:
    cls,why=coverage.disposition(name,v['line_number'],{});gaps.append(dict(file=name,routine=routine,line=v['line_number'],kind='line',classification=cls,reason=why))
   for j,branch in enumerate(v.get('branches',[])):
    if branch['count']==0:
     cls,why=coverage.disposition(name,v['line_number'],branch);gaps.append(dict(file=name,routine=routine,line=v['line_number'],kind='edge',index=j,branch=branch,classification=cls,reason=why))
run.compare(cov['summary'],summary);run.compare(cov['gaps'],gaps)
assert len(summary)==29 and len(gaps)==75
assert [sum(r[k] for r in summary) for k in ['hit_lines','lines','hit_edges','edges']]==[327,330,196,268]
manifest=load(preservation.MANIFEST);assert preservation.check(manifest)==[]
last=load(O/'preservation-final.json')
assert last['passed'] and last['members']==len(manifest['files'])==1466
assert last['manifest_sha256']==sha(preservation.MANIFEST) and last['execution_receipt_sha256']==sha(O/'last_execution.json')
assert last['transitions_before']==last['transitions_after']==[]
assert len(last['controls'])==3 and all(c['rejected'] for c in last['controls'])
# Check release-anchored tracked bytes directly from Git, separately from live hashes.
tree={}
for row in subprocess.check_output(['git','ls-tree','-r','-z',manifest['release']],cwd=ROOT).split(b'\0'):
 if row:
  info,name=row.split(b'\t',1);mode,kind,oid=info.split()
  if kind==b'blob':tree[name.decode()]=oid.decode()
tracked=[(n,tree[n]) for n in manifest['files'] if n in tree];assert len(tracked)==manifest['release_tracked_members']==633
content=subprocess.run(['git','cat-file','--batch'],cwd=ROOT,input=('\n'.join(h for n,h in tracked)+'\n').encode(),capture_output=True,check=True).stdout
pos=0
for name,oid in tracked:
 end=content.index(b'\n',pos);head=content[pos:end].split();assert head[:2]==[oid.encode(),b'blob']
 size=int(head[2]);body=content[end+1:end+1+size];assert profile.sha(body)==manifest['files'][name],name
 pos=end+1+size+1
assert pos==len(content)
result=dict(status='PASS',scope='Read-only audit of actual V profiles, all ordinary and O0 recordings, independent comparisons, thirteen behavioral mutants, coverage counters/gaps and release-anchored exact preservation. O0 instrumentation was not rerun centrally.',output=str(OUT),source_revision='573d667',delivery_revision='f179ce5',ordinary_receipt_sha256=sha(O/'last_execution.json'),oracle_sha256=sha(run.REF/'oracle.json'),report_sha256=sha(HERE/'REPORT.md'),coverage_sha256=sha(O/'coverage.json'),raw_gcov_sha256=sha(raw_paths[0]),coverage_routines=len(summary),coverage_totals=dict(lines=330,hit_lines=327,edges=268,hit_edges=196,gaps=75),ordinary_O0_numeric_differing_cases=30,numeric_differing_leaves=numeric_leaves,numeric_classes={repr(k):v for k,v in numeric_classes.items()},maximum_binary64_difference=max(binary64_deltas,key=lambda x:x['absolute_difference']),maximum_binary64_ulps=max(binary64_deltas,key=lambda x:x['ulps']),mutation_witnesses=mutation_witnesses,profiles_checked=len(profiles),protected_files=1466,release_tracked_files=633)
profile.save(OUT/'central-delivery-audit.json',result)
print('PASS V delivered profile/case/coverage/mutation/preservation audit',flush=True)
print('NUMERIC',result['numeric_classes'],result['maximum_binary64_difference'],result['maximum_binary64_ulps'],flush=True)
print('RECEIPT',OUT/'central-delivery-audit.json',sha(OUT/'central-delivery-audit.json'),flush=True)
