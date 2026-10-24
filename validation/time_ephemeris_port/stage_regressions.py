"""Y-only routing of accepted W/T/R callers; retain comparisons and input membership."""
import difflib,json,shutil,subprocess,hashlib,re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent;OUT=HERE/'build/regressions';STAGE=OUT/'routing'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def save(p,x):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(x,indent=2)+'\n')
def blocks(text):
 out=[]
 for m in re.finditer(r'\bunsafe(?: extern "C")? \{',text):
  pos=m.end();depth=1
  while depth:depth+=(text[pos]=='{')-(text[pos]=='}');pos+=1
  out.append(text[m.start():pos])
 return out

def stage():
 STAGE.mkdir(parents=True,exist_ok=True);records={};audit={}
 for name in ['get_obs_coord.rs','clock_text.rs','stdio.rs']:
  original=subprocess.check_output(['git','show','5920aff:rust/src/'+name],cwd=ROOT,text=True);current=(ROOT/'rust/src'/name).read_text()
  old,new=blocks(original),blocks(current)
  if name=='stdio.rs':
   # Existing fdopen/fread/fseek/fclose calls are byte-identical. The extern
   # declaration is extended and two exclusively borrowed call blocks are new.
   assert all(b in new for b in old[1:]);assert 'unsafe impl Send for Stream {}' in current
   assert len(new)==len(old)+2
   kind='0058 authorized getc/status blocks and exclusive Send; existing call bodies exact'
  else:assert old==new;kind='unsafe blocks unchanged from Y release'
  audit[name]=dict(original_sha256=hashlib.sha256(original.encode()).hexdigest(),current_sha256=sha(ROOT/'rust/src'/name),old_blocks=len(old),new_blocks=len(new),disposition=kind)
 save(STAGE/'unsafe_audit.json',audit)
 for name,short in [('pulsar_lifecycle_port','T'),('observation_storage_port','R'),('terrestrial_resources_port','W')]:
  origin=ROOT/'validation'/name;target=STAGE/name;target.mkdir(exist_ok=True);sources={};diffs=[]
  for p in origin.iterdir():
   if not p.is_file():continue
   raw=p.read_bytes();routed=raw
   if p.suffix=='.py':
    s=raw.decode();edits={}
    if p.name=='run.py':
     if short=='T':edits={"ROOT=Path(__file__).resolve().parents[2]":f"ROOT=Path({str(ROOT)!r})","OUT=ROOT/'rust/verification/pulsar_lifecycle'":f"OUT=Path({str(OUT/'T')!r})"}
     if short=='R':
      assert sha(p)=='7d6424654a666d3852255bd9264ba0fbb88ba3c6e57f7bf717da4087a6b10529'
      edits={"ROOT=HERE.parents[1]":f"ROOT=Path({str(ROOT)!r})","OUT=RUST/'verification/observation_storage'":f"OUT=Path({str(OUT/'R')!r})"}
     if short=='W':edits={"OUT=ROOT/'rust/verification/terrestrial_resources'":f"OUT=Path({str(HERE/'w')!r})"}
    if short=='W' and p.name=='freeze.py':edits={"ROOT=Path(__file__).resolve().parents[2]":f"ROOT=Path({str(ROOT)!r})"}
    for before,after in edits.items():assert s.count(before)==1,(short,p.name,before);s=s.replace(before,after)
    if short=='R' and p.name=='verify.py':
     needle="baseline=port.load(port.HERE/'baseline.json')['sources']";assert s.count(needle)==1
     s=s.replace(needle,needle+"\n baseline['rust/src/get_obs_coord.rs']="+repr(audit['get_obs_coord.rs']['current_sha256'])+" # W safe adapter; audited unchanged unsafe bodies.")
     before="p!='rust/src/clock_text.rs'";assert s.count(before)==1
     s=s.replace(before,"p not in ('rust/src/clock_text.rs','rust/src/stdio.rs') # Y stdio additions audited separately below")
     # Comment must not consume comprehension closing brace.
     s=s.replace(" # Y stdio additions audited separately below}","}")
     needle="assert all(unchanged_unsafe.values())";s=s.replace(needle,needle+"\n assert port.sha(port.ROOT/'rust/src/stdio.rs')=="+repr(audit['stdio.rs']['current_sha256'])+" # Explicit Y transfer, not an unchanged-file claim.")
    routed=s.encode()
   (target/p.name).write_bytes(routed);sources[p.name]=dict(original=sha(p),staged=sha(target/p.name))
   if raw!=routed:diffs.append(''.join(difflib.unified_diff(raw.decode().splitlines(True),routed.decode().splitlines(True),fromfile=str(p),tofile=str(target/p.name))))
  (STAGE/(short+'.diff')).write_text(''.join(diffs));records[short]=dict(files=sources,diff_sha256=sha(STAGE/(short+'.diff')))
  if short=='T':
   freeze=OUT/'T/freeze';freeze.mkdir(parents=True,exist_ok=True)
   for p in (ROOT/'rust/verification/pulsar_lifecycle/freeze').iterdir():
    if p.is_file():shutil.copyfile(p,freeze/p.name)
  if short=='W':
   (target/'build').mkdir(exist_ok=True);shutil.copyfile(origin/'build/freeze.json',target/'build/freeze.json')
   for name in ['corrected_native/reference.json.gz','corrected_native/contract.json','fixed00/probe']:
    dst=HERE/'w'/name;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(ROOT/'rust/verification/terrestrial_resources'/name,dst)
 save(STAGE/'staging.json',records);return STAGE
if __name__=='__main__':print(stage())
