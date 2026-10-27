#!/usr/bin/env python3
"""Execute selected explicit feature/provider profiles with the frozen AI gates."""
import argparse
import os
from pathlib import Path
import platform
import shutil
import sys
from common import ROOT, Run, sha

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','a2','provider','candidate','native-repository','target']:
        p.add_argument('--'+name,type=Path,required=True)
    features={'disabled':'','blas':'blas','lapack':'lapack','calceph':'calceph','combined':'calceph,lapack','qr-default':'calceph,lapack,qr-default'}
    p.add_argument('--profiles',nargs='+',choices=features,default=list(features))
    a=p.parse_args();run=Run(a.out);out=run.out;records=[]
    env=dict(os.environ,CALCEPH_DIR=str(a.provider.resolve()/'prefix'),VECLIB_MAXIMUM_THREADS='1',OPENBLAS_NUM_THREADS='1')
    env['TEMPO2_ACCELERATE' if platform.system()=='Darwin' else 'TEMPO2_OPENBLAS']='1'
    manifest=a.candidate.resolve()/'rust/Cargo.toml'
    for profile in a.profiles:
        core=out/('native-'+profile)
        run.run([sys.executable,ROOT/'validation/release/native_core.py','--out',core,
            '--native-repository',a.native_repository.resolve(),'--a2',a.a2.resolve(),
            '--provider',a.provider.resolve()/'prefix','--profile',profile],profile+'-native',env=env)
        flags=['--offline','--locked','--release','--manifest-path',manifest,'--target-dir',a.target.resolve()]
        if features[profile]:flags+=['--features',features[profile]]
        run.run(['cargo','test',*flags,'--test','native_backends'],profile+'-tests',env=env)
        run.run(['cargo','build',*flags,'--example','native_backends'],profile+'-build',env=env)
        rust=out/('rust-'+profile);shutil.copy2(a.target.resolve()/'release/examples/native_backends',rust)
        run.run([sys.executable,ROOT/'validation/release/check_backends.py','--out',out/('paired-'+profile),
            '--source-root',a.candidate.resolve(),'--native',core/'probe','--rust',rust,'--runtime',a.a2.resolve()/'runtime',
            '--ephemeris',a.provider.resolve()/'inpop17a.dat','--profile','combined' if profile=='qr-default' else profile],
            profile+'-paired',env=env,expected=None)
        records.append({'profile':profile,'comparison_exit':run.commands[-1]['status'],'rust':sha(rust)})
        print(records[-1],flush=True)
    run.run(['cargo','test','--offline','--locked','--release','--manifest-path',manifest,'--target-dir',a.target.resolve(),
        '--test','native_backends','--features','qr-default'],'qr-default-independent',env=env)
    run.save({'status':'failure' if any(r['comparison_exit'] for r in records) else 'pass','records':records})
    raise SystemExit(any(r['comparison_exit'] for r in records))
