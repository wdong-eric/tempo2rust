#!/usr/bin/env python3
"""Fresh native build and complete exact ordered comparison; --record is explicit."""
import argparse,gzip,json,re,shutil,subprocess,uuid
from pathlib import Path
from profile import HERE,BUILD,ENV,PRIMARY,compile_probe,save,sha,runners
from fixtures import cases
from inventory import inventory

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
    s=s.replace(src,'{source}').replace(str(BUILD),'{build}').replace(str(HERE),'{tim_io}')
    return re.sub(r'(\[\{source\}/sortToAs.C:\d+\] T=)[0-9.]+( s: )',r'\1{cpu_seconds}\2',s)

def runtime():
    d=BUILD/'runtime'/'observatory';d.mkdir(parents=True,exist_ok=True)
    (d/'observatories.dat').write_text('-4554231.5 2816759.1 -3454036.3 Parkes PKS UTC(PKS)\n3822625.0 -154105.0 5086486.0 Jodrell JBO UTC(JBO)\n')
    (d/'aliases').write_text('PKS Parkes pks_alias\nJBO Jodrell jbo_alias\n')
    return {str(p.relative_to(BUILD/'runtime')):sha(p.read_bytes()) for p in sorted((BUILD/'runtime').rglob('*')) if p.is_file()}

def execute(c,exe,p):
    execution_identity(exe,p)
    work=BUILD/'cases'/c['id']
    if work.exists():shutil.rmtree(work)
    work.mkdir(parents=True)
    for n,h in c['files'].items():
        f=work/n;f.parent.mkdir(parents=True,exist_ok=True);f.write_bytes(bytes.fromhex(h))
    actions=[[v.replace('{case}',str(work)) for v in a] for a in c['actions']]
    ap=work/'actions.tsv';ap.write_text('\n'.join('\t'.join('x'+v.encode().hex() for v in a) for a in actions)+'\n')
    ep=work/'events.jsonl';assert not ep.exists();nonce=uuid.uuid4().hex
    execution_identity(exe,p)
    proc=subprocess.run([str(exe),str(ap),str(ep),nonce],capture_output=True,env={**ENV,'TEMPO2':str(BUILD/'runtime')},timeout=30)
    execution_identity(exe,p)
    if not ep.exists():raise AssertionError(c['id']+': missing events')
    events=[json.loads(normalized(s,p)) for s in ep.read_text().splitlines()]
    assert events.pop(0)==dict(phase='launch',nonce=nonce),c['id']+': stale launch'
    result=dict(id=c['id'],classification=c['classification'],actions=c['actions'],files=c['files'],returncode=proc.returncode,events=events,
        stdout=normalized(proc.stdout.decode('latin1'),p),stderr=normalized(proc.stderr.decode('latin1'),p),outputs={n:(work/n).read_bytes().hex() if (work/n).exists() else None for n in c['outputs']},fresh_launch_verified=True)
    # Keep actual raw launch receipt/streams, while comparison only normalizes declared paths/timing.
    (work/'stdout.bin').write_bytes(proc.stdout);(work/'stderr.bin').write_bytes(proc.stderr)
    save(work/'receipt.json',dict(nonce=nonce,executable_sha256=p['executable_sha256'],returncode=proc.returncode,events_sha256=sha(ep.read_bytes()),stdout_sha256=sha(proc.stdout),stderr_sha256=sha(proc.stderr)))
    return result

def validate(c,r):
    assert r['returncode']==c['returncode'],(c['id'],r['returncode'],r['stdout'],r['stderr'])
    ev=r['events'];assert ev[0]['phase']=='initial' and ev[-1]['phase']=='exit_state',c['id']
    inner=ev[1:-1];expected=[]
    for i,a in enumerate(c['actions']):
        expected.append((i,'begin',a[0]))
        if c['returncode'] and i==len(c['actions'])-1:break
        expected.append((i,'end',a[0]))
    compare(expected,[(e['action'],e['phase'],e['op']) for e in inner],c['id']+'.events')
    for n,v in r['outputs'].items():assert v is not None,(c['id'],'missing output',n)

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--record',action='store_true');ap.add_argument('--select');a=ap.parse_args()
    receipt=HERE/'regeneration.json'
    if not a.record and not a.select:receipt.unlink(missing_ok=True)
    cs=cases();inv=inventory();rt=runtime()
    if a.select:cs=[c for c in cs if c['id']==a.select]
    assert cs
    exe,p=compile_probe();p['stable']['runtime_fixtures']=rt
    if a.record:execution_identity(exe,p,True)
    else:check_identity(json.loads((HERE/'profile.json').read_text()),p,exe)
    results=[];raw=[]
    for i,c in enumerate(cs):
        r=execute(c,exe,p);validate(c,r);results.append(r)
        work=BUILD/'cases'/c['id']
        raw.append(dict(id=c['id'],receipt=json.loads((work/'receipt.json').read_text()),events_jsonl=(work/'events.jsonl').read_text(),stdout_hex=(work/'stdout.bin').read_bytes().hex(),stderr_hex=(work/'stderr.bin').read_bytes().hex()))
        if i%10==0:print(i+1,len(cs),c['id'],flush=True)
    execution_identity(exe,p,True);compare(rt,{str(f.relative_to(BUILD/'runtime')):sha(f.read_bytes()) for f in sorted((BUILD/'runtime').rglob('*')) if f.is_file()},'runtime fixture identity')
    if a.select:
        if not a.record:
            gold={r['id']:r for r in json.loads((HERE/'observations.json').read_text())}
            for r in results:compare(gold[r['id']],r,'selected.'+r['id'])
        save(BUILD/'selected.json',results);return
    if a.record:
        save(HERE/'profile.json',p);save(HERE/'cases.json',cs);save(HERE/'inventory.json',inv);save(HERE/'observations.json',results)
        (HERE/'reference.raw.json.gz').write_bytes(gzip.compress(json.dumps(dict(profile=p,cases=raw),separators=(',',':')).encode(),mtime=0))
        print('RECORDED',len(cs))
    else:
        for n,data in [('cases',cs),('inventory',inv),('observations',results)]:compare(json.loads((HERE/(n+'.json')).read_text()),data,n)
        (HERE/'replay.raw.json.gz').write_bytes(gzip.compress(json.dumps(dict(profile=p,cases=raw),separators=(',',':')).encode(),mtime=0))
        save(receipt,dict(raw_replay_sha256=sha((HERE/'replay.raw.json.gz').read_bytes()),cases=len(cs),events=sum(len(r['events']) for r in results),executed_profile=p,execution_identity=execution_identity(exe,p,True),complete_ordered_comparison=True,observations_sha256=sha((HERE/'observations.json').read_bytes())))
        print('PASS complete fresh exact replay',len(cs))
if __name__=='__main__':main()
