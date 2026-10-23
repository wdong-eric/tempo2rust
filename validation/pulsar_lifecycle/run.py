"""Fresh executable identity, challenged launches, retained raw evidence and exact comparison."""
import argparse
import base64
import gzip
import json
from pathlib import Path
import re
import shutil
import subprocess
import uuid
from profile import HERE,ROOT,BUILD,ENV,compile_probe,runners,sha,save
from fixtures import cases

def compare(a,b,path='$'):
    if type(a) is not type(b):raise AssertionError(path+': type differs')
    if isinstance(a,dict):
        if a.keys()!=b.keys():raise AssertionError(path+': membership differs')
        for k in a:compare(a[k],b[k],path+'.'+k)
    elif isinstance(a,(list,tuple)):
        if len(a)!=len(b):raise AssertionError(path+': length differs')
        for i in range(len(a)):compare(a[i],b[i],path+f'[{i}]')
    elif a!=b:raise AssertionError(path+': value differs '+repr(a)[:90]+' / '+repr(b)[:90])

def execution_identity(exe,p):
    actual=dict(executable_path=str(exe.resolve()),executable_sha256=sha(exe.read_bytes()),runner_sources=runners())
    for k in ['executable_path','executable_sha256']:compare(p[k],actual[k],'execution.'+k)
    compare(p['stable']['runner_sources'],actual['runner_sources'],'execution.runner_sources')
    src=next(a[2:] for a in p['command'] if a.startswith('-I'))
    deps={n:sha(Path(n.replace('{source}',src)).read_bytes()) for n in p['stable']['header_dependencies']}
    compare(p['stable']['header_dependencies'],deps,'execution.dependencies')
    cc=p['command'][0];compare(p['stable']['compiler_sha256'],sha(Path(cc).read_bytes()),'execution.compiler')
    for key in ['compiler_backend']:
        d=p['stable'][key];compare(d['sha256'],sha(Path(d['path']).read_bytes()),'execution.'+key)
    for n,d in p['stable']['runtime'].items():compare(d['sha256'],sha(Path(d['path']).read_bytes()),'execution.runtime.'+n)
    actual['dependencies_sha256']=sha(json.dumps(deps,sort_keys=True).encode())
    return actual

def identity_gate(expected,p,exe):
    compare(expected['stable'],p['stable'],'stable')
    return execution_identity(exe,p)

def protected():
    hashes=json.loads((HERE/'protected_hashes.json').read_text())
    compare(hashes,{p:sha((ROOT/p).read_bytes()) for p in hashes},'protected')
    for name in ['tempo2','validation/reference']:
        assert not subprocess.check_output(['git','-C',str(ROOT/name),'status','--porcelain']),name

def validate(r):
    assert r['events'] and r['events'][0]['phase']=='profile'
    paths=0
    for e in r['events']:
        seen=set()
        for f in e['state']:
            assert f['path'] not in seen,f['path'];seen.add(f['path']);paths+=1
            if 'symbols' in f:
                assert sum(n for n,_ in f['symbols'])==f['pointer_count'] and all(n>0 for n,_ in f['symbols'])
            if 'runs' not in f:continue
            assert sum(n for n,_ in f['runs'])==f['count']
            prior=None
            for n,h in f['runs']:
                raw=bytes.fromhex(h);assert n>0 and len(raw)==f['unit'] and h==raw.hex() and prior!=h
                prior=h
    return paths

def execute(c,exe,p,retain=None):
    work=BUILD/'cases'/c['id']
    if work.exists():shutil.rmtree(work)
    work.mkdir(parents=True);ep=work/'events.jsonl';nonce=uuid.uuid4().hex
    before=execution_identity(exe,p)
    with (work/'stdout').open('wb') as stdout,(work/'stderr').open('wb') as stderr:
        proc=subprocess.run([str(exe),str(ep),nonce,*c['args']],stdout=stdout,stderr=stderr,env=ENV,timeout=180)
    compare(before,execution_identity(exe,p),'execution')
    assert ep.exists(),c['id']+': missing new events'
    raw=ep.read_bytes();events=[json.loads(s) for s in raw.splitlines()]
    assert events.pop(0)==dict(phase='launch',nonce=nonce),c['id']+': wrong fresh challenge'
    stdout=(work/'stdout').read_bytes();stderr=(work/'stderr').read_bytes()
    src=next(a[2:] for a in p['command'] if a.startswith('-I'))
    def norm(data):
        s=data.decode().replace(src,'{source}')
        # Only declared debug address presentations; scientific/state values untouched.
        if c['classification']=='host_abi':s=re.sub(r'0x[0-9a-f]{16}', '{address}',s)
        return s
    for e in events:e['stdout_position']=len(norm(stdout[:e['stdout_position']]).encode())
    result=dict(id=c['id'],classification=c['classification'],args=c['args'],returncode=proc.returncode,events=events,stdout=norm(stdout),stderr=norm(stderr))
    validate(result)
    if retain:
        bundle=dict(events_base64=base64.b64encode(raw).decode(),stdout_base64=base64.b64encode(stdout).decode(),stderr_base64=base64.b64encode(stderr).decode(),returncode=proc.returncode,nonce=nonce)
        retain.mkdir(parents=True,exist_ok=True)
        (retain/(c['id']+'.json.gz')).write_bytes(gzip.compress(json.dumps(bundle,separators=(',',':')).encode(),mtime=0))
    return result

def reference(case):return json.loads(gzip.decompress((HERE/'expected'/(case+'.json.gz')).read_bytes()))

def regenerate(record=False):
    (HERE/'regeneration.json').unlink(missing_ok=True);protected()
    cs=cases();compare(json.loads((HERE/'cases.json').read_text()),cs,'cases')
    from generate import generate
    compare((HERE/'generated.h').read_text(),generate(),'generated exporter')
    exe,p=compile_probe()
    if not record:identity_gate(json.loads((HERE/'profile.json').read_text()),p,exe)
    results=[]
    for c in cs:
        r=execute(c,exe,p,HERE/'raw' if record else BUILD/'replay_raw');compare(c['returncode'],r['returncode'],c['id']+'.status')
        data=json.dumps(r,separators=(',',':')).encode();ep=HERE/'expected'/(c['id']+'.json.gz')
        if record:ep.parent.mkdir(exist_ok=True);ep.write_bytes(gzip.compress(data,mtime=0))
        else:compare(reference(c['id']),r,c['id'])
        results.append(dict(id=c['id'],events=len(r['events']),field_records=validate(r),sha256=sha(data)))
        print(c['id'],r['returncode'],len(r['events']),flush=True)
    execution=execution_identity(exe,p);protected()
    if record:
        save(HERE/'profile.json',p);save(HERE/'observations.json',results)
    else:
        compare(json.loads((HERE/'observations.json').read_text()),results,'observations');identity_gate(json.loads((HERE/'profile.json').read_text()),p,exe)
        save(HERE/'regeneration.json',dict(status='PASS',cases=len(cs),events=sum(r['events'] for r in results),field_records=sum(r['field_records'] for r in results),profile=p,execution=execution,observations_sha256=sha((HERE/'observations.json').read_bytes())))
    return cs,results,exe,p
if __name__=='__main__':
    ap=argparse.ArgumentParser();ap.add_argument('--record',action='store_true');args=ap.parse_args();regenerate(args.record)
