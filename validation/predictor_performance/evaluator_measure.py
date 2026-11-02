#!/usr/bin/env python3
"""Fresh interleaved direct Rust leaf/model timings after frozen-output checks."""
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
    for name in ['before', 'after', 'predictor', 'out']:
        p.add_argument('--'+name, type=Path, required=True)
    p.add_argument('--iterations', type=int, default=100000)
    p.add_argument('--samples', type=int, default=7)
    a = p.parse_args()
    out = a.out.resolve(); out.mkdir(parents=True, exist_ok=False)
    engines = {name:getattr(a, name).resolve() for name in ['before', 'after']}
    env = dict(os.environ, LC_ALL='C', VECLIB_MAXIMUM_THREADS='1', OPENBLAS_NUM_THREADS='1')
    for key in ['TEMPO2', 'TEMPO2_CLOCK_PATH', 'DYLD_INSERT_LIBRARIES', 'LD_PRELOAD']:
        env.pop(key, None)
    order = [(mode, sample, engine) for mode in ['leaf', 'model']
             for sample in range(a.samples) for engine in engines]
    random.Random(124).shuffle(order)
    records, science = [], None
    for mode, sample, engine in order:
        label = f'{engine}-{mode}-{sample}'; usage = out/(label+'.usage')
        argv = [str(engines[engine]), str(a.predictor.resolve()), mode, str(a.iterations)]
        command = ['/usr/bin/time', '-l' if platform.system() == 'Darwin' else '-v', '-o', str(usage), *argv]
        start = time.perf_counter()
        r = subprocess.run(command, capture_output=True, env=env)
        wall = time.perf_counter()-start
        (out/(label+'.stdout')).write_bytes(r.stdout); (out/(label+'.stderr')).write_bytes(r.stderr)
        assert r.returncode == 0, (label, r.stderr)
        lines = r.stdout.splitlines()
        assert len(lines) == 29
        if science is None: science = lines[:-1]
        assert lines[:-1] == science
        match = re.fullmatch(rb'TIME (\w+) (\d+) ([\d.]+)', lines[-1])
        assert match and match[1].decode() == mode and int(match[2]) == a.iterations
        cpu, rss = parse_usage(usage.read_text(), platform.system())
        records.append(dict(label=label, engine=engine, mode=mode, sample=sample,
            iterations=a.iterations, command=command, evaluation_s=float(match[3]),
            wall_s=wall, cpu_s=cpu, rss_bytes=rss, load_average=os.getloadavg()))
        print(label, f'{float(match[3]):.9f}s', flush=True)
    sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
    (out/'receipt.json').write_text(json.dumps(dict(platform=platform.platform(), records=records,
        inputs={str(path):sha(path) for path in [*engines.values(), a.predictor.resolve(), Path(__file__), ROOT/'validation/release/process_usage.py']},
        environment={key:env.get(key) for key in ['LC_ALL', 'TEMPO2', 'TEMPO2_CLOCK_PATH', 'VECLIB_MAXIMUM_THREADS', 'OPENBLAS_NUM_THREADS', 'DYLD_INSERT_LIBRARIES', 'LD_PRELOAD']},
        files={path.name:sha(path) for path in out.iterdir() if path.is_file()}), indent=2)+'\n')
