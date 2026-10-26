"""Comparison-only replay of AF plan outputs with independent endpoint/count mutations."""
from pathlib import Path
from decimal import Decimal
import hashlib,importlib.util,json,shutil,sys,tempfile
sys.dont_write_bytecode=True
root=Path('/Users/Eric.Dong/Downloads/tempo2rust');here=root/'validation/predictors'
out=Path(tempfile.mkdtemp(prefix='tempo2-af-plan-review-'))
Path('/tmp/tempo2-af-plan-review-current').write_text(str(out)+'\n')
source=out/'recorded';source.mkdir()
original=here/'build/public-debug-final'
for name in ['isolated-bary-cheby','isolated-bary-polyco']:
 shutil.copytree(original/name,source/name)
for name in ['run_public.py','compare.py','CONTRACT.md']:shutil.copy2(here/name,out/name)
shutil.copytree(here/'fixtures',out/'fixtures')
sys.path.insert(0,str(out));spec=importlib.util.spec_from_file_location('af_plan_review',out/'run_public.py');mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
mod.ROOT=root;mod.HERE=here
native=here/'build/native-v5/predictor';rust=root/'rust/target/debug/tempo2-rust'
assert native.is_file() and rust.is_file(); # Copied by runner, never executed in this comparison-only review.
records=[]
for mutation in ['baseline','shift_first_start','shift_first_end','remove_interior_segment']:
 def cached_run(cmd,cwd,env,name):
  cwd=Path(cwd)
  if name=='generate':
   src=source/cwd.parent.name/cwd.name
   for f in src.iterdir():
    if f.is_file():shutil.copy2(f,cwd/f.name)
  else:
   src=source/cwd.name
   for suffix in ['stdout','stderr']:shutil.copy2(src/(name+'.'+suffix),cwd/(name+'.'+suffix))
   if name.startswith('inspect-'):shutil.copy2(src/(name+'.tsv'),Path(cmd[2]))
   elif name.endswith('-native'):shutil.copy2(src/(name[:-7]+'.tsv'),Path(cmd[2]))
   if name=='plan-t2pred.dat-native-rust':
    f=cwd/(name+'.stdout');d=json.loads(f.read_text());s=d['plan']['segments'];assert len(s)>2
    if mutation=='shift_first_start':s[0]['start_mjd']=str(Decimal(s[0]['start_mjd'])+Decimal('.00001'))
    if mutation=='shift_first_end':s[0]['end_mjd']=str(Decimal(s[0]['end_mjd'])+Decimal('.00001'))
    if mutation=='remove_interior_segment':del s[1]
    f.write_text(json.dumps(d)+'\n')
  return {'command':list(map(str,cmd)),'returncode':0,'stdout':name+'.stdout','stderr':name+'.stderr','source':'preserved output replay; no engine execution'}
 mod.run=cached_run
 sys.argv=['run_public.py','--native',str(native),'--rust',str(rust),'--out',str(out/mutation),'--case','isolated-bary']
 try:
  result=mod.main();receipt=json.loads((out/mutation/'receipt.json').read_text())
  records.append({'mutation':mutation,'runner_failed':bool(result),'case_failures':[(r['kind'],len(r['failures']))for r in receipt['cases']]})
 except Exception as e:records.append({'mutation':mutation,'runner_failed':True,'exception':type(e).__name__+': '+str(e)})
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
receipt={'review':'Comparison-only plan replay; no engine execution. Only one Rust plan record per run is changed; declared segment_count, averages and native records remain unchanged.','source_files':{n:sha(out/n) for n in ['run_public.py','compare.py','CONTRACT.md']},'records':records}
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
shutil.copy2(__file__,out/'central_af_plan_review.py')
print(json.dumps(receipt,indent=2));print('evidence',out)
