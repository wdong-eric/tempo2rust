"""Explicit local recording; ordinary execution always rebuilds and never rewrites oracle."""
import argparse
import gzip
import json
import os
from pathlib import Path
import shutil
import subprocess
import time
import uuid
from profile import HERE, BUILD, ENV, sha, save, compile_probe, check_live
from fixtures import catalog, membership
from inventory import inventory
OUT=HERE/'outputs'
REF=OUT/'reference-corrected'
RECEIPT=OUT/'last_execution.json'

def pack(path,data):
    path.parent.mkdir(parents=True,exist_ok=True)
    with gzip.GzipFile(filename=str(path),mode='wb',mtime=0) as f:
        f.write(json.dumps(data,separators=(',',':'),allow_nan=False).encode())

def unpack(path):
    with gzip.open(path,'rt') as f:return json.load(f)

def compare(expected,actual,path='root'):
    if type(expected)!=type(actual):raise AssertionError('type '+path)
    if isinstance(expected,dict):
        if set(expected)!=set(actual):raise AssertionError('membership '+path)
        if '.state' in path and list(expected)!=list(actual):raise AssertionError('field order '+path)
        for k in expected:compare(expected[k],actual[k],path+'.'+k)
    elif isinstance(expected,list):
        if len(expected)!=len(actual):raise AssertionError('length '+path)
        for i,(a,b) in enumerate(zip(expected,actual)):compare(a,b,path+'['+str(i)+']')
    elif expected!=actual:raise AssertionError('value '+path+': '+str(expected)[:100]+' != '+str(actual)[:100])

def projected(raw):
    events=[json.loads(line) for line in raw['events'].splitlines()]
    assert events[0]['phase']=='launch' and events[0]['nonce']==raw['nonce'],'launch challenge'
    events[0]={k:v for k,v in events[0].items() if k!='nonce'}
    return dict(id=raw['id'],events=events,stdout=raw['stdout'],stderr=raw['stderr'],status=raw['status'],final_files=raw['final_files'])

def launch(exe,profile,case):
    check_live(exe,profile)
    work=BUILD/'cases'/case['id']
    if work.exists():shutil.rmtree(work)
    work.mkdir(parents=True)
    for n,b in case['files'].items():
        p=work/n;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(b)
    def arg(a):
        a=str(a)
        return str(work/a[1:]) if a.startswith('@') else a
    actions=[[arg(a) for a in row] for row in case['actions']]
    input_bytes=('\n'.join('\t'.join('x'+a.encode().hex() for a in row) for row in actions)+'\n').encode()
    inp=work/'actions.txt';inp.write_bytes(input_bytes)
    resources={n:sha((work/n).read_bytes()) for n in case['files']}
    assert resources=={n:sha(b) for n,b in case['files'].items()},'fixture identity'
    nonce=uuid.uuid4().hex;eventpath=work/'events.jsonl';eventpath.unlink(missing_ok=True)
    env={**ENV,'TEMPO2':str(work/'unset-root')}
    t=time.time()
    p=subprocess.run([str(exe),str(inp),str(eventpath),nonce],env=env,capture_output=True,timeout=30)
    raw=dict(id=case['id'],nonce=nonce,command=[str(exe),str(inp),str(eventpath),nonce],input_text=input_bytes.decode(),environment={k:env[k] for k in ['LC_ALL','LANG','TEMPO2']},executable_sha256=sha(exe.read_bytes()),input_sha256=sha(input_bytes),resource_sha256=resources,
             events=eventpath.read_text(),stdout=p.stdout.hex(),stderr=p.stderr.hex(),status=p.returncode,
             final_files={n:sha((work/n).read_bytes()) for n in case['files']},elapsed_seconds=time.time()-t)
    assert inp.read_bytes()==input_bytes,'input changed during launch'
    check_live(exe,profile)
    obs=projected(raw)
    if p.returncode!=case['status']:
        pack(OUT/'failures'/(case['id']+'-'+nonce+'.json.gz'),raw)
    assert p.returncode==case['status'],(case['id'],'exit',p.returncode,p.stderr.decode(errors='replace'))
    assert obs['events'][1]['phase']=='initial' and obs['events'][-1]['phase']=='exit','event framing'
    begin=[e for e in obs['events'] if e['phase']=='begin'];end=[e for e in obs['events'] if e['phase']=='end']
    assert [e['action'] for e in begin]==list(range(len(actions))),'action membership'
    assert [e['op'] for e in begin]==[a[0] for a in actions],'action order'
    assert len(end)==len(actions)-(p.returncode!=0),'completion membership'
    return raw

def execute(exe,profile,cases,dest,reference=True):
    dest.mkdir(parents=True,exist_ok=True);records=[]
    for i,case in enumerate(cases):
        raw=launch(exe,profile,case)
        if reference:compare(projected(unpack(REF/(case['id']+'.json.gz'))),projected(raw),case['id'])
        path=dest/(case['id']+'.json.gz');pack(path,raw)
        records.append(dict(id=case['id'],path=str(path),sha256=sha(path.read_bytes()),events=len(raw['events'].splitlines()),status=raw['status']))
        if i%10==0:print('executed',i+1,'/',len(cases),case['id'],flush=True)
    for record in records:
        path=Path(record['path'])
        assert sha(path.read_bytes())==record['sha256'], 'saved artifact changed or collided: '+record['id']
        assert unpack(path)['id']==record['id'], 'saved case identity'
    return records

def main():
    RECEIPT.unlink(missing_ok=True)
    parser=argparse.ArgumentParser();parser.add_argument('--record',action='store_true');parser.add_argument('--select');args=parser.parse_args()
    cases=catalog();meta=dict(cases=membership(cases),inventory=inventory())
    if args.record:
        assert not args.select,'record must be complete'
        assert not (REF/'oracle.json').exists(),'existing oracle is immutable; choose an explicitly new local evidence directory in source'
    else:
        frozen=json.loads((REF/'oracle.json').read_text());compare(frozen['inputs'],meta,'inputs')
    exe,profile=compile_probe()
    if not args.record:compare(frozen['profile'],profile['stable'],'profile')
    selected=[c for c in cases if c['id']==args.select] if args.select else cases
    assert selected,'no selected case'
    records=execute(exe,profile,selected,REF if args.record else OUT/'replay',not args.record)
    if args.record:save(REF/'oracle.json',dict(inputs=meta,profile=profile['stable']))
    else:
        assert json.loads((REF/'oracle.json').read_text())==frozen,'oracle changed'
    receipt=dict(mode='record' if args.record else 'verify',complete=not args.select,profile=profile,cases=records,
                 totals=dict(cases=len(records),events=sum(r['events'] for r in records)),oracle_sha256=sha((REF/'oracle.json').read_bytes()))
    save(OUT/('recording.json' if args.record else 'last_execution.json'),receipt)
    print('PASS',receipt['totals'],profile['executable_sha256'],flush=True)
if __name__=='__main__':main()
