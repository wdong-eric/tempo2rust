#!/usr/bin/env python3
"""Freeze with --record; otherwise rebuild and compare every ordered field exactly."""
import argparse
import copy
import concurrent.futures
import json
import os
import shutil
import subprocess
import uuid
from profile import HERE, ROOT, BUILD, ENV, prepare, compile_probe, save, sha
from fixtures import cases
from inventory import inventory


def normalize(s):
    return s.replace(str(BUILD),'{build}').replace(str(HERE),'{parameters}')


def compare(a,b,path='$'):
    if type(a) is not type(b):raise AssertionError(path+': type differs')
    if isinstance(a,dict):
        if a.keys()!=b.keys():raise AssertionError(path+': field membership differs')
        for k in a:compare(a[k],b[k],path+'.'+k)
    elif isinstance(a,list):
        if len(a)!=len(b):raise AssertionError(path+': length differs')
        for i in range(len(a)):compare(a[i],b[i],path+f'[{i}]')
    elif a!=b:raise AssertionError(path+': '+repr(a)[:100]+' != '+repr(b)[:100])


def execute(c,exe):
    work=BUILD/'cases'/(c['id']+'-'+sha(c['id'].encode())[:10])
    if work.exists():shutil.rmtree(work)
    work.mkdir(parents=True)
    for name,data in c['files'].items():
        (work/name).parent.mkdir(parents=True,exist_ok=True);(work/name).write_text(data)
    actions=[[v.replace('{case}',str(work)) for v in a] for a in c['actions']]
    ap=work/'actions.tsv';ap.write_text('\n'.join('\t'.join('x'+v.encode().hex() for v in a) for a in actions)+'\n')
    ep=work/'events.jsonl'
    assert not ep.exists()
    nonce=uuid.uuid4().hex
    env={**ENV,'TEMPO2':str(BUILD/'runtime')}
    p=subprocess.run([str(exe),str(ap),str(ep),nonce],capture_output=True,text=True,env=env,timeout=90)
    ev=[json.loads(normalize(s)) for s in ep.read_text().splitlines()]
    assert ev.pop(0)=={'phase':'launch','nonce':nonce},(c['id'],'No fresh launch evidence')
    return dict(id=c['id'],actions=c['actions'],state=c['state'],files=c['files'],returncode=p.returncode,
                events=ev,stdout=normalize(p.stdout),stderr=normalize(p.stderr),fresh_launch_verified=True)


def validate(c,r):
    assert r['returncode']==c['returncode'],(c['id'],r['returncode'],r['stderr'],r['stdout'])
    begins=[e for e in r['events'] if e['phase']=='begin']
    ends=[e for e in r['events'] if e['phase']=='end']
    assert begins and len(begins)==len(ends)+(r['returncode']!=0),(c['id'],'bad event counts')
    if r['returncode']==0:assert len(ends)==len(c['actions']),c['id']
    for i,e in enumerate(begins):assert (e['action'],e['op'])==(i,c['actions'][i][0]),c['id']
    for i,e in enumerate(ends):assert (e['action'],e['op'])==(i,c['actions'][i][0]),c['id']
    expected=[]
    for i,a in enumerate(c['actions'][:len(begins)]):
        expected.append((i,'begin',a[0]))
        if i<len(ends):expected.append((i,'end',a[0]))
    compare(expected,[(e['action'],e['phase'],e['op']) for e in r['events']],c['id']+'.stages')


def main():
    parser=argparse.ArgumentParser();parser.add_argument('--record',action='store_true');parser.add_argument('--select');parser.add_argument('--no-build',action='store_true');args=parser.parse_args()
    cs=cases(); inv=inventory()
    if args.select:cs=[c for c in cs if c['id']==args.select]
    assert cs
    if args.no_build:exe=BUILD/'probe'; profile=json.loads((HERE/'profile.json').read_text())
    else:exe,profile=compile_probe(prepare())
    def one(c):
        r=execute(c,exe);validate(c,r);return r
    results=[]
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        for i,r in enumerate(pool.map(one,cs)):
            results.append(r)
            if i%25==0:print(f'{i+1}/{len(cs)}: {r["id"]}',flush=True)
    if args.select:
        save(BUILD/'selected.json',results);print('Selected cases passed');return
    output=HERE/'observations.json'
    if args.record:
        save(HERE/'cases.json',cs);save(HERE/'inventory.json',inv);save(HERE/'profile.json',profile);save(output,results)
        print('Recorded',len(results),'complete cases')
    else:
        compare(json.loads((HERE/'cases.json').read_text()),cs,'cases')
        compare(json.loads((HERE/'inventory.json').read_text()),inv,'inventory')
        compare(json.loads(output.read_text()),results,'observations')
        expected_profile=json.loads((HERE/'profile.json').read_text())
        for key in ['source_revision','compiler','compiler_sha256','command','platform','machine','locale','sources','generated','numeric_header_sha256','globals_data_sha256','probe_sha256','runtime_hashes','support_policy','mutation','coverage_instrumentation']:
            compare(expected_profile[key],profile[key],'profile.'+key)
        save(HERE/'regeneration.json',dict(cases=len(cs),events=sum(len(r['events']) for r in results),
             observations_sha256=sha(output.read_bytes()),cases_sha256=sha((HERE/'cases.json').read_bytes()),
             executed_profile=profile,complete_ordered_comparison=True))
        print('PASS:',len(results),'complete exact fresh regenerations')


if __name__=='__main__':main()
