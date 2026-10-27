#!/usr/bin/env python3
"""Route the unchanged AI comparator to explicitly built AL products/resources."""
import argparse
import importlib.util
from pathlib import Path
import sys
from common import ROOT

if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['out', 'source-root', 'native', 'rust', 'runtime', 'ephemeris']:
        p.add_argument('--' + name, required=True, type=Path)
    p.add_argument('--profile', choices=['disabled', 'blas', 'lapack', 'calceph', 'combined'], required=True)
    a = p.parse_args()
    path = a.source_root.resolve() / 'validation/native_backends/check.py'
    spec = importlib.util.spec_from_file_location('al_native_backend_check', path)
    check = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(check)
    check.RUNTIME = a.runtime.resolve()
    result = check.run(a.out, a.native.resolve(), a.rust.resolve(),
        a.profile in ('calceph', 'combined'), a.profile in ('blas', 'lapack', 'combined'),
        a.profile in ('lapack', 'combined'), a.ephemeris.resolve())
    print('failures:', result['failures'])
    sys.exit(bool(result['failures'] or result['changed_artifacts']))
