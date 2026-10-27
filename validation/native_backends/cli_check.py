"""Execute ordered native-style selectors and the independent JSON default."""
import argparse, itertools, json, os, re, shutil, subprocess
from pathlib import Path
from check import ROOT, RUNTIME, sha

def banner(out, native, rust, blas, lapack):
    out.mkdir(parents=True,exist_ok=False);records=[]
    expected=[b'* Using LAPACK acceleration for Cholesky decomposition' if lapack else b'* Using slow linear algebra code (compile with LAPACK for speed improvements)',b'* Using BLAS acceleration for matrix mulitplication' if blas else b'* Using slow matrix code (compile with BLAS for speed improvements)']
    for name,exe in [('native',native),('rust',rust)]:
        r=subprocess.run([str(exe)],cwd=out,env={**os.environ,'TEMPO2':str(RUNTIME)},capture_output=True,timeout=30)
        (out/(name+'.stdout')).write_bytes(r.stdout);(out/(name+'.stderr')).write_bytes(r.stderr)
        actual=[line for line in r.stdout.splitlines() if line.startswith(b'* Using ')]
        assert r.returncode==1 and actual==expected,(name,r.returncode,actual)
        records.append({'executable':str(exe),'sha256':sha(exe),'exit':r.returncode,'lines':[x.decode() for x in actual]})
    (out/'receipt.json').write_text(json.dumps({'records':records,'script':sha(__file__),'files':{str(p):sha(p) for p in out.iterdir() if p.is_file()}},indent=2)+'\n')

def run(out, native, rust, par, tim, lapack, qr_default):
    out=out.resolve();out.mkdir(parents=True,exist_ok=False)
    for name,exe in [('native',native),('rust',rust)]:shutil.copy2(exe,out/name)
    identity={str(p):sha(p) for p in [out/'native',out/'rust',par,tim,Path(__file__)]}
    env={**os.environ,'TEMPO2':str(RUNTIME)};records=[]
    selectors=[[],['-qrfit'],['-svdfit'],['-noaccel'],*map(list,itertools.permutations(['-qrfit','-svdfit','-noaccel']))]
    for index,flags in enumerate(selectors):
        mode=2 if qr_default else 1
        for flag in flags:
            if flag=='-noaccel':mode=0
            elif flag=='-qrfit':mode=2
            elif mode!=0:mode=1
        expected='QR' if lapack and mode==2 else 'SVD'
        for engine in ['native','rust']:
            folder=out/f'{index}-{engine}';folder.mkdir()
            cmd=[str(out/engine),'-npsr','1','-nobs','32','-f',str(par),str(tim),*flags]
            r=subprocess.run(cmd,cwd=folder,env=env,capture_output=True,timeout=90)
            (folder/'stdout').write_bytes(r.stdout);(folder/'stderr').write_bytes(r.stderr)
            actual=re.findall(rb'\b(QR|SVD) nparams=',r.stdout+r.stderr)
            records.append({'argv':cmd,'exit':r.returncode,'expected':expected,'actual':[s.decode() for s in actual]})
            assert r.returncode==0 and actual and set(actual)=={expected.encode()},records[-1]
    # JSON timing has always defaulted to acceleration 0, independently of
    # native QR_DEFAULT. The reported engine is the actual shared solver.
    cmd=[str(out/'rust'),'timing','--par',str(par),'--tim',str(tim),'--runtime',str(RUNTIME),'--fit']
    r=subprocess.run(cmd,cwd=out,env=env,capture_output=True,timeout=90)
    (out/'json.stdout').write_bytes(r.stdout);(out/'json.stderr').write_bytes(r.stderr)
    assert r.returncode==0,r.stderr
    obj=json.loads(r.stdout)
    def engines(x):
        if isinstance(x,dict):
            for k,v in x.items():
                if k=='solver_engine':yield v
                else:yield from engines(v)
        elif isinstance(x,list):
            for v in x:yield from engines(v)
    actual=list(engines(obj));assert actual and all('SVD' in x for x in actual),actual
    records.append({'argv':cmd,'exit':r.returncode,'actual':actual})
    assert all(sha(p)==h for p,h in identity.items())
    files={str(p):sha(p) for p in out.rglob('*') if p.is_file()}
    (out/'receipt.json').write_text(json.dumps({'identity':identity,'records':records,'files':files,'failures':0},indent=2)+'\n')
    print(out, len(records), 'passed',flush=True)

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('out',type=Path);p.add_argument('--native',type=Path,required=True);p.add_argument('--rust',type=Path,required=True);p.add_argument('--par',type=Path,required=True);p.add_argument('--tim',type=Path,required=True);p.add_argument('--lapack',action='store_true');p.add_argument('--qr-default',action='store_true');a=p.parse_args()
    run(a.out,a.native.resolve(),a.rust.resolve(),a.par.resolve(),a.tim.resolve(),a.lapack,a.qr_default)
