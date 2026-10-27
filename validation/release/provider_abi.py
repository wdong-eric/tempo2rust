#!/usr/bin/env python3
"""Measure actual selected BLAS/LAPACK/Fortran/CALCEPH ABI, not package labels."""
import argparse
import os
from pathlib import Path
import platform
from common import ROOT, Run, sha

if __name__ == '__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','provider','ephemeris']:
        p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();run=Run(a.out);out=run.out;provider=a.provider.resolve()
    cc=os.environ.get('CC','clang');fc=os.environ.get('FC','gfortran')
    for label,compiler in [('c',cc),('fortran',fc)]:run.run([compiler,'--version'],label+'-version')
    run.run([fc,'-fPIC','-c',ROOT/'validation/release/fortran_abi.f90','-o',out/'fortran.o'],'fortran-build')
    flags=['-framework','Accelerate'] if platform.system()=='Darwin' else ['-DT2_OPENBLAS','-lopenblas','-ldl']
    run.run([cc,'-O2','-D_GNU_SOURCE','-DT2_BLAS','-DT2_LAPACK','-I'+str(provider/'include'),
        ROOT/'validation/release/provider_probe.c',ROOT/'rust/native/backends.c',out/'fortran.o',
        provider/'lib/libcalceph.a',*flags,'-o',out/'probe'],'link')
    output=run.run([out/'probe',a.ephemeris.resolve()],'measure').decode()
    run.run(['otool','-L',out/'probe'] if platform.system()=='Darwin' else ['ldd',out/'probe'],'loaded-dependencies')
    if platform.system()=='Linux':
        run.run(['dpkg-query','-W','libopenblas-pthread-dev','libopenblas0-pthread','liblapacke-dev','gfortran-13'],'packages')
    run.save({'status':'pass','measurement':output,'executable':sha(out/'probe'),
        'inputs':{str(f):sha(f) for f in [ROOT/'validation/release/provider_probe.c',ROOT/'validation/release/fortran_abi.f90',
             ROOT/'rust/native/backends.c',provider/'include/calceph.h',provider/'lib/libcalceph.a',a.ephemeris]}})
