#!/usr/bin/env python3
"""Portable clean native A2 build and explicit runtime; no historical output writes."""
import argparse
import difflib
import os
from pathlib import Path
import platform
import re
import shutil
import tarfile
from common import PIN, ROOT, Run, identities, sha


def build(out, repository):
    run = Run(out)
    out = run.out
    if repository is None:
        repository = out / 'upstream.git'
        run.run(['git', 'init', '--bare', repository], 'git-init')
        run.run(['git', '-C', repository, 'fetch', '--depth=1',
                 'https://github.com/wdong-eric/tempo2.git', PIN], 'git-fetch')
    repository = repository.resolve()
    archive = out / 'source.tar'
    run.run(['git', '-C', repository, 'archive', '--format=tar', '-o', archive, PIN], 'archive')
    src = out / 'source'
    src.mkdir()
    with tarfile.open(archive) as tar:
        tar.extractall(src, filter='data')
    archive_hash = sha(archive)
    archive.unlink()
    # Runtime bytes have a separate identity and are never installed implicitly.
    shutil.move(src / 'T2runtime', out / 'runtime')
    shutil.copyfile(src / 'TKlongdouble.float128.h', src / 'TKlongdouble.h')
    shutil.copyfile(src / 'cholesky.h', src / 'TKcholesky.h')
    (src / 'config.h').write_text('#define PACKAGE_STRING "Tempo2 2026.09.1"\n'
        '#define VERSION "2026.09.1"\n'
        f'#define TEMPO2_ARCH "{platform.system().lower()}-{platform.machine()}-al-reference"\n')
    cc, cxx, fc = (os.environ.get(k, default) for k, default in
        [('NATIVE_CC', 'gcc'), ('NATIVE_CXX', 'g++'), ('FC', 'gfortran')])
    for label, compiler in [('cc', cc), ('cxx', cxx), ('fc', fc)]:
        run.run([compiler, '--version'], label + '-version')
    make = (src / 'Makefile.am').read_text()
    fit = re.search(r'^FITFUNCS=(.*)$', make, re.M).group(1)
    sources = re.search(r'^libtempo2_la_SOURCES = (.*)$', make, re.M).group(1)
    sources = sources.replace('$(headers)', 'enum_str.C').replace('$(FITFUNCS)', fit).split()
    sofa = re.search(r'^libsofa_la_SOURCES=(.*)$', (src / 'sofa/Makefile.am').read_text(), re.M).group(1).split()
    # Match A2's raw blobs rather than git-archive export-subst substitutions.
    for name in sources + ['tempo2.C']:
        if name != 'enum_str.C':
            (src / name).write_bytes(run.run(['git', '-C', repository, 'show', PIN + ':' + name], 'raw-' + name))
    run.run(['bash', 'extract_enums.sh'], 'enums', cwd=src, stdin=(src / 'tempo2.h').read_bytes())
    run.run([cc, src / 'convertxt.c', '-o', out / 'convertxt'], 'convertxt-build')
    (src / 't2help.h').write_bytes(run.run([out / 'convertxt', src / 'help.txt', 'help_txt'], 'help') +
                                 run.run([out / 'convertxt', src / 'help_extra.txt', 'help_extra_txt'], 'help-extra'))
    a2 = ROOT / 'validation/release/a2'
    for row in (a2 / 'provenance.tsv').read_text().splitlines()[1:]:
        name, blob, digest, commit = row.split('\t')
        assert sha(a2 / name) == digest, name
    run.run(['patch', '--batch', '-p1', '-i', a2 / 'initializer-correction.patch'], 'initializer-correction', cwd=src)
    patches = []
    for name in sources + ['tempo2.C']:
        path = src / name
        original = path.read_text()
        changed, n = re.subn(r'\b(?:std::)?fopen\s*\(', 'reference_fopen(', original)
        if name == 'global.C':
            for call in ['toa2utc', 'tai2ut1', 'tt2tb', 'vectorPulsar', 'readEphemeris', 'get_obsCoord', 'calculate_bclt', 'formBats', 'secularMotion']:
                changed = re.sub(r'(?m)^(\s*)(' + call + r'\(psr,npsr[^;]*;)',
                                 r'\1{ reference_stage("' + call + r'"); \2 }', changed)
            n += 1
        if n:
            changed = '#include "resource_trace.h"\n' + changed
        path.write_text(changed)
        patches.extend(difflib.unified_diff(original.splitlines(True), changed.splitlines(True), fromfile=name, tofile=name))
    (out / 'instrumentation.patch').write_text(''.join(patches))
    flags = ['-O2', '-std=gnu++11', '-DHAVE_CONFIG_H', '-I' + str(src), '-I' + str(a2)]
    objects = []
    for name in sources:
        obj = out / (name + '.o')
        args = flags if name.endswith('.C') else [f for f in flags if not f.startswith('-std=')]
        run.run([cxx if name.endswith('.C') else cc, *args, '-fPIC', '-c', src / name, '-o', obj], 'compile-' + name)
        objects.append(obj)
    for name in sofa:
        obj = out / (name + '.o')
        run.run([fc, '-O2', '-fno-second-underscore', '-fPIC', '-c', src / 'sofa' / name, '-o', obj], 'compile-' + name)
        objects.append(obj)
    trace = out / 'resource_trace.o'
    run.run([cxx, *flags, '-c', a2 / 'resource_trace.cc', '-o', trace], 'trace-build')
    library = out / 'libtempo2-reference.a'
    run.run(['ar', 'rcs', library, *objects, trace], 'archive-library')
    libs = [library, '-lquadmath', '-lgfortran']
    for name, source in [('tempo2-reference', src / 'tempo2.C'), ('export_reference', a2 / 'export_reference.cc'),
                         ('initialization_probe', a2 / 'initialization_probe.cc')]:
        run.run([cxx, *flags, source, *libs, '-o', out / name], name + '-link')
    run.run([cxx, '-O2', a2 / 'precision_probe.cc', '-lquadmath', '-o', out / 'precision_probe'], 'precision-build')
    run.run([out / 'precision_probe'], 'precision')
    gtest = src / 'tests/gtest-1.7.0'
    testflags = ['-I' + str(gtest), '-I' + str(gtest / 'include'), '-DTEST_DELTA=1e-9l',
                 '-DDATDIR="' + str(src / 'tests/test_data') + '"', '-pthread']
    run.run([cxx, *flags, *testflags, '-c', gtest / 'src/gtest-all.cc', '-o', out / 'gtest.o'], 'gtest-build')
    for name in ['testTempo2', 'testEndToEnd']:
        run.run([cxx, *flags, *testflags, src / 'tests' / (name + '.C'), out / 'gtest.o', *libs, '-o', out / name], name + '-link')
    # The historical clock overlay is separately selected, never a production default.
    historic = run.run(['git', '-C', repository, 'show',
        '0759584ebf8315bbbe6a6d0b41c08ebbcfe527f2:T2runtime/clock/gps2utc.clk'], 'historic-clock', expected=None)
    if not historic and (out / 'historic-clock.stderr').stat().st_size:
        run.run(['git', '-C', repository, 'fetch', '--depth=1', 'https://github.com/wdong-eric/tempo2.git',
                 '0759584ebf8315bbbe6a6d0b41c08ebbcfe527f2'], 'historical-fetch')
        historic = run.run(['git', '-C', repository, 'show',
            '0759584ebf8315bbbe6a6d0b41c08ebbcfe527f2:T2runtime/clock/gps2utc.clk'], 'historic-clock-retry')
    (out / 'gps2utc.clk').write_bytes(historic)
    assert sha(out / 'gps2utc.clk') == '30727fdfaaec0f1775543804210d057ffe62076594bb8884d8f8ccafafb676be'
    overlay = out / 'runtime-historical'
    shutil.copytree(out / 'runtime', overlay, copy_function=os.link)
    (overlay / 'clock/gps2utc.clk').write_bytes(historic)
    for name in ['tempo2-reference', 'export_reference', 'precision_probe']:
        run.run(['otool', '-L', out / name] if platform.system() == 'Darwin' else ['ldd', out / name], name + '-dependencies')
    run.save({'status': 'pass', 'scope': 'A2 construction and precision; not integrated qualification',
              'source_pin': PIN, 'source_archive_sha256': archive_hash, 'a2_sources': identities(a2),
              'source': identities(src), 'runtime': identities(out / 'runtime'),
              'historical_clock': sha(out / 'gps2utc.clk'),
              'library': sha(library), 'instrumentation': sha(out / 'instrumentation.patch'),
              'executables': {name: sha(out / name) for name in ['tempo2-reference', 'export_reference',
                  'precision_probe', 'initialization_probe', 'testTempo2', 'testEndToEnd']}})


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--native-repository', type=Path)
    args = parser.parse_args()
    build(args.out, args.native_repository)
