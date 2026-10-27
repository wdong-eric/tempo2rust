#!/usr/bin/env python3
"""Rebuild reviewed corrected core/provider source on the actual release host."""
import argparse
import difflib
import os
from pathlib import Path
import platform
import shutil
from common import PIN, ROOT, Run, identities, sha


def stage_sources(run, native_repository, construction, patch):
    """Reconstitute the declared reviewed source set before target adaptation."""
    out = run.out
    src = out / 'source'
    src.mkdir()
    rows = [row.split('\t') for row in (construction / 'provenance.tsv').read_text().splitlines()[1:]]
    for name, raw_hash, compiled_hash, origin, receipt, staged_hash in rows:
        if name.endswith('.cc'):
            shutil.copyfile(construction / name, src / name)
        else:
            raw = run.run(['git', '-C', native_repository, 'show', PIN + ':' + name], 'raw-' + name)
            (src / name).write_bytes(raw)
            assert sha(src / name) == raw_hash, name
            # Predecessor Python builders normalized text newlines. Record the
            # raw blob above, then reproduce that construction before patching.
            (src / name).write_text((src / name).read_text())
    run.run(['patch', '--batch', '-p1', '-i', construction / patch], 'corrections', cwd=src)
    for name, raw_hash, compiled_hash, origin, receipt, staged_hash in rows:
        assert sha(src / name) == staged_hash, name
    return src, rows


def build(out, native_repository, a2, provider, profile):
    run = Run(out)
    out = run.out
    construction = ROOT / 'validation/release/native'
    src, rows = stage_sources(run, native_repository, construction, 'core-corrections.patch')
    blas = profile in ('blas', 'lapack', 'combined', 'qr-default')
    lapack = profile in ('lapack', 'combined', 'qr-default')
    calceph = profile in ('calceph', 'combined', 'qr-default')
    config = (a2 / 'source/config.h').read_text() + '\n#define F77_FUNC(name,NAME) name ## _\n'
    for macro, active in [('HAVE_BLAS', blas), ('HAVE_LAPACK', lapack), ('HAVE_CALCEPH', calceph), ('QR_DEFAULT', profile == 'qr-default')]:
        if active:
            config += '#define ' + macro + ' 1\n'
    (src / 'config.h').write_text(config)
    shutil.copyfile(a2 / 'source/T2accel.h', src / 'T2accel.h')
    # Linux GCC Fortran character lengths are part of the ABI, unlike the
    # Accelerate f2c declarations. Keep every source matrix/solver expression.
    # Official lapack.h macros supply these lengths for LAPACK; BLAS uses CBLAS.
    if platform.system() == 'Linux' and blas:
        path = src / 'T2accel.C'
        original = path.read_text()
        start = original.index('#ifdef HAVE_BLAS')
        end = original.index('/**', start)
        declarations = '''#include <cblas.h>
#include <lapack.h>
static_assert(sizeof(blasint)==4 && sizeof(lapack_int)==4,"LP64 provider required");
#define F77_dgemv(t,m,n,alpha,a,lda,x,incx,beta,y,incy) cblas_dgemv(CblasColMajor,*(t)=='T'?CblasTrans:CblasNoTrans,*(m),*(n),*(alpha),a,*(lda),x,*(incx),*(beta),y,*(incy))
#define F77_dgemm(ta,tb,m,n,k,alpha,a,lda,b,ldb,beta,c,ldc) cblas_dgemm(CblasColMajor,*(ta)=='T'?CblasTrans:CblasNoTrans,*(tb)=='T'?CblasTrans:CblasNoTrans,*(m),*(n),*(k),*(alpha),a,*(lda),b,*(ldb),*(beta),c,*(ldc))
#define F77_dpotf2 LAPACK_dpotf2
#define F77_dpotrf LAPACK_dpotrf
#define F77_dtptri LAPACK_dtptri
#define F77_dgels LAPACK_dgels
#define F77_dtrmm(lr,u,t,d,n,m,alpha,a,lda,b,ldb) cblas_dtrmm(CblasColMajor,*(lr)=='R'?CblasRight:CblasLeft,*(u)=='U'?CblasUpper:CblasLower,*(t)=='T'?CblasTrans:CblasNoTrans,*(d)=='N'?CblasNonUnit:CblasUnit,*(n),*(m),*(alpha),a,*(lda),b,*(ldb))
#ifdef HAVE_LAPACK
'''
        modified = original[:start] + declarations + original[end:]
        path.write_text(modified)
        (out / 'linux-provider-abi.patch').write_text(''.join(difflib.unified_diff(
            original.splitlines(True), modified.splitlines(True), fromfile='reviewed/T2accel.C', tofile='linux/T2accel.C')))
    cc = os.environ.get('NATIVE_CC', 'gcc')
    cxx = os.environ.get('NATIVE_CXX', 'g++')
    flags = ['-O2', '-DHAVE_CONFIG_H', '-I' + str(src), '-I' + str(a2 / 'source'), '-I' + str(provider / 'include')]
    objects = []
    for name, *_ in rows:
        obj = out / (name + '.o')
        run.run([cc if name.endswith('.c') else cxx, *flags, '-fPIC', '-c', src / name, '-o', obj], 'compile-' + name)
        objects.append(obj)
    libraries = [a2 / 'libtempo2-reference.a']
    if calceph:
        libraries.append(provider / 'lib/libcalceph.a')
    if blas:
        libraries += ['-framework', 'Accelerate'] if platform.system() == 'Darwin' else ['-lopenblas']
    libraries += ['-lquadmath', '-lgfortran']
    main = run.run(['git', '-C', native_repository, 'show', PIN + ':tempo2.C'], 'main-source')
    (src / 'tempo2.C').write_bytes(main)
    for name, source in [('tempo2', src / 'tempo2.C'), ('probe', ROOT / 'validation/native_backends/probe.cc')]:
        run.run([cxx, *flags, source, *objects, *libraries, '-o', out / name], name + '-link')
        run.run(['otool', '-L', out / name] if platform.system() == 'Darwin' else ['ldd', out / name], name + '-dependencies')
    run.save({'status': 'pass', 'scope': 'corrected native construction; comparisons separate',
              'profile': profile, 'source_pin': PIN, 'source': identities(src),
              'construction': identities(construction), 'a2_archive': sha(a2 / 'libtempo2-reference.a'),
              'a2_receipt': sha(a2 / 'receipt.json'), 'objects': {p.name: sha(p) for p in objects},
              'executables': {name: sha(out / name) for name in ['tempo2', 'probe']}})


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['out', 'native-repository', 'a2', 'provider']:
        p.add_argument('--' + name, type=Path, required=True)
    p.add_argument('--profile', choices=['disabled', 'blas', 'lapack', 'calceph', 'combined', 'qr-default'], required=True)
    a = p.parse_args()
    build(a.out, a.native_repository.resolve(), a.a2.resolve(), a.provider.resolve(), a.profile)
