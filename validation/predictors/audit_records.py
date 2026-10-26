"""Comparison-only delivery audit, applying the exact observer to recorded JSON."""
from pathlib import Path
from copy import deepcopy
import argparse,json
from compare import plan,native_models,read_tsv,serialization,Checks,phase_bound,cheby_scale
p=argparse.ArgumentParser();p.add_argument('--public',type=Path,required=True);p.add_argument('--out',type=Path,required=True);a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False);records=[]
for case in sorted(a.public.glob('*-cheby'))+sorted(a.public.glob('*-polyco')):
 kind='cheby' if case.name.endswith('cheby') else 'polyco';result=json.loads((case/'rust/generate.stdout').read_text())['result'];original=result['predictor']['Cheby']['segments'] if kind=='cheby' else result['models']['segments']
 if kind=='cheby':
  source=read_tsv(case/'native/result.tsv');sc=Checks()
  for key,field in [('rms','rms_turn'),('mav','mav_turn')]:sc.number('source_test.'+key,result['source_test'][field],source[key,0],phase_bound(max(cheby_scale(m) for m in original)))
  records.append({'case':case.name,'file':'source_test','owner':'constructed','plan_checks':sc.count,'failures':sc.failures})
 for file in ['t2pred.dat'] if kind=='cheby' else ['polyco_new.dat','newpolyco.dat']:
  for owner in ['native','rust']:
   inspection=read_tsv(case/f'inspect-{file}-{owner}.tsv');models=native_models(kind,inspection);ref=read_tsv(case/f'plan-{file}-{owner}.tsv');actual=json.loads((case/f'plan-{file}-{owner}-rust.stdout').read_text());checks=plan(kind,actual,ref,models,'1400');row={'case':case.name,'file':file,'owner':owner,'plan_checks':checks.count,'failures':checks.failures};records.append(row)
   if owner=='rust':
    parsed=json.loads((case/f'exchange-{file}-{owner}-0-rust.stdout').read_text())['input']['predictor']['Cheby' if kind=='cheby' else 'T1']['segments'];sc=serialization(kind,original,parsed,file);row['serialization_checks']=sc.count;row['failures']+=sc.failures
    mutant=deepcopy(parsed);mutant[0]['cheby' if kind=='cheby' else 'coeff'] = {'coeff':['1e100'],'nx':1,'ny':1} if kind=='cheby' else ['1e100'];assert serialization(kind,original,mutant,file).failures
(a.out/'receipt.json').write_text(json.dumps(records,indent=2)+'\n');print('checks',sum(r['plan_checks']+r.get('serialization_checks',0) for r in records),'failures',sum(len(r['failures']) for r in records));raise SystemExit(any(r['failures'] for r in records))
