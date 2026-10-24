"""Explicit disposable native correction experiment; never modifies U's oracle."""
import difflib,gzip,importlib.util,json,os,sys
from pathlib import Path
from freeze import ROOT,HERE,check
U=ROOT/'validation/terrestrial_resources';OUT=ROOT/'rust/verification/terrestrial_resources'
sys.dont_write_bytecode=True;sys.path.insert(0,str(U))
import profile as native_profile
spec=importlib.util.spec_from_file_location('u_runner',U/'run.py');u=importlib.util.module_from_spec(spec);spec.loader.exec_module(u)
PATCH=('tropo.C','c = frac*bs[ilat2] + (1.0-frac)*cs[ilat1];','c = frac*cs[ilat2] + (1.0-frac)*cs[ilat1];')
def differences(a,b,path='$'):
 assert type(a) is type(b),(path,type(a),type(b))
 if isinstance(a,dict):
  assert list(a)==list(b),(path,'membership')
  return [d for k in a for d in differences(a[k],b[k],path+'.'+k)]
 if isinstance(a,list):
  assert len(a)==len(b),(path,'membership')
  return [d for i,(x,y) in enumerate(zip(a,b)) for d in differences(x,y,path+f'[{i}]')]
 return [] if a==b else [dict(path=path,original=a,corrected=b)]
def main():
 check();dest=OUT/'corrected_native';assert not (dest/'reference.json.gz').exists(),'explicit experiment does not overwrite'
 native_profile.BUILD=OUT
 # Keep both source and case prefix lengths identical to accepted U so logger
 # truncation is tested exactly. All other caller/execution logic is unchanged.
 raw=(U/'run.py').read_text();routed=raw.replace("BUILD/'cases'","WORK_CASES")
 assert routed.count('WORK_CASES')==2
 exec(compile(routed,str(U/'run.py'),'exec'),u.__dict__)
 u.WORK_CASES=OUT/'xxxx';u.BUILD=OUT
 (OUT/'routing').mkdir(exist_ok=True)
 (OUT/'routing/U-run.diff').write_text(''.join(difflib.unified_diff(raw.splitlines(True),routed.splitlines(True))))
 catalog=json.loads((U/'outputs/frozen/cases.json').read_text());original=u.read_gzip(U/'outputs/reference.json.gz')
 exe,profile=native_profile.compile_probe('fixed00',mutation=PATCH)
 assert len(str(exe.parent/'source'))==len(str(U/'build/ordinary/source'))
 results=[];changes=[];receipts=[]
 for case,old in zip(catalog,original):
  value,receipt=u.execute(case,exe,profile,dest/'raw');u.validate(case,value)
  delta=differences(old,value,case['id'])
  # This correction can only change NMF_wet and dependent delay scalars, never
  # inputs, resource state, diagnostics, membership, native outcome or hydrostatic.
  for d in delta:assert '.result.' in d['path'] and ('.wet.bits' in d['path'] or '.wet_mapping.bits' in d['path'] or '.delay.bits' in d['path']),d
  if delta:changes.append(dict(id=case['id'],fields=delta))
  results.append(value);receipts.append(receipt)
 u.write_gzip(dest/'reference.json.gz',results);u.write_gzip(dest/'receipts.json.gz',receipts)
 u.save(dest/'changes.json',changes);u.save(dest/'contract.json',dict(original_sha256=u.sha((U/'outputs/reference.json.gz').read_bytes()),corrected_sha256=u.sha((dest/'reference.json.gz').read_bytes()),patch=PATCH,profile=profile,cases=len(results),changes=changes))
 check();print('Corrected native contract:',len(results),'cases;',len(changes),'changed cases;',sum(len(c['fields']) for c in changes),'fields')
if __name__=='__main__':main()
