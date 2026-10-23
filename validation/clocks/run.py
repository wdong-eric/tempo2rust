#!/usr/bin/env python3
"""Execute pinned source actions; default checks, --record replaces only J goldens."""
import argparse
import copy
import json
import os
import re
import shutil
import subprocess
from pathlib import Path
from profile import HERE, ROOT, BUILD, SOURCE, ENV, prepare, blob, sha, save
from fixtures import make_cases

OUTPUTS = HERE/'outputs'
REAL = ROOT/'tempo2/T2runtime'


def compile_probe(src, cc, flags, name='probe', mutations=None):
    target=BUILD/name
    if mutations:
        staged=BUILD/(name+'_source')
        shutil.copytree(src,staged,dirs_exist_ok=True)
        src=staged
        for filename, old, new in mutations or []:
            p=src/filename;s=p.read_text();assert s.count(old)==1,(filename,old,s.count(old));p.write_text(s.replace(old,new))
    # Verbatim data prefix and CVSdisplayVersion; no unrelated timing routines or A2 changes.
    global_source=blob('global.C')
    globals_data=global_source[:global_source.index(b'void extra_delays')] + global_source[global_source.index(b'void CVSdisplayVersion'):] 
    (src/'global_data.C').write_bytes(globals_data)
    actual_flags=[f for f in flags if not f.startswith('-I')]+['-I'+str(src)]
    dependencies=['observatory.C','TKlog.C','TKlongdouble.C','tempo2Util.C','global_data.C']
    cmd=[cc,*actual_flags,str(HERE/'probe.cc'),*[str(src/n) for n in dependencies],
         '-lquadmath','-Wl,-map,'+str(BUILD/(name+'.map')),'-o',str(target)]
    completed=subprocess.run(cmd,text=True,capture_output=True,env=ENV)
    (BUILD/(name+'.compile.log')).write_text(completed.stdout+completed.stderr)
    if completed.returncode:raise RuntimeError(completed.stderr)
    profile=dict(command=cmd,compiler=subprocess.check_output([cc,'--version'],text=True).splitlines()[0],
                 numeric_header_sha256=sha((src/'TKlongdouble.h').read_bytes()),
                 executable_sha256=sha(target.read_bytes()),link_map_sha256=sha((BUILD/(name+'.map')).read_bytes()),
                 globals_prefix_sha256=sha(globals_data), globals_source_slices=['global.C:1-64', 'global.C:176-200'],
                 compile_log_sha256=sha((completed.stdout+completed.stderr).encode()),
                 dependencies={n:sha((src/n).read_bytes()) for n in dependencies},
                 primary={n:sha((src/n).read_bytes()) for n in ('clkcorr.C','tabulatedfunction.C','toa2utc.C')},
                 profile='GCC binary128 input, binary32 weights, binary64 tables',
                 compiler_sha256=sha(Path(cc).read_bytes()),
                 table_value_instrumentation='Macro renames raw TabulatedFunction_getValue to J_Table_getValue; forwarding function records binary64 arguments/results after return, then returns the unchanged value.',
                 dynamic_libraries=subprocess.check_output(['otool','-L',str(target)],text=True).splitlines()[1:],
                 runtime_hashes={n:sha(Path(subprocess.check_output([cc,'-print-file-name='+n],text=True).strip()).read_bytes()) for n in ('libquadmath.0.dylib','libstdc++.6.dylib','libgcc_s.1.1.dylib')})
    return target, profile


def normalize(text):
    # Only paths change; line numbers, messages, spacing and event order are exact.
    return text.replace(str(BUILD),'{build}').replace(str(REAL),'{real}').replace(str(HERE),'{clocks}')


def resource_manifest(cases):
    synthetic={c['id']:{p:dict(sha256=sha(s.encode()),size=len(s.encode())) for p,s in c['files'].items()} for c in cases}
    real={}
    for p in sorted([*REAL.glob('clock/*.clk'),*REAL.glob('observatory/*.dat'),REAL/'observatory/aliases']):
        relative=p.relative_to(ROOT/'tempo2').as_posix();data=p.read_bytes()
        assert data==blob(relative),relative
        real[relative]=dict(sha256=sha(data),size=len(data))
    return dict(synthetic=synthetic,real=real)


def execute(case, executable):
    work=BUILD/'cases'/case['id']
    if work.exists():shutil.rmtree(work)
    work.mkdir(parents=True)
    for name,data in case['files'].items():
        p=work/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(data)
    actions=[[str(v).replace('{case}',str(work)).replace('{real}',str(REAL)) for v in a] for a in case['actions']]
    actions_path=work/'actions.tsv'
    actions_path.write_text('\n'.join('\t'.join('x'+v.encode().hex() for v in a) for a in actions)+'\n')
    events_path=work/'events.jsonl'
    command=[str(executable),str(actions_path),str(events_path)]
    env=ENV
    try:
        proc=subprocess.run(command,text=True,capture_output=True,env=env,timeout=2 if case['classification']=='bounded' else 15)
        code=proc.returncode;stdout=proc.stdout;stderr=proc.stderr;timeout=False
    except subprocess.TimeoutExpired as e:
        code=None;stdout=e.stdout or b'';stderr=e.stderr or b'';timeout=True
        if isinstance(stdout,bytes):stdout=stdout.decode(errors='replace')
        if isinstance(stderr,bytes):stderr=stderr.decode(errors='replace')
    events=[json.loads(normalize(s)) for s in events_path.read_text().splitlines()] if events_path.exists() else []
    stdout=normalize(stdout);stderr=normalize(stderr)
    result=dict(id=case['id'],classification=case['classification'],actions=case['actions'],
                command=normalize(' '.join(command)),returncode=code,timeout=timeout,events=events,stdout=stdout,stderr=stderr)
    return result


