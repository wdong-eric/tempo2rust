"""Review the saved macOS event controls and Lomb production correction."""
from pathlib import Path
import hashlib
import importlib.util
import json
import subprocess
import sys

sys.dont_write_bytecode = True
root = Path('/Users/Eric.Dong/Downloads/tempo2rust')
build = root / 'validation/release/build'
out = Path('/tmp/tempo2-central-event-products-review-20261031')
out.mkdir(exist_ok=False)
files = {}

def sha(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()

def bind(path, expected=None):
    path = Path(path)
    digest = sha(path)
    assert expected is None or digest == expected, path
    files[str(path)] = digest

sys.path.insert(0, str(root / 'validation/release'))
from check_timing import compare
from event_profile import comparison
profile = build / 'second-followthrough-events-profile-v2'
profile_receipt = json.loads((profile / 'receipt.json').read_text())
rows = comparison(profile / 'native.tsv', profile / 'rust.tsv')
assert rows == profile_receipt['rows'] and len(rows) == 209
assert not any(not row['equal'] for row in rows)
bind(profile / 'native.tsv'); bind(profile / 'rust.tsv')
for path, expected in profile_receipt['inputs'].items(): bind(path, expected)
for name in ['ordinary_long_double.rs', 'event_profile_tests.rs', 't2fit_glitch.rs']:
    content = subprocess.check_output(['git', 'show', '5c8c0aa:rust/src/' + name], cwd=root)
    assert hashlib.sha256(content).hexdigest() == sha(root / 'rust/src' / name)

events = build / 'second-followthrough-events-v1'
record = json.loads((events / 'receipt.json').read_text())
assert record['status'] == 'pass' and not record['invariant_failures'] and not record['scientific_failures']
for path, expected in record['inputs'].items(): bind(path, expected)
controls = []
for case in ['events', 'without-events', 'gaussian-only']:
    for mean in [0, 1]:
        rust = (build / f'second-followthrough-timing-v1/events-mean{mean}/rust.json'
                if case == 'events' else events / f'{case}-{mean}-rust.json')
        value = json.loads(rust.read_text())
        baseline = events / f'{case}-{mean}-baseline.stdout'
        assert json.loads(baseline.read_text()) == value
        bind(rust); bind(baseline)
        original = events / f'{case}-{mean}-original/native.tsv'
        for variant in ['original', 'observed', 'f64-temporaries']:
            table = events / f'{case}-{mean}-{variant}/native.tsv'
            assert table.read_bytes() == original.read_bytes()
            errors, measurements = compare(table.read_text(), value)
            expected = next(r for r in record['records'] if (r['case'], r['mean'], r['variant']) == (case, mean, variant))
            assert not errors and errors == expected['rust_errors'] and measurements == expected['measurements']
            assert sha(rust) == expected['rust_output_sha256']
            bind(table)
        controls.append({'case': case, 'mean': mean, 'exact_before_after': True})
assert controls == record['product_controls']

fitroot = build / 'second-followthrough-event-fit-v1'
fit = json.loads((fitroot / 'receipt.json').read_text())
assert fit['status'] == 'pass' and fit['native_failures'] == 0 and fit['exact_before_after']
for path, expected in fit['inputs'].items(): bind(path, expected)
actual_fit = Path(next(p for p in fit['inputs'] if p.endswith('/rust.json')))
assert json.loads(actual_fit.read_text()) == json.loads((fitroot / 'baseline.stdout').read_text())
bind(fitroot / 'baseline.stdout')

lombroot = build / 'second-followthrough-lomb-v1'
lomb = json.loads((lombroot / 'receipt.json').read_text())
for path, expected in lomb['inputs'].items():
    path = Path(path)
    bind(path if path.is_absolute() else root / path, expected)
old = subprocess.check_output(['git', 'show', '4accd26:rust/src/cholesky_spectrum.rs'], cwd=root, text=True)
old = old[old.index('pub fn lomb('):old.index('/// TK_weightLS')].strip()
frozen = (root / 'validation/release/lomb_baseline.rs').read_text()
assert old == frozen[frozen.index('pub fn lomb('):].strip()
assert (lombroot / 'installed.stdout').read_bytes() == (lombroot / 'copied.stdout').read_bytes() == (lombroot / 'historical-fused.stdout').read_bytes()
spec = importlib.util.spec_from_file_location('central_lomb_compare', root / 'validation/analysis_tools/compare.py')
module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
for variant in ['installed', 'historical-fused', 'unfused']:
    for native in ['default', 'off', 'fast']:
        reference = build / 'integrated-v1/contraction' / ('lomb-' + native + '.tsv')
        table = lombroot / (variant + '.tsv')
        assert module.compare(reference, table) == lomb['comparisons'][variant][native]
        bind(reference); bind(table)

for directory in [profile, events, fitroot, lombroot]: bind(directory / 'receipt.json')
result = {'scope': 'Offline macOS source/control review; Linux execution and milestone acceptance remain pending',
    'production_commits': ['3e5f344', '5c8c0aa'], 'review_source': sha(__file__),
    'event_profile_checks': len(rows), 'event_products': controls, 'event_fit_exact_before_after': True,
    'lomb': lomb['comparisons'], 'files': files}
(out / 'receipt.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps({'out': str(out), 'sha256': sha(out / 'receipt.json'), 'files': len(files),
                  'event_profile_checks': len(rows), 'exact_event_products': len(controls)}))
