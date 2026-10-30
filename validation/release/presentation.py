#!/usr/bin/env python3
"""Focused 0106/0107 before/after application checks; no new science qualification."""
import argparse
import json
import os
from pathlib import Path
import re
from common import Run,sha
from diagnostic_attribution import Attribution


def normalized(data, kind, attribution):
    data=attribution.stream(data,kind)
    # Existing AH timing metadata allowance; never a result-value tolerance.
    data=re.sub(rb'(?m)^Finishing off: time taken = [0-9.]+ \(s\)$',
                b'Finishing off: time taken = <CPU> (s)',data)
    # Only the named source timing calls, not arbitrary T= values in message data.
    for file,lines in [('tempo2.C',[503,533,537,564,624]),('sortToAs.C',[15,18]),('formResiduals.C',[438,2454])]:
        for line in lines:
            prefix=attribution.native[file,line]
            data=re.sub(rb'(?m)^'+re.escape(prefix)+rb' T=[0-9.]+ s:',prefix+b' T=<CPU> s:',data)
    return data


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','candidate','baseline','prefix','runtime']:p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();run=Run(a.out);candidate=a.candidate.resolve();prefix=a.prefix.resolve()
    current=prefix/'bin/tempo2-rust';baseline=a.baseline.resolve();runtime=a.runtime.resolve()
    mapping=Attribution(candidate)
    for relative,digest in mapping.sources.items():assert sha(prefix/'share/tempo2-rust'/relative)==digest,relative
    example=prefix/'share/tempo2-rust/rust/TERMINAL_EXAMPLES.md';assert example.is_file()
    fixture=candidate/'validation/general_fitting/inputs'
    par=fixture/'robust-H.par';tim=fixture/'multi-frequency.tim'
    env=dict(os.environ,TEMPO2=str(runtime),LC_ALL='C',VECLIB_MAXIMUM_THREADS='1',OPENBLAS_NUM_THREADS='1',OMP_NUM_THREADS='1')
    outputs={};checks={}
    cases=[('report',[]),('versions',['-displayversion']),('debug',['-debug']),
           ('help',['--help']),('short-help',['-h']),('extended-help',['-H']),('version',['-v'])]
    for label,flags in cases:
        for role,exe in [('before',baseline),('after',current)]:
            work=run.out/(label+'-'+role);work.mkdir()
            args=flags if label in ['help','short-help','extended-help','version'] else ['-npsr','1','-nobs','100','-noaccel','-f',par,tim,*flags]
            expected=1 if label in ['short-help','extended-help','version'] else 0
            data=run.run([exe,*args],label+'-'+role,cwd=work,env=env,expected=expected)
            outputs[label,role]=data
            assert not (run.out/(label+'-'+role+'.stderr')).read_bytes(),(label,role)
        if label in ['report','versions','debug']:
            before=normalized(outputs[label,'before'],'native',mapping)
            after=normalized(outputs[label,'after'],'rust',mapping)
            (run.out/(label+'-before.projected.stdout')).write_bytes(before)
            (run.out/(label+'-after.projected.stdout')).write_bytes(after)
            checks[label]=before==after
            artifacts=[]
            for role in ['before','after']:
                work=run.out/(label+'-'+role)
                artifacts.append({str(path.relative_to(work)):sha(path) for path in work.rglob('*') if path.is_file()})
            checks[label+'-files']=artifacts[0]==artifacts[1]
        elif label=='version':checks[label]=outputs[label,'before']==outputs[label,'after']
    help_output=outputs['help','after'];short=outputs['short-help','after']
    checks['native-first-help']=help_output.index(b'tempo2-rust -f model.par arrivals.tim')<help_output.index(b'Machine interfaces (JSON)')
    checks['scope-help']=b'-pre still fits' in help_output and b'timing evaluates unless --fit' in help_output
    checks['plugin-boundary']=all(b'Plugin execution (-gr, -output and selection plugins) is unavailable' in outputs[name,'after'] for name in ['help','short-help','extended-help'])
    checks['runnable-short-example']=b' -f mypar.par mytim.tim' in short and b' -gr plk -f' not in short
    checks['installed-example']=b'robust-H.par' in example.read_bytes() and b'explicit machine interface' in example.read_bytes()
    for label,route in [('evaluation',['timing']),('fit',['timing','--fit']),('bats',['bats'])]:
        products={}
        for role,exe in [('before',baseline),('after',current)]:
            data=run.run([exe,*route,'--par',par,'--tim',tim,'--runtime',runtime],label+'-'+role,cwd=run.out,env=env)
            (run.out/(label+'-'+role+'.json')).write_bytes(data)
            products[role]=json.loads(data)
        checks['machine-'+label]=mapping.structured(products['before'],'native')==mapping.structured(products['after'],'rust')
        checks['machine-'+label+'-stderr']=normalized((run.out/(label+'-before.stderr')).read_bytes(),'native',mapping)==normalized((run.out/(label+'-after.stderr')).read_bytes(),'rust',mapping)
    run.save({'status':'pass' if all(checks.values()) else 'failure','checks':checks,
        'scope':'Presentation and unchanged before/after calculation products; not independent scientific equivalence',
        'baseline':sha(baseline),'current':sha(current),'inputs':{str(v):sha(v) for v in [par,tim,example]},
        'runtime':str(runtime),'diagnostic_attribution':mapping.manifest(),
        'runner':sha(__file__),'projection':sha(Path(__file__).with_name('diagnostic_attribution.py'))})
    print(json.dumps(checks,indent=2));raise SystemExit(not all(checks.values()))
