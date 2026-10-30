"""Offline central review of AL's committed observation and cache controls."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

sys.dont_write_bytecode = True
p = argparse.ArgumentParser(description=__doc__)
p.add_argument('--root', type=Path, required=True)
p.add_argument('--focused', type=Path, required=True)
p.add_argument('--out', type=Path, required=True)
a = p.parse_args()
root, out = a.root.resolve(), a.out.resolve()
out.mkdir(parents=True, exist_ok=False)
verified = {}

def sha(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()

def check(path, expected):
    path = Path(path)
    assert sha(path) == expected, path
    verified[str(path)] = expected

focused = json.loads((a.focused / 'receipt.json').read_text())
assert all(row['exit'] == 0 for row in focused['commands'])
for name, expected in focused['sources'].items():
    data = subprocess.check_output(['git', 'show', '0104447:' + name], cwd=root)
    assert hashlib.sha256(data).hexdigest() == expected, name
    target = out / 'source' / name
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(data)
sys.path.insert(0, str(out / 'source/validation/release'))
from m2 import observer_differences
from polyco_serialization import serialization

build = root / 'validation/release/build'
m2root = build / 'second-followthrough-m2-v1'
m2 = json.loads((m2root / 'receipt.json').read_text())
comparison = json.loads((build / 'second-followthrough-m2-comparison-v1/receipt.json').read_text())
check(m2root / 'receipt.json', comparison['observer_receipt'])
check(m2root / 'signed_nan_exporter.cc', m2['signed_observer']['source'])
check(m2root / 'observer.patch', m2['signed_observer']['patch'])
check(root / 'validation/fitting/export_reference.cc', m2['signed_observer']['historical_source'])
check(root / 'validation/fitting/compare.py', comparison['comparator'])
for key in ['inputs']:
    for path, expected in comparison[key].items(): check(path, expected)
    for path, expected in comparison['raw_observer'][key].items(): check(path, expected)
rawdir = Path(next(iter(comparison['raw_observer']['inputs']))).parent
neutral = 0
for name, expected in m2['outputs'].items():
    check(rawdir / name, expected)
for name, expected in m2['signed_observer']['outputs'].items():
    check(m2root / 'signed-reference' / name, expected)
raw_json = {name for name in m2['outputs'] if name.endswith('.json')}
signed_json = {name for name in m2['signed_observer']['outputs'] if name.endswith('.json')}
assert raw_json == signed_json
for name in sorted(raw_json):
    raw, signed = rawdir / name, m2root / 'signed-reference' / name
    assert bool(raw.stat().st_size) == bool(signed.stat().st_size)
    if raw.stat().st_size:
        assert observer_differences(json.loads(raw.read_text()), json.loads(signed.read_text())) == 0
        neutral += 1
assert neutral == 31 and len(comparison['records']) == 50
assert comparison['failures'] == comparison['raw_observer']['failures'] == 2
assert [r['name'] for r in comparison['records'] if r['failures']] == ['dependent_columns-shared-0']

predroot = build / 'second-followthrough-predictors-v2'
pred = json.loads((predroot / 'receipt.json').read_text())
check(out / 'source/validation/release/polyco_serialization.py', pred['source_serialization']['observer'])
assert len(pred['cases']) == 14
errors = [f for case in pred['cases'] for f in case['failures']]
assert len(errors) == 7 and all(f['field'].endswith('.log10rms') for f in errors)
serialization_rows = []
for case in pred['cases']:
    directory = predroot / (case['name'] + '-' + case['kind'])
    for name, expected in case['files'].items(): check(directory / name, expected)
    generated = json.loads((directory / 'rust/generate.stdout').read_text())['result']
    kind = case['kind']
    original = generated['predictor']['Cheby']['segments'] if kind == 'cheby' else generated['models']['segments']
    for name in (['t2pred.dat'] if kind == 'cheby' else ['polyco_new.dat', 'newpolyco.dat']):
        readback = json.loads((directory / ('exchange-' + name + '-rust-0-rust.stdout')).read_text())
        parsed = readback['input']['predictor']['Cheby' if kind == 'cheby' else 'T1']['segments']
        result = serialization(kind, original, parsed, directory / 'rust' / name)
        assert not result.failures, (case['name'], name, result.failures)
        serialization_rows.append({'case': case['name'], 'file': name, 'checks': result.count})
assert len(serialization_rows) == 21

cache_root = build / 'second-followthrough-cache-v2'
cache = json.loads((cache_root / 'receipt.json').read_text())
for name, expected in cache['files'].items(): check(cache_root / name, expected)
for name, expected in cache['inputs'].items(): check(name, expected)
assert all(r['exit'] == 0 for r in cache['records'])
blocks = lambda b: sorted(re.findall(rb'AK_BEGIN (\d+) (\d+)\n(.*?)AK_END \1 \2\n', b, re.S))
original = blocks((cache_root / 'asan-execute.stdout').read_bytes())
assert {(int(i), int(j)) for i, j, _ in original} == {(i, j) for i in range(4) for j in range(3)}
assert len(original) == 12
for label in ['asan-cache-cleanup', 'asan-library-owned']:
    assert blocks((cache_root / (label + '-execute.stdout')).read_bytes()) == original
storage = {f.name: sha(f) for f in (cache_root / 'asan').glob('*.tsv')}
assert storage
for label in ['asan-cache-cleanup', 'asan-library-owned']:
    assert {f.name: sha(f) for f in (cache_root / label).glob('*.tsv')} == storage
identity = cache['ownership_control']['identity']
prefix = Path(next(name for name in cache['inputs'] if name.endswith('libtempo2pred_rust.a'))).parents[1]
for name, expected in identity['source'].items():
    check(prefix / 'share/tempo2-rust/rust' / name, expected)
    if name != 'src/predictor_ffi.rs': check(cache_root / 'library-owned-cache-source' / name, expected)
assert identity['changed'] == ['src/predictor_ffi.rs']
check(cache_root / 'library-owned-cache-source/src/predictor_ffi.rs', identity['guard_source'])
check(cache_root / 'library-owned-cache.patch', identity['patch'])
check(cache_root / 'cache-target/release/libtempo2_rust.a', identity['archive'])

receipt = {'scope': 'Offline macOS observer/cache evidence review; no Linux acceptance or production cache change',
    'observer_commit': '0104447', 'cache_commit': 'ca60751', 'review_source': sha(__file__),
    'focused_receipt': sha(a.focused / 'receipt.json'), 'm2_neutral_products': neutral,
    'm2_records': len(comparison['records']), 'm2_retained_failures': comparison['failures'],
    'predictor_cases': len(pred['cases']), 'predictor_retained_rms_failures': len(errors),
    'serialization_rows': serialization_rows, 'cache_complete_workflows': len(original),
    'cache_storage_products': len(storage), 'guard_changed_files': identity['changed'],
    'verified_files': verified}
(out / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
print(json.dumps({'out': str(out), 'sha256': sha(out / 'receipt.json'), 'files': len(verified),
    'm2_neutral': neutral, 'predictor_serializations': len(serialization_rows), 'workflows': len(original)}))
