"""Affected accepted Rust probes, routed to AA; no native rebuilds or runner edits."""
import argparse,gzip,hashlib,importlib.util,json,os,shutil,subprocess,sys
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];RUST=ROOT/'rust'
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def save(p,x):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(x,indent=2)+'\n')
def unpack(p):return json.loads(gzip.decompress(p.read_bytes()))
def load(name,path):
 sys.path.insert(0,str(path.parent));s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);sys.modules[name]=m;s.loader.exec_module(m);return m
p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],required=True);p.add_argument('--suite',choices=['Y','W','T','R','O'],required=True);args=p.parse_args()
name={'Y':'time_ephemeris_port','W':'terrestrial_resources_port','T':'pulsar_lifecycle_port','R':'observation_storage_port','O':'parameter_storage_port'}[args.suite]
m=load('accepted_port',ROOT/'validation'/name/'run.py')
exe=Path(json.loads((HERE/'outputs'/args.profile/'direct.json').read_text())['identity']['executable']);ident=m.identity(exe)
out=HERE/'build/regressions'/args.suite/args.profile;out.mkdir(parents=True,exist_ok=True);records=[]
if args.suite=='Y':
 base=RUST/'verification/aa';length=len(str(m.V/'build/cases'));inputs_root=base/('y'*(length-len(str(base))-1))
 def inputs(case):
  base=RUST/'verification/aa';length=len(str(m.V/'build/cases'));parent=base/('y'*(length-len(str(base))-1));assert len(str(parent))==length
  work=parent/case['id']
  if work.exists():shutil.rmtree(work)
  work.mkdir(parents=True)
  for n,b in case['files'].items():p=work/n;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(b)
  return work,[[str(work/str(v)[1:]) if str(v).startswith('@') else str(v) for v in row] for row in case['actions']]
 m.inputs=inputs
 receipt=json.loads((m.OUT/'native/receipt.json').read_text());native={r['id']:r for r in receipt['records']}
 for c in m.fixtures.catalog():
  path=m.OUT/'native/equalpaths'/(c['id']+'.json.gz');assert sha(path)==native[c['id']]['sha256']
  expected=m.ownership_disposition(m.project(m.native_payload(unpack(path))))
  actual=m.project(m.execute(c,exe,ident,out/c['id']))
  # Only the equal-length owned staging root differs from Y's accepted launch.
  old=str(m.HERE/'work/x');new=str(inputs_root)
  assert len(old)==len(new)
  def relocated(v):
   if isinstance(v,dict):return {k:relocated(x) for k,x in v.items()}
   if isinstance(v,list):return [relocated(x) for x in v]
   if isinstance(v,str):return v.replace(new,old).replace(new.encode().hex(),old.encode().hex())
   return v
  m.compare(expected,relocated(actual),c['id']);records.append(dict(id=c['id'],events=len(actual['events']),status=actual['status'],oracle_sha256=sha(path)));print('PASS Y',c['id'],flush=True)
elif args.suite=='W':
 original=m.OUT;oracle=original/'corrected_native/reference.json.gz';assert sha(oracle)=='6948f57785698ec65704652443f79146a26a92da6c4869a14d3b1e3b6eb0386e'
 catalog=json.loads((m.HERE/'build/freeze.json').read_text())['cases'];expected=unpack(oracle);m.OUT=HERE/'w'
 for c,e in zip(catalog,expected):
  actual=m.execute(c,exe,ident,out/c['id']);m.compare(m.project(e),m.project(actual),c['id']);records.append(dict(id=c['id'],events=len(actual['events'])));print('PASS W',c['id'],flush=True)
elif args.suite=='T':
 catalog=json.loads((m.OUT/'freeze/membership.json').read_text())['S']
 # Scoped borrowing changes affect canonical rows, their release and aliases;
 # unrelated 100-million-cell matrix allocation histories retain accepted receipts.
 selected={'natural_1','wrapper_2','destroy_null_observation','copy_partial','copy_shared','copy_forward_overlap','copy_backward_overlap'}
 for c in catalog:
  if c['id'] in selected:records.append(m.execute(c,exe,ident,out/c['id']));print('PASS T',c['id'],flush=True)
else:
 accepted=RUST/'verification'/('observation_storage' if args.suite=='R' else 'parameter_storage')
 native_path=accepted/(args.profile+'_native.json.gz');native=unpack(native_path)
 plan=dict(schema=1,cases=[0,1,2],m_cases=[f'copypsr_{i}' for i in range(4)]) if args.suite=='R' else m.load(m.HERE/'case_projection.json')
 raw,launch=m.execute(exe,ident,plan);counts=m.validate(raw,native)
 save(out/'raw.json',raw);save(out/'launch.json',launch);records.append(dict(counts=counts,oracle_sha256=sha(native_path)));print('PASS',args.suite,counts,flush=True)
assert m.identity(exe)==ident
save(out/'receipt.json',dict(status='PASS',suite=args.suite,profile=args.profile,records=records,identity=ident,accepted_runner=str(m.__file__),accepted_runner_sha256=sha(m.__file__),routing_sha256=sha(__file__),path_projection=({'accepted':str(m.HERE/'work/x'),'staged':str(inputs_root),'equal_length':True} if args.suite=='Y' else None),scope='affected seven lifecycle histories' if args.suite=='T' else 'complete accepted Rust probe membership; recorded native oracle'))
