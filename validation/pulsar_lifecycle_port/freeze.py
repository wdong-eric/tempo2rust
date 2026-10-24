"""Freeze T inputs before production edits. Never overwrites a prior freeze."""
import hashlib,json,re,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'rust/verification/pulsar_lifecycle/freeze'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def save(name,value):
 p=OUT/name
 assert not p.exists(),p
 p.write_text(json.dumps(value,indent=2)+'\n')
def freeze():
 OUT.mkdir(parents=True,exist_ok=True)
 inv=json.loads((ROOT/'validation/pulsar_lifecycle/inventory.json').read_text())
 release=json.loads((ROOT/'decisions/evidence/pulsar-lifecycle/released-fields.json').read_text())
 fields=[dict(name=f['name'],source_type=f['type'],pointer_depth=f['pointers'],dimensions=f['extents']) for f in inv['types']['pulsar']]
 assert fields==release['fields'] and len(fields)==349
 hashes=inv['source_sha256']|{'t2fit_nestlike.h':'50db4fb3e38bb16eab83dbd2ce4331ee9fbbbbb314e60b17fbb003997996a9af'}
 for name,h in hashes.items():
  raw=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',inv['pin']+':'+name])
  assert hashlib.sha256(raw).hexdigest()==h,name
  # Native checkout has Git ident expansion; raw Git blobs are the oracle.
  assert subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'diff','--',name])==b''
 for kind,fs in inv['types'].items():
  for f in fs:
   f['rust_counterpart']=('SourcePulsar' if kind=='pulsar' else kind)+'.'+f['name']
   f['knownness']='per scalar/cell/byte Option; pointers distinguish unwritten, known null and owner'
   f['ownership']=('embedded O store; retired original buffers returned on replacement' if f['name']=='param' and kind=='pulsar' else 'context matrix owner (COW for independent Clone)' if f['pointers']==2 else 'context observation owner + record offset' if f['type']=='observation' and f['pointers'] else 'context opaque byte owner + offset' if f['pointers'] else 'typed callback identity/callable, no native ABI' if f['type'].endswith('Func') else 'embedded fixed extent; heap placement allowed')
 save('field-and-operation-map.json',inv)
 callers={}
 pattern=re.compile(r'TimingInput|parameter_store|fit_controls|text_parameters|observations|ClockPulsar|initialise_parameters|allocate_memory')
 for p in sorted((ROOT/'rust').rglob('*.rs')):
  if 'target' in p.parts:continue
  rows=[dict(line=i,text=l) for i,l in enumerate(p.read_text().splitlines(),1) if pattern.search(l)]
  if rows:callers[str(p.relative_to(ROOT))]=dict(sha256=sha(p),sites=rows)
 save('caller-map.json',dict(sites=callers,bridge={'MAX_PSR':1,'MAX_OBSN':'supplied records','fullSetup':1,'noWarnings':1,'parameter':'O canonical source store','observations':'R full source records; bridge writes unchanged','controls':'source fitMode/nits; detached snapshot with provenance','text':'source name/units/clock/ephemeris; original tokens provenance','clock':'borrowed source fields plus nobs=1/noWarnings=1/TT(TAI)/no SAT jumps'}))
 protected={}
 for directory in ['validation','rust/verification','rust/tests/reference','rust/tests/terrestrial','tempo2','decisions/evidence']:
  for p in (ROOT/directory).rglob('*'):
   rel=p.relative_to(ROOT)
   if not p.is_file() or any(x in rel.parts for x in ['.git','target','__pycache__','build']):continue
   if str(rel).startswith(('validation/pulsar_lifecycle_port/','rust/verification/pulsar_lifecycle/')):continue
   if str(rel) in ['validation/observation_storage_port/'+n+'.py' for n in ['run','native','verify']]:continue
   protected[str(rel)]=sha(p)
 save('protected.json',protected)
 save('baseline.json',dict(head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),raw=hashes,rust={str(p.relative_to(ROOT)):sha(p) for p in sorted((ROOT/'rust/src').glob('*.rs'))},nested={n:dict(head=subprocess.check_output(['git','-C',str(ROOT/n),'rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','-C',str(ROOT/n),'status','--porcelain'],text=True)) for n in ['tempo2','validation/reference']}))
 s=json.loads((ROOT/'validation/pulsar_lifecycle/cases.json').read_text())
 q=json.loads((ROOT/'validation/par_specialized/cases.json').read_text())
 save('membership.json',dict(S=s,Q=q,projection={'S':'all 30 cases, all ordered actions and state records; native sizeof/addresses/logger presentation separately qualified; matrix full rows and cells','Q':'initial TNsubtractPoly and all ten packed helper observations only; PAR behavior excluded','floats':'exact binary128 finite decoding into Rug128; signed zero; existing nonfinite classes','pointer':'symbolic owner identity and offset; native physical addresses excluded'}))
 print('frozen',len(fields),'pulsar fields;',len(protected),'protected files;',len(s),'S cases;',len(q),'Q cases')
if __name__=='__main__':freeze()
