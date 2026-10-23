#!/usr/bin/env python3
"""Default: fresh build, fresh challenged executions, complete ordered exact comparison."""
import argparse
import concurrent.futures
import json
from pathlib import Path
import shutil
import subprocess
import uuid
from profile import HERE,BUILD,ENV,compile_probe,runners,sha,save
from fixtures import cases


def compare(a,b,path='$'):
    if type(a) is not type(b):raise AssertionError(path+': type differs')
    if isinstance(a,dict):
        if a.keys()!=b.keys():raise AssertionError(path+': field membership differs')
        for k in a:compare(a[k],b[k],path+'.'+k)
    elif isinstance(a,(list,tuple)):
        if len(a)!=len(b):raise AssertionError(path+': length differs')
        for i in range(len(a)):compare(a[i],b[i],path+f'[{i}]')
    elif a!=b:raise AssertionError(path+': '+repr(a)[:80]+' != '+repr(b)[:80])


def execution_identity(exe,p):
    actual=dict(executable_path=str(exe.resolve()),executable_sha256=sha(exe.read_bytes()),runner_sources=runners())
    for k in ['executable_path','executable_sha256']:compare(p[k],actual[k],'execution.'+k)
    compare(p['stable']['runner_sources'],actual['runner_sources'],'execution.runner_sources')
    return actual


def identity_gate(expected,p,exe):
    compare(expected['stable'],p['stable'],'stable')
    return execution_identity(exe,p)


def execute(c,exe,p):
    work=BUILD/'cases'/c['id']
    if work.exists():shutil.rmtree(work)
    work.mkdir(parents=True);ep=work/'events.jsonl';ap=work/'actions.tsv'
    ap.write_text('\n'.join('\t'.join('x'+v.encode().hex() for v in a) for a in c['actions'])+'\n')
    nonce=uuid.uuid4().hex;before=execution_identity(exe,p)
    with (work/'stdout').open('wb') as stdout,(work/'stderr').open('wb') as stderr:
        result=subprocess.run([str(exe),str(ap),str(ep),nonce],stdout=stdout,stderr=stderr,env=ENV,timeout=30)
    after=execution_identity(exe,p);compare(before,after,'execution')
    assert ep.exists(),c['id']+': missing new event file'
    events=[json.loads(s) for s in ep.read_text().splitlines()]
    assert events and events.pop(0)==dict(phase='launch',nonce=nonce),c['id']+': wrong fresh challenge'
    def normalize(s):return s.replace(str(Path(p['command'][p['command'].index('-DHAVE_CONFIG_H')+1][2:])),'{source}')
    # The only normalization is the actual staged-source directory, not lines or message content.
    return dict(id=c['id'],classification=c['classification'],actions=c['actions'],returncode=result.returncode,
                events=events,stdout=normalize((work/'stdout').read_text()),stderr=normalize((work/'stderr').read_text()),fresh_launch=True)


def validate(c,r):
    compare(c['returncode'],r['returncode'],c['id']+'.returncode')
    expected=[]
    for i,a in enumerate(c['actions']):
        expected.append((i,'begin',a[0]))
        if c['returncode']==0 or i<len(c['actions'])-1:expected.append((i,'end',a[0]))
    compare(expected,[(e['action'],e['phase'],e['op']) for e in r['events']],c['id']+'.ordered_stages')
    for e in r['events']:assert e['stdout_position']>=0


def regenerate(record=False):
    (HERE/'regeneration.json').unlink(missing_ok=True)
    cs=cases();exe,p=compile_probe()
    if not record:identity_gate(json.loads((HERE/'profile.json').read_text()),p,exe)
    def one(c):
        r=execute(c,exe,p);validate(c,r);return r
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:results=list(pool.map(one,cs))
    execution=execution_identity(exe,p)
    if record:
        save(HERE/'cases.json',cs);save(HERE/'profile.json',p);save(HERE/'observations.json',results)
    else:
        compare(json.loads((HERE/'cases.json').read_text()),cs,'cases')
        compare(json.loads((HERE/'observations.json').read_text()),results,'observations')
        identity_gate(json.loads((HERE/'profile.json').read_text()),p,exe)
        save(HERE/'regeneration.json',dict(status='PASS',cases=len(cs),events=sum(len(r['events']) for r in results),profile=p,execution=execution,
             observations_sha256=sha((HERE/'observations.json').read_bytes()),complete_ordered_comparison=True))
    print(('Recorded' if record else 'PASS: fresh complete exact comparison'),len(cs),'cases',flush=True)
    return cs,results,exe,p


if __name__=='__main__':
    ap=argparse.ArgumentParser();ap.add_argument('--record',action='store_true');args=ap.parse_args();regenerate(args.record)
