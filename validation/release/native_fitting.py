#!/usr/bin/env python3
"""Portable rebuild of AE's full fitted-state observer for current/DDH gates."""
import argparse
import os
from pathlib import Path
import platform
from common import ROOT, PIN, Run, identities, sha
from native_core import stage_sources


def build(out, repository, a2):
    run = Run(out)
    out = run.out
    construction = ROOT / 'validation/release/native-fitting'
    src, rows = stage_sources(run, repository, construction, 'corrections.patch')
    cxx = os.environ.get('NATIVE_CXX', 'g++')
    flags = ['-O2', '-std=gnu++11', '-DHAVE_CONFIG_H', '-I' + str(a2 / 'source')]
    objects = []
    for name, *_ in rows:
        obj = out / (name + '.o')
        run.run([cxx, *flags, '-c', src / name, '-o', obj], 'compile-' + name)
        objects.append(obj)
    observer = ROOT / 'validation/general_fitting/native_fit.cc'
    run.run([cxx, *flags, observer, *objects, a2 / 'libtempo2-reference.a', '-lquadmath',
             '-lgfortran', '-o', out / 'fit'], 'link')
    run.run(['otool', '-L', out / 'fit'] if platform.system() == 'Darwin' else ['ldd', out / 'fit'], 'dependencies')
    run.save({'status': 'pass', 'scope': 'AE observer construction, comparisons separate',
              'pin': PIN, 'construction': identities(construction), 'source': identities(src),
              'observer': sha(observer), 'a2_receipt': sha(a2 / 'receipt.json'),
              'objects': {p.name: sha(p) for p in objects}, 'executable': sha(out / 'fit')})


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['out', 'native-repository', 'a2']:
        p.add_argument('--' + name, type=Path, required=True)
    a = p.parse_args()
    build(a.out, a.native_repository.resolve(), a.a2.resolve())
