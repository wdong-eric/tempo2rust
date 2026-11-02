#!/usr/bin/env python3
"""Compare direct canonical results and count allocations outside timing runs."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import subprocess

if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['before', 'after', 'predictor', 'out']:
        p.add_argument('--'+name, type=Path, required=True)
    a = p.parse_args()
    out = a.out.resolve(); out.mkdir(parents=True, exist_ok=False)
    env = dict(os.environ, LC_ALL='C')
    for key in ['TEMPO2', 'TEMPO2_CLOCK_PATH', 'DYLD_INSERT_LIBRARIES', 'LD_PRELOAD']:
        env.pop(key, None)
    sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
    commands, inputs, allocations = [], {}, {}
    science = None
    def run(argv, label, extra_env=None):
        argv = list(map(str, argv))
        r = subprocess.run(argv, capture_output=True, env=dict(env, **(extra_env or {})))
        (out/(label+'.stdout')).write_bytes(r.stdout); (out/(label+'.stderr')).write_bytes(r.stderr)
        commands.append(dict(label=label, command=argv, extra_environment=extra_env or {}, status=r.returncode))
        assert r.returncode == 0, (label, r.stderr)
        return r
    for engine, directory in [('before', a.before.resolve()), ('after', a.after.resolve())]:
        run([directory/'edges'], engine+'-edges')
        for mode in ['from', 'owned', 'leaf', 'model']:
            result = run([directory/'plain', a.predictor.resolve(), mode, 1], engine+'-'+mode+'-plain')
            lines = result.stdout.splitlines()
            assert len(lines) == 29 and lines[-1].startswith(b'TIME ')
            if science is None: science = lines[:-1]
            assert lines[:-1] == science, (engine, mode)
            counts = {}
            for count in [1, 1001]:
                darwin = platform.system() == 'Darwin'
                executable = directory/('plain' if darwin else 'counted')
                extra = {'DYLD_INSERT_LIBRARIES': str(directory/'counter.dylib')} if darwin else {}
                result = run([executable, a.predictor.resolve(), mode, count], f'{engine}-{mode}-{count}', extra)
                assert result.stdout.splitlines()[:-1] == science
                match = re.fullmatch(rb'AO_ALLOC malloc=(\d+) calloc=(\d+) realloc=(\d+) free=(\d+) bytes=(\d+)\n', result.stderr)
                assert match, result.stderr
                counts[count] = dict(zip(['malloc', 'calloc', 'realloc', 'free', 'bytes'], map(int, match.groups())))
                assert counts[count]['malloc'] > 0, 'observer did not intercept calls'
            # The argv length changes by three bytes between 1 and 1001.
            # Keep that tiny whole-process byte delta visible, without correction.
            allocations[engine+'-'+mode] = dict(raw=counts,
                per_iteration={key: (counts[1001][key]-counts[1][key])/1000 for key in counts[1]})
        for name in ['plain', 'edges', 'receipt.json', 'counter.dylib' if platform.system() == 'Darwin' else 'counted']:
            inputs[str(directory/name)] = sha(directory/name)
    before = (out/'before-edges.stdout').read_bytes()
    assert before == (out/'after-edges.stdout').read_bytes(), 'canonical edge results changed'
    assert len(before.splitlines()) == 5880
    for path in [a.predictor.resolve(), Path(__file__)]: inputs[str(path)] = sha(path)
    (out/'receipt.json').write_text(json.dumps(dict(platform=platform.platform(),
        commands=commands, inputs=inputs, allocations=allocations,
        exact_edge_results=5880, exact_finite_results=len(science),
        files={path.name:sha(path) for path in out.iterdir() if path.is_file()}), indent=2)+'\n')
    print('5880 edge results and 28 finite results exactly match')
    print(json.dumps({key:value['per_iteration'] for key,value in allocations.items()}, indent=2))
