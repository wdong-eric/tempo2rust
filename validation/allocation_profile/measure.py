#!/usr/bin/env python3
"""Fresh interleaved allocation controls and the unchanged AK serial CLI workloads.

Instrumented allocation intervals are not full-workflow timings. All samples,
including failed processes, are saved. There is no performance acceptance gate.
"""
import argparse
import datetime
import json
import os
from pathlib import Path
import platform
import statistics
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'validation/release'))
from common import sha
from process_usage import parse_usage


def execute(a):
    out = a.out.resolve(); out.mkdir(parents=True, exist_ok=False)
    builds = {'before': a.before.resolve(), 'after': a.after.resolve()}
    receipts = {k: json.loads((v / 'receipt.json').read_text()) for k, v in builds.items()}
    for name, directory in builds.items():
        assert all(sha(directory / 'bin' / p) == h for p, h in receipts[name]['binaries'].items())
    native = a.native_cli.resolve(); runtime = a.a2.resolve() / 'runtime'
    resources = {str(p.relative_to(runtime)): sha(p) for p in sorted(runtime.rglob('*')) if p.is_file()}
    env = dict(os.environ, TEMPO2=str(runtime), LC_ALL='C', AK_WORKERS='1', AK_REPEATS='5',
               OMP_NUM_THREADS='1', OMP_DYNAMIC='FALSE', OMP_THREAD_LIMIT='4', OMP_MAX_ACTIVE_LEVELS='1',
               VECLIB_MAXIMUM_THREADS='1', OPENBLAS_NUM_THREADS='1')
    records = []

    def run(label, cmd, scope, sample, engine, case):
        work = out / label; work.mkdir()
        usage = work / 'usage.txt'
        measure = ['/usr/bin/time', '-l' if platform.system() == 'Darwin' else '-v', '-o', str(usage)]
        started = datetime.datetime.now(datetime.timezone.utc).isoformat()
        load = os.getloadavg()
        start = time.perf_counter()
        with (work / 'stdout').open('wb') as stdout, (work / 'stderr').open('wb') as stderr:
            proc = subprocess.run([*measure, *map(str, cmd)], cwd=work, env=env,
                                  stdout=stdout, stderr=stderr, timeout=300)
        wall = time.perf_counter() - start
        cpu, rss = parse_usage(usage.read_text(), platform.system())
        row = {'label': label, 'scope': scope, 'sample': sample, 'engine': engine, 'case': case,
               'command': list(map(str, cmd)), 'started_utc': started, 'load_before': load,
               'load_after': os.getloadavg(), 'exit': proc.returncode,
               'wall_s': wall, 'cpu_s': cpu, 'maxrss_bytes': rss}
        if scope == 'observer' and proc.returncode == 0:
            row['phases'] = [json.loads(l) for l in (work / 'stdout').read_text().splitlines()]
            for phase in row['phases']:
                for r in phase['ranges']:
                    assert r['payload_bytes'] == 800000000
                    if r['allocation_live']:
                        assert r['mincore_errno'] == 0
                    assert r['resident_pages'] <= r['pages']
        if scope == 'cli':
            row['products'] = {str(p.relative_to(work)): sha(p) for p in sorted(work.rglob('*'))
                               if p.is_file() and p.name not in ['stdout', 'stderr', 'usage.txt']}
        records.append(row)
        (out / 'records.json').write_text(json.dumps(records, indent=2) + '\n')
        print(label, proc.returncode, f'{wall:.4f}s {rss / 2**20:.2f} MiB', flush=True)

    controls = [('matrix', n, 'untouched') for n in [1, 2]]
    controls += [('context', n, 'untouched') for n in [1, 2]]
    controls += [(kind, 1, control) for kind in ['matrix', 'context'] for control in ['read', 'zero', 'write', 'cow']]
    controls += [('context', 1, 'reinit')]
    for sample in range(a.samples):
        engines = ['before', 'after', 'native'] if sample % 2 == 0 else ['native', 'after', 'before']
        for kind, count, control in controls:
            for engine in engines:
                if engine == 'native' and control in ['cow', 'reinit']:
                    continue  # C++ has no independent-context COW contract.
                directory = builds['before'] if engine == 'native' else builds[engine]
                exe = directory / 'bin' / ('observe-native' if engine == 'native' else 'observe')
                case = f'{kind}-{count}-{control}'
                run(f'{case}-{engine}-{sample}', [exe, kind, count, control], 'observer', sample, engine, case)
    fit = ROOT / 'validation/general_fitting/inputs'
    pred = ROOT / 'validation/predictors/fixtures/isolated.par'
    cases = {
        'correlated-global': ['-f', fit / 'global0.par', fit / 'multi-frequency.tim',
                              '-f', fit / 'global1.par', fit / 'multi-frequency.tim',
                              '-dcf', fit / 'covariance.model', '-outpar', 'fitted.par'],
        'cheby': ['-f', pred, '-pred', 'PKS 58000 58000.03 1000 2000 8 4 3600']}
    for sample in range(a.samples):
        for case, args in cases.items():
            for engine in (['before', 'after', 'native'] if sample % 2 == 0 else ['native', 'after', 'before']):
                exe = native if engine == 'native' else builds[engine] / 'bin/cli'
                run(f'{case}-{engine}-{sample}', [exe, '-npsr', '2', '-nobs', '10000', '-noaccel', *args],
                    'cli', sample, engine, case)
    products = {}
    for case in cases:
        rows = [r for r in records if r['scope'] == 'cli' and r['case'] == case]
        good = [r for r in rows if r['engine'] in builds and r['exit'] == 0]
        products[case] = bool(good) and len(good) == 2 * a.samples and bool(good[0]['products']) and all(
            r['products'] == good[0]['products'] for r in good)
    summary = []
    for scope, case, engine in sorted({(r['scope'], r['case'], r['engine']) for r in records}):
        rows = [r for r in records if (r['scope'], r['case'], r['engine']) == (scope, case, engine)]
        summary.append({'scope': scope, 'case': case, 'engine': engine, 'samples': len(rows),
            **{k: {'median': statistics.median(r[k] for r in rows), 'min': min(r[k] for r in rows),
                   'max': max(r[k] for r in rows)} for k in ['wall_s', 'cpu_s', 'maxrss_bytes']}})
    inputs = {str(p): sha(p) for p in [pred, fit / 'global0.par', fit / 'global1.par',
               fit / 'multi-frequency.tim', fit / 'covariance.model', Path(__file__), native,
               a.a2 / 'receipt.json', ROOT / 'validation/parallel_execution/performance.py']}
    # Bind the complete runtime tree actually read, including paths absent from old receipts.
    assert resources == {str(p.relative_to(runtime)): sha(p) for p in sorted(runtime.rglob('*')) if p.is_file()}
    for name, directory in builds.items():
        assert all(sha(directory / 'bin' / p) == h for p, h in receipts[name]['binaries'].items())
    result = {'builds': {k: {'path': str(v), 'receipt_sha256': sha(v / 'receipt.json')} for k, v in builds.items()},
              'host': platform.platform(), 'inputs': inputs, 'runtime_files': resources,
              'environment': {k: env[k] for k in ['TEMPO2', 'LC_ALL', 'AK_WORKERS', 'AK_REPEATS', 'OMP_NUM_THREADS',
               'OMP_DYNAMIC', 'OMP_THREAD_LIMIT', 'OMP_MAX_ACTIVE_LEVELS', 'VECLIB_MAXIMUM_THREADS', 'OPENBLAS_NUM_THREADS']},
              'records': records, 'summary': summary, 'exact_rust_product_membership_and_bytes': products,
              'failed_processes': [r['label'] for r in records if r['exit'] != 0]}
    (out / 'receipt.json').write_text(json.dumps(result, indent=2) + '\n')
    assert not result['failed_processes'] and all(products.values()), 'see retained receipt'


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['before', 'after', 'native-cli', 'a2', 'out']:
        p.add_argument('--' + name, type=Path, required=True)
    p.add_argument('--samples', type=int, default=5)
    a = p.parse_args(); assert a.samples >= 3
    execute(a)
