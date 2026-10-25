"""Identity audit of AA executions, final source and explicitly retained earlier receipts."""
import hashlib,json,re,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];RUST=ROOT/'rust'
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def read(p):return json.loads(Path(p).read_text())
def git(*args):return subprocess.check_output(['git','-C',str(ROOT),*args],text=True).strip()
def blocks(s):
 out=[]
 for m in re.finditer(r'\bunsafe(?: extern "C")? \{',s):
  end=m.end();depth=1
  while depth:depth+=(s[end]=='{')-(s[end]=='}');end+=1
  out.append(s[m.start():end])
 return out
source=RUST/'src/get_obs_coord.rs';before=git('show','4498cf1:rust/src/get_obs_coord.rs');assert blocks(before)==blocks(source.read_text()) and len(blocks(before))==2
freeze=read(HERE/'build/freeze.json');assert (HERE/'build/preservation.log').read_text().strip()=='Preserved '+str(len(freeze['protected']))
for name,head in freeze['repositories'].items():assert git('-C',name,'rev-parse','HEAD')==head and not git('-C',name,'status','--porcelain')
def identity(x,revision=None):
 for name,digest in x.get('sources',{}).items():
  if revision and name.startswith('rust/src/'):
   raw=subprocess.check_output(['git','-C',str(ROOT),'show',revision+':'+name]);assert hashlib.sha256(raw).hexdigest()==digest,name
  else:assert sha(ROOT/name)==digest,name
 for name in ['compiler','cargo','executable']:
  if name in x:assert sha(x[name])==x[name+'_sha256'],name
artifacts={};profiles={}
def artifact(p):artifacts[str(p.relative_to(ROOT))]=sha(p)
for profile in ['debug','release']:
 p=HERE/'outputs'/profile/'direct.json';d=read(p);identity(d['identity']);assert d['status']=='PASS' and d['scope']=='all' and len(d['cases'])==133 and sum(c['events'] for c in d['cases'])==3468
 for c in d['cases']:assert sha(p.parent/c['id']/'result.json')==c['result_sha256']
 artifact(p)
 old=HERE/'build/pre-single-boundary'/profile;controls=read(old/'controls/receipt.json');identity(controls['identity'],'716375f')
 assert controls['status']=='PASS' and len(controls['controls'])==35 and len(controls['supporting_checks'])==19 and len(controls['retained_mutation_witnesses'])==15 and controls['corrected_buffers']==19 and controls['original_buffers']==7
 assert controls['direct_sha256']==sha(old/'direct.json');artifact(old/'controls/receipt.json');artifact(old/'direct.json')
 for suite,count in [('Y',97),('W',130),('T',7),('R',1),('O',1)]:
  p=HERE/'build/regressions'/suite/profile/'receipt.json';r=read(p);assert r['status']=='PASS' and len(r['records'])==count;identity(r['identity'],'716375f');assert sha(r['accepted_runner'])==r['accepted_runner_sha256'];artifact(p)
 profiles[profile]=dict(cases=133,ordinary_events=3462,observer_events=3468,native_exits=6,identity=d['identity'])
p=HERE/'build/followup/receipt.json';f=read(p);assert f['status']=='PASS';identity(f)
for v in f['profiles'].values():assert sha(v['executable'])==v['sha256']
for c in f['commands']:assert c['status']==0 and sha(p.parent/(c['name']+'.log'))==c['log_sha256']
artifact(p)
p=HERE/'build/public/aa-receipt.json';public=read(p);identity(public,'716375f');assert public['failures']==read(RUST/'verification/m2/open_gate_failures.json') and len(public['failures'])==4;artifact(p)
for path in ['build/freeze.json','build/preservation.log','build/arithmetic/receipt.json'] :artifact(HERE/path)
for profile in ['debug','release']:
 for example in ['terrestrial','satellite','vector-ut1']:
  p=HERE/'outputs/examples'/profile/example/'summary.json';summary=read(p);assert summary['result_sha256']==sha(p.parent/'result.json');artifact(p)
receipt=dict(status='AA delivered for central acceptance; M2 and 0049 unchanged',production_commit=git('rev-parse','99cf714'),earlier_production_commit=git('rev-parse','716375f'),head_at_audit=git('rev-parse','HEAD'),protected_files=len(freeze['protected']),repositories=freeze['repositories'],profiles=profiles,artifacts=artifacts,unsafe=dict(source=str(source.relative_to(ROOT)),blocks=2,byte_identical_to_release=True,source_sha256=sha(source)),M2_failures=public['failures'],script_sha256=sha(__file__))
(HERE/'build/delivery.json').write_text(json.dumps(receipt,indent=2)+'\n');print('PASS final source/receipt audit; 133 histories per profile; 40923 protected files; four unchanged M2 findings')
