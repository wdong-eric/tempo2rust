#!/usr/bin/env python3
"""Fresh native build and complete exact ordered comparison; --record is explicit."""
import argparse,gzip,json,re,shutil,subprocess,uuid
from pathlib import Path
from profile import HERE,BUILD,ENV,PRIMARY,compile_probe,save,sha,runners
from fixtures import cases
from inventory import inventory
from protected import snapshot
INITIAL=None

def compare(a,b,path='$'):
    if type(a) is not type(b):raise AssertionError(path+': type')
    if isinstance(a,dict):
        if a.keys()!=b.keys():raise AssertionError(path+': membership')
        for k in a:compare(a[k],b[k],path+'.'+k)
    elif isinstance(a,list):
        if len(a)!=len(b):raise AssertionError(path+': length')
        for i,(x,y) in enumerate(zip(a,b)):compare(x,y,path+f'[{i}]')
    elif a!=b:raise AssertionError(path+': '+repr(a)[:140]+' != '+repr(b)[:140])

def execution_identity(exe,p,full=False):
    actual=dict(path=str(exe.resolve()),sha256=sha(exe.read_bytes()),runners=runners())
    compare(str(Path(p['executable_path']).resolve()),actual['path'],'execution.path')
    compare(p['executable_sha256'],actual['sha256'],'execution.executable')
    compare(p['stable']['runner_sources'],actual['runners'],'execution.runners')
    if full:
        st=p['stable'];src=Path(p['command'][p['command'].index('-DHAVE_CONFIG_H')+1][2:])
        compare(st['compiler_sha256'],sha(Path(p['command'][0]).read_bytes()),'current.compiler')
        compare(st['compiler_backend']['sha256'],sha(Path(st['compiler_backend']['path']).read_bytes()),'current.backend')
        for section in ['sources','generated']:
            for n,h in st[section].items():compare(h,sha((src/n).read_bytes()),'current.'+n)
        for n,h in st['runtime_fixtures'].items():compare(h,sha((BUILD/'runtime'/n).read_bytes()),'current.runtime_fixture.'+n)
        for n,v in st['runtime'].items():compare(v['sha256'],sha(Path(v['path']).read_bytes()),'current.runtime.'+n)
        for n,h in st['header_dependencies'].items():compare(h,sha(Path(n.replace('{source}',str(src))).read_bytes()),'current.header.'+n)
    return actual

def check_identity(expected,p,exe):
    compare(expected['stable'],p['stable'],'profile')
    return execution_identity(exe,p,True)

def normalized(s,p):
    src=next(x[2:] for x in p['command'] if x.startswith('-I'))
    s=s.replace(src,'{source}').replace(str(BUILD),'{build}').replace(str(HERE),'{par_specialized}')
    return s

def runtime():
    root=BUILD/'runtime'
    if root.exists():shutil.rmtree(root)
    (root/'clock').mkdir(parents=True)
    for name,data in {'a.clk':'# A B\n50000 1\n60000 3\n','b.clk':'# B C\n50000 4\n60000 6\n','direct.clk':'# A C\n50000 20\n60000 22\n'}.items():(root/'clock'/name).write_text(data)
    (root/'observatory').mkdir()
    (root/'observatory'/'observatories.dat').write_text('-4554231.5 2816759.1 -3454036.3 Parkes PKS A\n')
    (root/'observatory'/'aliases').write_text('PKS Parkes\n')
    return {str(p.relative_to(root)):sha(p.read_bytes()) for p in sorted(root.rglob('*')) if p.is_file()}

