#!/usr/bin/env python3
"""Measure native Lomb/QR contraction; compare every default-profile quantity."""
import argparse
import importlib.util
import json
import os
from pathlib import Path
from common import ROOT, Run, sha


def comparator(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def measure(out, a2, core, rust_lomb, rust_backend):
    run = Run(out)
    out = run.out
    cxx = os.environ.get('NATIVE_CXX', 'g++')
    source = (a2 / 'source/TKspectrum.C').read_text()
    start = source.index('void TKlomb_d(')
    end = source.index('// Fourier transform', start)
    body = source[start:end].replace('void TKlomb_d(', 'void al_lomb(', 1)
    (out / 'lomb.cc').write_text('#include <cmath>\n#include "T2toolkit.h"\n' + body)
    flags = ['-O2', '-DHAVE_CONFIG_H', '-I' + str(a2 / 'source')]
    run.run([cxx, '-O2', '-ffp-contract=off', '-c', ROOT / 'validation/release/lomb_driver.cc',
             '-o', out / 'driver.o'], 'driver')
    run.run([cxx, '--version'], 'compiler')
    run.run([cxx, '-dumpmachine'], 'target')
    run.run([cxx, '-O2', '-Q', '--help=optimizers'], 'optimizers')
    identity = json.loads((core / 'receipt.json').read_text())
    qr_compile = next(c['argv'] for c in identity['commands'] if c['label'] == 'compile-T2accel.C')
    qr_link = next(c['argv'] for c in identity['commands'] if c['label'] == 'probe-link')
    outputs = {}
    for name, contract in [('default', []), ('off', ['-ffp-contract=off']), ('fast', ['-ffp-contract=fast'])]:
        obj = out / ('lomb-' + name + '.o')
        exe = out / ('lomb-' + name)
        run.run([cxx, *flags, *contract, '-c', out / 'lomb.cc', '-o', obj], 'lomb-' + name + '-compile')
        run.run([cxx, *flags, *contract, '-S', out / 'lomb.cc', '-o', out / ('lomb-' + name + '.s')], 'lomb-' + name + '-assembly')
        run.run([cxx, out / 'driver.o', obj, a2 / 'libtempo2-reference.a', '-lquadmath', '-lgfortran', '-o', exe], 'lomb-' + name + '-link')
        outputs[name] = run.run([exe], 'lomb-' + name).decode().splitlines()
        command = [str(out / ('qr-' + name + '.o')) if arg == str(core / 'T2accel.C.o') else arg for arg in qr_compile]
        run.run([*command, *contract], 'qr-' + name + '-compile')
        run.run([('-S' if arg == '-c' else str(out / ('qr-' + name + '.s')) if arg == str(out / ('qr-' + name + '.o')) else arg)
                 for arg in command] + contract, 'qr-' + name + '-assembly')
        command = [str(out / ('qr-' + name + '.o')) if arg == str(core / 'T2accel.C.o') else
                   str(out / ('qr-' + name)) if arg == str(core / 'probe') else arg for arg in qr_link]
        run.run(command, 'qr-' + name + '-link')
    rust = run.run([rust_lomb], 'lomb-rust').decode().splitlines()
    # Preserve every input and bin in source order, including the Nyquist bin.
    reference_order = [line.split('\t')[:3] for line in outputs['default']]
    for name, lines in [*outputs.items(), ('rust', rust)]:
        assert [line.split('\t')[:3] for line in lines] == reference_order, name
    units = {'x': 'f64_exact', 'y': 'f64_exact', 'count': 'exact', 'variance': 's^2',
             'frequency': 'frequency_day', 'power': 'dimensionless'}
    for name, lines in [*outputs.items(), ('rust', rust)]:
        rows = [line.split('\t') for line in lines]
        (out / ('lomb-' + name + '.tsv')).write_text(''.join(
            f'{case}.{kind}.{index}\t{units[kind]}\t{value}\n' for case, kind, index, bits, value in rows))
    ag = comparator('al_ag', ROOT / 'validation/analysis_tools/compare.py')
    lomb = ag.compare(out / 'lomb-default.tsv', out / 'lomb-rust.tsv')
    ai = comparator('al_ai', ROOT / 'validation/native_backends/check.py')
    qr = []
    for constraint in (0, 1):
        for scale in (0, 1):
            for mask in range(8):
                tag = f'{constraint}-{scale}-{mask}'
                for engine, exe in [('default', out / 'qr-default'), ('off', out / 'qr-off'),
                                    ('fast', out / 'qr-fast'), ('rust', rust_backend)]:
                    run.run([exe, 'solver', out / f'qr-{engine}-{tag}.tsv', constraint, scale, mask], f'qr-{engine}-{tag}')
                qr.append({'case': tag, 'comparison': ai.compare(out / f'qr-default-{tag}.tsv', out / f'qr-rust-{tag}.tsv'),
                           'default_equals_off': (out / f'qr-default-{tag}.tsv').read_bytes() == (out / f'qr-off-{tag}.tsv').read_bytes(),
                           'default_equals_fast': (out / f'qr-default-{tag}.tsv').read_bytes() == (out / f'qr-fast-{tag}.tsv').read_bytes()})
    failures = len(lomb['failures']) + sum(len(r['comparison']['failures']) for r in qr)
    run.save({'status': 'failure' if failures else 'pass', 'lomb': lomb, 'qr': qr,
              'failures': failures, 'lomb_default_equals_off': outputs['default'] == outputs['off'],
              'lomb_default_equals_fast': outputs['default'] == outputs['fast'],
              'inputs': {str(p): sha(p) for p in [a2 / 'receipt.json', core / 'receipt.json',
                         rust_lomb, rust_backend, ROOT / 'validation/analysis_tools/compare.py',
                         ROOT / 'validation/native_backends/check.py']}})
    return failures


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['out', 'a2', 'core', 'rust-lomb', 'rust-backend']:
        p.add_argument('--' + name, required=True, type=Path)
    a = p.parse_args()
    raise SystemExit(bool(measure(a.out, a.a2.resolve(), a.core.resolve(), a.rust_lomb.resolve(), a.rust_backend.resolve())))
