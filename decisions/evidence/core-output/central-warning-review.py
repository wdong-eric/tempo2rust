"""Locate the retained AH parallel-test assertion differences without rerunning it."""
from pathlib import Path
import argparse
import hashlib
import json
import re

p = argparse.ArgumentParser()
p.add_argument('--root', type=Path, default=Path('/Users/Eric.Dong/Downloads/tempo2rust'))
p.add_argument('--out', type=Path, required=True)
a = p.parse_args()
log = a.root / 'validation/core_output/build/tests-debug.log'
before = hashlib.sha256(log.read_bytes()).hexdigest()
text = log.read_text()
rows = [line.split(': ', 1)[1] for line in text.splitlines()
        if line.startswith(('  left: ', ' right: '))]
assert len(rows) == 2
# serde_json::Value's Rust Debug wrappers are removed lexically; JSON string
# tokens are consumed whole so source text within them is never rewritten.
pattern = r'String\(("(?:[^"\\]|\\.)*")\)|Number\(([^)]+)\)|Bool\((true|false)\)|Object |Array |\bNull\b|"(?:[^"\\]|\\.)*"'


def unwrap(m):
    for i in [1, 2, 3]:
        if m[i] is not None:
            return m[i]
    return {'Object ': '', 'Array ': '', 'Null': 'null'}.get(m[0], m[0])


left, right = [json.loads(re.sub(pattern, unwrap, row)) for row in rows]
differences = []
checked_equal_scalars = 0


def compare(left, right, path=''):
    global checked_equal_scalars
    assert type(left) is type(right), path
    if isinstance(left, dict):
        assert left.keys() == right.keys(), path
        for key in sorted(left):
            compare(left[key], right[key], path + '/' + key)
    elif isinstance(left, list):
        if len(left) != len(right):
            differences.append({'path': path, 'left': left, 'right': right})
        else:
            for i, (l, r) in enumerate(zip(left, right)):
                compare(l, r, path + '/' + str(i))
    elif left != right:
        differences.append({'path': path, 'left': left, 'right': right})
    else:
        checked_equal_scalars += 1


compare(left, right)
assert {d['path'] for d in differences} == {
    '/cycles/0/postfit/warnings', '/cycles/1/prefit/warnings', '/cycles/1/postfit/warnings'}
assert hashlib.sha256(log.read_bytes()).hexdigest() == before
result = {'scope': 'Diagnosis of the one retained assertion; no concurrent execution, race cause, or concurrency acceptance is claimed.',
          'source': str(log), 'source_sha256': before,
          'reviewer_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
          'failure': 'fitting_tests::terrestrial_fit_failure_recovers_with_the_same_runtime at src/fitting_tests.rs:302',
          'differences': differences, 'equal_scalar_values': checked_equal_scalars,
          'original_failure_retained': True}
a.out.parent.mkdir(parents=True, exist_ok=True)
a.out.write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps({'receipt': str(a.out), 'sha256': hashlib.sha256(a.out.read_bytes()).hexdigest(),
                  'equal_scalar_values': checked_equal_scalars, 'different_warning_lists': len(differences)}, indent=2))
