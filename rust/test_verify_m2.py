#!/usr/bin/env python3
"""Reject incomplete shared solver evidence, independently of numerical gates."""
import json
from pathlib import Path
import tempfile
from verify_m2 import FIT, OUTPUT, load_shared_cases


def mutations():
    declared=json.loads((FIT/'cases.json').read_text())
    cases={family:[c for c in declared[family] if c['name']==name]
           for family,name in [('solver','weighted_unscaled'),('timing','all')]}
    baseline={name:json.loads((OUTPUT/'debug'/name).read_text())
              for name in ['shared_weighted_unscaled.json','shared_all.json']}
    changes=[]
    for name,cycles in baseline.items():
        for label,value in [('non-array object',{}),('non-array null',None),
                            ('empty cycles',[]),('missing cycle',cycles[:-1]),
                            ('extra cycle',cycles+[cycles[-1]])]:
            changes.append((name+' '+label,{**baseline,name:value}))
        changes.append((name+' missing artifact',{k:v for k,v in baseline.items() if k!=name}))
    changes.append(('extra shared case',{**baseline,'shared_unexpected.json':baseline['shared_all.json']}))
    results=[]
    with tempfile.TemporaryDirectory(prefix='m2-coverage-') as td:
        dest=Path(td)
        for label,files in [('complete one- and two-cycle cases',baseline),*changes]:
            for p in dest.glob('*.json'):p.unlink()
            for name,value in files.items():(dest/name).write_text(json.dumps(value))
            try:loaded=load_shared_cases(cases,dest)
            except ValueError as exc:
                assert label!='complete one- and two-cycle cases',str(exc)
                results.append({'mutation':label,'detected':True,'error':str(exc)})
            else:
                assert label=='complete one- and two-cycle cases',label+' was not rejected'
                assert {name:len(a) for name,(_,_,a) in loaded.items()}=={'weighted_unscaled':1,'all':2}
    return results


if __name__=='__main__':print(json.dumps(mutations(),indent=2))