def execute(c,exe,p):
    execution_identity(exe,p)
    work=BUILD/'cases'/c['id']
    if work.exists():shutil.rmtree(work)
    work.mkdir(parents=True)
    for n,h in c['files'].items():
        f=work/n;f.parent.mkdir(parents=True,exist_ok=True);f.write_bytes(bytes.fromhex(h))
    actions=[[v.replace('{case}',str(work)) for v in a] for a in c['actions']]
    ap=work/'actions.tsv';ap.write_text('\n'.join('\t'.join('x'+v.encode().hex() for v in a) for a in actions)+'\n')
    input_hashes={n:sha((work/n).read_bytes()) for n in ['actions.tsv',*c['files']]}
    ep=work/'events.jsonl';assert not ep.exists();nonce=uuid.uuid4().hex
    execution_identity(exe,p)
    proc=subprocess.run([str(exe),str(ap),str(ep),nonce],capture_output=True,env={**ENV,'TEMPO2':str(BUILD/'runtime')},timeout=90)
    execution_identity(exe,p)
    compare(input_hashes,{n:sha((work/n).read_bytes()) for n in input_hashes},'executed_inputs')
    if not ep.exists():raise AssertionError(c['id']+': missing events')
    events=[json.loads(normalized(s,p)) for s in ep.read_text().splitlines()]
    assert events.pop(0)==dict(phase='launch',nonce=nonce),c['id']+': stale launch'
    global INITIAL
    initial=events[0].pop('state')
    if INITIAL is None:INITIAL=initial
    if p['mutation'] is None:compare(INITIAL,initial,'complete_initial_state')
    events[0]['state_sha256']=sha(json.dumps(initial,sort_keys=True,separators=(',',':')).encode())
    result=dict(id=c['id'],classification=c['classification'],actions=c['actions'],files=c['files'],returncode=proc.returncode,events=events,
        stdout=normalized(proc.stdout.decode('latin1'),p),stderr=normalized(proc.stderr.decode('latin1'),p),outputs={n:(work/n).read_bytes().hex() if (work/n).exists() else None for n in c['outputs']},fresh_launch_verified=True)
    # Keep actual raw launch receipt/streams, while comparison only normalizes declared paths/timing.
    (work/'stdout.bin').write_bytes(proc.stdout);(work/'stderr.bin').write_bytes(proc.stderr)
    save(work/'receipt.json',dict(command=[str(exe),str(ap),str(ep),nonce],environment=dict(LC_ALL='C',LANG='C',TEMPO2=str(BUILD/'runtime')),inputs_sha256=input_hashes,nonce=nonce,executable_sha256=p['executable_sha256'],returncode=proc.returncode,events_sha256=sha(ep.read_bytes()),stdout_sha256=sha(proc.stdout),stderr_sha256=sha(proc.stderr)))
    return result

def validate(c,r):
    assert r['returncode']==c['returncode'],(c['id'],r['returncode'],r['stdout'],r['stderr'])
    ev=r['events'];assert ev[0]['phase']=='initial' and ev[-1]['phase']=='exit_state',c['id']
    inner=[e for e in ev[1:-1] if e['phase'] in ['begin','end']];expected=[]
    for i,a in enumerate(c['actions']):
        expected.append((i,'begin',a[0]))
        if c['returncode'] and i==len(c['actions'])-1:break
        expected.append((i,'end',a[0]))
    compare(expected,[(e['action'],e['phase'],e['op']) for e in inner],c['id']+'.events')
    for ai,a in enumerate(c['actions']):
        if a[0]!='dispatch':continue
        parts=[e for e in ev if e.get('action')==ai and e['phase'].startswith('dispatch_')]
        expected=[];token=0
        for e in parts:
            expected.append((token,'dispatch_begin' if len(expected)%2==0 else 'dispatch_end'))
            if e['phase']=='dispatch_end':token+=1
        compare(expected,[(e['token'],e['phase']) for e in parts],c['id']+'.dispatch_order')
        if c['returncode']==0 or ai<len(c['actions'])-1:assert len(parts)%2==0,c['id']+': incomplete dispatch'
        else:assert len(parts)%2==1,c['id']+': missing terminating dispatch begin'
    for n,v in r['outputs'].items():assert v is not None,(c['id'],'missing output',n)

def read_observations():return json.loads(gzip.decompress((HERE/'observations.json.gz').read_bytes()))

