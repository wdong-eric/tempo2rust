#!/usr/bin/env python3
"""Pinned PIC CALCEPH and scientific input bootstrap; dependencies stay in build/."""
import argparse
import sys
if sys.version_info < (3, 12):
    raise SystemExit("Python 3.12 or newer is required for safe archive extraction")
from pathlib import Path
import shutil
import tarfile
import urllib.request
from common import Run, sha

SOURCE = ('https://codeload.github.com/achalumeau/calceph/tar.gz/718e7945ccd27216b522243c3ff8aab63ce1d99d',
          'b38f6ccc726bab51c5901bb74c655e13941753ca7c3d0054396a37b793040426')
DATA = ('https://ftp.imcce.fr/pub/ephem/planets/inpop17a/inpop17a_TDB_m100_p100_tt.dat',
        '32517ec82bfb298d53baeba4065750dec82143fab79357e4487f84fb439bc49c')


def build(out, cmake, source=None, data=None):
    run = Run(out)
    out = run.out
    for name, local, (url, digest) in [('calceph.tar.gz', source, SOURCE), ('inpop17a.dat', data, DATA)]:
        target = out / name
        if local:
            shutil.copyfile(local, target)
        else:
            with urllib.request.urlopen(url, timeout=120) as response, target.open('wb') as stream:
                shutil.copyfileobj(response, stream)
        if sha(target) != digest:
            run.save({'status': 'failure', 'input': name, 'actual': sha(target), 'expected': digest})
            raise ValueError('pinned input digest mismatch: ' + name)
    with tarfile.open(out / 'calceph.tar.gz') as archive:
        archive.extractall(out / 'source', filter='data')
    src = out / 'source/calceph-718e7945ccd27216b522243c3ff8aab63ce1d99d'
    prefix = out / 'prefix'
    run.run([cmake, '--version'], 'cmake-version')
    run.run([cmake, '-S', src, '-B', out / 'cmake', '-DCMAKE_INSTALL_PREFIX=' + str(prefix),
             '-DENABLE_FORTRAN=OFF', '-DCMAKE_BUILD_TYPE=Release', '-DBUILD_SHARED_LIBS=OFF',
             '-DCMAKE_POSITION_INDEPENDENT_CODE=ON'], 'configure')
    run.run([cmake, '--build', out / 'cmake', '--parallel', '2'], 'build')
    run.run([cmake, '--install', out / 'cmake'], 'install')
    run.save({'status': 'pass', 'scope': 'dependency construction; not scientific qualification',
              'source_url': SOURCE[0], 'source_sha256': SOURCE[1], 'data_url': DATA[0],
              'data_sha256': DATA[1], 'header': sha(prefix / 'include/calceph.h'),
              'archive': sha(prefix / 'lib/libcalceph.a'), 'builder': sha(__file__)})


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--cmake', default='cmake')
    parser.add_argument('--source', type=Path)
    parser.add_argument('--data', type=Path)
    a = parser.parse_args()
    build(a.out, a.cmake, a.source, a.data)
