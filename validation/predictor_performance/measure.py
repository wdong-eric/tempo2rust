#!/usr/bin/env python3
"""Fresh interleaved installed C evaluations; build/qualify before the quiet window.

Uses the unchanged portable release evaluation client and resource parser.
Complete science is checked separately with outputs.c and qualify_predictor.py.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import random
import re
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT/'validation/release'))
from process_usage import parse_usage

if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['before', 'after', 'native', 'predictor', 'out']:
        p.add_argument('--'+name, type=Path, required=True)
    for name in ['before-period', 'after-period', 'native-period']:
        p.add_argument('--'+name, type=Path, required=True)
    p.add_argument('--iterations', type=int, default=100000)
    p.add_argument('--samples', type=int, default=7)
    a = p.parse_args()
    out = a.out.resolve(); out.mkdir(parents=True, exist_ok=False)
    engines = {name: getattr(a, name).resolve() for name in ['before', 'after', 'native']}
    period = {name: getattr(a, name+'_period').resolve() for name in engines}
    env = dict(os.environ, LC_ALL='C', VECLIB_MAXIMUM_THREADS='1', OPENBLAS_NUM_THREADS='1')
    env.pop('TEMPO2', None); env.pop('TEMPO2_CLOCK_PATH', None)
    sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
    records = []
    # One iteration exposes load/parse/startup costs; the longer run records
    # the complete loaded phase/frequency interval and whole-process resources.
    order = [(n, sample, engine) for n in [0, 1, a.iterations]
             for sample in range(a.samples) for engine in engines]
    random.Random(123).shuffle(order)
    for n, sample, engine in order:
        label = f'{engine}-{n}-{sample}'
        usage = out/(label+'.usage')
        argv = ([str(period[engine]), str(a.predictor.resolve()), '58000.01'] if n == 0
                else [str(engines[engine]), str(a.predictor.resolve()), str(n)])
        command = ['/usr/bin/time', '-l' if platform.system() == 'Darwin' else '-v', '-o', str(usage), *argv]
        start = time.perf_counter()
        r = subprocess.run(command, capture_output=True, env=env)
        wall = time.perf_counter()-start
        (out/(label+'.stdout')).write_bytes(r.stdout); (out/(label+'.stderr')).write_bytes(r.stderr)
        assert r.returncode == 0, (label, r.stderr)
        match = re.fullmatch(rb'calls (\d+) checksum (\S+) evaluate_s ([\d.]+)\n', r.stdout) if n else None
        if n: assert match, (label, r.stdout)
        cpu, rss = parse_usage(usage.read_text(), platform.system())
        records.append(dict(label=label, engine=engine, iterations=n, sample=sample,
            command=command, wall_s=wall, cpu_s=cpu, rss_bytes=rss,
            evaluation_s=float(match[3]) if match else None,
            checksum=match[2].decode() if match else r.stdout.decode(),
            load_average=os.getloadavg()))
        print(label, f'{float(match[3]) if match else wall:.6f}s', flush=True)
    for n in [0, 1, a.iterations]:
        assert len({r['checksum'] for r in records if r['iterations'] == n}) == 1
    receipt = dict(platform=platform.platform(), records=records,
        inputs={str(path): sha(path) for path in [*engines.values(), *period.values(), a.predictor.resolve(), Path(__file__), ROOT/'validation/parallel_execution/evaluation.c']},
        environment={key: env.get(key) for key in ['LC_ALL', 'TEMPO2', 'TEMPO2_CLOCK_PATH', 'VECLIB_MAXIMUM_THREADS', 'OPENBLAS_NUM_THREADS']},
        files={path.name: sha(path) for path in out.iterdir() if path.is_file()})
    (out/'receipt.json').write_text(json.dumps(receipt, indent=2)+'\n')
