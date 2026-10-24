"""Pinned raw source/typed/caller inventory and read-only preservation receipts."""
import hashlib
import json
import re
import subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parent.parent
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
RELEASE='b8d6263'
PRIMARY={
 'calculate_bclt.C':['calculate_bclt'], 'shapiro_delay.C':['shapiro_delay'],
 'dm_delays.C':['dm_delays'], 'formBats.C':['formBats'], 'secularMotion.C':['secularMotion'],
 'refphs.C':['refphs_init','refphs_clean'],
 'sw_delay.C':['solarWindModel','findAngle','readCurrentSheet','outputResults','mcl2','amod','elsun2','calcRotN','mjd2date','convertEcliptic'],
 'ifunc.C':['ifunc','sinfunc'], 'shapelet.C':['iter_factorial','TNothpl','iterative_TNothpl','evaluateShapelet'],
 'storePrecision.C':['recordPrecision']}
def sha(b):return hashlib.sha256(b).hexdigest()
def blob(n):return subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':'+n])
def save(p,x):
 p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(x,indent=2,allow_nan=False)+'\n')
def uncomment(s):
 return re.sub(r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|//[^\n]*|/\*[\s\S]*?\*/',lambda m:re.sub(r'[^\n]',' ',m[0]) if m[0].startswith('/') else m[0],s)
def source_map():
 rows=[]
 for file,names in PRIMARY.items():
  raw=blob(file).decode();s=uncomment(raw)
  for name in names:
   for match in re.finditer(r'\b'+name+r'\s*\([^;{}]*\)\s*\{',s):
    end=match.end();depth=1
    while depth:depth+=(s[end]=='{')-(s[end]=='}');end+=1
    lo=s[:match.start()].count('\n')+1;hi=s[:end].count('\n')+1
    rows.append(dict(file=file,name=name,start=lo,end=hi,signature=s[match.start():match.end()-1].strip(),
     source=raw.splitlines()[lo-1:hi],decisions=[dict(line=i+1,text=l.strip()) for i,l in enumerate(s.splitlines()) if lo<=i+1<=hi and re.search(r'\b(if|else|for|while)\b|\?|^\s*#',l)],
     fields=sorted(set(re.findall(r'(?:\.|->)(\w+)',s[match.start():end])))))
 assert len(rows)==25,len(rows)
 names=sorted(set(n for ns in PRIMARY.values() for n in ns))
 callers=[]
 for p in subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'ls-tree','-r','--name-only',PIN],text=True).splitlines():
  if not p.endswith(('.C','.c','.h','.cpp')):continue
  for i,l in enumerate(uncomment(blob(p).decode(errors='replace')).splitlines()):
   hits=[n for n in names if re.search(r'\b'+n+r'\s*\(',l)]
   if hits:callers.append(dict(file=p,line=i+1,routines=hits,text=l.strip()))
 contract=subprocess.check_output(['git','show',RELEASE+':decisions/0063-propagation-dispersion-reference-stage.md'],cwd=ROOT,text=True)
 for file in PRIMARY:
  assert sha(blob(file)) in contract,(file,'release hash mismatch')
 return dict(pin=PIN,release=RELEASE,routines=rows,callers=callers,
  preprocessor={f:[dict(line=i+1,text=l) for i,l in enumerate(blob(f).decode().splitlines()) if l.lstrip().startswith('#')] for f in PRIMARY},
  hashes={f:sha(blob(f)) for f in [*PRIMARY,'tempo2.h','ifunc.h','shapelet.h','t2fit_solarwind.h']},
  types=json.loads((ROOT/'validation/pulsar_lifecycle/inventory.json').read_text())['types'],
  rust_context={f:sha(subprocess.check_output(['git','show','bd290cc:'+f],cwd=ROOT)) for f in ['rust/src/form_bats.rs','rust/src/tropo.rs']})
def freeze():
 out=HERE/'outputs/frozen';assert not out.exists(),'preserve existing freeze'
 save(out/'source_map.json',source_map())
 # Explicit accepted source and local evidence membership; active central/Y/Z/AA excluded.
 accepted=['reference','pulsar_lifecycle','numeric','terrestrial_resources','observer_coordinates']
 protected={}
 for d in accepted:
  path=ROOT/'validation'/d
  if not path.exists():continue
  for p in path.rglob('*'):
   if not p.is_file() or any(x in ('build','target','__pycache__','.git') for x in p.relative_to(path).parts):continue
   h=hashlib.sha256()
   with p.open('rb') as f:
    for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
   protected[str(p.relative_to(ROOT))]=h.hexdigest()
 save(out/'protected.json',protected)
 save(out/'repositories.json',{n:dict(head=subprocess.check_output(['git','-C',str(ROOT/n),'rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','-C',str(ROOT/n),'status','--porcelain'],text=True)) for n in ['tempo2','validation/reference']})
 print('Frozen 25 routines and',len(protected),'accepted artifacts')
def check():
 base=HERE/'outputs/frozen'
 for f,h in json.loads((base/'source_map.json').read_text())['hashes'].items():assert sha(blob(f))==h,f
 for p,h in json.loads((base/'protected.json').read_text()).items():
  hasher=hashlib.sha256()
  with (ROOT/p).open('rb') as f:
   for b in iter(lambda:f.read(1024*1024),b''):hasher.update(b)
  assert hasher.hexdigest()==h,'protected drift: '+p
 for n,v in json.loads((base/'repositories.json').read_text()).items():
  assert subprocess.check_output(['git','-C',str(ROOT/n),'rev-parse','HEAD'],text=True).strip()==v['head']
  assert subprocess.check_output(['git','-C',str(ROOT/n),'status','--porcelain'],text=True)==v['status']==''
if __name__=='__main__':freeze()
