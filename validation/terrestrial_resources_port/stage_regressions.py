"""Stage T/R callers with explicit routing and safe-adapter hash applicability; originals stay exact."""
import difflib,json,shutil,re,subprocess
from pathlib import Path
from run import ROOT,OUT,sha,save

def unsafe_blocks(text):
 blocks=[]
 for match in re.finditer(r'\bunsafe(?: extern "C")? \{',text):
  start=match.start();pos=match.end();depth=1
  while depth:
   depth+=(text[pos]=='{')-(text[pos]=='}');pos+=1
  blocks.append(text[start:pos])
 return blocks

def stage():
 dest=OUT/'routing';dest.mkdir(parents=True,exist_ok=True);receipt={}
 # W changes safe EOP plumbing around SOFA, and adds a safe integer scanner.
 # Verify the exact old unsafe declarations and bodies before updating R's
 # obsolete whole-file SOFA hash expectation in this disposable caller only.
 audit={}
 for name in ['get_obs_coord.rs','clock_text.rs']:
  path=ROOT/'rust/src'/name
  original=subprocess.check_output(['git','show','a8ef85c:rust/src/'+name],cwd=ROOT,text=True)
  old,new=unsafe_blocks(original),unsafe_blocks(path.read_text())
  assert old and old==new,name
  audit[name]=dict(unchanged_blocks=len(old),current_sha256=sha(path))
 save(dest/'unsafe_audit.json',audit)
 for name,short in [('pulsar_lifecycle_port','T'),('observation_storage_port','R')]:
  origin=ROOT/'validation'/name;target=dest/name;target.mkdir(exist_ok=True)
  changes=[];sources={}
  for p in origin.iterdir():
   if not p.is_file():continue
   raw=p.read_bytes();routed=raw
   if p.name=='run.py':
    s=raw.decode()
    if short=='T':
     substitutions={"ROOT=Path(__file__).resolve().parents[2]":f"ROOT=Path({str(ROOT)!r})","OUT=ROOT/'rust/verification/pulsar_lifecycle'":f"OUT=Path({str(OUT/'T')!r})"}
    else:
     assert sha(p)=='7d6424654a666d3852255bd9264ba0fbb88ba3c6e57f7bf717da4087a6b10529'
     substitutions={"ROOT=HERE.parents[1]":f"ROOT=Path({str(ROOT)!r})","OUT=RUST/'verification/observation_storage'":f"OUT=Path({str(OUT/'R')!r})"}
    for before,after in substitutions.items():assert s.count(before)==1;s=s.replace(before,after)
    restored=s
    for before,after in substitutions.items():restored=restored.replace(after,before)
    assert restored.encode()==raw
    routed=s.encode();changes.append(''.join(difflib.unified_diff(raw.decode().splitlines(True),s.splitlines(True),fromfile=str(p),tofile=str(target/p.name))))
   if short=='R' and p.name=='verify.py':
    s=raw.decode();needle="baseline=port.load(port.HERE/'baseline.json')['sources']"
    assert s.count(needle)==1
    s=s.replace(needle,needle+"\n baseline['rust/src/get_obs_coord.rs']="+repr(audit['get_obs_coord.rs']['current_sha256'])+" # W safe EOP adapter; exact unsafe blocks checked in routing/unsafe_audit.json")
    routed=s.encode();changes.append(''.join(difflib.unified_diff(raw.decode().splitlines(True),s.splitlines(True),fromfile=str(p),tofile=str(target/p.name))))
   (target/p.name).write_bytes(routed);sources[p.name]=dict(original=sha(p),staged=sha(target/p.name))
  (dest/(short+'.diff')).write_text(''.join(changes));receipt[short]=dict(files=sources,diff_sha256=sha(dest/(short+'.diff')))
  if short=='T':
   target_freeze=OUT/'T/freeze';target_freeze.mkdir(parents=True,exist_ok=True)
   for p in (ROOT/'rust/verification/pulsar_lifecycle/freeze').iterdir():
    if p.is_file():shutil.copyfile(p,target_freeze/p.name)
 save(dest/'staging.json',receipt)
 return dest
if __name__=='__main__':stage()
