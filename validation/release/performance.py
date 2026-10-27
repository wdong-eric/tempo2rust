#!/usr/bin/env python3
"""Bounded installed serial measurements; no performance acceptance threshold."""
import argparse
import json
import os
from pathlib import Path
import platform
import re
import resource
import time
from common import ROOT, Run, sha
from predictor_compare import native_number

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','prefix','a2','core','predictor','runtime']:
        p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();run=Run(a.out);out=run.out;prefix=a.prefix.resolve()
    cc=os.environ.get('CC','clang');cxx=os.environ.get('NATIVE_CXX','g++')
    run.run([cc,'-O2','-std=c11','-D_POSIX_C_SOURCE=200809L','-I'+str(prefix/'include'),
        '-c',ROOT/'validation/parallel_execution/evaluation.c','-o',out/'client.o'],'client')
    run.run([cxx,out/'client.o',a.a2.resolve()/'libtempo2-reference.a','-lquadmath','-lgfortran','-o',out/'native'],'native-link')
    run.run([cc,out/'client.o','-L'+str(prefix/'lib'),'-ltempo2pred_rust','-Wl,-rpath,'+str(prefix/'lib'),'-o',out/'rust'],'rust-link')
    records=[];checksums={}
    env=dict(os.environ,TEMPO2=str(a.runtime.resolve()),VECLIB_MAXIMUM_THREADS='1',OPENBLAS_NUM_THREADS='1',LC_ALL='C')
    for sample in range(3):
        for engine in ['native','rust']:
            label=f'predictor-{engine}-{sample}'
            raw=run.run([out/engine,a.predictor.resolve(),10000],label,env=env).decode()
            match=re.fullmatch(r'calls (\d+) checksum (\S+) evaluate_s ([\d.]+)\n',raw)
            assert match,raw
            checksums[engine]=native_number(match[2]);records.append({'label':label,'calls':int(match[1]),'checksum':match[2],'evaluation_s':float(match[3])})
        assert checksums['native']==checksums['rust']
    fit=ROOT/'validation/general_fitting/inputs'
    # Same serial CLI input/output workload on both installed/native executables.
    for sample in range(3):
        for engine,exe in [('native',a.core.resolve()/'tempo2'),('rust',prefix/'bin/tempo2-rust')]:
            label=f'cli-{engine}-{sample}';work=out/label;work.mkdir()
            argv=[exe,'-npsr','2','-nobs','10000','-noaccel','-f',fit/'global0.par',fit/'multi-frequency.tim',
                '-f',fit/'global1.par',fit/'multi-frequency.tim','-dcf',fit/'covariance.model','-outpar','fitted.par']
            usage=work/'usage.txt'
            measure=['/usr/bin/time','-l','-o',usage] if platform.system()=='Darwin' else ['/usr/bin/time','-v','-o',usage]
            started=time.perf_counter();run.run([*measure,*argv],label,cwd=work,env=env)
            records.append({'label':label,'wall_s':time.perf_counter()-started,'usage':usage.read_text(),
                'products':{p.name:sha(p) for p in work.iterdir() if p.is_file() and p!=usage}})
    run.save({'status':'pass','meaning':'Measurements only; fidelity is checked by separate gates; no speed threshold',
        'records':records,'inputs':{str(p):sha(p) for p in [a.predictor,a.core/'tempo2',prefix/'bin/tempo2-rust',
            ROOT/'validation/parallel_execution/evaluation.c',a.a2/'receipt.json']}})
