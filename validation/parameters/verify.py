#!/usr/bin/env python3
"""Integrity checks and disposable compiled source mutations, never canonical edits."""
import copy
import json
import subprocess
from profile import HERE, BUILD, sha, save, prepare, compile_probe
from run import compare, execute, validate

MUTATIONS = [
 ('dispatch', 'alias_DM', 'readParfile.C', '&(psr->param[param_dm]),0);', '&(psr->param[param_cm]),0);'),
 ('index', 'index_FB', 'readParfile.C', '&(psr->param[param_fb]),fbval);', '&(psr->param[param_fb]),0);'),
 ('duplicate', 'value_forms', 'readParfile.C', 'parameter->err[arr]     = 0.0;', 'parameter->err[arr]     = 0.0; parameter->prefitErr[arr] = 0.0;'),
 ('uncertainty', 'coordinate_duplicate', 'readParfile.C', 'parameter->prefitErr[arr]/=(12.0*60.0*60.0/M_PI);', 'parameter->prefitErr[arr]/=(24.0*60.0*60.0/M_PI);'),
 ('link_direction', 'link_DSHK_after', 'tempo2Util.C', 'return 1.0/psr->param[param_px].val[0];', 'return psr->param[param_px].val[0];'),
 ('finalization', 'final_defaults', 'readParfile.C', 'copyParam(psr->param[param_pepoch],&(psr->param[param_posepoch]));', '/* mutation: omit position epoch copy */'),
 ('wrapper_reset', 'repeat_full', 'readParfile.C', 'psr[p].nJumps=0;', 'psr[p].nJumps=1;'),
 ('failure_stage', 'final_no_pepoch', 'readParfile.C', 'printf("ERROR [PAR1]: Have not set a period epoch in %s\\n",filename);\n        exit(1);', 'printf("ERROR [PAR1]: Have not set a period epoch in %s\\n",filename);\n        return;'),
]


def must_reject(a,b,name):
    try:compare(a,b)
    except AssertionError as e:return dict(name=name,rejected=True,diagnostic=str(e))
    raise AssertionError('Comparator accepted mutation '+name)


def main():
    cs=json.loads((HERE/'cases.json').read_text());rs=json.loads((HERE/'observations.json').read_text())
    assert len(cs)==len(rs) and len({c['id'] for c in cs})==len(cs)
    for c,r in zip(cs,rs):validate(c,r)
    cm={c['id']:c for c in cs};rm={r['id']:r for r in rs}
    checks=[]
    for name,kind in [('case deletion','case'),('extra case','extra'),('case reorder','order'),('event deletion','event'),('field deletion','field'),('component deletion','component'),('class edit','class'),('action edit','action')]:
        b=copy.deepcopy(rs)
        if kind=='case':b.pop()
        elif kind=='extra':b.append(copy.deepcopy(b[0]))
        elif kind=='order':b[0],b[1]=b[1],b[0]
        elif kind=='event':b[0]['events'].pop()
        elif kind=='field':del b[0]['returncode']
        elif kind=='component':b[0]['events'][3]['result'].pop(next(iter(b[0]['events'][3]['result'])))
        elif kind=='class':next(e for e in b if e['id']=='numeric')['events'][1]['result']['class']='NaN'
        elif kind=='action':b[0]['actions'][0][2]='99'
        checks.append(must_reject(rs,b,name))
    # Actual launch challenge: execute a successful no-output program against a
    # previously populated case directory. Directory deletion must remove stale data.
    try:execute(cm['numeric'],__import__('pathlib').Path('/usr/bin/true'))
    except (FileNotFoundError,AssertionError) as e:checks.append(dict(name='stale output / no probe launch',rejected=True,diagnostic=type(e).__name__))
    else:raise AssertionError('Accepted no-op executable')
    alloc=[next(e['result'] for e in rm['allocate_'+str(i)]['events'] if e['phase']=='end') for i in [0,1]]
    compare(*alloc,'unused realloc argument')
    init=[rm[n]['events'][1]['result'] for n in ['registry','init_full']]
    a,b=copy.deepcopy(init);a['natural_controls'].pop('noWarnings');b['natural_controls'].pop('noWarnings');compare(a,b,'unused fullSetup')
    evidence=[];src=prepare()
    for name,case,file,old,new in MUTATIONS:
        exe,profile=compile_probe(src,'mutation_'+name,(file,old,new))
        observed=execute(cm[case],exe)
        rejection=must_reject(rm[case],observed,name)
        evidence.append(dict(name=name,case=case,profile=profile,rejection=rejection,observed=observed))
        print('Detected source mutation:',name,flush=True)
    save(HERE/'verification.json',dict(comparator_checks=checks,source_mutations=evidence,
         initialization_equivalence=['allocateMemory realloc=0/1','initialiseOne fullSetup=0/1 except explicit noWarnings input'],
         cases_sha256=sha((HERE/'cases.json').read_bytes()),observations_sha256=sha((HERE/'observations.json').read_bytes())))
    print('PASS: membership/freshness gates and',len(evidence),'compiled source mutations')


if __name__=='__main__':main()
