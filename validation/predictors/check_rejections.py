"""Replay constructed records and reject central's four proven comparison omissions."""
from pathlib import Path
from copy import deepcopy
import argparse, json
from compare import constructed, read_tsv, Checks, plan, native_models
p=argparse.ArgumentParser();p.add_argument('--public',type=Path,required=True);p.add_argument('--out',type=Path,required=True);a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False)
results=[]
for kind in ['cheby','polyco']:
    base=a.public/('isolated-bary-'+kind)
    data=json.loads((base/'rust/generate.stdout').read_text())['result'];ref=read_tsv(base/'native/result.tsv')
    original=constructed(kind,data,ref);assert not original.failures,original.failures
    changes=[('empty_cheby_coeff',lambda m:m['cheby'].__setitem__('coeff',[])),('wrong_cheby_dimensions',lambda m:m['cheby'].__setitem__('nx',999)),('wrong_cheby_frequency_range',lambda m:m.__setitem__('freq_start','1000000')),('changed_cheby_coefficient',lambda m:m['cheby']['coeff'].__setitem__(0,'1e100'))] if kind=='cheby' else [('changed_polyco_segment',lambda m:m.__setitem__('coeff',['1e100']))]
    for name,change in changes:
        mutant=deepcopy(data);models=mutant['predictor']['Cheby']['segments'] if kind=='cheby' else mutant['models']['segments'];change(models[0]);checks=constructed(kind,mutant,ref);assert checks.failures,name
        results.append({'mutation':name,'rejected':True,'failures':checks.failures})
base=a.public/'isolated-bary-cheby';record=json.loads((base/'plan-t2pred.dat-native-rust.stdout').read_text());ref=read_tsv(base/'plan-t2pred.dat-native.tsv');models=native_models('cheby',read_tsv(base/'inspect-t2pred.dat-native.tsv'))
assert not plan('cheby',record,ref,models,'1400').failures
for name,change in [('plan_start',lambda p:p['plan']['segments'][0].__setitem__('start_mjd',str(float(p['plan']['segments'][0]['start_mjd'])+0.00001))),('plan_end',lambda p:p['plan']['segments'][0].__setitem__('end_mjd',str(float(p['plan']['segments'][0]['end_mjd'])+0.00001))),('plan_deleted_interval',lambda p:p['plan']['segments'].pop(1)),('plan_selection',lambda p:p['plan']['segments'][0].__setitem__('start_segment',999)),('plan_declared_count',lambda p:p.__setitem__('segment_count',999))]:
    mutant=deepcopy(record);change(mutant);c=plan('cheby',mutant,ref,models,'1400');assert c.failures,name;results.append({'mutation':name,'rejected':True,'failures':c.failures})
for name,x,y in [('units',1,1000000),('selected_segment',1,0),('plan_count',3,4),('zero_sign','-0','0'),('nonfinite','NaN','1')]:
    c=Checks();c.number(name,x,y);assert c.failures;results.append({'mutation':name,'rejected':True})
(a.out/'receipt.json').write_text(json.dumps(results,indent=2)+'\n');print(len(results),'rejection controls passed')