def write_observations(results):(HERE/'observations.json.gz').write_bytes(gzip.compress(json.dumps(results,separators=(',',':')).encode(),mtime=0))

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--record',action='store_true');ap.add_argument('--select');a=ap.parse_args()
    receipt=HERE/'regeneration.json'
    if not a.record and not a.select:
        receipt.unlink(missing_ok=True)
        (HERE/'replay.raw.json.gz').unlink(missing_ok=True)
        if (HERE/'replay_raw').exists():shutil.rmtree(HERE/'replay_raw')
    compare(json.loads((HERE/'protected.json').read_text()),snapshot(),'protected_before')
    cs=cases();inv=inventory();rt=runtime()
    if a.select:cs=[c for c in cs if c['id']==a.select]
    assert cs
    exe,p=compile_probe();p['stable']['runtime_fixtures']=rt
    if a.record:execution_identity(exe,p,True)
    else:check_identity(json.loads((HERE/'profile.json').read_text()),p,exe)
    results=[];raw=[]
    rawdir=HERE/('reference_raw' if a.record else 'replay_raw')
    if not a.select:rawdir.mkdir(exist_ok=True)
    for i,c in enumerate(cs):
        r=execute(c,exe,p);validate(c,r);results.append(r)
        work=BUILD/'cases'/c['id']
        if not a.select:
            payload=dict(id=c['id'],receipt=json.loads((work/'receipt.json').read_text()),events_jsonl=(work/'events.jsonl').read_text(),stdout_hex=(work/'stdout.bin').read_bytes().hex(),stderr_hex=(work/'stderr.bin').read_bytes().hex())
            rawlines=payload['events_jsonl'].splitlines(keepends=True);initial_line=rawlines.pop(1);ih=sha(initial_line.encode());idpath=HERE/'raw_initial'/(ih+'.json.gz');idpath.parent.mkdir(exist_ok=True)
            if not idpath.exists():idpath.write_bytes(gzip.compress(initial_line.encode(),mtime=0))
            payload['events_jsonl']=''.join(rawlines);payload['initial_line']=dict(index=1,path=str(idpath.relative_to(HERE)),sha256=ih)
            target=rawdir/(c['id']+'.json.gz');target.write_bytes(gzip.compress(json.dumps(payload,separators=(',',':')).encode(),mtime=0))
            raw.append(dict(id=c['id'],path=str(target.relative_to(HERE)),sha256=sha(target.read_bytes())))
        if i%10==0:print(i+1,len(cs),c['id'],flush=True)
    compare(json.loads((HERE/'protected.json').read_text()),snapshot(),'protected_after')
    execution_identity(exe,p,True);compare(rt,{str(f.relative_to(BUILD/'runtime')):sha(f.read_bytes()) for f in sorted((BUILD/'runtime').rglob('*')) if f.is_file()},'runtime fixture identity')
    if a.select:
        if not a.record:
            gold={r['id']:r for r in read_observations()}
            for r in results:compare(gold[r['id']],r,'selected.'+r['id'])
        save(BUILD/'selected.json',results);return
    if a.record:
        (HERE/'initial_state.json.gz').write_bytes(gzip.compress(json.dumps(INITIAL,sort_keys=True,separators=(',',':')).encode(),mtime=0))
        save(HERE/'profile.json',p);save(HERE/'cases.json',cs);save(HERE/'inventory.json',inv);write_observations(results)
        (HERE/'reference.raw.json.gz').write_bytes(gzip.compress(json.dumps(dict(profile=p,cases=raw),separators=(',',':')).encode(),mtime=0))
        print('RECORDED',len(cs))
    else:
        compare(json.loads(gzip.decompress((HERE/'initial_state.json.gz').read_bytes())),INITIAL,'frozen_initial_state')
        for n,data in [('cases',cs),('inventory',inv)]:compare(json.loads((HERE/(n+'.json')).read_text()),data,n)
        compare(read_observations(),results,'observations')
        (HERE/'replay.raw.json.gz').write_bytes(gzip.compress(json.dumps(dict(profile=p,cases=raw),separators=(',',':')).encode(),mtime=0))
        save(receipt,dict(raw_replay_sha256=sha((HERE/'replay.raw.json.gz').read_bytes()),cases=len(cs),events=sum(len(r['events']) for r in results),executed_profile=p,execution_identity=execution_identity(exe,p,True),complete_ordered_comparison=True,observations_sha256=sha((HERE/'observations.json.gz').read_bytes())))
        print('PASS complete fresh exact replay',len(cs))
if __name__=='__main__':main()
