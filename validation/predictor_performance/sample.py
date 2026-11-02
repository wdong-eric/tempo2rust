#!/usr/bin/env python3
"""Bounded macOS sample of the unchanged installed evaluation client."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import time


def summarize(path):
    graph = path.read_text().split('Call graph:\n')[1].split('Total number in stack')[0]
    total = int(re.search(r'(\d+) Thread_', graph)[1])
    result = {'total': total}
    patterns = {'read': 'predictor_ffi4read', 'write': 'predictor_ffi5write',
                'polynomial': 'predictor_ffi10polynomial', 'evaluate': 'Cheby2D8evaluate',
                'native_get': 't2r_ld_get', 'locale': 'newlocale',
                'allocators': r'(?:malloc|calloc|realloc|free|_xzm_)'}
    for name, pattern in patterns.items():
        stack, count = [], 0
        for line in graph.splitlines():
            match = re.match(r'([\s+!:|]*)(\d+) (\S.*)', line)
            if not match: continue
            depth = len(match[1])
            while stack and stack[-1][0] >= depth: stack.pop()
            selected = bool(re.search(pattern, match[3].split('  (in ')[0]))
            if selected and not any(parent[1] for parent in stack): count += int(match[2])
            stack.append((depth, selected))
        result[name] = {'samples': count, 'percent': 100*count/total}
    return result


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['executable', 'predictor', 'out']:
        p.add_argument('--'+name, type=Path, required=True)
    p.add_argument('--iterations', type=int, default=400000)
    a = p.parse_args()
    out = a.out.resolve(); out.mkdir(parents=True, exist_ok=False)
    records = []
    for label in ['plain', 'sampled']:
        command = [str(a.executable.resolve()), str(a.predictor.resolve()), str(a.iterations)]
        with (out/(label+'.stdout')).open('w') as stdout, (out/(label+'.stderr')).open('w') as stderr:
            start = time.perf_counter()
            process = subprocess.Popen(command, stdout=stdout, stderr=stderr)
            if label == 'sampled':
                sample = ['/usr/bin/sample', str(process.pid), '5', '1', '-file', str(out/'sample.txt')]
                r = subprocess.run(sample, capture_output=True)
                (out/'sample.stdout').write_bytes(r.stdout); (out/'sample.stderr').write_bytes(r.stderr)
            status = process.wait()
            records.append(dict(label=label, command=command, status=status, wall_s=time.perf_counter()-start))
            assert status == 0
            if label == 'sampled': assert r.returncode == 0, r.stderr
    sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
    (out/'receipt.json').write_text(json.dumps(dict(records=records, summary=summarize(out/'sample.txt'),
        inputs={str(path.resolve()): sha(path) for path in [a.executable, a.predictor, Path(__file__)]}), indent=2)+'\n')