def compare(expected,actual,path='$'):
    """Complete recursive comparison: no zip truncation, NaN coercion or ignored fields."""
    if type(expected) is not type(actual):raise AssertionError(f'{path}: type mismatch')
    if isinstance(expected,dict):
        if expected.keys()!=actual.keys():raise AssertionError(f'{path}: field membership {expected.keys() ^ actual.keys()}')
        for k in expected:compare(expected[k],actual[k],path+'.'+k)
    elif isinstance(expected,list):
        if len(expected)!=len(actual):raise AssertionError(f'{path}: lengths {len(expected)} != {len(actual)}')
        for i in range(len(expected)):compare(expected[i],actual[i],f'{path}[{i}]')
    elif expected!=actual:raise AssertionError(f'{path}: {str(expected)[:180]!r} != {str(actual)[:180]!r}')


def validate_case(case,result):
    assert result['id']==case['id']
    assert result['returncode']==(0 if case['classification']=='ordinary' else 1),(case['id'],result['returncode'],result['stderr'])
    expected=[]
    for i,a in enumerate(case['actions'][:case['last_action']+1]):
        expected.append((i,'begin',a[0]))
        if result['returncode']==0 or i<case['last_action']:expected.append((i,'end',a[0]))
    compare(expected,[(e['event'],e['phase'],e['operation']) for e in result['events']],case['id']+'.events')
    for e in result['events']:
        assert set(e)==({'event','phase','operation'} if e['phase']=='begin' else {'event','phase','operation','result','clock_state','table_queries'})
        if e['phase']=='end':assert set(e['clock_state'])=={'initialized','functions','sequences','warnings','errors'}


def main():
    parser=argparse.ArgumentParser();parser.add_argument('--record',action='store_true');parser.add_argument('--case');parser.add_argument('--skip-mutations',action='store_true')
    args=parser.parse_args()
    src,cc,flags,source_profile=prepare()
    native,native_profile=compile_probe(src,cc,flags)
    cases=make_cases()
    if args.case:cases=[c for c in cases if c['id']==args.case];assert cases
    ordinary=[];isolated=[]
    for c in cases:
        if c['classification'] in ('undefined','bounded'):
            isolated.append(dict(id=c['id'], classification=c['classification'], actions=c['actions'], status='unrun', reason='Central reported an automated possible-cybersecurity-risk flag without naming the experiment; invalid-access and cycle experiments are withheld, with no alternate execution path attempted.'))
            print(c['id'], 'UNRUN (central safety status)',flush=True)
            continue
        r=execute(c,native);validate_case(c,r)
        replay=execute(c,native);validate_case(c,replay);compare(r,replay,c['id']+'.fresh_replay')
        (isolated if c['classification'] in ('undefined','bounded') else ordinary).append(r)
        print(c['id'], 'PASS',flush=True)
    payload=dict(cases=ordinary);failures=dict(cases=isolated)
    save(BUILD/'reference.json',payload);save(BUILD/'isolated.json',failures)
    if not args.case:
        manifest=resource_manifest(cases)
        profiles=dict(source=source_profile,native=native_profile,
                      harness_sha256=sha((HERE/'probe.cc').read_bytes()), fixtures_sha256=sha((HERE/'fixtures.py').read_bytes()),
                      cases_sha256=sha(json.dumps(cases,sort_keys=True).encode()),resource_manifest=manifest)
        save(BUILD/'manifest.json',profiles)
        if args.record:
            save(OUTPUTS/'reference.json',payload);save(OUTPUTS/'isolated.json',failures);save(HERE/'cases.json',cases);save(OUTPUTS/'manifest.json',profiles)
        else:
            compare(json.loads((OUTPUTS/'reference.json').read_text()),payload)
            compare(json.loads((OUTPUTS/'isolated.json').read_text()),failures)
            compare(json.loads((HERE/'cases.json').read_text()),cases)
            compare(json.loads((OUTPUTS/'manifest.json').read_text())['resource_manifest'],manifest)
            recorded=json.loads((OUTPUTS/'manifest.json').read_text())
            for key in ('harness_sha256','fixtures_sha256','cases_sha256'):compare(recorded[key],profiles[key],key)
            compare(recorded['source'],profiles['source'],'source_profile')
            for key in ('command','compiler','compiler_sha256','numeric_header_sha256','globals_prefix_sha256','globals_source_slices','dependencies','primary','profile','table_value_instrumentation','dynamic_libraries','runtime_hashes'):
                compare(recorded['native'][key],profiles['native'][key],'native_profile.'+key)
        save(OUTPUTS/'last_execution.json',dict(mode='record' if args.record else 'check',native=native_profile,source=source_profile,harness_sha256=sha((HERE/'probe.cc').read_bytes()),cases=len(cases),executed_cases=len(ordinary),withheld_cases=len(isolated),full_ordered_fresh_replay=True))
        print('PASS: complete ordered artifacts, fresh-process replay, inputs and source profile')
    if not args.case and not args.skip_mutations:
        from verify import verify
        checks=verify(cases,payload,failures,src,cc,flags)
        save(OUTPUTS/'checks.json',checks)


if __name__=='__main__':main()
