#!/usr/bin/env python3
"""Run existing M2 comparisons without sorting raw modes or weakening gates."""
import argparse
import importlib.util
import json
from pathlib import Path
import shutil
from common import ROOT, Run, sha

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--out',type=Path,required=True);p.add_argument('--candidate',type=Path,required=True)
    p.add_argument('--example',type=Path,required=True)
    a=p.parse_args();run=Run(a.out);out=run.out;candidate=a.candidate.resolve()
    run.run([a.example.resolve()],'public-fits')
    actual=candidate/'rust/target/m2';shutil.copytree(actual,out/'rust')
    fitting=candidate/'validation/fitting';cases=json.loads((fitting/'cases.json').read_text())
    spec=importlib.util.spec_from_file_location('al_m2_compare',ROOT/'validation/fitting/compare.py')
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    rows=[]
    def record(name,result):
        rows.append({'name':name,'failures':result.errors,'measurements':result.measurements,'maxima':result.maxima,'mode_permutations':result.mode_permutations})
    for family in ['solver','timing']:
        for c in cases[family]:
            if c.get('expected')=='boundary':continue
            name=c['name'];reference=json.loads((fitting/'reference'/(name+'.json')).read_text())
            values=json.loads((actual/('shared_'+name+'.json')).read_text())
            references=[reference] if family=='solver' else reference['cycles']
            for index,(ref,value) in enumerate(zip(references,values,strict=True)):
                record(name+'-shared-'+str(index),m.compare_shared(ref,value,reference['mode']))
            if family=='timing':
                par=(fitting/'inputs'/c['par']).read_text()
                epoch=next(line.split()[1] for line in par.splitlines() if line.startswith('PEPOCH '))
                record(name+'-timing',m.compare_timing(reference,json.loads((actual/(name+'.json')).read_text()),epoch))
    record('callbacks',m.compare_callbacks(json.loads((fitting/'reference/callbacks.json').read_text()),json.loads((actual/'callbacks.json').read_text())))
    failures=sum(len(r['failures']) for r in rows)
    run.save({'status':'failure' if failures else 'pass','failures':failures,'records':rows,
        'comparator':sha(ROOT/'validation/fitting/compare.py'),'inputs':{str(p):sha(p) for p in (fitting/'reference').glob('*.json')}})
    raise SystemExit(bool(failures))
