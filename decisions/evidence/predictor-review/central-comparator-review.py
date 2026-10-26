"""Comparison-only replay of preserved AF outputs with controlled JSON mutations."""
from pathlib import Path
import copy,hashlib,importlib.util,json,shutil,sys,tempfile
sys.dont_write_bytecode=True
root=Path('/Users/Eric.Dong/Downloads/tempo2rust');here=root/'validation/predictors'
out=Path(tempfile.mkdtemp(prefix='tempo2-af-comparator-review-'))
Path('/tmp/tempo2-af-comparator-review-current').write_text(str(out)+'\n')
original=here/'build/public-debug-v2';source=out/'recorded'
source.mkdir()
for name in ['isolated-bary-cheby','isolated-bary-polyco']:
 shutil.copytree(original/name,source/name)
for name in ['run_public.py','check_components.py','CONTRACT.md']:shutil.copy2(here/name,out/name)
sys.path.insert(0,str(out));spec=importlib.util.spec_from_file_location('af_public_review',out/'run_public.py');mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
mod.ROOT=root;mod.HERE=here
native=here/'build/native-v5/predictor';rust=root/'rust/target/debug/tempo2-rust'
records=[]
for mutation in ['baseline','empty_cheby_coefficients','wrong_cheby_dimensions','wrong_cheby_frequency_range','wrong_polyco_first_coefficients','empty_polyco_last_coefficients']:
 def cached_run(cmd,cwd,env,name):
  cwd=Path(cwd)
  if name=='generate':
   kind=cwd.parent.name.split('-')[-1];engine=cwd.name
   src=source/('isolated-bary-'+kind)/engine
   for f in src.iterdir():
    if f.is_file():shutil.copy2(f,cwd/f.name)
   if engine=='rust':
    file=cwd/'generate.stdout';data=json.loads(file.read_text())
    if kind=='cheby':
     m=data['result']['predictor']['Cheby']['segments'][0]
     if mutation=='empty_cheby_coefficients':m['cheby']['coeff']=[]
     if mutation=='wrong_cheby_dimensions':m['cheby']['nx']=999
     if mutation=='wrong_cheby_frequency_range':m['freq_start']='1000000'
    else:
     if mutation=='wrong_polyco_first_coefficients':data['result']['models']['segments'][0]['coeff']=['1e100']
     if mutation=='empty_polyco_last_coefficients':data['result']['coefficients']=[]
    file.write_text(json.dumps(data)+'\n')
  else:
   src=source/cwd.name
   for suffix in ['stdout','stderr']:
    shutil.copy2(src/(name+'.'+suffix),cwd/(name+'.'+suffix))
   if name.endswith('-native'):
    shutil.copy2(src/(name[:-7]+'.tsv'),Path(cmd[2]))
  return {'command':list(map(str,cmd)),'returncode':0,'stdout':name+'.stdout','stderr':name+'.stderr','source':'preserved output replay; no engine execution'}
 mod.run=cached_run
 sys.argv=['run_public.py','--native',str(native),'--rust',str(rust),'--out',str(out/mutation),'--case','isolated-bary']
 try:
  result=mod.main();receipt=json.loads((out/mutation/'receipt.json').read_text())
  records.append({'mutation':mutation,'runner_failed':bool(result),'case_failures':[(r['kind'],len(r['failures']))for r in receipt['cases']]})
 except Exception as e:
  records.append({'mutation':mutation,'runner_failed':True,'exception':type(e).__name__+': '+str(e)})
print(json.dumps(records,indent=2))
receipt={'review':'Only comparator logic was executed against preserved original outputs and controlled generated-model JSON changes; this is not a new engine validation.','source_run_public_sha256':hashlib.sha256((out/'run_public.py').read_bytes()).hexdigest(),'records':records}
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print('evidence',out)
