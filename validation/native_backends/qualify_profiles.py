"""Rebuild and execute isolated Cargo/native profiles; artifacts stay in build/."""
import json, os, shutil, subprocess, sys
from concurrent.futures import ThreadPoolExecutor
from decimal import getcontext
from pathlib import Path
from check import HERE, ROOT, run, sha
from cli_check import run as cli, banner
from native_build import build

if __name__=='__main__':
    base=Path(sys.argv[1]).resolve() if len(sys.argv)>1 else HERE/'build/profiles-published';base.mkdir(exist_ok=False)
    provider=HERE/'build/provider';eph=HERE/'build/inpop17a_TDB_m100_p100_tt.dat'
    env={**os.environ,'CALCEPH_DIR':str(provider),'TEMPO2_ACCELERATE':'1'};commands=[]
    profiles=[('disabled','',False),('blas','blas',False),('lapack','lapack',False),('calceph','calceph',False),('combined','calceph,lapack',False),('combined','calceph,lapack',True),('disabled','',True),('qr-default','calceph,lapack,qr-default',False)]
    jobs=[]
    for profile,features,release in profiles:
        label=profile+('-release' if release else '-debug');out=base/label;out.mkdir()
        native=build(out/'native',profile,provider)
        args=['cargo','--offline','--manifest-path',str(ROOT/'rust/Cargo.toml')]
        extra=(['--features',features] if features else [])+(['--release'] if release else [])
        for action,targets in [('test',['--test','native_backends','--','--test-threads=1']),('build',['--example','native_backends','--bins'])]:
            cmd=[args[0],action,*args[1:],*extra,*targets]
            with (out/(action+'.log')).open('wb') as log:r=subprocess.run(cmd,cwd=ROOT,env=env,stdout=log,stderr=subprocess.STDOUT)
            commands.append({'argv':cmd,'exit':r.returncode});assert r.returncode==0,cmd
        target=ROOT/'rust/target'/('release' if release else 'debug')
        shutil.copy2(target/'examples/native_backends',out/'rust-probe');shutil.copy2(target/'tempo2-rust',out/'rust-cli')
        banner(out/'banner',native/'tempo2',out/'rust-cli',profile in ('blas','lapack','combined','qr-default'),profile in ('lapack','combined','qr-default'))
        jobs.append((profile,label,out,native))
        print('BUILT',label,flush=True)
    def execute(job):
        getcontext().prec=70
        profile,label,out,native=job
        if profile!='qr-default':
            result=run(out/'paired',native/'probe',out/'rust-probe',profile in ('calceph','combined'),profile in ('blas','lapack','combined'),profile in ('lapack','combined'),eph)
            assert result['failures']==0 and not result['changed_artifacts'],label
        if profile in ('disabled','combined','qr-default'):
            inputs=(out/'paired/inputs') if profile!='qr-default' else HERE/'build/paired-combined-debug/inputs'
            cli(out/'cli',native/'tempo2',out/'rust-cli',inputs/'calceph.par',inputs/'pks.tim',profile!='disabled',profile=='qr-default')
        print('PROFILE PASSED',label,flush=True)
    with ThreadPoolExecutor(max_workers=2) as pool:
        list(pool.map(execute,jobs))
    (base/'receipt.json').write_text(json.dumps({'commands':commands,'script':sha(__file__)},indent=2)+'\n')
